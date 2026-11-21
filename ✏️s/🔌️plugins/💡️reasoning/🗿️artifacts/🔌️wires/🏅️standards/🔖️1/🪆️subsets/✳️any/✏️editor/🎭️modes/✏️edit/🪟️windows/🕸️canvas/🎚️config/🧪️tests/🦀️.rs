//! 🎟️ Native receiving and original owner quotes for the Wires canvas configuration.

use super::{SetCamera, WiresCanvasWindowConfigMutation, WiresCanvasWindowConfigOwner};
use semio_plugin::WindowConfigOwner;

#[test]
fn camera_fixture_matches_native_ordinal_receiving() {
    let fixture = include_str!("🧫️fixtures/🎥️camera/✅️ok/🦠️mutation/🔣️.json");
    let json: serde_json::Value = serde_json::from_str(fixture).expect("camera fixture");
    assert_eq!(json["kind"], "set-camera");
    let camera = serde_json::from_value(json["camera"].clone()).expect("independent camera oracle");
    let expected = WiresCanvasWindowConfigMutation::SetCamera(SetCamera { camera });
    let authored: WiresCanvasWindowConfigMutation =
        semio_framework_pack_json::from_json_str(fixture, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned camera mutation");
    assert_eq!(authored, expected);
    protocol::os_dsl::original_receiving::check_receiving_round_trip(&[authored]).expect("native camera mutation round trip");
}

#[test]
fn window_config_quotes_its_native_owners() {
    type Owner = WiresCanvasWindowConfigOwner;
    assert!(Owner::native_pair_demands().unwrap().capacity_bytes > 0);
    assert!(Owner::native_operation_wire().original_receiving.is_some());
    let demand = Owner::store_disposer_birth().unwrap();
    let disposer = Owner::build_store_disposer();
    assert_eq!(demand.capacity_bytes, std::alloc::Layout::for_value(disposer.as_ref()).size());
}
