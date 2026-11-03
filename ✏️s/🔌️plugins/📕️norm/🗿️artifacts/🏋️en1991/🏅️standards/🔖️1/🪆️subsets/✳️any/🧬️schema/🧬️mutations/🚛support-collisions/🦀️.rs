//! 🚛 SupportCollision mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️change-support-collision-position-x/🦀️.rs"]
pub mod change_support_collision_position_x;
#[path = "../⚙️change-support-collision-height/🦀️.rs"]
pub mod change_support_collision_height;
#[path = "../⚙️change-support-collision-assumed-force-y/🦀️.rs"]
pub mod change_support_collision_assumed_force_y;
#[path = "../⚙️change-support-collision-assumed-force-x/🦀️.rs"]
pub mod change_support_collision_assumed_force_x;
#[path = "../⚙️insert-support-collision/🦀️.rs"]
pub mod insert_support_collision;
#[path = "../⚙️remove-support-collision/🦀️.rs"]
pub mod remove_support_collision;
#[path = "../🛤️change-support-collision-location/🦀️.rs"]
pub mod change_support_collision_location;
//#endregion 🪢️Mounts
