//! 🥒️ The shared, domain-neutral step layer of every norm `🥒️.feature`, Rust side.
//!
//! The twin of `🐍️.py`: it reads the same `🧬️schema/🔣️.json` grammar, binds the same `{name}` holes and emits the same
//! projection keys, so the subject (this file) and the oracle (Python) are compared by the feature's profile. An
//! artifact adapter mounts this file with `#[path]` and contributes only a [`Binding`].
//!
//! See <https://cucumber.io/docs/gherkin/reference/>.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use std::collections::BTreeMap;

const GRAMMAR: &str = include_str!("🧬️schema/🔣️.json");

/// 📖️ The committed step grammar as `(family, [(keyword, template)])`.
pub fn grammar() -> Result<Vec<(String, Vec<String>)>, String> {
    let document = parse_json(GRAMMAR)?;
    let Some(Json::Object(families)) = document.get("families") else { return Err("grammar has no families".into()) };
    Ok(families.iter().map(|(name, family)| (name.clone(), family.array("steps").iter().map(|step| step.str("template")).collect())).collect())
}

/// 🔎️ Binds the `{name}` holes of `template` to `text`; `None` when the wording differs.
pub fn match_template(template: &str, text: &str) -> Option<BTreeMap<String, String>> {
    let mut segments: Vec<(bool, String)> = Vec::new();
    let mut rest = template;
    while !rest.is_empty() {
        match rest.find('{') {
            None => {
                segments.push((false, rest.to_string()));
                break;
            }
            Some(start) => {
                if start > 0 {
                    segments.push((false, rest[..start].to_string()));
                }
                let end = rest.find('}')?;
                segments.push((true, rest[start + 1..end].to_string()));
                rest = &rest[end + 1..];
            }
        }
    }
    let mut bound = BTreeMap::new();
    let mut at = 0usize;
    for (index, (hole, value)) in segments.iter().enumerate() {
        if !hole {
            let found = if index == 0 { if text.starts_with(value.as_str()) { 0 } else { return None } } else { at + text[at..].find(value.as_str())? };
            if index == segments.len() - 1 && !text.ends_with(value.as_str()) {
                return None;
            }
            at = found + value.len();
            continue;
        }
        match segments.get(index + 1) {
            None => {
                bound.insert(value.clone(), text[at..].to_string());
                at = text.len();
            }
            Some((_, following)) => {
                let stop = at + text[at..].find(following.as_str())?;
                bound.insert(value.clone(), text[at..stop].to_string());
                at = stop;
            }
        }
    }
    (at == text.len()).then_some(bound)
}

/// 🧩️ The family a scenario's steps spell and every bound hole; an unknown wording is an error, never a fallback.
pub fn parse_steps(steps: &[(String, String)]) -> Result<(String, BTreeMap<String, String>), String> {
    for (name, templates) in grammar()? {
        if templates.len() != steps.len() {
            continue;
        }
        let mut fields = BTreeMap::new();
        let mut all = true;
        for (template, (_, text)) in templates.iter().zip(steps) {
            match match_template(template, text) {
                Some(hit) => fields.extend(hit),
                None => {
                    all = false;
                    break;
                }
            }
        }
        if all {
            return Ok((name, fields));
        }
    }
    Err(format!("steps {:?} spell no norm step family; the wording must be one of 🧬️schema/🔣️.json", steps.iter().map(|(_, text)| text.as_str()).collect::<Vec<_>>()))
}

/// 🔢️ `a=1.5;b:text` into numeric and text inputs, in written order.
pub fn parse_inputs(text: &str) -> Result<(Vec<(String, f64)>, Vec<(String, String)>), String> {
    let mut numbers = Vec::new();
    let mut texts = Vec::new();
    for part in text.split(';').filter(|piece| !piece.trim().is_empty()) {
        if let Some((name, value)) = part.split_once('=') {
            numbers.push((name.trim().to_string(), value.trim().parse::<f64>().map_err(|error| format!("input {name}: {error}"))?));
        } else if let Some((name, value)) = part.split_once(':') {
            texts.push((name.trim().to_string(), value.trim().to_string()));
        } else {
            return Err(format!("input {part:?} is neither name=value nor name:value"));
        }
    }
    Ok((numbers, texts))
}

