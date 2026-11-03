//! 🧪️ Continuous-beam statics against PyNite (flexibility oracle `🧰️scripts/en1994-continuous-oracle.py`: numpy flexibility method and PyNite agree to 1e-4) and closed forms.

use super::*;

const Q: f64 = 10e3;
const EI1: f64 = 210e9 * 6e-4;

fn rel(a: f64, b: f64) -> f64 {
    (a - b).abs() / b.abs().max(1e-12)
}

fn cracked() -> Stiffness {
    Stiffness { ei1: EI1, ei2: 0.5 * EI1, cracked_fraction: 0.15 }
}

fn uncracked() -> Stiffness {
    Stiffness::uniform(EI1)
}

fn all(layout: &Layout) -> Vec<f64> {
    vec![Q; layout.lengths.len()]
}

#[test]
fn two_equal_spans_reproduce_the_closed_form_coefficients() {
    let layout = Layout::of(SpanPosition::End, 8.0, 0.0);
    let s = solve(&layout, &all(&layout), uncracked());
    assert!(rel(s.supports[1], -Q * 64.0 / 8.0) < 1e-6);
    assert!(rel(s.max_sagging_own(), 9.0 * Q * 64.0 / 128.0) < 1e-4);
    assert!(rel(s.shear(0, 8.0).abs(), 5.0 * Q * 8.0 / 8.0) < 1e-6);
    assert!(rel(s.deflection_own(), 1.7607e-3) < 1e-3);
}

#[test]
fn cracked_stiffness_over_fifteen_percent_lowers_the_support_moment_and_raises_the_deflection() {
    let layout = Layout::of(SpanPosition::End, 8.0, 0.0);
    let s = solve(&layout, &all(&layout), cracked());
    assert!(rel(s.supports[1], -64047.3) < 1e-3);
    assert!(rel(s.deflection_own(), 2.1625e-3) < 1e-3);
}

#[test]
fn end_span_with_an_unequal_neighbour_follows_the_three_moment_equation() {
    let layout = Layout::of(SpanPosition::End, 8.0, 6.0);
    let u = solve(&layout, &all(&layout), uncracked());
    assert!(rel(u.supports[1], -Q * (512.0 + 216.0) / (8.0 * 14.0)) < 1e-6);
    assert!(rel(u.deflection_own(), 2.2077e-3) < 1e-3);
    let c = solve(&layout, &all(&layout), cracked());
    assert!(rel(c.supports[1], -52038.4) < 1e-3);
    assert!(rel(c.deflection_own(), 2.5543e-3) < 1e-3);
}

#[test]
fn interior_span_of_three_spans_matches_pynite() {
    let layout = Layout::of(SpanPosition::Interior, 8.0, 6.0);
    let u = solve(&layout, &all(&layout), uncracked());
    assert!(rel(u.supports[1], -Q * (216.0 + 512.0) / 4.0 / (2.0 * 6.0 + 3.0 * 8.0)) < 1e-6);
    assert!((u.supports[1] - u.supports[2]).abs() < 1e-3);
    assert!(rel(u.deflection_own(), 1.0228e-3) < 1e-3);
    let c = solve(&layout, &all(&layout), cracked());
    assert!(rel(c.supports[1], -42539.0) < 1e-3);
    assert!(rel(c.deflection_own(), 1.4510e-3) < 1e-3);
}

#[test]
fn cantilever_tip_follows_the_neighbour_rotation_like_pynite() {
    let layout = Layout::of(SpanPosition::Cantilever, 3.0, 8.0);
    let u = solve(&layout, &all(&layout), uncracked());
    assert!(rel(u.supports[1], -Q * 9.0 / 2.0) < 1e-9);
    assert!(rel(u.deflection_own(), -1.4187e-3) < 1e-3);
    assert!(rel(solve(&layout, &all(&layout), cracked()).deflection_own(), -0.4883e-3) < 2e-3);
    let pattern = vec![0.0, Q];
    assert!(rel(solve(&layout, &pattern, uncracked()).deflection_own(), 3.6607e-3) < 1e-3);
    assert!(rel(solve(&layout, &pattern, cracked()).deflection_own(), 5.1473e-3) < 1e-3);
}

#[test]
fn single_loaded_end_span_matches_pynite_and_the_pattern_helper() {
    let layout = Layout::of(SpanPosition::End, 8.0, 8.0);
    let s = solve(&layout, &[Q, 0.0], cracked());
    assert!(rel(s.supports[1], -32023.6) < 1e-3);
    assert!(rel(s.deflection_own(), 3.2220e-3) < 1e-3);
    assert!(rel(span_deflection_m(SpanPosition::End, 8.0, 8.0, Q, Pattern::OwnSpan, cracked()), 3.2220e-3) < 1e-3);
}

#[test]
fn simple_span_reproduces_five_qL4_over_384_EI() {
    let w = span_deflection_m(SpanPosition::Simple, 8.0, 0.0, Q, Pattern::AllSpans, uncracked());
    assert!(rel(w, 5.0 * Q * 8.0_f64.powi(4) / (384.0 * EI1)) < 1e-4);
}

#[test]
fn hogging_zone_of_two_equal_spans_lies_between_the_contraflexure_points() {
    let layout = Layout::of(SpanPosition::End, 8.0, 0.0);
    let zones = solve(&layout, &all(&layout), uncracked()).hogging_zones();
    assert_eq!(zones.len(), 1);
    assert!(rel(zones[0].0, 6.0) < 1e-3 && rel(zones[0].1, 10.0) < 1e-3, "{zones:?}");
}

#[test]
fn layout_totals_and_supports_follow_the_span_position() {
    assert_eq!(Layout::of(SpanPosition::Simple, 8.0, 3.0).supports(), vec![0.0, 8.0]);
    assert_eq!(Layout::of(SpanPosition::Interior, 8.0, 6.0).supports(), vec![0.0, 6.0, 14.0, 20.0]);
    let cantilever = Layout::of(SpanPosition::Cantilever, 3.0, 8.0);
    assert_eq!(cantilever.supports(), vec![0.0, 8.0]);
    assert!(rel(cantilever.total(), 11.0) < 1e-12);
}

#[test]
fn profile_peaks_at_one_and_follows_the_span_position() {
    let two = Profile::of(SpanPosition::End, 8.0, 0.0);
    assert!(rel(two.hogging(0.5), 1.0) < 1e-6);
    assert!((0..=200).map(|k| two.sagging(k as f64 / 200.0 * 0.5)).fold(0.0, f64::max) > 0.999);
    assert!(rel((0..=400).map(|k| two.shear(k as f64 / 400.0)).fold(0.0, f64::max), 1.0) < 1e-6);
    let simple = Profile::of(SpanPosition::Simple, 8.0, 0.0);
    assert!(rel(simple.sagging(0.5), 1.0) < 1e-6 && simple.hogging(0.5) == 0.0);
    let tip = Profile::of(SpanPosition::Cantilever, 3.0, 8.0);
    assert!(tip.hogging(8.0 / 11.0) > 0.999 && tip.sagging(0.9) == 0.0);
}

#[test]
fn cracking_lowers_the_support_moment_and_raises_the_span_moment() {
    let (hog, sag) = cracking_ratios(SpanPosition::End, 8.0, 0.0, cracked(), uncracked());
    assert!(rel(hog, 64047.3 / 80000.0) < 1e-3);
    assert!(sag > 1.0 && sag < 1.2);
    assert_eq!(cracking_ratios(SpanPosition::Simple, 8.0, 0.0, cracked(), uncracked()), (1.0, 1.0));
}
