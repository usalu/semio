//! 📚️ Example compliant EN 1995-2 glulam pedestrian footbridge girder (fatigue, pedestrian comfort, ULS, SLS).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "glulam-footbridge";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Compliant Glulam Footbridge", "Nachweisfähige BSH-Fußgängerbrücke")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🌉️glulam-footbridge/🌉️glulam-footbridge/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
