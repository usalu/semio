//! 🧮️ Generation3d play app commands command — `flow-eval-tick`: the editor's binding of the
//! surface-neutral chain in `🧵️preview-eval`. The addressing law, the payload shape, the tick core
//! and the tessellate producer all live there, shared verbatim with `👁️viewer`; what is editor-only
//! is what this file keeps — the generate-mode fixture patch and the editor `Emit` type.
#![allow(clippy::result_large_err)]

use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, FRAMEWORK_TOOL_RUN_BODY_KEY};
pub use semio_s_artifact_procedural_3d_preview_eval::FlowEvalTick;



/// 🪟️ The BODY one preview window kind renders — the thing a chain hop's refresh has to name, as
/// opposed to the window kind its payload is addressed by.
pub fn preview_body_key(window_kind_id: &str) -> &'static str {
    if window_kind_id == semio_s_artifact_procedural_3d_mode_generate::windows::preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW {
        semio_s_artifact_procedural_3d_mode_generate::windows::preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW
    } else {
        semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_BODY_PREVIEW
    }
}

/// 🐢️ The editor's binding of [`preview_eval::chain_ui_scope`]: the addressed preview body, the
/// graph body only while this hop moved the computing chrome, and the two panels that read the
/// chain's own progress back (the artifact panel's evaluation status and the framework tool-run
/// panel's pill). The catalogue, the inspection panel, the utilities, tools, engagements, measures
/// and labels are identical before and after every hop of an evaluation.
pub(crate) fn chain_ui_scope(window_kind_id: &str, census_moved: bool) -> UiDirtyScope {
    preview_eval::chain_ui_scope(
        preview_body_key(window_kind_id),
        Some(semio_s_artifact_procedural_3d_mode_edit::windows::flow::GENERATION_3D_PLAY_BODY_MAIN),
        &[semio_s_artifact_procedural_3d_editor_panels::artifact::GENERATION_3D_PLAY_BODY_ARTIFACT, FRAMEWORK_TOOL_RUN_BODY_KEY],
        census_moved,
    )
}

