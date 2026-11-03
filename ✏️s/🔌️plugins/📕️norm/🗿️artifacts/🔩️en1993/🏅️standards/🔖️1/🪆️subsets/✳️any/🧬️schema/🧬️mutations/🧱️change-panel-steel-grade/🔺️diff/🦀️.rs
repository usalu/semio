//! 🔺️ `change-panel-steel-grade` diff: patches `steel_grade` of the addressed plated_panels block row.

use crate::change_panel_steel_grade::ChangePanelSteelGrade;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlatedPanelsDelta, En1993PlatedPanelsPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangePanelSteelGrade, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.plated_panels.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "plated-panels", Vec::<String>::new());
    };
    if row.steel_grade == payload.new_steel_grade {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The plated_panels block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { plated_panels: En1993PlatedPanelsDelta::modification(payload.id.clone(), En1993PlatedPanelsPatch { steel_grade: Some(payload.new_steel_grade.clone()), ..Default::default() }), ..Default::default() })
}
