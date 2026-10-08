//! 🔺️ `change-element-u` sparse diff.

use crate::mutations::change_element_u::ChangeElementU;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementU, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    let Some(element) = base.elements.iter().find(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if (element.u_value_w_m2k - payload.new_u_value_w_m2k).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element U-value already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows { modified: vec![Din18599ElementsPatch { id: payload.element_id.clone(), u_value_w_m2k: Some(payload.new_u_value_w_m2k), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
