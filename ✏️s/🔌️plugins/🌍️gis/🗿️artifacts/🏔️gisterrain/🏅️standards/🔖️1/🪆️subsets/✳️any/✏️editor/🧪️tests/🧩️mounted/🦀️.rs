//! 🧩️ Exercises GIS Terrain through the registered editor, host effects, and artifact Store.

use semio_s_artifact_gis_gisterrain_editor as artifact;
use artifact::{editor::gis3d::*, mutations::GisTerrainMutation, GIS_3D_TERRAIN_SCHEMA};
use artifact::editor::gis3d::commands::{example::{self, set_active_example}, exaggeration::set_exaggeration, view::{configure_terrain_brush, set_camera}};
use artifact::editor::gis3d::modes::view::windows::terrain;
use semio_framework::InteractiveJobClassification;
use semio_framework_test_fixtures::fixture_text;
use semio_os_kernel as protocol;
use semio_os_kernel as store;
use semio_plugin::{ArtifactEditor, ArtifactView, EditorApp, Media, MediaClass, MediaForm, MediaPayload, MediaType};
use semio_ui_locale::LocalizedLabel;
use serde_json::{Value, json};

#[path = "../🧰️context/🦀️.rs"]
mod context;

#[path = "📦️archive/🦀️.rs"]
mod archive;

use context::{app, close, dispatch, gis3d_app_manifest_for_tests, render};

static RETAINED_LIMITS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../🧪️tests/🧫️fixtures/🧫️retained-command-limits/🔣️.json"));

fn geojson_fixture() -> &'static str { fixture_text!("../../🎮️commands/🌍️geojson/🧪️tests/🧫️fixtures/🔣️.geojson") }

//#region 🔖️CommandSurface
/// 🎯️ One value per `app_commands!` row, in row order.
fn every_command() -> Vec<Gis3dCommand> {
    use artifact::editor::gis3d::commands::{
        edit_elevation, elevation_source::{clear_elevation_source, create_elevation_grid}, geojson, import_elevation,
        overlays::{add_overlay_position, clear_overlays, edit_overlay_shape, move_overlay_position, remove_overlay_feature},
    };
    vec![
        Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.5 }),
        Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() }),
        Gis3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example::DEFAULT_EXAMPLE_ID.into() }),
        Gis3dCommand::ClearElevationSource(clear_elevation_source::ClearElevationSource {}),
        Gis3dCommand::EditElevation(edit_elevation::EditElevation { operation: "set-sample".into(), column: 16, row: 16, radius: 0, value: 36, stroke: None }),
        Gis3dCommand::AddOverlayPosition(add_overlay_position::AddOverlayPosition { id: "marker".into(), longitude: 5.55, latitude: 50.62, label: Some("Survey".into()), elevation: None }),
        Gis3dCommand::RemoveOverlayFeature(remove_overlay_feature::RemoveOverlayFeature { id: "marker".into() }),
        Gis3dCommand::ClearOverlays(clear_overlays::ClearOverlays {}),
        Gis3dCommand::ImportElevationGrid(import_elevation::ImportElevationGrid {}),
        Gis3dCommand::ImportElevationGridFile(import_elevation::file::ImportElevationGridFile { payload: "NCOLS 1".into() }),
        Gis3dCommand::EditOverlayShape(edit_overlay_shape::EditOverlayShape {
            id: "route".into(), kind: "route".into(), mode: "replace".into(),
            coordinates: "[[5.55,50.62],[5.56,50.63]]".into(), label: None,
        }),
        Gis3dCommand::MoveOverlayPosition(move_overlay_position::MoveOverlayPosition { id: "marker".into(), longitude: 5.56, latitude: 50.63 }),
        Gis3dCommand::CreateElevationGrid(create_elevation_grid::CreateElevationGrid { west: 5.55, south: 50.58, east: 5.61, north: 50.64, columns: 4, rows: 3, height_millimetres: -30_125 }),
        Gis3dCommand::ExportElevationGrid(import_elevation::export::ExportElevationGrid {}),
        Gis3dCommand::GradeTerrainRegion(artifact::editor::gis3d::commands::grade_region::GradeTerrainRegion { id: "region".into(), height_millimetres: 100_000 }),
        Gis3dCommand::ImportGeoJson(geojson::ImportGeoJson {}),
        Gis3dCommand::ImportGeoJsonFile(geojson::file::ImportGeoJsonFile { payload: geojson_fixture().into() }),
        Gis3dCommand::ExportGeoJson(geojson::export::ExportGeoJson {}),
        Gis3dCommand::GenerateTerrainContours(artifact::editor::gis3d::commands::contours::GenerateTerrainContours { interval_metres: 25.0 }),
        Gis3dCommand::ConfigureTerrainBrush(configure_terrain_brush::ConfigureTerrainBrush { radius: 7, elevation_step_millimetres: 31, smooth_strength: 65 }),
    ]
}

