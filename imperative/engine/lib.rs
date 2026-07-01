//! ⚙️ Headless imperative engine: ordered path of side-effect steps.

use neural_engine::{Atom, Dictionary, Registry, Value, SCHEMA_KEY};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_NESTING_DEPTH: usize = 64;
const MAX_LOOP_ITERATIONS: u64 = 200_000;

// #region 🔖Path
/// 👣 One ordered side-effect invocation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: Dictionary,
    #[serde(default)]
    pub bodies: BTreeMap<String, Path>,
}

/// 🛤️ Ordered list of steps — position is execution order within a scope.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub steps: Vec<Step>,
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }
}
// #endregion 🔖Path

// #region 🔖EffectLog
/// 📋 One recorded side-effect from a step execution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectLogEntry {
    pub step_id: String,
    pub kind: String,
    pub input: Dictionary,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Dictionary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 📦 Result of running a path.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    pub scope: Dictionary,
    pub effects: Vec<EffectLogEntry>,
}
// #endregion 🔖EffectLog

// #region 🔖Executor
/// ▶️ Sequential executor over an imperative path.
pub struct Executor<'a> {
    registry: &'a Registry,
}

impl<'a> Executor<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    /// Runs steps strictly in list order; merges each output into scope; halts on first error.
    pub fn run(&self, path: &Path, seed: &Dictionary) -> RunResult {
        let mut scope = seed.clone();
        let mut effects = Vec::new();
        self.run_steps(&path.steps, &mut scope, &mut effects, 0);
        RunResult { scope, effects }
    }

    fn run_steps(&self, steps: &[Step], scope: &mut Dictionary, effects: &mut Vec<EffectLogEntry>, depth: usize) {
        if depth > MAX_NESTING_DEPTH {
            effects.push(EffectLogEntry {
                step_id: String::new(),
                kind: "control.depth".into(),
                input: Dictionary::new(),
                output: None,
                error: Some(format!("nesting depth exceeded {MAX_NESTING_DEPTH}")),
            });
            return;
        }
        for step in steps {
            if let Some(halt) = self.run_step(step, scope, effects, depth) {
                if halt {
                    break;
                }
            }
        }
    }

    fn run_step(&self, step: &Step, scope: &mut Dictionary, effects: &mut Vec<EffectLogEntry>, depth: usize) -> Option<bool> {
        match step.kind.as_str() {
            "control.if" => {
                let key = read_string_param(&step.params, "key").unwrap_or_default();
                let condition = read_scope_bool(scope, &key);
                let slot = if condition { "then" } else { "else" };
                let input = scope.merge(&step.params);
                effects.push(EffectLogEntry {
                    step_id: step.id.clone(),
                    kind: step.kind.clone(),
                    input,
                    output: Some(Dictionary::new().insert(
                        "branch",
                        Value::Atom(Atom::String(slot.into())),
                    )),
                    error: None,
                });
                if let Some(body) = step.bodies.get(slot) {
                    self.run_steps(&body.steps, scope, effects, depth + 1);
                }
                return None;
            }
            "control.while" => {
                let key = read_string_param(&step.params, "key").unwrap_or_default();
                let mut iterations = 0u64;
                while read_scope_bool(scope, &key) {
                    iterations += 1;
                    if iterations > MAX_LOOP_ITERATIONS {
                        effects.push(EffectLogEntry {
                            step_id: step.id.clone(),
                            kind: step.kind.clone(),
                            input: scope.merge(&step.params),
                            output: None,
                            error: Some(format!("while loop exceeded {MAX_LOOP_ITERATIONS} iterations")),
                        });
                        return Some(true);
                    }
                    if let Some(body) = step.bodies.get("body") {
                        self.run_steps(&body.steps, scope, effects, depth + 1);
                    }
                }
                return None;
            }
            "control.repeat" => {
                let count = read_number_param(&step.params, "count").unwrap_or(0.0).max(0.0) as u64;
                let capped = count.min(MAX_LOOP_ITERATIONS);
                if count > MAX_LOOP_ITERATIONS {
                    effects.push(EffectLogEntry {
                        step_id: step.id.clone(),
                        kind: step.kind.clone(),
                        input: scope.merge(&step.params),
                        output: None,
                        error: Some(format!("repeat count capped at {MAX_LOOP_ITERATIONS}")),
                    });
                }
                if let Some(body) = step.bodies.get("body") {
                    for index in 0..capped {
                        *scope = scope.clone().insert("index", Value::Atom(Atom::Integer(index as i64)));
                        self.run_steps(&body.steps, scope, effects, depth + 1);
                    }
                }
                return None;
            }
            _ => {}
        }
        let input = scope.merge(&step.params);
        match self.registry.dispatch(&step.kind, &input) {
            Ok(output) => {
                *scope = merge_output_into_scope(scope, &output);
                effects.push(EffectLogEntry {
                    step_id: step.id.clone(),
                    kind: step.kind.clone(),
                    input,
                    output: Some(output),
                    error: None,
                });
                None
            }
            Err(err) => {
                effects.push(EffectLogEntry {
                    step_id: step.id.clone(),
                    kind: step.kind.clone(),
                    input,
                    output: None,
                    error: Some(err.to_string()),
                });
                Some(true)
            }
        }
    }
}

