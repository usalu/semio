//! 🏛️ EN 1992 member prestress fire mutation leaves: member fire slab system, member prestress area and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏛️change-member-prestress-force/🦀️.rs"]
pub mod change_member_prestress_force;
#[path = "../🏛️change-member-prestress-area/🦀️.rs"]
pub mod change_member_prestress_area;
#[path = "../🏛️change-member-prestress-ecc/🦀️.rs"]
pub mod change_member_prestress_ecc;
#[path = "../🏛️change-member-prestress-loss-ratio/🦀️.rs"]
pub mod change_member_prestress_loss_ratio;
#[path = "../🏛️change-member-fire-slab-system/🦀️.rs"]
pub mod change_member_fire_slab_system;
//#endregion 🪢️Mounts
