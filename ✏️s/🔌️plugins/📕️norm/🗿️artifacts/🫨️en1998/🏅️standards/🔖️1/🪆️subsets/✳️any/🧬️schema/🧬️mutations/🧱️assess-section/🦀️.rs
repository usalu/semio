//! 🧱️ EN 1998 assess section mutation leaves: section depth m, section width m, effective depth m.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-assessment-section-depth-m/🦀️.rs"]
pub mod change_assessment_section_depth_m;
#[path = "../✏️change-assessment-section-width-m/🦀️.rs"]
pub mod change_assessment_section_width_m;
#[path = "../✏️change-assessment-eff-depth-m/🦀️.rs"]
pub mod change_assessment_eff_depth_m;
#[path = "../✏️change-assessment-shear-span-m/🦀️.rs"]
pub mod change_assessment_shear_span_m;
#[path = "../✏️change-assessment-axial-force-n/🦀️.rs"]
pub mod change_assessment_axial_force_n;
#[path = "../✏️change-assessment-diag-reinf-ratio/🦀️.rs"]
pub mod change_assessment_diag_reinf_ratio;
#[path = "../✏️change-assessment-shear-crack/🦀️.rs"]
pub mod change_assessment_shear_crack;
#[path = "../✏️change-assessment-shear-force-demand-n/🦀️.rs"]
pub mod change_assessment_shear_force_demand_n;
//#endregion 🪢️Mounts
