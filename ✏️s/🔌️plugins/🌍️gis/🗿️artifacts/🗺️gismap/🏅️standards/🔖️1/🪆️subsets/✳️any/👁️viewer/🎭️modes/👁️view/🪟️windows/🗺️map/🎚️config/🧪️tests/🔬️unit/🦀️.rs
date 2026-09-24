use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn camera() -> GisMapViewerCamera {
    GisMapViewerCamera { x: 1_250.5, y: -830.25, zoom: 3.5 }
}

#[test]
fn the_default_camera_is_admissible_and_the_degenerate_ones_are_not() {
    assert!(GisMapViewerCamera::default().is_valid());
    assert!(camera().is_valid());
    assert!(!GisMapViewerCamera { zoom: 0.0, ..camera() }.is_valid(), "a zero zoom is not a camera");
    assert!(!GisMapViewerCamera { zoom: -1.0, ..camera() }.is_valid());
    assert!(!GisMapViewerCamera { x: f64::NAN, ..camera() }.is_valid());
    assert!(!GisMapViewerCamera { y: f64::INFINITY, ..camera() }.is_valid());
}

#[test]
fn a_camera_prints_the_scene_camera_json_the_tiled_map_host_reads() {
    let json = camera().scene_camera_json();
    let value: serde_json::Value = serde_json::from_str(&json).expect("the scene camera is JSON");
    assert_eq!(value, serde_json::json!({ "x": 1_250.5, "y": -830.25, "zoom": 3.5 }), "{json}");
}

#[test]
fn set_camera_replaces_the_camera_and_inverts_back_to_the_base() {
    let base = GisMapViewerWindowConfig::default();
    let mutation = GisMapViewerWindowConfigMutation::SetCamera(SetCamera { camera: camera() });
    let next = mutation.diff(&base).diff().apply(&base).expect("the camera applies");
    assert_eq!(next.camera, camera());
    let mut restored = next;
    for inverse in mutation.inverse(&base) {
        restored = inverse.diff(&restored).diff().apply(&restored).expect("the inverse applies");
    }
    assert_eq!(restored, base, "panning back is exactly the inverse");
}

#[test]
fn re_setting_the_same_camera_is_a_declared_no_op() {
    let base = GisMapViewerWindowConfig { camera: camera() };
    let mutation = GisMapViewerWindowConfigMutation::SetCamera(SetCamera { camera: camera() });
    let outcome = mutation.diff(&base);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"), "a debounced duplicate must not publish a change");
    assert_eq!(outcome.diff().apply(&base).expect("the whole-record no-op applies"), base, "the no-op keeps the camera instead of resetting the record");
    assert!(mutation.inverse(&base).is_empty(), "a no-op has no inverse");
}

#[test]
fn the_window_config_round_trips_through_its_text_and_binary_codecs() {
    use store::{ArtifactDsl, ArtifactPack};
    let config = GisMapViewerWindowConfig { camera: camera() };
    assert_eq!(GisMapViewerWindowConfig::parse_dsl(&config.print_dsl()).expect("dsl round trip"), config);
    assert_eq!(GisMapViewerWindowConfig::decode_pack(&config.encode_pack()).expect("pack round trip"), config);
    let mutation = GisMapViewerWindowConfigMutation::SetCamera(SetCamera { camera: camera() });
    assert_eq!(GisMapViewerWindowConfigMutation::parse_op(&mutation.print_op()).expect("text op"), mutation);
    assert_eq!(GisMapViewerWindowConfigMutation::decode_op(&mutation.encode_op().expect("encode")).expect("binary op"), mutation);
}

#[test]
fn the_owner_binds_the_viewer_map_window_kind() {
    use semio_framework_plugin::WindowConfigOwner;
    assert_eq!(<GisMapViewerWindowConfigOwner as WindowConfigOwner>::WINDOW_KIND_ID, super::super::WINDOW_KIND_ID);
    assert_eq!(<GisMapViewerWindowConfigOwner as WindowConfigOwner>::SCHEMA, "gis.mapviewerwindowcfg");
}
