//! 🏢️ EN 1991 gravity mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️change-assumed-construction-qk/🦀️.rs"]
pub mod change_assumed_construction_qk;
#[path = "../🔥change-construction-activity/🦀️.rs"]
pub mod change_construction_activity;
#[path = "../🏢change-floor-assumed-qk/🦀️.rs"]
pub mod change_floor_assumed_qk;
#[path = "../🚧change-self-weight-assumed-gk/🦀️.rs"]
pub mod change_self_weight_assumed_gk;
#[path = "../➕️insert-floors/🦀️.rs"]
pub mod insert_floors;
#[path = "../➕️insert-self-weight-elements/🦀️.rs"]
pub mod insert_self_weight_elements;
#[path = "../➖️remove-floors/🦀️.rs"]
pub mod remove_floors;
#[path = "../➖️remove-self-weight-elements/🦀️.rs"]
pub mod remove_self_weight_elements;
//#endregion 🪢️Mounts
