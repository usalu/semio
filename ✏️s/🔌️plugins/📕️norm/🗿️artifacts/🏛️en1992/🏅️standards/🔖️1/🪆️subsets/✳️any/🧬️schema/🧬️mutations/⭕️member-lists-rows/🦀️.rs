//! ⭕ EN 1992 member lists rows mutation leaves: action, bar layer, section point and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⭕️insert-bar-layer/🦀️.rs"]
pub mod insert_bar_layer;
#[path = "../⭕️remove-bar-layer/🦀️.rs"]
pub mod remove_bar_layer;
#[path = "../🏋️insert-action/🦀️.rs"]
pub mod insert_action;
#[path = "../🏋️remove-action/🦀️.rs"]
pub mod remove_action;
#[path = "../📐️insert-section-point/🦀️.rs"]
pub mod insert_section_point;
#[path = "../📐️remove-section-point/🦀️.rs"]
pub mod remove_section_point;
//#endregion 🪢️Mounts
