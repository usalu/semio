//! 📄️ Lowpoly play app commands — whole-projection JSON replacement (`setProjectionJson`/
//! `setFixtureJson`, two wire-distinct aliases over the identical body).

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigOperation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::artifacts::lowpoly::op::LowpolyOperation;
use crate::artifacts::lowpoly::LowpolyProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

fn set_projection_from_json(json: &str) -> Emit<LowpolyOperation, LowpolyConfigOperation> {
    match serde_json::from_str::<LowpolyProjection>(json) {
        Ok(parsed) => Emit::operations(vec![LowpolyOperation::SetProjection { projection: parsed }]),
        Err(_) => Emit::default(),
    }
}

//#region 🔖️SetProjectionJson
pub mod set_projection_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-projection-json")]
    pub struct SetProjectionJson {
        pub json: String,
    }

    pub fn handle(payload: &SetProjectionJson, _doc: &DocumentView<'_, LowpolyProjection>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        Ok(set_projection_from_json(&payload.json))
    }
}
//#endregion 🔖️SetProjectionJson

//#region 🔖️SetFixtureJson
pub mod set_fixture_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-fixture-json")]
    pub struct SetFixtureJson {
        pub json: String,
    }

    pub fn handle(payload: &SetFixtureJson, _doc: &DocumentView<'_, LowpolyProjection>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        Ok(set_projection_from_json(&payload.json))
    }
}
//#endregion 🔖️SetFixtureJson

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;
    use crate::artifacts::lowpoly::engine::default_projection;

    #[test]
    fn set_projection_json_replaces_the_whole_document() {
        let mut a = app();
        let replacement = crate::artifacts::lowpoly::projection_from_mesh_json(&default_projection().objects[0].mesh_json, "obj-x", "X");
        let json = serde_json::to_string(&replacement).unwrap();
        dispatch(&mut a, LowpolyCommand::SetProjectionJson(super::set_projection_json::SetProjectionJson { json }));
        assert_eq!(a.projection().expect("projection").objects[0].id, "obj-x");
    }

    #[test]
    fn set_fixture_json_with_invalid_json_is_a_no_op() {
        let mut a = app();
        let before = a.projection().expect("projection");
        dispatch(&mut a, LowpolyCommand::SetFixtureJson(super::set_fixture_json::SetFixtureJson { json: "not json".into() }));
        assert_eq!(a.projection().expect("projection"), before);
    }
}
//#endregion 🧪️Tests
