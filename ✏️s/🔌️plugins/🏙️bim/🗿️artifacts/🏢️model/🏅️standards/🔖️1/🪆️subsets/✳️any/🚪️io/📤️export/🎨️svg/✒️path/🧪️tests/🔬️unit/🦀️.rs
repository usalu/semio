use super::*;

const ORIGIN: Frame = Frame { min_x: 0.0, max_y: 0.0, left: 0.0, top: 0.0, mm: 10.0 };

fn vertex(x: f64, y: f64, bulge: f64) -> PlanVertex {
    PlanVertex { x, y, bulge }
}

fn data(vertices: &[PlanVertex], closed: bool, frame: &Frame) -> String {
    commands(vertices, closed, frame)
        .iter()
        .map(|command| match command {
            PathCommand::MoveTo { x, y, .. } => format!("M{x} {y}"),
            PathCommand::LineTo { x, y, .. } => format!("L{x} {y}"),
            PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, .. } => format!("A{rx} {ry} {x_axis_rotation} {} {} {x} {y}", u8::from(*large_arc), u8::from(*sweep)),
            PathCommand::ClosePath => "Z".to_string(),
            other => panic!("{other:?}"),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn coordinates_snap_to_a_micrometre_of_paper_without_negative_zero() {
    assert_eq!([1.0, 0.5, -0.0004, 1.23456, -2.5, 100.0, 0.0].map(snap), [1.0, 0.5, 0.0, 1.235, -2.5, 100.0, 0.0]);
    assert_eq!(snap(-0.0), 0.0);
    assert!(snap(-0.0).is_sign_positive());
}

#[test]
fn the_frame_scales_to_paper_millimetres_and_flips_north_up() {
    assert_eq!([100, 50, 20, 200].map(mm_per_metre), [10.0, 20.0, 50.0, 5.0]);
    assert_eq!(mm_per_metre(0), 1000.0);
    let frame = Frame { min_x: 2.0, max_y: 5.0, left: 8.0, top: 16.0, mm: 10.0 };
    assert_eq!(frame.point(2.0, 5.0), (8.0, 16.0));
    assert_eq!(frame.point(3.0, 4.0), (18.0, 26.0));
    assert_eq!(Frame { mm: 20.0, ..frame }.point(3.0, 4.0), (28.0, 36.0));
}

#[test]
fn straight_paths_are_move_and_line_commands_and_closed_ones_end_in_z() {
    let corners = [vertex(0.0, 0.0, 0.0), vertex(1.0, 0.0, 0.0), vertex(1.0, 1.0, 0.0)];
    let frame = Frame { min_x: 0.0, max_y: 1.0, left: 0.0, top: 0.0, mm: 10.0 };
    assert_eq!(data(&corners, false, &frame), "M0 10 L10 10 L10 0");
    assert_eq!(data(&corners, true, &frame), "M0 10 L10 10 L10 0 Z");
}

#[test]
fn a_bulge_becomes_an_arc_whose_flags_follow_its_sign_and_size() {
    let chord = |bulge: f64| data(&[vertex(0.0, 0.0, bulge), vertex(1.0, 0.0, 0.0)], false, &ORIGIN);
    assert_eq!(chord(1.0), "M0 0 A5 5 0 0 0 10 0");
    assert_eq!(chord(-1.0), "M0 0 A5 5 0 0 1 10 0");
    assert_eq!(chord(2.0), "M0 0 A6.25 6.25 0 1 0 10 0");
    assert_eq!(chord(-2.0), "M0 0 A6.25 6.25 0 1 1 10 0");
    assert_eq!(chord(0.5), "M0 0 A6.25 6.25 0 0 0 10 0");
}

#[test]
fn the_closing_segment_of_a_ring_uses_the_bulge_of_the_last_vertex() {
    let ring = [vertex(0.0, 0.0, 0.0), vertex(1.0, 0.0, 0.0), vertex(1.0, 1.0, 1.0)];
    let frame = Frame { min_x: 0.0, max_y: 1.0, left: 0.0, top: 0.0, mm: 10.0 };
    assert!(data(&ring, true, &frame).ends_with("A7.071068 7.071068 0 0 0 0 10 Z"), "{}", data(&ring, true, &frame));
}

fn endpoint_arc_midpoint(from: (f64, f64), to: (f64, f64), radius: f64, large: bool, sweep: bool) -> (f64, f64) {
    let (dx, dy) = ((from.0 - to.0) / 2.0, (from.1 - to.1) / 2.0);
    let distance = dx * dx + dy * dy;
    let sign = if large == sweep { -1.0 } else { 1.0 };
    let coefficient = sign * ((radius * radius - distance).max(0.0) / distance).sqrt();
    let (cx, cy) = (coefficient * dy + (from.0 + to.0) / 2.0, -coefficient * dx + (from.1 + to.1) / 2.0);
    let start = (dy - (cy - (from.1 + to.1) / 2.0)).atan2(dx - (cx - (from.0 + to.0) / 2.0));
    let end = (-dy - (cy - (from.1 + to.1) / 2.0)).atan2(-dx - (cx - (from.0 + to.0) / 2.0));
    let mut delta = end - start;
    if !sweep && delta > 0.0 {
        delta -= std::f64::consts::TAU;
    }
    if sweep && delta < 0.0 {
        delta += std::f64::consts::TAU;
    }
    let middle = start + delta / 2.0;
    (cx + radius * middle.cos(), cy + radius * middle.sin())
}

#[test]
fn the_arc_flags_reproduce_the_midpoint_of_the_bulge_by_the_svg_endpoint_conversion() {
    let chords = [((0.0, 0.0), (1.0, 0.0)), ((1.0, 1.0), (0.2, 2.5)), ((-3.0, 2.0), (-3.5, -1.0))];
    for ((a, b), bulge) in chords.iter().flat_map(|chord| [0.3, 1.0, 2.0, -0.3, -1.0, -2.0].map(|bulge| (*chord, bulge))) {
        let text = data(&[vertex(a.0, a.1, bulge), vertex(b.0, b.1, 0.0)], false, &ORIGIN);
        let arc: Vec<&str> = text.split(" A").nth(1).expect("an arc").split(' ').collect();
        let (radius, large, sweep) = (arc[0].parse::<f64>().unwrap(), arc[3] == "1", arc[4] == "1");
        let (start, end) = (ORIGIN.point(a.0, a.1), ORIGIN.point(b.0, b.1));
        let got = endpoint_arc_midpoint(start, end, radius, large, sweep);
        let chord = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
        let right = ((b.1 - a.1) / chord, -(b.0 - a.0) / chord);
        let sagitta = bulge * chord / 2.0;
        let expected = ORIGIN.point((a.0 + b.0) / 2.0 + right.0 * sagitta, (a.1 + b.1) / 2.0 + right.1 * sagitta);
        assert!((got.0 - expected.0).abs() < 1e-2 && (got.1 - expected.1).abs() < 1e-2, "bulge {bulge} on {a:?}->{b:?}: {got:?} vs {expected:?} ({text})");
    }
}

#[test]
fn arcs_are_detected_only_on_the_segments_that_are_drawn() {
    let open = [vertex(0.0, 0.0, 0.0), vertex(1.0, 0.0, 0.7)];
    assert!(!has_arc(&open, false) && has_arc(&open, true));
    assert!(is_arc(&vertex(0.0, 0.0, -0.1)) && !is_arc(&vertex(0.0, 0.0, 0.0)));
}

#[test]
fn rings_and_paths_measure_like_the_shoelace_and_the_euclidean_length() {
    let square = [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)];
    assert_eq!(ring_area(&square), 12.0);
    assert_eq!(ring_area(&[(0.0, 3.0), (4.0, 3.0), (4.0, 0.0), (0.0, 0.0)]), 12.0);
    assert_eq!(path_length(&square, true), 14.0);
    assert_eq!(path_length(&square, false), 11.0);
}
