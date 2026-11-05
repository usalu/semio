//! 🏗️ EN 1990 members geometry mutation leaves: member design dimension, member dimension tolerance, member nominal dimension.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-nominal-dimension/🦀️.rs"]
pub mod change_member_nominal_dimension;
#[path = "../🏗️change-member-dimension-tolerance/🦀️.rs"]
pub mod change_member_dimension_tolerance;
#[path = "../🏗️change-member-design-dimension/🦀️.rs"]
pub mod change_member_design_dimension;
//#endregion 🪢️Mounts
