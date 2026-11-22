//! 🪜️ Mounted step composition and inspector mutations through the editor action registry.

use crate::context::{app, published_a_document_mutation, settled_action};
use semio_framework::DslValue;
use pack::snapshot::ArtifactPack;
use semio_s_artifact_process_process3d_model::snapshot::{Process3dSnapshot, ProcessMeasure, WorkingSolid};

#[semio_framework_async_macros::async_test]
async fn composed_step_timeline_persists_inspector_edits_and_order() {
    use semio_s_artifact_process_process3d_model::snapshot::{flow_child_handle, flow_snapshot_for_steps, genesis_process3d_child_pack, process_working_scene_from_snapshot, working_solid_child_handle};
    use semio_framework_value::ToValue as _;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️composed-steps.json")).expect("neutral step scenario");
    let mut app = app();
    let original = app.snapshot().expect("initial document");
    let start = original.step_payloads.len();
    for measure in fixture["steps"].as_array().expect("step kinds") {
        let args = DslValue::object([("measure".into(), DslValue::String(measure.as_str().expect("measure kind").into()))]);
        assert!(published_a_document_mutation(&settled_action(&mut app, "addStep", Some(&args)).1));
    }
    let added = app.snapshot().expect("document with all three steps");
    let cut_id = added.step_payloads[start].id.clone();
    let drill_id = added.step_payloads[start + 1].id.clone();
    let move_kind = fixture["move"]["measure"].as_str().expect("measure to move");
    let move_index = fixture["move"]["index"].as_u64().expect("destination index") as usize;
    let move_id = added.step_payloads[start..].iter().find(|step| step.measure.kind_slug() == move_kind).expect("step to move").id.clone();

    for edit in fixture["edits"].as_array().expect("inspector edits") {
        let value = edit["value"].as_str().map(|text| DslValue::String(text.into())).unwrap_or_else(|| DslValue::float(edit["value"].as_f64().expect("numeric edit")));
        let args = DslValue::object([
            ("target".into(), DslValue::String(cut_id.clone())),
            ("field".into(), DslValue::String(edit["field"].as_str().expect("field").into())),
            ("value".into(), value),
        ]);
        assert!(published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&args)).1));
        assert!(!published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&args)).1));
    }
    let drill_radius = fixture["drillRadius"].as_f64().expect("drill radius");
    let drill_args = |radius| DslValue::object([("target".into(), DslValue::String(drill_id.clone())), ("field".into(), DslValue::String("radius".into())), ("value".into(), DslValue::float(radius))]);
    assert!(!published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&drill_args(-drill_radius))).1));
    assert!(published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&drill_args(drill_radius))).1));

    let mut cut = app.snapshot().expect("document with inspector edits").step_payloads[start].clone();
    let same_step = DslValue::object([("step".into(), cut.to_value())]);
    assert!(!published_a_document_mutation(&settled_action(&mut app, "updateStep", Some(&same_step)).1));
    cut.enabled = false;
    let updated_step = DslValue::object([("step".into(), cut.to_value())]);
    assert!(published_a_document_mutation(&settled_action(&mut app, "updateStep", Some(&updated_step)).1));
    let move_args = DslValue::object([("id".into(), DslValue::String(move_id)), ("index".into(), (start + move_index).to_value())]);
    assert!(published_a_document_mutation(&settled_action(&mut app, "moveStep", Some(&move_args)).1));
    let edited = app.snapshot().expect("edited document");
    let loaded = Process3dSnapshot::decode_pack(&edited.encode_pack()).expect("persisted document");
    let scene = process_working_scene_from_snapshot(&loaded);
    let ProcessMeasure::Drill { radius, .. } = scene.steps[start + 2].measure else { panic!("the reordered drill is last"); };
    let actual = serde_json::json!({
        "order": scene.steps[start..].iter().map(|step| step.measure.kind_slug()).collect::<Vec<_>>(),
        "label": scene.steps[start].label,
        "positionX": scene.steps[start].measure.pose().position[0],
        "drillRadius": radius,
    });
    assert_eq!(actual, fixture["expected"]);
    assert_eq!(scene.steps[start], cut);
    assert_eq!(scene.steps[..start], original.step_payloads);
    let tool_ids = scene.steps.iter().filter_map(|step| {
        let solid = match &step.measure {
            ProcessMeasure::Cut { tool, .. } => tool,
            ProcessMeasure::Attach { component, .. } => component,
            ProcessMeasure::Drill { .. } => return None,
        };
        let handle = working_solid_child_handle(&format!("tool-{}", step.id), solid);
        assert!(loaded.tool_solids.contains(&handle));
        Some((step.id.clone(), handle.child_id))
    }).collect();
    let flow = flow_snapshot_for_steps(&scene.steps, &tool_ids);
    assert_eq!(loaded.steps, flow_child_handle(&flow));
    let pack = genesis_process3d_child_pack(&loaded, "steps", &loaded.steps.child_id).expect("timeline flow pack");
    assert_eq!(pack, flow.encode_pack());
    assert_eq!(flow.edges[start].from.node, scene.steps[start].id);
    assert_eq!(flow.edges[start].to.node, scene.steps[start + 1].id);
    assert_eq!(flow.edges[start + 1].to.node, scene.steps[start + 2].id);
}

