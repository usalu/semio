use semio_plugin::ArtifactEditor as _;
use semio_ui_locale::{Locale, Terminology};
use semio_s_artifact_norm_en1999_io::binary::snapshot::{decode_en1999_pack, encode_en1999_pack};
use semio_s_artifact_norm_en1999_io::text::snapshot::{decode_en1999_dsl, decode_en1999_snapshot_json, encode_en1999_dsl};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_plugin_norm_compliance::spec_trace::{problems, Emitted, Realisation};
use semio_s_plugin_norm_compliance::{CheckReport, NormFamily};
use semio_s_plugin_norm_structural_scene::projection_kit::test_support::assert_projection_laws;
use semio_s_plugin_norm_structural_scene::EntityRef;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const FIXTURE_ONLY: &[&str] = &[];

/// 🗂️ Every file below `dir`, depth first and sorted, so failures read in a stable order.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir).expect("the assets directory is readable").map(|entry| entry.expect("directory entry").path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// 📦️ Every file of the artifact's `🖼️assets` taxonomy node.
fn asset_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🖼️assets"), &mut files);
    files
}

/// 🏷️ The example id an asset belongs to: its directory name without the leading emoji.
fn id_of(path: &Path) -> String {
    path.parent().and_then(Path::file_name).expect("an asset sits in an example directory").to_string_lossy().trim_start_matches(|c: char| !c.is_ascii_alphanumeric()).to_owned()
}

/// 🔎️ The assets whose file name ends with `suffix`, paired with their example id.
fn assets_ending(suffix: &str) -> Vec<(String, PathBuf)> {
    asset_files().into_iter().filter(|path| path.file_name().is_some_and(|name| name.to_string_lossy().ends_with(suffix))).map(|path| (id_of(&path), path)).collect()
}

/// 📖️ Every committed `.dsl.semio` asset with its text.
fn dsl_assets() -> Vec<(String, String)> {
    assets_ending("dsl.semio").into_iter().map(|(id, path)| (id, std::fs::read_to_string(&path).expect("the dsl asset is utf-8 text"))).collect()
}

/// 🧩️ The asset text decoded through the artifact's own DSL codec.
fn decode(id: &str, text: &str) -> En1999Snapshot {
    decode_en1999_dsl(text).unwrap_or_else(|error| panic!("{id}: the bundled DSL does not decode: {error}"))
}

/// ⚖️ The settled report of a document, evaluated like the editor does.
fn report_of(document: &En1999Snapshot) -> CheckReport {
    <crate::En1999Family as NormFamily>::evaluate(document)
}

/// 🎯️ Every entity of the scene as a selection, so the handle laws see every handle.
fn select_all(scene: &semio_s_plugin_norm_structural_scene::StructuralScene) -> Vec<EntityRef> {
    scene.entities.iter().map(|entity| entity.entity.clone()).collect()
}

/// 🧭️ Every committed file is a DSL, pack or JSON snapshot, and every DSL is a palette example (or a declared fixture); every palette example has an English and a German title.
#[test]
fn every_asset_is_classified_and_in_the_palette_with_both_titles() {
    let palette = crate::En1999PlayApp::examples();
    let ids: Vec<String> = palette.iter().map(|example| example.id().to_owned()).collect();
    assert_eq!(ids.iter().collect::<BTreeSet<_>>().len(), ids.len(), "palette ids are unique: {ids:?}");
    for example in &palette {
        for (index, locale) in [Locale::En, Locale::De].into_iter().enumerate() {
            assert!(!example.label().resolve(Terminology::Native, locale).trim().is_empty(), "palette example {} has no title in language #{index}", example.id());
        }
    }
    for path in asset_files() {
        let name = path.file_name().expect("file name").to_string_lossy().into_owned();
        assert!(name.ends_with("dsl.semio") || name.ends_with("pack.semio") || name.ends_with(".json"), "{} is neither a DSL, a pack nor a JSON snapshot asset", path.display());
    }
    for (id, text) in dsl_assets() {
        match palette.iter().find(|example| example.id() == id) {
            Some(example) => assert_eq!(example.document_json(), text, "palette example {id} differs from its committed asset"),
            None => assert!(FIXTURE_ONLY.contains(&id.as_str()), "dsl asset {id} is orphaned: not in the palette and not a declared fixture"),
        }
    }
    let assets: BTreeSet<String> = dsl_assets().into_iter().map(|(id, _)| id).collect();
    for id in FIXTURE_ONLY {
        assert!(assets.contains(*id), "declared fixture {id} has no asset");
        assert!(!ids.iter().any(|palette_id| palette_id == id), "declared fixture {id} is a palette example after all");
    }
}

/// 🔁️ Decoding every palette document through the artifact's DSL codec and printing it again yields the identical bytes, and decoding that print yields the same document.
#[test]
fn every_palette_document_round_trips_byte_identically_through_the_dsl_codec() {
    let mut documents: Vec<(String, String)> = crate::En1999PlayApp::examples().iter().map(|example| (example.id().to_owned(), example.document_json())).collect();
    documents.extend(dsl_assets().into_iter().filter(|(id, _)| FIXTURE_ONLY.contains(&id.as_str())));
    assert!(!documents.is_empty());
    for (id, text) in documents {
        let document = decode(&id, &text);
        let printed = encode_en1999_dsl(&document);
        assert_eq!(printed, text, "{id}: print(decode(asset)) is not byte-identical to the asset");
        assert_eq!(decode(&id, &printed), document, "{id}: decode(print(document)) differs from the document");
    }
}

