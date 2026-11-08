//! 🔰 DIN V 18599 zone row insert, remove and label leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-zone/🦀️.rs"]
pub mod insert_zone;
#[path = "../➖remove-zone/🦀️.rs"]
pub mod remove_zone;
#[path = "../🟠change-zone-label-en/🦀️.rs"]
pub mod change_zone_label_en;
#[path = "../🟡change-zone-label-de/🦀️.rs"]
pub mod change_zone_label_de;
#[path = "../🟢change-zone-usage-profile/🦀️.rs"]
pub mod change_zone_usage_profile;
#[path = "../🔵change-zone-occupants/🦀️.rs"]
pub mod change_zone_occupants;
//#endregion 🪢️Mounts
