//! Diff for `retire-geometry-object`.

use super::mutation::RetireGeometryObject;
use crate::{Iso16757Diff, Iso16757Snapshot};

pub fn diff(payload: &RetireGeometryObject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.geometry.objects.contains_key(&payload.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.missing-id",
            format!("Geometry \"{}\" does not exist.", payload.id),
            [payload.id.clone()],
        );
    }
    let mut geometry = base.geometry.clone();
    geometry.objects.remove(&payload.id);
    protocol::MutationOutcome::new(Iso16757Diff { geometry: Some(geometry), ..Default::default() })
}
