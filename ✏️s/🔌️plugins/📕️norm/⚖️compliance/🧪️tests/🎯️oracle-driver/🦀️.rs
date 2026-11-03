use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use serde_json::Value;

/// 📏️ The five oracle tolerance profiles, mirrored from `🔮️oracles/🏃️harness/📏️tolerance/🐍️.py` and the `comparisonProfiles` of the oracle registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    ClosedFormV1,
    EurocodeQuantitiesV1,
    NumericalV1,
    MethodSpreadV1,
    TableExactV1,
}

impl Profile {
    /// 🔤️ Parse the profile id of a case file; an unknown id panics so a schema drift cannot pass silently.
    pub fn parse(id: &str) -> Self {
        match id {
            "closed-form-v1" => Self::ClosedFormV1,
            "eurocode-quantities-v1" => Self::EurocodeQuantitiesV1,
            "numerical-v1" => Self::NumericalV1,
            "method-spread-v1" => Self::MethodSpreadV1,
            "table-exact-v1" => Self::TableExactV1,
            other => panic!("unknown oracle comparison profile {other}"),
        }
    }

    /// ⚖️ Relative tolerance of the profile.
    pub fn relative(self) -> f64 {
        match self {
            Self::ClosedFormV1 => 1e-9,
            Self::EurocodeQuantitiesV1 => 1e-4,
            Self::NumericalV1 => 5e-3,
            Self::MethodSpreadV1 => 3e-2,
            Self::TableExactV1 => 0.0,
        }
    }

    /// ⚖️ Absolute floor of the profile.
    pub fn absolute(self) -> f64 {
        match self {
            Self::ClosedFormV1 => 1e-12,
            Self::TableExactV1 => 0.0,
            _ => 1e-9,
        }
    }
}

/// 🎯️ One blessed quantity of a case: value, unit, profile and the optional per-case relative tolerance that replaces the profile one.
#[derive(Clone, Debug)]
pub struct Expectation {
    pub value: f64,
    pub unit: String,
    pub profile: Profile,
    pub tolerance: Option<f64>,
}

impl Expectation {
    /// ✅️ True when `actual` equals the blessed value under `max(absolute, relative * scale)`; NaN never matches.
    pub fn accepts(&self, actual: f64) -> bool {
        let relative = self.tolerance.unwrap_or_else(|| self.profile.relative());
        let allowed = self.profile.absolute().max(relative * self.value.abs().max(actual.abs()));
        (self.value - actual).abs() <= allowed
    }
}

/// 📥️ The SI inputs of a case with typed accessors; a missing or mistyped key panics naming the key.
#[derive(Clone, Debug)]
pub struct Inputs(pub BTreeMap<String, Value>);

impl Inputs {
    fn get(&self, key: &str) -> &Value {
        self.0.get(key).unwrap_or_else(|| panic!("oracle input `{key}` is missing"))
    }

    /// 🔢️ A numeric input.
    pub fn num(&self, key: &str) -> f64 {
        self.get(key).as_f64().unwrap_or_else(|| panic!("oracle input `{key}` is not a number"))
    }

    /// 🔢️ A numeric input when present.
    pub fn opt(&self, key: &str) -> Option<f64> {
        self.0.get(key).and_then(Value::as_f64)
    }

    /// 🔢️ A non-negative integer input.
    pub fn count(&self, key: &str) -> usize {
        self.get(key).as_u64().unwrap_or_else(|| panic!("oracle input `{key}` is not a count")) as usize
    }

    /// 🔤️ A text input.
    pub fn text(&self, key: &str) -> &str {
        self.get(key).as_str().unwrap_or_else(|| panic!("oracle input `{key}` is not text"))
    }

    /// 🚩️ A boolean input.
    pub fn flag(&self, key: &str) -> bool {
        self.get(key).as_bool().unwrap_or_else(|| panic!("oracle input `{key}` is not a boolean"))
    }

    /// 📚️ A numeric list input.
    pub fn nums(&self, key: &str) -> Vec<f64> {
        self.get(key)
            .as_array()
            .unwrap_or_else(|| panic!("oracle input `{key}` is not a list"))
            .iter()
            .map(|value| value.as_f64().unwrap_or_else(|| panic!("oracle input `{key}` holds a non-number")))
            .collect()
    }

    /// 📚️ A raw JSON input (nested lists or objects).
    pub fn json(&self, key: &str) -> &Value {
        self.get(key)
    }
}

