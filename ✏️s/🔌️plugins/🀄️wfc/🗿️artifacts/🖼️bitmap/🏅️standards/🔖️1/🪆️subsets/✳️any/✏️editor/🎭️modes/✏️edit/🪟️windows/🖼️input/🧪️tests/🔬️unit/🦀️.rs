//! 🧪️ Input window — the window kind it declares and the canvas it renders for every example.

use super::*;
use crate::editor::bitmap::modes::edit::windows::input::config::BitmapInputWindowConfig;

#[test]
fn the_window_kind_is_an_interactive_canvas_with_a_migrated_action_roster() {
    let definition = definition();
    assert_eq!(definition.id, WFC_BITMAP_WINDOW_INPUT);
    assert_eq!(definition.body_key, BODY_KEY);
    assert_eq!(definition.surface_kind, SurfaceKind::Canvas2d);
    assert!(!definition.actions.is_empty());
    for action in &definition.actions {
        assert_eq!(action.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated, "an unmigrated verb is dispatch-dead in the shell");
    }
}

#[test]
fn every_example_renders_a_non_empty_canvas() {
    for snapshot in [crate::examples::rooms_16::snapshot(), crate::examples::flowers_24::snapshot()] {
        let layers = crate::bitmap_layers_json("in", snapshot.input.width, snapshot.input.height, &snapshot.input.palette, &snapshot.input.indices().expect("the example decodes"), &[]);
        assert!(layers.len() > 2, "an example sample draws real layers");
        assert!(layers.contains("in-extent"));
        assert!(render(&snapshot, &BitmapInputWindowConfig::default()).is_ok(), "the surface assembles");
    }
}

#[test]
fn the_window_config_defaults_to_the_first_palette_entry() {
    let config = BitmapInputWindowConfig::default();
    assert_eq!(config.active_color, 0);
    assert!(config.zoom > 1.0, "a 16-pixel sample would be sixteen screen pixels at zoom 1");
}
