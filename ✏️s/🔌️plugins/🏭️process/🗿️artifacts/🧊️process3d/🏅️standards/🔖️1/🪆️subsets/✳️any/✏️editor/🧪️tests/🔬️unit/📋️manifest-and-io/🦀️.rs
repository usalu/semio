//! 📋️ Process3d editor unit tests: the manifest stitch, the utilities, the typed media io and the media export and import.

use super::*;

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_process3d_app()).expect("app definition json");
    assert!(json.contains(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN), "window kind missing from the manifest");
    assert!(json.contains(edit::PROCESS3D_MODE_EDIT), "mode missing from the manifest");
    for body in [PROCESS_3D_PLAY_BODY_ARTIFACT, PROCESS_3D_PLAY_BODY_CATALOGUE, PROCESS_3D_PLAY_BODY_WORKSHOP, PROCESS_3D_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("3d.process"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_declares_four_flat_utilities_scoped_to_workpiece_window() {
    let definition = create_process3d_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["select", "cut", "drill", "attach"], "utilities declared in registry order");
    assert!(definition.utilities.iter().all(|utility| utility.group.is_none()), "process's select/cut/drill/attach are the window's entire top-level utility set, so none carry a visual group",);
    let window = definition.window_kinds.iter().find(|window| window.id == workpiece::PROCESS_3D_PLAY_WINDOW_MAIN).expect("workpiece window");
    let scoped: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(scoped, ["select", "cut", "drill", "attach"], "all four utilities scoped to the workpiece window kind");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️IoTests
/// 🔤️ `AppIo.export_formats`/`import_formats` (unlike `ArtifactKindSpec`) have no `export_stdio_kinds`/
/// `import_stdio_kinds` string-id peer and are never read by `register_app_io`, so they stay empty
/// here in step with `artifact_kind()`'s own now-empty lists (see that fn's doc).
#[semio_framework_async_macros::async_test]
async fn process3d_io_mirrors_the_declared_artifact_kind() {
    let io = process3d_io();
    assert_eq!(io.artifact_schema, semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA);
    assert_eq!(io.artifact.id, "3d.process");
    assert!(io.export_formats.is_empty());
    assert!(io.import_formats.is_empty());
}

/// 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe:
/// `geometry:in` and `brep:out` are declared with the right direction/kind/multiplicity.
#[semio_framework_async_macros::async_test]
async fn process3d_io_declares_geometry_in_and_brep_out_ports() {
    let io = process3d_io();
    let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
    assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
    assert!(geometry_in.kind_id.is_none());
    assert!(!geometry_in.required);
    assert_eq!(geometry_in.multiplicity, semio_framework_plugin::PortMultiplicity::Many);

    let brep_out = io.ports.iter().find(|port| port.id == "brep:out").expect("brep:out declared");
    assert_eq!(brep_out.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(brep_out.kind_id.as_deref(), Some("3d.process"));
    assert!(!brep_out.required);
    assert_eq!(brep_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    assert_eq!(brep_out.media_type.class, MediaClass::ThreeD);
    assert_eq!(brep_out.media_type.form, MediaForm::Brep);
}
//#endregion 🔖️IoTests

//#region 🔖️MediaTests
#[semio_framework_async_macros::async_test]
async fn export_brep_out_returns_step_text_structured_payload() {
    semio_framework_os_kernel::io::register_format_descriptors(semio_s_artifact_stdio_step::formats().expect("STEP format descriptors")).await.expect("register stdio format descriptors");
    let document = semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = Process3dPlayApp::export_media("brep:out", &doc).expect("export brep:out");
    assert_eq!(media.media_type.class, MediaClass::ThreeD);
    assert_eq!(media.media_type.form, MediaForm::Brep);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "3d.process");
            assert!(!json.is_empty());
        }
        MediaPayload::Binary { .. } | MediaPayload::Intrinsic { .. } => panic!("expected a Structured payload"),
    }
}

#[semio_framework_async_macros::async_test]
async fn export_unknown_port_is_not_implemented() {
    let document = semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    assert!(matches!(Process3dPlayApp::export_media("nonsense:out", &doc), Err(MediaError::NotImplemented)));
}

#[semio_framework_async_macros::async_test]
async fn import_geometry_in_rejects_unrecognized_schema() {
    let document = semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "unknown.schema".into(), json: "irrelevant".into() } };
    assert!(matches!(Process3dPlayApp::import_media("geometry:in", &media, &doc), Err(MediaError::Payload(port, _)) if port == "geometry:in"));
}
//#endregion 🔖️MediaTests
