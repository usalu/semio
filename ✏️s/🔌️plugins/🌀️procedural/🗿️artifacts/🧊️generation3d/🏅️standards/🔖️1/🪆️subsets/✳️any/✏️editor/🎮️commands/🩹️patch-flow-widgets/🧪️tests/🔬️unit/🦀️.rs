use super::*;

fn sliders() -> FlowHostSnapshot {
    dsl::json::from_json_str(r#"{"schema":"flow.hostSnapshot","camera":{"x":0.0,"y":0.0,"zoom":1.0},"widgets":[{"kind":"inputSlider","id":"height","label":"Height","value":6.0,"min":0.0,"max":10.0,"step":0.5},{"kind":"inputSlider","id":"radius","label":"Radius","value":2.0,"min":0.0,"max":10.0,"step":0.5},{"kind":"inputNote","id":"note","text":"Note"}],"synapses":[],"layout":{}}"#).expect("slider fixture decodes")
}

/// 🎚️ A value patch is one ABSOLUTE `change-slider-value` per addressed slider that moves — never a whole-widget scratch
/// diff, never a coalesce key — and an unchanged slider, a non-slider and a non-`value` field yield nothing.
#[test]
fn a_value_patch_yields_one_absolute_leaf_per_moving_slider() {
    let host = sliders();
    let patch = |ids: &[&str], field: &str, value: f64| PatchFlowWidgets { widget_ids: ids.iter().map(|id| id.to_string()).collect(), field: field.into(), value: Some(value) };
    assert_eq!(patch_leaves(&host, &patch(&["height", "radius", "note"], "value", 7.5)), vec![change_slider_value("height", 7.5), change_slider_value("radius", 7.5)]);
    assert_eq!(patch_leaves(&host, &patch(&["height"], "value", 6.0)), Vec::new(), "the value the slider already holds is no edit");
    assert_eq!(patch_leaves(&host, &patch(&["height"], "min", 1.0)), Vec::new(), "only the value field is a scrub of this verb");
    assert_eq!(patch_leaves(&host, &patch(&["height"], "value", f64::NAN)), Vec::new(), "a non-finite value is no edit");
    host.retire_cold();
}

/// 🐢️ A value patch repaints only what one slider value invalidates, whether it is a scrub tick or a one-shot.
#[test]
fn a_value_patch_declares_the_narrow_scope() {
    assert_ne!(crate::editor::generation3d::commands::node_graph_edit::slider_gesture_ui_scope(), semio_framework::kernel::UiDirtyScope::Full, "a slider value must not repaint the whole shell");
}
