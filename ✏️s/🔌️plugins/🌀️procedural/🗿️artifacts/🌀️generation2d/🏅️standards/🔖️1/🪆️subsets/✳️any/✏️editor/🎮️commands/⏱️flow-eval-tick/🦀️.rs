//! 🧮️ Generation2d play app commands command — `flow-eval-tick`: the editor's binding of the `previewEval`
//! run's hop in `🧵️preview-eval`. The payload shape and the tick core live there; what is editor-only is
//! what this file keeps — which fixture a preview window's target evaluates.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_preview_eval as preview_eval;
use semio_s_artifact_procedural_2d_preview_eval::PreviewEvalTarget;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_s_artifact_procedural_2d_mode_edit::windows::preview as edit_preview;
use semio_s_artifact_procedural_2d_mode_generate::windows::preview as generate_preview;
use semio_s_artifact_procedural_2d_preview_eval::PreviewEvalWindow;
use semio_framework_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_s_artifact_procedural_2d_preview_eval::FlowEvalTick;



/// 🎯️ Which session each preview window kind evaluates — the editor's roster for the `previewEval` run.
const GENERATION2D_PREVIEW_TARGETS: &[(&str, PreviewEvalTarget)] = &[(edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW, PreviewEvalTarget::Document), (generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW, PreviewEvalTarget::Generation)];

/// 🎯️ The target a preview window kind evaluates, or `None` for every other kind.
pub fn generation2d_preview_target(kind: &str) -> Option<PreviewEvalTarget> {
    GENERATION2D_PREVIEW_TARGETS.iter().find(|(candidate, _)| *candidate == kind).map(|(_, target)| *target)
}

/// 🪟️ Every attached preview window, in roster order — the windows the `previewEval` run evaluates.
pub fn generation2d_preview_windows(view: Option<&semio_framework_plugin::ViewModel>) -> Vec<PreviewEvalWindow<'_>> {
    semio_s_artifact_procedural_2d_preview_eval::attached_preview_windows(view, GENERATION2D_PREVIEW_TARGETS)
}

/// 🔢️ The digest of the text `target` evaluates for this document and config — what a hop records and
/// what the editor's poll compares.
pub fn target_digest(target: PreviewEvalTarget, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> u64 {
    let fixture = semio_framework_pack_json::to_json_string(&doc.snapshot.host_snapshot);
    match target {
        PreviewEvalTarget::Document => preview_eval::preview_eval_digest(&[&fixture]),
        PreviewEvalTarget::Generation => preview_eval::preview_eval_digest(&[&fixture, &semio_framework_pack_json::to_json_string(doc.snapshot.generation.as_state()), cfg.snapshot.selected_generation_id.as_deref().unwrap_or_default()]),
    }
}

/// 🧮️ One evaluation hop of `target` into `session`, plus what it owes the target's publication. The
/// document target evaluates the document; the generation target evaluates the selected generation's
/// patched fixture, or clears its evaluation while none is selected.
pub fn evaluate(window_id: &str, window_kind_id: &str, target: PreviewEvalTarget, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<(Emit<Generation2dMutation, Generation2dConfigMutation>, FlowEvalPublication), Fault> {
    let retained_grant=doc.retained_grant()?;
    let retained = session.eval_json().to_string();
    let retained = (!retained.is_empty()).then_some(retained);
    let outcome = match target {
        PreviewEvalTarget::Document => preview_eval::evaluate_tick(window_id, window_kind_id, &doc.snapshot.host_snapshot, session, retained.as_deref(), retained_grant).map_err(|error| Fault::from(error.to_string()))?,
        PreviewEvalTarget::Generation => {
            let mut state = doc.snapshot.generation.as_state().clone();
            state.selected_generation_id.clone_from(&cfg.snapshot.selected_generation_id);
            let Some(values) = semio_framework_artifact_playbook_playbook::selected_generation(&state).map(|selected| selected.values.clone()) else {
                preview_eval::settle_empty_tick(window_id, session, retained_grant).map_err(|error| Fault::from(error.to_string()))?;
                return Ok((Emit::default(), session.eval_publication_for(retained.as_deref())));
            };
            let host = semio_s_artifact_procedural_2d_io_text::snapshot::generation_preview_host(&doc.snapshot.host_snapshot, &values);
            let outcome = preview_eval::evaluate_tick(window_id, window_kind_id, &host.host_snapshot, session, retained.as_deref(), retained_grant);
            host.retire_cold();
            outcome.map_err(|error| Fault::from(error.to_string()))?
        }
    };
    Ok((Emit { extension_invocations: outcome.extension_invocations, ..Default::default() }, outcome.publication))
}

/// 🧩️ The `app_commands!` row, reached only by the marks-free `handle` fallback, which owns no preview
/// publication: it evaluates the target the payload's kind names into the session it is handed.
pub fn handle(payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let target = crate::flow_eval_tick::generation2d_preview_target(&payload.window_kind_id).ok_or_else(|| Fault::from("generation2d-flow-eval-window-kind-unknown"))?;
    Ok(evaluate(&payload.window_id, &payload.window_kind_id, target, doc, cfg, session)?.0)
}
