//! 🧱️ CAD play app commands — object lifecycle: create, patch (single and multi-selection), delete, duplicate.
//!
//! 🪆️ Object data lives inside the pane's composed `s.stdio.semio.model` CHILD document, and a parent
//! diff never embeds a child diff (`🔖️Composition` in `🏪️store/🦀️.rs`). Each handler here therefore
//! resolves the addressed pane's live working objects out of that child's in-process materialization,
//! decides the next object list, and emits one bounded parent op whose diff RE-MINTS the pane's
//! content-addressed child handle with the updated `CadWorkingScene` attached — the re-materialization
//! seam, the same shape `🌊️flow`'s `create-widget`/`move-widgets` use for their own content child.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{cad_pane_from_view, cad_pane_objects, create_object_mutations, delete_object_mutations, duplicate_object_mutations, ids_or_selection, make_object_for_typology, patch_objects_mutations};
use crate::op::CadMutation;
use crate::{CadPaneId, CadSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪟️ The pane a document-mutating object gesture lands in: the addressed window instance when the
/// host supplied one, otherwise the shape pane (the layout's first quadrant).
fn target_pane(ctx: &CadDispatchCtx) -> CadPaneId {
    ctx.view_state.as_ref().and_then(|view| cad_pane_from_view(view).ok()).unwrap_or(CadPaneId::Shape)
}

//#region 🔖️AddObject
pub mod add_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "add-object")]
    pub struct AddObject {
        pub typology: Option<String>,
    }

    pub fn handle(payload: &AddObject, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let pane = target_pane(ctx);
        let typology = payload.typology.clone().unwrap_or_else(|| "spatial.shape.primitive.box".to_string());
        let object = make_object_for_typology(&typology, cad_pane_objects(doc.snapshot, pane).len(), pane);
        Ok(Emit::mutations(create_object_mutations(doc.snapshot, pane, object)))
    }
}
//#endregion 🔖️AddObject

//#region 🔖️PatchObject
pub mod patch_object {
    use super::*;
    use crate::editor::cad::command_value_json;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
        let value = payload.value.as_deref().map(|value| command_value_json(&payload.field, value));
        let delta = payload.delta.map(protocol::DslValue::float);
        Ok(Emit::mutations(patch_objects_mutations(doc.snapshot, std::slice::from_ref(&payload.object_id), &payload.field, value.as_ref(), delta.as_ref())))
    }
}
//#endregion 🔖️PatchObject

//#region 🔖️PatchSelection
pub mod patch_selection {
    use super::*;
    use crate::editor::cad::command_value_json;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
        let value = payload.value.as_deref().map(|value| command_value_json(&payload.field, value));
        let delta = payload.delta.map(protocol::DslValue::float);
        Ok(Emit::mutations(patch_objects_mutations(doc.snapshot, &ids, &payload.field, value.as_ref(), delta.as_ref())))
    }
}
//#endregion 🔖️PatchSelection

//#region 🔖️DeleteObject
pub mod delete_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "delete-object")]
    pub struct DeleteObject {
        pub object_id: String,
    }

    pub fn handle(payload: &DeleteObject, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::mutations(delete_object_mutations(doc.snapshot, &payload.object_id)))
    }
}
//#endregion 🔖️DeleteObject

//#region 🔖️DuplicateObject
pub mod duplicate_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "duplicate-object")]
    pub struct DuplicateObject {
        pub object_id: String,
    }

    pub fn handle(payload: &DuplicateObject, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::mutations(duplicate_object_mutations(doc.snapshot, &payload.object_id)))
    }
}
//#endregion 🔖️DuplicateObject
