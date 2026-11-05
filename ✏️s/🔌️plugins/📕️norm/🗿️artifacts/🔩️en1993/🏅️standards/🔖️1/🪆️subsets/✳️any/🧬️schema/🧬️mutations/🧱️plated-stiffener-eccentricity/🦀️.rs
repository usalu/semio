//! 🧱️ EN 1993 plated panel mutation leaves: stiffener eccentricity, patch moment resistance, transverse stiffener inertia and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-stiffener-eccentricity/🦀️.rs"]
pub mod change_plated_stiffener_eccentricity;
#[path = "../🧱️change-plated-patch-moment-resistance/🦀️.rs"]
pub mod change_plated_patch_moment_resistance;
#[path = "../🧱️change-plated-corrugation-projection/🦀️.rs"]
pub mod change_plated_corrugation_projection;
#[path = "../🧱️change-plated-stiffener-flange-width/🦀️.rs"]
pub mod change_plated_stiffener_flange_width;
#[path = "../🧱️change-panel-design-temperature/🦀️.rs"]
pub mod change_panel_design_temperature;
//#endregion 🪢️Mounts
