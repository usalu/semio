//! 🏗️ EN 1998 foundations mutation leaves: retaining-wall.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️insert-retaining-wall/🦀️.rs"]
pub mod insert_retaining_wall;
#[path = "../➖️remove-retaining-wall/🦀️.rs"]
pub mod remove_retaining_wall;
#[path = "../✏️change-retaining-wall-height-m/🦀️.rs"]
pub mod change_retaining_wall_height_m;
#[path = "../✏️change-retaining-wall-phi-deg/🦀️.rs"]
pub mod change_retaining_wall_phi_deg;
#[path = "../✏️change-retaining-wall-soil-gamma/🦀️.rs"]
pub mod change_retaining_wall_soil_gamma;
#[path = "../✏️change-retaining-wall-r/🦀️.rs"]
pub mod change_retaining_wall_r;
#[path = "../✏️change-retaining-wall-h-rd-n-per-m/🦀️.rs"]
pub mod change_retaining_wall_h_rd_n_per_m;
//#endregion 🪢️Mounts
