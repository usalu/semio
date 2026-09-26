//! 📚️ Example `demo` — conforming manufacturer catalogue.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Conforming catalogue", "Konformer Katalog")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/📚️example/🦀️.rs"]
mod example;
