//! 🗃️ Shooting play app commands — whole-fixture load/reset/save/import shell effects.

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use crate::op::ShootingMutation;
use crate::ShootingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ImportSnapshotJson
pub mod import_snapshot_json {
    use super::*;

    /// 🛠️ Dev-only whole-fixture import — kept out of the command palette.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "import-snapshot-json")]
    pub struct ImportSnapshotJson {
        pub json: String,
    }

    pub fn handle(payload: &ImportSnapshotJson, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let parsed: Result<ShootingSnapshot, ()> = dsl::os_pack::json::parse(&payload.json).map_err(|_| ()).and_then(|json_value| {
            let dsl_value = dsl::os_pack::json::to_dsl_value(&json_value);
            dsl::FromValue::from_value(dsl_value).map_err(|_: dsl::ValueError| ())
        });
        match parsed {
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() {
            Some(crate::empty_shooting_snapshot())
        } else if payload.example_id == SHOOTING_EXAMPLE_DEFAULT_ID || payload.example_id == "base" {
            Some(crate::standards::v1::subsets::any::schema::default_snapshot())
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "reset-snapshot")]
    pub struct ResetSnapshot {}

    pub fn handle(_payload: &ResetSnapshot, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit { effects: vec![crate::editor::shooting::reset_document_effect(&crate::standards::v1::subsets::any::schema::default_snapshot())], ..Default::default() })
    }
}
//#endregion 🔖️ResetSnapshot

//#region 🔖️SaveDownload
pub mod save_download {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "save-download")]
    pub struct SaveDownload {}

    pub fn handle(_payload: &SaveDownload, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(doc.snapshot));
        let fixture_text = dsl::os_pack::json::to_string_pretty(&value);
        Ok(Emit::effect(Effect::DownloadMediaExport { filename: "shooting.shooting.ops".into(), mime_type: "text/plain".into(), data: fixture_text, encoding: None }))
    }
}
//#endregion 🔖️SaveDownload

//#region 🔖️LoadRequest
pub mod load_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "load-request")]
    pub struct LoadRequest {}

    pub fn handle(_payload: &LoadRequest, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(109), accept: ".ops,.dsl,.spk,application/octet-stream,text/plain".into(), read_as: None, import_action: "importSnapshotJson".into(), multiple: false }))
    }
}
//#endregion 🔖️LoadRequest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
