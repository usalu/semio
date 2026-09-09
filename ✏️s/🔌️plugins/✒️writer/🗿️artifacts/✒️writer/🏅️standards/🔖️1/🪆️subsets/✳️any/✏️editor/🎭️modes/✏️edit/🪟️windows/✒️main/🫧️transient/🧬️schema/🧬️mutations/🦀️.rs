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

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
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
