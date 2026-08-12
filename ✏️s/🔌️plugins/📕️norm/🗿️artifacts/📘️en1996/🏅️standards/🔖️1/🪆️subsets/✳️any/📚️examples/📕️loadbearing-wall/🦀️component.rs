//! 📚️ Example `loadbearing-wall`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "loadbearing-wall";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Loadbearing Wall", "Loadbearing Wall") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️loadbearing-wall.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
