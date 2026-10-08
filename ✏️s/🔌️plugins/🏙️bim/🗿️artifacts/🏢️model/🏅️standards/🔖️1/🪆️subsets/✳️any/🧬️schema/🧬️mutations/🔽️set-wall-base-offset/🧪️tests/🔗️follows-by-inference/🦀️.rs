//! 🔗️ `set-wall-base-offset` / `follows-by-inference`: the `wall-layout` inference follows the offset. A wall capped by its storey top is
//! shortened by a raised base, a wall with a free height is lifted whole, and a non-finite offset is refused in code.

use super::SetWallBaseOffset;
use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::{close, decode, layout, refusal};
use crate::{ModelMutation, ModelSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔽️set-wall-base-offset/✅️raises-the-base/📸️snapshot/⬅️before/🔣️.json");

fn offset(id: &str, base_offset: f64) -> ModelSnapshot {
    apply_model_mutation(&decode(BEFORE), &ModelMutation::SetWallBaseOffset(SetWallBaseOffset { id: id.into(), base_offset })).expect("the offset edit applies")
}

#[semio_framework_async_macros::async_test]
async fn a_raised_base_shortens_a_wall_capped_by_its_storey() {
    let (before, after) = (layout(&decode(BEFORE), "w-south"), layout(&offset("w-south", 0.15), "w-south"));
    assert!(close(after.base_z, before.base_z + 0.15) && close(after.top_z, before.top_z), "the storey top stays");
    assert!(close(after.height, before.height - 0.15) && close(after.volume / after.height, before.volume / before.height), "the same footprint under a lower height");
}

#[semio_framework_async_macros::async_test]
async fn a_raised_base_lifts_a_wall_with_a_free_height() {
    let (before, after) = (layout(&decode(BEFORE), "w-east"), layout(&offset("w-east", 0.15), "w-east"));
    assert!(close(after.base_z, before.base_z + 0.15) && close(after.top_z, before.top_z + 0.15) && close(after.height, before.height));
}

#[semio_framework_async_macros::async_test]
async fn a_non_finite_offset_is_refused() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mutation = ModelMutation::SetWallBaseOffset(SetWallBaseOffset { id: "w-south".into(), base_offset: value });
        assert_eq!(refusal(&mutation, &decode(BEFORE)), "mutation.invariant");
    }
}
