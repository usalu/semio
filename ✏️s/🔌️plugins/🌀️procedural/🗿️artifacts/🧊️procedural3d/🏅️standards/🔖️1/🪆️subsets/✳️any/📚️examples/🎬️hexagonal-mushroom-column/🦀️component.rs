//! 📚️ Example `hexagonal-mushroom-column`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "hexagonal-mushroom-column";
pub async fn label() -> LocalizedLabel { LocalizedLabel::native("Hexagonal Mushroom Column", "Hexagonal Mushroom Column") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️hexagonal-mushroom-column.dsl.semio");
pub async fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
