//! 📚️ Example `hexagonal-cut-concrete-forest-left`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "hexagonal-cut-concrete-forest-left";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links")
}
pub const ICON: &str = "list-tree";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
