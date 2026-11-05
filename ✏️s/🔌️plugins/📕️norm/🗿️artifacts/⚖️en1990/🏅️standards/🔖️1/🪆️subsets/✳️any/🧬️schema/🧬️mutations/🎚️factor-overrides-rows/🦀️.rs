//! 🎚 EN 1990 factor overrides rows mutation leaves: factor override.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🎚️insert-factor-override/🦀️.rs"]
pub mod insert_factor_override;
#[path = "../🎚️remove-factor-override/🦀️.rs"]
pub mod remove_factor_override;
//#endregion 🪢️Mounts
