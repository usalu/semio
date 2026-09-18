//! 🧪️ Per-window ownership laws — both panes own their own config instance, and the config/transient
//! envelopes round-trip through the exact codecs the retained loader reads.

use super::*;

#[test]
fn the_default_pane_config_snaps_to_a_visible_unit_grid() {
    let config = Grid2dWindowConfig::default();
    assert_eq!(config.camera_zoom, 1.0);
    assert!(config.grid_visible && config.grid_snap_enabled);
    assert!(config.active_tile_id.is_empty() && config.solve_json.is_empty());
}

#[test]
fn the_two_panes_own_two_distinct_config_instances() {
    assert_ne!(
        <Grid2dGridWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::WINDOW_KIND_ID,
        <Grid2dPreviewWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::WINDOW_KIND_ID
    );
    assert_eq!(<Grid2dGridWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::SCHEMA, <Grid2dPreviewWindowConfigOwner as semio_framework_plugin::WindowConfigOwner>::SCHEMA);
}

#[test]
fn the_config_round_trips_through_its_text_and_pack_envelopes() {
    let config = Grid2dWindowConfig { camera_x: 3.0, camera_y: -2.0, camera_zoom: 1.5, grid_visible: false, grid_snap_enabled: false, grid_factor: 8.0, active_tile_id: "empty".into(), solve_json: "{}".into() };
    let text = <Grid2dWindowConfig as store::ArtifactDsl>::print_dsl(&config);
    assert_eq!(<Grid2dWindowConfig as store::ArtifactDsl>::parse_dsl(&text).expect("config dsl parses"), config);
    let bytes = <Grid2dWindowConfig as store::ArtifactPack>::encode_pack(&config);
    assert_eq!(<Grid2dWindowConfig as store::ArtifactPack>::decode_pack(&bytes).expect("config pack decodes"), config);
    assert!(<Grid2dWindowConfig as store::ArtifactPack>::record_spec().is_some(), "a missing record spec fails every retained window load");
}

#[test]
fn the_transient_round_trips_and_absorbs_wholesale() {
    let transient = Grid2dWindowTransient { hovered_cell: "2,1".into() };
    let bytes = <Grid2dWindowTransient as store::ArtifactPack>::encode_pack(&transient);
    assert_eq!(<Grid2dWindowTransient as store::ArtifactPack>::decode_pack(&bytes).expect("transient decodes"), transient);
    let mut base = Grid2dWindowTransient::default();
    protocol::MutationDiff::absorb(&mut base, transient.clone());
    assert_eq!(base, transient);
}

#[test]
fn a_config_mutation_inverts_to_the_base_it_replaced() {
    let base = Grid2dWindowConfig::default();
    let next = Grid2dWindowConfig { grid_factor: 16.0, ..base.clone() };
    let mutation = Grid2dWindowConfigMutation::Snapshot { config: next };
    let inverse = protocol::Mutation::inverse(&mutation, &base);
    assert_eq!(inverse, vec![Grid2dWindowConfigMutation::Snapshot { config: base }]);
}

#[test]
fn a_config_write_without_a_window_is_refused() {
    let view = semio_framework_plugin::ViewModel::default();
    assert!(addressed_config(&view, Grid2dWindowConfig::default()).is_err(), "an unaddressed config write would land in the wrong pane's store");
    assert_eq!(kind_for_view(&view), None);
}
