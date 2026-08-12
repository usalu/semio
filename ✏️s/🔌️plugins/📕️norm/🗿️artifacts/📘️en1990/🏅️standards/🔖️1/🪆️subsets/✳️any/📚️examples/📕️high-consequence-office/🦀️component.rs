//! 📚️ Example `high-consequence-office`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "high-consequence-office";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("High Consequence Office", "High Consequence Office") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️high-consequence-office.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
