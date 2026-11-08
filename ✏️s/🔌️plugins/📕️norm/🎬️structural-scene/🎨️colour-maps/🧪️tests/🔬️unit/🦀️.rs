use super::metrics::*;
use super::*;
use serde_json::{json, Value};
use std::io::Write;
use std::process::Stdio;

fn oracle(case: &Value) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🎨️colour-maps/🔮️oracles/🐍️.py");
    let mut child = semio_s_plugin_norm_compliance::python_oracle::command().arg(path).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("python oracle");
    child.stdin.take().expect("stdin").write_all(case.to_string().as_bytes()).expect("write");
    let out = child.wait_with_output().expect("oracle");
    assert!(out.status.success(), "oracle failed");
    serde_json::from_slice(&out.stdout).expect("json")
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-9 * b.abs().max(1.0), "{a} != {b}");
}

fn f(v: &Value) -> f64 {
    v.as_f64().expect("number")
}

fn all() -> Vec<Ramp> {
    ALL_VARIANTS.into_iter().flat_map(|v| ALL_RAMPS.into_iter().map(move |r| ramp(r, v))).collect()
}

fn n_of(r: &Ramp) -> usize {
    if r.kind == MapKind::Failure { 5 } else { 9 }
}

fn case() -> Value {
    let ramps: Vec<Value> = all().iter().map(|r| json!({"stops": r.stops.iter().map(|(t, c)| json!([t, c])).collect::<Vec<_>>(), "bg": r.variant.background(), "n": n_of(r)})).collect();
    let pairs: Vec<Value> = ALL_VARIANTS.into_iter().map(|v| json!([ramp(RampId::Cool, v).sample(1.0), ramp(RampId::Failure, v).sample(0.0)])).collect();
    json!({"ramps": ramps, "pairs": pairs})
}

fn centre_exempt(r: &Ramp, i: usize) -> bool {
    r.kind == MapKind::Diverging && (i as f64 / (n_of(r) - 1) as f64 - 0.5).abs() < 0.2
}

#[test]
fn samples_and_metrics_match_the_python_oracle() {
    let o = oracle(&case());
    near(f(&o["white_black"]), contrast_ratio([255.0; 3], [0.0; 3]));
    for (r, e) in all().iter().zip(o["ramps"].as_array().expect("ramps")) {
        let samples = r.samples(n_of(&r));
        for (s, p) in samples.iter().zip(e["samples"].as_array().expect("samples")) {
            let p: Vec<u8> = p.as_array().expect("rgb").iter().map(|c| c.as_u64().expect("u8") as u8).collect();
            assert_eq!(s.to_vec(), p, "{:?} {:?}", r.id, r.variant);
        }
        for (s, c) in samples.iter().zip(e["contrast"].as_array().expect("contrast")) {
            near(contrast_ratio(to_srgb(*s), to_srgb(r.variant.background())), f(c));
        }
        for cvd in ALL_CVD {
            near(min_adjacent_delta_e(cvd, &samples), f(&e["adjacent"][cvd.id()]));
            near(delta_e(simulate(cvd, to_srgb(samples[0])), simulate(cvd, to_srgb(samples[samples.len() - 1]))), f(&e["ends"][cvd.id()]));
        }
    }
    for (v, e) in ALL_VARIANTS.into_iter().zip(o["pairs"].as_array().expect("pairs")) {
        for cvd in ALL_CVD {
            near(delta_e(simulate(cvd, to_srgb(ramp(RampId::Cool, v).sample(1.0))), simulate(cvd, to_srgb(ramp(RampId::Failure, v).sample(0.0)))), f(&e[cvd.id()]));
        }
    }
}

#[test]
fn every_ramp_meets_contrast_and_cvd_thresholds() {
    for r in all() {
        let samples = r.samples(n_of(&r));
        for (i, s) in samples.iter().enumerate().filter(|(i, _)| !centre_exempt(&r, *i)) {
            assert!(contrast_ratio(to_srgb(*s), to_srgb(r.variant.background())) >= r.variant.contrast_floor(), "{:?} {:?} sample {i}", r.id, r.variant);
        }
        for cvd in ALL_CVD {
            assert!(min_adjacent_delta_e(cvd, &samples) >= 2.0, "{:?} {:?} {cvd:?} adjacent", r.id, r.variant);
            let ends = delta_e(simulate(cvd, to_srgb(samples[0])), simulate(cvd, to_srgb(samples[samples.len() - 1])));
            assert!(ends >= if r.kind == MapKind::Failure { 15.0 } else { 30.0 }, "{:?} {:?} {cvd:?} ends {ends}", r.id, r.variant);
        }
    }
}

