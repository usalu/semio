//! 🌍️ EN 1997 design mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌍️change-annex/🦀️.rs"]
pub mod change_annex;
#[path = "../🧭️change-design-approach/🦀️.rs"]
pub mod change_design_approach;
#[path = "../📅️change-design-situation/🦀️.rs"]
pub mod change_design_situation;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1997_mut_design_more::{change_geotechnical_category, change_groundwater_level, change_investigation_depth, change_structure_id};
