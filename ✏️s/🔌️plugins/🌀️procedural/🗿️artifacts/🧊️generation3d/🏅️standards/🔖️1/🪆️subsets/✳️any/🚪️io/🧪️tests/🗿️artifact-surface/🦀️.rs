//! 📄️ `s.procedural.generation3d@1/*` DOCUMENT-IO SURFACE — the gate that would have failed on the
//! finding ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane closed.
//!
//! **What this holds.** `📓️io-codecs-2026-09-09.md` proved the nine leaves round-trip; the
//! user-journey audit then found that no command, menu item, button or keybinding in `✏️editor` or
//! `👁️viewer` named a single one of them (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1) —
//! library-complete IO with no way in or out. These laws hold the SURFACE: the roster the export
//! picker publishes, the extensions the file picker accepts, the filename and MIME every download
//! carries, and the import roster equal to the declared import dialects.
//!
//! **Why they are fixture-driven.** Every row is read from
//! `🧫️fixtures/🚪️io/🗿️artifact-surface.json`, whose TypeScript twin
//! (`🗿️artifact-surface/🟦️.ts`) re-derives the same answers without importing any of this — so a
//! roster that drifts in one implementation fails against the other rather than quietly becoming
//! the new truth.
//!
//! **Where the third-party oracle enters.** The geometry rows do not stop at "bytes came out":
//! every geometry format's export goes back in through its own import leaf and is handed to
//! `parry3d`'s `MassProperties::from_trimesh` — the same independent library the sibling round-trip
//! lane uses — so the surface is proved to move the committed unit cube, not merely to move bytes.
//!
//! @see ../../🦀️.rs — `document_io`, the composition point under test.
//! @see ../../../✏️editor/🎮️commands/📤️export-document, ../../../✏️editor/🎮️commands/📥️import-document.

use crate::{assert_oracle_agrees_on_unit_cube, project, retire_document, unit_cube_semio_mesh};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::document_io;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::{export_stdio_kinds, import_stdio_kinds, mesh_bridge};

//#region 🧫️Fixture
const SURFACE_JSON: &str = include_str!("../../../🧫️fixtures/🚪️io/🗿️artifact-surface.json");
const SCENE_IMPORT_JSON: &str = include_str!("../../🧫️fixtures/🎬️scene-import/🔣️.json");

/// 🎬️ Scene placement is authored as graph parameters and agrees with independent matrix algebra.
#[test]
fn gltf_scene_import_preserves_hierarchy_and_instances_as_editable_transforms() {
    use semio_framework_artifact_flow_flow::Widget;
    use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfDocument, GltfNode};
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::gltf::v2_0::any::apply_scene_nodes;
    use parry3d::na::{Matrix4, Quaternion, Translation3, UnitQuaternion, Vector3};
    let source: serde_json::Value = serde_json::from_str(SCENE_IMPORT_JSON).unwrap();
    let oracle = |node: &GltfNode| -> Matrix4<f64> {
        if let Some(matrix) = node.matrix { return Matrix4::from_column_slice(&matrix); }
        let [x, y, z] = node.translation.unwrap_or([0.0; 3]);
        let [qx, qy, qz, qw] = node.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        Translation3::new(x, y, z).to_homogeneous() * UnitQuaternion::from_quaternion(Quaternion::new(qw, qx, qy, qz)).to_homogeneous() * Matrix4::new_nonuniform_scaling(&Vector3::from(node.scale.unwrap_or([1.0; 3])))
    };
    let payload = mesh_bridge::polygon_mesh_from_mesh_data(&crate::unit_cube_mesh_data()).unwrap().to_string();
    for row in source["cases"].as_array().unwrap() {
        let document: GltfDocument = serde_json::from_value(row["document"].clone()).unwrap();
        let count = document.meshes.iter().map(|mesh| mesh.primitives.len()).sum();
        let mut snapshot = mesh_bridge::import_polygon_meshes(vec![payload.clone(); count]).unwrap();
        let before = snapshot.clone();
        apply_scene_nodes(&mut snapshot, &document).unwrap();
        if document.nodes.is_empty() { assert_eq!(snapshot, before); before.retire_cold(); snapshot.retire_cold(); continue; }
        before.retire_cold();
        let host = &snapshot.host_snapshot;
        let previews: Vec<_> = host.widgets.iter().filter_map(|widget| match widget { Widget::OutputPreview { id, .. } => Some(id), _ => None }).collect();
        assert_eq!(previews.len(), row["paths"].as_array().unwrap().len());
        for (instance, preview) in previews.into_iter().enumerate() {
            let mut from = host.synapses.iter().find(|wire| &wire.to == preview).unwrap().from.as_str();
            for ancestor in row["paths"][instance].as_array().unwrap().iter().rev() {
                let node = ancestor.as_u64().unwrap() as usize;
                let widget = host.widgets.iter().find(|widget| semio_s_artifact_procedural_generation3d::widget_id(widget) == from).unwrap();
                let Widget::Neuron { neuron_kind, params, preview, .. } = widget else { panic!("scene placement is a neuron") };
                assert_eq!(neuron_kind, "brep.mesh.transform"); assert!(!preview);
                let matrix = params.get("matrix").unwrap().as_dictionary().unwrap();
                assert_eq!(matrix.schema(), Some("list"));
                let expected = oracle(&document.nodes[node]);
                for index in 0..16 {
                    let value = matrix.get(&index.to_string()).unwrap().as_dictionary().unwrap().get("value").unwrap().as_atom().unwrap().as_f64().unwrap();
                    assert!((value - expected.as_slice()[index]).abs() < 1e-12);
                }
                from = host.synapses.iter().find(|wire| wire.to == from && wire.to_port == "mesh").unwrap().from.as_str();
            }
            let part = row["sources"][instance].as_u64().unwrap();
            assert_eq!(from, if part == 0 { "imported-geometry".into() } else { format!("imported-geometry-{part}") });
        }
        snapshot.retire_cold();
    }
    for row in source["refusals"].as_array().unwrap() {
        let mut document = serde_json::json!({ "asset": {"version":"2.0"}, "meshes":[{"primitives":[{"attributes":{}}]}] });
        for (key, value) in row.as_object().unwrap() { if key != "id" { document[key] = value.clone(); } }
        let document: GltfDocument = serde_json::from_value(document).unwrap();
        let mut snapshot = mesh_bridge::import_polygon_meshes(vec![payload.clone()]).unwrap();
        assert!(apply_scene_nodes(&mut snapshot, &document).is_err(), "{}", row["id"]);
        snapshot.retire_cold();
    }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(SURFACE_JSON).expect("the artifact-surface fixture is valid json")
}

fn rows(value: &serde_json::Value, key: &str) -> Vec<serde_json::Value> {
    value[key].as_array().unwrap_or_else(|| panic!("fixture has a `{key}` array")).clone()
}

fn text(row: &serde_json::Value, key: &str) -> String {
    row[key].as_str().unwrap_or_else(|| panic!("fixture row has a `{key}` string, got {row}")).to_string()
}
//#endregion 🧫️Fixture

//#region 📤️ExportRoster
/// ⚖️ LAW: the picker offers EXACTLY the formats this artifact claims it can export — no more (a row
/// the user picks that this artifact does not own could only fail) and no fewer (a claimed format
/// with no way to reach it is the gap this whole lane exists to close).
#[test]
fn export_roster_is_exactly_this_artifacts_export_claim() {
    let expected: Vec<String> = rows(&fixture(), "exportFormats").iter().map(|row| text(row, "id")).collect();
    let declared: Vec<String> = document_io::EXPORT_FORMATS.iter().map(|row| row.id.to_string()).collect();
    assert_eq!(declared, expected, "the export roster drifted from the fixture");
    let mut claimed: Vec<String> = export_stdio_kinds().iter().map(|kind| kind.trim_start_matches("stdio.").to_string()).collect();
    claimed.sort_unstable();
    let mut offered = declared.clone();
    offered.sort_unstable();
    assert_eq!(offered, claimed, "the picker and `export_stdio_kinds` must name the same formats");
}

