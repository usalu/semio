//! 🔥️ EN 1999 fire scenario mutation leaves: member id, theta a, duration s.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔥️change-fire-member-id/🦀️.rs"]
pub mod change_fire_member_id;
#[path = "../🔥️change-fire-theta-a/🦀️.rs"]
pub mod change_fire_theta_a;
#[path = "../🔥️change-fire-duration-s/🦀️.rs"]
pub mod change_fire_duration_s;
//#endregion 🪢️Mounts
