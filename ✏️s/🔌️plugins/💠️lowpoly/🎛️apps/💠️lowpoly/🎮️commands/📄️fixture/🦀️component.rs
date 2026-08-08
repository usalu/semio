//! 📄️ Lowpoly play app commands — whole-projection JSON replacement (`setSnapshotJson`/
//! `setFixtureJson`, two wire-distinct aliases over the identical body).

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

fn set_snapshot_from_json(json: &str) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    match serde_json::from_str::<LowpolySnapshot>(json) {
        Ok(parsed) => Emit::mutations(vec![LowpolyMutation::SetSnapshot { snapshot: parsed }]),
        Err(_) => Emit::default(),
    }
}

//#region 🔖️SetSnapshotJson
pub mod set_snapshot_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-snapshot-json")]
    pub struct SetSnapshotJson {
        pub json: String,
    }

    pub fn handle(payload: &SetSnapshotJson, _doc: &DocumentView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(set_snapshot_from_json(&payload.json))
    }
}
//#endregion 🔖️SetSnapshotJson

//#region 🔖️SetFixtureJson
pub mod set_fixture_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-fixture-json")]
    pub struct SetFixtureJson {
        pub json: String,
    }

    pub fn handle(payload: &SetFixtureJson, _doc: &DocumentView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(set_snapshot_from_json(&payload.json))
    }
}
//#endregion 🔖️SetFixtureJson

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;
    use crate::artifacts::lowpoly::engine::default_snapshot;

    #[test]
    fn set_snapshot_json_replaces_the_whole_document() {
        let mut a = app();
        let replacement = crate::artifacts::lowpoly::snapshot_from_mesh_json(&default_snapshot().objects[0].mesh_json, "obj-x", "X");
        let json = serde_json::to_string(&replacement).unwrap();
        dispatch(&mut a, LowpolyCommand::SetSnapshotJson(super::set_snapshot_json::SetSnapshotJson { json }));
        assert_eq!(a.snapshot().expect("projection").objects[0].id, "obj-x");
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
