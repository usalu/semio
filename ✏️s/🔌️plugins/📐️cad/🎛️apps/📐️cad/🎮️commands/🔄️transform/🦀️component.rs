//! 🔄️ CAD play app commands — rigid transforms on the current selection plus the declarative model-definition transformations.

use crate::apps::cad::config::{CadConfig, CadConfigOperation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadOperation;
use crate::artifacts::cad::CadProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{apply_transformation_operations, ids_or_selection, runtime_of};


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

    pub fn handle(payload: &TranslateSelection, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        let runtime = runtime_of(cfg);
        let ids = ids_or_selection(&payload.object_ids, runtime.selected_object_ids.as_slice());
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::amend(vec![CadOperation::TranslateObjects { object_ids: ids, dx: payload.dx, dy: payload.dy, dz: payload.dz }], "gumball.translate"))
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

    pub fn handle(payload: &RotateSelection, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        let runtime = runtime_of(cfg);
        let ids = ids_or_selection(&payload.object_ids, runtime.selected_object_ids.as_slice());
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::amend(vec![CadOperation::RotateObjects { object_ids: ids, ax: payload.ax, ay: payload.ay, az: payload.az, angle: payload.angle }], "gumball.rotate"))
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

    pub fn handle(payload: &ScaleSelection, _doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        let runtime = runtime_of(cfg);
        let ids = ids_or_selection(&payload.object_ids, runtime.selected_object_ids.as_slice());
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::amend(vec![CadOperation::ScaleObjects { object_ids: ids, sx: payload.sx, sy: payload.sy, sz: payload.sz }], "gumball.scale"))
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

    pub fn handle(payload: &ApplyTransformation, doc: &DocumentView<'_, CadProjection>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        Ok(Emit::operations(apply_transformation_operations(doc.projection, &payload.qid)))
    }
}
//#endregion 🔖️ApplyTransformation
