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
pub mod introduce_product_class {
    #[path = "../🏷️introduce-product-class/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏷️introduce-product-class/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🏷️introduce-product-class/🦠️mutation/🦀️.rs"]
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
pub mod introduce_product_series {
    #[path = "../📚introduce-product-series/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📚introduce-product-series/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../📚introduce-product-series/🦠️mutation/🦀️.rs"]
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
pub mod rename_product_group {
    #[path = "../🗂️rename-product-group/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗂️rename-product-group/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🗂️rename-product-group/🦠️mutation/🦀️.rs"]
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
pub mod retire_product_class {
    #[path = "../🗑️retire-product-class/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗑️retire-product-class/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🗑️retire-product-class/🦠️mutation/🦀️.rs"]
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
#[path = "."]
pub mod retire_product_series {
    #[path = "../🗑️retire-product-series/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗑️retire-product-series/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🗑️retire-product-series/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🪢️Mounts
