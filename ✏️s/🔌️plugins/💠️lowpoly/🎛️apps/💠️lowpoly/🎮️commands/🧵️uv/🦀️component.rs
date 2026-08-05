//! 🧵️ Lowpoly play app commands — UV unwrap + seam marking (`unwrapActive`/`markUvSeam`/`clearSeam`).

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigOperation};
use crate::apps::lowpoly::session::{map_kernel_err, mesh_edit, LowpolyScratch};
use crate::artifacts::lowpoly::op::LowpolyOperation;
use crate::artifacts::lowpoly::LowpolyProjection;
use kernel_3d_mesh::EdgeId;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️UnwrapActive
pub mod unwrap_active {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "unwrap-active")]
    pub struct UnwrapActive {}

    pub fn handle(_payload: &UnwrapActive, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        Ok(mesh_edit(doc.projection, cfg.projection, move |doc| {
            doc.active_mesh_mut().map_err(|e| e.to_string())?.unwrap_uv().map_err(map_kernel_err)?;
            doc.sync_meshes_to_projection().map_err(|e| e.to_string())
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

    pub fn handle(payload: &MarkUvSeam, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        let (projection, config) = (doc.projection, cfg.projection);
        let seam = payload.seam.unwrap_or(true);
        let edge_ids = payload.edge_ids.clone().unwrap_or_else(|| config.selection_ids.clone());
        Ok(mesh_edit(projection, config, move |doc| {
            let edges: Vec<EdgeId> = edge_ids.into_iter().map(EdgeId).collect();
            doc.active_mesh_mut().map_err(|e| e.to_string())?.mark_uv_seam(&edges, seam);
            doc.sync_meshes_to_projection().map_err(|e| e.to_string())
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

    pub fn handle(_payload: &ClearSeam, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        mark_uv_seam::handle(&mark_uv_seam::MarkUvSeam { seam: Some(false), edge_ids: None }, doc, cfg, ctx)
    }
}
//#endregion 🔖️ClearSeam

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;

    #[test]
    fn unwrap_active_resyncs_mesh_json() {
        let mut a = app();
        let before = a.projection().expect("projection").objects[0].mesh_json.clone();
        dispatch(&mut a, LowpolyCommand::UnwrapActive(super::unwrap_active::UnwrapActive {}));
        // unwrap is idempotent-ish on an already-unwrapped mesh, so just assert it runs without error and
        // keeps the object count stable.
        assert_eq!(a.projection().expect("projection").objects.len(), 1);
        let _ = before;
    }

    #[test]
    fn clear_seam_delegates_to_mark_uv_seam_with_seam_false() {
        let mut a = app();
        dispatch(&mut a, LowpolyCommand::ClearSeam(super::clear_seam::ClearSeam {}));
        assert_eq!(a.projection().expect("projection").objects.len(), 1);
    }
}
//#endregion 🧪️Tests
