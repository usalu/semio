//! ❄️ EN 1999 cold-formed sheet action mutation leaves: kind, category, source and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../❄️change-sheet-action-kind/🦀️.rs"]
pub mod change_sheet_action_kind;
#[path = "../❄️change-sheet-action-category/🦀️.rs"]
pub mod change_sheet_action_category;
#[path = "../❄️change-sheet-action-source/🦀️.rs"]
pub mod change_sheet_action_source;
#[path = "../❄️change-sheet-action-gk-line/🦀️.rs"]
pub mod change_sheet_action_g_k_line;
#[path = "../❄️change-sheet-action-qk-line/🦀️.rs"]
pub mod change_sheet_action_q_k_line;
#[path = "../❄️change-sheet-action-nk/🦀️.rs"]
pub mod change_sheet_action_n_k;
#[path = "../❄️change-sheet-action-vyk/🦀️.rs"]
pub mod change_sheet_action_v_y_k;
//#endregion 🪢️Mounts
