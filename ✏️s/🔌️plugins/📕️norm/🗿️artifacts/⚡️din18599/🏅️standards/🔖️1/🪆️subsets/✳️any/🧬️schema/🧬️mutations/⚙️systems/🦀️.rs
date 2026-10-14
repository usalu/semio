//! ⚙️ DIN V 18599 systems mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚿specify-dhw-system/🦀️.rs"]
pub mod specify_dhw_system;
#[path = "../🔥specify-heating-system/🦀️.rs"]
pub mod specify_heating_system;
#[path = "../❄️update-cooling/🦀️.rs"]
pub mod update_cooling;
#[path = "../💡update-lighting/🦀️.rs"]
pub mod update_lighting;
#[path = "../☀️update-renewables/🦀️.rs"]
pub mod update_renewables;
#[path = "../🌬️update-ventilation/🦀️.rs"]
pub mod update_ventilation;
//#endregion 🪢️Mounts
