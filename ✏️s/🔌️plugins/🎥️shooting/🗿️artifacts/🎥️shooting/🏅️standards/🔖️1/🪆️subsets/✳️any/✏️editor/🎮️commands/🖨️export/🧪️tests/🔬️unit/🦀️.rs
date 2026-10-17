use semio_framework_plugin::Effect;
#[allow(unused_imports)]
use crate::{editor::shooting::config::ShootingConfig, editor::shooting::config::ShootingConfigMutation};
#[allow(unused_imports)]
use crate::editor::shooting::ui_helpers_and_io::ShootingDispatchCtx;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::text::snapshot::shooting_icon_render_request_json;
#[allow(unused_imports)]
use crate::{ShootingShot, ShootingSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, DslValue, Emit, Fault, FaultCode, FaultOrigin, IconRenderExportItem};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::shooting::commands::export::*;
use crate::editor::shooting::unit_tests::context::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

#[semio_framework_async_macros::async_test]
async fn export_active_shot_produces_one_icon_render_item() {
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::ExportActiveShot(export_active_shot::ExportActiveShot {})).await;
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::IconRenderExport { items } => {
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].filename, "overview-svg.svg");
        }
        other => panic!("expected IconRenderExport, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn export_all_shots_produces_one_item_per_shot() {
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::ExportAllShots(export_all_shots::ExportAllShots {})).await;
    match &result.requested_effects[0] {
        Effect::IconRenderExport { items } => assert_eq!(items.len(), 2),
        other => panic!("expected IconRenderExport, got {other:?}"),
    }
}
