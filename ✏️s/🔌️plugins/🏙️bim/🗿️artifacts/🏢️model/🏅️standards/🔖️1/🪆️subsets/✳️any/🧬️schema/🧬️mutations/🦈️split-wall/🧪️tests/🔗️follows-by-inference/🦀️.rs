//! 🔗️ `split-wall` / `follows-by-inference`: the `wall-layout` inference of the two parts adds up to the layout of the whole wall (arc
//! length), both parts share type, top and height, and openings hosted beyond the split point change host with a re-based offset while
//! the others stay.

use super::SplitWall;
use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::{close, decode, layout};
use crate::{ModelMutation, ModelSnapshot};

const WITH_OPENINGS: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🦈️split-wall/🪝️hands-openings-over/📸️snapshot/⬅️before/🔣️.json");
const WITH_ARC: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🦈️split-wall/🌀️splits-an-arc/📸️snapshot/⬅️before/🔣️.json");

fn split(base: &ModelSnapshot, id: &str, t: f64) -> ModelSnapshot {
    apply_model_mutation(base, &ModelMutation::SplitWall(SplitWall { id: id.into(), t, new_id: format!("{id}-2") })).expect("the split applies")
}

#[semio_framework_async_macros::async_test]
async fn the_parts_of_a_straight_wall_add_up_to_the_whole() {
    let before = decode(WITH_OPENINGS);
    let after = split(&before, "w-south", 0.25);
    let (whole, first, second) = (layout(&before, "w-south"), layout(&after, "w-south"), layout(&after, "w-south-2"));
    assert!(close(first.length, 2.0) && close(second.length, 6.0) && close(first.length + second.length, whole.length));
    assert!(close(first.height, whole.height) && close(second.height, whole.height) && close(second.thickness, whole.thickness) && close(second.base_z, whole.base_z));
    assert_eq!(after.walls["w-south-2"].wall_type, after.walls["w-south"].wall_type);
    let (old, new) = (layout(&before, "w-east"), layout(&after, "w-east"));
    assert!(close(old.length, new.length) && close(old.height, new.height), "other walls keep their extent");
}

#[semio_framework_async_macros::async_test]
async fn the_parts_of_an_arc_add_up_to_the_arc_length() {
    let before = decode(WITH_ARC);
    let after = split(&before, "w-arc", 0.5);
    let (whole, first, second) = (layout(&before, "w-arc"), layout(&after, "w-arc"), layout(&after, "w-arc-2"));
    assert!((whole.length - 4.0 * std::f64::consts::PI).abs() < 1e-9, "a semicircle of radius 4 has arc length 4 pi");
    assert!((first.length + second.length - whole.length).abs() < 1e-8, "the quarter circles add up to the semicircle");
    assert!((first.length - second.length).abs() < 1e-8, "split in the middle, both parts are equal");
}

#[semio_framework_async_macros::async_test]
async fn openings_beyond_the_split_point_change_host_with_a_rebased_offset() {
    let before = decode(WITH_OPENINGS);
    let after = split(&before, "w-south", 0.5);
    let placed = |id: &str| (after.openings[id].host.clone(), after.openings[id].offset);
    assert_eq!(placed("o-1"), ("w-south".to_string(), 1.0), "before the split point it stays");
    assert_eq!(placed("o-2"), ("w-south-2".to_string(), 1.0), "offset 5 beyond 4 becomes 1 on the new wall");
    assert_eq!(placed("o-3"), ("w-south-2".to_string(), 3.0));
    assert_eq!(after.openings["o-4"], before.openings["o-4"], "an opening of another wall is untouched");
}
