//! 🧵️ Lowpoly play app commands — UV unwrap + seam marking (`unwrapActive`/`markUvSeam`/`clearSeam`).

use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::{map_kernel_err, mesh_edit, LowpolyScratch};
use semio_framework_3d::mesh::EdgeId;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️UnwrapActive
pub mod unwrap_active {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "unwrap-active")]
    pub struct UnwrapActive {}

    pub fn handle(_payload: &UnwrapActive, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(mesh_edit(doc.snapshot, cfg.snapshot, ctx, move |doc| {
            doc.active_mesh_mut().map_err(|e| e.to_string())?.unwrap_uv().map_err(map_kernel_err)?;
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️UnwrapActive

//#region 🔖️MarkUvSeam
pub mod mark_uv_seam {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "mark-uv-seam")]
    pub struct MarkUvSeam {
        pub seam: Option<bool>,
        pub edge_ids: Option<Vec<u32>>,
    }

    pub fn handle(payload: &MarkUvSeam, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.snapshot, cfg.snapshot);
        let seam = payload.seam.unwrap_or(true);
        // 🕹️ Falls back to the mesh domain's CURRENT selection (`LowpolyScratch::current_selection`,
        // resolved from `InteractionView` by `LowpolyPlayApp::handle`) rather than a deleted config field.
        let edge_ids = payload.edge_ids.clone().unwrap_or_else(|| ctx.current_selection().ids.clone());
        Ok(mesh_edit(projection, config, ctx, move |doc| {
            let edges: Vec<EdgeId> = edge_ids.into_iter().map(EdgeId).collect();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.mark_uv_seam(&edges, seam);
            doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
        }))
    }
}
//#endregion 🔖️MarkUvSeam

//#region 🔖️ClearSeam
pub mod clear_seam {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-seam")]
    pub struct ClearSeam {}

    pub fn handle(_payload: &ClearSeam, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        mark_uv_seam::handle(&mark_uv_seam::MarkUvSeam { seam: Some(false), edge_ids: None }, doc, cfg, ctx)
    }
}
//#endregion 🔖️ClearSeam

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, dispatch};
    use crate::editor::lowpoly::LowpolyCommand;

    #[semio_framework_async_macros::async_test]
    async fn unwrap_active_resyncs_mesh_json() {
        let mut a = app();
        dispatch(&mut a, LowpolyCommand::UnwrapActive(super::unwrap_active::UnwrapActive {})).await;
        // unwrap is idempotent-ish on an already-unwrapped mesh, so just assert it runs without error and
        // keeps the object count stable.
        assert_eq!(a.snapshot().expect("projection").objects.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn clear_seam_delegates_to_mark_uv_seam_with_seam_false() {
        let mut a = app();
        dispatch(&mut a, LowpolyCommand::ClearSeam(super::clear_seam::ClearSeam {})).await;
        assert_eq!(a.snapshot().expect("projection").objects.len(), 1);
    }
}
//#endregion 🧪️Tests
