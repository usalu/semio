//! 📄️ Lowpoly play app commands — whole-document JSON replacement (`importSnapshotJson`/
//! `replaceSnapshotJson`, two wire-distinct aliases over the identical body), both outside undo history
//! via `reset_document_effect` (a `Effect::LoadDocument`) — per `📓️taxonomy.md`, whole-document
//! replace has no `Mutation`-enum representative.

use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::LowpolyScratch;
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 📥️ Maximum UTF-8 JSON text accepted by either public snapshot import action.
pub const LOWPOLY_SNAPSHOT_JSON_BYTES: usize = 2 * 1024 * 1024;

fn reset_from_json(json: &str) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    if json.len() > LOWPOLY_SNAPSHOT_JSON_BYTES {
        return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("lowpoly.snapshot-json.capacity"), "Snapshot JSON exceeds its declared UTF-8 input budget"));
    }
    let parsed = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::lowpoly_json_decode::<LowpolySnapshot>(json)
        .map_err(|error| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("lowpoly.snapshot-json.invalid"), error.to_string()))?;
    parsed.validate_paint_pixels().map_err(|error| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new(semio_s_artifact_lowpoly_lowpoly_model::paint_texture::PaintTextureError::CODE), error.to_string()))?;
    Ok(Emit { effects: vec![semio_s_artifact_lowpoly_lowpoly_kit::reset_document_effect(&parsed)], ..Default::default() })
}

//#region 🔖️ImportSnapshotJson
pub mod set_snapshot_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "import-snapshot-json")]
    pub struct ImportSnapshotJson {
        pub json: String,
    }

    pub fn handle(payload: &ImportSnapshotJson, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        reset_from_json(&payload.json)
    }
}
//#endregion 🔖️ImportSnapshotJson

//#region 🔖️ReplaceSnapshotJson
pub mod replace_snapshot_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "replace-snapshot-json")]
    pub struct ReplaceSnapshotJson {
        pub json: String,
    }

    pub fn handle(payload: &ReplaceSnapshotJson, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        reset_from_json(&payload.json)
    }
}
//#endregion 🔖️ReplaceSnapshotJson

//#region 🧪️Tests
//#endregion 🧪️Tests
