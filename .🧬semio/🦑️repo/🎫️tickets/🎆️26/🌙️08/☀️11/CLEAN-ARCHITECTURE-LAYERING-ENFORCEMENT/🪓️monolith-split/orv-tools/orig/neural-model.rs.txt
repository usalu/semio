//! 🧱️ Neural engine model: dictionary, schema, tree and operator vocabulary with their retirement, hashing and validation laws.

use std::collections::{HashMap, HashSet, VecDeque};
use protocol::causal::transition::HistoryFoldIndex;
use std::sync::Arc;
use std::hash::Hasher;
/// 🔮️ Test-only: production moved to `ToValue`/`FromValue` below (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass — see `📓️orderedmap-tenth-seam.md`).
#[cfg(test)]
use serde::{Deserialize, Serialize};
use protocol::value::ordered::OrderedMap;
use protocol::value::{DslValue, FromValue, Number, ToValue, ValueError};
use semio_framework_value::{ValueKind, ValueType};
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

#[path = "../🧵️retirement/🦀️.rs"]
pub mod retirement;
pub use retirement::{ColdDictionaryBuilder,ColdValueOwner,ValueRetirement};
#[path = "../🧊️cold/🦀️.rs"]
pub mod cold;
pub use cold::{ColdOwner, ColdRetire};
#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

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
#[path="../📖️lookup/🦀️.rs"]
mod original_lookup;
pub use original_lookup::OriginalDictionaryLookup;

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
    /// 🪪️ Borrows the exact immutable source root without issuing another ownership alias.
    pub fn original_root_identity(&self)->Option<usize>{self.pairs.original_root_identity()}

    /// 🔗️ Retains only the exact immutable value of one ranked entry.
    pub fn value_shared_at_rank(&self,index:usize)->Option<semio_framework_value::ordered::SharedOwner<Value>> {self.pairs.value_shared_at_rank(index)}

    /// 📍️ Borrows an original ranked entry from the existing ordered dictionary owner.
    pub fn entry_at_rank(&self,index:usize)->Option<(&String,&Value)> {self.pairs.entry_at_rank(index)}

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

    /// 🧬️ Borrows the ordered owner so the budgeted merge hands shared roots over without cloning values.
    pub fn pairs(&self) -> &OrderedMap<Value> {
        &self.pairs
    }

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
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(self.len())?;
            let mut entries = Vec::<(String, DslValue)>::guard_decoded(control.allocate_vec(self.len())?);
            for (key, value) in self.iter() {
                entries.get_mut().push((control.copy_text(key)?, value.to_value_controlled(control)?));
                control.step()?;
            }
            Ok(DslValue::Object(entries.take()))
        }))
    }
}

impl semio_framework_pack_json::JsonWriteSource for Dictionary {
    fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,ValueError> {
        use semio_framework_pack_json::JsonWriteNode;
        let mut dictionary=self;
        for (depth,index) in path.iter().enumerate() {
            let (_,value)=dictionary.entry_at_rank(*index).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"JSON dictionary path is absent"))?;
            if depth+1==path.len() {return Ok(match value {Value::Dictionary(value)=>JsonWriteNode::Object(value.len()),Value::Atom(Atom::Null)=>JsonWriteNode::Null,Value::Atom(Atom::Boolean(value))=>JsonWriteNode::Bool(*value),Value::Atom(Atom::Integer(value))=>JsonWriteNode::Number(Number::Int(*value)),Value::Atom(Atom::Decimal(value))=>JsonWriteNode::Number(Number::Float(*value)),Value::Atom(Atom::String(value))=>JsonWriteNode::String(value)});}
            dictionary=value.as_dictionary().ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"JSON dictionary path crosses a scalar"))?;
        }
        Ok(JsonWriteNode::Object(dictionary.len()))
    }
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError> {
        let mut dictionary=self;
        for position in path {dictionary=dictionary.entry_at_rank(*position).and_then(|(_,value)|value.as_dictionary()).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"JSON dictionary member path is absent"))?;}
        dictionary.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"JSON dictionary member is absent"))
    }
}

