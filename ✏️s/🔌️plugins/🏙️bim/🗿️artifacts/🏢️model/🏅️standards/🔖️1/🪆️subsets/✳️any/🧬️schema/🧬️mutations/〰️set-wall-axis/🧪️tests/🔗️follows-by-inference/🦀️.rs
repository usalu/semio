//! 🔗️ `set-wall-axis` / `follows-by-inference`: the `wall-layout` inference follows the new axis. A longer axis lengthens the layout, a
//! bulge curves it (arc length of `tan(sweep / 4)`), hosted openings keep their record, and a bad bulge is refused in code.

use super::SetWallAxis;
use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::{close, decode, layout, refusal};
use crate::{Axis, ModelMutation, ModelSnapshot, Point2};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/〰️set-wall-axis/✅️stretches-under-an-opening/📸️snapshot/⬅️before/🔣️.json");

fn reshaped(axis: Axis) -> ModelSnapshot {
    apply_model_mutation(&decode(BEFORE), &ModelMutation::SetWallAxis(SetWallAxis { id: "w-south".into(), axis })).expect("the axis edit applies")
}

fn along(end_x: f64) -> Axis {
    Axis::Line { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: end_x, y: 0.0 } }
}

#[semio_framework_async_macros::async_test]
async fn a_longer_axis_lengthens_the_inferred_layout() {
    let (before, after) = (layout(&decode(BEFORE), "w-south"), layout(&reshaped(along(10.0)), "w-south"));
    assert!(close(before.length, 8.0) && close(after.length, 10.0));
    assert!(close(after.height, before.height) && close(after.thickness, before.thickness) && close(after.base_z, before.base_z), "only the plan extent moved");
}

#[semio_framework_async_macros::async_test]
async fn a_bulge_curves_the_wall_and_lengthens_its_layout() {
    let curved = reshaped(Axis::Arc { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 8.0, y: 0.0 }, bulge: 0.5 });
    assert!((layout(&curved, "w-south").length - 9.272952180016122).abs() < 1e-9, "arc length of bulge 0.5 over a chord of 8");
}

#[semio_framework_async_macros::async_test]
async fn the_other_walls_keep_their_extent_and_the_hosted_opening_its_record() {
    let (before, after) = (decode(BEFORE), reshaped(along(10.0)));
    let (old, new) = (layout(&before, "w-east"), layout(&after, "w-east"));
    assert!(close(new.length, old.length) && close(new.height, old.height) && close(new.thickness, old.thickness));
    assert_eq!(after.openings, before.openings, "hosted openings are untouched, their placement is inferred");
}

#[semio_framework_async_macros::async_test]
async fn a_non_finite_bulge_is_refused() {
    for bulge in [f64::NAN, f64::INFINITY] {
        let mutation = ModelMutation::SetWallAxis(SetWallAxis { id: "w-south".into(), axis: Axis::Arc { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 8.0, y: 0.0 }, bulge } });
        assert_eq!(refusal(&mutation, &decode(BEFORE)), "mutation.invariant");
    }
}
