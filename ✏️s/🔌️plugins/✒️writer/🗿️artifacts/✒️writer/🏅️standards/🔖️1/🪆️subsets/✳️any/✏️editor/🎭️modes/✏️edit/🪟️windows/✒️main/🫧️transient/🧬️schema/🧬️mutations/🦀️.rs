use super::WriterMainWindowTransient;

#[path = "📐️set-editor-selection/🦀️.rs"]
mod set_editor_selection;
pub use set_editor_selection::SetEditorSelection;
#[path = "🔍️set-lint-generation/🦀️.rs"]
mod set_lint_generation;
pub use set_lint_generation::SetLintGeneration;
#[path = "💬️set-engagement-input/🦀️.rs"]
mod set_engagement_input;
pub use set_engagement_input::SetEngagementInput;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = WriterMainWindowTransient, diff = WriterMainWindowTransient, schema = "writer.mainwindowtransient")]
pub enum WriterMainWindowTransientMutation {
    #[dsl(key = "set-editor-selection")]
    SetEditorSelection(SetEditorSelection),
    #[dsl(key = "set-lint-generation")]
    SetLintGeneration(SetLintGeneration),
    #[dsl(key = "set-engagement-input")]
    SetEngagementInput(SetEngagementInput),
}

impl protocol::OpText for WriterMainWindowTransientMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for WriterMainWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `WriterMainWindowTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn writer_main_window_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<WriterMainWindowTransient, WriterMainWindowTransientMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
