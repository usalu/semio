//! 📚️ Example `demo` — compliant ETICS dwelling.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Compliant ETICS dwelling", "Regelgerechtes WDVS-Wohnhaus")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example_tests;
