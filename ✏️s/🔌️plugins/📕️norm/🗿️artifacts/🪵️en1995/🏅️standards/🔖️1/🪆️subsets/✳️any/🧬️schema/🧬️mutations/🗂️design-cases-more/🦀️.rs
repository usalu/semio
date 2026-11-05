//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-case/🦀️.rs"]
pub mod insert_case;
#[path = "../➖️remove-case/🦀️.rs"]
pub mod remove_case;
#[path = "../🔢️change-case-number/🦀️.rs"]
pub mod change_case_number;
#[path = "../🔤️change-case-text/🦀️.rs"]
pub mod change_case_text;
//#endregion 🪢️Mounts
