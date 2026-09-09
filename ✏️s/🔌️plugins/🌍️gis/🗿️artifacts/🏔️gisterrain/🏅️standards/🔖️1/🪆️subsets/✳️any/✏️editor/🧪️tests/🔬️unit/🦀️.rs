use super::*;
use crate::editor::gis3d::testkit::{app, close, dispatch, gis3d_app_manifest_for_testkit, render};
use semio_framework_plugin::EditorApp;
use serde_json::json;

const RETAINED_LIMITS: &str = include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json");

//#region 🔖️CommandSurface
/// 🎯️ One value per `app_commands!` row, in row order.
fn every_command() -> Vec<Gis3dCommand> {
    vec![Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.5 }), Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() })]
}

/// 🏷️ The wire keyword each row prints under — the kebab `as` literal, independent of the camelCase
/// manifest action id.
const WIRE_KEYWORDS: &[&str] = &["exaggeration", "camera"];

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(Gis3dCommand::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 2, "every Gis3dCommand row must be covered by every_command()");
}

#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    assert_eq!(every_command().len(), WIRE_KEYWORDS.len());
    for (command, keyword) in every_command().iter().zip(WIRE_KEYWORDS) {
        store::os_store::test_support::assert_op_text_binary_equivalence(command);
        let printed = protocol::OpText::print_op(command);
        assert!(printed == *keyword || printed.starts_with(&format!("{keyword} ")), "row {} printed {printed:?}, expected the {keyword:?} wire keyword", command.command_id());
    }
}

/// 🎯️ Every declared action maps to a typed command. The pre-migration `gis3d_ui` crate had NO
/// `command_from_action` override at all — it inherited the trait default, which errors for every
/// action, so the whole `{action,args}` host wire was dead. That crate never compiled (see the
/// migration ticket), which is why the gap was invisible; this test locks the fix in.
/// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
/// `command_id`. Uses the framework's own harness, which stages each action's declared args and
/// knows the framework-injected ids to skip (`undo`/`copy`/`recordTutorial`/…).
///
/// 🩹️ This is the test that would have caught the pre-migration gap: `gis3d_ui` had NO
/// `command_from_action` override, so every declared action fell through to the trait default's
/// hard error and the whole `{action,args}` host wire was dead.
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Gis3dPlayApp>>(gis3d_app_manifest_for_testkit).await;
    assert!(Gis3dPlayApp::command_from_action("noSuchAction", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn command_from_action_reads_the_nested_camera_object() {
    let camera = Gis3dPlayApp::command_from_action("setCamera", Some(&dsl::json::to_dsl_value(&dsl::json!({ "camera": { "position": [1.0, 2.0, 3.0] } })))).expect("setCamera");
    assert!(matches!(camera, Gis3dCommand::SetCamera(ref payload) if payload.camera_json.contains("position")));
}

#[test]
fn retained_command_factory_matches_the_language_neutral_maximum_oracle() {
    let fixture: Value = serde_json::from_str(RETAINED_LIMITS).expect("GIS terrain retained limits decode through serde_json");
    let maximum = fixture.get("maximumTextBytes").and_then(Value::as_u64).expect("maximumTextBytes") as usize;
    let additional = fixture.get("rejectedAdditionalBytes").and_then(Value::as_u64).expect("rejectedAdditionalBytes") as usize;
    let expected_items = fixture.get("expectedWorkItems").and_then(Value::as_u64).expect("expectedWorkItems") as usize;
    let tool_ids = fixture.get("toolIds").and_then(Value::as_array).expect("toolIds").iter().map(|value| value.as_str().expect("tool id")).collect::<Vec<_>>();
    assert_eq!(maximum, GIS3D_RETAINED_RAW_BYTES);
    assert_eq!(expected_items, GIS3D_RETAINED_WORK_ITEMS);
    assert_eq!(tool_ids, GIS3D_RETAINED_TOOL_IDS);
    let snapshot = default_terrain_document();
    let interaction = protocol::InteractionState::default();
    let accepted = Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: format!("{{}}{}", " ".repeat(maximum - 2)) });
    let rejected = Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: format!("{{}}{}", " ".repeat(maximum + additional - 2)) });
    assert_eq!(gis3d_retained_extent(&accepted, &snapshot, &interaction), Some(expected_items));
    assert_eq!(gis3d_retained_extent(&rejected, &snapshot, &interaction), None);
    let factory = Gis3dCommandJobFactory::new("s.gis.gisterrain@1/*#editor");
    assert_eq!(factory.execution_contract(), ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500));
    assert!(Gis3dPlayApp::command_from_action("setCamera", Some(&dsl::json::to_dsl_value(&dsl::json!({ "cameraJson": "c".repeat(maximum + additional) })))).is_err());
}
//#endregion 🔖️CommandSurface

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_gis3d_app();
    assert_eq!(definition.modes.len(), 1);
    assert_eq!(definition.window_kinds.len(), 1);
    assert!(definition.window_kinds.iter().flat_map(|window| &window.actions).all(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated));
    // 🧷️ gis3d declares no app panel tabs of its own; whatever is present comes from the framework.
    assert!(!definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref().is_some_and(|key| key.starts_with("gis3d.play."))), "gis3d declares no app panels");
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == semio_s_artifact_gis_gismap::GISMAP_DIALECT.artifact_kind));
    // 🧱️ `3d.mesh` is NO LONGER independently registered here (ticket
    // `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — duplicate `ArtifactKindSpec` deleted, see
    // `crate::🦀️.rs`'s removal comment). `scene:out`'s
    // `kind_id: Some("3d.mesh".into())` media-port tag (asserted separately below) still
    // references the canonical kind by id; this manifest just no longer redundantly declares it.
    assert!(!definition.artifact_kinds.iter().any(|kind| kind.id == "3d.mesh"), "3d.mesh is composed via GisTerrainSnapshot.mesh now, never a standalone ArtifactKindSpec");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = app().await;
    assert!(render(&mut app, "gis3d.play.nope").await.contains("Unknown body"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn retained_commands_publish_only_their_declared_store_lanes() {
    fn lane_name(lane: &semio_framework_plugin::app::TypedOperationResultLane) -> &'static str {
        match lane {
            semio_framework_plugin::app::TypedOperationResultLane::Artifact => "artifact",
            semio_framework_plugin::app::TypedOperationResultLane::Config => "config",
            semio_framework_plugin::app::TypedOperationResultLane::WindowConfig => "window-config",
            semio_framework_plugin::app::TypedOperationResultLane::Ui => "ui",
            semio_framework_plugin::app::TypedOperationResultLane::Terminal => "terminal",
            lane => panic!("unexpected Terrain publication lane {lane:?}"),
        }
    }
    let mut app = app().await;
    let fixture: Value = serde_json::from_str(RETAINED_LIMITS).expect("GIS terrain retained completion oracle");
    let expected = |command: &str| fixture["publicationLanes"][command].as_array().expect("publication lanes").iter().map(|lane| lane.as_str().expect("publication lane").to_string()).collect::<Vec<_>>();
    let camera = dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: "{}".into() })).await;
    let camera_lanes = camera.lanes.iter().map(lane_name).map(str::to_string).collect::<Vec<_>>();
    assert_eq!(camera_lanes, expected("setCamera"));
    let exaggeration = dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.0 })).await;
    let exaggeration_lanes = exaggeration.lanes.iter().map(lane_name).map(str::to_string).collect::<Vec<_>>();
    assert_eq!(exaggeration_lanes, expected("setExaggeration"));
    assert_eq!(app.snapshot().expect("settled Terrain snapshot").exaggeration, 2.0);
    close(&mut app);
}
//#endregion 🔖️Manifest

