//! 🗑️ Lowpoly play app commands — whole-object and component removal, and object duplication.
//! `deleteSelection` (Delete/Backspace) removes what the mesh domain currently selects: whole objects at
//! object granularity (`DeleteObject` per object), else the selected faces / the faces along the selected
//! edges / the faces around the selected vertices of the active object (a mesh rebuild, the faces variant
//! every polygon modeller ships). `duplicateObject` (⌘D) copies the active object beside itself.

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::{mesh_edit, LowpolyScratch};
use crate::editor::lowpoly::view::resolve_active_object_id;
use crate::op::LowpolyMutation;
use crate::{LowpolyObject, LowpolySnapshot};
use semio_framework_3d::mesh::{EdgeId, FaceId, VertexId};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let selection = ctx.current_selection().clone();
        let component_level = matches!(selection.mode.as_str(), "vertex" | "edge" | "face") && !selection.ids.is_empty();
        if !component_level {
            // 🗿️ Object granularity: every selected object, else the active one — never the last object.
            // 🗿️ Object granularity: every selected object still in the document (a selection may name an
            // object a previous delete already removed), else the active one.
            let mut ids: Vec<String> = ctx.selected_object_ids().iter().filter(|id| projection.objects.iter().any(|object| &&object.id == id)).cloned().collect();
            if ids.is_empty() {
                ids.push(ctx.selection_object_id().filter(|id| projection.objects.iter().any(|object| object.id == *id)).map_or_else(|| resolve_active_object_id(projection, config), str::to_string));
            }
            ids.retain(|id| projection.objects.iter().any(|object| &object.id == id));
            if ids.is_empty() {
                return Err(Fault::from("lowpoly delete: nothing selected"));
            }
            if ids.len() >= projection.objects.len() {
                return Err(Fault::from("lowpoly delete: a document keeps at least one object"));
            }
            let mutations = ids.into_iter().map(|id| LowpolyMutation::DeleteObject(crate::mutations::delete_object::DeleteObject { id })).collect();
            return Ok(Emit::commit(mutations, "Delete objects"));
        }
        let ids = selection.ids;
        let mode = selection.mode;
        mesh_edit(projection, config, ctx, move |doc| {
            match mode.as_str() {
                "face" => doc.delete_faces(&ids.iter().map(|id| FaceId(*id)).collect::<Vec<_>>()),
                "edge" => doc.delete_edges(&ids.iter().map(|id| EdgeId(*id)).collect::<Vec<_>>()),
                _ => doc.delete_vertices(&ids.iter().map(|id| VertexId(*id)).collect::<Vec<_>>()),
            }
            .map_err(|e| e.to_string())?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        })
        .map_err(Fault::from)
    }
}
//#endregion 🔖️DeleteSelection

//#region 🔖️DuplicateObject
pub mod duplicate_object {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "duplicate-object")]
    pub struct DuplicateObject {
        pub object_id: Option<String>,
    }

    /// 🧬️ The copy is a fresh object id (`obj-<next serial>`) with the same geometry (its own
    /// content-addressed handle) and paint, named "<name> copy", offset one unit along X so it is visible.
    pub fn handle(payload: &DuplicateObject, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let source_id = payload.object_id.clone().or_else(|| ctx.selection_object_id().map(str::to_string)).unwrap_or_else(|| resolve_active_object_id(projection, config));
        let source = projection.objects.iter().find(|object| object.id == source_id).ok_or_else(|| Fault::from(format!("lowpoly duplicate: object {source_id} is not in the document")))?;
        if source.mesh_content.is_empty() {
            return Err(Fault::from(format!("lowpoly duplicate: object {source_id} carries no mesh content")));
        }
        let serial = projection.objects.iter().filter_map(|object| object.id.strip_prefix("obj-")?.parse::<u32>().ok()).max().unwrap_or(100) + 1;
        let id = format!("obj-{serial}");
        let mut transform = source.transform.clone();
        transform.position[0] += 1.0;
        let copy = LowpolyObject {
            id: id.clone(),
            name: format!("{} copy", source.name),
            transform,
            smooth_shading: source.smooth_shading,
            mesh: Some(crate::mesh_child_handle(&id, &source.mesh_content)),
            paint_layers: source.paint_layers.iter().cloned().map(crate::LowpolyPaintLayer::compacted).collect(),
            mesh_content: source.mesh_content.clone(),
        };
        let index = projection.objects.iter().position(|object| object.id == source_id).map_or(projection.objects.len(), |position| position + 1);
        // 🎯️ The copy becomes the selection too — the mesh domain's selection is framework-owned, so the
        // app asks the host to redispatch `interactionSelect` on the copy's row (a Delete right after a
        // ⌘D then removes the copy, not the still-selected source).
        let targets = serde_json::to_string(&vec![serde_json::json!({ "granularity": crate::editor::lowpoly::view::MESH_GRANULARITY_OBJECT, "id": crate::editor::lowpoly::view::document_object_row_id(&id) })]).unwrap_or_default();
        let select = semio_framework::kernel::Effect::DispatchAction {
            req: semio_framework_plugin::RequestId(1_302),
            action: semio_framework::INTERACTION_SELECT_ACTION_ID.into(),
            args: semio_framework::optional_json_to_dsl(Some(serde_json::json!({ "domainId": crate::editor::lowpoly::view::MESH_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" }))),
            delay_ms: 0,
        };
        Ok(Emit {
            artifact_mutations: vec![LowpolyMutation::CreateObject(crate::mutations::create_object::CreateObject { index, object: copy })],
            config_mutations: vec![LowpolyConfigMutation::SetActiveObject { object_id: id }],
            effects: vec![select],
            ..Default::default()
        })
    }
}
//#endregion 🔖️DuplicateObject

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