#[path="../📤️projection/🦀️.rs"]
mod dictionary_projection;

impl FromValue for Dictionary {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut input=retirement::RetainedDictionaryInput::new(value);
        loop {
            let copy=4096.max(input.next_copy_byte_demand().expect("cold dictionary input copy demand"));
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:input.next_capacity_byte_demand(copy).expect("cold dictionary input capacity demand"),maximum_release_bytes:input.next_release_byte_demand().expect("cold dictionary input release demand"),maximum_depth:input.next_depth_demand().expect("cold dictionary input depth demand")};
            match input.step(grant) {
                Ok(retirement::DictionaryInputStep {dictionary:Some(dictionary),..})=>return Ok(dictionary),
                Ok(_)=>{},
                Err(error)=>{
                    input.cancel();
                    while !input.terminal_is_empty() {
                        let copy=4096.max(input.next_close_copy_byte_demand().expect("cold input close copy demand"));
                        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:input.next_close_capacity_byte_demand(copy).expect("cold input close capacity demand"),maximum_release_bytes:input.next_close_release_byte_demand().expect("cold input close release demand"),maximum_depth:input.next_close_depth_demand().expect("cold input close depth demand")};
                        input.close_step(grant).expect("cold dictionary input closure");
                    }
                    return Err(error);
                },
            }
        }
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Dictionary")); };
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(entries.len())?;
            let mut builder = ColdDictionaryBuilder::new();
            for (key, value) in entries {
                let key = control.copy_text(key)?;
                let value = Value::from_value_controlled(value, control)?;
                builder.insert_controlled(key, value, control)?;
                control.step()?;
            }
            Ok(builder.finish())
        }))
    }
    fn default_value_controlled(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| { control.begin_stage(1)?; control.step()?; Ok(Self::new()) })
    }
    fn retire_decoded(self) { self.retire_cold(); }
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

    /// 🔎️ Borrows schema classification from this evaluator-owned value.
    pub fn kind(&self) -> ValueKind<'_> {
        match self {
            Value::Atom(Atom::Null) => ValueKind::Null,
            Value::Atom(Atom::Boolean(_)) => ValueKind::Boolean,
            Value::Atom(Atom::Integer(_)) => ValueKind::Integer,
            Value::Atom(Atom::Decimal(_)) => ValueKind::Decimal,
            Value::Atom(Atom::String(_)) => ValueKind::Text,
            Value::Dictionary(dictionary) => ValueKind::Dictionary(dictionary.schema()),
        }
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
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        match self { Self::Atom(value) => value.to_value_controlled(control), Self::Dictionary(value) => value.to_value_controlled(control) }
    }
}

impl FromValue for Value {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            object @ DslValue::Object(_) => Ok(Value::Dictionary(Dictionary::from_value(object)?)),
            other => Ok(Value::Atom(Atom::from_value(other)?)),
        }
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        match value { DslValue::Object(_) => Dictionary::from_value_controlled(value, control).map(Self::Dictionary), _ => Atom::from_value_controlled(value, control).map(Self::Atom) }
    }
    fn retire_decoded(self) { self.retire_cold(); }
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
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        match self { Self::Null => ().to_value_controlled(control), Self::Boolean(value) => value.to_value_controlled(control), Self::Integer(value) => value.to_value_controlled(control), Self::Decimal(value) => value.to_value_controlled(control), Self::String(value) => value.to_value_controlled(control) }
    }
}

