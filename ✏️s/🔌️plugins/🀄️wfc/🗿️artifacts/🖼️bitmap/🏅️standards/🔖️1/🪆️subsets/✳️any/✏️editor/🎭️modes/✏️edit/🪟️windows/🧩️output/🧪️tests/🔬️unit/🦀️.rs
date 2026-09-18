//! 🧪️ Output window — the read-only inferred canvas, its staleness guard and its action roster.

use super::*;
use crate::editor::bitmap::modes::edit::windows::output::config::BitmapOutputWindowConfig;
use crate::schema::snapshot::decode_base64;
use crate::schema::snapshot::encode_base64;

#[test]
fn the_window_kind_is_a_canvas_with_a_migrated_action_roster() {
    let definition = definition();
    assert_eq!(definition.id, WFC_BITMAP_WINDOW_OUTPUT);
    assert_eq!(definition.body_key, BODY_KEY);
    assert_eq!(definition.surface_kind, SurfaceKind::Canvas2d);
    for action in &definition.actions {
        assert_eq!(action.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated);
    }
}

#[test]
fn an_unsolved_output_still_renders_its_extent_for_every_example() {
    for snapshot in [crate::examples::rooms_16::snapshot(), crate::examples::flowers_24::snapshot()] {
        let layers = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &snapshot.pinned);
        assert!(layers.contains("out-extent"), "a pane with no solve is never structurally empty");
        assert!(render(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default()).is_ok());
    }
}

#[test]
fn a_cached_solve_of_the_wrong_extent_is_discarded_rather_than_reshaped() {
    let snapshot = crate::examples::rooms_16::snapshot();
    let stale = BitmapTransient { output_pixels: Some(encode_base64(&[0, 1, 1, 0])), contradiction: false, output_width: 2, output_height: 2 };
    assert!(render(&snapshot, &stale, &BitmapOutputWindowConfig::default()).is_ok());
    let fresh = BitmapTransient {
        output_pixels: Some(encode_base64(&vec![0u8; (snapshot.output.width * snapshot.output.height) as usize])),
        contradiction: false,
        output_width: snapshot.output.width,
        output_height: snapshot.output.height,
    };
    let stale_layers = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &snapshot.pinned);
    let fresh_indices = decode_base64(fresh.output_pixels.as_deref().expect("fresh pixels")).expect("fresh pixels decode");
    let fresh_layers = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &fresh_indices, &snapshot.pinned);
    assert_ne!(stale_layers, fresh_layers, "a fresh solve draws more than a bare extent");
}

#[test]
fn hiding_the_pin_overlay_drops_exactly_the_pin_layers() {
    let snapshot = crate::examples::flowers_24::snapshot();
    let with_pins = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &snapshot.pinned);
    let without = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &[]);
    assert!(with_pins.contains("out-pin-0-0"));
    assert!(!without.contains("out-pin-"));
}
