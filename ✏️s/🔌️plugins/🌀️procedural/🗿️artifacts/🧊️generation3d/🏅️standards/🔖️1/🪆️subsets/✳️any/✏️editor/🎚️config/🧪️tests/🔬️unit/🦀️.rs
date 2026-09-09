use super::*;
use protocol::Mutation;

#[test]
fn generation3d_config_default_matches_the_former_runtime_defaults() {
    let config = Generation3dConfig::default();
    assert_eq!(config.show_mode, "shaded");
    assert_eq!(config.sun(), semio_framework_plugin::WorldSunConfig::default());
    let pack = <Generation3dConfig as store::ArtifactPack>::encode_pack_with(&config, &store::PackEncodeOptions::default()).expect("the app default must be pack-encodable before the registry constructs its store");
    assert_eq!(<Generation3dConfig as store::ArtifactPack>::decode_pack_with(&pack, &store::PackDecodeOptions::default()).expect("the app default pack must decode"), config);
}

fn config_round_trip(base: &Generation3dConfig, operation: &Generation3dConfigMutation) -> Generation3dConfig {
    let forward = operation.diff(base).into_parts().0;
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).into_parts().0;
    }
    assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
    forward
}

#[test]
fn config_set_camera_and_preview_camera_round_trip() {
    let base = Generation3dConfig::default();
    let next = config_round_trip(&base, &Generation3dConfigMutation::SetCamera(SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }));
    assert_eq!(next.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
    let camera = Generation3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 };
    let next2 = config_round_trip(&next, &Generation3dConfigMutation::SetPreviewCamera(SetPreviewCamera { camera: camera.clone() }));
    assert_eq!(next2.preview_camera, camera);
}

#[test]
fn config_set_sun_round_trip_as_raw_json() {
    let base = Generation3dConfig::default();
    let next = config_round_trip(&base, &Generation3dConfigMutation::SetSun(SetSun { json: "{\"enabled\":true}".into() }));
    assert_eq!(next.sun_json, "{\"enabled\":true}");
}

#[test]
fn config_set_selected_generation_round_trips() {
    let base = Generation3dConfig::default();
    let next = config_round_trip(&base, &Generation3dConfigMutation::SetSelectedGeneration(SetSelectedGeneration { selected_generation_id: Some("generation-1".into()) }));
    assert_eq!(next.selected_generation_id, Some("generation-1".to_string()));
}

#[test]
fn config_op_text_round_trips_every_variant() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetLodMode(SetLodMode { value: "coarse".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetShowMode(SetShowMode { value: "wireframe".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetCamera(SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetPreviewCamera(SetPreviewCamera { camera: Generation3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], fov: 45.0 } }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetSun(SetSun { json: "{}".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetSelectedGeneration(SetSelectedGeneration { selected_generation_id: Some("g1".into()) }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetSnapshot(SetSnapshot { config: Generation3dConfig::default() }));
}

/// 📜️ Every leaf descriptor's `owner` must name a REAL directory that is an immediate child of this
/// aggregate's own mutation root, carrying both its descriptor and its payload schema — the
/// invariant the surface's former hand-written provisional descriptors all broke.
#[test]
fn every_config_leaf_owner_directory_exists_on_disk() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../..");
    for descriptor in <Generation3dConfigMutation as Mutation<Generation3dConfig>>::DESCRIPTORS {
        let path = std::path::Path::new(root).join(descriptor.owner);
        assert!(path.is_dir(), "leaf owner directory must exist: {}", descriptor.owner);
        assert!(path.join("🔣️.json").is_file(), "leaf descriptor must exist: {}", descriptor.owner);
        assert!(path.join("🧬️schema/🔣️.json").is_file(), "leaf payload schema must exist: {}", descriptor.owner);
    }
}

/// 📜️ The aggregate must expose exactly one descriptor per declared variant, in binary-tag order.
#[test]
fn config_leaf_descriptors_cover_every_variant_in_tag_order() {
    let kinds: Vec<&str> = <Generation3dConfigMutation as Mutation<Generation3dConfig>>::DESCRIPTORS.iter().map(|descriptor| descriptor.semantic_kind).collect();
    assert_eq!(kinds, vec!["set-snapshot", "set-lod-mode", "set-show-mode", "set-camera", "set-preview-camera", "set-sun", "set-selected-generation"]);
}
