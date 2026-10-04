//! 🧲️ Lowpoly play app commands — the transform gumball: translate, rotate and scale the mesh-domain selection.
//!
//! 🛠️ Every gumball gesture runs through the lowpoly TOOL (`🖌️session`'s `lowpoly_tool` statechart under the
//! `🛠️tool-machine` runner). Both hosts accumulate a drag locally (`World3dHost`'s instance preview, the wgpu world
//! engine's gesture) and dispatch its NET pose delta once on release, so one gesture is one dispatch, one
//! `ToolTransaction`, one edit and one history row whose relative leaves (`move-`/`rotate-`/`scale-selection`, the
//! literal object, vertices, pivot and parameters) time travel edits. A cancelled drag dispatches nothing and leaves
//! zero trace (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5,
//! §17.6).

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::{lowpoly_tool_emit, LowpolyScratch};
use crate::editor::lowpoly::view::try_build_doc;
use crate::mutations::{move_selection::MoveSelection, rotate_selection::RotateSelection, scale_selection::ScaleSelection};
use crate::op::LowpolyMutation;
use crate::LowpolySnapshot;
use semio_framework_3d::mesh::VertexId;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🎯️Selection
/// 🎯️ The motion one gumball dispatch states, before it is bound to the selection's objects and vertices.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LowpolyGumballMotion {
    Translate([f32; 3]),
    Rotate { axis: [f32; 3], angle: f32 },
    Scale([f32; 3]),
}

/// 🧮️ The relative leaves one gumball dispatch yields on `projection`: a component selection (vertices, edges, faces
/// of the active object) moves its vertices about their centroid; an object selection moves every vertex of each
/// selected object (the active one when none is named) about the centroid of them all — one leaf per object, ONE
/// transaction. The leaves name what the selection resolves to NOW, literally, so they replay without the selection.
pub fn lowpoly_gumball_leaves(projection: &LowpolySnapshot, config: &LowpolyConfig, ctx: &LowpolyScratch, motion: LowpolyGumballMotion) -> Result<Vec<LowpolyMutation>, Fault> {
    let doc = try_build_doc(projection, config, ctx).map_err(|error| Fault::from(format!("lowpoly gumball: compute session refused: {error}")))?;
    let component = matches!(doc.selection().mode.as_str(), "vertex" | "edge" | "face");
    let targets: Vec<(String, Vec<u32>)> = if component {
        let vertices = doc.selection_vertex_ids().map_err(|error| Fault::from(format!("lowpoly gumball: {error}")))?;
        if vertices.is_empty() {
            return Ok(Vec::new());
        }
        vec![(doc.active_object_id().to_string(), vertices.into_iter().map(|vertex| vertex.0).collect())]
    } else {
        let named: Vec<String> = ctx.selected_object_ids().iter().filter(|id| projection.objects.iter().any(|object| &object.id == *id)).cloned().collect();
        let objects = if named.is_empty() { vec![doc.active_object_id().to_string()] } else { named };
        objects.into_iter().map(|object| (object, Vec::new())).collect()
    };
    let mut sum = [0.0_f64; 3];
    let mut count = 0_usize;
    for (object_id, vertices) in &targets {
        let Some(mesh) = doc.object_index(object_id).ok().and_then(|index| doc.mesh_at(index)) else { continue };
        let ids: Vec<VertexId> = if vertices.is_empty() { (0..mesh.vertex_count() as u32).map(VertexId).collect() } else { vertices.iter().copied().map(VertexId).collect() };
        for vertex in ids {
            let Ok(position) = mesh.vertex_position(vertex) else { continue };
            for axis in 0..3 {
                sum[axis] += f64::from(position.0[axis]);
            }
            count += 1;
        }
    }
    let pivot = if count == 0 { [0.0; 3] } else { sum.map(|value| (value / count as f64) as f32) };
    Ok(targets
        .into_iter()
        .map(|(object_id, vertex_ids)| match motion {
            LowpolyGumballMotion::Translate(offset) => LowpolyMutation::MoveSelection(MoveSelection { object_id, vertex_ids, offset }),
            LowpolyGumballMotion::Rotate { axis, angle } => LowpolyMutation::RotateSelection(RotateSelection { object_id, vertex_ids, pivot, axis, angle }),
            LowpolyGumballMotion::Scale(factor) => LowpolyMutation::ScaleSelection(ScaleSelection { object_id, vertex_ids, pivot, factor }),
        })
        .collect())
}

/// 📤️ One gumball dispatch through the tool: its leaves as ONE committed transaction of `<appId>#<verb>`, or nothing.
fn gumball_emit(verb: &str, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &LowpolyScratch, motion: LowpolyGumballMotion) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    Ok(lowpoly_tool_emit(verb, doc, lowpoly_gumball_leaves(doc.snapshot, cfg.snapshot, ctx, motion)?))
}
//#endregion 🎯️Selection

//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub dx: f32,
        pub dy: f32,
        pub dz: f32,
    }

    pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        gumball_emit("translateSelection", doc, cfg, ctx, LowpolyGumballMotion::Translate([payload.dx, payload.dy, payload.dz]))
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub ax: f32,
        pub ay: f32,
        pub az: f32,
        pub angle: f32,
    }

    pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        gumball_emit("rotateSelection", doc, cfg, ctx, LowpolyGumballMotion::Rotate { axis: [payload.ax, payload.ay, payload.az], angle: payload.angle })
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub sx: f32,
        pub sy: f32,
        pub sz: f32,
    }

    pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        gumball_emit("scaleSelection", doc, cfg, ctx, LowpolyGumballMotion::Scale([payload.sx, payload.sy, payload.sz]))
    }
}
//#endregion 🔖️ScaleSelection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
