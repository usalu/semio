//! 📚️ EN 1993 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🔩️high-strength-connection/🧪️tests/🧩️example/🦀️.rs"]
mod example;
#[path = "✅️heb240-compliant/🦀️.rs"]
pub mod heb240_compliant;
#[path = "🔩️high-strength-connection/🦀️.rs"]
pub mod high_strength_connection;
//#endregion 🪢️Mounts
