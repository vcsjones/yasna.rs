// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UtcTime {
    bytes: Vec<u8>,
}

impl UtcTime {
    pub fn new(bytes: Vec<u8>) -> Self {
        return UtcTime {
            bytes: bytes,
        };
    }
}
