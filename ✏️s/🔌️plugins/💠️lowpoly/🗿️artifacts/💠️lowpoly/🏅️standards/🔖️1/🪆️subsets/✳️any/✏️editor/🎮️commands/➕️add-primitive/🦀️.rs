//! ➕️ Lowpoly play app command — appends a new primitive mesh object and makes it active. A lone
//! command in its group, so (per TEMPLATE.md §5.7's `module_inception` rule) the payload lives directly
//! at this file's top level rather than in a same-named inner `pub mod`.

use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation, SetActiveObjectEdit};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::LowpolyScratch;
use semio_s_artifact_lowpoly_lowpoly_view::{build_doc, primitive_kind};
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️AddPrimitive
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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
    let Some(mut new_object) = build.snapshot().objects.iter().find(|object| object.id == new_id).cloned() else {
        return Ok(Emit::default());
    };
    new_object.paint_layers = new_object.paint_layers.into_iter().map(semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolyPaintLayer::compacted).collect();
    let index = projection.objects.len();
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the newly-added object used to also
    // reset the mesh domain's selection/targets to whole-object/empty here — that state is
    // framework-owned `InteractionState` now, only ever mutated by the framework's own injected
    // `interactionSelect` handling, never by an app command's `Emit::config_mutations`.
    Ok(Emit {
        artifact_mutations: vec![LowpolyMutation::CreateObject(semio_s_artifact_lowpoly_lowpoly_mut_objects::create_object::CreateObject { index, object: new_object })],
        config_mutations: vec![LowpolyConfigMutation::SetActiveObject(SetActiveObjectEdit { object_id: new_id })],
        ..Default::default()
    })
}
//#endregion 🔖️AddPrimitive

//#region 🧪️Tests
//#endregion 🧪️Tests
