//! 🧲️ Lowpoly play app commands — the gumball transform gesture (`transformBegin`/`translateSelection`/
//! `rotateSelection`/`scaleSelection`/`transformEnd`). Mid-drag ticks emit zero operations; the whole
//! drag commits as one `Objects(Patch)` on `transformEnd` — see `crate::editor::lowpoly::session::LowpolyScratch`.

use crate::op::LowpolyMutation;
use crate::LowpolySnapshot;
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::{LowpolyScratch, Transform};
use semio_framework_3d::mesh::Vec3;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
