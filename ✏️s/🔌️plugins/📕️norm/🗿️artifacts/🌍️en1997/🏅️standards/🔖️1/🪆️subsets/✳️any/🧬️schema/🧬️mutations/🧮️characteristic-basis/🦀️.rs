//! 🧮 EN 1997 characteristic value basis mutation leaves: layer id, parameter, basis.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧮️change-characteristic-basis-layer-id/🦀️.rs"]
pub mod change_characteristic_basis_layer_id;
#[path = "../🧮️change-characteristic-basis-parameter/🦀️.rs"]
pub mod change_characteristic_basis_parameter;
#[path = "../🧮️change-characteristic-basis-basis/🦀️.rs"]
pub mod change_characteristic_basis_basis;
//#endregion 🪢️Mounts
