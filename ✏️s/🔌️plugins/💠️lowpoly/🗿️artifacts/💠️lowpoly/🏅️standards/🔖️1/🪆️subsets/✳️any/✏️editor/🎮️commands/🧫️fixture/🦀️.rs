//! 📄️ Lowpoly play app commands — whole-projection JSON replacement (`importSnapshotJson`/
//! `setFixtureJson`, two wire-distinct aliases over the identical body), both outside undo history
//! via `reset_document_effect` (a `Effect::LoadDocument`) — per `📓️taxonomy.md`, whole-document
//! replace has no `Mutation`-enum representative.

use crate::op::LowpolyMutation;
use crate::LowpolySnapshot;
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

fn reset_from_json(json: &str) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    match dsl::json::from_json_str::<LowpolySnapshot>(json) {
        Ok(parsed) => Emit { effects: vec![crate::editor::lowpoly::reset_document_effect(&parsed)], ..Default::default() },
        Err(_) => Emit::default(),
    }
}

//#region 🔖️ImportSnapshotJson
pub mod set_snapshot_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
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

//#region 🔖️SetFixtureJson
pub mod set_fixture_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-fixture-json")]
    pub struct SetFixtureJson {
        pub json: String,
    }

    pub fn handle(payload: &SetFixtureJson, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(reset_from_json(&payload.json))
    }
}
//#endregion 🔖️SetFixtureJson

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
