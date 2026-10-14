//! 🧊️ Lowpoly object mutations: create, delete, rename, move, rotate, scale, reorder and smooth-shade objects.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

//#region 🧩️Modules
#[path = "."]
pub mod create_object {
    #[path = "../🌱️create-object/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🌱️create-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌱️create-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_object {
    #[path = "../💀️delete-object/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../💀️delete-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../💀️delete-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod reorder_objects {
    #[path = "../🔀️reorder-objects/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔀️reorder-objects/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔀️reorder-objects/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod rename_object {
    #[path = "../🏷️rename-object/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🏷️rename-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏷️rename-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_object_smooth_shading {
    #[path = "../🔘️change-object-smooth/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔘️change-object-smooth/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔘️change-object-smooth/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod move_object {
    #[path = "../↗️move-object/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../↗️move-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../↗️move-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod rotate_object {
    #[path = "../🔄️rotate-object/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔄️rotate-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔄️rotate-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod scale_object {
    #[path = "../📐️scale-object/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📐️scale-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📐️scale-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
