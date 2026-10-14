//! 🕸️ CAD scene-graph node mutations: create, delete and rename nodes.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[path = "."]
pub mod create_node {
    #[path = "../➕create-node/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../➕create-node/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../➕create-node/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_node {
    #[path = "../🗑️delete-node/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🗑️delete-node/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗑️delete-node/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod rename_node {
    #[path = "../🏷️rename-node/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🏷️rename-node/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏷️rename-node/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
