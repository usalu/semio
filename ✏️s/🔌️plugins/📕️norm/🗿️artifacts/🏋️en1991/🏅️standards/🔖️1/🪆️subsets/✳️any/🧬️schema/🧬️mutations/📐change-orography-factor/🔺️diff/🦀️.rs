//! Diff for `change-orography-factor`.
use super::ChangeOrographyFactor;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeOrographyFactor, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.orography_factor == payload.new_orography_factor {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { orography_factor: Some(payload.new_orography_factor), ..Default::default() })
}
