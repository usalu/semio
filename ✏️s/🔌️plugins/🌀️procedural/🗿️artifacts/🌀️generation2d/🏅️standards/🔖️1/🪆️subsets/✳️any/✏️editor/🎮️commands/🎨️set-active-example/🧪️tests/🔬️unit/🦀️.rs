use super::*;
use semio_framework_plugin::HistoryView;

#[test]
fn set_active_example_loads_the_demo_document() {
    let empty = empty_generation2d_snapshot();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&empty, &history);
    let config = Generation2dConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emitted = emit(&SetActiveExample { example_id: crate::examples::demo::ID.into() }, &doc, &cfg).expect("demo loads");
    assert!(!emitted.artifact_mutations.is_empty(), "demo must change the empty document");
}
