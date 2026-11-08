//! 🔰 DIN V 18599 envelope element numeric field leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⬜change-element-area-m2/🦀️.rs"]
pub mod change_element_area_m2;
#[path = "../🔶change-element-orientation-deg/🦀️.rs"]
pub mod change_element_orientation_deg;
#[path = "../🔷change-element-tilt-deg/🦀️.rs"]
pub mod change_element_tilt_deg;
#[path = "../🔸change-element-g-value/🦀️.rs"]
pub mod change_element_g_value;
#[path = "../🔹change-element-fc/🦀️.rs"]
pub mod change_element_fc;
#[path = "../🔻change-element-adjacency/🦀️.rs"]
pub mod change_element_adjacency;
//#endregion 🪢️Mounts
