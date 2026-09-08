
use super::*;

#[test]
fn next_id_retries_past_collisions() {
    let existing = vec!["n0".to_string(), "n2".to_string()];
    assert_eq!(next_id(existing.into_iter(), "n"), "n3");
}

#[test]
fn hex_to_rgb01_parses_pure_colors() {
    assert_eq!(hex_to_rgb01("#ffffff"), (1.0, 1.0, 1.0));
    assert_eq!(hex_to_rgb01("#000000"), (0.0, 0.0, 0.0));
    assert_eq!(hex_to_rgb01("#ff0000"), (1.0, 0.0, 0.0));
}

#[test]
fn von_mises_color_maps_extremes_midpoint_and_clamps() {
    assert_eq!(von_mises_color(0.0, 0.0, 100.0), VON_MISES_BANDS[0]);
    assert_eq!(von_mises_color(100.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() - 1]);
    assert_eq!(von_mises_color(50.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() / 2]);
    assert_eq!(von_mises_color(-10.0, 0.0, 100.0), VON_MISES_BANDS[0]);
    assert_eq!(von_mises_color(200.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() - 1]);
}

#[test]
fn parse_result_display_unknown_mode_falls_back_to_static() {
    let args = DslValue::object([("mode".to_string(), DslValue::String("bogus".to_string()))]);
    assert_eq!(parse_result_display(Some(&args)).mode, DisplayMode::Static);
}

#[test]
fn parse_result_display_missing_args_defaults_to_static_with_no_source() {
    let display = parse_result_display(None);
    assert_eq!(display.mode, DisplayMode::Static);
    assert!(display.source_id.is_none());
}
