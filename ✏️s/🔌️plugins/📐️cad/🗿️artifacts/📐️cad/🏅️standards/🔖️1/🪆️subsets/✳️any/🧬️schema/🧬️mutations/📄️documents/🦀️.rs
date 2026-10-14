//! 📄️ CAD child-document collection mutations: create and delete drawing and brep children.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[path = "../🧊️create-brep/🦀️.rs"]
pub mod create_brep;
#[path = "../🧹delete-brep/🦀️.rs"]
pub mod delete_brep;
#[path = "."]
pub mod create_drawing {
    #[path = "../📐️create-drawing/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📐️create-drawing/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📐️create-drawing/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_drawing {
    #[path = "../🧹delete-drawing/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🧹delete-drawing/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧹delete-drawing/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
