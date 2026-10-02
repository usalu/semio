# Neutral Value Type Original Inputs

Original full bytes captured before this cut. These are source receipts, not compiler/runtime evidence.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs

Bytes 128775; SHA-256 d960ab80d8b545649f24ec23cfb60b061249c3b032a139c473991efc9d737e4b.

````text
//! 🧠️ Headless neural engine: dictionary in, dictionary out.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::mem::ManuallyDrop;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// 🔮️ Test-only: production moved to `ToValue`/`FromValue` below (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass — see `📓️orderedmap-tenth-seam.md`).
#[cfg(test)]
use serde::{Deserialize, Serialize};
use protocol::value::ordered::OrderedMap;
use protocol::value::{DslValue, FromValue, Number, ToValue, ValueError};

#[path = "🧵️retirement/🦀️.rs"]
pub mod retirement;
pub use retirement::{ColdDictionaryBuilder, ColdValueOwner, ValueRetirement, ValueRetirementStep};

#[path = "🧊️cold/🦀️.rs"]
pub mod cold;
pub use cold::{ColdOwner, ColdRetire};

#[path = "📔️registry/🦀️.rs"]
pub mod registry;
pub use registry::{RegistryRetirement, SharedRegistry};

// #region 🔖️Dictionary
/// 📚️ Immutable, unordered, collision-free key-value collection. `serde` is TEST-ONLY
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass — see
/// `📓️orderedmap-tenth-seam.md`): production now routes through the hand-written `ToValue`/
/// `FromValue` below, mirroring `OrderedMap<V>`'s own `#[cfg(test)]`-gated `Serialize` in
/// `🌱️value/🗂️ordered/🦀️.rs`. All three former blockers moved off serde in this pass — the `Value`
/// enum's derive, `Neuron`'s derive, and the `serde_json::to_string(&merged)` call in the
/// evaluator's pending-extension branch (now `pack::json::to_json_string(&merged.to_value())`).
/// No `#[derive(Serialize)]` here (not even test-gated): `OrderedMap<Value>`'s own `Serialize` is
/// `#[cfg(test)]`-gated inside `replication`'s OWN compilation unit — cfg(test) never crosses a
/// crate boundary, so it stays invisible when `replication` is pulled in as this crate's ordinary
/// (non-test) dependency, even while THIS crate's tests run. The oracle `Serialize` below is
/// hand-written instead, over `Value: Serialize` (local to this crate, genuinely test-cfg'd here).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dictionary {
    pairs: OrderedMap<Value>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_schema(schema: impl Into<String>) -> Self {
        Self::new().insert(SCHEMA_KEY, Value::Atom(Atom::String(schema.into())))
    }

    /// 🧊️ Explicit synchronous dictionary construction; retained callers use immutable sharing and typed update cursors.
    pub fn insert(self, key: impl Into<String>, value: Value) -> Self {
        let mut builder = ColdDictionaryBuilder::from_dictionary(self);
        builder.insert(key.into(), value);
        builder.finish()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.pairs.get(key)
    }

    pub fn schema(&self) -> Option<&str> {
        self.get(SCHEMA_KEY).and_then(|v| v.as_atom()).and_then(|a| a.as_str())
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.pairs.keys()
    }

    /// 🔎️ Borrows ordered entries without cloning nested values.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&String, &Value)> + ExactSizeIterator {
        self.pairs.iter()
    }

    /// 📤️ Moves exact dictionary ownership into nested byte-aware retirement without cloning values.
    pub fn into_retirement(self) -> ValueRetirement { ValueRetirement::from_dictionary(self) }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn merge(&self, other: &Dictionary) -> Dictionary {
        let mut builder = ColdDictionaryBuilder::from_dictionary(self.clone());
        for (k, v) in &other.pairs {
            builder.insert(k.clone(), v.clone());
        }
        builder.finish()
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        if let Err(_retirement) = std::mem::take(&mut self.pairs).release_shared() {
            assert!(std::thread::panicking(), "final Dictionary ownership must be explicitly retired or owned by a cold boundary");
        }
    }
}

/// 🧊️ Test-only oracle mirror of the hand-written `Deserialize` below; manual (not derived) for the
/// same reason `Deserialize` is hand-written rather than delegating to `OrderedMap`'s own — see
/// `Dictionary`'s struct docstring above. Serializes each entry directly over `Value: Serialize`.
#[cfg(test)]
impl Serialize for Dictionary {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (key, value) in self.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

/// 🧊️ Cold decoding stages every replacement and partial failure in a domain-aware builder.
/// 🔮️ Test-only — production decoding routes through `FromValue` below.
#[cfg(test)]
impl<'de> Deserialize<'de> for Dictionary {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Dictionary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a neural dictionary") }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut builder = ColdDictionaryBuilder::new();
                while let Some((key, value)) = access.next_entry::<String, Value>()? { builder.insert(key, value); }
                Ok(builder.finish())
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

/// 🔁️ First-party analog of the hand-written `Deserialize` above — routes through the same
/// `ColdDictionaryBuilder` retirement contract, never constructs `Dictionary` fields directly.
impl ToValue for Dictionary {
    fn to_value(&self) -> DslValue {
        DslValue::Object(self.iter().map(|(key, value)| (key.clone(), value.to_value())).collect())
    }
}

impl FromValue for Dictionary {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { return Err(ValueError::new("expected an object for Dictionary")) };
        let mut builder = ColdDictionaryBuilder::new();
        for (key, entry) in entries {
            builder.insert(key, Value::from_value(entry)?);
        }
        Ok(builder.finish())
    }
}

/// 🔑️ Dot-separated camelCase segment path.
pub type Key = String;

/// 💎️ Atom or nested dictionary. `serde` is TEST-ONLY — see `Dictionary`'s docstring above.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(untagged))]
pub enum Value {
    Atom(Atom),
    Dictionary(Dictionary),
}

impl Value {
    pub fn null() -> Self {
        Self::Atom(Atom::Null)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Atom(Atom::Null))
    }

    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Value::Atom(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_dictionary(&self) -> Option<&Dictionary> {
        match self {
            Value::Dictionary(d) => Some(d),
            _ => None,
        }
    }
}

/// 🔁️ Mirrors `#[serde(untagged)]` above: an object decodes as `Dictionary`, anything else as `Atom`.
impl ToValue for Value {
    fn to_value(&self) -> DslValue {
        match self {
            Value::Atom(atom) => atom.to_value(),
            Value::Dictionary(dictionary) => dictionary.to_value(),
        }
    }
}

impl FromValue for Value {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            object @ DslValue::Object(_) => Ok(Value::Dictionary(Dictionary::from_value(object)?)),
            other => Ok(Value::Atom(Atom::from_value(other)?)),
        }
    }
}

/// ⚛️ Immutable non-dictionary value. `serde` is TEST-ONLY — see `Dictionary`'s docstring above.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(untagged))]
pub enum Atom {
    Null,
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    String(String),
}

impl Atom {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Atom::Boolean(b) => Some(*b),
            Atom::Integer(i) => Some(*i != 0),
            Atom::Decimal(d) => Some(*d != 0.0),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Atom::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            Atom::Integer(i) => Some(*i as f64),
            Atom::Decimal(d) => Some(*d),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Atom::String(s) => Some(s),
            _ => None,
        }
    }
}

/// 🔁️ `DslValue::Number` carries its own exact variant, so each one maps to exactly one `Atom`:
/// `Int`/`UInt` to `Atom::Integer` and `Float` to `Atom::Decimal`.
///
/// 🐛️ `Float` used to take a whole-valued shortcut back to `Integer`, which made the round trip
/// LOSSY at the one hop that is supposed to preserve the carrier: `Atom::Decimal(42.0)` prints as
/// `42.0` (`pack::json`'s `write_float`), parses back as `Number::Float(42.0)` — the JSON layer
/// distinguishes `42` from `42.0` by the text precisely so it does not have to guess — and then
/// became `Atom::Integer(42)` here. Every `flowEvalResolve` seeds a node cache from exactly this
/// JSON (`seed_flow_eval_node_cache`), so a decimal output came back from an extension hop as an
/// integer and compared unequal to the value the kernel had produced
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
impl ToValue for Atom {
    fn to_value(&self) -> DslValue {
        match self {
            Atom::Null => DslValue::Null,
            Atom::Boolean(b) => DslValue::Bool(*b),
            Atom::Integer(i) => DslValue::Number(Number::Int(*i)),
            Atom::Decimal(d) => DslValue::Number(Number::Float(*d)),
            Atom::String(s) => DslValue::String(s.clone()),
        }
    }
}

impl FromValue for Atom {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(Atom::Null),
            DslValue::Bool(b) => Ok(Atom::Boolean(b)),
            DslValue::Number(Number::Int(value)) => Ok(Atom::Integer(value)),
            DslValue::Number(Number::UInt(value)) => Ok(Atom::Integer(value as i64)),
            DslValue::Number(Number::Float(value)) => Ok(Atom::Decimal(value)),
            DslValue::String(s) => Ok(Atom::String(s)),
            DslValue::Bytes(_) => Err(ValueError::new("expected an atom, found bytes")),
            DslValue::Array(_) | DslValue::Object(_) => Err(ValueError::new("expected an atom, found an array or object")),
        }
    }
}
// #endregion 🔖️Dictionary

// #region 🔖️Schema
pub const SCHEMA_KEY: &str = "$schema";

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", tag = "kind", content = "of"))]
pub enum ValueType {
    Boolean,
    Integer,
    Decimal,
    Text,
    List(Box<ValueType>),
    Schema(String),
    #[default]
    Any,
}

impl ValueType {
    pub fn id(&self) -> String {
        match self {
            ValueType::Boolean => "boolean".into(),
            ValueType::Integer => "integer".into(),
            ValueType::Decimal => "number".into(),
            ValueType::Text => "text".into(),
            ValueType::List(_) => "list".into(),
            ValueType::Schema(id) => id.clone(),
            ValueType::Any => "value".into(),
        }
    }

    pub fn matches(&self, value: &Value) -> bool {
        if value.is_null() {
            return false;
        }
        match self {
            ValueType::Any => true,
            ValueType::Boolean => value.as_atom().is_some_and(|a| matches!(a, Atom::Boolean(_))),
            ValueType::Integer => value.as_atom().is_some_and(|a| matches!(a, Atom::Integer(_))),
            ValueType::Decimal => value.as_atom().and_then(|a| a.as_f64()).is_some(),
            ValueType::Text => value.as_atom().and_then(|a| a.as_str()).is_some(),
            ValueType::List(_) => value.as_dictionary().is_some_and(|d| d.schema() == Some("list")),
            ValueType::Schema(schema) => value.as_dictionary().is_some_and(|d| d.schema() == Some(schema.as_str())),
        }
    }
}

/// 🔁️ Mirrors `#[serde(tag = "kind", content = "of")]` above (`serde` stays unconditional here —
/// `ValueType` never touches `Dictionary`/`Value`, so it was never part of the tenth-seam blocker).
impl ToValue for ValueType {
    fn to_value(&self) -> DslValue {
        match self {
            ValueType::Boolean => DslValue::object([("kind".to_string(), "boolean".to_value())]),
            ValueType::Integer => DslValue::object([("kind".to_string(), "integer".to_value())]),
            ValueType::Decimal => DslValue::object([("kind".to_string(), "decimal".to_value())]),
            ValueType::Text => DslValue::object([("kind".to_string(), "text".to_value())]),
            ValueType::List(inner) => DslValue::object([("kind".to_string(), "list".to_value()), ("of".to_string(), inner.to_value())]),
            ValueType::Schema(id) => DslValue::object([("kind".to_string(), "schema".to_value()), ("of".to_string(), id.to_value())]),
            ValueType::Any => DslValue::object([("kind".to_string(), "any".to_value())]),
        }
    }
}

impl FromValue for ValueType {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let kind = value.get("kind").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("kind"))?;
        match kind.as_str() {
            "boolean" => Ok(ValueType::Boolean),
            "integer" => Ok(ValueType::Integer),
            "decimal" => Ok(ValueType::Decimal),
            "text" => Ok(ValueType::Text),
            "list" => Ok(ValueType::List(Box::new(value.get("of").cloned().map(ValueType::from_value).transpose()?.ok_or_else(|| ValueError::new("of"))?))),
            "schema" => Ok(ValueType::Schema(value.get("of").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("of"))?)),
            "any" => Ok(ValueType::Any),
            other => Err(ValueError::new(format!("unknown ValueType kind '{other}'"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FieldSpec {
    pub key: String,
    pub value: ValueType,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub default: Option<Value>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
}

impl FieldSpec {
    pub fn new(key: impl Into<String>, value: ValueType) -> Self {
        Self { key: key.into(), value, default: None, label: None }
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default.replace(default).retire_cold();
        self
    }

    pub fn decimal(key: impl Into<String>) -> Self {
        Self::new(key, ValueType::Decimal)
    }

    pub fn decimal_default(key: impl Into<String>, default: f64) -> Self {
        Self::decimal(key).with_default(Value::Atom(Atom::Decimal(default)))
    }
}

impl ToValue for FieldSpec {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("key".into(), self.key.to_value()),
            ("value".into(), self.value.to_value()),
            ("default".into(), self.default.to_value()),
            ("label".into(), self.label.to_value()),
        ])
    }
}

impl FromValue for FieldSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for FieldSpec")) };
        Ok(Self {
            key: value.get("key").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("key"))?,
            value: value.get("value").cloned().map(ValueType::from_value).transpose()?.ok_or_else(|| ValueError::new("value"))?,
            default: value.get("default").cloned().map(Option::<Value>::from_value).transpose()?.unwrap_or_default(),
            label: value.get("label").cloned().map(Option::<String>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🔮️ `serde` is TEST-ONLY — see `Dictionary`'s docstring; `fields: Vec<FieldSpec>` fans in `Value` transitively.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Schema {
    pub id: String,
    pub module: String,
    pub name: String,
    pub icon: String,
    pub summary: String,
    pub fields: Vec<FieldSpec>,
}

impl ToValue for Schema {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("module".into(), self.module.to_value()),
            ("name".into(), self.name.to_value()),
            ("icon".into(), self.icon.to_value()),
            ("summary".into(), self.summary.to_value()),
            ("fields".into(), self.fields.to_value()),
        ])
    }
}

impl FromValue for Schema {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for Schema")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            module: value.get("module").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("module"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("name"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("icon"))?,
            summary: value.get("summary").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("summary"))?,
            fields: value.get("fields").cloned().map(Vec::<FieldSpec>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🏷️ Schema id with display metadata for pickers.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SchemaRef {
    pub id: String,
    pub name: String,
    pub icon: String,
}

impl ToValue for SchemaRef {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("id".into(), self.id.to_value()), ("name".into(), self.name.to_value()), ("icon".into(), self.icon.to_value())])
    }
}

impl FromValue for SchemaRef {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for SchemaRef")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("name"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("icon"))?,
        })
    }
}

impl Schema {
    pub fn validate(&self, dictionary: &Dictionary) -> Result<(), EvalError> {
        match dictionary.schema() {
            Some(id) if id == self.id => {}
            Some(id) => return Err(EvalError::InvalidInput(format!("schema {id} does not match {}", self.id))),
            None => return Err(EvalError::MissingInput(SCHEMA_KEY.into())),
        }
        for field in &self.fields {
            let Some(value) = dictionary.get(&field.key) else {
                return Err(EvalError::MissingInput(field.key.clone()));
            };
            if !field.value.matches(value) {
                return Err(EvalError::InvalidInput(format!("field {} does not match {}", field.key, field.value.id())));
            }
        }
        Ok(())
    }

    pub fn default_dictionary(&self) -> Dictionary {
        let mut dictionary = Dictionary::with_schema(self.id.clone());
        for field in &self.fields {
            if let Some(default) = &field.default {
                dictionary = dictionary.insert(field.key.clone(), default.clone());
            }
        }
        dictionary
    }
}
// #endregion 🔖️Schema

//#region ⚠️ Errors
/// 🚨️ Schema field/channel conversion, instance-read, and cardinality-parse failures.
#[derive(Clone, Debug, PartialEq)]
pub enum NeuralEngineError {
    /// 🔢️ Atom value doesn't match the scalar type a schema field declares.
    FieldTypeMismatch { kind: &'static str, reason: &'static str },
    /// 📦️ Field value isn't a dictionary where a list/schema/any field required one.
    FieldNotDictionary,
    /// 🕳️ Channel value is null where a value was required.
    ChannelNull,
    /// 📦️ Channel value isn't the wrapping dictionary its scalar type expects.
    ChannelTypeMismatch { kind: &'static str },
    /// 📦️ Channel value isn't a dictionary where a list/schema/any channel required one.
    ChannelNotDictionary,
    /// 🕳️ Channel dictionary is missing its `value` entry.
    ChannelMissingValue { kind: &'static str },
    /// 🔍️ A schema instance or field channel is absent from the input dictionary.
    Missing(String),
    /// ⚠️ A schema instance dictionary carries the wrong `$schema` tag.
    Invalid(String),
    /// 🔍️ A declared schema field is absent from the constructed dictionary.
    MissingField(String),
    /// 🚫️ Neither an instance nor any field inputs were provided to a schema component.
    NoInputProvided,
    /// 🔢️ A cardinality symbol string didn't parse to a known cardinality.
    InvalidCardinality(String),
    /// 🧬️ The constructed/modified dictionary failed schema validation.
    Validation(EvalError),
}

impl std::fmt::Display for NeuralEngineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldTypeMismatch { kind, reason } => write!(formatter, "{kind} field is not {reason}"),
            Self::FieldNotDictionary => formatter.write_str("field is not a dictionary"),
            Self::ChannelNull => formatter.write_str("channel value is null"),
            Self::ChannelTypeMismatch { kind } => write!(formatter, "{kind} channel is not a dictionary"),
            Self::ChannelNotDictionary => formatter.write_str("channel is not a dictionary"),
            Self::ChannelMissingValue { kind } => write!(formatter, "{kind} channel is missing value"),
            Self::Missing(detail) => write!(formatter, "missing {detail}"),
            Self::Invalid(detail) => write!(formatter, "invalid {detail}"),
            Self::MissingField(field) => write!(formatter, "missing field {field}"),
            Self::NoInputProvided => formatter.write_str("no instance or field inputs provided"),
            Self::InvalidCardinality(value) => write!(formatter, "invalid cardinality: {value}"),
            Self::Validation(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for NeuralEngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(error) => Some(error),
            _ => None,
        }
    }
}

impl From<EvalError> for NeuralEngineError {
    fn from(error: EvalError) -> Self {
        Self::Validation(error)
    }
}
//#endregion ⚠️ Errors

// #region 🔖️SchemaComponent
fn schema_component_operator_id(schema: &Schema) -> String {
    format!("{}.{}", schema.module, schema.id)
}

fn should_auto_register_schema_component(schema: &Schema) -> bool {
    schema.module != "core" && !schema.fields.is_empty() && schema.id != "list" && schema.id != "dictionary"
}

fn schema_field_input_cardinality(value: &ValueType) -> Cardinality {
    match value {
        ValueType::List(_) => Cardinality::ZeroOrMore,
        _ => Cardinality::ZeroOrOne,
    }
}

fn schema_field_output_cardinality(value: &ValueType) -> Cardinality {
    match value {
        ValueType::List(_) => Cardinality::ZeroOrMore,
        _ => Cardinality::ExactlyOne,
    }
}

/// 🔤️ The value schemas a declared schema FIELD carries, in `ChannelSpec::value_types` vocabulary.
/// `Integer` and `Decimal` collapse to one `number` port type — whether a number is whole is an
/// input-validation concern, never a wiring one — and `Any` declares nothing, which stays connectable.
fn field_channel_value_types(value: &ValueType) -> Vec<String> {
    match value {
        ValueType::Any => Vec::new(),
        ValueType::List(_) => vec![VALUE_TYPE_LIST.into()],
        ValueType::Integer | ValueType::Decimal => vec![VALUE_TYPE_NUMBER.into()],
        _ => vec![value.id()],
    }
}

fn field_channel_operators(value: &ValueType) -> Vec<String> {
    match value {
        ValueType::List(inner) => vec![inner.id()],
        _ => vec![value.id()],
    }
}

/// 🔢️ The port type every numeric channel carries — one `Dictionary::with_schema("number")`.
pub const VALUE_TYPE_NUMBER: &str = "number";

/// 🔤️ The port type every text channel carries.
pub const VALUE_TYPE_TEXT: &str = "text";

/// ☑️ The port type every boolean channel carries.
pub const VALUE_TYPE_BOOLEAN: &str = "boolean";

/// 📃️ The port type every list channel carries — `Dictionary::with_schema("list")`, whatever the items are.
pub const VALUE_TYPE_LIST: &str = "list";

/// 🧭️ The port type a `math` vector carries.
pub const VALUE_TYPE_VECTOR: &str = "vector";

/// 📍️ The port type a `math` point carries.
pub const VALUE_TYPE_POINT: &str = "point";

/// 🔷️ The port type every brep kernel handle carries — `Dictionary::with_schema("geometry")`, whether
/// the handle is a vertex, a wire, a face, a surface, a solid or a compound.
pub const VALUE_TYPE_GEOMETRY: &str = "geometry";

/// 🏷️ The id an OUTPUT channel takes when it carries the same noun as one of its operator's own
/// INPUTS. An operator's input ids and output ids are disjoint, because `"{nodeId}@{portId}"` is the
/// one public name a wire endpoint has: a graph, a journalled edit and a pointer press all address a
/// port by that string, so a port id that names a channel on both sides names two handles and the
/// press resolves to whichever the lookup happened to reach first. The display names (`code`,
/// `abbreviation`, `fullName`) are untouched — this is an identity, not a label.
///
/// @see `✏️s/🧑‍💻dev/🌊️flow/🧫️fixtures/🔌️port-sides/🔣️.json` — the catalogue-wide law
pub fn produced_channel_id(input_id: &str) -> String {
    format!("{input_id}Out")
}

/// 🧩️ Builds construct/deconstruct/modify operator metadata for a schema.
pub fn schema_component_info(schema: &Schema) -> OperatorInfo {
    let operator_id = schema_component_operator_id(schema);
    let mut inputs = vec![ChannelSpec::requires(&schema.id, &[schema.id.as_str()]).with_value_types(&[schema.id.as_str()]).with_cardinality(Cardinality::ZeroOrOne)];
    for field in &schema.fields {
        let operators = field_channel_operators(&field.value);
        inputs.push(ChannelSpec::requires(&field.key, &operators).with_value_types(&field_channel_value_types(&field.value)).with_cardinality(schema_field_input_cardinality(&field.value)));
    }
    let (instance_code, instance_abbreviation, instance_full_name) = derive_channel_names(&schema.id);
    let mut outputs = vec![ChannelSpec::named(instance_code, instance_abbreviation, produced_channel_id(&schema.id), instance_full_name).with_operators(vec![schema.id.clone()]).with_value_types(&[schema.id.as_str()])];
    for field in &schema.fields {
        let (code, abbreviation, full_name) = derive_channel_names(&field.key);
        outputs.push(ChannelSpec::named(code, abbreviation, produced_channel_id(&field.key), full_name).with_operators(field_channel_operators(&field.value)).with_value_types(&field_channel_value_types(&field.value)).with_cardinality(schema_field_output_cardinality(&field.value)));
    }
    outputs.push(ChannelSpec::list_output("errors", vec![]));
    OperatorInfo {
        id: operator_id,
        extension: schema.module.clone(),
        name: schema.name.clone(),
        abbreviation: schema.name.clone(),
        icon: schema.icon.clone(),
        summary: format!("Constructs, deconstructs, or modifies {}", schema.name),
        inputs,
        outputs,
        group: vec!["Schemas".into()],
        ..Default::default()
    }
}

fn schema_errors_list(messages: &[String]) -> Dictionary {
    let mut list = Dictionary::with_schema("list");
    for (index, message) in messages.iter().enumerate() {
        list = list.insert(index.to_string(), Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(message.clone())))));
    }
    list
}

fn schema_input_present(input: &Dictionary, key: &str) -> bool {
    input.get(key).is_some_and(|value| !value.is_null())
}

fn field_to_channel(value: &Value, value_type: &ValueType) -> Result<Value, NeuralEngineError> {
    match value_type {
        ValueType::Decimal => {
            let number = value.as_atom().and_then(|atom| atom.as_f64()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "decimal", reason: "numeric" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(number)))))
        }
        ValueType::Integer => {
            let number = value
                .as_atom()
                .and_then(|atom| match atom {
                    Atom::Integer(value) => Some(*value),
                    Atom::Decimal(value) => Some(value.round() as i64),
                    _ => None,
                })
                .ok_or(NeuralEngineError::FieldTypeMismatch { kind: "integer", reason: "integral" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Integer(number)))))
        }
        ValueType::Boolean => {
            let boolean = value.as_atom().and_then(|atom| atom.as_bool()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "boolean", reason: "boolean" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(boolean)))))
        }
        ValueType::Text => {
            let text = value.as_atom().and_then(|atom| atom.as_str()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "text", reason: "text" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(text.to_string())))))
        }
        ValueType::List(_) | ValueType::Schema(_) | ValueType::Any => value.as_dictionary().cloned().map(Value::Dictionary).ok_or(NeuralEngineError::FieldNotDictionary),
    }
}

fn channel_to_field(value: &Value, value_type: &ValueType) -> Result<Value, NeuralEngineError> {
    if value.is_null() {
        return Err(NeuralEngineError::ChannelNull);
    }
    match value_type {
        ValueType::Decimal => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "decimal" })?;
            let number = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_f64()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "decimal" })?;
            Ok(Value::Atom(Atom::Decimal(number)))
        }
        ValueType::Integer => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "integer" })?;
            let number = dictionary
                .get("value")
                .and_then(|entry| entry.as_atom())
                .and_then(|atom| match atom {
                    Atom::Integer(value) => Some(*value),
                    Atom::Decimal(value) => Some(value.round() as i64),
                    _ => None,
                })
                .ok_or(NeuralEngineError::ChannelMissingValue { kind: "integer" })?;
            Ok(Value::Atom(Atom::Integer(number)))
        }
        ValueType::Boolean => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "boolean" })?;
            let boolean = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_bool()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "boolean" })?;
            Ok(Value::Atom(Atom::Boolean(boolean)))
        }
        ValueType::Text => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "text" })?;
            let text = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_str()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "text" })?;
            Ok(Value::Atom(Atom::String(text.to_string())))
        }
        ValueType::List(_) | ValueType::Schema(_) | ValueType::Any => value.as_dictionary().cloned().map(Value::Dictionary).ok_or(NeuralEngineError::ChannelNotDictionary),
    }
}

fn read_schema_instance<'a>(input: &'a Dictionary, schema: &Schema) -> Result<&'a Dictionary, NeuralEngineError> {
    let instance = input.get(&schema.id).and_then(|value| value.as_dictionary()).ok_or_else(|| NeuralEngineError::Missing(schema.id.clone()))?;
    if instance.schema() != Some(schema.id.as_str()) {
        return Err(NeuralEngineError::Invalid(schema.id.clone()));
    }
    Ok(instance)
}

fn read_schema_field_input(input: &Dictionary, field: &FieldSpec) -> Result<Value, NeuralEngineError> {
    let value = input.get(&field.key).ok_or_else(|| NeuralEngineError::Missing(field.key.clone()))?;
    channel_to_field(value, &field.value)
}

/// 🧩️ Construct, deconstruct, or modify dictionaries for one schema.
pub struct SchemaComponent {
    pub schema: Schema,
}

impl SchemaComponent {
    fn construct(&self, input: &Dictionary, provided: &[&FieldSpec]) -> Result<Dictionary, NeuralEngineError> {
        let mut dictionary = ColdDictionaryBuilder::from_dictionary(self.schema.default_dictionary());
        for field in provided {
            dictionary.insert(field.key.clone(), read_schema_field_input(input, field)?);
        }
        for field in &self.schema.fields {
            if dictionary.dictionary().get(&field.key).is_none() {
                return Err(NeuralEngineError::MissingField(field.key.clone()));
            }
        }
        self.schema.validate(dictionary.dictionary())?;
        Ok(dictionary.finish())
    }

    fn deconstruct(&self, input: &Dictionary) -> Result<Dictionary, NeuralEngineError> {
        let instance = read_schema_instance(input, &self.schema)?;
        self.schema.validate(instance)?;
        Ok(instance.clone())
    }

    fn modify(&self, input: &Dictionary, provided: &[&FieldSpec]) -> Result<Dictionary, NeuralEngineError> {
        let mut dictionary = ColdDictionaryBuilder::from_dictionary(read_schema_instance(input, &self.schema)?.clone());
        for field in provided {
            dictionary.insert(field.key.clone(), read_schema_field_input(input, field)?);
        }
        self.schema.validate(dictionary.dictionary())?;
        Ok(dictionary.finish())
    }

    fn success_output(&self, instance: &Dictionary) -> Result<Dictionary, NeuralEngineError> {
        let mut output = ColdDictionaryBuilder::from_dictionary(Dictionary::new().insert(produced_channel_id(&self.schema.id), Value::Dictionary(instance.clone())));
        for field in &self.schema.fields {
            // 🛡️ `instance` only reaches here after `Schema::validate` confirmed every
            // declared field.key is present, so this lookup can never miss.
            let value = instance.get(&field.key).expect("validated field");
            let channel = field_to_channel(value, &field.value)?;
            output.insert(produced_channel_id(&field.key), channel);
        }
        output.insert("errors".into(), Value::Dictionary(schema_errors_list(&[])));
        Ok(output.finish())
    }

    fn error_output(&self, messages: &[String]) -> Dictionary {
        let mut output = Dictionary::new().insert(produced_channel_id(&self.schema.id), Value::null()).insert("errors", Value::Dictionary(schema_errors_list(messages)));
        for field in &self.schema.fields {
            output = output.insert(produced_channel_id(&field.key), Value::null());
        }
        output
    }
}

impl Operator for SchemaComponent {
        fn retire_cold(self: Box<Self>) { self.schema.retire_cold(); }

    fn retirement_is_empty(&self) -> bool {
        self.schema.id.is_empty() && self.schema.module.is_empty() && self.schema.name.is_empty() && self.schema.icon.is_empty() && self.schema.summary.is_empty() && self.schema.fields.is_empty()
    }

    fn retire_step(&mut self, maximum_items: usize, maximum_bytes: usize, values: &mut ValueRetirement) -> Result<ValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(ValueRetirementStep::Blocked); }
        if self.retirement_is_empty() { return Ok(ValueRetirementStep::Complete); }
        values.push_schema(std::mem::take(&mut self.schema));
        Ok(ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let has_instance = schema_input_present(input, &self.schema.id);
        let provided: Vec<&FieldSpec> = self.schema.fields.iter().filter(|field| schema_input_present(input, &field.key)).collect();
        let has_fields = !provided.is_empty();
        let result = match (has_instance, has_fields) {
            (false, false) => Err(NeuralEngineError::NoInputProvided),
            (false, true) => self.construct(input, &provided),
            (true, false) => self.deconstruct(input),
            (true, true) => self.modify(input, &provided),
        };
        Ok(match result.and_then(|instance| self.success_output(&ColdOwner::new(instance))) {
            Ok(output) => output,
            Err(error) => self.error_output(&[error.to_string()]),
        })
    }
}
// #endregion 🔖️SchemaComponent

// #region 🔖️Tree
/// 🌳️ Directed acyclic graph of neurons and synapses. `serde` is TEST-ONLY — see `Dictionary`'s
/// docstring; mutually recursive with `Neuron` via `Neuron.tree: Option<Box<Tree>>`, so both moved
/// off serde together.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct Tree {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
}

/// 🔵️ Neuron instance bound to a kind. `serde` is TEST-ONLY — see `Tree`'s docstring above.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct Neuron {
    pub id: String,
    pub kind: String,
    #[cfg_attr(test, serde(default))]
    pub params: Dictionary,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub tree: Option<Box<Tree>>,
}

impl Neuron {
    pub fn with_kind(id: impl Into<String>, kind: impl Into<String>, params: Dictionary) -> Self {
        Self { id: id.into(), kind: kind.into(), params, tree: None }
    }
}

impl Tree {
    /// 📜️ Derives contract input and output channels from boundary neurons.
    pub fn contract(&self) -> (Vec<ChannelSpec>, Vec<ChannelSpec>) {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for neuron in &self.neurons {
            let (channel_id, operators) = contract_channel(neuron);
            if neuron.kind == INPUT_KIND {
                inputs.push(ChannelSpec::requires(channel_id, &operators));
            } else if neuron.kind == OUTPUT_KIND {
                outputs.push(ChannelSpec::provides(channel_id, operators));
            }
        }
        (inputs, outputs)
    }
}

fn default_from_port() -> String {
    String::new()
}

fn default_to_port() -> String {
    String::new()
}

/// 🔗️ Directed connection between two port endpoints.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Synapse {
    pub id: String,
    pub from: String,
    pub to: String,
    #[cfg_attr(test, serde(default = "default_from_port"))]
    pub from_port: String,
    #[cfg_attr(test, serde(default = "default_to_port"))]
    pub to_port: String,
}

impl ToValue for Synapse {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("from".into(), self.from.to_value()),
            ("to".into(), self.to.to_value()),
            ("fromPort".into(), self.from_port.to_value()),
            ("toPort".into(), self.to_port.to_value()),
        ])
    }
}

impl FromValue for Synapse {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for Synapse")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            from: value.get("from").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("from"))?,
            to: value.get("to").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("to"))?,
            from_port: value.get("fromPort").cloned().map(String::from_value).transpose()?.unwrap_or_else(default_from_port),
            to_port: value.get("toPort").cloned().map(String::from_value).transpose()?.unwrap_or_else(default_to_port),
        })
    }
}

impl ToValue for Neuron {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("kind".into(), self.kind.to_value()),
            ("params".into(), self.params.to_value()),
            ("tree".into(), self.tree.to_value()),
        ])
    }
}

impl FromValue for Neuron {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for Neuron")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            kind: value.get("kind").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("kind"))?,
            params: value.get("params").cloned().map(Dictionary::from_value).transpose()?.unwrap_or_default(),
            tree: value.get("tree").cloned().map(Option::<Box<Tree>>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

impl ToValue for Tree {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("neurons".into(), self.neurons.to_value()), ("synapses".into(), self.synapses.to_value())])
    }
}

impl FromValue for Tree {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for Tree")) };
        Ok(Self {
            neurons: value.get("neurons").cloned().map(Vec::<Neuron>::from_value).transpose()?.unwrap_or_default(),
            synapses: value.get("synapses").cloned().map(Vec::<Synapse>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}
// #endregion 🔖️Tree

// #region 🔖️Contract
pub const INPUT_KIND: &str = "input";
pub const OUTPUT_KIND: &str = "output";
pub const CLUSTER_KIND: &str = "cluster";

fn contract_channel(neuron: &Neuron) -> (String, Vec<String>) {
    let channel_id = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
    let operators = neuron.params.get("operators").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|raw| raw.split(',').map(str::trim).filter(|entry| !entry.is_empty()).map(str::to_string).collect()).unwrap_or_default();
    (channel_id, operators)
}

/// 🧩️ Builds operator metadata for a cluster neuron from its inner contract.
pub fn cluster_operator_info(id: &str, name: &str, tree: &Tree) -> OperatorInfo {
    let (inputs, outputs) = tree.contract();
    OperatorInfo { id: id.into(), extension: "flow".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:🧩️".into(), summary: "Nested tree operator".into(), inputs, outputs, ..Default::default() }
}
// #endregion 🔖️Contract

// #region 🔖️OperatorRecord
/// ⚙️ Eval error from an operator.
#[derive(Clone, Debug, PartialEq)]
pub enum EvalError {
    UnknownKind(String),
    MissingInput(String),
    InvalidInput(String),
    CardinalityViolation(String),
    HeterogeneousList(String),
    CycleDetected,
    PendingExtension { extension_id: String, operator_id: String, node_hash: u64 },
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::UnknownKind(k) => write!(f, "unknown kind: {k}"),
            EvalError::MissingInput(k) => write!(f, "missing input: {k}"),
            EvalError::InvalidInput(m) => write!(f, "invalid input: {m}"),
            EvalError::CardinalityViolation(m) => write!(f, "cardinality violation: {m}"),
            EvalError::HeterogeneousList(m) => write!(f, "heterogeneous list: {m}"),
            EvalError::CycleDetected => write!(f, "cycle detected"),
            EvalError::PendingExtension { extension_id, operator_id, .. } => write!(f, "pending extension {extension_id} operator {operator_id}"),
        }
    }
}

impl std::error::Error for EvalError {}

/// 📈️ Monotone progress of one [`OperatorJob`]. `units_done` never decreases; `units_total` is the
/// plan known so far and may be revised upward by a job whose later stages are only plannable once
/// an earlier one has run (a boolean cannot count its result's validation units before it has a
/// result). `phase` is the job's own domain vocabulary — the framework never interprets it, it only
/// carries it to the surface that labels it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OperatorProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub phase: &'static str,
}

/// ⏱️ Outcome of one budgeted [`OperatorJob::step`].
#[derive(Clone, Debug, PartialEq)]
pub enum OperatorJobStep {
    /// 🔁 Budget spent, work remains — call `step` again.
    Working(OperatorProgress),
    /// ✅ Terminal: the operator's out dictionary.
    Done(Dictionary),
    /// 🛑 Terminal: [`OperatorJob::cancel`] retired the job; nothing is produced.
    Cancelled(OperatorProgress),
}

/// ⏱️ A single operator evaluation split into budgetable units so a host can drive it across many
/// turns inside an interactive step ceiling, paint progress, and cancel it — the operator twin of
/// the kernel's resumable tessellation job.
///
/// Domain-NEUTRAL by construction: the framework knows only units, a phase tag and the three
/// outcomes. Which units an operator has (a face pair, a validation entity, a solver iteration) is
/// entirely the domain extension's business, and an operator that has no sub-structure simply does
/// not offer a job (see [`Operator::step_plan`]'s default) and is evaluated in one call as before
/// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
pub trait OperatorJob: Send {
    /// ⏱️ Advances by at most `budget` units. A `budget` of zero is a legal progress probe.
    fn step(&mut self, budget: usize) -> Result<OperatorJobStep, EvalError>;
    /// 📈️ Progress right now — safe to read between steps and after termination.
    fn progress(&self) -> OperatorProgress;
    /// 🛑️ Retires the job at the next observable boundary. A job that already produced its output
    /// is never retired: supersession may only stop work still in flight.
    fn cancel(&mut self);
}

/// 🧮️ Computational unit: one dictionary to another.
pub trait Operator: Send + Sync {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError>;
    /// 🧊️ The cold-boundary form of [`Operator::evaluate`], mirroring
    /// [`Registry::dispatch_cold`]: takes EXACT ownership of `input` and answers inside a cold
    /// boundary, on the refusal path too. A batch caller (a test harness, an export bridge) that
    /// evaluates an operator directly therefore never becomes the final owner of either
    /// dictionary, and never has to open-code [`Dictionary`]'s drop law
    /// (`final Dictionary ownership must be explicitly retired or owned by a cold boundary`).
    // 🚫️async: E1 pure operator evaluation mirroring `evaluate` (no I/O) — see R9
    fn evaluate_cold(&self, input: Dictionary) -> Result<ColdOwner<Dictionary>, EvalError> {
        let input = ColdOwner::new(input);
        self.evaluate(&input).map(ColdOwner::new)
    }
    /// ⏱️ The budgeted, resumable form of this operator's evaluation, when it has one. `None` (the
    /// default, and the answer for every operator whose cost is microseconds) means "evaluate me in
    /// one call". An operator that answers `Some` MUST produce, through its job, exactly what
    /// [`Operator::evaluate`] would have produced for the same input — the stepped path IS the
    /// algorithm, never a second implementation to drift from.
    fn step_plan(&self, _input: &Dictionary) -> Result<Option<Box<dyn OperatorJob>>, EvalError> {
        Ok(None)
    }
    /// 🪶️ Only compiler-proven trivial operators are terminal without domain-specific field retirement.
    fn retirement_is_empty(&self) -> bool { !std::mem::needs_drop::<Self>() }
    /// 🧹️ Transfers or retires one granted domain frontier while the caller retains the operator itself.
    fn retire_step(&mut self, maximum_items: usize, maximum_bytes: usize, _values: &mut ValueRetirement) -> Result<ValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(ValueRetirementStep::Blocked); }
        if self.retirement_is_empty() { Ok(ValueRetirementStep::Complete) } else { Err("neural.operator-retirement-not-implemented") }
    }
    /// 🧊️ Explicit cold registry teardown; implementations owning neural domains retire those fields here.
    fn retire_cold(self: Box<Self>) { drop(self); }
}

// #region 🔖️Cardinality
/// 🔢️ Channel multiplicity: exactly one, optional, or homogeneous list collections.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Cardinality {
    #[default]
    ExactlyOne,
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Exactly(usize),
}

impl Cardinality {
    pub fn symbol(&self) -> String {
        match self {
            Self::ExactlyOne => "!".into(),
            Self::ZeroOrOne => "?".into(),
            Self::ZeroOrMore => "*".into(),
            Self::OneOrMore => "+".into(),
            Self::Exactly(count) => count.to_string(),
        }
    }

    pub fn from_symbol(raw: &str) -> Result<Self, NeuralEngineError> {
        match raw.trim() {
            "!" => Ok(Self::ExactlyOne),
            "?" => Ok(Self::ZeroOrOne),
            "*" => Ok(Self::ZeroOrMore),
            "+" => Ok(Self::OneOrMore),
            digits if digits.chars().all(|ch| ch.is_ascii_digit()) && !digits.is_empty() => digits.parse::<usize>().map(Self::Exactly).map_err(|_| NeuralEngineError::InvalidCardinality(raw.to_string())),
            other => Err(NeuralEngineError::InvalidCardinality(other.to_string())),
        }
    }

    pub fn is_collection(&self) -> bool {
        match self {
            Self::ZeroOrMore | Self::OneOrMore => true,
            Self::Exactly(count) => *count != 1,
            _ => false,
        }
    }

    pub fn accepts(&self, count: usize) -> bool {
        match self {
            Self::ExactlyOne => count == 1,
            Self::ZeroOrOne => count <= 1,
            Self::ZeroOrMore => true,
            Self::OneOrMore => count >= 1,
            Self::Exactly(expected) => count == *expected,
        }
    }

    pub fn count_range(&self) -> (usize, Option<usize>) {
        match self {
            Self::ExactlyOne => (1, Some(1)),
            Self::ZeroOrOne => (0, Some(1)),
            Self::ZeroOrMore => (0, None),
            Self::OneOrMore => (1, None),
            Self::Exactly(count) => (*count, Some(*count)),
        }
    }

    pub fn range_contains(&self, other: &Self) -> bool {
        let (min, max) = self.count_range();
        let (other_min, other_max) = other.count_range();
        if other_min < min {
            return false;
        }
        match (max, other_max) {
            (Some(limit), Some(other_limit)) => other_limit <= limit,
            (Some(_limit), None) => false,
            (None, Some(_other_limit)) => other_min >= min,
            (None, None) => true,
        }
    }
}

/// 🔮️ Test-only — `ChannelSpec.cardinality` was the only production caller and moved to `ToValue`/
/// `FromValue` below (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass).
#[cfg(test)]
impl Serialize for Cardinality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.symbol())
    }
}

#[cfg(test)]
impl<'de> Deserialize<'de> for Cardinality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::from_symbol(&raw).map_err(serde::de::Error::custom)
    }
}

impl ToValue for Cardinality {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.symbol())
    }
}

impl FromValue for Cardinality {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::String(raw) = value else { return Err(ValueError::new("expected a string for Cardinality")) };
        Self::from_symbol(&raw).map_err(|error| ValueError::new(error.to_string()))
    }
}
// #endregion 🔖️Cardinality

/// ➕️ Variadic input or output slot specification. `serde` is TEST-ONLY — see `Dictionary`'s docstring.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct VariadicSpec {
    pub slot_key: String,
    pub min: usize,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub max: Option<usize>,
}

impl ToValue for VariadicSpec {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("slotKey".into(), self.slot_key.to_value()), ("min".into(), self.min.to_value()), ("max".into(), self.max.to_value())])
    }
}

impl FromValue for VariadicSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for VariadicSpec")) };
        Ok(Self {
            slot_key: value.get("slotKey").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("slotKey"))?,
            min: value.get("min").cloned().map(usize::from_value).transpose()?.ok_or_else(|| ValueError::new("min"))?,
            max: value.get("max").cloned().map(Option::<usize>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🔌️ Declared operator channel with required/provided operator capabilities. `serde` is TEST-ONLY
/// — see `Dictionary`'s docstring; `default: Option<Value>` fans in `Value` transitively.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ChannelSpec {
    pub code: String,
    pub abbreviation: String,
    pub name: String,
    pub full_name: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub operators: Vec<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub value_types: Vec<String>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub default: Option<Value>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default))]
    pub cardinality: Cardinality,
}

impl ToValue for ChannelSpec {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("code".into(), self.code.to_value()),
            ("abbreviation".into(), self.abbreviation.to_value()),
            ("name".into(), self.name.to_value()),
            ("fullName".into(), self.full_name.to_value()),
            ("operators".into(), self.operators.to_value()),
            ("valueTypes".into(), self.value_types.to_value()),
            ("default".into(), self.default.to_value()),
            ("label".into(), self.label.to_value()),
            ("cardinality".into(), self.cardinality.to_value()),
        ])
    }
}

impl FromValue for ChannelSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for ChannelSpec")) };
        Ok(Self {
            code: value.get("code").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("code"))?,
            abbreviation: value.get("abbreviation").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("abbreviation"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("name"))?,
            full_name: value.get("fullName").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("fullName"))?,
            operators: value.get("operators").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            value_types: value.get("valueTypes").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            default: value.get("default").cloned().map(Option::<Value>::from_value).transpose()?.unwrap_or_default(),
            label: value.get("label").cloned().map(Option::<String>::from_value).transpose()?.unwrap_or_default(),
            cardinality: value.get("cardinality").cloned().map(Cardinality::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

fn derive_channel_names(name: &str) -> (String, String, String) {
    let code = if name.len() <= 2 { name.to_uppercase() } else { name.chars().take(2).collect::<String>().to_uppercase() };
    let abbreviation = if name.len() <= 3 { name.to_string() } else { name.chars().take(3).collect() };
    let mut full = String::new();
    let mut capitalize = true;
    for ch in name.chars() {
        if ch == '_' {
            capitalize = true;
            continue;
        }
        if capitalize {
            full.extend(ch.to_uppercase());
            capitalize = false;
        } else {
            full.push(ch);
        }
    }
    if full.is_empty() {
        full = name.to_string();
    }
    (code, abbreviation, full)
}

impl ChannelSpec {
    pub fn named(code: impl Into<String>, abbreviation: impl Into<String>, name: impl Into<String>, full_name: impl Into<String>) -> Self {
        Self { code: code.into(), abbreviation: abbreviation.into(), name: name.into(), full_name: full_name.into(), operators: Vec::new(), value_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn requires(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        let name = name.into();
        let (code, abbreviation, full_name) = derive_channel_names(&name);
        Self { code, abbreviation, name, full_name, operators: operators.iter().map(|entry| entry.as_ref().to_string()).collect(), value_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn provides(name: impl Into<String>, operators: Vec<String>) -> Self {
        let name = name.into();
        let (code, abbreviation, full_name) = derive_channel_names(&name);
        Self { code, abbreviation, name, full_name, operators, value_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn with_operators(mut self, operators: Vec<String>) -> Self {
        self.operators = operators;
        self
    }

    /// 🔤️ Declares the VALUE SCHEMAS this channel carries — the `Dictionary::schema()` a value on
    /// this wire actually has (`"geometry"`, `"vector"`, `"point"`, `"number"`, `"text"`, `"list"`),
    /// not the operator ids `operators` holds. An input lists every schema it accepts, an output
    /// every schema it may produce, and `Registry::channel_compatible` refuses a pair whose declared
    /// sets are disjoint. Empty means undeclared, which stays connectable.
    ///
    /// @see `🧫️fixtures/🔌️port-types/🔣️.json` — the catalogue-wide law
    pub fn with_value_types(mut self, value_types: &[impl AsRef<str>]) -> Self {
        self.value_types = value_types.iter().map(|entry| entry.as_ref().to_string()).collect();
        self
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default.replace(default).retire_cold();
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_cardinality(mut self, cardinality: Cardinality) -> Self {
        self.cardinality = cardinality;
        self
    }

    pub fn number(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_NUMBER])
    }

    pub fn number_default(name: impl Into<String>, default: f64, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_NUMBER]).with_default(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(default)))))
    }

    pub fn integer_default(name: impl Into<String>, default: i64, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_NUMBER]).with_default(Value::Atom(Atom::Integer(default)))
    }

    pub fn boolean_default(name: impl Into<String>, default: bool, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_BOOLEAN]).with_default(Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(default)))))
    }

    pub fn text_default(name: impl Into<String>, default: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_TEXT]).with_default(Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(default.into())))))
    }

    pub fn list(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_LIST]).with_cardinality(Cardinality::ZeroOrMore)
    }

    pub fn list_output(name: impl Into<String>, operators: Vec<String>) -> Self {
        Self::provides(name, operators).with_value_types(&[VALUE_TYPE_LIST]).with_cardinality(Cardinality::ZeroOrMore)
    }

    pub fn dictionary(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators)
    }

    pub fn any(name: impl Into<String>) -> Self {
        Self::requires(name, &[] as &[&str])
    }

    pub fn wildcard() -> Self {
        Self::any("*")
    }
}

/// 📤️ Wraps a payload dictionary under a named output channel.
pub fn channel_output(name: &str, payload: Dictionary) -> Dictionary {
    Dictionary::new().insert(name, Value::Dictionary(payload))
}

/// 📇️ Catalogue metadata for an operator. `serde` is TEST-ONLY — see `Dictionary`'s docstring;
/// `inputs`/`outputs: Vec<ChannelSpec>` fan in `Value` transitively.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct OperatorInfo {
    pub id: String,
    pub extension: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<ChannelSpec>,
    pub outputs: Vec<ChannelSpec>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub variadic_input: Option<VariadicSpec>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub variadic_output: Option<VariadicSpec>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub group: Vec<String>,
}

impl ToValue for OperatorInfo {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("extension".into(), self.extension.to_value()),
            ("name".into(), self.name.to_value()),
            ("abbreviation".into(), self.abbreviation.to_value()),
            ("icon".into(), self.icon.to_value()),
            ("summary".into(), self.summary.to_value()),
            ("inputs".into(), self.inputs.to_value()),
            ("outputs".into(), self.outputs.to_value()),
            ("variadicInput".into(), self.variadic_input.to_value()),
            ("variadicOutput".into(), self.variadic_output.to_value()),
            ("group".into(), self.group.to_value()),
        ])
    }
}

impl FromValue for OperatorInfo {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for OperatorInfo")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            extension: value.get("extension").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("extension"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("name"))?,
            abbreviation: value.get("abbreviation").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("abbreviation"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("icon"))?,
            summary: value.get("summary").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("summary"))?,
            inputs: value.get("inputs").cloned().map(Vec::<ChannelSpec>::from_value).transpose()?.unwrap_or_default(),
            outputs: value.get("outputs").cloned().map(Vec::<ChannelSpec>::from_value).transpose()?.unwrap_or_default(),
            variadic_input: value.get("variadicInput").cloned().map(Option::<VariadicSpec>::from_value).transpose()?.unwrap_or_default(),
            variadic_output: value.get("variadicOutput").cloned().map(Option::<VariadicSpec>::from_value).transpose()?.unwrap_or_default(),
            group: value.get("group").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

pub struct OperatorImpl {
    pub schemas: Vec<String>,
    pub operator: Box<dyn Operator>,
}

pub struct OperatorRecord {
    pub info: OperatorInfo,
    pub implementations: Vec<OperatorImpl>,
}

/// 📋️ Registry of schemas and operators by id.
#[derive(Default)]
pub struct Registry {
    schemas: BTreeMap<String, Schema>,
    operators: BTreeMap<String, OperatorRecord>,
    operator_produces: BTreeMap<String, Vec<String>>,
    schema_providers: BTreeMap<String, BTreeSet<String>>,
    finalized: bool,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_schema(&mut self, schema: Schema) {
        self.schemas.insert(schema.id.clone(), schema).retire_cold();
        self.finalized = false;
    }

    pub fn register_operator(&mut self, info: OperatorInfo, implementations: Vec<OperatorImpl>, produces: &[&str]) {
        let id = info.id.clone();
        for schema in produces {
            self.schema_providers.entry(schema.to_string()).or_default();
        }
        for implementation in &implementations {
            for schema in &implementation.schemas {
                self.schema_providers.entry(schema.clone()).or_default().insert(id.clone());
            }
        }
        self.operator_produces.insert(id.clone(), produces.iter().map(|entry| (*entry).to_string()).collect());
        self.operators.insert(id, OperatorRecord { info, implementations }).retire_cold();
        self.finalized = false;
    }

    pub fn finalize(&mut self) {
        if self.finalized {
            return;
        }
        let schema_ids: Vec<String> = self.schemas.keys().cloned().collect();
        for schema_id in schema_ids {
            let Some(schema) = self.schemas.get(&schema_id).cloned().map(ColdOwner::new) else { continue };
            if !should_auto_register_schema_component(&schema) {
                continue;
            }
            let operator_id = schema_component_operator_id(&schema);
            if self.operators.contains_key(&operator_id) {
                continue;
            }
            let info = schema_component_info(&schema);
            let produces = vec![schema.id.clone()];
            for produced in &produces {
                self.schema_providers.entry(produced.clone()).or_default();
            }
            self.schema_providers.entry(schema.id.clone()).or_default().insert(operator_id.clone());
            self.operator_produces.insert(operator_id.clone(), produces);
            self.operators.insert(operator_id, OperatorRecord { info, implementations: vec![OperatorImpl { schemas: vec![], operator: Box::new(SchemaComponent { schema: schema.into_inner() }) }] }).retire_cold();
        }
        let operator_produces = self.operator_produces.clone();
        let schema_providers = self.schema_providers.clone();
        for (operator_id, operator) in &mut self.operators {
            let produces = operator_produces.get(operator_id).cloned().unwrap_or_default();
            for channel in &mut operator.info.outputs {
                if !channel.operators.is_empty() {
                    continue;
                }
                let mut provided = HashSet::new();
                for schema in &produces {
                    if let Some(providers) = schema_providers.get(schema) {
                        provided.extend(providers.iter().cloned());
                    }
                }
                let mut operators: Vec<String> = provided.into_iter().collect();
                operators.sort();
                channel.operators = operators;
            }
        }
        self.finalized = true;
    }

    pub fn operators_for_schema(&self, schema_id: &str) -> Vec<String> {
        let mut operators: Vec<String> = self.schema_providers.get(schema_id).map(|entries| entries.iter().cloned().collect()).unwrap_or_default();
        operators.sort();
        operators
    }

    /// 🔌️ Whether a wire may carry `output` into `input` — the ONE port-compatibility oracle every
    /// node-graph surface answers to. Declared VALUE SCHEMAS decide it: a pair is refused only when
    /// both sides declare and the declared sets are disjoint, so an undeclared channel stays
    /// connectable and no shipped graph is retro-refused.
    ///
    /// @see `🧫️fixtures/🔌️port-types/🔣️.json` — the fixture that owns the compatible/incompatible pairs
    pub fn channel_compatible(output: &ChannelSpec, input: &ChannelSpec) -> bool {
        if output.value_types.is_empty() || input.value_types.is_empty() {
            return true;
        }
        output.value_types.iter().any(|provided| input.value_types.iter().any(|accepted| accepted == provided))
    }

    pub fn schema(&self, schema_id: &str) -> Option<&Schema> {
        self.schemas.get(schema_id)
    }

    pub fn operator(&self, operator_id: &str) -> Option<&OperatorRecord> {
        self.operators.get(operator_id)
    }

    pub fn operator_info(&self, operator_id: &str) -> Option<&OperatorInfo> {
        self.operators.get(operator_id).map(|entry| &entry.info)
    }

    /// 📚️ Borrows current metadata in registry order without acquiring nested default-value owners.
    pub fn operator_infos(&self) -> impl DoubleEndedIterator<Item = &OperatorInfo> + ExactSizeIterator {
        self.operators.values().map(|entry| &entry.info)
    }

    pub fn schema_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.schemas.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// 🏷️ Lightweight schema metadata for pickers and catalogues.
    pub fn schema_refs(&self) -> Vec<SchemaRef> {
        self.schema_ids().into_iter().filter_map(|id| self.schemas.get(&id).map(|schema| SchemaRef { id: schema.id.clone(), name: schema.name.clone(), icon: schema.icon.clone() })).collect()
    }

    pub fn schema_catalogue(&self) -> Vec<Schema> {
        let mut items: Vec<Schema> = self.schemas.values().cloned().collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }

    pub fn operator_catalogue(&self) -> Vec<OperatorInfo> {
        let mut items: Vec<OperatorInfo> = self.operators.values().map(|entry| Self::finalize_operator_info(&entry.info, self.operator_produces.get(&entry.info.id).map(Vec::as_slice), &self.schema_providers)).collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }

    fn finalize_operator_info(info: &OperatorInfo, produces: Option<&[String]>, schema_providers: &BTreeMap<String, BTreeSet<String>>) -> OperatorInfo {
        let mut finalized = info.clone();
        let produces = produces.unwrap_or(&[]);
        for channel in &mut finalized.outputs {
            if !channel.operators.is_empty() {
                continue;
            }
            let mut provided = HashSet::new();
            for schema in produces {
                if let Some(providers) = schema_providers.get(schema) {
                    provided.extend(providers.iter().cloned());
                }
            }
            let mut operators: Vec<String> = provided.into_iter().collect();
            operators.sort();
            channel.operators = operators;
        }
        finalized
    }

    pub fn dispatch(&self, operator_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
        validate_neuron_inputs(input, Some(&operator.info))?;
        let signature = operator_signature(&operator.info, input);
        let implementation = operator
            .implementations
            .iter()
            .find(|implementation| implementation.schemas == signature)
            .or_else(|| operator.implementations.iter().find(|implementation| implementation.schemas.is_empty()))
            .ok_or_else(|| EvalError::InvalidInput(format!("no implementation for {operator_id}({})", signature.join(", "))))?;
        let output = ColdOwner::new(implementation.operator.evaluate(input)?);
        validate_operator_outputs(&operator.info, &output)?;
        Ok(output.into_inner())
    }

    /// 🧊️ The cold-boundary form of [`Registry::dispatch`]: takes EXACT ownership of `input` and
    /// answers the operator's dictionary already inside its cold boundary, so a batch caller never
    /// becomes the final owner of either dictionary and never has to remember
    /// [`Dictionary`]'s drop law (`final Dictionary ownership must be explicitly retired or owned
    /// by a cold boundary`) by hand. The refusal path retires `input` too, which a caller writing
    /// `registry.dispatch(id, &input)?` cannot do without a second owner.
    ///
    /// This is the same shape the guest path already takes
    /// (`flow_extension_sdk::evaluate_json` wraps both its input and its answer in a
    /// [`ColdOwner`]); it is named here so every batch caller — test harness, export bridge,
    /// catalogue probe — states one cold scope instead of open-coding two.
    // 🚫️async: E1 pure registry dispatch mirroring `dispatch` (no I/O) — see R9
    pub fn dispatch_cold(&self, operator_id: &str, input: Dictionary) -> Result<ColdOwner<Dictionary>, EvalError> {
        let input = ColdOwner::new(input);
        self.dispatch(operator_id, &input).map(ColdOwner::new)
    }

    /// ⏱️ The budgeted form of [`Registry::dispatch`]: resolves the same operator and
    /// implementation, validates the same inputs, and asks the implementation for a resumable job.
    /// `Ok(None)` means this operator has no sub-structure and the caller should `dispatch` it in
    /// one call. The job's `Done` dictionary still has to pass [`validate_operator_outputs`], which
    /// is why [`Registry::finish_job`] — not the caller — closes it.
    // 🚫️async: E1 pure registry lookup mirroring `dispatch` (no I/O) — see R9
    pub fn dispatch_job(&self, operator_id: &str, input: &Dictionary) -> Result<Option<Box<dyn OperatorJob>>, EvalError> {
        let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
        validate_neuron_inputs(input, Some(&operator.info))?;
        let signature = operator_signature(&operator.info, input);
        let implementation = operator
            .implementations
            .iter()
            .find(|implementation| implementation.schemas == signature)
            .or_else(|| operator.implementations.iter().find(|implementation| implementation.schemas.is_empty()))
            .ok_or_else(|| EvalError::InvalidInput(format!("no implementation for {operator_id}({})", signature.join(", "))))?;
        implementation.operator.step_plan(input)
    }

    /// ✅️ Holds a finished job's output to the SAME output contract [`Registry::dispatch`] holds a
    /// one-shot evaluation to, so a stepped answer and a one-shot answer are indistinguishable
    /// downstream.
    // 🚫️async: E1 pure registry lookup (no I/O) — see R9
    pub fn finish_job(&self, operator_id: &str, output: Dictionary) -> Result<Dictionary, EvalError> {
        let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
        let output = ColdOwner::new(output);
        validate_operator_outputs(&operator.info, &output)?;
        Ok(output.into_inner())
    }
}
// #endregion 🔖️OperatorRecord

// #region 🔖️Cache
fn hash_str<H: Hasher>(hasher: &mut H, value: &str) {
    value.hash(hasher);
}

fn hash_atom<H: Hasher>(hasher: &mut H, atom: &Atom) {
    match atom {
        Atom::Null => 0u8.hash(hasher),
        Atom::Boolean(value) => value.hash(hasher),
        Atom::Integer(value) => value.hash(hasher),
        Atom::Decimal(value) => value.to_bits().hash(hasher),
        Atom::String(value) => hash_str(hasher, value),
    }
}

fn hash_value<H: Hasher>(hasher: &mut H, value: &Value) {
    match value {
        Value::Atom(atom) => hash_atom(hasher, atom),
        Value::Dictionary(dict) => {
            0u8.hash(hasher);
            hash_dictionary(hasher, dict);
        }
    }
}

fn hash_dictionary<H: Hasher>(hasher: &mut H, dictionary: &Dictionary) {
    for (key, value) in &dictionary.pairs {
        hash_str(hasher, key);
        hash_value(hasher, value);
    }
}

/// 🔑️ Content-addressable cache key from operator kind and resolved input dictionary.
pub fn node_hash(kind: &str, input: &Dictionary) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hash_str(&mut hasher, kind);
    hash_dictionary(&mut hasher, input);
    hasher.finish()
}

/// 🧠️ Epoch-bounded in-process cache for DAG node outputs.
#[derive(Default)]
pub struct NeuralCache {
    entries: ManuallyDrop<Mutex<BTreeMap<u64, (u64, Dictionary)>>>,
    retirement: ManuallyDrop<Mutex<ValueRetirement>>,
    epoch: AtomicU64,
}

struct NeuralCacheRetirementState {
    cache: Option<std::sync::Arc<NeuralCache>>,
    entries: BTreeMap<u64, (u64, Dictionary)>,
    retirement: ValueRetirement,
    terminal: bool,
}

/// 🧹️ Exact cache-root handoff and byte-aware nested retirement; u64 tree metadata has fixed machine-width height.
pub struct NeuralCacheRetirement { state: ManuallyDrop<NeuralCacheRetirementState> }

impl NeuralCacheRetirement {
    pub fn new(cache: std::sync::Arc<NeuralCache>) -> Self {
        Self { state: ManuallyDrop::new(NeuralCacheRetirementState { cache: Some(cache), entries: BTreeMap::new(), retirement: ValueRetirement::default(), terminal: false }) }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> ValueRetirementStep {
        if maximum_items == 0 || maximum_bytes == 0 { return ValueRetirementStep::Blocked; }
        let state = &mut *self.state;
        if !state.retirement.terminal_is_empty() { return state.retirement.close_step(maximum_items, maximum_bytes); }
        if let Some(cache) = state.cache.take() {
            if let Some(mut cache) = std::sync::Arc::into_inner(cache) {
                state.entries = std::mem::take(cache.entries.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner));
                state.retirement = std::mem::take(cache.retirement.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner));
            }
            return ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some((_, (_, value))) = state.entries.pop_first() {
            state.retirement.push_dictionary(value);
            return ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 };
        }
        state.terminal = true;
        ValueRetirementStep::Complete
    }

    pub fn terminal_nonopaque_is_empty(&self) -> bool {
        self.state.terminal && self.state.cache.is_none() && self.state.entries.is_empty() && self.state.retirement.terminal_is_empty()
    }
}

impl Drop for NeuralCacheRetirement {
    fn drop(&mut self) {
        if !self.terminal_nonopaque_is_empty() { assert!(std::thread::panicking(), "NeuralCacheRetirement must reach terminal-empty before release"); return; }
        unsafe { ManuallyDrop::drop(&mut self.state); }
    }
}

impl Drop for NeuralCache {
    fn drop(&mut self) {
        let empty = self.entries.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty()
            && self.retirement.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).terminal_is_empty();
        if !empty { assert!(std::thread::panicking(), "final NeuralCache must be explicitly retired"); return; }
        unsafe { ManuallyDrop::drop(&mut self.entries); ManuallyDrop::drop(&mut self.retirement); }
    }
}

impl NeuralCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_epoch(&self) {
        self.epoch.fetch_add(1, Ordering::Relaxed);
    }

    pub fn current_epoch(&self) -> u64 {
        self.epoch.load(Ordering::Relaxed)
    }

    pub fn len(&self) -> usize {
        self.entries.lock().map_or(0, |entries| entries.len())
    }

    pub fn is_empty(&self) -> bool {
        self.entries.lock().map_or(true, |entries| entries.is_empty())
    }

    /// 🔎️ Whether `key` has a cached entry (from any epoch) — a hit here means
    /// [`NeuralCache::get_or_insert_with`] would return without calling `compute`.
    pub fn contains(&self, key: u64) -> bool {
        let epoch = self.epoch.load(Ordering::Relaxed);
        if let Ok(mut entries) = self.entries.lock() {
            if let Some(entry) = entries.get_mut(&key) {
                entry.0 = epoch;
                return true;
            }
        }
        false
    }

    /// 🌱️ Pre-seeds a node output (host-mediated extension eval) so the next budgeted pass hits the cache.
    pub fn seed(&self, key: u64, value: Dictionary) {
        let epoch = self.epoch.load(Ordering::Relaxed);
        let displaced = self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(key, (epoch, value));
        if let Some((_, value)) = displaced { self.retirement.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push_dictionary(value); }
    }

    pub fn get(&self, key: u64) -> Option<Dictionary> {
        let epoch = self.epoch.load(Ordering::Relaxed);
        if let Ok(mut entries) = self.entries.lock() {
            if let Some(entry) = entries.get_mut(&key) {
                entry.0 = epoch;
                return Some(entry.1.clone());
            }
        }
        None
    }

    pub fn get_or_insert_with<F>(&self, key: u64, compute: F) -> Result<Dictionary, EvalError>
    where
        F: FnOnce() -> Result<Dictionary, EvalError>,
    {
        let epoch = self.epoch.load(Ordering::Relaxed);
        if let Ok(mut entries) = self.entries.lock() {
            if let Some(entry) = entries.get_mut(&key) {
                entry.0 = epoch;
                return Ok(entry.1.clone());
            }
        }
        let value = compute()?;
        self.seed(key, value.clone());
        Ok(value)
    }

    pub fn sweep(&self) {
        let epoch = self.epoch.load(Ordering::Relaxed);
        let mut entries = self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let expired: Vec<_> = entries.iter().filter(|(_, (entry_epoch, _))| *entry_epoch != epoch).map(|(key, _)| *key).collect();
        let mut retirement = self.retirement.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for key in expired { if let Some((_, value)) = entries.remove(&key) { retirement.push_dictionary(value); } }
    }
}

fn eval_error_dictionary(err: &EvalError) -> Dictionary {
    Dictionary::new().insert("error", Value::Atom(Atom::String(err.to_string())))
}

fn evaluate_cached_output<F>(cache: &NeuralCache, kind: &str, merged: &Dictionary, dispatch: F) -> Dictionary
where
    F: FnOnce() -> Result<Dictionary, EvalError>,
{
    let key = node_hash(kind, merged);
    match cache.get_or_insert_with(key, dispatch) {
        Ok(dict) => dict,
        Err(err) => eval_error_dictionary(&err),
    }
}
// #endregion 🔖️Cache

// #region 🔖️DirtyPropagation
fn hash_neuron_key<H: Hasher>(hasher: &mut H, neuron: &Neuron) {
    hash_str(hasher, &neuron.kind);
    hash_dictionary(hasher, &neuron.params);
    if let Some(sub_tree) = neuron.tree.as_deref() {
        1u8.hash(hasher);
        hash_subtree(hasher, sub_tree);
    } else {
        0u8.hash(hasher);
    }
}

fn hash_subtree<H: Hasher>(hasher: &mut H, tree: &Tree) {
    let mut neurons: Vec<&Neuron> = tree.neurons.iter().collect();
    neurons.sort_by(|a, b| a.id.cmp(&b.id));
    for neuron in neurons {
        hash_str(hasher, &neuron.id);
        hash_neuron_key(hasher, neuron);
    }
    let mut synapses: Vec<&Synapse> = tree.synapses.iter().collect();
    synapses.sort_by(|a, b| (&a.from, &a.from_port, &a.to, &a.to_port).cmp(&(&b.from, &b.from_port, &b.to, &b.to_port)));
    for syn in synapses {
        hash_str(hasher, &syn.from);
        hash_str(hasher, &syn.from_port);
        hash_str(hasher, &syn.to);
        hash_str(hasher, &syn.to_port);
    }
}

fn neuron_key_hash(neuron: &Neuron) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hash_neuron_key(&mut hasher, neuron);
    hasher.finish()
}

fn incoming_edges_signature(tree: &Tree, neuron_id: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut incoming: Vec<&Synapse> = tree.synapses.iter().filter(|syn| syn.to == neuron_id).collect();
    incoming.sort_by(|a, b| (&a.from, &a.from_port, &a.to_port).cmp(&(&b.from, &b.from_port, &b.to_port)));
    for syn in incoming {
        hash_str(&mut hasher, &syn.from);
        hash_str(&mut hasher, &syn.from_port);
        hash_str(&mut hasher, &syn.to_port);
    }
    hasher.finish()
}

/// 🧬️ Per-neuron structural/adjacency fingerprint, keyed once by id in [`TreeSnapshot`] instead
/// of duplicated across parallel maps — cuts id clones on [`TreeSnapshot::capture`] from four
/// per neuron down to one.
#[derive(Clone, Debug, Default, PartialEq)]
struct NeuronSnapshot {
    key: u64,
    incoming: u64,
    /// `[to, ...]` — who reads this neuron's output, used for forward dirty propagation.
    dependents: Vec<String>,
}

/// 📸️ Structural fingerprint of a tree+seeds pair, used by [`compute_dirty_set`] to diff two
/// evaluations without re-hashing or re-walking neurons that provably didn't change.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TreeSnapshot {
    neurons: BTreeMap<String, NeuronSnapshot>,
    seed_keys: BTreeMap<String, u64>,
}

impl TreeSnapshot {
    pub fn capture(tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Self {
        let mut neurons: BTreeMap<String, NeuronSnapshot> = tree.neurons.iter().map(|neuron| (neuron.id.clone(), NeuronSnapshot { key: neuron_key_hash(neuron), incoming: incoming_edges_signature(tree, &neuron.id), dependents: Vec::new() })).collect();
        for syn in &tree.synapses {
            if !neurons.contains_key(&syn.to) {
                continue;
            }
            if let Some(source) = neurons.get_mut(&syn.from) {
                source.dependents.push(syn.to.clone());
            }
        }
        let mut seed_keys = BTreeMap::new();
        for (id, dict) in seeds {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            hash_dictionary(&mut hasher, dict);
            seed_keys.insert(id.clone(), hasher.finish());
        }
        Self { neurons, seed_keys }
    }
}

/// 🧭️ Forward-propagates dirtiness from directly-changed neurons to all descendants.
///
/// `previous == None` means "first evaluation ever" — everything is dirty. Otherwise a neuron
/// is directly dirty if it's new, its structural key (kind/params/subtree) changed, its incoming
/// synapse set changed, or its seed value changed; a surviving dependent of a *removed* neuron
/// (looked up via `previous`'s adjacency, since removed neurons vanish from `current`) is also
/// directly dirty. Every neuron reachable from the directly-dirty set via `current`'s `from -> to`
/// adjacency is dirty too — everything else is provably unaffected.
pub fn compute_dirty_set(previous: Option<&TreeSnapshot>, current: &TreeSnapshot) -> HashSet<String> {
    let Some(previous) = previous else {
        return current.neurons.keys().cloned().collect();
    };
    let mut direct: HashSet<String> = HashSet::new();
    for (id, snapshot) in &current.neurons {
        let prev = previous.neurons.get(id);
        let is_new = prev.is_none();
        let structurally_changed = prev.map(|p| p.key) != Some(snapshot.key);
        let rewired = prev.map(|p| p.incoming) != Some(snapshot.incoming);
        if is_new || structurally_changed || rewired {
            direct.insert(id.clone());
        }
    }
    let mut seed_ids: HashSet<&String> = previous.seed_keys.keys().collect();
    seed_ids.extend(current.seed_keys.keys());
    for id in seed_ids {
        if current.neurons.contains_key(id) && previous.seed_keys.get(id) != current.seed_keys.get(id) {
            direct.insert(id.clone());
        }
    }
    for (removed_id, removed_snapshot) in &previous.neurons {
        if current.neurons.contains_key(removed_id) {
            continue;
        }
        for dependent in &removed_snapshot.dependents {
            if current.neurons.contains_key(dependent) {
                direct.insert(dependent.clone());
            }
        }
    }
    let mut dirty: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = direct.into_iter().collect();
    while let Some(id) = queue.pop_front() {
        if !dirty.insert(id.clone()) {
            continue;
        }
        if let Some(snapshot) = current.neurons.get(&id) {
            for dep in &snapshot.dependents {
                if !dirty.contains(dep) {
                    queue.push_back(dep.clone());
                }
            }
        }
    }
    dirty
}
// #endregion 🔖️DirtyPropagation

// #region 🔖️Evaluator
/// ⏳️ Topo-ordered neuron ids still needing work when a budgeted walk stops at `from_index` having
/// `parked` the extension requests of one wave.
///
/// ⛓️ A parked neuron sits BEFORE `from_index` — the walk went past it to gather the rest of its
/// wave — so it is named explicitly and unconditionally: a neuron whose answer has not landed is
/// still owed whatever the dirty set says about it, and dropping it here would make the chain call
/// itself converged while an answer is still crossing.
fn budgeted_remaining(order: &[String], from_index: usize, parked: &HashSet<String>, dirty: &HashSet<String>) -> Vec<String> {
    order
        .iter()
        .enumerate()
        .filter(|(index, id)| parked.contains(id.as_str()) || (*index >= from_index && (dirty.is_empty() || dirty.contains(id.as_str()))))
        .map(|(_, id)| id.clone())
        .collect()
}

/// ⛓️ Whether `neuron_id` draws an input from a neuron THIS walk could not produce — one whose
/// extension request is parked, or one already blocked behind such a request.
///
/// 🚨️ [`collect_neuron_input`] SILENTLY SKIPS a source with no output (`else { continue }`), so a
/// neuron downstream of a parked request would otherwise be dispatched against a half-built input
/// and cache a wrong answer under a hash that claims to describe the real one. Waiting is not an
/// optimisation here; it is the only correct answer.
fn waits_on_parked(tree: &Tree, parked: &HashSet<String>, neuron_id: &str) -> bool {
    !parked.is_empty() && tree.synapses.iter().any(|synapse| synapse.to == neuron_id && parked.contains(&synapse.from))
}

/// 📡️ Resolved neuron inputs and outputs from one evaluation pass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvalChannels {
    pub outputs: BTreeMap<String, Dictionary>,
    pub inputs: BTreeMap<String, Dictionary>,
}

/// ⏳️ Result of a budget-limited evaluation pass — `remaining` (in topo order) is empty once the
/// whole dirty set has been walked; a non-empty `remaining` means resume with another budgeted call.
///
/// 🌊️ `pending_extensions` is ONE TOPOLOGICAL WAVE: every neuron whose inputs were ready in this
/// walk and whose operator lives in a plugin, not in this process. They are gathered together
/// because they are independent by construction — no member of a wave consumes another member's
/// output — so they can all be in flight at once and the chain costs ONE round trip per
/// dependency LEVEL instead of one per node (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetedEval {
    pub channels: EvalChannels,
    pub remaining: Vec<String>,
    pub pending_extensions: Vec<PendingExtensionEval>,
}

/// ⏱️ A wall-clock guard a budgeted walk consults BETWEEN neurons. Carried as a plain `fn` pointer
/// rather than a clock trait or an `Instant`: this crate is a leaf evaluator with no dependency on
/// the tracing module that owns the process clock, and `std::time::Instant` is not usable on every
/// wasm target the guest builds for.
#[derive(Clone, Copy, Debug)]
pub struct EvalStepDeadline {
    pub now_us: fn() -> Option<u64>,
    pub deadline_us: u64,
}

impl PartialEq for EvalStepDeadline {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.now_us, other.now_us) && self.deadline_us == other.deadline_us
    }
}

impl EvalStepDeadline {
    /// ⌛️ Whether the walk has run past its wall-clock allowance. A clock that answers nothing (bare
    /// wasm with no host clock installed) never expires the walk — the node count stays the only cap.
    fn expired(&self) -> bool {
        (self.now_us)().is_some_and(|now_us| now_us >= self.deadline_us)
    }
}

/// ⏳️ What ONE budgeted walk may spend. `dispatches` is the historical cache-missed-neuron count;
/// `deadline` is the preemption point a NODE count alone cannot provide — one expensive operator
/// (a `brep.bool.fuse` on a complex solid) runs to completion inside a single non-preemptible
/// reactor step regardless of how few nodes it is, which is what parked the browser's main thread
/// for 3.5-18.4 s per hop (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️audit-guest-tick-cost-2026-09-12.md` §1.3). The deadline is consulted only AFTER at least one
/// dispatch, so a walk that starts already over budget still makes progress instead of re-arming
/// forever.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvalStepBudget {
    pub dispatches: usize,
    pub deadline: Option<EvalStepDeadline>,
}

impl EvalStepBudget {
    /// 🔍️ A pure probe: dispatches nothing, reports every neuron that would still need work.
    pub const PROBE: EvalStepBudget = EvalStepBudget { dispatches: 0, deadline: None };
    /// ♾️ Walks the whole dirty set in one call, however long it takes.
    pub const UNBOUNDED: EvalStepBudget = EvalStepBudget { dispatches: usize::MAX, deadline: None };

    /// 🔢️ A node-count-only budget — no wall-clock preemption.
    pub const fn dispatches(dispatches: usize) -> EvalStepBudget {
        EvalStepBudget { dispatches, deadline: None }
    }

    /// ⏱️ The same node count, preempted at `deadline_us` on `now_us`'s clock.
    pub const fn until(dispatches: usize, now_us: fn() -> Option<u64>, deadline_us: u64) -> EvalStepBudget {
        EvalStepBudget { dispatches, deadline: Some(EvalStepDeadline { now_us, deadline_us }) }
    }

    /// ⏱️ The wall instant this budget preempts at, if any.
    ///
    /// 🔁️ Readable because a guest TURN may now run more than one dag walk — a fold that continues
    /// its chain inline runs the next wave inside the answer's own turn — and the rule that keeps
    /// the 8 ms interactive hold is that every walk of one turn shares ONE deadline instead of
    /// opening a fresh allowance apiece. A law can only state that if the deadline is observable
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub const fn deadline_us(&self) -> Option<u64> {
        match self.deadline {
            Some(deadline) => Some(deadline.deadline_us),
            None => None,
        }
    }

    /// 🛑️ Whether the walk must yield before dispatching another cache-missed neuron.
    fn exhausted(&self, spent: usize) -> bool {
        spent >= self.dispatches || (spent != 0 && self.deadline.is_some_and(|deadline| deadline.expired()))
    }
}

/// ⏳️ One contributed operator that must be evaluated in its owning plugin before the graph can resume.
///
/// 🪪️ `neuron_id` names the node the request BELONGS to, so a census can say which nodes of a wave
/// are actually outstanding at their plugin instead of guessing from the head of a remaining list.
#[derive(Clone, Debug, PartialEq)]
pub struct PendingExtensionEval {
    pub neuron_id: String,
    pub extension_id: String,
    pub operator_id: String,
    pub node_hash: u64,
    pub input_json: String,
}

/// 🔄️ Topological evaluation over a neural tree.
pub struct Evaluator<'a> {
    registry: &'a Registry,
}

impl<'a> Evaluator<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    pub fn evaluate(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Result<BTreeMap<String, Dictionary>, EvalError> {
        let EvalChannels { outputs, inputs } = self.evaluate_channels(tree, seeds, &HashMap::new())?;
        inputs.retire_cold(); Ok(outputs)
    }

    pub fn evaluate_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
    ) -> Result<BTreeMap<String, Dictionary>, EvalError> {
        let EvalChannels { outputs, inputs } = self.evaluate_channels_with(tree, seeds, operator_infos, dispatch)?;
        inputs.retire_cold(); Ok(outputs)
    }

    pub fn evaluate_channels(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>, operator_infos: &HashMap<String, OperatorInfo>) -> Result<EvalChannels, EvalError> {
        self.evaluate_channels_with(tree, seeds, operator_infos, &|kind, input| self.registry.dispatch(kind, input))
    }

    pub fn evaluate_channels_sequential_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<EvalChannels, EvalError> {
        let cache = ColdOwner::new(NeuralCache::new());
        cache.begin_epoch();
        let result = self.evaluate_channels_sequential_cached(tree, seeds, operator_infos, dispatch, &cache, &HashSet::new(), None);
        cache.sweep();
        result
    }

    #[allow(clippy::too_many_arguments, reason = "incremental cache eval needs tree+seeds+infos+dispatch+cache+dirty+previous together; splitting into a params struct would ripple into flow/core/rs call sites outside this ticket's scope")]
    pub fn evaluate_channels_sequential_cached(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
    ) -> Result<EvalChannels, EvalError> {
        self.evaluate_channels_budgeted(tree, seeds, operator_infos, dispatch, cache, dirty, previous, EvalStepBudget::UNBOUNDED).map(|budgeted| budgeted.channels)
    }

    /// ⏳️ Sequential topo walk that stops after computing `budget.dispatches` cache-missed (i.e.
    /// actually dispatched) neurons OR once `budget.deadline` has passed, returning the
    /// not-yet-computed neuron ids as `remaining` so a caller can resume with another budgeted call
    /// — used to spread a heavy evaluation across many cheap ticks instead of blocking a thread for
    /// the whole graph. [`EvalStepBudget::PROBE`] is a pure probe: nothing is dispatched,
    /// `remaining` reports every neuron that would still need work.
    ///
    /// 🧯️ A dispatch that FAILS is the node's answer for that input exactly like a success, so it is
    /// seeded into `cache` too — the same rule a contributed extension's error answer already follows
    /// (`seed_node_cache`). Left uncached, a failing node costing more than one tick's deadline was
    /// re-dispatched first on every resumed tick, the walk never reached the nodes behind it, and the
    /// run ticked forever: `sphere-cut-with-torus` at the tangent radius 2.5 re-ran its refused
    /// `brep.bool.cut` for 900 s while `brep.measure.volume` stayed `computing`.
    #[allow(clippy::too_many_arguments, reason = "mirrors evaluate_channels_sequential_cached's params plus a budget; see that method's reason")]
    pub fn evaluate_channels_budgeted(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
        budget: EvalStepBudget,
    ) -> Result<BudgetedEval, EvalError> {
        let order = topo_order(tree)?;
        let mut outputs = ColdOwner::new(seeds.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<String, Dictionary>>());
        let mut inputs = ColdOwner::new(BTreeMap::<String, Dictionary>::new());
        let mut spent = 0usize;
        let mut parked: HashSet<String> = HashSet::new();
        let mut pending_extensions: Vec<PendingExtensionEval> = Vec::new();
        let mut stopped_at = order.len();
        for (index, neuron_id) in order.iter().enumerate() {
            if !dirty.contains(neuron_id) {
                if let Some(prev) = previous {
                    if let (Some(out), Some(inp)) = (prev.outputs.get(neuron_id), prev.inputs.get(neuron_id)) {
                        outputs.insert(neuron_id.clone(), out.clone()).retire_cold();
                        inputs.insert(neuron_id.clone(), inp.clone()).retire_cold();
                        continue;
                    }
                }
            }
            if waits_on_parked(tree, &parked, neuron_id) {
                parked.insert(neuron_id.clone());
                continue;
            }
            let neuron = tree.neurons.iter().find(|n| n.id == *neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
            let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
            let input = ColdOwner::new(collect_neuron_input(tree, &outputs, neuron_id, operator_info)?);
            inputs.insert(neuron_id.clone(), input.clone()).retire_cold();
            if let Some(seed) = seeds.get(neuron_id) {
                outputs.insert(neuron_id.clone(), seed.clone()).retire_cold();
                continue;
            }
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                outputs.insert(neuron_id.clone(), input.merge(&neuron.params)).retire_cold();
                continue;
            }
            // 🚧️ A budget-exhausted cache miss (cluster or operator) stops the walk here; this
            // neuron and everything from `order[index..]` becomes `remaining`. Exhaustion is either
            // the dispatch count or the wall-clock deadline — see [`EvalStepBudget`]. Clusters have
            // no single cache key of their own (their inner neurons are cached individually), so a
            // cluster is conservatively always charged as a miss.
            if let Some(sub_tree) = neuron.tree.as_deref() {
                if budget.exhausted(spent) {
                    stopped_at = index;
                    break;
                }
                let out = self.evaluate_cluster_sequential(sub_tree, &input, operator_infos, dispatch, cache)?;
                outputs.insert(neuron_id.clone(), out).retire_cold();
                spent += 1;
                continue;
            }
            let merged = ColdOwner::new(input.merge(&neuron.params));
            let key = node_hash(&neuron.kind, &merged);
            let is_miss = !cache.contains(key);
            if is_miss && budget.exhausted(spent) {
                stopped_at = index;
                break;
            }
            let out = if let Some(cached) = cache.get(key) {
                cached
            } else {
                match dispatch(&neuron.kind, &merged) {
                    Err(EvalError::PendingExtension { extension_id, operator_id, node_hash }) => {
                        // 🌊️ The wave gathers rather than returns: this neuron's answer has to come
                        // from its plugin, but every LATER neuron that does not depend on it can
                        // still be walked in this same call, so its own request rides the same round
                        // trip instead of costing a whole further hop.
                        pending_extensions.push(PendingExtensionEval { neuron_id: neuron_id.clone(), extension_id, operator_id, node_hash, input_json: pack::json::to_json_string(&*merged) });
                        parked.insert(neuron_id.clone());
                        spent += 1;
                        continue;
                    }
                    Err(err) => {
                        let fault = eval_error_dictionary(&err);
                        cache.seed(key, fault.clone());
                        fault
                    }
                    Ok(dict) => {
                        cache.seed(key, dict.clone());
                        dict
                    }
                }
            };
            outputs.insert(neuron_id.clone(), out).retire_cold();
            if is_miss {
                spent += 1;
            }
        }
        let remaining = budgeted_remaining(&order, stopped_at, &parked, dirty);
        Ok(BudgetedEval { channels: EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() }, remaining, pending_extensions })
    }

    pub fn evaluate_channels_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
    ) -> Result<EvalChannels, EvalError> {
        let cache = ColdOwner::new(NeuralCache::new());
        cache.begin_epoch();
        let result = self.evaluate_channels_cached(tree, seeds, operator_infos, dispatch, &cache, &HashSet::new(), None);
        cache.sweep();
        result
    }

    #[allow(clippy::too_many_arguments, reason = "incremental cache eval needs tree+seeds+infos+dispatch+cache+dirty+previous together; splitting into a params struct would ripple into flow/core/rs call sites outside this ticket's scope")]
    pub fn evaluate_channels_cached(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
    ) -> Result<EvalChannels, EvalError> {
        let levels = topo_levels(tree)?;
        let mut outputs = ColdOwner::new(seeds.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<String, Dictionary>>());
        let mut inputs = ColdOwner::new(BTreeMap::<String, Dictionary>::new());
        for level in levels {
            let mut level_inputs = ColdOwner::new(BTreeMap::<String, Dictionary>::new());
            let mut level_outputs = ColdOwner::new(BTreeMap::<String, Dictionary>::new());
            let mut deferred_clusters = ColdOwner::new(Vec::<(String, Tree, Dictionary)>::new());
            let mut compute_jobs = ColdOwner::new(Vec::<(String, String, Dictionary)>::new());

            for neuron_id in &level {
                if !dirty.contains(neuron_id) {
                    if let Some(prev) = previous {
                        if let (Some(out), Some(inp)) = (prev.outputs.get(neuron_id), prev.inputs.get(neuron_id)) {
                            level_outputs.insert(neuron_id.clone(), out.clone()).retire_cold();
                            level_inputs.insert(neuron_id.clone(), inp.clone()).retire_cold();
                            continue;
                        }
                    }
                }
                let neuron = tree.neurons.iter().find(|n| n.id == *neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
                let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
                let input = ColdOwner::new(collect_neuron_input(tree, &outputs, neuron_id, operator_info)?);
                level_inputs.insert(neuron_id.clone(), input.clone()).retire_cold();
                if let Some(seed) = seeds.get(neuron_id) {
                    level_outputs.insert(neuron_id.clone(), seed.clone()).retire_cold();
                    continue;
                }
                if let Some(sub_tree) = neuron.tree.as_deref().cloned() {
                    deferred_clusters.push((neuron_id.clone(), sub_tree, input.into_inner()));
                    continue;
                }
                if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                    level_outputs.insert(neuron_id.clone(), input.merge(&neuron.params)).retire_cold();
                    continue;
                }
                compute_jobs.push((neuron_id.clone(), neuron.kind.clone(), input.merge(&neuron.params)));
            }

            for (neuron_id, kind, merged) in compute_jobs.into_inner() {
                let merged = ColdOwner::new(merged);
                let out = evaluate_cached_output(cache, &kind, &merged, || dispatch(&kind, &merged));
                level_outputs.insert(neuron_id, out).retire_cold();
            }

            for (neuron_id, sub_tree, input) in deferred_clusters.into_inner() {
                let sub_tree = ColdOwner::new(sub_tree); let input = ColdOwner::new(input);
                let out = self.evaluate_cluster(&sub_tree, &input, operator_infos, dispatch, cache)?;
                level_outputs.insert(neuron_id, out).retire_cold();
            }

            for (key, value) in level_inputs.into_inner() { inputs.insert(key, value).retire_cold(); }
            for (key, value) in level_outputs.into_inner() { outputs.insert(key, value).retire_cold(); }
        }
        Ok(EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() })
    }

    /// 🧮️ Evaluates a tree as a function: in dictionary to out dictionary via boundary neurons.
    pub fn evaluate_function(&self, tree: &Tree, in_dict: &Dictionary) -> Result<Dictionary, EvalError> {
        self.evaluate_function_with(tree, in_dict, &HashMap::new(), &|kind, input| self.registry.dispatch(kind, input))
    }

    /// 🧮️ Evaluates a tree as a function with custom dispatch and operator metadata.
    pub fn evaluate_function_with(&self, tree: &Tree, in_dict: &Dictionary, operator_infos: &HashMap<String, OperatorInfo>, dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync)) -> Result<Dictionary, EvalError> {
        let seeds = ColdOwner::new(seed_input_boundaries(tree, in_dict));
        let channels = ColdOwner::new(self.evaluate_channels_with(tree, &seeds, operator_infos, dispatch)?);
        collect_output_boundaries(tree, &channels)
    }

    /// 🧮️ Evaluates a tree as a function with caching and custom dispatch.
    pub fn evaluate_function_cached(
        &self,
        tree: &Tree,
        in_dict: &Dictionary,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let seeds = ColdOwner::new(seed_input_boundaries(tree, in_dict));
        let channels = ColdOwner::new(self.evaluate_channels_cached(tree, &seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(tree, &channels)
    }

    fn evaluate_cluster_sequential(
        &self,
        sub_tree: &Tree,
        parent_input: &Dictionary,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let sub_seeds = ColdOwner::new(seed_input_boundaries(sub_tree, parent_input));
        let sub_channels = ColdOwner::new(self.evaluate_channels_sequential_cached(sub_tree, &sub_seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(sub_tree, &sub_channels)
    }

    fn evaluate_cluster(
        &self,
        sub_tree: &Tree,
        parent_input: &Dictionary,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let sub_seeds = ColdOwner::new(seed_input_boundaries(sub_tree, parent_input));
        // 🧩️ Nested cluster subtrees are evaluated atomically (v1 limitation): any change inside
        // re-evaluates the whole cluster rather than propagating dirtiness within it. Never stale,
        // just not maximally incremental for nested clusters.
        let sub_channels = ColdOwner::new(self.evaluate_channels_cached(sub_tree, &sub_seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(sub_tree, &sub_channels)
    }
}

fn operator_info_for_neuron<'a>(neuron: &Neuron, operator_infos: &'a HashMap<String, OperatorInfo>, registry_info: Option<&'a OperatorInfo>) -> Option<&'a OperatorInfo> {
    if neuron.tree.is_some() {
        return None;
    }
    operator_infos.get(&neuron.kind).or(registry_info)
}

fn boundary_seed_dictionary(value: &Value) -> Dictionary {
    match value {
        Value::Dictionary(dict) => dict.clone(),
        other => Dictionary::new().insert("value", other.clone()),
    }
}

fn boundary_output_value(input: &Dictionary) -> Option<Value> {
    if input.len() == 1 {
        return input.keys().next().and_then(|key| input.get(key).cloned());
    }
    if input.is_empty() {
        return None;
    }
    Some(Value::Dictionary(input.clone()))
}

/// 🌱️ Seeds input boundary neurons from an in dictionary keyed by channel name.
pub fn seed_input_boundaries(tree: &Tree, in_dict: &Dictionary) -> HashMap<String, Dictionary> {
    let mut seeds = HashMap::new();
    for neuron in &tree.neurons {
        if neuron.kind != INPUT_KIND {
            continue;
        }
        let (channel_id, _) = contract_channel(neuron);
        let Some(value) = in_dict.get(&channel_id) else {
            continue;
        };
        seeds.insert(neuron.id.clone(), boundary_seed_dictionary(value));
    }
    seeds
}

/// 📤️ Collects output boundary neuron values into an out dictionary keyed by channel name.
pub fn collect_output_boundaries(tree: &Tree, channels: &EvalChannels) -> Result<Dictionary, EvalError> {
    let mut out = ColdDictionaryBuilder::new();
    for neuron in &tree.neurons {
        if neuron.kind != OUTPUT_KIND {
            continue;
        }
        let (channel_id, _) = contract_channel(neuron);
        let Some(neuron_input) = channels.inputs.get(&neuron.id) else {
            return Err(EvalError::MissingInput(format!("output boundary {channel_id}")));
        };
        let Some(value) = boundary_output_value(neuron_input) else {
            return Err(EvalError::MissingInput(format!("output boundary {channel_id}")));
        };
        out.insert(channel_id, value);
    }
    Ok(out.finish())
}

fn synapse_source_value(src_out: &Dictionary, from_port: &str) -> Value {
    if from_port.is_empty() || from_port == "out" {
        if from_port == "out" {
            if let Some(value) = src_out.get("out") {
                return value.clone();
            }
            if src_out.len() == 1 {
                if let Some(key) = src_out.keys().next() {
                    if let Some(value) = src_out.get(key) {
                        return value.clone();
                    }
                }
            }
        }
        return Value::Dictionary(src_out.clone());
    }
    src_out.get(from_port).cloned().unwrap_or_else(|| Value::Dictionary(Dictionary::new().insert("error", Value::Atom(Atom::String(format!("missing channel {from_port}"))))))
}

fn insert_variadic_slot(acc: Dictionary, slot_key: &str, port_id: &str, value: Value) -> Dictionary {
    let mut slots = acc.get(slot_key).and_then(|v| v.as_dictionary()).cloned().unwrap_or_default();
    slots = slots.insert(port_id.to_string(), value);
    acc.insert(slot_key.to_string(), Value::Dictionary(slots))
}

fn insert_fixed_port(acc: Dictionary, port_key: &str, value: Value) -> Dictionary {
    acc.insert(port_key.to_string(), value)
}

/// 💉️ Fills missing declared input keys from operator channel defaults.
pub fn inject_channel_defaults(acc: Dictionary, operator_info: &OperatorInfo) -> Dictionary {
    let mut acc = acc;
    for spec in &operator_info.inputs {
        if spec.name == "*" || acc.get(&spec.name).is_some() {
            continue;
        }
        if let Some(default) = &spec.default {
            acc = acc.insert(spec.name.clone(), default.clone());
        }
    }
    acc
}

fn inject_channel_defaults_for_operator(acc: Dictionary, operator_info: Option<&OperatorInfo>) -> Dictionary {
    match operator_info {
        Some(info) => inject_channel_defaults(acc, info),
        None => acc,
    }
}

fn list_item_count(list: &Dictionary) -> usize {
    list.keys().filter_map(|key| key.parse::<usize>().ok()).count()
}

fn validate_homogeneous_list(list: &Dictionary) -> Result<(), EvalError> {
    let mut expected: Option<String> = None;
    for key in list.keys().filter_map(|key| key.parse::<usize>().ok().map(|index| index.to_string())) {
        let Some(value) = list.get(&key) else { continue };
        if value.is_null() {
            continue;
        }
        let Some(item) = value.as_dictionary() else {
            return Err(EvalError::HeterogeneousList(format!("list item {key} is not a dictionary")));
        };
        let schema = item.schema().unwrap_or("").to_string();
        match &expected {
            None => expected = Some(schema),
            Some(current) if current == &schema => {}
            Some(current) => {
                return Err(EvalError::HeterogeneousList(format!("list mixes schema {current} and {schema}")));
            }
        }
    }
    Ok(())
}

fn validate_channel_value(channel: &ChannelSpec, value: Option<&Value>) -> Result<(), EvalError> {
    if channel.name == "*" {
        return Ok(());
    }
    if let Some(value) = value {
        if value.is_null() {
            return Ok(());
        }
    }
    if channel.cardinality.is_collection() {
        let count = match value {
            None => 0,
            Some(Value::Dictionary(list)) if list.schema() == Some("list") => list_item_count(list),
            Some(_) => {
                return Err(EvalError::CardinalityViolation(format!("channel {} expects a list dictionary", channel.name)));
            }
        };
        if !channel.cardinality.accepts(count) {
            return Err(EvalError::CardinalityViolation(format!("channel {} cardinality {} rejects count {count}", channel.name, channel.cardinality.symbol())));
        }
        if let Some(Value::Dictionary(list)) = value {
            validate_homogeneous_list(list)?;
        }
        return Ok(());
    }
    let count = usize::from(value.is_some());
    if !channel.cardinality.accepts(count) {
        return Err(EvalError::CardinalityViolation(format!("channel {} cardinality {} rejects count {count}", channel.name, channel.cardinality.symbol())));
    }
    Ok(())
}

fn validate_neuron_inputs(acc: &Dictionary, operator_info: Option<&OperatorInfo>) -> Result<(), EvalError> {
    let Some(info) = operator_info else {
        return Ok(());
    };
    if info.variadic_input.is_some() {
        return Ok(());
    }
    for channel in &info.inputs {
        if channel.name == "*" {
            continue;
        }
        let value = acc.get(&channel.name);
        if value.is_none() && channel.default.is_none() {
            continue;
        }
        validate_channel_value(channel, value)?;
    }
    Ok(())
}

fn validate_operator_outputs(info: &OperatorInfo, output: &Dictionary) -> Result<(), EvalError> {
    if info.variadic_output.is_some() {
        return Ok(());
    }
    for channel in &info.outputs {
        validate_channel_value(channel, output.get(&channel.name))?;
    }
    Ok(())
}

fn collect_neuron_input(tree: &Tree, outputs: &BTreeMap<String, Dictionary>, neuron_id: &str, operator_info: Option<&OperatorInfo>) -> Result<Dictionary, EvalError> {
    let mut acc = Dictionary::new();
    let variadic = operator_info.and_then(|info| info.variadic_input.as_ref());
    for syn in &tree.synapses {
        if syn.to != neuron_id {
            continue;
        }
        let Some(src_out) = outputs.get(&syn.from) else { continue };
        let value = synapse_source_value(src_out, &syn.from_port);
        if let Some(spec) = variadic {
            let port_id = if syn.to_port.is_empty() { "0" } else { syn.to_port.as_str() };
            acc = insert_variadic_slot(acc, &spec.slot_key, port_id, value);
            continue;
        }
        if syn.to_port.is_empty() {
            if let Value::Dictionary(dict) = value {
                let next = acc.merge(&dict);
                acc.retire_cold(); dict.retire_cold(); acc = next;
            }
            continue;
        }
        acc = insert_fixed_port(acc, &syn.to_port, value);
    }
    let acc = ColdOwner::new(inject_channel_defaults_for_operator(acc, operator_info));
    validate_neuron_inputs(&acc, operator_info)?;
    Ok(acc.into_inner())
}

fn channel_schema(input: &Dictionary, channel: &ChannelSpec) -> String {
    input
        .get(&channel.name)
        .and_then(|value| value.as_dictionary())
        .and_then(|dictionary| dictionary.schema())
        .map(str::to_string)
        .or_else(|| channel.default.as_ref().and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.schema()).map(str::to_string))
        .unwrap_or_default()
}

fn operator_signature(info: &OperatorInfo, input: &Dictionary) -> Vec<String> {
    if let Some(variadic) = &info.variadic_input {
        return input
            .get(&variadic.slot_key)
            .and_then(|value| value.as_dictionary())
            .map(|items| {
                let mut keys: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
                keys.sort_unstable();
                keys.into_iter().filter_map(|index| items.get(&index.to_string())).filter_map(|value| value.as_dictionary()).filter_map(|dictionary| dictionary.schema()).map(str::to_string).collect()
            })
            .unwrap_or_default();
    }
    info.inputs.iter().filter(|channel| channel.name != "*").map(|channel| channel_schema(input, channel)).collect()
}

fn topo_order(tree: &Tree) -> Result<Vec<String>, EvalError> {
    Ok(topo_levels(tree)?.into_iter().flatten().collect())
}

// 🚧️ Not migrated to `graph::algorithms::topo_levels`: doing so would create a circular crate
// dependency (`neural_engine` → `semio-framework-graph` → `graph::manifest` → `neural_engine`,
// via manifest's `PropertyValue` → `neural_engine::Value` conversion). Fixing that requires relocating
// `Value`/`Atom` out of `neural_engine` into a shared lower crate — out of this ticket's scope.
fn topo_levels(tree: &Tree) -> Result<Vec<Vec<String>>, EvalError> {
    let ids: HashSet<String> = tree.neurons.iter().map(|n| n.id.clone()).collect();
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    for id in &ids {
        incoming.insert(id.clone(), vec![]);
    }
    for syn in &tree.synapses {
        if !ids.contains(&syn.from) || !ids.contains(&syn.to) {
            continue;
        }
        incoming.entry(syn.to.clone()).or_default().push(syn.from.clone());
    }
    let mut indegree: HashMap<String, usize> = incoming.iter().map(|(k, v)| (k.clone(), v.len())).collect();
    let mut queue: VecDeque<String> = indegree.iter().filter(|(_, &d)| d == 0).map(|(k, _)| k.clone()).collect();
    queue.make_contiguous().sort();
    let mut levels = Vec::new();
    let mut visited = 0usize;
    while !queue.is_empty() {
        let mut level: Vec<String> = queue.drain(..).collect();
        level.sort();
        visited += level.len();
        for n in &level {
            for syn in &tree.synapses {
                if syn.from != *n {
                    continue;
                }
                if let Some(d) = indegree.get_mut(&syn.to) {
                    *d = d.saturating_sub(1);
                    if *d == 0 {
                        queue.push_back(syn.to.clone());
                    }
                }
            }
        }
        levels.push(level);
    }
    if visited != ids.len() {
        return Err(EvalError::CycleDetected);
    }
    Ok(levels)
}
// #endregion 🔖️Evaluator

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
````

## 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs

Bytes 13914; SHA-256 9aad5bf0d548a03b678c597623ec8d631a47b58b782c93088b2854014601748c.

````text
//! 🧹️ Exact nested-value retirement and explicitly synchronous construction owners.

use super::{Atom, ChannelSpec, Dictionary, EvalChannels, FieldSpec, NeuronSnapshot, OperatorInfo, Schema, TreeSnapshot, Value, ValueType};
use protocol::value::ordered::{Grant, OrderedMap, Retirement, RetirementStep};
use std::collections::{BTreeMap, LinkedList};
use std::mem::{size_of, ManuallyDrop};
use std::sync::Arc;

//#region 🧵️DomainRetirement
enum Owner {
    Map(Retirement<Value>), Value(Value), Shared(Arc<Value>),
    /// 🎟️ A byte buffer plus the payload bytes still to be drawn down before it is freed. A
    /// `Vec<u8>` cannot be freed in pieces, so the grant is charged against `remaining_bytes` one
    /// turn at a time and the whole buffer is released once the charge reaches zero — the
    /// `min(grant, left)` drawdown the language-agnostic contract states
    /// (`🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts`). An all-or-nothing release would answer
    /// `Blocked` to every fixed-page driver in the tree and spin forever
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    Bytes { values: Vec<u8>, remaining_bytes: usize },
    Strings(Vec<String>),
    Dictionaries(BTreeMap<String, Dictionary>), Snapshot(TreeSnapshot), Neurons(BTreeMap<String, NeuronSnapshot>), Seeds(BTreeMap<String, u64>),
    Operator(OperatorInfo), Channels(Vec<ChannelSpec>), Schema(Schema), Fields(Vec<FieldSpec>), Type(ValueType),
}

/// 🎟️ One byte-buffer owner whose drawdown charge starts at its live payload length.
fn byte_owner(values: Vec<u8>) -> Owner {
    let remaining_bytes = values.len();
    Owner::Bytes { values, remaining_bytes }
}
/// 🎟️ Exact released payload bytes and one retained structural ownership operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueRetirementStep { Blocked, Pending { released_items: usize, released_bytes: usize }, Complete }

/// 🔒️ Owns all nested dictionary and string frontiers until explicit terminal-empty close.
#[must_use = "nested value retirement must reach terminal-empty before drop"]
pub struct ValueRetirement { owners: ManuallyDrop<LinkedList<Owner>> }
impl Default for ValueRetirement { fn default() -> Self { Self { owners: ManuallyDrop::new(LinkedList::new()) } } }
impl ValueRetirement {
    pub fn from_value(value: Value) -> Self { let mut owner = Self::default(); owner.push_value(value); owner }
    pub fn from_dictionary(value: Dictionary) -> Self { let mut owner = Self::default(); owner.push_dictionary(value); owner }
    pub fn push_value(&mut self, value: Value) { self.owners.push_back(Owner::Value(value)); }
    pub fn push_shared(&mut self, value: Arc<Value>) { self.owners.push_back(Owner::Shared(value)); }
    pub fn push_dictionary(&mut self, mut dictionary: Dictionary) { self.push_map(std::mem::take(&mut dictionary.pairs)); }
    pub fn text(&mut self, text: String) { self.owners.push_back(byte_owner(text.into_bytes())); }
    pub fn push_dictionaries(&mut self, values: BTreeMap<String, Dictionary>) { self.owners.push_back(Owner::Dictionaries(values)); }
    pub fn push_channels(&mut self, channels: EvalChannels) { self.push_dictionaries(channels.outputs); self.push_dictionaries(channels.inputs); }
    pub fn push_snapshot(&mut self, snapshot: TreeSnapshot) { self.owners.push_back(Owner::Snapshot(snapshot)); }
    pub fn push_operator(&mut self, operator: OperatorInfo) { self.owners.push_back(Owner::Operator(operator)); }
    pub fn push_schema(&mut self, schema: Schema) { self.owners.push_back(Owner::Schema(schema)); }
    pub fn push_strings(&mut self, strings: Vec<String>) { self.owners.push_back(Owner::Strings(strings)); }
    pub fn terminal_is_empty(&self) -> bool { self.owners.is_empty() }
    pub fn allocated_bytes(&self) -> usize {
        self.owners.iter().fold(0usize, |total, owner| total.saturating_add(match owner {
            Owner::Map(values) => values.allocated_bytes(),
            Owner::Bytes { values, .. } => values.capacity(),
            Owner::Strings(values) => values.capacity().saturating_mul(size_of::<String>()),
            Owner::Channels(values) => values.capacity().saturating_mul(size_of::<ChannelSpec>()),
            Owner::Fields(values) => values.capacity().saturating_mul(size_of::<FieldSpec>()),
            _ => 0,
        }))
    }
    /// 🎟️ One byte of credit per turn is all this frontier ever needs: every owner is either
    /// structural or charged `min(grant, left)` against its live payload, so any positive grant
    /// makes progress. Named rather than inlined because [`crate::retained::FlowRetirement`] asks
    /// its nested owners for their close demand (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn next_close_byte_demand(&self) -> Result<usize, &'static str> {
        Ok(usize::from(!self.owners.is_empty()))
    }
    pub(crate) fn push_map(&mut self, map: OrderedMap<Value>) { let retirement = map.retire(); if !retirement.is_empty() { self.owners.push_back(Owner::Map(retirement)); } }


    /// 🎟️ Releases one owner and at most `maximum_bytes` payload bytes. TOTAL under any positive
    /// grant: `Blocked` means the caller offered no credit at all, never that an owner is too big
    /// for this page. Every driver in the tree hands a fixed page (1, 64, 4096) and only ever
    /// closes, so an owner that could refuse a positive grant is an unbreakable spin — see the
    /// drawdown law in `🧪️tests/🧪️source-contract/🟦️.ts`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> ValueRetirementStep {
        if self.owners.is_empty() { return ValueRetirementStep::Complete; }
        if maximum_items == 0 || maximum_bytes == 0 { return ValueRetirementStep::Blocked; }
        let owner = self.owners.pop_front().expect("checked nonempty neural retirement");
        let mut released_bytes = 0;
        match owner {
            Owner::Map(mut map) => {
                let step = map.advance(Grant { maximum_items, maximum_bytes });
                if !map.is_empty() { self.owners.push_front(Owner::Map(map)); }
                match step {
                    RetirementStep::OwnedValue(value) => self.owners.push_front(Owner::Value(value)),
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::Blocked => return ValueRetirementStep::Blocked,
                    RetirementStep::Complete => {}
                }
            }
            Owner::Shared(value) => if let Some(value) = Arc::into_inner(value) { self.owners.push_front(Owner::Value(value)); },
            Owner::Value(Value::Dictionary(dictionary)) => self.push_dictionary(dictionary),
            Owner::Value(Value::Atom(Atom::String(text))) => self.owners.push_front(byte_owner(text.into_bytes())),
            Owner::Value(Value::Atom(_)) => {}
            Owner::Strings(mut values) if !values.is_empty() => {
                if let Some(value) = values.pop() { self.text(value); }
                self.owners.push_front(Owner::Strings(values));
            }
            Owner::Strings(values) => drop(values),
            Owner::Dictionaries(mut values) => {
                if let Some((key, value)) = values.pop_first() { self.text(key); self.push_dictionary(value); }
                if !values.is_empty() { self.owners.push_front(Owner::Dictionaries(values)); }
            }
            Owner::Snapshot(value) => { self.owners.push_front(Owner::Neurons(value.neurons)); self.owners.push_front(Owner::Seeds(value.seed_keys)); }
            Owner::Neurons(mut values) => {
                if let Some((key, value)) = values.pop_first() { self.text(key); self.owners.push_back(Owner::Strings(value.dependents)); }
                if !values.is_empty() { self.owners.push_front(Owner::Neurons(values)); }
            }
            Owner::Seeds(mut values) => {
                if let Some((key, _)) = values.pop_first() { self.text(key); }
                if !values.is_empty() { self.owners.push_front(Owner::Seeds(values)); }
            }
            Owner::Operator(value) => {
                self.text(value.id); self.text(value.extension); self.text(value.name); self.text(value.abbreviation); self.text(value.icon); self.text(value.summary);
                self.owners.push_back(Owner::Channels(value.inputs)); self.owners.push_back(Owner::Channels(value.outputs)); self.owners.push_back(Owner::Strings(value.group));
                if let Some(value) = value.variadic_input { self.text(value.slot_key); }
                if let Some(value) = value.variadic_output { self.text(value.slot_key); }
            }
            Owner::Channels(mut values) if !values.is_empty() => {
                if let Some(value) = values.pop() {
                    self.text(value.code); self.text(value.abbreviation); self.text(value.name); self.text(value.full_name);
                    if let Some(label) = value.label { self.text(label); }
                    if let Some(default) = value.default { self.push_value(default); }
                    self.owners.push_back(Owner::Strings(value.operators));
                }
                self.owners.push_front(Owner::Channels(values));
            }
            Owner::Channels(values) => drop(values),
            Owner::Schema(value) => {
                self.text(value.id); self.text(value.module); self.text(value.name); self.text(value.icon); self.text(value.summary);
                self.owners.push_back(Owner::Fields(value.fields));
            }
            Owner::Fields(mut values) if !values.is_empty() => {
                if let Some(value) = values.pop() {
                    self.text(value.key);
                    if let Some(label) = value.label { self.text(label); }
                    if let Some(default) = value.default { self.push_value(default); }
                    self.owners.push_back(Owner::Type(value.value));
                }
                self.owners.push_front(Owner::Fields(values));
            }
            Owner::Fields(values) => drop(values),
            Owner::Type(ValueType::Schema(id)) => self.text(id),
            Owner::Type(ValueType::List(inner)) => self.owners.push_front(Owner::Type(*inner)),
            Owner::Type(_) => {}
            Owner::Bytes { values, remaining_bytes } => {
                released_bytes = maximum_bytes.min(remaining_bytes);
                let remaining_bytes = remaining_bytes - released_bytes;
                if remaining_bytes != 0 {
                    self.owners.push_front(Owner::Bytes { values, remaining_bytes });
                    return ValueRetirementStep::Pending { released_items: 1, released_bytes };
                }
                drop(values);
            }
        }
        ValueRetirementStep::Pending { released_items: 1, released_bytes }
    }
}
impl Drop for ValueRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(), "neural values must finish explicit domain retirement before drop"); } }
}
//#endregion 🧵️DomainRetirement

//#region 🧊️ColdOwners
/// 🧊️ Explicit synchronous boundary; never used by retained advance or close operations.
pub fn retire_value_cold(mut owner: ValueRetirement) {
    while !matches!(owner.close_step(1, 4096), ValueRetirementStep::Complete) {}
}

/// 🧊️ Cold construction owns replacement and error cleanup; its name makes unbounded work explicit.
pub struct ColdDictionaryBuilder { dictionary: Option<Dictionary> }
impl Default for ColdDictionaryBuilder { fn default() -> Self { Self { dictionary: Some(Dictionary::new()) } } }
impl ColdDictionaryBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn from_dictionary(dictionary: Dictionary) -> Self { Self { dictionary: Some(dictionary) } }
    pub fn dictionary(&self) -> &Dictionary { self.dictionary.as_ref().unwrap() }
    pub fn insert(&mut self, key: String, value: Value) {
        let dictionary = self.dictionary.as_mut().unwrap();
        let mut update = dictionary.pairs.begin_set(key, value);
        let grant = Grant { maximum_items: 1, maximum_bytes: 4096 };
        while !update.is_complete() { update.advance(grant); }
        let displaced = std::mem::replace(&mut dictionary.pairs, update.take_result().unwrap());
        let mut retirement = ValueRetirement::default(); retirement.push_map(displaced); retire_value_cold(retirement);
        update.begin_close();
        loop {
            match update.close_step(grant) {
                RetirementStep::OwnedValue(value) => retire_value_cold(ValueRetirement::from_value(value)),
                RetirementStep::Complete => break,
                RetirementStep::Blocked => unreachable!("positive cold builder grant"),
                RetirementStep::Progress { .. } => {}
            }
        }
        assert!(update.terminal_is_empty());
    }
    pub fn finish(mut self) -> Dictionary { self.dictionary.take().unwrap() }
}
impl Drop for ColdDictionaryBuilder {
    fn drop(&mut self) { if let Some(dictionary) = self.dictionary.take() { retire_value_cold(ValueRetirement::from_dictionary(dictionary)); } }
}

/// 🧊️ Explicit cold value scope for batch evaluation, decoding, and tests; retained owners use ValueRetirement.
pub struct ColdValueOwner { value: Option<Value> }
impl ColdValueOwner {
    pub fn new(value: Value) -> Self { Self { value: Some(value) } }
    pub fn value(&self) -> &Value { self.value.as_ref().unwrap() }
    pub fn into_value(mut self) -> Value { self.value.take().unwrap() }
}
impl Drop for ColdValueOwner {
    fn drop(&mut self) { if let Some(value) = self.value.take() { retire_value_cold(ValueRetirement::from_value(value)); } }
}
//#endregion 🧊️ColdOwners

#[cfg(test)]
#[path = "🧪️tests/🧵️retirement/🦀️.rs"]
mod tests;
````

## 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust/Cargo.toml

Bytes 1187; SHA-256 26da977ee82c0ad87eab27f3935f048fba826cce9f8fdcc2e9c684fba4ee34d0.

````text
[package]
workspace = "../../../../../../../.."
name = "semio-framework-os-kernel-neural-engine"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true

[package.metadata.semio]
role = "product"
id = "os-neural-engine"

[lints]
workspace = true

[lib]
name = "neural_engine"
crate-type = ["rlib"]
path = "🦀️.rs"

[dependencies]
semio-framework-value = { path = "../../../../../../../🔨️modules/🌱️value/📦️packages/🦀️rust" }
semio-framework-replication = { path = "../../../../../../../🔨️modules/📡️replication/📦️packages/🦀️rust" }
# 🎒️ `pack::json::to_json_string` — the evaluator's pending-extension branch needs a JSON string of
# a `Dictionary` for its host-bound `input_json` payload (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-
# AND-ARTIFACTS, 26/09/01, tenth-seam pass). No cycle: `pack` depends on `replication`, not the
# reverse, and this crate already depends on `replication` directly.
pack = { path = "../../../../../../../🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }

[dev-dependencies]
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
````

## 🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml

Bytes 1981; SHA-256 ba2b9dc265c46784098448e55c3f6fbadc357e820288938bff75052650863cd2.

````text
[package]
workspace = "../../../../.."
name = "semio-framework-graph"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Graph vocabulary, index-based algorithms, drawing layouts and the compile-time graph manifest registry — the framework-internal graph surface every framework crate may name without reaching a plugin"
build = false

[package.metadata.semio]
role = "framework"
id = "graph"

[lints]
workspace = true

[lib]
name = "semio_framework_graph"
path = "🦀️.rs"

[dependencies]
semio-framework-ui-locale = { path = "../../../🖱️ui/🌐️locale/📦️packages/🦀️rust" }
semio-framework-value = { path = "../../../🌱️value/📦️packages/🦀️rust" }
semio-framework-ui-contract = { path = "../../../🖱️ui/🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }
geometry = { path = "../../../📐️geometry/📦️packages/🦀️rust", package = "semio-framework-geometry" }
semio-framework-os-kernel = { path = "../../../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
neural_engine = { path = "../../../../🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust", package = "semio-framework-os-kernel-neural-engine" }
semio-framework-value-derive = { path = "../../../🌱️value/✨️derive/📦️packages/🦀️rust", package = "semio-framework-value-derive" }
# ⚠️ `serde`/`serde_json` are fully removed — every type in this crate carries `ToValue`/
# `FromValue` instead of `Serialize`/`Deserialize`. `⚙️engine/🦀️.rs`'s `property_bag_from_value`/
# `property_bag_to_value` now take/return `dsl_core::DslValue`; `♾️infinite`'s board ports
# (the former external caller) bridge their own `serde_json::Value`-typed `user_data` field at
# the call site via the existing `DslValue::from` bridge. RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02.
````

## 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs

Bytes 671; SHA-256 7215a515fd2d8fdbbd9ca50a6590b56b522d57a87fc7b00015a141cb56d59c7e.

````text
//! 🌱️ Actual neutral value package; types are mounted once below replication and products.
extern crate self as semio_framework_value;
#[path = "../../🦀️.rs"]
pub mod value;
pub use value::*;
pub use serde;
pub use serde_json;
pub use semio_framework_io_base64::{base64_standard_encode,base64_standard_decode};
pub use semio_framework_value_derive::{FromValue, RetainedClone, RetireOwned, ToValue};

#[path = "../../♻️retirement/🧬️contract/🦀️.rs"]
mod retirement_contract;
pub use retirement_contract::*;
#[path = "../../♻️retirement/🦀️.rs"]
pub mod retirement;
#[path = "../../🧬️retained-clone/🦀️.rs"]
pub mod retained_clone;
````

## 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts

Bytes 2450; SHA-256 671eb4fa2443d9c8f08e8ece07be127833cb158d1c873a1947b343f729f1014b.

````text
#!/usr/bin/env bun
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🌱️ Tests the actual neutral value package under its explicitly supplied native policy. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args[0] === "portable") {
      if (args.length !== 1) throw Error("Expected test portable");
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🔁️codec/🧪️tests/🛬️controlled/🟦️.ts")], this.repoRoot, "value:portable:construction", 15_000);
      return;
    }
    const { rest } = resolveTestLevel(args);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-value"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}
/** 🛬️ Exercises canonical borrowed construction, allocation admission and owner retirement. */
class ControlledValueTestScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    const {rest}=resolveTestLevel(args);
    await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","controlled_value_",...rest]},readCargoTestPolicyV1(process.env));
  }
}
/** 🛫️ Exercises explicit borrowed output construction and cumulative encoding admission. */
class ControlledEncodingTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{const{rest}=resolveTestLevel(args);await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","controlled_value_encoding_",...rest]},readCargoTestPolicyV1(process.env));}
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-controlled-construction", ControlledValueTestScript).register("test-controlled-encoding",ControlledEncodingTestScript), { defaultCommand: "test" });
````

## 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json

Bytes 1592; SHA-256 1c80a474365ea37e0259c3b034ca5e8434efb41ce17bcc8c09160609e8c95515.

````text
{
  "name": "@semio-tech/value-rs",
  "projectType": "library",
  "namedInputs": {
    "default": [
      "{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/**/*",
      "sharedGlobals"
    ]
  },
  "targets": {
    "test": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-controlled-construction": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-controlled-construction",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-controlled-encoding": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-controlled-encoding",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-portable": {
      "executor": "nx:run-commands",
      "cache": false,
      "inputs": [
        "default",
        "^production"
      ],
      "outputs": [],
      "dependsOn": [],
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test portable"
      }
    }
  }
}
````

## 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/package.json

Bytes 202; SHA-256 0ffdc52dd3fadc49e05fa87d5d99f163542eead6137a7a742582bc845f5d7f24.

````text
{
  "name": "@semio-tech/value-rs",
  "private": true,
  "type": "module",
  "nx": {
    "includedScripts": []
  },
  "scripts": {
    "test-portable": "nx run @semio-tech/value-rs:test-portable"
  }
}
````

## 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs

Bytes 29577; SHA-256 cbaf1577e77e9661ccc8b6d724b084841cfd13c095856d5755f2d7b269b76f87.

````text
//! 📜️ Compile-time graph manifest kernel: schema, registry, and strict validation.

use neural_engine::Value;
pub use neural_engine::ValueType;

pub use crate::manifest::Manifest as GraphManifest;

//#region ⚠️ Errors
// 🌉️ `value_type_from_value` below is a `#[value(deserialize_with = "...")]` hook (RUNTIME-
// DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02 Phase 2) — the direct successor of
// the old serde `deserialize_with = "deserialize_value_type"` hook (and its `GraphManifestError`/
// `parse_value_type_value` helpers), NOT dead code: `ValueType`'s own native `dsl_core::FromValue`
// impl decodes its internally-tagged `{"kind": "boolean"}` shape only, but every `*.manifest.json`
// fixture (embedded verbatim as `${PREFIX}_MANIFEST_JSON` by `🤖️generated/🦀️*.rs`) spells
// `valueType` as a bare string (`"boolean"`/`"text"`/...) or a `{"schema": "..."}` object — the
// same gap the serde hook used to bridge. Confirmed by `cargo test -p semio-framework-graph`:
// dropping this hook broke `nakagin_manifest_loads` et al. with `ValueError("nodeKinds.41.
// properties.0.valueType.kind")` before this fix landed. The encode direction needs no matching
// hook: `ValueType::to_value`'s native shape is self-consistent for `Manifest::to_value()`'s own
// round trip (nothing needs it to reproduce the fixture text byte-for-byte) — the old
// `serialize_value_type` hook was itself just a thin wrapper over the same native `to_value` call.
fn value_type_from_value(value: dsl_core::DslValue) -> Result<ValueType, dsl_core::ValueError> {
    if let Ok(value_type) = <ValueType as dsl_core::FromValue>::from_value(value.clone()) {
        return Ok(value_type);
    }
    match value {
        dsl_core::DslValue::String(s) => Ok(match s.as_str() {
            "boolean" | "bool" => ValueType::Boolean,
            "integer" | "int" => ValueType::Integer,
            "number" | "decimal" | "float" => ValueType::Decimal,
            "text" | "string" => ValueType::Text,
            "object" | "any" => ValueType::Any,
            _ => ValueType::Schema(s),
        }),
        dsl_core::DslValue::Object(entries) if entries.len() == 1 => match entries.first() {
            Some((key, dsl_core::DslValue::String(schema))) if key == "schema" => Ok(ValueType::Schema(schema.clone())),
            _ => Err(dsl_core::ValueError::new(format!("unsupported valueType object {:?}", dsl_core::DslValue::Object(entries)))),
        },
        other => Err(dsl_core::ValueError::new(format!("unsupported valueType {other:?}"))),
    }
}
//#endregion ⚠️ Errors

// #region 🔖️Property
/// 📊️ Runtime property value for graph instances.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum PropertyValue {
    #[default]
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<PropertyValue>),
    Object(std::collections::BTreeMap<String, PropertyValue>),
}

#[path = "♻️retirement/🦀️.rs"]
mod retirement;

impl PropertyValue {
    // 🚫️async: E1 pure accessor passed by name into `Option::and_then` (a sync fn-pointer slot) at
    // every call site in this crate; no consumer awaits it directly. See R9.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    // 🚫️async: E1 pure accessor, same reason as `as_str` above — see R9.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&std::collections::BTreeMap<String, PropertyValue>> {
        match self {
            Self::Object(m) => Some(m),
            _ => None,
        }
    }
}

//#region 🔖️DslField
// 🌱️ `PropertyValue` is structurally a dynamic JSON-equivalent literal (Null/Bool/Number/String/
// Array/Object), exactly like `dsl_core::DslValue` itself, so it binds as `Shape::Value` rather than
// through `#[derive(dsl_core::DslEnum)]`: the derive's tuple-variant codegen treats every single-field
// unnamed variant as a "newtype" delegating to the inner type's own `Shape::Record` (see
// `dsl_core::__rt::newtype_variant_spec`), which panics for a primitive/collection inner type such as
// `bool`/`f64`/`Vec<Self>`/`BTreeMap<String, Self>` — none of which are `Shape::Record`. Binding
// directly through `DslValue` (mirroring the engine's own `serde_json::Value` bridge) is both
// correct and the natural fit for an untyped recursive value type, and it needs no attributes on
// the Array/Object variants: recursion is carried by `DslValue` itself, not by field-level nesting.
fn property_value_to_dsl_value(value: &PropertyValue) -> dsl_core::DslValue {
    match value {
        PropertyValue::Null => dsl_core::DslValue::Null,
        PropertyValue::Bool(b) => dsl_core::DslValue::Bool(*b),
        PropertyValue::Number(n) => dsl_core::DslValue::float(*n),
        PropertyValue::String(s) => dsl_core::DslValue::String(s.clone()),
        PropertyValue::Array(items) => {
            // 🔀️ Plain sync recursion — no suspension point, so no `Box::pin` is needed.
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(property_value_to_dsl_value(item));
            }
            dsl_core::DslValue::Array(out)
        }
        PropertyValue::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (k, v) in map {
                out.push((k.clone(), property_value_to_dsl_value(v)));
            }
            dsl_core::DslValue::Object(out)
        }
    }
}

fn dsl_value_to_property_value(value: &dsl_core::DslValue) -> PropertyValue {
    match value {
        dsl_core::DslValue::Null => PropertyValue::Null,
        dsl_core::DslValue::Bool(b) => PropertyValue::Bool(*b),
        dsl_core::DslValue::Number(n) => PropertyValue::Number(n.as_f64()),
        dsl_core::DslValue::String(s) => PropertyValue::String(s.clone()),
        dsl_core::DslValue::Bytes(bytes) => PropertyValue::Array(bytes.iter().map(|byte| PropertyValue::Number(f64::from(*byte))).collect()),
        dsl_core::DslValue::Array(items) => {
            // 🔀️ Same rewrite as `property_value_to_dsl_value` above, mirrored.
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(dsl_value_to_property_value(item));
            }
            PropertyValue::Array(out)
        }
        dsl_core::DslValue::Object(entries) => {
            let mut out = std::collections::BTreeMap::new();
            for (k, v) in entries {
                out.insert(k.clone(), dsl_value_to_property_value(v));
            }
            PropertyValue::Object(out)
        }
    }
}

impl dsl_core::DslField for PropertyValue {
    // 🚫️async: E1 impl of externally-declared trait `dsl_core::DslField` — every method is
    // E4-tagged sync in the trait itself (fn-pointer transitivity through `Shape::Record`/`Table`),
    // see `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs`.
    fn shape() -> dsl_core::Shape {
        dsl_core::Shape::Value
    }

    fn to_value(&self) -> dsl_core::FieldValue {
        dsl_core::FieldValue::Value(property_value_to_dsl_value(self))
    }

    fn from_value(value: &dsl_core::FieldValue) -> Result<Self, String> {
        match value {
            dsl_core::FieldValue::Value(dsl_value) => Ok(dsl_value_to_property_value(dsl_value)),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}
//#endregion 🔖️DslField

//#region 🔖️ToFromValue
/// 🌱️ `ToValue`/`FromValue` (the `DslValue`-tree pair `Mutation`/`MutationDiff` payloads need —
/// distinct from `DslField`/`FieldValue` above, the text/binary DSL grammar's own trait, see that
/// region's header note) for the identical reason `DslField` binds as `Shape::Value`: reuse the
/// same recursive `property_value_to_dsl_value`/`dsl_value_to_property_value` walk rather than a
/// second one. An untagged enum (this was `#[serde(untagged)]`, now serde-free — Phase 2,
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS 26/09/02) has no `#[derive(ToValue,
/// FromValue)]` equivalent — it needs exactly this kind of hand-written structural match, per the
/// fan-out playbook's "Not supported by the derive" list.
impl dsl_core::ToValue for PropertyValue {
    fn to_value(&self) -> dsl_core::DslValue {
        property_value_to_dsl_value(self)
    }
}

impl dsl_core::FromValue for PropertyValue {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(dsl_value_to_property_value(&value))
    }
}
//#endregion 🔖️ToFromValue

/// 🏷️ Compile-time property kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PropertyKind {
    Data,
    Derived,
}

/// 📋️ Property definition on a kind.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PropertyDef {
    pub name: String,
    pub kind: PropertyKind,
    // 🌉️ `deserialize_with` mirrors the old serde hook — see `value_type_from_value`'s own
    // docstring above (this crate's `⚠️ Errors` region) for why it is still needed. The plain
    // per-field `ToValue::to_value` stays for the encode direction (no `serialize_with`).
    #[value(default, deserialize_with = "value_type_from_value")]
    pub value_type: ValueType,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}

/// 📏️ The supplied retirement byte grant cannot release the exact schema string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueTypeRetirementError {
    pub required_bytes: usize,
    pub maximum_bytes: usize,
}

impl std::fmt::Display for ValueTypeRetirementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value type retirement requires {} bytes, granted {}", self.required_bytes, self.maximum_bytes)
    }
}

impl std::error::Error for ValueTypeRetirementError {}

impl PropertyDef {
    /// 🧹️ Detaches at most one exact nested value-type string or list box. A terminal
    /// definition has only the definitionally shallow `Any` tag left for its final drop.
    pub fn retire_value_type_step(&mut self, maximum_bytes: usize) -> Result<Option<String>, ValueTypeRetirementError> {
        if let ValueType::Schema(value) = &self.value_type {
            if value.len() > maximum_bytes {
                return Err(ValueTypeRetirementError { required_bytes: value.len(), maximum_bytes });
            }
        }
        match std::mem::take(&mut self.value_type) {
            ValueType::Schema(value) => Ok(Some(value)),
            ValueType::List(value) => {
                self.value_type = *value;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn value_type_terminal_is_empty(&self) -> bool {
        matches!(self.value_type, ValueType::Any)
    }
}

pub type PropertyBag = std::collections::BTreeMap<String, PropertyValue>;

// #endregion 🔖️Property

// #region 🔖️Manifest
/// 🔌️ Port direction on a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PortDirection {
    In,
    Out,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PortModelAxis {
    #[default]
    Ported,
    Normal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum DirectednessAxis {
    #[default]
    Directed,
    Undirected,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ManifestAxes {
    #[value(default)]
    pub port_model: PortModelAxis,
    #[value(default)]
    pub directedness: DirectednessAxis,
}

/// 🏷️ Kind row in a manifest family.
///
/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` requires every field's type to
/// carry the same rename convention as a per-field attribute, but `presentation` is already the
/// schema-erased `DslValue` itself (arbitrary-shaped, no meaningful rename), so it is simpler to
/// spell the whole impl by hand than to special-case one field's attribute.
#[derive(Clone, Debug, PartialEq)]
pub struct KindDef {
    pub id: String,
    pub name: String,
    pub properties: Vec<PropertyDef>,
    pub ports: Vec<String>,
    pub direction: Option<PortDirection>,
    pub presentation: Option<dsl_core::DslValue>,
}

impl dsl_core::ToValue for KindDef {
    fn to_value(&self) -> dsl_core::DslValue {
        let mut entries: Vec<(String, dsl_core::DslValue)> = vec![
            ("id".to_string(), dsl_core::ToValue::to_value(&self.id)),
            ("name".to_string(), dsl_core::ToValue::to_value(&self.name)),
            ("properties".to_string(), dsl_core::ToValue::to_value(&self.properties)),
            ("ports".to_string(), dsl_core::ToValue::to_value(&self.ports)),
        ];
        if self.direction.is_some() {
            entries.push(("direction".to_string(), dsl_core::ToValue::to_value(&self.direction)));
        }
        if let Some(presentation) = &self.presentation {
            entries.push(("presentation".to_string(), presentation.clone()));
        }
        dsl_core::DslValue::object(entries)
    }
}

impl dsl_core::FromValue for KindDef {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(fields) = value else {
            return Err(dsl_core::ValueError::new(format!("expected an object for KindDef, found {value:?}")));
        };
        let mut id = None;
        let mut name = String::new();
        let mut properties = Vec::new();
        let mut ports = Vec::new();
        let mut direction = None;
        let mut presentation = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "name" => name = <String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("name"))?,
                "properties" => properties = <Vec<PropertyDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("properties"))?,
                "ports" => ports = <Vec<String> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("ports"))?,
                "direction" => direction = Some(<PortDirection as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("direction"))?),
                "presentation" => presentation = Some(entry),
                _ => {}
            }
        }
        Ok(KindDef {
            id: id.ok_or_else(|| dsl_core::ValueError::new("KindDef missing id"))?,
            name,
            properties,
            ports,
            direction,
            presentation,
        })
    }
}

impl KindDef {
    pub fn display_name(&self) -> &str {
        if self.name.is_empty() {
            &self.id
        } else {
            &self.name
        }
    }
}

/// 📜️ Compile-time schema for a graph.
///
/// 🌉️ Hand-written, not derived — same reason as `KindDef` above: `edge_tips`/`kind_compatibility`
/// are schema-erased `DslValue` trees, arbitrary-shaped, so a plain field-list derive buys nothing
/// over spelling the impl directly.
#[derive(Clone, Debug, PartialEq)]
pub struct Manifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub axes: ManifestAxes,
    pub node_kinds: Vec<KindDef>,
    pub edge_kinds: Vec<KindDef>,
    pub port_kinds: Vec<KindDef>,
    pub wire_kinds: Vec<KindDef>,
    pub layer_kinds: Vec<KindDef>,
    pub language_kinds: Vec<KindDef>,
    pub surface_kinds: Vec<KindDef>,
    pub window_kinds: Vec<KindDef>,
    pub file_node_kinds: Vec<KindDef>,
    pub descriptor_kinds: Vec<KindDef>,
    pub edge_tips: Vec<dsl_core::DslValue>,
    pub kind_compatibility: Vec<dsl_core::DslValue>,
}

impl dsl_core::ToValue for Manifest {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::object([
            ("schema".to_string(), dsl_core::ToValue::to_value(&self.schema)),
            ("id".to_string(), dsl_core::ToValue::to_value(&self.id)),
            ("name".to_string(), dsl_core::ToValue::to_value(&self.name)),
            ("axes".to_string(), dsl_core::ToValue::to_value(&self.axes)),
            ("nodeKinds".to_string(), dsl_core::ToValue::to_value(&self.node_kinds)),
            ("edgeKinds".to_string(), dsl_core::ToValue::to_value(&self.edge_kinds)),
            ("portKinds".to_string(), dsl_core::ToValue::to_value(&self.port_kinds)),
            ("wireKinds".to_string(), dsl_core::ToValue::to_value(&self.wire_kinds)),
            ("layerKinds".to_string(), dsl_core::ToValue::to_value(&self.layer_kinds)),
            ("languageKinds".to_string(), dsl_core::ToValue::to_value(&self.language_kinds)),
            ("surfaceKinds".to_string(), dsl_core::ToValue::to_value(&self.surface_kinds)),
            ("windowKinds".to_string(), dsl_core::ToValue::to_value(&self.window_kinds)),
            ("fileNodeKinds".to_string(), dsl_core::ToValue::to_value(&self.file_node_kinds)),
            ("descriptorKinds".to_string(), dsl_core::ToValue::to_value(&self.descriptor_kinds)),
            ("edgeTips".to_string(), dsl_core::DslValue::Array(self.edge_tips.clone())),
            ("kindCompatibility".to_string(), dsl_core::DslValue::Array(self.kind_compatibility.clone())),
        ])
    }
}

impl dsl_core::FromValue for Manifest {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(fields) = value else {
            return Err(dsl_core::ValueError::new(format!("expected an object for Manifest, found {value:?}")));
        };
        let mut schema = None;
        let mut id = None;
        let mut name = String::new();
        let mut axes = ManifestAxes::default();
        let mut node_kinds = Vec::new();
        let mut edge_kinds = Vec::new();
        let mut port_kinds = Vec::new();
        let mut wire_kinds = Vec::new();
        let mut layer_kinds = Vec::new();
        let mut language_kinds = Vec::new();
        let mut surface_kinds = Vec::new();
        let mut window_kinds = Vec::new();
        let mut file_node_kinds = Vec::new();
        let mut descriptor_kinds = Vec::new();
        let mut edge_tips = Vec::new();
        let mut kind_compatibility = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(<String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                "id" => id = Some(<String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "name" => name = <String as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("name"))?,
                "axes" => axes = <ManifestAxes as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("axes"))?,
                "nodeKinds" => node_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("nodeKinds"))?,
                "edgeKinds" => edge_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("edgeKinds"))?,
                "portKinds" => port_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("portKinds"))?,
                "wireKinds" => wire_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("wireKinds"))?,
                "layerKinds" => layer_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("layerKinds"))?,
                "languageKinds" => language_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("languageKinds"))?,
                "surfaceKinds" => surface_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("surfaceKinds"))?,
                "windowKinds" => window_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("windowKinds"))?,
                "fileNodeKinds" => file_node_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("fileNodeKinds"))?,
                "descriptorKinds" => descriptor_kinds = <Vec<KindDef> as dsl_core::FromValue>::from_value(entry).map_err(|e| e.under("descriptorKinds"))?,
                "edgeTips" => {
                    let dsl_core::DslValue::Array(items) = entry else {
                        return Err(dsl_core::ValueError::new("expected an array for edgeTips").under("edgeTips"));
                    };
                    edge_tips = items;
                }
                "kindCompatibility" => {
                    let dsl_core::DslValue::Array(items) = entry else {
                        return Err(dsl_core::ValueError::new("expected an array for kindCompatibility").under("kindCompatibility"));
                    };
                    kind_compatibility = items;
                }
                _ => {}
            }
        }
        Ok(Manifest {
            schema: schema.ok_or_else(|| dsl_core::ValueError::new("Manifest missing schema"))?,
            id: id.ok_or_else(|| dsl_core::ValueError::new("Manifest missing id"))?,
            name,
            axes,
            node_kinds,
            edge_kinds,
            port_kinds,
            wire_kinds,
            layer_kinds,
            language_kinds,
            surface_kinds,
            window_kinds,
            file_node_kinds,
            descriptor_kinds,
            edge_tips,
            kind_compatibility,
        })
    }
}

impl Manifest {
    pub fn node_kind(&self, id: &str) -> Option<&KindDef> {
        self.node_kinds.iter().find(|k| k.id == id)
    }

    pub fn edge_kind(&self, id: &str) -> Option<&KindDef> {
        self.edge_kinds.iter().find(|k| k.id == id)
    }

    pub fn port_kind(&self, id: &str) -> Option<&KindDef> {
        self.port_kinds.iter().find(|k| k.id == id)
    }

    pub fn wire_kind(&self, id: &str) -> Option<&KindDef> {
        self.wire_kinds.iter().find(|k| k.id == id)
    }

    pub fn layer_kind(&self, id: &str) -> Option<&KindDef> {
        self.layer_kinds.iter().find(|k| k.id == id)
    }

    pub fn language_kind(&self, id: &str) -> Option<&KindDef> {
        self.language_kinds.iter().find(|k| k.id == id)
    }

}
// #endregion 🔖️Manifest

// #region 🔖️Validator
/// 🛡️ Strict manifest validation errors.
#[derive(Clone, Debug, PartialEq)]
pub struct ManifestValidationError {
    pub path: String,
    pub message: String,
}

impl ManifestValidationError {
    fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self { path: path.into(), message: message.into() }
    }
}

/// 🛡️ Validates runtime graph instances against a compile-time manifest.
#[derive(Clone, Debug)]
pub struct ManifestValidator<'a> {
    manifest: &'a Manifest,
}

impl<'a> ManifestValidator<'a> {
    pub fn new(manifest: &'a Manifest) -> Self {
        Self { manifest }
    }

    pub fn validate_node_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.node_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("nodes/{kind}"), format!("unknown node kind {kind:?}")))
        }
    }

    pub fn validate_edge_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.edge_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("edges/{kind}"), format!("unknown edge kind {kind:?}")))
        }
    }

    pub fn validate_port_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.port_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("ports/{kind}"), format!("unknown port kind {kind:?}")))
        }
    }

    pub fn validate_wire_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.wire_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("wires/{kind}"), format!("unknown wire kind {kind:?}")))
        }
    }

    pub fn validate_layer_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.layer_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("layers/{kind}"), format!("unknown layer kind {kind:?}")))
        }
    }

    pub fn validate_language_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.language_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("languages/{kind}"), format!("unknown language kind {kind:?}")))
        }
    }

    pub fn validate_node_properties(&self, kind: &str, properties: &PropertyBag) -> Result<(), ManifestValidationError> {
        let Some(def) = self.manifest.node_kind(kind) else {
            return self.validate_node_kind(kind);
        };
        self.validate_property_bag(&format!("nodes/{kind}/properties"), &def.properties, properties)
    }

    pub fn validate_edge_properties(&self, kind: &str, properties: &PropertyBag) -> Result<(), ManifestValidationError> {
        let Some(def) = self.manifest.edge_kind(kind) else {
            return self.validate_edge_kind(kind);
        };
        self.validate_property_bag(&format!("edges/{kind}/properties"), &def.properties, properties)
    }

    fn validate_property_bag(&self, path: &str, defs: &[PropertyDef], bag: &PropertyBag) -> Result<(), ManifestValidationError> {
        for def in defs {
            if def.kind == PropertyKind::Derived {
                continue;
            }
            let Some(value) = bag.get(&def.name) else {
                continue;
            };
            if !property_value_matches_type(value, &def.value_type) {
                return Err(ManifestValidationError::new(format!("{path}/{}", def.name), format!("property type mismatch for {}", def.value_type.id())));
            }
        }
        for key in bag.keys() {
            if !defs.iter().any(|d| d.name == *key) {
                return Err(ManifestValidationError::new(format!("{path}/{key}"), format!("unknown property {key:?}")));
            }
        }
        Ok(())
    }

    pub fn validate_trinity_graph(&self, nodes: &[TrinityNodeRef<'_>], edges: &[TrinityEdgeRef<'_>]) -> Result<(), ManifestValidationError> {
        for node in nodes {
            self.validate_node_kind(node.kind)?;
            self.validate_node_properties(node.kind, node.properties)?;
            for port in node.ports {
                self.validate_port_kind(port.kind)?;
                if let Some(node_def) = self.manifest.node_kind(node.kind) {
                    if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == port.kind) {
                        return Err(ManifestValidationError::new(format!("nodes/{}/ports/{}", node.id, port.kind), format!("port kind {} not declared on node kind {}", port.kind, node.kind)));
                    }
                }
            }
        }
        for edge in edges {
            self.validate_edge_kind(edge.kind)?;
            self.validate_edge_properties(edge.kind, edge.properties)?;
        }
        Ok(())
    }
}

fn property_value_matches_type(value: &PropertyValue, expected: &ValueType) -> bool {
    if matches!(expected, ValueType::Any) {
        return true;
    }
    match value {
        PropertyValue::Object(_) if matches!(expected, ValueType::Schema(_)) => true,
        _ => {
            let neural = property_value_to_neural(value);
            expected.matches(&neural)
        }
    }
}

fn property_value_to_neural(value: &PropertyValue) -> Value {
    match value {
        PropertyValue::Null => Value::null(),
        PropertyValue::Bool(b) => Value::Atom(neural_engine::Atom::Boolean(*b)),
        PropertyValue::Number(n) => Value::Atom(neural_engine::Atom::Decimal(*n)),
        PropertyValue::String(s) => Value::Atom(neural_engine::Atom::String(s.clone())),
        PropertyValue::Array(_) | PropertyValue::Object(_) => Value::Atom(neural_engine::Atom::Null),
    }
}

/// 🔌️ Trinity node reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityNodeRef<'a> {
    pub id: &'a str,
    pub kind: &'a str,
    pub properties: &'a PropertyBag,
    pub ports: &'a [TrinityPortRef<'a>],
}

/// 🔌️ Trinity port reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityPortRef<'a> {
    pub kind: &'a str,
}

/// 🔗️ Trinity edge reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityEdgeRef<'a> {
    pub kind: &'a str,
    pub properties: &'a PropertyBag,
}

// #endregion 🔖️Validator

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
````

## ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs

Bytes 89591; SHA-256 64552f21a7ed7f1f9fdf654646874a19007b4350f62586c5eb16c1148737f484.

````text
//! 🌐️ First-party Semio geometry session with explicit instance ownership.
use semio_framework_os_flow::mesh::*;
use neural_engine::{Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType, VALUE_TYPE_GEOMETRY, VALUE_TYPE_NUMBER, VALUE_TYPE_POINT, VALUE_TYPE_VECTOR};
use semio_framework_3d::brep::engine::{Brep, BrepKernel, GeometryHandle, GeometryKind, ParamDomain, PointClassification, Vec3};
use semio_framework_3d::brep::queries::tessellation::{TessellationJob, TessellationStep};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use semio_framework_3d::brep::engine::retirement::{PayloadRetirement, NativeRetirementStep, RetirementFrontier};
use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex, RwLock};

// 🔀️ dedyn-fw-os-misc, O1/R11 case 3: `BrepKernel` has exactly one impl (`Brep`, in `🗄️stdio`) —
// every `dyn BrepKernel` call site was already handing this module a concrete `Brep`, so the trait
// object was a no-op coercion, not a real seam. Deleting it also clears an existing O1 violation:
// every `BrepKernel` method is `async fn`, which is not dyn-compatible (E0038) — `dyn BrepKernel`
// could not have compiled as-is.





// #region 🔖️Helpers

/// 🔓️ Read-only kernel access — lets concurrent queries (tessellate, volume, closest-point, …)
/// proceed in parallel with each other while still serializing against mutating operations.

pub fn kind_label(kind: GeometryKind) -> &'static str {
    match kind {
        GeometryKind::Vertex => "vertex",
        GeometryKind::Edge => "edge",
        GeometryKind::Wire => "wire",
        GeometryKind::Face => "face",
        GeometryKind::Shell => "shell",
        GeometryKind::Solid => "solid",
        GeometryKind::Compound => "compound",
        GeometryKind::Curve => "curve",
        GeometryKind::Surface => "surface",
    }
}

pub fn geometry_dict(kernel: &Brep, handle: &GeometryHandle) -> Result<Dictionary, EvalError> {
    let kind = kernel.kind(handle).map_err(|error| map_kernel_error(&error))?;
    Ok(Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))).insert("kind", Value::Atom(Atom::String(kind_label(kind).into()))))
}

pub fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

pub fn point_dictionary(point: Vec3) -> Dictionary {
    Dictionary::with_schema("point").insert("x", Value::Atom(Atom::Decimal(point[0]))).insert("y", Value::Atom(Atom::Decimal(point[1]))).insert("z", Value::Atom(Atom::Decimal(point[2])))
}

/// 🧭️ A three-axis input. `read_xyz` only asks for `x`/`y`/`z`, so a `point` satisfies a `vector`
/// channel and the declaration says so — the accepted set, not one nominal type, is what the
/// port-compatibility oracle intersects.
pub fn vector_channel(id: &str, operator_id: &str, default: Vec3) -> ChannelSpec {
    ChannelSpec::requires(id, &["math.vector", operator_id]).with_value_types(&[VALUE_TYPE_VECTOR, VALUE_TYPE_POINT]).with_default(Value::Dictionary(vector_dictionary(default)))
}

pub fn vector_dictionary(vector: Vec3) -> Dictionary {
    Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(vector[0]))).insert("y", Value::Atom(Atom::Decimal(vector[1]))).insert("z", Value::Atom(Atom::Decimal(vector[2])))
}

pub fn text_dictionary(value: impl Into<String>) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value.into())))
}

pub fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

pub fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

pub fn read_geometry(input: &Dictionary, key: &str) -> Result<GeometryHandle, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let handle = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).ok_or_else(|| EvalError::MissingInput(format!("{key}.handle")))?;
    Ok(GeometryHandle(handle.to_string()))
}

pub fn read_optional_geometry(input: &Dictionary, key: &str) -> Option<GeometryHandle> {
    input.get(key).and_then(|value| value.as_dictionary()).and_then(|dict| dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|handle| GeometryHandle(handle.to_string())))
}

/// 🚫️ Requires all three axes present and numeric — a missing/malformed `x`/`y`/`z` is a real
/// caller error, never a silent `0.0` (audit §13.2: silent defaults hide bad input as valid
/// geometry). `label` names the offending field in the error.
pub fn read_xyz_dict(dict: &Dictionary, label: &str) -> Result<Vec3, EvalError> {
    let axis = |name: &str| dict.get(name).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(format!("{label}.{name}")));
    Ok([axis("x")?, axis("y")?, axis("z")?])
}

pub fn read_xyz(input: &Dictionary, key: &str) -> Result<Vec3, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    read_xyz_dict(dict, key)
}

pub fn read_list(input: &Dictionary, key: &str) -> Result<Dictionary, EvalError> {
    input.get(key).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("list")).cloned().ok_or_else(|| EvalError::MissingInput(key.into()))
}

pub fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

pub fn read_point_list(input: &Dictionary, key: &str) -> Result<Vec<Vec3>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let dict = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be a point")))?;
            read_xyz_dict(dict, &format!("{key}[{index}]"))
        })
        .collect()
}

pub fn read_geometry_list(input: &Dictionary, key: &str) -> Result<Vec<GeometryHandle>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let dict = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be geometry")))?;
            dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|handle| GeometryHandle(handle.to_string())).ok_or_else(|| EvalError::MissingInput(format!("{key}[{index}].handle")))
        })
        .collect()
}

/// 🕳️ Like [`read_geometry_list`] but treats a genuinely ABSENT `key` as an empty list — for
/// optional list inputs only. A present-but-malformed value (wrong schema, a non-geometry entry)
/// still propagates its `EvalError` instead of silently becoming empty, unlike a bare
/// `.unwrap_or_default()` on the strict reader would (audit §13.2).
pub fn read_geometry_list_or_empty(input: &Dictionary, key: &str) -> Result<Vec<GeometryHandle>, EvalError> {
    if input.get(key).is_none() {
        return Ok(Vec::new());
    }
    read_geometry_list(input, key)
}

pub fn read_nested_point_lists(input: &Dictionary, key: &str) -> Result<Vec<Vec<Vec3>>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let sub = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("list")).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be a point list")))?;
            list_indices(sub)
                .into_iter()
                .map(|sub_index| {
                    let dict = sub.get(&sub_index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}][{sub_index}] must be a point")))?;
                    read_xyz_dict(dict, &format!("{key}[{index}][{sub_index}]"))
                })
                .collect()
        })
        .collect()
}

pub fn points_to_grid(points: &[Vec3], rows: usize) -> Result<Vec<Vec<Vec3>>, EvalError> {
    if rows == 0 {
        return Err(EvalError::InvalidInput("rows must be positive".into()));
    }
    if !points.len().is_multiple_of(rows) {
        return Err(EvalError::InvalidInput("points length must divide evenly by rows".into()));
    }
    let cols = points.len() / rows;
    Ok((0..rows).map(|row| (0..cols).map(|col| points[row * cols + col]).collect()).collect())
}

pub fn wire_from_points(kernel: &mut Brep, points: &[Vec3]) -> Result<GeometryHandle, EvalError> {
    if points.len() >= 2 {
        kernel.polyline_wire(points).map_err(|error| map_kernel_error(&error))
    } else if let Some(point) = points.first() {
        kernel.vertex(*point).map_err(|error| map_kernel_error(&error))
    } else {
        Err(EvalError::InvalidInput("no intersection".into()))
    }
}

pub fn domain_span(domain: ParamDomain) -> f64 {
    domain.max - domain.min
}

pub fn classify_number(classification: PointClassification) -> f64 {
    match classification {
        PointClassification::Inside => 0.0,
        PointClassification::Outside => 1.0,
        PointClassification::OnBoundary => 2.0,
    }
}



pub fn map_kernel_error(error: &semio_framework_3d::brep::engine::BrepError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

pub fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

/// 🔷️ A brep kernel handle input — `read_geometry` reads `handle` off a `geometry` dictionary, so
/// every wire, face, surface, solid and compound is the SAME port type.
pub fn geometry_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id]).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn list_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::list(id, &[operator_id])
}

/// 📍️ A three-axis input read through `read_xyz` — see [`vector_channel`] for why both schemas count.
pub fn point_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id]).with_value_types(&[VALUE_TYPE_POINT, VALUE_TYPE_VECTOR])
}

pub fn out_solid(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Sld", "solid", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_wire(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("W", "Wre", "wire", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_curve(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("C", "Crv", "curve", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_face(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("F", "Fce", "face", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

/// 📤️ The `face` an operator MAKES, for the operators that are also GIVEN a `face` — one operator's
/// input ids and output ids are disjoint, because `"{nodeId}@{portId}"` is the only public name a
/// wire endpoint has.
///
/// @see `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` — `produced_channel_id`
pub fn out_face_result(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("F", "Fce", neural_engine::produced_channel_id("face"), full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_surface(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Srf", "surface", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_geometry(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("G", "Geo", "geometry", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

/// 📤️ The `geometry` an operator MAKES — see [`out_face_result`].
pub fn out_geometry_result(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("G", "Geo", neural_engine::produced_channel_id("geometry"), full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_compound(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("C", "Cmp", "compound", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_point(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("P", "Pnt", "point", full_name).with_value_types(&[VALUE_TYPE_POINT])
}

/// 📤️ The `point` an operator FINDS, for the operators that are also GIVEN a `point` — see [`out_face_result`].
pub fn out_point_result(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("P", "Pnt", neural_engine::produced_channel_id("point"), full_name).with_value_types(&[VALUE_TYPE_POINT])
}

pub fn out_normal(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("N", "Nrm", "normal", full_name).with_value_types(&[VALUE_TYPE_VECTOR])
}

pub fn out_span() -> ChannelSpec {
    ChannelSpec::named("S", "Spn", "span", "DomainSpan").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_curvature() -> ChannelSpec {
    ChannelSpec::named("K", "Cur", "curvature", "CurveCurvature").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_volume() -> ChannelSpec {
    ChannelSpec::named("V", "Vol", "volume", "MeasuredVolume").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_area() -> ChannelSpec {
    ChannelSpec::named("A", "Are", "area", "MeasuredArea").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_length() -> ChannelSpec {
    ChannelSpec::named("L", "Len", "length", "MeasuredLength").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_center() -> ChannelSpec {
    ChannelSpec::named("P", "CoM", "center", "CenterOfMass").with_value_types(&["point"])
}

pub fn out_box() -> ChannelSpec {
    ChannelSpec::named("B", "Box", "box", "BoundingBox").with_value_types(&["geometry"])
}

pub fn out_distance() -> ChannelSpec {
    ChannelSpec::named("D", "Dst", "distance", "MeasuredDistance").with_value_types(&["number"])
}

pub fn out_classification() -> ChannelSpec {
    ChannelSpec::named("C", "Cls", "classification", "PointClassification").with_value_types(&["number"])
}

pub fn out_report() -> ChannelSpec {
    ChannelSpec::named("R", "Rpt", "report", "ValidationReport").with_value_types(&["text"])
}

pub fn out_vertex() -> ChannelSpec {
    ChannelSpec::named("V", "Vtx", "vertex", "Vertex").with_value_types(&["geometry"])
}


pub fn out_stl() -> ChannelSpec {
    ChannelSpec::named("L", "Stl", "stl", "StlExport").with_value_types(&["text"])
}

pub fn out_obj() -> ChannelSpec {
    ChannelSpec::named("O", "Obj", "obj", "ObjExport").with_value_types(&["text"])
}


/// 🪪️ Expands only ambiguous channel shorthand to its full semantic identifier.
fn distinct_channels(mut channels: Vec<ChannelSpec>) -> Vec<ChannelSpec> {
    let codes: Vec<_> = channels.iter().map(|channel| channel.code.clone()).collect();
    let abbreviations: Vec<_> = channels.iter().map(|channel| channel.abbreviation.clone()).collect();
    for channel in &mut channels {
        if codes.iter().filter(|code| **code == channel.code).count() > 1 { channel.code = channel.name.to_uppercase(); }
        if abbreviations.iter().filter(|abbreviation| **abbreviation == channel.abbreviation).count() > 1 { channel.abbreviation = channel.name.clone(); }
    }
    channels
}

#[allow(
    clippy::too_many_arguments,
    reason = "positional operator-metadata builder mirroring this file's registration table shape (id/name/abbreviation/icon/summary/inputs/outputs/group columns); ~20 call sites, restructuring into a params struct would only churn call sites with no behavior change"
)]
pub fn operator_info_with_outputs(id: &str, name: &str, abbreviation: &str, icon: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>, group: &[&str]) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        extension: "brep".into(),
        name: name.into(),
        abbreviation: abbreviation.into(),
        icon: icon.into(),
        summary: summary.into(),
        inputs: distinct_channels(inputs),
        outputs: distinct_channels(outputs),
        group: group.iter().map(|entry| (*entry).to_string()).collect(),
        ..Default::default()
    }
}

pub fn register_untyped(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operator>, produces: &[&str]) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: operation }], produces);
}

pub fn register_typed(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operator>, produces: &[&str]) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: operation }], produces);
}

#[allow(clippy::too_many_arguments, reason = "positional geometry-operator registration helper; ~68 call sites forming this file's operator table, restructuring into a params struct would only churn call sites with no behavior change")]
pub fn reg_geo(registry: &mut Registry, id: &str, name: &str, abbr: &str, icon: &str, summary: &str, inputs: Vec<ChannelSpec>, output: ChannelSpec, group: &[&str], operation: Box<dyn Operator>) {
    register_untyped(registry, operator_info_with_outputs(id, name, abbr, icon, summary, inputs, vec![output], group), operation, &["geometry"]);
}

pub fn geometry_schema() -> Schema {
    Schema {
        id: "geometry".into(),
        module: "brep".into(),
        name: "Geometry".into(),
        icon: "emoji:🔷️".into(),
        summary: "Opaque brep geometry handle".into(),
        fields: vec![FieldSpec::new("handle", ValueType::Text), FieldSpec::new("kind", ValueType::Text).with_default(Value::Atom(Atom::String("solid".into())))],
    }
}

pub fn empty_list_value() -> Value {
    Value::Dictionary(Dictionary::with_schema("list"))
}

pub fn topology_element_schema(id: &str, name: &str, icon: &str) -> Schema {
    Schema { id: id.into(), module: "brep".into(), name: name.into(), icon: icon.into(), summary: format!("{name} topology element"), fields: vec![FieldSpec::new("handle", ValueType::Text)] }
}

pub fn brep_schema() -> Schema {
    Schema {
        id: "brep".into(),
        module: "brep".into(),
        name: "Brep".into(),
        icon: "emoji:🧊️".into(),
        summary: "Construct, deconstruct, or modify a brep from vertices, edges, and faces".into(),
        fields: vec![
            FieldSpec::new("vertex", ValueType::List(Box::new(ValueType::Schema("vertex".into())))).with_default(empty_list_value()),
            FieldSpec::new("edge", ValueType::List(Box::new(ValueType::Schema("edge".into())))).with_default(empty_list_value()),
            FieldSpec::new("face", ValueType::List(Box::new(ValueType::Schema("face".into())))).with_default(empty_list_value()),
        ],
    }
}

pub fn topology_list(schema: &str, handles: Vec<GeometryHandle>) -> Dictionary {
    handles
        .into_iter()
        .enumerate()
        .fold(Dictionary::with_schema("list"), |list, (index, handle)| list.insert(index.to_string(), Value::Dictionary(Dictionary::with_schema(schema).insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))))))
}

pub struct BrepDeconstruct(pub SessionCapture);

impl Operator for BrepDeconstruct {
    fn retirement_is_empty(&self) -> bool { self.0.terminal_is_empty() }
    fn retire_step(&mut self, items:usize, bytes:usize, _: &mut neural_engine::ValueRetirement) -> Result<neural_engine::ValueRetirementStep,&'static str> {
        self.0.close_step(items,bytes).map_err(|_| "brep.geometry-capture-retirement-failed")
    }
    fn retire_cold(mut self:Box<Self>) { self.0.retire_cold(); }
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let shape = read_geometry(input, "brep")?;
            let topology = kernel.deconstruct(&shape).map_err(|error| map_kernel_error(&error))?;
            Ok(Dictionary::new()
                .insert(neural_engine::produced_channel_id("brep"), Value::Dictionary(geometry_dict(kernel, &shape)?))
                .insert("vertex", Value::Dictionary(topology_list("vertex", topology.vertices)))
                .insert("edge", Value::Dictionary(topology_list("edge", topology.edges)))
                .insert("face", Value::Dictionary(topology_list("face", topology.faces)))
                .insert("errors", Value::Dictionary(Dictionary::with_schema("list"))))
        })
    }
}

pub fn topology_output(code: &str, abbreviation: &str, name: &str, schema: &str) -> ChannelSpec {
    ChannelSpec::named(code, abbreviation, name, name).with_operators(vec![schema.to_string()]).with_value_types(&["list"]).with_cardinality(Cardinality::ZeroOrMore)
}

pub fn text_schema() -> Schema {
    Schema { id: "text".into(), module: "brep".into(), name: "Text".into(), icon: "emoji:📝️".into(), summary: "Text payload".into(), fields: vec![FieldSpec::new("value", ValueType::Text)] }
}

// #endregion 🔖️Helpers

// #region ⚠️ Errors
/// 🧯️ Internal error type for the brep module's media import/export bridging helpers (`export_solid_json`/`import_solid_json` still surface it flattened to JSON `{"error"}` strings, matching prior behaviour byte-for-byte).
#[derive(Debug)]
pub enum BrepModuleError {
    LockPoisoned,
    Kernel(semio_framework_3d::brep::engine::BrepError),
    Codec(EvalError),
    Mesh(String),
    UnsupportedExportFormat(String),
    UnsupportedImportFormat(String),
    InvalidArgs(String),
    UnknownMethod(String),
}

impl std::fmt::Display for BrepModuleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockPoisoned => formatter.write_str("brep kernel lock poisoned"),
            Self::Kernel(error) => std::fmt::Display::fmt(error, formatter),
            Self::Codec(error) => std::fmt::Display::fmt(error, formatter),
            Self::Mesh(detail) => formatter.write_str(detail),
            Self::UnsupportedExportFormat(format) => write!(formatter, "unsupported solid export format: {format}"),
            Self::UnsupportedImportFormat(format) => write!(formatter, "unsupported solid import format: {format}"),
            Self::InvalidArgs(detail) => write!(formatter, "invalid brep_invoke args: {detail}"),
            Self::UnknownMethod(method) => write!(formatter, "unknown brep_invoke method: {method}"),
        }
    }
}

impl std::error::Error for BrepModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Kernel(error) => Some(error),
            Self::Codec(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_3d::brep::engine::BrepError> for BrepModuleError {
    fn from(error: semio_framework_3d::brep::engine::BrepError) -> Self {
        Self::Kernel(error)
    }
}

impl From<EvalError> for BrepModuleError {
    fn from(error: EvalError) -> Self {
        Self::Codec(error)
    }
}
// #endregion ⚠️ Errors

// #region 🔖️Tessellation
/// 🧹️ Retains only geometry handles referenced by the current evaluation outputs.

/// 🩺️ One blocking finding from the pre-tessellation validate gate, in the typed shape the preview
/// status object carries — never a re-parsed prose string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewDiagnostic {
    pub entity: String,
    pub code: String,
    pub message: String,
}

/// ⏱️ What one budgeted [`tessellate_step`] call achieved.
#[derive(Clone, Debug, PartialEq)]
pub enum TessellationStepOutcome {
    /// 🔁 Budget spent, work remains — step again next turn.
    Working { units_done: usize, units_total: usize, faces_done: usize, faces_total: usize, phase: &'static str },
    /// ✅ The mesh is ready (freshly tessellated, or served from the LOD cache).
    Ready { mesh: semio_framework::MeshData, units_total: usize, faces_total: usize },
    /// 🛑 A cancel retired the job; nothing is produced and the slot is free.
    Cancelled,
    /// 🩺️ The validate gate rejected the topology before any triangle was produced.
    Invalid { issues: Vec<PreviewDiagnostic> },
    /// 💥 The kernel refused the handle or a unit faulted.
    Failed { message: String },
}

/// ⏱️ Retained resumable tessellations keyed by `(handle, tolerance bits)`. Bounded: a new job past
/// the ceiling evicts the least recently stepped one rather than growing without limit.
const TESSELLATION_JOB_CAPACITY: usize = 32;

struct RetainedTessellation {
    job: TessellationJob,
    last_step: u64,
}

#[derive(Default)]
struct TessellationJobRegistry {
    jobs: ManuallyDrop<BTreeMap<(String, u64), RetainedTessellation>>,
    clock: u64,
}



/// 🧹️ Drops retained tessellations whose handle is no longer live. An empty `live` clears them all.

/// 🛑️ Retires the in-flight tessellation of `handle` at `tolerance`. Returns true when a job was
/// actually retired — a cancel for an already-finished or never-started job is a no-op, not a fault.

/// 🛑️ Retires every in-flight tessellation — the explicit "cancel preview evaluation" gesture.

/// 📈️ Progress of the in-flight tessellation of `handle` at `tolerance`, if one is retained.

/// 🎚️ The cached mesh for `handle` at `tolerance` OR at any FINER tolerance already computed — a
/// finer mesh is a valid, better-than-requested answer, so switching the LOD mode from fine to
/// coarse serves the existing mesh instead of retessellating.

/// ⏱️ Advances the tessellation of `handle` at `tolerance` by at most `budget` units and reports
/// what happened. This is the ONLY tessellation entry point that respects an interactive step
/// ceiling: it never runs more than `budget` face/edge units before returning, so a host can drive
/// it from inside its own maintenance budget, paint progress, and cancel it.
///
/// The validate gate runs once, on first admission of a `(handle, tolerance)` pair — a solid with a
/// blocking (`code` not prefixed `warning-`) validation issue never reaches the tessellator.

/// 🧊️ Tessellates a geometry handle owned by the in-process brep kernel into preview `MeshData`.
/// Unbudgeted convenience over [`tessellate_step`] for callers with no interactive ceiling (export
/// bridges, schema tests) — the incremental path is still the only algorithm underneath.

/// 🌐️ One budgeted tessellate ROUND TRIP as the extension-boundary JSON envelope: progress/phase
/// always, plus one base64 `pack` mesh-body chunk per continuation once the mesh is ready. `chunk`
/// selects which chunk to ship; `chunks` tells the caller how many there are in total.
///
/// ⏱️ `budget` bounds ONE step in face/edge units — the granularity at which a cancel can land —
/// and `wall_micros` bounds the whole round trip in wall time: the call keeps stepping while the
/// job is still working AND the deadline has not passed. Both bounds are needed because units are
/// not time: the cheapest step measured on this kernel is 29 µs and the dearest 5.9 s, so a unit
/// budget alone either wastes a whole interactive round trip on microseconds of work or overruns
/// any ceiling on a single face (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

// 🚫️async: E1 pure codec helper (no I/O), consumed from sync envelope call sites — see R9
fn failed_envelope_json(code: &str, message: &str) -> String {
    use semio_framework_os_flow::os_pack::json::{object, Value};
    semio_framework_os_flow::os_pack::json::to_string(&object([
        ("done".to_string(), Value::Bool(true)),
        ("cancellable".to_string(), Value::Bool(false)),
        ("phase".to_string(), Value::String("failed".to_string())),
        ("error".to_string(), Value::String(message.to_string())),
        ("errorCode".to_string(), Value::String(code.to_string())),
    ]))
}


/// 🗑️ Disposes a geometry handle owned by the in-process brep kernel.
// #endregion 🔖️Tessellation





// #region 🔖️MediaExport
/// 📤️ Exports geometry handles owned by the in-process brep kernel to a solid/mesh interchange format. STEP/OBJ/STL go through the kernel's native codecs; GLB bridges through tessellation (`tessellate` → merged `MeshData` → `GlbExporter`) since the kernel has no native GLB writer wired here. Binary formats are base64-encoded. Returns `{"data","binary","format"}` or `{"error"}` JSON.

/// 🧊️ Bridges GLB export through mesh tessellation: tessellates every shape, merges the resulting triangle soup into one `MeshData`, and encodes it with the shared `GlbExporter` mesh codec (the same codec every other app uses for GLB).
pub fn export_glb_via_tessellation(kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepModuleError> {
    use semio_framework::MeshExporter;
    let mut merged = semio_framework::MeshData::default();
    for shape in shapes {
        let transfer = kernel.tessellate(shape, deflection)?;
        let mesh = semio_framework_3d::brep::engine::mesh_data_from_mesh_transfer(&transfer);
        let offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(mesh.positions);
        merged.normals.extend(mesh.normals);
        merged.indices.extend(mesh.indices.into_iter().map(|index| index + offset));
    }
    semio_framework::GlbExporter.export(&merged).map_err(BrepModuleError::Mesh)
}

/// 📥️ Imports STEP/OBJ/STL solid data (or GLB mesh data, bridged through the kernel's OBJ ingestion since it has no raw-mesh entry point) into the in-process kernel. STEP/OBJ expect UTF-8 text in `data`; STL/GLB expect base64-encoded bytes. Returns `{"handles":[...]}` or `{"error"}` JSON.

/// 🧊️ Bridges GLB import through the mesh codec: decodes GLB bytes to `MeshData` via `GlbImporter`, re-encodes it as OBJ text, and ingests that through the kernel's own OBJ importer.
pub fn import_glb_via_tessellation(kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<String>, BrepModuleError> {
    use semio_framework::MeshImporter;
    let mesh = semio_framework::GlbImporter.import(bytes).map_err(BrepModuleError::Mesh)?;
    let obj_text = semio_framework::mesh_to_obj(&mesh, "glb-import");
    kernel.import_obj(&obj_text, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)
}
// #endregion 🔖️MediaExport

// #region 🔖️GenericInvoke
/// 🌉️ `brep_invoke` argument/result JSON shape: `{"error": "..."}` on failure, otherwise one of
/// `{"handle": "..."}` / `{"handles": [...]}` / `{"value": ...}` / a raw `MeshTransfer` object /
/// `{"vertices": [...], "edges": [...], "faces": [...], "shells": [...]}` for `deconstruct`.
fn invoke_args(args_json: &str) -> Result<semio_framework_os_flow::os_pack::json::Value, BrepModuleError> {
    semio_framework_os_flow::os_pack::json::parse(args_json).map_err(|error| BrepModuleError::InvalidArgs(error.to_string()))
}

fn arg_f64(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<f64, BrepModuleError> {
    args.get(key).and_then(|value| value.as_f64()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing number {key}")))
}

fn arg_f64_or(args: &semio_framework_os_flow::os_pack::json::Value, key: &str, fallback: f64) -> f64 {
    args.get(key).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

fn arg_usize(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<usize, BrepModuleError> {
    args.get(key).and_then(|value| value.as_u64()).map(|value| value as usize).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing integer {key}")))
}

fn arg_bool_or(args: &semio_framework_os_flow::os_pack::json::Value, key: &str, fallback: bool) -> bool {
    args.get(key).and_then(|value| value.as_bool()).unwrap_or(fallback)
}

fn arg_string(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<String, BrepModuleError> {
    args.get(key).and_then(|value| value.as_str()).map(str::to_string).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing string {key}")))
}

fn value_vec3(value: &semio_framework_os_flow::os_pack::json::Value) -> Result<Vec3, BrepModuleError> {
    let items = value.as_array().ok_or_else(|| BrepModuleError::InvalidArgs("expected a 3-number array".to_string()))?;
    if items.len() != 3 {
        return Err(BrepModuleError::InvalidArgs("expected a 3-number array".to_string()));
    }
    let axis = |index: usize| items[index].as_f64().ok_or_else(|| BrepModuleError::InvalidArgs("expected a 3-number array".to_string()));
    Ok([axis(0)?, axis(1)?, axis(2)?])
}

fn arg_vec3(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<Vec3, BrepModuleError> {
    let value = args.get(key).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point {key}")))?;
    value_vec3(value)
}

fn arg_points(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<Vec<Vec3>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point array {key}")))?;
    items.iter().map(value_vec3).collect()
}

fn arg_handle(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<GeometryHandle, BrepModuleError> {
    arg_string(args, key).map(GeometryHandle)
}

fn arg_handles(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<Vec<GeometryHandle>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing handle array {key}")))?;
    items.iter().map(|item| item.as_str().map(|text| GeometryHandle(text.to_string())).ok_or_else(|| BrepModuleError::InvalidArgs(format!("{key} entries must be strings")))).collect()
}

fn handle_result(handle: GeometryHandle) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([("handle".to_string(), semio_framework_os_flow::os_pack::json::Value::String(handle.0))])
}

fn handles_result(handles: Vec<GeometryHandle>) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([("handles".to_string(), semio_framework_os_flow::os_pack::json::array(handles.into_iter().map(|handle| semio_framework_os_flow::os_pack::json::Value::String(handle.0))))])
}

fn number_result(value: f64) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([("value".to_string(), semio_framework_os_flow::os_pack::json::Value::Number(semio_framework_os_flow::os_pack::json::Number::Float(value)))])
}

fn vec3_result(value: Vec3) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([(
        "value".to_string(),
        semio_framework_os_flow::os_pack::json::array(value.into_iter().map(semio_framework_os_flow::os_pack::json::Number::Float).map(semio_framework_os_flow::os_pack::json::Value::Number)),
    )])
}

fn string_result(value: String) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([("value".to_string(), semio_framework_os_flow::os_pack::json::Value::String(value))])
}

fn unit_result() -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([])
}

fn topology_result(topology: semio_framework_3d::brep::engine::BrepTopology) -> semio_framework_os_flow::os_pack::json::Value {
    let handle_array = |handles: Vec<GeometryHandle>| semio_framework_os_flow::os_pack::json::array(handles.into_iter().map(|handle| semio_framework_os_flow::os_pack::json::Value::String(handle.0)));
    semio_framework_os_flow::os_pack::json::object([
        ("vertices".to_string(), handle_array(topology.vertices)),
        ("edges".to_string(), handle_array(topology.edges)),
        ("faces".to_string(), handle_array(topology.faces)),
        ("shells".to_string(), handle_array(topology.shells)),
    ])
}

fn mesh_result(mesh: &semio_framework_3d::brep::engine::MeshTransfer) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::from_dsl_value(&semio_framework_os_flow::os_dsl::ToValue::to_value(mesh))
}

/// 🌉️ Dispatches one `BrepKernel` method by name over `os_pack::json` args (see `handle_result`
/// and friends above for the response shapes); the sole bridge every `SemioBrepKernel` TS method
/// (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts`) calls into. Every arm is declared, argument for
/// argument, by the verb catalog `🔣️.json` beside this file (schema `🧬️schema/🔣️.json`); the bridge law
/// `🌊️flow/🧪️tests/📐️brep-invoke` holds the two to each other.

/// 🌐️ `brep_invoke` implementation shared by the wasm export and native callers/tests: returns
/// the result JSON or `{"error": "..."}` — never panics on malformed input.
// #endregion 🔖️GenericInvoke

/// 🧾️ Records real heap layouts; cloned handles clear the synchronous consuming-call receipt.
#[derive(Default)]
struct ShellAllocator {
    layout_bytes: std::sync::atomic::AtomicUsize,
    receipt: std::sync::atomic::AtomicPtr<std::sync::atomic::AtomicUsize>,
}
impl Clone for ShellAllocator {
    fn clone(&self) -> Self { Self { layout_bytes:std::sync::atomic::AtomicUsize::new(self.layout_bytes.load(std::sync::atomic::Ordering::Relaxed)),receipt:std::sync::atomic::AtomicPtr::default() } }
}
unsafe impl std::alloc::Allocator for ShellAllocator {
    fn allocate(&self,layout:std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>,std::alloc::AllocError> {
        let allocation = std::alloc::Allocator::allocate(&std::alloc::Global,layout)?;
        self.layout_bytes.store(layout.size(),std::sync::atomic::Ordering::Relaxed);
        Ok(allocation)
    }
    unsafe fn deallocate(&self,pointer:std::ptr::NonNull<u8>,layout:std::alloc::Layout) {
        unsafe { std::alloc::Allocator::deallocate(&std::alloc::Global,pointer,layout); }
        let receipt = self.receipt.load(std::sync::atomic::Ordering::Relaxed);
        if !receipt.is_null() { unsafe { (*receipt).fetch_add(layout.size(),std::sync::atomic::Ordering::Relaxed); } }
    }
}
type ShellArc<T> = Arc<T,ShellAllocator>;
fn shell<T>(value:T) -> ShellArc<T> { Arc::new_in(value,ShellAllocator::default()) }
fn shell_bytes<T>(owner:&ShellArc<T>) -> usize { Arc::allocator(owner).layout_bytes.load(std::sync::atomic::Ordering::Relaxed) }
fn record_shell<T>(owner:&ShellArc<T>,receipt:&std::sync::atomic::AtomicUsize) { Arc::allocator(owner).receipt.store(std::ptr::from_ref(receipt).cast_mut(),std::sync::atomic::Ordering::Relaxed); }

#[derive(Clone)]
pub struct Session { state: ShellArc<SessionState> }

/// 🎟️ Owns one exact authority reader until reference release or real final-family retirement.
#[must_use = "session captures require explicit retirement"]
pub struct SessionCapture { session: ManuallyDrop<Option<Session>>, exclusive: bool }
impl std::ops::Deref for SessionCapture {
    type Target = Session;
    fn deref(&self) -> &Session { self.session.as_ref().expect("open geometry capture") }
}
impl SessionCapture {
    pub fn terminal_is_empty(&self) -> bool { self.session.is_none() }
    /// 🧾️ Returns actual possible allocator-layout bytes for the next shallow shell release.
    pub fn shell_byte_requirement(&self) -> usize {
        let Some(session) = self.session.as_ref() else { return 0 };
        let state = &session.state;
        shell_bytes(state) + if self.exclusive { shell_bytes(&state.kernel) + shell_bytes(&state.mesh_cache) + shell_bytes(&state.claims) + shell_bytes(&state.next_authority) + shell_bytes(&state.retirement) } else { 0 }
    }
    /// 🧹️ Releases a non-final reader atomically; the final reader retains the actual family cursor.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<neural_engine::ValueRetirementStep,String> {
        use neural_engine::ValueRetirementStep as Step;
        if self.terminal_is_empty() { return Ok(Step::Complete); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        if !self.exclusive {
            if maximum_bytes < self.shell_byte_requirement() { return Ok(Step::Blocked); }
            let receipt = std::sync::atomic::AtomicUsize::new(0);
            let Session { state } = self.session.take().expect("owned geometry capture");
            record_shell(&state,&receipt);
            match Arc::into_inner(state) {
                None => {},
                Some(state) => { *self.session = Some(Session { state:shell(state) }); self.exclusive = true; }
            }
            return Ok(Step::Pending { released_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed) });
        }
        let session = self.session.as_ref().expect("exclusive geometry capture");
        if session.terminal_is_empty() {
            if maximum_bytes < self.shell_byte_requirement() { return Ok(Step::Blocked); }
            let receipt = std::sync::atomic::AtomicUsize::new(0);
            let Session { state } = self.session.take().expect("terminal geometry capture");
            record_shell(&state,&receipt);
            if let Some(state) = Arc::into_inner(state) {
                record_shell(&state.kernel,&receipt); record_shell(&state.mesh_cache,&receipt);
                record_shell(&state.claims,&receipt); record_shell(&state.next_authority,&receipt); record_shell(&state.retirement,&receipt);
                drop(state);
            }
            return Ok(Step::Pending { released_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed) });
        }
        Ok(match session.close_step(maximum_items,maximum_bytes)? {
            Step::Complete => Step::Pending { released_items:0,released_bytes:0 },
            step => step,
        })
    }
    /// 🧊️ Drains a capture at an explicit cold boundary; retained registries supply individual grants.
    pub fn retire_cold(&mut self) {
        while !self.terminal_is_empty() {
            let step = self.close_step(1,4096.max(self.shell_byte_requirement())).expect("cold geometry capture retirement");
            assert!(!matches!(step,neural_engine::ValueRetirementStep::Blocked),"cold geometry capture is paused");
            if matches!(step,neural_engine::ValueRetirementStep::Pending { released_items:0,released_bytes:0 }) {
                assert!(self.session.as_ref().is_some_and(Session::terminal_is_empty),"cold geometry capture waits on a live family authority");
            }
        }
    }
}
impl Drop for SessionCapture {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(),"geometry capture requires explicit retirement before drop"); } }
}

/// 🔌️ Owner-supplied geometry operations; the retained session owns all authority and retirement.
pub trait GeometryOperations: Send + Sync {
    fn export(&self, _: &Brep, format: &str, _: &[GeometryHandle], _: f64) -> Result<(String,bool),BrepModuleError> { Err(BrepModuleError::UnsupportedExportFormat(format.into())) }
    fn import(&self, _: &mut Brep, format: &str, _: &str, _: f64) -> Result<Vec<GeometryHandle>,BrepModuleError> { Err(BrepModuleError::UnsupportedImportFormat(format.into())) }
    fn invoke(&self, _: &mut Brep, method: &str, _: &semio_framework_os_flow::os_pack::json::Value) -> Result<semio_framework_os_flow::os_pack::json::Value,BrepModuleError> { Err(BrepModuleError::UnknownMethod(method.into())) }
}
impl GeometryOperations for () {}

struct SessionState {
    operations: &'static dyn GeometryOperations,
    kernel: ShellArc<RwLock<ManuallyDrop<Brep>>>,
    mesh_cache: ShellArc<Mutex<ManuallyDrop<BTreeMap<(String, u64), semio_framework::MeshData>>>>,
    claims: ShellArc<Mutex<BTreeMap<u64, BTreeSet<String>>>>,
    next_authority: ShellArc<std::sync::atomic::AtomicU64>,
    authority: u64,
    closed: std::sync::atomic::AtomicBool,
    jobs: Mutex<TessellationJobRegistry>,
    retirement: ShellArc<Mutex<SessionRetirement>>,
}
#[derive(Default)]
struct SessionRetirement { payloads:PayloadRetirement, extracted:bool, paused:bool }
struct RetiredClaims(BTreeSet<String>);
impl RetirementFrontier for RetiredClaims {
    fn advance(&mut self,payloads:&mut PayloadRetirement) -> bool { if let Some(handle) = self.0.pop_first() { payloads.text(handle); } self.0.is_empty() }
}
struct RetiredJobs(BTreeMap<(String,u64),RetainedTessellation>);
impl RetirementFrontier for RetiredJobs {
    fn advance(&mut self,payloads:&mut PayloadRetirement) -> bool {
        if let Some(((handle,_),retained)) = self.0.pop_first() { payloads.text(handle); retained.job.detach_retirement(payloads); }
        self.0.is_empty()
    }
}
struct RetiredMeshes(BTreeMap<(String,u64),semio_framework::MeshData>);
impl RetirementFrontier for RetiredMeshes {
    fn advance(&mut self,payloads:&mut PayloadRetirement) -> bool {
        if let Some(((handle,_),mesh)) = self.0.pop_first() {
            payloads.text(handle); payloads.pod(mesh.positions); payloads.pod(mesh.normals); payloads.pod(mesh.colors); payloads.pod(mesh.indices); payloads.pod(mesh.uvs); payloads.pod(mesh.face_ids); payloads.pod(mesh.vertex_ids); payloads.pod(mesh.edge_positions); payloads.pod(mesh.edge_ids); payloads.pod(mesh.edge_uvs); payloads.pod(mesh.edge_is_seam);
            if let Some(texture) = mesh.paint_texture_base64 { payloads.text(texture); }
        }
        self.0.is_empty()
    }
}
impl Drop for SessionState {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.closed.load(std::sync::atomic::Ordering::Acquire),"geometry authority requires explicit close before drop");
            assert!(self.jobs.get_mut().expect("geometry jobs").jobs.is_empty(),"geometry jobs require explicit retirement transfer");
        }
    }
}
impl Drop for SessionRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.extracted && self.payloads.terminal_is_empty(),"geometry family requires terminal-empty retirement before drop"); } }
}
impl Default for Session { fn default() -> Self { Self::new() } }
impl Session {
    /// 🔗️ Captures this exact authority for a supplied operator or retained composition reader.
    pub fn capture(&self) -> SessionCapture { SessionCapture { session:ManuallyDrop::new(Some(self.clone())),exclusive:false } }
    /// 🔌️ Supplies one separately admitted retained-job authority over this kernel family.
    pub fn port(&self) -> Box<dyn semio_framework_os_flow::geometry::GeometryPort> {
        let mut claims = self.state.claims.lock().expect("geometry claims");
        let authority = self.state.next_authority.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let closed = self.is_closed();
        if !closed { claims.insert(authority,BTreeSet::new()); }
        let authority = Session { state: shell(SessionState {
            operations:self.state.operations, kernel: self.state.kernel.clone(), mesh_cache: self.state.mesh_cache.clone(),
            claims: self.state.claims.clone(), next_authority: self.state.next_authority.clone(), authority,
            closed: std::sync::atomic::AtomicBool::new(closed), jobs: Mutex::new(TessellationJobRegistry::default()), retirement:self.state.retirement.clone(),
        }) };
        Box::new(SessionPort { authority:ManuallyDrop::new(Some(authority)),anchor:self.capture() })
    }
    /// 🚪️ Seals this authority and transfers its claims and jobs to persistent retirement.
    pub fn begin_close(&self) {
        let mut claims = self.state.claims.lock().expect("geometry claims");
        if self.state.closed.swap(true, std::sync::atomic::Ordering::AcqRel) { return; }
        let own_claims = claims.remove(&self.state.authority).unwrap_or_default();
        let jobs = std::mem::take(&mut *self.state.jobs.lock().expect("geometry jobs").jobs);
        let mut retirement = self.state.retirement.lock().expect("geometry retirement");
        retirement.payloads.frontier(RetiredClaims(own_claims));
        retirement.payloads.frontier(RetiredJobs(jobs));
    }
    /// 🛑️ Pauses retirement without reopening authority or dropping owned resources.
    pub fn cancel_close(&self) { self.state.retirement.lock().expect("geometry retirement").paused = true; }
    /// ▶️ Resumes the retained close cursor after cancellation.
    pub fn resume_close(&self) { self.state.retirement.lock().expect("geometry retirement").paused = false; }
    /// 🎟️ Drains one structural frontier and at most the supplied allocation-byte credit.
    pub fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> Result<neural_engine::ValueRetirementStep,String> {
        use neural_engine::ValueRetirementStep as Step;
        {
            let retirement = self.state.retirement.lock().map_err(|_| "geometry retirement lock poisoned")?;
            if retirement.extracted && retirement.payloads.terminal_is_empty() { return Ok(Step::Complete); }
            if maximum_items == 0 || maximum_bytes == 0 || retirement.paused { return Ok(Step::Blocked); }
        }
        self.begin_close();
        let claims = self.state.claims.lock().map_err(|_| "geometry claims lock poisoned")?;
        let mut retirement = self.state.retirement.lock().map_err(|_| "geometry retirement lock poisoned")?;
        if retirement.extracted && retirement.payloads.terminal_is_empty() { return Ok(Step::Complete); }
        if maximum_items == 0 || maximum_bytes == 0 || retirement.paused { return Ok(Step::Blocked); }
        if !retirement.extracted && claims.is_empty() {
            let Ok(mut kernel) = self.state.kernel.try_write() else { return Ok(Step::Pending { released_items:0,released_bytes:0 }) };
            let Ok(mut cache) = self.state.mesh_cache.try_lock() else { return Ok(Step::Pending { released_items:0,released_bytes:0 }) };
            kernel.detach_retirement(&mut retirement.payloads);
            retirement.payloads.frontier(RetiredMeshes(std::mem::take(&mut **cache)));
            retirement.extracted = true;
            return Ok(Step::Pending { released_items:1,released_bytes:0 });
        }
        Ok(match retirement.payloads.close_step(maximum_items,maximum_bytes) {
            NativeRetirementStep::Blocked => Step::Blocked,
            NativeRetirementStep::Pending { released_items,released_bytes } => Step::Pending { released_items,released_bytes },
            NativeRetirementStep::Complete => Step::Pending { released_items:0,released_bytes:0 },
        })
    }
    pub fn terminal_is_empty(&self) -> bool {
        let retirement = self.state.retirement.lock().expect("geometry retirement");
        self.is_closed() && retirement.extracted && retirement.payloads.terminal_is_empty()
    }
    /// 🧊️ Synchronous authority release for cold callers; retained owners use close_step.
    pub fn close(&self) {
        self.begin_close();
        loop {
            match self.close_step(1,4096) {
                Ok(neural_engine::ValueRetirementStep::Pending { released_items:0,released_bytes:0 }) | Ok(neural_engine::ValueRetirementStep::Complete) | Ok(neural_engine::ValueRetirementStep::Blocked) | Err(_) => return,
                Ok(neural_engine::ValueRetirementStep::Pending { .. }) => {},
            }
        }
    }
    pub fn is_closed(&self) -> bool { self.state.closed.load(std::sync::atomic::Ordering::Acquire) }
    pub fn new() -> Self { Self::with_operations(&()) }
    /// 🔌️ Admits an explicit static owner operation table without retaining extra payload state.
    pub fn with_operations(operations: &'static dyn GeometryOperations) -> Self {
        Self { state: shell(SessionState { operations, kernel: shell(RwLock::new(ManuallyDrop::new(Brep::new()))), mesh_cache: shell(Mutex::new(ManuallyDrop::new(BTreeMap::new()))), claims: shell(Mutex::new(BTreeMap::from([(1,BTreeSet::new())]))), next_authority: shell(std::sync::atomic::AtomicU64::new(2)), authority:1,closed:std::sync::atomic::AtomicBool::new(false),jobs:Mutex::new(TessellationJobRegistry::default()),retirement:shell(Mutex::new(SessionRetirement::default())) }) }
    }
fn kernel(&self) -> &RwLock<ManuallyDrop<Brep>> { &self.state.kernel }
fn mesh_cache(&self) -> &Mutex<ManuallyDrop<BTreeMap<(String, u64), semio_framework::MeshData>>> { &self.state.mesh_cache }
pub fn evict_mesh_cache_for_handles(&self, handles: &[String]) {
    if self.is_closed() { return; }
    if handles.is_empty() {
        if let Ok(mut cache) = self.mesh_cache().lock() {
            cache.clear();
        }
        return;
    }
    let live: HashSet<&str> = handles.iter().map(String::as_str).collect();
    if let Ok(mut cache) = self.mesh_cache().lock() {
        cache.retain(|(handle, _), _| live.contains(handle.as_str()));
    }
}
pub fn evict_mesh_cache_for_handle(&self, handle: &str) {
    if self.is_closed() { return; }
    if let Ok(mut cache) = self.mesh_cache().lock() {
        cache.retain(|(cached_handle, _), _| cached_handle != handle);
    }
}
pub fn with_kernel<T>(&self, f: impl FnOnce(&mut Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut claims = self.state.claims.lock().map_err(|_| EvalError::InvalidInput("geometry claims lock poisoned".into()))?;
    let mut guard = self.kernel().write().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    if self.is_closed() { return Err(EvalError::InvalidInput("geometry.session-closed".into())); }
    let before = guard.live_handles();
    let result = f(&mut guard);
    claims.entry(self.state.authority).or_default().extend(guard.live_handles().difference(&before).cloned());
    result
}
pub fn with_kernel_read<T>(&self, f: impl FnOnce(&Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let guard = self.kernel().read().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    if self.is_closed() { return Err(EvalError::InvalidInput("geometry.session-closed".into())); }
    f(&guard)
}
pub fn retain_geometry_handles(&self, live: &[String]) {
    let mut claims = self.state.claims.lock().expect("geometry claims");
    if self.is_closed() { return; }
    claims.insert(self.state.authority, live.iter().cloned().collect());
    let merged = claims.values().flat_map(|handles| handles.iter().cloned()).collect::<HashSet<_>>();
    if let Ok(mut guard) = self.kernel().write() { guard.retain(&merged); }
    self.evict_mesh_cache_for_handles(&merged.into_iter().collect::<Vec<_>>());
    self.retain_tessellation_jobs(live);
}
fn tessellation_jobs(&self) -> &Mutex<TessellationJobRegistry> { &self.state.jobs }
pub fn retain_tessellation_jobs(&self, live: &[String]) {
    if self.is_closed() { return; }
    let Ok(mut registry) = self.tessellation_jobs().lock() else { return };
    if live.is_empty() {
        registry.jobs.clear();
        return;
    }
    let live_set: HashSet<&str> = live.iter().map(String::as_str).collect();
    registry.jobs.retain(|(handle, _), _| live_set.contains(handle.as_str()));
}
pub fn cancel_tessellation(&self, handle: &str, tolerance: f64) -> bool {
    let Ok(mut registry) = self.tessellation_jobs().lock() else { return false };
    match registry.jobs.remove(&(handle.to_string(), tolerance.to_bits())) {
        Some(mut retained) => {
            retained.job.cancel();
            true
        }
        None => false,
    }
}
pub fn cancel_all_tessellations(&self) -> usize {
    let Ok(mut registry) = self.tessellation_jobs().lock() else { return 0 };
    let count = registry.jobs.len();
    for (_, retained) in registry.jobs.iter_mut() {
        retained.job.cancel();
    }
    registry.jobs.clear();
    count
}
pub fn tessellation_progress(&self, handle: &str, tolerance: f64) -> Option<(usize, usize, &'static str)> {
    let registry = self.tessellation_jobs().lock().ok()?;
    let retained = registry.jobs.get(&(handle.to_string(), tolerance.to_bits()))?;
    let progress = retained.job.progress();
    Some((progress.units_done, progress.units_total, progress.phase.tag()))
}
pub fn cached_mesh_at_or_finer(&self, handle: &str, tolerance: f64) -> Option<semio_framework::MeshData> {
    if self.is_closed() { return None; }
    let cache = self.mesh_cache().lock().ok()?;
    if let Some(exact) = cache.get(&(handle.to_string(), tolerance.to_bits())) {
        return Some(exact.clone());
    }
    let mut best: Option<(f64, &semio_framework::MeshData)> = None;
    for ((cached_handle, bits), mesh) in cache.iter() {
        if cached_handle != handle {
            continue;
        }
        let cached_tolerance = f64::from_bits(*bits);
        if cached_tolerance > tolerance {
            continue;
        }
        if best.is_none_or(|(current, _)| cached_tolerance > current) {
            best = Some((cached_tolerance, mesh));
        }
    }
    best.map(|(_, mesh)| mesh.clone())
}
pub fn tessellate_step(&self, handle: &str, tolerance: f64, budget: usize) -> TessellationStepOutcome {
    if self.is_closed() { return TessellationStepOutcome::Failed { message: "geometry.session-closed".into() }; }
    let mut claims = self.state.claims.lock().expect("geometry claims");
    if self.is_closed() { return TessellationStepOutcome::Failed { message: "geometry.session-closed".into() }; }
    claims.entry(self.state.authority).or_default().insert(handle.into());
    if let Some(mesh) = self.cached_mesh_at_or_finer(handle, tolerance) {
        return TessellationStepOutcome::Ready { units_total: 0, faces_total: 0, mesh };
    }
    let key = (handle.to_string(), tolerance.to_bits());
    let Ok(guard) = self.kernel().read() else {
        return TessellationStepOutcome::Failed { message: "brep kernel lock poisoned".to_string() };
    };
    let geometry = GeometryHandle(handle.to_string());
    let Ok(mut registry) = self.tessellation_jobs().lock() else {
        return TessellationStepOutcome::Failed { message: "tessellation job registry lock poisoned".to_string() };
    };
    if !registry.jobs.contains_key(&key) {
        if let Err(issues) = guard.validate_gate_sync(&geometry) {
            return TessellationStepOutcome::Invalid { issues: issues.into_iter().map(|issue| PreviewDiagnostic { entity: issue.entity, code: issue.code.to_string(), message: issue.message }).collect() };
        }
        let job = match guard.tessellate_job_sync(&geometry, tolerance) {
            Ok(job) => job,
            Err(error) => return TessellationStepOutcome::Failed { message: error.to_string() },
        };
        if registry.jobs.len() >= TESSELLATION_JOB_CAPACITY {
            if let Some(oldest) = registry.jobs.iter().min_by_key(|(_, retained)| retained.last_step).map(|(key, _)| key.clone()) {
                registry.jobs.remove(&oldest);
            }
        }
        let clock = registry.clock;
        registry.jobs.insert(key.clone(), RetainedTessellation { job, last_step: clock });
    }
    registry.clock += 1;
    let clock = registry.clock;
    let Some(retained) = registry.jobs.get_mut(&key) else {
        return TessellationStepOutcome::Failed { message: "tessellation job vanished between admission and step".to_string() };
    };
    retained.last_step = clock;
    let step = retained.job.step(guard.tessellation_body(), budget);
    match step {
        Err(error) => {
            registry.jobs.remove(&key);
            TessellationStepOutcome::Failed { message: error.to_string() }
        }
        Ok(TessellationStep::Working(progress)) => TessellationStepOutcome::Working { units_done: progress.units_done, units_total: progress.units_total, faces_done: progress.faces_done, faces_total: progress.faces_total, phase: progress.phase.tag() },
        Ok(TessellationStep::Cancelled(_)) => {
            registry.jobs.remove(&key);
            TessellationStepOutcome::Cancelled
        }
        Ok(TessellationStep::Done(progress)) => {
            let Some(retained) = registry.jobs.remove(&key) else {
                return TessellationStepOutcome::Failed { message: "finished tessellation job vanished".to_string() };
            };
            let Some((transfer, _report)) = retained.job.into_mesh() else {
                return TessellationStepOutcome::Failed { message: "finished tessellation job produced no mesh".to_string() };
            };
            let mesh = semio_framework_3d::brep::engine::mesh_data_from_mesh_transfer(&transfer);
            if let Ok(mut cache) = self.mesh_cache().lock() {
                cache.insert(key, mesh.clone());
            }
            TessellationStepOutcome::Ready { mesh, units_total: progress.units_total, faces_total: progress.faces_total }
        }
    }
}
pub fn tessellate_geometry(&self, handle: &str, tolerance: f64) -> Result<semio_framework::MeshData, String> {
    loop {
        match self.tessellate_step(handle, tolerance, usize::MAX) {
            TessellationStepOutcome::Ready { mesh, .. } => return Ok(mesh),
            TessellationStepOutcome::Working { .. } => continue,
            TessellationStepOutcome::Cancelled => return Err("tessellation cancelled".to_string()),
            TessellationStepOutcome::Failed { message } => return Err(message),
            TessellationStepOutcome::Invalid { issues } => {
                let joined = issues.iter().map(|issue| format!("[{}] {}: {}", issue.code, issue.entity, issue.message)).collect::<Vec<_>>().join("; ");
                return Err(format!("validation rejected the solid before tessellation: {joined}"));
            }
        }
    }
}
pub fn tessellate_step_envelope_json(&self, handle: &str, tolerance: f64, budget: usize, wall_micros: u64, chunk: usize) -> String {
    use semio_framework_os_flow::os_pack::json::{object, Value};
    let deadline = semio_framework_job::default_now_us().map(|now| now.saturating_add(wall_micros));
    let mut outcome = self.tessellate_step(handle, tolerance, budget);
    while matches!(outcome, TessellationStepOutcome::Working { .. }) {
        let Some(deadline) = deadline else { break };
        if semio_framework_job::default_now_us().is_none_or(|now| now >= deadline) {
            break;
        }
        outcome = self.tessellate_step(handle, tolerance, budget);
    }
    let envelope = match outcome {
        TessellationStepOutcome::Working { units_done, units_total, faces_done, faces_total, phase } => object([
            ("done".to_string(), Value::Bool(false)),
            ("cancellable".to_string(), Value::Bool(true)),
            ("phase".to_string(), Value::String(phase.to_string())),
            ("unitsDone".to_string(), Value::from(units_done as u64)),
            ("unitsTotal".to_string(), Value::from(units_total as u64)),
            ("facesDone".to_string(), Value::from(faces_done as u64)),
            ("facesTotal".to_string(), Value::from(faces_total as u64)),
        ]),
        TessellationStepOutcome::Ready { mesh, units_total, faces_total } => {
            let body = match encode_mesh_pack(&mesh) {
                Ok(bytes) => bytes,
                Err(message) => return failed_envelope_json("tessellate.encode", &message),
            };
            let chunks = chunk_mesh_base64(&encode_base64(&body));
            let index = chunk.min(chunks.len().saturating_sub(1));
            object([
                ("done".to_string(), Value::Bool(true)),
                ("cancellable".to_string(), Value::Bool(false)),
                ("phase".to_string(), Value::String("complete".to_string())),
                ("unitsDone".to_string(), Value::from(units_total as u64)),
                ("unitsTotal".to_string(), Value::from(units_total as u64)),
                ("facesDone".to_string(), Value::from(faces_total as u64)),
                ("facesTotal".to_string(), Value::from(faces_total as u64)),
                ("chunk".to_string(), Value::from(index as u64)),
                ("chunks".to_string(), Value::from(chunks.len() as u64)),
                ("packBytes".to_string(), Value::from(body.len() as u64)),
                ("meshPack".to_string(), Value::String(chunks[index].clone())),
            ])
        }
        TessellationStepOutcome::Cancelled => object([("done".to_string(), Value::Bool(true)), ("cancellable".to_string(), Value::Bool(false)), ("phase".to_string(), Value::String("cancelled".to_string()))]),
        TessellationStepOutcome::Invalid { issues } => object([
            ("done".to_string(), Value::Bool(true)),
            ("cancellable".to_string(), Value::Bool(false)),
            ("phase".to_string(), Value::String("invalid".to_string())),
            ("diagnostics".to_string(), Value::Array(issues.iter().map(|issue| object([("entity".to_string(), Value::String(issue.entity.clone())), ("code".to_string(), Value::String(issue.code.clone())), ("message".to_string(), Value::String(issue.message.clone()))])).collect())),
        ]),
        TessellationStepOutcome::Failed { message } => return failed_envelope_json("tessellate.failed", &message),
    };
    semio_framework_os_flow::os_pack::json::to_string(&envelope)
}
pub fn tessellate_geometry_json_for_wasm(&self, handle: &str, tolerance: f64) -> String {
    match self.tessellate_geometry(handle, tolerance) {
        Ok(mesh) => semio_framework_os_flow::os_pack::json::to_json_string(&mesh),
        Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".to_string(), semio_framework_os_flow::os_pack::json::Value::String(error))])),
    }
}
pub fn dispose_geometry(&self, handle: &str) -> Result<(), String> {
    let mut claims = self.state.claims.lock().map_err(|_| "geometry claims lock poisoned")?;
    if self.is_closed() { return Err("geometry.session-closed".into()); }
    if claims.iter().any(|(authority, handles)| *authority != self.state.authority && handles.contains(handle)) { return Err("geometry.handle-retained-by-other-authority".into()); }
    let mut kernel = self.kernel().write().map_err(|_| "geometry kernel lock poisoned")?;
    kernel.dispose(&GeometryHandle(handle.into()));
    if let Some(own) = claims.get_mut(&self.state.authority) { own.remove(handle); }
    self.evict_mesh_cache_for_handle(handle);
    if let Ok(mut jobs) = self.tessellation_jobs().lock() { jobs.jobs.retain(|(job_handle, _), _| job_handle != handle); }
    Ok(())
}
pub fn export_solid_json(&self, handles: &[String], format: &str, deflection: f64) -> String {
    let shapes: Vec<GeometryHandle> = handles.iter().cloned().map(GeometryHandle).collect();
    let outcome: Result<(String, bool), BrepModuleError> = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned).and_then(|guard| {
        if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
        let guard = &**guard;
        match format {

            "obj" => guard.export_obj(&shapes, deflection).map(|text| (text, false)).map_err(BrepModuleError::from),
            "stl" => guard.export_stl(&shapes, deflection).map(|data| (encode_base64(&data), true)).map_err(BrepModuleError::from),
            "glb" => export_glb_via_tessellation(guard, &shapes, deflection).map(|data| (encode_base64(&data), true)),
            other => self.state.operations.export(guard,other,&shapes,deflection),
        }
    });
    match outcome {
        Ok((data, binary)) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([
            ("data".to_string(), semio_framework_os_flow::os_pack::json::Value::String(data)),
            ("binary".to_string(), semio_framework_os_flow::os_pack::json::Value::Bool(binary)),
            ("format".to_string(), semio_framework_os_flow::os_pack::json::Value::String(format.to_string())),
        ])),
        Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".to_string(), semio_framework_os_flow::os_pack::json::Value::String(error.to_string()))])),
    }
}
pub fn import_solid_json(&self, format: &str, data: &str, tolerance: f64) -> String {
    let mut claims = self.state.claims.lock().expect("geometry claims");
    let outcome: Result<Vec<String>, BrepModuleError> = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned).and_then(|mut guard| {
        if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
        let guard = &mut **guard;
        match format {

            "obj" => guard.import_obj(data, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from),
            "stl" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| guard.import_stl(&bytes, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)),
            "glb" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| import_glb_via_tessellation(guard, &bytes, tolerance)),
            other => self.state.operations.import(guard,other,data,tolerance).map(|handles| handles.into_iter().map(|handle| handle.0).collect()),
        }
    });
    if let Ok(handles) = &outcome { claims.entry(self.state.authority).or_default().extend(handles.iter().cloned()); }
    match outcome {
        Ok(handles) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("handles".to_string(), semio_framework_os_flow::os_pack::json::from_dsl_value(&semio_framework_os_flow::os_dsl::ToValue::to_value(&handles)))])),
        Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".to_string(), semio_framework_os_flow::os_pack::json::Value::String(error.to_string()))])),
    }
}
fn brep_invoke_inner(&self, method: &str, args_json: &str) -> Result<semio_framework_os_flow::os_pack::json::Value, BrepModuleError> {
    let mut claims = self.state.claims.lock().map_err(|_| BrepModuleError::LockPoisoned)?;
    if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
    let args = invoke_args(args_json)?;
    let result = match method {
        "box" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.box_prim(arg_f64(&args, "width")?, arg_f64(&args, "depth")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sphere" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sphere_prim(arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cylinder" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cylinder_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cone" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cone_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "lineCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.line_curve(arg_vec3(&args, "start")?, arg_vec3(&args, "end")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "circleCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.circle_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "arcCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .arc_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?, arg_f64(&args, "startAngle")?, arg_f64(&args, "endAngle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "ellipseCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .ellipse_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "semiMajor")?, arg_f64(&args, "semiMinor")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "interpolateCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.interpolate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "approximateCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .approximate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?, arg_usize(&args, "controlPoints")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "polylineWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.polyline_wire(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rectangleWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.rectangle_wire(arg_f64(&args, "width")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromPoints" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.planar_face_from_points(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.planar_face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrudeWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.extrude_wire(&arg_handle(&args, "wire")?, arg_vec3(&args, "vector")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrude" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.extrude(&arg_handle(&args, "face")?, arg_vec3(&args, "direction")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "revolve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .revolve(&arg_handle(&args, "face")?, arg_vec3(&args, "axisOrigin")?, arg_vec3(&args, "axisDirection")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "loft" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.loft(&arg_handles(&args, "profiles")?, arg_bool_or(&args, "smooth", false)).map(handle_result).map_err(BrepModuleError::from)
        }
        "sweep" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sweep(&arg_handle(&args, "profile")?, &arg_handle(&args, "path")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "thickenFace" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.thicken_face(&arg_handle(&args, "face")?, arg_f64(&args, "thickness")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "offsetFace" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.offset_face(&arg_handle(&args, "face")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "fuse" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.fuse(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cut" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cut(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "intersect" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.intersect(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "translate" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.translate(&arg_handle(&args, "shape")?, arg_vec3(&args, "offset")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rotate" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.rotate(&arg_handle(&args, "shape")?, arg_vec3(&args, "axis")?, arg_f64(&args, "angle")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rotateAbout" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .rotate_about(&arg_handle(&args, "shape")?, arg_vec3(&args, "origin")?, arg_vec3(&args, "axis")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "scale" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.scale(&arg_handle(&args, "shape")?, arg_f64(&args, "factor")?, arg_vec3(&args, "center")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "mirror" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.mirror(&arg_handle(&args, "shape")?, arg_vec3(&args, "origin")?, arg_vec3(&args, "normal")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sewFaces" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sew_faces(&arg_handles(&args, "faces")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "faceFromWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "healSolid" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.heal_solid(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "volume" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.volume(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "area" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.area(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "length" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.length(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "centerOfMass" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.center_of_mass(&arg_handle(&args, "shape")?).map(vec3_result).map_err(BrepModuleError::from)
        }
        "distance" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.distance(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(number_result).map_err(BrepModuleError::from)
        }
        "deconstruct" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.deconstruct(&arg_handle(&args, "shape")?).map(topology_result).map_err(BrepModuleError::from)
        }
        "tessellate" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.tessellate(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-3)).map(|mesh| mesh_result(&mesh)).map_err(BrepModuleError::from)
        }
        "dispose" => {
            let handle = arg_handle(&args, "handle")?;
            if claims.iter().any(|(authority, handles)| *authority != self.state.authority && handles.contains(&handle.0)) { return Err(BrepModuleError::InvalidArgs("geometry.handle-retained-by-other-authority".into())); }
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.dispose(&handle);
            if let Some(own) = claims.get_mut(&self.state.authority) { own.remove(&handle.0); }
            self.evict_mesh_cache_for_handle(&handle.0);
            if let Ok(mut jobs) = self.tessellation_jobs().lock() { jobs.jobs.retain(|(job_handle, _), _| job_handle != &handle.0); }
            Ok(unit_result())
        }
        "retain" => {
            let live = arg_handles(&args, "handles")?.into_iter().map(|handle| handle.0).collect::<Vec<_>>();
            claims.insert(self.state.authority, live.iter().cloned().collect());
            let merged = claims.values().flat_map(|handles| handles.iter().cloned()).collect::<HashSet<_>>();
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.retain(&merged);
            self.evict_mesh_cache_for_handles(&merged.into_iter().collect::<Vec<_>>());
            self.retain_tessellation_jobs(&live);
            Ok(unit_result())
        }
        other => {
            let mut guard = self.kernel().write().map_err(|_|BrepModuleError::LockPoisoned)?;
            self.state.operations.invoke(&mut guard,other,&args)
        },
    };
    if let Ok(value) = &result {
        fn gather(value: &semio_framework_os_flow::os_pack::json::Value, handles: &mut BTreeSet<String>) {
            if let Some(handle) = value.get("handle").and_then(|value| value.as_str()) { handles.insert(handle.into()); }
            for key in ["handles", "vertices", "edges", "faces", "shells"] {
                if let Some(values) = value.get(key).and_then(|value| value.as_array()) { handles.extend(values.iter().filter_map(|value| value.as_str().map(str::to_string))); }
            }
        }
        gather(value, claims.entry(self.state.authority).or_default());
    }
    result
}
pub fn brep_invoke_json(&self, method: &str, args_json: &str) -> String {
    match self.brep_invoke_inner(method, args_json) {
        Ok(value) => semio_framework_os_flow::os_pack::json::to_string(&value),
        Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".to_string(), semio_framework_os_flow::os_pack::json::Value::String(error.to_string()))])),
    }
}
}

/// ⚓️ Retains the producing family while retiring a separately admitted job authority.
struct SessionPort { authority:ManuallyDrop<Option<Session>>, anchor:SessionCapture }
impl Drop for SessionPort {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.authority.is_none() && self.anchor.terminal_is_empty(),"geometry port requires explicit retirement before drop"); } }
}
impl semio_framework_os_flow::geometry::GeometryPort for SessionPort {
    fn retain(&self, handles: &[String]) { if let Some(authority) = self.authority.as_ref() { authority.retain_geometry_handles(handles); } }
    fn tessellate_step(&self, handle: &str, tolerance: f64, units: usize) -> semio_framework_os_flow::geometry::GeometryStep {
        use semio_framework_os_flow::geometry::GeometryStep;
        let Some(authority) = self.authority.as_ref() else { return GeometryStep::Failed("geometry session is closed".into()) };
        match authority.tessellate_step(handle, tolerance, units) {
            TessellationStepOutcome::Working { units_done, units_total, phase, .. } => GeometryStep::Working { units_done, units_total, phase: phase.into() },
            TessellationStepOutcome::Ready { mesh, .. } => GeometryStep::Ready(mesh),
            TessellationStepOutcome::Cancelled => GeometryStep::Cancelled,
            TessellationStepOutcome::Failed { message } => GeometryStep::Failed(message),
            TessellationStepOutcome::Invalid { issues } => GeometryStep::Failed(issues.into_iter().map(|issue| issue.message).collect::<Vec<_>>().join("; ")),
        }
    }
    fn dispose(&self, handle: &str) -> Result<(), String> { self.authority.as_ref().ok_or("geometry session is closed")?.dispose_geometry(handle) }
    fn cancel(&self) -> usize { self.authority.as_ref().map_or(0,Session::cancel_all_tessellations) }
    fn begin_close(&self) { if let Some(authority) = self.authority.as_ref() { authority.begin_close(); } }
    fn terminal_is_empty(&self) -> bool { self.authority.is_none() && self.anchor.terminal_is_empty() }
    fn next_close_byte_demand(&self) -> usize { self.authority.as_ref().map_or_else(|| self.anchor.shell_byte_requirement(),|authority| shell_bytes(&authority.state)) }
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize) -> Result<neural_engine::ValueRetirementStep,String> {
        use neural_engine::ValueRetirementStep as Step;
        if self.authority.is_none() && self.anchor.terminal_is_empty() { return Ok(Step::Complete); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        if self.anchor.session.as_ref().is_some_and(|session| session.state.retirement.lock().expect("geometry retirement").paused) { return Ok(Step::Blocked); }
        if let Some(authority) = self.authority.as_ref() {
            if maximum_bytes < shell_bytes(&authority.state) { return Ok(Step::Blocked); }
            authority.begin_close();
            let receipt = std::sync::atomic::AtomicUsize::new(0);
            let Session { state } = self.authority.take().expect("geometry port authority");
            record_shell(&state,&receipt);
            if let Some(state) = Arc::into_inner(state) { drop(state); }
            return Ok(Step::Pending { released_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed) });
        }
        self.anchor.close_step(maximum_items,maximum_bytes)
    }
}
#[cfg(all(target_arch = "wasm32", not(target_env = "p2"), feature = "browser-publication"))]
mod browser {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub struct BrowserSession { session: super::Session }
    #[wasm_bindgen]
    impl BrowserSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self { Self { session: super::Session::new() } }
        pub fn brep_invoke(&self, method: &str, arguments: &str) -> String { self.session.brep_invoke_json(method, arguments) }
        pub fn tessellate(&self, handle: &str, tolerance: f64) -> String { self.session.tessellate_geometry_json_for_wasm(handle, tolerance) }
        pub fn dispose(&self, handle: &str) -> Result<(), JsValue> { self.session.dispose_geometry(handle).map_err(|error| JsValue::from_str(&error)) }
        pub fn begin_close(&self) { self.session.begin_close(); }
        pub fn cancel_close(&self) { self.session.cancel_close(); }
        pub fn resume_close(&self) { self.session.resume_close(); }
        pub fn terminal_is_empty(&self) -> bool { self.session.terminal_is_empty() }
        pub fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> String {
            match self.session.close_step(maximum_items,maximum_bytes) {
                Ok(neural_engine::ValueRetirementStep::Blocked) => "{\"phase\":\"blocked\",\"items\":0,\"bytes\":0}".into(),
                Ok(neural_engine::ValueRetirementStep::Complete) => "{\"phase\":\"complete\",\"items\":0,\"bytes\":0}".into(),
                Ok(neural_engine::ValueRetirementStep::Pending { released_items,released_bytes }) => format!("{{\"phase\":\"pending\",\"items\":{released_items},\"bytes\":{released_bytes}}}"),
                Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".into(),semio_framework_os_flow::os_pack::json::Value::String(error))])),
            }
        }
    }
}
````

## ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs

Bytes 30037; SHA-256 f48b9eb010e7da598eeef60c8dd663bfc08af66d0eaf2df58050cad0f2fbba2d.

````text
//! 🖊️ Flow draw module: 2D vector-graphics operators backed by [`flow_extension_sdk::DrawingStore`].

use flow_extension_sdk::with_drawing_kernel as with_kernel;
use flow_extension_sdk::{DrawingHandle, DrawingKernel, DrawingStore, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle};
use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use semio_framework_2d::{DrawingError, Vec2};

// #region 🔖️Helpers

fn map_kernel_error(error: &DrawingError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

fn kind_label(kind: flow_extension_sdk::DrawingKind) -> &'static str {
    match kind {
        flow_extension_sdk::DrawingKind::Rect => "rect",
        flow_extension_sdk::DrawingKind::Ellipse => "ellipse",
        flow_extension_sdk::DrawingKind::Circle => "circle",
        flow_extension_sdk::DrawingKind::Line => "line",
        flow_extension_sdk::DrawingKind::Polygon => "polygon",
        flow_extension_sdk::DrawingKind::Path => "path",
        flow_extension_sdk::DrawingKind::Text => "text",
        flow_extension_sdk::DrawingKind::Group => "group",
    }
}

fn drawing_dict(kernel: &DrawingStore, handle: &DrawingHandle) -> Result<Dictionary, EvalError> {
    let kind = kernel.kind(handle).map_err(|error| map_kernel_error(&error))?;
    Ok(Dictionary::with_schema("draw.drawing").insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))).insert("kind", Value::Atom(Atom::String(kind_label(kind).into()))))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_drawing(input: &Dictionary, key: &str) -> Result<DrawingHandle, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let handle = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).ok_or_else(|| EvalError::MissingInput(format!("{key}.handle")))?;
    Ok(DrawingHandle(handle.to_string()))
}

fn read_point_list(input: &Dictionary, key: &str) -> Result<Vec<Vec2>, EvalError> {
    let list = input.get(key).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("list")).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse().ok()).collect();
    indices.sort_unstable();
    indices
        .into_iter()
        .map(|index| {
            let dict = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be a point")))?;
            Ok([dict.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0), dict.get("y").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0)])
        })
        .collect()
}

fn read_rgba(input: &Dictionary, key: &str) -> [f64; 4] {
    [
        read_channel_number(input, &format!("{key}R")).unwrap_or(0.0),
        read_channel_number(input, &format!("{key}G")).unwrap_or(0.0),
        read_channel_number(input, &format!("{key}B")).unwrap_or(0.0),
        read_channel_number(input, &format!("{key}A")).unwrap_or(1.0),
    ]
}

fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

fn drawing_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id])
}

fn list_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::list(id, &[operator_id])
}

fn text_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::text_default(id, "", &[operator_id])
}

fn out_drawing(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("D", "Drw", "draw.drawing", full_name)
}

#[allow(
    clippy::too_many_arguments,
    reason = "positional operator-metadata builder mirroring this file's registration table shape (id/name/abbr/icon/summary/inputs/outputs/group columns); restructuring into a params struct would only churn call sites with no behavior change"
)]
fn operator_info(id: &str, name: &str, abbr: &str, icon: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>, group: &[&str]) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        extension: "draw".into(),
        name: name.into(),
        abbreviation: abbr.into(),
        icon: icon.into(),
        summary: summary.into(),
        inputs,
        outputs,
        group: group.iter().map(|entry| (*entry).to_string()).collect(),
        ..Default::default()
    }
}

fn drawing_schema() -> Schema {
    Schema {
        id: "draw.drawing".into(),
        module: "draw".into(),
        name: "Drawing".into(),
        icon: "emoji:🖊️".into(),
        summary: "Opaque 2D drawing handle".into(),
        fields: vec![FieldSpec::new("handle", ValueType::Text), FieldSpec::new("kind", ValueType::Text)],
    }
}

// #endregion 🔖️Helpers

// #region 🔖️ShapeMutations
struct ShapeRect;
impl Operator for ShapeRect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let width = read_channel_number(input, "width")?;
            let height = read_channel_number(input, "height")?;
            let handle = k.rect(x, y, width, height).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeEllipse;
impl Operator for ShapeEllipse {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let cx = read_channel_number(input, "cx")?;
            let cy = read_channel_number(input, "cy")?;
            let rx = read_channel_number(input, "rx")?;
            let ry = read_channel_number(input, "ry")?;
            let handle = k.ellipse(cx, cy, rx, ry).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeCircle;
impl Operator for ShapeCircle {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let cx = read_channel_number(input, "cx")?;
            let cy = read_channel_number(input, "cy")?;
            let r = read_channel_number(input, "r")?;
            let handle = k.circle(cx, cy, r).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeLine;
impl Operator for ShapeLine {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x1 = read_channel_number(input, "x1")?;
            let y1 = read_channel_number(input, "y1")?;
            let x2 = read_channel_number(input, "x2")?;
            let y2 = read_channel_number(input, "y2")?;
            let handle = k.line(x1, y1, x2, y2).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapePolygon;
impl Operator for ShapePolygon {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let points = read_point_list(input, "points")?;
            let handle = k.polygon(&points).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️ShapeMutations

// #region 🔖️PathMutations
struct PathPolyline;
impl Operator for PathPolyline {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let points = read_point_list(input, "points")?;
            let handle = k.polyline_path(&points).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct PathRect;
impl Operator for PathRect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let width = read_channel_number(input, "width")?;
            let height = read_channel_number(input, "height")?;
            let handle = k.rect_path(x, y, width, height).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️PathMutations

// #region 🔖️StyleMutations
struct StyleFill;
impl Operator for StyleFill {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let color = read_rgba(input, "color");
            let handle = k.set_fill(&drawing, FillStyle::Solid { color }).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct StyleStroke;
impl Operator for StyleStroke {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let color = read_rgba(input, "color");
            let width = read_channel_number(input, "width").unwrap_or(1.0);
            let stroke = StrokeStyle { color, width, cap: LineCap::Butt, join: LineJoin::Miter, dash: Vec::new() };
            let handle = k.set_stroke(&drawing, stroke).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️StyleMutations

// #region 🔖️XformMutations
struct XformTranslate;
impl Operator for XformTranslate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let dx = read_channel_number(input, "dx")?;
            let dy = read_channel_number(input, "dy")?;
            let handle = k.translate(&drawing, dx, dy).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct XformRotate;
impl Operator for XformRotate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let angle = read_channel_number(input, "angle")?;
            let handle = k.rotate(&drawing, angle).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct XformScale;
impl Operator for XformScale {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let sx = read_channel_number(input, "sx")?;
            let sy = read_channel_number(input, "sy").unwrap_or(sx);
            let handle = k.scale(&drawing, sx, sy).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️XformMutations

// #region 🔖️GroupMutations
struct GroupMerge;
impl Operator for GroupMerge {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = k.group(&[a, b]).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️GroupMutations

// #region 🔖️BoolMutations
struct BoolUnion;
impl Operator for BoolUnion {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = k.bool_union(&a, &b).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct BoolDifference;
impl Operator for BoolDifference {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = k.bool_difference(&a, &b).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct BoolIntersection;
impl Operator for BoolIntersection {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = k.bool_intersection(&a, &b).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️BoolMutations

// #region 🔖️TextMutations
struct DrawText;
impl Operator for DrawText {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let content = read_text(input, "text")?;
            let size = read_channel_number(input, "size").unwrap_or(16.0);
            let handle = k.text(x, y, &content, size).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️TextMutations

// #region 🔖️GradientMutations
struct GradientLinear;
impl Operator for GradientLinear {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let x1 = read_channel_number(input, "x1")?;
            let y1 = read_channel_number(input, "y1")?;
            let x2 = read_channel_number(input, "x2")?;
            let y2 = read_channel_number(input, "y2")?;
            let stops = vec![GradientStop { offset: 0.0, color: read_rgba(input, "start") }, GradientStop { offset: 1.0, color: read_rgba(input, "end") }];
            let handle = k.linear_gradient_fill(&drawing, x1, y1, x2, y2, &stops).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️GradientMutations

// #region 🔖️ClipMutations
struct ClipApply;
impl Operator for ClipApply {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let target = read_drawing(input, "target")?;
            let clip = read_drawing(input, "clip")?;
            let handle = k.apply_clip(&target, &clip).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️ClipMutations

/// 📦️ Registers all draw operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(drawing_schema());
    let shape = &["Shapes"];
    let paths = &["Paths"];
    let style = &["Style"];
    let xform = &["Transform"];
    let group = &["Group"];
    let boolean = &["Boolean"];
    let text = &["Text"];
    let gradient = &["Gradient"];
    let clip = &["Clip"];

    registry.register_operator(
        operator_info(
            "draw.shape.rect",
            "Rect",
            "Rct",
            "emoji:▭️",
            "Axis-aligned rectangle",
            vec![number_channel("x", "draw.shape.rect", 0.0), number_channel("y", "draw.shape.rect", 0.0), number_channel("width", "draw.shape.rect", 10.0), number_channel("height", "draw.shape.rect", 10.0)],
            vec![out_drawing("Rectangle")],
            shape,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapeRect) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.shape.ellipse",
            "Ellipse",
            "Ell",
            "emoji:⬭️",
            "Ellipse",
            vec![number_channel("cx", "draw.shape.ellipse", 0.0), number_channel("cy", "draw.shape.ellipse", 0.0), number_channel("rx", "draw.shape.ellipse", 10.0), number_channel("ry", "draw.shape.ellipse", 5.0)],
            vec![out_drawing("Ellipse")],
            shape,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapeEllipse) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.shape.circle",
            "Circle",
            "Cir",
            "emoji:⚪️",
            "Circle",
            vec![number_channel("cx", "draw.shape.circle", 0.0), number_channel("cy", "draw.shape.circle", 0.0), number_channel("r", "draw.shape.circle", 5.0)],
            vec![out_drawing("Circle")],
            shape,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapeCircle) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.shape.line",
            "Line",
            "Lin",
            "emoji:╱️",
            "Line segment",
            vec![number_channel("x1", "draw.shape.line", 0.0), number_channel("y1", "draw.shape.line", 0.0), number_channel("x2", "draw.shape.line", 10.0), number_channel("y2", "draw.shape.line", 10.0)],
            vec![out_drawing("Line")],
            shape,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapeLine) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.shape.polygon", "Polygon", "Pol", "emoji:⬡️", "Closed polygon", vec![list_channel("points", "draw.shape.polygon")], vec![out_drawing("Polygon")], shape),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapePolygon) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.path.polyline", "Polyline", "Pln", "emoji:〰", "Open polyline path", vec![list_channel("points", "draw.path.polyline")], vec![out_drawing("PolylinePath")], paths),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(PathPolyline) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.path.rect",
            "Rect Path",
            "Rph",
            "emoji:▭️",
            "Rectangle path",
            vec![number_channel("x", "draw.path.rect", 0.0), number_channel("y", "draw.path.rect", 0.0), number_channel("width", "draw.path.rect", 10.0), number_channel("height", "draw.path.rect", 10.0)],
            vec![out_drawing("RectPath")],
            paths,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(PathRect) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.style.fill",
            "Fill",
            "Fil",
            "emoji:🪣️",
            "Solid fill",
            vec![
                drawing_channel("drawing", "draw.style.fill"),
                number_channel("colorR", "draw.style.fill", 1.0),
                number_channel("colorG", "draw.style.fill", 1.0),
                number_channel("colorB", "draw.style.fill", 1.0),
                number_channel("colorA", "draw.style.fill", 1.0),
            ],
            vec![out_drawing("FilledDrawing")],
            style,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(StyleFill) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.style.stroke",
            "Stroke",
            "Str",
            "emoji:🖌️",
            "Stroke outline",
            vec![
                drawing_channel("drawing", "draw.style.stroke"),
                number_channel("width", "draw.style.stroke", 1.0),
                number_channel("colorR", "draw.style.stroke", 0.0),
                number_channel("colorG", "draw.style.stroke", 0.0),
                number_channel("colorB", "draw.style.stroke", 0.0),
                number_channel("colorA", "draw.style.stroke", 1.0),
            ],
            vec![out_drawing("StrokedDrawing")],
            style,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(StyleStroke) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.xform.translate",
            "Translate",
            "Trn",
            "emoji:↔",
            "Translate drawing",
            vec![drawing_channel("drawing", "draw.xform.translate"), number_channel("dx", "draw.xform.translate", 0.0), number_channel("dy", "draw.xform.translate", 0.0)],
            vec![out_drawing("TranslatedDrawing")],
            xform,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(XformTranslate) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.xform.rotate", "Rotate", "Rot", "emoji:🔄️", "Rotate drawing", vec![drawing_channel("drawing", "draw.xform.rotate"), number_channel("angle", "draw.xform.rotate", 0.0)], vec![out_drawing("RotatedDrawing")], xform),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(XformRotate) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.xform.scale",
            "Scale",
            "Scl",
            "emoji:↕️",
            "Scale drawing",
            vec![drawing_channel("drawing", "draw.xform.scale"), number_channel("sx", "draw.xform.scale", 1.0), number_channel("sy", "draw.xform.scale", 1.0)],
            vec![out_drawing("ScaledDrawing")],
            xform,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(XformScale) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.group.merge", "Merge", "Mrg", "emoji:🗂️", "Merge drawings into a group", vec![drawing_channel("a", "draw.group.merge"), drawing_channel("b", "draw.group.merge")], vec![out_drawing("MergedGroup")], group),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(GroupMerge) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.bool.union", "Union", "Uni", "emoji:∪", "Boolean union", vec![drawing_channel("a", "draw.bool.union"), drawing_channel("b", "draw.bool.union")], vec![out_drawing("UnionDrawing")], boolean),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(BoolUnion) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.bool.difference", "Difference", "Dif", "emoji:−", "Boolean difference", vec![drawing_channel("a", "draw.bool.difference"), drawing_channel("b", "draw.bool.difference")], vec![out_drawing("DifferenceDrawing")], boolean),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(BoolDifference) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.bool.intersection",
            "Intersection",
            "Int",
            "emoji:∩",
            "Boolean intersection",
            vec![drawing_channel("a", "draw.bool.intersection"), drawing_channel("b", "draw.bool.intersection")],
            vec![out_drawing("IntersectionDrawing")],
            boolean,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(BoolIntersection) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.text",
            "Text",
            "Txt",
            "emoji:🔤️",
            "Text label",
            vec![number_channel("x", "draw.text", 0.0), number_channel("y", "draw.text", 0.0), text_channel("text", "draw.text"), number_channel("size", "draw.text", 16.0)],
            vec![out_drawing("TextDrawing")],
            text,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(DrawText) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.gradient.linear",
            "Linear Gradient",
            "Lgr",
            "emoji:🌈️",
            "Linear gradient fill",
            vec![
                drawing_channel("drawing", "draw.gradient.linear"),
                number_channel("x1", "draw.gradient.linear", 0.0),
                number_channel("y1", "draw.gradient.linear", 0.0),
                number_channel("x2", "draw.gradient.linear", 10.0),
                number_channel("y2", "draw.gradient.linear", 0.0),
                number_channel("startR", "draw.gradient.linear", 1.0),
                number_channel("startG", "draw.gradient.linear", 0.0),
                number_channel("startB", "draw.gradient.linear", 0.0),
                number_channel("startA", "draw.gradient.linear", 1.0),
                number_channel("endR", "draw.gradient.linear", 0.0),
                number_channel("endG", "draw.gradient.linear", 0.0),
                number_channel("endB", "draw.gradient.linear", 1.0),
                number_channel("endA", "draw.gradient.linear", 1.0),
            ],
            vec![out_drawing("GradientDrawing")],
            gradient,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(GradientLinear) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.clip.apply", "Clip", "Clp", "emoji:✂️", "Apply clip path", vec![drawing_channel("target", "draw.clip.apply"), drawing_channel("clip", "draw.clip.apply")], vec![out_drawing("ClippedDrawing")], clip),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ClipApply) }],
        &["draw.drawing"],
    );
    registry.finalize();
}

// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::build_manifest_json;
    build_manifest_json("draw", "Draw", env!("CARGO_PKG_VERSION"), &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![], vec![])
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::module_registry;
    use flow_extension_sdk::{build_manifest_json, evaluate_invoke_json, flow_extension_topic_contribution};
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "draw";
    const EXTENSION_LABEL: &str = "Draw";

    // 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires
    // a plain sync fn). `.mode`/`.contributes_topic`/`.handler` are still `async fn` in
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's
    // path_scope); bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request.
    // See R9.
    fn bundle() -> ExtensionBundle {
        let manifest_json = build_manifest_json("draw", "Draw", env!("CARGO_PKG_VERSION"), &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![], vec![]);
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "draw", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "draw", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-draw", "Draw", env!("CARGO_PKG_VERSION")).extends("flow").depends_on("flow", semio_framework::tree_pin!());
        let bundle = bundle.mode(ExecutionMode::Linked);
        let bundle = bundle.contributes_topic(flow_topic.topic, flow_topic.payload);
        let bundle = bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload);
        bundle.handler("evaluate", |req| evaluate_invoke_json(&neural_engine::ColdOwner::new(module_registry()), req).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err)))
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️extension-guest-standalone/🦀️.rs");

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
````

## ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️.rs

Bytes 9703; SHA-256 484003e849bdfae3f62237095b2ba1c947d02da0ed22285e99b631de4ac07d6b.

````text
//! 🧱️ Flow core module: schema constructors for primitive dictionaries.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};

// #region 🔖️Number
/// 🔢️ Emits a number dictionary.
pub struct Number;

impl Operator for Number {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("number", number_dictionary(read_number(input, "value").or_else(|_| read_number(input, "number"))?)))
    }
}
// #endregion 🔖️Number

// #region 🔖️Text
/// 📝️ Emits a text dictionary.
pub struct Text;

impl Operator for Text {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(read_text(input, "value").or_else(|_| read_text(input, "text"))?)))
    }
}
// #endregion 🔖️Text

// #region 🔖️Boolean
/// 🔀️ Emits a boolean dictionary.
pub struct Boolean;

impl Operator for Boolean {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("boolean", boolean_dictionary(read_bool(input, "value").or_else(|_| read_bool(input, "boolean")).unwrap_or(false))))
    }
}
// #endregion 🔖️Boolean

// #region 🔖️Image
/// 🖼️ Emits an image dictionary.
pub struct Image;

impl Operator for Image {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("image", Dictionary::with_schema("image").insert("dataUrl", Value::Atom(Atom::String(read_text(input, "dataUrl").unwrap_or_default())))))
    }
}
// #endregion 🔖️Image

// #region 🔖️Variable
/// 🔣️ Forwards a named dictionary channel unchanged.
pub struct VariableRelay;

impl Operator for VariableRelay {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let name = read_text(input, "name")?;
        let schema = read_text(input, "schema").unwrap_or_else(|_| "dictionary".into());
        let payload = input.get(&name).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(name.clone()))?;
        if let Some(actual) = payload.schema() {
            if actual != schema.as_str() {
                return Err(EvalError::InvalidInput(format!("expected schema {schema}, got {actual}")));
            }
        }
        Ok(channel_output(&name, payload.clone()))
    }
}
// #endregion 🔖️Variable

// #region 🔖️Helpers
pub fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

pub fn text_dictionary(value: String) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value)))
}

pub fn boolean_dictionary(value: bool) -> Dictionary {
    Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(value)))
}

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_bool(input: &Dictionary, key: &str) -> Result<bool, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn schema(id: &str, name: &str, summary: &str, fields: Vec<FieldSpec>) -> Schema {
    Schema { id: id.into(), module: "core".into(), name: name.into(), icon: "emoji:🧱️".into(), summary: summary.into(), fields }
}

fn operator<O: Operator + 'static>(id: &str, name: &str, summary: &str, outputs: Vec<ChannelSpec>, operation: O) -> (OperatorInfo, Vec<OperatorImpl>) {
    (
        OperatorInfo { id: id.into(), extension: "core".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:🧱️".into(), summary: summary.into(), inputs: vec![], outputs, ..Default::default() },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(operation) }],
    )
}

// #endregion 🔖️Helpers

/// 📦️ Registers core schemas and value operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(schema("number", "Number", "Decimal number", vec![FieldSpec::decimal_default("value", 0.0)]));
    registry.register_schema(schema("text", "Text", "Text value", vec![FieldSpec::new("value", ValueType::Text).with_default(Value::Atom(Atom::String(String::new())))]));
    registry.register_schema(schema("boolean", "Boolean", "Boolean value", vec![FieldSpec::new("value", ValueType::Boolean).with_default(Value::Atom(Atom::Boolean(false)))]));
    registry.register_schema(schema("list", "List", "Index-keyed dictionary list", vec![]));
    registry.register_schema(schema("dictionary", "Dictionary", "Arbitrary dictionary", vec![]));
    registry.register_schema(schema("image", "Image", "Image data URL", vec![FieldSpec::new("dataUrl", ValueType::Text).with_default(Value::Atom(Atom::String(String::new())))]));

    let (info, implementations) = operator("core.number", "Number", "Produces a number dictionary", vec![ChannelSpec::named("N", "Num", "number", "Number")], Number);
    registry.register_operator(info, implementations, &["number"]);
    let (info, implementations) = operator("core.text", "Text", "Produces a text dictionary", vec![ChannelSpec::named("T", "Txt", "text", "Text")], Text);
    registry.register_operator(info, implementations, &["text"]);
    let (info, implementations) = operator("core.boolean", "Bool", "Produces a boolean dictionary", vec![ChannelSpec::named("B", "Boo", "boolean", "Boolean")], Boolean);
    registry.register_operator(info, implementations, &["boolean"]);
    let (info, implementations) = operator("core.image", "Image", "Produces an image dictionary", vec![ChannelSpec::named("I", "Img", "image", "Image")], Image);
    registry.register_operator(info, implementations, &["image"]);
    registry.register_operator(
        OperatorInfo {
            id: "core.variable".into(),
            extension: "core".into(),
            name: "Variable".into(),
            abbreviation: "Var".into(),
            icon: "emoji:🔣️".into(),
            summary: "Relays a named typed dictionary".into(),
            inputs: vec![ChannelSpec::wildcard()],
            outputs: vec![ChannelSpec::wildcard()],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(VariableRelay) }],
        &[],
    );
    registry.finalize();
}

// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::build_manifest_json;
    build_manifest_json("core", "Core", env!("CARGO_PKG_VERSION"), &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![], vec![])
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
/// 🧩️ Runtime-installable flow extension bundle for `core`.
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::{extension_manifest_json, module_registry};
    use flow_extension_sdk::{evaluate_invoke_json, flow_extension_topic_contribution};
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "core";
    const EXTENSION_LABEL: &str = "Core";

    // 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires
    // a plain sync fn). `.mode`/`.contributes_topic`/`.handler` are still `async fn` in
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's
    // path_scope); bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request.
    // See R9.
    fn bundle() -> ExtensionBundle {
        let manifest_json = extension_manifest_json();
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "core", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "core", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-primitive", EXTENSION_LABEL, env!("CARGO_PKG_VERSION")).extends("flow").depends_on("flow", semio_framework::tree_pin!());
        let bundle = bundle.mode(ExecutionMode::Linked);
        let bundle = bundle.contributes_topic(flow_topic.topic, flow_topic.payload);
        let bundle = bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload);
        bundle.handler("evaluate", |req| evaluate_invoke_json(&neural_engine::ColdOwner::new(module_registry()), req).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err)))
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️extension-guest-standalone/🦀️.rs");

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
````

## ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️.rs

Bytes 25828; SHA-256 8291e22a93d197dba63591b6b30117e307e337b238736528abeef14ca6998558.

````text
//! 🏗️ Flow bim module: semantic building information modeling operators.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType, VariadicSpec};

// #region 🔖️Schemas
fn material_schema() -> Schema {
    Schema {
        id: "material".into(),
        module: "bim".into(),
        name: "Material".into(),
        icon: "emoji:🧱️".into(),
        summary: "Building material with thermal and structural properties".into(),
        fields: vec![FieldSpec::new("name", ValueType::Text), FieldSpec::decimal_default("density", 2400.0), FieldSpec::decimal_default("conductivity", 1.4), FieldSpec::decimal_default("strength", 30.0)],
    }
}

fn space_schema() -> Schema {
    Schema {
        id: "space".into(),
        module: "bim".into(),
        name: "Space".into(),
        icon: "emoji:🏠️".into(),
        summary: "Occupiable space with area and height".into(),
        fields: vec![FieldSpec::new("name", ValueType::Text), FieldSpec::decimal_default("area", 20.0), FieldSpec::decimal_default("height", 2.8)],
    }
}

fn wall_schema() -> Schema {
    Schema {
        id: "wall".into(),
        module: "bim".into(),
        name: "Wall".into(),
        icon: "emoji:🧱️".into(),
        summary: "Structural or partition wall".into(),
        fields: vec![FieldSpec::decimal_default("length", 4.0), FieldSpec::decimal_default("height", 2.8), FieldSpec::decimal_default("thickness", 0.2)],
    }
}

fn slab_schema() -> Schema {
    Schema {
        id: "slab".into(),
        module: "bim".into(),
        name: "Slab".into(),
        icon: "emoji:⬜️".into(),
        summary: "Horizontal slab element".into(),
        fields: vec![FieldSpec::decimal_default("width", 10.0), FieldSpec::decimal_default("depth", 8.0), FieldSpec::decimal_default("thickness", 0.25)],
    }
}

fn column_schema() -> Schema {
    Schema {
        id: "column".into(),
        module: "bim".into(),
        name: "Column".into(),
        icon: "emoji:🏛️".into(),
        summary: "Vertical structural column".into(),
        fields: vec![FieldSpec::decimal_default("width", 0.4), FieldSpec::decimal_default("depth", 0.4), FieldSpec::decimal_default("height", 3.0)],
    }
}

fn window_schema() -> Schema {
    Schema {
        id: "window".into(),
        module: "bim".into(),
        name: "Window".into(),
        icon: "emoji:🪟️".into(),
        summary: "Glazed opening".into(),
        fields: vec![FieldSpec::decimal_default("width", 1.2), FieldSpec::decimal_default("height", 1.4), FieldSpec::decimal_default("sill", 0.9)],
    }
}

fn story_schema() -> Schema {
    Schema {
        id: "story".into(),
        module: "bim".into(),
        name: "Story".into(),
        icon: "emoji:🏢️".into(),
        summary: "Building story with elements and spaces".into(),
        fields: vec![
            FieldSpec::decimal_default("elevation", 0.0),
            FieldSpec::decimal_default("height", 3.0),
            FieldSpec::new("elements", ValueType::List(Box::new(ValueType::Any))),
            FieldSpec::new("spaces", ValueType::List(Box::new(ValueType::Schema("space".into())))),
        ],
    }
}

fn building_schema() -> Schema {
    Schema {
        id: "building".into(),
        module: "bim".into(),
        name: "Building".into(),
        icon: "emoji:🏗️".into(),
        summary: "Assembled building model".into(),
        fields: vec![FieldSpec::new("name", ValueType::Text), FieldSpec::new("stories", ValueType::List(Box::new(ValueType::Schema("story".into()))))],
    }
}
// #endregion 🔖️Schemas

// #region 🔖️Helpers
fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn text_dictionary(value: impl Into<String>) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value.into())))
}

fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

fn text_channel(id: &str, operator_id: &str, default: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id]).with_default(Value::Dictionary(text_dictionary(default)))
}

fn out_material() -> ChannelSpec {
    ChannelSpec::named("M", "Mat", "material", "Material")
}

fn out_space() -> ChannelSpec {
    ChannelSpec::named("S", "Spc", "space", "Space")
}

fn out_wall() -> ChannelSpec {
    ChannelSpec::named("W", "Wal", "wall", "Wall")
}

fn out_slab() -> ChannelSpec {
    ChannelSpec::named("S", "Slb", "slab", "Slab")
}

fn out_column() -> ChannelSpec {
    ChannelSpec::named("C", "Col", "column", "Column")
}

fn out_window() -> ChannelSpec {
    ChannelSpec::named("W", "Win", "window", "Window")
}

fn out_story() -> ChannelSpec {
    ChannelSpec::named("S", "Sty", "story", "Story")
}

fn out_building() -> ChannelSpec {
    ChannelSpec::named("B", "Bld", "building", "Building")
}

fn out_floor_area() -> ChannelSpec {
    ChannelSpec::named("A", "FlA", "floorArea", "FloorArea")
}

fn out_gross_volume() -> ChannelSpec {
    ChannelSpec::named("V", "GrV", "grossVolume", "GrossVolume")
}

/// 🏷️ Descriptive metadata for a bim operator, grouped to keep `operator_info` under clippy's arg-count limit.
/// `Copy` (all fields are borrowed `&str`) so `operator_info` can take it by value without tripping
/// `clippy::needless_pass_by_value` — found via a real `cargo clippy --all-targets -- -D warnings` run
/// while de-sandwiching this crate's package layout (pre-existing, unrelated to the relocation itself;
/// this lint combination had apparently never run against this crate before).
#[derive(Clone, Copy)]
struct OperatorMeta<'a> {
    id: &'a str,
    name: &'a str,
    abbreviation: &'a str,
    icon: &'a str,
    summary: &'a str,
}

fn operator_info(meta: OperatorMeta<'_>, inputs: Vec<ChannelSpec>, output: ChannelSpec, group: &[&str]) -> OperatorInfo {
    OperatorInfo {
        id: meta.id.into(),
        extension: "bim".into(),
        name: meta.name.into(),
        abbreviation: meta.abbreviation.into(),
        icon: meta.icon.into(),
        summary: meta.summary.into(),
        inputs,
        outputs: vec![output],
        group: group.iter().map(|entry| (*entry).to_string()).collect(),
        ..Default::default()
    }
}

fn register_element<O: Operator + 'static>(registry: &mut Registry, info: OperatorInfo, operation: O, schema_id: &str) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: Box::new(operation) }], &[schema_id, "element"]);
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_field_number(dict: &Dictionary, key: &str) -> Option<f64> {
    dict.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64())
}

#[cfg(test)]
fn read_field_text(dict: &Dictionary, key: &str) -> Option<String> {
    dict.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string)
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

fn list_from_variadic(items: Option<&Dictionary>) -> Dictionary {
    let mut out = Dictionary::with_schema("list");
    let Some(items) = items else {
        return out;
    };
    let mut indices: Vec<String> = items.keys().cloned().collect();
    indices.sort();
    for (next, key) in indices.into_iter().enumerate() {
        if let Some(value) = items.get(&key) {
            out = out.insert(next.to_string(), value.clone());
        }
    }
    out
}

fn collect_elements_and_spaces(items: Option<&Dictionary>) -> (Dictionary, Dictionary) {
    let mut elements = Dictionary::with_schema("list");
    let mut spaces = Dictionary::with_schema("list");
    let Some(items) = items else {
        return (elements, spaces);
    };
    let mut element_index = 0usize;
    let mut space_index = 0usize;
    let mut keys: Vec<String> = items.keys().cloned().collect();
    keys.sort();
    for key in keys {
        let Some(dict) = items.get(&key).and_then(|value| value.as_dictionary()) else {
            continue;
        };
        if dict.schema() == Some("space") {
            spaces = spaces.insert(space_index.to_string(), Value::Dictionary(dict.clone()));
            space_index += 1;
        } else {
            elements = elements.insert(element_index.to_string(), Value::Dictionary(dict.clone()));
            element_index += 1;
        }
    }
    (elements, spaces)
}

fn story_floor_area(story: &Dictionary) -> f64 {
    if let Some(slab) = story.get("slab").and_then(|value| value.as_dictionary()) {
        return read_field_number(slab, "width").unwrap_or(0.0) * read_field_number(slab, "depth").unwrap_or(0.0);
    }
    let mut total = 0.0;
    if let Some(spaces) = story.get("spaces").and_then(|value| value.as_dictionary()) {
        for index in list_indices(spaces) {
            if let Some(space) = spaces.get(&index.to_string()).and_then(|value| value.as_dictionary()) {
                total += read_field_number(space, "area").unwrap_or(0.0);
            }
        }
    }
    total
}

fn story_gross_volume(story: &Dictionary) -> f64 {
    story_floor_area(story) * read_field_number(story, "height").unwrap_or(0.0)
}

fn building_stories(building: &Dictionary) -> Option<&Dictionary> {
    building.get("stories").and_then(|value| value.as_dictionary())
}
// #endregion 🔖️Helpers

// #region 🔖️Elements
struct MaterialElement;

impl Operator for MaterialElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "material",
            Dictionary::with_schema("material")
                .insert("name", Value::Atom(Atom::String(read_channel_text(input, "name").unwrap_or_else(|_| "Concrete".into()))))
                .insert("density", Value::Atom(Atom::Decimal(read_channel_number(input, "density")?)))
                .insert("conductivity", Value::Atom(Atom::Decimal(read_channel_number(input, "conductivity")?)))
                .insert("strength", Value::Atom(Atom::Decimal(read_channel_number(input, "strength")?))),
        ))
    }
}

struct SpaceElement;

impl Operator for SpaceElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "space",
            Dictionary::with_schema("space")
                .insert("name", Value::Atom(Atom::String(read_channel_text(input, "name").unwrap_or_else(|_| "Space".into()))))
                .insert("area", Value::Atom(Atom::Decimal(read_channel_number(input, "area")?)))
                .insert("height", Value::Atom(Atom::Decimal(read_channel_number(input, "height")?))),
        ))
    }
}

struct WallElement;

impl Operator for WallElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "wall",
            Dictionary::with_schema("wall")
                .insert("length", Value::Atom(Atom::Decimal(read_channel_number(input, "length")?)))
                .insert("height", Value::Atom(Atom::Decimal(read_channel_number(input, "height")?)))
                .insert("thickness", Value::Atom(Atom::Decimal(read_channel_number(input, "thickness")?))),
        ))
    }
}

struct SlabElement;

impl Operator for SlabElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "slab",
            Dictionary::with_schema("slab")
                .insert("width", Value::Atom(Atom::Decimal(read_channel_number(input, "width")?)))
                .insert("depth", Value::Atom(Atom::Decimal(read_channel_number(input, "depth")?)))
                .insert("thickness", Value::Atom(Atom::Decimal(read_channel_number(input, "thickness")?))),
        ))
    }
}

struct ColumnElement;

impl Operator for ColumnElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "column",
            Dictionary::with_schema("column")
                .insert("width", Value::Atom(Atom::Decimal(read_channel_number(input, "width")?)))
                .insert("depth", Value::Atom(Atom::Decimal(read_channel_number(input, "depth")?)))
                .insert("height", Value::Atom(Atom::Decimal(read_channel_number(input, "height")?))),
        ))
    }
}

struct WindowElement;

impl Operator for WindowElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "window",
            Dictionary::with_schema("window")
                .insert("width", Value::Atom(Atom::Decimal(read_channel_number(input, "width")?)))
                .insert("height", Value::Atom(Atom::Decimal(read_channel_number(input, "height")?)))
                .insert("sill", Value::Atom(Atom::Decimal(read_channel_number(input, "sill")?))),
        ))
    }
}
// #endregion 🔖️Elements

// #region 🔖️Assembly
struct AssembleStory;

impl Operator for AssembleStory {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let elevation = read_channel_number(input, "elevation").unwrap_or(0.0);
        let height = read_channel_number(input, "height")?;
        let (elements, spaces) = collect_elements_and_spaces(input.get("elements").and_then(|value| value.as_dictionary()));
        let mut story =
            Dictionary::with_schema("story").insert("elevation", Value::Atom(Atom::Decimal(elevation))).insert("height", Value::Atom(Atom::Decimal(height))).insert("elements", Value::Dictionary(elements)).insert("spaces", Value::Dictionary(spaces));
        if let Some(slab) = input.get("slab").and_then(|value| value.as_dictionary()) {
            story = story.insert("slab", Value::Dictionary(slab.clone()));
        }
        Ok(channel_output("story", story))
    }
}

struct AssembleBuilding;

impl Operator for AssembleBuilding {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let name = read_channel_text(input, "name").unwrap_or_else(|_| "Building".into());
        let stories = list_from_variadic(input.get("stories").and_then(|value| value.as_dictionary()));
        Ok(channel_output("building", Dictionary::with_schema("building").insert("name", Value::Atom(Atom::String(name))).insert("stories", Value::Dictionary(stories))))
    }
}
// #endregion 🔖️Assembly

// #region 🔖️Measure
struct FloorArea;

impl Operator for FloorArea {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let building = read_building(input, "building")?;
        let mut total = 0.0;
        if let Some(stories) = building_stories(building) {
            for index in list_indices(stories) {
                if let Some(story) = stories.get(&index.to_string()).and_then(|value| value.as_dictionary()) {
                    total += story_floor_area(story);
                }
            }
        }
        Ok(channel_output("floorArea", number_dictionary(total)))
    }
}

struct GrossVolume;

impl Operator for GrossVolume {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let building = read_building(input, "building")?;
        let mut total = 0.0;
        if let Some(stories) = building_stories(building) {
            for index in list_indices(stories) {
                if let Some(story) = stories.get(&index.to_string()).and_then(|value| value.as_dictionary()) {
                    total += story_gross_volume(story);
                }
            }
        }
        Ok(channel_output("grossVolume", number_dictionary(total)))
    }
}

fn read_building<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input.get(key).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("building")).ok_or_else(|| EvalError::MissingInput(key.into()))
}
// #endregion 🔖️Measure

/// 📦️ Registers bim schemas and operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(material_schema());
    registry.register_schema(space_schema());
    registry.register_schema(wall_schema());
    registry.register_schema(slab_schema());
    registry.register_schema(column_schema());
    registry.register_schema(window_schema());
    registry.register_schema(story_schema());
    registry.register_schema(building_schema());

    register_element(
        registry,
        operator_info(
            OperatorMeta { id: "bim.element.material", name: "Material", abbreviation: "Mat", icon: "emoji:🧱️", summary: "Defines a building material" },
            vec![
                text_channel("name", "bim.element.material", "Concrete"),
                number_channel("density", "bim.element.material", 2400.0),
                number_channel("conductivity", "bim.element.material", 1.4),
                number_channel("strength", "bim.element.material", 30.0),
            ],
            out_material(),
            &["Elements"],
        ),
        MaterialElement,
        "material",
    );
    register_element(
        registry,
        operator_info(
            OperatorMeta { id: "bim.element.space", name: "Space", abbreviation: "Space", icon: "emoji:🏠️", summary: "Defines an occupiable space" },
            vec![text_channel("name", "bim.element.space", "Space"), number_channel("area", "bim.element.space", 20.0), number_channel("height", "bim.element.space", 2.8)],
            out_space(),
            &["Elements"],
        ),
        SpaceElement,
        "space",
    );
    register_element(
        registry,
        operator_info(
            OperatorMeta { id: "bim.element.wall", name: "Wall", abbreviation: "Wall", icon: "emoji:🧱️", summary: "Defines a wall element" },
            vec![number_channel("length", "bim.element.wall", 4.0), number_channel("height", "bim.element.wall", 2.8), number_channel("thickness", "bim.element.wall", 0.2)],
            out_wall(),
            &["Elements"],
        ),
        WallElement,
        "wall",
    );
    register_element(
        registry,
        operator_info(
            OperatorMeta { id: "bim.element.slab", name: "Slab", abbreviation: "Slab", icon: "emoji:⬜️", summary: "Defines a slab element" },
            vec![number_channel("width", "bim.element.slab", 10.0), number_channel("depth", "bim.element.slab", 8.0), number_channel("thickness", "bim.element.slab", 0.25)],
            out_slab(),
            &["Elements"],
        ),
        SlabElement,
        "slab",
    );
    register_element(
        registry,
        operator_info(
            OperatorMeta { id: "bim.element.column", name: "Column", abbreviation: "Col", icon: "emoji:🏛️", summary: "Defines a column element" },
            vec![number_channel("width", "bim.element.column", 0.4), number_channel("depth", "bim.element.column", 0.4), number_channel("height", "bim.element.column", 3.0)],
            out_column(),
            &["Elements"],
        ),
        ColumnElement,
        "column",
    );
    register_element(
        registry,
        operator_info(
            OperatorMeta { id: "bim.element.window", name: "Window", abbreviation: "Win", icon: "emoji:🪟️", summary: "Defines a window element" },
            vec![number_channel("width", "bim.element.window", 1.2), number_channel("height", "bim.element.window", 1.4), number_channel("sill", "bim.element.window", 0.9)],
            out_window(),
            &["Elements"],
        ),
        WindowElement,
        "window",
    );

    registry.register_operator(
        OperatorInfo {
            variadic_input: Some(VariadicSpec { slot_key: "elements".into(), min: 0, max: None }),
            ..operator_info(
                OperatorMeta { id: "bim.assemble.story", name: "Assemble Story", abbreviation: "Story", icon: "emoji:🏢️", summary: "Assembles a story from elements and optional slab" },
                vec![number_channel("elevation", "bim.assemble.story", 0.0), number_channel("height", "bim.assemble.story", 3.0), ChannelSpec::requires("slab", &["bim.element.slab"])],
                out_story(),
                &["Assembly"],
            )
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(AssembleStory) }],
        &["story"],
    );
    registry.register_operator(
        OperatorInfo {
            variadic_input: Some(VariadicSpec { slot_key: "stories".into(), min: 1, max: None }),
            ..operator_info(
                OperatorMeta { id: "bim.assemble.building", name: "Assemble Building", abbreviation: "Building", icon: "emoji:🏗️", summary: "Assembles a building from stories" },
                vec![text_channel("name", "bim.assemble.building", "Building")],
                out_building(),
                &["Assembly"],
            )
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(AssembleBuilding) }],
        &["building"],
    );

    registry.register_operator(
        operator_info(
            OperatorMeta { id: "bim.measure.floorArea", name: "Floor Area", abbreviation: "Area", icon: "emoji:📐️", summary: "Total floor area across all stories" },
            vec![ChannelSpec::requires("building", &["bim.assemble.building"])],
            out_floor_area(),
            &["Measure"],
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(FloorArea) }],
        &["number"],
    );
    registry.register_operator(
        operator_info(
            OperatorMeta { id: "bim.measure.grossVolume", name: "Gross Volume", abbreviation: "Vol", icon: "emoji:📦️", summary: "Gross building volume across all stories" },
            vec![ChannelSpec::requires("building", &["bim.assemble.building"])],
            out_gross_volume(),
            &["Measure"],
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(GrossVolume) }],
        &["number"],
    );

    registry.finalize();
}

// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::build_manifest_json;
    build_manifest_json("bim", "Bim", env!("CARGO_PKG_VERSION"), &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![], vec![])
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::module_registry;
    use flow_extension_sdk::{build_manifest_json, evaluate_invoke_json, flow_extension_topic_contribution};
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "bim";
    const EXTENSION_LABEL: &str = "Bim";

    // 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires
    // a plain sync fn). `.mode`/`.contributes_topic`/`.handler` are still `async fn` in
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's
    // path_scope); bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request.
    // See R9.
    fn bundle() -> ExtensionBundle {
        let manifest_json = build_manifest_json("bim", "Bim", env!("CARGO_PKG_VERSION"), &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![], vec![]);
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "bim", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "bim", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-bim", "Bim", env!("CARGO_PKG_VERSION")).extends("flow").depends_on("flow", semio_framework::tree_pin!());
        let bundle = bundle.mode(ExecutionMode::Linked);
        let bundle = bundle.contributes_topic(flow_topic.topic, flow_topic.payload);
        let bundle = bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload);
        bundle.handler("evaluate", |req| evaluate_invoke_json(&neural_engine::ColdOwner::new(module_registry()), req).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err)))
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️extension-guest-standalone/🦀️.rs");

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
````

## ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs

Bytes 40183; SHA-256 ef5a898caa3c5ed33b64365cbdc7cc06ae92925e8510a5b6cd7fdf6785063b77.

````text
//! 🥽️ Indexed mesh widgets with explicit polygon data and B-Rep preview conversion.
use super::*;
use neural_engine::{Atom, FieldSpec, Schema, ValueType};
use semio_framework_3d::mesh::{EdgeId, FaceId, HalfedgeMesh, MeshKernelError, Vec3 as MeshVector, VertexId, WeldMode, MirrorAxis, MeshModelingJob, MeshModelingStep, MeshModelingProgress};
use std::collections::{HashMap, HashSet};

const LIMIT: usize = 100_000;
fn invalid(message: impl Into<String>) -> EvalError { EvalError::InvalidInput(message.into()) }
fn mesh_error(error: MeshKernelError) -> EvalError { invalid(error.to_string()) }
fn scalar(input: &Dictionary, name: &str) -> Result<f32, EvalError> {
    let number = read_channel_number(input, name)?;
    if !number.is_finite() || number.abs() > f32::MAX as f64 { return Err(invalid(format!("{name} must be finite"))); }
    Ok(number as f32)
}
fn positive(input: &Dictionary, name: &str) -> Result<f32, EvalError> {
    let number = scalar(input, name)?;
    if number <= 0.0 { return Err(invalid(format!("{name} must be positive"))); }
    Ok(number)
}
fn count(input: &Dictionary, name: &str, min: u32, max: u32) -> Result<u32, EvalError> {
    let number = read_channel_number(input, name)?;
    if !number.is_finite() || number.fract() != 0.0 || number < min as f64 || number > max as f64 { return Err(invalid(format!("{name} must be an integer in {min}..={max}"))); }
    Ok(number as u32)
}
fn vector(input: &Dictionary, name: &str) -> Result<MeshVector, EvalError> {
    let value = read_xyz(input, name)?;
    if value.iter().any(|number| !number.is_finite() || number.abs() > f32::MAX as f64) { return Err(invalid(format!("{name} must be finite"))); }
    Ok(MeshVector(value.map(|number| number as f32)))
}
fn selection(input: &Dictionary, name: &str, bound: usize) -> Result<Vec<u32>, EvalError> {
    let text = read_text(input, name)?;
    if text.len() > 16_000_000 { return Err(invalid("selection exceeds 16 MB")); }
    let json = pack::json::parse(&text).map_err(|error| invalid(error.to_string()))?;
    let entries = json.as_array().ok_or_else(|| invalid("selection must be an array of indices"))?;
    if entries.is_empty() || entries.len() > LIMIT * 6 { return Err(invalid("select 1..600000 elements")); }
    let mut seen = HashSet::new();
    entries.iter().map(|entry| {
        let id = entry.as_u64().filter(|id| *id < bound as u64).ok_or_else(|| invalid("selection index out of range"))? as u32;
        if !seen.insert(id) && name != "edges" && name != "selection" { return Err(invalid("duplicate selection index")); }
        Ok(id)
    }).collect()
}
fn component_vertices(input: &Dictionary, mesh: &HalfedgeMesh) -> Result<Vec<VertexId>, EvalError> {
    let mode = read_text(input, "mode")?;
    let bound = match mode.as_str() {
        "vertex" => mesh.vertex_count(), "edge" => mesh.halfedge_count(), "face" => mesh.face_count(),
        _ => return Err(invalid("component mode must be vertex, edge, or face")),
    };
    let mut vertices = Vec::new();
    for id in selection(input, "selection", bound)? {
        match mode.as_str() {
            "vertex" => vertices.push(VertexId(id)),
            "face" => vertices.extend(mesh.face_vertex_ids(FaceId(id)).map_err(mesh_error)?),
            _ => { let (a, b) = mesh.edge_endpoints(EdgeId(id)).map_err(mesh_error)?; vertices.extend([a, b]); }
        }
    }
    vertices.sort_unstable_by_key(|vertex| vertex.0);
    vertices.dedup();
    Ok(vertices)
}
fn component_pivot(input: &Dictionary, mesh: &HalfedgeMesh, vertices: &[VertexId]) -> Result<MeshVector, EvalError> {
    match read_text(input, "pivot")?.as_str() {
        "point" => vector(input, "center"),
        "selection" => {
            let mut sum = [0.0f64; 3];
            for &vertex in vertices {
                let point = mesh.vertex_position(vertex).map_err(mesh_error)?;
                for axis in 0..3 { sum[axis] += point.0[axis] as f64; }
            }
            Ok(MeshVector(sum.map(|value| (value / vertices.len() as f64) as f32)))
        }
        _ => Err(invalid("pivot must be selection or point")),
    }
}
fn decode_mesh(text: &str) -> Result<HalfedgeMesh, EvalError> {
    if text.len() > 16_000_000 { return Err(invalid("mesh input exceeds 16 MB")); }
    let json = pack::json::parse(text).map_err(|error| invalid(error.to_string()))?;
    if json.as_object().is_none_or(|object| object.iter().any(|(key, _)| key != "vertices" && key != "faces")) { return Err(invalid("unknown mesh field")); }
    let vertices = json.get("vertices").and_then(|v| v.as_array()).ok_or_else(|| invalid("vertices must be an array"))?;
    let faces = json.get("faces").and_then(|v| v.as_array()).ok_or_else(|| invalid("faces must be an array"))?;
    if vertices.len() < 3 || vertices.len() > LIMIT || faces.is_empty() || faces.len() > LIMIT { return Err(invalid("mesh requires 3..100000 vertices and 1..100000 faces")); }
    let positions: Vec<[f32; 3]> = vertices.iter().map(|vertex| {
        let point = vertex.as_array().filter(|point| point.len() == 3).ok_or_else(|| invalid("vertex must have three coordinates"))?;
        let mut result = [0.0; 3];
        for axis in 0..3 {
            let number = point[axis].as_f64().filter(|number| number.is_finite() && number.abs() <= f32::MAX as f64).ok_or_else(|| invalid("coordinate must be finite"))?;
            result[axis] = number as f32;
        }
        Ok(result)
    }).collect::<Result<_, EvalError>>()?;
    let mut corner_count = 0;
    let polygons: Vec<Vec<u32>> = faces.iter().map(|face| {
        let indices = face.as_array().filter(|indices| indices.len() >= 3).ok_or_else(|| invalid("face requires at least three indices"))?;
        corner_count += indices.len();
        if corner_count > LIMIT * 6 { return Err(invalid("mesh exceeds 600000 polygon corners")); }
        let mut seen = HashSet::new();
        indices.iter().map(|index| {
            let id = index.as_u64().filter(|id| *id < positions.len() as u64).ok_or_else(|| invalid("face index out of range"))? as u32;
            if !seen.insert(id) { return Err(invalid("face contains a repeated vertex")); }
            Ok(id)
        }).collect()
    }).collect::<Result<_, EvalError>>()?;
    HalfedgeMesh::from_faces(&positions, &polygons).map_err(mesh_error)
}
struct PolygonData { vertices:Vec<[f32;3]>, faces:Vec<Vec<u32>> }
impl PolygonData {
    fn from_mesh(mesh:&HalfedgeMesh) -> Result<Self,EvalError> {
        let vertices = (0..mesh.vertex_count()).map(|id| mesh.vertex_position(VertexId(id as u32)).map(|point| point.0)).collect::<Result<Vec<_>,_>>().map_err(mesh_error)?;
        let faces = (0..mesh.face_count()).map(|id| mesh.face_vertex_ids(FaceId(id as u32)).map(|vertices| vertices.into_iter().map(|vertex| vertex.0).collect())).collect::<Result<Vec<_>,_>>().map_err(mesh_error)?;
        Ok(Self { vertices,faces })
    }
    fn encode(&self) -> String {
        let vertices = self.vertices.iter().map(|point| pack::json::array(point.iter().map(|number| pack::json::Value::from(*number as f64))));
        let faces = self.faces.iter().map(|vertices| pack::json::array(vertices.iter().map(|vertex| pack::json::Value::from(*vertex))));
        pack::json::to_string(&pack::json::object([("vertices".into(),pack::json::array(vertices)),("faces".into(),pack::json::array(faces))]))
    }
    fn mesh(&self) -> Result<HalfedgeMesh,EvalError> { HalfedgeMesh::from_faces(&self.vertices,&self.faces).map_err(mesh_error) }
}
fn encode_mesh(mesh: &HalfedgeMesh) -> Result<String, EvalError> { Ok(PolygonData::from_mesh(mesh)?.encode()) }
fn indexed_triangle_mesh(positions: &[f32], indices: &[u32]) -> Result<HalfedgeMesh, EvalError> {
    if positions.len() % 3 != 0 || indices.len() % 3 != 0 { return Err(invalid("invalid triangulation buffers")); }
    let mut unique = HashMap::new();
    let mut vertices = Vec::new();
    let mut remap = Vec::new();
    for point in positions.chunks_exact(3) {
        let key = point.iter().map(|number| if *number == 0.0 { 0 } else { number.to_bits() }).collect::<Vec<_>>();
        let id = *unique.entry(key).or_insert_with(|| { vertices.push([point[0], point[1], point[2]]); vertices.len() as u32 - 1 });
        remap.push(id);
    }
    let faces = indices.chunks_exact(3).map(|triangle| triangle.iter().map(|id| remap.get(*id as usize).copied().ok_or_else(|| invalid("triangulation index out of range"))).collect()).collect::<Result<Vec<Vec<u32>>, EvalError>>()?;
    HalfedgeMesh::from_faces(&vertices, &faces).map_err(mesh_error)
}
fn read_mesh(input: &Dictionary, name: &str) -> Result<HalfedgeMesh, EvalError> {
    let mesh = input.get(name).and_then(|value| value.as_dictionary()).filter(|mesh| mesh.schema() == Some("mesh")).ok_or_else(|| invalid(format!("{name} requires a mesh")))?;
    let data = mesh.get("data").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).ok_or_else(|| invalid("mesh data is missing"))?;
    decode_mesh(data)
}
fn mesh_output(mesh: &HalfedgeMesh) -> Result<Dictionary, EvalError> {
    let polygons = PolygonData::from_mesh(mesh)?;
    let data = polygons.encode();
    let transfer = polygons.mesh()?.tessellate().map_err(mesh_error)?;
    let preview = semio_framework_mesh_engine::MeshData { positions: transfer.positions, normals: transfer.normals, indices: transfer.indices, face_ids: transfer.face_ids, vertex_ids: transfer.vertex_ids, edge_positions: transfer.edge_positions, edge_ids: transfer.edge_ids, uvs: transfer.uvs, edge_uvs: transfer.edge_uvs, edge_is_seam: transfer.edge_is_seam, ..Default::default() };
    if preview.positions.iter().any(|number| !number.is_finite()) { return Err(invalid("mesh operation produced non-finite coordinates")); }
    let preview = encode_base64(&encode_mesh_pack(&preview).map_err(invalid)?);
    let mesh = Dictionary::with_schema("mesh").insert("data", Value::Atom(Atom::String(data))).insert("preview", Value::Atom(Atom::String(preview)));
    Ok(channel_output("meshOut", mesh))
}
fn analyze(mesh: &HalfedgeMesh) -> Result<Dictionary, EvalError> {
    let mut edges = HashMap::<(u32, u32), (usize, i32)>::new();
    for face in 0..mesh.face_count() {
        let vertices = mesh.face_vertex_ids(FaceId(face as u32)).map_err(mesh_error)?;
        for i in 0..vertices.len() {
            let a = vertices[i].0;
            let b = vertices[(i + 1) % vertices.len()].0;
            let entry = edges.entry((a.min(b), a.max(b))).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
        }
    }
    let boundary = edges.values().filter(|edge| edge.0 == 1).count();
    let non_manifold = edges.values().filter(|edge| edge.0 > 2).count();
    let inconsistent = edges.values().filter(|edge| edge.0 == 2 && edge.1 != 0).count();
    let triangles = mesh.tessellate().map_err(mesh_error)?;
    let mut area = 0.0f64;
    let mut volume = 0.0f64;
    let reference = mesh.vertex_position(VertexId(0)).map_err(mesh_error)?.0.map(f64::from);
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cross = |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for id in 0..mesh.vertex_count() {
        let point = mesh.vertex_position(VertexId(id as u32)).map_err(mesh_error)?.0;
        for axis in 0..3 { min[axis] = min[axis].min(point[axis]); max[axis] = max[axis].max(point[axis]); }
    }
    let mut degenerate = 0;
    for triangle in triangles.indices.chunks_exact(3) {
        let point = |index: u32| [triangles.positions[index as usize * 3] as f64, triangles.positions[index as usize * 3 + 1] as f64, triangles.positions[index as usize * 3 + 2] as f64];
        let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
        let normal = cross(sub(b, a), sub(c, a));
        let triangle_area = normal[0].hypot(normal[1]).hypot(normal[2]) / 2.0;
        area += triangle_area;
        if triangle_area == 0.0 { degenerate += 1; }
        let relative = sub(a, reference);
        let normal = cross(sub(b, reference), sub(c, reference));
        volume += relative.iter().zip(normal).map(|(a, b)| a * b).sum::<f64>() / 6.0;
    }
    let mut report = Dictionary::new();
    for (key, value) in [("vertices", mesh.vertex_count() as f64), ("faces", mesh.face_count() as f64), ("edges", edges.len() as f64), ("triangles", triangles.indices.len() as f64 / 3.0), ("boundaryEdges", boundary as f64), ("nonManifoldEdges", non_manifold as f64), ("inconsistentEdges", inconsistent as f64), ("degenerateTriangles", degenerate as f64), ("area", area)] {
        report = report.insert(key, Value::Dictionary(number_dictionary(value)));
    }
    if boundary == 0 && non_manifold == 0 && inconsistent == 0 && degenerate == 0 { report = report.insert("volume", Value::Dictionary(number_dictionary(volume.abs()))); }
    Ok(report.insert("minimum", Value::Dictionary(point_dictionary(min.map(f64::from)))).insert("maximum", Value::Dictionary(point_dictionary(max.map(f64::from)))))
}

fn operator_progress(progress: MeshModelingProgress) -> neural_engine::OperatorProgress {
    neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase }
}

struct MeshOperatorJob {
    job: Option<MeshModelingJob>,
    output: Option<HalfedgeMesh>,
    progress: neural_engine::OperatorProgress,
    cancelled: bool,
}

impl neural_engine::OperatorJob for MeshOperatorJob {
    fn step(&mut self, budget: usize) -> Result<neural_engine::OperatorJobStep, EvalError> {
        if self.cancelled { return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress)); }
        if let Some(mesh) = &self.output { return mesh_output(mesh).map(neural_engine::OperatorJobStep::Done); }
        let job = self.job.as_mut().ok_or_else(|| invalid("mesh job is missing"))?;
        match job.step(budget).map_err(mesh_error)? {
            MeshModelingStep::Working(progress) => {
                self.progress = operator_progress(progress);
                Ok(neural_engine::OperatorJobStep::Working(self.progress))
            }
            MeshModelingStep::Cancelled(progress) => {
                self.progress = operator_progress(progress); self.cancelled = true; self.job = None;
                Ok(neural_engine::OperatorJobStep::Cancelled(self.progress))
            }
            MeshModelingStep::Done(mesh) => {
                self.progress = operator_progress(job.progress());
                self.job = None; self.output = Some(mesh);
                mesh_output(self.output.as_ref().unwrap()).map(neural_engine::OperatorJobStep::Done)
            }
        }
    }
    fn progress(&self) -> neural_engine::OperatorProgress { self.progress }
    fn cancel(&mut self) {
        if self.output.is_some() || self.cancelled { return; }
        if let Some(job) = &mut self.job { job.cancel(); self.progress = operator_progress(job.progress()); }
        self.cancelled = true; self.job = None;
    }
}

struct MeshOperation(&'static str, SessionCapture);
impl Operator for MeshOperation {
    retire_geometry_capture!(1);
    fn step_plan(&self, input: &Dictionary) -> Result<Option<Box<dyn neural_engine::OperatorJob>>, EvalError> {
        if !matches!(self.0, "bevel" | "decimate") { return Ok(None); }
        let mesh = read_mesh(input, "mesh")?;
        let job = if self.0 == "bevel" {
            let edges = selection(input, "edges", mesh.halfedge_count())?.into_iter().map(EdgeId).collect::<Vec<_>>();
            mesh.bevel_job(&edges, positive(input, "amount")?, count(input, "segments", 1, 64)?).map_err(mesh_error)?
        } else {
            let ratio = positive(input, "ratio")?;
            if ratio > 1.0 { return Err(invalid("decimation ratio must be at most one")); }
            mesh.decimate_job(ratio).map_err(mesh_error)?
        };
        let progress = operator_progress(job.progress());
        Ok(Some(Box::new(MeshOperatorJob { job: Some(job), output: None, progress, cancelled: false })))
    }
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let mut mesh = match self.0 {
            "construct" => decode_mesh(&read_text(input, "data")?)?,
            "box" => HalfedgeMesh::box_prim(positive(input, "width")?, positive(input, "height")?, positive(input, "depth")?).map_err(mesh_error)?,
            "plane" => HalfedgeMesh::plane_prim(positive(input, "width")?, positive(input, "depth")?).map_err(mesh_error)?,
            "sphere" => HalfedgeMesh::ico_sphere_prim(positive(input, "radius")?, count(input, "subdivisions", 0, 5)?).map_err(mesh_error)?,
            "cylinder" => HalfedgeMesh::cylinder_prim(positive(input, "radius")?, positive(input, "height")?, count(input, "segments", 3, 1024)?).map_err(mesh_error)?,
            "cone" => HalfedgeMesh::cone_prim(positive(input, "radius")?, positive(input, "height")?, count(input, "segments", 3, 1024)?).map_err(mesh_error)?,
            "fromBrep" => self.1.with_kernel_read(|kernel| {
                let transfer = kernel.tessellate(&read_geometry(input, "geometry")?, positive(input, "deflection")? as f64).map_err(|error| map_kernel_error(&error))?;
                indexed_triangle_mesh(&transfer.position, &transfer.index)
            })?,
            _ => read_mesh(input, "mesh")?,
        };
        match self.0 {
            "translate" => mesh.translate(vector(input, "offset")?).map_err(mesh_error)?,
            "rotate" => {
                let axis = vector(input, "axis")?;
                if axis.0.iter().all(|coordinate| *coordinate == 0.0) { return Err(invalid("rotation axis cannot be zero")); }
                mesh.rotate(axis, scalar(input, "angle")?).map_err(mesh_error)?;
            }
            "scale" => {
                let factors = vector(input, "factor")?;
                if factors.0.iter().any(|value| *value == 0.0) { return Err(invalid("scale factors cannot be zero")); }
                mesh.scale(factors).map_err(mesh_error)?;
                if factors.0.iter().filter(|value| **value < 0.0).count() % 2 == 1 { mesh.flip_faces(&(0..mesh.face_count()).map(|id| FaceId(id as u32)).collect::<Vec<_>>()).map_err(mesh_error)?; }
            }
            "translateComponents" | "rotateComponents" | "scaleComponents" => {
                let ids = component_vertices(input, &mesh)?;
                match self.0 {
                    "translateComponents" => mesh.move_vertices(&ids, vector(input, "offset")?).map_err(mesh_error)?,
                    "rotateComponents" => {
                        let pivot = component_pivot(input, &mesh, &ids)?;
                        mesh.rotate_vertices(&ids, vector(input, "axis")?, scalar(input, "angle")?, pivot).map_err(mesh_error)?;
                    }
                    _ => {
                        let pivot = component_pivot(input, &mesh, &ids)?;
                        mesh.scale_vertices(&ids, vector(input, "factor")?, pivot).map_err(mesh_error)?;
                    }
                }
            }
            "moveVertices" => {
                let ids = selection(input, "vertices", mesh.vertex_count())?.into_iter().map(VertexId).collect::<Vec<_>>();
                mesh.move_vertices(&ids, vector(input, "offset")?).map_err(mesh_error)?;
            }
            "bevel" | "dissolveEdges" => {
                let ids = selection(input, "edges", mesh.halfedge_count())?.into_iter().map(EdgeId).collect::<Vec<_>>();
                if self.0 == "bevel" { mesh.bevel_edges(&ids, positive(input, "amount")?, count(input, "segments", 1, 64)?).map_err(mesh_error)?; }
                else { mesh.dissolve_edges(&ids).map_err(mesh_error)?; }
            }
            "moveProportional" | "snapVertices" | "mergeVertices" | "dissolveVertices" => {
                let ids = selection(input, "selection", mesh.vertex_count())?.into_iter().map(VertexId).collect::<Vec<_>>();
                match self.0 {
                    "moveProportional" => mesh.move_vertices_proportional(&ids, vector(input, "offset")?, vector(input, "center")?, positive(input, "radius")?).map_err(mesh_error)?,
                    "snapVertices" => mesh.snap_vertices_to_grid(&ids, positive(input, "grid")?).map_err(mesh_error)?,
                    "dissolveVertices" => mesh.dissolve_vertices(&ids).map_err(mesh_error)?,
                    _ => {
                        let mode = match read_text(input, "mode")?.as_str() { "first" => WeldMode::First, "center" => WeldMode::Center, "distance" => WeldMode::ByDistance, _ => return Err(invalid("merge mode must be first, center, or distance")) };
                        let tolerance = scalar(input, "tolerance")?;
                        if tolerance < 0.0 { return Err(invalid("merge tolerance must be nonnegative")); }
                        mesh.merge_vertices(&ids, mode, tolerance).map_err(mesh_error)?;
                    }
                }
            }
            "mirror" => {
                let axis = match read_text(input, "axis")?.as_str() { "x" => MirrorAxis::X, "y" => MirrorAxis::Y, "z" => MirrorAxis::Z, _ => return Err(invalid("mirror axis must be x, y, or z")) };
                let tolerance = scalar(input, "tolerance")?;
                if tolerance < 0.0 { return Err(invalid("mirror tolerance must be nonnegative")); }
                mesh.mirror(axis, tolerance).map_err(mesh_error)?;
            }
            "decimate" => {
                let ratio = positive(input, "ratio")?;
                if ratio > 1.0 { return Err(invalid("decimation ratio must be at most one")); }
                mesh.decimate(ratio).map_err(mesh_error)?;
            }
            "mergeCoplanar" => { mesh.merge_coplanar_faces().map_err(mesh_error)?; }
            "loopCut" => {
                let ids = selection(input, "edges", mesh.halfedge_count())?.into_iter().map(EdgeId).collect::<Vec<_>>();
                mesh.loop_cut(&ids, count(input, "cuts", 1, 256)?).map_err(mesh_error)?;
            }
            "knifeCut" => mesh.knife_cut(FaceId(count(input, "face", 0, mesh.face_count().saturating_sub(1) as u32)?), vector(input, "start")?, vector(input, "end")?).map_err(mesh_error)?,
            "extrude" | "inset" | "subdivide" | "flip" | "deleteFaces" => {
                let ids = selection(input, "faces", mesh.face_count())?.into_iter().map(FaceId).collect::<Vec<_>>();
                match self.0 {
                    "extrude" => mesh.extrude_faces(&ids, scalar(input, "distance")?).map_err(mesh_error)?,
                    "inset" => mesh.inset_faces(&ids, positive(input, "amount")?).map_err(mesh_error)?,
                    "subdivide" => mesh.subdivide_faces(&ids).map_err(mesh_error)?,
                    "flip" => mesh.flip_faces(&ids).map_err(mesh_error)?,
                    _ => {
                        let positions = (0..mesh.vertex_count()).map(|id| mesh.vertex_position(VertexId(id as u32)).map(|point| point.0)).collect::<Result<Vec<_>, _>>().map_err(mesh_error)?;
                        let faces = (0..mesh.face_count()).filter(|id| !ids.contains(&FaceId(*id as u32))).map(|id| mesh.face_vertex_ids(FaceId(id as u32)).map(|vertices| vertices.into_iter().map(|id| id.0).collect())).collect::<Result<Vec<Vec<u32>>, _>>().map_err(mesh_error)?;
                        if faces.is_empty() { return Err(invalid("deletion would leave an empty mesh")); }
                        mesh = HalfedgeMesh::from_faces(&positions, &faces).map_err(mesh_error)?;
                    }
                }
            }
            "triangulate" => mesh.triangulate().map_err(mesh_error)?,
            "weld" => { mesh.weld_coincident_vertices(positive(input, "tolerance")?).map_err(mesh_error)?; }
            "orient" => { mesh.orient_faces_consistently().map_err(mesh_error)?; }
            "fillHoles" => { mesh.fill_holes().map_err(mesh_error)?; }
            "inspectVertex" => {
                let id = VertexId(count(input, "index", 0, mesh.vertex_count().saturating_sub(1) as u32)?);
                return Ok(channel_output("point", point_dictionary(mesh.vertex_position(id).map_err(mesh_error)?.0.map(f64::from))));
            }
            "inspectEdge" => {
                let id = EdgeId(count(input, "index", 0, mesh.halfedge_count().saturating_sub(1) as u32)?);
                let (a, b) = mesh.edge_endpoints(id).map_err(mesh_error)?;
                let start = mesh.vertex_position(a).map_err(mesh_error)?.0.map(f64::from);
                let end = mesh.vertex_position(b).map_err(mesh_error)?.0.map(f64::from);
                let length = (end[0] - start[0]).hypot(end[1] - start[1]).hypot(end[2] - start[2]);
                return Ok(Dictionary::new().insert("start", Value::Dictionary(point_dictionary(start))).insert("end", Value::Dictionary(point_dictionary(end))).insert("length", Value::Dictionary(number_dictionary(length))));
            }
            "inspectFace" => {
                let id = FaceId(count(input, "index", 0, mesh.face_count().saturating_sub(1) as u32)?);
                let ids = mesh.face_vertex_ids(id).map_err(mesh_error)?;
                let normal = mesh.face_normal(id).map_err(mesh_error)?.0.map(f64::from);
                if normal.iter().all(|value| *value == 0.0) { return Err(invalid("face normal is degenerate")); }
                let mut center = [0.0; 3];
                for &vertex in &ids {
                    let point = mesh.vertex_position(vertex).map_err(mesh_error)?.0;
                    for axis in 0..3 { center[axis] += point[axis] as f64 / ids.len() as f64; }
                }
                let vertices = pack::json::to_string(&pack::json::array(ids.iter().map(|id| pack::json::Value::from(id.0))));
                return Ok(Dictionary::new().insert("vertices", Value::Dictionary(text_dictionary(vertices))).insert("normal", Value::Dictionary(vector_dictionary(normal))).insert("center", Value::Dictionary(point_dictionary(center))));
            }
            "analyze" => return analyze(&mesh),
            "exportObj" => return Ok(channel_output("text", text_dictionary(mesh.to_obj().map_err(mesh_error)?))),
            "exportJson" => return Ok(channel_output("text", text_dictionary(encode_mesh(&mesh)?))),
            "toBrep" => return self.1.with_kernel(|kernel| {
                let handle = kernel.import_obj(&mesh.to_obj().map_err(mesh_error)?, positive(input, "tolerance")? as f64).map_err(|error| map_kernel_error(&error))?;
                Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
            }),
            _ => {}
        }
        if mesh.vertex_count() > LIMIT || mesh.face_count() > LIMIT { return Err(invalid("result exceeds mesh capacity")); }
        mesh_output(&mesh)
    }
}

pub(super) fn register_mesh(registry: &mut Registry, session: &Session) {
    registry.register_schema(Schema { id: "mesh".into(), module: "brep".into(), name: "Polygon Mesh".into(), icon: "emoji:🥽️".into(), summary: "Indexed polygon vertices and faces".into(), fields: vec![FieldSpec::new("data", ValueType::Text), FieldSpec::new("preview", ValueType::Text)] });
    let definitions: &[(&str, &str, &str, &[(&str, f64)])] = &[
        ("construct", "Construct Mesh", "Mesh Creation", &[]),
        ("box", "Mesh Box", "Mesh Creation", &[("width", 1.0), ("height", 1.0), ("depth", 1.0)]),
        ("plane", "Mesh Plane", "Mesh Creation", &[("width", 1.0), ("depth", 1.0)]),
        ("sphere", "Mesh Sphere", "Mesh Creation", &[("radius", 1.0), ("subdivisions", 2.0)]),
        ("cylinder", "Mesh Cylinder", "Mesh Creation", &[("radius", 1.0), ("height", 1.0), ("segments", 32.0)]),
        ("cone", "Mesh Cone", "Mesh Creation", &[("radius", 1.0), ("height", 1.0), ("segments", 32.0)]),
        ("fromBrep", "Mesh from B-Rep", "Mesh Conversion", &[("deflection", 0.1)]),
        ("toBrep", "Faceted B-Rep from Mesh", "Mesh Conversion", &[("tolerance", 0.001)]),
        ("translate", "Translate Mesh", "Mesh Editing", &[]),
        ("rotate", "Rotate Mesh", "Mesh Editing", &[("angle", 0.0)]),
        ("scale", "Scale Mesh", "Mesh Editing", &[]),
        ("moveVertices", "Move Mesh Vertices", "Mesh Editing", &[]),
        ("translateComponents", "Move Mesh Components", "Mesh Editing", &[]),
        ("rotateComponents", "Rotate Mesh Components", "Mesh Editing", &[("angle", 0.0)]),
        ("scaleComponents", "Scale Mesh Components", "Mesh Editing", &[]),
        ("bevel", "Bevel Mesh Edges", "Mesh Editing", &[("amount", 0.1), ("segments", 1.0)]),
        ("dissolveEdges", "Dissolve Mesh Edges", "Mesh Editing", &[]),
        ("dissolveVertices", "Dissolve Mesh Vertices", "Mesh Editing", &[]),
        ("mergeVertices", "Merge Mesh Vertices", "Mesh Editing", &[("tolerance", 0.0001)]),
        ("moveProportional", "Move Mesh Proportionally", "Mesh Editing", &[("radius", 1.0)]),
        ("snapVertices", "Snap Mesh Vertices to Grid", "Mesh Editing", &[("grid", 1.0)]),
        ("mirror", "Mirror Mesh Half", "Mesh Editing", &[("tolerance", 0.0001)]),
        ("decimate", "Simplify Mesh", "Mesh Editing", &[("ratio", 0.5)]),
        ("mergeCoplanar", "Merge Coplanar Mesh Faces", "Mesh Repair", &[]),
        ("loopCut", "Cut Mesh Loops", "Mesh Editing", &[("cuts", 1.0)]),
        ("knifeCut", "Knife Cut Mesh Face", "Mesh Editing", &[("face", 0.0)]),
        ("extrude", "Extrude Mesh Faces", "Mesh Editing", &[("distance", 1.0)]),
        ("inset", "Inset Mesh Faces", "Mesh Editing", &[("amount", 0.1)]),
        ("subdivide", "Subdivide Mesh Faces", "Mesh Editing", &[]),
        ("flip", "Flip Mesh Faces", "Mesh Editing", &[]),
        ("deleteFaces", "Delete Mesh Faces", "Mesh Editing", &[]),
        ("triangulate", "Triangulate Mesh", "Mesh Editing", &[]),
        ("weld", "Weld Mesh Vertices", "Mesh Repair", &[("tolerance", 0.0001)]),
        ("orient", "Orient Mesh Faces", "Mesh Repair", &[]),
        ("fillHoles", "Fill Mesh Holes", "Mesh Repair", &[]),
        ("inspectVertex", "Inspect Mesh Vertex", "Mesh Analysis", &[("index", 0.0)]),
        ("inspectEdge", "Inspect Mesh Edge", "Mesh Analysis", &[("index", 0.0)]),
        ("inspectFace", "Inspect Mesh Face", "Mesh Analysis", &[("index", 0.0)]),
        ("analyze", "Analyze Mesh", "Mesh Analysis", &[]),
        ("exportObj", "Mesh to OBJ", "Mesh Interchange", &[]),
        ("exportJson", "Mesh to JSON", "Mesh Interchange", &[]),
    ];
    for &(operation, name, group, parameters) in definitions {
        let id = format!("brep.mesh.{operation}");
        let mut inputs = Vec::new();
        if !matches!(operation, "construct" | "box" | "plane" | "sphere" | "cylinder" | "cone" | "fromBrep") { inputs.push(ChannelSpec::requires("mesh", &[&id]).with_value_types(&["mesh"])); }
        if operation == "construct" { inputs.push(ChannelSpec::requires("data", &[&id]).with_value_types(&["text"])); }
        if operation == "fromBrep" { inputs.push(geometry_channel("geometry", &id)); }
        if matches!(operation, "extrude" | "inset" | "subdivide" | "flip" | "deleteFaces" | "moveVertices" | "loopCut" | "bevel" | "dissolveEdges") {
            inputs.push(ChannelSpec::requires(match operation { "moveVertices" => "vertices", "loopCut" | "bevel" | "dissolveEdges" => "edges", _ => "faces" }, &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("[0]"))));
        }
        if matches!(operation, "moveProportional" | "snapVertices" | "mergeVertices" | "dissolveVertices") {
            inputs.push(ChannelSpec::requires("selection", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary(if operation == "mergeVertices" { "[0,1]" } else { "[0]" }))));
        }
        if operation == "moveProportional" { inputs.push(vector_channel("center", &id, [0.0; 3])); }
        if operation == "mergeVertices" { inputs.push(ChannelSpec::requires("mode", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("center")))); }
        if operation == "mirror" { inputs.push(ChannelSpec::requires("axis", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("x")))); }
        if operation.ends_with("Components") {
            for (key, value) in [("mode", "vertex"), ("selection", "[0]")] { inputs.push(ChannelSpec::requires(key, &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary(value)))); }
            if operation != "translateComponents" {
                inputs.push(ChannelSpec::requires("pivot", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("selection"))));
                inputs.push(ChannelSpec::requires("center", &[&id]).with_value_types(&["point", "vector"]).with_default(Value::Dictionary(point_dictionary([0.0; 3]))));
            }
        }
        if operation == "knifeCut" {
            for (key, point) in [("start", [0.0, -1.0, 0.0]), ("end", [0.0, 1.0, 0.0])] {
                inputs.push(ChannelSpec::requires(key, &[&id]).with_value_types(&["point", "vector"]).with_default(Value::Dictionary(point_dictionary(point))));
            }
        }
        match operation {
            "translate" | "moveVertices" | "translateComponents" | "moveProportional" => inputs.push(vector_channel("offset", &id, [0.0, 0.0, 1.0])),
            "rotate" | "rotateComponents" => inputs.push(vector_channel("axis", &id, [0.0, 0.0, 1.0])),
            "scale" | "scaleComponents" => inputs.push(vector_channel("factor", &id, [1.0, 1.0, 1.0])),
            _ => {}
        }
        inputs.extend(parameters.iter().map(|(key, value)| number_channel(key, &id, *value)));
        let (outputs, produced) = match operation {
            "analyze" => {
                let mut channels: Vec<_> = ["vertices", "faces", "edges", "triangles", "boundaryEdges", "nonManifoldEdges", "inconsistentEdges", "degenerateTriangles", "area", "volume"].iter().map(|&key| ChannelSpec::named(key, key, key, key).with_value_types(&["number"])).collect();
                for channel in &mut channels { if channel.name == "volume" { channel.cardinality = neural_engine::Cardinality::ZeroOrOne; } }
                channels.extend([out_point("Minimum"), out_point("Maximum")].into_iter().zip(["minimum", "maximum"]).map(|(mut channel, name)| { channel.name = name.into(); channel }));
                (channels, vec!["number", "point"])
            }
            "inspectVertex" => (vec![out_point("VertexPosition")], vec!["point"]),
            "inspectEdge" => (vec![ChannelSpec::named("S", "Start", "start", "EdgeStart").with_value_types(&["point"]), ChannelSpec::named("E", "End", "end", "EdgeEnd").with_value_types(&["point"]), out_length()], vec!["point", "number"]),
            "inspectFace" => (vec![ChannelSpec::named("V", "Verts", "vertices", "FaceVertices").with_value_types(&["text"]), out_normal("FaceNormal"), out_center().with_value_types(&["point"])], vec!["text", "vector", "point"]),
            "toBrep" => (vec![out_geometry("FacetedGeometry")], vec!["geometry"]),
            "exportObj" | "exportJson" => (vec![ChannelSpec::named("T", "Text", "text", "MeshText").with_value_types(&["text"])], vec!["text"]),
            _ => (vec![ChannelSpec::named("M", "Mesh", "meshOut", "PolygonMesh").with_value_types(&["mesh"])], vec!["mesh"]),
        };
        let summary = match operation {
            "construct" => "Create a polygon mesh from JSON vertices and zero-based face indices.",
            "box" => "Create a closed box with six quad faces and independent width, height, and depth.",
            "plane" => "Create one open quad in the horizontal plane.",
            "sphere" => "Create a closed triangular sphere; each subdivision increases surface detail.",
            "cylinder" => "Create a closed cylinder with polygon caps and quad sides.",
            "cone" => "Create a closed cone with a polygon base and triangular sides.",
            "fromBrep" => "Tessellate a B-Rep into a triangle mesh; smaller deflection produces more detail.",
            "toBrep" => "Convert mesh polygons to planar B-Rep faces without reconstructing curved surfaces.",
            "translate" => "Move every mesh vertex by the offset vector.",
            "rotate" => "Rotate the mesh around an axis through the origin; angle is in radians.",
            "scale" => "Scale each axis about the origin; negative factors mirror and preserve outward winding.",
            "moveVertices" => "Move selected zero-based vertex indices by the offset vector.",
            "translateComponents" => "Move selected vertices, edges, or faces; shared vertices move once.",
            "rotateComponents" => "Rotate selected components about their centroid or a chosen point; angle is in radians.",
            "scaleComponents" => "Scale selected components about their centroid or a chosen point, preserving polygon indices.",
            "bevel" => "Round selected edges of a closed convex mesh with 1–64 profile segments; reject widths crossing adjacent vertices.",
            "dissolveEdges" => "Remove selected connecting edges and join their neighboring polygon faces.",
            "dissolveVertices" => "Join a planar connected vertex neighborhood into one polygon without removing its surface.",
            "mergeVertices" => "Merge selected vertices at the first position, their mean, or within the distance tolerance; clean collapsed polygon loops.",
            "moveProportional" => "Move selected vertices fully and surrounding vertices with linear distance falloff from the center within the radius.",
            "snapVertices" => "Snap selected vertices to an origin-aligned grid with positive spacing.",
            "mirror" => "Reflect a one-sided mesh across an origin axis plane; weld only matching seam vertices within the tolerance.",
            "decimate" => "Approximate mesh simplification by shortest-edge collapse with winding and manifold checks; may stop before the target ratio.",
            "mergeCoplanar" => "Join adjacent coplanar faces without changing their surface.",
            "loopCut" => "Cut connected quad strips through selected preview edges; 1–256 cuts share vertices and crossing strips form grids.",
            "knifeCut" => "Split one face along the projected line through two points, sharing new boundary vertices with its neighbors.",
            "extrude" => "Extrude selected faces along their normals and connect the boundary with side faces.",
            "inset" => "Inset each selected face by a positive distance, preserving connected border faces.",
            "subdivide" => "Split selected faces into triangles while preserving their boundary edges.",
            "flip" => "Reverse the winding and normals of selected faces.",
            "deleteFaces" => "Remove selected faces, leaving open boundaries for further editing.",
            "triangulate" => "Triangulate polygon faces, including concave planar polygons.",
            "weld" => "Merge vertices within the tolerance and remove collapsed faces.",
            "orient" => "Make adjacent face winding consistent across connected components.",
            "fillHoles" => "Cap open boundary loops with polygon faces.",
            "inspectVertex" => "Read the position of a zero-based vertex without changing mesh data.",
            "inspectEdge" => "Read endpoints and length of a preview halfedge index without changing mesh data.",
            "inspectFace" => "Read polygon corner indices, unit normal, and the arithmetic mean of corner positions.",
            "analyze" => "Measure surface area, bounds, and topology; volume is available for closed, consistently oriented meshes.",
            "exportObj" => "Serialize mesh vertices and polygon faces as OBJ text.",
            "exportJson" => "Serialize editable indexed vertices and polygon faces as JSON text.",
            _ => name,
        };
        register_untyped(registry, operator_info_with_outputs(&id, name, name, "emoji:🥽️", summary, inputs, outputs, &[group]), Box::new(MeshOperation(operation, session.capture())), &produced);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
````

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs

Bytes 109076; SHA-256 a266704d641a8628cb83b80bdb9be9968b7ccc25461a6ae351d2d92cd5a4eb81.

````text
//! 📡️ `trinity.graph` artifact — state-patch wire codec for the raw document operation
//! (constitutional: spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("../🧬️mutations/💾️binary/📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::mutations::{change_data_property, create_edge, create_node, delete_edge, delete_node, move_node, remove_data_property, rename_node, set_query};
use crate::standards::v1::subsets::any::schema::snapshot::text::{port_dsl_to_port, port_to_port_dsl, PortDsl};
use crate::{Edge, EntityRef, JackSnapshot, Node, Port, PropertyBag, PropertyDef, PropertyValue};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use store::TextError;

//#region 🔖️DslMirrors
/// 🏷️ The `entity` half of `EntityRefDsl` — a plain 2-variant scalar tag (`dsl::DslScalar`, not
/// `DslEnum`): `EntityRefDsl` needs `dsl::DslField` (to bind as an ordinary record field on
/// `TrinityGraphOperationDsl`'s variants), and a `DslRecord` of `{ kind, id }` gets that directly,
/// unlike a tagged-variant `DslEnum`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar)]
enum EntityKindDsl {
    Node,
    Edge,
}

/// 🎯️ Local twin of `EntityRef` purely for the DSL engine's tuple-variant limitation — a flat
/// `{ kind, id }` twin, converted at the op-text boundary via `From`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct EntityRefDsl {
    kind: EntityKindDsl,
    id: String,
}

impl From<&EntityRef> for EntityRefDsl {
    fn from(value: &EntityRef) -> Self {
        match value {
            EntityRef::Node(id) => EntityRefDsl { kind: EntityKindDsl::Node, id: id.clone() },
            EntityRef::Edge(id) => EntityRefDsl { kind: EntityKindDsl::Edge, id: id.clone() },
        }
    }
}

impl From<EntityRefDsl> for EntityRef {
    fn from(value: EntityRefDsl) -> Self {
        match value.kind {
            EntityKindDsl::Node => EntityRef::Node(value.id),
            EntityKindDsl::Edge => EntityRef::Edge(value.id),
        }
    }
}

/// ⚡️ Local mirror of `TrinityGraphMutation` for `protocol::OpText`/`OpBinary` — `entity: EntityRef`
/// and `ports` fields transitively carry foreign/tuple-variant shapes, so the real enum (whose
/// variants each wrap a handcrafted `🦠️mutation` payload struct) can't derive `dsl::DslOps`
/// directly; this mirror's own variant names ARE the wire keywords (kept in lockstep with the real
/// enum's semantic slugs: `RenameNode` -> `rename-node`, etc.).
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum TrinityGraphOperationDsl {
    CreateNode {
        id: String,
        kind: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[dsl(table)]
        ports: Vec<PortDsl>,
    },
    DeleteNode {
        id: String,
    },
    CreateEdge {
        id: String,
        kind: String,
        source: String,
        target: String,
        properties: PropertyBag,
    },
    DeleteEdge {
        id: String,
    },
    RenameNode {
        id: String,
        name: String,
    },
    MoveNode {
        id: String,
        x: f64,
        y: f64,
    },
    ChangeDataProperty {
        entity: EntityRefDsl,
        key: String,
        value: PropertyValue,
    },
    RemoveDataProperty {
        entity: EntityRefDsl,
        key: String,
    },
    SetQuery {
        value: String,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for TrinityGraphOperationDsl {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &(spec_fn.ordinary)(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &(spec_fn.ordinary)(), dsl::JoinMode::Inline)
    }
}

impl OpBinary for TrinityGraphOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../🧬️mutations/💾️binary/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../🧬️mutations/💾️binary/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn trinity_graph_operation_to_dsl(operation: &TrinityGraphMutation) -> TrinityGraphOperationDsl {
    match operation {
        TrinityGraphMutation::CreateNode(payload) => {
            let node = &payload.node;
            TrinityGraphOperationDsl::CreateNode { id: node.id.clone(), kind: node.kind.clone(), name: node.name.clone(), x: node.x, y: node.y, width: node.width, height: node.height, ports: node.ports.iter().map(port_to_port_dsl).collect() }
        }
        TrinityGraphMutation::DeleteNode(payload) => TrinityGraphOperationDsl::DeleteNode { id: payload.id.clone() },
        TrinityGraphMutation::CreateEdge(payload) => {
            let edge = &payload.edge;
            TrinityGraphOperationDsl::CreateEdge { id: edge.id.clone(), kind: edge.kind.clone(), source: edge.source.clone(), target: edge.target.clone(), properties: edge.properties.clone() }
        }
        TrinityGraphMutation::DeleteEdge(payload) => TrinityGraphOperationDsl::DeleteEdge { id: payload.id.clone() },
        TrinityGraphMutation::RenameNode(payload) => TrinityGraphOperationDsl::RenameNode { id: payload.id.clone(), name: payload.new_name.clone() },
        TrinityGraphMutation::MoveNode(payload) => TrinityGraphOperationDsl::MoveNode { id: payload.id.clone(), x: payload.x, y: payload.y },
        TrinityGraphMutation::ChangeDataProperty(payload) => TrinityGraphOperationDsl::ChangeDataProperty { entity: (&payload.entity).into(), key: payload.key.clone(), value: payload.new_value.clone() },
        TrinityGraphMutation::RemoveDataProperty(payload) => TrinityGraphOperationDsl::RemoveDataProperty { entity: (&payload.entity).into(), key: payload.key.clone() },
        TrinityGraphMutation::SetQuery(payload) => TrinityGraphOperationDsl::SetQuery { value: payload.value.clone() },
    }
}

fn trinity_graph_operation_from_dsl(operation: TrinityGraphOperationDsl) -> TrinityGraphMutation {
    match operation {
        TrinityGraphOperationDsl::CreateNode { id, kind, name, x, y, width, height, ports } => create_node(Node { id, kind, name, x, y, width, height, properties: PropertyBag::new(), ports: ports.into_iter().map(port_dsl_to_port).collect() }),
        TrinityGraphOperationDsl::DeleteNode { id } => delete_node(id),
        TrinityGraphOperationDsl::CreateEdge { id, kind, source, target, properties } => create_edge(Edge { id, kind, source, target, properties }),
        TrinityGraphOperationDsl::DeleteEdge { id } => delete_edge(id),
        TrinityGraphOperationDsl::RenameNode { id, name } => rename_node(id, name),
        TrinityGraphOperationDsl::MoveNode { id, x, y } => move_node(id, x, y),
        TrinityGraphOperationDsl::ChangeDataProperty { entity, key, value } => change_data_property(entity.into(), key, value),
        TrinityGraphOperationDsl::RemoveDataProperty { entity, key } => remove_data_property(entity.into(), key),
        TrinityGraphOperationDsl::SetQuery { value } => set_query(value),
    }
}
//#endregion 🔖️DslMirrors

//#region 🔖️OpText
/// ⚡️ One-line textual notation for [`TrinityGraphMutation`] (`protocol::OpText`), delegating to the
/// derive-generated `TrinityGraphOperationDsl` mirror.
impl OpText for TrinityGraphMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        <TrinityGraphOperationDsl as OpText>::parse_op(line).map(trinity_graph_operation_from_dsl)
    }

    fn print_op(&self) -> String {
        <TrinityGraphOperationDsl as OpText>::print_op(&trinity_graph_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `TrinityGraphOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl OpBinary for TrinityGraphMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        trinity_graph_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        TrinityGraphOperationDsl::decode_op(bytes).map(trinity_graph_operation_from_dsl)
    }
}
//#endregion 🔖️OpText

/// 📦️ Encodes a Trinity graph `Mutation` to its binary command form.
pub fn encode_op(operation: &TrinityGraphMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a Trinity graph `Mutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<TrinityGraphMutation, protocol::ProtocolError> {
    TrinityGraphMutation::decode_op(bytes)
}

//#region 🔖️OwnedSprCatalog
const JACK_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

enum JackMutationFields {
    CreateNode(Option<Node>),
    DeleteNode(String),
    CreateEdge(Option<Edge>),
    DeleteEdge(String),
    RenameNode { id: String, name: String },
    MoveNode(String),
    ChangeDataProperty { entity: Option<EntityRef>, key: String, value: Option<PropertyValue> },
    RemoveDataProperty { entity: Option<EntityRef>, key: String },
    SetQuery(String),
}

enum JackRetirementOwner {
    Snapshot(JackSnapshot),
    SceneRoot(std::sync::Arc<crate::JackWorkingScene>),
    Scene(crate::JackWorkingScene),
    Mutation(TrinityGraphMutation),
    MutationFields(JackMutationFields),
    Property(PropertyValue),
    Bag(PropertyBag),
    Node(Node),
    Edge(Edge),
    Port(Port),
    PropertyDef(PropertyDef),
    NodeKind(crate::NodeKindDef),
    EdgeKind(crate::EdgeKindDef),
    PortKind(crate::PortKindDef),
}

struct JackOwnedRetirement {
    owner: std::mem::ManuallyDrop<Option<JackRetirementOwner>>,
    active: std::mem::ManuallyDrop<Option<Box<JackOwnedRetirement>>>,
    phase: u8,
}

impl JackOwnedRetirement {
    fn new(owner: JackRetirementOwner) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some(owner)), active: std::mem::ManuallyDrop::new(None), phase: 0 }
    }

    /// ✂️ Releases a string within the grant: one larger than `maximum_bytes` is paged off from its tail
    /// (char-boundary safe) instead of refused — a refusal is indistinguishable from a stall and pins every
    /// archive hydration that retires the owner.
    fn page_string_tail(value: &mut String, maximum_bytes: usize) -> Option<store::SnapshotRetirementStep> {
        if value.len() <= maximum_bytes {
            return None;
        }
        let mut cut = value.len() - maximum_bytes;
        while cut < value.len() && !value.is_char_boundary(cut) {
            cut += 1;
        }
        let released_bytes = value.len() - cut;
        value.truncate(cut);
        value.shrink_to_fit();
        Some(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes })
    }

    fn string_step(value: &mut String, maximum_items: usize, maximum_bytes: usize) -> store::SnapshotRetirementStep {
        if maximum_items == 0 || maximum_bytes == 0 {
            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(step) = Self::page_string_tail(value, maximum_bytes) {
            return step;
        }
        let released_bytes = value.len();
        drop(std::mem::take(value));
        store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes }
    }

    fn phased_string_step(value: &mut String, phase: &mut u8, next: u8, maximum_items: usize, maximum_bytes: usize) -> store::SnapshotRetirementStep {
        let step = Self::string_step(value, maximum_items, maximum_bytes);
        if matches!(step, store::SnapshotRetirementStep::Pending { released_items: 1, .. }) {
            *phase = next;
        }
        step
    }

    fn optional_string_step(value: &mut Option<String>, maximum_items: usize, maximum_bytes: usize) -> Option<store::SnapshotRetirementStep> {
        let string = value.as_mut()?;
        if maximum_items == 0 || maximum_bytes == 0 {
            return Some(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(step) = Self::page_string_tail(string, maximum_bytes) {
            return Some(step);
        }
        let released_bytes = string.len();
        drop(value.take());
        Some(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes })
    }

    /// 📏️ Bounded owned-byte size of one manifest entry — `Err` as soon as it exceeds the grant, so the
    /// estimate itself never walks more than one grant's worth of fields.
    fn properties_owned_bytes(properties: &[PropertyDef], maximum_bytes: usize) -> Result<usize, &'static str> {
        properties.iter().try_fold(0usize, |bytes, property| {
            let remaining = maximum_bytes.checked_sub(bytes).ok_or("jack-retirement.entry-too-large")?;
            bytes.checked_add(JackSnapshotCloneAuthority::property_owned_bytes(property, remaining)?.max(1)).filter(|bytes| *bytes <= maximum_bytes).ok_or("jack-retirement.entry-too-large")
        })
    }

    fn kind_owned_bytes(owner: &JackRetirementOwner, maximum_bytes: usize) -> Result<usize, &'static str> {
        let (name, properties, port_kinds): (&str, &[PropertyDef], &[String]) = match owner {
            JackRetirementOwner::NodeKind(kind) => (&kind.name, &kind.properties, &kind.port_kinds),
            JackRetirementOwner::EdgeKind(kind) => (&kind.name, &kind.properties, &[]),
            JackRetirementOwner::PortKind(kind) => (&kind.name, &kind.properties, &[]),
            JackRetirementOwner::PropertyDef(property) => return JackSnapshotCloneAuthority::property_owned_bytes(property, maximum_bytes).map(|bytes| bytes.max(1)),
            _ => return Err("jack-retirement.entry-not-shallow"),
        };
        let ports = port_kinds.iter().try_fold(name.len().max(1), |bytes, port| bytes.checked_add(port.len().max(1)).filter(|bytes| *bytes <= maximum_bytes)).ok_or("jack-retirement.entry-too-large")?;
        ports.checked_add(Self::properties_owned_bytes(properties, maximum_bytes.checked_sub(ports).ok_or("jack-retirement.entry-too-large")?)?).filter(|bytes| *bytes <= maximum_bytes).ok_or("jack-retirement.entry-too-large")
    }

    /// 🧺️ A manifest entry whose whole owned size fits the grant is one released item: its nested
    /// fields are already accounted by the bounded estimate, so it drops in this step instead of costing
    /// a nested cursor turn per string (the 42-kind Nakagin manifest took ~55 s per retired snapshot).
    fn release_or_spawn(active: &mut std::mem::ManuallyDrop<Option<Box<JackOwnedRetirement>>>, owner: JackRetirementOwner, maximum_items: usize, maximum_bytes: usize) -> store::SnapshotRetirementStep {
        match Self::kind_owned_bytes(&owner, maximum_bytes) {
            Ok(released_bytes) if maximum_items > 0 => {
                drop(owner);
                store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes }
            }
            _ => Self::spawn(active, owner),
        }
    }

    fn spawn(active: &mut std::mem::ManuallyDrop<Option<Box<JackOwnedRetirement>>>, owner: JackRetirementOwner) -> store::SnapshotRetirementStep {
        **active = Some(Box::new(Self::new(owner)));
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    }

    fn entity_step(value: &mut Option<EntityRef>, maximum_items: usize, maximum_bytes: usize) -> Option<store::SnapshotRetirementStep> {
        let entity = value.as_mut()?;
        let id = match entity {
            EntityRef::Node(id) | EntityRef::Edge(id) => id,
        };
        if maximum_items == 0 || maximum_bytes == 0 {
            return Some(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(step) = Self::page_string_tail(id, maximum_bytes) {
            return Some(step);
        }
        let released_bytes = id.len();
        drop(std::mem::take(id));
        drop(value.take());
        Some(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes })
    }

    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> store::SnapshotRetirementStep {
        let Some(owner) = self.owner.as_mut() else { return store::SnapshotRetirementStep::Complete };
        match owner {
            JackRetirementOwner::Snapshot(value) => match self.phase {
                0 => {
                    let scene = match value.content.take_local_owner::<crate::JackWorkingScene>() {
                        Ok(scene) => scene,
                        Err(_) => return store::SnapshotRetirementStep::Blocked,
                    };
                    self.phase = 1;
                    scene.map_or(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }, |scene| Self::spawn(&mut self.active, JackRetirementOwner::SceneRoot(scene)))
                }
                1 => Self::phased_string_step(&mut value.schema, &mut self.phase, 2, maximum_items, maximum_bytes),
                2 => Self::phased_string_step(&mut value.name, &mut self.phase, 3, maximum_items, maximum_bytes),
                3 => {
                    if let Some(step) = Self::optional_string_step(&mut value.manifest_id, maximum_items, maximum_bytes) {
                        return step;
                    }
                    self.phase = 4;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                4 => {
                    if let Some(kind) = value.manifest.node_kinds.pop() {
                        return Self::release_or_spawn(&mut self.active, JackRetirementOwner::NodeKind(kind), maximum_items, maximum_bytes);
                    }
                    self.phase = 5;
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                5 => {
                    if let Some(kind) = value.manifest.edge_kinds.pop() {
                        return Self::release_or_spawn(&mut self.active, JackRetirementOwner::EdgeKind(kind), maximum_items, maximum_bytes);
                    }
                    self.phase = 6;
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                6 => {
                    if let Some(kind) = value.manifest.port_kinds.pop() {
                        return Self::release_or_spawn(&mut self.active, JackRetirementOwner::PortKind(kind), maximum_items, maximum_bytes);
                    }
                    self.phase = 7;
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                7 => Self::phased_string_step(&mut value.content.child_id, &mut self.phase, 8, maximum_items, maximum_bytes),
                8 => Self::phased_string_step(&mut value.content.target.artifact_id, &mut self.phase, 9, maximum_items, maximum_bytes),
                9 => Self::phased_string_step(&mut value.content.target.dialect.artifact_kind, &mut self.phase, 10, maximum_items, maximum_bytes),
                10 => Self::phased_string_step(&mut value.content.target.dialect.standard, &mut self.phase, 11, maximum_items, maximum_bytes),
                11 => Self::phased_string_step(&mut value.content.target.dialect.subset, &mut self.phase, 12, maximum_items, maximum_bytes),
                12 => {
                    if let Some(step) = Self::optional_string_step(&mut value.root_node_id, maximum_items, maximum_bytes) {
                        return step;
                    }
                    self.phase = 13;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                13 => Self::phased_string_step(&mut value.query, &mut self.phase, 14, maximum_items, maximum_bytes),
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
            JackRetirementOwner::SceneRoot(_) => {
                if maximum_items == 0 {
                    return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
                }
                let scene = match self.owner.take() {
                    Some(JackRetirementOwner::SceneRoot(scene)) => scene,
                    _ => unreachable!("Jack scene root owner remains exact"),
                };
                match std::sync::Arc::try_unwrap(scene) {
                    Ok(scene) => {
                        *self.owner = Some(JackRetirementOwner::Scene(scene));
                        store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                    }
                    Err(scene) => {
                        drop(scene);
                        store::SnapshotRetirementStep::Complete
                    }
                }
            }
            JackRetirementOwner::Scene(scene) => {
                if let Some(node) = scene.nodes.pop() {
                    return Self::spawn(&mut self.active, JackRetirementOwner::Node(node));
                }
                if let Some(edge) = scene.edges.pop() {
                    return Self::spawn(&mut self.active, JackRetirementOwner::Edge(edge));
                }
                drop(self.owner.take());
                store::SnapshotRetirementStep::Complete
            }
            JackRetirementOwner::Mutation(_) => {
                let mutation = match self.owner.take() {
                    Some(JackRetirementOwner::Mutation(value)) => value,
                    _ => unreachable!("Jack mutation owner variant remains exact"),
                };
                let fields = match mutation {
                    TrinityGraphMutation::CreateNode(value) => JackMutationFields::CreateNode(Some(value.node)),
                    TrinityGraphMutation::DeleteNode(value) => JackMutationFields::DeleteNode(value.id),
                    TrinityGraphMutation::CreateEdge(value) => JackMutationFields::CreateEdge(Some(value.edge)),
                    TrinityGraphMutation::DeleteEdge(value) => JackMutationFields::DeleteEdge(value.id),
                    TrinityGraphMutation::RenameNode(value) => JackMutationFields::RenameNode { id: value.id, name: value.new_name },
                    TrinityGraphMutation::MoveNode(value) => JackMutationFields::MoveNode(value.id),
                    TrinityGraphMutation::ChangeDataProperty(value) => JackMutationFields::ChangeDataProperty { entity: Some(value.entity), key: value.key, value: Some(value.new_value) },
                    TrinityGraphMutation::RemoveDataProperty(value) => JackMutationFields::RemoveDataProperty { entity: Some(value.entity), key: value.key },
                    TrinityGraphMutation::SetQuery(value) => JackMutationFields::SetQuery(value.value),
                };
                *self.owner = Some(JackRetirementOwner::MutationFields(fields));
                store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
            }
            JackRetirementOwner::MutationFields(fields) => match fields {
                JackMutationFields::CreateNode(value) => {
                    if let Some(value) = value.take() {
                        return Self::spawn(&mut self.active, JackRetirementOwner::Node(value));
                    }
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
                JackMutationFields::DeleteNode(value) | JackMutationFields::DeleteEdge(value) | JackMutationFields::MoveNode(value) | JackMutationFields::SetQuery(value) => {
                    if self.phase == 0 {
                        return Self::phased_string_step(value, &mut self.phase, 1, maximum_items, maximum_bytes);
                    }
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
                JackMutationFields::CreateEdge(value) => {
                    if let Some(value) = value.take() {
                        return Self::spawn(&mut self.active, JackRetirementOwner::Edge(value));
                    }
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
                JackMutationFields::RenameNode { id, name } => {
                    let value = if self.phase == 0 { id } else { name };
                    if self.phase < 2 {
                        let next = self.phase + 1;
                        return Self::phased_string_step(value, &mut self.phase, next, maximum_items, maximum_bytes);
                    }
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
                JackMutationFields::ChangeDataProperty { entity, key, value } => match self.phase {
                    0 => {
                        if let Some(step) = Self::entity_step(entity, maximum_items, maximum_bytes) {
                            return step;
                        }
                        self.phase = 1;
                        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                    }
                    1 => Self::phased_string_step(key, &mut self.phase, 2, maximum_items, maximum_bytes),
                    2 => {
                        if let Some(value) = value.take() {
                            self.phase = 3;
                            return Self::spawn(&mut self.active, JackRetirementOwner::Property(value));
                        }
                        self.phase = 3;
                        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                    }
                    _ => {
                        drop(self.owner.take());
                        store::SnapshotRetirementStep::Complete
                    }
                },
                JackMutationFields::RemoveDataProperty { entity, key } => match self.phase {
                    0 => {
                        if let Some(step) = Self::entity_step(entity, maximum_items, maximum_bytes) {
                            return step;
                        }
                        self.phase = 1;
                        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                    }
                    1 => Self::phased_string_step(key, &mut self.phase, 2, maximum_items, maximum_bytes),
                    _ => {
                        drop(self.owner.take());
                        store::SnapshotRetirementStep::Complete
                    }
                },
            },
            JackRetirementOwner::Property(value) => match value {
                PropertyValue::String(value) if self.phase == 0 => Self::phased_string_step(value, &mut self.phase, 1, maximum_items, maximum_bytes),
                PropertyValue::Array(values) => {
                    if let Some(value) = values.pop() {
                        return Self::spawn(&mut self.active, JackRetirementOwner::Property(value));
                    }
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
                PropertyValue::Object(values) => {
                    if let Some((key, value)) = values.pop_first() {
                        if key.len() > maximum_bytes || maximum_items == 0 {
                            values.insert(key, value);
                            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        let released_bytes = key.len();
                        drop(key);
                        *self.active = Some(Box::new(Self::new(JackRetirementOwner::Property(value))));
                        return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
                    }
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
            JackRetirementOwner::Bag(values) => {
                if let Some((key, value)) = values.pop_first() {
                    if key.len() > maximum_bytes || maximum_items == 0 {
                        values.insert(key, value);
                        return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    let released_bytes = key.len();
                    drop(key);
                    *self.active = Some(Box::new(Self::new(JackRetirementOwner::Property(value))));
                    return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
                }
                drop(self.owner.take());
                store::SnapshotRetirementStep::Complete
            }
            JackRetirementOwner::Node(value) => match self.phase {
                0 => Self::phased_string_step(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes),
                1 => Self::phased_string_step(&mut value.kind, &mut self.phase, 2, maximum_items, maximum_bytes),
                2 => Self::phased_string_step(&mut value.name, &mut self.phase, 3, maximum_items, maximum_bytes),
                3 => {
                    if !value.properties.is_empty() {
                        self.phase = 4;
                        return Self::spawn(&mut self.active, JackRetirementOwner::Bag(std::mem::take(&mut value.properties)));
                    }
                    self.phase = 4;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                4 => {
                    if let Some(port) = value.ports.pop() {
                        return Self::spawn(&mut self.active, JackRetirementOwner::Port(port));
                    }
                    self.phase = 5;
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
            JackRetirementOwner::Edge(value) => match self.phase {
                0 => Self::phased_string_step(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes),
                1 => Self::phased_string_step(&mut value.kind, &mut self.phase, 2, maximum_items, maximum_bytes),
                2 => Self::phased_string_step(&mut value.source, &mut self.phase, 3, maximum_items, maximum_bytes),
                3 => Self::phased_string_step(&mut value.target, &mut self.phase, 4, maximum_items, maximum_bytes),
                4 => {
                    if !value.properties.is_empty() {
                        self.phase = 5;
                        return Self::spawn(&mut self.active, JackRetirementOwner::Bag(std::mem::take(&mut value.properties)));
                    }
                    self.phase = 5;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
            JackRetirementOwner::Port(value) => match self.phase {
                0 => Self::phased_string_step(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes),
                1 => Self::phased_string_step(&mut value.kind, &mut self.phase, 2, maximum_items, maximum_bytes),
                2 => {
                    if !value.properties.is_empty() {
                        self.phase = 3;
                        return Self::spawn(&mut self.active, JackRetirementOwner::Bag(std::mem::take(&mut value.properties)));
                    }
                    self.phase = 3;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
            JackRetirementOwner::PropertyDef(value) => match self.phase {
                0 => Self::phased_string_step(&mut value.name, &mut self.phase, 1, maximum_items, maximum_bytes),
                1 => {
                    if let Some(step) = Self::optional_string_step(&mut value.expr, maximum_items, maximum_bytes) {
                        return step;
                    }
                    self.phase = 2;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                2 => {
                    if maximum_items == 0 {
                        return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    let value_type = match value.retire_value_type_step(maximum_bytes) {
                        Ok(value_type) => value_type,
                        Err(_) => return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 },
                    };
                    if let Some(value_type) = value_type {
                        let released_bytes = value_type.len();
                        drop(value_type);
                        return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
                    }
                    if !value.value_type_terminal_is_empty() {
                        return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    self.phase = 3;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
            JackRetirementOwner::NodeKind(value) => match self.phase {
                0 => Self::phased_string_step(&mut value.name, &mut self.phase, 1, maximum_items, maximum_bytes),
                1 => {
                    if let Some(value) = value.properties.pop() {
                        return Self::release_or_spawn(&mut self.active, JackRetirementOwner::PropertyDef(value), maximum_items, maximum_bytes);
                    }
                    self.phase = 2;
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                2 => {
                    if let Some(port_kind) = value.port_kinds.pop() {
                        if port_kind.len() > maximum_bytes || maximum_items == 0 {
                            value.port_kinds.push(port_kind);
                            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        let released_bytes = port_kind.len();
                        drop(port_kind);
                        return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
                    }
                    self.phase = 3;
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
            JackRetirementOwner::EdgeKind(value) => match self.phase {
                0 => Self::phased_string_step(&mut value.name, &mut self.phase, 1, maximum_items, maximum_bytes),
                1 => {
                    if let Some(value) = value.properties.pop() {
                        return Self::release_or_spawn(&mut self.active, JackRetirementOwner::PropertyDef(value), maximum_items, maximum_bytes);
                    }
                    self.phase = 2;
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
            JackRetirementOwner::PortKind(value) => match self.phase {
                0 => Self::phased_string_step(&mut value.name, &mut self.phase, 1, maximum_items, maximum_bytes),
                1 => {
                    if let Some(value) = value.properties.pop() {
                        return Self::release_or_spawn(&mut self.active, JackRetirementOwner::PropertyDef(value), maximum_items, maximum_bytes);
                    }
                    self.phase = 2;
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                _ => {
                    drop(self.owner.take());
                    store::SnapshotRetirementStep::Complete
                }
            },
        }
    }
}

impl store::ErasedSnapshotRetirement for JackOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(maximum_items.min(1), maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Jack nested retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        let step = self.advance(maximum_items.min(1), maximum_bytes);
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.active.is_none()
    }
}

impl Drop for JackOwnedRetirement {
    fn drop(&mut self) {
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(self), "Jack owner reached Drop before cursor retirement reached terminal-empty");
    }
}

pub struct JackSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<JackSnapshot> for JackSnapshotRetirementFactory {
    fn retire_owned(&self, value: JackSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(JackOwnedRetirement::new(JackRetirementOwner::Snapshot(value)))
    }
}

struct JackSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<JackSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for JackSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Jack root retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, value));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err(owner) => {
                *self.owner = Some(owner);
                Ok(store::SnapshotRetirementStep::Blocked)
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(self.owner.is_none() && self.retirement.is_none(), "Jack snapshot root reached Drop before exact Arc handback");
    }
}

impl store::SnapshotRetirementFactory<JackSnapshot> for JackSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<JackSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(JackSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None) })
    }
}

pub struct JackMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<TrinityGraphMutation> for JackMutationRetirementFactory {
    fn retire_owned(&self, value: TrinityGraphMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(JackOwnedRetirement::new(JackRetirementOwner::Mutation(value)))
    }
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its fixed-size owned field authority inline so cursor transitions do not allocate another retirement owner.")]
enum JackSnapshotDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct JackSnapshotDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: JackSnapshotDecodeState,
    value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackSnapshotDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: JackSnapshotDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot> for JackSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
        if matches!(self.state, JackSnapshotDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("jack-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = JackSnapshotDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let JackSnapshotDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("jack-envelope.snapshot-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("jack-envelope.snapshot-pack-missing", token.start))?;
                let value = <JackSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| diagnostic("jack-envelope.snapshot-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Jack snapshot pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = JackSnapshotDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("jack-envelope.snapshot-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<JackSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, JackSnapshotDecodeState::Ready) {
            return Err(self.diagnostic("jack-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("jack-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = JackSnapshotDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(self.retirement.is_some()) * store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let JackSnapshotDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = JackSnapshotDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, value));
                self.state = JackSnapshotDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = JackSnapshotDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("Jack snapshot retirement remains retained");
        match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "jack-envelope.snapshot-retirement-fault", offset: 0, line: 0, column: 0, path })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = JackSnapshotDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("jack-envelope.snapshot-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, JackSnapshotDecodeState::Published | JackSnapshotDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotDecodeAuthority {
    fn drop(&mut self) {
        assert!(store::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(self), "Jack snapshot decode reached Drop before publication or bounded retirement");
    }
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its fixed-size owned field authority inline so cursor transitions do not allocate another retirement owner.")]
enum JackMutationDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct JackMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: JackMutationDecodeState,
    value: std::mem::ManuallyDrop<Option<TrinityGraphMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackMutationDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: JackMutationDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<TrinityGraphMutation> for JackMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
        if matches!(self.state, JackMutationDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("jack-envelope.mutation-pack-must-be-scalar", token.start));
            }
            self.state = JackMutationDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let JackMutationDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("jack-envelope.mutation-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("jack-envelope.mutation-pack-missing", token.start))?;
                let value = TrinityGraphMutation::decode_op(bytes).map_err(|_| diagnostic("jack-envelope.mutation-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Jack mutation pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = JackMutationDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("jack-envelope.mutation-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<TrinityGraphMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, JackMutationDecodeState::Ready) {
            return Err(self.diagnostic("jack-envelope.mutation-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("jack-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = JackMutationDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let JackMutationDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = JackMutationDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, value));
                self.state = JackMutationDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = JackMutationDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("Jack mutation retirement remains retained");
        match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "jack-envelope.mutation-retirement-fault", offset: 0, line: 0, column: 0, path })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = JackMutationDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("jack-envelope.mutation-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, JackMutationDecodeState::Published | JackMutationDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(store::ArtifactEnvelopeMutationFieldAuthority::terminal_is_empty(self), "Jack mutation decode reached Drop before publication or bounded retirement");
    }
}

struct JackRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for JackRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "jack-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

pub struct JackEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<JackSnapshot, TrinityGraphMutation> for JackEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<JackSnapshot, TrinityGraphMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(JackSnapshotRetirementFactory), std::sync::Arc::new(JackMutationRetirementFactory), self.edit_history_decoder())
            .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<JackSnapshot, TrinityGraphMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot>> {
        Box::new(JackSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<TrinityGraphMutation>> {
        Box::new(JackMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(JackRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<TrinityGraphMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(JackMutationRetirementFactory))
    }
}

pub fn jack_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<JackSnapshot, TrinityGraphMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(JackEnvelopeOwnedFieldCatalog), std::sync::Arc::new(JackSnapshotRetirementFactory), std::sync::Arc::new(JackMutationRetirementFactory))
}
//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization
enum JackSnapshotCloneKind {
    Node { source: usize, property: usize, port: usize, value: crate::NodeKindDef },
    Edge { source: usize, property: usize, value: crate::EdgeKindDef },
    Port { source: usize, property: usize, value: crate::PortKindDef },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JackSnapshotCloneStep {
    Pending { copied_bytes: usize },
    Complete,
}

pub struct JackSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    active: std::mem::ManuallyDrop<Option<JackSnapshotCloneKind>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: u8,
    index: usize,
    retain_local_owner: bool,
    terminal: bool,
}

impl Default for JackSnapshotCloneAuthority {
    fn default() -> Self {
        Self::new()
    }
}

impl JackSnapshotCloneAuthority {
    pub fn new() -> Self {
        Self::with_local_owner(true)
    }

    pub fn metadata_only() -> Self {
        Self::with_local_owner(false)
    }

    fn with_local_owner(retain_local_owner: bool) -> Self {
        let content = store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
        Self {
            value: std::mem::ManuallyDrop::new(Some(JackSnapshot { schema: String::new(), name: String::new(), manifest_id: None, manifest: Default::default(), camera: Default::default(), content, root_node_id: None, query: String::new() })),
            active: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            index: 0,
            retain_local_owner,
            terminal: false,
        }
    }

    fn clone_string(source: &str, maximum_bytes: usize) -> Result<String, &'static str> {
        if source.len() > maximum_bytes {
            return Err("jack-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "jack-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn value_type_owned_bytes(source: &semio_framework_graph::manifest::ValueType, maximum_bytes: usize) -> Result<usize, &'static str> {
        let mut bytes = 0usize;
        let mut value = source;
        loop {
            match value {
                semio_framework_graph::manifest::ValueType::List(inner) => {
                    bytes = bytes.checked_add(size_of::<Box<semio_framework_graph::manifest::ValueType>>()).ok_or("jack-store.initializer-property-size")?;
                    if bytes > maximum_bytes {
                        return Err("jack-store.initializer-property-too-large");
                    }
                    value = inner;
                }
                semio_framework_graph::manifest::ValueType::Schema(schema) => {
                    bytes = bytes.checked_add(schema.len()).ok_or("jack-store.initializer-property-size")?;
                    return (bytes <= maximum_bytes).then_some(bytes).ok_or("jack-store.initializer-property-too-large");
                }
                _ => return Ok(bytes),
            }
        }
    }

    fn property_owned_bytes(source: &PropertyDef, maximum_bytes: usize) -> Result<usize, &'static str> {
        let value_type = Self::value_type_owned_bytes(&source.value_type, maximum_bytes)?;
        source.name.len().checked_add(source.expr.as_ref().map_or(0, String::len)).and_then(|bytes| bytes.checked_add(value_type)).filter(|bytes| *bytes <= maximum_bytes).ok_or("jack-store.initializer-property-too-large")
    }

    fn clone_property(source: &PropertyDef, maximum_bytes: usize) -> Result<PropertyDef, &'static str> {
        Self::property_owned_bytes(source, maximum_bytes)?;
        Ok(source.clone())
    }

    fn begin_kind(&mut self, source: &JackSnapshot, maximum_bytes: usize) -> Result<bool, &'static str> {
        let target = self.value.as_mut().ok_or("jack-store.initializer-clone-target")?;
        match self.phase {
            4 => {
                if self.index == 0 && target.manifest.node_kinds.capacity() == 0 {
                    target.manifest.node_kinds.try_reserve_exact(source.manifest.node_kinds.len()).map_err(|_| "jack-store.initializer-node-kind-admission")?;
                }
                let Some(kind) = source.manifest.node_kinds.get(self.index) else {
                    self.phase = 5;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_| "jack-store.initializer-node-property-admission")?;
                let mut port_kinds = Vec::new();
                port_kinds.try_reserve_exact(kind.port_kinds.len()).map_err(|_| "jack-store.initializer-node-port-admission")?;
                *self.active =
                    Some(JackSnapshotCloneKind::Node { source: self.index, property: 0, port: 0, value: crate::NodeKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, properties, port_kinds } });
                Ok(true)
            }
            5 => {
                if self.index == 0 && target.manifest.edge_kinds.capacity() == 0 {
                    target.manifest.edge_kinds.try_reserve_exact(source.manifest.edge_kinds.len()).map_err(|_| "jack-store.initializer-edge-kind-admission")?;
                }
                let Some(kind) = source.manifest.edge_kinds.get(self.index) else {
                    self.phase = 6;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_| "jack-store.initializer-edge-property-admission")?;
                *self.active = Some(JackSnapshotCloneKind::Edge { source: self.index, property: 0, value: crate::EdgeKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, properties } });
                Ok(true)
            }
            6 => {
                if self.index == 0 && target.manifest.port_kinds.capacity() == 0 {
                    target.manifest.port_kinds.try_reserve_exact(source.manifest.port_kinds.len()).map_err(|_| "jack-store.initializer-port-kind-admission")?;
                }
                let Some(kind) = source.manifest.port_kinds.get(self.index) else {
                    self.phase = 7;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_| "jack-store.initializer-port-property-admission")?;
                *self.active =
                    Some(JackSnapshotCloneKind::Port { source: self.index, property: 0, value: crate::PortKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, direction: kind.direction, properties } });
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn advance(&mut self, source: &JackSnapshot, maximum_bytes: usize) -> Result<JackSnapshotCloneStep, &'static str> {
        if maximum_bytes == 0 {
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes: 0 });
        }
        if let Some(active) = self.active.as_mut() {
            let (completed, copied_bytes) = match active {
                JackSnapshotCloneKind::Node { source: source_index, property, port, value } => {
                    let source = source.manifest.node_kinds.get(*source_index).ok_or("jack-store.initializer-node-kind-stale")?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else if let Some(kind) = source.port_kinds.get(*port) {
                        value.port_kinds.push(Self::clone_string(kind, maximum_bytes)?);
                        *port += 1;
                        (false, kind.len())
                    } else {
                        (true, 0)
                    }
                }
                JackSnapshotCloneKind::Edge { source: source_index, property, value } => {
                    let source = source.manifest.edge_kinds.get(*source_index).ok_or("jack-store.initializer-edge-kind-stale")?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else {
                        (true, 0)
                    }
                }
                JackSnapshotCloneKind::Port { source: source_index, property, value } => {
                    let source = source.manifest.port_kinds.get(*source_index).ok_or("jack-store.initializer-port-kind-stale")?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else {
                        (true, 0)
                    }
                }
            };
            if completed {
                let active = self.active.take().expect("completed Jack kind clone remains exact");
                let target = self.value.as_mut().ok_or("jack-store.initializer-clone-target")?;
                match active {
                    JackSnapshotCloneKind::Node { value, .. } => target.manifest.node_kinds.push(value),
                    JackSnapshotCloneKind::Edge { value, .. } => target.manifest.edge_kinds.push(value),
                    JackSnapshotCloneKind::Port { value, .. } => target.manifest.port_kinds.push(value),
                }
                self.index += 1;
            }
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes });
        }
        if self.begin_kind(source, maximum_bytes)? {
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes: 0 });
        }
        let target = self.value.as_mut().ok_or("jack-store.initializer-clone-target")?;
        let copied_bytes = match self.phase {
            0 => {
                target.schema = Self::clone_string(&source.schema, maximum_bytes)?;
                source.schema.len()
            }
            1 => {
                target.name = Self::clone_string(&source.name, maximum_bytes)?;
                source.name.len()
            }
            2 => {
                target.manifest_id = source.manifest_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.manifest_id.as_deref().map_or(0, str::len)
            }
            3 => {
                target.camera = source.camera.clone();
                0
            }
            7 => {
                target.content.child_id = Self::clone_string(&source.content.child_id, maximum_bytes)?;
                if self.retain_local_owner {
                    if let Some(owner) = source.content.local_owner::<crate::JackWorkingScene>() {
                        target.content.set_local_owner(owner);
                    }
                }
                source.content.child_id.len()
            }
            8 => {
                target.content.target.artifact_id = Self::clone_string(&source.content.target.artifact_id, maximum_bytes)?;
                source.content.target.artifact_id.len()
            }
            9 => {
                target.content.target.dialect.artifact_kind = Self::clone_string(&source.content.target.dialect.artifact_kind, maximum_bytes)?;
                source.content.target.dialect.artifact_kind.len()
            }
            10 => {
                target.content.target.dialect.standard = Self::clone_string(&source.content.target.dialect.standard, maximum_bytes)?;
                source.content.target.dialect.standard.len()
            }
            11 => {
                target.content.target.dialect.subset = Self::clone_string(&source.content.target.dialect.subset, maximum_bytes)?;
                source.content.target.dialect.subset.len()
            }
            12 => {
                target.root_node_id = source.root_node_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.root_node_id.as_deref().map_or(0, str::len)
            }
            13 => {
                target.query = Self::clone_string(&source.query, maximum_bytes)?;
                source.query.len()
            }
            _ => {
                self.terminal = true;
                return Ok(JackSnapshotCloneStep::Complete);
            }
        };
        self.phase += 1;
        Ok(JackSnapshotCloneStep::Pending { copied_bytes })
    }

    fn step(&mut self, source: &JackSnapshot, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let phase = self.phase;
        let active = self.active.is_some();
        let step = self.advance(source, JACK_OWNED_FIELD_BYTES)?;
        if !active {
            match phase {
                0 => digest.observe(source.schema.as_bytes()),
                1 => digest.observe(source.name.as_bytes()),
                2 => digest.observe(source.manifest_id.as_deref().unwrap_or_default().as_bytes()),
                7 => digest.observe(source.content.child_id.as_bytes()),
                8 => digest.observe(source.content.target.artifact_id.as_bytes()),
                9 => digest.observe(source.content.target.dialect.artifact_kind.as_bytes()),
                10 => digest.observe(source.content.target.dialect.standard.as_bytes()),
                11 => digest.observe(source.content.target.dialect.subset.as_bytes()),
                12 => digest.observe(source.root_node_id.as_deref().unwrap_or_default().as_bytes()),
                13 => digest.observe(source.query.as_bytes()),
                _ => {}
            }
        }
        match step {
            JackSnapshotCloneStep::Pending { copied_bytes } => {
                cx.consume_fuel(copied_bytes.max(1) as u64);
                Ok(false)
            }
            JackSnapshotCloneStep::Complete => Ok(true),
        }
    }

    pub fn take_value(&mut self) -> Option<JackSnapshot> {
        if !self.terminal || self.active.is_some() {
            return None;
        }
        self.value.take()
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(active) = self.active.take() {
                let owner = match active {
                    JackSnapshotCloneKind::Node { value, .. } => JackRetirementOwner::NodeKind(value),
                    JackSnapshotCloneKind::Edge { value, .. } => JackRetirementOwner::EdgeKind(value),
                    JackSnapshotCloneKind::Port { value, .. } => JackRetirementOwner::PortKind(value),
                };
                *self.retirement = Some(Box::new(JackOwnedRetirement::new(owner)));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Jack clone retirement remains exact");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err("Jack clone retirement reported false terminal".into()),
            step => Ok(step),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.active.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Jack snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

pub fn jack_document_store_owners() -> store::DocumentStoreOwners<JackSnapshot, TrinityGraphMutation> {
    store::DocumentStoreOwners::new(
        std::sync::Arc::new(JackSnapshotRetirementFactory),
        std::sync::Arc::new(JackSnapshotRetirementFactory),
        std::sync::Arc::new(JackMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<JackSnapshot, TrinityGraphMutation>::new()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JackStoreInitializationPhase {
    ValidateEnvelope,
    ValidateEditPair { left: usize, right: usize },
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FindApplied { position: usize, scan: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    HashInverse { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize, scan: usize },
    HashRedoForward { position: usize, edit: usize, mutation: usize },
    HashRedoInverse { position: usize, edit: usize, mutation: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct JackStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<JackSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<JackSnapshot, TrinityGraphMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone: std::mem::ManuallyDrop<Option<JackSnapshotCloneAuthority>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    phase: JackStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl JackStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self {
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(Some(JackSnapshotCloneAuthority::new())),
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"jack.initial"))),
            edit_digest: std::mem::ManuallyDrop::new(None),
            phase: JackStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = JackStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, String> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, JACK_OWNED_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= JACK_OWNED_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err("Jack store initializer retirement exceeded its exact grant".into()),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err("Jack store initializer retirement reported a false terminal".into()),
        }
    }

    fn pump_terminal_retirement(&mut self) -> Result<bool, String> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&JackSnapshotRetirementFactory, 1, JACK_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Jack initialization runtime reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if let Some(clone) = self.clone.as_mut() {
            match clone.close_step(1, JACK_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    drop(self.clone.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Jack snapshot clone reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(jack_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, JACK_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err("Jack initialization envelope retirement reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.clone.is_none()
            && self.initial_digest.is_none()
            && self.edit_digest.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"jack-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::Cancelled) {
            self.phase = JackStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_bytes());
            self.phase = JackStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.phase {
            JackStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"jack-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::TRINITY_GRAPH_SCHEMA || envelope.id.is_empty() || envelope.id.len() > JACK_OWNED_FIELD_BYTES {
                    self.fail(b"jack-store.initializer-envelope-invalid");
                } else {
                    self.phase = JackStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("validated Jack envelope remains retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = JackStoreInitializationPhase::CloneInitial;
                } else if right >= envelope.vcs.edits.len() {
                    self.phase = JackStoreInitializationPhase::ValidateEditPair { left: left + 1, right: left + 2 };
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id || envelope.vcs.edits[left].id.len() > JACK_OWNED_FIELD_BYTES {
                    self.fail(b"jack-store.initializer-duplicate-or-hostile-edit");
                } else {
                    self.phase = JackStoreInitializationPhase::ValidateEditPair { left, right: right + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Jack envelope remains retained during initial clone").vcs.initial_snapshot;
                let clone = self.clone.as_mut().expect("Jack initial clone authority remains retained");
                let complete = match clone.step(source, self.initial_digest.as_mut().expect("Jack initial digest remains retained"), cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("Jack initial snapshot was built one semantic field at a time");
                    drop(self.clone.take());
                    let initial_digest = self.initial_digest.take().expect("Jack initial digest remains retained").finish();
                    let envelope = self.envelope.as_ref().expect("Jack envelope remains retained during runtime construction");
                    *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, initial_digest));
                    self.phase = JackStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = JackStoreInitializationPhase::FindApplied { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = JackStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = JackStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = JackStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::FindApplied { position, scan } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Writer runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = JackStoreInitializationPhase::FindRedo { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"jack-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let id = edit.id.clone();
                    let sequence_number = edit.sequence_number;
                    let started_at = edit.started_at.clone();
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"jack.edit");
                    digest.observe(id.as_bytes());
                    digest.observe(&sequence_number.to_be_bytes());
                    digest.observe(started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = JackStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = JackStoreInitializationPhase::FindApplied { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack applied edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = JackStoreInitializationPhase::HashInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let encoded = match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= JACK_OWNED_FIELD_BYTES => encoded,
                    _ => {
                        self.fail(b"jack-store.initializer-forward-encoding");
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                self.edit_digest.as_mut().expect("Writer edit digest remains retained").observe(&encoded);
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Writer runtime current snapshot remains retained");
                let (diff, messages) = operation.diff(current).into_parts();
                if messages.iter().any(|message| message.level == protocol::Severity::Fatal) {
                    self.fail(b"jack-store.initializer-fatal-mutation");
                    return semio_framework_job::StepOutcome::Yield;
                }
                match diff.apply(current) {
                    Ok(next) => {
                        let previous = std::mem::replace(current, next);
                        *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, previous));
                        self.phase = JackStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    Err(error) => {
                        self.fault = Some(error.to_string().into_bytes());
                        self.phase = JackStoreInitializationPhase::RetireFault;
                    }
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = JackStoreInitializationPhase::CommitApplied { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= JACK_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Writer edit digest remains retained").observe(&encoded);
                        self.phase = JackStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"jack-store.initializer-inverse-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack applied edit remains retained");
                let id = entry.id.clone();
                let actor = entry.actor.clone();
                let digest = self.edit_digest.take().expect("Jack applied edit digest remains retained").finish();
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained");
                if let Err(error) = runtime.push_applied(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = JackStoreInitializationPhase::RetireFault;
                } else {
                    runtime.set_local_actor_id(actor);
                    self.phase = JackStoreInitializationPhase::FindApplied { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::FindRedo { position, scan } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = JackStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"jack-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let id = edit.id.clone();
                    let sequence_number = edit.sequence_number;
                    let started_at = edit.started_at.clone();
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"jack.edit");
                    digest.observe(id.as_bytes());
                    digest.observe(&sequence_number.to_be_bytes());
                    digest.observe(started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = JackStoreInitializationPhase::HashRedoForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = JackStoreInitializationPhase::FindRedo { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::HashRedoForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack redo edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = JackStoreInitializationPhase::HashRedoInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= JACK_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Jack redo digest remains retained").observe(&encoded);
                        self.phase = JackStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"jack-store.initializer-redo-forward-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack redo edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = JackStoreInitializationPhase::CommitRedo { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= JACK_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Jack redo digest remains retained").observe(&encoded);
                        self.phase = JackStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"jack-store.initializer-redo-inverse-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack redo edit remains retained").id.clone();
                let digest = self.edit_digest.take().expect("Jack redo digest remains retained").finish();
                if let Err(error) = self.runtime.as_mut().expect("Writer runtime remains retained").push_redo(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = JackStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = JackStoreInitializationPhase::FindRedo { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"jack-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("Jack envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Writer runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, jack_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = JackStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
            JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    *self.initial_digest = None;
                    *self.edit_digest = None;
                    self.terminal_handoff = true;
                    if self.phase == JackStoreInitializationPhase::RetireCancelled {
                        self.phase = JackStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = JackStoreInitializationPhase::Fault;
                        let fault = self.fault.take().unwrap_or_else(|| b"jack-store.initializer-fault".to_vec());
                        let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, &fault).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            JackStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            JackStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            JackStoreInitializationPhase::Fault => {
                let fault = self.fault.as_deref().unwrap_or(b"jack-store.initializer-fault");
                let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, fault).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
            }
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, JackStoreInitializationPhase::Cancelled | JackStoreInitializationPhase::Fault) {
            self.phase = JackStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework_plugin::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes < JACK_OWNED_FIELD_BYTES {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement() {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                *self.initial_digest = None;
                *self.edit_digest = None;
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::Plugin, semio_framework_plugin::FaultCode::new("artifact-store.initializer-close"), format!("Jack initializer close failed: {error}"))),
        }
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<JackSnapshot, TrinityGraphMutation>> {
        if self.phase != JackStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        *self.initial_digest = None;
        *self.edit_digest = None;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for JackStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn jack_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<JackSnapshot, TrinityGraphMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(JackStoreInitializationAuthority::new(envelope, operation, generation)))
}

//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
````
