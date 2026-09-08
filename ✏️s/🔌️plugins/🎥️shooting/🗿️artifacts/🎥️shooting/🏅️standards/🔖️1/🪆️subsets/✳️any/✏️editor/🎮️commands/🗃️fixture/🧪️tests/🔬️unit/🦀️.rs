
use super::*;
use crate::editor::shooting::ShootingCommand;
use crate::editor::shooting::testkit::{dispatch, shooting_app};

/// 🧬️ `reset_snapshot::handle` emits a `Effect::LoadDocument` (outside undo history), not an
/// `artifact_mutations` entry — driven directly through `handle` (not `dispatch`, which routes
/// through `VcsArtifactApp` and never applies `effects` to its own store, that's the real host's
/// job), same as the already-migrated `fem2d` sibling's `commands::example` tests.
#[semio_framework_async_macros::async_test]
async fn reset_snapshot_restores_default_snapshot() {
    use semio_framework_plugin::Effect;
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::AddShot(crate::editor::shooting::commands::shot::add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() })).await;
    assert_eq!(app.snapshot().expect("snapshot").shots.len(), 3);
    let snapshot = app.snapshot().expect("snapshot");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = ShootingConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot };
    let mut ctx = ShootingDispatchCtx::default();
    let emit = reset_snapshot::handle(&reset_snapshot::ResetSnapshot {}, &doc, &cfg, &mut ctx).expect("handle");
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("resetSnapshot must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let restored = <ShootingSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert_eq!(restored.shots.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn load_request_declares_the_import_snapshot_json_import_action() {
    use semio_framework_plugin::Effect;
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::LoadRequest(load_request::LoadRequest {})).await;
    match &result.requested_effects[0] {
        Effect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "importSnapshotJson"),
        other => panic!("expected RequestFileOpen, got {other:?}"),
    }
}
