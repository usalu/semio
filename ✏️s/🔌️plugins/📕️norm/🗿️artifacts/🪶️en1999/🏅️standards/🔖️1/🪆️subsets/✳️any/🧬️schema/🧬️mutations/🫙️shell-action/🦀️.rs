//! 🫙 EN 1999 shell action mutation leaves: kind, category, source and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🫙️change-shell-action-kind/🦀️.rs"]
pub mod change_shell_action_kind;
#[path = "../🫙️change-shell-action-category/🦀️.rs"]
pub mod change_shell_action_category;
#[path = "../🫙️change-shell-action-source/🦀️.rs"]
pub mod change_shell_action_source;
#[path = "../🫙️change-shell-action-gk-line/🦀️.rs"]
pub mod change_shell_action_g_k_line;
#[path = "../🫙️change-shell-action-qk-line/🦀️.rs"]
pub mod change_shell_action_q_k_line;
#[path = "../🫙️change-shell-action-nk/🦀️.rs"]
pub mod change_shell_action_n_k;
#[path = "../🫙️change-shell-action-vyk/🦀️.rs"]
pub mod change_shell_action_v_y_k;
//#endregion 🪢️Mounts
