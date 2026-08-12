//! 📄️ Lowpoly play app commands — whole-projection JSON replacement (`importSnapshotJson`/
//! `setFixtureJson`, two wire-distinct aliases over the identical body), both outside undo history
//! via `reset_document_effect` (a `HostEffect::LoadDocument`) — per `📓️taxonomy.md`, whole-document
//! replace has no `Mutation`-enum representative.

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

fn reset_from_json(json: &str) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    match serde_json::from_str::<LowpolySnapshot>(json) {
        Ok(parsed) => Emit { effects: vec![crate::apps::lowpoly::reset_document_effect(&parsed)], ..Default::default() },
        Err(_) => Emit::default(),
    }
}

//#region 🔖️ImportSnapshotJson
pub mod set_snapshot_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
mod tests {
    use super::*;
    use crate::apps::lowpoly::config::LowpolyConfig;
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;
    use crate::artifacts::lowpoly::schema::default_snapshot;

    /// 🧬️ `importSnapshotJson`/`setFixtureJson` emit a `HostEffect::LoadDocument` (outside undo
    /// history), not an `artifact_mutations` entry — driven directly through `handle` (not
    /// `dispatch`, which routes through `VcsArtifactApp` and never applies `effects` to its own
    /// store, that's the real host's job), same pattern as the already-migrated `shooting` sibling.
    #[test]
    fn import_snapshot_json_replaces_the_whole_document() {
        let replacement = crate::artifacts::lowpoly::snapshot_from_mesh_json(&default_snapshot().objects[0].mesh_json, "obj-x", "X");
        let json = serde_json::to_string(&replacement).unwrap();
        let snapshot = default_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView { snapshot: &snapshot, history: &history };
        let cfg_snapshot = LowpolyConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let mut scratch = LowpolyScratch::default();
        let emit = set_snapshot_json::handle(&set_snapshot_json::ImportSnapshotJson { json }, &doc, &cfg, &mut scratch).expect("handle");
        let semio_framework_plugin::HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("importSnapshotJson must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <LowpolySnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(loaded.objects[0].id, "obj-x");
    }

    #[test]
    fn set_fixture_json_with_invalid_json_is_a_no_op() {
        let mut a = app();
        let before = a.snapshot().expect("projection");
        dispatch(&mut a, LowpolyCommand::SetFixtureJson(super::set_fixture_json::SetFixtureJson { json: "not json".into() }));
        assert_eq!(a.snapshot().expect("projection"), before);
    }
}
//#endregion 🧪️Tests
