//! 🌀️ DIN EN 16798 ventilation mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌬️change-vent-design-airflow/🦀️.rs"]
pub mod change_vent_design_airflow;
#[path = "../♻️change-vent-heat-recovery/🦀️.rs"]
pub mod change_vent_heat_recovery;
#[path = "../🌀️change-vent-sfp/🦀️.rs"]
pub mod change_vent_sfp;
#[path = "../🎓️change-vent-sfp-class/🦀️.rs"]
pub mod change_vent_sfp_class;
#[path = "../⚙️change-vent-system-type/🦀️.rs"]
pub mod change_vent_system_type;
#[path = "../🆕️insert-vent-system/🦀️.rs"]
pub mod insert_vent_system;
#[path = "../🗑️remove-vent-system/🦀️.rs"]
pub mod remove_vent_system;
//#endregion 🪢️Mounts
