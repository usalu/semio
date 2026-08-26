//! ✏️ Lowpoly play app command — patches a scalar field (name / smooth shading) on an object. A lone
//! command in its group, so (per TEMPLATE.md §5.7's `module_inception` rule) the payload lives directly
//! at this file's top level rather than in a same-named inner `pub mod`.

use crate::artifacts::lowpoly::mutations::change_object_smooth_shading::mutation::ChangeObjectSmoothShading;
use crate::artifacts::lowpoly::mutations::rename_object::mutation::RenameObject;
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
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
    let mutation = match payload.field.as_str() {
        "name" => value.as_ref().and_then(|entry| entry.as_str()).map(|new_name| LowpolyMutation::RenameObject(RenameObject { id: payload.object_id.clone(), new_name: new_name.to_string() })),
        "smoothShading" => {
            let new_smooth_shading = value.as_ref().and_then(|entry| entry.as_bool()).unwrap_or(!object.smooth_shading);
            Some(LowpolyMutation::ChangeObjectSmoothShading(ChangeObjectSmoothShading { id: payload.object_id.clone(), new_smooth_shading }))
        }
        _ => None,
    };
    match mutation {
        Some(mutation) => Ok(Emit::mutations(vec![mutation])),
        None => Ok(Emit::default()),
    }
}
//#endregion 🔖️PatchObject

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::lowpoly::testkit::{app, dispatch};
    use crate::editor::lowpoly::LowpolyCommand;

    #[semio_framework_async_macros::async_test]
    async fn patch_object_name_emits_operation() {
        let mut a = app();
        let object_id = a.snapshot().expect("projection").objects[0].id.clone();
        dispatch(&mut a, LowpolyCommand::PatchObject(PatchObject { object_id, field: "name".into(), value_json: Some(serde_json::to_string("Renamed").unwrap()) })).await;
        assert_eq!(a.snapshot().expect("projection").objects[0].name, "Renamed");
    }
}
//#endregion 🧪️Tests
