//! 🧭️ Shooting play app commands — the transform gumball: translate/rotate/scale the selected assets.
//! Every drag tick coalesces into one undo step via `Emit::amend`'s coalesce key.

use crate::op::ShootingMutation;
use crate::ShootingSnapshot;
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎯️ Falls back to the current `"assets"` interaction-domain selection (read once per dispatch into
/// `ShootingDispatchCtx::selected_asset_ids` — see that struct's doc comment) when the command carries
/// no explicit ids.
fn mesh_selection_ids_typed(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() {
        fallback.to_vec()
    } else {
        ids.to_vec()
    }
}

//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub asset_ids: Vec<String>,
        pub dx: f64,
        pub dy: f64,
        pub dz: f64,
    }

    pub fn handle(payload: &TranslateSelection, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let ids = mesh_selection_ids_typed(&payload.asset_ids, &ctx.selected_asset_ids);
        if ids.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::amend(vec![ShootingMutation::DragAssets(crate::mutations::drag_assets::DragAssets { asset_ids: ids, dx: payload.dx, dy: payload.dy, dz: payload.dz })], "gumball-translate"))
        }
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub asset_ids: Vec<String>,
        pub ax: f64,
        pub ay: f64,
        pub az: f64,
        pub angle: f64,
    }

    pub fn handle(payload: &RotateSelection, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let ids = mesh_selection_ids_typed(&payload.asset_ids, &ctx.selected_asset_ids);
        if ids.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::amend(vec![ShootingMutation::RotateAssets(crate::mutations::rotate_assets::RotateAssets { asset_ids: ids, ax: payload.ax, ay: payload.ay, az: payload.az, angle: payload.angle })], "gumball-rotate"))
        }
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub asset_ids: Vec<String>,
        pub sx: f64,
        pub sy: f64,
        pub sz: f64,
    }

    pub fn handle(payload: &ScaleSelection, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let ids = mesh_selection_ids_typed(&payload.asset_ids, &ctx.selected_asset_ids);
        if ids.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::amend(vec![ShootingMutation::ScaleAssets(crate::mutations::scale_assets::ScaleAssets { asset_ids: ids, sx: payload.sx, sy: payload.sy, sz: payload.sz })], "gumball-scale"))
        }
    }
}
//#endregion 🔖️ScaleSelection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
