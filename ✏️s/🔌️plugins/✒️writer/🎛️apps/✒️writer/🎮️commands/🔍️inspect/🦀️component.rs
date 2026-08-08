//! 🔍️ Writer play app commands — completions request and lint pass. Both are config-only View
//! commands: they bump a signal so the renderer recomputes, never a document operation.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️RequestCompletions
pub mod request_completions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "request-completions")]
    pub struct RequestCompletions {}

    pub fn handle(_payload: &RequestCompletions, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        let config = cfg.projection;
        Ok(Emit::config(vec![WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
    }
}
//#endregion 🔖️RequestCompletions

//#region 🔖️LintDocument
pub mod lint_document {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "lint-document")]
    pub struct LintDocument {}

    pub fn handle(_payload: &LintDocument, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        let config = cfg.projection;
        Ok(Emit::config(vec![WriterConfigMutation::SetLintSignal { value: config.lint_signal + 1 }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
    }
}
//#endregion 🔖️LintDocument

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::lint_document;
    use crate::apps::writer::commands::text::set_active_example;
    use crate::apps::writer::testkit::new_app_with_registry;
    use crate::apps::writer::WriterCommand;

    #[test]
    fn lint_is_a_view_action_and_example_default_materializes() {
        let mut app = new_app_with_registry();
        // lintDocument is a declared View action: registry kind discipline requires it emit no operations.
        let result = app.dispatch_typed(WriterCommand::LintDocument(lint_document::LintDocument {}), &semio_framework_plugin::testkit::meta("local")).expect("lint");
        assert!(result.document_mutations.is_empty(), "lint re-runs diagnostics into runtime, never the document");
        // setActiveExample fired with the declared default example ("jack").
        app.dispatch_typed(WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }), &semio_framework_plugin::testkit::meta("local")).expect("example");
        assert!(!app.projection().expect("projection").text.is_empty(), "jack default materialized from the registry");
    }
}
//#endregion 🧪️Tests
