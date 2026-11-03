//! 🧱️ EN 1993 foundation block mutation leaves: grout fck, grout thickness, anchor count and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-foundation-grout-fck/🦀️.rs"]
pub mod change_foundation_grout_fck;
#[path = "../🧱️change-foundation-grout-thickness/🦀️.rs"]
pub mod change_foundation_grout_thickness;
#[path = "../🧱️change-foundation-anchor-count/🦀️.rs"]
pub mod change_foundation_anchor_count;
#[path = "../🧱️change-foundation-anchor-class/🦀️.rs"]
pub mod change_foundation_anchor_class;
#[path = "../🧱️change-foundation-anchor-diameter/🦀️.rs"]
pub mod change_foundation_anchor_diameter;
#[path = "../🧱️change-foundation-anchor-stress-area/🦀️.rs"]
pub mod change_foundation_anchor_stress_area;
#[path = "../🧱️change-foundation-anchor-embedment/🦀️.rs"]
pub mod change_foundation_anchor_embedment;
#[path = "../🧱️change-foundation-anchor-edge-distance/🦀️.rs"]
pub mod change_foundation_anchor_edge_distance;
#[path = "../🧱️change-frame-action-my/🦀️.rs"]
pub mod change_frame_action_my;
//#endregion 🪢️Mounts
