//! 🔗️ EN 1999 connection action mutation leaves: v z k, m y k, m z k and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-conn-action-vzk/🦀️.rs"]
pub mod change_conn_action_v_z_k;
#[path = "../🔗️change-conn-action-myk/🦀️.rs"]
pub mod change_conn_action_m_y_k;
#[path = "../🔗️change-conn-action-mzk/🦀️.rs"]
pub mod change_conn_action_m_z_k;
#[path = "../🔗️change-conn-action-torsion-k/🦀️.rs"]
pub mod change_conn_action_torsion_k;
#[path = "../🔗️change-conn-action-local-load-k/🦀️.rs"]
pub mod change_conn_action_local_load_k;
//#endregion 🪢️Mounts