fn read_string_param(params: &Dictionary, key: &str) -> Option<String> {
    params
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_str())
        .map(str::to_string)
}

fn read_number_param(params: &Dictionary, key: &str) -> Option<f64> {
    params.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64())
}

fn read_scope_bool(scope: &Dictionary, key: &str) -> bool {
    scope
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_bool())
        .unwrap_or(false)
}

fn merge_output_into_scope(scope: &Dictionary, output: &Dictionary) -> Dictionary {
    let mut merged = scope.clone();
    for key in output.keys() {
        if key == SCHEMA_KEY {
            continue;
        }
        if let Some(value) = output.get(key) {
            if let Some(payload) = value.as_dictionary() {
                if payload.len() == 1 && payload.get(SCHEMA_KEY).is_some() {
                    merged = merged.merge(payload);
                    continue;
                }
                if key == "message" || key == "delay" {
                    merged = merged.merge(payload);
                    continue;
                }
            }
            merged = merged.insert(key.clone(), value.clone());
        }
    }
    merged
}
// #endregion 🔖Executor

// #region 🔖Compile
/// 📝 Emits one line of source per step, e.g. `state.increment(by=5, key="counter");`
pub fn compile_to_text(path: &Path) -> String {
    compile_steps(&path.steps, 0)
}

fn compile_steps(steps: &[Step], indent: usize) -> String {
    let pad = "  ".repeat(indent);
    steps
        .iter()
        .map(|step| compile_step(step, indent, &pad))
        .collect::<Vec<_>>()
        .join("\n")
}

fn compile_step(step: &Step, indent: usize, pad: &str) -> String {
    match step.kind.as_str() {
        "control.if" => {
            let key = read_string_param(&step.params, "key").unwrap_or_else(|| "condition".into());
            let then_body = step
                .bodies
                .get("then")
                .map(|path| compile_steps(&path.steps, indent + 1))
                .unwrap_or_default();
            let else_body = step
                .bodies
                .get("else")
                .map(|path| compile_steps(&path.steps, indent + 1))
                .unwrap_or_default();
            if else_body.is_empty() {
                format!("{pad}if ({key}) {{\n{then_body}\n{pad}}}")
            } else {
                format!("{pad}if ({key}) {{\n{then_body}\n{pad}}} else {{\n{else_body}\n{pad}}}")
            }
        }
        "control.while" => {
            let key = read_string_param(&step.params, "key").unwrap_or_else(|| "condition".into());
            let body = step
                .bodies
                .get("body")
                .map(|path| compile_steps(&path.steps, indent + 1))
                .unwrap_or_default();
            format!("{pad}while ({key}) {{\n{body}\n{pad}}}")
        }
        "control.repeat" => {
            let count = read_number_param(&step.params, "count").unwrap_or(0.0);
            let body = step
                .bodies
                .get("body")
                .map(|path| compile_steps(&path.steps, indent + 1))
                .unwrap_or_default();
            format!("{pad}repeat ({count}) {{\n{body}\n{pad}}}")
        }
        _ => {
            let params: Vec<String> = step
                .params
                .keys()
                .filter(|key| key.as_str() != SCHEMA_KEY)
                .map(|key| format!("{}={}", key, format_value(step.params.get(key).expect("key"))))
                .collect();
            if params.is_empty() {
                format!("{pad}{}();", step.kind)
            } else {
                format!("{pad}{}({});", step.kind, params.join(", "))
            }
        }
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Atom(atom) => match atom {
            Atom::Null => "null".into(),
            Atom::Boolean(v) => v.to_string(),
            Atom::Integer(v) => v.to_string(),
            Atom::Decimal(v) => {
                if v.fract().abs() < f64::EPSILON {
                    format!("{:.0}", v)
                } else {
                    v.to_string()
                }
            }
            Atom::String(v) => format!("\"{}\"", v.replace('"', "\\\"")),
        },
        Value::Dictionary(dict) => serde_json::to_string(dict).unwrap_or_else(|_| "{}".into()),
    }
}
// #endregion 🔖Compile

