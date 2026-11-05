//! 🔗️ EN 1999 connection action mutation leaves: kind, category, source and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-conn-action-kind/🦀️.rs"]
pub mod change_conn_action_kind;
#[path = "../🔗️change-conn-action-category/🦀️.rs"]
pub mod change_conn_action_category;
#[path = "../🔗️change-conn-action-source/🦀️.rs"]
pub mod change_conn_action_source;
#[path = "../🔗️change-conn-action-gk-line/🦀️.rs"]
pub mod change_conn_action_g_k_line;
#[path = "../🔗️change-conn-action-qk-line/🦀️.rs"]
pub mod change_conn_action_q_k_line;
#[path = "../🔗️change-conn-action-nk/🦀️.rs"]
pub mod change_conn_action_n_k;
#[path = "../🔗️change-conn-action-vyk/🦀️.rs"]
pub mod change_conn_action_v_y_k;
//#endregion 🪢️Mounts
