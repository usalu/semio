use super::{WriterMainWindowTransient, WriterMainWindowTransientDiff, WriterOptionalSelection};

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
#[mutations(snapshot = WriterMainWindowTransient, diff = WriterMainWindowTransientDiff, schema = "writer.mainwindowtransient")]
pub enum WriterMainWindowTransientMutation {
    #[dsl(key = "set-editor-selection")]
    SetEditorSelection(SetEditorSelection),
    #[dsl(key = "set-lint-generation")]
    SetLintGeneration(SetLintGeneration),
    #[dsl(key = "set-engagement-input")]
    SetEngagementInput(SetEngagementInput),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
