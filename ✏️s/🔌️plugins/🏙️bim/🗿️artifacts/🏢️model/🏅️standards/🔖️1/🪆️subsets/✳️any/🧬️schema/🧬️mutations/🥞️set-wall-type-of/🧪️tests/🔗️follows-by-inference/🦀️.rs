//! 🔗️ `set-wall-type-of` / `follows-by-inference`: the `wall-layout` inference follows the type. The thickness is the sum of its layers,
//! so the thickness and the volume change with it while the plan extent and the height stay.

use super::SetWallTypeOf;
use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::{close, decode, layout};
use crate::ModelMutation;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🥞️set-wall-type-of/✅️retypes-the-wall/📸️snapshot/⬅️before/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn a_thinner_type_thins_the_wall_and_its_volume() {
    let before = decode(BEFORE);
    let after = apply_model_mutation(&before, &ModelMutation::SetWallTypeOf(SetWallTypeOf { id: "w-east".into(), wall_type: "wt-150".into() })).expect("the type edit applies");
    let (old, new) = (layout(&before, "w-east"), layout(&after, "w-east"));
    assert!(close(old.thickness, 0.3) && close(new.thickness, 0.15));
    assert!(new.volume < old.volume && close(new.length, old.length) && close(new.height, old.height));
    let other = layout(&after, "w-south");
    assert!(close(other.thickness, 0.3) && close(other.length, 8.0), "walls of the old type keep their thickness and extent");
}
