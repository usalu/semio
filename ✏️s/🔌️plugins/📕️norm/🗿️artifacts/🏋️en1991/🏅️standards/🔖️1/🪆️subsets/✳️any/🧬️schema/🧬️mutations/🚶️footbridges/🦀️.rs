//! 🚶 EN 1991 footbridges mutation leaves: footbridge, footbridge deck width, footbridge loaded length and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚶️change-footbridge-loaded-length/🦀️.rs"]
pub mod change_footbridge_loaded_length;
#[path = "../🚶️change-footbridge-deck-width/🦀️.rs"]
pub mod change_footbridge_deck_width;
#[path = "../🚶️change-footbridge-natural-frequency/🦀️.rs"]
pub mod change_footbridge_natural_frequency;
#[path = "../🚶️change-footbridge-assumed-service-axle/🦀️.rs"]
pub mod change_footbridge_assumed_service_axle;
#[path = "../🚶️insert-footbridge/🦀️.rs"]
pub mod insert_footbridge;
#[path = "../🚶️remove-footbridge/🦀️.rs"]
pub mod remove_footbridge;
//#endregion 🪢️Mounts
