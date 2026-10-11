//! 🔄️ CAD play app commands — rigid transforms on the current selection plus the declarative model-definition transformations.
//!
//! 🧭️ Each gesture is ONE transform-tool transaction (`crate::editor::cad::modes::edit::tools::transform`): the
//! relative `drag-`/`rotate-`/`scale-elements` child leaf per touched pane, landed on that pane's composed
//! `s.stdio.semio@v1/model` child (design §20.15), so the gumball drag persists, both world windows re-render, and history
//! edits the drag's own inputs.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::modes::edit::tools::transform::{cad_pane_models, cad_transform_tool_emit, CadToolEntry, CadTransformRecord};
use crate::editor::cad::{apply_transformation_entries, ids_or_selection};
use crate::op::CadMutation;
use crate::CadSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub object_ids: Vec<String>,
        pub dx: f64,
        pub dy: f64,
        pub dz: f64,
    }

    pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let ids = ids_or_selection(&payload.object_ids, &ctx.interaction.ids);
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        Ok(cad_transform_tool_emit(doc, "translateSelection", vec![CadToolEntry::Transform(CadTransformRecord::drag(ids, [payload.dx, payload.dy, payload.dz]))]))
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub object_ids: Vec<String>,
        pub ax: f64,
        pub ay: f64,
        pub az: f64,
        pub angle: f64,
    }

    pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let ids = ids_or_selection(&payload.object_ids, &ctx.interaction.ids);
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        Ok(cad_transform_tool_emit(doc, "rotateSelection", vec![CadToolEntry::Transform(CadTransformRecord::rotate(ids, [payload.ax, payload.ay, payload.az], payload.angle))]))
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub object_ids: Vec<String>,
        pub sx: f64,
        pub sy: f64,
        pub sz: f64,
    }

    pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let ids = ids_or_selection(&payload.object_ids, &ctx.interaction.ids);
        if ids.is_empty() {
            return Ok(Emit::default());
        }
        Ok(cad_transform_tool_emit(doc, "scaleSelection", vec![CadToolEntry::Transform(CadTransformRecord::scale(ids, [payload.sx, payload.sy, payload.sz]))]))
    }
}
//#endregion 🔖️ScaleSelection

//#region 🔖️ApplyTransformation
pub mod apply_transformation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "apply-transformation")]
    pub struct ApplyTransformation {
        pub qid: String,
    }

    /// 🔄️ Refused by name while the derivation rules that bake a transformation into the target pane's model child are
    /// unwritten (`apply_transformation_entries` yields nothing) — never an empty success.
    pub fn handle(payload: &ApplyTransformation, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let entries = apply_transformation_entries(&cad_pane_models(doc.snapshot, &doc.children), &payload.qid);
        if entries.is_empty() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("cad.apply-transformation-unavailable"), format!("applyTransformation cannot bake \"{}\": no derivation rule bakes this transformation into a pane model yet", payload.qid)));
        }
        Ok(cad_transform_tool_emit(doc, "applyTransformation", entries))
    }
}
//#endregion 🔖️ApplyTransformation
