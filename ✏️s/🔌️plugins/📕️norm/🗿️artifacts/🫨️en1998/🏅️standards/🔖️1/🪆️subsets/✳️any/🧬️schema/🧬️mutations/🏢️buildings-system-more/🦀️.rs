//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-system-q0/🦀️.rs"]
pub mod change_system_q0;
#[path = "../✏️change-system-alpha-u-over-alpha1/🦀️.rs"]
pub mod change_system_alpha_u_over_alpha_1;
#[path = "../✏️change-system-kw/🦀️.rs"]
pub mod change_system_k_w;
#[path = "../➕️insert-system/🦀️.rs"]
pub mod insert_system;
#[path = "../➖️remove-system/🦀️.rs"]
pub mod remove_system;
//#endregion 🪢️Mounts
