//! 🔧️ EN 1993 parametric section mutation leaves: section id, label, shape and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔧️change-param-section-id/🦀️.rs"]
pub mod change_param_section_id;
#[path = "../🔧️change-param-label/🦀️.rs"]
pub mod change_param_label;
#[path = "../🔧️change-param-shape/🦀️.rs"]
pub mod change_param_shape;
#[path = "../🔧️change-param-height/🦀️.rs"]
pub mod change_param_height;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_param_section_more::{change_param_width, change_param_web_thickness, change_param_flange_thickness, change_param_tolerance};
