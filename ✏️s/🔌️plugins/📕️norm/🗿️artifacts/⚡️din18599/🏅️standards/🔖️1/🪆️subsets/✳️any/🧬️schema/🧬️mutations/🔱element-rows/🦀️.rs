//! 🔰 DIN V 18599 envelope element row insert, remove and label leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🆕insert-element/🦀️.rs"]
pub mod insert_element;
#[path = "../🚮remove-element/🦀️.rs"]
pub mod remove_element;
#[path = "../🟩change-element-label-en/🦀️.rs"]
pub mod change_element_label_en;
#[path = "../🟦change-element-label-de/🦀️.rs"]
pub mod change_element_label_de;
#[path = "../🟪change-element-kind/🦀️.rs"]
pub mod change_element_kind;
#[path = "../🟫change-element-zone-id/🦀️.rs"]
pub mod change_element_zone_id;
//#endregion 🪢️Mounts
