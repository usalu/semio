//! 🌉 `change-wind-bridge-traffic-kind` diff: patches `traffic_kind` of the addressed bridge wind case row.

use crate::change_wind_bridge_traffic_kind::ChangeWindBridgeTrafficKind;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindBridgeDelta, En1991WindBridgePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeWindBridgeTrafficKind, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.wind_bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bridge wind case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.traffic_kind == payload.new_traffic_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge wind case already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_bridges: En1991WindBridgeDelta::modification(&payload.id, En1991WindBridgePatch { traffic_kind: Some(payload.new_traffic_kind), ..Default::default() }), ..Default::default() })
}
