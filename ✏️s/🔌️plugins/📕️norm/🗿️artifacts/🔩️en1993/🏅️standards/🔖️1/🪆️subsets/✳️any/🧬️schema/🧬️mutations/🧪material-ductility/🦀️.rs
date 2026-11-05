//! 🧪 MaterialDuctility mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-material-ductility-material-id/🦀️.rs"]
pub mod change_material_ductility_material_id;
#[path = "../🧱️change-material-ductility-ultimate-strain/🦀️.rs"]
pub mod change_material_ductility_ultimate_strain;
#[path = "../🧱️change-material-ductility-elongation/🦀️.rs"]
pub mod change_material_ductility_elongation;
#[path = "../➕️insert-material-ductility/🦀️.rs"]
pub mod insert_material_ductility;
#[path = "../➖️remove-material-ductility/🦀️.rs"]
pub mod remove_material_ductility;
//#endregion 🪢️Mounts
