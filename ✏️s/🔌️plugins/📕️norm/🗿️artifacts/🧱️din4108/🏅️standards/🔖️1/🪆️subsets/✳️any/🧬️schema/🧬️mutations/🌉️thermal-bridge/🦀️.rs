//! 🌉️ DIN 4108 thermal bridge mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏷️change-thermal-bridge-bb2-type/🦀️.rs"]
pub mod change_thermal_bridge_bb2_type;
#[path = "../↔️change-thermal-bridge-length/🦀️.rs"]
pub mod change_thermal_bridge_length;
#[path = "../🔘change-thermal-bridge-psi/🦀️.rs"]
pub mod change_thermal_bridge_psi;
#[path = "../🌉️insert-thermal-bridge/🦀️.rs"]
pub mod insert_thermal_bridge;
#[path = "../🧊remove-thermal-bridge/🦀️.rs"]
pub mod remove_thermal_bridge;
//#endregion 🪢️Mounts
