//! 🪗️ SheetProfile mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-sheet-profile/🦀️.rs"]
pub mod insert_sheet_profile;
#[path = "../➖remove-sheet-profile/🦀️.rs"]
pub mod remove_sheet_profile;
#[path = "../🦴️change-sheet-profile-sheet-id/🦀️.rs"]
pub mod change_sheet_profile_sheet_id;
#[path = "../🦴️change-sheet-profile-web-height/🦀️.rs"]
pub mod change_sheet_profile_web_height;
#[path = "../🦴️change-sheet-profile-bottom-flange/🦀️.rs"]
pub mod change_sheet_profile_bottom_flange;
#[path = "../🦴️change-sheet-profile-top-flange/🦀️.rs"]
pub mod change_sheet_profile_top_flange;
#[path = "../🦴️change-sheet-profile-pitch/🦀️.rs"]
pub mod change_sheet_profile_pitch;
//#endregion 🪢️Mounts
