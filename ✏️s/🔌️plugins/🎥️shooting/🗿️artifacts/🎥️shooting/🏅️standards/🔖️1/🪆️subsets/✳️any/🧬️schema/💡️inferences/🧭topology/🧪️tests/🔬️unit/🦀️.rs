use super::*;
use crate::{ShootingCamera, ShootingSavedCamera, ShootingShot};

fn saved_camera(id: &str) -> ShootingSavedCamera {
    ShootingSavedCamera { id: id.into(), label: id.into(), camera: ShootingCamera::default() }
}

fn shot(id: &str, camera_id: Option<&str>) -> ShootingShot {
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
