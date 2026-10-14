//! 🪟️ DIN 4108 windows mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏change-zone-window-area/🦀️.rs"]
pub mod change_zone_window_area;
#[path = "../☀️change-zone-window-g-value/🦀️.rs"]
pub mod change_zone_window_g_value;
#[path = "../📐change-zone-window-inclination-deg/🦀️.rs"]
pub mod change_zone_window_inclination_deg;
#[path = "../🧭change-zone-window-orientation/🦀️.rs"]
pub mod change_zone_window_orientation;
#[path = "../⛱️change-zone-window-shading-fc/🦀️.rs"]
pub mod change_zone_window_shading_fc;
#[path = "../🪟insert-zone-window/🦀️.rs"]
pub mod insert_zone_window;
#[path = "../🚫️remove-zone-window/🦀️.rs"]
pub mod remove_zone_window;
//#endregion 🪢️Mounts
