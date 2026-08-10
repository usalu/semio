//! 🤝️ CAD play app commands — the engagement REPL: input, submit, keyed transitions, abort, and the two world-pointer events that drive a live construction interaction.

use crate::apps::cad::config::{CadConfig, CadConfigMutation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{cad_pane_id_from_suffix, engagement_submit_mutations, runtime_of, snapshot_of, start_interaction_session, try_commit_session_mutations};
use crate::artifacts::cad::engine::interaction::apply_event;
use crate::artifacts::cad::CadPaneId;
use serde_json::json;


//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {
        pub pane: Option<String>,
    }

    pub fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let pane_id = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_suffix);
        let ops = engagement_submit_mutations(doc.snapshot, &mut runtime, pane_id);
        let mut emit = Emit::mutations(ops);
        emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
        Ok(emit)
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
        pub pane: Option<String>,
    }

    pub fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.engagement_input = payload.value.clone();
        runtime.engagement_pane = payload.pane.clone();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementPossibleSelect
pub mod engagement_possible_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-possible-select")]
    pub struct EngagementPossibleSelect {
        pub pane: Option<String>,
        pub possible_id: String,
    }

    pub fn handle(payload: &EngagementPossibleSelect, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let pane_id = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_suffix);
        let step = runtime.engagement_session.as_mut().and_then(|session| apply_event(session, &payload.possible_id, None).then(|| session.state.clone()));
        if let Some(step) = step {
            runtime.engagement_step = step;
        } else if !start_interaction_session(&mut runtime, pane_id, &payload.possible_id) {
            runtime.engagement_input = payload.possible_id.clone();
        }
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
    }
}
//#endregion 🔖️EngagementPossibleSelect

//#region 🔖️EngagementRepeatLast
pub mod engagement_repeat_last {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-repeat-last")]
    pub struct EngagementRepeatLast {
        pub pane: Option<String>,
    }

    pub fn handle(payload: &EngagementRepeatLast, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let pane_id = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_suffix);
        if runtime.engagement_session.is_none() {
            if let Some(interaction_id) = runtime.last_finalized_interaction_id.clone() {
                start_interaction_session(&mut runtime, pane_id, &interaction_id);
                return Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]));
            }
        }
        runtime.engagement_step = "Idle".into();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
    }
}
//#endregion 🔖️EngagementRepeatLast

//#region 🔖️EngagementAbort
pub mod engagement_abort {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-abort")]
    pub struct EngagementAbort {}

    pub fn handle(_payload: &EngagementAbort, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.engagement_input.clear();
        runtime.engagement_session = None;
        runtime.engagement_step = "Idle".into();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
    }
}
//#endregion 🔖️EngagementAbort

//#region 🔖️WorldPointerDown
pub mod world_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-down")]
    pub struct WorldPointerDown {
        pub pane: Option<String>,
        pub surface_id: Option<String>,
        pub x: Option<f64>,
        pub y: Option<f64>,
        pub z: Option<f64>,
    }

    pub fn handle(payload: &WorldPointerDown, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg);
        let pane_id = payload.pane.as_deref().map(cad_pane_id_from_suffix).or_else(|| payload.surface_id.as_deref().and_then(|surface_id| surface_id.rsplit('/').next()).map(cad_pane_id_from_suffix)).unwrap_or(CadPaneId::Shape);
        // 📍️ `apply_event`'s payload for a pointer event is the raw position value itself
        // (mirrors the pre-B1 `args.get("position")` extraction — NOT re-wrapped in another
        // `{"position": ...}` object).
        let point_value = (payload.x.is_some() || payload.y.is_some() || payload.z.is_some()).then(|| json!([payload.x.unwrap_or(0.0), payload.y.unwrap_or(0.0), payload.z.unwrap_or(0.0)]));
        let commit = runtime.engagement_session.as_mut().and_then(|session| apply_event(session, "pointer.down", point_value.as_ref()).then(|| (session.state.clone(), session.clone())));
        if let Some((step, snapshot)) = commit {
            runtime.engagement_step = step;
            let ops = try_commit_session_mutations(document, &mut runtime, pane_id, &snapshot);
            let mut emit = Emit::mutations(ops);
            emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
            return Ok(emit);
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️WorldPointerDown

//#region 🔖️WorldPointerMove
pub mod world_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-move")]
    pub struct WorldPointerMove {
        pub x: Option<f64>,
        pub y: Option<f64>,
        pub z: Option<f64>,
    }

    pub fn handle(payload: &WorldPointerMove, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        // Live rubber-band preview during an active engagement session: applies `pointer.move`
        // (updating the session's cursor/preview context) without ever committing an object or
        // touching VCS history — coalesced (`amend_config`) so a whole drag is one undo step.
        let point_value = (payload.x.is_some() || payload.y.is_some() || payload.z.is_some()).then(|| json!([payload.x.unwrap_or(0.0), payload.y.unwrap_or(0.0), payload.z.unwrap_or(0.0)]));
        let mut runtime = runtime_of(cfg);
        if let Some(session) = runtime.engagement_session.as_mut() {
            apply_event(session, "pointer.move", point_value.as_ref());
            let mut seq = ctx.preview_seq.borrow_mut();
            *seq = seq.wrapping_add(1);
            drop(seq);
            Ok(Emit::amend_config(vec![snapshot_of(&runtime, cfg.snapshot)], "engagement.pointer-move"))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️WorldPointerMove
