//! 🧱️ EN 1993 plated panel mutation leaves: patch moment, patch moment resistance, bending moment and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-patch-moment/🦀️.rs"]
pub mod change_plated_patch_moment;
#[path = "../🧱️change-plated-patch-moment-res/🦀️.rs"]
pub mod change_plated_patch_moment_res;
#[path = "../🧱️change-plated-bending-moment/🦀️.rs"]
pub mod change_plated_bending_moment;
#[path = "../🧱️change-plated-plastic-moment/🦀️.rs"]
pub mod change_plated_plastic_moment;
#[path = "../🧱️change-plated-flange-moment/🦀️.rs"]
pub mod change_plated_flange_moment;
#[path = "../🧱️change-plated-shear-force/🦀️.rs"]
pub mod change_plated_shear_force;
#[path = "../🧱️change-plated-trans-stiff-inert/🦀️.rs"]
pub mod change_plated_trans_stiff_inert;
//#endregion 🪢️Mounts
