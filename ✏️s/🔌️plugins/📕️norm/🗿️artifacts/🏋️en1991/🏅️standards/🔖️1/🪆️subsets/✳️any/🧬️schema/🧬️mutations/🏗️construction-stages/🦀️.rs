//! 🏗️ EN 1991 construction stages mutation leaves: construction stages, stage activity, stage assumed qk and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-stage-activity/🦀️.rs"]
pub mod change_stage_activity;
#[path = "../🏗️change-stage-duration-days/🦀️.rs"]
pub mod change_stage_duration_days;
#[path = "../🏗️change-stage-return-years/🦀️.rs"]
pub mod change_stage_return_years;
#[path = "../🏗️change-stage-assumed-qk/🦀️.rs"]
pub mod change_stage_assumed_qk;
#[path = "../🏗️insert-construction-stage/🦀️.rs"]
pub mod insert_construction_stage;
#[path = "../🏗️remove-construction-stage/🦀️.rs"]
pub mod remove_construction_stage;
//#endregion 🪢️Mounts
