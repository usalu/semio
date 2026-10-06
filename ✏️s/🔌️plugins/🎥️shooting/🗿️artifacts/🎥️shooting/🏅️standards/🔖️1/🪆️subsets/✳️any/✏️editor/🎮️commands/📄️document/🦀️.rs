//! 🗃️ Shooting play app commands — whole-document load/reset/save/import shell effects.

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
use crate::ShootingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ImportSnapshotJson
pub mod import_snapshot_json {
    use super::*;

    /// 🛠️ Dev-only whole-document import — kept out of the command palette.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "import-snapshot-json")]
    pub struct ImportSnapshotJson {
        pub json: String,
    }

    pub fn handle(payload: &ImportSnapshotJson, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let parsed: Result<ShootingSnapshot, ()> = semio_framework_pack_json::parse(&payload.json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| ()).and_then(|json_value| {
            let dsl_value = semio_framework_pack_json::to_dsl_value(&json_value);
            semio_framework_value::FromValue::from_value(dsl_value).map_err(|_: semio_framework_value::ValueError| ())
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

    pub const SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() {
            Some(crate::empty_shooting_snapshot())
        } else if payload.example_id == SHOOTING_EXAMPLE_DEFAULT_ID || payload.example_id == "base" || payload.example_id == "demo" {
            Some(crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot())
        } else if payload.example_id == SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT || payload.example_id == "forest-left" {
            crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::examples::hexagonal_cut_concrete_forest_left::PRIMARY_TEXT)
                .ok()
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "reset-snapshot")]
    pub struct ResetSnapshot {}

    pub fn handle(_payload: &ResetSnapshot, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit { effects: vec![crate::editor::shooting::reset_document_effect(&crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot())], ..Default::default() })
    }
}
//#endregion 🔖️ResetSnapshot

//#region 🔖️SaveDownload
pub mod save_download {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "save-download")]
    pub struct SaveDownload {}

    pub fn handle(_payload: &SaveDownload, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(doc.snapshot));
        let document_text = semio_framework_pack_json::to_string_pretty(&value);
        Ok(Emit::effect(Effect::DownloadMediaExport { filename: "shooting.shooting.ops".into(), mime_type: "text/plain".into(), data: document_text, encoding: None }))
    }
}
//#endregion 🔖️SaveDownload

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
