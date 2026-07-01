//! ⚙️ Headless imperative engine: ordered path of side-effect steps.

use neural_engine::{Dictionary, Registry, Value, SCHEMA_KEY};
use serde::{Deserialize, Serialize};

// #region 🔖Path
/// 👣 One ordered side-effect invocation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: Dictionary,
}

/// 🛤️ Linear ordered list of steps — no edges, position is execution order.
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
        for step in &path.steps {
            let input = scope.merge(&step.params);
            match self.registry.dispatch(&step.kind, &input) {
                Ok(output) => {
                    scope = merge_output_into_scope(&scope, &output);
                    effects.push(EffectLogEntry {
                        step_id: step.id.clone(),
                        kind: step.kind.clone(),
                        input,
                        output: Some(output),
                        error: None,
                    });
                }
                Err(err) => {
                    effects.push(EffectLogEntry {
                        step_id: step.id.clone(),
                        kind: step.kind.clone(),
                        input,
                        output: None,
                        error: Some(err.to_string()),
                    });
                    break;
                }
            }
        }
        RunResult { scope, effects }
    }
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
    path.steps
        .iter()
        .map(|step| {
            let params: Vec<String> = step
                .params
                .keys()
                .filter(|key| key.as_str() != SCHEMA_KEY)
                .map(|key| format!("{}={}", key, format_value(step.params.get(key).expect("key"))))
                .collect();
            if params.is_empty() {
                format!("{}();", step.kind)
            } else {
                format!("{}({});", step.kind, params.join(", "))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Atom(atom) => match atom {
            neural_engine::Atom::Null => "null".into(),
            neural_engine::Atom::Boolean(v) => v.to_string(),
            neural_engine::Atom::Integer(v) => v.to_string(),
            neural_engine::Atom::Decimal(v) => {
                if v.fract().abs() < f64::EPSILON {
                    format!("{:.0}", v)
                } else {
                    v.to_string()
                }
            }
            neural_engine::Atom::String(v) => format!("\"{}\"", v.replace('"', "\\\"")),
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
    registry
}

/// 📚 Merges catalogue sections from all installed imperative modules.
pub fn imperative_catalogue_json(registry: &Registry) -> String {
    let core: serde_json::Value = serde_json::from_str(&imperative_module_core::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let text: serde_json::Value = serde_json::from_str(&imperative_module_text::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let mut sections = core
        .get("sections")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    if let Some(text_sections) = text.get("sections").and_then(|value| value.as_array()) {
        sections.extend(text_sections.iter().cloned());
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
                },
                Step {
                    id: "s2".into(),
                    kind: "state.increment".into(),
                    params: Dictionary::new()
                        .insert("key", Value::Atom(Atom::String("counter".into())))
                        .insert("by", Value::Atom(Atom::Decimal(3.0))),
                },
                Step {
                    id: "s3".into(),
                    kind: "log.print".into(),
                    params: Dictionary::new().insert("message", Value::Atom(Atom::String("done".into()))),
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
    fn compile_to_text_emits_one_line_per_step() {
        let path = Path {
            steps: vec![Step {
                id: "s1".into(),
                kind: "state.increment".into(),
                params: Dictionary::new()
                    .insert("key", Value::Atom(Atom::String("counter".into())))
                    .insert("by", Value::Atom(Atom::Decimal(5.0))),
            }],
        };
        assert_eq!(compile_to_text(&path), "state.increment(by=5, key=\"counter\");");
    }
}
