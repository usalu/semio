//! 🧱️ CAD play app commands — object lifecycle: create, patch (single and multi-selection), delete, duplicate.
//!
//! 🪆️ Object data lives inside the pane's composed `s.stdio.semio@v1/model` CHILD document and is edited only there
//! (design §20.15 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): each handler reads the panes' composed models
//! through the child view, decides its child-lane leaves (`insert-element`, `remove-element`, `set-element`, the relative
//! `drag-elements`) and publishes them on the child that holds the object. The parent document never changes, the pane's
//! child id stays stable, and every leaf is its own editable history row.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::modes::edit::tools::transform::{cad_child_leaves_emit, cad_pane_models, cad_transform_tool_emit, CadToolEntry, CadTransformRecord};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{axis3_index, cad_pane_from_view, create_object_entry, delete_object_leaves, duplicate_object_entries, ids_or_selection, make_object_for_typology, patch_objects_leaves};
use crate::op::CadMutation;
use crate::{CadPaneId, CadSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪟️ The pane a document-mutating object gesture lands in: the addressed window instance when the
/// host supplied one, otherwise the shape pane (the layout's first quadrant).
fn target_pane(ctx: &CadDispatchCtx) -> CadPaneId {
    ctx.view_state.as_ref().and_then(|view| cad_pane_from_view(view).ok()).unwrap_or(CadPaneId::Shape)
}

/// 🧭️ An inspector origin DELTA (`origin.x` … with `delta`, no absolute `value`) is a drag of the addressed objects along
/// that axis — the transform tool's relative `drag-elements`, never an absolute placement; `None` for every other edit.
fn origin_delta_emit(doc: &ArtifactView<'_, CadSnapshot>, verb: &str, ids: Vec<String>, field: &str, value: Option<&String>, delta: Option<f64>) -> Option<Emit<CadMutation, CadConfigMutation>> {
    let (None, Some(delta), Some(axis)) = (value, delta, axis3_index(field, "origin")) else { return None };
    let mut offset = [0.0; 3];
    offset[axis] = delta;
    Some(cad_transform_tool_emit(doc, verb, vec![CadToolEntry::Transform(CadTransformRecord::drag(ids, offset))]))
}

/// 🎯️ The plain child edit that applies `field`'s absolute or per-object edit across `ids`: one `set-element` per object
/// that changes, on the pane child holding it.
fn patch_emit(doc: &ArtifactView<'_, CadSnapshot>, ids: &[String], field: &str, value: Option<&String>, delta: Option<f64>) -> Emit<CadMutation, CadConfigMutation> {
    let models = cad_pane_models(doc.snapshot, &doc.children);
    let value = value.map(|value| crate::editor::cad::command_value_json(field, value));
    let delta = delta.map(semio_framework_value::DslValue::float);
    let leaves = patch_objects_leaves(&models, ids, field, value.as_ref(), delta.as_ref());
    cad_child_leaves_emit(&models, None, leaves)
}

//#region 🔖️AddObject
pub mod add_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "add-object")]
    pub struct AddObject {
        pub typology: Option<String>,
    }

    pub fn handle(payload: &AddObject, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let pane = target_pane(ctx);
        let typology = payload.typology.clone().unwrap_or_else(|| "spatial.shape.primitive.box".to_string());
        let models = cad_pane_models(doc.snapshot, &doc.children);
        let Some(model) = models.model(pane) else {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("cad.object.pane-uncomposed"), format!("addObject cannot land in the {} pane: it composes no model child", pane.model_definition_id())));
        };
        let object = make_object_for_typology(&typology, model.elements.len(), pane);
        Ok(cad_transform_tool_emit(doc, "addObject", vec![create_object_entry(pane, &object)]))
    }
}
//#endregion 🔖️AddObject

//#region 🔖️PatchObject
pub mod patch_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "patch-object")]
    pub struct PatchObject {
        pub object_id: String,
        pub field: String,
        pub value: Option<String>,
        pub delta: Option<f64>,
    }

    pub fn handle(payload: &PatchObject, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        if payload.object_id.is_empty() {
            return Ok(Emit::default());
        }
        if let Some(emit) = origin_delta_emit(doc, "patchObject", vec![payload.object_id.clone()], &payload.field, payload.value.as_ref(), payload.delta) {
            return Ok(emit);
        }
        Ok(patch_emit(doc, std::slice::from_ref(&payload.object_id), &payload.field, payload.value.as_ref(), payload.delta))
    }
}
//#endregion 🔖️PatchObject

//#region 🔖️PatchSelection
pub mod patch_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "patch-selection")]
    pub struct PatchSelection {
        pub object_ids: Vec<String>,
        pub field: String,
        pub value: Option<String>,
        pub delta: Option<f64>,
    }

    pub fn handle(payload: &PatchSelection, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let ids = ids_or_selection(&payload.object_ids, &ctx.interaction.ids);
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        if let Some(emit) = origin_delta_emit(doc, "patchSelection", ids.clone(), &payload.field, payload.value.as_ref(), payload.delta) {
            return Ok(emit);
        }
        Ok(patch_emit(doc, &ids, &payload.field, payload.value.as_ref(), payload.delta))
    }
}
//#endregion 🔖️PatchSelection

//#region 🔖️DeleteObject
pub mod delete_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "delete-object")]
    pub struct DeleteObject {
        pub object_id: String,
    }

    pub fn handle(payload: &DeleteObject, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let models = cad_pane_models(doc.snapshot, &doc.children);
        let leaves = delete_object_leaves(&models, &payload.object_id);
        Ok(cad_child_leaves_emit(&models, None, leaves))
    }
}
//#endregion 🔖️DeleteObject

//#region 🔖️DuplicateObject
pub mod duplicate_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "duplicate-object")]
    pub struct DuplicateObject {
        pub object_id: String,
    }

    pub fn handle(payload: &DuplicateObject, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let entries = duplicate_object_entries(&cad_pane_models(doc.snapshot, &doc.children), &payload.object_id);
        Ok(cad_transform_tool_emit(doc, "duplicateObject", entries))
    }
}
//#endregion 🔖️DuplicateObject
