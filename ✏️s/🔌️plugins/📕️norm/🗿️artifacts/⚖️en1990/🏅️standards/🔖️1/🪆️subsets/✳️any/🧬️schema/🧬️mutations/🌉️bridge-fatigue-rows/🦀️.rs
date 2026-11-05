//! 🌉 EN 1990 bridge fatigue rows mutation leaves: bridge sls, fatigue spectrum and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️insert-bridge-sls/🦀️.rs"]
pub mod insert_bridge_sls;
#[path = "../🌉️remove-bridge-sls/🦀️.rs"]
pub mod remove_bridge_sls;
#[path = "../🔄️insert-fatigue-spectrum/🦀️.rs"]
pub mod insert_fatigue_spectrum;
#[path = "../🔄️remove-fatigue-spectrum/🦀️.rs"]
pub mod remove_fatigue_spectrum;
//#endregion 🪢️Mounts

#[cfg(test)]
#[path = "🧪️tests/🛬️native/🦀️.rs"]
mod native_receiving_tests;
