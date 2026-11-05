//! 🧱️ EN 1998 assess steel mutation leaves: tension steel area m2, compression steel area m2, bar diameter mm.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-assessment-as1-area-m2/🦀️.rs"]
pub mod change_assessment_as1_area_m2;
#[path = "../✏️change-assessment-as2-area-m2/🦀️.rs"]
pub mod change_assessment_as2_area_m2;
#[path = "../✏️change-assessment-bar-diameter-mm/🦀️.rs"]
pub mod change_assessment_bar_diameter_mm;
#[path = "../✏️change-assessment-steel-yield-mpa/🦀️.rs"]
pub mod change_assessment_steel_yield_mpa;
#[path = "../✏️change-assessment-fc-mpa/🦀️.rs"]
pub mod change_assessment_fc_mpa;
//#endregion 🪢️Mounts
