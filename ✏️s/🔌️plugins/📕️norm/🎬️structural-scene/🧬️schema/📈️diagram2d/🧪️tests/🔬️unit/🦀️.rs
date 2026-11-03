use crate::*;
use value::{FromValue, ToValue};

fn copy(s: &str) -> LocalizedCopy {
    LocalizedCopy::new(s, format!("{s}-de"))
}

fn stroke() -> Stroke {
    Stroke { width_px: 2.0, dash: Dash::Solid, tone: Tone::Info }
}

fn axis(scale: AxisScale) -> Axis {
    Axis { label: copy("axis"), quantity: QuantityKind::Force, unit: "kN".into(), scale, domain: Some(Interval::new(1.0, 100.0)), ticks: TickRule::Auto }
}

fn diagram() -> Diagram2d {
    let points = vec![Vec2::new(1.0, 2.0), Vec2::new(10.0, 20.0), Vec2::new(100.0, 40.0)];
    Diagram2d {
        id: "mn".into(),
        title: copy("M-N"),
        description: copy("Interaction curve"),
        axes: vec![axis(AxisScale::Linear), axis(AxisScale::Linear)],
        series: vec![
            Series::Polyline { id: "curve".into(), label: copy("curve"), points: points.clone(), style: stroke() },
            Series::Polygon { id: "region".into(), label: copy("region"), ring: points.clone(), fill: Fill { tone: Tone::Neutral, opacity: 0.4 }, hatch: Some(Hatch::Diagonal) },
            Series::Band { id: "band".into(), label: copy("band"), lower: points.clone(), upper: points.clone(), fill: Fill { tone: Tone::Info, opacity: 0.2 } },
            Series::Scatter { id: "demand".into(), label: copy("demand"), points: vec![ScatterPoint { at: Vec2::new(5.0, 5.0), tone: Tone::Warning, label: None }] },
            Series::Limit { id: "u1".into(), label: copy("U = 1"), axis: AxisRef::Y, value: 1.0, style: stroke() },
            Series::Ray { id: "ray".into(), label: copy("ray"), origin: Vec2::ZERO, through: Vec2::new(5.0, 5.0), to_boundary: true, style: stroke() },
            Series::Section {
                id: "stress".into(),
                label: copy("stress"),
                shape: SectionShape::parametric(ProfileKind::Rect, &[("h", 0.5), ("b", 0.3)]),
                strain: None,
                stress: Some(SectionProfile { points: vec![Vec2::new(0.0, 0.0), Vec2::new(0.2, -1.0)], unit: "MPa".into() }),
                neutral_axis: Some(Line2 { origin: Vec2::ZERO, direction: Vec2::new(1.0, 0.0) }),
            },
            Series::Surface3 { id: "surface".into(), label: copy("surface"), mesh: DiagramMesh { vertices: vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)], triangles: vec![Triangle { a: 0, b: 1, c: 2 }] } },
        ],
        markers: vec![Marker { id: "tb".into(), at: Vec2::new(1.0, 2.0), shape: MarkerShape::Diamond, tone: Tone::Info, label: copy("T_B"), utilisation: Some(0.8), bound_check: Some("c1".into()), binding: Some(FieldBinding::new("seismic.tb", Affine::IDENTITY, "s")) }],
        annotations: vec![DiagramNote { id: "note".into(), at: Vec2::new(2.0, 2.0), text: copy("note"), tone: Tone::Neutral }],
        focus: Some(EntityRef::new(EntityKind::Member, "b1", "members[id=b1]")),
        legend: true,
        table_twin: DataTable { columns: vec![DataColumn { header: copy("N"), unit: "kN".into(), values: vec![1.0, 2.0] }, DataColumn { header: copy("M"), unit: "kN·m".into(), values: vec![3.0, 4.0] }] },
    }
}

fn rules(d: &Diagram2d) -> Vec<(String, String)> {
    d.validate().err().unwrap_or_default().into_iter().map(|v| (v.path, v.rule)).collect()
}

#[test]
fn a_complete_diagram_is_valid_and_round_trips_through_the_value_codec() {
    let d = diagram();
    assert_eq!(d.validate(), Ok(()));
    let back = Diagram2d::from_value(d.to_value()).expect("decode");
    assert_eq!(back, d);
    assert_eq!(canonical_json(&back), canonical_json(&d));
}

