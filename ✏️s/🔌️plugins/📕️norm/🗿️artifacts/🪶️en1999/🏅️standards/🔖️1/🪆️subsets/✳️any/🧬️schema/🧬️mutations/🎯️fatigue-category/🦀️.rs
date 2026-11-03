//! 🎯️ EN 1999 fatigue detail site mutation leaves: member id, element id, detail category and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🎯️change-category-member-id/🦀️.rs"]
pub mod change_category_member_id;
#[path = "../🎯️change-category-element-id/🦀️.rs"]
pub mod change_category_element_id;
#[path = "../🎯️change-category-detail-category/🦀️.rs"]
pub mod change_category_detail_category;
#[path = "../🎯️change-category-detail-type/🦀️.rs"]
pub mod change_category_detail_type;
#[path = "../🎯️change-category-location-x/🦀️.rs"]
pub mod change_category_location_x;
#[path = "../🎯️change-category-m1/🦀️.rs"]
pub mod change_category_m1;
#[path = "../🎯️change-category-m2/🦀️.rs"]
pub mod change_category_m2;
//#endregion 🪢️Mounts
