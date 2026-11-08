//! 🧱️ FireInsulation mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-fire-insulation/🦀️.rs"]
pub mod insert_fire_insulation;
#[path = "../➖remove-fire-insulation/🦀️.rs"]
pub mod remove_fire_insulation;
#[path = "../🦴️change-fire-insulation-fire-id/🦀️.rs"]
pub mod change_fire_insulation_fire_id;
#[path = "../🦴️change-fire-insulation-density/🦀️.rs"]
pub mod change_fire_insulation_density;
#[path = "../🦴️change-fire-insulation-specific-heat/🦀️.rs"]
pub mod change_fire_insulation_specific_heat;
#[path = "../🦴️change-fire-insulation-conductivity/🦀️.rs"]
pub mod change_fire_insulation_conductivity;
#[path = "../🦴️change-fire-insulation-thickness/🦀️.rs"]
pub mod change_fire_insulation_thickness;
//#endregion 🪢️Mounts
