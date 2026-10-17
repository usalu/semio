//! 🧰️ EN 1991 silos mutation leaves (part 2).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌀change-assumed-silo-pressure/🦀️.rs"]
pub mod change_assumed_silo_pressure;
#[path = "../🌾change-silo-bulk-density/🦀️.rs"]
pub mod change_silo_bulk_density;
#[path = "../🏷️change-silo-height/🦀️.rs"]
pub mod change_silo_height;
#[path = "../⚙️change-silo-k/🦀️.rs"]
pub mod change_silo_k;
#[path = "../🔎change-silo-mu/🦀️.rs"]
pub mod change_silo_mu;
//#endregion 🪢️Mounts
