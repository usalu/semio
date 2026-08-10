//! ✏️ Lowpoly play app command — patches a scalar field (name / smooth shading) on an object. A lone
//! command in its group, so (per TEMPLATE.md §5.7's `module_inception` rule) the payload lives directly
//! at this file's top level rather than in a same-named inner `pub mod`.

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::{LowpolyObjectPatch, LowpolySnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️PatchObject
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-object")]
pub struct PatchObject {
    pub object_id: String,
    pub field: String,
    pub value_json: Option<String>,
}

pub fn handle(payload: &PatchObject, doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    let projection = doc.snapshot;
    let value = payload.value_json.as_deref().and_then(|json| serde_json::from_str::<Value>(json).ok());
    let Some(object) = projection.objects.iter().find(|object| object.id == payload.object_id) else { return Ok(Emit::default()) };
    let patch = match payload.field.as_str() {
        "name" => LowpolyObjectPatch { name: value.as_ref().and_then(|entry| entry.as_str()).map(str::to_string), ..Default::default() },
        "smoothShading" => LowpolyObjectPatch { smooth_shading: Some(value.as_ref().and_then(|entry| entry.as_bool()).unwrap_or(!object.smooth_shading)), ..Default::default() },
        _ => LowpolyObjectPatch::default(),
    };
    if patch == LowpolyObjectPatch::default() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![LowpolyMutation::ObjectsPatch { id: payload.object_id.clone(), patch }]))
}
//#endregion 🔖️PatchObject

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;

    #[test]
    fn patch_object_name_emits_operation() {
        let mut a = app();
        let object_id = a.snapshot().expect("projection").objects[0].id.clone();
        dispatch(&mut a, LowpolyCommand::PatchObject(PatchObject { object_id, field: "name".into(), value_json: Some(serde_json::to_string("Renamed").unwrap()) }));
        assert_eq!(a.snapshot().expect("projection").objects[0].name, "Renamed");
    }
}
//#endregion 🧪️Tests
