//! 🔄️ CAD play app commands — rigid transforms on the current selection plus the declarative model-definition transformations.
//!
//! 🧭️ Each gesture is ONE transform-tool transaction (`crate::editor::cad::modes::edit::tools::transform`): the
//! parametric `drag-`/`rotate-`/`scale-selection` leaf per touched pane, whose diff re-mints that pane's content-addressed
//! composed model child from the transformed working scene, so the gumball drag persists, both world windows re-render,
//! and history edits the drag's own inputs.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::modes::edit::tools::transform::{cad_transform_tool_emit, CadToolEntry, CadTransformRecord};
use crate::editor::cad::{apply_transformation_mutations, ids_or_selection};
use crate::op::CadMutation;
use crate::CadSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "apply-transformation")]
    pub struct ApplyTransformation {
        pub qid: String,
    }

    /// 🔄️ Refused by name while baking into composed pane models is unimplemented (`apply_transformation_mutations`
    /// has nothing to bake into since the composable-artifact migration) — never an empty success.
    pub fn handle(payload: &ApplyTransformation, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mutations = apply_transformation_mutations(doc.snapshot, &payload.qid);
        if mutations.is_empty() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("cad.apply-transformation-unavailable"), format!("applyTransformation cannot bake \"{}\": composed pane models accept no baked transformation yet", payload.qid)));
        }
        Ok(Emit::mutations(mutations))
    }
}
//#endregion 🔖️ApplyTransformation
