//! 🗼️ EN 1993 tower leg mutation leaves: kw factor, lateral force coefficient, permitted amplitude ratio and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🗼️change-tower-kw-factor/🦀️.rs"]
pub mod change_tower_kw_factor;
#[path = "../🗼️change-tower-lateral-force-coef/🦀️.rs"]
pub mod change_tower_lateral_force_coef;
#[path = "../🗼️change-tower-perm-amplitude-ratio/🦀️.rs"]
pub mod change_tower_perm_amplitude_ratio;
#[path = "../🗼️change-tower-ring-mode/🦀️.rs"]
pub mod change_tower_ring_mode;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_tower_kw_more::{change_tower_shell_thickness, change_tower_guy_area, change_tower_guy_angle, change_tower_guy_length};
