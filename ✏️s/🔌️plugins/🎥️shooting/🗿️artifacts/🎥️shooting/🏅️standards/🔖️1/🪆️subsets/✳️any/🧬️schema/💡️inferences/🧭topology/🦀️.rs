//! 🧭 `topology` — one named inference: the shooting document's shot→camera reference graph
//! recast as a topology. `ShootingShot.camera_id` is the ONLY cross-entity reference this snapshot
//! carries (`Option<String>` into `saved_cameras`), so the honest derived stat per the
//! workflow/dag-shaped inference category is: saved cameras are roots (`depth` 0), a shot that
//! resolves to a real saved camera sits one level below it (`depth` 1), an unresolved/absent
//! `camera_id` stays a root too (`depth` 0). `cycleFree` is always `true` — a saved camera can never
//! reference a shot back, so the reference graph is structurally acyclic by construction, not by
//! traversal. Whole-snapshot scalar, so a plain function suffices — no `InferredField`/per-entity
//! caching needed (see the family root's doc comment for why).

use crate::ShootingSnapshot;
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Topology
/// 🧭️ Shooting's shot→camera reference topology — see module doc for the honest-derivation shape.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ShootingTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`ShootingTopology`] from a shooting snapshot's saved cameras + shots.
pub fn compute_shooting_topology(snapshot: &ShootingSnapshot) -> ShootingTopology {
    let camera_ids: BTreeSet<&String> = snapshot.saved_cameras.iter().map(|camera| &camera.id).collect();

    let mut topo_order = Vec::with_capacity(snapshot.saved_cameras.len() + snapshot.shots.len());
    let mut depth = BTreeMap::new();

    for camera in &snapshot.saved_cameras {
        topo_order.push(camera.id.clone());
        depth.insert(camera.id.clone(), 0);
    }
    for shot in &snapshot.shots {
        topo_order.push(shot.id.clone());
        let shot_depth = match &shot.camera_id {
            Some(camera_id) if camera_ids.contains(camera_id) => 1,
            _ => 0,
        };
        depth.insert(shot.id.clone(), shot_depth);
    }

    ShootingTopology { topo_order, depth, cycle_free: true, node_count: (snapshot.saved_cameras.len() + snapshot.shots.len()) as u32 }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
