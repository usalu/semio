use crate::catalogue::*;
use crate::*;
use std::collections::BTreeMap;

const FEATURE: &str = include_str!("../../../🧪️tests/🏋️en1991-catalogue/🥒️.feature");
const ORACLE: &str = "../../../../../🔮️oracles/⚖️compliance/🐍️.py";

struct Row { entry: String, annex: AnnexChoice, inputs: String, key: String, value: f64, tolerance: f64 }

fn rows() -> Vec<Row> {
    let mut out = Vec::new();
    let mut in_examples = false;
    let mut header_seen = false;
    for line in FEATURE.lines().map(str::trim) {
        if line.starts_with("Examples:") { in_examples = true; continue; }
        if !in_examples || !line.starts_with('|') { continue; }
        if !header_seen { header_seen = true; continue; }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        assert_eq!(cells.len(), 7, "row {line}");
        out.push(Row { entry: cells[1].into(), annex: if cells[2] == "de" { AnnexChoice::De } else { AnnexChoice::En }, inputs: cells[3].into(), key: cells[4].into(), value: cells[5].parse().expect("value"), tolerance: cells[6].parse().expect("tolerance") });
    }
    out
}

fn case_from(index: usize, row: &Row) -> Case {
    let mut case = Case::new(&index.to_string(), row.annex);
    for pair in row.inputs.split(';').filter(|p| !p.is_empty()) {
        if let Some((k, v)) = pair.split_once('=') {
            case = case.with_n(k, v.parse::<f64>().expect("number"));
        } else {
            let (k, v) = pair.split_once(':').expect("name=value or name:value");
            case = case.with_s(k, v);
        }
    }
    case
}

fn encode(entry: &str, case: &Case) -> String {
    let mut line = format!("{entry}|{}|annex={}", case.id, if case.de() { "de" } else { "en" });
    for (k, v) in &case.num { line.push_str(&format!("|n:{k}={v:?}")); }
    for (k, v) in &case.text { line.push_str(&format!("|s:{k}={v}")); }
    for (k, v) in &case.vec { line.push_str(&format!("|v:{k}={}", v.iter().map(|x| format!("{x:?}")).collect::<Vec<_>>().join(","))); }
    line
}

#[semio_framework_async_macros::async_test]
async fn feature_rows_match_the_reference_values() {
    let all = rows();
    assert!(all.len() >= 40, "{}", all.len());
    for (i, row) in all.iter().enumerate() {
        let outcomes = evaluate_outcomes(&row.entry, &case_from(i, row)).unwrap_or_else(|e| panic!("{} row {i}: {e}", row.entry));
        let got = outcomes.iter().find(|o| o.key == row.key).unwrap_or_else(|| panic!("{} lacks {}", row.entry, row.key)).value;
        assert!((got - row.value).abs() <= row.tolerance * row.value.abs().max(1e-12), "{} {} row {i}: {got} vs {}", row.entry, row.key, row.value);
    }
}

#[semio_framework_async_macros::async_test]
async fn python_oracle_agrees_with_the_catalogue() {
    let all = rows();
    let mut cases: Vec<(String, Case)> = all.iter().enumerate().map(|(i, r)| (r.entry.clone(), case_from(i, r))).collect();
    let extra = [
        ("E12.09", de_case("x1").with_n("width", 1.0).with_n("height", 1.0).with_n("distance", 1.0)),
        ("E12.09", de_case("x2").with_n("width", 3.0).with_n("height", 2.0).with_n("distance", 4.0)),
        ("E12.06", de_case("x3").with_n("theta_g_c", 841.8).with_n("theta_m_c", 500.0)),
        ("E13.10", de_case("x4").with_n("v", 0.4).with_n("n_years", 100.0)),
        ("E16.01", de_case("x5").with_n("duration_days", 2.0)),
        ("E14.12", Case::new("x6", AnnexChoice::En).with_n("h", 80.0).with_n("b", 25.0).with_n("terrain", 2.0).with_n("v_b", 27.0).with_n("delta", 0.08)),
        ("E4.03", Case::new("x7", AnnexChoice::En).with_n("gamma", 7000.0).with_n("phi", 35.0).with_n("dc", 8.0).with_n("z", 15.0)),
        ("E12.02", de_case("x8").with_n("a_f", 50.0).with_n("a_t", 184.84).with_n("a_v", 20.0).with_n("h_eq", 2.0).with_n("b", 1160.0).with_n("q_fd_mj", 80.0)),
        ("E14.05", Case::new("x9", AnnexChoice::En).with_n("z", 45.0).with_n("terrain", 4.0).with_n("v_b", 30.0).with_n("rho", 1.2)),
    ];
    for (entry, case) in extra { cases.push((entry.to_string(), case)); }
    let stdin: String = cases.iter().map(|(e, c)| encode(e, c) + "\n").collect();
    let out = semio_s_plugin_norm_compliance::python_oracle::run_ok(ORACLE_PATH(), &[std::ffi::OsStr::new("--catalogue")], Some(stdin.as_bytes()));
    let text = String::from_utf8(out.stdout).expect("utf-8");
    let mut oracle: BTreeMap<(String, String, String), f64> = BTreeMap::new();
    for line in text.lines() {
        let p: Vec<&str> = line.split('|').collect();
        assert_eq!(p.len(), 4, "{line}");
        oracle.insert((p[0].into(), p[1].into(), p[2].into()), p[3].parse().expect("number"));
    }
    let mut compared = 0;
    for (entry, case) in &cases {
        let outcomes = evaluate_outcomes(entry, case).expect("evaluates");
        for o in &outcomes {
            if let Some(expected) = oracle.get(&(entry.clone(), case.id.clone(), o.key.clone())) {
                let tolerance = if entry == "E13.10" || entry == "E16.01" { 2e-3 } else if entry == "E12.06" { 1e-3 } else { 1e-6 };
                assert!((o.value - expected).abs() <= tolerance * expected.abs().max(1e-9), "{entry} {} {}: rust {} oracle {expected}", case.id, o.key, o.value);
                compared += 1;
            }
        }
    }
    assert!(compared >= 80, "only {compared} values compared");
    let covered: std::collections::BTreeSet<&str> = oracle.keys().map(|k| k.0.as_str()).collect();
    assert!(covered.len() >= 30, "oracle covers {} entries", covered.len());
}

fn de_case(id: &str) -> Case { Case::new(id, AnnexChoice::De) }

#[allow(non_snake_case)]
fn ORACLE_PATH() -> std::path::PathBuf { std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(ORACLE) }
