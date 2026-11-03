//! 🔥 LocalFire mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️change-local-fire-assumed-flux/🦀️.rs"]
pub mod change_local_fire_assumed_flux;
#[path = "../⚙️change-local-fire-horizontal-distance/🦀️.rs"]
pub mod change_local_fire_horizontal_distance;
#[path = "../⚙️change-local-fire-ceiling-height/🦀️.rs"]
pub mod change_local_fire_ceiling_height;
#[path = "../⚙️change-local-fire-heat-release-rate/🦀️.rs"]
pub mod change_local_fire_heat_release_rate;
#[path = "../⚙️change-local-fire-fire-diameter/🦀️.rs"]
pub mod change_local_fire_fire_diameter;
#[path = "../⚙️insert-local-fire/🦀️.rs"]
pub mod insert_local_fire;
#[path = "../⚙️remove-local-fire/🦀️.rs"]
pub mod remove_local_fire;
//#endregion 🪢️Mounts