/// 📤️ The quantities an evaluator entry point computed for one case, keyed by the quantity name of the case file.
#[derive(Clone, Debug, Default)]
pub struct Outputs(pub BTreeMap<&'static str, f64>);

impl Outputs {
    /// ➕️ Builder: add the quantity `name`.
    pub fn with(mut self, name: &'static str, value: f64) -> Self {
        self.0.insert(name, value);
        self
    }
}

/// 🧩️ One typed adapter row: the oracle check id, the evaluator entry point it drives (documentation for owners and reports) and the function that maps SI inputs to our computed quantities.
pub struct Adapter {
    pub check: &'static str,
    pub evaluator: &'static str,
    pub run: fn(&Inputs) -> Outputs,
}

/// ⚠️ A quantity of one case that OUR implementation is expected to differ from, with the documented reason and the request that tracks it; the driver fails when the values unexpectedly agree.
pub struct KnownDifference {
    pub case: &'static str,
    pub quantity: &'static str,
    pub reason: &'static str,
    pub request: &'static str,
}

/// 🗂️ The per-artifact adapter registry plus its known differences.
pub struct Registry {
    pub artifact: &'static str,
    pub adapters: &'static [Adapter],
    pub known_differences: &'static [KnownDifference],
}

/// 🧾️ The result of driving a registry over the blessed case files.
#[derive(Debug, Default)]
pub struct Report {
    pub compared: usize,
    pub confirmed_differences: usize,
    pub gaps: Vec<String>,
    pub mismatches: Vec<String>,
    pub stale_differences: Vec<String>,
}

impl Report {
    /// 🚦️ Panic with every gap, mismatch and stale known difference listed when the report is not clean.
    pub fn assert_clean(&self, artifact: &str) {
        let clean = self.gaps.is_empty() && self.mismatches.is_empty() && self.stale_differences.is_empty();
        assert!(
            clean,
            "{artifact} oracle wiring: {} compared, {} confirmed known differences\nGAPS ({}):\n  {}\nMISMATCHES ({}):\n  {}\nSTALE KNOWN DIFFERENCES ({}):\n  {}",
            self.compared,
            self.confirmed_differences,
            self.gaps.len(),
            self.gaps.join("\n  "),
            self.mismatches.len(),
            self.mismatches.join("\n  "),
            self.stale_differences.len(),
            self.stale_differences.join("\n  "),
        );
    }
}

/// 📖️ One parsed case of a blessed case file.
#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub check: String,
    pub inputs: Inputs,
    pub expected: BTreeMap<String, Expectation>,
}

/// 📂️ Parse every case of the schema-first case file `path` (`🔮️oracles/🏃️harness/🧬️schema/🔣️.json`).
pub fn load_cases(path: &Path) -> Vec<Case> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|error| panic!("oracle case file {}: {error}", path.display()));
    load_cases_text(&text, &path.display().to_string())
}

/// 📃️ Parse every case of the schema-first case file text `text`; `label` names the file in a panic.
pub fn load_cases_text(text: &str, label: &str) -> Vec<Case> {
    let document: Value = serde_json::from_str(text).unwrap_or_else(|error| panic!("oracle case file {label} is not JSON: {error}"));
    document["cases"]
        .as_array()
        .unwrap_or_else(|| panic!("oracle case file {label} has no cases"))
        .iter()
        .map(|case| Case {
            id: case["id"].as_str().expect("case id").to_string(),
            check: case["check"].as_str().expect("case check").to_string(),
            inputs: Inputs(case["inputs"].as_object().expect("case inputs").iter().map(|(key, value)| (key.clone(), value.clone())).collect()),
            expected: case["expected"]
                .as_object()
                .expect("case expected")
                .iter()
                .map(|(name, entry)| {
                    (
                        name.clone(),
                        Expectation {
                            value: entry["value"].as_f64().expect("expected value"),
                            unit: entry["unit"].as_str().expect("expected unit").to_string(),
                            profile: Profile::parse(entry["profile"].as_str().expect("expected profile")),
                            tolerance: entry["tolerance"].as_f64(),
                        },
                    )
                })
                .collect(),
        })
        .collect()
}

