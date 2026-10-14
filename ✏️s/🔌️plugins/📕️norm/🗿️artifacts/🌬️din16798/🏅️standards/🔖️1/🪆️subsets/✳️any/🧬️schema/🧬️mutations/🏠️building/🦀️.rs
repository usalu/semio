//! 🏠️ DIN EN 16798 building mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌍️change-annex/🦀️.rs"]
pub mod change_annex;
#[path = "../🏚️change-cellar-area/🦀️.rs"]
pub mod change_cellar_area;
#[path = "../🌀change-cellar-ventilation/🦀️.rs"]
pub mod change_cellar_ventilation;
#[path = "../🏠️change-envelope-n50/🦀️.rs"]
pub mod change_envelope_n50;
#[path = "../📦️change-envelope-volume/🦀️.rs"]
pub mod change_envelope_volume;
#[path = "../🌙️change-night-setback/🦀️.rs"]
pub mod change_night_setback;
#[path = "../🌫️change-outdoor-co2/🦀️.rs"]
pub mod change_outdoor_co2;
#[path = "../🔄️change-theta-rm/🦀️.rs"]
pub mod change_theta_rm;
//#endregion 🪢️Mounts
