//! 🏢️ EN 1998 buildings overturning mutation leaves: overturning base x m, overturning base y m, tie down moment nm.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-building-ot-base-xm/🦀️.rs"]
pub mod change_building_ot_base_x_m;
#[path = "../✏️change-building-ot-base-ym/🦀️.rs"]
pub mod change_building_ot_base_y_m;
#[path = "../✏️change-building-tiedown-nm/🦀️.rs"]
pub mod change_building_tiedown_nm;
//#endregion 🪢️Mounts
