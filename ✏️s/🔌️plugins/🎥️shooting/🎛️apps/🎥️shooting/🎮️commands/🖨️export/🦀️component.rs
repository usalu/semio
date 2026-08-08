//! 🖨️ Shooting play app command — export the active shot or every shot as icon-render requests. A
//! shell effect, no operations either way.
//!
//! This is the ONE command whose manifest action id is payload-dependent (`exportActiveShot` vs
//! `exportAllShots`, mirroring the two real `.shell_action(...)` declarations in the manifest) — see
//! `crate::apps::shooting::ShootingPlayApp::command_id`'s override, since `app_commands!`'s generated
//! `command_id()` is a static 1:1 row→literal mapping with no payload-conditional escape hatch.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::artifacts::shooting::engine::shooting_icon_render_request_json;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::{ShootingSnapshot, ShootingShot};
use semio_framework_plugin::{ConfigView, DocumentView, DslValue, Emit, Fault, HostEffect, IconRenderExportItem};
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

    pub fn handle(payload: &ExportShots, doc: &DocumentView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        if let Some(asset) = crate::artifacts::shooting::engine::active_asset(doc.snapshot) {
            let shots: Vec<&ShootingShot> = if payload.all { snapshot.shots.iter().collect() } else { crate::artifacts::shooting::engine::active_shot(doc.snapshot).into_iter().collect() };
            let items: Vec<IconRenderExportItem> = shots
                .iter()
                .map(|shot| IconRenderExportItem {
                    filename: format!("{}.{}", shot.id, if shot.format == "png" { "png" } else { "svg" }),
                    request: serde_json::from_str::<Value>(&shooting_icon_render_request_json(doc.snapshot, shot, asset, &config.camera)).ok().and_then(|value| semio_framework_plugin::to_dsl_value(&value).ok()).unwrap_or(DslValue::Null),
                })
                .collect();
            if !items.is_empty() {
                return Ok(Emit::effect(HostEffect::IconRenderExport { items }));
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
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn export_active_shot_produces_one_icon_render_item() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::ExportShots(export_shots::ExportShots { all: false }));
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            HostEffect::IconRenderExport { items } => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].filename, "overview-svg.svg");
            }
            other => panic!("expected IconRenderExport, got {other:?}"),
        }
    }

    #[test]
    fn export_all_shots_produces_one_item_per_shot() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::ExportShots(export_shots::ExportShots { all: true }));
        match &result.requested_effects[0] {
            HostEffect::IconRenderExport { items } => assert_eq!(items.len(), 2),
            other => panic!("expected IconRenderExport, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
