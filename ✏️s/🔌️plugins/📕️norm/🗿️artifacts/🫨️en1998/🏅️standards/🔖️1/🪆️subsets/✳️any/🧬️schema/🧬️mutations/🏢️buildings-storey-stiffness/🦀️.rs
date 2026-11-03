//! 🏢️ EN 1998 building storey stiffness mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-storey-centre-of-stiffness-xm/🦀️.rs"]
pub mod change_storey_centre_of_stiffness_x_m;
#[path = "../✏️change-storey-centre-of-stiffness-ym/🦀️.rs"]
pub mod change_storey_centre_of_stiffness_y_m;
#[path = "../✏️change-storey-drift-ym/🦀️.rs"]
pub mod change_storey_drift_y_m;
#[path = "../✏️change-storey-shear-resistance-n/🦀️.rs"]
pub mod change_storey_shear_resistance_n;
//#endregion 🪢️Mounts
