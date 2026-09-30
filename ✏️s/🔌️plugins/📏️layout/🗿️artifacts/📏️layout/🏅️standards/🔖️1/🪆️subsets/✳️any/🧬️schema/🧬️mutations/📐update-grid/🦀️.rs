//! 📐 `update-grid` — replaces the document baseline grid.

use crate::mutations::LayoutMutation;
use crate::{GridSettings, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateGrid {
    pub baseline_grid: f64,
    pub baseline_offset: f64,
    pub snap_to_baseline: bool,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdateGrid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "grid", kind: "update-grid", record: "UpdatedGrid" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_grid(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_update_grid(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Update baseline grid", "Grundlinienraster aktualisieren")
    }
}

pub fn diff_update_grid(payload: &UpdateGrid, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !payload.baseline_grid.is_finite() || payload.baseline_grid <= 0.0 || !payload.baseline_offset.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Baseline grid must be a positive finite size and the offset must be finite.", Vec::<String>::new());
    }
    let next = GridSettings { baseline_grid: payload.baseline_grid, baseline_offset: payload.baseline_offset, snap_to_baseline: payload.snap_to_baseline };
    if base.grid == next {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Baseline grid is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { grid: Some(next), ..Default::default() })
}

pub fn inverse_update_grid(_payload: &UpdateGrid, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::UpdateGrid(UpdateGrid { baseline_grid: base.grid.baseline_grid, baseline_offset: base.grid.baseline_offset, snap_to_baseline: base.grid.snap_to_baseline })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
