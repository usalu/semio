use super::testing::close;
use super::*;
use crate::{Axis, ModelDiff, ModelSnapshot, Point2, Profile, TopConstraint, Vertex};
use protocol::OutcomeCode;
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::Point;

fn line(start: (f64, f64), end: (f64, f64)) -> Axis {
    Axis::Line { start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 } }
}

fn arc(start: (f64, f64), end: (f64, f64), bulge: f64) -> Axis {
    Axis::Arc { start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 }, bulge }
}

fn kernel(axis: &Axis) -> BulgeSeg {
    let (start, end) = ends(axis);
    let bulge = if let Axis::Arc { bulge, .. } = axis { *bulge } else { 0.0 };
    BulgeSeg::new(Point::new(start.x, start.y), Point::new(end.x, end.y), bulge)
}

#[semio_framework_async_macros::async_test]
async fn an_axis_needs_finite_ends_length_and_a_real_arc() {
    assert_eq!(flaw(&line((0.0, 0.0), (8.0, 0.0))), None);
    assert_eq!(flaw(&arc((0.0, 0.0), (8.0, 0.0), 0.5)), None);
    assert_eq!(flaw(&arc((0.0, 0.0), (8.0, 0.0), -3.0)), None, "a major clockwise arc is a wall");
    for bad in [line((f64::NAN, 0.0), (8.0, 0.0)), line((0.0, 0.0), (f64::INFINITY, 0.0)), line((1.0, 1.0), (1.0, 1.0)), line((0.0, 0.0), (5e-10, 0.0))] {
        assert_eq!(flaw(&bad).expect("refused").path, Vec::<String>::new(), "{bad:?}");
    }
    for bulge in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, 1e-12, 1e300, -1e300] {
        let flaw = flaw(&arc((0.0, 0.0), (8.0, 0.0), bulge)).unwrap_or_else(|| panic!("bulge {bulge} is refused"));
        assert_eq!((flaw.code, flaw.path), (OutcomeCode::Invariant, vec!["bulge".to_string()]), "bulge {bulge}");
    }
}

