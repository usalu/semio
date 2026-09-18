//! 🧪️ Viewer input window — read-only, no actions, and non-empty for every example.

use super::*;

#[test]
fn the_window_declares_no_actions_and_no_utilities() {
    let definition = definition();
    assert_eq!(definition.id, WFC_BITMAP_VIEW_WINDOW_INPUT);
    assert!(definition.actions.is_empty(), "a viewer window may not carry a mutation action");
    assert!(definition.utilities.is_empty());
}

#[test]
fn every_example_renders() {
    for snapshot in [crate::examples::rooms_16::snapshot(), crate::examples::flowers_24::snapshot()] {
        assert!(render(&snapshot).is_ok());
    }
}
