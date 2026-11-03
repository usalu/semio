//! 🏋️ EN 1999 member action mutation leaves: m z k, torsion k, local load k.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️change-action-mzk/🦀️.rs"]
pub mod change_action_m_z_k;
#[path = "../🏋️change-action-torsion-k/🦀️.rs"]
pub mod change_action_torsion_k;
#[path = "../🏋️change-action-local-load-k/🦀️.rs"]
pub mod change_action_local_load_k;
//#endregion 🪢️Mounts
