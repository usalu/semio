use super::*;
use semio_s_plugin_norm_structural_scene::{DataColumn, DataTable, Interval, Stroke};
use serde_json::{json, Value};
use std::io::Write;
use std::process::Stdio;

fn copy(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn v(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

fn axis(scale: AxisScale, domain: Option<(f64, f64)>) -> Axis {
    Axis { label: copy("Moment", "Moment"), quantity: semio_s_plugin_norm_structural_scene::QuantityKind::Moment, unit: "kNm".into(), scale, domain: domain.map(|(a, b)| Interval::new(a, b)), ticks: TickRule::Auto }
}

fn diagram() -> Diagram2d {
    let stroke = Stroke { width_px: 2.0, dash: Dash::Solid, tone: Tone::Neutral };
    Diagram2d {
        id: "mn".into(),
        title: copy("Interaction", "Interaktion"),
        description: copy("M-N interaction", "M-N-Interaktion"),
        axes: vec![axis(AxisScale::Linear, Some((0.0, 400.0))), axis(AxisScale::Linear, Some((0.0, 3000.0)))],
        series: vec![
            Series::Polygon { id: "region".into(), label: copy("Capacity", "Tragfähigkeit"), ring: vec![v(0.0, 0.0), v(300.0, 500.0), v(350.0, 2000.0), v(0.0, 2800.0)], fill: Fill { tone: Tone::Info, opacity: 0.25 }, hatch: None },
            Series::Polyline { id: "curve".into(), label: copy("Curve", "Kurve"), points: vec![v(0.0, 0.0), v(200.0, 1500.0), v(500.0, 3500.0)], style: stroke.clone() },
            Series::Ray { id: "ray".into(), label: copy("Ray", "Strahl"), origin: v(0.0, 0.0), through: v(100.0, 800.0), to_boundary: true, style: Stroke { width_px: 1.5, dash: Dash::Dashed, tone: Tone::Warning } },
        ],
        markers: vec![Marker { id: "load".into(), at: v(100.0, 800.0), shape: MarkerShape::Diamond, tone: Tone::Error, label: copy("Load", "Last"), utilisation: Some(0.62), bound_check: Some("c1".into()), binding: None }],
        annotations: vec![],
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![DataColumn { header: copy("M", "M"), unit: "kNm".into(), values: vec![0.0, 200.0, 12345.678] }, DataColumn { header: copy("N", "N"), unit: "kN".into(), values: vec![0.0, 1500.5] }] },
    }
}

fn options(locale: Locale) -> LoweringOptions {
    LoweringOptions { width: 640.0, height: 420.0, locale, dark: false, selected: None, hovered: None, min_target_px: 0 }
}

fn tests_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🪟️visual-windows/📈️diagrams2d/🧪️tests")
}

fn oracle(case: &Value) -> Value {
    let script = tests_dir().join("🔮️oracle/🐍️.py");
    let mut child = semio_s_plugin_norm_compliance::python_oracle::command().arg(script).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("python oracle");
    child.stdin.take().expect("stdin").write_all(case.to_string().as_bytes()).expect("write");
    let out = child.wait_with_output().expect("oracle");
    assert!(out.status.success(), "oracle failed");
    serde_json::from_slice(&out.stdout).expect("json")
}

