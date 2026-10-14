//! 🧱️ DIN 4108 elements mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../↔️change-element-adjacent/🦀️.rs"]
pub mod change_element_adjacent;
#[path = "../📐️change-element-area/🦀️.rs"]
pub mod change_element_area;
#[path = "../📈️change-element-delta-uf/🦀️.rs"]
pub mod change_element_delta_u_f;
#[path = "../📈️change-element-delta-ug/🦀️.rs"]
pub mod change_element_delta_u_g;
#[path = "../📈️change-element-delta-ur/🦀️.rs"]
pub mod change_element_delta_u_r;
#[path = "../📐change-element-inclination-deg/🦀️.rs"]
pub mod change_element_inclination_deg;
#[path = "../🏷️change-element-kind/🦀️.rs"]
pub mod change_element_kind;
#[path = "../🧭change-element-orientation-deg/🦀️.rs"]
pub mod change_element_orientation_deg;
#[path = "../🏠️insert-element/🦀️.rs"]
pub mod insert_element;
#[path = "../🚫️remove-element/🦀️.rs"]
pub mod remove_element;
//#endregion 🪢️Mounts
