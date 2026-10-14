//! 🔩️ EN 1995 connections mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-connection-fastener-type/🦀️.rs"]
pub mod change_connection_fastener_type;
#[path = "../🏷️change-connection-label-de/🦀️.rs"]
pub mod change_connection_label_de;
#[path = "../🏷️change-connection-label-en/🦀️.rs"]
pub mod change_connection_label_en;
#[path = "../🌧️change-connection-service-class/🦀️.rs"]
pub mod change_connection_service_class;
#[path = "../🛡️change-connection-strength-class/🦀️.rs"]
pub mod change_connection_strength_class;
#[path = "../➕️insert-connection/🦀️.rs"]
pub mod insert_connection;
#[path = "../➖️remove-connection/🦀️.rs"]
pub mod remove_connection;
//#endregion 🪢️Mounts
