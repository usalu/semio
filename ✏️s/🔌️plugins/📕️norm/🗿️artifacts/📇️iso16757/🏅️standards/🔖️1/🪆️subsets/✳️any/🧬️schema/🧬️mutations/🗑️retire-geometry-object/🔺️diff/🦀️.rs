//! Diff for `retire-geometry-object`.

use crate::retire_geometry_object::mutation::RetireGeometryObject;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_diff::{Iso16757Diff, Iso16757GeometryObjectsRows};

pub fn diff(payload: &RetireGeometryObject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.geometry.objects.contains_key(&payload.id) {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Geometry \"{}\" does not exist.", payload.id),
            [payload.id.clone()],
        );
    }
    protocol::MutationOutcome::new(Iso16757Diff { geometry_objects: Some(Iso16757GeometryObjectsRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
