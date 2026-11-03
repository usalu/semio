//! 📏️ EN 1993 member mutation leaves: web material id, torsion load, torque position.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-member-web-material-id/🦀️.rs"]
pub mod change_member_web_material_id;
#[path = "../📏️change-member-torsion-load/🦀️.rs"]
pub mod change_member_torsion_load;
#[path = "../📏️change-member-torque-position/🦀️.rs"]
pub mod change_member_torque_position;
//#endregion 🪢️Mounts
