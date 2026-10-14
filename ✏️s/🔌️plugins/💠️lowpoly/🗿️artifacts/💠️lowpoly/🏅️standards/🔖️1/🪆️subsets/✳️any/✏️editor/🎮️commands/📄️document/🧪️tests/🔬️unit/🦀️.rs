extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use semio_s_artifact_lowpoly_lowpoly_commands::document::*;
use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::LowpolyScratch;
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use semio_s_artifact_lowpoly_lowpoly_config::LowpolyConfig;
use crate::context::{app, dispatch};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;
use semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot;

/// 🧬️ `importSnapshotJson`/`replaceSnapshotJson` emit a `Effect::LoadDocument` (outside undo
/// history), not an `artifact_mutations` entry — driven directly through `handle` (not
/// `dispatch`, which routes through `VcsArtifactApp` and never applies `effects` to its own
/// store, that's the real host's job), same pattern as the already-migrated `shooting` sibling.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_json_replaces_the_whole_document() {
    let mesh_json = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_mesh_workspace()["obj-1"].clone();
    let mut replacement=semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::snapshot_from_mesh_json("", "obj-x", "X");
    replacement.objects[0].mesh=Some(semio_s_artifact_lowpoly_lowpoly_model::snapshot::managed_mesh_child_handle("obj-x",&mesh_json));
    replacement.objects[0].mesh_state=Some(mesh_json);
    let json = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::lowpoly_json_encode(&replacement);
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
    let loaded = <LowpolySnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert_eq!(loaded.objects[0].id, "obj-x");
}

#[semio_framework_async_macros::async_test]
async fn replace_snapshot_json_with_invalid_json_is_a_no_op() {
    let mut a = app().await;
    let before = a.snapshot().expect("projection");
    dispatch(&mut a, LowpolyCommand::ReplaceSnapshotJson(replace_snapshot_json::ReplaceSnapshotJson { json: "not json".into() })).await;
    assert_eq!(a.snapshot().expect("projection"), before);
}
