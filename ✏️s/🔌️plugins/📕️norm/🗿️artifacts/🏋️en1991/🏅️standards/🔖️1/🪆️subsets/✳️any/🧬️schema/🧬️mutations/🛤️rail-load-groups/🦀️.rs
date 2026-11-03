//! 🛤️ RailLoadGroup mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️insert-rail-load-group/🦀️.rs"]
pub mod insert_rail_load_group;
#[path = "../⚙️remove-rail-load-group/🦀️.rs"]
pub mod remove_rail_load_group;
#[path = "../⚙️change-rail-load-group-assumed-lateral/🦀️.rs"]
pub mod change_rail_load_group_assumed_lateral;
#[path = "../⚙️change-rail-load-group-assumed-longitudinal/🦀️.rs"]
pub mod change_rail_load_group_assumed_longitudinal;
#[path = "../⚙️change-rail-load-group-assumed-udl/🦀️.rs"]
pub mod change_rail_load_group_assumed_udl;
#[path = "../⚙️change-rail-load-group-assumed-axle-load/🦀️.rs"]
pub mod change_rail_load_group_assumed_axle_load;
#[path = "../⚙️change-rail-load-group-factor-lateral/🦀️.rs"]
pub mod change_rail_load_group_factor_lateral;
#[path = "../⚙️change-rail-load-group-factor-longitudinal/🦀️.rs"]
pub mod change_rail_load_group_factor_longitudinal;
#[path = "../⚙️change-rail-load-group-factor-vertical/🦀️.rs"]
pub mod change_rail_load_group_factor_vertical;
#[path = "../🛤️change-rail-load-group-load-group/🦀️.rs"]
pub mod change_rail_load_group_load_group;
//#endregion 🪢️Mounts
