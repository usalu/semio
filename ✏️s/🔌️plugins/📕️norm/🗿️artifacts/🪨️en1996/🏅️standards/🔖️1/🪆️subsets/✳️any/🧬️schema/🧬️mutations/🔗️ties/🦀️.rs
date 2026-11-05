//! 🔗️ EN 1996 wall ties, anchors and ring ties mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-tie-kind/🦀️.rs"]
pub mod change_tie_kind;
#[path = "../🔗️change-tie-position/🦀️.rs"]
pub mod change_tie_position;
#[path = "../🔗️change-tie-level/🦀️.rs"]
pub mod change_tie_level;
#[path = "../🔗️change-tie-capacity/🦀️.rs"]
pub mod change_tie_capacity;
#[path = "../➕️insert-tie/🦀️.rs"]
pub mod insert_tie;
#[path = "../➖️remove-tie/🦀️.rs"]
pub mod remove_tie;
//#endregion 🪢️Mounts
