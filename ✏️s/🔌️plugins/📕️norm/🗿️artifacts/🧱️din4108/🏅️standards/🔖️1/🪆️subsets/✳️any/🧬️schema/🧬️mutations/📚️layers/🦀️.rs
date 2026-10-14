//! 📚️ DIN 4108 layers mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏷️change-layer-application-type/🦀️.rs"]
pub mod change_layer_application_type;
#[path = "../🏷️change-layer-compressive-class/🦀️.rs"]
pub mod change_layer_compressive_class;
#[path = "../🌡️change-layer-lambda/🦀️.rs"]
pub mod change_layer_lambda;
#[path = "../🧽️change-layer-material-id/🦀️.rs"]
pub mod change_layer_material_id;
#[path = "../💧change-layer-mu/🦀️.rs"]
pub mod change_layer_mu;
#[path = "../📏️change-layer-thickness/🦀️.rs"]
pub mod change_layer_thickness;
#[path = "../➕️insert-layer/🦀️.rs"]
pub mod insert_layer;
#[path = "../➖️remove-layer/🦀️.rs"]
pub mod remove_layer;
#[path = "../🔀️reorder-layers/🦀️.rs"]
pub mod reorder_layers;
//#endregion 🪢️Mounts
