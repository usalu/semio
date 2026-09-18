//! 🧪️ Viewer output window — read-only, no actions, and non-empty for every example.

use super::*;

#[test]
fn the_window_declares_no_actions_and_no_utilities() {
    let definition = definition();
    assert_eq!(definition.id, WFC_BITMAP_VIEW_WINDOW_OUTPUT);
    assert!(definition.actions.is_empty());
    assert!(definition.utilities.is_empty());
}

#[test]
fn every_example_renders_its_declared_output_extent() {
    for snapshot in [crate::examples::rooms_16::snapshot(), crate::examples::flowers_24::snapshot()] {
        assert!(render(&snapshot).is_ok());
        let layers = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &snapshot.pinned);
        assert!(layers.contains("out-extent"));
    }
}
