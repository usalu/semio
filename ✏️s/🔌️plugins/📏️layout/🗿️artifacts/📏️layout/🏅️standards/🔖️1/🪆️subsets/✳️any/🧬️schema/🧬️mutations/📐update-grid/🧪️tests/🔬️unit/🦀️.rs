use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_grid_replaces_the_baseline_and_inverse_restores_it() {
    let base = crate::standards::v1::subsets::any::schema::default_document();
    let mutation = LayoutMutation::UpdateGrid(UpdateGrid { baseline_grid: 18.0, baseline_offset: 4.0, snap_to_baseline: false });
    let next = mutation.diff(&base).diff().apply(&base).expect("grid applies");
    assert_eq!(next.grid.baseline_grid, 18.0);
    assert_eq!(next.grid.baseline_offset, 4.0);
    assert!(!next.grid.snap_to_baseline);
    assert_eq!(next.pages, base.pages);
    let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff().apply(&next).expect("inverse applies");
    assert_eq!(restored.grid, base.grid);
}
