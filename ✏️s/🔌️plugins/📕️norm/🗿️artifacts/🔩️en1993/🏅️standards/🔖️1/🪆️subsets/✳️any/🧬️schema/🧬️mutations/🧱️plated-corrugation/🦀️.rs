//! 🧱️ EN 1993 plated panel mutation leaves: corrugation flat, corrugation projection, corrugation depth and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-corrugation-flat/🦀️.rs"]
pub mod change_plated_corrugation_flat;
#[path = "../🧱️change-plated-corr-projection/🦀️.rs"]
pub mod change_plated_corr_projection;
#[path = "../🧱️change-plated-corrugation-depth/🦀️.rs"]
pub mod change_plated_corrugation_depth;
#[path = "../🧱️change-plated-web-height/🦀️.rs"]
pub mod change_plated_web_height;
//#endregion 🪢️Mounts