/// 🏷️ The wire keyword each row prints under — the kebab `as` literal, independent of the camelCase
/// manifest action id.
const WIRE_KEYWORDS: &[&str] = &[
    "exaggeration", "camera", "active-example", "clear-elevation-source", "edit-elevation",
    "add-overlay-position", "remove-overlay-feature", "clear-overlays",
    "import-elevation-grid", "import-elevation-grid-file", "edit-overlay-shape", "move-overlay-position",
    "create-elevation-grid",
    "export-elevation-grid",
    "grade-terrain-region",
    "import-geojson", "import-geojson-file", "export-geojson",
    "generate-terrain-contours", "configure-terrain-brush",
];

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(Gis3dCommand::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert!(ids.contains(&"importElevationGridFile") && ids.contains(&"importGeoJsonFile"));
    let common_factory = ids.into_iter().filter(|id| !matches!(*id, "importElevationGridFile" | "importGeoJsonFile" | "exportGeoJson")).collect::<Vec<_>>();
    assert_eq!(common_factory, GIS3D_RETAINED_TOOL_IDS, "every common retained command has a fixture");
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

/// 🎯️ Every app-declared action bridges through `command_from_action` and round-trips `command_id`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    semio_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Gis3dPlayApp>>(|| {
        let mut app = gis3d_app_manifest_for_tests();
        for (id, fixture) in [("importElevationGridFile", ascii_grid_fixture()), ("importGeoJsonFile", geojson_fixture())] {
            let callback = app.definition.actions.iter_mut().find(|action| action.id == id).expect("native picker callback");
            callback.args.push(semio_plugin::ActionArgDef::text("payload", LocalizedLabel::native("File", "Datei")).default_value(&fixture));
        }
        app
    }).await;
    assert!(Gis3dPlayApp::command_from_action("noSuchAction", &mut None, &mut None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn command_from_action_reads_the_nested_camera_object() {
    let mut args = Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "camera": { "position": [1.0, 2.0, 3.0] } })));
    let camera = Gis3dPlayApp::command_from_action("setCamera", &mut args, &mut None).expect("setCamera");
    assert!(matches!(camera, Gis3dCommand::SetCamera(ref payload) if payload.camera_json.contains("position")));
}

//#endregion 🔖️CommandSurface

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_gis3d_app();
    assert_eq!(definition.modes.len(), 1);
    assert_eq!(definition.window_kinds.len(), 1);
    assert!(definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).all(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated));
    assert!(!definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref().is_some_and(|key| key.starts_with("gis3d.play."))), "gis3d declares no app panels");
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == semio_s_artifact_gis_gismap::GISMAP_DIALECT.artifact_kind));
    assert!(!definition.artifact_kinds.iter().any(|kind| kind.id == "3d.mesh"));
}

/// 📐️ The reserved Measures and Engagements sections project for a Terrain window whose host utility register is non-empty.
#[semio_framework_async_macros::async_test]
async fn window_sections_project_while_the_host_utility_register_is_populated() {
    use semio_plugin::PluginApp;
    let mut app = app().await;
    let mut view = context::main_window_view();
    view.window_instances = vec![
        semio_plugin::ViewWindowInstance { id: "terrain-a".into(), window_kind_id: terrain::GIS3D_PLAY_WINDOW_MAIN.into() },
        semio_plugin::ViewWindowInstance { id: "terrain-b".into(), window_kind_id: terrain::GIS3D_PLAY_WINDOW_MAIN.into() },
    ];
    view.window_id = Some("terrain-a".into());
    view.active_utility_by_window_id.insert("terrain-a".into(), Some("terrainRaise".into()));
    view.active_utility_by_window_id.insert("terrain-b".into(), None);
    for _ in 0..2 {
        assert!(app.0.window_measures(&view).await.values().all(Vec::is_empty), "Terrain declares no window measures");
        assert!(app.0.window_engagements(&view).await.len() <= 2);
        assert!(app.0.tool_measures(&view).await.is_empty());
        assert_eq!(view.active_utility_by_window_id.len(), 2, "the section projection returns every borrowed alias of the host register");
    }
    view.active_utility_by_window_id.retire_cold();
    view.tool_run_trace_cursor_by_window_id.retire_cold();
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = app().await;
    assert!(render(&mut app, "gis3d.play.nope").await.contains("Unknown body"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn retained_commands_publish_only_their_declared_store_lanes() {
    fn lane_name(lane: &semio_plugin::app::TypedOperationResultLane) -> &'static str {
        match lane {
            semio_plugin::app::TypedOperationResultLane::Artifact => "artifact",
            semio_plugin::app::TypedOperationResultLane::Config => "config",
            semio_plugin::app::TypedOperationResultLane::WindowConfig => "window-config",
            semio_plugin::app::TypedOperationResultLane::Ui => "ui",
            semio_plugin::app::TypedOperationResultLane::Terminal => "terminal",
            lane => panic!("unexpected Terrain publication lane {lane:?}"),
        }
    }
    let mut app = app().await;
    let fixture: Value = serde_json::from_str((*RETAINED_LIMITS)).expect("GIS terrain retained completion oracle");
    let expected = |command: &str| fixture["publicationLanes"][command].as_array().expect("publication lanes").iter().map(|lane| lane.as_str().expect("publication lane").to_string()).collect::<Vec<_>>();
    let camera = dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: "{}".into() })).await;
    let camera_lanes = camera.lanes.iter().map(lane_name).map(str::to_string).collect::<Vec<_>>();
    assert_eq!(camera_lanes, expected("setCamera"));
    let exaggeration = dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.0 })).await;
    let exaggeration_lanes = exaggeration.lanes.iter().map(lane_name).map(str::to_string).collect::<Vec<_>>();
    assert_eq!(exaggeration_lanes, expected("setExaggeration"));
    assert_eq!(app.snapshot().expect("settled Terrain snapshot").exaggeration, 2.0);
    let example = dispatch(&mut app, Gis3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example::DEFAULT_EXAMPLE_ID.into() })).await;
    let example_lanes = example.lanes.iter().map(lane_name).map(str::to_string).collect::<Vec<_>>();
    assert_eq!(example_lanes, expected("setActiveExample"));
    assert_eq!(app.snapshot().expect("settled Terrain snapshot").exaggeration, 1.5, "the curated example's authored relief reaches the document");
    close(&mut app);
}
//#endregion 🔖️Manifest