/// ⚖️ LAW: every row resolves its OWNING artifact's own descriptor, and the download it builds takes
/// its filename, MIME and encoding from there — never from a table restated in this artifact.
#[test]
fn every_export_row_builds_its_download_from_the_owning_artifacts_descriptor() {
    for row in rows(&fixture(), "exportFormats") {
        let id = text(&row, "id");
        let declared = document_io::EXPORT_FORMATS.iter().find(|candidate| candidate.id == id).expect("declared row");
        assert_eq!(declared.kind_id, text(&row, "kindId"), "{id}: representation id");
        assert_eq!(declared.label_en, text(&row, "labelEn"), "{id}: English label");
        assert_eq!(declared.label_de, text(&row, "labelDe"), "{id}: German label");
        let descriptor = document_io::descriptor_of(declared).unwrap_or_else(|error| panic!("{id}: its owning artifact publishes no descriptor ({error})"));
        assert_eq!(descriptor.extensions.first().map(String::as_str), Some(text(&row, "extension").as_str()), "{id}: extension");
        assert_eq!(descriptor.mimes.first().map(String::as_str), Some(text(&row, "mime").as_str()), "{id}: MIME");
        assert_eq!(descriptor.is_binary, row["binary"].as_bool().expect("binary flag"), "{id}: binary claim");
        let envelope = document_io::document_export_envelope(declared, b"probe".to_vec()).unwrap_or_else(|error| panic!("{id}: envelope ({error})"));
        assert_eq!(envelope.filename, text(&row, "filename"), "{id}: download filename");
        assert_eq!(envelope.mime_type, text(&row, "mime"), "{id}: download MIME");
        if descriptor.is_binary {
            assert_eq!(envelope.encoding.as_deref(), Some("base64"), "{id}: a binary format's download is base64");
            assert_eq!(mesh_bridge::base64_decode(&envelope.data).expect("base64 decodes"), b"probe".to_vec(), "{id}: base64 payload is the exact bytes");
        } else {
            assert_eq!(envelope.encoding, None, "{id}: a textual format's download carries no encoding");
            assert_eq!(envelope.data, "probe", "{id}: textual payload is the exact bytes");
        }
    }
}

/// ⚖️ LAW: a textual row handed bytes that are not UTF-8 is a TYPED error, never a lossy download —
/// the user would otherwise save a file quietly different from what the codec produced.
#[test]
fn a_textual_export_refuses_bytes_that_are_not_utf8() {
    let row = document_io::EXPORT_FORMATS.iter().find(|row| row.id == "obj").expect("obj is a textual row");
    let error = document_io::document_export_envelope(row, vec![0xff, 0xfe]).expect_err("invalid utf-8 must not become a download");
    assert!(error.to_string().contains("not UTF-8"), "the error names the cause, got {error}");
}

/// ⚖️ LAW: a format the artifact does not claim is refused BY NAME, and the message lists what it
/// does claim — the difference between a dead menu row and an answerable one.
#[test]
fn an_unclaimed_export_format_is_refused_by_name() {
    let error = document_io::export_geometry(&unit_cube_semio_mesh(), "step").expect_err("`step` is not one of this artifact's formats");
    let message = error.to_string();
    assert!(message.contains("step"), "the refusal names the format, got {message}");
    assert!(message.contains("stl"), "the refusal lists what IS offered, got {message}");
}

/// ⚖️ LAW — THIRD-PARTY ORACLE: every GEOMETRY row really writes the committed unit cube. The bytes
/// go out through `export_mesh_bytes`, come back through that format's own import leaf, and the
/// recovered triangles are handed to `parry3d` — an independent library sharing no code with this
/// repository — which has to agree on volume 1.0 and on `[0,0,0]..[1,1,1]` bounds. A codec that
/// dropped a face or reversed a winding moves that number; a triangle count alone does not.
#[test]
fn every_geometry_export_format_moves_the_committed_cube_past_the_oracle() {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts as import_leaves;
    let mesh = unit_cube_semio_mesh();
    for row in rows(&fixture(), "exportFormats").into_iter().filter(|row| row["geometry"].as_bool().unwrap_or(false)) {
        let id = text(&row, "id");
        let bytes = document_io::export_mesh_bytes(&mesh, &id).unwrap_or_else(|error| panic!("{id}: the unit cube exports ({error})"));
        assert!(!bytes.is_empty(), "{id}: an export must write bytes");
        let back = match id.as_str() {
            "stl" => import_leaves::stl::v_ascii::any::mesh_from_bytes(&bytes),
            "obj" => import_leaves::obj::v3_0::any::mesh_from_bytes(&bytes),
            "ply" => import_leaves::ply::v1_0::any::mesh_from_bytes(&bytes),
            "gltf" => import_leaves::gltf::v2_0::any::mesh_from_bytes(&bytes),
            "las" => semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::decode_mesh(&bytes, semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::SemioMeshFormat::Las).map_err(|error| mesh_bridge::io_error(error.to_string())),
            "dwg" => import_leaves::dwg::v_ac1018::any::mesh_from_bytes(&bytes),
            other => panic!("{other}: the fixture calls this a geometry row but no import leaf is named"),
        }
        .unwrap_or_else(|error| panic!("{id}: our own bytes re-import ({error})"));
        // ☁️ LAS is a point cloud: it keeps every position and no face at all, so the oracle's
        // trimesh verdict does not apply — its own round-trip case already asserts the positions.
        if id == "las" {
            assert_eq!(project(&back).vertex_count, 8, "las: the cube's 8 positions survive");
            continue;
        }
        assert_oracle_agrees_on_unit_cube(&id, &back);
    }
}

/// ⚖️ LAW: `txt` is the one FULL-FIDELITY target and it touches no geometry — it is the document's
/// own text, so it must work with no flow evaluator at all.
#[test]
fn the_text_export_is_the_documents_own_text_and_needs_no_evaluator() {
    let document = semio_s_artifact_procedural_generation3d::Generation3dSnapshot::default();
    let bytes = document_io::export_document_bytes(&document).expect("txt exports without an evaluator");
    retire_document(document);
    let written = String::from_utf8(bytes).expect("txt is utf-8");
    assert!(!written.is_empty(), "the document's text is not empty");
    assert!(document_io::export_mesh_bytes(&unit_cube_semio_mesh(), "txt").is_err(), "txt has no mesh-only half and says so");
}
//#endregion 📤️ExportRoster

//#region 📥️ImportRoster
/// ⚖️ LAW: the file picker offers EXACTLY the declared import dialects — a format no document can be
/// rebuilt from (las, png) is not declared at all, so there is nothing to withhold.
#[test]
fn the_import_picker_offers_exactly_the_declared_import_dialects() {
    let fixture = fixture();
    let expected: Vec<String> = rows(&fixture, "importFormats").iter().map(|row| text(row, "id")).collect();
    let declared: Vec<String> = document_io::IMPORT_FORMATS.iter().map(|row| row.id.to_string()).collect();
    assert_eq!(declared, expected, "the import roster drifted from the fixture");
    let mut offered = declared;
    offered.sort_unstable();
    let mut registry: Vec<String> = import_stdio_kinds().iter().map(|kind| kind.trim_start_matches("stdio.").to_string()).collect();
    registry.sort_unstable();
    assert_eq!(offered, registry, "offered must be exactly `import_stdio_kinds`");
}

/// ⚖️ LAW: the `accept` filter the picker is opened with is every importable extension, taken from
/// each format's OWN artifact.
#[test]
fn the_accept_filter_lists_every_importable_extension() {
    let fixture = fixture();
    assert_eq!(document_io::import_accept_filter().expect("accept filter"), text(&fixture, "acceptFilter"));
    for row in rows(&fixture, "importFormats") {
        let id = text(&row, "id");
        let declared = document_io::IMPORT_FORMATS.iter().find(|candidate| candidate.id == id).expect("declared row");
        let descriptor = document_io::descriptor_of(declared).unwrap_or_else(|error| panic!("{id}: descriptor ({error})"));
        assert_eq!(descriptor.extensions.first().map(String::as_str), Some(text(&row, "extension").as_str()), "{id}: extension");
    }
}

/// ⚖️ LAW: a picked file resolves to its row by the owning artifact's own extension claim, case
/// insensitively, and a name no row claims is refused by name rather than becoming an empty
/// document — the exact failure this ticket removed from the leaves themselves.
#[test]
fn a_picked_files_name_resolves_by_the_owning_artifacts_extension() {
    for row in rows(&fixture(), "importFormats") {
        let id = text(&row, "id");
        let extension = text(&row, "extension");
        assert_eq!(document_io::import_row_for_name(&format!("model{extension}")).expect("lower case resolves").id, id);
        assert_eq!(document_io::import_row_for_name(&format!("MODEL{}", extension.to_uppercase())).expect("upper case resolves").id, id, "{id}: extensions match case-insensitively");
    }
    let error = document_io::import_row_for_name("model.step").expect_err("`.step` is not importable here").to_string();
    assert!(error.contains("model.step"), "the refusal names the file, got {error}");
}

/// ⚖️ LAW: the payload shapes a shell can answer with all decode to the same bytes.
#[test]
fn every_declared_payload_shape_decodes_to_its_bytes() {
    for row in rows(&fixture()["dataUrl"], "cases") {
        let id = text(&row, "id");
        let payload = text(&row, "payload");
        let decoded = document_io::import_payload_bytes(&payload).unwrap_or_else(|error| panic!("{id}: decode ({error})"));
        assert_eq!(String::from_utf8(decoded.into_owned()).expect("utf-8"), text(&row, "bytes"), "{id}: decoded bytes");
    }
}

