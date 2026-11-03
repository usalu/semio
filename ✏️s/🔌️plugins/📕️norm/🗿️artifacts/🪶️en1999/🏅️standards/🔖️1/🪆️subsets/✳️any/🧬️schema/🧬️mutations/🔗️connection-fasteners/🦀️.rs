//! 🔗️ EN 1999 connection mutation leaves: bolts gauge, bolts plate thickness, welds filler alloy and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-conn-bolts-gauge/🦀️.rs"]
pub mod change_conn_bolts_gauge;
#[path = "../🔗️change-conn-bolts-plate-thickness/🦀️.rs"]
pub mod change_conn_bolts_plate_thickness;
#[path = "../🔗️change-conn-welds-filler-alloy/🦀️.rs"]
pub mod change_conn_welds_filler_alloy;
#[path = "../🔗️change-conn-welds-length/🦀️.rs"]
pub mod change_conn_welds_length;
#[path = "../🔗️change-conn-welds-beta-w/🦀️.rs"]
pub mod change_conn_welds_beta_w;
#[path = "../🔗️change-conn-welds-haz-extent/🦀️.rs"]
pub mod change_conn_welds_haz_extent;
//#endregion 🪢️Mounts