/// ⚖️ Relative closeness with an absolute floor for an expected zero, the rule every projection profile shares.
pub fn within(actual: f64, expected: f64, tolerance: f64) -> bool {
    let delta = (actual - expected).abs();
    delta <= tolerance * expected.abs().max(actual.abs()).max(1e-12) || delta <= 1e-12
}

/// 🛑️ Fails the scenario with both numbers when `actual` leaves the tolerance.
pub fn require_within(label: &str, actual: f64, expected: f64, tolerance: f64) -> Result<(), String> {
    if within(actual, expected, tolerance) { Ok(()) } else { Err(format!("{label}: got {actual}, expected {expected} within relative {tolerance}")) }
}

fn number(value: f64) -> Json {
    Json::Number(value)
}

fn text(value: &str) -> Json {
    Json::String(value.to_string())
}

fn number_map(values: &[(String, f64)]) -> Json {
    let mut sorted: Vec<&(String, f64)> = values.iter().filter(|(_, value)| value.is_finite()).collect();
    sorted.sort_by(|left, right| left.0.cmp(&right.0));
    Json::Object(sorted.into_iter().map(|(key, value)| (key.clone(), number(*value))).collect())
}

/// 🔌️ What one artifact contributes to the subject role: its kernel behind neutral functions, one per family it binds.
pub struct Binding {
    pub fixture_case: Box<dyn Fn(&Context, &Json, usize) -> Result<Paired, String>>,
    pub example: Box<dyn Fn(&Context, &str, &str, &[(String, f64)], &[(String, String)]) -> Result<Vec<(String, f64)>, String>>,
    pub vector: Box<dyn Fn(&Context, &str, &str) -> Result<f64, String>>,
    pub check: Box<dyn Fn(&Context, &str, &str) -> Result<(String, f64), String>>,
    pub utilisation: Box<dyn Fn(&Context, &str, &str, &str, &str) -> Result<f64, String>>,
    pub scene: Box<dyn Fn(&Context, &str, &str) -> Result<(String, usize, usize), String>>,
    pub round_trip: Box<dyn Fn(&Context, &str, &str) -> Result<bool, String>>,
    pub comparison: Box<dyn Fn(&Context, &str) -> Result<Vec<(String, f64)>, String>>,
}

/// 🔗️ One catalogue case answered twice: by the kernel and by the independent oracle the adapter ran itself, with the relative bound the feature's profile allows.
pub struct Paired {
    pub entry: String,
    pub kernel: Vec<(String, f64)>,
    pub oracle: Vec<(String, f64)>,
    pub relative: f64,
}

/// 🐍️ The repo-resolved interpreter: `SEMIO_PYTHON`, else the repository `.venv`; never a bare `python3`.
pub fn interpreter(repo_root: &std::path::Path) -> Result<std::path::PathBuf, String> {
    if let Some(value) = std::env::var_os("SEMIO_PYTHON").filter(|value| !value.is_empty()) {
        return Ok(std::path::PathBuf::from(value));
    }
    [".venv/bin/python", ".venv/Scripts/python.exe"].iter().map(|relative| repo_root.join(relative)).find(|candidate| candidate.is_file()).ok_or_else(|| "no Python interpreter: SEMIO_PYTHON is unset and the repository has no .venv; run `repo dependencies ensure`".to_string())
}

/// 🔮️ Runs an independent Python oracle isolated (`-I`), feeds `request` on stdin and parses its JSON stdout.
pub fn run_oracle(context: &Context, script: &std::path::Path, args: &[&str], request: Option<&str>) -> Result<Json, String> {
    use std::io::Write;
    let mut child = std::process::Command::new(interpreter(&context.repo_root)?)
        .arg("-I")
        .arg(script)
        .args(args)
        .stdin(if request.is_some() { std::process::Stdio::piped() } else { std::process::Stdio::null() })
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|error| format!("oracle spawn failed: {error}"))?;
    if let Some(body) = request {
        child.stdin.take().ok_or("oracle stdin")?.write_all(body.as_bytes()).map_err(|error| error.to_string())?;
    }
    let output = child.wait_with_output().map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("oracle {} failed: {}", script.display(), String::from_utf8_lossy(&output.stderr)));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let start = text.find(|c: char| c == '{' || c == '[').ok_or("oracle printed no JSON")?;
    parse_json(&text[start..])
}

