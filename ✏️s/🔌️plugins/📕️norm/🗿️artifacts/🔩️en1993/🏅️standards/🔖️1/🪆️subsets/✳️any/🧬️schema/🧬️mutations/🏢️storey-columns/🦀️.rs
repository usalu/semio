//! 🏢️ EN 1993 storey mutation leaves: columns, plastic analysis, building height and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏢️change-storey-columns/🦀️.rs"]
pub mod change_storey_columns;
#[path = "../🏢️change-storey-plastic-analysis/🦀️.rs"]
pub mod change_storey_plastic_analysis;
#[path = "../🏢️change-storey-building-height/🦀️.rs"]
pub mod change_storey_building_height;
#[path = "../🏢️change-storey-bracing-load/🦀️.rs"]
pub mod change_storey_bracing_load;
#[path = "../🏢️change-storey-bracing-length/🦀️.rs"]
pub mod change_storey_bracing_length;
#[path = "../🏢️change-storey-bracing-deflection/🦀️.rs"]
pub mod change_storey_bracing_deflection;
//#endregion 🪢️Mounts
