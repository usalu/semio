use super::*;
use crate::editor::lowpoly::unit_tests::context::{app, committed_edits, dispatch};
use crate::editor::lowpoly::LowpolyCommand;

fn faces(snapshot: &LowpolySnapshot) -> usize {
    snapshot.objects.iter().filter(|object| !object.mesh_content.is_empty()).map(|object| semio_framework_3d::mesh::HalfedgeMesh::from_json(&object.mesh_content).expect("mesh content parses").face_count()).sum()
}

/// 📤️ Every supported format exports the live document as a text download, and each export
/// re-imports to the same face count — the round trip a user's file actually makes.
#[semio_framework_async_macros::async_test]
async fn export_round_trips_through_every_format() {
    let mut a = app().await;
    let document = a.snapshot().expect("projection");
    let expected = faces(&document);
    assert!(expected > 0, "the boot document carries real faces");
    for format in ["obj", "ply", "stl"] {
        let (filename, mime, text) = export_mesh_text(&document, format).expect("export");
        assert!(filename.ends_with(&format!(".{format}")), "{filename}");
        assert!(!mime.is_empty() && !text.is_empty());
        let imported = import_mesh_bytes(&filename, text.as_bytes()).expect("re-import");
        if format == "stl" {
            // 🔺️ STL is triangles only: every n-gon comes back as its fan, never fewer faces.
            assert!(faces(&imported) >= expected, "stl keeps every face as triangles: {} < {expected}", faces(&imported));
        } else {
            assert_eq!(faces(&imported), expected, "{format} keeps every face");
        }
    }
    let edits = committed_edits(&mut a).await;
    dispatch(&mut a, LowpolyCommand::ExportMesh(export_mesh::ExportMesh { format: "obj".into() })).await;
    assert_eq!(committed_edits(&mut a).await, edits, "an export commits nothing");
    assert_eq!(a.snapshot().expect("projection"), document, "an export leaves the document untouched");
}

/// 📥️ A data-url payload (what the shell hands back for `readAs: dataUrl`) decodes like raw text.
#[semio_framework_async_macros::async_test]
async fn opened_file_bytes_reads_raw_text_and_base64_data_urls() {
    let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
    assert_eq!(opened_file_bytes(obj).expect("raw"), obj.as_bytes());
    let data_url = format!("data:model/obj;base64,{}", base64_codec::base64_standard_encode(obj.as_bytes()));
    assert_eq!(opened_file_bytes(&data_url).expect("data url"), obj.as_bytes());
    assert!(import_mesh_bytes("triangle.obj", obj.as_bytes()).is_ok());
    assert!(import_mesh_bytes("triangle.step", obj.as_bytes()).is_err(), "unknown formats refuse loudly");
}

/// 📥️ `importMeshFile` lands as a whole-document `LoadDocument` effect, never a history edit.
#[semio_framework_async_macros::async_test]
async fn import_mesh_file_emits_a_load_document_effect() {
    let obj = "o Quad\nv 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\n";
    let projection = crate::schema::default_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let cfg = ConfigView { snapshot: &LowpolyConfig::default(), window: None };
    let mut ctx = LowpolyScratch::default();
    let emit = import_mesh_file::handle(&import_mesh_file::ImportMeshFile { name: "quad.obj".into(), payload: obj.into() }, &doc, &cfg, &mut ctx).expect("import");
    assert!(emit.artifact_mutations.is_empty());
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("a LoadDocument effect") else { panic!("expected LoadDocument") };
    let loaded = <LowpolySnapshot as store::ArtifactPack>::decode_pack(pack).expect("loaded document decodes");
    assert_eq!(loaded.objects.len(), 1);
    assert_eq!(loaded.objects[0].name, "Quad");
    assert_eq!(faces(&loaded), 1);
}
