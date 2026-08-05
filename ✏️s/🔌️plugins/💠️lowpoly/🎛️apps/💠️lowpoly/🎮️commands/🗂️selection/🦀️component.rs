//! 🗂️ Lowpoly play app commands — selection view state: active object, granularity/ids, component
//! toggles, active paint layer, and the selection method/merge-mode defaults. All config-only (never a
//! document operation).

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigOperation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::apps::lowpoly::view::{apply_component_selection, enable_selection_target_kind, selection_keys_for, selection_targets_from_config};
use crate::artifacts::lowpoly::engine::LowpolyDocument;
use crate::artifacts::lowpoly::op::LowpolyOperation;
use crate::artifacts::lowpoly::LowpolyProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveObject
pub mod set_active_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-object")]
    pub struct SetActiveObject {
        pub object_id: String,
    }

    pub fn handle(payload: &SetActiveObject, doc: &DocumentView<'_, LowpolyProjection>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        if doc.projection.objects.iter().any(|object| object.id == payload.object_id) {
            Ok(Emit::config(vec![LowpolyConfigOperation::SetActiveObject { object_id: payload.object_id.clone() }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetActiveObject

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub mode: String,
        pub ids: Vec<u32>,
    }

    pub fn handle(payload: &SetSelection, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        let normalized = LowpolyDocument::normalize_selection_mode(&payload.mode);
        let keys = selection_keys_for(doc.projection, cfg.projection, &normalized, &payload.ids);
        Ok(Emit::config(vec![LowpolyConfigOperation::SetSelection { mode: normalized, ids: payload.ids.clone() }, LowpolyConfigOperation::SetSelectionKeys { keys }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️ToggleSelectionKind
pub mod toggle_selection_kind {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-selection-kind")]
    pub struct ToggleSelectionKind {
        pub kind: String,
    }

    pub fn handle(payload: &ToggleSelectionKind, _doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        let config = cfg.projection;
        let mut targets = selection_targets_from_config(config);
        let enabled = match payload.kind.as_str() {
            "vertex" => {
                targets.vertex = !targets.vertex;
                targets.vertex
            }
            "edge" => {
                targets.edge = !targets.edge;
                targets.edge
            }
            "face" => {
                targets.face = !targets.face;
                targets.face
            }
            _ => {
                targets.mesh = !targets.mesh;
                targets.mesh
            }
        };
        let mut config_operations = vec![LowpolyConfigOperation::SetSelectionTargets { mesh: targets.mesh, vertex: targets.vertex, edge: targets.edge, face: targets.face }];
        if enabled {
            config_operations.push(LowpolyConfigOperation::SetSelection { mode: LowpolyDocument::normalize_selection_mode(&payload.kind), ids: config.selection_ids.clone() });
            config_operations.push(LowpolyConfigOperation::SetHoveredTarget { object_id: None, mode: None, id: None });
            config_operations.push(LowpolyConfigOperation::SetHoveredObject { object_id: None });
        }
        Ok(Emit::config(config_operations))
    }
}
//#endregion 🔖️ToggleSelectionKind

//#region 🔖️ToggleSelectionTarget
pub mod toggle_selection_target {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-selection-target")]
    pub struct ToggleSelectionTarget {
        pub object_id: String,
        pub mode: String,
        pub id: u32,
        pub merge: String,
    }

    pub fn handle(payload: &ToggleSelectionTarget, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        let (projection, config) = (doc.projection, cfg.projection);
        if !projection.objects.iter().any(|object| object.id == payload.object_id) {
            return Ok(Emit::default());
        }
        let (new_mode, ids, keys, targets) = apply_component_selection(config, projection, &payload.mode, &[payload.id], &payload.merge);
        Ok(Emit::config(vec![
            LowpolyConfigOperation::SetActiveObject { object_id: payload.object_id.clone() },
            LowpolyConfigOperation::SetSelectionTargets { mesh: targets.mesh, vertex: targets.vertex, edge: targets.edge, face: targets.face },
            LowpolyConfigOperation::SetSelection { mode: new_mode, ids },
            LowpolyConfigOperation::SetSelectionKeys { keys },
        ]))
    }
}
//#endregion 🔖️ToggleSelectionTarget

//#region 🔖️SetActivePaintLayer
pub mod set_active_paint_layer {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-paint-layer")]
    pub struct SetActivePaintLayer {
        pub layer_index: u32,
    }

    pub fn handle(payload: &SetActivePaintLayer, _doc: &DocumentView<'_, LowpolyProjection>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigOperation::SetActivePaintLayer { value: payload.layer_index }]))
    }
}
//#endregion 🔖️SetActivePaintLayer

//#region 🔖️SetSelectionMethod
pub mod set_selection_method {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection-method")]
    pub struct SetSelectionMethod {
        pub value: String,
    }

    pub fn handle(payload: &SetSelectionMethod, _doc: &DocumentView<'_, LowpolyProjection>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigOperation::SetSelectionMethod { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetSelectionMethod

//#region 🔖️SetSelectionModeDefault
pub mod set_selection_mode_default {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection-mode-default")]
    pub struct SetSelectionModeDefault {
        pub value: String,
    }

    pub fn handle(payload: &SetSelectionModeDefault, _doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        let next = match payload.value.as_str() {
            "additive" | "subtractive" | "invertive" | "default" => payload.value.clone(),
            _ => cfg.projection.selection_mode_default.clone(),
        };
        Ok(Emit::config(vec![LowpolyConfigOperation::SetSelectionModeDefault { value: next }]))
    }
}
//#endregion 🔖️SetSelectionModeDefault

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;

    #[test]
    fn selection_is_view_state_and_emits_no_operations() {
        let mut a = app();
        let result = dispatch(&mut a, LowpolyCommand::WorldPick(crate::apps::lowpoly::commands::world::world_pick::WorldPick { granularity: "face".into(), merge: "replace".into(), id: Some(0) }));
        assert!(result.operations.is_empty(), "picking must not create an undoable operation");
    }
}
//#endregion 🧪️Tests
