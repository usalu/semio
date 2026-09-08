//! 🖨️ Shooting play app command — export the active shot or every shot as icon-render requests. A
//! shell effect, no operations either way.
//!
//! This is the ONE command whose manifest action id is payload-dependent (`exportActiveShot` vs
//! `exportAllShots`, mirroring the two real `.shell_action(...)` declarations in the manifest) — see
//! `crate::editor::shooting::ShootingPlayApp::command_id`'s override, since `app_commands!`'s generated
//! `command_id()` is a static 1:1 row→literal mapping with no payload-conditional escape hatch.

use crate::op::ShootingMutation;
use crate::schema::shooting_icon_render_request_json;
use crate::{ShootingShot, ShootingSnapshot};
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, DslValue, Effect, Emit, Fault, IconRenderExportItem};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ExportShots
pub mod export_shots {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "export-shots")]
    pub struct ExportShots {
        pub all: bool,
    }

    pub fn handle(payload: &ExportShots, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        if let Some(asset) = crate::schema::active_asset(doc.snapshot) {
            let shots: Vec<&ShootingShot> = if payload.all { snapshot.shots.iter().collect() } else { crate::schema::active_shot(doc.snapshot).into_iter().collect() };
            let items: Vec<IconRenderExportItem> = shots
                .iter()
                .map(|shot| IconRenderExportItem {
                    filename: format!("{}.{}", shot.id, if shot.format == "png" { "png" } else { "svg" }),
                    request: dsl::os_pack::json::parse(&shooting_icon_render_request_json(doc.snapshot, shot, asset, &config.camera)).map_or(DslValue::Null, |value| dsl::os_pack::json::to_dsl_value(&value)),
                })
                .collect();
            if !items.is_empty() {
                return Ok(Emit::effect(Effect::IconRenderExport { items }));
            }
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️ExportShots

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
