//! 📚️ Example `aluminium-roof-purlin`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "aluminium-roof-purlin";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Aluminium Roof Purlin", "Aluminium Roof Purlin") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️aluminium-roof-purlin.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
