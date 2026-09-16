use super::translate_selection::{TranslateSelection, handle};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig};
use store::ArtifactDsl;

type Fem2dSnapshot = crate::Fem2dSnapshot;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("demo document parses")
}

#[test]
fn translate_selection_coalesces_with_gumball_key() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let emit = handle(&TranslateSelection { ids: vec!["n1".into()], dx: 0.5, dy: -0.25, dz: 0.0 }, &view, &cfg).expect("emit");
    assert!(!emit.artifact_mutations.is_empty());
    assert_eq!(emit.coalesce_key.as_deref(), Some("gumball-translate"));
    let [Fem2dMutation::ReplaceNode(replace)] = emit.artifact_mutations.as_slice() else { panic!("one replace") };
    assert_eq!(replace.id, "n1");
}
