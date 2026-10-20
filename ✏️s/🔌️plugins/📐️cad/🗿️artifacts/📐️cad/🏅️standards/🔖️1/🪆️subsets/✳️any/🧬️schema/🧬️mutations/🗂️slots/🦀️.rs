//! 🗂️ CAD model-slot lifecycle mutations: create and delete the shape, building, energy and structure-classic model children.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[path = "."]
pub mod create_shape_model {
    #[path = "../🧱create-shape-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🧱create-shape-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧱create-shape-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_shape_model {
    #[path = "../🧨delete-shape-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🧨delete-shape-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧨delete-shape-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod create_building_model {
    #[path = "../🏢create-building-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🏢create-building-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏢create-building-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_building_model {
    #[path = "../💥delete-building-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../💥delete-building-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../💥delete-building-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod create_energy_model {
    #[path = "../⚡create-energy-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../⚡create-energy-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../⚡create-energy-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_energy_model {
    #[path = "../🔌delete-energy-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔌delete-energy-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔌delete-energy-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod create_structure_classic_model {
    #[path = "../🏛️create-structure-classic-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🏛️create-structure-classic-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏛️create-structure-classic-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_structure_classic_model {
    #[path = "../💣delete-structure-classic-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../💣delete-structure-classic-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../💣delete-structure-classic-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
