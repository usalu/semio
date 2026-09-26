//! 📚️ Example `composite-floor-beam-failing`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "composite-floor-beam-failing";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Failing Composite Floor Beam", "Versagender Verbundträger Geschossdecke")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢composite-floor-beam-failing/🏢composite-floor-beam-failing/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
