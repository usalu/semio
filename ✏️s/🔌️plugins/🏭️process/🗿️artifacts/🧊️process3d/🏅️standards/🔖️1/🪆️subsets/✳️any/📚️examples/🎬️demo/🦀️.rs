//! 📚️ Example `demo`.

extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod art_process3d_demo_tests;
#[cfg(test)]
#[path = "../🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs"]
mod art_process3d_concrete_forest_tests;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod concrete_forest {
        #[path = "../🌲️concrete-forest/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
//#endregion 🧩️Modules

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
