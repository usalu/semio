//! 🗃️ Shooting play app commands — whole-fixture load/reset/save/import shell effects.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSnapshotJson
pub mod set_snapshot_json {
    use super::*;

    /// 🛠️ Dev-only whole-fixture import — kept out of the command palette.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "snapshot-json")]
    pub struct SetSnapshotJson {
        pub json: String,
    }

    pub fn handle(payload: &SetSnapshotJson, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match serde_json::from_str::<ShootingSnapshot>(&payload.json) {
            Ok(snapshot) => Ok(Emit::mutations(vec![ShootingMutation::SetSnapshot { snapshot }])),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetSnapshotJson

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    pub const SHOOTING_EXAMPLE_DEFAULT_ID: &str = "base-icon";

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() {
            Some(crate::artifacts::shooting::empty_shooting_snapshot())
        } else if payload.example_id == SHOOTING_EXAMPLE_DEFAULT_ID || payload.example_id == "base" {
            Some(crate::artifacts::shooting::engine::default_snapshot())
        } else {
            None
        };
        match next {
            Some(snapshot) => Ok(Emit::mutations(vec![ShootingMutation::SetSnapshot { snapshot }])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetActiveExample

//#region 🔖️ResetSnapshot
pub mod reset_snapshot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reset-snapshot")]
    pub struct ResetSnapshot {}

    pub fn handle(_payload: &ResetSnapshot, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::SetSnapshot { snapshot: crate::artifacts::shooting::engine::default_snapshot() }]))
    }
}
//#endregion 🔖️ResetSnapshot

//#region 🔖️SaveDownload
pub mod save_download {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "save-download")]
    pub struct SaveDownload {}

    pub fn handle(_payload: &SaveDownload, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match serde_json::to_string_pretty(doc.snapshot) {
            Ok(fixture_text) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: "shooting.shooting.ops".into(), mime_type: "text/plain".into(), data: fixture_text, encoding: None })),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SaveDownload

//#region 🔖️LoadRequest
pub mod load_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "load-request")]
    pub struct LoadRequest {}

    pub fn handle(_payload: &LoadRequest, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".ops,.dsl,.spk,application/octet-stream,text/plain".into(), read_as: None, import_action: "setSnapshotJson".into(), multiple: false }))
    }
}
//#endregion 🔖️LoadRequest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn reset_snapshot_restores_default_snapshot() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::AddShot(crate::apps::shooting::commands::shot::add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() }));
        assert_eq!(app.snapshot().expect("snapshot").shots.len(), 3);
        dispatch(&mut app, ShootingCommand::ResetSnapshot(reset_snapshot::ResetSnapshot {}));
        assert_eq!(app.snapshot().expect("snapshot").shots.len(), 2);
    }

    #[test]
    fn load_request_declares_the_set_snapshot_json_import_action() {
        use semio_framework_plugin::HostEffect;
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::LoadRequest(load_request::LoadRequest {}));
        match &result.requested_effects[0] {
            HostEffect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "setSnapshotJson"),
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
