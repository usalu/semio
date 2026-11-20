//! 🔗️ A Layout document whose linked image preview is embedded in the snapshot.

use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;

pub const ID: &str = "linked-asset";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🔗️linked-asset/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, LocalizedLabel::native("Linked Asset", "Verknüpftes Bild"), PRIMARY_TEXT, "image")
}