/// 🚧️ Whether the `previewEval` run may start or continue on this graph at all — see
/// [`preview_eval::may_rearm`]. The editor's `pending_effects` asks the SAME question the hop asks
/// before it gives an uncontributed window up.
pub fn may_rearm(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> bool {
    preview_eval::may_rearm(host_snapshot)
}

/// 🧭️ The operator kinds of `host_snapshot` the live registry cannot serve ([`preview_eval::unserved_kinds`]): what a route that owes the previews hands the carrying call so an unservable graph also asks the host for its extensions.
pub fn unserved_kinds(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> Vec<String> {
    preview_eval::unserved_kinds(host_snapshot)
}

/// 🪟️ The one `windowId`/`windowKindId` argument object every hop of the chain carries.
pub fn window_args(window_id: &str, window_kind_id: &str) -> semio_framework_value::DslValue {
    preview_eval::window_args(window_id, window_kind_id)
}

/// 🧮️ One evaluation tick, plus what the calling surface owes its retained preview publication.
///
/// 🧬️ The editor-only half: a tick addressed at the GENERATE preview evaluates the patched
/// generation fixture (`generation_host_snapshot_for`) rather than the document's own, and evaluates
/// nothing at all until a generation is selected.
pub fn evaluate(
    window_id: &str,
    window_kind_id: &str,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    turn_started_us: Option<u64>,
    effect: &mut semio_framework_value::RetainedAllocationEffect<'_>,
    mounted: Option<&mut preview_eval::MountedPreviewTick>,
) -> Result<(Option<(Emit<Generation3dMutation, Generation3dConfigMutation>, FlowEvalPublication)>, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
    let generate = window_kind_id == semio_s_artifact_procedural_3d_mode_generate::windows::preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW;
    let state = doc.snapshot.generation.as_state();
    let selected = if generate {
        let generation = semio_s_artifact_procedural_3d_model::generation_by_id(state, cfg.snapshot.selected_generation_id.as_deref());
        if generation.is_none() {
            session.begin_window_tick(window_id, effect.grant())?;
            session.note_window_tick_outcome(window_id, false, effect.grant())?;
            return Ok((Some((Emit { ui_scope: chain_ui_scope(window_kind_id, false), ..Default::default() }, FlowEvalPublication::Changed(None))), Default::default()));
        }
        generation
    } else {
        None
    };
    let patched = selected.map(|generation| semio_s_artifact_procedural_3d_model::generation_host_snapshot_for(&doc.snapshot.host_snapshot, state, Some(&generation.id)));
    let host_snapshot = patched.as_ref().unwrap_or(&doc.snapshot.host_snapshot);
    let source = if generate { preview_eval::PreviewEvaluationSource::Current } else { preview_eval::PreviewEvaluationSource::WithFallback };
    let project = |seeds: &mut _| {
        if let Some(generation) = selected {
            semio_os_flow::forms_bridge::add_generation_variable_seeds(host_snapshot, |id| generation.values.get(id), seeds);
        }
    };
    let tolerance = preview_eval::preview_tolerance(&cfg.snapshot.lod_mode);
    let outcome = match mounted {
        Some(mounted) => mounted.step(window_id, window_kind_id, host_snapshot, tolerance, session, retained_eval, source, turn_started_us, effect, project),
        None => preview_eval::evaluate_tick_with_seeds(window_id, window_kind_id, host_snapshot, tolerance, session, retained_eval, source, turn_started_us, effect, project).map(|outcome| (Some(outcome), Default::default())),
    };
    if let Some(displaced) = patched {
        displaced.retire_cold();
    }
    let (outcome, progress) = outcome?;
    Ok((outcome.map(|outcome| (
        Emit { extension_invocations: outcome.extension_invocations, ui_scope: chain_ui_scope(window_kind_id, outcome.census_moved), ..Default::default() },
        outcome.publication,
    )), progress))
}

/// 🔁️ The wave a just-folded extension answer unblocked, run INLINE inside that answer's own guest
/// turn — the `flowEvalResolve` → `flowEvalTick` pair this chain used to pay per dependency LEVEL.
///
/// 🪟️ It is the very same [`evaluate`] the dispatched hop runs, publishing into the very same
/// addressed window transient, because the answer routes are window-addressed exactly as the tick
/// is. That is the whole point: intermediate geometry and the monotone status pill are what a user
/// cancels out of, so a continuation that advanced the chain without publishing them would have
/// bought hops by going blind (`📓️flow-tick-coalescing-2026-09-14.md` §7 item 4).
///
/// 🅿️ A refused continuation publishes nothing and touches no latch: the window still owes the hop
/// it owed, and the `previewEval` run job dispatches it on its next step. "Park" is not a state —
/// it is the absence of this call.
pub fn continue_inline(
    window_id: &str,
    window_kind_id: &str,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    turn_started_us: Option<u64>,
    effect: &mut semio_framework_value::RetainedAllocationEffect<'_>,
) -> Result<(Emit<Generation3dMutation, Generation3dConfigMutation>, FlowEvalPublication), Fault> {
    if !session.inline_continuation_admitted(window_id, turn_started_us, semio_framework_job::default_now_us()) {
        return Ok((Emit { ui_scope: chain_ui_scope(window_kind_id, false), ..Default::default() }, FlowEvalPublication::Retained));
    }
    session.arm_window_tick(window_id, effect.grant()).map_err(|error| Fault::from(error.to_string()))?;
    evaluate(window_id, window_kind_id, doc, cfg, session, retained_eval, turn_started_us, effect, None)
        .map_err(|error| Fault::from(error.to_string()))?
        .0.ok_or_else(|| Fault::from("generation3d-inline-preview-did-not-complete"))
}

pub fn handle(_payload: &FlowEvalTick, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Err(Fault::from("generation3d-flow-eval-tick-requires-retained-route"))
}
