//! 🏗️ EN 1998 foundations bearing mutation leaves: liq layer thickness m, footing width m, bearing axial n.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-foundation-liq-layer-m/🦀️.rs"]
pub mod change_foundation_liq_layer_m;
#[path = "../✏️change-foundation-footing-width-m/🦀️.rs"]
pub mod change_foundation_footing_width_m;
#[path = "../✏️change-foundation-bear-axial-n/🦀️.rs"]
pub mod change_foundation_bear_axial_n;
#[path = "../✏️change-foundation-bear-shear-n/🦀️.rs"]
pub mod change_foundation_bear_shear_n;
#[path = "../✏️change-foundation-bear-moment-nm/🦀️.rs"]
pub mod change_foundation_bear_moment_nm;
#[path = "../✏️change-foundation-bearing-n-max-n/🦀️.rs"]
pub mod change_foundation_bearing_n_max_n;
#[path = "../✏️change-foundation-soil-phi-deg/🦀️.rs"]
pub mod change_foundation_soil_phi_deg;
//#endregion 🪢️Mounts
