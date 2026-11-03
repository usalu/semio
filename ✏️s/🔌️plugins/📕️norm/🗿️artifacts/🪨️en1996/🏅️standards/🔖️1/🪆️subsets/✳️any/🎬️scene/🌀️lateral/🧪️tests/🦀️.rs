use super::*;

#[test]
fn the_cracked_depth_is_zero_inside_the_middle_third_and_three_e_minus_half_t_outside() {
    let t = 0.3;
    assert_eq!(cracked_depth(t, 0.05), 0.0);
    assert_eq!(cracked_depth(t, t / 6.0), 0.0);
    assert!((cracked_depth(t, 0.1) - 0.15).abs() < 1e-12);
    assert!((cracked_depth(t, 0.15) - 0.3).abs() < 1e-12);
    assert!((cracked_depth(t, 0.2) - 0.3).abs() < 1e-12);
}

#[test]
fn the_wall_strip_follows_the_pinned_beam_closed_form() {
    let (h, t, e, w) = (2.8, 0.2, 5.0e9, 600.0);
    let curve = strip_line(h, t, e, w).expect("kernel");
    let ei = e * STRIP_WIDTH * t.powi(3) / 12.0;
    let expected = 5.0 * w * h.powi(4) / (384.0 * ei);
    let middle = curve.iter().map(|(_, u)| u[2].abs()).fold(0.0, f64::max);
    assert!((middle - expected).abs() < 1e-3 * expected, "{middle} vs {expected}");
    let quarter = curve.iter().min_by(|a, b| (a.0 - 0.25).abs().total_cmp(&(b.0 - 0.25).abs())).map(|(_, u)| u[2].abs()).expect("sample");
    assert!((quarter / middle - 0.7125).abs() < 1e-3);
}
