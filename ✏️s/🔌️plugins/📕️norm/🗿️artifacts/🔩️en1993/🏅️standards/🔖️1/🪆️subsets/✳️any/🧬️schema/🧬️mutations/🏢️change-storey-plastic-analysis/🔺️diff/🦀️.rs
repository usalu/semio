//! 🔺️ `change-storey-plastic-analysis` diff: patches `plastic_analysis` of the addressed storey row.

use crate::change_storey_plastic_analysis::ChangeStoreyPlasticAnalysis;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993StoreyDelta, En1993StoreyPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeStoreyPlasticAnalysis, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.storeys.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new());
    };
    if row.plastic_analysis == payload.new_plastic_analysis {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The storey already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { storeys: En1993StoreyDelta::modification(payload.id.clone(), En1993StoreyPatch { plastic_analysis: Some(payload.new_plastic_analysis), ..Default::default() }), ..Default::default() })
}
