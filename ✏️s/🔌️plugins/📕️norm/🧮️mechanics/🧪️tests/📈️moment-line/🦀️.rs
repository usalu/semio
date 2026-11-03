use crate::moment_line::*;
use semio_s_plugin_norm_compliance::python_oracle;
use std::collections::BTreeMap;
use std::fmt::Write;

fn close(actual: f64, expected: f64, scale: f64, what: &str) {
    assert!((actual - expected).abs() <= 1e-9 * scale.max(1.0), "{what}: {actual} vs {expected} (scale {scale})");
}

fn udl(w: f64) -> SpanLoad {
    SpanLoad { udl: w, points: Vec::new() }
}

fn point(at: f64, force: f64) -> SpanLoad {
    SpanLoad { udl: 0.0, points: vec![PointLoad { at, force }] }
}

#[test]
fn single_cantilever_and_continuous_beams_reproduce_the_textbook_closed_forms() {
    let (w, l, p) = (12_000.0, 5.0, 30_000.0);
    let single = MomentLine::single(l, udl(w)).unwrap();
    close(single.moment_at(l / 2.0), w * l * l / 8.0, w * l * l, "simply supported udl midspan");
    close(single.extremes(0).1, w * l * l / 8.0, w * l * l, "simply supported udl peak");
    assert_eq!(single.reactions(), vec![w * l / 2.0, w * l / 2.0]);
    let centre = MomentLine::single(l, point(l / 2.0, p)).unwrap();
    close(centre.moment_at(l / 2.0), p * l / 4.0, p * l, "simply supported point midspan");
    close(centre.moment_at(l / 4.0), p * l / 8.0, p * l, "simply supported point quarter");
    close(centre.shear_at(1.0), p / 2.0, p, "shear left of the point load");
    close(centre.shear_at(4.0), -p / 2.0, p, "shear right of the point load");
    let cantilever = MomentLine::cantilever(l, udl(w)).unwrap();
    close(cantilever.moment_at(0.0), -w * l * l / 2.0, w * l * l, "cantilever udl root");
    close(cantilever.moment_at(l / 2.0), -w * l * l / 8.0, w * l * l, "cantilever udl midway");
    close(cantilever.moment_at(l), 0.0, w * l * l, "cantilever free end");
    assert_eq!(cantilever.reactions(), vec![w * l, 0.0]);
    let tip = MomentLine::cantilever(l, point(l - 1e-9, p)).unwrap();
    close(tip.moment_at(0.0), -p * (l - 1e-9), p * l, "cantilever tip load root");
    let two = MomentLine::continuous(&[l, l], &[udl(w), udl(w)], false).unwrap();
    close(two.support_moment(1), -w * l * l / 8.0, w * l * l, "two equal spans over the middle support");
    close(two.reactions()[0], 3.0 * w * l / 8.0, w * l, "two equal spans end reaction");
    close(two.reactions()[1], 10.0 * w * l / 8.0, w * l, "two equal spans middle reaction");
    close(two.extremes(0).1, 9.0 * w * l * l / 128.0, w * l * l, "two equal spans sagging peak 9/128");
    let propped = MomentLine::continuous(&[l], &[udl(w)], true).unwrap();
    close(propped.support_moment(0), -w * l * l / 8.0, w * l * l, "propped cantilever root");
    close(propped.reactions()[0], 5.0 * w * l / 8.0, w * l, "propped cantilever root reaction");
}

#[test]
fn refusals_name_the_defect() {
    assert!(MomentLine::single(0.0, udl(1.0)).is_err());
    assert!(MomentLine::continuous(&[1.0, 2.0], &[udl(1.0)], false).is_err());
    assert!(MomentLine::single(2.0, point(2.0, 1.0)).is_err());
    assert!(MomentLine::single(2.0, udl(f64::NAN)).is_err());
    assert!(MomentLine::solve(&[1.0, 1.0], &[udl(1.0), udl(1.0)], BeamSupports::Cantilever).is_err());
}

#[test]
fn a_cantilever_carries_a_tip_load_at_its_free_end() {
    let (l, p) = (4.0, 9_000.0);
    let tip = MomentLine::cantilever(l, point(l, p)).unwrap();
    close(tip.support_moment(0), -p * l, p * l, "tip load root moment");
    close(tip.moment_at(l / 2.0), -p * l / 2.0, p * l, "tip load halfway");
    close(tip.moment_at(l), 0.0, p * l, "free end");
    close(tip.reactions()[0], p, p, "tip load root reaction");
    assert!(MomentLine::single(l, point(l, p)).is_err(), "only a cantilever has a free end");
}

fn next(seed: &mut u64) -> f64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*seed >> 11) as f64) / (1u64 << 53) as f64
}

struct Beam {
    id: String,
    spans: Vec<f64>,
    loads: Vec<SpanLoad>,
    supports: BeamSupports,
    xs: Vec<f64>,
}

