//! 🧹️ DIN EN 16798 air hygiene mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱change-vent-duct-class/🦀️.rs"]
pub mod change_vent_duct_class;
#[path = "../🕳️change-vent-duct-leakage/🦀️.rs"]
pub mod change_vent_duct_leakage;
#[path = "../🧽change-vent-filter-sup/🦀️.rs"]
pub mod change_vent_filter_sup;
#[path = "../📅️change-vent-inspection/🦀️.rs"]
pub mod change_vent_inspection;
#[path = "../🏞️change-vent-oda-class/🦀️.rs"]
pub mod change_vent_oda_class;
//#endregion 🪢️Mounts
