//! 📚️ Example non-compliant EN 1995-2 footbridge girder — dense crowd, low damping and a steep fatigue curve.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "overloaded-footbridge";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Non-Compliant Overloaded Footbridge", "Nicht nachweisbare überlastete Fußgängerbrücke")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/⚠️overloaded-footbridge/⚠️overloaded-footbridge/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
