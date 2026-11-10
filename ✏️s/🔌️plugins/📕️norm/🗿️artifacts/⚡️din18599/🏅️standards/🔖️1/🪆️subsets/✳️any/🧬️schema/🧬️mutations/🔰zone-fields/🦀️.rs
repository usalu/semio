//! 🔰 DIN V 18599 zone numeric field leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🟣change-zone-area-m2/🦀️.rs"]
pub mod change_zone_area_m2;
#[path = "../🟤change-zone-volume-m3/🦀️.rs"]
pub mod change_zone_volume_m3;
#[path = "../🔴change-zone-theta-i-heat-c/🦀️.rs"]
pub mod change_zone_theta_i_heat_c;
#[path = "../🟥change-zone-theta-i-cool-c/🦀️.rs"]
pub mod change_zone_theta_i_cool_c;
#[path = "../🟧change-zone-internal-gains-w-m2/🦀️.rs"]
pub mod change_zone_internal_gains_w_m2;
#[path = "../🟨change-zone-lighting-power-w-m2/🦀️.rs"]
pub mod change_zone_lighting_power_w_m2;
#[path = "../🧊change-zone-thermal-mass/🦀️.rs"]
pub mod change_zone_thermal_mass;
//#endregion 🪢️Mounts
