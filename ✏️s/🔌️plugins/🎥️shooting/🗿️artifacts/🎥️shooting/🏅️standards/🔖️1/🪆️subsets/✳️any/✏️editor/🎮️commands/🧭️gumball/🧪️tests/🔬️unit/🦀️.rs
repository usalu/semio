
use super::*;
use crate::editor::shooting::ShootingCommand;
use crate::editor::shooting::testkit::{dispatch, shooting_app};
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn gumball_transform_drag_coalesces_into_one_edit() {
    let mut app = shooting_app().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    for dx in [1.0, 2.0, 3.0] {
        dispatch(&mut app, ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec![asset_id.clone()], dx, dy: 0.0, dz: 0.0 })).await;
    }
    app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("undo");
    let restored = app.snapshot().expect("snapshot");
    let original = crate::standards::v1::subsets::any::schema::default_snapshot().assets.iter().find(|asset| asset.id == asset_id).map(|asset| asset.origin).expect("original origin");
    assert_eq!(restored.assets.iter().find(|asset| asset.id == asset_id).unwrap().origin, original, "undoing the coalesced drag restores the pre-drag origin");
}

#[semio_framework_async_macros::async_test]
async fn empty_selection_is_a_no_operation() {
    let mut app = shooting_app().await;
    // No explicit ids and an empty config selection: nothing to transform.
    let result = dispatch(&mut app, ShootingCommand::RotateSelection(rotate_selection::RotateSelection { asset_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 })).await;
    assert!(result.mutations.is_empty());
}
