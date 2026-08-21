//! 🗃️ Shooting play app commands — whole-fixture load/reset/save/import shell effects.

use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ImportSnapshotJson
pub mod import_snapshot_json {
    use super::*;

    /// 🛠️ Dev-only whole-fixture import — kept out of the command palette.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-snapshot-json")]
    pub struct ImportSnapshotJson {
        pub json: String,
    }

    pub async fn handle(payload: &ImportSnapshotJson, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match serde_json::from_str::<ShootingSnapshot>(&payload.json) {
            Ok(snapshot) => Ok(Emit { effects: vec![crate::editor::shooting::reset_document_effect(&snapshot)], ..Default::default() }),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ImportSnapshotJson

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    pub const SHOOTING_EXAMPLE_DEFAULT_ID: &str = "base-icon";

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub async fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() {
            Some(crate::artifacts::shooting::empty_shooting_snapshot())
        } else if payload.example_id == SHOOTING_EXAMPLE_DEFAULT_ID || payload.example_id == "base" {
            Some(crate::artifacts::shooting::schema::default_snapshot())
        } else {
            None
        };
        match next {
            Some(snapshot) => Ok(Emit { effects: vec![crate::editor::shooting::reset_document_effect(&snapshot)], ..Default::default() }),
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

    pub async fn handle(_payload: &ResetSnapshot, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit { effects: vec![crate::editor::shooting::reset_document_effect(&crate::artifacts::shooting::schema::default_snapshot())], ..Default::default() })
    }
}
//#endregion 🔖️ResetSnapshot

//#region 🔖️SaveDownload
pub mod save_download {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "save-download")]
    pub struct SaveDownload {}

    pub async fn handle(_payload: &SaveDownload, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match serde_json::to_string_pretty(doc.snapshot) {
            Ok(fixture_text) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: "shooting.shooting.ops".into(), mime_type: "text/plain".into(), data: fixture_text, encoding: None })),
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

    pub async fn handle(_payload: &LoadRequest, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(109), accept: ".ops,.dsl,.spk,application/octet-stream,text/plain".into(), read_as: None, import_action: "importSnapshotJson".into(), multiple: false }))
    }
}
//#endregion 🔖️LoadRequest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app};
    use crate::editor::shooting::ShootingCommand;

    /// 🧬️ `reset_snapshot::handle` emits a `Effect::LoadDocument` (outside undo history), not an
    /// `artifact_mutations` entry — driven directly through `handle` (not `dispatch`, which routes
    /// through `VcsArtifactApp` and never applies `effects` to its own store, that's the real host's
    /// job), same as the already-migrated `fem2d` sibling's `commands::example` tests.
    #[semio_framework_async_macros::async_test]
    async fn reset_snapshot_restores_default_snapshot() {
        use semio_framework_plugin::Effect;
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::AddShot(crate::editor::shooting::commands::shot::add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() }));
        assert_eq!(app.snapshot().expect("snapshot").shots.len(), 3);
        let snapshot = app.snapshot().expect("snapshot");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = ShootingConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let mut ctx = crate::editor::shooting::ShootingDispatchCtx::default();
        let emit = reset_snapshot::handle(&reset_snapshot::ResetSnapshot {}, &doc, &cfg, &mut ctx).expect("handle");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("resetSnapshot must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let restored = <ShootingSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(restored.shots.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn load_request_declares_the_import_snapshot_json_import_action() {
        use semio_framework_plugin::Effect;
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::LoadRequest(load_request::LoadRequest {}));
        match &result.requested_effects[0] {
            Effect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "importSnapshotJson"),
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