/// 🩹 The remedy law over every palette document (the examples and the full showcase) and every fixture-only asset: each remedy of each check names an existing field of the right type, applies to a document that still decodes, stays inside the field range and lowers the utilisation or the severity of its check.
#[test]
fn every_palette_document_obeys_the_remedy_law() {
    let mut documents: Vec<(String, String)> = crate::En1999PlayApp::examples().iter().map(|example| (example.id().to_owned(), example.document_json())).collect();
    documents.extend(dsl_assets().into_iter().filter(|(id, _)| FIXTURE_ONLY.contains(&id.as_str())));
    let mut findings = Vec::new();
    for (id, text) in documents {
        findings.extend(semio_s_plugin_norm_app_surface::remedy_law_findings(&id, &decode(&id, &text), report_of, |path| crate::field_meta::en1999_field_meta(path).and_then(|spec| spec.range).map(|range| (range.min, range.max))));
    }
    assert!(findings.is_empty(), "{} remedy law findings:\n{}", findings.len(), findings.join("\n"));
}

/// 🎬️ Every bundled document evaluates, projects the live and the full scene, and obeys the scene, diagram and handle laws.
#[test]
fn every_bundled_asset_evaluates_projects_and_obeys_the_projection_laws() {
    let assets = dsl_assets();
    assert!(!assets.is_empty());
    for (id, text) in assets {
        let document = decode(&id, &text);
        let report = report_of(&document);
        assert!(!report.checks.is_empty(), "{id}: the document emits no check");
        let diagrams = semio_s_artifact_norm_en1999_scene::diagrams(&document, &report, None);
        let live = crate::modes::edit::windows::viewport::scene_of(&document, &report);
        assert_projection_laws(&live, &diagrams, &crate::modes::edit::windows::viewport::handles_of(&document, &select_all(&live)));
        let full = crate::modes::edit::windows::viewport::full_scene_of(&document, &report);
        assert_projection_laws(&full, &diagrams, &crate::modes::edit::windows::viewport::handles_of(&document, &select_all(&full)));
    }
}

/// 🧭️ Every bundled document emits checks the spec trace knows, and the trace table has no structural defect: the only open rows are those no bundled document or catalogue case reaches.
#[test]
fn every_bundled_asset_emits_traced_checks_and_the_trace_has_no_structural_defect() {
    let table = &semio_s_artifact_norm_en1999_checks::spec_trace::TABLE;
    let mut emitted = Emitted::default();
    for (id, text) in dsl_assets() {
        let report = report_of(&decode(&id, &text));
        let traced = table.items.iter().filter(|item| matches!(item.realisation, Realisation::Check(fragments) if fragments.iter().any(|fragment| report.checks.iter().any(|check| check.id.contains(fragment))))).count();
        assert!(traced > 0, "{id}: no emitted check id belongs to a traced spec row");
        emitted = emitted.with_checks(report.checks.iter().map(|check| check.id.clone()));
    }
    let structural: Vec<String> = problems(table, &emitted).into_iter().filter(|problem| !problem.contains("no example document or design case emits") && !problem.contains("produces no check")).collect();
    assert!(structural.is_empty(), "{} spec trace has structural defects:\n{}", table.code, structural.join("\n"));
}

/// 🗜️ Every committed pack and JSON snapshot twin decodes to the document of the DSL of the same example, a pack prints back to the identical bytes, and no example carries two packs.
#[test]
fn every_pack_and_json_twin_equals_the_dsl_document_of_its_example() {
    let documents: Vec<(String, En1999Snapshot)> = dsl_assets().into_iter().map(|(id, text)| { let document = decode(&id, &text); (id, document) }).collect();
    let document_of = |id: &str| documents.iter().find(|(known, _)| known == id).map(|(_, document)| document).unwrap_or_else(|| panic!("{id}: a pack or JSON asset without a DSL asset is orphaned"));
    let packs = assets_ending("pack.semio");
    assert_eq!(packs.iter().map(|(id, _)| id).collect::<BTreeSet<_>>().len(), packs.len(), "an example carries more than one pack");
    for (id, path) in packs {
        let bytes = std::fs::read(&path).expect("the pack asset is readable");
        assert_eq!(&decode_en1999_pack(&bytes).unwrap_or_else(|error| panic!("{id}: the pack does not decode: {error}")), document_of(&id), "{id}: the pack is stale against its DSL");
        assert_eq!(encode_en1999_pack(document_of(&id)), bytes, "{id}: the pack is not the canonical print of its document");
    }
    for (id, path) in assets_ending(".json") {
        let text = std::fs::read_to_string(&path).expect("the JSON asset is utf-8 text");
        assert_eq!(&decode_en1999_snapshot_json(&text).unwrap_or_else(|error| panic!("{id}: the JSON twin does not decode: {error}")), document_of(&id), "{id}: the JSON twin is stale against its DSL");
    }
}

/// 🎮️ The demo session is a command script of this artifact with an English and a German title.
#[test]
fn the_demo_session_asset_is_this_artifacts_command_script_with_both_titles() {
    assert!(crate::demo_session::PRIMARY_TEXT.starts_with("semio norm.en1999.cmd v1"));
    for locale in [Locale::En, Locale::De] {
        assert!(!crate::demo_session::label().resolve(Terminology::Native, locale).trim().is_empty());
    }
}
