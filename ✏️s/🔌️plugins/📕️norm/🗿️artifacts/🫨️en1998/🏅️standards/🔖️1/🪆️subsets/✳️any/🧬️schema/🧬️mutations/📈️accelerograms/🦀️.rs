//! 📈️ EN 1998 accelerograms mutation leaves: building id, label, samples.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-accelerogram-building-id/🦀️.rs"]
pub mod change_accelerogram_building_id;
#[path = "../✏️change-accelerogram-label/🦀️.rs"]
pub mod change_accelerogram_label;
#[path = "../✏️change-accelerogram-samples/🦀️.rs"]
pub mod change_accelerogram_samples;
#[path = "../✏️change-accelerogram-dt-s/🦀️.rs"]
pub mod change_accelerogram_dt_s;
#[path = "../✏️change-accelerogram-scale-factor/🦀️.rs"]
pub mod change_accelerogram_scale_factor;
#[path = "../➕️insert-accelerogram/🦀️.rs"]
pub mod insert_accelerogram;
#[path = "../➖️remove-accelerogram/🦀️.rs"]
pub mod remove_accelerogram;
//#endregion 🪢️Mounts
