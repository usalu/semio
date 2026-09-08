//! 🗂️ Lowpoly play app commands — view state genuinely outside the "mesh" interaction domain: active
//! object and active paint layer. All config-only (never a document operation).
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `SetSelection`/`ToggleSelectionKind`/
//! `ToggleSelectionTarget`/`SetSelectionMethod`/`SetSelectionModeDefault` are DELETED — the framework's
//! injected `interactionSelect`/`setSelectionMode`/`setInteractionGranularity` verbs (declared via
//! `AppBuilder::interaction`) now own the mesh domain's selection/granularity/mode entirely; see
//! `🧭️view/🦀️.rs`'s `🔖️MeshDomain` region for the target-id/selection-resolution boundary.

use crate::op::LowpolyMutation;
use crate::LowpolySnapshot;
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveObject
pub mod set_active_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-active-object")]
    pub struct SetActiveObject {
        pub object_id: String,
    }

    pub fn handle(payload: &SetActiveObject, doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
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

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-active-paint-layer")]
    pub struct SetActivePaintLayer {
        pub layer_index: u32,
    }

    pub fn handle(payload: &SetActivePaintLayer, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigMutation::SetActivePaintLayer { value: payload.layer_index }]))
    }
}
//#endregion 🔖️SetActivePaintLayer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
