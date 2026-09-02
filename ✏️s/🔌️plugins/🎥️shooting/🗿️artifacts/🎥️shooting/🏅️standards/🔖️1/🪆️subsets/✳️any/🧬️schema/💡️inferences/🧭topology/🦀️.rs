//! 🧭 `topology` — one named inference: the shooting document's shot→camera reference graph
//! recast as a topology. `ShootingShot.camera_id` is the ONLY cross-entity reference this snapshot
//! carries (`Option<String>` into `saved_cameras`), so the honest derived stat per the
//! workflow/dag-shaped inference category is: saved cameras are roots (`depth` 0), a shot that
//! resolves to a real saved camera sits one level below it (`depth` 1), an unresolved/absent
//! `camera_id` stays a root too (`depth` 0). `cycleFree` is always `true` — a saved camera can never
//! reference a shot back, so the reference graph is structurally acyclic by construction, not by
//! traversal. Whole-snapshot scalar, so a plain function suffices — no `InferredField`/per-entity
//! caching needed (see the family root's doc comment for why).

use crate::artifacts::shooting::ShootingSnapshot;
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
pub async fn compute_shooting_topology(snapshot: &ShootingSnapshot) -> ShootingTopology {
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
mod tests {
    use super::*;
    use crate::artifacts::shooting::{ShootingCamera, ShootingSavedCamera, ShootingShot};

    async fn saved_camera(id: &str) -> ShootingSavedCamera {
        ShootingSavedCamera { id: id.into(), label: id.into(), camera: ShootingCamera::default() }
    }

    async fn shot(id: &str, camera_id: Option<&str>) -> ShootingShot {
        ShootingShot { id: id.into(), label: id.into(), width: 1024, height: 768, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: camera_id.map(Into::into) }
    }

    #[semio_framework_async_macros::async_test]
    async fn shot_referencing_a_real_camera_sits_one_level_below_it() {
        let snapshot = ShootingSnapshot { saved_cameras: vec![saved_camera("cam-1")], shots: vec![shot("shot-1", Some("cam-1"))], ..ShootingSnapshot::default() };
        let topology = compute_shooting_topology(&snapshot);
        assert_eq!(topology.depth.get("cam-1"), Some(&0));
        assert_eq!(topology.depth.get("shot-1"), Some(&1));
        assert!(topology.cycle_free);
        assert_eq!(topology.node_count, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn shot_with_a_dangling_camera_ref_stays_a_root() {
        let snapshot = ShootingSnapshot { saved_cameras: Vec::new(), shots: vec![shot("shot-1", Some("missing-cam"))], ..ShootingSnapshot::default() };
        let topology = compute_shooting_topology(&snapshot);
        assert_eq!(topology.depth.get("shot-1"), Some(&0));
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_is_the_vacuous_topology() {
        let topology = compute_shooting_topology(&ShootingSnapshot::default());
        assert!(topology.topo_order.is_empty());
        assert!(topology.cycle_free);
        assert_eq!(topology.node_count, 0);
    }
}
//#endregion 🧪️Tests