impl FromValue for Atom {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(Atom::Null),
            DslValue::Bool(b) => Ok(Atom::Boolean(b)),
            DslValue::Number(Number::Int(value)) => Ok(Atom::Integer(value)),
            DslValue::Number(Number::UInt(value)) => i64::try_from(value).map(Atom::Integer).map_err(|_| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unsigned atom exceeds the signed integer range")),
            DslValue::Number(Number::Float(value)) => Ok(Atom::Decimal(value)),
            DslValue::String(s) => Ok(Atom::String(s)),
            DslValue::Bytes(_) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an atom, found bytes")),
            DslValue::Array(_) | DslValue::Object(_) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an atom, found an array or object")),
        }
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => <()>::from_value_controlled(value, control).map(|()| Self::Null),
            DslValue::Bool(_) => bool::from_value_controlled(value, control).map(Self::Boolean),
            DslValue::Number(Number::Int(_) | Number::UInt(_)) => i64::from_value_controlled(value, control).map(Self::Integer),
            DslValue::Number(Number::Float(_)) => f64::from_value_controlled(value, control).map(Self::Decimal),
            DslValue::String(_) => String::from_value_controlled(value, control).map(Self::String),
            _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an atom")),
        }
    }
}

/// 🌲️ Canonical projection follows the `ToValue` wire shape: dictionaries are rank-ordered objects, atoms are JSON scalars.
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Atom {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> {
        use semio_framework_pack_json::ArtifactCanonicalJsonNode as Node;
        Ok(match self { Atom::Null => Node::Null, Atom::Boolean(value) => Node::Bool(*value), Atom::Integer(value) => Node::I64(*value), Atom::Decimal(value) => Node::F64(*value), Atom::String(value) => Node::String(value) })
    }
}

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Dictionary {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> { Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(self.len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, ValueError> {
        self.entry_at_rank(ordinal).map(|(_, value)| value as &dyn semio_framework_pack_json::ArtifactCanonicalJsonTree).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical neural dictionary ordinal is absent"))
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, ValueError> {
        self.entry_at_rank(ordinal).map(|(key, _)| key.as_str().into()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical neural dictionary key is absent"))
    }
}

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Value {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> {
        use semio_framework_pack_json::ArtifactCanonicalJsonTree as CanonicalTree;
        match self { Value::Atom(value) => CanonicalTree::canonical_tree_node(value), Value::Dictionary(value) => CanonicalTree::canonical_tree_node(value) }
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, ValueError> {
        use semio_framework_pack_json::ArtifactCanonicalJsonTree as CanonicalTree;
        match self { Value::Atom(value) => CanonicalTree::canonical_tree_child(value, ordinal), Value::Dictionary(value) => CanonicalTree::canonical_tree_child(value, ordinal) }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, ValueError> {
        use semio_framework_pack_json::ArtifactCanonicalJsonTree as CanonicalTree;
        match self { Value::Atom(value) => CanonicalTree::canonical_tree_key(value, ordinal), Value::Dictionary(value) => CanonicalTree::canonical_tree_key(value, ordinal) }
    }
}
// #endregion 🔖️Dictionary
// #region 🔖️Schema
pub const SCHEMA_KEY: &str = "$schema";

#[cfg(test)]
#[path = "../🧪️tests/🏷️type/🦀️.rs"]
mod value_type_reference;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FieldSpec {
    pub key: String,
    #[cfg_attr(test, serde(with = "value_type_reference"))]
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
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for FieldSpec")) };
        Ok(Self {
            key: value.get("key").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "key"))?,
            value: value.get("value").cloned().map(ValueType::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "value"))?,
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
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Schema")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?,
            module: value.get("module").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "module"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "name"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "icon"))?,
            summary: value.get("summary").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "summary"))?,
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
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for SchemaRef")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "name"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "icon"))?,
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
            if !field.value.matches(value.kind()) {
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
// #endregion 🔖️SchemaComponent
// #region 🔖️Tree
/// 🌳️ Directed acyclic graph of neurons and synapses. `serde` is TEST-ONLY — see `Dictionary`'s
/// docstring; mutually recursive with `Neuron` via `Neuron.tree: Option<Box<Tree>>`, so both moved
/// off serde together.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct Tree {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
}

/// 🔵️ Neuron instance bound to a kind. `serde` is TEST-ONLY — see `Tree`'s docstring above.
#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
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

#[path = "../🚦️native/🦀️.rs"]
mod native_controlled;

impl ToValue for Synapse {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { native_controlled::encode_synapse(self, control) }
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
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { native_controlled::decode_synapse(value, control) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Synapse")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?,
            from: value.get("from").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "from"))?,
            to: value.get("to").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "to"))?,
            from_port: value.get("fromPort").cloned().map(String::from_value).transpose()?.unwrap_or_else(default_from_port),
            to_port: value.get("toPort").cloned().map(String::from_value).transpose()?.unwrap_or_else(default_to_port),
        })
    }
}

