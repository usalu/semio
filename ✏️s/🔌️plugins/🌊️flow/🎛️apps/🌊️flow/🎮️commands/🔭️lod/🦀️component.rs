//! 🔭️ Flow play app commands — the level-of-detail and proximity-select canvas measures.
//! The matching chrome controls live in `🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️options/{🔭️lod,📏️proximity}`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::{op::FlowMutation, FlowFixture};
use flow::{dag::DagDrawLod, FlowEvalSession, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLodMode
pub mod set_lod_mode {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-lod-mode")]
    pub struct SetLodMode {
        pub value: String,
    }

    /// 🎚️ Unknown lod ids are rejected outright (rather than clamped) — the select control only ever
    /// offers `FLOW_LOD_MODE_AUTOMATIC` plus the real `DagDrawLod` ids.
    pub fn handle(payload: &SetLodMode, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        if payload.value == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(&payload.value).is_some() {
            Ok(Emit::config(vec![FlowConfigMutation::SetLodMode { value: payload.value.clone() }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetLodMode

//#region 🔖️SetProximityDistance
pub mod set_proximity_distance {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-proximity-distance")]
    pub struct SetProximityDistance {
        pub value: f64,
    }

    pub fn handle(payload: &SetProximityDistance, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(Emit::config(vec![FlowConfigMutation::SetProximityDistance { value: payload.value.max(0.0) }]))
    }
}
//#endregion 🔖️SetProximityDistance

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app, render};
    use crate::apps::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};

    #[test]
    fn set_lod_mode_rejects_unknown_and_accepts_known() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetLodMode(set_lod_mode::SetLodMode { value: "bogus".into() }));
        dispatch(&mut app, FlowCommand::SetLodMode(set_lod_mode::SetLodMode { value: "micro".into() }));
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN);
        assert!(json.contains("\\\"forcedLabel\\\":\\\"micro\\\"") || json.contains("\"forcedLabel\":\"micro\""));
    }

    #[test]
    fn default_runtime_enables_proximity_distance() {
        let mut app = flow_app();
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN);
        assert!(json.contains("proximityDistance") && !json.contains(r#""proximityDistance":0"#));
    }

    #[test]
    fn set_proximity_distance_updates_scene_lod_json() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetProximityDistance(set_proximity_distance::SetProximityDistance { value: 96.0 }));
        assert!(render(&mut app, FLOW_PLAY_BODY_MAIN).contains("96"));
    }

    #[test]
    fn negative_proximity_distances_clamp_to_zero() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::SetProximityDistance(set_proximity_distance::SetProximityDistance { value: -10.0 }));
        assert!(result.document_mutations.is_empty(), "a view command emits no document operations");
        assert!(!render(&mut app, FLOW_PLAY_BODY_MAIN).contains("-10"));
    }
}
//#endregion 🧪️Tests
