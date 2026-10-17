//! 🪜️ EN 1998 buildings mutation leaves (part 2).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-elevation-regular/🦀️.rs"]
pub mod change_elevation_regular;
#[path = "../⚖️change-storey-permanent-gk-n/🦀️.rs"]
pub mod change_storey_permanent_gk_n;
#[path = "../💪️change-system-v-rd-n/🦀️.rs"]
pub mod change_system_v_rd_n;
#[path = "../➕️insert-building/🦀️.rs"]
pub mod insert_building;
#[path = "../➖️remove-building/🦀️.rs"]
pub mod remove_building;
//#endregion 🪢️Mounts
