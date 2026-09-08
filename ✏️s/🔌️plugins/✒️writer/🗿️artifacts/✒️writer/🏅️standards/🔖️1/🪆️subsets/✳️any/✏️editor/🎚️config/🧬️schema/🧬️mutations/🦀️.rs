//! 🧬️ Writer config mutation collection.

use super::*;
#[path = "📸️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "📐️set-editor-selection/🦀️.rs"]
mod set_editor_selection;
pub use set_editor_selection::SetEditorSelection;
#[path = "🔔️set-format-signal/🦀️.rs"]
mod set_format_signal;
pub use set_format_signal::SetFormatSignal;
#[path = "🔍️set-lint-signal/🦀️.rs"]
mod set_lint_signal;
pub use set_lint_signal::SetLintSignal;
#[path = "🔢️set-revision/🦀️.rs"]
mod set_revision;
pub use set_revision::SetRevision;
#[path = "⚙️set-editor-settings/🦀️.rs"]
mod set_editor_settings;
pub use set_editor_settings::SetEditorSettings;
#[path = "💬️set-engagement-input/🦀️.rs"]
mod set_engagement_input;
pub use set_engagement_input::SetEngagementInput;
#[path = "📷️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "🗣️set-locale/🦀️.rs"]
mod set_locale;
pub use set_locale::SetLocale;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = WriterConfig, diff = WriterConfig, schema = "writer.config")]
pub enum WriterConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-editor-selection")]
    SetEditorSelection(SetEditorSelection),
    #[dsl(key = "set-format-signal")]
    SetFormatSignal(SetFormatSignal),
    #[dsl(key = "set-lint-signal")]
    SetLintSignal(SetLintSignal),
    #[dsl(key = "set-revision")]
    SetRevision(SetRevision),
    #[dsl(key = "set-editor-settings")]
    SetEditorSettings(SetEditorSettings),
    #[dsl(key = "set-engagement-input")]
    SetEngagementInput(SetEngagementInput),
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-locale")]
    SetLocale(SetLocale),
}

impl protocol::OpText for WriterConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(&keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec = variants.iter().find(|(key, _)| key == &keyword).expect("declared variant").1();
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for WriterConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}
