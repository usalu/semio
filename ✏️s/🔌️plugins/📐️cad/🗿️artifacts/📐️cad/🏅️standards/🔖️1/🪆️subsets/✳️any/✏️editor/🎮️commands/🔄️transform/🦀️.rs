//! 🔄️ CAD play app commands — rigid transforms on the current selection plus the declarative model-definition transformations.
//!
//! 🪆️ Each gesture resolves the addressed objects out of their pane's composed `s.stdio.semio.model`
//! child materialization and emits ONE parent op per touched pane; that op's diff re-mints the pane's
//! content-addressed child handle with the transformed working scene attached, so the gumball drag
//! persists and both world windows re-render. See `crate::editor::cad::translate_objects_mutations`.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{apply_transformation_mutations, ids_or_selection, rotate_objects_mutations, scale_objects_mutations, translate_objects_mutations};
use crate::op::CadMutation;
use crate::CadSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
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
        Ok(Emit::mutations(translate_objects_mutations(doc.snapshot, &ids, [payload.dx, payload.dy, payload.dz])))
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
        Ok(Emit::mutations(rotate_objects_mutations(doc.snapshot, &ids, [payload.ax, payload.ay, payload.az], payload.angle)))
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
        Ok(Emit::mutations(scale_objects_mutations(doc.snapshot, &ids, [payload.sx, payload.sy, payload.sz])))
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

    pub fn handle(payload: &ApplyTransformation, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::mutations(apply_transformation_mutations(doc.snapshot, &payload.qid)))
    }
}
//#endregion 🔖️ApplyTransformation
