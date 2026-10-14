//! 🎲️ Generation3d generation mutation leaves: create, delete, select and rename generations and change their answers and preview. Every leaf is generic over the dispatch enum.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod create_generation {
    #[path = "../➕create-generation/🦀️.rs"]
    mod component;
    #[path = "../➕create-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../➕create-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod delete_generation {
    #[path = "../🗑️delete-generation/🦀️.rs"]
    mod component;
    #[path = "../🗑️delete-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗑️delete-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod select_generation {
    #[path = "../👆️select-generation/🦀️.rs"]
    mod component;
    #[path = "../👆️select-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../👆️select-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod rename_generation {
    #[path = "../🏷️rename-generation/🦀️.rs"]
    mod component;
    #[path = "../🏷️rename-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏷️rename-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod change_generation_value {
    #[path = "../🔧️change-generation-value/🦀️.rs"]
    mod component;
    #[path = "../🔧️change-generation-value/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔧️change-generation-value/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod change_generation_preview {
    #[path = "../📝️change-generation-preview/🦀️.rs"]
    mod component;
    #[path = "../📝️change-generation-preview/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📝️change-generation-preview/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
