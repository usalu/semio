//! 🧠 Headless neural engine: dictionary in, dictionary out.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};

// #region 🔖Dictionary
/// 📚 Immutable, unordered, collision-free key-value collection.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Dictionary {
    pairs: BTreeMap<String, Value>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_schema(schema: impl Into<String>) -> Self {
        Self::new().insert(SCHEMA_KEY, Value::Atom(Atom::String(schema.into())))
    }

    pub fn insert(mut self, key: impl Into<String>, value: Value) -> Self {
        self.pairs.insert(key.into(), value);
        self
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

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn merge(&self, other: &Dictionary) -> Dictionary {
        let mut pairs = self.pairs.clone();
        for (k, v) in &other.pairs {
            pairs.insert(k.clone(), v.clone());
        }
        Dictionary { pairs }
    }
}

/// 🔑 Dot-separated camelCase segment path.
pub type Key = String;

/// 💎 Atom or nested dictionary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Atom(Atom),
    Dictionary(Dictionary),
}

impl Value {
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

/// ⚛️ Immutable non-dictionary value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Atom {
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    String(String),
}

impl Atom {
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
// #endregion 🔖Dictionary

// #region 🔖Schema
pub const SCHEMA_KEY: &str = "$schema";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "of")]
pub enum ValueType {
    Boolean,
    Integer,
    Decimal,
    Text,
    List(Box<ValueType>),
    Schema(String),
    Any,
}

impl Default for ValueType {
    fn default() -> Self {
        Self::Any
    }
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldSpec {
    pub key: String,
    pub value: ValueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl FieldSpec {
    pub fn new(key: impl Into<String>, value: ValueType) -> Self {
        Self { key: key.into(), value, default: None, label: None }
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    pub fn decimal(key: impl Into<String>) -> Self {
        Self::new(key, ValueType::Decimal)
    }

    pub fn decimal_default(key: impl Into<String>, default: f64) -> Self {
        Self::decimal(key).with_default(Value::Atom(Atom::Decimal(default)))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub id: String,
    pub module: String,
    pub name: String,
    pub icon: String,
    pub summary: String,
    pub fields: Vec<FieldSpec>,
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
// #endregion 🔖Schema

// #region 🔖Tree
/// 🌳 Directed acyclic graph of neurons and synapses.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
}

/// 🔵 Neuron instance bound to a kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Neuron {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: Dictionary,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<Box<Tree>>,
}

impl Neuron {
    pub fn with_kind(id: impl Into<String>, kind: impl Into<String>, params: Dictionary) -> Self {
        Self {
            id: id.into(),
            kind: kind.into(),
            params,
            tree: None,
        }
    }
}

impl Tree {
    /// 📜 Derives contract input and output channels from boundary neurons.
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
    "out".into()
}

fn default_to_port() -> String {
    "in".into()
}

/// 🔗 Directed connection between two port endpoints.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Synapse {
    pub id: String,
    pub from: String,
    pub to: String,
    #[serde(default = "default_from_port")]
    pub from_port: String,
    #[serde(default = "default_to_port")]
    pub to_port: String,
}
// #endregion 🔖Tree

// #region 🔖Contract
pub const INPUT_KIND: &str = "input";
pub const OUTPUT_KIND: &str = "output";
pub const CLUSTER_KIND: &str = "cluster";

fn contract_channel(neuron: &Neuron) -> (String, Vec<String>) {
    let channel_id = neuron
        .params
        .get("channel")
        .and_then(|value| value.as_atom())
        .and_then(|atom| atom.as_str())
        .unwrap_or(neuron.id.as_str())
        .to_string();
    let operators = neuron
        .params
        .get("operators")
        .and_then(|value| value.as_atom())
        .and_then(|atom| atom.as_str())
        .map(|raw| raw.split(',').map(str::trim).filter(|entry| !entry.is_empty()).map(str::to_string).collect())
        .unwrap_or_default();
    (channel_id, operators)
}

/// 🧩 Builds operator metadata for a cluster neuron from its inner contract.
pub fn cluster_operator_info(id: &str, name: &str, tree: &Tree) -> OperatorInfo {
    let (inputs, outputs) = tree.contract();
    OperatorInfo {
        id: id.into(),
        module: "flow".into(),
        name: name.into(),
        abbreviation: name.into(),
        icon: "emoji:🧩".into(),
        summary: "Nested tree operator".into(),
        inputs,
        outputs,
        ..Default::default()
    }
}
// #endregion 🔖Contract

// #region 🔖Operator
/// ⚙️ Eval error from an operator.
#[derive(Clone, Debug, PartialEq)]
pub enum EvalError {
    UnknownKind(String),
    MissingInput(String),
    InvalidInput(String),
    CycleDetected,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::UnknownKind(k) => write!(f, "unknown kind: {k}"),
            EvalError::MissingInput(k) => write!(f, "missing input: {k}"),
            EvalError::InvalidInput(m) => write!(f, "invalid input: {m}"),
            EvalError::CycleDetected => write!(f, "cycle detected"),
        }
    }
}

