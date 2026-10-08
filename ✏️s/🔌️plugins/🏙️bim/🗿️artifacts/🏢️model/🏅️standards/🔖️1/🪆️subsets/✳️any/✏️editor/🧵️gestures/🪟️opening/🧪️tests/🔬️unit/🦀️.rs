use super::*;
use crate::editor::bim::gestures::session::Shape;
use crate::editor::bim::gestures::tests::fixture::{room, Rig};

fn created(step: &Step) -> &crate::mutations::create_opening::CreateOpening {
    match step.mutations.as_slice() {
        [ModelMutation::CreateOpening(create)] => create,
        other => panic!("one create-opening expected, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn a_window_stands_on_the_hovered_wall_at_the_foot_of_the_pointer() {
    let mut rig = Rig::plan("window", room());
    rig.mv(2.0, 0.05);
    assert!(rig.shows(Shape::Path), "the cut ghost follows the pointer");
    let step = rig.down(2.0, 0.05);
    let create = created(&step);
    assert_eq!((create.opening.host.as_str(), create.opening.offset, create.opening.flip_facing), ("w-south", 2.0, false));
    assert_eq!(create.opening.kind, OpeningKind::Window { window_type: "win-12".into() });
    let step = Rig::plan("window", room()).down(2.0, -0.05);
    assert!(created(&step).opening.flip_facing, "the pointer on the right of the travel faces the opening to the right");
}

#[semio_framework_async_macros::async_test]
async fn the_opening_is_kept_inside_its_host_and_clear_of_its_neighbours() {
    let mut rig = Rig::plan("window", room());
    let step = rig.down(0.2, 0.0);
    assert_eq!(created(&step).opening.offset, 0.6, "near the corner the centre is pushed in by half the width");
    let overlap = rig.down(0.9, 0.0);
    assert!(overlap.mutations.is_empty() && overlap.refused == Some(REJECTED), "the second window would overlap the first");
    assert!(rig.preview.marks.iter().any(|mark| mark.style == Style::Warning), "the ghost warns");
}

#[semio_framework_async_macros::async_test]
async fn away_from_every_wall_nothing_happens() {
    let mut rig = Rig::plan("window", room());
    let step = rig.down(4.0, 3.0);
    assert!(step.mutations.is_empty() && step.refused.is_none());
    assert!(rig.preview.marks.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_door_a_void_and_a_missing_type() {
    let step = Rig::plan("door", room()).down(3.0, 0.0);
    assert_eq!(created(&step).opening.kind, OpeningKind::Door { door_type: "door-09".into() });
    let step = Rig::plan("opening", room()).down(3.0, 6.0);
    let create = created(&step);
    assert_eq!((create.opening.host.as_str(), create.opening.kind.clone()), ("w-north", OpeningKind::Void { width: VOID_WIDTH, height: VOID_HEIGHT }));
    let mut snapshot = room();
    snapshot.window_types.clear();
    assert_eq!(Rig::plan("window", snapshot).down(3.0, 0.0).refused, Some(TYPE_MISSING));
}

#[semio_framework_async_macros::async_test]
async fn the_host_search_prefers_the_closest_wall_and_fits_offsets() {
    let snapshot = room();
    let hit = nearest_host(&snapshot, "st-ground", [7.9, 0.1], 0.08).expect("the south and east walls meet near the corner");
    assert!(hit.host == "w-south" || hit.host == "w-east");
    assert_eq!(nearest_host(&snapshot, "st-first", [4.0, 0.0], 0.08), None, "another storey has no host here");
    assert_eq!(fitted_offset(2.0, 1.2, 0.1), Some(0.6));
    assert_eq!(fitted_offset(1.0, 1.2, 0.5), None);
}
