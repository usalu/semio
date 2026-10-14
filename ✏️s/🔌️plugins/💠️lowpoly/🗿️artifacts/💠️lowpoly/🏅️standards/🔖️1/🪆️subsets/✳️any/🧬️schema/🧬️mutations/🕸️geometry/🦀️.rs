//! 🕸️ Lowpoly geometry mutations: mesh children, vertex positions and selection transforms.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

//#region 🧩️Modules
#[path = "."]
pub mod create_mesh {
    #[path = "../🕸️create-mesh/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🕸️create-mesh/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🕸️create-mesh/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_mesh {
    #[path = "../🧨delete-mesh/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🧨delete-mesh/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧨delete-mesh/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod move_selection {
    #[path = "../🚚️move-selection/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🚚️move-selection/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🚚️move-selection/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod rotate_selection {
    #[path = "../🌀️rotate-selection/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🌀️rotate-selection/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌀️rotate-selection/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod scale_selection {
    #[path = "../🔍️scale-selection/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔍️scale-selection/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔍️scale-selection/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod set_vertex_positions {
    #[path = "../📍️set-vertex-positions/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📍️set-vertex-positions/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📍️set-vertex-positions/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
