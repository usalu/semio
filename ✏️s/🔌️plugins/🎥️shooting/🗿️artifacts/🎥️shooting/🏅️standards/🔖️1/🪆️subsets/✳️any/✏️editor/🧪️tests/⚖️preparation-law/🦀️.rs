//! ⚖️ Runtime proof that both shooting editing lanes publish through their store factories: the document lane's paged `mutation_apply_preparation_factory` and the config lane's `config_apply_preparation_factory` each drive a real edit through the canonical sealer to `Prepared` and close under exactly quoted grants.
use semio_framework_os_kernel as store;
use semio_s_artifact_shooting_shooting::editor::shooting::config::{SetCamera, ShootingConfig, ShootingConfigMutation};
use semio_s_artifact_shooting_shooting::standards::v1::subsets::any::schema::mutations::create_shot::CreateShot;
use semio_s_artifact_shooting_shooting::{ShootingMutation, ShootingShot, ShootingSnapshot};
use std::sync::Arc;

#[global_allocator]
static SHOOTING_PREPARATION_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

fn grant() -> store::ArtifactStoreOneItemGrant {
    store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 }
}

/// 📄️ A structural document edit reaches `Prepared` through the paged document factory and its inverse deletes the created shot again.
#[test]
fn document_lane_create_shot_reaches_prepared_and_closes_under_quoted_grants() {
    let factory: Arc<dyn store::ArtifactStoreOneItemPreparationFactory<ShootingSnapshot, ShootingMutation>> = store::mutation_apply_preparation_factory::<ShootingSnapshot, ShootingMutation>();
    let shot = ShootingShot { id: "shot-1".into(), label: "Hero".into(), width: 1280, height: 720, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: None };
    let mutation = ShootingMutation::CreateShot(CreateShot { shot: shot.clone(), index: None });
    let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, ShootingSnapshot::default(), mutation, grant());
    assert_eq!(law.post.shots, vec![shot]);
    assert_eq!(law.inverse.len(), 1, "creating one shot is undone by exactly one row");
}

/// 🎚️ A config field-set reaches `Prepared` through the config factory and its inverse restores the displaced camera.
#[test]
fn config_lane_set_camera_reaches_prepared_and_closes_under_quoted_grants() {
    let factory: Arc<dyn store::ArtifactStoreOneItemPreparationFactory<ShootingConfig, ShootingConfigMutation>> = store::snapshot_clone_preparation::config_apply_preparation_factory::<ShootingConfig, ShootingConfigMutation>();
    let base = ShootingConfig::default();
    let mut camera = base.camera.clone();
    camera.position = [4.0, 5.0, 6.0];
    let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), ShootingConfigMutation::SetCamera(SetCamera { camera: camera.clone() }), grant());
    assert_eq!(law.post.camera, camera);
    assert_eq!(law.inverse, vec![ShootingConfigMutation::SetCamera(SetCamera { camera: base.camera })]);
}
