//! 💥 EN 1991 impact actions position mutation leaves: impact action assumed force, impact action height, impact action x and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../💥️change-impact-action-height/🦀️.rs"]
pub mod change_impact_action_height;
#[path = "../💥️change-impact-action-x/🦀️.rs"]
pub mod change_impact_action_x;
#[path = "../💥️change-impact-action-y/🦀️.rs"]
pub mod change_impact_action_y;
#[path = "../💥️change-impact-action-assumed-force/🦀️.rs"]
pub mod change_impact_action_assumed_force;
//#endregion 🪢️Mounts
