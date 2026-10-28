use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use super::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn update_grid_replaces_the_baseline_and_inverse_restores_it() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let mutation = LayoutMutation::UpdateGrid(UpdateGrid { baseline_grid: 18.0, baseline_offset: 4.0, snap_to_baseline: false });
    let next = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("grid applies");
    assert_eq!(next.grid.baseline_grid, 18.0);
    assert_eq!(next.grid.baseline_offset, 4.0);
    assert!(!next.grid.snap_to_baseline);
    assert_eq!(next.pages, base.pages);
    let restored = protocol::apply_diff_cold(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse applies");
    assert_eq!(restored.grid, base.grid);
}
