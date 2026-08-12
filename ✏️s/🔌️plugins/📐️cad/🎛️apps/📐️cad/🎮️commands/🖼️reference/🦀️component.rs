//! 🖼️ CAD play app commands — the per-pane reference overlays: patch, select, hover.

use crate::apps::cad::config::{CadConfig, CadConfigMutation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::mutations::change_reference_hidden::mutation::ChangeReferenceHidden;
use crate::artifacts::cad::mutations::change_reference_locked::mutation::ChangeReferenceLocked;
use crate::artifacts::cad::mutations::change_reference_width::mutation::ChangeReferenceWidth;
use crate::artifacts::cad::mutations::move_reference::mutation::MoveReference;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{axis3_index, cad_pane_id_from_suffix, clear_component_selection, command_value_json, resolve_number_edit, runtime_of, snapshot_of};
use crate::artifacts::cad::{cad_pane_from_model_definition_id, CadPaneId};
use serde_json::{json, Value};


//#region 🔖️PatchCadPlayReference
pub mod patch_cad_play_reference {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-cad-play-reference")]
    pub struct PatchCadPlayReference {
        pub model_definition_id: String,
        pub reference_id: String,
        pub field: String,
        pub value: Option<String>,
        pub delta: Option<f64>,
    }

    pub fn handle(payload: &PatchCadPlayReference, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let value_json = payload.value.as_deref().map(|entry| command_value_json(&payload.field, entry));
        let delta_json = payload.delta.map(|entry| json!(entry));
        let mutation = match payload.field.as_str() {
            "hidden" => value_json.as_ref().and_then(Value::as_bool).map(|new_hidden| CadMutation::ChangeReferenceHidden(ChangeReferenceHidden { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_hidden })),
            "locked" => value_json.as_ref().and_then(Value::as_bool).map(|new_locked| CadMutation::ChangeReferenceLocked(ChangeReferenceLocked { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_locked })),
            "widthWorld" => {
                let current = document.references_by_model_definition_id.get(&payload.model_definition_id).and_then(|refs| refs.iter().find(|reference| reference.id == payload.reference_id)).map_or(0.0, |reference| reference.width_world);
                resolve_number_edit(current, value_json.as_ref(), delta_json.as_ref()).map(|new_width_world| CadMutation::ChangeReferenceWidth(ChangeReferenceWidth { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_width_world }))
            }
            _ => axis3_index(&payload.field, "origin").and_then(|axis| {
                let mut origin = document.references_by_model_definition_id.get(&payload.model_definition_id).and_then(|refs| refs.iter().find(|reference| reference.id == payload.reference_id)).map_or([0.0, 0.0, 0.0], |reference| reference.origin);
                let updated = resolve_number_edit(origin[axis], value_json.as_ref(), delta_json.as_ref())?;
                origin[axis] = updated;
                Some(CadMutation::MoveReference(MoveReference { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_origin: origin }))
            }),
        };
        match mutation {
            Some(mutation) => Ok(Emit::mutations(vec![mutation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchCadPlayReference

//#region 🔖️SetReferenceSelection
pub mod set_reference_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reference-selection")]
    pub struct SetReferenceSelection {
        pub pane: Option<String>,
        pub model_definition_id: Option<String>,
        pub reference_id: Option<String>,
    }

    pub fn handle(payload: &SetReferenceSelection, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let pane_id = payload.pane.as_deref().map(cad_pane_id_from_suffix).or_else(|| payload.model_definition_id.as_deref().and_then(cad_pane_from_model_definition_id)).unwrap_or(CadPaneId::Shape);
        runtime.selected_reference_model_definition_id = Some(pane_id.model_definition_id().into());
        runtime.selected_reference_id = payload.reference_id.clone();
        runtime.selected_object_ids.clear();
        runtime.selected_node_ids.clear();
        runtime.selected_primitive_id = None;
        runtime.selected_primitive_kind = None;
        runtime.active_object_id = None;
        clear_component_selection(&mut runtime);
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
    }
}
//#endregion 🔖️SetReferenceSelection

//#region 🔖️ReferenceHover
pub mod reference_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reference-hover")]
    pub struct ReferenceHover {
        pub reference_id: Option<String>,
    }

    pub fn handle(payload: &ReferenceHover, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.hovered_object_id = payload.reference_id.as_deref().map(|id| format!("reference:{id}"));
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
    }
}
//#endregion 🔖️ReferenceHover
