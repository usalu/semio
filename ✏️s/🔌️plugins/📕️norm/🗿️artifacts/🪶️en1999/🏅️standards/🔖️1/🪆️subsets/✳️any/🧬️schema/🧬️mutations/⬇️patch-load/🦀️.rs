//! ⬇️ EN 1999 web patch load mutation leaves: subject id, position, x and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⬇️change-patch-subject-id/🦀️.rs"]
pub mod change_patch_subject_id;
#[path = "../⬇️change-patch-position/🦀️.rs"]
pub mod change_patch_position;
#[path = "../⬇️change-patch-x/🦀️.rs"]
pub mod change_patch_x;
#[path = "../⬇️change-patch-bearing-length/🦀️.rs"]
pub mod change_patch_bearing_length;
#[path = "../⬇️change-patch-web-height/🦀️.rs"]
pub mod change_patch_web_height;
#[path = "../⬇️change-patch-web-angle/🦀️.rs"]
pub mod change_patch_web_angle;
#[path = "../⬇️change-patch-corner-radius/🦀️.rs"]
pub mod change_patch_corner_radius;
//#endregion 🪢️Mounts
