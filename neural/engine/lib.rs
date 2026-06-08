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

    pub fn insert(mut self, key: impl Into<String>, value: Value) -> Self {
        self.pairs.insert(key.into(), value);
        self
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.pairs.get(key)
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

// #region 🔖NeuronKind
/// ⚙️ Eval error from a neuron kind.
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
pub trait Function: Send + Sync {
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

/// 🔌 Declared neuron input port with type and optional default.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputSpec {
    pub id: String,
    #[serde(rename = "type")]
    pub value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl InputSpec {
    pub fn new(id: impl Into<String>, value_type: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value_type: value_type.into(),
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

    pub fn number(id: impl Into<String>) -> Self {
        Self::new(id, "number")
    }

    pub fn number_default(id: impl Into<String>, default: f64) -> Self {
        Self::number(id).with_default(Value::Atom(Atom::Decimal(default)))
    }

    pub fn integer_default(id: impl Into<String>, default: i64) -> Self {
        Self::new(id, "integer").with_default(Value::Atom(Atom::Integer(default)))
    }

    pub fn boolean_default(id: impl Into<String>, default: bool) -> Self {
        Self::new(id, "boolean").with_default(Value::Atom(Atom::Boolean(default)))
    }

    pub fn text_default(id: impl Into<String>, default: impl Into<String>) -> Self {
        Self::new(id, "text").with_default(Value::Atom(Atom::String(default.into())))
    }

    pub fn list(id: impl Into<String>) -> Self {
        Self::new(id, "list")
    }

    pub fn dictionary(id: impl Into<String>) -> Self {
        Self::new(id, "dictionary")
    }

    pub fn value(id: impl Into<String>) -> Self {
        Self::new(id, "value")
    }

    pub fn wildcard() -> Self {
        Self::new("*", "value")
    }
}

/// 📇 Catalogue metadata for a neuron kind.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeuronKindInfo {
    pub id: String,
    pub module: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<InputSpec>,
    pub outputs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_input: Option<VariadicSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_output: Option<VariadicSpec>,
}

struct RegistryEntry {
    info: NeuronKindInfo,
    function: Box<dyn Function>,
}

/// 📋 Registry of neuron kinds by id.
pub struct Registry {
    kinds: HashMap<String, RegistryEntry>,
}

impl Default for Registry {
    fn default() -> Self {
        Self { kinds: HashMap::new() }
    }
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, info: NeuronKindInfo, function: Box<dyn Function>) {
        let id = info.id.clone();
        self.kinds.insert(id, RegistryEntry { info, function });
    }

    pub fn get(&self, kind_id: &str) -> Option<&dyn Function> {
        self.kinds.get(kind_id).map(|entry| entry.function.as_ref())
    }

    pub fn kind_info(&self, kind_id: &str) -> Option<&NeuronKindInfo> {
        self.kinds.get(kind_id).map(|entry| &entry.info)
    }

    pub fn catalogue(&self) -> Vec<NeuronKindInfo> {
        let mut items: Vec<NeuronKindInfo> = self.kinds.values().map(|entry| entry.info.clone()).collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }
}
// #endregion 🔖NeuronKind

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
        kind_infos: &HashMap<String, NeuronKindInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<HashMap<String, Dictionary>, EvalError> {
        Ok(self.evaluate_channels_with(tree, seeds, kind_infos, dispatch)?.outputs)
    }

    pub fn evaluate_channels(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        kind_infos: &HashMap<String, NeuronKindInfo>,
    ) -> Result<EvalChannels, EvalError> {
        self.evaluate_channels_with(tree, seeds, kind_infos, &mut |kind, input| {
            self.registry
                .get(kind)
                .ok_or_else(|| EvalError::UnknownKind(kind.into()))?
                .evaluate(input)
        })
    }

    pub fn evaluate_channels_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        kind_infos: &HashMap<String, NeuronKindInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<EvalChannels, EvalError> {
        let order = topo_order(tree)?;
        let mut outputs: HashMap<String, Dictionary> = seeds.clone();
        let mut inputs: HashMap<String, Dictionary> = HashMap::new();
        for neuron_id in order {
            let neuron = tree.neurons.iter().find(|n| n.id == neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
            let kind_info = kind_infos.get(&neuron.kind).or_else(|| self.registry.kind_info(&neuron.kind));
            let input = collect_neuron_input(tree, &outputs, &neuron_id, kind_info);
            inputs.insert(neuron_id.clone(), input.clone());
            if let Some(seed) = seeds.get(&neuron_id) {
                outputs.insert(neuron_id.clone(), seed.clone());
                continue;
            }
            let out = dispatch(&neuron.kind, &input.merge(&neuron.params))?;
            outputs.insert(neuron_id.clone(), out);
        }
        Ok(EvalChannels { outputs, inputs })
    }
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
    match value {
        Value::Dictionary(dict) => {
            if let Some(v) = dict.get("number").or_else(|| dict.get("text")).or_else(|| dict.get("dictionary")) {
                return acc.insert(port_key.to_string(), v.clone());
            }
            if dict.len() == 1 {
                if let Some(v) = dict.keys().next().and_then(|k| dict.get(k)) {
                    return acc.insert(port_key.to_string(), v.clone());
                }
            }
            acc.insert(port_key.to_string(), Value::Dictionary(dict))
        }
        other => acc.insert(port_key.to_string(), other),
    }
}

/// 💉 Fills missing declared input keys from neuron kind defaults.
pub fn inject_input_defaults(acc: Dictionary, kind_info: &NeuronKindInfo) -> Dictionary {
    let mut acc = acc;
    for spec in &kind_info.inputs {
        if spec.id == "*" || acc.get(&spec.id).is_some() {
            continue;
        }
        if let Some(default) = &spec.default {
            acc = acc.insert(spec.id.clone(), default.clone());
        }
    }
    acc
}

fn inject_input_defaults_for_kind(acc: Dictionary, kind_info: Option<&NeuronKindInfo>) -> Dictionary {
    match kind_info {
        Some(info) => inject_input_defaults(acc, info),
        None => acc,
    }
}

fn collect_neuron_input(tree: &Tree, outputs: &HashMap<String, Dictionary>, neuron_id: &str, kind_info: Option<&NeuronKindInfo>) -> Dictionary {
    let mut acc = Dictionary::new();
    let variadic = kind_info.and_then(|info| info.variadic_input.as_ref());
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
    inject_input_defaults_for_kind(acc, kind_info)
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

    impl Function for Echo {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            Ok(input.clone())
        }
    }

    struct Double;

    impl Function for Double {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            let n = input.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number".into()))?;
            Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n * 2.0))))
        }
    }

    #[test]
    fn dictionary_json_round_trip() {
        let d = Dictionary::new().insert("number", Value::Atom(Atom::Decimal(3.1))).insert("text", Value::Atom(Atom::String("hi".into())));
        let json = serde_json::to_string(&d).unwrap();
        let back: Dictionary = serde_json::from_str(&json).unwrap();
        assert_eq!(d, back);
    }

    fn echo_info() -> NeuronKindInfo {
        NeuronKindInfo {
            id: "echo".into(),
            module: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣".into(),
            summary: "Forwards input".into(),
            inputs: vec![InputSpec::value("x")],
            outputs: vec!["x".into()],
            ..Default::default()
        }
    }

    fn double_info() -> NeuronKindInfo {
        NeuronKindInfo {
            id: "double".into(),
            module: "test".into(),
            name: "Double".into(),
            abbreviation: "Dbl".into(),
            icon: "emoji:✖️".into(),
            summary: "Doubles number".into(),
            inputs: vec![InputSpec::number("number")],
            outputs: vec!["number".into()],
            ..Default::default()
        }
    }

    #[test]
    fn registry_dispatches_kind() {
        let mut reg = Registry::new();
        reg.register(echo_info(), Box::new(Echo));
        let out = reg.get("echo").unwrap().evaluate(&Dictionary::new().insert("x", Value::Atom(Atom::Integer(1)))).unwrap();
        assert_eq!(out.get("x").and_then(|v| v.as_atom()), Some(&Atom::Integer(1)));
    }

    #[test]
    fn registry_catalogue_lists_kinds() {
        let mut reg = Registry::new();
        reg.register(echo_info(), Box::new(Echo));
        reg.register(double_info(), Box::new(Double));
        let catalogue = reg.catalogue();
        assert_eq!(catalogue.len(), 2);
        assert_eq!(catalogue[0].id, "double");
        assert_eq!(catalogue[1].id, "echo");
    }

    #[test]
    fn evaluate_with_custom_dispatch() {
        let tree = Tree {
            neurons: vec![Neuron {
                id: "b".into(),
                kind: "double".into(),
                params: Dictionary::new().insert("number", Value::Atom(Atom::Decimal(3.0))),
            }],
            synapses: vec![],
        };
        let out = Evaluator::new(&Registry::new())
            .evaluate_with(&tree, &HashMap::new(), &HashMap::new(), &mut |kind, input| {
                assert_eq!(kind, "double");
                let n = input
                    .get("number")
                    .and_then(|v| v.as_atom())
                    .and_then(|a| a.as_f64())
                    .ok_or_else(|| EvalError::MissingInput("number".into()))?;
                Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n * 2.0))))
            })
            .unwrap();
        assert_eq!(out.get("b").and_then(|d| d.get("number")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(6.0));
    }

    #[test]
    fn two_neuron_pipeline() {
        let mut reg = Registry::new();
        reg.register(echo_info(), Box::new(Echo));
        reg.register(double_info(), Box::new(Double));
        let tree = Tree {
            neurons: vec![
                Neuron { id: "a".into(), kind: "echo".into(), params: Dictionary::new() },
                Neuron { id: "b".into(), kind: "double".into(), params: Dictionary::new() },
            ],
            synapses: vec![Synapse {
                id: "s1".into(),
                from: "a".into(),
                to: "b".into(),
                from_port: "out".into(),
                to_port: "in".into(),
            }],
        };
        let mut seeds = HashMap::new();
        seeds.insert("a".into(), Dictionary::new().insert("number", Value::Atom(Atom::Decimal(2.0))));
        let out = Evaluator::new(&reg).evaluate(&tree, &seeds).unwrap();
        assert_eq!(out.get("b").and_then(|d| d.get("number")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
    }

    #[test]
    fn evaluate_channels_returns_resolved_inputs_per_neuron() {
        let mut reg = Registry::new();
        reg.register(double_info(), Box::new(Double));
        let tree = Tree {
            neurons: vec![Neuron { id: "add".into(), kind: "double".into(), params: Dictionary::new() }],
            synapses: vec![Synapse {
                id: "s1".into(),
                from: "slider".into(),
                to: "add".into(),
                from_port: "out".into(),
                to_port: "number".into(),
            }],
        };
        let mut seeds = HashMap::new();
        seeds.insert("slider".into(), Dictionary::new().insert("number", Value::Atom(Atom::Decimal(3.0))));
        let channels = Evaluator::new(&reg).evaluate_channels(&tree, &seeds, &HashMap::from([(double_info().id.clone(), double_info())])).unwrap();
        let input = channels.inputs.get("add").expect("add input");
        assert_eq!(input.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
        assert_eq!(
            channels.outputs.get("add").and_then(|d| d.get("number")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()),
            Some(6.0)
        );
    }

    #[test]
    fn collect_routes_fixed_port_by_key() {
        let tree = Tree {
            neurons: vec![Neuron { id: "add".into(), kind: "math.add".into(), params: Dictionary::new() }],
            synapses: vec![
                Synapse {
                    id: "s1".into(),
                    from: "slider".into(),
                    to: "add".into(),
                    from_port: "out".into(),
                    to_port: "a".into(),
                },
                Synapse {
                    id: "s2".into(),
                    from: "note".into(),
                    to: "add".into(),
                    from_port: "out".into(),
                    to_port: "b".into(),
                },
            ],
        };
        let mut outputs = HashMap::new();
        outputs.insert("slider".into(), Dictionary::new().insert("number", Value::Atom(Atom::Decimal(2.0))));
        outputs.insert("note".into(), Dictionary::new().insert("number", Value::Atom(Atom::Decimal(3.0))));
        let input = collect_neuron_input(&tree, &outputs, "add", None);
        assert_eq!(input.get("a").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
        assert_eq!(input.get("b").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn collect_routes_variadic_slots_in_order() {
        let kind = NeuronKindInfo {
            id: "dictionary.merge".into(),
            module: "dictionary".into(),
            name: "Merge".into(),
            abbreviation: "Merge".into(),
            icon: "emoji:🔀".into(),
            summary: "Merge".into(),
            inputs: vec![],
            outputs: vec!["dictionary".into()],
            variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
            ..Default::default()
        };
        let tree = Tree {
            neurons: vec![Neuron { id: "merge".into(), kind: "dictionary.merge".into(), params: Dictionary::new() }],
            synapses: vec![
                Synapse {
                    id: "s1".into(),
                    from: "a".into(),
                    to: "merge".into(),
                    from_port: "out".into(),
                    to_port: "0".into(),
                },
                Synapse {
                    id: "s2".into(),
                    from: "b".into(),
                    to: "merge".into(),
                    from_port: "out".into(),
                    to_port: "1".into(),
                },
            ],
        };
        let mut outputs = HashMap::new();
        outputs.insert(
            "a".into(),
            Dictionary::new().insert("dictionary", Value::Dictionary(Dictionary::new().insert("x", Value::Atom(Atom::Decimal(1.0))))),
        );
        outputs.insert(
            "b".into(),
            Dictionary::new().insert("dictionary", Value::Dictionary(Dictionary::new().insert("y", Value::Atom(Atom::Decimal(2.0))))),
        );
        let input = collect_neuron_input(&tree, &outputs, "merge", Some(&kind));
        let items = input.get("items").and_then(|v| v.as_dictionary()).expect("items");
        assert!(items.get("0").is_some());
        assert!(items.get("1").is_some());
    }

    #[test]
    fn collect_injects_declared_defaults_for_unconnected_inputs() {
        let kind = NeuronKindInfo {
            id: "list.get".into(),
            module: "list".into(),
            name: "Get".into(),
            abbreviation: "Get".into(),
            icon: "emoji:🔍".into(),
            summary: "Get".into(),
            inputs: vec![
                InputSpec::list("list"),
                InputSpec::number_default("index", 0.0),
                InputSpec::boolean_default("wrap", false),
            ],
            outputs: vec!["value".into()],
            ..Default::default()
        };
        let tree = Tree {
            neurons: vec![Neuron { id: "get".into(), kind: "list.get".into(), params: Dictionary::new() }],
            synapses: vec![],
        };
        let input = collect_neuron_input(&tree, &HashMap::new(), "get", Some(&kind));
        assert_eq!(input.get("index").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
        assert_eq!(input.get("wrap").and_then(|v| v.as_atom()).and_then(|a| a.as_bool()), Some(false));
    }
}
// #endregion 🔖Tests