#[test]
fn hit_distances_match_shapely() {
    let shapes = [
        PickShape::Path { points: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]] },
        PickShape::Area { ring: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]] },
        PickShape::Point { at: [5.0, 5.0], radius: 2.0 },
        PickShape::Rect { min: [0.0, 0.0], max: [4.0, 3.0] },
    ];
    let probes = [[3.0, 4.0], [12.0, 12.0], [-5.0, 2.0], [5.0, 9.0], [7.0, 7.0]];
    let json_shape = |s: &PickShape| match s {
        PickShape::Path { points } => json!({"kind": "path", "points": points}),
        PickShape::Area { ring } => json!({"kind": "area", "points": ring}),
        PickShape::Point { at, radius } => json!({"kind": "point", "at": at, "radius": radius}),
        PickShape::Rect { min, max } => json!({"kind": "rect", "min": min, "max": max}),
    };
    let queries: Vec<Value> = shapes.iter().flat_map(|s| probes.iter().map(move |p| json!([json_shape(s), p]))).collect();
    let o = oracle(&json!({"queries": queries, "log": [-1, 4], "linear": [0.0, 1.0, 0.25]}));
    let expected: Vec<f64> = o["distances"].as_array().expect("array").iter().map(|d| d.as_f64().expect("number")).collect();
    let actual: Vec<f64> = shapes.iter().flat_map(|s| probes.iter().map(move |p| s.distance(*p))).collect();
    for (a, e) in actual.iter().zip(expected) {
        assert!((a - e).abs() < 1e-9, "{a} != {e}");
    }
    let log: Vec<f64> = o["log"].as_array().expect("array").iter().map(|d| d.as_f64().expect("number")).collect();
    let ours = log_ticks(0.1, 1.0e4, false);
    assert_eq!(ours.len(), log.len());
    assert!(ours.iter().zip(log).all(|(a, b)| (a - b).abs() <= 1e-9 * b));
    assert_eq!(ticks_with_step(0.0, 1.0, 0.25), vec![0.0, 0.25, 0.5, 0.75, 1.0]);
}