impl ToValue for Neuron {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { native_controlled::encode_neuron(self, control) }
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
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { native_controlled::decode_neuron(value, control) }
    fn retire_decoded(self) { native_controlled::retire_neuron(self) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Neuron")) };
        let id = value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?;
        let kind = value.get("kind").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "kind"))?;
        let params = ColdOwner::new(value.get("params").cloned().map(Dictionary::from_value).transpose()?.unwrap_or_default());
        let tree = value.get("tree").cloned().map(Option::<Box<Tree>>::from_value).transpose()?.unwrap_or_default();
        Ok(Self { id, kind, params: params.into_inner(), tree })
    }
}

impl ToValue for Tree {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { native_controlled::encode_tree(self, control) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("neurons".into(), self.neurons.to_value()), ("synapses".into(), self.synapses.to_value())])
    }
}

impl FromValue for Tree {
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { native_controlled::decode_tree(value, control) }
    fn retire_decoded(self) { native_controlled::retire_tree(self) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Tree")) };
        let mut neurons = ColdOwner::new(Vec::new());
        if let Some(entries) = value.get("neurons") {
            let DslValue::Array(entries) = entries else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for neurons")); };
            for (index, entry) in entries.iter().enumerate() {
                neurons.push(Neuron::from_value(entry.clone()).map_err(|error| error.under(index))?);
            }
        }
        let synapses = value.get("synapses").cloned().map(Vec::<Synapse>::from_value).transpose()?.unwrap_or_default();
        Ok(Self { neurons: neurons.into_inner(), synapses })
    }
}
// #endregion 🔖️Tree
// #region 🔖️Contract
pub const INPUT_KIND: &str = "input";
pub const OUTPUT_KIND: &str = "output";
pub const CLUSTER_KIND: &str = "cluster";

pub fn contract_channel(neuron: &Neuron) -> (String, Vec<String>) {
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
#[derive(Clone, Debug, PartialEq,semio_framework_value::RetireOwned)]
pub enum EvalError {
    Retained(ValueError),
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
            EvalError::Retained(error)=>std::fmt::Display::fmt(error,f),
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
impl From<ValueError> for EvalError{fn from(error:ValueError)->Self{Self::Retained(error)}}

/// 🪢️ Keeps the complete evaluation refusal or original native turn failure with its paid receipt.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum OperatorWorkFailure{
 Evaluation{primary:EvalError,retained:RetainedCloneProgress,receipt_refusal:Option<ValueError>},
 Original(semio_framework_job::OriginalTurnFailure),
}
impl OperatorWorkFailure{
 /// 🧾️ Posts one genuine failed evaluation event without formatting or replacing its original primary.
 pub fn debit_evaluation(primary:EvalError,retained:RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>)->Self{Self::Evaluation{primary,retained,receipt_refusal:cx.consume_retained(retained).err()}}
 /// 📏️ Borrows the same failed producer's actual physical effects.
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Evaluation{retained,..}=>*retained,Self::Original(failure)=>failure.primary.retained_progress()}}
}
impl From<EvalError> for OperatorWorkFailure{fn from(primary:EvalError)->Self{let retained=match &primary{EvalError::Retained(primary)=>primary.retained_progress(),_=>Default::default()};Self::Evaluation{primary,retained,receipt_refusal:None}}}
impl From<semio_framework_job::OriginalTurnFailure> for OperatorWorkFailure{fn from(primary:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(primary)}}
impl From<ValueError> for OperatorWorkFailure{fn from(primary:ValueError)->Self{Self::Original(primary.into())}}
#[path="../⚠️failure/🦀️.rs"]
mod original_failure_projection;

