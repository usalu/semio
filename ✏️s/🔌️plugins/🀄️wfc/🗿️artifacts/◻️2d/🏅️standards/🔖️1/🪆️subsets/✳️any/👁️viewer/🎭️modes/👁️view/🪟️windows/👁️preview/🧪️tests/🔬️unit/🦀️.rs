//! 🧪️ The viewer's board window — read-only projection of the AUTHORED board.

use super::{board_layers_json, definition, render, WFC_2D_VIEW_BODY, WFC_2D_VIEW_WINDOW};

#[test]
fn window_identity_is_the_declared_board() {
    assert_eq!(WFC_2D_VIEW_WINDOW, "wfc-2d-board");
    assert_eq!(definition().body_key, WFC_2D_VIEW_BODY);
    assert!(definition().actions.is_empty(), "a viewer declares no actions");
}

#[test]
fn every_slot_paints_and_pins_carry_their_media() {
    let document = crate::examples::wall_roof_facade_strip::document();
    let layers = board_layers_json(&document);
    for slot in &document.slots {
        assert!(layers.contains(&format!("\"slot-{}\"", slot.id)));
    }
    assert!(layers.contains("tile-bay-1-top-roof-0"), "the pinned roof bay paints its tile");
}

#[test]
fn render_produces_a_surface_for_every_example() {
    for document in crate::examples::documents() {
        render(&document).expect("the board window renders");
    }
}
