//! 🎻 EN 1996 prestressing tendons mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🎻change-prestress-force/🦀️.rs"]
pub mod change_prestress_force;
#[path = "../🎻change-prestress-area/🦀️.rs"]
pub mod change_prestress_area;
#[path = "../🎻change-prestress-strength/🦀️.rs"]
pub mod change_prestress_strength;
#[path = "../🎻change-prestress-position/🦀️.rs"]
pub mod change_prestress_position;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1996_mut_prestress_more::{change_prestress_eccentricity, change_prestress_loss, insert_prestress, remove_prestress};
