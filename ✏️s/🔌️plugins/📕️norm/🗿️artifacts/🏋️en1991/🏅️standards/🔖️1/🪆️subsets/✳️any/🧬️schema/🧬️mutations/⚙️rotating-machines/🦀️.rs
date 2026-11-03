//! ⚙ EN 1991 rotating machines mutation leaves: rotating machine, rotating machine eccentricity, rotating machine frequency and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️change-rotating-machine-mass/🦀️.rs"]
pub mod change_rotating_machine_mass;
#[path = "../⚙️change-rotating-machine-eccentricity/🦀️.rs"]
pub mod change_rotating_machine_eccentricity;
#[path = "../⚙️change-rotating-machine-frequency/🦀️.rs"]
pub mod change_rotating_machine_frequency;
#[path = "../⚙️insert-rotating-machine/🦀️.rs"]
pub mod insert_rotating_machine;
#[path = "../⚙️remove-rotating-machine/🦀️.rs"]
pub mod remove_rotating_machine;
//#endregion 🪢️Mounts