#[test]
fn exactly_two_axes_and_unique_ids_are_required() {
    let mut d = diagram();
    d.axes.pop();
    d.markers[0].id = "curve".into();
    let found = rules(&d);
    assert!(found.contains(&("$.axes".into(), "exactly-two".into())));
    assert!(found.contains(&("$.series".into(), "unique-ids".into())));
}

#[test]
fn log_axes_reject_non_positive_data_and_domains() {
    let mut d = diagram();
    d.axes[0] = Axis { domain: Some(Interval::new(0.0, 10.0)), ..axis(AxisScale::Log10) };
    d.series[0] = Series::Polyline { id: "curve".into(), label: copy("c"), points: vec![Vec2::new(-1.0, 1.0), Vec2::new(2.0, 2.0)], style: stroke() };
    let found = rules(&d);
    assert!(found.contains(&("$.axes[0].domain".into(), "positive-on-log-axis".into())));
    assert!(found.contains(&("$.series[0].points[0]".into(), "positive-on-log-axis".into())));
}

#[test]
fn the_table_twin_columns_must_have_equal_length() {
    let mut d = diagram();
    d.table_twin.columns[1].values.pop();
    assert!(rules(&d).contains(&("$.tableTwin.columns".into(), "equal-length".into())));
    assert_eq!(diagram().table_twin.rows(), 2);
}

#[test]
fn degenerate_series_are_refused() {
    let mut d = diagram();
    d.series[1] = Series::Polygon { id: "region".into(), label: copy("r"), ring: vec![Vec2::ZERO, Vec2::new(1.0, 0.0)], fill: Fill { tone: Tone::Neutral, opacity: 2.0 }, hatch: None };
    d.series[5] = Series::Ray { id: "ray".into(), label: copy("r"), origin: Vec2::ZERO, through: Vec2::ZERO, to_boundary: false, style: stroke() };
    d.series[7] = Series::Surface3 { id: "surface".into(), label: copy("s"), mesh: DiagramMesh { vertices: vec![Vec3::ZERO], triangles: vec![Triangle { a: 0, b: 1, c: 2 }] } };
    let found = rules(&d);
    assert!(found.contains(&("$.series[1].ring".into(), "at-least-three".into())));
    assert!(found.contains(&("$.series[1].fill.opacity".into(), "unit-interval".into())));
    assert!(found.contains(&("$.series[5].through".into(), "distinct-from-origin".into())));
    assert!(found.contains(&("$.series[7].mesh.triangles[0]".into(), "index-in-range".into())));
}

#[test]
fn every_label_must_be_bilingual() {
    let mut d = diagram();
    d.title = LocalizedCopy::new("M-N", "");
    assert!(rules(&d).contains(&("$.title.de".into(), "non-empty".into())));
}

#[test]
fn tick_rules_validate_their_parameters() {
    assert!(TickRule::Step { step: 0.0 }.validate().is_err());
    assert!(TickRule::Explicit { values: vec![2.0, 1.0] }.validate().is_err());
    assert_eq!(TickRule::Explicit { values: vec![1.0, 2.0] }.validate(), Ok(()));
}

#[test]
fn wire_form_is_camel_case_with_adjacent_variant_tags() {
    let json = canonical_json(&Series::Limit { id: "u1".into(), label: copy("l"), axis: AxisRef::X, value: 1.0, style: stroke() });
    assert!(json.contains("\"kind\":\"limit\"") && json.contains("\"widthPx\":2.0") && json.contains("\"data\""));
}

#[test]
fn reference_markers_and_the_basis_note_are_read_from_their_ids() {
    let mut d = diagram();
    let marker = |id: &str| Marker { id: id.into(), at: Vec2::ZERO, shape: MarkerShape::Circle, tone: Tone::Neutral, label: copy("m"), utilisation: None, bound_check: None, binding: None };
    assert!(marker("reference:corner-tb").is_reference() && !marker("corner-tb").is_reference() && !marker("referenced:x").is_reference());
    assert!(d.basis().is_none());
    d.annotations.push(DiagramNote { id: BASIS_NOTE_ID.into(), at: Vec2::ZERO, text: copy("basis"), tone: Tone::Neutral });
    d.annotations.push(DiagramNote { id: "note:a".into(), at: Vec2::ZERO, text: copy("a"), tone: Tone::Neutral });
    assert_eq!(d.basis().map(|b| b.en.as_str()), Some("basis"));
    assert_eq!(d.plotted_notes().map(|n| n.id.as_str()).collect::<Vec<_>>(), vec!["note:a"]);
}
