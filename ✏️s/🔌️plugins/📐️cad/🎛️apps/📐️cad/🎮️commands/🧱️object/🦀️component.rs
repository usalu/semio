//! 🧱️ CAD play app commands — object lifecycle: create, patch (single and multi-selection), delete, duplicate.

use crate::apps::cad::config::{CadConfig, CadConfigMutation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{command_value_json, ids_or_selection, make_object_for_typology, patch_objects_mutations, runtime_of, snapshot_of};
use crate::artifacts::cad::engine::next_cad_id;
use crate::artifacts::cad::{cad_all_objects, cad_find_object_pane, cad_pane_from_model_definition_id, cad_pane_objects, CadPaneId};
use semio_framework_plugin::SelectionSet;
use serde_json::json;


//#region 🔖️AddObject
pub mod add_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-object")]
    pub struct AddObject {
        pub typology: Option<String>,
    }

    pub fn handle(payload: &AddObject, doc: &DocumentView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg);
        let typology = payload.typology.as_deref().unwrap_or("spatial.shape.primitive.box");
        let pane = cad_pane_from_model_definition_id(&document.active_model_definition_id).unwrap_or(CadPaneId::Shape);
        let object = make_object_for_typology(typology, cad_pane_objects(document, pane).len(), pane);
        runtime.selected_object_ids = SelectionSet::from(vec![object.id.clone()]);
        let mut emit = Emit::mutations(vec![CadMutation::AddObject { pane, object }]);
        emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
        Ok(emit)
    }
}
//#endregion 🔖️AddObject

//#region 🔖️PatchObject
pub mod patch_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-object")]
    pub struct PatchObject {
        pub object_id: String,
        pub field: String,
        pub value: Option<String>,
        pub delta: Option<f64>,
    }

    pub fn handle(payload: &PatchObject, doc: &DocumentView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let value_json = payload.value.as_deref().map(|entry| command_value_json(&payload.field, entry));
        let delta_json = payload.delta.map(|entry| json!(entry));
        Ok(Emit::mutations(patch_objects_mutations(doc.snapshot, std::slice::from_ref(&payload.object_id), &payload.field, value_json.as_ref(), delta_json.as_ref())))
    }
}
//#endregion 🔖️PatchObject

//#region 🔖️PatchSelection
pub mod patch_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-selection")]
    pub struct PatchSelection {
        pub object_ids: Vec<String>,
        pub field: String,
        pub value: Option<String>,
        pub delta: Option<f64>,
    }

    pub fn handle(payload: &PatchSelection, doc: &DocumentView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let runtime = runtime_of(cfg);
        let ids = ids_or_selection(&payload.object_ids, runtime.selected_object_ids.as_slice());
        let value_json = payload.value.as_deref().map(|entry| command_value_json(&payload.field, entry));
        let delta_json = payload.delta.map(|entry| json!(entry));
        Ok(Emit::mutations(patch_objects_mutations(doc.snapshot, &ids, &payload.field, value_json.as_ref(), delta_json.as_ref())))
    }
}
//#endregion 🔖️PatchSelection

//#region 🔖️DeleteObject
pub mod delete_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-object")]
    pub struct DeleteObject {
        pub object_id: String,
    }

    pub fn handle(payload: &DeleteObject, doc: &DocumentView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg);
        if let Some(pane) = cad_find_object_pane(document, &payload.object_id) {
            runtime.selected_object_ids.remove_id(&payload.object_id);
            let mut emit = Emit::mutations(vec![CadMutation::RemoveObject { pane, object_id: payload.object_id.clone() }]);
            emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
            return Ok(emit);
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️DeleteObject

//#region 🔖️DuplicateObject
pub mod duplicate_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "duplicate-object")]
    pub struct DuplicateObject {
        pub object_id: String,
    }

    pub fn handle(payload: &DuplicateObject, doc: &DocumentView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg);
        let duplicate_target = cad_all_objects(document).find(|(object, _)| object.id == payload.object_id).map(|(object, pane)| (object.clone(), pane));
        if let Some((mut duplicate, pane)) = duplicate_target {
            duplicate.id = next_cad_id("object");
            duplicate.label = format!("{} copy", duplicate.label);
            runtime.selected_object_ids = SelectionSet::from(vec![duplicate.id.clone()]);
            let mut emit = Emit::mutations(vec![CadMutation::AddObject { pane, object: duplicate }]);
            emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
            return Ok(emit);
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️DuplicateObject