/// ⚖️ LAW: the whole user path works — the bytes an export writes, wrapped exactly as the shell
/// wraps a picked file, come back as a real previewable document carrying those same bytes.
#[test]
fn a_picked_file_becomes_a_previewable_document_carrying_its_own_bytes() {
    let bytes = document_io::export_mesh_bytes(&unit_cube_semio_mesh(), "stl").expect("the cube exports as stl");
    let payload = format!("data:model/stl;base64,{}", mesh_bridge::base64_encode(&bytes));
    let document = document_io::import_document("cube.stl", &payload).expect("a picked stl imports");
    let (neuron_kind, planted) = mesh_bridge::imported_source(&document).expect("the import plants a source note and an import neuron").to_owned();
    let planted = planted.to_string();
    let neuron_kind = neuron_kind.to_string();
    retire_document(document);
    assert_eq!(neuron_kind, "brep.io.importStl", "an stl re-enters the graph through the stl import operator");
    assert_eq!(mesh_bridge::base64_decode(&planted).expect("the planted note is base64"), bytes, "the planted payload is the picked file, byte for byte");
}
//#endregion 📥️ImportRoster

/// 🚪️ Document and prepared geometry inputs have distinct truthful format contracts.
#[test]
fn geometry_exports_require_prepared_geometry_and_document_exports_preserve_text() {
    let fixture = fixture();
    let mesh = unit_cube_semio_mesh();
    for format in fixture["exportInputs"]["preparedGeometry"].as_array().expect("geometry inputs") {
        let export = document_io::export_geometry(&mesh, format.as_str().expect("format")).expect("prepared geometry exports");
        assert!(!export.data.is_empty());
    }
    assert!(document_io::export_geometry(&mesh, fixture["exportInputs"]["rejectDocumentAsGeometry"].as_str().unwrap()).is_err());
    let document = semio_s_artifact_procedural_generation3d::Generation3dSnapshot::default();
    let export = document_io::export_document(&document).expect("document exports without geometry");
    assert_eq!(export.filename, "generation3d.txt");
    assert!(!export.data.is_empty());
    retire_document(document);
}

/// 🧊️ Prepared geometry preserves independent meshes and the oracle's combined volume.
#[test]
fn prepared_geometry_merge_preserves_indices_bounds_and_oracle_volume() {
    let expected = fixture()["mergeGeometry"].clone();
    let first = crate::unit_cube_mesh_data();
    let mut second = first.clone();
    for position in second.positions.chunks_exact_mut(3) {
        for axis in 0..3 { position[axis] += expected["translation"][axis].as_f64().unwrap() as f32; }
    }
    let merged = mesh_bridge::merge_meshes(&[first, second]).expect("compatible surfaces merge");
    assert_eq!(merged.positions.len() / 3, expected["vertexCount"].as_u64().unwrap() as usize);
    assert_eq!(merged.indices.len() / 3, expected["triangleCount"].as_u64().unwrap() as usize);
    let mesh = mesh_bridge::semio_mesh_from_mesh_data(&merged).expect("merged geometry validates");
    let projection = project(&mesh);
    for axis in 0..3 {
        crate::assert_close("merged min", projection.min[axis], expected["min"][axis].as_f64().unwrap());
        crate::assert_close("merged max", projection.max[axis], expected["max"][axis].as_f64().unwrap());
    }
    let (volume, min, max) = crate::oracle_volume_and_bounds(&crate::triangles_of(&mesh));
    crate::assert_close("merged oracle volume", volume, expected["volume"].as_f64().unwrap());
    for axis in 0..3 {
        crate::assert_close("merged oracle min", min[axis], expected["min"][axis].as_f64().unwrap());
        crate::assert_close("merged oracle max", max[axis], expected["max"][axis].as_f64().unwrap());
    }
}

/// 🎨️ General geometry media merges retain compatible channels and scoped asset references.
#[test]
fn prepared_geometry_merge_preserves_surface_metadata_or_refuses_incompatible_domains() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    let meshes = fixture["cases"].as_array().unwrap().iter().map(|row| {
        let value = semio_framework_pack_json::parse(&row["prepared"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap()
    }).collect::<Vec<_>>();
    for row in fixture["mergeSurface"]["cases"].as_array().unwrap() {
        let parts = row["parts"].as_array().unwrap().iter().map(|index| meshes[index.as_u64().unwrap() as usize].clone()).collect::<Vec<_>>();
        let merged = mesh_bridge::merge_meshes(&parts).unwrap();
        assert_eq!(merged.positions.len() / 3, row["vertices"].as_u64().unwrap() as usize);
        assert_eq!(merged.indices.len() / 3, row["faces"].as_u64().unwrap() as usize);
        let materials = row["materials"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>();
        let textures = row["textures"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(merged.materials.keys().map(String::as_str).collect::<Vec<_>>(), materials);
        assert_eq!(merged.textures.keys().map(String::as_str).collect::<Vec<_>>(), textures);
        let material = &merged.attributes["material"];
        assert_eq!(material.values.iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>(), materials);
        assert_eq!(material.indices.as_deref(), Some([0, 1].as_slice()));
        for material in merged.materials.values() { assert!(merged.textures.contains_key(material.get("baseColorTexture").and_then(|value| value.as_str()).unwrap())); }
        assert_eq!(merged.indices, [0, 1, 2, 3, 4, 5]);
        assert_eq!(merged.attributes["uv2"].domain_len(), 6);
        assert_eq!(merged.attributes["owner"].domain_len(), 2);
    }
    for parts in fixture["mergeSurface"]["refusals"].as_array().unwrap() {
        let parts = parts.as_array().unwrap().iter().map(|index| meshes[index.as_u64().unwrap() as usize].clone()).collect::<Vec<_>>();
        let error = mesh_bridge::merge_meshes(&parts).unwrap_err();
        assert!(error.to_string().contains("compatible channel declarations"));
    }
    eprintln!("[DEBUG] generation3d surface merge cases=1 refusals=1 independentThreeFixture=true");
}

/// 📤️ Graph codecs require valid declared documents and finite caller-prepared geometry.
#[test]
fn document_geometry_bridge_validates_both_explicit_inputs() {
    let document = semio_s_artifact_procedural_generation3d::Generation3dSnapshot::default();
    let value = semio_framework_value::ToValue::to_value(&document);
    retire_document(document);
    let mesh = crate::unit_cube_mesh_data();
    assert_eq!(mesh_bridge::generation3d_mesh_from_document(&value, &mesh).expect("explicit geometry"), mesh);
    assert!(mesh_bridge::generation3d_mesh_from_document(&value, &Default::default()).is_err());
    let mut invalid = mesh.clone();
    invalid.positions[0] = f32::NAN;
    assert!(mesh_bridge::generation3d_mesh_from_document(&value, &invalid).is_err());
    assert!(mesh_bridge::generation3d_mesh_from_document(&semio_framework_value::DslValue::null(), &mesh).is_err());
}

/// 🗄️ The graph registry advertises only genuine document conversion routes.
#[test]
fn graph_registry_does_not_claim_geometry_materialization() {
    let exports: Vec<_> = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::io_registry::entries().iter()
        .filter(|entry| entry.writes.artifact_kind != "s.procedural.generation3d")
        .map(|entry| entry.writes.artifact_kind).collect();
    assert_eq!(exports, vec!["s.stdio.txt"]);
}

/// 🎨️ The editable import retains every declared channel and rejects malformed buffers.
#[test]
fn prepared_surface_channels_enter_the_editable_mesh_without_loss() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let owned = semio_framework_pack_json::parse(&row["prepared"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mesh = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&owned)).unwrap();
        let document = mesh_bridge::import_mesh_data(&mesh).unwrap();
        let (operator, data) = mesh_bridge::imported_source(&document).unwrap();
        let operator = operator.to_string();
        let actual: serde_json::Value = serde_json::from_str(data).unwrap();
        retire_document(document);
        assert_eq!(operator, "brep.mesh.construct");
        assert_json_numbers_and_structure(&actual, &row["polygon"]);
    }
    for row in fixture["refusals"].as_array().unwrap() {
        let mut prepared = fixture["cases"][0]["prepared"].clone();
        prepared[row["field"].as_str().unwrap()] = row["value"].clone();
        let owned = semio_framework_pack_json::parse(&prepared.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mesh = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&owned)).unwrap();
        assert!(mesh_bridge::import_mesh_data(&mesh).is_err(), "{}", row["id"]);
    }
    eprintln!("[DEBUG] generation3d editable surface cases=2 refusals=4 serdeOracle=true");
}

