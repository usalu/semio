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
    Integer(i64),
    Decimal(f64),
    String(String),
}

impl Atom {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
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

/// 🔗 Directed connection between two port endpoints.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Synapse {
    pub id: String,
    pub from: String,
    pub to: String,
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

/// 📇 Catalogue metadata for a neuron kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeuronKindInfo {
    pub id: String,
    pub module: String,
    pub name: String,
    pub summary: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
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

    pub fn catalogue(&self) -> Vec<NeuronKindInfo> {
        let mut items: Vec<NeuronKindInfo> = self.kinds.values().map(|entry| entry.info.clone()).collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }
}
// #endregion 🔖NeuronKind

// #region 🔖Evaluator
/// 🔄 Topological evaluation over a neural tree.
pub struct Evaluator<'a> {
    registry: &'a Registry,
}

impl<'a> Evaluator<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    pub fn evaluate(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Result<HashMap<String, Dictionary>, EvalError> {
        self.evaluate_with(tree, seeds, &mut |kind, input| {
            self.registry
                .get(kind)
                .ok_or_else(|| EvalError::UnknownKind(kind.into()))?
                .evaluate(input)
        })
    }

    pub fn evaluate_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<HashMap<String, Dictionary>, EvalError> {
        let order = topo_order(tree)?;
        let mut outputs: HashMap<String, Dictionary> = seeds.clone();
        for neuron_id in order {
            let neuron = tree.neurons.iter().find(|n| n.id == neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
            let input = collect_neuron_input(tree, &outputs, &neuron_id);
            if let Some(seed) = seeds.get(&neuron_id) {
                outputs.insert(neuron_id.clone(), seed.clone());
                continue;
            }
            let out = dispatch(&neuron.kind, &input.merge(&neuron.params))?;
            outputs.insert(neuron_id.clone(), out);
        }
        Ok(outputs)
    }
}

fn collect_neuron_input(tree: &Tree, outputs: &HashMap<String, Dictionary>, neuron_id: &str) -> Dictionary {
    let mut acc = Dictionary::new();
    for syn in &tree.synapses {
        if syn.to != neuron_id {
            continue;
        }
        if let Some(src_out) = outputs.get(&syn.from) {
            acc = acc.merge(src_out);
        }
    }
    acc
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
            summary: "Forwards input".into(),
            inputs: vec!["x".into()],
            outputs: vec!["x".into()],
        }
    }

    fn double_info() -> NeuronKindInfo {
        NeuronKindInfo {
            id: "double".into(),
            module: "test".into(),
            name: "Double".into(),
            summary: "Doubles number".into(),
            inputs: vec!["number".into()],
            outputs: vec!["number".into()],
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
            neurons: vec![Neuron { id: "b".into(), kind: "double".into(), params: Dictionary::new() }],
            synapses: vec![],
        };
        let mut seeds = HashMap::new();
        seeds.insert("b".into(), Dictionary::new().insert("number", Value::Atom(Atom::Decimal(3.0))));
        let out = Evaluator::new(&Registry::new()).evaluate_with(&tree, &seeds, &mut |kind, input| {
            assert_eq!(kind, "double");
            let n = input.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number".into()))?;
            Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n * 2.0))))
        }).unwrap();
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
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into() }],
        };
        let mut seeds = HashMap::new();
        seeds.insert("a".into(), Dictionary::new().insert("number", Value::Atom(Atom::Decimal(2.0))));
        let out = Evaluator::new(&reg).evaluate(&tree, &seeds).unwrap();
        assert_eq!(out.get("b").and_then(|d| d.get("number")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
    }
}
// #endregion 🔖Tests
