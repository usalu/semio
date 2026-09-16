//! 🖨️ Shooting play app commands — export the active shot (`exportActiveShot`) or every shot
//! (`exportAllShots`) as icon-render requests. Shell effects, no operations either way. Two rows with
//! one shared body: `app_commands!`'s `command_id()` is a static row→literal mapping, so each manifest
//! `.shell_action(...)` owns its own payload (ticket 26/09/16/SHOOTING-PLUGIN-END-TO-END retired the
//! payload-dependent `command_id` override this pair replaced).

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use crate::op::ShootingMutation;
use crate::standards::v1::subsets::any::schema::shooting_icon_render_request_json;
use crate::{ShootingShot, ShootingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, DslValue, Effect, Emit, Fault, IconRenderExportItem};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Export
fn export(all: bool, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let config = cfg.snapshot;
    if let Some(asset) = crate::standards::v1::subsets::any::schema::active_asset(doc.snapshot) {
        let shots: Vec<&ShootingShot> = if all { snapshot.shots.iter().collect() } else { crate::standards::v1::subsets::any::schema::active_shot(doc.snapshot).into_iter().collect() };
        let items: Vec<IconRenderExportItem> = shots
            .iter()
            .map(|shot| IconRenderExportItem {
                filename: format!("{}.{}", shot.id, if shot.format == "png" { "png" } else { "svg" }),
                request: dsl::os_pack::json::parse(&shooting_icon_render_request_json(doc.snapshot, shot, asset, &config.camera, config.center_model)).map_or(DslValue::Null, |value| dsl::os_pack::json::to_dsl_value(&value)),
            })
            .collect();
        if !items.is_empty() {
            return Ok(Emit::effect(Effect::IconRenderExport { items }));
        }
    }
    Ok(Emit::default())
}
//#endregion 🔖️Export

//#region 🔖️ExportActiveShot
pub mod export_active_shot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "export-active-shot")]
    pub struct ExportActiveShot {}

    pub fn handle(_payload: &ExportActiveShot, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        export(false, doc, cfg)
    }
}
//#endregion 🔖️ExportActiveShot

//#region 🔖️ExportAllShots
pub mod export_all_shots {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "export-all-shots")]
    pub struct ExportAllShots {}

    pub fn handle(_payload: &ExportAllShots, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        export(true, doc, cfg)
    }
}
//#endregion 🔖️ExportAllShots

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
