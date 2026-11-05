//! 🧱️ EN 1998 diaphragms core mutation leaves: building id, storey id, material.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-diaphragm-building-id/🦀️.rs"]
pub mod change_diaphragm_building_id;
#[path = "../✏️change-diaphragm-storey-id/🦀️.rs"]
pub mod change_diaphragm_storey_id;
#[path = "../✏️change-diaphragm-material/🦀️.rs"]
pub mod change_diaphragm_material;
#[path = "../✏️change-diaphragm-thickness-m/🦀️.rs"]
pub mod change_diaphragm_thickness_m;
#[path = "../➕️insert-diaphragm/🦀️.rs"]
pub mod insert_diaphragm;
#[path = "../➖️remove-diaphragm/🦀️.rs"]
pub mod remove_diaphragm;
//#endregion 🪢️Mounts