/// 🔢️ The numeric members of a JSON object in key order.
pub fn numbers_of(value: Option<&Json>) -> Vec<(String, f64)> {
    match value {
        Some(Json::Object(entries)) => entries.iter().filter_map(|(key, item)| if let Json::Number(number) = item { Some((key.clone(), *number)) } else { None }).collect(),
        _ => Vec::new(),
    }
}

impl Binding {
    /// 🧱️ A binding that serves no family; artifacts replace the functions they implement.
    pub fn none() -> Binding {
        Binding {
            fixture_case: Box::new(|_, _, _| Err("this artifact binds no catalogue-fixture-case".into())),
            example: Box::new(|_, _, _, _, _| Err("this artifact binds no catalogue-example".into())),
            vector: Box::new(|_, _, _| Err("this artifact binds no clause-vector".into())),
            check: Box::new(|_, _, _| Err("this artifact binds no check-outcome".into())),
            utilisation: Box::new(|_, _, _, _, _| Err("this artifact binds no oracle-check".into())),
            scene: Box::new(|_, _, _| Err("this artifact binds no scene-law".into())),
            round_trip: Box::new(|_, _, _| Err("this artifact binds no mutation-round-trip".into())),
            comparison: Box::new(|_, _| Err("this artifact binds no oracle-comparison".into())),
        }
    }
}

fn field<'a>(fields: &'a BTreeMap<String, String>, name: &str) -> Result<&'a str, String> {
    fields.get(name).map(String::as_str).ok_or_else(|| format!("step hole {name} is unbound"))
}

fn tolerance(fields: &BTreeMap<String, String>, name: &str) -> Result<f64, String> {
    field(fields, name)?.parse::<f64>().map_err(|error| format!("{name}: {error}"))
}

