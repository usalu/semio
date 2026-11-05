//! 🪜 BucklingLength mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-buckling-length-member-id/🦀️.rs"]
pub mod change_buckling_length_member_id;
#[path = "../🧱️change-buckling-length-beam-stiffness-b/🦀️.rs"]
pub mod change_buckling_length_beam_stiffness_b;
#[path = "../🧱️change-buckling-length-column-stiffness-b/🦀️.rs"]
pub mod change_buckling_length_column_stiffness_b;
#[path = "../🧱️change-buckling-length-beam-stiffness-a/🦀️.rs"]
pub mod change_buckling_length_beam_stiffness_a;
#[path = "../🧱️change-buckling-length-column-stiffness-a/🦀️.rs"]
pub mod change_buckling_length_column_stiffness_a;
#[path = "../📏️change-buckling-length-restraint-b/🦀️.rs"]
pub mod change_buckling_length_restraint_b;
#[path = "../📏️change-buckling-length-restraint-a/🦀️.rs"]
pub mod change_buckling_length_restraint_a;
#[path = "../📏️change-buckling-length-sway/🦀️.rs"]
pub mod change_buckling_length_sway;
#[path = "../📏️change-buckling-length-plane/🦀️.rs"]
pub mod change_buckling_length_plane;
#[path = "../➕️insert-buckling-length/🦀️.rs"]
pub mod insert_buckling_length;
#[path = "../➖️remove-buckling-length/🦀️.rs"]
pub mod remove_buckling_length;
//#endregion 🪢️Mounts
