//! 📏️ EN 1993 member mutation leaves: design temperature, toughness stress ratio, radiation shift and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-member-design-temperature/🦀️.rs"]
pub mod change_member_design_temperature;
#[path = "../📏️change-member-tough-stress-ratio/🦀️.rs"]
pub mod change_member_tough_stress_ratio;
#[path = "../📏️change-member-radiation-shift/🦀️.rs"]
pub mod change_member_radiation_shift;
#[path = "../📏️change-member-stress-shift/🦀️.rs"]
pub mod change_member_stress_shift;
#[path = "../📏️change-member-margin-shift/🦀️.rs"]
pub mod change_member_margin_shift;
#[path = "../📏️change-member-strain-rate-shift/🦀️.rs"]
pub mod change_member_strain_rate_shift;
#[path = "../📏️change-member-cold-forming-radius/🦀️.rs"]
pub mod change_member_cold_forming_radius;
//#endregion 🪢️Mounts
