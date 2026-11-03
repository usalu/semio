//! 📏️ EN 1993 member mutation leaves: ltb restraint spacing, deflection limit ratio, toughness stress ratio.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-member-ltb-restraint-spacing/🦀️.rs"]
pub mod change_member_ltb_restraint_spacing;
#[path = "../📏️change-member-deflection-limit-ratio/🦀️.rs"]
pub mod change_member_deflection_limit_ratio;
#[path = "../📏️change-member-toughness-stress-ratio/🦀️.rs"]
pub mod change_member_toughness_stress_ratio;
//#endregion 🪢️Mounts
