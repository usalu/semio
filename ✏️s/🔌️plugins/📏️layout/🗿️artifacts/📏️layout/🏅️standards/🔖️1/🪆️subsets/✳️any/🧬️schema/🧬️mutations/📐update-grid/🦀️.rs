//! 📐 `update-grid` — replaces the document baseline grid.

#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::GridPatch;
use semio_s_artifact_layout_layout_model::GridSettings;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateGrid {
    pub baseline_grid: f64,
    pub baseline_offset: f64,
    pub snap_to_baseline: bool,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for UpdateGrid where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_grid::UpdateGrid> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "grid", kind: "update-grid", record: "UpdatedGrid" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update baseline grid", "Grundlinienraster aktualisieren")
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for UpdateGrid where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_grid::UpdateGrid> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_grid(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_update_grid(self, base)?
    
    })
}
}

pub fn diff_update_grid(payload: &UpdateGrid, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !payload.baseline_grid.is_finite() || payload.baseline_grid <= 0.0 || !payload.baseline_offset.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Baseline grid must be a positive finite size and the offset must be finite.", Vec::<String>::new());
    }
    let next = GridSettings { baseline_grid: payload.baseline_grid, baseline_offset: payload.baseline_offset, snap_to_baseline: payload.snap_to_baseline };
    if base.grid == next {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Baseline grid is already set to that value.");
    }
    let patch = GridPatch {
        baseline_grid: (!semio_framework_value::equal_f64(next.baseline_grid, base.grid.baseline_grid)).then_some(next.baseline_grid),
        baseline_offset: (!semio_framework_value::equal_f64(next.baseline_offset, base.grid.baseline_offset)).then_some(next.baseline_offset),
        snap_to_baseline: (next.snap_to_baseline != base.grid.snap_to_baseline).then_some(next.snap_to_baseline),
    };
    protocol::MutationOutcome::new(LayoutDiff { grid: Some(patch), ..Default::default() })
}

pub fn inverse_update_grid<Op: From<crate::update_grid::UpdateGrid>>(_payload: &UpdateGrid, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(UpdateGrid { baseline_grid: base.grid.baseline_grid, baseline_offset: base.grid.baseline_offset, snap_to_baseline: base.grid.snap_to_baseline })]

    })())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;


impl PartialEq for UpdateGrid {
    fn eq(&self, other: &Self) -> bool {
        semio_framework_value::equal_f64(self.baseline_grid, other.baseline_grid)
            && semio_framework_value::equal_f64(self.baseline_offset, other.baseline_offset)
            && self.snap_to_baseline == other.snap_to_baseline
    }
}
