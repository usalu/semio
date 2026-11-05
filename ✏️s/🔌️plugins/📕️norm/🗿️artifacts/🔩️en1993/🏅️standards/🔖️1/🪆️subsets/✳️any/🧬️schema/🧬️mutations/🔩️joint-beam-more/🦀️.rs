//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-xform-parameter/🦀️.rs"]
pub mod change_joint_xform_parameter;
#[path = "../🔩️change-joint-assumed-class/🦀️.rs"]
pub mod change_joint_assumed_class;
#[path = "../🔩️change-joint-base-plate-length/🦀️.rs"]
pub mod change_joint_base_plate_length;
#[path = "../🔩️change-joint-concrete-fcd/🦀️.rs"]
pub mod change_joint_concrete_fcd;
#[path = "../🔩️change-joint-concrete-modulus/🦀️.rs"]
pub mod change_joint_concrete_modulus;
//#endregion 🪢️Mounts
