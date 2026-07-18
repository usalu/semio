//! 🔗 Neural tree adapter over compiled DAG wire rows.

use mathematical_graph_dsl::{WireEdge, WireNode};
use mathematical_graph_manifest::{PropertyBag, PropertyValue};
use neural_engine::{Atom, Dictionary, Neuron, Synapse, Tree, Value};
use serde_json::Value as JsonValue;

const CLUSTER_KIND: &str = "cluster";

//#region ⚠️ Errors
/// 🚨 Failure modes when adapting a DAG fixture into neural wire rows.
#[derive(Debug, thiserror::Error)]
pub enum NeuralDagError {
    /// 🧩 The fixture document is not valid JSON.
    #[error("invalid DAG fixture JSON: {0}")]
    Json(#[from] serde_json::Error),
}
//#endregion ⚠️ Errors

fn property_value_to_neural(value: &PropertyValue) -> Value {
    match value {
        PropertyValue::String(s) => Value::Atom(Atom::String(s.clone())),
        PropertyValue::Number(n) => Value::Atom(Atom::Decimal(*n)),
        PropertyValue::Bool(b) => Value::Atom(Atom::Boolean(*b)),
        PropertyValue::Null => Value::Atom(Atom::Null),
        PropertyValue::Array(items) => {
            let mut dict = Dictionary::new();
            for (index, row) in items.iter().enumerate() {
                dict = dict.insert(index.to_string(), property_value_to_neural(row));
            }
            Value::Dictionary(dict)
        }
        PropertyValue::Object(map) => {
            let mut dict = Dictionary::new();
            for (key, row) in map {
                dict = dict.insert(key, property_value_to_neural(row));
            }
            Value::Dictionary(dict)
        }
    }
}

fn dictionary_from_property_bag(bag: &PropertyBag) -> Dictionary {
    let mut dict = Dictionary::new();
    for (key, value) in bag {
        dict = dict.insert(key, property_value_to_neural(value));
    }
    dict
}

fn cluster_tree_from_node(node: &WireNode) -> Option<Tree> {
    let PropertyValue::String(json) = node.properties.get("clusterTree")? else {
        return None;
    };
    serde_json::from_str(json).ok()
}

/// 🌳 Build a neural execution tree from compiled DAG wire rows.
pub fn tree_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> Tree {
    let neurons = nodes
        .iter()
        .map(|node| {
            let mut params = dictionary_from_property_bag(&node.properties);
            if node.kind == CLUSTER_KIND {
                if let Some(PropertyValue::String(name)) = node.properties.get("name") {
                    params = params.insert("name", Value::Atom(Atom::String(name.clone())));
                }
            }
            let nested = if node.kind == CLUSTER_KIND { cluster_tree_from_node(node).map(Box::new) } else { None };
            Neuron { id: node.id.clone(), kind: node.kind.clone(), params, tree: nested }
        })
        .collect();
    let synapses = edges.iter().enumerate().map(|(index, edge)| Synapse { id: format!("synapse-{index}"), from: edge.from.clone(), to: edge.to.clone(), from_port: edge.from_port.clone(), to_port: edge.to_port.clone() }).collect();
    Tree { neurons, synapses }
}

/// 🌳 Build wire rows from a DAG fixture JSON document.
pub fn wire_rows_from_dag_fixture_json(json: &str) -> Result<(Vec<WireNode>, Vec<WireEdge>), NeuralDagError> {
    let value: JsonValue = serde_json::from_str(json)?;
    let nodes = value
        .get("nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|row| {
            let obj = row.as_object()?;
            let id = obj.get("id")?.as_str()?.to_string();
            let kind = obj.get("operatorKind").or_else(|| obj.get("operator_kind")).and_then(|v| v.as_str()).map(str::to_string).or_else(|| obj.get("kind").and_then(|v| v.as_str()).map(str::to_string)).unwrap_or_else(|| "node".to_string());
            let properties: PropertyBag = obj.get("properties").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            Some(WireNode { id, kind, port: None, properties })
        })
        .collect();
    let edges = value
        .get("edges")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|row| {
            let obj = row.as_object()?;
            let source = obj.get("source")?.as_str()?;
            let target = obj.get("target")?.as_str()?;
            let (from, from_port) = split_endpoint(source);
            let (to, to_port) = split_endpoint(target);
            let properties: PropertyBag = obj.get("properties").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            Some(WireEdge { from, from_port, to, to_port, directed: true, properties })
        })
        .collect();
    Ok((nodes, edges))
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    if let Some((node, port)) = endpoint.rsplit_once(':') {
        return (node.to_string(), port.to_string());
    }
    (endpoint.to_string(), "out".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_from_dag_builds_neurons_and_synapses() {
        let nodes = vec![WireNode { id: "a".into(), kind: "core.number".into(), port: None, properties: PropertyBag::new() }, WireNode { id: "b".into(), kind: "math.add".into(), port: None, properties: PropertyBag::new() }];
        let edges = vec![WireEdge { from: "a".into(), from_port: "number".into(), to: "b".into(), to_port: "a".into(), directed: true, properties: PropertyBag::new() }];
        let tree = tree_from_dag(&nodes, &edges);
        assert_eq!(tree.neurons.len(), 2);
        assert_eq!(tree.synapses.len(), 1);
    }
}
