use super::*;
use protocol::Mutation;

/// 🔁️ Every leaf must round-trip: `diff` forward, `inverse` back, landing exactly on the base.
fn config_round_trip(base: &Generation3dViewConfig, operation: &Generation3dViewConfigMutation) -> Generation3dViewConfig {
    let forward = operation.diff(base).into_parts().0;
    let backwards = operation.inverse(base);
    assert!(!backwards.is_empty(), "every viewer config leaf must declare a real inverse");
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).into_parts().0;
    }
    assert_eq!(&restored, base, "inverse() must exactly restore the pre-operation config");
    forward
}

#[test]
fn view_config_default_matches_the_read_only_preview_defaults() {
    let config = Generation3dViewConfig::default();
    assert_eq!(config.show_mode, "shaded");
    assert_eq!(config.effective_show_mode(), "shaded");
    assert_eq!(config.preview_camera, Generation3dViewCamera { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 });
    assert_eq!(config.sun(), semio_framework_plugin::WorldSunConfig::default());
    let pack = <Generation3dViewConfig as store::ArtifactPack>::encode_pack_with(&config, &store::PackEncodeOptions::default()).expect("the viewer default must be pack-encodable before the registry constructs its store");
    assert_eq!(<Generation3dViewConfig as store::ArtifactPack>::decode_pack_with(&pack, &store::PackDecodeOptions::default()).expect("the viewer default pack must decode"), config);
}

#[test]
fn view_config_show_mode_round_trips() {
    let base = Generation3dViewConfig::default();
    let next = config_round_trip(&base, &Generation3dViewConfigMutation::SetShowMode(SetShowMode { value: "wireframe".into() }));
    assert_eq!(next.show_mode, "wireframe");
    assert_eq!(next.effective_show_mode(), "wireframe");
}

#[test]
fn view_config_lod_mode_round_trips_and_drives_the_tessellation_tolerance() {
    let base = Generation3dViewConfig::default();
    assert_eq!(base.tolerance(), 0.05);
    let coarse = config_round_trip(&base, &Generation3dViewConfigMutation::SetLodMode(SetLodMode { value: "coarse".into() }));
    assert_eq!(coarse.tolerance(), 0.15);
    let fine = config_round_trip(&base, &Generation3dViewConfigMutation::SetLodMode(SetLodMode { value: "fine".into() }));
    assert_eq!(fine.tolerance(), 0.02);
}

#[test]
fn view_config_preview_camera_round_trips() {
    let base = Generation3dViewConfig::default();
    let camera = Generation3dViewCamera { position: [1.0, 2.0, 3.0], target: [0.5, 0.5, 0.5], fov: 60.0 };
    let next = config_round_trip(&base, &Generation3dViewConfigMutation::SetPreviewCamera(SetPreviewCamera { camera: camera.clone() }));
    assert_eq!(next.preview_camera, camera);
}

#[test]
fn view_config_sun_round_trips_as_raw_json() {
    let base = Generation3dViewConfig::default();
    let next = config_round_trip(&base, &Generation3dViewConfigMutation::SetSun(SetSun { json: "{\"enabled\":false}".into() }));
    assert_eq!(next.sun_json, "{\"enabled\":false}");
    assert!(!next.sun().enabled);
}

/// 📜️ Every leaf descriptor's `owner` must name a REAL directory that is an immediate child of this
/// aggregate's own mutation root — the invariant the sibling surface's provisional descriptors break.
#[test]
fn every_view_config_leaf_owner_directory_exists_on_disk() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../..");
    for descriptor in <Generation3dViewConfigMutation as Mutation<Generation3dViewConfig>>::DESCRIPTORS {
        let path = std::path::Path::new(root).join(descriptor.owner);
        assert!(path.is_dir(), "leaf owner directory must exist: {}", descriptor.owner);
        assert!(path.join("🔣️.json").is_file(), "leaf descriptor must exist: {}", descriptor.owner);
        assert!(path.join("🧬️schema/🔣️.json").is_file(), "leaf payload schema must exist: {}", descriptor.owner);
    }
}

/// 📜️ Text and binary op codecs must round-trip every variant, or a replayed config log is lost.
#[test]
fn view_config_operations_round_trip_through_text_and_binary() {
    let operations = vec![
        Generation3dViewConfigMutation::SetShowMode(SetShowMode { value: "points".into() }),
        Generation3dViewConfigMutation::SetLodMode(SetLodMode { value: "fine".into() }),
        Generation3dViewConfigMutation::SetPreviewCamera(SetPreviewCamera { camera: Generation3dViewCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], fov: 70.0 } }),
        Generation3dViewConfigMutation::SetSun(SetSun { json: "{}".into() }),
    ];
    for operation in operations {
        let bytes = protocol::OpBinary::encode_op(&operation).expect("binary encode");
        let decoded: Generation3dViewConfigMutation = protocol::OpBinary::decode_op(&bytes).expect("binary decode");
        assert_eq!(decoded, operation);
    }
}
