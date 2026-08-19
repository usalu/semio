//! 📚️ Example `sphere-cut-with-torus`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "sphere-cut-with-torus";
pub async fn label() -> LocalizedLabel { LocalizedLabel::native("Sphere Cut With Torus", "Sphere Cut With Torus") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️sphere-cut-with-torus.dsl.semio");
pub async fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
