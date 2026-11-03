use super::*;

#[test]
fn influence_ordinates_are_the_textbook_values_of_a_simple_span() {
    assert!((eta_moment(5.0, 10.0) - 2.5).abs() < 1e-12);
    assert!((eta_moment(2.0, 10.0) - 1.0).abs() < 1e-12);
    assert_eq!(eta_moment(-0.1, 10.0), 0.0);
    assert!((eta_reaction(2.5, 10.0) - 0.75).abs() < 1e-12);
    assert_eq!(eta_reaction(10.1, 10.0), 0.0);
}

#[test]
fn the_envelope_matches_the_blessed_third_party_oracle_values() {
    let near = |actual: f64, expected: f64| assert!((actual - expected).abs() <= 1e-9 * expected, "{actual} != {expected}");
    let (m, r) = envelope(600_000.0, 20.0, 1.2);
    near(m, 5_640_000.0);
    near(r, 1_164_000.0);
    let (m, r) = envelope(300_000.0, 1.0, 1.2);
    near(m, 75_000.0);
    near(r, 300_000.0);
    let (m, a) = effects(300_000.0, 10.6, 20.0, 1.2);
    near(m, 300_000.0 * (eta_moment(10.6, 20.0) + eta_moment(9.4, 20.0)));
    near(a, 300_000.0 * (eta_reaction(10.6, 20.0) + eta_reaction(9.4, 20.0)));
}

#[test]
fn the_section_envelope_equals_a_dense_sweep_of_the_influence_lines_and_the_textbook_support_values() {
    let (axle, span, spacing) = (300_000.0, 12.0, 1.2);
    for section in [0.0, 2.4, 3.6, 6.0, 9.6, 12.0] {
        let (m, v_max, v_min) = part_2::tandem_section_envelope(axle, span, spacing, section);
        let (mut sweep_m, mut sweep_max, mut sweep_min) = (0.0_f64, 0.0_f64, 0.0_f64);
        for k in 0..=2_800 {
            let front = k as f64 * 0.005 - 0.2;
            let pair = |effect: &dyn Fn(f64) -> f64| axle * (effect(front) + effect(front - spacing));
            sweep_m = sweep_m.max(pair(&|x| part_2::eta_moment_at(section, x, span)));
            for right in [true, false] {
                let v = pair(&|x| part_2::eta_shear_at(section, x, span, right));
                sweep_max = sweep_max.max(v);
                sweep_min = sweep_min.min(v);
            }
        }
        assert!((m - sweep_m).abs() < 5_000.0 && (v_max - sweep_max).abs() < 5_000.0 && (v_min - sweep_min).abs() < 5_000.0, "section {section}: {m} {v_max} {v_min} against {sweep_m} {sweep_max} {sweep_min}");
    }
    let (m, v_max, v_min) = part_2::tandem_section_envelope(axle, span, spacing, 3.6);
    assert!((m - 1_404_000.0).abs() < 1e-6 && (v_max - 390_000.0).abs() < 1e-6 && (v_min + 150_000.0).abs() < 1e-6, "blessed PyNite values at x = 3.6 m");
    let (m, v_max, v_min) = part_2::tandem_section_envelope(axle, span, spacing, 6.0);
    assert!((m - 1_620_000.0).abs() < 1e-6 && (v_max - 270_000.0).abs() < 1e-6 && (v_min + 270_000.0).abs() < 1e-6, "blessed PyNite values at mid-span");
    let (m, v_max, _) = part_2::tandem_section_envelope(axle, span, spacing, 0.0);
    assert!(m.abs() < 1e-9 && (v_max - 570_000.0).abs() < 1e-6, "the support reaction of the tandem: 2 Q minus the lead share");
    let (m, v, w) = part_2::udl_section_envelope(10_000.0, span, 3.0);
    assert!((m - 135_000.0).abs() < 1e-9 && (v - 10_000.0 * 81.0 / 24.0).abs() < 1e-9 && (w + 10_000.0 * 9.0 / 24.0).abs() < 1e-9);
}

#[test]
fn the_crane_runway_envelope_matches_the_blessed_pynite_values_and_follows_the_document() {
    let (m, v_max, v_min) = part_2::tandem_section_envelope(200_000.0, 30.0, 3.6, 7.5);
    assert!((m - 2_070_000.0).abs() < 1e-6 && (v_max - 276_000.0).abs() < 1e-6 && (v_min + 76_000.0).abs() < 1e-6, "blessed PyNite values at x = 7.5 m");
    let (moment, shear, reaction, section) = part_3::runway_envelope(200_000.0, 30.0, 3.6, 40);
    assert!((moment - 2_640_000.0).abs() < 1e-6 && (section - 15.0).abs() < 1e-9, "mid-span governs the moment: {moment} at {section}");
    assert!((reaction - (400_000.0 - 200_000.0 * 3.6 / 30.0)).abs() < 1e-6 && (shear - reaction).abs() < 1e-6, "end reaction with both wheels on the span: {reaction}");
    let mut d = En1991Snapshot::default();
    assert!(crane_envelope_diagrams(&d).is_empty(), "a crane that is not claimed has no envelope");
    d.crane_claimed = true;
    let ids: Vec<String> = crane_envelope_diagrams(&d).into_iter().map(|diagram| diagram.id).collect();
    assert_eq!(ids, vec!["crane:envelope-moment", "crane:envelope-shear"]);
    let peak = |d: &En1991Snapshot| crane_envelope_diagrams(d)[0].markers[0].at.y;
    let before = peak(&d);
    d.crane_runway_length = 40.0;
    assert!(peak(&d) > before, "a longer runway raises the largest moment");
}
