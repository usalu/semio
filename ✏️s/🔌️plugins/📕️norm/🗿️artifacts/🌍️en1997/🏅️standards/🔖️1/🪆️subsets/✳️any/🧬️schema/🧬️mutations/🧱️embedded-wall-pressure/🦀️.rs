//! 🧱 EN 1997 embedded wall mutation leaves: delta active deg, delta passive deg, surcharge and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-emb-wall-delta-active-deg/🦀️.rs"]
pub mod change_emb_wall_delta_active_deg;
#[path = "../🧱️change-emb-wall-delta-passive-deg/🦀️.rs"]
pub mod change_emb_wall_delta_passive_deg;
#[path = "../🧱️change-emb-wall-surcharge/🦀️.rs"]
pub mod change_emb_wall_surcharge;
#[path = "../🧱️change-emb-wall-undr-strength/🦀️.rs"]
pub mod change_emb_wall_undr_strength;
//#endregion 🪢️Mounts
