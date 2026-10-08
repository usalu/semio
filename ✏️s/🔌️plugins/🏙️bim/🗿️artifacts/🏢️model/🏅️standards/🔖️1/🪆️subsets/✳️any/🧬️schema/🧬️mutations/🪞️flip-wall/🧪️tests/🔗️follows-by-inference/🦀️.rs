//! 🔗️ `flip-wall` / `follows-by-inference`: flipping reverses the stored axis and changes no extent of the `wall-layout` inference;
//! flipping twice restores the axis exactly.

use super::FlipWall;
use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::{close, decode, layout};
use crate::{ModelMutation, ModelSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪞️flip-wall/🌀️reverses-an-arc/📸️snapshot/⬅️before/🔣️.json");

fn flip(snapshot: &ModelSnapshot, id: &str) -> ModelSnapshot {
    apply_model_mutation(snapshot, &ModelMutation::FlipWall(FlipWall { id: id.into() })).expect("the flip applies")
}

#[semio_framework_async_macros::async_test]
async fn a_flipped_wall_keeps_its_extent_and_flips_back() {
    let before = decode(BEFORE);
    for id in ["w-south", "w-arc"] {
        let once = flip(&before, id);
        assert_ne!(once.walls[id].axis, before.walls[id].axis);
        let (old, new) = (layout(&before, id), layout(&once, id));
        assert!(close(old.length, new.length) && close(old.height, new.height) && close(old.thickness, new.thickness), "{id}");
        assert_eq!(flip(&once, id).walls[id].axis, before.walls[id].axis, "{id}: flipping twice is the identity");
    }
}
