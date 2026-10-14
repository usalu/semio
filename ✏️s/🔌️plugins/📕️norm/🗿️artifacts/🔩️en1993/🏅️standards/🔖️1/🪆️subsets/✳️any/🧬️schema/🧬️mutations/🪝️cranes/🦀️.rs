//! 🪝️ EN 1993 cranes mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-crane-runway/🦀️.rs"]
pub mod insert_crane_runway;
#[path = "../➕️insert-tension-component/🦀️.rs"]
pub mod insert_tension_component;
#[path = "../➖️remove-crane-runway/🦀️.rs"]
pub mod remove_crane_runway;
#[path = "../➖️remove-tension-component/🦀️.rs"]
pub mod remove_tension_component;
#[path = "../🏗️update-crane-inputs/🦀️.rs"]
pub mod update_crane_inputs;
#[path = "../🪢️update-tension-component-inputs/🦀️.rs"]
pub mod update_tension_component_inputs;
//#endregion 🪢️Mounts