#[test]
fn failure_side_breaks_from_the_safe_side() {
    for v in ALL_VARIANTS {
        let safe = utilisation_cue(v, 1.0, CheckStatus::Pass, true, DEFAULT_DOMAIN_MAX);
        let fail = utilisation_cue(v, 1.0001, CheckStatus::Pass, true, DEFAULT_DOMAIN_MAX);
        assert_eq!((safe.status, safe.hatch), (CheckStatus::Warning, Some(Hatch::Horizontal)));
        assert_eq!((fail.status, fail.hatch, fail.glyph), (CheckStatus::Fail, Some(Hatch::Diagonal), "✗"));
        assert_eq!(utilisation_rgb(v, 1.0, 1.2), ramp(RampId::Cool, v).sample(1.0));
        assert_eq!(utilisation_rgb(v, 1.0001, 1.2), ramp(RampId::Failure, v).sample(0.0005));
        for cvd in ALL_CVD {
            let d = delta_e(simulate(cvd, to_srgb(ramp(RampId::Cool, v).sample(1.0))), simulate(cvd, to_srgb(ramp(RampId::Failure, v).sample(0.0))));
            assert!(d >= 15.0, "{v:?} {cvd:?} {d}");
        }
        assert_eq!(utilisation_rgb(v, 2.0, 1.2), utilisation_rgb(v, 1.2, 1.2));
        assert_eq!(utilisation_rgb(v, f64::NAN, 1.2), rgb(variant_neutral(v)).into_array());
    }
}

trait IntoArray {
    fn into_array(self) -> [u8; 3];
}

