//! 🧮️ Generation3d play app commands command — `flow-eval-tick`: the editor's binding of the
//! surface-neutral chain in `🧵️preview-eval`. The addressing law, the payload shape, the tick core
//! and the tessellate producer all live there, shared verbatim with `👁️viewer`; what is editor-only
//! is what this file keeps — the generate-mode fixture patch and the editor `Emit` type.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::FlowEvalTick;

/// 🚧️ Whether the `previewEval` run may start or continue on this graph at all — see
/// [`preview_eval::may_rearm`]. The editor's `pending_effects` asks the SAME question the hop asks
/// before it gives an uncontributed window up.
pub fn may_rearm(fixture: &semio_framework_artifact_flow_flow::FlowHostDocument) -> bool {
    preview_eval::may_rearm(fixture)
}

/// 🪟️ The one `windowId`/`windowKindId` argument object every hop of the chain carries.
pub fn window_args(window_id: &str, window_kind_id: &str) -> dsl::DslValue {
    preview_eval::window_args(window_id, window_kind_id)
}

/// 🧮️ One evaluation tick, plus what the calling surface owes its retained preview publication.
///
/// 🧬️ The editor-only half: a tick addressed at the GENERATE preview evaluates the patched
/// generation fixture (`generation_host_document_for`) rather than the document's own, and evaluates
/// nothing at all until a generation is selected.
pub fn evaluate(
    window_id: &str,
    window_kind_id: &str,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    turn_started_us: Option<u64>,
) -> Result<(Emit<Generation3dMutation, Generation3dConfigMutation>, FlowEvalPublication), Fault> {
    let generate = window_kind_id == crate::editor::generation3d::modes::generate::windows::preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW;
    let mut patched = None;
    if generate {
        let mut state = doc.snapshot.generation.as_state().clone();
        state.selected_generation_id.clone_from(&cfg.snapshot.selected_generation_id);
        if semio_framework_artifact_playbook_playbook::selected_generation(&state).is_none() {
            // 🔒️ This tick RAN — it just had nothing to evaluate. Discharging the window's latch here
            // is what lets the run settle and a later gesture owe a fresh evaluation; leaving it armed
            // would make a generate preview that opened before any generation exists wait forever
            // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
            session.begin_window_tick(window_id);
            session.note_window_tick_outcome(window_id, false);
            return Ok((Emit::default(), session.eval_publication_for(retained_eval)));
        }
        patched = Some(crate::standards::v1::subsets::any::schema::generation_host_document_for(&doc.snapshot.host_document, &state, state.selected_generation_id.as_deref()));
    }
    let fixture = patched.as_ref().unwrap_or(&doc.snapshot.host_document);
    let outcome = preview_eval::evaluate_tick(window_id, window_kind_id, fixture, preview_eval::preview_tolerance(&cfg.snapshot.lod_mode), session, retained_eval, turn_started_us);
    if let Some(fixture) = patched {
        fixture.retire_cold();
    }
    Ok((Emit { extension_invocations: outcome.extension_invocations, ..Default::default() }, outcome.publication))
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
) -> Result<(Emit<Generation3dMutation, Generation3dConfigMutation>, FlowEvalPublication), Fault> {
    if !session.inline_continuation_admitted(window_id, turn_started_us, semio_framework_job::default_now_us()) {
        return Ok((Emit::default(), FlowEvalPublication::Retained));
    }
    session.arm_window_tick(window_id);
    evaluate(window_id, window_kind_id, doc, cfg, session, retained_eval, turn_started_us)
}

pub fn handle(payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    evaluate(&payload.window_id, &payload.window_kind_id, doc, cfg, session, None, None).map(|(emit, _)| emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
