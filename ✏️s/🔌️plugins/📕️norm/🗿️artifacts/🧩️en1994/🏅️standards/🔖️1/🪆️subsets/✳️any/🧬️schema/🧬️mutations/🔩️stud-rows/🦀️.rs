//! 🔩 Stud mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📍️insert-stud/🦀️.rs"]
pub mod insert_stud;
#[path = "../📍️remove-stud/🦀️.rs"]
pub mod remove_stud;
#[path = "../➡️change-stud-fu-pa/🦀️.rs"]
pub mod change_stud_f_u_pa;
#[path = "../➡️change-stud-height-m/🦀️.rs"]
pub mod change_stud_height_m;
#[path = "../➡️change-stud-diameter-m/🦀️.rs"]
pub mod change_stud_diameter_m;
#[path = "../➡️change-stud-xm/🦀️.rs"]
pub mod change_stud_x_m;
//#endregion 🪢️Mounts
