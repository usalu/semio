//! 📄️ Lowpoly play app commands — whole-document JSON replacement (`importSnapshotJson`/
//! `replaceSnapshotJson`, two wire-distinct aliases over the identical body), both outside undo history
//! via `reset_document_effect` (a `Effect::LoadDocument`) — per `📓️taxonomy.md`, whole-document
//! replace has no `Mutation`-enum representative.

use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::LowpolyScratch;
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

fn reset_from_json(json: &str) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    match semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::lowpoly_json_decode::<LowpolySnapshot>(json) {
        Ok(parsed) => Emit { effects: vec![semio_s_artifact_lowpoly_lowpoly_kit::reset_document_effect(&parsed)], ..Default::default() },
        Err(_) => Emit::default(),
    }
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
        Ok(reset_from_json(&payload.json))
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
        Ok(reset_from_json(&payload.json))
    }
}
//#endregion 🔖️ReplaceSnapshotJson

//#region 🧪️Tests
//#endregion 🧪️Tests
