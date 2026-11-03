//! 🥫 FormingGain mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-forming-gain-forming/🦀️.rs"]
pub mod change_forming_gain_forming;
#[path = "../➕️insert-forming-gain/🦀️.rs"]
pub mod insert_forming_gain;
#[path = "../➖️remove-forming-gain/🦀️.rs"]
pub mod remove_forming_gain;
#[path = "../🧱️change-forming-gain-bends/🦀️.rs"]
pub mod change_forming_gain_bends;
#[path = "../🧱️change-forming-gain-member-id/🦀️.rs"]
pub mod change_forming_gain_member_id;
#[path = "../🧱️change-forming-gain-gross-area/🦀️.rs"]
pub mod change_forming_gain_gross_area;
#[path = "../🧱️change-forming-gain-ultimate-strength/🦀️.rs"]
pub mod change_forming_gain_ultimate_strength;
#[path = "../🧱️change-forming-gain-used-yield/🦀️.rs"]
pub mod change_forming_gain_used_yield;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_member_warping_more::{change_frame_member_basis_member_id, change_frame_action_end_ratio, change_frame_action_start_ratio, change_frame_action_fz, change_frame_action_fx};