#[semio_framework_async_macros::async_test]
async fn stock_resize_publishes_authored_geometry_and_composed_child_together() {
    use semio_s_artifact_process_process3d_model::snapshot::{genesis_process3d_child_pack, process_working_scene_from_snapshot, working_solid_child_handle};

    let mut app = app();
    let before = app.snapshot().expect("initial stock");
    let mut solid = before.stock_payload.solid.clone();
    let WorkingSolid::Box { width, .. } = &mut solid else { panic!("the default stock is a box"); };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️composed-steps.json")).expect("neutral scenario");
    let resize = &fixture["stockResize"];
    assert_eq!(*width, resize["beforeWidth"].as_f64().expect("initial width"));
    let requested = resize["width"].as_f64().expect("requested width");
    *width = requested;
    let args = DslValue::object([
        ("target".into(), DslValue::String(before.stock_id.clone())),
        ("field".into(), DslValue::String("width".into())),
        ("value".into(), DslValue::String(requested.to_string())),
    ]);
    assert!(published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&args)).1));
    assert!(!published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&args)).1));
    let after = app.snapshot().expect("resized stock");
    let loaded = Process3dSnapshot::decode_pack(&after.encode_pack()).expect("reopened document");
    let scene = process_working_scene_from_snapshot(&loaded);
    assert_eq!(scene.stock.solid, solid);
    let volume = semio_s_artifact_process_process3d_inferences::processed_volume(&scene, Some(0)).expect("resized stock volume");
    assert!((volume - resize["volume"].as_f64().expect("expected volume")).abs() < 1e-9);
    assert_ne!(loaded.stock_solid, before.stock_solid);
    assert_eq!(loaded.stock_solid, working_solid_child_handle("stock", &solid));
    assert!(genesis_process3d_child_pack(&loaded, "stockSolid", &loaded.stock_solid.child_id).is_some());
    assert_eq!(loaded.step_payloads, before.step_payloads);
    assert_eq!(loaded.steps, before.steps);
}

#[semio_framework_async_macros::async_test]
async fn machine_inspector_updates_only_changed_parameters() {
    let mut app = app();
    let before = app.snapshot().expect("initial workshop");
    let (machine, capability, parameter) = before.workshop.machines.iter().find_map(|machine| {
        machine.capabilities.iter().find_map(|capability| {
            let parameter = capability.parameters.iter().find(|parameter| parameter.value >= 0.001)?;
            Some((machine, capability, parameter))
        })
    }).expect("editable machine parameter");
    let args = |value: f64| DslValue::object([
        ("target".into(), DslValue::String(format!("machine:{}", machine.id))),
        ("field".into(), DslValue::String(format!("{}.{}", capability.id, parameter.id))),
        ("value".into(), DslValue::String(value.to_string())),
    ]);
    let original = parameter.value;
    assert!(!published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&args(original))).1));
    let requested = original + 0.125;
    assert!(published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&args(requested))).1));
    assert!(!published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&args(requested))).1));
    let after = app.snapshot().expect("edited workshop");
    let edited = after.workshop.machines.iter().find(|edited| edited.id == machine.id).expect("edited machine");
    let edited_capability = edited.capabilities.iter().find(|edited| edited.id == capability.id).expect("edited capability");
    let edited_parameter = edited_capability.parameters.iter().find(|edited| edited.id == parameter.id).expect("edited parameter");
    assert_eq!(edited_parameter.value, requested);
}

