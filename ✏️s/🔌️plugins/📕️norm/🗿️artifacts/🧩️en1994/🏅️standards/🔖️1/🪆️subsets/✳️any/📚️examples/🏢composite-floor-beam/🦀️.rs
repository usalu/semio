//! 📚️ Example `composite-floor-beam`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "composite-floor-beam";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Composite Floor Beam", "Verbundträger Geschossdecke")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢composite-floor-beam/🏢composite-floor-beam/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
