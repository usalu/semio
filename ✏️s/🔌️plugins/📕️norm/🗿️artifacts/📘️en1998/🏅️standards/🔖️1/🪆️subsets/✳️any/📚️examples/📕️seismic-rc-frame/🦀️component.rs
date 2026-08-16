//! 📚️ Example `seismic-rc-frame`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "seismic-rc-frame";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Seismic Rc Frame", "Seismic Rc Frame")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️seismic-rc-frame.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
