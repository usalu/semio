//! 🕳️ EN 1993 hollow joint mutation leaves: member id, label, chord width and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🕳️change-hollow-member-id/🦀️.rs"]
pub mod change_hollow_member_id;
#[path = "../🕳️change-hollow-label/🦀️.rs"]
pub mod change_hollow_label;
#[path = "../🕳️change-hollow-chord-width/🦀️.rs"]
pub mod change_hollow_chord_width;
#[path = "../🕳️change-hollow-chord-height/🦀️.rs"]
pub mod change_hollow_chord_height;
#[path = "../🕳️change-hollow-chord-thickness/🦀️.rs"]
pub mod change_hollow_chord_thickness;
#[path = "../🕳️change-hollow-chord-fy/🦀️.rs"]
pub mod change_hollow_chord_fy;
#[path = "../🕳️change-hollow-chord-stress/🦀️.rs"]
pub mod change_hollow_chord_stress;
//#endregion 🪢️Mounts
