use super::*;
use crate::editor::shooting::testkit::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

#[semio_framework_async_macros::async_test]
async fn export_active_shot_produces_one_icon_render_item() {
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::ExportShots(export_shots::ExportShots { all: false })).await;
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
    let result = dispatch(&mut app, ShootingCommand::ExportShots(export_shots::ExportShots { all: true })).await;
    match &result.requested_effects[0] {
        Effect::IconRenderExport { items } => assert_eq!(items.len(), 2),
        other => panic!("expected IconRenderExport, got {other:?}"),
    }
}
