//! 📦️ VDI 3805 products mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📦️add-product/🦀️.rs"]
pub mod add_product;
#[path = "../🎛️change-product-configuration/🦀️.rs"]
pub mod change_product_configuration;
#[path = "../🗑️remove-product/🦀️.rs"]
pub mod remove_product;
#[path = "../🏷️rename-product/🦀️.rs"]
pub mod rename_product;
//#endregion 🪢️Mounts
