//! 🤝️ CAD play app commands — the engagement REPL: input, submit, keyed transitions, abort, and the two world-pointer events
//! that drive a live construction interaction. Every step is per-frame state of the addressed world window — its transient,
//! never config, never history (design §17.4 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); only a commit lands, as
//! ONE transform-tool transaction.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::engine::interaction::{apply_event, inject_selection};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::modes::edit::tools::transform::{cad_transform_tool_emit, CadToolEntry};
use crate::editor::cad::{cad_pane_id_from_suffix, engagement_submit_entries, publish_engagement, runtime_of, start_interaction_session, try_commit_session_entries, CadPlayRuntime};
use crate::op::CadMutation;
use crate::CadPaneId;
use crate::CadSnapshot;
use semio_framework_value::DslValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧵️ The emit every engagement command ends with: the step publishes the runtime's engagement state to the addressed
/// window's transient, and a commit is ONE transform-tool transaction — the interaction's leaves as one document edit
/// stamped with its `TransactionRef` (tool `<appId>#<interaction id>`).
fn engagement_emit(doc: &ArtifactView<'_, CadSnapshot>, runtime: &CadPlayRuntime, entries: Vec<CadToolEntry>, ctx: &mut CadDispatchCtx) -> Emit<CadMutation, CadConfigMutation> {
    publish_engagement(runtime, ctx);
    if entries.is_empty() {
        return Emit::default();
    }
    cad_transform_tool_emit(doc, runtime.last_finalized_interaction_id.as_deref().unwrap_or("engagement"), entries)
}

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {
        pub pane: Option<String>,
    }

    pub fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg, &ctx.window_transient);
        let pane_id = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_suffix);
        if let Some(session) = runtime.engagement_session.as_mut() {
            inject_selection(session, &ctx.interaction.ids);
        }
        let entries = engagement_submit_entries(doc.snapshot, &mut runtime, pane_id);
        Ok(engagement_emit(doc, &runtime, entries, ctx))
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
        pub pane: Option<String>,
    }

    pub fn handle(payload: &EngagementInput, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg, &ctx.window_transient);
        runtime.engagement_input = payload.value.clone();
        runtime.engagement_pane = payload.pane.clone();
        Ok(engagement_emit(doc, &runtime, Vec::new(), ctx))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementPossibleSelect
pub mod engagement_possible_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "engagement-possible-select")]
    pub struct EngagementPossibleSelect {
        pub pane: Option<String>,
        pub possible_id: String,
    }

    pub fn handle(payload: &EngagementPossibleSelect, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg, &ctx.window_transient);
        let pane_id = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_suffix);
        // 🎯️ A keyed transition (`confirm`, `close`, …) may be guarded on the selection the user made
        // in the viewport — feed the live `"cad"` ids in first, then apply the transition.
        let step = runtime.engagement_session.as_mut().and_then(|session| {
            inject_selection(session, &ctx.interaction.ids);
            apply_event(session, &payload.possible_id, None).then(|| (session.state.clone(), session.clone()))
        });
        if let Some((step, snapshot)) = step {
            runtime.engagement_step = step;
            let entries = try_commit_session_entries(doc.snapshot, &mut runtime, pane_id, &snapshot);
            return Ok(engagement_emit(doc, &runtime, entries, ctx));
        }
        if start_interaction_session(&mut runtime, pane_id, &payload.possible_id) {
            if let Some(session) = runtime.engagement_session.as_mut() {
                inject_selection(session, &ctx.interaction.ids);
                runtime.engagement_step = session.state.clone();
            }
        } else {
            runtime.engagement_input = payload.possible_id.clone();
        }
        Ok(engagement_emit(doc, &runtime, Vec::new(), ctx))
    }
}
//#endregion 🔖️EngagementPossibleSelect

