//! 🧪️ The per-pane config — defaults, the armed-tile fallback and mutation inverses.

use crate::editor::wfc2d::config::{wfc2d_active_tile_id, ChangeActiveTile, ChangeCamera, Wfc2dConfig, Wfc2dConfigMutation};

#[test]
fn default_camera_is_identity() {
    let config = Wfc2dConfig::default();
    assert_eq!((config.camera_x, config.camera_y, config.camera_zoom), (0.0, 0.0, 1.0));
    assert!(config.active_tile_id.is_empty());
}

/// 🀄️ An unset or unknown armed tile falls back to the document's first tile — never an empty id a
/// pin verb would refuse.
#[test]
fn the_armed_tile_falls_back_to_the_document() {
    let document = crate::examples::two_room_corridor::document();
    assert_eq!(wfc2d_active_tile_id(&Wfc2dConfig::default(), &document).as_deref(), Some("corridor"));
    let armed = Wfc2dConfig { active_tile_id: "room".into(), ..Default::default() };
    assert_eq!(wfc2d_active_tile_id(&armed, &document).as_deref(), Some("room"));
    let stale = Wfc2dConfig { active_tile_id: "ghost".into(), ..Default::default() };
    assert_eq!(wfc2d_active_tile_id(&stale, &document).as_deref(), Some("corridor"));
    assert_eq!(wfc2d_active_tile_id(&armed, &crate::Wfc2dSnapshot::default()), None);
}

/// ↩️ Every config mutation restores the exact prior value.
#[test]
fn config_mutations_are_invertible() {
    use protocol::MutationKind;
    let base = Wfc2dConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0, active_tile_id: "room".into() };
    let camera = ChangeCamera { x: 9.0, y: 9.0, zoom: 9.0 };
    let next = <ChangeCamera as MutationKind<Wfc2dConfig, Wfc2dConfigMutation>>::diff(&camera, &base).diff().clone();
    assert_eq!(next.camera_x, 9.0);
    assert_eq!(next.active_tile_id, "room", "a camera change never disarms the tile");
    let inverse = <ChangeCamera as MutationKind<Wfc2dConfig, Wfc2dConfigMutation>>::inverse(&camera, &base);
    assert_eq!(inverse, vec![Wfc2dConfigMutation::ChangeCamera(ChangeCamera { x: 1.0, y: 2.0, zoom: 3.0 })]);
    let tile = ChangeActiveTile { tile_id: "corridor".into() };
    let inverse = <ChangeActiveTile as MutationKind<Wfc2dConfig, Wfc2dConfigMutation>>::inverse(&tile, &base);
    assert_eq!(inverse, vec![Wfc2dConfigMutation::ChangeActiveTile(ChangeActiveTile { tile_id: "room".into() })]);
}
