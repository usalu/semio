use semio_framework_test_fixtures::fixture_text;

#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use crate::editor::gis3d::modes::view::windows::terrain::config::*;
use protocol::{ColdMutation, Mutation};

#[test]
fn gis3d_config_json_requires_the_camera_field_and_rejects_unknown_fields() {
    let decode = |text| semio_framework_pack_json::from_json_str::<GisTerrainWindowConfig>(text, semio_framework_pack_json::JsonMemberPolicy::Reject);
    assert!(decode(r#"{}"#).is_err());
    assert!(decode(r#"{"locale":"en-US"}"#).is_err());
    assert!(decode(r#"{"cameraJson":null}"#).is_err());
    assert!(decode(r#"{"cameraJson":"{}","extra":true}"#).is_err());
    assert!(decode(r#"{"cameraJson":"{}"}"#).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_default_uses_automatic_framing() {
    let config = GisTerrainWindowConfig::default();
    assert!(config.camera_json.is_empty());
}

#[test]
fn gis3d_camera_returns_to_the_overview_after_a_geographic_frame_change() {
    use crate::schema::terrain_descriptor::{TerrainDescriptorJson, TerrainOverlays, TerrainProjectOrigin};
    use crate::standards::v1::subsets::any::io::text::snapshot::terrain_camera_frame;
    use serde_json::{Value, json};

    let mut descriptor = TerrainDescriptorJson {
        schema: "gis.terrain".into(),
        project_origin: TerrainProjectOrigin { lon: 9.7, lat: 52.4 },
        overlays: TerrainOverlays::default(),
        exaggeration: 1.5,
    };
    let config = GisTerrainWindowConfig {
        camera_json: json!({ "position": [10.0, 20.0, 50.0], "terrainFrame": terrain_camera_frame(&descriptor) }).to_string(),
        ..Default::default()
    };
    let document = crate::GisTerrainSnapshot { imported_map: Some(Default::default()), ..Default::default() };
    let kept: Value = serde_json::from_str(&config.clone().scene_camera(&descriptor, &document)).unwrap();
    assert_eq!(kept, json!({ "position": [10.0, 20.0, 50.0] }));
    descriptor.project_origin.lon = 8.7;
    let framed: Value = serde_json::from_str(&config.scene_camera(&descriptor, &document)).unwrap();
    assert_eq!(framed["terrainFrame"], json!(terrain_camera_frame(&descriptor)));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_dsl_round_trips_default_and_populated() {
    store::os_store::test_support::assert_dsl_round_trip(&GisTerrainWindowConfig::default());
    let populated = GisTerrainWindowConfig {
        camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into(),
        brush: Some(TerrainBrushSettings { radius: 7, elevation_step_millimetres: 31, smooth_strength: 65 }),
    };
    store::os_store::test_support::assert_dsl_round_trip(&populated);
    store::os_store::test_support::assert_dsl_pack_equivalence(&populated);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = GisTerrainWindowConfig::default();
    let operation = GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
    let next = protocol::apply_diff_cold(operation.diff(&base).diff(), &base).expect("apply");
    assert_eq!(next.camera_json, r#"{"position":[1.0,2.0,3.0]}"#);
    let backwards = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(backwards, vec![GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: base.camera_json.clone() })]);
    assert_eq!(protocol::apply_diff_cold(backwards[0].diff(&next).diff(), &next).expect("restore"), base);
}

#[semio_framework_async_macros::async_test]
async fn terrain_brush_diff_applies_reverses_and_refuses_an_invalid_change() {
    use protocol::{ColdDiffAlgebra, ColdMutationDiff, OpBinary, OpText};

    let base = GisTerrainWindowConfig { camera_json: "{}".into(), ..Default::default() };
    let brush = TerrainBrushSettings { radius: 7, elevation_step_millimetres: 31, smooth_strength: 65 };
    let operation = GisTerrainWindowConfigMutation::SetTerrainBrush(SetTerrainBrush { brush: Some(brush) });
    let leaf = <SetTerrainBrush as protocol::MutationLeaf>::DESCRIPTOR;
    assert_eq!(leaf.semantic_kind, "set-terrain-brush");
    let payload_documents = <SetTerrainBrush as protocol::MutationLeaf>::PAYLOAD_SCHEMA_DOCUMENTS;
    assert!(payload_documents.iter().any(|document| document.contains("/gisterrain/window/config/brush.json")));
    let outcome = operation.diff(&base);
    let next = protocol::apply_diff_cold(outcome.diff(), &base).expect("apply brush diff");
    assert_eq!(next.brush, Some(brush));
    assert_eq!(next.camera_json, base.camera_json);
    assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&base), &next).unwrap(), base);
    let reverse = operation.inverse(&base).unwrap();
    assert_eq!(protocol::apply_diff_cold(reverse[0].diff(&next).diff(), &next).unwrap(), base);
    for mutation in std::iter::once(&operation).chain(&reverse) {
        assert_eq!(GisTerrainWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), *mutation);
        assert_eq!(GisTerrainWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), *mutation);
    }

    let camera = GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: "next".into() });
    let mut combined = outcome.diff().clone();
    combined.absorb(camera.diff(&base).diff().clone());
    let combined = protocol::apply_diff_cold(&combined, &base).expect("apply combined diff");
    assert_eq!(combined.brush, Some(brush));
    assert_eq!(combined.camera_json, "next");

    let invalid_brush = TerrainBrushSettings { radius: 33, ..brush };
    let invalid_operation = GisTerrainWindowConfigMutation::SetTerrainBrush(SetTerrainBrush { brush: Some(invalid_brush) });
    let refused = invalid_operation.diff(&base);
    let protocol::MutationAdmission::Refused(refused) = refused.admit() else { panic!("invalid brush was admitted") };
    assert_eq!(refused.messages()[0].target, ["brush"]);
    assert_eq!(refused.diff(), &GisTerrainWindowConfigDiff::default());

    let invalid = GisTerrainWindowConfigDiff::from(GisTerrainWindowConfigDelta {
        brush: Some(TerrainBrushChange { brush: Some(invalid_brush) }),
        ..Default::default()
    });
    assert_eq!(protocol::apply_diff_cold(&invalid, &base).unwrap_err().target, vec!["brush"]);
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
                use crate::editor::gis3d::unit_tests::context::{Gis3dApp, app, close, dispatch_at, render_at};
                use crate::editor::gis3d::{Gis3dCommand, Gis3dPlayApp};
                use semio_plugin::{ArtifactEditor, PluginApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

                let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🧫️fixtures/🔬️window/🔣️.json")).expect("neutral Terrain window fixture");
                let left_id = fixture["leftWindowId"].as_str().expect("left window id");
                let right_id = fixture["rightWindowId"].as_str().expect("right window id");
                let roster = ViewModel { window_instances: [left_id, right_id].into_iter().map(|id| ViewWindowInstance { id: id.into(), window_kind_id: GisTerrainWindowConfigOwner::WINDOW_KIND_ID.into() }).collect(), ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
                let mut left = roster.for_window_instance(left_id).expect("left context");
                let mut right = roster.for_window_instance(right_id).expect("right context");
                let brush = &fixture["brush"];
                let active_utility = brush["activeUtility"].as_str().expect("brush utility");
                left.active_utility_id = Some(active_utility.into());
                right.active_utility_id = Some(active_utility.into());
                let mut running = app().await;
                let mut reopened = app().await;
                let document_before = running.snapshot().expect("document before");
                let app_config_before = running.config_pack().await.expect("app config before");
                async fn generations(app: &mut Gis3dApp, left: &ViewModel, right: &ViewModel) -> [Option<u64>; 2] {
                    [
                        app.window_config_generation(left).await.expect("left generation"),
                        app.window_config_generation(right).await.expect("right generation"),
                    ]
                }
                let mut before = generations(&mut running, &left, &right).await;
                let advanced = |before: [Option<u64>; 2], after: [Option<u64>; 2], window_id| {
                    let target = [left_id, right_id].iter().position(|id| *id == window_id).expect("known window");
                    assert_eq!(before[1 - target], after[1 - target], "sibling generation changed: {before:?} → {after:?}");
                    assert!(after[target].is_some_and(|generation| before[target].is_none_or(|previous| generation > previous)),
                        "{window_id} did not advance: {before:?} → {after:?}");
                };

                for row in fixture["cases"].as_array().expect("cases") {
                    let window_id = row["windowId"].as_str().expect("case window");
                    let context = roster.for_window_instance(window_id).expect("case context");
                    let receipt = dispatch_at(&mut running, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: row["cameraJson"].as_str().expect("camera").into() }), context).await;
                    assert_eq!(receipt.lanes.iter().filter(|lane| **lane == semio_plugin::app::TypedOperationResultLane::WindowConfig).count(), 1);
                    let after = generations(&mut running, &left, &right).await;
                    advanced(before, after, window_id);
                    before = after;
                }

                let settings = semio_framework_pack_json::parse(&brush["settings"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("brush settings");
                let mut args = Some(semio_framework_pack_json::to_dsl_value(&settings));
                let command = Gis3dPlayApp::command_from_action("configureTerrainBrush", &mut args, &mut None).expect("configure action");
                let brush_window = brush["windowId"].as_str().expect("brush window");
                let target = roster.for_window_instance(brush_window).expect("brush context");
                let receipt = dispatch_at(&mut running, command, target).await;
                assert_eq!(receipt.lanes.iter().filter(|lane| **lane == semio_plugin::app::TypedOperationResultLane::WindowConfig).count(), 1);
                advanced(before, generations(&mut running, &left, &right).await, brush_window);

                assert_eq!(running.snapshot().expect("document after"), document_before);
                let app_config_after = running.config_pack().await.expect("app config after");
                assert_eq!(app_config_after.pack, app_config_before.pack);
                assert_eq!(app_config_after.spr, app_config_before.spr);
                assert_eq!(running.window_transient_generation(&left).expect("left transient inventory"), None);
                assert_eq!(running.window_transient_generation(&right).expect("right transient inventory"), None);

                let left_json = render_at(&mut running, crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE, &left).await;
                let right_json = render_at(&mut running, crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE, &right).await;
                let decode_scene = |json: &str| semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_plugin::World3dScene>(json).expect("terrain scene");
                let brush_action = |scene: &semio_plugin::World3dScene| {
                    serde_json::from_str::<serde_json::Value>(scene.interaction_json.as_deref().expect("brush interaction")).expect("brush action")["terrainBrush"].clone()
                };
                let left_scene = decode_scene(&left_json);
                let right_scene = decode_scene(&right_json);
                assert_eq!(left_scene.camera_json, fixture["cases"][0]["cameraJson"].as_str().expect("left expected"));
                assert_eq!(right_scene.camera_json, fixture["cases"][1]["cameraJson"].as_str().expect("right expected"));
                assert_eq!(brush_action(&left_scene), brush["expectedTarget"]);
                assert_eq!(brush_action(&right_scene), brush["expectedOther"]);

                let packs = running.window_config_packs().await.expect("window packs");
                assert_eq!(packs.len(), 2);
                for pack in packs {
                    semio_plugin::restore_fixture_window_config(&mut *reopened.0, pack, reopened.1.cx()).await.expect("reload window pack");
                }
                let reopened_left = decode_scene(&render_at(&mut reopened, crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE, &left).await);
                let reopened_right = decode_scene(&render_at(&mut reopened, crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE, &right).await);
                assert_eq!(reopened_left.camera_json, left_scene.camera_json);
                assert_eq!(reopened_right.camera_json, right_scene.camera_json);
                assert_eq!(brush_action(&reopened_left), brush["expectedTarget"]);
                assert_eq!(brush_action(&reopened_right), brush["expectedOther"]);

                close(&mut reopened);
                close(&mut running);
            })
        })
        .expect("spawn Terrain window config law")
        .join()
        .expect("Terrain window config law thread");
}
