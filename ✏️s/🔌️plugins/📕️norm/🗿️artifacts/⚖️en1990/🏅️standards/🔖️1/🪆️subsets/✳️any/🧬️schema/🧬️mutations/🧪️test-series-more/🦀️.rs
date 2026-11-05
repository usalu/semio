//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧪️change-test-std-dev/🦀️.rs"]
pub mod change_test_std_dev;
#[path = "../🧪️change-test-variation-known/🦀️.rs"]
pub mod change_test_variation_known;
#[path = "../🧪️change-test-conversion-factor/🦀️.rs"]
pub mod change_test_conversion_factor;
#[path = "../🧪️change-test-required/🦀️.rs"]
pub mod change_test_required;
#[path = "../🧪️change-test-theory-variation/🦀️.rs"]
pub mod change_test_theory_variation;
//#endregion 🪢️Mounts
