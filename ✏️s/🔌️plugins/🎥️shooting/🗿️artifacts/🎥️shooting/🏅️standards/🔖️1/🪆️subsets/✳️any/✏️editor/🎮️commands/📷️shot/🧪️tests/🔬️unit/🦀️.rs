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
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
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
