//! 🏢️ EN 1998 buildings envelope mutation leaves: v s30 m s, setback ratio, plan envelope gap ratio.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-site-vs30-ms/🦀️.rs"]
pub mod change_site_v_s30_m_s;
#[path = "../✏️change-building-setback-ratio/🦀️.rs"]
pub mod change_building_setback_ratio;
#[path = "../✏️change-building-plan-env-gap-ratio/🦀️.rs"]
pub mod change_building_plan_env_gap_ratio;
#[path = "../✏️change-building-joint-gap-m/🦀️.rs"]
pub mod change_building_joint_gap_m;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_buildings_envelope_more::{change_building_col_moment_sum_nm, change_building_beam_moment_sum_nm, change_building_storey_stiffness_ratio, change_building_storey_stiffness_basis};
