//! 🔩️ EN 1993 connections mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-joint/🦀️.rs"]
pub mod insert_joint;
#[path = "../➕️insert-load-case/🦀️.rs"]
pub mod insert_load_case;
#[path = "../➕️insert-member-action/🦀️.rs"]
pub mod insert_member_action;
#[path = "../➖️remove-joint/🦀️.rs"]
pub mod remove_joint;
#[path = "../➖️remove-load-case/🦀️.rs"]
pub mod remove_load_case;
#[path = "../➖️remove-member-action/🦀️.rs"]
pub mod remove_member_action;
#[path = "../🔩️update-bolt-inputs/🦀️.rs"]
pub mod update_bolt_inputs;
#[path = "../⬜️update-hss-inputs/🦀️.rs"]
pub mod update_hss_inputs;
#[path = "../🧲️update-weld-inputs/🦀️.rs"]
pub mod update_weld_inputs;
//#endregion 🪢️Mounts