impl std::error::Error for EvalError {}

/// 🧮 Computational unit: one dictionary to another.
pub trait Operation: Send + Sync {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError>;
}

/// ➕ Variadic input or output slot specification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariadicSpec {
    pub slot_key: String,
    pub min: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
}

/// 🔌 Declared operator channel with required/provided operator capabilities.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSpec {
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operators: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl ChannelSpec {
    pub fn requires(id: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self {
            id: id.into(),
            operators: operators.iter().map(|entry| entry.as_ref().to_string()).collect(),
            default: None,
            label: None,
        }
    }

    pub fn provides(id: impl Into<String>, operators: Vec<String>) -> Self {
        Self {
            id: id.into(),
            operators,
            default: None,
            label: None,
        }
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn number(id: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(id, operators)
    }

    pub fn number_default(id: impl Into<String>, default: f64, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(id, operators).with_default(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(default)))))
    }

    pub fn integer_default(id: impl Into<String>, default: i64, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(id, operators).with_default(Value::Atom(Atom::Integer(default)))
    }

    pub fn boolean_default(id: impl Into<String>, default: bool, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(id, operators)
            .with_default(Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(default)))))
    }

    pub fn text_default(id: impl Into<String>, default: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(id, operators)
            .with_default(Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(default.into())))))
    }

    pub fn list(id: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(id, operators)
    }

    pub fn dictionary(id: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(id, operators)
    }

    pub fn any(id: impl Into<String>) -> Self {
        Self::requires(id, &[] as &[&str])
    }

    pub fn wildcard() -> Self {
        Self::any("*")
    }
}

/// 📇 Catalogue metadata for an operator.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorInfo {
    pub id: String,
    pub module: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<ChannelSpec>,
    pub outputs: Vec<ChannelSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_input: Option<VariadicSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_output: Option<VariadicSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<String>,
}

pub struct OperatorImpl {
    pub schemas: Vec<String>,
    pub operation: Box<dyn Operation>,
}

pub struct Operator {
    pub info: OperatorInfo,
    pub implementations: Vec<OperatorImpl>,
}

/// 📋 Registry of schemas and operators by id.
#[derive(Default)]
pub struct Registry {
    schemas: HashMap<String, Schema>,
    operators: HashMap<String, Operator>,
    operator_produces: HashMap<String, Vec<String>>,
    schema_providers: HashMap<String, HashSet<String>>,
    finalized: bool,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_schema(&mut self, schema: Schema) {
        self.schemas.insert(schema.id.clone(), schema);
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
        self.operators.insert(id, Operator { info, implementations });
        self.finalized = false;
    }

