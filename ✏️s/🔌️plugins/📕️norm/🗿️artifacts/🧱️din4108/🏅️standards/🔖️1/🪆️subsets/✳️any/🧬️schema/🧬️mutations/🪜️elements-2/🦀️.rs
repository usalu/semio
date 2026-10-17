//! 🪜️ DIN 4108 elements mutation leaves (part 2).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📈️change-element-delta-uf/🦀️.rs"]
pub mod change_element_delta_u_f;
#[path = "../📈️change-element-delta-ur/🦀️.rs"]
pub mod change_element_delta_u_r;
#[path = "../🏷️change-element-kind/🦀️.rs"]
pub mod change_element_kind;
#[path = "../🏠️insert-element/🦀️.rs"]
pub mod insert_element;
#[path = "../🚫️remove-element/🦀️.rs"]
pub mod remove_element;
//#endregion 🪢️Mounts
