//! 🎲️ Generation2d generation mutation leaves: create, delete, select and rename generations and change their answers. Every leaf is generic over the dispatch enum.
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
pub use create_generation::create_generation;
#[path = "."]
pub mod delete_generation {
    #[path = "../➖delete-generation/🦀️.rs"]
    mod component;
    #[path = "../➖delete-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../➖delete-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
pub use delete_generation::delete_generation;
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
pub use select_generation::select_generation;
#[path = "."]
pub mod rename_generation {
    #[path = "../🏷️rename/🦀️.rs"]
    mod component;
    #[path = "../🏷️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏷️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
pub use rename_generation::rename_generation;
#[path = "."]
pub mod change_generation_value {
    #[path = "../🔢️change/🦀️.rs"]
    mod component;
    #[path = "../🔢️change/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔢️change/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
pub use change_generation_value::change_generation_value;
