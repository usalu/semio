//! 📚️ Example `failing-thin-insulation`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "failing-thin-insulation";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Failing thin insulation", "Versagende Dämmung")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️failing-thin-insulation/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example_tests;
