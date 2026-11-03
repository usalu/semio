//! 🏗️ EN 1990 members material mutation leaves: member mat design strength, member mat partial factor, member material conv factor and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-material-strength-k/🦀️.rs"]
pub mod change_member_material_strength_k;
#[path = "../🏗️change-member-material-conv-factor/🦀️.rs"]
pub mod change_member_material_conv_factor;
#[path = "../🏗️change-member-mat-partial-factor/🦀️.rs"]
pub mod change_member_mat_partial_factor;
#[path = "../🏗️change-member-mat-design-strength/🦀️.rs"]
pub mod change_member_mat_design_strength;
//#endregion 🪢️Mounts
