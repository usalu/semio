//! Diff for `change-bridge-v-rd-n`.
use super::ChangeBridgeVRdN;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeBridgeVRdN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut bridges = base.bridges.clone();
    let Some(b) = bridges.get_mut(payload.index) else { return protocol::MutationOutcome::error("mutation.target-missing", "bridge", Vec::<String>::new()); };
    b.v_rd_n = payload.new_v_rd_n;
    protocol::MutationOutcome::new(En1998Diff { bridges: Some(bridges), ..Default::default() })
}