/// 🎯️ The `(status, utilisation)` of `check` among `rows` of `(id, status, utilisation)`: the exact id, else the worst applicable row of every id below it (`<check>.<suffix>`); the twin of `check_outcome` in `🐍️.py`.
pub fn pick_outcome(rows: &[(String, &'static str, f64)], check: &str) -> Result<(String, f64), String> {
    if let Some((_, status, utilisation)) = rows.iter().find(|(id, _, _)| id == check) {
        return Ok((status.to_string(), *utilisation));
    }
    let prefix = format!("{check}.");
    let below: Vec<&(String, &'static str, f64)> = rows.iter().filter(|(id, status, _)| id.starts_with(&prefix) && *status != "notApplicable").collect();
    if below.is_empty() {
        return Err(format!("the report holds no check {check}"));
    }
    let utilisation = below.iter().map(|row| row.2).fold(f64::MIN, f64::max);
    let status = if below.iter().any(|row| row.1 == "fail") { "fail" } else if below.iter().any(|row| row.1 == "warning") { "warning" } else { "pass" };
    Ok((status.to_string(), utilisation))
}

/// 🎯️ The subject-role handler for any norm family: one function dispatching on the family the scenario's steps spell.
pub fn subject_handler(binding: Binding) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |context| {
        let (family, fields) = parse_steps(&context.scenario.steps)?;
        let body = match family.as_str() {
            "catalogue-fixture-case" => {
                let index = field(&fields, "index")?.parse::<usize>().map_err(|error| format!("index: {error}"))?;
                let paired = (binding.fixture_case)(context, &context.input_json(field(&fields, "fixture")?)?, index)?;
                if paired.entry != field(&fields, "entry")? {
                    return Err(format!("fixture index {index} holds {}, the feature says {}", paired.entry, field(&fields, "entry")?));
                }
                let mut values = Vec::new();
                for (key, named) in &paired.oracle {
                    let ours = paired.kernel.iter().find(|(name, _)| name == key).map(|(_, value)| *value).ok_or_else(|| format!("{} outcome {key} missing on the kernel side", paired.entry))?;
                    require_within(&format!("{} {key}", paired.entry), ours, *named, paired.relative)?;
                    values.push((key.clone(), ours));
                }
                Json::Object(vec![("entry".into(), text(&paired.entry)), ("index".into(), number(index as f64)), ("values".into(), number_map(&values))])
            }
            "catalogue-example" => {
                let (numbers, texts) = parse_inputs(field(&fields, "inputs")?)?;
                let answer = (binding.example)(context, field(&fields, "entry")?, field(&fields, "annex")?, &numbers, &texts)?;
                let key = field(&fields, "key")?;
                let value = answer.iter().find(|(name, _)| name == key).map(|(_, value)| *value).ok_or_else(|| format!("outcome {key} missing"))?;
                require_within(&format!("{} {key}", field(&fields, "entry")?), value, tolerance(&fields, "expected")?, tolerance(&fields, "tolerance")?)?;
                Json::Object(vec![("entry".into(), text(field(&fields, "entry")?)), ("annex".into(), text(field(&fields, "annex")?)), ("key".into(), text(key)), ("value".into(), number(value))])
            }
            "clause-vector" => {
                let value = (binding.vector)(context, field(&fields, "vector")?, field(&fields, "quantity")?)?;
                Json::Object(vec![("vector".into(), text(field(&fields, "vector")?)), ("quantity".into(), text(field(&fields, "quantity")?)), ("value".into(), number(value))])
            }
            "published-example" => {
                let value = (binding.vector)(context, field(&fields, "example")?, field(&fields, "quantity")?)?;
                require_within(&format!("{} {}", field(&fields, "example")?, field(&fields, "quantity")?), value, tolerance(&fields, "expected")?, tolerance(&fields, "tolerance")?)?;
                Json::Object(vec![("example".into(), text(field(&fields, "example")?)), ("quantity".into(), text(field(&fields, "quantity")?)), ("value".into(), number(value))])
            }
            "oracle-check" => {
                let value = (binding.utilisation)(context, field(&fields, "subject")?, field(&fields, "annex")?, field(&fields, "check")?, field(&fields, "quantity")?)?;
                Json::Object(vec![("check".into(), text(field(&fields, "check")?)), ("quantity".into(), text(field(&fields, "quantity")?)), ("utilisation".into(), number(value))])
            }
            "check-outcome" => {
                let (status, utilisation) = (binding.check)(context, field(&fields, "fixture")?, field(&fields, "check")?)?;
                require_within(&format!("{} utilisation", field(&fields, "check")?), utilisation, tolerance(&fields, "utilisation")?, tolerance(&fields, "tolerance")?)?;
                if status != field(&fields, "status")? {
                    return Err(format!("{} status {status}, expected {}", field(&fields, "check")?, field(&fields, "status")?));
                }
                Json::Object(vec![("check".into(), text(field(&fields, "check")?)), ("status".into(), text(&status)), ("utilisation".into(), number(utilisation))])
            }
            "scene-law" => {
                let (kind, handles, diagrams) = (binding.scene)(context, field(&fields, "fixture")?, field(&fields, "entity")?)?;
                let (want_handles, want_diagrams) = (tolerance(&fields, "handles")? as usize, tolerance(&fields, "diagrams")? as usize);
                if kind != field(&fields, "kind")? || handles != want_handles || diagrams != want_diagrams {
                    return Err(format!("entity {} is {kind} with {handles} handles and {diagrams} diagrams, the feature says {} with {want_handles} and {want_diagrams}", field(&fields, "entity")?, field(&fields, "kind")?));
                }
                Json::Object(vec![("entity".into(), text(field(&fields, "entity")?)), ("kind".into(), text(&kind)), ("handles".into(), number(handles as f64)), ("diagrams".into(), number(diagrams as f64))])
            }
            "mutation-round-trip" => {
                let restored = (binding.round_trip)(context, field(&fields, "before")?, field(&fields, "mutation")?)?;
                if !restored {
                    return Err(format!("applying {} and then its computed inverse did not restore {}", field(&fields, "mutation")?, field(&fields, "before")?));
                }
                Json::Object(vec![("mutation".into(), text(field(&fields, "mutation")?)), ("restored".into(), Json::Bool(restored))])
            }
            "oracle-comparison" => {
                let values = (binding.comparison)(context, field(&fields, "fixture")?)?;
                Json::Object(vec![("fixture".into(), text(field(&fields, "fixture")?)), ("values".into(), number_map(&values))])
            }
            other => return Err(format!("family {other} has no subject binding in the shared step layer")),
        };
        Ok(Outcome::with_raw(body.to_string().into_bytes(), body))
    }
}

/// 🧭️ The subject-role adapter of a norm case; the oracle role is the Python adapter, so the subject is never its own reference.
pub fn norm_adapter(binding: Binding) -> Adapter {
    let handler = std::rc::Rc::new(subject_handler(binding));
    let mut adapter = Adapter::new("rust");
    for (family, _) in grammar().expect("committed norm step grammar") {
        let shared = handler.clone();
        adapter = adapter.subject(&family, move |context| shared(context));
    }
    adapter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_holes_bind_and_wrong_wording_is_refused() {
        let bound = match_template("the output \"{key}\" equals {expected} within relative tolerance {tolerance}", "the output \"fk\" equals 8.935e6 within relative tolerance 1e-3").expect("match");
        assert_eq!(bound["key"], "fk");
        assert_eq!(bound["expected"], "8.935e6");
        assert_eq!(bound["tolerance"], "1e-3");
        assert!(match_template("the entry is evaluated", "the entry is computed").is_none());
    }

    #[test]
    fn inputs_split_numbers_and_text() {
        let (numbers, texts) = parse_inputs("fb_pa=20e6;material:clay;gamma_m=1.5").expect("inputs");
        assert_eq!(numbers, vec![("fb_pa".to_string(), 20e6), ("gamma_m".to_string(), 1.5)]);
        assert_eq!(texts, vec![("material".to_string(), "clay".to_string())]);
    }

    #[test]
    fn every_family_spells_its_own_steps_only() {
        for (name, templates) in grammar().expect("grammar") {
            let literal: Vec<(String, String)> = templates.iter().map(|template| ("Given".to_string(), match_filled(template))).collect();
            assert_eq!(parse_steps(&literal).expect("family").0, name);
        }
    }

    fn match_filled(template: &str) -> String {
        let mut out = String::new();
        let mut rest = template;
        while let Some(start) = rest.find('{') {
            out.push_str(&rest[..start]);
            out.push('x');
            rest = &rest[rest.find('}').expect("closed hole") + 1..];
        }
        out.push_str(rest);
        out
    }

    #[test]
    fn outcome_picks_the_exact_row_or_the_worst_row_below() {
        let rows = vec![("a.wall".to_string(), "pass", 0.4), ("a.wall.lc1".to_string(), "pass", 0.5), ("a.wall.lc2".to_string(), "fail", 1.2), ("a.wallet".to_string(), "pass", 9.0)];
        assert_eq!(pick_outcome(&rows, "a.wall").expect("exact"), ("pass".to_string(), 0.4));
        assert_eq!(pick_outcome(&rows, "a.wall.lc").is_err(), true);
        let below: Vec<(String, &'static str, f64)> = rows.into_iter().skip(1).collect();
        assert_eq!(pick_outcome(&below, "a.wall").expect("worst"), ("fail".to_string(), 1.2));
    }

    #[test]
    fn tolerance_rule_matches_the_python_twin() {
        assert!(within(1.0, 1.0001, 1e-3));
        assert!(!within(8.9e6, 8.935e6, 1e-3));
        assert!(within(1e-15, 0.0, 1e-9));
    }
}
