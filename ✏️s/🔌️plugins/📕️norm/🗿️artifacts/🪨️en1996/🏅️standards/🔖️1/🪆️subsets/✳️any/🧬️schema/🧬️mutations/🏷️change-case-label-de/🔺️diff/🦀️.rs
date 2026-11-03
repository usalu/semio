use crate::change_case_label_de::ChangeCaseLabelDe;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_plugin_norm_compliance::design_case::{CaseRowDelta, CaseRowPatch};
pub fn diff(payload: &ChangeCaseLabelDe, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
    };
    protocol::MutationOutcome::new(En1996Diff { cases: CaseRowDelta::modification(&case.id, CaseRowPatch { label_de: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
}
