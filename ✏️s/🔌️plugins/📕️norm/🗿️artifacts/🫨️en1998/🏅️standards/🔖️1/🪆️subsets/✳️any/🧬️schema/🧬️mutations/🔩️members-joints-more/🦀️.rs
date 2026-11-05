//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-joint-ym/🦀️.rs"]
pub mod change_joint_y_m;
#[path = "../✏️change-joint-column-moment-rd-nm/🦀️.rs"]
pub mod change_joint_column_moment_rd_nm;
#[path = "../✏️change-joint-beam-moment-rd-nm/🦀️.rs"]
pub mod change_joint_beam_moment_rd_nm;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_members_joints_rest::{insert_joint, remove_joint, insert_tie_beam, remove_tie_beam, insert_seismic_slope, remove_seismic_slope};
