use crate::remove_uplift_case::RemoveUpliftCase;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997UpliftCasesRows};

pub fn diff(payload: &RemoveUpliftCase, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.uplift_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("uplift-case #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { uplift_cases: Some(En1997UpliftCasesRows::removal(&base.uplift_cases, payload.index)), ..Default::default() })
}
