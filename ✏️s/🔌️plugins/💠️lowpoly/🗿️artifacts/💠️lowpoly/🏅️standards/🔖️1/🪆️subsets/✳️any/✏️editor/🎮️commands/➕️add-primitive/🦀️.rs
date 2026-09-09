//! ➕️ Lowpoly play app command — appends a new primitive mesh object and makes it active. A lone
//! command in its group, so (per TEMPLATE.md §5.7's `module_inception` rule) the payload lives directly
//! at this file's top level rather than in a same-named inner `pub mod`.

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::editor::lowpoly::view::{build_doc, primitive_kind};
use crate::op::LowpolyMutation;
use crate::LowpolySnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️AddPrimitive
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[dsl(keyword = "add-primitive")]
pub struct AddPrimitive {
    pub kind: Option<String>,
}

pub fn handle(payload: &AddPrimitive, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    let projection = doc.snapshot;
    let kind = primitive_kind(payload.kind.as_deref().unwrap_or("box")).to_string();
    let Some(mut build) = build_doc(projection, cfg.snapshot, ctx) else { return Ok(Emit::default()) };
    let Ok(new_id) = build.add_primitive(&kind) else { return Ok(Emit::default()) };
    if build.sync_meshes_to_snapshot().is_err() {
        return Ok(Emit::default());
    }
    ctx.set_mesh_workspace_map(build.mesh_workspace().clone());
    let Some(new_object) = build.snapshot().objects.iter().find(|object| object.id == new_id).cloned() else {
        return Ok(Emit::default());
    };
    let index = projection.objects.len();
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the newly-added object used to also
    // reset the mesh domain's selection/targets to whole-object/empty here — that state is
    // framework-owned `InteractionState` now, only ever mutated by the framework's own injected
    // `interactionSelect` handling, never by an app command's `Emit::config_mutations`.
    Ok(Emit {
        artifact_mutations: vec![LowpolyMutation::CreateObject(crate::mutations::create_object::CreateObject { index, object: new_object })],
        config_mutations: vec![LowpolyConfigMutation::SetActiveObject { object_id: new_id }],
        ..Default::default()
    })
}
//#endregion 🔖️AddPrimitive

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
