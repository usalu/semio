//! 🔬️ Re-projects the raw bytes both roles already produced, under the CORRECTED conformance
//! projection, and reports per-case parity — the same comparison the harness makes, minus re-running
//! the two producers, which the fix does not touch. Temporary; belongs to the ticket folder.

use semio_repo_test_host::Json;
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_7::subsets;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn project(subset: &str, bytes: &[u8]) -> Result<Json, String> {
    match subset {
        "a" => subsets::a::project_conformance(bytes),
        "e" => subsets::e::project_conformance(bytes),
        "h" => subsets::h::project_conformance(bytes),
        "ua" => subsets::ua::project_conformance(bytes),
        "vt" => subsets::vt::project_conformance(bytes),
        "x" => subsets::x::project_conformance(bytes),
        other => Err(format!("unknown subset {other}")),
    }
}

fn render(value: &Json) -> String {
    match value {
        Json::Null => "null".to_string(),
        Json::Bool(flag) => flag.to_string(),
        Json::Number(number) => format!("{number}"),
        Json::String(text) => format!("{text:?}"),
        Json::Array(items) => format!("[{}]", items.iter().map(render).collect::<Vec<_>>().join(",")),
        Json::Object(entries) => format!("{{{}}}", entries.iter().map(|(key, value)| format!("{key}:{}", render(value))).collect::<Vec<_>>().join(",")),
    }
}

fn diff(path: &str, left: &Json, right: &Json, out: &mut Vec<String>) {
    match (left, right) {
        (Json::Object(a), Json::Object(b)) => {
            let mut keys: Vec<&String> = a.iter().map(|(k, _)| k).collect();
            for (k, _) in b {
                if !keys.contains(&k) {
                    keys.push(k);
                }
            }
            for key in keys {
                let la = a.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or(Json::Null);
                let lb = b.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or(Json::Null);
                diff(&format!("{path}.{key}"), &la, &lb, out);
            }
        }
        (Json::Array(a), Json::Array(b)) => {
            if a.len() != b.len() {
                out.push(format!("{path}: array length {} vs {}", a.len(), b.len()));
                return;
            }
            for (index, (x, y)) in a.iter().zip(b.iter()).enumerate() {
                diff(&format!("{path}[{index}]"), x, y, out);
            }
        }
        (x, y) => {
            if render(x) != render(y) {
                out.push(format!("{path}: {} vs {}", render(x), render(y)));
            }
        }
    }
}

fn role_dir(results: &Path, case: &str, role: &str) -> Option<PathBuf> {
    let suffix = format!("-{case}-{role}-rust");
    fs::read_dir(results).ok()?.filter_map(|entry| entry.ok()).map(|entry| entry.path()).find(|path| path.file_name().and_then(|name| name.to_str()).map(|name| name.ends_with(&suffix)).unwrap_or(false))
}


/// 🏅️ Re-produces the CORRECTED oracle's own `mutate-set-snapshot` output — the one scenario whose
/// cached raw bytes predate the `conformant_title` fix — and compares it with the subject's cached
/// raw under the same projection.
fn recheck_set_snapshot(results: &Path, case: &str, subset: &str) {
    const FIXTURE: &str = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";
    let Some(subject) = role_dir(results, case, "subject") else { return };
    let spec = Json::Object(vec![
        ("kind".to_string(), Json::String("set-snapshot".to_string())),
        ("params".to_string(), Json::Object(vec![("conformance".to_string(), Json::String("stamped".to_string()))])),
    ]);
    let original = fs::read(FIXTURE).expect("the committed bachelor thesis");
    let (arrange, apply): (fn(&[u8], &Json) -> Result<Vec<u8>, String>, fn(&[u8], &Json) -> Result<Vec<u8>, String>) = match subset {
        "h" => (subsets::h::oracle_arrange, subsets::h::oracle_apply_mutation),
        "ua" => (subsets::ua::oracle_arrange, subsets::ua::oracle_apply_mutation),
        other => panic!("no recheck wired for {other}"),
    };
    let base = arrange(&original, &spec).expect("arrange");
    let produced = apply(&base, &spec).expect("apply");
    let a = project(subset, &produced).expect("oracle projection");
    let b = project(subset, &fs::read(subject.join("mutate-set-snapshot.subject.raw")).unwrap()).expect("subject projection");
    let mut differences = Vec::new();
    diff("$", &a, &b, &mut differences);
    println!("{case}: mutate-set-snapshot re-produced by the CORRECTED oracle -> {}", if differences.is_empty() { "EQUAL".to_string() } else { differences.join(" | ") });
}

fn main() {
    let results = PathBuf::from("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tests/results");
    for (case, subset) in [("mutate-pdf-1-7-a", "a"), ("mutate-pdf-1-7-e", "e"), ("mutate-pdf-1-7-h", "h"), ("mutate-pdf-1-7-ua", "ua"), ("mutate-pdf-1-7-vt", "vt"), ("mutate-pdf-1-7-x", "x")] {
        let (Some(oracle), Some(subject)) = (role_dir(&results, case, "oracle"), role_dir(&results, case, "subject")) else {
            println!("{case}: MISSING result directory");
            continue;
        };
        let mut scenarios: BTreeMap<String, ()> = BTreeMap::new();
        for entry in fs::read_dir(&oracle).unwrap().filter_map(|entry| entry.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(stem) = name.strip_suffix(".oracle.raw") {
                scenarios.insert(stem.to_string(), ());
            }
        }
        let (mut equal, mut total) = (0usize, 0usize);
        let mut first: Vec<String> = Vec::new();
        for scenario in scenarios.keys() {
            let left = oracle.join(format!("{scenario}.oracle.raw"));
            let right = subject.join(format!("{scenario}.subject.raw"));
            if !right.exists() {
                continue;
            }
            total += 1;
            let a = project(subset, &fs::read(&left).unwrap()).expect("oracle projection");
            let b = project(subset, &fs::read(&right).unwrap()).expect("subject projection");
            let mut differences = Vec::new();
            diff("$", &a, &b, &mut differences);
            if differences.is_empty() {
                equal += 1;
            } else {
                first.push(format!("   {scenario}: {}", differences.join(" | ")));
            }
        }
        println!("{case}: {equal}/{total}");
        for line in first.iter().take(4) {
            println!("{line}");
        }
    }
    for (case, subset) in [("mutate-pdf-1-7-h", "h"), ("mutate-pdf-1-7-ua", "ua")] {
        recheck_set_snapshot(&results, case, subset);
    }
}