//#region 🔖️Media
#[semio_framework_async_macros::async_test]
async fn export_media_scene_out_produces_a_workflow_mesh_document() {
    use semio_framework_value::{DslValue, FromValue};

    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = Gis3dPlayApp::export_media("scene:out", &doc).expect("scene:out export");
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
    assert_eq!(schema, "mesh.document");
    let payload: DslValue = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("workflow mesh document");
    assert_eq!(payload.get("schema").and_then(DslValue::as_str), Some(schema.as_str()));
    let mesh = semio_plugin::MeshData::from_value(payload.get("mesh").expect("mesh field").clone()).expect("mesh transfer");
    let expected = artifact::schema::imported_map::gis_terrain_mesh_buffers(&document).expect("triangle mesh");
    assert_eq!(mesh.positions, expected.positions);
    assert_eq!(mesh.indices, expected.indices);
    assert_eq!(mesh.normals, expected.normals);
    assert_eq!(mesh.uvs, expected.uvs);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn import_media_map_in_writes_the_imported_features_operation() {
    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let incoming = json!({ "positions": [{ "id": "imported-1", "lon": 1.0, "lat": 2.0 }] }).to_string();
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Intrinsic { schema: "2d.map".into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&incoming,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) } };
    let emit = Gis3dPlayApp::import_media("map:in", &media, &doc).expect("map:in import");
    use artifact::mutations::change_imported_features::ChangeImportedFeatures;
    assert_eq!(emit.artifact_mutations, vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: Some(artifact::schema::imported_map::ImportedMap::from_json(&incoming).unwrap()) })]);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn registered_map_import_replaces_and_replays_exact_snapshots() {
    use semio_plugin::artifact_app_laws::meta;

    let fixture: Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📥️map-history/🔣️.json")).expect("map history fixture");
    let maps = fixture["maps"].as_array().expect("two map revisions");
    let port = fixture["port"].as_str().expect("map input");
    let schema = fixture["schema"].as_str().expect("map media schema");
    let mut app = app().await;
    let initial = app.snapshot().expect("original terrain");
    let actor = meta("local");

    for (index, expected) in maps.iter().enumerate() {
        let json = expected.to_string();
        let payload = if index == 0 {
            let value = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("intrinsic map"));
            MediaPayload::Intrinsic { schema: schema.into(), value }
        } else {
            MediaPayload::Structured { schema: schema.into(), json }
        };
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload };
        context::import_media(&mut app, port, media, &actor).await;
        let imported = app.snapshot().expect("published terrain").imported_map.expect("published map");
        let actual: Value = serde_json::from_str(&imported.to_json().expect("exact JSON carrier")).expect("map oracle");
        assert_eq!(&actual, expected);
        let body = context::render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await;
        let scene = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_plugin::World3dScene>(&body).expect("rendered terrain");
        let world: Value = serde_json::from_str(scene.terrain_json.as_deref().expect("terrain grid lane")).unwrap();
        for kind in ["positions", "routes", "regions"] {
            let ids = |data: &Value| data.as_array().unwrap().iter().map(|feature| feature["id"].clone()).collect::<Vec<_>>();
            assert_eq!(ids(&world["overlays"][kind]), ids(&expected[kind]));
        }
    }

    let read = |app: &context::Gis3dApp| {
        let snapshot = app.snapshot().expect("terrain history projection");
        assert_eq!(snapshot.exaggeration, initial.exaggeration);
        assert_eq!(snapshot.mesh, initial.mesh);
        assert_eq!(snapshot.elevation_source, initial.elevation_source);
        snapshot.imported_map.map(|map| serde_json::from_str::<Value>(&map.to_json().unwrap()).unwrap())
    };
    context::history_verb(&mut app, "undo").await;
    assert_eq!(read(&app).as_ref(), Some(&maps[0]));
    context::history_verb(&mut app, "undo").await;
    let original_map = initial.imported_map.as_ref().map(|map| serde_json::from_str::<Value>(&map.to_json().unwrap()).unwrap());
    assert_eq!(read(&app), original_map);
    context::history_verb(&mut app, "redo").await;
    assert_eq!(read(&app).as_ref(), Some(&maps[0]));
    context::history_verb(&mut app, "redo").await;
    assert_eq!(read(&app).as_ref(), Some(&maps[1]));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn registered_elevation_import_persists_and_replays_authored_sources() {
    use semio_plugin::artifact_app_laws::meta;

    let law: Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🏔️elevation-history/🔣️.json")).unwrap();
    let size = law["resolution"].as_u64().unwrap() as usize;
    let actor = meta("local");
    let mut app = app().await;
    let initial = app.snapshot().unwrap().elevation_source;
    for (revision, intrinsic) in [(&law["first"], true), (&law["second"], false)] {
        let height_millimetres = revision["heightMeters"].as_i64().unwrap() * 1000;
        let expected = json!({ "bounds": revision["bounds"], "rowsMillimetres": vec![vec![height_millimetres; size]; size] });
        let schema = law["schema"].as_str().unwrap().into();
        let json = expected.to_string();
        let payload = if intrinsic {
            let value = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
            MediaPayload::Intrinsic { schema, value }
        } else {
            MediaPayload::Structured { schema, json }
        };
        let media = Media {
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            payload,
        };
        context::import_media(&mut app, law["port"].as_str().unwrap(), media, &actor).await;

        let snapshot = app.snapshot().unwrap();
        let history = semio_plugin::HistoryView::empty();
        let exported = Gis3dPlayApp::export_media("elevation:out", &ArtifactView::new(&snapshot, &history)).unwrap();
        let MediaPayload::Intrinsic { schema, value } = exported.payload else { panic!("intrinsic elevation output") };
        assert_eq!(schema, law["schema"].as_str().unwrap());
        let actual: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&value)).unwrap();
        assert_eq!(actual, expected);
    }
    for (verb, expected) in [("undo", &law["undo"]), ("redo", &law["redo"])] {
        for height in expected.as_array().unwrap() {
            context::history_verb(&mut app, verb).await;
            let snapshot = app.snapshot().unwrap();
            if let Some(height) = height.as_i64() {
                assert_eq!(snapshot.elevation_source.map(|source| i64::from(source.rows_millimetres[0][0])), Some(height * 1000));
            } else {
                assert_eq!(snapshot.elevation_source, initial);
            }
        }
    }
    close(&mut app);
}

