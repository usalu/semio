//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-pier-modulus-mpa/🦀️.rs"]
pub mod change_pier_modulus_mpa;
#[path = "../✏️change-pier-moment-rd-nm/🦀️.rs"]
pub mod change_pier_moment_rd_nm;
#[path = "../✏️change-pier-shear-rd-n/🦀️.rs"]
pub mod change_pier_shear_rd_n;
#[path = "../✏️change-pier-reinforcement-ratio/🦀️.rs"]
pub mod change_pier_reinforcement_ratio;
#[path = "../✏️change-pier-cover-m/🦀️.rs"]
pub mod change_pier_cover_m;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_bridges_piers_rest::{change_pier_concrete_fck_mpa, change_pier_steel_fyk_mpa, change_pier_axial_load_n, insert_pier, remove_pier};