#[test]
fn inspector_patches_the_same_ids_that_the_panel_resolves() {
    use semio_plugin::{ArtifactView, ConfigView, HistoryView};
    use semio_s_artifact_process_process3d_commands::inspector::patch_inspector::{PatchInspector, handle};
    use semio_s_artifact_process_process3d_config::Process3dConfig;
    use semio_s_artifact_process_process3d_kit::Process3dDispatchCtx;
    use semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document;
    use semio_s_artifact_process_process3d_mutations::Process3dMutation;

    let mut snapshot = default_document();
    snapshot.stock_id = format!("machine:{}", snapshot.workshop.machines[0].id);
    snapshot.step_payloads[0].id = "step:crosscut".into();
    snapshot.step_payloads[1].id = "machine:uninstalled".into();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = Process3dConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let rename = |target: &str| {
        let mut ctx = Process3dDispatchCtx { interaction: Default::default(), view_state: None };
        let patch = PatchInspector { target: target.into(), field: "label".into(), number: None, text: Some("Edited".into()) };
        handle(&patch, &doc, &cfg, &mut ctx).expect("inspector mutation").artifact_mutations
    };

    assert!(matches!(rename(&snapshot.stock_id).as_slice(), [Process3dMutation::ChangeStockLabel(_)]));
    for id in ["step:crosscut", "machine:uninstalled"] {
        assert!(matches!(rename(id).as_slice(), [Process3dMutation::RenameStep(rename)] if rename.id == id));
    }
    assert!(rename("step:step:crosscut").is_empty());
}

#[test]
fn imported_cube_reopens_and_exports_using_the_same_geometry() {
    use semio_s_artifact_process_process3d_inferences::{processed_mesh, processed_volume};
    use semio_s_artifact_process_process3d_io::{export_process3d_model, import_process3d_model};
    use semio_s_artifact_process_process3d_model::snapshot::{genesis_process3d_child_pack, process_working_scene_from_snapshot, PROCESS3D_IMPORTED_BREP_PACK_BYTES};
    use pack::snapshot::ArtifactDsl;

    let model = include_bytes!("../../🧫️fixtures/📦️import-stock.obj");
    let url = format!("data:application/octet-stream;base64,{}", base64_codec::base64_standard_encode(model));
    let imported = import_process3d_model("cube.obj", &url).expect("closed cube within the picker and geometry limits");
    let reopened = Process3dSnapshot::decode_pack(&imported.encode_pack()).expect("reopen binary process");
    let reopened_text = Process3dSnapshot::parse_dsl(&reopened.print_dsl()).expect("reopen text process");
    assert_eq!(reopened_text, imported);
    let WorkingSolid::ImportedSolid { brep_pack } = &reopened.stock_payload.solid else {
        panic!("closed OBJ must carry a durable B-Rep");
    };
    assert!(!brep_pack.is_empty() && brep_pack.len() <= PROCESS3D_IMPORTED_BREP_PACK_BYTES);
    assert!(genesis_process3d_child_pack(&reopened, "stockSolid", &reopened.stock_solid.child_id).is_some());
    let scene = process_working_scene_from_snapshot(&reopened);
    assert!((processed_volume(&scene, None).expect("restored cube volume") - 8.0).abs() < 1e-8);
    assert!(processed_mesh(&scene, None).expect("replayed cube mesh").is_some());
    let export = export_process3d_model(&scene, None, "obj").expect("restored geometry exports").expect("OBJ output");
    assert!(export.filename.ends_with(".obj"));
}

#[test]
fn malformed_brep_and_oversized_picked_model_report_failures() {
    use semio_s_artifact_process_process3d_inferences::processed_mesh;
    use semio_s_artifact_process_process3d_io::{import_process3d_model, Process3dImportError, PROCESS3D_MODEL_DATA_URL_BYTES};
    use semio_s_artifact_process_process3d_model::snapshot::{ProcessWorkingScene, PROCESS3D_MODEL_FILE_BYTES};

    let header = "data:application/octet-stream;base64,";
    let over = format!("{header}{}", "A".repeat(PROCESS3D_MODEL_DATA_URL_BYTES + 1 - header.len()));
    assert!(matches!(import_process3d_model("cube.obj", &over), Err(Process3dImportError::InputTooLarge)));
    let over = format!("{header}{}", base64_codec::base64_standard_encode(vec![0; PROCESS3D_MODEL_FILE_BYTES as usize + 1]));
    assert!(over.len() <= PROCESS3D_MODEL_DATA_URL_BYTES);
    assert!(matches!(import_process3d_model("cube.obj", &over), Err(Process3dImportError::InputTooLarge)));
    let mut scene = ProcessWorkingScene::default();
    scene.stock.solid = WorkingSolid::ImportedSolid { brep_pack: "AQID".into() };
    assert!(processed_mesh(&scene, None).is_err(), "a malformed durable solid must not show substitute geometry");
}