fn assert_json_numbers_and_structure(actual: &serde_json::Value, expected: &serde_json::Value) {
    match (actual, expected) {
        (serde_json::Value::Number(actual), serde_json::Value::Number(expected)) => assert_eq!(actual.as_f64(), expected.as_f64()),
        (serde_json::Value::Array(actual), serde_json::Value::Array(expected)) => {
            assert_eq!(actual.len(), expected.len());
            for (actual, expected) in actual.iter().zip(expected) { assert_json_numbers_and_structure(actual, expected); }
        }
        (serde_json::Value::Object(actual), serde_json::Value::Object(expected)) => {
            assert_eq!(actual.len(), expected.len());
            for (key, expected) in expected { assert_json_numbers_and_structure(actual.get(key).unwrap_or_else(|| panic!("missing '{key}'")), expected); }
        }
        _ => assert_eq!(actual, expected),
    }
}

/// 🧩️ Native imports preserve the same concave polygon and channels as the independent triangulator fixture.
#[test]
fn polygon_surface_import_preserves_original_faces_and_corner_or_vertex_channels() {
    use semio_framework_value::FromValue;
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::{obj::v3_0::any as obj, ply::v1_0::any as ply};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧩️polygon-import/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let source = semio_framework_pack_json::parse(&row["source"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let document = match row["format"].as_str().unwrap() {
            "obj" => {
                let source = semio_s_artifact_stdio_obj::ObjSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&source)).unwrap();
                let mut incomplete = source.clone();
                incomplete.faces[0].vertices[0].texcoord = None;
                assert!(obj::deserialize(&incomplete).is_err());
                obj::deserialize(&source).unwrap()
            }
            "ply" => {
                let source = semio_s_artifact_stdio_ply::PlySnapshot::from_value(semio_framework_pack_json::to_dsl_value(&source)).unwrap();
                let mut incomplete = source.clone();
                incomplete.elements[0].count += 1;
                assert!(ply::deserialize(&incomplete).is_err());
                ply::deserialize(&source).unwrap()
            }
            format => panic!("unknown polygon fixture '{format}'"),
        };
        let (kind, payload) = mesh_bridge::imported_source(&document).unwrap();
        let kind = kind.to_owned();
        let payload = payload.to_owned();
        retire_document(document);
        assert_eq!(kind, "brep.mesh.construct");
        let actual: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert_json_numbers_and_structure(&actual, &row["polygon"]);
        assert_eq!(actual["faces"][0].as_array().unwrap().len(), 8);
    }
    eprintln!("[DEBUG] generation3d polygon import cases=2 refusals=2 sharedThreeEarcutFixture=true");
}

/// 🎨️ Surface exports retain normals, UVs, color and owned material assignments.
#[test]
fn prepared_surface_exports_keep_the_supported_channels() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let value = semio_framework_pack_json::parse(&row["prepared"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let prepared = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap();
        let output = mesh_bridge::semio_mesh_from_mesh_data(&prepared).unwrap();
        let primitive = &output.meshes[0].primitives[0];
        assert_eq!(primitive.positions.len(), 3);
        assert_eq!(primitive.indices, [0, 1, 2]);
        if row["prepared"].get("normals").is_some() {
            assert_eq!(primitive.normals.len(), 3);
            assert_eq!(primitive.uvs.len(), 3);
            assert_eq!(primitive.colors.len(), 3);
            assert_eq!(primitive.uvs[1].u, 1.0);
            assert_eq!(primitive.colors[0].r, 1.0);
        } else {
            assert_eq!(primitive.uvs[1].u, 0.5);
            assert_eq!(primitive.material_id.as_deref(), Some("red"));
            assert_eq!(output.materials[0].base_color.r, 1.0);
            assert_eq!(output.materials[0].metallic, 0.25);
            assert_eq!(output.textures[0].bytes, [137, 80, 78, 71]);
        }
        let positions = primitive.positions.iter().map(|point| parry3d::math::Point::new(point.x as f32, point.y as f32, point.z as f32)).collect::<Vec<_>>();
        let triangle = parry3d::shape::Triangle::new(positions[0], positions[1], positions[2]);
        assert_eq!(triangle.area(), 0.5, "independent Parry surface area");
    }
    println!("[DEBUG] generation3d exported surface fixtures=2 independentParry=true");
}

/// 🎨️ Per-face shading expands shared vertices without losing indexed authored samples.
#[test]
fn prepared_face_surface_channels_expand_with_independent_triangle_area() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    for row in fixture["faceChannels"].as_array().unwrap() {
        let source = semio_framework_pack_json::parse(&row["prepared"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mesh = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&source)).unwrap();
        let output = mesh_bridge::semio_mesh_from_mesh_data(&mesh).unwrap();
        let primitive = &output.meshes[0].primitives[0];
        let positions: Vec<_> = primitive.positions.iter().flat_map(|point| [point.x, point.y, point.z]).collect();
        assert_eq!(positions, row["expanded"]["positions"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>());
        assert_eq!(serde_json::json!(primitive.indices), row["expanded"]["indices"]);
        let normals: Vec<_> = primitive.normals.iter().flat_map(|point| [point.x, point.y, point.z]).collect();
        let uvs: Vec<_> = primitive.uvs.iter().flat_map(|point| [point.u, point.v]).collect();
        let colors: Vec<_> = primitive.colors.iter().flat_map(|point| [point.r as f64, point.g as f64, point.b as f64, point.a as f64]).collect();
        for (name, actual) in [("normals", normals), ("uvs", uvs), ("colors", colors)] {
            assert_eq!(actual, row["expanded"][name].as_array().unwrap().iter().flat_map(|tuple| tuple.as_array().unwrap()).map(|value| value.as_f64().unwrap()).collect::<Vec<_>>());
        }
        let area: f32 = primitive.indices.chunks_exact(3).map(|indices| {
            let points = [indices[0], indices[1], indices[2]].map(|index| { let point = &primitive.positions[index as usize]; parry3d::math::Point::new(point.x as f32, point.y as f32, point.z as f32) });
            parry3d::shape::Triangle::new(points[0], points[1], points[2]).area()
        }).sum();
        assert_eq!(area as f64, row["expanded"]["area"].as_f64().unwrap());
    }
    println!("[DEBUG] generation3d face surface samples=6 indexedAliases=true independentParryAndThree=true");
}

/// 🧊️ Quantized authored channels retain normalized values and reject mismatched accessor domains.
#[test]
fn gltf_surface_accessor_domains_match_the_independent_normalized_fixture() {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts as import_leaves;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    for row in fixture["gltfAccessors"]["cases"].as_array().unwrap() {
        let source = row["document"].to_string();
        let mesh = import_leaves::gltf::v2_0::any::mesh_from_bytes(source.as_bytes()).unwrap();
        let primitive = &mesh.meshes[0].primitives[0];
        let normals: Vec<_> = primitive.normals.iter().map(|point| vec![point.x, point.y, point.z]).collect();
        let uvs: Vec<_> = primitive.uvs.iter().map(|point| vec![point.u, point.v]).collect();
        let colors: Vec<_> = primitive.colors.iter().map(|point| vec![point.r as f64, point.g as f64, point.b as f64, point.a as f64]).collect();
        for (name, actual) in [("normals", normals), ("uvs", uvs), ("colors", colors)] {
            let expected = row["expected"][name].as_array().unwrap();
            assert_eq!(actual.len(), expected.len(), "{name} domain count");
            for (actual, expected) in actual.iter().zip(expected) {
                for (actual, expected) in actual.iter().zip(expected.as_array().unwrap()) {
                    assert!((actual - expected.as_f64().unwrap()).abs() < 1e-7, "{name} authored value: {actual} != {expected}");
                }
            }
        }
        assert_eq!(serde_json::json!(primitive.indices), row["expected"]["indices"]);
        for refusal in fixture["gltfAccessors"]["refusals"].as_array().unwrap() {
            let mut malformed = row["document"].clone();
            malformed["accessors"][refusal["accessor"].as_u64().unwrap() as usize][refusal["field"].as_str().unwrap()] = refusal["value"].clone();
            assert!(import_leaves::gltf::v2_0::any::mesh_from_bytes(malformed.to_string().as_bytes()).is_err(), "{}", refusal["id"]);
        }
    }
    println!("[DEBUG] generation3d normalized glTF surface cases=1 refusals=5 independentThree=true");
}

