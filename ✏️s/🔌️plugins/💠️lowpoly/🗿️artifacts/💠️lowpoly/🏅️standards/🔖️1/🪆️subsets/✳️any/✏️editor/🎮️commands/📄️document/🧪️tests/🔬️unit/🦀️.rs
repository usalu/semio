use super::*;
use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::testkit::{app, dispatch};
use crate::editor::lowpoly::LowpolyCommand;
use crate::schema::default_snapshot;

/// 🧬️ `importSnapshotJson`/`replaceSnapshotJson` emit a `Effect::LoadDocument` (outside undo
/// history), not an `artifact_mutations` entry — driven directly through `handle` (not
/// `dispatch`, which routes through `VcsArtifactApp` and never applies `effects` to its own
/// store, that's the real host's job), same pattern as the already-migrated `shooting` sibling.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_json_replaces_the_whole_document() {
    let mesh_json = crate::schema::default_mesh_workspace()["obj-1"].clone();
    let replacement = crate::snapshot_from_mesh_json(&mesh_json, "obj-x", "X");
    let json = serde_json::to_string(&Into::<serde_json::Value>::into(dsl::ToValue::to_value(&replacement))).unwrap();
    let snapshot = default_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = LowpolyConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let mut scratch = LowpolyScratch::default();
    let emit = set_snapshot_json::handle(&set_snapshot_json::ImportSnapshotJson { json }, &doc, &cfg, &mut scratch).expect("handle");
    let semio_framework_plugin::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("importSnapshotJson must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <LowpolySnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert_eq!(loaded.objects[0].id, "obj-x");
}

#[semio_framework_async_macros::async_test]
async fn replace_snapshot_json_with_invalid_json_is_a_no_op() {
    let mut a = app().await;
    let before = a.snapshot().expect("projection");
    dispatch(&mut a, LowpolyCommand::ReplaceSnapshotJson(replace_snapshot_json::ReplaceSnapshotJson { json: "not json".into() })).await;
    assert_eq!(a.snapshot().expect("projection"), before);
}
