//! 🗂️ CAD play app commands — everything that moves the selection or hover cursor: marquee, world pick, tree select, component selection, hover.

use crate::apps::cad::config::{CadConfig, CadConfigMutation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{apply_component_selection, cad_pane_id_from_suffix, cad_pane_id_from_surface_id, clear_component_selection, resolve_active_object_id, runtime_of, snapshot_of};
use crate::apps::cad::config::CadHoverTarget;
use crate::artifacts::cad::engine::primary_primitive_kind;
use crate::artifacts::cad::{cad_all_objects, cad_pane_objects, CadPaneId};
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};


//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub mode: String,
        pub ids: Vec<u32>,
        pub object_id: Option<String>,
        pub merge: String,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_node_ids.clear();
        runtime.selected_primitive_id = None;
        runtime.selected_primitive_kind = None;
        runtime.selected_reference_model_definition_id = None;
        runtime.selected_reference_id = None;
        let resolved_object_id = payload.object_id.clone().or_else(|| resolve_active_object_id(&runtime));
        apply_component_selection(&mut runtime, &payload.mode, &payload.ids, &payload.merge, resolved_object_id.as_deref());
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SetNodeSelection
pub mod set_node_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-node-selection")]
    pub struct SetNodeSelection {
        pub node_ids: Vec<String>,
    }

    pub fn handle(payload: &SetNodeSelection, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_node_ids = payload.node_ids.clone();
        runtime.selected_object_ids.clear();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]))
    }
}
//#endregion 🔖️SetNodeSelection

//#region 🔖️WorldSelect
pub mod world_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-select")]
    pub struct WorldSelect {
        pub ids: Vec<String>,
        pub merge: String,
    }

    pub fn handle(payload: &WorldSelect, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_object_ids = merge_world_selection_ids(&runtime.selected_object_ids, &payload.ids, &payload.merge);
        runtime.selected_node_ids.clear();
        runtime.selected_primitive_id = None;
        runtime.selected_primitive_kind = None;
        runtime.selected_reference_model_definition_id = None;
        runtime.selected_reference_id = None;
        runtime.active_object_id = runtime.selected_object_ids.first().map(str::to_string);
        clear_component_selection(&mut runtime);
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]))
    }
}
//#endregion 🔖️WorldSelect