/// 🎬️ Submits a Terrain media port and reports the pending owner if its fixture turns expire.
async fn submit_terrain_media(app: &mut context::Gis3dApp, port: &str) -> semio_plugin::ArtifactMediaExportHandle {
    app.renew_turn();
    semio_plugin::artifact_app_laws::submit_fixture_media_export(&mut *app.0, port, app.1.cx()).await.unwrap_or_else(|failure| {
        let progress = app.0.media.media_export_preparation.as_deref().map(|preparation| (
            std::mem::size_of_val(preparation),
            preparation.snapshot_source.ready(), preparation.snapshot.is_some(),
            preparation.original_read.is_some(), preparation.original_children.is_some(),
            preparation.lease.is_some(), preparation.completion.is_some(), preparation.chunks.is_some(),
            preparation.request.is_some(), preparation.assembled, preparation.published,
        ));
        panic!("submit Terrain media {port}: {failure:?}, preparation {progress:?}, remaining grant {:?}", app.1.cx().retained_grant())
    })
}

#[semio_framework_async_macros::async_test]
async fn interrupted_media_export_preparation_retires_under_mounted_policy() {
    use semio_plugin::{MountedOwnerPolicyV1, PluginApp};
    use semio_plugin::artifact_app_laws::{close_registered_fixture_app, ensure_fixture_cancellation_issuer};

    let mut app = app().await;
    app.renew_turn();
    ensure_fixture_cancellation_issuer(&mut *app.0, app.1.cx()).expect("birth Terrain media issuer");
    app.renew_turn();
    let mut receiver = None;
    assert!(!app.0.submit_media_export("scene:out", &mut receiver, app.1.cx()).await.expect("start preparing a scene export"));
    assert!(receiver.is_none());
    assert!(app.0.media.media_export_preparation.is_some());

    app.renew_turn();
    close_registered_fixture_app(&mut *app.0, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("retire interrupted preparation under mounted close credit");
    assert!(app.0.close_terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn registered_terrain_exports_publish_elevation_and_scene_media() {
    use semio_plugin::{ArtifactMediaExportPoll, MountedOwnerPolicyV1, PluginApp};
    use semio_plugin::artifact_app_laws::{OriginalFixtureCaller, poll_fixture_media_export, retain_fixture_media_export_result, take_fixture_media_export_chunk};

    let mut app = app().await;
    let initial = app.snapshot().unwrap();
    let maintenance = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance;
    for (port, expected_schema) in [("elevation:out", "gis.elevation-grid"), ("scene:out", "mesh.document")] {
        let handle = submit_terrain_media(&mut app, port).await;
        let mut result = None;
        for _ in 0..10_000 {
            match poll_fixture_media_export(&mut *app.0, &handle, app.1.cx()).await.unwrap() {
                ArtifactMediaExportPoll::Running { .. } => (),
                ArtifactMediaExportPoll::Complete(media) => { result = Some(media); break; }
                other => panic!("Terrain media export {port} refused: {other:?}"),
            }
        }
        let media = result.expect("terrain media export completed");
        assert_eq!(media.schema, expected_schema);
        retain_fixture_media_export_result(&mut *app.0, &handle, media, app.1.cx()).await.unwrap();
        let mut output = Vec::new();
        let mut terminal = false;
        while let Some(chunk) = take_fixture_media_export_chunk(&mut *app.0, &handle, &mut terminal, app.1.cx()).await.unwrap() {
            output.extend(chunk);
        }
        assert!(terminal, "Terrain media download {port} reached its terminal chunk");
        let actual: Value = serde_json::from_slice(&output).unwrap();
        if port == "elevation:out" {
            let source = initial.elevation_source.as_ref().expect("seeded authored elevation");
            assert_eq!(actual, json!({ "bounds": source.bounds, "rowsMillimetres": &source.rows_millimetres }));
        } else {
            assert_eq!(actual["schema"], expected_schema);
            let MediaPayload::Structured { json: expected, .. } = gis3d_scene_media(&initial).unwrap().payload else { unreachable!() };
            assert_eq!(actual, serde_json::from_str::<Value>(&expected).unwrap());
        }
        let operation = handle.operation_id.0;
        for _ in 0..OriginalFixtureCaller::maximum_turns() {
            if app.0.media.snapshot_retirements.get(operation).is_none() {
                break;
            }
            app.renew_turn();
            app.0.maintenance_step(app.1.cx(), maintenance).expect("retire the completed Terrain media download");
        }
        assert!(app.0.media.snapshot_retirements.get(operation).is_none(), "Terrain media download {port} returned its original snapshot");
    }
    assert_eq!(app.snapshot().unwrap(), initial);
    close(&mut app);
}


#[semio_framework_async_macros::async_test]
async fn mounted_terrain_export_can_cancel_a_live_projection_and_close() {
    use semio_plugin::{ArtifactMediaExportPoll, PluginApp};
    use semio_plugin::artifact_app_laws::poll_fixture_media_export;

    for worker_started in [false, true] {
        let mut app = app().await;
        let original = app.snapshot().unwrap();
        let handle = submit_terrain_media(&mut app, "scene:out").await;
        let mut at_cancellation_point = false;
        for _ in 0..10_000 {
            let poll = poll_fixture_media_export(&mut *app.0, &handle, app.1.cx()).await.unwrap();
            assert!(matches!(poll, ArtifactMediaExportPoll::Running { .. }), "{poll:?}");
            let active = app.0.media.media_exports.get(handle.operation_id.0).expect("retain the scene export");
            at_cancellation_point = if worker_started {
                active.outcome_pending || active.resume_pending
            } else {
                active.session_rejected.as_ref().is_some_and(|source| source.terminal_is_empty())
            };
            if at_cancellation_point {
                break;
            }
        }
        assert!(at_cancellation_point, "scene export did not reach the cancellation point with worker started = {worker_started}");
        app.0.cancel_media_export(&handle).await.unwrap();
        assert_eq!(app.snapshot().unwrap(), original);
        close(&mut app);
    }
}

#[semio_framework_async_macros::async_test]
async fn media_ports_declare_map_elevation_and_scene() {
    let ports = Gis3dPlayApp::media_ports().await;
    assert!(ports.iter().any(|port| port.id == "map:in"));
    assert!(ports.iter().any(|port| port.id == "elevation:in"));
    assert!(ports.iter().any(|port| port.id == "elevation:out"));
    assert!(ports.iter().any(|port| port.id == "scene:out"));
}

/// 🧭️ Relocated from the artifact's `⚙️engine` tests (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `gis3d_io`/`gis3d_scene_media`.
#[semio_framework_async_macros::async_test]
async fn gis3d_io_declares_the_map_in_and_scene_out_ports() {
    let io = gis3d_io();
    assert_eq!(io.artifact_schema, GIS_3D_TERRAIN_SCHEMA);
    let ports = io.all_ports().await;
    let map_in = ports.iter().find(|port| port.id == "map:in").expect("map:in declared");
    assert_eq!(map_in.direction, semio_plugin::MediaPortDirection::In);
    assert_eq!(map_in.kind_id.as_deref(), Some(semio_s_artifact_gis_gismap::GISMAP_DIALECT.artifact_kind));
    let scene_out = ports.iter().find(|port| port.id == "scene:out").expect("scene:out declared");
    assert_eq!(scene_out.direction, semio_plugin::MediaPortDirection::Out);
    assert!(scene_out.kind_id.is_none());
}

#[semio_framework_async_macros::async_test]
async fn scene_out_refuses_maps_without_local_elevation_coverage() {
    let mut document = artifact::standards::v1::subsets::any::io::text::snapshot::empty_gis_terrain_snapshot();
    document.elevation_source = None;
    document.imported_map = Some(artifact::schema::imported_map::ImportedMap::from_json(
        r#"{"positions":[{"id":"outside-coverage","lon":-122.42,"lat":37.77}]}"#,
    ).expect("imported map"));
    assert!(matches!(gis3d_scene_media(&document), Err(semio_plugin::MediaError::Payload(port, _)) if port == "scene:out"));
}
//#endregion 🔖️Media

//#region 🎬️StageableActionVocabulary
/// ⛰️ The terrain's one document mutation must be dispatchable from the Actions rail with nothing
/// typed, AND its staged default must differ from the fixture's own exaggeration — otherwise the rail
/// re-applies the value already in the document and the edit diffs to zero operations, which is
/// exactly how `setExaggeration` behaved before it had an argument schema.
#[semio_framework_async_macros::async_test]
async fn set_exaggeration_stages_a_bounded_default_that_actually_edits() {
    let definition = create_gis3d_app();
    let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == "setExaggeration").expect("setExaggeration is declared");
    assert_eq!(action.kind, semio_plugin::ActionKind::Mutation);
    assert!(action.in_palette, "the rail can stage it");
    let arg = action.args.iter().find(|arg| arg.id == "value").expect("a staged value");
    let semio_plugin::ArgSchema::Number { min, max, .. } = arg.schema else {
        panic!("exaggeration stages as a bounded number");
    };
    assert_eq!((min, max), (Some(GIS3D_EXAGGERATION_MINIMUM), Some(GIS3D_EXAGGERATION_MAXIMUM)));
    assert_eq!(arg.default.as_ref().and_then(semio_framework_value::DslValue::as_f64), Some(GIS3D_EXAGGERATION_STAGED_DEFAULT));

    let mut app = context::app().await;
    let seeded = app.snapshot().expect("projection").exaggeration;
    assert_ne!(seeded, GIS3D_EXAGGERATION_STAGED_DEFAULT, "a staged default equal to the seeded value would dispatch a no-op");
    let staged = semio_framework::effective_action_args(&action.args, &semio_framework_value::DslValue::Object(Vec::new()), None);
    let command = <EditorApp<Gis3dPlayApp> as semio_plugin::ArtifactApp>::command_from_action("setExaggeration", &mut Some(staged), &mut None).await.expect("the staged default bridges to a command");
    semio_plugin::artifact_app_laws::assert_undo_redo_round_trip(&mut *app.0, command, |app| app.snapshot().expect("projection").exaggeration, seeded, GIS3D_EXAGGERATION_STAGED_DEFAULT, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn clear_elevation_action_publishes_a_diff_and_undo_restores_the_grid() {
    let definition = create_gis3d_app();
    let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window))
        .find(|action| action.id == "clearElevationSource").expect("elevation action");
    assert!(action.in_palette);
    let mut app = context::app().await;
    let original = app.snapshot().expect("terrain example").elevation_source;
    assert!(original.is_some());
    let command = Gis3dPlayApp::command_from_action(&action.id, &mut None, &mut None).expect("elevation action bridge");
    let result = context::dispatch(&mut app, command).await;
    assert!(result.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
    assert!(app.snapshot().expect("cleared terrain").elevation_source.is_none());
    let body = context::render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await;
    assert!(body.contains("This terrain has no elevation grid."));
    context::history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("restored terrain").elevation_source, original);
    let body = context::render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await;
    assert!(body.contains("world-3d"));
    context::history_verb(&mut app, "redo").await;
    assert!(app.snapshot().expect("redone terrain").elevation_source.is_none());
    context::close(&mut app);
}
//#endregion 🎬️StageableActionVocabulary

fn ascii_grid_fixture() -> &'static str { fixture_text!("../../🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🔼️slope.asc") }

#[semio_framework_async_macros::async_test]
async fn mounted_esri_ascii_picker_imports_through_the_artifact_store_and_history() {
    use semio_framework::kernel::{import_chunk_arguments, import_payload_chunks};
    use semio_plugin::artifact_app_laws as laws;
    use artifact::editor::gis3d::commands::import_elevation;

    let mut app = context::app().await;
    let actor = laws::meta("local");
    let policy = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let original = app.snapshot().expect("initial terrain");
    let picked = laws::dispatch_fixture_action(&mut *app.0, import_elevation::REQUEST_ACTION, None, &actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("picker action");
    let picker = laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("publish ASCII Grid picker");
    assert!(picked.requested_effects.iter().chain(&picker.effects).any(|effect| matches!(effect,
        semio_plugin::Effect::RequestFileOpen { import_action, max_file_bytes: Some(import_elevation::MAX_FILE_BYTES), .. }
        if import_action == import_elevation::FILE_ACTION
    )));
    assert_eq!(app.snapshot().expect("terrain while awaiting picker"), original);

    let fixture: Value = serde_json::from_str(fixture_text!("../../🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let chunks = import_payload_chunks(ascii_grid_fixture());
    assert_eq!(chunks.len(), 1);
    let args = import_chunk_arguments("slope.asc", &chunks[0], None);
    laws::dispatch_fixture_action(&mut *app.0, import_elevation::FILE_ACTION, Some(args), &actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("picker result action");
    laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("retained elevation file publication");
    let imported = app.snapshot().expect("terrain with the local elevation grid");
    let source = imported.elevation_source.as_ref().expect("imported elevation grid");
    assert_eq!(json!(source.bounds), fixture["bounds"]);
    assert_eq!(source.rows_millimetres.iter().flatten().map(|&height| i64::from(height)).sum::<i64>(), fixture["checksum"].as_i64().unwrap() * 1000);
    assert_eq!(imported.exaggeration, original.exaggeration);
    assert_eq!(imported.imported_map, original.imported_map);

    context::history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("terrain after undo"), original);
    context::history_verb(&mut app, "redo").await;
    assert_eq!(app.snapshot().expect("terrain after redo"), imported);

    let export = laws::dispatch_fixture_action(&mut *app.0, import_elevation::export::ACTION, None, &actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("export action");
    let download = laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("publish ASCII Grid download");
    let exported = export.requested_effects.iter().chain(&download.effects).find_map(|effect| match effect {
        semio_plugin::Effect::DownloadMediaExport { filename, data, encoding: None, .. } if filename == "terrain-elevation.asc" => Some(data),
        _ => None,
    }).expect("native ASCII Grid download");
    assert!(exported.starts_with("NCOLS 33\nNROWS 33\nXLLCENTER 6.625\nYLLCENTER 44.625\nCELLSIZE 0.015625\n"));
    assert_eq!(app.snapshot().expect("terrain after download"), imported);
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn mounted_maximum_ascii_picker_publishes_one_history_edit() {
    use artifact::editor::gis3d::commands::import_elevation;
    use semio_framework::kernel::{import_chunk_arguments, import_payload_chunks};
    use semio_plugin::artifact_app_laws as laws;

    let fixture: Value = serde_json::from_str(fixture_text!("../../🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let maximum = &fixture["maximumPicker"];
    let side = maximum["side"].as_u64().unwrap() as usize;
    let size = maximum["bytes"].as_u64().unwrap() as usize;
    assert_eq!(size, import_elevation::MAX_FILE_BYTES as usize);
    let mut payload = format!(
        "NCOLS {side}\nNROWS {side}\nXLLCENTER {}\nYLLCENTER {}\nCELLSIZE {}\nNODATA_VALUE {}\n",
        maximum["west"], maximum["south"], maximum["cellSize"], maximum["noData"].as_str().unwrap()
    );
    for row in 0..side {
        let elevation = maximum[if row == 0 { "northSample" } else { "otherSample" }].as_str().unwrap();
        payload.push_str(&vec![elevation; side].join(" "));
        payload.push('\n');
    }
    payload.extend(std::iter::repeat_n(' ', size - payload.len()));
    let chunks = import_payload_chunks(&payload);
    assert_eq!(chunks.len(), maximum["chunkCount"].as_u64().unwrap() as usize);

    let mut app = context::app().await;
    let actor = laws::meta("local");
    let policy = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let original = app.snapshot().expect("initial terrain");
    for chunk in &chunks {
        let args = import_chunk_arguments("signed-maximum.asc", chunk, None);
        app.renew_turn();
        let admitted = laws::dispatch_fixture_action(
            &mut *app.0, import_elevation::FILE_ACTION, Some(args), &actor,
            &mut laws::fixture_identity(), app.1.cx(), policy.close,
        ).await.expect("admit picker chunk");
        assert!(admitted.mutations.is_empty());
        assert_eq!(app.snapshot().unwrap(), original);
    }
    app.renew_turn();
    let completed = laws::settle_registered_typed_operation(
        &mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx(),
    ).await.expect("publish complete ASCII Grid");
    assert_eq!(completed.completions, 1);
    let imported = app.snapshot().unwrap();
    let source = imported.elevation_source.as_ref().unwrap();
    assert_eq!(json!(source.bounds), maximum["bounds"]);
    assert_eq!(source.rows_millimetres.len(), side);
    for (index, row) in source.rows_millimetres.iter().enumerate() {
        let expected = maximum[if index + 1 == side { "northMillimetres" } else { "otherMillimetres" }].as_i64().unwrap() as i32;
        assert_eq!(row.as_slice(), vec![expected; side]);
    }
    context::history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().unwrap(), original);
    context::history_verb(&mut app, "redo").await;
    assert_eq!(app.snapshot().unwrap(), imported);
    context::close(&mut app);
}

mod geojson_exchange {
    //! 🧪️ Verifies GeoJSON geometry, property fidelity, and Terrain artifact history.

    use super::{artifact, context};
    use artifact::editor::gis3d::commands::{geojson::{FILE_ACTION, REQUEST_ACTION, export}, import_elevation::MAX_FILE_BYTES};
    use semio_framework_value::DslValue;
    use semio_plugin::{ActionMeta, Effect, MountedOwnerPolicyV1, PluginApp};
    use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::text::geojson as codec;

    use semio_framework_value::ToValue;
    use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::geojson::io::text::snapshot::read_geojson_text;

    use semio_framework_test_fixtures::fixture_text;
    static FIXTURE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../🎮️commands/🌍️geojson/🧪️tests/🧫️fixtures/🔣️.geojson"));

    /// 🔎️ Captures the operation admitted by the original GeoJSON command dispatch.
    async fn start_export(app: &mut context::Gis3dApp, actor: &ActionMeta) -> semio_framework_job::Operation {
        use semio_plugin::artifact_app_laws as laws;

        let policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
        app.renew_turn();
        laws::dispatch_fixture_action(&mut *app.0, export::ACTION, None, actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("start GeoJSON export");
        let operations = &app.0.tools.latest_wins_commands;
        assert_eq!(operations.len(), 1);
        let (_, id) = operations.next_id_from(0).expect("admitted export slot");
        let pending = operations.get(id).expect("admitted export owner");
        assert_eq!(pending.admission.as_ref().expect("export registration").verb, export::ACTION);
        assert_eq!(pending.metadata().unwrap().instance_id, actor.instance_id);
        pending.operation
    }

    #[semio_framework_async_macros::async_test]
    async fn geojson_picker_download_and_history_use_the_registered_terrain() {
        use semio_framework::kernel::{import_chunk_arguments, import_payload_chunks};
        use semio_plugin::artifact_app_laws as laws;

        let mut app = context::app().await;
        let actor = laws::meta("local");
        let policy = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
        let original = app.snapshot().unwrap();
        let open = laws::dispatch_fixture_action(&mut *app.0, REQUEST_ACTION, None, &actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("GeoJSON picker");
        let picker = laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("publish GeoJSON picker");
        assert!(open.requested_effects.iter().chain(&picker.effects).any(|effect| matches!(effect,
            Effect::RequestFileOpen { import_action, .. } if import_action == FILE_ACTION
        )));
        assert_eq!(app.snapshot().unwrap(), original);
        let chunks = import_payload_chunks(*FIXTURE);
        assert_eq!(chunks.len(), 1);
        let args = import_chunk_arguments("survey.geojson", &chunks[0], None);
        laws::dispatch_fixture_action(&mut *app.0, FILE_ACTION, Some(args), &actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("GeoJSON picker callback");
        laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("retained overlay edit");
        let imported = app.snapshot().unwrap();
        assert!(imported.imported_map.as_ref().expect("saved GeoJSON").same(&codec::decode(*FIXTURE, MAX_FILE_BYTES as usize).unwrap()));
        assert_eq!(imported.elevation_source, original.elevation_source);

        let export = laws::dispatch_fixture_action(&mut *app.0, export::ACTION, None, &actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("GeoJSON export");
        let receipt = laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("settled download");
        let data = export.requested_effects.iter().chain(&receipt.effects).find_map(|effect| match effect {
            Effect::DownloadMediaExport { filename, mime_type, data, encoding: None } if filename == "terrain-overlays.geojson" && mime_type == "application/geo+json" => Some(data),
            _ => None,
        }).expect("native GeoJSON download");
        assert_eq!(read_geojson_text(data).unwrap().features, read_geojson_text(*FIXTURE).unwrap().features);
        assert_eq!(app.snapshot().unwrap(), imported);
        context::history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().unwrap(), original);
        context::history_verb(&mut app, "redo").await;
        assert_eq!(app.snapshot().unwrap(), imported);
        context::close(&mut app);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewport_position_move_has_one_undoable_overlay_publication() {
        use artifact::editor::gis3d::commands::{elevation_source::create_elevation_grid::CreateElevationGrid, overlays::add_overlay_position::AddOverlayPosition};
        use artifact::editor::gis3d::Gis3dCommand;
        use semio_framework_test_fixtures::repo_text;
        use semio_plugin::artifact_app_laws as laws;
        let fixture: serde_json::Value = serde_json::from_str(repo_text!("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🗺️overlay/🧪️tests/🧫️fixtures/🖱️move-position.json")).unwrap();
        let marker = &fixture["marker"];
        let id = marker["id"].as_str().unwrap();
        let action = fixture["descriptor"]["action"].as_str().unwrap();
        let drop = &fixture["cases"][0]["action"];
        let mut app = context::app().await;
        let grid: serde_json::Value = serde_json::from_str(repo_text!("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🧪️tests/🧫️fixtures/📦️elevation-grid/🔣️.json")).unwrap();
        let [west, south, east, north] = serde_json::from_value(grid["bounds"].clone()).unwrap();
        context::dispatch(&mut app, Gis3dCommand::CreateElevationGrid(CreateElevationGrid {
            west, south, east, north, columns: 33, rows: 33, height_millimetres: 40_000,
        })).await;
        context::dispatch(&mut app, Gis3dCommand::AddOverlayPosition(AddOverlayPosition {
            id: id.into(),
            longitude: marker["coordinates"][0].as_f64().unwrap(),
            latitude: marker["coordinates"][1].as_f64().unwrap(),
            label: marker["label"].as_str().map(str::to_owned),
            elevation: marker["elevation"].as_f64(),
        })).await;
        let before = app.snapshot().unwrap();
        let actor = laws::meta("local");
        let policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
        app.renew_turn();
        let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({
            "id": id, "longitude": drop["longitude"].as_f64().unwrap(), "latitude": drop["latitude"].as_f64().unwrap(),
            "windowId": "gis3d-main", "surfaceId": "gis3d.play.composite"
        }));
        laws::dispatch_fixture_action(&mut *app.0, action, Some(args), &actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("admit viewport move");
        laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("publish viewport move");

        let moved = app.snapshot().unwrap();
        assert_eq!(moved.elevation_source, before.elevation_source);
        let data = codec::encode(moved.imported_map.as_ref()).unwrap();
        let geojson: serde_json::Value = serde_json::from_str(&data).unwrap();
        let feature = geojson["features"].as_array().unwrap().iter().find(|entry| entry["id"] == id).expect("moved GeoJSON pin");
        assert_eq!(feature["geometry"]["coordinates"], serde_json::json!([drop["longitude"], drop["latitude"], marker["elevation"]]));
        let pins = &moved.imported_map.as_ref().unwrap().positions;
        let pin = pins.iter().find(|feature| feature.get("id").and_then(DslValue::as_str) == Some(id)).unwrap();
        assert_eq!(pin.get("label").and_then(DslValue::as_str), marker["label"].as_str());
        context::history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().unwrap(), before);
        context::history_verb(&mut app, "redo").await;
        assert_eq!(app.snapshot().unwrap(), moved);
        context::close(&mut app);
    }

    #[semio_framework_async_macros::async_test]
    async fn geojson_large_download_acks_and_cancelled_job_has_no_download() {
        use semio_plugin::artifact_app_laws as laws;
        use semio_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        use semio_plugin::app::{ArtifactOutputChunks, TypedOperationResultLane};

        let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🎮️commands/🌍️geojson/🧪️tests/🧫️fixtures/🔣️large.json")).unwrap();
        let count = fixture["routeCount"].as_u64().unwrap();
        let text = "s".repeat(fixture["noteWidth"].as_u64().unwrap() as usize);
        let routes: Vec<_> = (0..count).map(|index| serde_json::json!({
            "id": format!("contour-{index}"),
            "points": fixture["coordinates"],
            "semioTerrainContour": true,
            "elevationMetres": fixture["elevationMetres"],
            "note": text,
        })).collect();
        let payload = MediaPayload::Structured { schema: "2d.map".into(), json: serde_json::json!({"routes": routes}).to_string() };
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload };
        let mut app = context::app().await;
        let actor = laws::meta("local");
        let policy = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
        context::import_media(&mut app, "map:in", media, &actor).await;
        let original = app.snapshot().unwrap();

        let operation = start_export(&mut app, &actor).await.operation.0;
        app.renew_turn();
        let receipt = laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("download result and exact page acknowledgments");
        assert!(receipt.lanes.contains(&TypedOperationResultLane::Download));
        let mut payload = Vec::new();
        while let Some(page) = app.0.take_segmented_download_chunk(operation).await.expect("segmented page") {
            assert!(!page.is_empty() && page.len() <= ArtifactOutputChunks::CHUNK_BYTES);
            payload.extend(page);
            assert!(payload.len() <= 256 * 1024);
        }
        assert!(payload.len() > codec::MAX_DOCUMENT_BYTES);
        assert!(app.0.take_segmented_download_chunk(operation).await.is_err());
        let collection: serde_json::Value = serde_json::from_slice(&payload).unwrap();
        let features = collection["features"].as_array().unwrap();
        assert_eq!(features.len(), count as usize);
        assert_eq!(features[0]["properties"]["elevationMetres"], fixture["elevationMetres"]);
        assert_eq!(features[0]["properties"]["semioTerrainContour"], true);
        assert_eq!(features[count as usize - 1]["id"], format!("contour-{}", count - 1));

        let active = start_export(&mut app, &actor).await;
        let mut ownership_receipts = Vec::new();
        for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
            if app.0.tools.tool_operations.get(active.operation.0).is_some() {
                break;
            }
            app.renew_turn();
            app.0.maintenance_step(app.1.cx(), policy.maintenance).expect("retire GeoJSON admission sources");
            app.renew_turn();
            app.0.advance_typed_operation_publication(&mut laws::fixture_identity(), app.1.cx()).await.expect("mount the admitted GeoJSON worker");
            laws::retain_original_ownership_receipts(&mut *app.0, policy, &mut ownership_receipts).expect("acknowledge GeoJSON worker admission");
        }
        let mounted = app.0.tools.tool_operations.get(active.operation.0).expect("the admitted GeoJSON export mounted");
        assert_eq!(mounted.operation.operation, active.operation);
        assert_eq!(mounted.operation.generation, active.generation);
        assert_eq!(mounted.operation.base_revision, active.base_revision);
        assert!(!mounted.terminal_seen && mounted.cancellation_lease.is_some());
        let cancel = DslValue::object([
            ("operationId".into(), format!("{:016x}", active.operation.0).to_value()),
            ("generation".into(), format!("{:016x}", active.generation.0).to_value()),
        ]);
        app.renew_turn();
        laws::dispatch_fixture_action(&mut *app.0, "cancelTypedOperation", Some(cancel), &actor, &mut laws::fixture_identity(), app.1.cx(), policy.close).await.expect("cancel the exact export generation");
        app.renew_turn();
        let stopped = laws::settle_registered_typed_operation(&mut *app.0, actor.instance_id, policy, &mut laws::fixture_identity(), app.1.cx()).await.expect("cancelled export retirement");
        assert!(!stopped.lanes.contains(&TypedOperationResultLane::Download));
        assert!(app.0.take_segmented_download_chunk(active.operation.0).await.is_err());
        assert_eq!(app.snapshot().unwrap(), original);
        context::close(&mut app);
    }
}