#[semio_framework_async_macros::async_test]
async fn flipping_twice_restores_the_axis_and_keeps_the_curve() {
    for axis in [line((0.0, 0.0), (8.0, 3.0)), arc((0.0, 0.0), (8.0, 0.0), 0.5), arc((1.0, 2.0), (4.0, -2.0), -1.7)] {
        let flipped = flipped(&axis);
        assert_ne!(flipped, axis);
        assert_eq!(super::flipped(&flipped), axis);
        assert!(close(length(&flipped), length(&axis)));
        let (forward, backward) = (kernel(&axis), kernel(&flipped));
        assert!(forward.point_at(0.3).distance(backward.point_at(0.7)) < 1e-9, "the same curve is run the other way");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_length_of_an_arc_is_the_kernel_length() {
    for axis in [line((0.0, 0.0), (8.0, 3.0)), arc((0.0, 0.0), (8.0, 0.0), 0.5), arc((1.0, 2.0), (4.0, -2.0), -1.7), arc((0.0, 0.0), (8.0, 0.0), 1.0)] {
        assert!(close(length(&axis), kernel(&axis).length()), "{axis:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_split_agrees_with_the_first_party_bulge_kernel() {
    for axis in [line((0.0, 0.0), (8.0, 3.0)), arc((0.0, 0.0), (8.0, 0.0), 0.5), arc((1.0, 2.0), (4.0, -2.0), -1.7), arc((0.0, 0.0), (8.0, 0.0), 1.0), arc((0.0, 0.0), (2.0, 5.0), 2.5)] {
        for t in [0.1, 0.25, 0.5, 0.8, 0.95] {
            let parts = split(&axis, t).unwrap_or_else(|| panic!("{axis:?} splits at {t}"));
            let (first, second) = kernel(&axis).split_at(t);
            let (a, b) = (kernel(&parts.first), kernel(&parts.second));
            assert!(a.end.distance(first.end) < 1e-8 && b.start.distance(a.end) == 0.0, "{axis:?} at {t}: shared middle point");
            assert!((a.bulge - first.bulge).abs() < 1e-8 && (b.bulge - second.bulge).abs() < 1e-8, "{axis:?} at {t}: sub-arc bulges tan(sweep share / 4)");
            assert!(close(length(&parts.first) + length(&parts.second), length(&axis)) || (length(&parts.first) + length(&parts.second) - length(&axis)).abs() < 1e-8, "{axis:?} at {t}: the parts add up to the axis");
            assert!((parts.at - first.length()).abs() < 1e-8, "{axis:?} at {t}: the first part's arc length");
            assert_eq!((ends(&parts.first).0, ends(&parts.second).1), ends(&axis), "the outer end points are kept exactly");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn a_semicircle_splits_at_its_closed_form_midpoint() {
    let parts = split(&arc((0.0, 0.0), (8.0, 0.0), 1.0), 0.5).expect("splits");
    let (middle, quarter) = (ends(&parts.first).1, (std::f64::consts::PI / 8.0).tan());
    assert_eq!((middle.x, middle.y), (4.0, -4.0));
    assert!(matches!(parts.first, Axis::Arc { bulge, .. } if (bulge - quarter).abs() < 1e-12));
    assert!(matches!(parts.second, Axis::Arc { bulge, .. } if (bulge - quarter).abs() < 1e-12));
}

#[semio_framework_async_macros::async_test]
async fn a_split_is_deterministic_and_snapped_to_a_nanometre() {
    let axis = arc((0.1, 0.2), (7.3, 1.1), 0.37);
    let parts = split(&axis, 0.3333333333333333).expect("splits");
    assert_eq!(split(&axis, 0.3333333333333333), Some(parts.clone()));
    let middle = ends(&parts.first).1;
    assert_eq!((snap(middle.x), snap(middle.y)), (middle.x, middle.y));
    assert_eq!(snap(-0.0), 0.0);
    assert!(snap(-0.0).is_sign_positive());
}

#[semio_framework_async_macros::async_test]
async fn a_split_needs_t_strictly_inside_and_two_valid_parts() {
    let axis = arc((0.0, 0.0), (8.0, 0.0), 0.5);
    for t in [0.0, 1.0, -0.1, 1.5, f64::NAN, f64::INFINITY, 1e-12, 1.0 - 1e-12] {
        assert_eq!(split(&axis, t), None, "t = {t}");
    }
    assert_eq!(split(&line((1.0, 1.0), (1.0, 1.0)), 0.5), None, "an invalid axis splits nowhere");
    assert!(split(&axis, 0.5).is_some());
}

#[semio_framework_async_macros::async_test]
async fn a_mullion_profile_needs_sound_dimensions() {
    let sound = [Profile::Rectangle { width: 0.05, depth: 0.15 }, Profile::Circle { diameter: 0.08 }, Profile::IShape { width: 0.1, depth: 0.2, web: 0.01, flange: 0.02 }, Profile::Custom { outline: vec![Vertex { point: Point2 { x: 0.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 1.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 0.0, y: 1.0 }, bulge: 0.0 }] }];
    for profile in &sound {
        assert_eq!(profile_flaw(profile), None, "{profile:?}");
    }
    let broken = [
        Profile::Rectangle { width: 0.0, depth: 0.15 },
        Profile::Rectangle { width: 0.05, depth: f64::NAN },
        Profile::Circle { diameter: -1.0 },
        Profile::IShape { width: 0.1, depth: 0.2, web: 0.1, flange: 0.02 },
        Profile::IShape { width: 0.1, depth: 0.2, web: 0.01, flange: 0.1 },
        Profile::Custom { outline: vec![Vertex { point: Point2 { x: 0.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 1.0, y: 0.0 }, bulge: 0.0 }] },
        Profile::Custom { outline: vec![Vertex { point: Point2 { x: f64::NAN, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 1.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 0.0, y: 1.0 }, bulge: 0.0 }] },
    ];
    for profile in &broken {
        assert_eq!(profile_flaw(profile).map(|flaw| flaw.code), Some(OutcomeCode::Invariant), "{profile:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn spacings_and_free_tops_need_positive_finite_lengths() {
    let base = ModelSnapshot::default();
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(spacing_flaw("u_spacing", value).map(|flaw| flaw.path), Some(vec!["u_spacing".to_string()]), "{value}");
        assert!(top_flaw(&base, "st", &TopConstraint::Unconnected { height: value }).is_some(), "{value}");
    }
    assert_eq!(spacing_flaw("u_spacing", 1.5), None);
    assert_eq!(top_flaw(&base, "st", &TopConstraint::Unconnected { height: 2.4 }), None);
    assert_eq!(top_flaw(&base, "st", &TopConstraint::StoreyTop { offset: -0.2 }), None);
    assert!(top_flaw(&base, "st", &TopConstraint::StoreyTop { offset: f64::NAN }).is_some());
    assert_eq!(top_flaw(&base, "st", &TopConstraint::Storey { storey: "st-up".into(), offset: 0.0 }).map(|flaw| (flaw.code, flaw.path)), Some((OutcomeCode::TargetMissing, vec!["top".to_string(), "storey".to_string()])));
}

#[semio_framework_async_macros::async_test]
async fn a_flaw_is_seen_from_its_enclosing_field() {
    let flaw = flaw(&arc((0.0, 0.0), (8.0, 0.0), 0.0)).expect("refused").under(&["curtain_wall", "axis"]);
    assert_eq!(flaw.path, ["curtain_wall", "axis", "bulge"]);
    let outcome = flaw.refuse();
    assert_eq!(outcome.diff(), &ModelDiff::default());
}
