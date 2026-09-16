use super::*;
use crate::editor::fem2d::modes::edit::windows::model as model_window;
use crate::editor::fem2d::modes::edit::windows::results as results_window;
use semio_framework_plugin::{HistoryView, ViewModel, ViewWindowInstance};
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("demo document parses")
}

fn addressed(kind: &str) -> ViewModel {
    ViewModel { window_id: Some("w".into()), window_instances: vec![ViewWindowInstance { id: "w".into(), window_kind_id: kind.into() }], ..Default::default() }
}

fn focus(doc: &Fem2dSnapshot, kind: &str, id: &str) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = HistoryView::empty();
    let view = ArtifactView::new(doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    handle_window(&FocusEntity { id: id.into() }, &view, &cfg, &addressed(kind))
}

#[semio_framework_async_macros::async_test]
async fn focus_entity_writes_one_window_config_row_for_every_addressable_kind() {
    let doc = demo();
    for id in ["ridge", "e3", "s1", "l6", "r1"] {
        let emit = focus(&doc, model_window::WINDOW_KIND_ID, id).expect("focus");
        assert!(emit.artifact_mutations.is_empty(), "framing the camera is never a document mutation ({id})");
        assert_eq!(emit.window_config_mutations.len(), 1, "focusEntity writes exactly one window-config row ({id})");
        assert_eq!(emit.window_config_mutations[0].window_kind_id(), model_window::WINDOW_KIND_ID);
        assert_eq!(emit.window_config_mutations[0].window_id(), "w");
    }
}

#[semio_framework_async_macros::async_test]
async fn focus_entity_frames_the_addressed_window_whichever_canvas_it_is() {
    let doc = demo();
    let emit = focus(&doc, results_window::WINDOW_KIND_ID, "ridge").expect("focus in results");
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), results_window::WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn focus_entity_refuses_an_unknown_entity_and_an_unaddressed_window() {
    let doc = demo();
    assert!(focus(&doc, model_window::WINDOW_KIND_ID, "not-an-entity").is_err(), "an unaddressable entity must fault rather than silently move the camera");
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    assert!(handle_window(&FocusEntity { id: "ridge".into() }, &view, &cfg, &ViewModel::default()).is_err());
    assert!(handle(&FocusEntity { id: "ridge".into() }, &view, &cfg).is_err(), "the doc-scoped route always refuses");
}