/// 🚧️ Excessive declared domains fail before decoded buffers or graph projections are allocated.
#[test]
fn gltf_import_declared_domains_are_admitted_before_allocation() {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::gltf::v2_0::any;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    let admission:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚧️import-admission/🔣️.json")).unwrap();
    let source=&fixture["gltfSurfaceAuthoring"]["cases"][0]["document"];
    for row in admission["refusals"].as_array().unwrap() {
        let mut invalid=source.clone();
        if let Some(index)=row["accessor"].as_u64() {invalid["accessors"][index as usize]["count"]=row["count"].clone();}
        if let Some(count)=row["indicesCount"].as_u64() {let index=invalid["meshes"][0]["primitives"][0]["indices"].as_u64().unwrap() as usize;invalid["accessors"][index]["count"]=count.into();}
        if let Some(mode)=row["mode"].as_u64() {invalid["meshes"][0]["primitives"][0]["mode"]=mode.into();}
        if let Some(channels)=row["channels"].as_u64() {
            if let Some(count)=row["vertexCount"].as_u64() {for accessor in invalid["accessors"].as_array_mut().unwrap() {if accessor["type"]!="SCALAR" {accessor["count"]=count.into();}}}
            while invalid["meshes"][0]["primitives"][0]["attributes"].as_object().unwrap().len()<channels as usize {
                let mut accessor=invalid["accessors"][0].clone();accessor["type"]=row.get("channelType").cloned().unwrap_or_else(||"VEC3".into());
                let values=invalid["accessors"].as_array_mut().unwrap();let index=values.len();values.push(accessor);
                invalid["meshes"][0]["primitives"][0]["attributes"].as_object_mut().unwrap().insert(format!("_CHANNEL_{index}"),index.into());
            }
        }
        if let Some(count)=row["primitives"].as_u64() {let primitive=invalid["meshes"][0]["primitives"][0].clone();invalid["meshes"][0]["primitives"]=serde_json::Value::Array(vec![primitive;count as usize]);}
        let mut bytes=invalid.to_string().into_bytes();
        if let Some(count)=row["inputBytes"].as_u64() {bytes.resize(count as usize,b' ');}
        let error=any::mesh_from_bytes(&bytes).err().unwrap_or_else(||panic!("{} exceeded admission without refusal",row["id"]));
        assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit,"{}: {error}",row["id"]);
    }
    println!("[DEBUG] glTF declared-domain refusals={} before primitive allocation independentGltfTransformAndThreeCounts=true",admission["refusals"].as_array().unwrap().len());
}

/// 🎨️ Authored UV sets, tangents and material sampling enter the same editable source.
#[test]
fn gltf_authored_surface_metadata_enters_the_editable_constructor() {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::gltf::v2_0::any;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    for row in fixture["gltfSurfaceAuthoring"]["cases"].as_array().unwrap() {
        let document = any::deserialize_bytes(row["document"].to_string().as_bytes()).unwrap();
        let (kind, data) = mesh_bridge::imported_source(&document).unwrap();
        assert_eq!(kind, "brep.mesh.construct");
        let polygon: serde_json::Value = serde_json::from_str(data).unwrap();
        for (name, expected) in row["expected"]["attributes"].as_object().unwrap() {
            assert_json_numbers_and_structure(&polygon["attributes"][name], expected);
        }
        for (name, expected) in row["expected"]["material"].as_object().unwrap() {
            assert_json_numbers_and_structure(&polygon["materials"]["mat-0"][name], expected);
        }
        let expected_image = mesh_bridge::base64_decode(row["document"]["images"][0]["uri"].as_str().unwrap().split_once(',').unwrap().1).unwrap();
        assert_eq!(polygon["textures"]["tex-0"]["bytes"], serde_json::json!(expected_image));
        retire_document(document);
        for refusal in fixture["gltfSurfaceAuthoring"]["refusals"].as_array().unwrap() {
            let mut malformed = row["document"].clone();
            malformed["materials"][0][refusal["materialField"].as_str().unwrap()][refusal["field"].as_str().unwrap()] = refusal["value"].clone();
            let result = any::deserialize_bytes(malformed.to_string().as_bytes());
            match result {
                Ok(document) => { retire_document(document); panic!("{} must refuse", refusal["id"]); },
                Err(_) => {},
            }
        }
    }
    println!("[DEBUG] generation3d authored glTF channels=3 materialRoles=5 refusals=3 independentThreeAndGltfTransform=true");
}

