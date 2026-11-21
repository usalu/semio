use super::*;
use crate::editor::gis3d::{Gis3dPlayApp, unit_tests::context};
use semio_plugin::{ArtifactEditor, ArtifactView, ConfigView, HistoryView, Media, MediaClass, MediaForm, MediaPayload, MediaType, MountedOwnerPolicyV1, artifact_app_laws};
use serde_json::Value;

const HISTORY: &str = include_str!("../../../🧪️tests/🧫️fixtures/🗺️overlay-history/🔣️.json");
const ROLES: [&str; 3] = ["positions", "routes", "regions"];

fn intrinsic(json: &Value) -> DslValue {
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(
        &json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).unwrap())
}

fn map_json(document: &GisTerrainSnapshot) -> Value {
    serde_json::from_str(&document.imported_map.as_ref().unwrap().to_json().unwrap()).unwrap()
}

fn feature_ids(map: &Value) -> Value {
    Value::Array(ROLES.iter().map(|role| {
        Value::Array(map[*role].as_array().unwrap().iter().map(|item| item["id"].clone()).collect())
    }).collect())
}

#[semio_framework_async_macros::async_test]
async fn registered_overlay_edits_preserve_grid_and_replay_history() {
    let law: Value = serde_json::from_str(HISTORY).unwrap();
    let app_manifest = context::gis3d_app_manifest_for_tests();
    for id in ["addOverlayPosition", "moveOverlayPosition"] {
        let action = app_manifest.definition.actions.iter().find(|action| action.id == id).unwrap();
        let latitude = action.args.iter().find(|arg| arg.id == "latitude").unwrap();
        assert_eq!(latitude.schema, semio_framework::ArgSchema::number(Some(-90.0), Some(90.0), None, false));
    }
    let mut app = context::app().await;
    let actor = artifact_app_laws::meta("local");
    let policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let medium = Media {
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        payload: MediaPayload::Intrinsic { schema: "2d.map".into(), value: intrinsic(&law["map"]) },
    };
    context::import_media(&mut app, "map:in", medium, &actor).await;
    let initial = app.snapshot().unwrap();
    assert!(initial.elevation_source.is_some());
    assert_eq!(map_json(&initial), law["map"]);
    let mut revisions = vec![map_json(&initial)];
    for row in law["edits"].as_array().unwrap() {
        let mut meta = artifact_app_laws::meta("local");
        meta.view_state = Some(artifact_app_laws::receive_view_fixture(context::main_window_view(), artifact_app_laws::OriginalWorkFixture::stock()));
        artifact_app_laws::dispatch_fixture_action(
            &mut *app.0, row["action"].as_str().unwrap(), Some(intrinsic(&row["args"])), &meta,
            &mut artifact_app_laws::fixture_identity(), app.1.cx(), policy.close,
        ).await.unwrap();
        let receipt = artifact_app_laws::settle_registered_typed_operation(
            &mut *app.0, meta.instance_id, policy, &mut artifact_app_laws::fixture_identity(), app.1.cx(),
        ).await.unwrap();
        assert!(receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
        let snapshot = app.snapshot().unwrap();
        let map = map_json(&snapshot);
        assert_eq!(feature_ids(&map), row["ids"]);
        assert_eq!(map["source"], law["map"]["source"]);
        assert_eq!(snapshot.elevation_source, initial.elevation_source);
        assert_eq!(snapshot.mesh, initial.mesh);
        if row["action"] == "addOverlayPosition" { assert_eq!(map["positions"][1], law["marker"]); }
        if let Some(check) = row.get("check") {
            assert_eq!(map[check["role"].as_str().unwrap()][check["index"].as_u64().unwrap() as usize], check["record"]);
        }
        revisions.push(map);
    }
    for (verb, expected) in [
        ("undo", revisions.iter().rev().skip(1).collect::<Vec<_>>()),
        ("redo", revisions.iter().skip(1).collect::<Vec<_>>()),
    ] {
        for map in expected {
            context::history_verb(&mut app, verb).await;
            let snapshot = app.snapshot().unwrap();
            assert_eq!(&map_json(&snapshot), map);
            assert_eq!(snapshot.elevation_source, initial.elevation_source);
        }
    }
    context::close(&mut app);
}

#[test]
fn overlay_edit_refuses_invalid_geometry_and_duplicate_ids() {
    let law: Value = serde_json::from_str(HISTORY).unwrap();
    let snapshot = GisTerrainSnapshot {
        imported_map: Some(ImportedMap::from_media_value(&intrinsic(&law["map"])).unwrap()),
        ..Default::default()
    };
    let history = HistoryView::empty();
    let view = ArtifactView::new(&snapshot, &history);
    let authored = GisTerrainSnapshot {
        elevation_source: crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document().elevation_source,
        ..snapshot.clone()
    };
    let authored_view = ArtifactView::new(&authored, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    for (action, cases, scenario) in [("addOverlayPosition", "invalid", &view), ("editOverlayShape", "invalidShapes", &view), ("addOverlayPosition", "invalidAuthored", &authored_view)] {
        for args in law[cases].as_array().unwrap() {
            let command = Gis3dPlayApp::command_from_action(action, &mut Some(intrinsic(args)), &mut None).unwrap();
            assert!(command.dispatch(scenario, &cfg).is_err(), "{args}");
        }
    }
}
