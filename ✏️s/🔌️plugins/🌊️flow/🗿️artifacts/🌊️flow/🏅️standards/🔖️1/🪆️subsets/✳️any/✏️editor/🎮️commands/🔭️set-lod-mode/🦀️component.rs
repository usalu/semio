//! 🔭️ 🔭️ Flow play app commands command — `set-lod-mode`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::{dag::DagDrawLod, FlowEvalSession, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SetLodMode {
    pub value: String,
}

/// 🎚️ Unknown lod ids are rejected outright (rather than clamped) — the select control only ever
/// offers `FLOW_LOD_MODE_AUTOMATIC` plus the real `DagDrawLod` ids.
pub async fn handle(payload: &SetLodMode, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    if payload.value == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(&payload.value).is_some() {
        Ok(Emit::config(vec![FlowConfigMutation::SetLodMode { value: payload.value.clone() }]))
    } else {
        Ok(Emit::default())
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app, render};
    use crate::editor::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};

    #[semio_framework_async_macros::async_test]
    async fn set_lod_mode_rejects_unknown_and_accepts_known() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetLodMode(SetLodMode { value: "bogus".into() }));
        dispatch(&mut app, FlowCommand::SetLodMode(SetLodMode { value: "micro".into() }));
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN);
        assert!(json.contains("\\\"forcedLabel\\\":\\\"micro\\\"") || json.contains("\"forcedLabel\":\"micro\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn default_runtime_enables_proximity_distance() {
        let mut app = flow_app();
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN);
        assert!(json.contains("proximityDistance") && !json.contains(r#""proximityDistance":0"#));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_proximity_distance_updates_scene_lod_json() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetProximityDistance(crate::editor::flow::commands::set_proximity_distance::SetProximityDistance { value: 96.0 }));
        assert!(render(&mut app, FLOW_PLAY_BODY_MAIN).contains("96"));
    }

    #[semio_framework_async_macros::async_test]
    async fn negative_proximity_distances_clamp_to_zero() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::SetProximityDistance(crate::editor::flow::commands::set_proximity_distance::SetProximityDistance { value: -10.0 }));
        assert!(result.mutations.is_empty(), "a view command emits no document operations");
        assert!(!render(&mut app, FLOW_PLAY_BODY_MAIN).contains("-10"));
    }
}
//#endregion 🧪️Tests
