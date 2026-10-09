use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::Axis;

fn place(ids: &[&str], at: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(selected);
    run(&snapshot, |doc, cfg| handle(&PlaceAt { ids: ids.iter().map(|id| id.to_string()).collect(), at: at.into() }, doc, cfg, &mut ctx))
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

fn line(snapshot: &ModelSnapshot, wall: &str) -> [f64; 4] {
    let Axis::Line { start, end } = &snapshot.walls[wall].axis else { panic!("{wall} is straight") };
    [start.x, start.y, end.x, end.y]
}

#[semio_framework_async_macros::async_test]
async fn the_reference_point_of_the_first_element_lands_on_the_typed_position() {
    let emit = place(&["w-south"], "10, 5", &[]).expect("places");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::PlaceElements(_)]));
    assert_eq!(line(&applied(&demo(), &emit), "w-south"), [10.0, 5.0, 18.0, 5.0]);
}

#[semio_framework_async_macros::async_test]
async fn a_group_keeps_its_shape_and_the_selection_is_the_default_group() {
    let emit = place(&[], "10;5", &["w-south", "w-east"]).expect("places the selection");
    let after = applied(&demo(), &emit);
    assert_eq!(line(&after, "w-south"), [10.0, 5.0, 18.0, 5.0]);
    assert_eq!(line(&after, "w-east"), [18.0, 5.0, 18.0, 11.0], "the east wall moved by the same vector");
    assert_eq!(line(&after, "w-north"), line(&demo(), "w-north"), "an unselected wall stays");
}

#[semio_framework_async_macros::async_test]
async fn placing_where_it_already_is_writes_nothing() {
    assert!(place(&["w-south"], "0 0", &[]).expect("a no-op").artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_bad_position_or_nothing_placeable_is_refused_with_its_own_code() {
    assert_eq!(code(place(&["w-south"], "right here", &[])), Some("bim.place.point-invalid".to_string()));
    assert_eq!(code(place(&["w-south"], "1", &[])), Some("bim.place.point-invalid".to_string()));
    assert_eq!(code(place(&[], "1, 1", &[])), Some("bim.place.target-missing".to_string()));
    assert_eq!(code(place(&["st-ground"], "1, 1", &[])), Some("bim.place.unsupported".to_string()));
}
