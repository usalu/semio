//! 🧮️ 🧮️ Generation3d play app commands command — `flow-eval-tick`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{flow_host_with_session, FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, ExtensionInvocation, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪟️ The evaluation tick names the preview window that OWNS the evaluation it advances.
///
/// The tick publishes into one window's retained transient (`Generation3dPreviewWindowTransientOwner`),
/// so the retained route is window-scoped and its work refuses any command that does not name that
/// window. The chain is entirely self-dispatched — `Generation3dPlayApp::pending_effects` arms the
/// first tick and every tick/resolve re-arms the next through `Effect::DispatchAction` — and an
/// effect carries no window of its own: the shell redispatches it under whichever window is current
/// (the flow window `procedural-main` in the served app). Carrying the id ON THE PAYLOAD is how
/// `retained_window_transient_target` can capture the preview window's transient authority, the same
/// way `TrinityJackCommand::RunQuery` carries its `results_window_id`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {
    pub window_id: String,
}

/// 🔁️ The self-redispatch every hop of the chain arms, addressed to the SAME preview window.
pub fn rearm(window_id: &str, req: u64) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(req), action: "flowEvalTick".into(), args: Some(window_args(window_id)), delay_ms: 0 }
}

/// 🪟️ The one `windowId` argument object every hop of the chain carries, on the redispatch and on
/// the extension request alike — `reactor::extension_response_args` echoes an invocation request's
/// own fields back onto the response action, so the window address survives the round trip without
/// the SDK ever learning what it means.
pub fn window_args(window_id: &str) -> dsl::DslValue {
    dsl::DslValue::object([("windowId".to_string(), dsl::DslValue::String(window_id.to_string()))])
}

/// 🧮️ One evaluation tick, plus what the calling surface owes its retained preview publication.
///
/// ⏱️ The tick's own wall cost is recorded into `semio_framework_os_flow`'s evaluation-step ledger
/// ONLY while `semio_framework_job::runtime_diagnostics_enabled()` is armed — this is the ONE app
/// work step the 8 ms interactive ceiling governs, and a normal boot must not pay two clock reads
/// per tick to measure it.
///
/// 📤️ `retained_eval` is the evaluation that surface ALREADY holds — the tick republishes only when it
/// differs, so the redispatch ticks that move no node allocate and retire nothing.
pub fn evaluate(window_id: &str, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession, retained_eval: Option<&str>) -> Result<(Emit<Generation3dMutation, Generation3dConfigMutation>, FlowEvalPublication), Fault> {
    let started_us = semio_framework_job::runtime_diagnostics_enabled().then(semio_framework_job::default_now_us).flatten();
    let fixture = &doc.snapshot.fixture;
    let mut host = flow_host_with_session(fixture, session);
    let more = session.tick(&mut host);
    let effects = if more { vec![rearm(window_id, 103)] } else { Vec::new() };
    let pending_extension_eval = host.take_pending_extension_eval();
    // 🧹️ The host's cloned fixture owns retirement-tracked ordered maps and is dead from here on —
    // close it before the invocation build, never leave it to drop glue.
    host.retire_cold();
    let mut extension_invocations = Vec::new();
    if let Some(pending) = pending_extension_eval {
        let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
            ("operatorId".to_string(), dsl::DslValue::String(pending.operator_id.clone())),
            ("inputJson".to_string(), dsl::DslValue::String(pending.input_json.clone())),
            ("nodeHash".to_string(), dsl::DslValue::uint(pending.node_hash)),
            ("windowId".to_string(), dsl::DslValue::String(window_id.to_string())),
        ]));
        extension_invocations.push(ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve"));
    } else if !more {
        extension_invocations.extend(crate::editor::generation3d::preview_tessellate_invocations(window_id, session, fixture, cfg.snapshot));
    }
    let publication = session.eval_publication_for(retained_eval);
    if let (Some(started_us), Some(finished_us)) = (started_us, started_us.and_then(|_| semio_framework_job::default_now_us())) {
        semio_framework_os_flow::record_flow_eval_step(finished_us.saturating_sub(started_us));
    }
    Ok((Emit { effects, extension_invocations, ..Default::default() }, publication))
}

pub fn handle(payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    evaluate(&payload.window_id, doc, cfg, session, None).map(|(emit, _)| emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