#[test]
fn imported_brep_pack_has_its_own_encoded_geometry_budget() {
    use semio_s_artifact_process_process3d_model::snapshot::{brep_snapshot_for_working_solid, pack_imported_brep, PROCESS3D_IMPORTED_BREP_PACK_BYTES};

    let mut snapshot = brep_snapshot_for_working_solid(&WorkingSolid::Box { width: 2.0, depth: 2.0, height: 2.0 });
    let WorkingSolid::ImportedSolid { brep_pack } = pack_imported_brep(&snapshot).expect("ordinary box fits") else {
        panic!("expected an exact B-Rep carrier");
    };
    assert!(brep_pack.len() <= PROCESS3D_IMPORTED_BREP_PACK_BYTES);
    let vertex = snapshot.vertices[0].clone();
    snapshot.vertices.extend((0..8192).map(|index| {
        let mut vertex = vertex.clone();
        vertex.id = format!("extra-{index}");
        vertex.point.x = f64::from(index);
        vertex
    }));
    assert!(snapshot.encode_pack().len().div_ceil(3) * 4 > PROCESS3D_IMPORTED_BREP_PACK_BYTES);
    assert!(pack_imported_brep(&snapshot).is_none());
}

#[semio_framework_async_macros::async_test]
async fn ordinary_step_and_stl_stock_fit_the_picked_model_contract() {
    use semio_framework::kernel::Effect;
    use semio_s_artifact_process_process3d_inferences::processed_volume;
    use semio_s_artifact_process_process3d_io::{export_process3d_model, PROCESS3D_MODEL_DATA_URL_BYTES, PROCESS3D_MODEL_FILE_BYTES};
    use semio_s_artifact_process_process3d_model::snapshot::{ProcessWorkingScene, process_working_scene_from_snapshot};

    let mut source = ProcessWorkingScene::default();
    source.stock.solid = WorkingSolid::Box { width: 2.0, depth: 2.0, height: 2.0 };
    let mut app = app();
    let mut imported_volume = |filename: String, source_bytes: Vec<u8>| {
        assert!(source_bytes.len() <= PROCESS3D_MODEL_FILE_BYTES as usize, "{filename} needs {} bytes", source_bytes.len());
        let encoded = base64_codec::base64_standard_encode(source_bytes);
        let url = format!("data:application/octet-stream;base64,{encoded}");
        assert!(url.len() <= PROCESS3D_MODEL_DATA_URL_BYTES, "{filename} needs {} encoded bytes", url.len());
        let args = DslValue::object([("name".into(), DslValue::String(filename)), ("payload".into(), DslValue::String(url))]);
        let result = settled_action(&mut app, "importModelFile", Some(&args)).0;
        let pack = result.requested_effects.iter().find_map(|effect| match effect {
            Effect::LoadDocument { pack, .. } => Some(pack),
            _ => None,
        }).expect("cube imports through the retained editor");
        let reopened = Process3dSnapshot::decode_pack(pack).expect("binary reopen");
        processed_volume(&process_working_scene_from_snapshot(&reopened), None).expect("exact solid replays")
    };
    for format in ["step", "stl"] {
        let export = export_process3d_model(&source, None, format).expect("cube export").expect("solid file");
        let data = export.data.as_str().expect("file payload");
        let source_bytes = if export.encoding.as_deref() == Some("base64") {
            base64_codec::base64_standard_decode(data).expect("exported binary file")
        } else {
            data.as_bytes().to_vec()
        };
        let volume = imported_volume(export.filename, source_bytes);
        assert!((volume - 8.0).abs() < 1e-8, "{format}: {volume}");
    }
    let mut millimetre_box = include_bytes!("../../../../../../../../../../../🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🧪️tests/🧫️fixtures/⚽️cut-sphere-from-box/🅰️operand-a.step").to_vec();
    assert!(millimetre_box.len() < PROCESS3D_MODEL_FILE_BYTES as usize);
    millimetre_box.resize(PROCESS3D_MODEL_FILE_BYTES as usize, b' ');
    let volume = imported_volume(format!("{}.STEP", "制".repeat(250)), millimetre_box);
    assert!((volume - 8e-6).abs() < 1e-12, "20 mm cube in cubic metres: {volume}");
}