/// 🧩️ Authored palettes and domains survive full and material-split editable glTF imports.
#[test]
fn gltf_authored_attribute_palettes_restore_editable_source_domains() {
    use semio_framework_value::FromValue;
    use semio_s_artifact_stdio_gltf::schema::snapshot::GltfJson;
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::{import::deserializers::artifacts::gltf::v2_0::any as import,export::serializers::artifacts::gltf::v2_0::any as export};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    let metadata:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️authored-attributes/🔣️.json")).unwrap();
    let prepared=&fixture["gltfExport"]["prepared"];
    let value=semio_framework_pack_json::parse(&prepared.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh=semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap();
    let exported=export::serialize_prepared_meshes(&[mesh]).unwrap();
    for row in metadata["cases"].as_array().unwrap() {
        let mut source=exported.clone();
        if row["faces"].as_array().unwrap().len()==1 {
            let channels=source.document.meshes[0].primitives[0].attributes.clone();
            for(_,index)in channels {let accessor=&mut source.document.accessors[index];assert_eq!(accessor.count,6);accessor.byte_offset+=3*accessor.kind.components()*accessor.component_type.byte_size();accessor.count=3;}
            source.document.meshes[0].primitives[0].indices=None;
        }
        source.document.meshes[0].primitives[0].extras=Some(serde_json::from_value(serde_json::json!({"semioAttributes":prepared["attributes"],"semioAttributeStreams":metadata["streams"],"semioSourceVertices":row["vertices"],"semioSourceCorners":row["corners"],"semioSourceFaces":row["faces"]})).unwrap());
        let document=import::deserialize(&source).unwrap();
        let(kind,payload)=mesh_bridge::imported_source(&document).unwrap();assert_eq!(kind,"brep.mesh.construct");
        let polygon:serde_json::Value=serde_json::from_str(payload).unwrap();
        for(name,indices)in row["expected"].as_object().unwrap() {
            for field in ["domain","semantic","interpolation"] {assert_eq!(polygon["attributes"][name][field],prepared["attributes"][name][field],"{} {name} {field}",row["id"]);}
            assert_json_numbers_and_structure(&polygon["attributes"][name]["indices"],indices);
            let values=if name=="material" {serde_json::json!(["mat-0","mat-0"])}else{prepared["attributes"][name]["values"].clone()};
            assert_json_numbers_and_structure(&polygon["attributes"][name]["values"],&values);
        }
        assert_eq!(polygon["attributes"].as_object().unwrap().len(),row["expected"].as_object().unwrap().len());
        assert_json_numbers_and_structure(&polygon["faces"],&row["indices"]);
        let area:f32=polygon["faces"].as_array().unwrap().iter().map(|face|{let points:Vec<_>=face.as_array().unwrap().iter().map(|index|{let point=&polygon["vertices"][index.as_u64().unwrap() as usize];parry3d::math::Point::new(point[0].as_f64().unwrap() as f32,point[1].as_f64().unwrap() as f32,point[2].as_f64().unwrap() as f32)}).collect();parry3d::shape::Triangle::new(points[0],points[1],points[2]).area()}).sum();
        assert_eq!(area,if row["faces"].as_array().unwrap().len()==1 {0.5}else{1.0});
        retire_document(document);
        let mut partial=source.clone();
        let mut attributes=serde_json::Map::new();for name in metadata["fallback"]["authored"].as_array().unwrap() {let name=name.as_str().unwrap();attributes.insert(name.into(),prepared["attributes"][name].clone());}
        partial.document.meshes[0].primitives[0].extras=Some(serde_json::from_value(serde_json::json!({"semioAttributes":attributes,"semioAttributeStreams":{},"semioSourceVertices":row["vertices"],"semioSourceCorners":row["corners"],"semioSourceFaces":row["faces"]})).unwrap());
        let document=import::deserialize(&partial).unwrap();let(_,payload)=mesh_bridge::imported_source(&document).unwrap();let polygon:serde_json::Value=serde_json::from_str(payload).unwrap();
        for name in metadata["fallback"]["expected"].as_array().unwrap() {assert!(polygon["attributes"][name.as_str().unwrap()].is_object(),"fallback {name}");}
        assert_eq!(polygon["attributes"]["normal"]["domain"],"vertex");assert_eq!(polygon["attributes"]["normal"]["values"].as_array().unwrap().len(),row["vertices"].as_array().unwrap().len());
        for value in polygon["attributes"]["normal"]["values"].as_array().unwrap() {assert_json_numbers_and_structure(value,&metadata["fallback"]["normal"]);}
        retire_document(document);
        if row["faces"].as_array().unwrap().len()==1 {for refusal in metadata["refusals"].as_array().unwrap(){
            let mut invalid=source.clone();let Some(GltfJson::Object(extras))=&mut invalid.document.meshes[0].primitives[0].extras else{panic!("authored metadata")};let field=refusal["field"].as_str().unwrap();
            if refusal["remove"].as_bool()==Some(true){extras.retain(|(name,_)|name!=field);}else{extras.iter_mut().find(|(name,_)|name==field).unwrap().1=serde_json::from_value(refusal["value"].clone()).unwrap();}
            match import::deserialize(&invalid){Err(_)=>{},Ok(document)=>{retire_document(document);panic!("{} must refuse malformed source provenance",refusal["id"]);}}
        }}
    }
    println!("[DEBUG] glTF authored import domains=vertex,face,corner channels=10 cases=2 refusals={} fallbackNormals=true independentParryArea=true",metadata["refusals"].as_array().unwrap().len());
}

/// 🗣️ The format warning codes agree with the same neutral surface fixture as the portable twin.
#[test]
fn prepared_surface_format_diagnostics_are_fixture_driven() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    for row in fixture["diagnostics"].as_array().unwrap() {
        let source = &fixture["cases"][row["case"].as_u64().unwrap() as usize]["prepared"];
        let value = semio_framework_pack_json::parse(&source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let prepared = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap();
        let actual = document_io::format_diagnostics(&[prepared], row["format"].as_str().unwrap());
        assert_eq!(actual.iter().map(|diagnostic| diagnostic.code).collect::<Vec<_>>(), row["codes"].as_array().unwrap().iter().map(|code| code.as_str().unwrap()).collect::<Vec<_>>());
    }
    for row in fixture["diagnosticDetails"].as_array().unwrap() {
        let meshes: Vec<_> = row["parts"].as_array().unwrap().iter().map(|part| {
            let mut source = fixture["cases"][part.as_u64().unwrap() as usize]["prepared"].clone();
            for field in ["positions", "colors"] { if let Some(value) = row.get(field) { source[field] = value.clone(); } }
            if let Some(attributes) = row["extraAttributes"].as_object() { for (name, value) in attributes { source["attributes"][name] = value.clone(); } }
            if let Some(fields) = row["extraMaterial"].as_object() { for (name, value) in fields { source["materials"]["red"][name] = value.clone(); } }
            let value = semio_framework_pack_json::parse(&source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap()
        }).collect();
        let actual = document_io::format_diagnostics(&meshes, row["format"].as_str().unwrap());
        assert_eq!(actual.iter().map(|diagnostic| diagnostic.code).collect::<Vec<_>>(), row["codes"].as_array().unwrap().iter().map(|code| code.as_str().unwrap()).collect::<Vec<_>>(), "{}", row["id"]);
    }
    println!("[DEBUG] generation3d surface format diagnostics=12 independentSerde=true");
}

/// 🎨️ Partial authored materials use the owning PBR defaults instead of refusing valid surfaces.
#[test]
fn prepared_surface_export_preserves_partial_materials_with_owning_defaults() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    for row in fixture["materialDefaults"].as_array().unwrap() {
        let mut source = fixture["cases"][1]["prepared"].clone();
        source["materials"]["red"] = row["source"].clone();
        let value = semio_framework_pack_json::parse(&source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mesh = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap();
        let output = mesh_bridge::semio_mesh_from_mesh_data(&mesh).unwrap();
        let material = &output.materials[0];
        for (index, value) in [material.base_color.r, material.base_color.g, material.base_color.b, material.base_color.a].into_iter().enumerate() { assert!((value as f64-row["expected"]["baseColor"][index].as_f64().unwrap()).abs()<1e-7); }
        assert!((material.metallic as f64-row["expected"]["metallic"].as_f64().unwrap()).abs()<1e-7);
        assert!((material.roughness as f64-row["expected"]["roughness"].as_f64().unwrap()).abs()<1e-7);
    }
    println!("[DEBUG] generation3d material defaults=2 independentGltfTransform=true");
}

/// 🔌️ A hidden source exports through its addressed output without gathering another preview.
#[test]
fn prepared_surface_export_scope_uses_the_existing_graph_connections() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    let mut host = fixture["exportScope"]["host"].clone();
    host["schema"] = "flow.host_snapshot".into();
    host["camera"] = serde_json::json!({"x":0,"y":0,"zoom":1});
    host["layout"] = serde_json::json!({});
    let value = semio_framework_pack_json::parse(&host.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let host = semio_framework_artifact_flow_flow::FlowHostSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap();
    for row in fixture["exportScope"]["cases"].as_array().unwrap() {
        let actual = mesh_bridge::export_source_channels(&host, row["request"]["widgetId"].as_str()).unwrap();
        let expected = row["channels"].as_array().unwrap().iter().map(|channel| (channel[0].as_str().unwrap().to_string(), channel[1].as_str().map(str::to_string))).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    for id in fixture["exportScope"]["refusals"].as_array().unwrap() { assert!(mesh_bridge::export_source_channels(&host, id.as_str()).is_err()); }
    host.retire_cold();
    println!("[DEBUG] generation3d export scope fixtures=4 independentSerde=true");
}

/// 📄️ The real registry preserves imported graph documents and retires its rebuilt roots.
#[test]
fn imported_graph_text_registry_round_trip_preserves_graph_and_retires_rebuilt_snapshot() {
    use {semio_framework_artifact_reference::Dialect,semio_framework_plugin::ErasedComposeSource,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::{import, io_registry};
    let contract = fixture()["registryText"].clone();
    assert_eq!(contract["source"], "imported-unit-cube");
    assert_eq!(contract["preserveGraph"], true);
    let document = mesh_bridge::import_mesh_data(&crate::unit_cube_mesh_data()).expect("real imported graph");
    assert_eq!(document.host_snapshot.widgets.len(), 3);
    let expected = semio_framework_value::ToValue::to_value(&document);
    let bytes = document_io::export_document_bytes(&document).expect("real document text");
    let json = String::from_utf8(semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&document).expect("real document json")).expect("json is utf8");
    let independent: serde_json::Value = serde_json::from_str(&json).expect("third-party json reader");
    assert_eq!(independent["hostSnapshot"]["widgets"].as_array().expect("imported graph widgets").len(), 3);
    retire_document(document);
    let entry = io_registry::entries().iter().find(|entry| entry.writes.artifact_kind == contract["target"].as_str().unwrap()).expect("true graph text registry entry");
    for kind in contract["sourceKinds"].as_array().unwrap() {
        let source = match kind.as_str().unwrap() {
            "native" => ErasedComposeSource { dialect: semio_s_artifact_procedural_generation3d::GENERATION3D_DIALECT, payload: IoPayload::Text(String::from_utf8(bytes.clone()).unwrap()) },
            "json-bridge" => ErasedComposeSource { dialect: Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY }, payload: IoPayload::Text(json.clone()) },
            unknown => panic!("unsupported registry fixture source {unknown}"),
        };
        let sources = [source];
        let composed = ::semio_framework_async::poll::resolve_ready((entry.compose)(&sources)).expect("actual registry exports imported graph");
        assert_eq!(composed.dialect, entry.writes);
        let IoPayload::Text(text) = composed.payload else { panic!("text registry must return text") };
        let restored = import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(text.as_bytes()).expect("registry text imports");
        let actual = semio_framework_value::ToValue::to_value(&restored);
        retire_document(restored);
        assert_eq!(actual, expected, "registry preserves the full imported graph for {kind}");
    }
}

/// 🎨️ Prepared glTF export retains rich channels and independent role sampling without preview data.
#[test]
fn gltf_prepared_export_retains_rich_channels_and_role_sampling() {
    use semio_framework_value::FromValue;
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::gltf::v2_0::any;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();
    let row = &fixture["gltfExport"];
    let value = semio_framework_pack_json::parse(&row["prepared"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut mesh = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap();
    mesh.component_references = serde_json::from_value(row["componentReferences"].clone()).unwrap();
    mesh.validate_component_references().unwrap();
    let cutoff_case: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🖌️render/🧫️fixtures/🎨️world3d-inline-surface/🔣️.json")).unwrap();
    let cutoff = cutoff_case["alphaCutoffCase"]["value"].as_f64().unwrap();
    let mut mask = mesh.clone();
    let semio_framework_value::DslValue::Object(fields) = mask.materials.get_mut("paint").unwrap() else { panic!("owned material fields") };
    fields.iter_mut().find(|(name, _)| name == "alphaCutoff").unwrap().1 = semio_framework_value::ToValue::to_value(&cutoff);
    let mask = semio_framework_plugin::MeshData::from_value(semio_framework_value::ToValue::to_value(&mask)).unwrap();
    let masked = any::serialize_prepared_meshes(std::slice::from_ref(&mask)).unwrap();
    assert_eq!(masked.document.materials[0].alpha_cutoff as f64, cutoff);
    let output = any::serialize_prepared_meshes(std::slice::from_ref(&mesh)).unwrap();
    let mut projection_callback = |_| true;
    let mut projection_control = semio_framework_value::NativeEncodeControl::new(64_000_000, &mut projection_callback);
    let controlled_json = semio_framework_pack_json::to_json_string_controlled(&output.document, &mut projection_control).expect("rich glTF physical JSON has controlled owners");
    let independent_controlled: serde_json::Value = serde_json::from_str(&controlled_json).unwrap();
    assert_eq!(independent_controlled["meshes"][0]["primitives"][0]["extras"]["semioAttributes"]["label"], row["prepared"]["attributes"]["label"]);
    let primitive = &output.document.meshes[0].primitives[0];
    for (semantic, expected) in row["expanded"].as_object().unwrap() {
        let index = primitive.attributes.iter().find(|(name, _)| name == semantic).unwrap_or_else(|| panic!("missing authored glTF channel {semantic}" )).1;
        let accessor = &output.document.accessors[index];
        let view = &output.document.buffer_views[accessor.buffer_view.unwrap()];
        let values: Vec<f64> = output.buffers[view.buffer][view.byte_offset + accessor.byte_offset..view.byte_offset + view.byte_length].chunks_exact(4).map(|bytes| f32::from_le_bytes(bytes.try_into().unwrap()) as f64).collect();
        assert_eq!(values, expected.as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>(), "{semantic}");
        assert_eq!(accessor.count, 6);
    }
    let material = &output.document.materials[primitive.material.unwrap()];
    let material_metadata = serde_json::to_value(material.extras.as_ref().unwrap()).unwrap();
    assert_eq!(material_metadata["semioMaterialId"].as_str(), row["materialIdentity"]["id"].as_str());
    assert_eq!(material_metadata["semioMaterialPart"].as_f64(), row["materialIdentity"]["part"].as_f64());
    let duplicate_output = any::serialize_prepared_meshes(&[mesh.clone(), mesh.clone()]).unwrap();
    for (part, expected) in row["materialIdentity"]["duplicateParts"].as_array().unwrap().iter().enumerate() {
        let metadata = serde_json::to_value(duplicate_output.document.materials[part].extras.as_ref().unwrap()).unwrap();
        assert_eq!(metadata["semioMaterialId"].as_str(), row["materialIdentity"]["id"].as_str());
        assert_eq!(metadata["semioMaterialPart"].as_f64(), expected.as_f64());
    }

    assert_eq!(material.emissive_factor, [0.125, 0.25, 0.5]);
    assert_eq!(material.alpha_mode, semio_s_artifact_stdio_gltf::schema::snapshot::GltfAlphaMode::Mask);
    assert_eq!(material.alpha_cutoff, 0.25);
    assert!(material.double_sided);
    assert_eq!(material.normal_texture.as_ref().unwrap().scale, 0.5);
    assert_eq!(material.occlusion_texture.as_ref().unwrap().strength, 0.25);
    let pbr = material.pbr_metallic_roughness.as_ref().unwrap();
    for (role, index, set) in [
        ("baseColorTexture", pbr.base_color_texture.as_ref().unwrap().index, pbr.base_color_texture.as_ref().unwrap().tex_coord),
        ("metallicRoughnessTexture", pbr.metallic_roughness_texture.as_ref().unwrap().index, pbr.metallic_roughness_texture.as_ref().unwrap().tex_coord),
        ("normalTexture", material.normal_texture.as_ref().unwrap().index, material.normal_texture.as_ref().unwrap().tex_coord),
        ("occlusionTexture", material.occlusion_texture.as_ref().unwrap().index, material.occlusion_texture.as_ref().unwrap().tex_coord),
        ("emissiveTexture", material.emissive_texture.as_ref().unwrap().index, material.emissive_texture.as_ref().unwrap().tex_coord),
    ] {
        assert_eq!(set, row["prepared"]["materials"]["paint"]["textureCoordinates"][role].as_u64().unwrap());
        let texture = &output.document.textures[index];
        let sampler = &output.document.samplers[texture.sampler.unwrap()];
        let expected = &row["prepared"]["materials"]["paint"]["textureSamplers"][role];
        assert_eq!(sampler.wrap_s, expected["wrapS"].as_u64().unwrap());
        assert_eq!(sampler.wrap_t, expected["wrapT"].as_u64().unwrap());
        assert_eq!(sampler.min_filter, expected["minFilter"].as_u64());
        assert_eq!(sampler.mag_filter, expected["magFilter"].as_u64());
        assert_eq!(texture.source, Some(0));
    }
    assert_eq!(output.document.images.len(), 1);
    let extra = serde_json::to_value(primitive.extras.as_ref().unwrap()).unwrap();
    assert_eq!(extra["semioAttributeStreams"], row["attributeStreams"]);
    assert_eq!(extra["semioAttributes"]["label"], row["prepared"]["attributes"]["label"]);
    assert_eq!(extra["semioAttributes"]["selected"]["values"], row["prepared"]["attributes"]["selected"]["values"]);
    for field in ["domain", "semantic", "interpolation"] { assert_eq!(extra["semioAttributes"]["selected"][field], row["prepared"]["attributes"]["selected"][field]); }
    assert_eq!(extra["semioAttributes"]["selected"]["indices"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as u32).collect::<Vec<_>>(), mesh.attributes["selected"].indices.as_ref().unwrap().as_slice());
    assert_eq!(extra["semioSourceVertices"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as u64).collect::<Vec<_>>(), row["prepared"]["indices"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap()).collect::<Vec<_>>());
    assert_eq!(extra["semioSourceFaces"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as u64).collect::<Vec<_>>(), vec![0,1]);
    assert_eq!(extra["semioSourceCorners"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as u64).collect::<Vec<_>>(), vec![0,1,2,3,4,5]);
    assert_eq!(serde_json::to_value(output.document.meshes[0].extras.as_ref().unwrap()).unwrap()["semioComponentReferences"], row["componentReferences"]);
    let component_metadata = serde_json::to_value(output.document.meshes[0].extras.as_ref().unwrap()).unwrap();
    for (extra, field) in [("semioFaceIds", "faceIds"), ("semioEdgeIds", "edgeIds"), ("semioVertexIds", "vertexIds"), ("semioEdgePositions", "edgePositions")] {
        assert_eq!(component_metadata[extra].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>(), row["prepared"][field].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>());
    }
    let mut edge_source = row["prepared"].clone();
    edge_source["attributes"]["edgeLabel"] = row["edgeAttribute"].clone();
    let edge_value = semio_framework_pack_json::parse(&edge_source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let edge_mesh = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&edge_value)).unwrap();
    let edge_output = any::serialize_prepared_meshes(std::slice::from_ref(&edge_mesh)).unwrap();
    let edge_metadata = serde_json::to_value(edge_output.document.meshes[0].primitives[0].extras.as_ref().unwrap()).unwrap();
    assert_eq!(edge_metadata["semioAttributes"]["edgeLabel"]["values"], row["edgeAttribute"]["values"]);
    assert_eq!(edge_metadata["semioAttributes"]["edgeLabel"]["indices"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as u32).collect::<Vec<_>>(), vec![1,0]);
    let mut edge_uv_source = row["prepared"].clone();
    edge_uv_source["attributes"]["edgeUv"] = row["edgeUvAttribute"].clone();
    let edge_uv_value = semio_framework_pack_json::parse(&edge_uv_source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let edge_uv_mesh = semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&edge_uv_value)).unwrap();
    let edge_uv_output = any::serialize_prepared_meshes(std::slice::from_ref(&edge_uv_mesh)).expect("unused Edge UV palette does not replace surface UV sampling");
    let edge_uv_metadata = serde_json::to_value(edge_uv_output.document.meshes[0].primitives[0].extras.as_ref().unwrap()).unwrap();
    assert_eq!(edge_uv_metadata["semioAttributes"]["edgeUv"]["values"], row["edgeUvAttribute"]["values"]);
    assert!(edge_uv_metadata["semioAttributeStreams"].get("edgeUv").is_none());
    let mut seam_only = row["prepared"].clone();
    seam_only["attributes"].as_object_mut().unwrap().retain(|name, _| row["seamOnlyAttributes"].as_array().unwrap().iter().any(|value| value.as_str() == Some(name)));
    let parse = |value: &serde_json::Value| {
        let value = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        semio_framework_plugin::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&value)).unwrap()
    };
    let seam_only = any::serialize_prepared_meshes(&[parse(&seam_only)]).unwrap();
    let primitive_seam = &seam_only.document.meshes[0].primitives[0];
    let position = primitive_seam.attributes.iter().find(|(name, _)| name == "POSITION").unwrap().1;
    assert_eq!(seam_only.document.accessors[position].count, 6, "authored tangent alone preserves corner seams");
    for refusal in row["refusals"].as_array().unwrap() {
        let mut prepared = row["prepared"].clone();
        let path = refusal["path"].as_array().unwrap();
        let mut target = &mut prepared;
        for key in &path[..path.len()-1] { target = &mut target[key.as_str().unwrap()]; }
        let key = path.last().unwrap().as_str().unwrap();
        if refusal["remove"].as_bool() == Some(true) { target.as_object_mut().unwrap().remove(key); }
        else { target[key] = refusal["value"].clone(); }
        let error = any::serialize_prepared_meshes(&[parse(&prepared)]).err().expect("invalid prepared export must refuse");
        assert!(error.message.contains(refusal["message"].as_str().unwrap()), "{}", error.message);
    }
    let bytes = any::serialize_prepared_meshes_bytes(std::slice::from_ref(&mesh)).unwrap();
    let wire: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(wire["buffers"][0]["uri"].as_str().unwrap().starts_with("data:application/octet-stream;base64,"));
    assert_eq!(wire["meshes"][0]["primitives"][0]["attributes"]["TEXCOORD_63"].as_u64(), primitive.attributes.iter().find(|(name, _)| name == "TEXCOORD_63").map(|(_, index)| *index as u64));
    let envelope = document_io::export_prepared_geometry(std::slice::from_ref(&mesh), "gltf").unwrap();
    assert_eq!(envelope.data.as_bytes(), bytes.as_slice());
    assert_eq!(envelope.mime_type, "model/gltf+json");
    #[cfg(feature = "component-app-assembly")]
    {
        use semio_s_artifact_procedural_generation3d::{editor::generation3d::commands::export_document as editor, viewer::generation3d::commands::export_document as viewer};
        let snapshot = semio_s_artifact_procedural_generation3d::Generation3dSnapshot::default();
        let history = semio_framework_plugin::HistoryView { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, alternatives: Vec::new(), current_checkpoint_id: None, commands: Vec::new(), command_filter: semio_framework_plugin::app::HistoryCommandFilter::default() };
        let doc = semio_framework_plugin::ArtifactView::new(&snapshot, &history);
        let editor = editor::emit(&editor::ExportDocument { format: "gltf".into(), widget_id: None }, &doc, Some(std::slice::from_ref(&mesh))).unwrap();
        let viewer = viewer::emit(&viewer::ExportDocument { format: "gltf".into() }, &doc, Some(std::slice::from_ref(&mesh))).unwrap();
        assert!(editor.artifact_mutations.is_empty());
        for effects in [&editor.effects, &viewer.effects] {
            assert_eq!(effects.len(), 1);
            match &effects[0] {
                semio_framework_plugin::Effect::DownloadMediaExport { data, filename, mime_type, .. } => { assert_eq!(data.as_bytes(), bytes.as_slice()); assert!(filename.ends_with(".gltf")); assert_eq!(mime_type, &envelope.mime_type); },
                _ => panic!("rich glTF command must publish exactly one download"),
            }
        }
    }
    let mut observed = Vec::new();
    let mut callback = |progress| { observed.push(progress); true };
    let mut control = semio_framework_value::NativeEncodeControl::new(64_000_000, &mut callback);
    assert_eq!(any::serialize_prepared_meshes_bytes_controlled(std::slice::from_ref(&mesh), &mut control).unwrap(), bytes);
    let owned_bytes = control.owned_bytes();
    assert!(owned_bytes > bytes.len());
    drop(control);
    let boundaries = observed.len();
    assert!(boundaries >= 10);
    for cancel_at in 1..=boundaries {
        let mut seen = 0;
        let mut callback = |_| { seen += 1; seen < cancel_at };
        let mut control = semio_framework_value::NativeEncodeControl::new(64_000_000, &mut callback);
        let error = any::serialize_prepared_meshes_bytes_controlled(std::slice::from_ref(&mesh), &mut control).err().expect("controlled refusal");
        assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::Canceled);
    }
    let mut admitted = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(1, &mut admitted);
    assert_eq!(any::serialize_prepared_meshes_bytes_controlled(std::slice::from_ref(&mesh), &mut control).err().expect("controlled refusal").kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
    let mut exact_callback = |_| true;
    let mut exact_control = semio_framework_value::NativeEncodeControl::new(owned_bytes, &mut exact_callback);
    assert_eq!(any::serialize_prepared_meshes_bytes_controlled(std::slice::from_ref(&mesh), &mut exact_control).unwrap(), bytes);
    let mut narrow_callback = |_| true;
    let mut narrow_control = semio_framework_value::NativeEncodeControl::new(owned_bytes - 1, &mut narrow_callback);
    assert_eq!(any::serialize_prepared_meshes_bytes_controlled(std::slice::from_ref(&mesh), &mut narrow_control).unwrap_err().kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
    let mut stress = mesh.clone();
    stress.attributes.get_mut("label").unwrap().values[0] = semio_framework_value::DslValue::String("x".repeat(row["encoding"]["metadataStressLength"].as_u64().unwrap() as usize));
    let stress_bytes = any::serialize_prepared_meshes_bytes(std::slice::from_ref(&stress)).unwrap();
    let independent_stress: serde_json::Value = serde_json::from_slice(&stress_bytes).unwrap();
    assert_eq!(independent_stress["meshes"][0]["primitives"][0]["extras"]["semioAttributes"]["label"]["values"][0].as_str().unwrap().len(), row["encoding"]["metadataStressLength"].as_u64().unwrap() as usize);
    if let Some(directory) = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join("prepared-rich.gltf"), &bytes).unwrap();
        std::fs::write(directory.join("prepared-rich-edge.gltf"), any::serialize_prepared_meshes_bytes(std::slice::from_ref(&edge_uv_mesh)).unwrap()).unwrap();
    }
    println!("[DEBUG] prepared glTF byte/control export: bytes={} cancellationBoundaries={boundaries} ownershipRefusal=true envelope=true", bytes.len());
    println!("[DEBUG] prepared glTF export: vertices=6 uvSets=2 colorSets=2 tangent=true numericCustom=true images=1 roleSamplers=5");
}

