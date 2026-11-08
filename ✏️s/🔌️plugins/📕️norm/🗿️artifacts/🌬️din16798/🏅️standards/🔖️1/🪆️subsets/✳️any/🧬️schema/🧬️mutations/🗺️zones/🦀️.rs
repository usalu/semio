//! 🗺️ DIN EN 16798 zones mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛋️change-zone-comfort-category/🦀️.rs"]
pub mod change_zone_comfort_category;
#[path = "../📐️change-zone-floor-area/🦀️.rs"]
pub mod change_zone_floor_area;
#[path = "../👥️change-zone-occupants/🦀️.rs"]
pub mod change_zone_occupants;
#[path = "../🏭️change-zone-pollution-class/🦀️.rs"]
pub mod change_zone_pollution_class;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_din16798_mut_zones_more::{change_zone_id, change_zone_name, change_zone_usage_type, change_zone_vent_method, change_zone_vent_system_id, insert_zone, remove_zone};
