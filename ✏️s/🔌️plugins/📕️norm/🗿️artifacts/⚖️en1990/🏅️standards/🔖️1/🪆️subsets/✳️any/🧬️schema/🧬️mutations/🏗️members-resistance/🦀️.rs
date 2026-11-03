//! 🏗️ EN 1990 members resistance mutation leaves: member rd equ destab, member rd equ stab, member rd fat and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-rd-str/🦀️.rs"]
pub mod change_member_rd_str;
#[path = "../🏗️change-member-rd-geo/🦀️.rs"]
pub mod change_member_rd_geo;
#[path = "../🏗️change-member-rd-equ-stab/🦀️.rs"]
pub mod change_member_rd_equ_stab;
#[path = "../🏗️change-member-rd-equ-destab/🦀️.rs"]
pub mod change_member_rd_equ_destab;
#[path = "../🏗️change-member-rd-fat/🦀️.rs"]
pub mod change_member_rd_fat;
//#endregion 🪢️Mounts
