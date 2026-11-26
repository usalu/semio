//! 📤️ First IFC export and committed repaint through the mounted BIM editor.
use semio_plugin::{ArtifactEditor as _, Effect};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::context::{bim_app, dispatch, render_text, view};
use semio_s_artifact_bim_model_bim::editor::bim::{BimCommand, BimModelApp};
use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_entity::CreateEntity;
use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
use semio_s_artifact_bim_model_move_storey::editor::bim::commands::place_elements;
use semio_ui_locale::Locale;

#[semio_framework_async_macros::async_test]
async fn the_mounted_editor_analyzes_and_downloads_both_ifc_dialects() {
    use protocol::Inference as _;
    use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::encode;
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
    use semio_s_artifact_stdio_contract::part21::parse_part21;

    let mut app = bim_app().await;
    for (kind, parent, name) in [("slab-type", "", "Floor type"), ("slab", "st-ground", "Ground floor")] {
        let floor = BimCommand::CreateEntity(CreateEntity { kind: kind.into(), parent: parent.into(), name: name.into() });
        assert!(dispatch(&mut app, floor).await.edited_document());
    }
    let initial = view(Locale::En, &[(plan::WINDOW_KIND_ID, plan::WINDOW_KIND_ID)], Some(plan::WINDOW_KIND_ID));
    let mut previous_plan = None;
    let mut previous_ifc4 = None;
    for formats in [&["ifc2x3", "ifc4"][..], &["ifc4"][..]] {
        let snapshot = app.snapshot().expect("snapshot");
        assert!(!snapshot.walls.is_empty() && !snapshot.slabs.is_empty());
        let surface = render_text(&mut app, plan::BODY_KEY, &initial).await;
        let plan = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_plugin::Canvas2dScene>(&surface).expect("assembled plan scene").layers_json;
        assert!(plan.contains("w-south"), "the ground plan paints the authored south wall");
        let first_paint = previous_plan.is_none();
        if let Some(previous) = previous_plan.as_ref() {
            assert_ne!(previous, &plan);
        }
        previous_plan = Some(plan);
        for &format in formats {
            let arguments = semio_plugin::DslValue::object([("format".to_string(), semio_plugin::DslValue::String(format.into()))]);
            let command = BimModelApp::command_from_action("exportModel", &mut Some(arguments), &mut None).expect("export action");
            let result = dispatch(&mut app, command).await;
            assert!(!result.edited_document());
            let [Effect::DownloadMediaExport { filename, mime_type, data, encoding: None }] = result.requested_effects.as_slice() else {
                panic!("expected the mounted IFC download: {:?}", result.requested_effects);
            };
            assert!(filename.ends_with(".ifc"));
            assert_eq!(mime_type, "application/x-step");
            let document = parse_part21(data).expect("the owning Part-21 codec accepts the file");
            assert_eq!(document.by_type("IfcProject").count(), 1);
            assert!(document.by_type("IfcWallStandardCase").chain(document.by_type("IfcWall")).count() > 0);
            assert!(document.by_type("IfcSlab").count() > 0);
            let inferred = ModelInference::infer(&snapshot).expect("independent cold inference");
            let expected = encode(format, &snapshot, &inferred).expect("the registered serializer");
            assert_eq!(data, &expected.data);
            if format == "ifc4" {
                if let Some(previous) = previous_ifc4.replace(data.clone()) {
                    assert_ne!(&previous, data);
                }
            }
        }
        let analyze = BimModelApp::command_from_action("analyseModel", &mut None, &mut None).expect("analyze action");
        assert!(!dispatch(&mut app, analyze).await.edited_document());
        assert_eq!(app.snapshot().expect("snapshot"), snapshot);
        if first_paint {
            let edit = BimCommand::PlaceAt(place_elements::PlaceAt { ids: vec!["w-south".into()], at: "1, 4".into() });
            assert!(dispatch(&mut app, edit).await.edited_document());
        }
    }
}
