//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌡️change-thermal-section-alpha-micro/🦀️.rs"]
pub mod change_thermal_section_alpha_micro;
#[path = "../🌡️change-thermal-assumed-range/🦀️.rs"]
pub mod change_thermal_assumed_range;
#[path = "../🌡️insert-thermal-section/🦀️.rs"]
pub mod insert_thermal_section;
#[path = "../🌡️remove-thermal-section/🦀️.rs"]
pub mod remove_thermal_section;
//#endregion 🪢️Mounts
