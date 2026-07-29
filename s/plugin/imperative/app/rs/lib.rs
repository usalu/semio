//! ⚙️ Imperative app — document entities (constitutional: general).

pub use imperative_engine::{Path, Step};
pub use neural_engine::{Dictionary, Registry};

use neural_engine::{Atom, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// #region 🔖Document
/// 📍 Address of a nested step list inside a control step body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
}

/// 📄 Imperative path document envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeDocument {
    pub schema: String,
    pub path: Path,
    #[serde(default)]
    pub seed: Dictionary,
}

impl Default for ImperativeDocument {
    fn default() -> Self {
        Self { schema: "imperative.document".into(), path: Path::new(), seed: Dictionary::new() }
    }
}

//#region 🔖Dsl
/// 🌱 `Value`/`Atom`/`Dictionary`/`Step`/`Path` are all defined in `neural_engine`/`imperative_engine`
/// (foreign crates out of scope for this conversion), so none of them can carry a
/// `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires the impl target type to live in
/// the crate that also owns the trait or the type, and neither is true for a foreign type receiving a
/// foreign derive. `ValueDsl`/`StepNodeDsl`/`PathDsl` below are local structural twins that the real
/// types convert to/from right at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary.
///
/// `ValueDsl` deliberately does NOT route through `dsl_schema`'s built-in `Shape::Value`/`DslValue`
/// dynamic-literal primitive: `DslValue::Number(f64)` merges `Atom::Integer`/`Atom::Decimal` into one
/// case (the OLD hand-rolled printer distinguished them by always giving a `Decimal` a trailing `.`),
/// which is a real, observable loss of fidelity — an existing test parses a bare `1` and expects
/// `Atom::Integer(1)` back. So `ValueDsl` is its own typed record instead: exactly one of its
/// mutually-exclusive `Option` fields is ever `Some`, each keyed so the ACTUAL Rust variant (not a
/// text heuristic) decides which one, which is exactly as precise as the old hand-rolled `Atom` match.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct ValueDsl {
    /// 🕳️ Presence-only flag (the payload is never inspected) — `Atom::Null`'s tag.
    null: Option<bool>,
    #[dsl(key = "bool")]
    boolean: Option<bool>,
    #[dsl(key = "int")]
    integer: Option<i64>,
    decimal: Option<f64>,
    text: Option<String>,
    #[dsl(key = "dict")]
    dictionary: Option<BTreeMap<String, ValueDsl>>,
}

pub fn value_to_value_dsl(value: &Value) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        Value::Atom(Atom::Null) => dsl_value.null = Some(true),
        Value::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        Value::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        Value::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        Value::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        Value::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_map(dict)),
    }
    dsl_value
}

pub fn value_dsl_to_value(dsl_value: &ValueDsl) -> Value {
    if dsl_value.null.is_some() {
        return Value::Atom(Atom::Null);
    }
    if let Some(b) = dsl_value.boolean {
        return Value::Atom(Atom::Boolean(b));
    }
    if let Some(i) = dsl_value.integer {
        return Value::Atom(Atom::Integer(i));
    }
    if let Some(d) = dsl_value.decimal {
        return Value::Atom(Atom::Decimal(d));
    }
    if let Some(s) = &dsl_value.text {
        return Value::Atom(Atom::String(s.clone()));
    }
    match &dsl_value.dictionary {
        Some(entries) => Value::Dictionary(value_dsl_map_to_dictionary(entries)),
        None => Value::Atom(Atom::Null),
    }
}

pub fn dictionary_to_value_dsl_map(dict: &Dictionary) -> BTreeMap<String, ValueDsl> {
    dict.keys().map(|key| (key.clone(), value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")))).collect()
}

pub fn value_dsl_map_to_dictionary(entries: &BTreeMap<String, ValueDsl>) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, (key, value)| dict.insert(key.clone(), value_dsl_to_value(value)))
}

/// 📦 `None` when `dict` is empty, mirroring the old printer's "omit an empty dictionary section".
pub fn dictionary_to_option_dsl_map(dict: &Dictionary) -> Option<BTreeMap<String, ValueDsl>> {
    (!dict.is_empty()).then(|| dictionary_to_value_dsl_map(dict))
}

pub fn option_dsl_map_to_dictionary(entries: Option<BTreeMap<String, ValueDsl>>) -> Dictionary {
    entries.map(|entries| value_dsl_map_to_dictionary(&entries)).unwrap_or_default()
}

