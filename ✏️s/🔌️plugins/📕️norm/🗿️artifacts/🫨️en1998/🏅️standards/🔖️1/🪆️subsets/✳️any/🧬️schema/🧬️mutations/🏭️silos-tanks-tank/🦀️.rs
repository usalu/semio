//! 🏭️ EN 1998 silos-tanks mutation leaves: tank.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛢️insert-tank/🦀️.rs"]
pub mod insert_tank;
#[path = "../➖️remove-tank/🦀️.rs"]
pub mod remove_tank;
#[path = "../✏️change-tank-height-m/🦀️.rs"]
pub mod change_tank_height_m;
#[path = "../✏️change-tank-radius-m/🦀️.rs"]
pub mod change_tank_radius_m;
#[path = "../✏️change-tank-permanent-gk-n/🦀️.rs"]
pub mod change_tank_permanent_gk_n;
#[path = "../✏️change-tank-content-qk-n/🦀️.rs"]
pub mod change_tank_content_qk_n;
#[path = "../✏️change-tank-content-category/🦀️.rs"]
pub mod change_tank_content_category;
//#endregion 🪢️Mounts
