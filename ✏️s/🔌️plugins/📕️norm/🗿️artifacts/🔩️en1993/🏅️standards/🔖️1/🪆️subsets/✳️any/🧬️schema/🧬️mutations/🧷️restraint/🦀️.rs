//! 🧷️ EN 1993 restraint mutation leaves: member id, label, start support and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧷️change-restraint-member-id/🦀️.rs"]
pub mod change_restraint_member_id;
#[path = "../🧷️change-restraint-label/🦀️.rs"]
pub mod change_restraint_label;
#[path = "../🧷️change-restraint-start-support/🦀️.rs"]
pub mod change_restraint_start_support;
#[path = "../🧷️change-restraint-end-support/🦀️.rs"]
pub mod change_restraint_end_support;
#[path = "../🧷️change-rest-intermediate-position/🦀️.rs"]
pub mod change_rest_intermediate_position;
#[path = "../🧷️change-restraint-torsional-spring/🦀️.rs"]
pub mod change_restraint_torsional_spring;
//#endregion 🪢️Mounts