/// 👣 `Step`'s recursive `bodies: BTreeMap<String, Path>` mirrors through `Path`'s own single field —
/// `StepNodeDsl` is a one-variant `DslEnum` (not a plain `DslRecord`) purely so the mutual recursion
/// with `PathDsl` goes through `dsl::DslVariants`'s LAZY `fn() -> RecordSpec` pointers: building
/// `PathDsl`'s `RecordSpec` needs `StepNodeDsl`'s variant table, and vice versa, and only the lazy
/// pointer indirection keeps that finite instead of recursing forever just to construct the schema.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum StepNodeDsl {
    Step {
        #[dsl(positional)]
        id: String,
        kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        bodies: BTreeMap<String, PathDsl>,
    },
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct PathDsl {
    #[dsl(statements, block)]
    steps: Vec<StepNodeDsl>,
}

pub fn step_to_step_node_dsl(step: &Step) -> StepNodeDsl {
    StepNodeDsl::Step {
        id: step.id.clone(),
        kind: step.kind.clone(),
        params: dictionary_to_option_dsl_map(&step.params),
        bodies: step.bodies.iter().map(|(slot, path)| (slot.clone(), path_to_path_dsl(path))).collect(),
    }
}

pub fn step_node_dsl_to_step(node: StepNodeDsl) -> Step {
    let StepNodeDsl::Step { id, kind, params, bodies } = node;
    Step { id, kind, params: option_dsl_map_to_dictionary(params), bodies: bodies.into_iter().map(|(slot, path)| (slot, path_dsl_to_path(path))).collect() }
}

pub fn path_to_path_dsl(path: &Path) -> PathDsl {
    PathDsl { steps: path.steps.iter().map(step_to_step_node_dsl).collect() }
}

pub fn path_dsl_to_path(path_dsl: PathDsl) -> Path {
    Path { steps: path_dsl.steps.into_iter().map(step_node_dsl_to_step).collect() }
}

/// 📄 Local mirror of `ImperativeDocument` — see the region's opening doc comment for why `path:
/// Path`/`seed: Dictionary` can't stay as-is under a direct `#[derive(dsl::DslDocument)]`.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "imperative")]
#[dsl(layout = "lines")]
struct ImperativeDocumentDsl {
    schema: String,
    seed: Option<BTreeMap<String, ValueDsl>>,
    #[dsl(statements, block)]
    steps: Vec<StepNodeDsl>,
}

impl store::DocumentDsl for ImperativeDocument {
    const EXTENSION: &'static str = "imperative";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <ImperativeDocumentDsl as store::DocumentDsl>::parse_dsl(text)?;
        Ok(ImperativeDocument {
            schema: parsed.schema,
            path: Path { steps: parsed.steps.into_iter().map(step_node_dsl_to_step).collect() },
            seed: option_dsl_map_to_dictionary(parsed.seed),
        })
    }

    fn print_dsl(&self) -> String {
        let mirror = ImperativeDocumentDsl {
            schema: self.schema.clone(),
            seed: dictionary_to_option_dsl_map(&self.seed),
            steps: self.path.steps.iter().map(step_to_step_node_dsl).collect(),
        };
        <ImperativeDocumentDsl as store::DocumentDsl>::print_dsl(&mirror)
    }
}

//#region 🔖Pack
/// 📦 Binary counterpart of the `store::DocumentDsl` impl above. `ImperativeDocument` carries a manual
/// `DocumentDsl` impl (not `#[derive(dsl::DslDocument)]` itself — see the region's opening doc comment)
/// so it did NOT automatically gain `store::DocumentPack` from `dsl_derive`'s expansion in wave 1. This
/// mirrors the derive-emitted shape (wave1-report.txt §2) exactly, substituting `ImperativeDocumentDsl`'s
/// `__dsl_spec`/`__dsl_to_record`/`__dsl_from_record` trio for `Self`'s (unavailable here) and routing
/// the same mirror-struct conversion `parse_dsl`/`print_dsl` already use.
impl store::DocumentPack for ImperativeDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let mirror = ImperativeDocumentDsl {
            schema: self.schema.clone(),
            seed: dictionary_to_option_dsl_map(&self.seed),
            steps: self.path.steps.iter().map(step_to_step_node_dsl).collect(),
        };
        store::pack_rt::encode_document(&ImperativeDocumentDsl::__dsl_spec(), &mirror.__dsl_to_record(), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (record, _report) = store::pack_rt::decode_document(bytes, &ImperativeDocumentDsl::__dsl_spec(), options)?;
        let parsed = ImperativeDocumentDsl::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;
        Ok(ImperativeDocument {
            schema: parsed.schema,
            path: Path { steps: parsed.steps.into_iter().map(step_node_dsl_to_step).collect() },
            seed: option_dsl_map_to_dictionary(parsed.seed),
        })
    }
}
//#endregion 🔖Pack
//#endregion 🔖Dsl
// #endregion 🔖Document
