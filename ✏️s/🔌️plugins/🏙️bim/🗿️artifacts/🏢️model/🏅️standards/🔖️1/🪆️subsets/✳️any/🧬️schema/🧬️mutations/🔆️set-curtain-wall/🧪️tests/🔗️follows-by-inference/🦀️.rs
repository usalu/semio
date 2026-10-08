//! 🔗️ `set-curtain-wall` / `follows-by-inference`: the patch names exactly the changed fields (so inference gating sees only those
//! regions), unnamed fields stay, and the `wall-layout` of every wall is untouched. Non-finite values are refused in code.

use super::SetCurtainWall;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::compute_wall_layout;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::{decode, refusal};
use crate::{Axis, ModelMutation, Point2};
use protocol::{DiffRegions, Mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔆️set-curtain-wall/✅️re-grids-the-facade/📸️snapshot/⬅️before/🔣️.json");

fn named() -> SetCurtainWall {
    SetCurtainWall { id: "cw-1".into(), axis: None, base_offset: None, top: None, u_spacing: None, v_spacing: None, mullion: None, panel_material: None, mullion_material: None, name: None }
}

#[semio_framework_async_macros::async_test]
async fn the_diff_touches_exactly_the_named_fields_that_change() {
    let before = decode(BEFORE);
    let mutation = ModelMutation::SetCurtainWall(SetCurtainWall { u_spacing: Some(2.0), v_spacing: Some(1.2), name: Some("South".into()), ..named() });
    let (diff, _) = mutation.diff(&before).into_parts();
    assert_eq!(diff.touches().paths, vec!["curtain_walls/cw-1/u_spacing".to_string(), "curtain_walls/cw-1/name".to_string()], "the unchanged v_spacing is not written");
    let after = protocol::apply_diff(&diff, &before).expect("the diff applies");
    let (old, new) = (&before.curtain_walls["cw-1"], &after.curtain_walls["cw-1"]);
    assert_eq!((new.u_spacing, new.v_spacing, new.name.as_str()), (2.0, old.v_spacing, "South"));
    assert_eq!((&new.axis, &new.top, &new.mullion), (&old.axis, &old.top, &old.mullion));
    assert_eq!(compute_wall_layout(&after), compute_wall_layout(&before));
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_names_only_the_changed_fields_at_their_base_values() {
    let before = decode(BEFORE);
    let mutation = ModelMutation::SetCurtainWall(SetCurtainWall { u_spacing: Some(2.0), v_spacing: Some(1.2), ..named() });
    let undo = mutation.inverse(&before).expect("inverse builds");
    assert_eq!(undo, vec![ModelMutation::SetCurtainWall(SetCurtainWall { u_spacing: Some(1.5), ..named() })]);
}

#[semio_framework_async_macros::async_test]
async fn non_finite_values_are_refused() {
    let before = decode(BEFORE);
    let axis = Axis::Arc { start: Point2 { x: 0.0, y: 8.0 }, end: Point2 { x: 8.0, y: 8.0 }, bulge: f64::NAN };
    for broken in [SetCurtainWall { axis: Some(axis), ..named() }, SetCurtainWall { base_offset: Some(f64::NAN), ..named() }, SetCurtainWall { u_spacing: Some(f64::INFINITY), ..named() }] {
        assert_eq!(refusal(&ModelMutation::SetCurtainWall(broken), &before), "mutation.invariant");
    }
}