/// 📥️ Returns the same original input when an implementation declares no resumable plan.
#[derive(semio_framework_value::RetireOwned)]
pub struct PendingOperatorMetadata{pub extension_id:String,pub operator_id:String}
#[derive(semio_framework_value::RetireOwned)]
pub struct OperatorPendingPlan{pub metadata:protocol::value::ordered::SharedOwner<PendingOperatorMetadata>,pub input:Dictionary}
pub enum OperatorPlanAdmission {Job(Box<dyn OperatorJob>),Immediate(Dictionary),Pending(OperatorPendingPlan)}
impl OperatorPlanAdmission {
    /// 🌊️ Leases original contribution metadata and moves its input without payload work.
    pub fn pending(metadata:&protocol::value::ordered::SharedOwner<PendingOperatorMetadata>,input:Dictionary,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(EvalError,Dictionary)>{if grant.maximum_items==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original pending plan requires an admitted item").into(),input))}if grant.maximum_depth==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original pending plan requires admitted depth").into(),input))}Ok((Self::Pending(OperatorPendingPlan{metadata:metadata.clone(),input}),RetainedCloneProgress{copied_items:1,..Default::default()}))}

    /// 📤️ Hands back the exact original no-plan input with one admitted ownership event and no physical effects.
    pub fn immediate(input:Dictionary,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(EvalError,Dictionary)>{if grant.maximum_items==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original immediate plan requires one item").into(),input))}if grant.maximum_depth==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original immediate plan requires admitted depth").into(),input))}Ok((Self::Immediate(input),RetainedCloneProgress{copied_items:1,..Default::default()}))}
}

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
    /// ⏱️ Advances by at most `budget` units under independently supplied physical credits, preserving the actual receipt through terminal output.
    fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(OperatorJobStep,RetainedCloneProgress),OperatorWorkFailure>;
    /// 🧾️ Preserves the actual last normal receipt through successful and failed child turns.
    fn normal_step_progress(&self)->RetainedCloneProgress;
    fn next_step_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_step_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_step_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_step_depth_demand(&self) -> Result<usize, ValueError>;
    /// 📈️ Progress right now — safe to read between steps and after termination.
    fn progress(&self) -> OperatorProgress;
    /// 🛑️ Retires the job at the next observable boundary. A job that already produced its output
    /// is never retired: supersession may only stop work still in flight.
    fn cancel(&mut self);
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>;
    fn next_close_capacity_byte_demand(&self,maximum_copy_bytes:usize)->Result<usize,ValueError>;
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>;
    fn next_close_depth_demand(&self)->Result<usize,ValueError>;
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn terminal_is_empty(&self)->bool;

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
    /// ⏱️ Admits a concrete plan by moving the exact original input under the caller's independent physical currencies.
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)>;
    fn next_plan_copy_byte_demand(&self,input:&Dictionary)->Result<usize,ValueError>;
    fn next_plan_capacity_byte_demand(&self,input:&Dictionary,maximum_copy_bytes:usize)->Result<usize,ValueError>;
    fn next_plan_release_byte_demand(&self,input:&Dictionary)->Result<usize,ValueError>;
    fn next_plan_depth_demand(&self,input:&Dictionary)->Result<usize,ValueError>;
    /// 🪶️ Only compiler-proven trivial operators are terminal without domain-specific field retirement.
    fn retirement_is_empty(&self) -> bool { !std::mem::needs_drop::<Self>() }
    fn next_retire_copy_byte_demand(&self)->Result<usize,ValueError> {if self.retirement_is_empty(){Ok(0)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator copy ownership is not declared"))}}
    fn next_retire_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError> {if self.retirement_is_empty(){Ok(0)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator capacity ownership is not declared"))}}
    fn next_retire_release_byte_demand(&self)->Result<usize,ValueError> {if self.retirement_is_empty(){Ok(0)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator release ownership is not declared"))}}
    fn next_retire_depth_demand(&self)->Result<usize,ValueError> {if self.retirement_is_empty(){Ok(0)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator depth ownership is not declared"))}}
    /// 🧹️ Retires or transfers actual fields while preserving the original dynamic operator shell.
    fn retire_step(&mut self,_grant:RetainedCloneGrant,_values:&mut ValueRetirement)->Result<RetainedCloneStep,ValueError> {
        if self.retirement_is_empty(){Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator retained ownership is not declared"))}
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
        let DslValue::String(raw) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a string for Cardinality")) };
        Self::from_symbol(&raw).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
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
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for VariadicSpec")) };
        Ok(Self {
            slot_key: value.get("slotKey").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "slotKey"))?,
            min: value.get("min").cloned().map(usize::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "min"))?,
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
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub item_types: Vec<String>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub default: Option<Value>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default))]
    pub cardinality: Cardinality,
}

impl ToValue for ChannelSpec {
    fn to_value(&self) -> DslValue {
        let mut fields = vec![
            ("code".into(), self.code.to_value()),
            ("abbreviation".into(), self.abbreviation.to_value()),
            ("name".into(), self.name.to_value()),
            ("fullName".into(), self.full_name.to_value()),
            ("operators".into(), self.operators.to_value()),
            ("valueTypes".into(), self.value_types.to_value()),
            ("default".into(), self.default.to_value()),
            ("label".into(), self.label.to_value()),
            ("cardinality".into(), self.cardinality.to_value()),
        ];
        if !self.item_types.is_empty() { fields.push(("itemTypes".into(), self.item_types.to_value())); }
        DslValue::Object(fields)
    }
}

impl FromValue for ChannelSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for ChannelSpec")) };
        Ok(Self {
            code: value.get("code").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "code"))?,
            abbreviation: value.get("abbreviation").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "abbreviation"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "name"))?,
            full_name: value.get("fullName").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "fullName"))?,
            operators: value.get("operators").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            value_types: value.get("valueTypes").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            item_types: value.get("itemTypes").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            default: value.get("default").cloned().map(Option::<Value>::from_value).transpose()?.unwrap_or_default(),
            label: value.get("label").cloned().map(Option::<String>::from_value).transpose()?.unwrap_or_default(),
            cardinality: value.get("cardinality").cloned().map(Cardinality::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

pub fn derive_channel_names(name: &str) -> (String, String, String) {
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
        Self { code: code.into(), abbreviation: abbreviation.into(), name: name.into(), full_name: full_name.into(), operators: Vec::new(), value_types: Vec::new(), item_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn requires(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        let name = name.into();
        let (code, abbreviation, full_name) = derive_channel_names(&name);
        Self { code, abbreviation, name, full_name, operators: operators.iter().map(|entry| entry.as_ref().to_string()).collect(), value_types: Vec::new(), item_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn provides(name: impl Into<String>, operators: Vec<String>) -> Self {
        let name = name.into();
        let (code, abbreviation, full_name) = derive_channel_names(&name);
        Self { code, abbreviation, name, full_name, operators, value_types: Vec::new(), item_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
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

    /// 📃️ Declares homogeneous collection element schemas independently of the list's wire schema.
    pub fn with_item_types(mut self, item_types: &[impl AsRef<str>]) -> Self {
        self.item_types = item_types.iter().map(|entry| entry.as_ref().to_string()).collect();
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
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for OperatorInfo")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?,
            extension: value.get("extension").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "extension"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "name"))?,
            abbreviation: value.get("abbreviation").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "abbreviation"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "icon"))?,
            summary: value.get("summary").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "summary"))?,
            inputs: value.get("inputs").cloned().map(Vec::<ChannelSpec>::from_value).transpose()?.unwrap_or_default(),
            outputs: value.get("outputs").cloned().map(Vec::<ChannelSpec>::from_value).transpose()?.unwrap_or_default(),
            variadic_input: value.get("variadicInput").cloned().map(Option::<VariadicSpec>::from_value).transpose()?.unwrap_or_default(),
            variadic_output: value.get("variadicOutput").cloned().map(Option::<VariadicSpec>::from_value).transpose()?.unwrap_or_default(),
            group: value.get("group").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
        })
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

/// 📸️ Retains the exact original tree and seed source allocations of an evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeSnapshot {
    pub(crate) tree: Arc<Tree>,
    pub(crate) seeds: Arc<HistoryFoldIndex<String,Dictionary>>,
}
impl Default for TreeSnapshot{fn default()->Self{Self{tree:Arc::new(Tree::default()),seeds:Arc::new(HistoryFoldIndex::new())}}}
impl TreeSnapshot {
    /// 🧊️ Captures synchronous diagnostic inputs within their existing cold caller scope.
    pub fn capture(tree:&Tree,seeds:&HashMap<String,Dictionary>)->Self{Self{tree:Arc::new(tree.clone()),seeds:Arc::new(seeds.iter().map(|(key,value)|(key.clone(),value.clone())).collect())}}
    /// 🔗️ Moves the exact preborn immutable sources under the original admitted handoff.
    pub fn capture_retained(tree:Arc<Tree>,seeds:Arc<HistoryFoldIndex<String,Dictionary>>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,Arc<Tree>,Arc<HistoryFoldIndex<String,Dictionary>>)>{
        if grant.maximum_items==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original snapshot handoff requires admitted item"),tree,seeds))}
        if grant.maximum_depth==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original snapshot handoff requires admitted depth"),tree,seeds))}
        Ok((Self{tree,seeds},RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    pub fn tree_source(&self)->&Tree{&self.tree}
    pub fn seed_source(&self)->&HistoryFoldIndex<String,Dictionary>{&self.seeds}
}
/// 🧭️ Diagnoses original structural, wiring, seed, removal, and downstream changes synchronously.
pub fn compute_dirty_set(previous:Option<&TreeSnapshot>,current:&TreeSnapshot)->HashSet<String>{
    let Some(previous)=previous else{return current.tree.neurons.iter().map(|node|node.id.clone()).collect()};
    let contains=|id:&str|current.tree.neurons.iter().any(|node|node.id==id);
    let mut direct=HashSet::new();
    for node in &current.tree.neurons{
        let original=previous.tree.neurons.iter().find(|old|old.id==node.id);
        let changed=original.is_none_or(|old|neuron_key_hash(old)!=neuron_key_hash(node)||incoming_edges_signature(&previous.tree,&old.id)!=incoming_edges_signature(&current.tree,&node.id));
        if changed||previous.seeds.get(&node.id)!=current.seeds.get(&node.id){direct.insert(node.id.clone());}
    }
    for removed in &previous.tree.neurons{if !contains(&removed.id){for edge in &previous.tree.synapses{if edge.from==removed.id&&contains(&edge.to){direct.insert(edge.to.clone());}}}}
    let mut dirty=HashSet::new();let mut queue:VecDeque<String>=direct.into_iter().collect();
    while let Some(id)=queue.pop_front(){if !dirty.insert(id.clone()){continue}for edge in &current.tree.synapses{if edge.from==id&&contains(&edge.to)&&!dirty.contains(&edge.to){queue.push_back(edge.to.clone());}}}
    dirty
}
// #endregion 🔖️DirtyPropagation
// #region 🔖️Evaluator

/// 📡️ Resolved neuron inputs and outputs from one evaluation pass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvalChannels {
    pub outputs: HistoryFoldIndex<String, Dictionary>,
    pub inputs: HistoryFoldIndex<String, Dictionary>,
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

pub fn validate_neuron_inputs(acc: &Dictionary, operator_info: Option<&OperatorInfo>) -> Result<(), EvalError> {
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

pub fn validate_operator_outputs(info: &OperatorInfo, output: &Dictionary) -> Result<(), EvalError> {
    if info.variadic_output.is_some() {
        return Ok(());
    }
    for channel in &info.outputs {
        validate_channel_value(channel, output.get(&channel.name))?;
    }
    Ok(())
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

pub fn operator_signature(info: &OperatorInfo, input: &Dictionary) -> Vec<String> {
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
// #endregion 🔖️Evaluator
