//! 🔁 FatigueModel mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️change-fatigue-model-assumed-udl/🦀️.rs"]
pub mod change_fatigue_model_assumed_udl;
#[path = "../⚙️change-fatigue-model-assumed-axle-load/🦀️.rs"]
pub mod change_fatigue_model_assumed_axle_load;
#[path = "../⚙️insert-fatigue-model/🦀️.rs"]
pub mod insert_fatigue_model;
#[path = "../⚙️remove-fatigue-model/🦀️.rs"]
pub mod remove_fatigue_model;
#[path = "../🏗️change-fatigue-model-lane/🦀️.rs"]
pub mod change_fatigue_model_lane;
#[path = "../🛤️change-fatigue-model-model/🦀️.rs"]
pub mod change_fatigue_model_model;
//#endregion 🪢️Mounts
