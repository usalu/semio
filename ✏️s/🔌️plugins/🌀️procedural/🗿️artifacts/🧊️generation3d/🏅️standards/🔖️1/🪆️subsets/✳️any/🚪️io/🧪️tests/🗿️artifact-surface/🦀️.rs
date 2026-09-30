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
            "las" => semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::decode_mesh(&bytes, semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::SemioMeshFormat::Las).map_err(|error| semio_framework_os_kernel::TextError::new(error, semio_framework_os_kernel::TextSpan::at(1, 1))),
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
        let decoded = document_io::import_payload_bytes(&text(&row, "payload")).unwrap_or_else(|error| panic!("{id}: decode ({error})"));
        assert_eq!(String::from_utf8(decoded).expect("utf-8"), text(&row, "bytes"), "{id}: decoded bytes");
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
    let merged = mesh_bridge::merge_meshes(&[first, second]);
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

/// 📤️ Graph codecs require valid declared documents and finite caller-prepared geometry.
#[test]
fn document_geometry_bridge_validates_both_explicit_inputs() {
    let document = semio_s_artifact_procedural_generation3d::Generation3dSnapshot::default();
    let value = semio_framework_os_kernel::ToValue::to_value(&document);
    retire_document(document);
    let mesh = crate::unit_cube_mesh_data();
    assert_eq!(mesh_bridge::generation3d_mesh_from_document(&value, &mesh).expect("explicit geometry"), mesh);
    assert!(mesh_bridge::generation3d_mesh_from_document(&value, &Default::default()).is_err());
    let mut invalid = mesh.clone();
    invalid.positions[0] = f32::NAN;
    assert!(mesh_bridge::generation3d_mesh_from_document(&value, &invalid).is_err());
    assert!(mesh_bridge::generation3d_mesh_from_document(&semio_framework_os_kernel::DslValue::null(), &mesh).is_err());
}

/// 🗄️ The graph registry advertises only genuine document conversion routes.
#[test]
fn graph_registry_does_not_claim_geometry_materialization() {
    let exports: Vec<_> = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::io_registry::entries().iter()
        .filter(|entry| entry.writes.artifact_kind != "s.procedural.generation3d")
        .map(|entry| entry.writes.artifact_kind).collect();
    assert_eq!(exports, vec!["s.stdio.txt"]);
}

/// 📄️ The real registry preserves imported graph documents and retires its rebuilt roots.
#[test]
fn imported_graph_text_registry_round_trip_preserves_graph_and_retires_rebuilt_snapshot() {
    use semio_framework_plugin::{Dialect, ErasedComposeSource, IoPayload, StandardId, SubsetId};
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::{import, io_registry};
    let contract = fixture()["registryText"].clone();
    assert_eq!(contract["source"], "imported-unit-cube");
    assert_eq!(contract["preserveGraph"], true);
    let document = mesh_bridge::import_mesh_data(&crate::unit_cube_mesh_data()).expect("real imported graph");
    assert_eq!(document.host_snapshot.widgets.len(), 3);
    let expected = semio_framework_os_kernel::ToValue::to_value(&document);
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
        let composed = semio_framework_plugin::resolve_ready((entry.compose)(&sources)).expect("actual registry exports imported graph");
        assert_eq!(composed.dialect, entry.writes);
        let IoPayload::Text(text) = composed.payload else { panic!("text registry must return text") };
        let restored = import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(text.as_bytes()).expect("registry text imports");
        let actual = semio_framework_os_kernel::ToValue::to_value(&restored);
        retire_document(restored);
        assert_eq!(actual, expected, "registry preserves the full imported graph for {kind}");
    }
}
