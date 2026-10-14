//! 📚️ Example `hexagonal-cut-concrete-forest-left`.

extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod art_lowpoly_hexagonal_cut_concrete_forest_left_tests;
//#endregion 🧩️Modules

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "hexagonal-cut-concrete-forest-left";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
