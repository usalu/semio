//! 🏗 EN 1991 construction loads mutation leaves: construction load, construction load kind, construction load magnitude and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-construction-load-stage/🦀️.rs"]
pub mod change_construction_load_stage;
#[path = "../🏗️change-construction-load-kind/🦀️.rs"]
pub mod change_construction_load_kind;
#[path = "../🏗️change-construction-load-magnitude/🦀️.rs"]
pub mod change_construction_load_magnitude;
#[path = "../🏗️insert-construction-load/🦀️.rs"]
pub mod insert_construction_load;
#[path = "../🏗️remove-construction-load/🦀️.rs"]
pub mod remove_construction_load;
//#endregion 🪢️Mounts
