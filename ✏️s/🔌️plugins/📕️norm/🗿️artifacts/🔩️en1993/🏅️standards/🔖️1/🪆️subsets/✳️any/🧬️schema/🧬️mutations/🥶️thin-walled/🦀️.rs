//! 🥶️ EN 1993 thin walled mutation leaves.

//#region 🪢️Mounts
#[path = "../➕️insert-cold-formed-member/🦀️.rs"]
pub mod insert_cold_formed_member;
#[path = "../➕️insert-plated-panel/🦀️.rs"]
pub mod insert_plated_panel;
#[path = "../➕️insert-silo-shell/🦀️.rs"]
pub mod insert_silo_shell;
#[path = "../➖️remove-cold-formed-member/🦀️.rs"]
pub mod remove_cold_formed_member;
#[path = "../➖️remove-plated-panel/🦀️.rs"]
pub mod remove_plated_panel;
#[path = "../➖️remove-silo-shell/🦀️.rs"]
pub mod remove_silo_shell;
#[path = "../🥶️update-cold-formed-inputs/🦀️.rs"]
pub mod update_cold_formed_inputs;
#[path = "../🧱️update-plated-inputs/🦀️.rs"]
pub mod update_plated_inputs;
#[path = "../🛢️update-silo-shell-inputs/🦀️.rs"]
pub mod update_silo_shell_inputs;
//#endregion 🪢️Mounts

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
