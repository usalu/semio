//! 📚️ Example `nonconforming` — multi-violation manufacturer dataset.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "nonconforming";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Nonconforming Valve Dataset", "Nicht-konformer Ventildatensatz")
}
pub const ICON: &str = "file-warning";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🌶️nonconforming/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