//#region 🔖️Media
#[semio_framework_async_macros::async_test]
async fn export_media_scene_out_produces_a_3d_mesh_structured_payload() {
    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = Gis3dPlayApp::export_media("scene:out", &doc).expect("scene:out export");
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
    assert_eq!(schema, "3d.mesh");
    assert!(json.contains("exaggeration"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn import_media_map_in_writes_the_imported_features_operation() {
    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let incoming = json!({ "positions": [{ "id": "imported-1", "lon": 1.0, "lat": 2.0 }] }).to_string();
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.map".into(), json: incoming.clone() } };
    let emit = Gis3dPlayApp::import_media("map:in", &media, &doc).expect("map:in import");
    use crate::mutations::change_imported_features::ChangeImportedFeatures;
    assert_eq!(emit.artifact_mutations, vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: incoming })]);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn media_ports_declare_map_in_and_scene_out() {
    let ports = Gis3dPlayApp::media_ports().await;
    assert!(ports.iter().any(|port| port.id == "map:in"));
    assert!(ports.iter().any(|port| port.id == "scene:out"));
}

/// 🧭️ Relocated from the artifact's `⚙️engine` tests (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `gis3d_io`/`gis3d_scene_media`.
#[semio_framework_async_macros::async_test]
async fn gis3d_io_declares_the_map_in_and_scene_out_ports() {
    let io = gis3d_io();
    assert_eq!(io.document_schema, GIS_3D_TERRAIN_SCHEMA);
    let ports = io.all_ports().await;
    let map_in = ports.iter().find(|port| port.id == "map:in").expect("map:in declared");
    assert_eq!(map_in.direction, semio_framework_plugin::MediaPortDirection::In);
    assert_eq!(map_in.kind_id.as_deref(), Some(semio_s_artifact_gis_gismap::GISMAP_DIALECT.artifact_kind));
    let scene_out = ports.iter().find(|port| port.id == "scene:out").expect("scene:out declared");
    assert_eq!(scene_out.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(scene_out.kind_id.as_deref(), Some("3d.mesh"));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_scene_media_exports_the_terrain_descriptor() {
    let document = default_terrain_document();
    let media = gis3d_scene_media(&document);
    let MediaPayload::Structured { schema, json } = media.payload else {
        panic!("expected a structured scene:out payload");
    };
    assert_eq!(schema, "3d.mesh");
    assert!(json.contains("exaggeration"));
}
//#endregion 🔖️Media
