//! 📚️ Example `high-strength-connection` — overloaded non-compliant steel frame.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "high-strength-connection";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Overloaded Steel Frame", "Überlasteter Stahlrahmen")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
