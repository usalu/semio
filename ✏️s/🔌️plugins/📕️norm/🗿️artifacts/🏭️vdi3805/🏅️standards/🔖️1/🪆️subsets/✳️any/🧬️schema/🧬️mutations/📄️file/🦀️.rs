//! 📄️ VDI 3805 file mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📅️change-correction-as-of/🦀️.rs"]
pub mod change_correction_as_of;
#[path = "../🔖️change-edition-profile/🦀️.rs"]
pub mod change_edition_profile;
#[path = "../🚧️change-limits/🦀️.rs"]
pub mod change_limits;
#[path = "../🏭️change-manufacturer-file/🦀️.rs"]
pub mod change_manufacturer_file;
#[path = "../🔒️change-strict-mode/🦀️.rs"]
pub mod change_strict_mode;
#[path = "../🧹️remove-edition-profile/🦀️.rs"]
pub mod remove_edition_profile;
//#endregion 🪢️Mounts