    pub fn finalize(&mut self) {
        if self.finalized {
            return;
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
        let mut operators: Vec<String> = self
            .schema_providers
            .get(schema_id)
            .map(|entries| entries.iter().cloned().collect())
            .unwrap_or_default();
        operators.sort();
        operators
    }

    pub fn channel_compatible(output: &ChannelSpec, input: &ChannelSpec) -> bool {
        if input.operators.is_empty() {
            return true;
        }
        input.operators.iter().all(|required| output.operators.iter().any(|provided| provided == required))
    }

    pub fn schema(&self, schema_id: &str) -> Option<&Schema> {
        self.schemas.get(schema_id)
    }

    pub fn operator(&self, operator_id: &str) -> Option<&Operator> {
        self.operators.get(operator_id)
    }

    pub fn operator_info(&self, operator_id: &str) -> Option<&OperatorInfo> {
        self.operators.get(operator_id).map(|entry| &entry.info)
    }

    pub fn schema_catalogue(&self) -> Vec<Schema> {
        let mut items: Vec<Schema> = self.schemas.values().cloned().collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }

    pub fn operator_catalogue(&self) -> Vec<OperatorInfo> {
        let mut items: Vec<OperatorInfo> = self
            .operators
            .values()
            .map(|entry| Self::finalize_operator_info(&entry.info, self.operator_produces.get(&entry.info.id), &self.schema_providers))
            .collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }

    fn finalize_operator_info(
        info: &OperatorInfo,
        produces: Option<&Vec<String>>,
        schema_providers: &HashMap<String, HashSet<String>>,
    ) -> OperatorInfo {
        let mut finalized = info.clone();
        let produces = produces.cloned().unwrap_or_default();
        for channel in &mut finalized.outputs {
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
        finalized
    }

    pub fn dispatch(&self, operator_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
        let signature = operator_signature(&operator.info, input);
        let implementation = operator
            .implementations
            .iter()
            .find(|implementation| implementation.schemas == signature)
            .or_else(|| operator.implementations.iter().find(|implementation| implementation.schemas.is_empty()))
            .ok_or_else(|| EvalError::InvalidInput(format!("no implementation for {operator_id}({})", signature.join(", "))))?;
        implementation.operation.evaluate(input)
    }
}
// #endregion 🔖Operator

// #region 🔖Evaluator
/// 📡 Resolved neuron inputs and outputs from one evaluation pass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvalChannels {
    pub outputs: HashMap<String, Dictionary>,
    pub inputs: HashMap<String, Dictionary>,
}

/// 🔄 Topological evaluation over a neural tree.
pub struct Evaluator<'a> {
    registry: &'a Registry,
}

impl<'a> Evaluator<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    pub fn evaluate(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Result<HashMap<String, Dictionary>, EvalError> {
        Ok(self.evaluate_channels(tree, seeds, &HashMap::new())?.outputs)
    }

    pub fn evaluate_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<HashMap<String, Dictionary>, EvalError> {
        Ok(self.evaluate_channels_with(tree, seeds, operator_infos, dispatch)?.outputs)
    }

    pub fn evaluate_channels(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
    ) -> Result<EvalChannels, EvalError> {
        self.evaluate_channels_with(tree, seeds, operator_infos, &mut |kind, input| self.registry.dispatch(kind, input))
    }

    pub fn evaluate_channels_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<EvalChannels, EvalError> {
        let order = topo_order(tree)?;
        let mut outputs: HashMap<String, Dictionary> = seeds.clone();
        let mut inputs: HashMap<String, Dictionary> = HashMap::new();
        for neuron_id in order {
            let neuron = tree.neurons.iter().find(|n| n.id == neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
            let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
            let input = collect_neuron_input(tree, &outputs, &neuron_id, operator_info);
            inputs.insert(neuron_id.clone(), input.clone());
            if let Some(seed) = seeds.get(&neuron_id) {
                outputs.insert(neuron_id.clone(), seed.clone());
                continue;
            }
            if let Some(sub_tree) = neuron.tree.as_deref() {
                let out = self.evaluate_cluster(sub_tree, &input, operator_infos, dispatch)?;
                outputs.insert(neuron_id.clone(), out);
                continue;
            }
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                outputs.insert(neuron_id.clone(), input.merge(&neuron.params));
                continue;
            }
            let out = dispatch(&neuron.kind, &input.merge(&neuron.params))?;
            outputs.insert(neuron_id.clone(), out);
        }
        Ok(EvalChannels { outputs, inputs })
    }

    fn evaluate_cluster(
        &self,
        sub_tree: &Tree,
        parent_input: &Dictionary,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<Dictionary, EvalError> {
        let mut sub_seeds = HashMap::new();
        for neuron in &sub_tree.neurons {
            if neuron.kind != INPUT_KIND {
                continue;
            }
            let (channel_id, _) = contract_channel(neuron);
            let Some(value) = parent_input.get(&channel_id) else { continue };
            sub_seeds.insert(neuron.id.clone(), boundary_seed_dictionary(value));
        }
        let sub_channels = self.evaluate_channels_with(sub_tree, &sub_seeds, operator_infos, dispatch)?;
        let mut out = Dictionary::new();
        for neuron in &sub_tree.neurons {
            if neuron.kind != OUTPUT_KIND {
                continue;
            }
            let (channel_id, _) = contract_channel(neuron);
            let Some(neuron_input) = sub_channels.inputs.get(&neuron.id) else {
                return Err(EvalError::MissingInput(format!("cluster output boundary {channel_id}")));
            };
            let Some(value) = boundary_output_value(neuron_input) else {
                return Err(EvalError::MissingInput(format!("cluster output boundary {channel_id}")));
            };
            out = out.insert(channel_id, value);
        }
        Ok(out)
    }
}

