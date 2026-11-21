//! 📚️ Example `demo`.

use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub fn source() -> ExampleSource {
    static PRIMARY_TEXT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
        let document = crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document();
        <crate::GisTerrainSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&document)
    });
    ExampleSource::new(ID, label(), PRIMARY_TEXT.as_str(), ICON)
}
