//! ❄️ EN 1999 cold-formed sheet mutation leaves: material id, thickness, width and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../❄️change-sheet-material-id/🦀️.rs"]
pub mod change_sheet_material_id;
#[path = "../❄️change-sheet-thickness/🦀️.rs"]
pub mod change_sheet_thickness;
#[path = "../❄️change-sheet-width/🦀️.rs"]
pub mod change_sheet_width;
#[path = "../❄️change-sheet-span/🦀️.rs"]
pub mod change_sheet_span;
#[path = "../❄️change-sheet-welded/🦀️.rs"]
pub mod change_sheet_welded;
//#endregion 🪢️Mounts
