//! 🔷️ Lowpoly play app commands — mesh-editing operations that run a kernel edit against the active
//! object's compute session and emit the resulting `Objects(Patch)` diff (`extrude`/`inset`/`bevel`/
//! `loopCut`/`subdivide`/`triangulate`/`mirror`/`decimate`/`flipFaces`/`merge`/`dissolve`/`snap`/
//! `toggleSmooth`). Every handler shares `crate::editor::lowpoly::session::mesh_edit`, threading `ctx`
//! through it — `mesh_edit` needs the session-local `mesh_workspace` cache (round 2 of ticket
//! 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's round-trip law fix) to build/re-sync the compute
//! session, even though no handler here reads `ctx` directly itself.

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::{map_kernel_err, mesh_edit, LowpolyScratch};
use crate::editor::lowpoly::view::{mirror_axis_from_param, utility_param_f32, utility_param_u32, utility_params_value};
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_framework_3d::mesh::{FaceId, MirrorAxis, WeldMode};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Extrude
pub mod extrude {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "extrude")]
    pub struct Extrude {
        pub extrude_distance: Option<f32>,
    }

    pub async fn handle(payload: &Extrude, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let params = utility_params_value(config);
        let distance = payload.extrude_distance.unwrap_or_else(|| utility_param_f32(&params, "extrudeDistance", 0.25));
        Ok(mesh_edit(projection, config, ctx, move |doc| {
            let faces = doc.selected_face_ids();
            if faces.is_empty() {
                return Err("no faces selected".into());
            }
            doc.active_mesh_mut().map_err(|e| e.to_string())?.extrude_faces(&faces, distance).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Extrude

//#region 🔖️Inset
pub mod inset {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "inset")]
    pub struct Inset {
        pub inset_amount: Option<f32>,
    }

    pub async fn handle(payload: &Inset, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let params = utility_params_value(config);
        let amount = payload.inset_amount.unwrap_or_else(|| utility_param_f32(&params, "insetAmount", 0.1));
        Ok(mesh_edit(projection, config, ctx, move |doc| {
            let faces = doc.selected_face_ids();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.inset_faces(&faces, amount).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Inset

//#region 🔖️Bevel
pub mod bevel {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "bevel")]
    pub struct Bevel {
        pub bevel_amount: Option<f32>,
        pub bevel_segments: Option<u32>,
    }

    pub async fn handle(payload: &Bevel, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let params = utility_params_value(config);
        let amount = payload.bevel_amount.unwrap_or_else(|| utility_param_f32(&params, "bevelAmount", 0.05));
        let segments = payload.bevel_segments.unwrap_or_else(|| utility_param_u32(&params, "bevelSegments", 1));
        Ok(mesh_edit(projection, config, ctx, move |doc| {
            let edges = doc.selected_edge_ids();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.bevel_edges(&edges, amount, segments).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Bevel

//#region 🔖️LoopCut
pub mod loop_cut {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "loop-cut")]
    pub struct LoopCut {
        pub loop_cuts: Option<u32>,
    }

    pub async fn handle(payload: &LoopCut, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let params = utility_params_value(config);
        let cuts = payload.loop_cuts.unwrap_or_else(|| utility_param_u32(&params, "loopCuts", 1));
        Ok(mesh_edit(projection, config, ctx, move |doc| {
            let edges = doc.selected_edge_ids();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.loop_cut(&edges, cuts).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️LoopCut

//#region 🔖️Subdivide
pub mod subdivide {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "subdivide")]
    pub struct Subdivide {}

    pub async fn handle(_payload: &Subdivide, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(mesh_edit(doc.snapshot, cfg.snapshot, ctx, move |doc| {
            let faces = doc.selected_face_ids();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.subdivide_faces(&faces).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Subdivide

//#region 🔖️Triangulate
pub mod triangulate {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "triangulate")]
    pub struct Triangulate {}

    pub async fn handle(_payload: &Triangulate, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(mesh_edit(doc.snapshot, cfg.snapshot, ctx, move |doc| {
            doc.active_mesh_mut().map_err(|e| e.to_string())?.triangulate().map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Triangulate

//#region 🔖️Mirror
pub mod mirror {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "mirror")]
    pub struct Mirror {
        pub axis: Option<String>,
    }

    pub async fn handle(payload: &Mirror, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let params = utility_params_value(config);
        let axis = payload.axis.as_deref().map_or_else(
            || mirror_axis_from_param(&params),
            |value| match value {
                "y" => MirrorAxis::Y,
                "z" => MirrorAxis::Z,
                _ => MirrorAxis::X,
            },
        );
        Ok(mesh_edit(projection, config, ctx, move |doc| {
            doc.active_mesh_mut().map_err(|e| e.to_string())?.mirror(axis, 0.001).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Mirror

//#region 🔖️Decimate
pub mod decimate {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "decimate")]
    pub struct Decimate {
        pub decimate_ratio: Option<f32>,
    }

    pub async fn handle(payload: &Decimate, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let params = utility_params_value(config);
        let ratio = payload.decimate_ratio.unwrap_or_else(|| utility_param_f32(&params, "decimateRatio", 0.5));
        Ok(mesh_edit(projection, config, ctx, move |doc| {
            doc.active_mesh_mut().map_err(|e| e.to_string())?.decimate(ratio).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Decimate

//#region 🔖️FlipFaces
pub mod flip_faces {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "flip-faces")]
    pub struct FlipFaces {
        pub face_ids: Vec<u32>,
    }

    pub async fn handle(payload: &FlipFaces, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let face_ids = payload.face_ids.clone();
        Ok(mesh_edit(doc.snapshot, cfg.snapshot, ctx, move |doc| {
            let faces: Vec<FaceId> = if !face_ids.is_empty() {
                face_ids.into_iter().map(FaceId).collect()
            } else if !doc.selected_face_ids().is_empty() {
                doc.selected_face_ids()
            } else {
                doc.selection().ids.iter().map(|id| FaceId(*id)).collect()
            };
            doc.active_mesh_mut().map_err(|e| e.to_string())?.flip_faces(&faces).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️FlipFaces

//#region 🔖️Merge
pub mod merge {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "merge")]
    pub struct Merge {}

    pub async fn handle(_payload: &Merge, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(mesh_edit(doc.snapshot, cfg.snapshot, ctx, move |doc| {
            let verts = doc.selected_vertex_ids();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.merge_vertices(&verts, WeldMode::Center, 0.001).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Merge

//#region 🔖️Dissolve
pub mod dissolve {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "dissolve")]
    pub struct Dissolve {}

    pub async fn handle(_payload: &Dissolve, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(mesh_edit(doc.snapshot, cfg.snapshot, ctx, move |doc| {
            let edges = doc.selected_edge_ids();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.dissolve_edges(&edges).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Dissolve

//#region 🔖️Snap
pub mod snap {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "snap")]
    pub struct Snap {}

    pub async fn handle(_payload: &Snap, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let params = utility_params_value(config);
        let grid = utility_param_f32(&params, "snapGrid", 0.25);
        Ok(mesh_edit(projection, config, ctx, move |doc| {
            let verts = doc.selected_vertex_ids();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.snap_vertices_to_grid(&verts, grid).map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️Snap

//#region 🔖️ToggleSmooth
pub mod toggle_smooth {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-smooth")]
    pub struct ToggleSmooth {}

    pub async fn handle(_payload: &ToggleSmooth, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(mesh_edit(doc.snapshot, cfg.snapshot, ctx, move |doc| {
            if let Some(index) = doc.active_index() {
                let smooth = !doc.snapshot().objects[index].smooth_shading;
                doc.snapshot_mut().objects[index].smooth_shading = smooth;
                let faces: Vec<FaceId> = (0..doc.active_mesh().map_err(|e| e.to_string())?.face_count()).map(|index| FaceId(index as u32)).collect();
                let mesh = doc.active_mesh_mut().map_err(|e| e.to_string())?;
                mesh.set_shading(&faces, smooth).map_err(map_kernel_err)?;
                mesh.recompute_normals().map_err(map_kernel_err)?;
            }
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️ToggleSmooth

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, app_with_registry, dispatch, select_face};
    use crate::editor::lowpoly::LowpolyCommand;
    use semio_framework_plugin::PluginApp;

    /// 🧪️ Rewritten (round 2 of ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's round-trip law
    /// fix) to assert on the persisted `mesh` HANDLE rather than reconstructing a `LowpolyDocument`
    /// and counting faces: the live half-edge-mesh JSON no longer lives on `LowpolyObject` at all, and
    /// store-level undo/redo (which this test exercises) bypasses `ArtifactApp::handle` entirely, so
    /// the app's own session-local `mesh_workspace` cache is never resynced by it — there is no
    /// honest way to rebuild real post-undo geometry from outside the live session any more. The
    /// handle still round-trips correctly through undo either way, which is what this test now checks.
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking a face is now the
    /// framework's injected `interactionSelect`, so this needs `app_with_registry()` (the "mesh" domain
    /// must be declared to select against).
    #[semio_framework_async_macros::async_test]
    async fn extrude_selected_face_grows_mesh_and_undo_restores() {
        let mut a = app_with_registry();
        let object_id = a.snapshot().expect("projection").objects[0].id.clone();
        let before_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
        select_face(&mut a, &object_id, 0);
        dispatch(&mut a, LowpolyCommand::Extrude(super::extrude::Extrude { extrude_distance: None }));
        let after_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
        assert_ne!(after_mesh, before_mesh, "extrude must change the mesh handle");
        a.handle_action("undo", None, &semio_framework_plugin::testkit::meta("a")).unwrap();
        let restored_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
        assert_eq!(restored_mesh, before_mesh, "undo restores the pre-extrude mesh handle");
    }

    #[semio_framework_async_macros::async_test]
    async fn extrude_reads_staged_arg_distance_into_the_operation() {
        // 🧪️ Arg-form action: the staged `extrudeDistance` (not the config backing store) drives the edit.
        let mut small = app_with_registry();
        let mut large = app_with_registry();
        let object_id = small.snapshot().expect("projection").objects[0].id.clone();
        select_face(&mut small, &object_id, 0);
        select_face(&mut large, &object_id, 0);
        dispatch(&mut small, LowpolyCommand::Extrude(super::extrude::Extrude { extrude_distance: Some(0.1) }));
        dispatch(&mut large, LowpolyCommand::Extrude(super::extrude::Extrude { extrude_distance: Some(1.5) }));
        let small_handle = small.snapshot().expect("projection").objects.iter().find(|o| o.id == object_id).unwrap().mesh.clone();
        let large_handle = large.snapshot().expect("projection").objects.iter().find(|o| o.id == object_id).unwrap().mesh.clone();
        assert_ne!(small_handle, large_handle, "different staged extrude distances must produce different meshes");
    }

    #[semio_framework_async_macros::async_test]
    async fn toggle_smooth_emits_op_and_flips_shading() {
        let mut a = app();
        let before = a.snapshot().expect("projection").objects[0].smooth_shading;
        dispatch(&mut a, LowpolyCommand::ToggleSmooth(super::toggle_smooth::ToggleSmooth {}));
        assert_ne!(a.snapshot().expect("projection").objects[0].smooth_shading, before);
    }
}
//#endregion 🧪️Tests
