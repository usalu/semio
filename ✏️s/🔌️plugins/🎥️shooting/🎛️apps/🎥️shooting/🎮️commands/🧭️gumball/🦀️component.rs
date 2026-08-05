//! 🧭️ Shooting play app commands — the transform gumball: translate/rotate/scale the selected assets.
//! Every drag tick coalesces into one undo step via `Emit::amend`'s coalesce key.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigOperation};
use crate::artifacts::shooting::op::ShootingOperation;
use crate::artifacts::shooting::ShootingFixture;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🎯️ Falls back to the current config selection when the command carries no explicit ids.
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

    pub fn handle(payload: &TranslateSelection, _doc: &DocumentView<'_, ShootingFixture>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault> {
        let ids = mesh_selection_ids_typed(&payload.asset_ids, &cfg.projection.selected_asset_ids);
        if ids.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::amend(vec![ShootingOperation::TranslateAssets { asset_ids: ids, dx: payload.dx, dy: payload.dy, dz: payload.dz }], "gumball-translate"))
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

    pub fn handle(payload: &RotateSelection, _doc: &DocumentView<'_, ShootingFixture>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault> {
        let ids = mesh_selection_ids_typed(&payload.asset_ids, &cfg.projection.selected_asset_ids);
        if ids.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::amend(vec![ShootingOperation::RotateAssets { asset_ids: ids, ax: payload.ax, ay: payload.ay, az: payload.az, angle: payload.angle }], "gumball-rotate"))
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

    pub fn handle(payload: &ScaleSelection, _doc: &DocumentView<'_, ShootingFixture>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault> {
        let ids = mesh_selection_ids_typed(&payload.asset_ids, &cfg.projection.selected_asset_ids);
        if ids.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::amend(vec![ShootingOperation::ScaleAssets { asset_ids: ids, sx: payload.sx, sy: payload.sy, sz: payload.sz }], "gumball-scale"))
        }
    }
}
//#endregion 🔖️ScaleSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn gumball_transform_drag_coalesces_into_one_edit() {
        let mut app = shooting_app();
        let asset_id = app.projection().expect("projection").assets[0].id.clone();
        for dx in [1.0, 2.0, 3.0] {
            dispatch(&mut app, ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec![asset_id.clone()], dx, dy: 0.0, dz: 0.0 }));
        }
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        let restored = app.projection().expect("projection");
        let original = crate::artifacts::shooting::engine::default_fixture().assets.iter().find(|asset| asset.id == asset_id).map(|asset| asset.origin).expect("original origin");
        assert_eq!(restored.assets.iter().find(|asset| asset.id == asset_id).unwrap().origin, original, "undoing the coalesced drag restores the pre-drag origin");
    }

    #[test]
    fn empty_selection_is_a_no_operation() {
        let mut app = shooting_app();
        // No explicit ids and an empty config selection: nothing to transform.
        let result = dispatch(&mut app, ShootingCommand::RotateSelection(rotate_selection::RotateSelection { asset_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 }));
        assert!(result.operations.is_empty());
    }
}
//#endregion 🧪️Tests