//#region 🔖️EngagementRepeatLast
pub mod engagement_repeat_last {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "engagement-repeat-last")]
    pub struct EngagementRepeatLast {
        pub pane: Option<String>,
    }

    pub fn handle(payload: &EngagementRepeatLast, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg, &ctx.window_transient);
        let pane_id = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_suffix);
        if runtime.engagement_session.is_none() {
            if let Some(interaction_id) = runtime.last_finalized_interaction_id.clone() {
                start_interaction_session(&mut runtime, pane_id, &interaction_id);
                return Ok(engagement_emit(doc, &runtime, Vec::new(), ctx));
            }
        }
        runtime.engagement_step = "Idle".into();
        Ok(engagement_emit(doc, &runtime, Vec::new(), ctx))
    }
}
//#endregion 🔖️EngagementRepeatLast

//#region 🔖️EngagementAbort
pub mod engagement_abort {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "engagement-abort")]
    pub struct EngagementAbort {}

    pub fn handle(_payload: &EngagementAbort, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg, &ctx.window_transient);
        runtime.engagement_input.clear();
        runtime.engagement_session = None;
        runtime.engagement_step = "Idle".into();
        Ok(engagement_emit(doc, &runtime, Vec::new(), ctx))
    }
}
//#endregion 🔖️EngagementAbort

//#region 🔖️WorldPointerDown
pub mod world_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "world-pointer-down")]
    pub struct WorldPointerDown {
        pub pane: Option<String>,
        pub surface_id: Option<String>,
        pub x: Option<f64>,
        pub y: Option<f64>,
        pub z: Option<f64>,
    }

    pub fn handle(payload: &WorldPointerDown, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg, &ctx.window_transient);
        let pane_id = payload.pane.as_deref().map(cad_pane_id_from_suffix).or_else(|| payload.surface_id.as_deref().and_then(|surface_id| surface_id.rsplit('/').next()).map(cad_pane_id_from_suffix)).unwrap_or(CadPaneId::Shape);
        // 📍️ `apply_event`'s payload for a pointer event is the raw position value itself
        // (mirrors the pre-B1 `args.get("position")` extraction — NOT re-wrapped in another
        // `{"position": ...}` object).
        let point_value =
            (payload.x.is_some() || payload.y.is_some() || payload.z.is_some()).then(|| semio_framework_value::DslValue::Array(vec![DslValue::float(payload.x.unwrap_or(0.0)), DslValue::float(payload.y.unwrap_or(0.0)), DslValue::float(payload.z.unwrap_or(0.0))]));
        let commit = runtime.engagement_session.as_mut().and_then(|session| {
            inject_selection(session, &ctx.interaction.ids);
            apply_event(session, "pointer.down", point_value.as_ref()).then(|| (session.state.clone(), session.clone()))
        });
        if let Some((step, snapshot)) = commit {
            runtime.engagement_step = step;
            let entries = try_commit_session_entries(document, &mut runtime, pane_id, &snapshot);
            return Ok(engagement_emit(doc, &runtime, entries, ctx));
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️WorldPointerDown

//#region 🔖️WorldPointerMove
pub mod world_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "world-pointer-move")]
    pub struct WorldPointerMove {
        pub x: Option<f64>,
        pub y: Option<f64>,
        pub z: Option<f64>,
    }

    pub fn handle(payload: &WorldPointerMove, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        // Live rubber-band preview during an active engagement session: applies `pointer.move`
        // (updating the session's cursor/preview context) in the window transient only — never an
        // object, never config, never a history row.
        let point_value =
            (payload.x.is_some() || payload.y.is_some() || payload.z.is_some()).then(|| semio_framework_value::DslValue::Array(vec![DslValue::float(payload.x.unwrap_or(0.0)), DslValue::float(payload.y.unwrap_or(0.0)), DslValue::float(payload.z.unwrap_or(0.0))]));
        let mut runtime = runtime_of(cfg, &ctx.window_transient);
        if let Some(session) = runtime.engagement_session.as_mut() {
            apply_event(session, "pointer.move", point_value.as_ref());
            Ok(engagement_emit(doc, &runtime, Vec::new(), ctx))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️WorldPointerMove
