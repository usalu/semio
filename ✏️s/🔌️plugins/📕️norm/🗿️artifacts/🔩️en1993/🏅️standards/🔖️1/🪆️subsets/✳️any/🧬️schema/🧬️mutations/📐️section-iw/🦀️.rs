//! 📐️ EN 1993 section mutation leaves: iw, w el y, w el z and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-section-iw/🦀️.rs"]
pub mod change_section_iw;
#[path = "../📐️change-section-w-el-y/🦀️.rs"]
pub mod change_section_w_el_y;
#[path = "../📐️change-section-w-el-z/🦀️.rs"]
pub mod change_section_w_el_z;
#[path = "../📐️change-section-w-pl-y/🦀️.rs"]
pub mod change_section_w_pl_y;
#[path = "../📐️change-section-w-pl-z/🦀️.rs"]
pub mod change_section_w_pl_z;
#[path = "../📐️change-section-area-net/🦀️.rs"]
pub mod change_section_area_net;
//#endregion 🪢️Mounts