impl IntoArray for Rgb {
    fn into_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

#[test]
fn colour_is_never_the_only_channel() {
    let mut seen = Vec::new();
    for status in [CheckStatus::Pass, CheckStatus::Warning, CheckStatus::Fail, CheckStatus::NotApplicable] {
        for verified in [true, false] {
            let ratio = match status { CheckStatus::Pass => 0.5, CheckStatus::Warning => 0.95, CheckStatus::Fail => 1.1, CheckStatus::NotApplicable => 0.0 };
            let cue = utilisation_cue(Variant::Light, ratio, status, verified, DEFAULT_DOMAIN_MAX);
            assert!(!cue.label.en.is_empty() && !cue.label.de.is_empty());
            assert_eq!(cue.verified, verified);
            if !verified && status != CheckStatus::NotApplicable {
                assert_eq!(cue.hatch, Some(Hatch::Dots));
                assert!(cue.glyph.ends_with('?'));
            }
            let key = (cue.glyph, cue.hatch.map(|h| format!("{h:?}")));
            if status == CheckStatus::NotApplicable && !verified { continue; }
            assert!(!seen.contains(&key), "duplicate cue {key:?}");
            seen.push(key);
        }
    }
    assert_eq!(seen.len(), 7);
}

#[test]
fn scalar_fields_pick_the_right_ramp_and_centre_diverging_at_zero() {
    for v in ALL_VARIANTS {
        let mid = scalar_rgb(ResultField::Stress, v, 0.0, -50.0, 200.0);
        assert_eq!(mid, ramp(RampId::Diverging, v).sample(0.5));
        assert_eq!(scalar_rgb(ResultField::Stress, v, 200.0, -50.0, 200.0), ramp(RampId::Diverging, v).sample(1.0));
        assert_eq!(scalar_rgb(ResultField::Temperature, v, 20.0, 20.0, 800.0), ramp(RampId::Warm, v).sample(0.0));
        assert_eq!(scalar_rgb(ResultField::Settlement, v, 30.0, 0.0, 30.0), ramp(RampId::Cool, v).sample(1.0));
        assert_eq!(scalar_rgb(ResultField::of(QuantityKind::VentilationRate).expect("airflow"), v, 1.0, 0.0, 1.0), ramp(RampId::Cool, v).sample(1.0));
        assert_eq!(scalar_rgb(ResultField::of(QuantityKind::Power).expect("heat"), v, 0.0, 0.0, 1.0), ramp(RampId::Warm, v).sample(0.0));
        assert_eq!(ResultField::of(QuantityKind::Length), None);
        let l = legend(ResultField::Pressure, v, -1.0, 3.0, 5);
        assert_eq!(l.len(), 5);
        assert_eq!((l[0].0, l[4].0), (-3.0, 3.0));
    }
    assert_eq!(RampId::parse("warm"), Some(RampId::Warm));
    assert_eq!(Variant::parse("highContrast"), Some(Variant::HighContrast));
    assert_eq!(Variant::parse("sepia"), None);
}

const SHARMA: [([f64; 3], [f64; 3], f64); 6] = [
    ([50.0, 2.6772, -79.7751], [50.0, 0.0, -82.7485], 2.0425),
    ([50.0, 3.1571, -77.2803], [50.0, 0.0, -82.7485], 2.8615),
    ([50.0, 2.5, 0.0], [50.0, 0.0, -2.5], 4.3065),
    ([50.0, 2.5, 0.0], [73.0, 25.0, -18.0], 27.1492),
    ([50.0, 2.5, 0.0], [61.0, -5.0, 29.0], 22.8977),
    ([50.0, 2.5, 0.0], [56.0, -27.0, -3.0], 31.9030),
];

fn status_palette(v: Variant) -> [[u8; 3]; 4] {
    let cue = |ratio: f64, status: CheckStatus| utilisation_cue(v, ratio, status, true, DEFAULT_DOMAIN_MAX).rgb.into_array();
    [cue(0.3, CheckStatus::Pass), cue(0.95, CheckStatus::Warning), cue(1.000_001, CheckStatus::Fail), cue(0.0, CheckStatus::NotApplicable)]
}

#[test]
fn ciede2000_equals_the_published_reference_pairs_and_the_python_oracle() {
    let pairs: Vec<Value> = SHARMA.iter().map(|(a, b, _)| json!([a, b])).collect();
    let o = oracle(&json!({"ramps": [], "pairs": [], "lab_pairs": pairs}));
    for ((a, b, expected), seen) in SHARMA.iter().zip(o["ciede2000"].as_array().expect("ciede2000")) {
        assert!((delta_e2000_lab(*a, *b) - expected).abs() < 5e-5, "{a:?} {b:?}");
        near(delta_e2000_lab(*a, *b), f(seen));
        near(delta_e2000_lab(*b, *a), delta_e2000_lab(*a, *b));
    }
    assert_eq!(delta_e2000_lab([50.0, 10.0, 10.0], [50.0, 10.0, 10.0]), 0.0);
}

#[test]
fn status_colours_stay_distinguishable_under_protan_deutan_and_tritan_simulation() {
    let palettes: Vec<Value> = ALL_VARIANTS.into_iter().map(|v| json!(status_palette(v))).collect();
    let o = oracle(&json!({"ramps": [], "pairs": [], "palettes": palettes}));
    for (v, expected) in ALL_VARIANTS.into_iter().zip(o["palettes"].as_array().expect("palettes")) {
        let p = status_palette(v);
        for cvd in ALL_CVD {
            near(min_pairwise_delta_e2000(cvd, &p), f(&expected[cvd.id()]));
            assert!(min_pairwise_delta_e2000(cvd, &p) >= 5.0, "{v:?} {cvd:?} all pairs");
            assert!(min_pairwise_delta_e2000(cvd, &[p[0], p[2]]) >= 20.0, "{v:?} {cvd:?} pass against fail");
            assert!(min_pairwise_delta_e2000(cvd, &[p[1], p[2]]) >= 16.0, "{v:?} {cvd:?} warning against fail");
        }
    }
}

#[test]
fn utilisation_ramps_keep_their_ends_apart_in_ciede2000_under_every_deficiency() {
    for v in ALL_VARIANTS {
        let (safe, failing) = (ramp(RampId::Cool, v).samples(9), ramp(RampId::Failure, v).samples(5));
        for cvd in ALL_CVD {
            assert!(min_pairwise_delta_e2000(cvd, &[safe[0], safe[8]]) >= 15.0, "{v:?} {cvd:?} safe ramp");
            assert!(min_pairwise_delta_e2000(cvd, &[safe[8], failing[0]]) >= 10.0, "{v:?} {cvd:?} break at one");
        }
    }
}
