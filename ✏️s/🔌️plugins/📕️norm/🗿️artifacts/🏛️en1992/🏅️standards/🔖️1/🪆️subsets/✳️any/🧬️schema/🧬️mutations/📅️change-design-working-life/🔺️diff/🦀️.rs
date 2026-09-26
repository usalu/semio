use crate::diff::En1992Diff;
use crate::mutations::change_design_working_life::ChangeDesignWorkingLife;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeDesignWorkingLife, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if (base.design_working_life_years - payload.new_years).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { design_working_life_years: Some(payload.new_years), ..Default::default() })
}
