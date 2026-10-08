//! 🔺️ `change-limits` — sparse diff construction.

use super::ChangeLimits;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805LimitsPatch};

//#region 🔖️Diff

pub fn diff(payload: &ChangeLimits, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.limits == payload.new_limits {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Limits already has this value.");
    }
    let (old, new) = (&base.limits, &payload.new_limits);
    protocol::MutationOutcome::new(Vdi3805Diff {
        limits: Some(Vdi3805LimitsPatch {
            max_file_bytes: (old.max_file_bytes != new.max_file_bytes).then(|| new.max_file_bytes.clone()),
            max_records: (old.max_records != new.max_records).then(|| new.max_records.clone()),
            max_field_length: (old.max_field_length != new.max_field_length).then(|| new.max_field_length.clone()),
            max_nesting_depth: (old.max_nesting_depth != new.max_nesting_depth).then(|| new.max_nesting_depth.clone()),
        }),
        ..Default::default()
    })
}
