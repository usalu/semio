use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::Axis;

fn flip(ids: &[&str], selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(selected);
    run(&snapshot, |doc, cfg| handle(&FlipWalls { ids: ids.iter().map(|id| id.to_string()).collect() }, doc, cfg, &mut ctx))
}

fn ends(snapshot: &ModelSnapshot, wall: &str) -> (crate::Point2, crate::Point2) {
    match &snapshot.walls[wall].axis {
        Axis::Line { start, end } | Axis::Arc { start, end, .. } => (*start, *end),
    }
}

#[semio_framework_async_macros::async_test]
async fn flipping_two_walls_reverses_both_axes_in_one_emit() {
    let emit = flip(&["w-south", "w-east"], &[]).expect("flips");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::FlipWall(_), ModelMutation::FlipWall(_)]));
    let (before, after) = (demo(), applied(&demo(), &emit));
    for wall in ["w-south", "w-east"] {
        let ((start, end), (flipped_start, flipped_end)) = (ends(&before, wall), ends(&after, wall));
        assert_eq!((flipped_start, flipped_end), (end, start), "{wall} runs the other way");
    }
}

#[semio_framework_async_macros::async_test]
async fn without_ids_the_selected_walls_are_flipped_and_other_selected_kinds_are_skipped() {
    let emit = flip(&[], &["w-south", "st-ground"]).expect("flips the selected wall");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::FlipWall(flip)] if flip.id == "w-south"));
}

#[semio_framework_async_macros::async_test]
async fn no_wall_among_the_targets_is_refused_with_its_own_code() {
    for (ids, selected) in [(&["st-ground"][..], &[][..]), (&[][..], &[][..])] {
        assert_eq!(flip(ids, selected).err().map(|fault| fault.code.0), Some("bim.flip.wall-missing".to_string()));
    }
}
