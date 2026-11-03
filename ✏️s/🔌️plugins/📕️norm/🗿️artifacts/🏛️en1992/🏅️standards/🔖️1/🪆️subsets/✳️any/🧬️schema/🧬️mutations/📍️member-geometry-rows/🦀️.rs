//! 📍 EN 1992 member geometry rows mutation leaves: bar point, stud ring, tendon point and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📍️insert-bar-point/🦀️.rs"]
pub mod insert_bar_point;
#[path = "../📍️remove-bar-point/🦀️.rs"]
pub mod remove_bar_point;
#[path = "../🧬️insert-tendon-point/🦀️.rs"]
pub mod insert_tendon_point;
#[path = "../🧬️remove-tendon-point/🦀️.rs"]
pub mod remove_tendon_point;
#[path = "../🔩️insert-stud-row/🦀️.rs"]
pub mod insert_stud_row;
#[path = "../🔩️remove-stud-row/🦀️.rs"]
pub mod remove_stud_row;
#[path = "../🕳️insert-opening/🦀️.rs"]
pub mod insert_opening;
#[path = "../🕳️remove-opening/🦀️.rs"]
pub mod remove_opening;
//#endregion 🪢️Mounts
