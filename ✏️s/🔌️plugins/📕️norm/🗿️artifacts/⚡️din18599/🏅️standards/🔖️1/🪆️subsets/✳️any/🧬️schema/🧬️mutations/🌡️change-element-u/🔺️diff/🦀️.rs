//! 🔺️ `change-element-u` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_element_u::ChangeElementU;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeElementU, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    let mut elements = base.elements.clone();
    for e in &mut elements {
        if e.id == payload.element_id {
            if (e.u_value_w_m2k - payload.new_u_value_w_m2k).abs() < f64::EPSILON {
                return protocol::MutationOutcome::empty().warn("mutation.no-op", "Element U-value already has this value.");
            }
            e.u_value_w_m2k = payload.new_u_value_w_m2k;
        }
    }
    protocol::MutationOutcome::new(Din18599Diff { elements: Some(crate::diff::Din18599ElementList { values: elements }), ..Default::default() })
}
