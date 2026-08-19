//! 🖨️ Shooting play app command — export the active shot or every shot as icon-render requests. A
//! shell effect, no operations either way.
//!
//! This is the ONE command whose manifest action id is payload-dependent (`exportActiveShot` vs
//! `exportAllShots`, mirroring the two real `.shell_action(...)` declarations in the manifest) — see
//! `crate::editor::shooting::ShootingPlayApp::command_id`'s override, since `app_commands!`'s generated
//! `command_id()` is a static 1:1 row→literal mapping with no payload-conditional escape hatch.

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use crate::artifacts::shooting::schema::shooting_icon_render_request_json;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::{ShootingSnapshot, ShootingShot};
use semio_framework_plugin::{ConfigView, ArtifactView, DslValue, Emit, Fault, Effect, IconRenderExportItem};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️ExportShots
pub mod export_shots {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-shots")]
    pub struct ExportShots {
        pub all: bool,
    }

    pub async fn handle(payload: &ExportShots, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        if let Some(asset) = crate::artifacts::shooting::schema::active_asset(doc.snapshot) {
            let shots: Vec<&ShootingShot> = if payload.all { snapshot.shots.iter().collect() } else { crate::artifacts::shooting::schema::active_shot(doc.snapshot).into_iter().collect() };
            let items: Vec<IconRenderExportItem> = shots
                .iter()
                .map(|shot| IconRenderExportItem {
                    filename: format!("{}.{}", shot.id, if shot.format == "png" { "png" } else { "svg" }),
                    request: serde_json::from_str::<Value>(&shooting_icon_render_request_json(doc.snapshot, shot, asset, &config.camera)).ok().and_then(|value| semio_framework_plugin::to_dsl_value(&value).ok()).unwrap_or(DslValue::Null),
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
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app};
    use crate::editor::shooting::ShootingCommand;

    #[test]
    async fn export_active_shot_produces_one_icon_render_item() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::ExportShots(export_shots::ExportShots { all: false }));
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::IconRenderExport { items } => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].filename, "overview-svg.svg");
            }
            other => panic!("expected IconRenderExport, got {other:?}"),
        }
    }

    #[test]
    async fn export_all_shots_produces_one_item_per_shot() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::ExportShots(export_shots::ExportShots { all: true }));
        match &result.requested_effects[0] {
            Effect::IconRenderExport { items } => assert_eq!(items.len(), 2),
            other => panic!("expected IconRenderExport, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
