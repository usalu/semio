//! 📚️ Example `retail-hydrocarbon-fire`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "retail-hydrocarbon-fire";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Retail Hydrocarbon Fire", "Retail Hydrocarbon Fire") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
