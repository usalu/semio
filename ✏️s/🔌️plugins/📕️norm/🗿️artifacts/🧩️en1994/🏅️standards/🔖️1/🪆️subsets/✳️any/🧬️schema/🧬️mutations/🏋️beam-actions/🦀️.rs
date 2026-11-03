//! 🏋️ EN 1994 beam action mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏷️change-beam-action-category/🦀️.rs"]
pub mod change_beam_action_category;
#[path = "../🔁️change-beam-action-delta-sigma-pa/🦀️.rs"]
pub mod change_beam_action_delta_sigma_pa;
#[path = "../🔁️change-beam-action-delta-tau-pa/🦀️.rs"]
pub mod change_beam_action_delta_tau_pa;
#[path = "../🏋️change-beam-action-kind/🦀️.rs"]
pub mod change_beam_action_kind;
#[path = "../📍️change-beam-action-point-force-n/🦀️.rs"]
pub mod change_beam_action_point_force_n;
#[path = "../🧭️change-beam-action-stage/🦀️.rs"]
pub mod change_beam_action_stage;
//#endregion 🪢️Mounts