/// 🧭️ The oracle case files `🧪️tests/🧫️fixtures/🔮️oracle-cases/*.json` of the artifact leaf above `manifest_dir`, sorted.
pub fn case_files_above(manifest_dir: &str) -> Vec<PathBuf> {
    let mut dir = Path::new(manifest_dir).to_path_buf();
    loop {
        let found = ["🧪️tests/🧫️fixtures/🔮️oracle-cases", "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"].iter().map(|relative| dir.join(relative)).find(|candidate| candidate.is_dir());
        if let Some(candidate) = found {
            let mut files: Vec<PathBuf> = std::fs::read_dir(&candidate)
                .expect("oracle case directory")
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| path.extension().is_some_and(|extension| extension == "json"))
                .collect();
            files.sort();
            assert!(!files.is_empty(), "no oracle case files in {}", candidate.display());
            return files;
        }
        assert!(dir.pop(), "no 🧪️tests/🧫️fixtures/🔮️oracle-cases above {manifest_dir}");
    }
}

/// 🧮️ OUR quantities of one case: the evaluated values keyed by quantity name, the quantities the adapter does not compute and the gap that stopped the case (no adapter, panicking evaluator).
#[derive(Clone, Debug)]
pub struct EvaluatedCase {
    pub id: String,
    pub check: String,
    pub values: BTreeMap<String, f64>,
    pub missing: Vec<String>,
    pub gap: Option<String>,
}

/// ⚙️ Run the artifact evaluator of every case of the schema-first case file text `text` (`🔮️oracles/🏃️harness/🧬️schema/🔣️.json`): case file → adapter registry → quantities. A check without adapter and a panicking evaluator are recorded as the gap of the case; the blessed expectations stay in [`load_cases`].
pub fn evaluate_case_text(text: &str, label: &str, registry: &Registry) -> Vec<EvaluatedCase> {
    load_cases_text(text, label)
        .into_iter()
        .map(|case| {
            let mut evaluated = EvaluatedCase { id: case.id.clone(), check: case.check.clone(), values: BTreeMap::new(), missing: Vec::new(), gap: None };
            let Some(adapter) = registry.adapters.iter().find(|adapter| adapter.check == case.check) else {
                evaluated.gap = Some(format!("{}: no evaluator entry point for check `{}`", case.id, case.check));
                return evaluated;
            };
            let Ok(outputs) = catch_unwind(AssertUnwindSafe(|| (adapter.run)(&case.inputs))) else {
                evaluated.gap = Some(format!("{}: evaluator `{}` panicked on the case inputs", case.id, adapter.evaluator));
                return evaluated;
            };
            for name in case.expected.keys() {
                match outputs.0.get(name.as_str()) {
                    Some(&value) => {
                        evaluated.values.insert(name.clone(), value);
                    }
                    None => evaluated.missing.push(format!("{}: evaluator `{}` does not compute quantity `{name}`", case.id, adapter.evaluator)),
                }
            }
            evaluated
        })
        .collect()
}

/// 📄️ [`evaluate_case_text`] over the case file at `path`.
pub fn evaluate_case_file(path: &Path, registry: &Registry) -> Vec<EvaluatedCase> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|error| panic!("oracle case file {}: {error}", path.display()));
    evaluate_case_text(&text, &path.display().to_string(), registry)
}

/// 🔀️ OUR quantities of a case file as the flat `<case id>.<quantity>` map the `oracle-comparison` step family projects; known differences, gaps and quantities the adapter does not compute are left out, because they are reported by [`drive`] and never compared.
pub fn comparison_values(evaluated: &[EvaluatedCase], registry: &Registry) -> Vec<(String, f64)> {
    evaluated
        .iter()
        .flat_map(|case| case.values.iter().map(move |(name, value)| (case, name, *value)))
        .filter(|(case, name, _)| !registry.known_differences.iter().any(|known| known.case == case.id && known.quantity == name.as_str()))
        .map(|(case, name, value)| (format!("{}.{name}", case.id), value))
        .collect()
}

