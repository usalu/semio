use crate::change_case_label_en::ChangeCaseLabelEn;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_plugin_norm_compliance::design_case::{CaseRowDelta, CaseRowPatch};
pub fn diff(payload: &ChangeCaseLabelEn, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { cases: CaseRowDelta::modification(&case.id, CaseRowPatch { label_en: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
}
