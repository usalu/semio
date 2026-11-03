//! 🪆 EN 1997 nested row mutation leaves: insert and remove of loadCases, testProfiles, tests and points.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-load-case/🦀️.rs"]
pub mod insert_load_case;
#[path = "../➖️remove-load-case/🦀️.rs"]
pub mod remove_load_case;
#[path = "../➕️insert-test-profile/🦀️.rs"]
pub mod insert_test_profile;
#[path = "../➖️remove-test-profile/🦀️.rs"]
pub mod remove_test_profile;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1997_mut_nested_rows_more::{insert_anchor_test, remove_anchor_test, insert_lab_point, remove_lab_point, insert_ground_investigation_reading, remove_ground_investigation_reading};
