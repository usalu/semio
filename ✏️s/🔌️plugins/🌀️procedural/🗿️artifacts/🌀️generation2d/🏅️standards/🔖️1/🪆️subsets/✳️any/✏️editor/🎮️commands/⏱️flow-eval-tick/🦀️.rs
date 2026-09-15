//! 🧮️ Generation2d play app commands command — `flow-eval-tick`: the editor's binding of the `previewEval`
//! run's hop in `🧵️preview-eval`. The payload shape and the tick core live there; what is editor-only is
//! what this file keeps — which fixture a preview window's target evaluates.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::preview_eval::{self, PreviewEvalTarget};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::FlowEvalTick;

/// 🔢️ The digest of the text `target` evaluates for this document and config — what a hop records and
/// what the editor's poll compares.
pub fn target_digest(target: PreviewEvalTarget, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> u64 {
    let fixture = dsl::json::to_json_string(&doc.snapshot.host_document);
    match target {
        PreviewEvalTarget::Document => preview_eval::preview_eval_digest(&[&fixture]),
        PreviewEvalTarget::Generation => preview_eval::preview_eval_digest(&[&fixture, &dsl::json::to_json_string(doc.snapshot.generation.as_state()), cfg.snapshot.selected_generation_id.as_deref().unwrap_or_default()]),
    }
}

/// 🧮️ One evaluation hop of `target` into `session`, plus what it owes the target's publication. The
/// document target evaluates the document; the generation target evaluates the selected generation's
/// patched fixture, or clears its evaluation while none is selected.
pub fn evaluate(window_id: &str, window_kind_id: &str, target: PreviewEvalTarget, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> (Emit<Generation2dMutation, Generation2dConfigMutation>, FlowEvalPublication) {
    let retained = session.eval_json().to_string();
    let retained = (!retained.is_empty()).then_some(retained);
    let outcome = match target {
        PreviewEvalTarget::Document => preview_eval::evaluate_tick(window_id, window_kind_id, &doc.snapshot.host_document, session, retained.as_deref()),
        PreviewEvalTarget::Generation => {
            let mut state = doc.snapshot.generation.as_state().clone();
            state.selected_generation_id.clone_from(&cfg.snapshot.selected_generation_id);
            let Some(values) = semio_framework_artifact_playbook_playbook::selected_generation(&state).map(|selected| selected.values.clone()) else {
                preview_eval::settle_empty_tick(window_id, session);
                return (Emit::default(), session.eval_publication_for(retained.as_deref()));
            };
            let host = crate::standards::v1::subsets::any::schema::generation_preview_host(&doc.snapshot.host_document, &values);
            let outcome = preview_eval::evaluate_tick(window_id, window_kind_id, &host.host_document, session, retained.as_deref());
            host.retire_cold();
            outcome
        }
    };
    (Emit { extension_invocations: outcome.extension_invocations, ..Default::default() }, outcome.publication)
}

/// 🧩️ The `app_commands!` row, reached only by the marks-free `handle` fallback, which owns no preview
/// publication: it evaluates the target the payload's kind names into the session it is handed.
pub fn handle(payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let target = crate::editor::generation2d::generation2d_preview_target(&payload.window_kind_id).ok_or_else(|| Fault::from("generation2d-flow-eval-window-kind-unknown"))?;
    Ok(evaluate(&payload.window_id, &payload.window_kind_id, target, doc, cfg, session).0)
}
