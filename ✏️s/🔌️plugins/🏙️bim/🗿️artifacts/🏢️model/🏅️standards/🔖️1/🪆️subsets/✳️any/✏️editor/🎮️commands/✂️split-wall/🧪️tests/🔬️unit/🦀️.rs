use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::Axis;

fn split(ids: &[&str], at: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(selected);
    run(&snapshot, |doc, cfg| handle(&SplitWallAt { ids: ids.iter().map(|id| id.to_string()).collect(), at: at.into() }, doc, cfg, &mut ctx))
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

#[semio_framework_async_macros::async_test]
async fn typed_fractions_read_as_decimals_commas_and_percent() {
    assert_eq!(fraction(""), Some(0.5));
    assert_eq!(fraction("0.25"), Some(0.25));
    assert_eq!(fraction(" 0,75 "), Some(0.75));
    assert_eq!(fraction("25%"), Some(0.25));
    for refused in ["0", "1", "-0.1", "1.5", "100%", "half", "NaN", "inf"] {
        assert_eq!(fraction(refused), None, "{refused}");
    }
}

#[semio_framework_async_macros::async_test]
async fn splitting_the_south_wall_in_the_middle_makes_two_walls_that_meet_at_four_metres() {
    let emit = split(&["w-south"], "", &[]).expect("splits");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SplitWall(split)] if split.id == "w-south" && (split.t - 0.5).abs() < 1e-12));
    let after = applied(&demo(), &emit);
    assert_eq!(after.walls.len(), demo().walls.len() + 1);
    let ModelMutation::SplitWall(split) = &emit.artifact_mutations[0] else { panic!("a split") };
    let (Axis::Line { end: first_end, .. }, Axis::Line { start: second_start, .. }) = (&after.walls["w-south"].axis, &after.walls[&split.new_id].axis) else { panic!("two straight walls") };
    assert_eq!((first_end.x, first_end.y, second_start.x, second_start.y), (4.0, 0.0, 4.0, 0.0));
    assert_eq!(emit.effects.len(), 1, "the result selects both halves");
}

#[semio_framework_async_macros::async_test]
async fn the_selected_walls_split_at_the_typed_fraction_each_with_its_own_new_id() {
    let emit = split(&[], "25%", &["w-south", "w-east", "st-ground"]).expect("splits the two selected walls");
    let ids: Vec<&str> = emit.artifact_mutations.iter().map(|mutation| match mutation { ModelMutation::SplitWall(split) => split.new_id.as_str(), other => panic!("{other:?}") }).collect();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
    assert_eq!(applied(&demo(), &emit).walls.len(), demo().walls.len() + 2);
}

#[semio_framework_async_macros::async_test]
async fn a_bad_fraction_or_no_wall_is_refused_with_its_own_code() {
    assert_eq!(code(split(&["w-south"], "1.2", &[])), Some("bim.split.fraction-invalid".to_string()));
    assert_eq!(code(split(&["st-ground"], "", &[])), Some("bim.split.target-missing".to_string()));
    assert_eq!(code(split(&[], "", &[])), Some("bim.split.target-missing".to_string()));
}
