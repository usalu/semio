use super::*;

#[test]
fn drift_triangles_raise_the_base_coefficient_linearly() {
    let rising = Drift { start: 6.0, length: 4.0, peak: 2.0, rising: true };
    let falling = Drift { start: 1.0, length: 2.0, peak: 1.6, rising: false };
    let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} != {b}");
    near(mu_at(0.8, &[rising, falling], 5.0), 0.8);
    near(mu_at(0.8, &[rising, falling], 8.0), 1.4);
    near(mu_at(0.8, &[rising, falling], 10.0), 2.0);
    near(mu_at(0.8, &[rising, falling], 1.0), 1.6);
    near(mu_at(0.8, &[rising, falling], 2.0), 1.2);
    near(mu_at(0.8, &[Drift { start: 0.0, length: 1.0, peak: 0.5, rising: true }], 1.0), 0.8);
}

#[test]
fn samples_cover_the_section_from_zero_to_one() {
    let s = samples(10.0, 5, 1.0, &[], |mu| mu * 1000.0);
    assert_eq!(s.len(), 6);
    assert_eq!((s[0].at, s[5].at), (0.0, 1.0));
    assert!(s.iter().all(|p| (p.value.value - 1000.0).abs() < 1e-9));
}
