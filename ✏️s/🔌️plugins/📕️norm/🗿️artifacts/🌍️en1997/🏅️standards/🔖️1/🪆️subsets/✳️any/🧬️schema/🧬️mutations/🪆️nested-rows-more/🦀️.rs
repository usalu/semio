//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-anchor-test/🦀️.rs"]
pub mod insert_anchor_test;
#[path = "../➖️remove-anchor-test/🦀️.rs"]
pub mod remove_anchor_test;
#[path = "../➕️insert-lab-point/🦀️.rs"]
pub mod insert_lab_point;
#[path = "../➖️remove-lab-point/🦀️.rs"]
pub mod remove_lab_point;
#[path = "../➕️insert-ground-investigation-reading/🦀️.rs"]
pub mod insert_ground_investigation_reading;
#[path = "../➖️remove-ground-investigation-reading/🦀️.rs"]
pub mod remove_ground_investigation_reading;
//#endregion 🪢️Mounts
