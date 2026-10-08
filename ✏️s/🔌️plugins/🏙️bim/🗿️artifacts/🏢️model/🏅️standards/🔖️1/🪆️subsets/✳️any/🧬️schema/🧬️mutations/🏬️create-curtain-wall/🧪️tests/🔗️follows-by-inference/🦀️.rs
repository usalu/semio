//! 🔗️ `create-curtain-wall` / `follows-by-inference`: the created curtain wall is one structural `curtain_walls/<id>` region, so it
//! gates only inference fields that read curtain walls, and the `wall-layout` of every wall is untouched. Bad parameters are refused in code.

use super::CreateCurtainWall;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::compute_wall_layout;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::{decode, refusal};
use crate::{CurtainWall, ModelMutation, Profile, TopConstraint};
use protocol::{DiffRegions, Mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏬️create-curtain-wall/✅️adds-a-facade/📸️snapshot/⬅️before/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏬️create-curtain-wall/✅️adds-a-facade/🦠️mutation/🔣️.json");

fn facade() -> CreateCurtainWall {
    let ModelMutation::CreateCurtainWall(payload) = semio_framework_pack_json::from_json_str::<ModelMutation>(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes") else { panic!("a create-curtain-wall payload") };
    payload
}

#[semio_framework_async_macros::async_test]
async fn the_curtain_wall_is_one_structural_region_and_leaves_the_wall_layouts_alone() {
    let before = decode(BEFORE);
    let (diff, _) = ModelMutation::CreateCurtainWall(facade()).diff(&before).into_parts();
    assert_eq!(diff.touches().paths, vec!["curtain_walls/cw-1".to_string()]);
    let after = protocol::apply_diff(&diff, &before).expect("the diff applies");
    assert_eq!(after.curtain_walls["cw-1"], facade().curtain_wall);
    assert_eq!(compute_wall_layout(&after), compute_wall_layout(&before));
}

#[semio_framework_async_macros::async_test]
async fn unsound_parameters_are_refused() {
    let before = decode(BEFORE);
    let with = |change: &dyn Fn(&mut CurtainWall)| {
        let mut payload = facade();
        change(&mut payload.curtain_wall);
        ModelMutation::CreateCurtainWall(payload)
    };
    for broken in [
        with(&|wall| wall.v_spacing = f64::NAN),
        with(&|wall| wall.base_offset = f64::INFINITY),
        with(&|wall| wall.top = TopConstraint::Unconnected { height: 0.0 }),
        with(&|wall| wall.mullion = Profile::Rectangle { width: 0.0, depth: 0.15 }),
    ] {
        assert_eq!(refusal(&broken, &before), "mutation.invariant");
    }
    assert_eq!(refusal(&with(&|wall| wall.mullion_material = "m-missing".into()), &before), "mutation.target-missing");
}
