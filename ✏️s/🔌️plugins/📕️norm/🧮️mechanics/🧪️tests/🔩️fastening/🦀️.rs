use crate::fastening::*;
use semio_s_plugin_norm_compliance::python_oracle;
use std::collections::BTreeMap;
use std::sync::OnceLock;

const CASES: &str = include_str!("🧾️cases.json");

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol * expected.abs().max(1e-300), "{what}: {actual} vs {expected} (rel tol {tol})");
}

fn text<'a>(line: &'a str, key: &str) -> &'a str {
    let tail = line.split_once(&format!("\"{key}\":")).unwrap_or_else(|| panic!("missing {key} in {line}")).1;
    let end = tail.find(|c| c == ',' || c == '}').expect("end");
    &tail[..end]
}

fn num(line: &str, key: &str) -> f64 {
    text(line, key).trim().parse().unwrap_or_else(|_| panic!("number {key}"))
}

fn anchors(line: &str) -> Vec<[f64; 2]> {
    let body = line.split_once("\"anchors\":[").expect("anchors").1.split_once("],\"e_nx\"").expect("end").0;
    body.split("],[").map(|p| p.trim_matches(|c| c == '[' || c == ']')).map(|p| {
        let (x, y) = p.split_once(',').expect("pair");
        [x.parse().expect("x"), y.parse().expect("y")]
    }).collect()
}

fn lines() -> impl Iterator<Item = &'static str> {
    CASES.lines().filter(|l| l.starts_with("{\"id\""))
}

fn oracle() -> &'static BTreeMap<String, f64> {
    static CACHE: OnceLock<BTreeMap<String, f64>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔮️oracles/🔩️fastening/🐍️.py");
        let out = python_oracle::run_ok(&script, &[], None);
        String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.to_string(), v.parse().expect("number"))).collect()
    })
}

fn quantities(line: &str) -> Vec<(String, f64)> {
    let id = text(line, "id").trim_matches('"').to_string();
    let n = |k: &str| num(line, k);
    match text(line, "kind").trim_matches('"') {
        "cone" => {
            let pts = anchors(line);
            let input = ConeInput { anchors: &pts, region: Region::single_edge(n("c1")), e_nx: n("e_nx"), e_ny: n("e_ny"), ..ConeInput::single(n("f_ck"), n("h_ef"), n("k1"), 0.0) };
            let c = cone_breakout(&input);
            vec![(format!("{id}.n_rk_c"), c.n_rk_c), (format!("{id}.ratio"), c.ratio), (format!("{id}.psi_s"), c.psi_s)]
        }
        "pullout" => vec![(format!("{id}.n_rk_p"), pullout_n_rk_p(n("k2"), n("f_ck"), n("d"), n("d_h")))],
        "steel" => {
            let g = SteelPartialFactors { f_uk_max: 800.0, ratio_max: 0.8, slope_n: 1.2, min_n: 1.4, slope_v: 1.0, min_v: 1.25, other: 1.5 };
            vec![(format!("{id}.n_rk_s"), steel_n_rk_s(n("a_s"), n("f_uk"))), (format!("{id}.gamma_n"), g.gamma_ms(n("f_uk"), n("f_yk"), false)), (format!("{id}.gamma_v"), g.gamma_ms(n("f_uk"), n("f_yk"), true))]
        }
        "edge" => vec![(format!("{id}.v_rk_c"), edge_breakout(&EdgeInput::single(n("f_ck"), n("k9"), n("d_nom"), n("h_ef"), n("c1"), n("h"))).v_rk_c)],
        "pryout" => vec![(format!("{id}.v_rk_cp"), pryout_v_rk_cp(n("n_rk_c"), n("h_ef")))],
        "interaction" => vec![(format!("{id}.value"), interaction(n("n_ed"), n("n_rd"), n("v_ed"), n("v_rd"), n("exponent")))],
        other => panic!("kind {other}"),
    }
}

#[test]
fn cases_match_stated_values_and_the_oracle() {
    let mut count = 0;
    for line in lines() {
        for (key, actual) in quantities(line) {
            let stated = key.split_once('.').map(|(_, q)| format!("\"expect_{q}\":")).expect("key");
            if let Some(tail) = line.split_once(&stated) {
                close(actual, tail.1.split(|c| c == ',' || c == '}').next().expect("value").parse().expect("number"), 1e-9, &key);
            }
            close(actual, oracle()[&key], 1e-9, &key);
            count += 1;
        }
    }
    assert!(count >= 19, "{count}");
}

#[test]
fn worked_cone_example_by_hand() {
    let c = cone_breakout(&ConeInput::single(30.0, 100.0, 8.9, 100.0));
    close(c.c_cr, 150.0, 1e-12, "c_cr");
    close(c.n0_rk_c, 8.9 * 30.0_f64.sqrt() * 1000.0, 1e-12, "N0");
    close(c.ratio, 250.0 / 300.0, 1e-12, "A_c,N/A0");
    close(c.psi_s, 0.9, 1e-12, "psi_s");
    close(c.psi_re, 1.0, 1e-12, "psi_re");
    close(c.n_rk_c, c.n0_rk_c * 250.0 / 300.0 * 0.9, 1e-12, "N_Rk,c");
}

#[test]
fn eccentricity_and_compression_zone_reduce_or_raise_the_cone() {
    close(psi_ec_n(50.0, 0.0, 100.0), 0.75, 1e-12, "psi_ec");
    close(psi_m_n(100.0, 100.0), 2.0 - 100.0 / 150.0, 1e-12, "psi_M");
    close(psi_m_n(0.0, 100.0), 1.0, 1e-12, "no zone");
    close(psi_re_n(60.0, false), 0.8, 1e-12, "psi_re thin");
    close(psi_re_n(60.0, true), 1.0, 1e-12, "psi_re dense");
}

#[test]
fn group_cone_is_the_union_of_overlapping_footprints() {
    let pair = [[0.0, 0.0], [100.0, 0.0]];
    close(cone_area_ratio(&pair, 150.0, Region::FREE), 400.0 / 300.0, 1e-12, "overlap");
    let far = [[0.0, 0.0], [400.0, 0.0]];
    close(cone_area_ratio(&far, 150.0, Region::FREE), 2.0, 1e-12, "isolated");
}

#[test]
fn splitting_and_interaction_helpers() {
    close(splitting_n_rk_sp(50.0, 40.0, 1.25), 50.0, 1e-12, "sp");
    close(psi_h_sp(200.0, 100.0, 40.0), (0.8_f64).powf(2.0 / 3.0).max(1.0).min(2.0_f64.powf(2.0 / 3.0)), 1e-12, "psi_h");
    close(interaction(-5.0, 10.0, 0.0, 0.0, 2.0), 0.0, 1e-12, "negative action");
    close(steel_v_rk_s_lever(1.0, 100.0, 500.0, 50.0, 0.5), 600.0, 1e-12, "lever");
    close(steel_v_rk_s(100.0, 500.0, 0.8), 20000.0, 1e-12, "grout");
}
