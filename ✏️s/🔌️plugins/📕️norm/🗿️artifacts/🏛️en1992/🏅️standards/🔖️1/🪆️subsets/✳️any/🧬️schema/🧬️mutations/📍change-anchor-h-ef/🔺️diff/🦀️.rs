use crate::diff::{En1992AnchorList, En1992Diff};
use crate::mutations::change_anchor_h_ef::ChangeAnchorHEf;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeAnchorHEf, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut anchors = base.anchors.clone();
    let Some(a) = anchors.iter_mut().find(|a| a.id == payload.anchor_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Anchor {} not found.", payload.anchor_id), Vec::<String>::new());
    };
    if (a.h_ef - payload.new_value).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    a.h_ef = payload.new_value;
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorList { values: anchors }), ..Default::default() })
}
