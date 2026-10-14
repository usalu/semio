//! 📚️ Example `noncompliant-office`.
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "noncompliant-office";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Non-compliant office", "Nicht konformes Büro")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/⚠️noncompliant-office/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
pub fn snapshot() -> Din16798Snapshot {
    Din16798Snapshot::noncompliant_office()
}
