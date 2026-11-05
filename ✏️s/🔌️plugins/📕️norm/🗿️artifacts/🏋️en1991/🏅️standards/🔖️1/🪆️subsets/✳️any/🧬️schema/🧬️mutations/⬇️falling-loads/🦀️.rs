//! ⬇ EN 1991 falling loads mutation leaves: falling, falling assumed force, falling drop height and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⬇️change-falling-mass/🦀️.rs"]
pub mod change_falling_mass;
#[path = "../⬇️change-falling-drop-height/🦀️.rs"]
pub mod change_falling_drop_height;
#[path = "../⬇️change-falling-stopping-distance/🦀️.rs"]
pub mod change_falling_stopping_distance;
#[path = "../⬇️change-falling-x/🦀️.rs"]
pub mod change_falling_x;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1991_mut_falling_loads_more::{change_falling_y, change_falling_assumed_force, insert_falling, remove_falling};
