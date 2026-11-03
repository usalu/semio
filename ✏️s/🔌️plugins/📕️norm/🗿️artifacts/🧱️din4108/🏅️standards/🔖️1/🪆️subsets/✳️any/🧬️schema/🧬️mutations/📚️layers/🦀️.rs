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
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_din4108_mut_layers_more::{change_layer_mu, change_layer_thickness, insert_layer, remove_layer, reorder_layers};