// #region 🔖ModuleRegistry
/// 📦 Builds the composed imperative operator registry from all installed modules.
pub fn imperative_module_registry() -> Registry {
    let mut registry = Registry::new();
    imperative_module_core::register(&mut registry);
    imperative_module_text::register(&mut registry);
    imperative_module_math::register(&mut registry);
    imperative_module_logic::register(&mut registry);
    registry
}

/// 📚 Merges catalogue sections from all installed imperative modules.
pub fn imperative_catalogue_json(registry: &Registry) -> String {
    let core: serde_json::Value =
        serde_json::from_str(&imperative_module_core::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let text: serde_json::Value =
        serde_json::from_str(&imperative_module_text::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let math: serde_json::Value =
        serde_json::from_str(&imperative_module_math::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let logic: serde_json::Value =
        serde_json::from_str(&imperative_module_logic::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let control: serde_json::Value = serde_json::from_str(&imperative_module_control::catalogue_json()).unwrap_or(serde_json::json!({}));
    let mut sections = core
        .get("sections")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    for module in [&text, &math, &logic, &control] {
        if let Some(module_sections) = module.get("sections").and_then(|value| value.as_array()) {
            sections.extend(module_sections.iter().cloned());
        }
    }
    serde_json::to_string(&serde_json::json!({
        "schema": "imperative.catalogue/v1",
        "sections": sections,
    }))
    .unwrap_or_else(|_| "{}".into())
}
// #endregion 🔖ModuleRegistry

#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{Atom, Registry};

    fn test_registry() -> Registry {
        imperative_module_registry()
    }

    #[test]
    fn composed_registry_runs_text_operators() {
        let registry = imperative_module_registry();
        let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abc".into())));
        let output = registry.dispatch("text.uppercase", &input).expect("dispatch");
        let value = output
            .get("text")
            .and_then(|v| v.as_dictionary())
            .and_then(|dict| dict.get("value"))
            .and_then(|v| v.as_atom())
            .and_then(|a| a.as_str());
        assert_eq!(value, Some("ABC"));
    }

    #[test]
    fn executor_runs_steps_in_order() {
        let registry = test_registry();
        let executor = Executor::new(&registry);
        let path = Path {
            steps: vec![
                Step {
                    id: "s1".into(),
                    kind: "state.set".into(),
                    params: Dictionary::new()
                        .insert("key", Value::Atom(Atom::String("counter".into())))
                        .insert("value", Value::Atom(Atom::Decimal(0.0))),
                    bodies: BTreeMap::new(),
                },
                Step {
                    id: "s2".into(),
                    kind: "state.increment".into(),
                    params: Dictionary::new()
                        .insert("key", Value::Atom(Atom::String("counter".into())))
                        .insert("by", Value::Atom(Atom::Decimal(3.0))),
                    bodies: BTreeMap::new(),
                },
                Step {
                    id: "s3".into(),
                    kind: "log.print".into(),
                    params: Dictionary::new().insert("message", Value::Atom(Atom::String("done".into()))),
                    bodies: BTreeMap::new(),
                },
            ],
        };
        let result = executor.run(&path, &Dictionary::new());
        assert_eq!(result.effects.len(), 3);
        assert!(result.effects.iter().all(|entry| entry.error.is_none()));
        let counter = result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        assert_eq!(counter, Some(3.0));
    }

    #[test]
    fn executor_runs_control_if_then_branch() {
        let registry = test_registry();
        let executor = Executor::new(&registry);
        let mut bodies = BTreeMap::new();
        bodies.insert(
            "then".into(),
            Path {
                steps: vec![Step {
                    id: "t1".into(),
                    kind: "state.set".into(),
                    params: Dictionary::new()
                        .insert("key", Value::Atom(Atom::String("result".into())))
                        .insert("value", Value::Atom(Atom::String("yes".into()))),
                    bodies: BTreeMap::new(),
                }],
            },
        );
        let path = Path {
            steps: vec![
                Step {
                    id: "s1".into(),
                    kind: "state.set".into(),
                    params: Dictionary::new()
                        .insert("key", Value::Atom(Atom::String("flag".into())))
                        .insert("value", Value::Atom(Atom::Boolean(true))),
                    bodies: BTreeMap::new(),
                },
                Step {
                    id: "s2".into(),
                    kind: "control.if".into(),
                    params: Dictionary::new().insert("key", Value::Atom(Atom::String("flag".into()))),
                    bodies,
                },
            ],
        };
        let result = executor.run(&path, &Dictionary::new());
        let value = result
            .scope
            .get("result")
            .and_then(|v| v.as_atom())
            .and_then(|a| a.as_str());
        assert_eq!(value, Some("yes"));
    }

    #[test]
    fn executor_runs_control_repeat() {
        let registry = test_registry();
        let executor = Executor::new(&registry);
        let mut bodies = BTreeMap::new();
        bodies.insert(
            "body".into(),
            Path {
                steps: vec![Step {
                    id: "b1".into(),
                    kind: "state.increment".into(),
                    params: Dictionary::new()
                        .insert("key", Value::Atom(Atom::String("counter".into())))
                        .insert("by", Value::Atom(Atom::Decimal(1.0))),
                    bodies: BTreeMap::new(),
                }],
            },
        );
        let path = Path {
            steps: vec![
                Step {
                    id: "s1".into(),
                    kind: "state.set".into(),
                    params: Dictionary::new()
                        .insert("key", Value::Atom(Atom::String("counter".into())))
                        .insert("value", Value::Atom(Atom::Decimal(0.0))),
                    bodies: BTreeMap::new(),
                },
                Step {
                    id: "s2".into(),
                    kind: "control.repeat".into(),
                    params: Dictionary::new().insert("count", Value::Atom(Atom::Decimal(3.0))),
                    bodies,
                },
            ],
        };
        let result = executor.run(&path, &Dictionary::new());
        let counter = result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        assert_eq!(counter, Some(3.0));
    }

    #[test]
    fn compile_to_text_emits_one_line_per_step() {
        let path = Path {
            steps: vec![Step {
                id: "s1".into(),
                kind: "state.increment".into(),
                params: Dictionary::new()
                    .insert("key", Value::Atom(Atom::String("counter".into())))
                    .insert("by", Value::Atom(Atom::Decimal(5.0))),
                bodies: BTreeMap::new(),
            }],
        };
        assert_eq!(compile_to_text(&path), "state.increment(by=5, key=\"counter\");");
    }

    #[test]
    fn compile_to_text_emits_nested_control_blocks() {
        let mut bodies = BTreeMap::new();
        bodies.insert(
            "then".into(),
            Path {
                steps: vec![Step {
                    id: "t1".into(),
                    kind: "log.print".into(),
                    params: Dictionary::new().insert("message", Value::Atom(Atom::String("yes".into()))),
                    bodies: BTreeMap::new(),
                }],
            },
        );
        let path = Path {
            steps: vec![Step {
                id: "s1".into(),
                kind: "control.if".into(),
                params: Dictionary::new().insert("key", Value::Atom(Atom::String("flag".into()))),
                bodies,
            }],
        };
        let text = compile_to_text(&path);
        assert!(text.contains("if (flag)"));
        assert!(text.contains("log.print"));
    }
}
