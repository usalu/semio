//! 🏢️ EN 1998 buildings mutation leaves: system.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-system-direction/🦀️.rs"]
pub mod change_system_direction;
#[path = "../✏️change-system-system-type/🦀️.rs"]
pub mod change_system_system_type;
#[path = "../✏️change-system-material/🦀️.rs"]
pub mod change_system_material;
#[path = "../✏️change-system-ductility-class/🦀️.rs"]
pub mod change_system_ductility_class;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_buildings_system_more::{change_system_q0, change_system_alpha_u_over_alpha_1, change_system_k_w, insert_system, remove_system};
