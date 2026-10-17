//! 🏢️ EN 1998 buildings mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📎️change-annex/🦀️.rs"]
pub mod change_annex;
#[path = "../🧭️change-building-plan-regular/🦀️.rs"]
pub mod change_building_plan_regular;
#[path = "../📏️change-storey-drift-xm/🦀️.rs"]
pub mod change_storey_drift_xm;
#[path = "../📐️change-storey-stiffness-x/🦀️.rs"]
pub mod change_storey_stiffness_x;
#[path = "../🌚️update-site/🦀️.rs"]
pub mod update_site;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_buildings_2::{change_elevation_regular, change_storey_permanent_gk_n, change_system_v_rd_n, insert_building, remove_building};
