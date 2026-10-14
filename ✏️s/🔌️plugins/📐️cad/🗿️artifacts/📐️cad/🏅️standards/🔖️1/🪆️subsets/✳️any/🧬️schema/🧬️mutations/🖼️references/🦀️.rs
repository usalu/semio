//! 🖼️ CAD reference mutations: visibility, lock, width, placement, media and list replacement of references.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[path = "."]
pub mod change_reference_hidden {
    #[path = "../👁️change-reference-hidden/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../👁️change-reference-hidden/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../👁️change-reference-hidden/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_reference_locked {
    #[path = "../🔒change-reference-locked/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔒change-reference-locked/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔒change-reference-locked/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_reference_width {
    #[path = "../📏change-reference-width/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📏change-reference-width/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📏change-reference-width/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod move_reference {
    #[path = "../📍move-reference/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📍move-reference/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📍move-reference/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod replace_reference_media {
    #[path = "../🖇️replace-reference-media/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🖇️replace-reference-media/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🖇️replace-reference-media/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod replace_references {
    #[path = "../📎replace-references/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📎replace-references/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📎replace-references/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
