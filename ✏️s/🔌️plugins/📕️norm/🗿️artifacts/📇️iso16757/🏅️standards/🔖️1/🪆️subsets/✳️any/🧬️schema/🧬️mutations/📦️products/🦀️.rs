//! 📦️ ISO 16757 products mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "."]
pub mod introduce_product {
    #[path = "../📦️introduce-product/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📦️introduce-product/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../📦️introduce-product/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod introduce_product_group {
    #[path = "../🧺️introduce-product-group/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧺️introduce-product-group/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🧺️introduce-product-group/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod rename_product {
    #[path = "../🏷️rename-product/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏷️rename-product/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🏷️rename-product/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod retire_product {
    #[path = "../🚫️retire-product/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🚫️retire-product/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🚫️retire-product/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod retire_product_group {
    #[path = "../🧹️retire-product-group/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧹️retire-product-group/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🧹️retire-product-group/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_iso16757_mut_products_2::{introduce_product_class, introduce_product_series, rename_product_group, retire_product_class, retire_product_series};
