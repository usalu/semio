use crate::modes::edit::windows::{checks, diagrams, outliner, properties, viewport};
use crate::modes::edit::*;

#[test]
fn the_default_layout_lists_all_five_window_kinds() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    for kind in [outliner::WINDOW_OUTLINER, properties::WINDOW_PROPERTIES, viewport::WINDOW_VIEWPORT, checks::WINDOW_CHECKS, diagrams::WINDOW_DIAGRAMS] {
        assert!(json.contains(kind), "layout must reference {kind}: {json}");
    }
}

#[test]
fn the_mode_is_the_apps_default() {
    assert_eq!(definition().id, semio_s_plugin_norm_app_surface::MODE_EDIT);
}
