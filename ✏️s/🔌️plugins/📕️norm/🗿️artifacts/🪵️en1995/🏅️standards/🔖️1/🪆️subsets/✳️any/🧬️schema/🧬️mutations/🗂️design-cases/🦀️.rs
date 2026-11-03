//! 🗂️ EN 1995 design-case mutation leaves: insert, remove, set number, set text, label, entry id and the ULS equation.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📇️change-case-entry-id/🦀️.rs"]
pub mod change_case_entry_id;
#[path = "../🏷️change-case-label-de/🦀️.rs"]
pub mod change_case_label_de;
#[path = "../🏷️change-case-label-en/🦀️.rs"]
pub mod change_case_label_en;
#[path = "../🧮️change-uls-equation/🦀️.rs"]
pub mod change_uls_equation;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1995_mut_design_cases_more::{insert_case, remove_case, change_case_number, change_case_text};
