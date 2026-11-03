//! 🔺️ `change-ground-reading-depth` diff: patches `depth` of the addressed investigation reading row.

use crate::change_ground_reading_depth::ChangeGroundReadingDepth;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997InvestigationsPatch, En1997InvestigationsReadingsPatch, En1997InvestigationsReadingsRows, En1997InvestigationsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeGroundReadingDepth, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_depth.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.investigations.iter().find(|row| row.id == payload.investigation_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("investigation {} missing", payload.investigation_id), vec![payload.investigation_id.clone()]);
    };
    let Some(r1) = r0.readings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("ground-investigation-reading {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r1.depth == payload.new_depth {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The investigation reading already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { investigations: Some(En1997InvestigationsRows::modification(&payload.investigation_id, En1997InvestigationsPatch { readings: En1997InvestigationsReadingsRows::modification(&payload.id, En1997InvestigationsReadingsPatch { depth: Some(payload.new_depth), ..Default::default() }), ..Default::default() })), ..Default::default() })
}
