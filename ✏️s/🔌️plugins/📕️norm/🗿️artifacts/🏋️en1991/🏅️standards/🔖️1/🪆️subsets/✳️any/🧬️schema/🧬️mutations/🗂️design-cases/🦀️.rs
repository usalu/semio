//! 🗂️ EN 1991 design-case mutation leaves: insert, remove, set number, set text, label and entry id.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📇️change-case-entry-id/🦀️.rs"]
pub mod change_case_entry_id;
#[path = "../🏷️change-case-label-de/🦀️.rs"]
pub mod change_case_label_de;
#[path = "../🏷️change-case-label-en/🦀️.rs"]
pub mod change_case_label_en;
#[path = "../➕️insert-case/🦀️.rs"]
pub mod insert_case;
#[path = "../➖️remove-case/🦀️.rs"]
pub mod remove_case;
#[path = "../🔢️change-case-number/🦀️.rs"]
pub mod change_case_number;
#[path = "../🔤️change-case-text/🦀️.rs"]
pub mod change_case_text;
//#endregion 🪢️Mounts
