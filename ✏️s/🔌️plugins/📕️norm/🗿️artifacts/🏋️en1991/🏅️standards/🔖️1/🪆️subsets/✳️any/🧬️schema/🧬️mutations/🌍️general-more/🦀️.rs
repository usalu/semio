//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏙️change-storey-count/🦀️.rs"]
pub mod change_storey_count;
#[path = "../🌉change-structure-kind/🦀️.rs"]
pub mod change_structure_kind;
#[path = "../🏠change-width/🦀️.rs"]
pub mod change_width;
#[path = "../➕️insert-accidental-case/🦀️.rs"]
pub mod insert_accidental_case;
#[path = "../➖️remove-accidental-case/🦀️.rs"]
pub mod remove_accidental_case;
//#endregion 🪢️Mounts