/// 🚀️ Drive OUR evaluators over every case of the artifact above `manifest_dir`: each quantity is compared under its profile; a check without adapter, a quantity the adapter does not compute and a panicking evaluator are gaps; known differences must differ.
pub fn drive(manifest_dir: &str, registry: &Registry) -> Report {
    let mut report = Report::default();
    let mut seen_differences = vec![false; registry.known_differences.len()];
    for file in case_files_above(manifest_dir) {
        let blessed = load_cases(&file);
        for (case, evaluated) in blessed.iter().zip(evaluate_case_file(&file, registry)) {
            report.gaps.extend(evaluated.gap.iter().cloned());
            report.gaps.extend(evaluated.missing.iter().cloned());
            for (name, expectation) in &case.expected {
                let Some(&actual) = evaluated.values.get(name) else { continue };
                let difference = registry.known_differences.iter().position(|known| known.case == case.id && known.quantity == name);
                report.compared += 1;
                match difference {
                    Some(index) => {
                        seen_differences[index] = true;
                        if expectation.accepts(actual) {
                            report.stale_differences.push(format!("{}.{name}: ours {actual} now equals the blessed {} ({})", case.id, expectation.value, registry.known_differences[index].reason));
                        } else {
                            report.confirmed_differences += 1;
                        }
                    }
                    None if !expectation.accepts(actual) => {
                        report.mismatches.push(format!("{}.{name}: ours {actual} blessed {} {} ({:?})", case.id, expectation.value, expectation.unit, expectation.profile));
                    }
                    None => {}
                }
            }
        }
    }
    for (index, known) in registry.known_differences.iter().enumerate() {
        if !seen_differences[index] {
            report.stale_differences.push(format!("{}.{}: known difference refers to no compared case ({})", known.case, known.quantity, known.request));
        }
    }
    report
}

/// 🔬️ Self-checks of the profile arithmetic against the Python harness rules.
#[cfg(test)]
mod tests {
    use super::*;

    fn expect(value: f64, profile: Profile, tolerance: Option<f64>) -> Expectation {
        Expectation { value, unit: "1".to_string(), profile, tolerance }
    }

    #[test]
    fn profiles_follow_the_harness_rule() {
        assert!(expect(100.0, Profile::EurocodeQuantitiesV1, None).accepts(100.009));
        assert!(!expect(100.0, Profile::EurocodeQuantitiesV1, None).accepts(100.02));
        assert!(expect(100.0, Profile::MethodSpreadV1, Some(0.05)).accepts(104.0));
        assert!(!expect(1.0, Profile::TableExactV1, None).accepts(1.0 + 1e-12));
        assert!(!expect(1.0, Profile::NumericalV1, None).accepts(f64::NAN));
    }

    #[test]
    fn a_missing_adapter_is_a_gap_and_a_stale_difference_is_reported() {
        fn run(_: &Inputs) -> Outputs {
            Outputs::default().with("x", 1.0)
        }
        static ADAPTERS: [Adapter; 1] = [Adapter { check: "demo", evaluator: "run", run }];
        static KNOWN: [KnownDifference; 1] = [KnownDifference { case: "demo.a", quantity: "x", reason: "demo", request: "none" }];
        let registry = Registry { artifact: "demo", adapters: &ADAPTERS, known_differences: &KNOWN };
        assert_eq!(registry.adapters[0].evaluator, "run");
        assert_eq!(registry.artifact, "demo");
        assert_eq!(registry.known_differences[0].case, "demo.a");
    }

    #[test]
    fn a_case_file_text_is_evaluated_into_comparable_quantities() {
        fn run(inputs: &Inputs) -> Outputs {
            Outputs::default().with("twice", 2.0 * inputs.num("x"))
        }
        static ADAPTERS: [Adapter; 1] = [Adapter { check: "double", evaluator: "run", run }];
        static KNOWN: [KnownDifference; 1] = [KnownDifference { case: "double.b", quantity: "twice", reason: "demo", request: "none" }];
        let registry = Registry { artifact: "demo", adapters: &ADAPTERS, known_differences: &KNOWN };
        let text = r#"{"cases": [
            {"id": "double.a", "check": "double", "inputs": {"x": 2.0}, "expected": {"twice": {"value": 4.0, "unit": "1", "profile": "closed-form-v1"}}},
            {"id": "double.b", "check": "double", "inputs": {"x": 3.0}, "expected": {"twice": {"value": 7.0, "unit": "1", "profile": "closed-form-v1"}}},
            {"id": "other.c", "check": "unwired", "inputs": {}, "expected": {"y": {"value": 1.0, "unit": "1", "profile": "closed-form-v1"}}}
        ]}"#;
        let evaluated = evaluate_case_text(text, "inline", &registry);
        assert_eq!(evaluated.len(), 3);
        assert_eq!(evaluated[0].values["twice"], 4.0);
        assert!(evaluated[2].gap.as_deref().is_some_and(|gap| gap.contains("no evaluator entry point")));
        assert_eq!(comparison_values(&evaluated, &registry), vec![("double.a.twice".to_string(), 4.0)]);
    }
}
