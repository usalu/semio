use semio_s_artifact_norm_en1992_diff::En1992Diff;
use crate::change_design_working_life::ChangeDesignWorkingLife;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeDesignWorkingLife, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if (base.design_working_life_years - payload.new_years).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { design_working_life_years: Some(payload.new_years), ..Default::default() })
}
