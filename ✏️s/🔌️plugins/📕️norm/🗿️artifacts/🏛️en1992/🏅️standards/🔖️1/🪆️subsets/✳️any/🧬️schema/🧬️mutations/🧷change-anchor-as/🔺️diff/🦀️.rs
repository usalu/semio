use crate::diff::{En1992Diff, En1992AnchorsRows, En1992AnchorsPatch};
use super::ChangeAnchorAs;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeAnchorAs, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(a) = base.anchors.iter().find(|a| a.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} not found.", payload.anchor_id), Vec::<String>::new());
    };
    if (a.a_s - payload.new_value).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        anchors: Some(En1992AnchorsRows { modified: vec![En1992AnchorsPatch { id: payload.anchor_id.clone(), a_s: Some(payload.new_value), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
