//! 🧱️ EN 1998 members wall mutation leaves: fastener diameter mm, timber thickness mm, wall thickness m.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-member-fastener-diameter-mm/🦀️.rs"]
pub mod change_member_fastener_diameter_mm;
#[path = "../✏️change-member-timber-thickness-mm/🦀️.rs"]
pub mod change_member_timber_thickness_mm;
#[path = "../✏️change-member-wall-thickness-m/🦀️.rs"]
pub mod change_member_wall_thickness_m;
#[path = "../✏️change-member-wall-eff-height-m/🦀️.rs"]
pub mod change_member_wall_eff_height_m;
#[path = "../✏️change-assessment-chord-demand-rad/🦀️.rs"]
pub mod change_assessment_chord_demand_rad;
#[path = "../✏️change-assessment-chord-yield-rad/🦀️.rs"]
pub mod change_assessment_chord_yield_rad;
#[path = "../✏️change-assessment-chord-ult-rad/🦀️.rs"]
pub mod change_assessment_chord_ult_rad;
//#endregion 🪢️Mounts
