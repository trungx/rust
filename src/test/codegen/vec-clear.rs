// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-debug: the debug assertions get in the way
// compile-flags: -O

#![crate_type = "lib"]

// CHECK-LABEL: @vec_clear
#[no_mangle]
pub fn vec_clear(x: &mut Vec<u32>) {
    // CHECK-NOT: load
    // CHECK-NOT: icmp
    x.clear()
}
