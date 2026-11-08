//! 🔺️ `change-element-area-m2` sparse diff.

use crate::change_element_area_m2::ChangeElementAreaM2;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementAreaM2, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_area_m2.is_finite() || payload.new_area_m2 <= 0.0 || payload.new_area_m2 > 100000.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "area-m2 must lie above 0 to 100000.", Vec::<String>::new());
    }
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if row.area_m2 == payload.new_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element area m2 already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows::modification(&payload.element_id, Din18599ElementsPatch { area_m2: Some(payload.new_area_m2), ..Default::default() })),
        ..Default::default()
    })
}