fn beams(count: usize) -> Vec<Beam> {
    let mut seed = 20_261_103_u64;
    (0..count)
        .map(|index| {
            let kind = (next(&mut seed) * 4.0) as usize;
            let n = if kind >= 2 { 1 } else { 1 + (next(&mut seed) * 5.0) as usize };
            let supports = match kind {
                0 => BeamSupports::Pinned,
                1 => BeamSupports::ClampedStart,
                _ => if kind == 2 { BeamSupports::Cantilever } else { BeamSupports::Pinned },
            };
            let spans: Vec<f64> = (0..n).map(|_| 0.8 + next(&mut seed) * 7.0).collect();
            let loads: Vec<SpanLoad> = spans
                .iter()
                .map(|l| SpanLoad {
                    udl: if next(&mut seed) < 0.8 { (next(&mut seed) - 0.2) * 40_000.0 } else { 0.0 },
                    points: (0..(next(&mut seed) * 4.0) as usize).map(|_| PointLoad { at: l * (0.02 + 0.96 * next(&mut seed)), force: (next(&mut seed) - 0.3) * 60_000.0 }).collect(),
                })
                .collect();
            let total: f64 = spans.iter().sum();
            let xs = (0..=12).map(|k| total * k as f64 / 12.0).collect();
            Beam { id: format!("beam{index}"), spans, loads, supports, xs }
        })
        .collect()
}

fn list(values: &[f64]) -> String {
    values.iter().map(|v| format!("{v:?}")).collect::<Vec<_>>().join(",")
}

fn request(beams: &[Beam]) -> String {
    let mut text = String::new();
    for b in beams {
        let kind = if b.supports == BeamSupports::Cantilever { "cantilever" } else { "pinned" };
        let udls: Vec<f64> = b.loads.iter().map(|l| l.udl).collect();
        let points: Vec<String> = b.loads.iter().enumerate().flat_map(|(j, l)| l.points.iter().map(move |p| format!("{j}:{:?}:{:?}", p.at, p.force))).collect();
        let points = if points.is_empty() { "-".to_string() } else { points.join(",") };
        let _ = writeln!(text, "{} {kind} {} {} {} {points} {}", b.id, u8::from(b.supports == BeamSupports::ClampedStart), list(&b.spans), list(&udls), list(&b.xs));
    }
    text
}

#[test]
fn the_moment_line_matches_the_numpy_slope_deflection_oracle_on_random_beams() {
    let all = beams(240);
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔮️oracles/📈️moment-line/🐍️.py");
    let out = python_oracle::run_ok(&script, &[], Some(request(&all).as_bytes()));
    let oracle: BTreeMap<String, f64> = String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.to_string(), v.parse().expect("number"))).collect();
    let mut checked = 0;
    for b in &all {
        let line = MomentLine::solve(&b.spans, &b.loads, b.supports).unwrap();
        let reactions = line.reactions();
        let scale = line.diagram(8).iter().fold(0.0_f64, |m, p| m.max(p[1].abs())).max(reactions.iter().fold(0.0_f64, |m, r| m.max(r.abs())));
        for (k, x) in b.xs.iter().enumerate() {
            close(line.moment_at(*x), oracle[&format!("{}.m{k}", b.id)], scale, &format!("{} M({x})", b.id));
            checked += 1;
        }
        for j in 0..=b.spans.len() {
            let expected = oracle[&format!("{}.s{j}", b.id)];
            close(line.support_moment(j), expected, scale, &format!("{} support moment {j}", b.id));
            close(reactions[j], oracle[&format!("{}.r{j}", b.id)], scale, &format!("{} reaction {j}", b.id));
            checked += 2;
        }
        let (lo, hi) = (0..b.spans.len()).map(|j| line.extremes(j)).fold((f64::INFINITY, f64::NEG_INFINITY), |(a, c), (l, h)| (a.min(l), c.max(h)));
        let sampled = line.diagram(2000).iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, c), p| (a.min(p[1]), c.max(p[1])));
        assert!(lo <= sampled.0 + 1e-9 * scale.max(1.0) && hi >= sampled.1 - 1e-9 * scale.max(1.0), "{} extremes bound the sampled line", b.id);
        assert!((lo - sampled.0).abs() <= 2e-2 * scale.max(1.0) && (hi - sampled.1).abs() <= 2e-2 * scale.max(1.0), "{} extremes {lo} {hi} vs sampled {sampled:?}", b.id);
        close(reactions.iter().sum::<f64>(), b.loads.iter().zip(&b.spans).map(|(l, s)| l.udl * s + l.points.iter().map(|p| p.force).sum::<f64>()).sum::<f64>(), scale, &format!("{} vertical equilibrium", b.id));
    }
    assert!(checked > 5_000);
}
