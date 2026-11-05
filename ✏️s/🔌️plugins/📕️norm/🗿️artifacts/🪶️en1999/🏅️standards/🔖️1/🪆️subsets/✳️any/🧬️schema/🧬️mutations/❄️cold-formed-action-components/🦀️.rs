//! ❄️ EN 1999 cold-formed sheet action mutation leaves: v z k, m y k, m z k and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../❄️change-sheet-action-vzk/🦀️.rs"]
pub mod change_sheet_action_v_z_k;
#[path = "../❄️change-sheet-action-myk/🦀️.rs"]
pub mod change_sheet_action_m_y_k;
#[path = "../❄️change-sheet-action-mzk/🦀️.rs"]
pub mod change_sheet_action_m_z_k;
#[path = "../❄️change-sheet-action-torsion-k/🦀️.rs"]
pub mod change_sheet_action_torsion_k;
#[path = "../❄️change-sheet-action-local-load-k/🦀️.rs"]
pub mod change_sheet_action_local_load_k;
//#endregion 🪢️Mounts
