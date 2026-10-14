//! 🏢️ DIN V 18599 building mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱change-attachment/🦀️.rs"]
pub mod change_attachment;
#[path = "../🎛️change-automation-class/🦀️.rs"]
pub mod change_automation_class;
#[path = "../🏠️change-building-category/🦀️.rs"]
pub mod change_building_category;
#[path = "../🌉change-delta-u-wb/🦀️.rs"]
pub mod change_delta_u_wb;
#[path = "../⚖️change-geg-qp-factor/🦀️.rs"]
pub mod change_geg_qp_factor;
#[path = "../📦change-heated-volume-m3/🦀️.rs"]
pub mod change_heated_volume_m3;
#[path = "../🧮change-method/🦀️.rs"]
pub mod change_method;
#[path = "../📐️change-net-floor-area-m2/🦀️.rs"]
pub mod change_net_floor_area_m2;
#[path = "../🏷️change-use-class/🦀️.rs"]
pub mod change_use_class;
//#endregion 🪢️Mounts
