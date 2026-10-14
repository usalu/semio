//! 🎯️ EN 1990 reliability mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐change-beta-computed/🦀️.rs"]
pub mod change_beta_computed;
#[path = "../⚠️change-consequence-class/🦀️.rs"]
pub mod change_consequence_class;
#[path = "../🔍change-inspection-level/🦀️.rs"]
pub mod change_inspection_level;
#[path = "../🎯change-reliability-class/🦀️.rs"]
pub mod change_reliability_class;
#[path = "../👁️change-supervision-level/🦀️.rs"]
pub mod change_supervision_level;
//#endregion 🪢️Mounts
