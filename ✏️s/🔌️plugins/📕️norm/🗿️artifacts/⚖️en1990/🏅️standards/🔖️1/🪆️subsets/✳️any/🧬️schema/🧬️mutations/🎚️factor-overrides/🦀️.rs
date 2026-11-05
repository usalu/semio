//! 🎚 EN 1990 factor overrides mutation leaves: factor override factor, factor override justif, factor override target and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🎚️change-factor-override-factor/🦀️.rs"]
pub mod change_factor_override_factor;
#[path = "../🎚️change-factor-override-target/🦀️.rs"]
pub mod change_factor_override_target;
#[path = "../🎚️change-factor-override-value/🦀️.rs"]
pub mod change_factor_override_value;
#[path = "../🎚️change-factor-override-justif/🦀️.rs"]
pub mod change_factor_override_justif;
//#endregion 🪢️Mounts
