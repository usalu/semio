//! 🧲️ Lowpoly play app commands — the gumball transform gesture (`transformBegin`/`translateSelection`/
//! `rotateSelection`/`scaleSelection`/`transformEnd`). Mid-drag ticks emit zero operations; the whole
//! drag commits as one `Objects(Patch)` on `transformEnd` — see `crate::editor::lowpoly::session::LowpolyScratch`.

use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::{LowpolyScratch, Transform};
use semio_framework_3d::mesh::Vec3;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️TransformBegin
pub mod transform_begin {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "transform-begin")]
    pub struct TransformBegin {}

    pub fn handle(_payload: &TransformBegin, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        ctx.begin_transform_drag();
        Ok(Emit::default())
    }
}
//#endregion 🔖️TransformBegin

//#region 🔖️TransformEnd
pub mod transform_end {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "transform-end")]
    pub struct TransformEnd {}

    pub fn handle(_payload: &TransformEnd, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(ctx.end_transform_drag())
    }
}
//#endregion 🔖️TransformEnd

//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub mode: Option<String>,
        pub ids: Option<Vec<u32>>,
        pub dx: f32,
        pub dy: f32,
        pub dz: f32,
    }

    pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let mode = payload.mode.clone().unwrap_or_else(|| "mesh".into());
        let ids = payload.ids.clone().unwrap_or_default();
        Ok(ctx.transform_selection(doc.snapshot, cfg.snapshot, &mode, ids, Transform::Translate(Vec3::new(payload.dx, payload.dy, payload.dz)), "Translate selection"))
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub mode: Option<String>,
        pub ids: Option<Vec<u32>>,
        pub ax: f32,
        pub ay: f32,
        pub az: f32,
        pub angle: f32,
    }

    pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let mode = payload.mode.clone().unwrap_or_else(|| "mesh".into());
        let ids = payload.ids.clone().unwrap_or_default();
        Ok(ctx.transform_selection(doc.snapshot, cfg.snapshot, &mode, ids, Transform::Rotate { axis: Vec3::new(payload.ax, payload.ay, payload.az), angle: payload.angle }, "Rotate selection"))
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub mode: Option<String>,
        pub ids: Option<Vec<u32>>,
        pub sx: f32,
        pub sy: f32,
        pub sz: f32,
    }

    pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let mode = payload.mode.clone().unwrap_or_else(|| "mesh".into());
        let ids = payload.ids.clone().unwrap_or_default();
        Ok(ctx.transform_selection(doc.snapshot, cfg.snapshot, &mode, ids, Transform::Scale(Vec3::new(payload.sx, payload.sy, payload.sz)), "Scale selection"))
    }
}
//#endregion 🔖️ScaleSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::app;
    use crate::editor::lowpoly::LowpolyCommand;
    use semio_framework_plugin::{testkit, PluginApp};

    #[semio_framework_async_macros::async_test]
    async fn gumball_drag_coalesces_to_one_committed_edit() {
        // 🧲️ THE COALESCING REGRESSION: a multi-tick gumball translate must emit ZERO operations mid-drag and
        // exactly ONE commit operation (base → final mesh) on drag end — never a full-mesh patch per tick.
        let mut a = app().await;
        let before_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
        a.dispatch_typed(LowpolyCommand::TransformBegin(super::transform_begin::TransformBegin {}), &testkit::meta("a")).await.unwrap();
        let tick_a = a.dispatch_typed(LowpolyCommand::TranslateSelection(super::translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 0.5, dy: 0.0, dz: 0.0 }), &testkit::meta("a")).await.unwrap();
        let tick_b = a.dispatch_typed(LowpolyCommand::TranslateSelection(super::translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 0.25, dy: 0.0, dz: 0.0 }), &testkit::meta("a")).await.unwrap();
        assert!(tick_a.mutations.is_empty() && tick_b.mutations.is_empty(), "mid-drag transform ticks emit no operations");
        assert_eq!(a.snapshot().expect("projection").objects[0].mesh, before_mesh, "no operation reached the document mid-drag");
        let end = a.dispatch_typed(LowpolyCommand::TransformEnd(super::transform_end::TransformEnd {}), &testkit::meta("a")).await.unwrap();
        assert_eq!(end.mutations.len(), 1, "the whole drag commits as exactly one operation");
        let after_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
        assert_ne!(after_mesh, before_mesh, "the drag moved the mesh");
        a.handle_action("undo", None, &testkit::meta("a")).await.unwrap();
        assert_eq!(a.snapshot().expect("projection").objects[0].mesh, before_mesh, "one undo reverts the whole coalesced gumball drag");
    }
}
//#endregion 🧪️Tests
