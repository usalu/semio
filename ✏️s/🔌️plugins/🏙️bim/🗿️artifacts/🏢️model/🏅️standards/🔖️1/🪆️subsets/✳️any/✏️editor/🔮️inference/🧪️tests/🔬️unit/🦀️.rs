use super::*;

fn demo() -> ModelSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
}

fn fresh(snapshot: &ModelSnapshot) -> ModelInference {
    <ModelInference as protocol::Inference<ModelSnapshot>>::infer(snapshot).expect("fresh inference")
}

fn edited(snapshot: &ModelSnapshot, mutation: crate::ModelMutation) -> ModelSnapshot {
    crate::mutations::apply_model_mutation(snapshot, &mutation).expect("mutation applies")
}

#[semio_framework_async_macros::async_test]
async fn the_session_agrees_with_a_fresh_inference_of_every_field() {
    let snapshot = demo();
    let mut session = ModelInferenceSession::new();
    assert_eq!(session.refresh(&snapshot), &fresh(&snapshot), "a field of ModelInference is missing from the session's fields! table");
}

#[semio_framework_async_macros::async_test]
async fn the_first_refresh_computes_every_field() {
    let mut session = ModelInferenceSession::new();
    session.refresh(&demo());
    assert_eq!(session.recomputed().len(), <ModelInference as protocol::InferenceSpec<ModelSnapshot>>::fields().len());
}

#[semio_framework_async_macros::async_test]
async fn a_storey_height_edit_re_infers_the_dependent_fields_and_matches_a_fresh_run() {
    let snapshot = demo();
    let mut session = ModelInferenceSession::new();
    session.refresh(&snapshot);
    let taller = edited(&snapshot, crate::ModelMutation::SetStoreyHeight(crate::mutations::set_storey_height::SetStoreyHeight { id: "st-ground".into(), height: 3.5 }));
    let inference = session.refresh(&taller).clone();
    assert!(session.recomputed().contains(&"storey_levels") && session.recomputed().contains(&"wall_layout"));
    assert_eq!(inference, fresh(&taller));
    assert!((inference.wall_layout["w-south"].height - 3.5).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn an_edit_that_no_geometry_reads_recomputes_no_geometry() {
    let snapshot = demo();
    let mut session = ModelInferenceSession::new();
    session.refresh(&snapshot);
    let mut unrelated = snapshot.clone();
    unrelated.project.description = "other".into();
    let inference = session.refresh(&unrelated).clone();
    assert!(!session.recomputed().contains(&"element_solids") && !session.recomputed().contains(&"wall_layout") && !session.recomputed().contains(&"quantities"), "project metadata moves no geometry, recomputed: {:?}", session.recomputed());
    assert_eq!(inference, fresh(&unrelated));
}

#[semio_framework_async_macros::async_test]
async fn an_unchanged_snapshot_is_answered_from_memory() {
    let snapshot = demo();
    let mut session = ModelInferenceSession::new();
    session.refresh(&snapshot);
    session.refresh(&snapshot);
    assert_eq!(session.recomputed().len(), <ModelInference as protocol::InferenceSpec<ModelSnapshot>>::fields().len());
}

#[semio_framework_async_macros::async_test]
async fn sessions_are_per_instance_and_closing_drops_them() {
    let snapshot = demo();
    with_inference(Some(7), &snapshot, |inference| assert!(!inference.storey_levels.is_empty()));
    assert!(!terminal_is_empty(7));
    assert!(terminal_is_empty(8));
    assert_eq!(close(7), PluginCloseStep::Complete);
    assert!(terminal_is_empty(7));
}
