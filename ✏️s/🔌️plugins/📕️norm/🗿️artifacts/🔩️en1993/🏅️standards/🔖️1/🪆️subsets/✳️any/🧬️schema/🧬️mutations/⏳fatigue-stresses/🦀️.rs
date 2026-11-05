//! ⏳ FatigueStress mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-fatigue-stress-relief/🦀️.rs"]
pub mod change_fatigue_stress_relief;
#[path = "../🧱️change-fatigue-stress-min-case-id/🦀️.rs"]
pub mod change_fatigue_stress_min_case_id;
#[path = "../🧱️change-fatigue-stress-max-case-id/🦀️.rs"]
pub mod change_fatigue_stress_max_case_id;
#[path = "../🧱️change-fatigue-stress-detail-id/🦀️.rs"]
pub mod change_fatigue_stress_detail_id;
#[path = "../🧱️change-fatigue-stress-cycles/🦀️.rs"]
pub mod change_fatigue_stress_cycles;
#[path = "../🧱️change-fatigue-stress-thickness/🦀️.rs"]
pub mod change_fatigue_stress_thickness;
#[path = "../🧱️change-fatigue-stress-concentration/🦀️.rs"]
pub mod change_fatigue_stress_concentration;
#[path = "../🧱️change-fatigue-stress-fibre-y/🦀️.rs"]
pub mod change_fatigue_stress_fibre_y;
#[path = "../🧱️change-fatigue-stress-fibre-z/🦀️.rs"]
pub mod change_fatigue_stress_fibre_z;
#[path = "../➕️insert-fatigue-stress/🦀️.rs"]
pub mod insert_fatigue_stress;
#[path = "../➖️remove-fatigue-stress/🦀️.rs"]
pub mod remove_fatigue_stress;
//#endregion 🪢️Mounts
