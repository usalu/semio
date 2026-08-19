//! 📚️ Example `liquid-retaining-fem-anchor`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "liquid-retaining-fem-anchor";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Liquid Retaining Fem Anchor", "Liquid Retaining Fem Anchor")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️liquid-retaining-fem-anchor.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
