//! 🧮️ Generation3d play app commands command — `flow-eval-tick`: the editor's binding of the
//! surface-neutral chain in `🧵️preview-eval`. The addressing law, the payload shape, the tick core
//! and the tessellate producer all live there, shared verbatim with `👁️viewer`; what is editor-only
//! is what this file keeps — the generate-mode fixture patch and the editor `Emit` type.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

pub use crate::preview_eval::FlowEvalTick;

/// 🔁️ The self-redispatch every hop of the chain arms, addressed to the SAME preview window.
pub fn rearm(window_id: &str, window_kind_id: &str, req: u64) -> Effect {
    preview_eval::rearm(window_id, window_kind_id, req)
}

/// 🚧️ Whether a chain may be armed for this graph at all — see [`preview_eval::may_rearm`]. The
/// editor's `pending_effects` poll asks the SAME question the tick's own re-arm asks, so an
/// uncontributed graph is not spun from the poll after the tick stopped spinning it itself.
pub fn may_rearm(fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> bool {
    preview_eval::may_rearm(fixture)
}

/// 🪟️ The one `windowId`/`windowKindId` argument object every hop of the chain carries.
pub fn window_args(window_id: &str, window_kind_id: &str) -> dsl::DslValue {
    preview_eval::window_args(window_id, window_kind_id)
}

/// 🧮️ One evaluation tick, plus what the calling surface owes its retained preview publication.
///
/// 🧬️ The editor-only half: a tick addressed at the GENERATE preview evaluates the patched
/// generation fixture (`generation_fixture_for`) rather than the document's own, and evaluates
/// nothing at all until a generation is selected.
pub fn evaluate(
    window_id: &str,
    window_kind_id: &str,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
) -> Result<(Emit<Generation3dMutation, Generation3dConfigMutation>, FlowEvalPublication), Fault> {
    let generate = window_kind_id == crate::editor::generation3d::modes::generate::windows::preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW;
    let mut patched = None;
    if generate {
        let mut state = doc.snapshot.generation.as_state().clone();
        state.selected_generation_id.clone_from(&cfg.snapshot.selected_generation_id);
        if semio_framework_artifact_playbook_playbook::selected_generation(&state).is_none() {
            // 🔒️ This tick RAN — it just had nothing to evaluate. Discharging the window's arming
            // latch here is what lets the next gesture arm a fresh chain; leaving it armed would
            // make a generate preview that opened before any generation exists unarmable forever
            // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
            session.begin_window_tick(window_id);
            session.note_window_tick_outcome(window_id, false);
            return Ok((Emit::default(), session.eval_publication_for(retained_eval)));
        }
        patched = Some(crate::standards::v1::subsets::any::schema::generation_fixture_for(&doc.snapshot.fixture, &state, state.selected_generation_id.as_deref()));
    }
    let fixture = patched.as_ref().unwrap_or(&doc.snapshot.fixture);
    let outcome = preview_eval::evaluate_tick(window_id, window_kind_id, fixture, preview_eval::preview_tolerance(&cfg.snapshot.lod_mode), session, retained_eval);
    if let Some(fixture) = patched {
        fixture.retire_cold();
    }
    Ok((Emit { effects: outcome.effects, extension_invocations: outcome.extension_invocations, ..Default::default() }, outcome.publication))
}

pub fn handle(payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    evaluate(&payload.window_id, &payload.window_kind_id, doc, cfg, session, None).map(|(emit, _)| emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
