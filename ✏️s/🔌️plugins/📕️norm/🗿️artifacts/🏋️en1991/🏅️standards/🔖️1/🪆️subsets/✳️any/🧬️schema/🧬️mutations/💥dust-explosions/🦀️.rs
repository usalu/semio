//! 💥 DustExplosion mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️insert-dust-explosion/🦀️.rs"]
pub mod insert_dust_explosion;
#[path = "../⚙️remove-dust-explosion/🦀️.rs"]
pub mod remove_dust_explosion;
#[path = "../⚙️change-dust-explosion-design-pressure/🦀️.rs"]
pub mod change_dust_explosion_design_pressure;
#[path = "../⚙️change-dust-explosion-static-opening-pressure/🦀️.rs"]
pub mod change_dust_explosion_static_opening_pressure;
#[path = "../⚙️change-dust-explosion-vent-area/🦀️.rs"]
pub mod change_dust_explosion_vent_area;
#[path = "../⚙️change-dust-explosion-volume/🦀️.rs"]
pub mod change_dust_explosion_volume;
#[path = "../⚙️change-dust-explosion-p-max/🦀️.rs"]
pub mod change_dust_explosion_p_max;
#[path = "../⚙️change-dust-explosion-kst/🦀️.rs"]
pub mod change_dust_explosion_kst;
//#endregion 🪢️Mounts
