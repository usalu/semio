//! 🌀 EN 1991 dynamic elements mutation leaves: dynamic element, dynamic element height, dynamic element width and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌀️change-dynamic-element-height/🦀️.rs"]
pub mod change_dynamic_element_height;
#[path = "../🌀️change-dynamic-element-width/🦀️.rs"]
pub mod change_dynamic_element_width;
#[path = "../🌀️change-dynamic-first-frequency/🦀️.rs"]
pub mod change_dynamic_first_frequency;
#[path = "../🌀️change-dynamic-log-decrement/🦀️.rs"]
pub mod change_dynamic_log_decrement;
#[path = "../🌀️insert-dynamic-element/🦀️.rs"]
pub mod insert_dynamic_element;
#[path = "../🌀️remove-dynamic-element/🦀️.rs"]
pub mod remove_dynamic_element;
//#endregion 🪢️Mounts