#[test]
fn nice_ticks_use_one_two_five_steps() {
    assert_eq!(nice_ticks(0.0, 10.0, 5), vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
    assert_eq!(nice_ticks(0.0, 100.0, 4), vec![0.0, 50.0, 100.0]);
    assert_eq!(nice_ticks(-1.0, 1.0, 4), vec![-1.0, -0.5, 0.0, 0.5, 1.0]);
    assert_eq!(log_ticks(1.0, 100.0, true).len(), 19);
}

#[test]
fn axis_maps_round_trip_for_every_scale() {
    for scale in [AxisScale::Linear, AxisScale::Log10, AxisScale::Sqrt] {
        let m = AxisMap { scale, lo: 1.0, hi: 1000.0, p0: 0.0, p1: 300.0 };
        assert!((m.map(1.0)).abs() < 1e-9 && (m.map(1000.0) - 300.0).abs() < 1e-9);
        assert!((m.unmap(m.map(37.0)) - 37.0).abs() < 1e-9);
    }
    assert!((AxisMap { scale: AxisScale::Log10, lo: 1.0, hi: 1000.0, p0: 0.0, p1: 300.0 }.map(10.0) - 100.0).abs() < 1e-9);
}

#[test]
fn numbers_follow_the_locale() {
    assert_eq!(format_number(0.34, Locale::En), "0.34");
    assert_eq!(format_number(0.34, Locale::De), "0,34");
    assert_eq!(format_number(12345.678, Locale::En), "12\u{2009}346");
    assert_eq!(format_number(-0.0, Locale::En), "0");
    assert_eq!(format_percent(0.625, Locale::De), "62,5\u{202f}%");
}

#[test]
fn lowering_emits_axes_series_marker_legend_in_both_languages() {
    let d = diagram();
    let en = lower_diagram(&d, &options(Locale::En));
    let de = lower_diagram(&d, &options(Locale::De));
    for id in ["grid", "axes", "series:region", "series:region:hatch", "series:curve", "series:ray", "marker:load", "legend:text:curve", "tick-x:100"] {
        assert!(en.layers_json.contains(&format!("\"id\":\"{id}\"")), "{id}");
    }
    assert!(en.layers_json.contains("Interaction") && en.layers_json.contains("Capacity") && en.layers_json.contains("Load 62\u{202f}%"));
    assert!(de.layers_json.contains("Interaktion") && de.layers_json.contains("Tragfähigkeit") && de.layers_json.contains("Last 62\u{202f}%"));
    assert!(en.layers_json.contains("Moment [kNm]"));
    assert!(en.layers_json.starts_with('[') && en.layers_json.ends_with(']'));
}

#[test]
fn lowering_is_deterministic_and_clips_to_the_plot() {
    let d = diagram();
    let a = lower_diagram(&d, &options(Locale::En));
    assert_eq!(a, lower_diagram(&d, &options(Locale::En)));
    let curve = a.targets.iter().find(|t| t.element == "curve").expect("curve");
    let PickShape::Path { points } = &curve.shape else { panic!("path") };
    assert_eq!(points.len(), 3);
    assert!(!a.layers_json.contains("NaN"));
}

#[test]
fn selected_elements_get_heavier_strokes() {
    let d = diagram();
    let plain = lower_diagram(&d, &options(Locale::En));
    let mut o = options(Locale::En);
    o.selected = Some("curve".into());
    assert_ne!(plain.layers_json, lower_diagram(&d, &o).layers_json);
}

#[test]
fn marker_hit_selects_its_check_and_beats_overlapping_series() {
    let d = diagram();
    let lowered = lower_diagram(&d, &options(Locale::En));
    let marker = lowered.targets.iter().find(|t| t.element == "load").expect("marker");
    let PickShape::Point { at, .. } = marker.shape else { panic!("point") };
    let hit = pick(&lowered.targets, [at[0] + 2.0, at[1] - 1.0], 6.0).expect("hit");
    assert_eq!(hit.target.element, "load");
    let selection = selection_for(&d, &hit);
    assert_eq!((selection.check_id.as_deref(), selection.kind), (Some("c1"), ElementKind::Marker));
    assert!(tooltip(&d, &hit, Locale::De).contains("62\u{202f}%"));
    assert!(pick(&lowered.targets, [-50.0, -50.0], 6.0).is_none());
}

#[test]
fn area_is_picked_inside_and_lines_beat_areas() {
    let d = diagram();
    let lowered = lower_diagram(&d, &options(Locale::En));
    let PickShape::Area { ring } = &lowered.targets.iter().find(|t| t.element == "region").expect("region").shape else { panic!("area") };
    let c = [ring.iter().map(|p| p[0]).sum::<f64>() / ring.len() as f64, ring.iter().map(|p| p[1]).sum::<f64>() / ring.len() as f64];
    assert_eq!(pick(&lowered.targets, c, 1.0).map(|h| h.target.element), Some("region".to_string()));
}

#[test]
fn keyboard_focus_order_wraps() {
    let order = focus_order(&diagram());
    assert_eq!(order, ["region", "curve", "ray", "load"]);
    assert_eq!(step_focus(&order, None, 1).as_deref(), Some("region"));
    assert_eq!(step_focus(&order, Some("load"), 1).as_deref(), Some("region"));
    assert_eq!(step_focus(&order, Some("region"), -1).as_deref(), Some("load"));
    assert_eq!(step_focus(&[], None, 1), None);
}

#[test]
fn a_marker_pick_radius_follows_the_device_hit_edge() {
    use crate::device_profiles::{device_profile, PRIORITY};
    assert_eq!(marker_hit_radius(0), 8.0, "the drawn radius plus two keeps its size where the device asks for nothing");
    let desktop = lower_diagram(&diagram(), &options(Locale::En));
    let touch = lower_diagram(&diagram(), &LoweringOptions { min_target_px: device_profile(crate::viewport3d::config::DeviceClass::Mobile).min_target_px, ..options(Locale::En) });
    let radius = |lowered: &Lowered| match lowered.targets.iter().find(|t| t.element == "load").expect("marker").shape {
        PickShape::Point { radius, .. } => radius,
        _ => panic!("point"),
    };
    assert_eq!((radius(&desktop), radius(&touch)), (8.0, 22.0), "a 44 px touch edge is a 22 px radius");
    for class in PRIORITY {
        assert!(marker_hit_radius(device_profile(class).min_target_px) * 2.0 >= f64::from(device_profile(class).min_target_px), "{class:?}: the pick area spans the profile minimum");
    }
    let marker = touch.targets.iter().find(|t| t.element == "load").expect("marker");
    let PickShape::Point { at, .. } = marker.shape else { panic!("point") };
    assert!(pick(&desktop.targets, [at[0] + 15.0, at[1]], 0.0).map_or(true, |hit| hit.target.element != "load"), "15 px away misses the 8 px desktop marker");
    assert_eq!(pick(&touch.targets, [at[0] + 15.0, at[1]], 0.0).expect("a finger 15 px off").target.element, "load");
}

#[test]
fn table_twin_pads_short_columns_and_lists_markers() {
    let t = table_twin(&diagram(), Locale::De);
    assert_eq!(t.headers, ["M [kNm]", "N [kN]"]);
    assert_eq!(t.rows.len(), 3);
    assert_eq!(t.rows[1], ["200", "1500,5"]);
    assert_eq!(t.rows[2][1], "");
    assert_eq!(t.marker_rows[0][3], "62\u{202f}%");
    assert_eq!(t.marker_headers[0], "Markierung");
}

#[test]
fn accessible_description_names_axes_series_and_markers() {
    let en = accessible_description(&diagram(), Locale::En);
    assert!(en.contains("Horizontal axis") && en.contains("Capacity, Curve, Ray") && en.contains("Load (diamond)") && en.contains("utilisation 62\u{202f}%"));
    assert!(accessible_description(&diagram(), Locale::De).contains("Vertikale Achse"));
}

#[test]
fn log_axes_resolve_decade_domains() {
    let mut d = diagram();
    d.axes[0] = axis(AxisScale::Log10, None);
    d.series = vec![Series::Polyline { id: "sn".into(), label: copy("S-N", "S-N"), points: vec![v(2.0e4, 300.0), v(3.0e6, 80.0)], style: Stroke { width_px: 2.0, dash: Dash::Solid, tone: Tone::Neutral } }];
    d.markers.clear();
    assert_eq!(resolve_domain(&d, 0), (1.0e4, 1.0e7));
}

#[test]
fn tones_are_distinct_colour_vision_safe_and_paired_with_patterns() {
    let tones = [Tone::Neutral, Tone::Info, Tone::Warning, Tone::Error];
    for (i, a) in tones.iter().enumerate() {
        for b in &tones[i + 1..] {
            assert_ne!(tone_rgb(*a), tone_rgb(*b));
            assert_ne!(default_hatch(*a), default_hatch(*b));
            assert_ne!(tone_marker(*a), tone_marker(*b));
        }
    }
}

#[test]
fn hatch_lines_stay_inside_the_ring() {
    let ring = [[0.0, 0.0], [20.0, 0.0], [20.0, 20.0], [0.0, 20.0]];
    for h in [Hatch::Diagonal, Hatch::CrossDiagonal, Hatch::Horizontal, Hatch::Dots, Hatch::Vertical] {
        let lines = hatch_lines(&ring, h, 4.0);
        assert!(!lines.is_empty());
        assert!(lines.iter().flatten().all(|p| p[0] >= -1e-9 && p[0] <= 20.0 + 1e-9 && p[1] >= -1e-9 && p[1] <= 20.0 + 1e-9));
    }
}

#[test]
fn directional_hatches_run_along_and_across_the_x_axis() {
    let ring = [[0.0, 0.0], [20.0, 0.0], [20.0, 10.0], [0.0, 10.0]];
    let along = hatch_lines(&ring, Hatch::Horizontal, 2.0);
    let across = hatch_lines(&ring, Hatch::Vertical, 2.0);
    assert!(!along.is_empty() && !across.is_empty());
    assert!(along.iter().all(|[a, b]| (a[1] - b[1]).abs() < 1e-9 && (a[0] - b[0]).abs() > 1.0));
    assert!(across.iter().all(|[a, b]| (a[0] - b[0]).abs() < 1e-9 && (a[1] - b[1]).abs() > 1.0));
}

#[test]
fn golden_layers_match_the_committed_fixture() {
    let layers = lower_diagram(&diagram(), &options(Locale::En)).layers_json;
    let fixture = tests_dir().join("🧫️fixtures/🔣️layers.json");
    if std::env::var_os("SEMIO_UPDATE_GOLDEN").is_some() {
        std::fs::write(&fixture, &layers).expect("write golden");
    }
    assert_eq!(layers, std::fs::read_to_string(&fixture).expect("golden"));
    let parsed: Value = serde_json::from_str(&layers).expect("valid json");
    assert!(parsed.as_array().expect("array").iter().all(|l| l["id"].is_string()));
}
