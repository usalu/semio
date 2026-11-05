//! 🧷️ EN 1993 restraint mutation leaves: intermediate position, lateral spring position, lateral spring stiffness and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧷️change-restraint-intermediate-position/🦀️.rs"]
pub mod change_restraint_intermediate_position;
#[path = "../🧷️change-restraint-lat-spring-position/🦀️.rs"]
pub mod change_restraint_lat_spring_position;
#[path = "../🧷️change-restraint-lat-spring-stiffness/🦀️.rs"]
pub mod change_restraint_lat_spring_stiffness;
#[path = "../🧷️change-restraint-hinge-to-restraint/🦀️.rs"]
pub mod change_restraint_hinge_to_restraint;
//#endregion 🪢️Mounts
