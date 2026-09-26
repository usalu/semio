use crate::diff::En1992Diff;
use crate::mutations::change_delta_c_dev::ChangeDeltaCDev;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeDeltaCDev, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if (base.delta_c_dev - payload.new_delta_c_dev).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { delta_c_dev: Some(payload.new_delta_c_dev), ..Default::default() })
}
