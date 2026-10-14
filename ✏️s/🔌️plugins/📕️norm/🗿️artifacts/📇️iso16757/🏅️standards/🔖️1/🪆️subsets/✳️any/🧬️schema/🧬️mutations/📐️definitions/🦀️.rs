//! 📐️ ISO 16757 definitions mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "."]
pub mod introduce_geometry_object {
    #[path = "../📐introduce-geometry-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📐introduce-geometry-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../📐introduce-geometry-object/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod introduce_product_index {
    #[path = "../🔎introduce-product-index/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔎introduce-product-index/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🔎introduce-product-index/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod introduce_property_definition {
    #[path = "../📐️introduce-property-definition/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📐️introduce-property-definition/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../📐️introduce-property-definition/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod introduce_subject {
    #[path = "../🌳️introduce-subject/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌳️introduce-subject/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🌳️introduce-subject/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod retire_geometry_object {
    #[path = "../🗑️retire-geometry-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗑️retire-geometry-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🗑️retire-geometry-object/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod retire_product_index {
    #[path = "../🗑️retire-product-index/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗑️retire-product-index/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🗑️retire-product-index/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod retire_property_definition {
    #[path = "../🧽️retire-property-definition/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧽️retire-property-definition/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🧽️retire-property-definition/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod retire_subject {
    #[path = "../✂️retire-subject/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../✂️retire-subject/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../✂️retire-subject/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🪢️Mounts
