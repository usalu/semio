//! 🧪️ The per-pane config: it round-trips through its own envelope, every operation has a real
//! inverse, and the armed-tile fallback never names a tile the document does not declare.

use super::*;
use protocol::Mutation;

#[test]
fn the_config_round_trips_through_its_own_text_and_pack_envelopes() {
    let config = Wfc3dConfig { camera_x: 1.5, camera_y: -2.0, camera_zoom: 3.0, active_tile_id: "room".into() };
    let text = <Wfc3dConfig as store::ArtifactDsl>::print_dsl(&config);
    assert_eq!(<Wfc3dConfig as store::ArtifactDsl>::parse_dsl(&text).expect("config text parses"), config);
    let bytes = <Wfc3dConfig as store::ArtifactPack>::encode_pack(&config);
    assert_eq!(<Wfc3dConfig as store::ArtifactPack>::decode_pack(&bytes).expect("config pack decodes"), config);
}

#[test]
fn the_default_pane_looks_through_an_unzoomed_camera_with_nothing_armed() {
    let config = Wfc3dConfig::default();
    assert_eq!((config.camera_x, config.camera_y, config.camera_zoom), (0.0, 0.0, 1.0));
    assert!(config.active_tile_id.is_empty());
}

/// ↩️ Every config operation is VCS'd like document content, so each one has to restore the prior
/// pane state exactly.
#[test]
fn every_config_operation_inverts_back_to_the_prior_pane_state() {
    let base = Wfc3dConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0, active_tile_id: "room".into() };
    let operations = vec![
        Wfc3dConfigMutation::ChangeCamera(ChangeCamera { x: 9.0, y: 8.0, zoom: 7.0 }),
        Wfc3dConfigMutation::ChangeActiveTile(ChangeActiveTile { tile_id: "corridor".into() }),
        Wfc3dConfigMutation::ReplaceConfig(ReplaceConfig { config: Wfc3dConfig::default() }),
    ];
    for operation in operations {
        let mut restored = operation.diff(&base).diff().clone();
        for back in operation.inverse(&base) {
            restored = back.diff(&restored).diff().clone();
        }
        assert_eq!(restored, base, "{operation:?} must invert exactly");
    }
}

/// 🀄️ An armed id the document no longer declares must NOT be handed to `pin-slot`; the fallback is
/// the document's own first tile.
#[test]
fn a_stale_armed_tile_falls_back_to_the_documents_first_tile() {
    let document = crate::examples::two_room_corridor::snapshot();
    let stale = Wfc3dConfig { active_tile_id: "ghost".into(), ..Default::default() };
    assert_eq!(wfc3d_active_tile_id(&stale, &document).as_deref(), Some("corridor"));
    let armed = Wfc3dConfig { active_tile_id: "room".into(), ..Default::default() };
    assert_eq!(wfc3d_active_tile_id(&armed, &document).as_deref(), Some("room"));
    let mut empty = document;
    empty.tiles.clear();
    assert_eq!(wfc3d_active_tile_id(&armed, &empty), None, "an empty catalogue arms nothing rather than an id `pin-slot` would refuse");
}
