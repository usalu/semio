//! 🔍️ 🔍️ Writer play app commands command — `lint-document`.

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "lint-document")]
pub struct LintDocument {}

pub async fn handle(_payload: &LintDocument, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    Ok(Emit::config(vec![WriterConfigMutation::SetLintSignal { value: config.lint_signal + 1 }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::LintDocument;
        use crate::editor::writer::commands::set_active_example;
    use crate::editor::writer::testkit::new_app_with_registry;
    use crate::editor::writer::WriterCommand;
    use crate::artifacts::writer::{writer_text, WriterSnapshot};
    use semio_framework::kernel::Effect;

    #[semio_framework_async_macros::async_test]
    async fn lint_is_a_view_action_and_example_default_materializes() {
        let mut app = new_app_with_registry();
        // lintDocument is a declared View action: registry kind discipline requires it emit no operations.
        let result = app.dispatch_typed(WriterCommand::LintDocument(LintDocument {}), &semio_framework_plugin::testkit::meta("local")).expect("lint");
        assert!(result.mutations.is_empty(), "lint re-runs diagnostics into runtime, never the document");
        // setActiveExample fired with the declared default example ("jack") — whole-document replace
        // is not an in-history mutation (`SetSnapshot` is banned outright), so this surfaces as a
        // `Effect::LoadDocument`, not a live `app.snapshot()` change (see `reset_document_effect`).
        let result = app.dispatch_typed(WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }), &semio_framework_plugin::testkit::meta("local")).expect("example");
        let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("expected a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let projection = <WriterSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(!writer_text(&projection).is_empty(), "jack default materialized from the registry");
    }
}
//#endregion 🧪️Tests
