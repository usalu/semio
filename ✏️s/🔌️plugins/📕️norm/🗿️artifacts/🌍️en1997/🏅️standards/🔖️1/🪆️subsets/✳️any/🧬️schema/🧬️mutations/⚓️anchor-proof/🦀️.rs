//! ⚓️ EN 1997 anchor mutation leaves: bond stress, design force, tendon yield force and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚓️change-anchor-bond-stress/🦀️.rs"]
pub mod change_anchor_bond_stress;
#[path = "../⚓️change-anchor-design-force/🦀️.rs"]
pub mod change_anchor_design_force;
#[path = "../⚓️change-anchor-tendon-yield-force/🦀️.rs"]
pub mod change_anchor_tendon_yield_force;
#[path = "../⚓️change-anchor-min-free-length/🦀️.rs"]
pub mod change_anchor_min_free_length;
#[path = "../⚓️change-anchor-test-factor/🦀️.rs"]
pub mod change_anchor_test_factor;
//#endregion 🪢️Mounts
