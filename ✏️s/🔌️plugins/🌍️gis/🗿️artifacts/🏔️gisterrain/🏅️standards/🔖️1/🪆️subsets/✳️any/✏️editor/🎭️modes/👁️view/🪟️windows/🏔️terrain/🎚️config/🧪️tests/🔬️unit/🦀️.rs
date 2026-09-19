use super::*;
use protocol::{Mutation, MutationDiff};

#[test]
fn gis3d_config_serde_is_strict_and_requires_the_camera_field() {
    assert!(serde_json::from_str::<GisTerrainWindowConfig>(r#"{}"#).is_err());
    assert!(serde_json::from_str::<GisTerrainWindowConfig>(r#"{"locale":"en-US"}"#).is_err());
    assert!(serde_json::from_str::<GisTerrainWindowConfig>(r#"{"cameraJson":null}"#).is_err());
    assert!(serde_json::from_str::<GisTerrainWindowConfig>(r#"{"cameraJson":"{}","extra":true}"#).is_err());
    assert!(serde_json::from_str::<GisTerrainWindowConfig>(r#"{"cameraJson":"{}"}"#).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_default_matches_the_pre_migration_view_defaults() {
    let config = GisTerrainWindowConfig::default();
    assert!(config.camera_json.contains("800"));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_dsl_round_trips_default_and_populated() {
    store::os_store::test_support::assert_dsl_round_trip(&GisTerrainWindowConfig::default());
    let populated = GisTerrainWindowConfig { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() };
    store::os_store::test_support::assert_dsl_round_trip(&populated);
    store::os_store::test_support::assert_dsl_pack_equivalence(&populated);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = GisTerrainWindowConfig::default();
    let operation = GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
    let next = operation.diff(&base).diff().apply(&base).expect("apply");
    assert_eq!(next.camera_json, r#"{"position":[1.0,2.0,3.0]}"#);
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: base.camera_json.clone() })]);
    assert_eq!(backwards[0].diff(&next).diff().apply(&next).expect("restore"), base);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_operation_lines_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() }));
}

fn block_on_terrain_window_config<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[test]
fn gis_terrain_window_config_isolates_two_registered_windows_and_reloads() {
    std::thread::Builder::new()
        .name("gis-terrain-window-config-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            block_on_terrain_window_config(async {
                use crate::editor::gis3d::commands::view::set_camera;
                use crate::editor::gis3d::unit_tests::context::{app, close, dispatch_at, render_at};
                use crate::editor::gis3d::Gis3dCommand;
                use semio_framework_plugin::{PluginApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

                let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-config-ownership/🔣️.json")).expect("neutral Terrain window fixture");
                let left_id = fixture["leftWindowId"].as_str().expect("left window id");
                let right_id = fixture["rightWindowId"].as_str().expect("right window id");
                let roster = ViewModel { window_instances: [left_id, right_id].into_iter().map(|id| ViewWindowInstance { id: id.into(), window_kind_id: GisTerrainWindowConfigOwner::WINDOW_KIND_ID.into() }).collect(), ..Default::default() };
                let left = roster.for_window_instance(left_id).expect("left context");
                let right = roster.for_window_instance(right_id).expect("right context");
                let mut running = Box::new(app().await);
                let mut reopened = Box::new(app().await);
                let document_before = running.snapshot().expect("document before");
                let app_config_before = running.config_pack().await.expect("app config before");

                for row in fixture["cases"].as_array().expect("cases") {
                    let window_id = row["windowId"].as_str().expect("case window");
                    let context = roster.for_window_instance(window_id).expect("case context");
                    let receipt = dispatch_at(&mut running, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: row["cameraJson"].as_str().expect("camera").into() }), context).await;
                    assert_eq!(receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count(), 1);
                }

                assert_eq!(running.snapshot().expect("document after"), document_before);
                let app_config_after = running.config_pack().await.expect("app config after");
                assert_eq!(app_config_after.pack, app_config_before.pack);
                assert_eq!(app_config_after.spr, app_config_before.spr);
                assert_eq!(running.window_config_generation(&left).await.expect("left generation"), Some(1));
                assert_eq!(running.window_config_generation(&right).await.expect("right generation"), Some(1));
                assert_eq!(running.window_transient_generation(&left).expect("left transient inventory"), None);
                assert_eq!(running.window_transient_generation(&right).expect("right transient inventory"), None);

                let left_json = render_at(&mut running, super::super::GIS3D_PLAY_BODY_COMPOSITE, &left).await;
                let right_json = render_at(&mut running, super::super::GIS3D_PLAY_BODY_COMPOSITE, &right).await;
                let left_scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::World3dScene>(&left_json).expect("left scene");
                let right_scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::World3dScene>(&right_json).expect("right scene");
                assert_eq!(left_scene.camera_json, fixture["cases"][0]["cameraJson"].as_str().expect("left expected"));
                assert_eq!(right_scene.camera_json, fixture["cases"][1]["cameraJson"].as_str().expect("right expected"));

                let packs = running.window_config_packs().await.expect("window packs");
                assert_eq!(packs.len(), 2);
                for pack in packs {
                    reopened.load_window_config_pack(pack).await.expect("reload window pack");
                }
                let reopened_left = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::World3dScene>(&render_at(&mut reopened, super::super::GIS3D_PLAY_BODY_COMPOSITE, &left).await).expect("reopened left");
                let reopened_right = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::World3dScene>(&render_at(&mut reopened, super::super::GIS3D_PLAY_BODY_COMPOSITE, &right).await).expect("reopened right");
                assert_eq!(reopened_left.camera_json, left_scene.camera_json);
                assert_eq!(reopened_right.camera_json, right_scene.camera_json);

                close(&mut reopened);
                close(&mut running);
                eprintln!("[DEBUG] two registered Terrain windows published and reloaded independent camera config; transient inventory remained empty");
            })
        })
        .expect("spawn Terrain window config law")
        .join()
        .expect("Terrain window config law thread");
}
