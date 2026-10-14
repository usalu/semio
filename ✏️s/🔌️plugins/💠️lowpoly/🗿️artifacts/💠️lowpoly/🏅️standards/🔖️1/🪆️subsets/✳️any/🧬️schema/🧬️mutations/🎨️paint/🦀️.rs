//! 🎨️ Lowpoly paint-layer mutations: insert, remove, edit, rename, restyle layers and apply strokes.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

//#region 🧩️Modules
#[path = "."]
pub mod insert_paint_layer {
    #[path = "../➕️insert-paint-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../➕️insert-paint-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../➕️insert-paint-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod remove_paint_layer {
    #[path = "../➖️remove-paint-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../➖️remove-paint-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../➖️remove-paint-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod rename_paint_layer {
    #[path = "../🔖️rename-paint-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔖️rename-paint-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔖️rename-paint-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_paint_layer_visible {
    #[path = "../👁️change-paint-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../👁️change-paint-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../👁️change-paint-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_paint_layer_opacity {
    #[path = "../🌫️change-paint-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🌫️change-paint-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌫️change-paint-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_paint_layer_blend_mode {
    #[path = "../🎛️change-paint-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🎛️change-paint-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎛️change-paint-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod edit_paint_layer {
    #[path = "../🎨️edit-paint-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🎨️edit-paint-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎨️edit-paint-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod apply_paint_stroke {
    #[path = "../🖌️apply-paint-stroke/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🖌️apply-paint-stroke/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🖌️apply-paint-stroke/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
