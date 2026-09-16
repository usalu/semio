//! 📚️ Example `demo`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "base-icon";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Default Base Icon", "Standard-Basissymbol")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
