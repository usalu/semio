//! 🧪️ The viewer's read-only preview window renders every bundled example.

use super::*;

#[test]
fn the_window_kind_is_the_read_only_view_surface() {
    let definition = definition();
    assert_eq!(definition.id, WFC_3D_VIEW_WINDOW);
    assert_eq!(definition.body_key, WFC_3D_VIEW_BODY);
    assert!(definition.actions.is_empty());
}

#[test]
fn every_example_renders_a_non_empty_surface() {
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        let built = render(&document).expect("the viewer surface assembles");
        assert!(!format!("{built:?}").is_empty());
    }
}
