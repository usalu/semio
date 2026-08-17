//! 🧭️ Shooting play app commands — the transform gumball: translate/rotate/scale the selected assets.
//! Every drag tick coalesces into one undo step via `Emit::amend`'s coalesce key.

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
            Ok(Emit::amend(vec![ShootingMutation::DragAssets(crate::artifacts::shooting::mutations::drag_assets::mutation::DragAssets { asset_ids: ids, dx: payload.dx, dy: payload.dy, dz: payload.dz })], "gumball-translate"))
        }
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
            Ok(Emit::amend(vec![ShootingMutation::RotateAssets(crate::artifacts::shooting::mutations::rotate_assets::mutation::RotateAssets { asset_ids: ids, ax: payload.ax, ay: payload.ay, az: payload.az, angle: payload.angle })], "gumball-rotate"))
        }
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
            Ok(Emit::amend(vec![ShootingMutation::ScaleAssets(crate::artifacts::shooting::mutations::scale_assets::mutation::ScaleAssets { asset_ids: ids, sx: payload.sx, sy: payload.sy, sz: payload.sz })], "gumball-scale"))
        }
    }
}
//#endregion 🔖️ScaleSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app};
    use crate::editor::shooting::ShootingCommand;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn gumball_transform_drag_coalesces_into_one_edit() {
        let mut app = shooting_app();
        let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
        for dx in [1.0, 2.0, 3.0] {
            dispatch(&mut app, ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec![asset_id.clone()], dx, dy: 0.0, dz: 0.0 }));
        }
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        let restored = app.snapshot().expect("snapshot");
        let original = crate::artifacts::shooting::schema::default_snapshot().assets.iter().find(|asset| asset.id == asset_id).map(|asset| asset.origin).expect("original origin");
        assert_eq!(restored.assets.iter().find(|asset| asset.id == asset_id).unwrap().origin, original, "undoing the coalesced drag restores the pre-drag origin");
    }

    #[test]
    fn empty_selection_is_a_no_operation() {
        let mut app = shooting_app();
        // No explicit ids and an empty config selection: nothing to transform.
        let result = dispatch(&mut app, ShootingCommand::RotateSelection(rotate_selection::RotateSelection { asset_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
