//! 🏗️ EN 1998 foundations liq mutation leaves: liq n160, liq fines percent, liq depth m.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-foundation-liq-n160/🦀️.rs"]
pub mod change_foundation_liq_n160;
#[path = "../✏️change-foundation-liq-fines-percent/🦀️.rs"]
pub mod change_foundation_liq_fines_percent;
#[path = "../✏️change-foundation-liq-depth-m/🦀️.rs"]
pub mod change_foundation_liq_depth_m;
#[path = "../✏️change-foundation-liq-water-depth-m/🦀️.rs"]
pub mod change_foundation_liq_water_depth_m;
#[path = "../✏️change-foundation-liq-unit-wt-nm3/🦀️.rs"]
pub mod change_foundation_liq_unit_wt_n_m3;
#[path = "../✏️change-foundation-liq-magnitude/🦀️.rs"]
pub mod change_foundation_liq_magnitude;
//#endregion 🪢️Mounts
