//! ⚙️ Operation tests of the CAD editor unit tests: the composed panes, object operations, the derived transformations and the file effects. See [`super`].

use super::*;

//#region 🔖️Operations
/// 🈳️ A new document composes every pane from its first render: `default_document()` names the four EMPTY model children,
/// each derives an empty genesis pack, and `addObject` refuses by name only where no model child is composed at all.
#[semio_framework_async_macros::async_test]
async fn a_new_document_composes_four_empty_panes_and_add_object_needs_a_composed_pane() {
    let scene = default_document();
    for pane in CadPaneId::all() {
        let child = semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&scene, pane).unwrap_or_else(|| panic!("{pane:?} pane composes its empty model child"));
        assert_eq!(child.child_id, semio_s_artifact_cad_cad_model::snapshot::cad_empty_pane_child(pane).child_id);
        let pack = semio_s_artifact_cad_cad_play::cad_genesis_child_pack(&scene, semio_s_artifact_cad_cad_model::snapshot::cad_pane_model_slot(pane), &child.child_id).unwrap_or_else(|| panic!("{pane:?} pane derives its genesis pack"));
        let model = <semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack).expect("genesis pack decodes");
        assert!(model.elements.is_empty(), "{pane:?} pane is born empty");
    }
    let fault = drive_result(&CadPlayApp::default(), &empty_cad_snapshot(), "addObject", Some(json!({ "typology": "building.building.column" })), &CadConfig::default()).err().expect("a pane without a model child refuses addObject");
    assert_eq!(fault.code.0, "cad.object.pane-uncomposed");
}

/// 🧱️ `addObject` lands ONE `insert-element` on the addressed pane's model child and never touches the parent document
/// (design §20.15).
#[semio_framework_async_macros::async_test]
async fn add_object_through_wrapper_grows_the_composed_pane() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("snapshot");
    let objects_before = pane_objects(&app, CadPaneId::Shape).await;
    app.0.dispatch_typed(CadCommand::AddObject(add_object::AddObject { typology: Some("building.building.column".into()) }), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("add object dispatch");
    settle(&mut app).await;
    assert_eq!(app.snapshot().expect("snapshot"), before, "addObject never moves the parent document");
    let objects = pane_objects(&app, CadPaneId::Shape).await;
    assert_eq!(objects.len(), objects_before.len() + 1, "the shape pane's model child holds one more element");
    assert_eq!(objects.last().map(|object| object.typology.as_str()), Some("building.building.column"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn derive_transformation_populates_energy_pane() {
    let object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let derived = run_derive_from_geometry(&mut kernel, &[object], "energy");
    assert!(!derived.is_empty());
    assert!(derived.iter().any(|object| object.typology.starts_with("energy.energy.")));
}

#[semio_framework_async_macros::async_test]
async fn forest_transformation_uses_live_shape_pane() {
    let live_box = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let box_derived = run_derive_from_geometry(&mut kernel, &[live_box], "energy");
    assert!(!box_derived.is_empty(), "a live box must derive at least a hull");

    let live_wall = make_object_for_typology("building.building.wall", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let wall_derived = run_derive_from_geometry(&mut kernel, &[live_wall], "energy");
    assert!(!wall_derived.is_empty(), "a live wall panel must derive at least a hull");

    fn derived_typologies(objects: &[semio_s_artifact_cad_cad_model::geometry::CadObject]) -> Vec<String> {
        let mut rows: Vec<String> = objects.iter().map(|object| object.typology.clone()).collect();
        rows.sort();
        rows
    }
    fn derived_faces(objects: &[semio_s_artifact_cad_cad_model::geometry::CadObject]) -> Vec<String> {
        let mut rows: Vec<String> = objects.iter().map(|object| format!("{} @ [{:.3}, {:.3}, {:.3}]", object.typology, object.origin[0], object.origin[1], object.origin[2])).collect();
        rows.sort();
        rows
    }
    let box_typologies = derived_typologies(&box_derived);
    let wall_typologies = derived_typologies(&wall_derived);
    assert_eq!(
        wall_typologies, box_typologies,
        "classification reads face normals only, so a wall panel and a cube must carry the same typology multiset:\n  box:  {box_typologies:?}\n  wall: {wall_typologies:?}"
    );
    let box_faces = derived_faces(&box_derived);
    let wall_faces = derived_faces(&wall_derived);
    assert_ne!(
        wall_faces, box_faces,
        "a thin wall panel and a cube must place their classified faces differently, proving the derive tracks the LIVE input's real shape, not a memoized result:\n  box:  {box_faces:?}\n  wall: {wall_faces:?}"
    );
}

#[semio_framework_async_macros::async_test]
async fn save_selected_emits_download_effect() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig::default();
    let emit = drive_in_window(&app, &scene, "saveSelected", None, &config, "cad-shape-secondary", shape::WINDOW_KIND_ID).expect("addressed CAD window export");
    assert!(emit.artifact_mutations.is_empty(), "export must not mutate the document");
    assert_eq!(emit.effects.len(), 1);
    match &emit.effects[0] {
        Effect::DownloadMediaExport { filename, data, .. } => {
            assert_eq!(filename, "cad.selected.spatial.dsl");
            assert!(!data.contains("activeModelDefinitionId"));
            assert!(data.contains("spatial.shape"));
        }
        other => panic!("expected DownloadMediaExport, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn load_raw_request_emits_file_open_effect() {
    let app = CadPlayApp::default();
    let emit = drive(&app, &default_document(), "loadRawRequest", None);
    match &emit.effects[0] {
        Effect::RequestFileOpen { import_action, read_as, .. } => {
            assert_eq!(import_action, "importCadFile");
            assert_eq!(read_as.as_deref(), Some("dataUrl"));
        }
        other => panic!("expected RequestFileOpen, got {other:?}"),
    }
}
//#endregion 🔖️Operations
