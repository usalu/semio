//! 🔺️ `change-load-case-design-situation` diff: patches `design_situation` of the addressed footing load case row.

use crate::change_load_case_design_situation::ChangeLoadCaseDesignSituation;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997FootingsLoadCasesPatch, En1997FootingsLoadCasesRows, En1997FootingsPatch, En1997FootingsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeLoadCaseDesignSituation, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.footings.iter().find(|row| row.id == payload.footing_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.footing_id), vec![payload.footing_id.clone()]);
    };
    let Some(r1) = r0.load_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("load-case {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r1.design_situation == payload.new_design_situation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The footing load case already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows::modification(&payload.footing_id, En1997FootingsPatch { load_cases: En1997FootingsLoadCasesRows::modification(&payload.id, En1997FootingsLoadCasesPatch { design_situation: Some(payload.new_design_situation), ..Default::default() }), ..Default::default() })), ..Default::default() })
}