#[test]
fn document_io_retained_envelope_resumes_bytes_cancels_and_retires_without_publication() {
    use semio_framework_value::{NativeEncodeControl,retirement::owned_retirement};
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎨️surface/🔣️.json")).unwrap();let continuation=&fixture["documentContinuation"];let law=&continuation["envelope"];let text=law["text"].as_str().unwrap().repeat(law["textRepeats"].as_u64().unwrap()as usize);let binary=(0..law["binaryLength"].as_u64().unwrap()).map(|i|i as u8).collect::<Vec<_>>();let text_row=document_io::EXPORT_FORMATS.iter().find(|r|r.id=="txt").unwrap();let binary_row=document_io::EXPORT_FORMATS.iter().find(|r|r.id=="las").unwrap();
    let mut child=Command::new("bun").args(["-e","await Bun.write(Bun.stdout,Buffer.from(await Bun.stdin.arrayBuffer()).toString('base64'));"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&binary).unwrap();let oracle=child.wait_with_output().unwrap();assert!(oracle.status.success());let encoded=String::from_utf8(oracle.stdout).unwrap();
    let retire=|envelope:document_io::Generation3dDocumentEnvelope|{let mut retirement=owned_retirement(envelope);while !retirement.terminal_is_empty() {if let semio_framework_value::SnapshotRetirementStep::Pending {released_bytes,..}=retirement.close_step(1,law["retirementBytes"].as_u64().unwrap()as usize).unwrap() {assert!(released_bytes<=3);}}};
    for budget in continuation["budgets"].as_array().unwrap() {for (row,bytes,expected) in [(text_row,text.as_bytes().to_vec(),text.as_str()),(binary_row,binary.clone(),encoded.as_str())] {
        let pointer=bytes.as_ptr();let mut envelope=document_io::Generation3dDocumentEnvelope::new(row,bytes);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut accepted);let mut turns=0;let output=loop {let before=(envelope.position(),control.owned_bytes());assert!(envelope.step(0,&mut control).unwrap().is_none());assert_eq!((envelope.position(),control.owned_bytes()),before);turns+=1;assert!(turns<100000);if let Some(output)=envelope.step(budget.as_u64().unwrap()as usize,&mut control).unwrap() {break output;}};assert_eq!(output.data,expected);if row.id=="txt" {assert_eq!(output.data.as_ptr(),pointer);}assert!(envelope.step(1,&mut control).unwrap().is_none());retire(envelope);
    }}
    for stop in [0,1,2,32,87] {let mut envelope=document_io::Generation3dDocumentEnvelope::new(binary_row,binary.clone());let canceled=std::cell::Cell::new(false);let mut callback=|_|!canceled.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);for _ in 0..stop {assert!(envelope.step(1,&mut control).unwrap().is_none());}canceled.set(true);let position=envelope.position();assert!(envelope.step(1,&mut control).is_err());assert_eq!(envelope.position(),position);retire(envelope);}
    for row in law["invalidUtf8"].as_array().unwrap() {let bytes=row.as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();let mut envelope=document_io::Generation3dDocumentEnvelope::new(text_row,bytes);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut accepted);assert!(envelope.step(100,&mut control).is_err());retire(envelope);}
    eprintln!("[DEBUG] retained document envelope budgets1/8/256; independent Buffer base64; zero-copy validated UTF8; cancel and malformed bytes suppress output");
}
