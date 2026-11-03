//! 🧱 EN 1997 embedded wall mutation leaves: kind, retained height, embedment and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-emb-wall-kind/🦀️.rs"]
pub mod change_emb_wall_kind;
#[path = "../🧱️change-emb-wall-retained-height/🦀️.rs"]
pub mod change_emb_wall_retained_height;
#[path = "../🧱️change-emb-wall-embedment/🦀️.rs"]
pub mod change_emb_wall_embedment;
#[path = "../🧱️change-emb-wall-prop-depth/🦀️.rs"]
pub mod change_emb_wall_prop_depth;
#[path = "../🧱️change-emb-wall-thickness/🦀️.rs"]
pub mod change_emb_wall_thickness;
#[path = "../🧱️change-emb-wall-backfill-gamma/🦀️.rs"]
pub mod change_emb_wall_backfill_gamma;
#[path = "../🧱️change-emb-wall-phi-deg/🦀️.rs"]
pub mod change_emb_wall_phi_deg;
//#endregion 🪢️Mounts
