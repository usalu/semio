use super::*;
use crate::editor::fem3d::modes::edit::windows::model;
use crate::editor::fem3d::unit_tests::context::view;
use semio_framework_plugin::HistoryView;

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_demo_snapshot()
}

/// 🎯️ LAW: focusing re-aims the orbit at the entity and carries the eye offset along, on the
/// window kind the command is addressed to; an unknown entity or no window is refused.
#[test]
fn focus_entity_re_aims_the_addressed_window_orbit() {
    let doc = demo();
    let history = HistoryView::empty();
    let view_doc = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle_window(&FocusEntity { id: "n20_l1".into() }, &view_doc, &cfg, &view(model::FEM3D_WINDOW_MODEL)).expect("focus");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "model-left");
    let initial = crate::viewport::INITIAL;
    let focused = focused_orbit(&initial, [8.0, 0.0, 2.8]);
    assert_eq!(focused.target, [8.0, 0.0, 2.8]);
    assert_eq!(focused.position, [8.0 + initial.position[0] - initial.target[0], 0.0 + initial.position[1] - initial.target[1], 2.8 + initial.position[2] - initial.target[2]], "the eye keeps its offset");
    assert_eq!(focused.zoom, initial.zoom);
    assert_eq!(focused.up, initial.up);
    assert!(handle_window(&FocusEntity { id: "ghost".into() }, &view_doc, &cfg, &view(model::FEM3D_WINDOW_MODEL)).is_err());
    assert!(handle_window(&FocusEntity { id: "n20_l1".into() }, &view_doc, &cfg, &semio_framework_plugin::ViewModel::default()).is_err());
    assert!(handle(&FocusEntity { id: "n20_l1".into() }, &view_doc, &cfg).is_err());
    let results = handle_window(&FocusEntity { id: "sol1".into() }, &view_doc, &cfg, &view(results::FEM3D_WINDOW_RESULTS)).expect("focus on results");
    assert_eq!(results.window_config_mutations[0].window_id(), "results-left");
}