fn operator_info_for_neuron<'a>(
    neuron: &Neuron,
    operator_infos: &'a HashMap<String, OperatorInfo>,
    registry_info: Option<&'a OperatorInfo>,
) -> Option<&'a OperatorInfo> {
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

fn synapse_source_value(src_out: &Dictionary, from_port: &str) -> Value {
    if from_port.is_empty() || from_port == "out" {
        return Value::Dictionary(src_out.clone());
    }
    src_out.get(from_port).cloned().unwrap_or(Value::Dictionary(src_out.clone()))
}

fn insert_variadic_slot(acc: Dictionary, slot_key: &str, port_id: &str, value: Value) -> Dictionary {
    let mut slots = acc.get(slot_key).and_then(|v| v.as_dictionary()).cloned().unwrap_or_default();
    slots = slots.insert(port_id.to_string(), value);
    acc.insert(slot_key.to_string(), Value::Dictionary(slots))
}

fn insert_fixed_port(acc: Dictionary, port_key: &str, value: Value) -> Dictionary {
    acc.insert(port_key.to_string(), value)
}

/// 💉 Fills missing declared input keys from operator channel defaults.
pub fn inject_channel_defaults(acc: Dictionary, operator_info: &OperatorInfo) -> Dictionary {
    let mut acc = acc;
    for spec in &operator_info.inputs {
        if spec.id == "*" || acc.get(&spec.id).is_some() {
            continue;
        }
        if let Some(default) = &spec.default {
            acc = acc.insert(spec.id.clone(), default.clone());
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

fn collect_neuron_input(tree: &Tree, outputs: &HashMap<String, Dictionary>, neuron_id: &str, operator_info: Option<&OperatorInfo>) -> Dictionary {
    let mut acc = Dictionary::new();
    let variadic = operator_info.and_then(|info| info.variadic_input.as_ref());
    for syn in &tree.synapses {
        if syn.to != neuron_id {
            continue;
        }
        let Some(src_out) = outputs.get(&syn.from) else { continue };
        let value = synapse_source_value(src_out, &syn.from_port);
        if let Some(spec) = variadic {
            let port_id = if syn.to_port.is_empty() || syn.to_port == "in" { "0" } else { syn.to_port.as_str() };
            acc = insert_variadic_slot(acc, &spec.slot_key, port_id, value);
            continue;
        }
        if syn.to_port.is_empty() || syn.to_port == "in" {
            if let Value::Dictionary(dict) = value {
                acc = acc.merge(&dict);
            }
            continue;
        }
        acc = insert_fixed_port(acc, &syn.to_port, value);
    }
    inject_channel_defaults_for_operator(acc, operator_info)
}

fn channel_schema(input: &Dictionary, channel: &ChannelSpec) -> String {
    input
        .get(&channel.id)
        .and_then(|value| value.as_dictionary())
        .and_then(|dictionary| dictionary.schema())
        .map(str::to_string)
        .or_else(|| {
            channel
                .default
                .as_ref()
                .and_then(|value| value.as_dictionary())
                .and_then(|dictionary| dictionary.schema())
                .map(str::to_string)
        })
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
                keys.into_iter()
                    .filter_map(|index| items.get(&index.to_string()))
                    .filter_map(|value| value.as_dictionary())
                    .filter_map(|dictionary| dictionary.schema())
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();
    }
    info.inputs.iter().filter(|channel| channel.id != "*").map(|channel| channel_schema(input, channel)).collect()
}

fn topo_order(tree: &Tree) -> Result<Vec<String>, EvalError> {
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
    let mut order = Vec::new();
    while let Some(n) = queue.pop_front() {
        order.push(n.clone());
        for syn in &tree.synapses {
            if syn.from != n {
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
    if order.len() != ids.len() {
        return Err(EvalError::CycleDetected);
    }
    Ok(order)
}
// #endregion 🔖Evaluator

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    struct Echo;

    impl Operation for Echo {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            Ok(input.clone())
        }
    }

    struct Double;

    impl Operation for Double {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            let value = input
                .get("number")
                .and_then(|v| v.as_dictionary())
                .and_then(|d| d.get("value"))
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput("number.value".into()))?;
            Ok(number_dictionary(value * 2.0))
        }
    }

    fn number_schema() -> Schema {
        Schema {
            id: "number".into(),
            module: "core".into(),
            name: "Number".into(),
            icon: "emoji:#".into(),
            summary: "Number dictionary".into(),
            fields: vec![FieldSpec::decimal_default("value", 0.0)],
        }
    }

    fn number_dictionary(value: f64) -> Dictionary {
        Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
    }

    fn echo_info() -> OperatorInfo {
        OperatorInfo {
            id: "echo".into(),
            module: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣".into(),
            summary: "Forwards input".into(),
            inputs: vec![ChannelSpec::any("x")],
            outputs: vec![ChannelSpec::provides("out", vec![])],
            ..Default::default()
        }
    }

    fn double_info() -> OperatorInfo {
        OperatorInfo {
            id: "double".into(),
            module: "test".into(),
            name: "Double".into(),
            abbreviation: "Dbl".into(),
            icon: "emoji:✖️".into(),
            summary: "Doubles number".into(),
            inputs: vec![ChannelSpec::number("number", &["double"])],
            outputs: vec![ChannelSpec::provides("out", vec![])],
            ..Default::default()
        }
    }

    #[test]
    fn dictionary_schema_round_trip() {
        let d = number_dictionary(3.1);
        let json = serde_json::to_string(&d).unwrap();
        let back: Dictionary = serde_json::from_str(&json).unwrap();
        assert_eq!(back.schema(), Some("number"));
        assert_eq!(d, back);
    }

    #[test]
    fn schema_validates_required_fields() {
        let schema = number_schema();
        let d = number_dictionary(2.0);
        schema.validate(&d).unwrap();
        assert!(schema.validate(&Dictionary::with_schema("point")).is_err());
    }

    #[test]
    fn registry_dispatches_operator() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operation: Box::new(Echo) }], &[]);
        let out = reg.dispatch("echo", &Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0)))).unwrap();
        assert!(out.get("x").is_some());
    }

    #[test]
    fn registry_catalogue_lists_operators_and_schemas() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operation: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operation: Box::new(Double) }], &["number"]);
        assert_eq!(reg.schema_catalogue()[0].id, "number");
        assert_eq!(reg.operator_catalogue()[0].id, "double");
        assert_eq!(reg.operator_catalogue()[1].id, "echo");
    }

    #[test]
    fn evaluate_with_custom_dispatch() {
        let tree = Tree {
            neurons: vec![Neuron::with_kind(
                "b",
                "double",
                Dictionary::new().insert("number", Value::Dictionary(number_dictionary(3.0))),
            )],
            synapses: vec![],
        };
        let out = Evaluator::new(&Registry::new())
            .evaluate_with(&tree, &HashMap::new(), &HashMap::new(), &mut |kind, input| {
                assert_eq!(kind, "double");
                Double.evaluate(input)
            })
            .unwrap();
        assert_eq!(out.get("b").and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(6.0));
    }

    #[test]
    fn two_neuron_pipeline() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operation: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operation: Box::new(Double) }], &["number"]);
        let tree = Tree {
            neurons: vec![
                Neuron::with_kind("a", "echo", number_dictionary(2.0)),
                Neuron::with_kind("b", "double", Dictionary::new()),
            ],
            synapses: vec![Synapse {
                id: "s1".into(),
                from: "a".into(),
                to: "b".into(),
                from_port: "out".into(),
                to_port: "number".into(),
            }],
        };
        let out = Evaluator::new(&reg).evaluate(&tree, &HashMap::new()).unwrap();
        assert_eq!(out.get("b").and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
    }

    #[test]
    fn evaluate_channels_returns_resolved_inputs_per_neuron() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operation: Box::new(Double) }], &["number"]);
        let tree = Tree {
            neurons: vec![Neuron::with_kind("add", "double", Dictionary::new())],
            synapses: vec![Synapse {
                id: "s1".into(),
                from: "slider".into(),
                to: "add".into(),
                from_port: "out".into(),
                to_port: "number".into(),
            }],
        };
        let mut seeds = HashMap::new();
        seeds.insert("slider".into(), number_dictionary(3.0));
        let channels = Evaluator::new(&reg).evaluate_channels(&tree, &seeds, &HashMap::from([(double_info().id.clone(), double_info())])).unwrap();
        assert_eq!(channels.inputs.get("add").and_then(|d| d.get("number")).and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
        assert_eq!(
            channels.outputs.get("add").and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()),
            Some(6.0)
        );
    }

    #[test]
    fn collect_routes_fixed_port_by_key() {
        let tree = Tree {
            neurons: vec![Neuron::with_kind("add", "math.add", Dictionary::new())],
            synapses: vec![
                Synapse { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: "out".into(), to_port: "a".into() },
                Synapse { id: "s2".into(), from: "note".into(), to: "add".into(), from_port: "out".into(), to_port: "b".into() },
            ],
        };
        let mut outputs = HashMap::new();
        outputs.insert("slider".into(), number_dictionary(2.0));
        outputs.insert("note".into(), number_dictionary(3.0));
        let input = collect_neuron_input(&tree, &outputs, "add", None);
        assert_eq!(input.get("a").and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
        assert_eq!(input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
    }

    #[test]
    fn collect_routes_variadic_slots_in_order() {
        let operator = OperatorInfo {
            id: "dictionary.merge".into(),
            module: "dictionary".into(),
            name: "Merge".into(),
            abbreviation: "Merge".into(),
            icon: "emoji:🔀".into(),
            summary: "Merge".into(),
            inputs: vec![],
            outputs: vec![ChannelSpec::provides("out", vec![])],
            variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
            ..Default::default()
        };
        let tree = Tree {
            neurons: vec![Neuron::with_kind("merge", "dictionary.merge", Dictionary::new())],
            synapses: vec![
                Synapse { id: "s1".into(), from: "a".into(), to: "merge".into(), from_port: "out".into(), to_port: "0".into() },
                Synapse { id: "s2".into(), from: "b".into(), to: "merge".into(), from_port: "out".into(), to_port: "1".into() },
            ],
        };
        let mut outputs = HashMap::new();
        outputs.insert("a".into(), Dictionary::with_schema("dictionary"));
        outputs.insert("b".into(), Dictionary::with_schema("dictionary"));
        let input = collect_neuron_input(&tree, &outputs, "merge", Some(&operator));
        let items = input.get("items").and_then(|v| v.as_dictionary()).expect("items");
        assert!(items.get("0").is_some());
        assert!(items.get("1").is_some());
    }

    struct AddNumbers;

    impl Operation for AddNumbers {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            let a = input
                .get("a")
                .and_then(|v| v.as_dictionary())
                .and_then(|d| d.get("value"))
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput("a".into()))?;
            let b = input
                .get("b")
                .and_then(|v| v.as_dictionary())
                .and_then(|d| d.get("value"))
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput("b".into()))?;
            Ok(number_dictionary(a + b))
        }
    }

    fn add_info() -> OperatorInfo {
        OperatorInfo {
            id: "math.add".into(),
            module: "math".into(),
            name: "Add".into(),
            abbreviation: "Add".into(),
            icon: "emoji:➕".into(),
            summary: "Adds numbers".into(),
            inputs: vec![ChannelSpec::number("a", &["math.add"]), ChannelSpec::number("b", &["math.add"])],
            outputs: vec![ChannelSpec::provides("out", vec![])],
            ..Default::default()
        }
    }

    fn input_boundary(id: &str, channel: &str) -> Neuron {
        Neuron::with_kind(
            id,
            INPUT_KIND,
            Dictionary::new()
                .insert("channel", Value::Atom(Atom::String(channel.into())))
                .insert("operators", Value::Atom(Atom::String("math.add".into()))),
        )
    }

    fn output_boundary(id: &str, channel: &str) -> Neuron {
        Neuron::with_kind(
            id,
            OUTPUT_KIND,
            Dictionary::new()
                .insert("channel", Value::Atom(Atom::String(channel.into())))
                .insert("operators", Value::Atom(Atom::String("math.add".into()))),
        )
    }

    #[test]
    fn cluster_contract_derives_channels() {
        let tree = Tree {
            neurons: vec![input_boundary("in_a", "a"), input_boundary("in_b", "b"), output_boundary("out_sum", "sum")],
            synapses: vec![],
        };
        let (inputs, outputs) = tree.contract();
        assert_eq!(inputs.len(), 2);
        assert_eq!(outputs.len(), 1);
        assert_eq!(inputs[0].id, "a");
        assert_eq!(outputs[0].id, "sum");
        let info = cluster_operator_info("cluster-1", "Add cluster", &tree);
        assert_eq!(info.inputs.len(), 2);
        assert_eq!(info.outputs[0].id, "sum");
    }

    #[test]
    fn cluster_runs_inner_tree() {
        let inner = Tree {
            neurons: vec![
                input_boundary("in_a", "a"),
                input_boundary("in_b", "b"),
                Neuron::with_kind("add", "math.add", Dictionary::new()),
                output_boundary("out_sum", "sum"),
            ],
            synapses: vec![
                Synapse { id: "s1".into(), from: "in_a".into(), to: "add".into(), from_port: "out".into(), to_port: "a".into() },
                Synapse { id: "s2".into(), from: "in_b".into(), to: "add".into(), from_port: "out".into(), to_port: "b".into() },
                Synapse { id: "s3".into(), from: "add".into(), to: "out_sum".into(), from_port: "out".into(), to_port: "in".into() },
            ],
        };
        let tree = Tree {
            neurons: vec![
                Neuron::with_kind("a_src", "core.number", Dictionary::new()),
                Neuron::with_kind("b_src", "core.number", Dictionary::new()),
                Neuron {
                    id: "cluster".into(),
                    kind: CLUSTER_KIND.into(),
                    params: Dictionary::new(),
                    tree: Some(Box::new(inner)),
                },
            ],
            synapses: vec![
                Synapse { id: "s_a".into(), from: "a_src".into(), to: "cluster".into(), from_port: "out".into(), to_port: "a".into() },
                Synapse { id: "s_b".into(), from: "b_src".into(), to: "cluster".into(), from_port: "out".into(), to_port: "b".into() },
            ],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operation: Box::new(AddNumbers) }], &["number"]);
        let mut seeds = HashMap::new();
        seeds.insert("a_src".into(), number_dictionary(2.0));
        seeds.insert("b_src".into(), number_dictionary(3.0));
        let out = Evaluator::new(&reg).evaluate(&tree, &seeds).unwrap();
        assert_eq!(
            out.get("cluster").and_then(|d| d.get("sum")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()),
            Some(5.0)
        );
    }

    #[test]
    fn cluster_shakability_round_trip() {
        let inner = Tree {
            neurons: vec![input_boundary("in_a", "a"), output_boundary("out_a", "a")],
            synapses: vec![Synapse { id: "s1".into(), from: "in_a".into(), to: "out_a".into(), from_port: "out".into(), to_port: "in".into() }],
        };
        let tree = Tree {
            neurons: vec![
                Neuron::with_kind("a_src", "core.number", Dictionary::new()),
                Neuron {
                    id: "cluster".into(),
                    kind: CLUSTER_KIND.into(),
                    params: Dictionary::new(),
                    tree: Some(Box::new(inner)),
                },
            ],
            synapses: vec![Synapse { id: "s0".into(), from: "a_src".into(), to: "cluster".into(), from_port: "out".into(), to_port: "a".into() }],
        };
        let json = serde_json::to_string(&tree).unwrap();
        let back: Tree = serde_json::from_str(&json).unwrap();
        let mut seeds = HashMap::new();
        seeds.insert("a_src".into(), number_dictionary(7.0));
        let out = Evaluator::new(&Registry::new()).evaluate(&back, &seeds).unwrap();
        assert_eq!(
            out.get("cluster").and_then(|d| d.get("a")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()),
            Some(7.0)
        );
    }

    #[test]
    fn collect_injects_declared_defaults_for_unconnected_inputs() {
        let operator = OperatorInfo {
            id: "list.get".into(),
            module: "list".into(),
            name: "Get".into(),
            abbreviation: "Get".into(),
            icon: "emoji:🔍".into(),
            summary: "Get".into(),
            inputs: vec![
                ChannelSpec::list("list", &["list.get"]),
                ChannelSpec::number_default("index", 0.0, &["list.get"]),
                ChannelSpec::boolean_default("wrap", false, &["list.get"]),
            ],
            outputs: vec![ChannelSpec::any("out")],
            ..Default::default()
        };
        let tree = Tree {
            neurons: vec![Neuron::with_kind("get", "list.get", Dictionary::new())],
            synapses: vec![],
        };
        let input = collect_neuron_input(&tree, &HashMap::new(), "get", Some(&operator));
        assert_eq!(
            input.get("index").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()),
            Some(0.0)
        );
        assert_eq!(
            input.get("wrap").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()),
            Some(false)
        );
    }
}
// #endregion 🔖Tests
