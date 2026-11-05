//! 🪵️ EN 1993 pile mutation leaves: interlock bending factor, interlock stiffness factor.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪵️change-pile-interlock-bending-factor/🦀️.rs"]
pub mod change_pile_interlock_bending_factor;
#[path = "../🪵️change-pile-interlock-stiffness-factor/🦀️.rs"]
pub mod change_pile_interlock_stiffness_factor;
//#endregion 🪢️Mounts
