//! 📚️ Example `concrete-forest` — the reused hexagonal-cut concrete forest piece processed by every
//! machine of the concrete catalog.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "concrete-forest";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Concrete Forest", "Betonwald")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
