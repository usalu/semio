//! 🔄️ EN 1999 fatigue detail mutation leaves: member id, detail category, delta sigma c and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔄️change-fatigue-member-id/🦀️.rs"]
pub mod change_fatigue_member_id;
#[path = "../🔄️change-fatigue-detail-category/🦀️.rs"]
pub mod change_fatigue_detail_category;
#[path = "../🔄️change-fatigue-delta-sigma-c/🦀️.rs"]
pub mod change_fatigue_delta_sigma_c;
#[path = "../🔄️change-fatigue-delta-sigma-ed/🦀️.rs"]
pub mod change_fatigue_delta_sigma_ed;
#[path = "../🔄️change-fatigue-n-cycles/🦀️.rs"]
pub mod change_fatigue_n_cycles;
#[path = "../🔄️change-fatigue-m1/🦀️.rs"]
pub mod change_fatigue_m1;
#[path = "../🔄️change-fatigue-m2/🦀️.rs"]
pub mod change_fatigue_m2;
//#endregion 🪢️Mounts
