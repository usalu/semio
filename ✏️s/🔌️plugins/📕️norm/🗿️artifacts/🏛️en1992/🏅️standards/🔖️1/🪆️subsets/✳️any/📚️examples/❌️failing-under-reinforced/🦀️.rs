//! 📚️ Example `failing-under-reinforced`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "failing-under-reinforced";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Failing Under-Reinforced", "Unterbewehrtes Versagensbeispiel")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/❌️failing-under-reinforced/❌️failing-under-reinforced/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
