//! 📚️ Example `demo`.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🎬️ Materializes this explicit host-graph example into its composed child cache.
pub fn snapshot_from_text(text:&str)->Result<crate::FlowSnapshot,store::TextError>{<semio_framework_artifact_flow_flow::FlowHostSnapshot as store::ArtifactDsl>::parse_dsl(text).map(crate::FlowSnapshot::from_host_snapshot)}
