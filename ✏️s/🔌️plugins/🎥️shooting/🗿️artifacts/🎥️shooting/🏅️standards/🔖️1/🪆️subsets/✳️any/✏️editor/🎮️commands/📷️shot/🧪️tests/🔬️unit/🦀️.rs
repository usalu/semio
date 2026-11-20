#[allow(unused_imports)]
use crate::{editor::shooting::config::ShootingConfig, editor::shooting::config::ShootingConfigMutation};
#[allow(unused_imports)]
use crate::editor::shooting::ui_helpers_and_io::ShootingDispatchCtx;
#[allow(unused_imports)]
use crate::mutations::change_shot_format::ChangeShotFormat;
#[allow(unused_imports)]
use crate::mutations::change_shot_height::ChangeShotHeight;
#[allow(unused_imports)]
use crate::mutations::change_shot_shape::ChangeShotShape;
#[allow(unused_imports)]
use crate::mutations::change_shot_width::ChangeShotWidth;
#[allow(unused_imports)]
use crate::mutations::create_shot::CreateShot;
#[allow(unused_imports)]
use crate::mutations::rename_shot::RenameShot;
#[allow(unused_imports)]
use crate::mutations::set_active_shot::SetActiveShot as SetActiveShotMutation;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
#[allow(unused_imports)]
use crate::ShootingShot;
#[allow(unused_imports)]
use semio_framework_pack_json::json;
#[allow(unused_imports)]
use semio_framework_pack_json::Value;
#[allow(unused_imports)]
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::shooting::commands::shot::*;
use crate::editor::shooting::unit_tests::context::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

#[semio_framework_async_macros::async_test]
async fn set_active_shot_label_patches_active_shot() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero Shot".into() })).await;
    assert_eq!(crate::active_shot(&app.snapshot().expect("snapshot")).unwrap().label, "Hero Shot");
}

#[semio_framework_async_macros::async_test]
async fn add_shot_action_appends_shot() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() })).await;
    assert!(app.snapshot().expect("snapshot").shots.iter().any(|shot| shot.format == "svg" && shot.shape == "ellipse"));
}

#[semio_framework_async_macros::async_test]
async fn set_active_shot_updates_fixture() {
    let mut app = shooting_app().await;
    let second_id = app.snapshot().expect("snapshot").shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
    dispatch(&mut app, ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some(second_id.clone()) })).await;
    assert_eq!(app.snapshot().expect("snapshot").active_shot_id, second_id);
}

#[semio_framework_async_macros::async_test]
async fn inspector_edits_reach_the_exported_drawing_and_camera() {
    use crate::editor::shooting::commands::export::export_active_shot::ExportActiveShot;
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎮️commands/📷️shot/🧪️tests/🧫️fixtures/🔢️field-values.json")).expect("drawing settings");
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "rectangle".into() })).await;
    let shot_id = app.snapshot().expect("snapshot").active_shot_id.clone();
    for key in ["drawing", "camera", "poseFields"] {
        for (field, v) in fixture[key].as_object().expect("field map") {
            let value = v.as_str().map_or_else(|| v.to_string(), str::to_owned);
            dispatch(&mut app, ShootingCommand::PatchShots(patch_shots::PatchShots { shot_ids: vec![shot_id.clone()], field: field.clone(), value })).await;
        }
    }
    let result = dispatch(&mut app, ShootingCommand::ExportActiveShot(ExportActiveShot {})).await;
    let semio_plugin::Effect::IconRenderExport { items } = &result.requested_effects[0] else { panic!("SVG export effect") };
    let request: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&items[0].request)).expect("independent export request oracle");
    assert_eq!(request["fit"]["enabled"], false);
    let all = dispatch(&mut app, ShootingCommand::ExportAllShots(crate::editor::shooting::commands::export::export_all_shots::ExportAllShots {})).await;
    let semio_plugin::Effect::IconRenderExport { items } = &all.requested_effects[0] else { panic!("all-shot SVG and PNG export effect") };
    assert!(items.len() > 1);
    for item in items {
        let request: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&item.request)).expect("sibling export request");
        assert_eq!(request["fit"]["enabled"], item.filename != format!("{shot_id}.svg"), "fit for {}", item.filename);
    }
    for (field, key) in [("position", "expectedPosition"), ("target", "expectedTarget"), ("up", "expectedUp")] {
        for (actual, expected) in request["camera"][field].as_array().expect("coordinates").iter().zip(fixture[key].as_array().expect("expected coordinates")) {
            assert_eq!(actual.as_f64(), expected.as_f64());
        }
    }
    for key in ["drawing", "camera"] {
        for (field, expected) in fixture[key].as_object().expect("fields") {
            let actual = &request[key][field];
            if let Some(number) = expected.as_f64() {
                assert_eq!(actual.as_f64(), Some(number), "{field}");
            } else {
                assert_eq!(actual, expected, "{field}");
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn editing_an_uncaptured_shot_keeps_the_camera_with_that_shot() {
    let mut app = shooting_app().await;
    let base = app.snapshot().expect("snapshot");
    let target_id = base.active_shot_id.clone();
    assert!(crate::active_shot(&base).unwrap().camera_id.is_none());
    dispatch(&mut app, ShootingCommand::PatchShots(patch_shots::PatchShots {
        shot_ids: vec![target_id.clone()],
        field: "projection".into(),
        value: "orthographic".into(),
    })).await;
    let edited = app.snapshot().expect("edited snapshot");
    let selected = edited.shots.iter().find(|shot| shot.id == target_id).unwrap();
    let camera = edited.saved_cameras.iter().find(|camera| Some(&camera.id) == selected.camera_id.as_ref()).unwrap();
    assert_eq!(camera.camera.projection.as_deref(), Some("orthographic"));
    assert_eq!(selected.auto_fit, None);
    dispatch(&mut app, ShootingCommand::PatchShots(patch_shots::PatchShots {
        shot_ids: vec![target_id.clone()],
        field: "fov".into(),
        value: "42".into(),
    })).await;
    let current = app.snapshot().expect("orthographic shot");
    assert_eq!(crate::active_shot(&current).unwrap().auto_fit, None);
    for (actual, original) in edited.shots.iter().zip(&base.shots) {
        if original.id != target_id {
            assert_eq!(actual, original);
        }
    }
}