//#region 🔖️WorldHover
pub mod world_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-hover")]
    pub struct WorldHover {
        pub object_id: Option<String>,
    }

    pub fn handle(payload: &WorldHover, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.hovered_object_id = payload.object_id.clone();
        runtime.hovered_target = runtime.hovered_object_id.as_ref().map(|object_id| CadHoverTarget { object_id: Some(object_id.clone()), mode: Some("mesh".into()), id: Some(0) });
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]))
    }
}
//#endregion 🔖️WorldHover

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-hover")]
    pub struct SetHover {
        pub object_id: Option<String>,
        pub mode: Option<String>,
        pub id: Option<u32>,
    }

    pub fn handle(payload: &SetHover, doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.projection;
        let mut runtime = runtime_of(cfg);
        if payload.object_id.is_none() {
            runtime.hovered_target = None;
            runtime.hovered_object_id = None;
        } else {
            let mut mode = payload.mode.clone();
            // 🧵️ Curve-primitive objects (structure beams/columns/walls) are whole instances.
            if mode.as_deref() == Some("edge") {
                if let Some(object_id) = payload.object_id.as_deref() {
                    if cad_all_objects(document).find(|(object, _)| object.id == object_id).is_some_and(|(object, _)| primary_primitive_kind(object) == "curve") {
                        mode = Some("mesh".into());
                    }
                }
            }
            runtime.hovered_object_id = payload.object_id.clone();
            runtime.hovered_target = Some(CadHoverTarget { object_id: payload.object_id.clone(), mode, id: payload.id });
        }
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️WorldPick
pub mod world_pick {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pick")]
    pub struct WorldPick {
        pub id: Option<u64>,
        pub merge: String,
        pub granularity: String,
        pub object_id: Option<String>,
        pub surface_id: Option<String>,
        pub pane: Option<String>,
    }

    pub fn handle(payload: &WorldPick, doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.projection;
        let mut runtime = runtime_of(cfg);
        if payload.id.is_none() {
            if payload.merge == "replace" {
                runtime.selected_object_ids.clear();
                runtime.selected_primitive_id = None;
                runtime.selected_primitive_kind = None;
                runtime.active_object_id = None;
                clear_component_selection(&mut runtime);
            }
            return Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]));
        }
        if matches!(payload.granularity.as_str(), "edge" | "face" | "vertex") {
            let resolved_object_id = payload.object_id.clone().or_else(|| runtime.hovered_target.as_ref().and_then(|target| target.object_id.clone())).or_else(|| runtime.hovered_object_id.clone()).or_else(|| resolve_active_object_id(&runtime));
            // 🧵️ Curve centerlines are the model-definition objects — select the instance, not an edge component.
            let curve_object_id = resolved_object_id
                .as_deref()
                .and_then(|object_id| cad_all_objects(document).find(|(object, _)| object.id == object_id).map(|(object, _)| object).filter(|object| primary_primitive_kind(object) == "curve").map(|object| object.id.clone()));
            if let Some(curve_id) = curve_object_id {
                runtime.selected_object_ids = merge_world_selection_ids(&runtime.selected_object_ids, std::slice::from_ref(&curve_id), &payload.merge);
                runtime.active_object_id = Some(curve_id);
                runtime.selected_node_ids.clear();
                runtime.selected_primitive_id = None;
                runtime.selected_primitive_kind = None;
                runtime.selected_reference_model_definition_id = None;
                runtime.selected_reference_id = None;
                clear_component_selection(&mut runtime);
                return Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]));
            }
            let component_id = payload.id.unwrap_or(0) as u32;
            apply_component_selection(&mut runtime, &payload.granularity, &[component_id], &payload.merge, resolved_object_id.as_deref());
            runtime.selected_node_ids.clear();
            runtime.selected_primitive_id = None;
            runtime.selected_primitive_kind = None;
            runtime.selected_reference_model_definition_id = None;
            runtime.selected_reference_id = None;
            return Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]));
        }
        let index = payload.id.unwrap_or(0) as usize;
        let pane_id = payload.surface_id.as_deref().map(cad_pane_id_from_surface_id).or_else(|| payload.pane.as_deref().map(cad_pane_id_from_suffix)).unwrap_or(CadPaneId::Shape);
        if let Some(object) = cad_pane_objects(document, pane_id).iter().filter(|object| object.visible).nth(index) {
            let picked_id = object.id.clone();
            runtime.selected_object_ids = merge_world_selection_ids(&runtime.selected_object_ids, std::slice::from_ref(&picked_id), &payload.merge);
            runtime.active_object_id = Some(picked_id);
            runtime.selected_node_ids.clear();
            runtime.selected_primitive_id = None;
            runtime.selected_primitive_kind = None;
            runtime.selected_reference_model_definition_id = None;
            runtime.selected_reference_id = None;
            clear_component_selection(&mut runtime);
        }
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]))
    }
}
//#endregion 🔖️WorldPick

//#region 🔖️SetSelectionMethod
pub mod set_selection_method {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection-method")]
    pub struct SetSelectionMethod {
        pub method: String,
    }

    pub fn handle(payload: &SetSelectionMethod, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selection_method = payload.method.clone();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]))
    }
}
//#endregion 🔖️SetSelectionMethod

//#region 🔖️SetPrimitiveSelection
pub mod set_primitive_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-primitive-selection")]
    pub struct SetPrimitiveSelection {
        pub object_id: String,
        pub primitive_id: Option<String>,
        pub kind: Option<String>,
    }

    pub fn handle(payload: &SetPrimitiveSelection, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_object_ids = SelectionSet::from(vec![payload.object_id.clone()]);
        runtime.selected_node_ids.clear();
        runtime.selected_primitive_id = payload.primitive_id.clone();
        runtime.selected_primitive_kind = payload.kind.clone();
        runtime.selected_reference_model_definition_id = None;
        runtime.selected_reference_id = None;
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.projection)]))
    }
}
//#endregion 🔖️SetPrimitiveSelection
