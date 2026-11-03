use crate::change_case_entry_id::ChangeCaseEntryId;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_plugin_norm_compliance::design_case::{CaseRowDelta, CaseRowPatch};
pub fn diff(payload: &ChangeCaseEntryId, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { cases: CaseRowDelta::modification(&case.id, CaseRowPatch { entry_id: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
}
