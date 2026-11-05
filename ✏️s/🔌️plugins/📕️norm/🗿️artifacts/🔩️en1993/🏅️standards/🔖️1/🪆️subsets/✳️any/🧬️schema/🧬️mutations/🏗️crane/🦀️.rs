//! 🏗️ EN 1993 crane runway mutation leaves: member id, wheel contact length, dispersion and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-crane-member-id/🦀️.rs"]
pub mod change_crane_member_id;
#[path = "../🏗️change-crane-wheel-contact-length/🦀️.rs"]
pub mod change_crane_wheel_contact_length;
#[path = "../🏗️change-crane-dispersion/🦀️.rs"]
pub mod change_crane_dispersion;
#[path = "../🏗️change-crane-web-thickness/🦀️.rs"]
pub mod change_crane_web_thickness;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_crane_more::{change_crane_fy, change_crane_phi, change_crane_max_wheel_load, change_crane_load_factor};
