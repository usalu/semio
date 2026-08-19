//! 🔄️ CAD play app commands — rigid transforms on the current selection plus the declarative model-definition transformations.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::editor::cad::{apply_transformation_mutations, ids_or_selection};


//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub object_ids: Vec<String>,
        pub dx: f64,
        pub dy: f64,
        pub dz: f64,
    }

    pub async fn handle(payload: &TranslateSelection, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let ids = ids_or_selection(&payload.object_ids, &ctx.interaction.ids);
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        // ⚠️ `drag-objects` retired (26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 3): object
        // placement now lives inside composed `s.stdio.semio.model` CHILD documents; no
        // child-dispatch seam exists yet on `Emit<CadMutation, _>`. Documented no-op.
        let _ = (ids, payload.dx, payload.dy, payload.dz);
        Ok(Emit::default())
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub object_ids: Vec<String>,
        pub ax: f64,
        pub ay: f64,
        pub az: f64,
        pub angle: f64,
    }

    pub async fn handle(payload: &RotateSelection, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let ids = ids_or_selection(&payload.object_ids, &ctx.interaction.ids);
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        // ⚠️ Same documented gap as `translate_selection` — `rotate-objects` retired.
        let _ = (ids, payload.ax, payload.ay, payload.az, payload.angle);
        Ok(Emit::default())
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub object_ids: Vec<String>,
        pub sx: f64,
        pub sy: f64,
        pub sz: f64,
    }

    pub async fn handle(payload: &ScaleSelection, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let ids = ids_or_selection(&payload.object_ids, &ctx.interaction.ids);
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        // ⚠️ Same documented gap as `translate_selection` — `scale-objects` retired.
        let _ = (ids, payload.sx, payload.sy, payload.sz);
        Ok(Emit::default())
    }
}
//#endregion 🔖️ScaleSelection

//#region 🔖️ApplyTransformation
pub mod apply_transformation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "apply-transformation")]
    pub struct ApplyTransformation {
        pub qid: String,
    }

    pub async fn handle(payload: &ApplyTransformation, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::mutations(apply_transformation_mutations(doc.snapshot, &payload.qid)))
    }
}
//#endregion 🔖️ApplyTransformation
