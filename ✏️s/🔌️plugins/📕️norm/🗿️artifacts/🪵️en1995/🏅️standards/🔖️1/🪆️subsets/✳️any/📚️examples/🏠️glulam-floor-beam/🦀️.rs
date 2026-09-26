//! 📚️ Example compliant glulam floor beam with a bolted support connection (EN 1995-1-1, German annex).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "glulam-floor-beam";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Compliant Glulam Floor Beam", "Nachweisfähiger BSH-Deckenträger")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
