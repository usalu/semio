use super::*;
use semio_framework::kernel::Effect;
use semio_framework_plugin::ActionKind;

fn empty_view() -> (Fem3dSnapshot, semio_framework_plugin::HistoryView) {
    (Fem3dSnapshot::default(), semio_framework_plugin::HistoryView::empty())
}

/// 🧬️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
/// see `📓️taxonomy.md`'s forbidden vocabulary), so this now surfaces as a `Effect::LoadDocument`
/// carrying the replacement document's pack bytes, not an `artifact_mutations` entry — `dispatch`'s
/// in-process `VcsArtifactApp` never applies `effects` to its own store (that's the real host's
/// job), so this asserts directly on the `Emit` `import_media`-style rather than through
/// `app.snapshot()`.
#[test]
fn set_active_example_loads_default_fixture_3d() {
    let (snapshot, history) = empty_view();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = NoConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let emit = handle(&SetActiveExample { example_id: crate::examples::demo::ID.into() }, &doc, &cfg).expect("handle");
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert!(!loaded.nodes.is_empty(), "expected the default fixture's nodes");
}

/// 🧬️ `setActiveExample` replaces document content via a `Effect::LoadDocument`, so it MUST be
/// declared as a Mutation, not a View/Shell action — the framework's "View/Shell actions must not
/// emit operations" guard would otherwise reject it.
#[test]
fn set_active_example_is_declared_as_operation_3d() {
    let definition = crate::editor::fem3d::create_fem3d_app();
    let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
    assert!(matches!(action.kind, ActionKind::Mutation), "loading an example emits a document-replace effect, so it is a Mutation");
    assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
}

/// 🪟️ Example replacement preserves every exact window preference.
#[test]
fn set_active_example_does_not_publish_app_or_window_config() {
    let (snapshot, history) = empty_view();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = NoConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let emit = handle(&SetActiveExample { example_id: crate::examples::demo::ID.into() }, &doc, &cfg).expect("handle");
    assert!(emit.config_mutations.is_empty());
    assert!(emit.window_config_mutations.is_empty());
}

#[test]
fn set_active_example_unknown_id_resets_to_empty_document() {
    let (snapshot, history) = empty_view();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = NoConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let emit = handle(&SetActiveExample { example_id: "nonsense".into() }, &doc, &cfg).expect("handle");
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert!(loaded.nodes.is_empty());
}
