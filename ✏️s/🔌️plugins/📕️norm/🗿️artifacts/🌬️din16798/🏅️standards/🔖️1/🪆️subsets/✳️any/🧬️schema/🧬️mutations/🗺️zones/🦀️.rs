//! 🗺️ DIN EN 16798 zones mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
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
#[path = "../🏢️change-zone-usage-type/🦀️.rs"]
pub mod change_zone_usage_type;
#[path = "../📐️change-zone-vent-method/🦀️.rs"]
pub mod change_zone_vent_method;
#[path = "../🔗change-zone-vent-system-id/🦀️.rs"]
pub mod change_zone_vent_system_id;
#[path = "../➕️insert-zone/🦀️.rs"]
pub mod insert_zone;
#[path = "../➖️remove-zone/🦀️.rs"]
pub mod remove_zone;
//#endregion 🪢️Mounts