#[test]
fn picked_step_with_two_solids_requires_one_workpiece() {
    use semio_s_artifact_process_process3d_inferences::ProcessKernelReplay;
    use semio_s_artifact_process_process3d_io::{import_process3d_model, Process3dImportError, PROCESS3D_MODEL_FILE_BYTES};

    let mut replay = ProcessKernelReplay::new();
    let first = replay.kernel_mut().box_prim_sync(1.0, 1.0, 1.0).unwrap();
    let second = replay.kernel_mut().translate_sync(&first, [2.0, 0.0, 0.0]).unwrap();
    let source = semio_s_artifact_stdio_step::geometry::export_step(replay.kernel(), &[first, second]).unwrap();
    assert!(source.len() <= PROCESS3D_MODEL_FILE_BYTES as usize);
    let encoded = base64_codec::base64_standard_encode(source.into_bytes());
    let url = format!("data:application/octet-stream;base64,{encoded}");
    assert!(matches!(import_process3d_model("assembly.step", &url), Err(Process3dImportError::MultipleSolids)));
}

/// 🩻️ A STEP whose edges name a curve the reader cannot translate reports that cause, never a missing solid.
#[test]
fn malformed_step_reports_the_reader_cause_instead_of_a_missing_solid() {
    use semio_s_artifact_process_process3d_inferences::ProcessKernelReplay;
    use semio_s_artifact_process_process3d_io::{import_process3d_model, Process3dImportError};

    let mut replay = ProcessKernelReplay::new();
    let cube = replay.kernel_mut().box_prim_sync(1.0, 1.0, 1.0).unwrap();
    let source = semio_s_artifact_stdio_step::geometry::export_step(replay.kernel(), &[cube]).unwrap();
    assert!(source.contains("= LINE("));
    let malformed = source.replace("= LINE(", "= OFFSET_CURVE_3D(");
    let url = format!("data:application/octet-stream;base64,{}", base64_codec::base64_standard_encode(malformed.into_bytes()));
    let error = import_process3d_model("cube.step", &url).expect_err("an untranslatable edge curve is refused");
    assert!(matches!(&error, Process3dImportError::InvalidData(cause) if cause.contains("OFFSET_CURVE_3D")), "{error:?}");
    assert!(error.to_string().contains("OFFSET_CURVE_3D"));
}

#[test]
fn open_glb_remains_displayable_after_reopen_without_admitting_solid_operations() {
    use semio_framework::mesh_io::{GlbExporter, GlbImporter, MeshExporter, MeshImporter};
    use semio_s_artifact_process_process3d_inferences::processed_volume;
    use semio_s_artifact_process_process3d_io::import_process3d_model;
    use semio_s_artifact_process_process3d_model::snapshot::{decode_process3d_model_data_url, process_working_scene_from_snapshot};

    let mut mesh = semio_plugin::MeshData {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        indices: vec![0, 1, 2],
        ..Default::default()
    };
    mesh.compute_normals();
    let bytes = GlbExporter.export(&mesh).expect("triangle GLB");
    let url = format!("data:application/octet-stream;base64,{}", base64_codec::base64_standard_encode(&bytes));
    let imported = import_process3d_model("triangle.GLB", &url).expect("display-only GLB");
    let reopened = Process3dSnapshot::decode_pack(&imported.encode_pack()).expect("binary reopen");
    let WorkingSolid::ImportedMesh { mesh_url } = &reopened.stock_payload.solid else {
        panic!("open geometry must remain a display-only mesh");
    };
    let reopened_bytes = decode_process3d_model_data_url(mesh_url).expect("persisted GLB file");
    assert_eq!(reopened_bytes, bytes);
    let preview = GlbImporter.import(&reopened_bytes).expect("preview mesh");
    assert_eq!(preview.positions, mesh.positions);
    assert_eq!(preview.indices, mesh.indices);
    assert!(processed_volume(&process_working_scene_from_snapshot(&reopened), None).is_err());
}
