//! 🔗️ EN 1998 critical region mutation leaves (part 1).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-critical-region/🦀️.rs"]
pub mod insert_critical_region;
#[path = "../➖️remove-critical-region/🦀️.rs"]
pub mod remove_critical_region;
#[path = "../✏️change-critical-region-building-id/🦀️.rs"]
pub mod change_critical_region_building_id;
#[path = "../✏️change-critical-region-member-id/🦀️.rs"]
pub mod change_critical_region_member_id;
#[path = "../📍️change-critical-region-section-width-m/🦀️.rs"]
pub mod change_critical_region_section_width_m;
#[path = "../📍️change-critical-region-section-depth-m/🦀️.rs"]
pub mod change_critical_region_section_depth_m;
#[path = "../📍️change-critical-region-clear-length-m/🦀️.rs"]
pub mod change_critical_region_clear_length_m;
#[path = "../📍️change-critical-region-hooped-length-m/🦀️.rs"]
pub mod change_critical_region_hooped_length_m;
//#endregion 🪢️Mounts
