//! 🗂️ CAD play app commands — everything that moves the selection or hover cursor: marquee, world pick, tree select, component selection, hover.

use crate::apps::cad::config::{CadConfig, CadConfigMutation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{apply_component_selection, cad_pane_id_from_suffix, cad_pane_id_from_surface_id, clear_component_selection, resolve_active_object_id, runtime_of, snapshot_of};
use crate::apps::cad::config::CadHoverTarget;
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::primary_primitive_kind;
use crate::artifacts::cad::CadPaneId;
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

    pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_node_ids.clear();
        runtime.selected_primitive_id = None;
        runtime.selected_primitive_kind = None;
        runtime.selected_reference_model_definition_id = None;
        runtime.selected_reference_id = None;
        let resolved_object_id = payload.object_id.clone().or_else(|| resolve_active_object_id(&runtime));
        apply_component_selection(&mut runtime, &payload.mode, &payload.ids, &payload.merge, resolved_object_id.as_deref());
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
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

    pub fn handle(payload: &SetNodeSelection, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_node_ids = payload.node_ids.clone();
        runtime.selected_object_ids.clear();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
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

    pub fn handle(payload: &WorldSelect, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_object_ids = merge_world_selection_ids(&runtime.selected_object_ids, &payload.ids, &payload.merge);
        runtime.selected_node_ids.clear();
        runtime.selected_primitive_id = None;
        runtime.selected_primitive_kind = None;
        runtime.selected_reference_model_definition_id = None;
        runtime.selected_reference_id = None;
        runtime.active_object_id = runtime.selected_object_ids.first().map(str::to_string);
        clear_component_selection(&mut runtime);
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
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

    pub fn handle(payload: &WorldHover, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.hovered_object_id = payload.object_id.clone();
        runtime.hovered_target = runtime.hovered_object_id.as_ref().map(|object_id| CadHoverTarget { object_id: Some(object_id.clone()), mode: Some("mesh".into()), id: Some(0) });
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
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

    pub fn handle(payload: &SetHover, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg);
        if payload.object_id.is_none() {
            runtime.hovered_target = None;
            runtime.hovered_object_id = None;
        } else {
            // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the curve-primitive
            // whole-instance special-case used to scan `CadSnapshot`'s inline object list, which no
            // longer exists (object data lives inside composed `s.stdio.semio.model` CHILD
            // documents, unresolved at this boundary). Documented reduced-fidelity gap: `edge` mode
            // is no longer downgraded to `mesh` for curve objects.
            let mode = payload.mode.clone();
            let _ = document;
            runtime.hovered_object_id = payload.object_id.clone();
            runtime.hovered_target = Some(CadHoverTarget { object_id: payload.object_id.clone(), mode, id: payload.id });
        }
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
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

    pub fn handle(payload: &WorldPick, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg);
        if payload.id.is_none() {
            if payload.merge == "replace" {
                runtime.selected_object_ids.clear();
                runtime.selected_primitive_id = None;
                runtime.selected_primitive_kind = None;
                runtime.active_object_id = None;
                clear_component_selection(&mut runtime);
            }
            return Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]));
        }
        if matches!(payload.granularity.as_str(), "edge" | "face" | "vertex") {
            let resolved_object_id = payload.object_id.clone().or_else(|| runtime.hovered_target.as_ref().and_then(|target| target.object_id.clone())).or_else(|| runtime.hovered_object_id.clone()).or_else(|| resolve_active_object_id(&runtime));
            // ⚠️ Same documented gap as `set_hover` — curve-centerline whole-instance selection can
            // no longer scan `CadSnapshot`'s (now-deleted) inline object list.
            let _ = document;
            let curve_object_id: Option<String> = None;
            if let Some(curve_id) = curve_object_id {
                runtime.selected_object_ids = merge_world_selection_ids(&runtime.selected_object_ids, std::slice::from_ref(&curve_id), &payload.merge);
                runtime.active_object_id = Some(curve_id);
                runtime.selected_node_ids.clear();
                runtime.selected_primitive_id = None;
                runtime.selected_primitive_kind = None;
                runtime.selected_reference_model_definition_id = None;
                runtime.selected_reference_id = None;
                clear_component_selection(&mut runtime);
                return Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]));
            }
            let component_id = payload.id.unwrap_or(0) as u32;
            apply_component_selection(&mut runtime, &payload.granularity, &[component_id], &payload.merge, resolved_object_id.as_deref());
            runtime.selected_node_ids.clear();
            runtime.selected_primitive_id = None;
            runtime.selected_primitive_kind = None;
            runtime.selected_reference_model_definition_id = None;
            runtime.selected_reference_id = None;
            return Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]));
        }
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: mesh-level world-pick by
        // pane index used to scan `CadSnapshot`'s inline per-pane object list (now composed
        // `s.stdio.semio.model` CHILD documents, unresolved at this boundary). Documented
        // reduced-fidelity gap: index-based mesh pick no longer resolves an object id.
        let _ = (payload.id, payload.surface_id.as_deref().map(cad_pane_id_from_surface_id), payload.pane.as_deref().map(cad_pane_id_from_suffix), document);
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
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

    pub fn handle(payload: &SetSelectionMethod, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selection_method = payload.method.clone();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
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

    pub fn handle(payload: &SetPrimitiveSelection, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_object_ids = SelectionSet::from(vec![payload.object_id.clone()]);
        runtime.selected_node_ids.clear();
        runtime.selected_primitive_id = payload.primitive_id.clone();
        runtime.selected_primitive_kind = payload.kind.clone();
        runtime.selected_reference_model_definition_id = None;
        runtime.selected_reference_id = None;
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)]))
    }
}
//#endregion 🔖️SetPrimitiveSelection
