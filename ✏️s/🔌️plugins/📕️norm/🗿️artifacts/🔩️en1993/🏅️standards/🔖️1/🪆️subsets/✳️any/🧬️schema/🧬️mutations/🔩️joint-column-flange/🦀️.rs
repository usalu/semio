//! 🔩️ EN 1993 joint mutation leaves: column flange thickness, beam moment resistance, column moment resistance and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-column-flange-thickness/🦀️.rs"]
pub mod change_joint_column_flange_thickness;
#[path = "../🔩️change-joint-beam-moment-resistance/🦀️.rs"]
pub mod change_joint_beam_moment_resistance;
#[path = "../🔩️change-joint-column-moment-resistance/🦀️.rs"]
pub mod change_joint_column_moment_resistance;
#[path = "../🔩️change-joint-transformation-parameter/🦀️.rs"]
pub mod change_joint_transformation_parameter;
#[path = "../🔩️change-joint-beam-web-thickness/🦀️.rs"]
pub mod change_joint_beam_web_thickness;
#[path = "../🔩️change-joint-beam-web-fy/🦀️.rs"]
pub mod change_joint_beam_web_fy;
#[path = "../🔩️change-joint-weld-force-longitudinal/🦀️.rs"]
pub mod change_joint_weld_force_longitudinal;
#[path = "../🔩️change-joint-weld-force-transverse/🦀️.rs"]
pub mod change_joint_weld_force_transverse;
//#endregion 🪢️Mounts
