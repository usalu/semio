//! 🗂️ Lowpoly play app commands — view state genuinely outside the "mesh" interaction domain: active
//! object and active paint layer. All config-only (never a document operation).
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `SetSelection`/`ToggleSelectionKind`/
//! `ToggleSelectionTarget`/`SetSelectionMethod`/`SetSelectionModeDefault` are DELETED — the framework's
//! injected `interactionSelect`/`setSelectionMode`/`setInteractionGranularity` verbs (declared via
//! `AppBuilder::interaction`) now own the mesh domain's selection/granularity/mode entirely; see
//! `🧭️view/🦀️component.rs`'s `🔖️MeshDomain` region for the target-id/selection-resolution boundary.

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveObject
pub mod set_active_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-object")]
    pub struct SetActiveObject {
        pub object_id: String,
    }

    pub async fn handle(payload: &SetActiveObject, doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        if doc.snapshot.objects.iter().any(|object| object.id == payload.object_id) {
            Ok(Emit::config(vec![LowpolyConfigMutation::SetActiveObject { object_id: payload.object_id.clone() }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetActiveObject

//#region 🔖️SetActivePaintLayer
pub mod set_active_paint_layer {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-paint-layer")]
    pub struct SetActivePaintLayer {
        pub layer_index: u32,
    }

    pub async fn handle(payload: &SetActivePaintLayer, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigMutation::SetActivePaintLayer { value: payload.layer_index }]))
    }
}
//#endregion 🔖️SetActivePaintLayer

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, dispatch};
    use crate::editor::lowpoly::LowpolyCommand;
    use semio_framework_plugin::PluginApp;

    #[test]
    async fn set_active_object_is_view_state_and_emits_no_operations() {
        let mut a = app();
        let object_id = a.snapshot().expect("projection").objects[0].id.clone();
        let result = dispatch(&mut a, LowpolyCommand::SetActiveObject(super::set_active_object::SetActiveObject { object_id }));
        assert!(result.mutations.is_empty(), "setting the active object must not create an undoable operation");
    }

    #[test]
    async fn set_active_paint_layer_is_view_state_and_emits_no_operations() {
        let mut a = app();
        let result = dispatch(&mut a, LowpolyCommand::SetActivePaintLayer(super::set_active_paint_layer::SetActivePaintLayer { layer_index: 0 }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
