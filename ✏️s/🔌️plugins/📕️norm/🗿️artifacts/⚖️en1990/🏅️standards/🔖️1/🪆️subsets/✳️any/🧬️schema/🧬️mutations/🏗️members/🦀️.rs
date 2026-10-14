//! 🏗️ EN 1990 members mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉change-bridge-sls/🦀️.rs"]
pub mod change_bridge_sls;
#[path = "../🔗change-effects/🦀️.rs"]
pub mod change_effects;
#[path = "../🏗️change-members/🦀️.rs"]
pub mod change_members;
#[path = "../📎insert-effect/🦀️.rs"]
pub mod insert_effect;
#[path = "../🔩insert-member/🦀️.rs"]
pub mod insert_member;
#[path = "../✂️remove-effect/🦀️.rs"]
pub mod remove_effect;
#[path = "../🪚remove-member/🦀️.rs"]
pub mod remove_member;
//#endregion 🪢️Mounts
