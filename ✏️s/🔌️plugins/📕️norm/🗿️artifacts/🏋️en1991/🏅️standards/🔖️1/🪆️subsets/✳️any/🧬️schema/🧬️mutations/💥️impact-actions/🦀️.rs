//! 💥 EN 1991 impact actions mutation leaves: impact action, impact action kind, impact action mass and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../💥️change-impact-action-kind/🦀️.rs"]
pub mod change_impact_action_kind;
#[path = "../💥️change-impact-action-mass/🦀️.rs"]
pub mod change_impact_action_mass;
#[path = "../💥️change-impact-action-speed/🦀️.rs"]
pub mod change_impact_action_speed;
#[path = "../💥️change-impact-action-stiffness/🦀️.rs"]
pub mod change_impact_action_stiffness;
#[path = "../💥️insert-impact-action/🦀️.rs"]
pub mod insert_impact_action;
#[path = "../💥️remove-impact-action/🦀️.rs"]
pub mod remove_impact_action;
//#endregion 🪢️Mounts
