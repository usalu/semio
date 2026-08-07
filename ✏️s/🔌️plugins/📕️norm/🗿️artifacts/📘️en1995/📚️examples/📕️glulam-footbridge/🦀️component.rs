//! 📚️ Example `glulam-footbridge`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "glulam-footbridge";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Glulam Footbridge", "Glulam Footbridge") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️glulam-footbridge.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
