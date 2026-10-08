//! 🔗️ `delete-curtain-wall` / `follows-by-inference`: the deletion is one structural `curtain_walls/<id>` region and the `wall-layout`
//! of every wall is untouched.

use crate::standards::v1::subsets::any::schema::inferences::wall_layout::compute_wall_layout;
use crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall::DeleteCurtainWall;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::decode;
use crate::ModelMutation;
use protocol::{DiffRegions, Mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🦖️delete-curtain-wall/✅️removes/📸️snapshot/⬅️before/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_deletion_is_one_structural_region_and_leaves_the_wall_layouts_alone() {
    let before = decode(BEFORE);
    let (diff, _) = ModelMutation::DeleteCurtainWall(DeleteCurtainWall { id: "cw-1".into() }).diff(&before).into_parts();
    assert_eq!(diff.touches().paths, vec!["curtain_walls/cw-1".to_string()]);
    let after = protocol::apply_diff(&diff, &before).expect("the diff applies");
    assert!(after.curtain_walls.is_empty());
    assert_eq!(compute_wall_layout(&after), compute_wall_layout(&before));
}
