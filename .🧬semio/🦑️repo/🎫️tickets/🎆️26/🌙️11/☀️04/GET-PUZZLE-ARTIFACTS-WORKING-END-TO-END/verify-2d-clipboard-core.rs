use semio_framework_pack_json::{json, JsonMemberPolicy, Value};
use semio_framework_value::DslValue;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum ClipboardError {
    EmptySelection,
    IncompatibleMediaType(String),
    ParseFailed(String),
}

pub struct ClipboardFragment {
    schema: String,
    media_type: String,
    dsl_text: String,
    pack_bytes: Option<Vec<u8>>,
    source_app: String,
    label: String,
}

fn puzzle2d_clipboard_media_type() -> String {
    "2d-design".to_owned()
}

type Puzzle2dEdge = Value;
type Puzzle2dMutation = Value;
type Puzzle2dPlaySnapshot = Value;

struct Puzzle2dRecorder {
    value: Value,
    rows: Vec<Value>,
    host_reads: Cell<usize>,
}

impl Puzzle2dRecorder {
    fn new(value: &Value) -> Self {
        Self { value: value.clone(), rows: vec![], host_reads: Cell::new(0) }
    }

    fn value(&self) -> &Value {
        self.host_reads.set(self.host_reads.get() + 1);
        &self.value
    }

    fn into_rows(self) -> Vec<Value> {
        assert_eq!(self.host_reads.into_inner(), 1, "paste reads the destination host view once");
        self.rows
    }

    fn create(&mut self, node: &Value) -> Option<String> {
        let id = node.get("id")?.as_str()?.to_owned();
        if board_snapshot_nodes(&self.value).iter().any(|node| node["id"].as_str() == Some(&id)) {
            return None;
        }
        self.value.as_object_mut().unwrap().get_mut("nodes").unwrap().as_array_mut().unwrap().push(node.clone());
        self.rows.push(node.clone());
        Some(id)
    }

    fn connect_edge(&mut self, edge: Puzzle2dEdge) -> bool {
        if board_snapshot_edges(&self.value).iter().any(|row| row["id"] == edge["id"]) {
            return false;
        }
        self.value.as_object_mut().unwrap().get_mut("edges").unwrap().as_array_mut().unwrap().push(edge.clone());
        self.rows.push(edge);
        true
    }
}

fn main() {
    let occupied_handles = std::env::args().nth(1).as_deref() == Some("handle-namespace");
    let before = semio_framework_pack_json::parse(FIXTURE, JsonMemberPolicy::Reject).unwrap();
    let oracle: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    let strings = |key| oracle[key].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect::<Vec<_>>();
    for id in strings("cutRefused") {
        assert!(puzzle2d_selection_is_locked(&before, &[id]));
    }
    for id in strings("cutAllowed") {
        assert!(!puzzle2d_selection_is_locked(&before, &[id]));
    }
    let fragment = puzzle2d_copy_fragment_from(&before, &strings("selected")).unwrap();
    let copied: serde_json::Value = serde_json::from_str(&fragment.dsl_text).unwrap();
    assert_eq!(copied["nodes"].as_array().unwrap().len(), 2);
    assert_eq!(copied["edges"].as_array().unwrap().len(), 2);
    let occupied = |name: &str, prefix: &str| {
        oracle["occupiedSerials"][name].as_array().unwrap().iter()
            .map(|number| json!({ "id": format!("{prefix}-{}", number.as_u64().unwrap()) }))
            .collect::<Vec<_>>()
    };
    let mut destination_nodes = occupied("nodes", "node");
    let mut destination_edges = occupied("edges", "edge");
    let expected_ids = if occupied_handles {
        let namespace = &before["handleNamespace"];
        destination_nodes.extend_from_slice(namespace["nodes"].as_array().unwrap());
        destination_edges.extend_from_slice(namespace["edges"].as_array().unwrap());
        &oracle["handleNamespace"]["pastedIds"]
    } else {
        &oracle["pastedIds"]
    };
    let labeling = &oracle["labeling"];
    let kind = labeling["kind"].as_str().unwrap();
    let catalog = labeling["catalog"].as_str().unwrap();
    if occupied_handles {
        for (node, label) in destination_nodes.iter_mut().zip(labeling["peers"].as_array().unwrap()) {
            let object = node.as_object_mut().unwrap();
            object.insert("nodeKind", json!(kind));
            if let Some(label) = label.as_str() {
                object.insert("text", json!(label));
            }
        }
    }
    let snapshot = json!({
        "nodes": destination_nodes, "edges": destination_edges,
        "meta": { "kindCatalogs": { "nodes": [{ "id": kind, "name": catalog }] } }
    });
    let request = json!({ "anchor": "centroid", "position": [100.0, -10.0, 0.0] });
    let placement = PastePlacement::from_action_args(&semio_framework_pack_json::to_dsl_value(&request)).unwrap();
    let (rows, pasted) = puzzle2d_paste_operations_on(&snapshot, &fragment, &placement).unwrap();
    assert_eq!(serde_json::json!(pasted), expected_ids["nodes"]);
    let [a, b, left, right] = rows.as_slice() else { panic!("expected two nodes and two connections") };
    let labels = [a["text"].as_str().unwrap(), b["text"].as_str().unwrap()];
    assert_eq!(serde_json::json!(labels), labeling[if occupied_handles { "occupied" } else { "empty" }]);
    assert_eq!(serde_json::json!([left["id"].as_str(), right["id"].as_str()]), expected_ids["edges"]);
    for (node, [x, y]) in [a, b].into_iter().zip([[99.0, -11.0], [101.0, -9.0]]) {
        assert_eq!((node["x"].as_f64(), node["y"].as_f64()), (Some(x), Some(y)));
        assert_ne!(node["handles"][0]["id"], node["handles"][1]["id"]);
    }
    let handle = |point: &serde_json::Value| {
        let node = point[0].as_u64().unwrap() as usize;
        let port = point[1].as_u64().unwrap() as usize;
        &[a, b][node]["handles"][port]["id"]
    };
    let mut endpoint_owners = HashMap::new();
    for node in board_snapshot_nodes(&snapshot).iter().chain([a, b]) {
        let owner = node["id"].as_str().unwrap();
        let endpoints = std::iter::once(node).chain(node.get("handles").and_then(Value::as_array).into_iter().flatten());
        for endpoint in endpoints {
            let id = endpoint["id"].as_str().unwrap();
            assert!(endpoint_owners.insert(id, owner).is_none(), "duplicate endpoint {id}");
        }
    }
    for (index, edge) in [left, right].into_iter().enumerate() {
        assert_eq!(&edge["source"], handle(&oracle["endpoints"][index][0]));
        assert_eq!(&edge["target"], handle(&oracle["endpoints"][index][1]));
        for (field, endpoint) in ["source", "target"].into_iter().zip(oracle["endpoints"][index].as_array().unwrap()) {
            let node = endpoint[0].as_u64().unwrap() as usize;
            assert_eq!(endpoint_owners[edge[field].as_str().unwrap()], pasted[node]);
        }
        assert_eq!(edge["gap"].as_f64(), oracle["edges"][index]["gap"].as_f64());
        assert_eq!(edge["locked"].as_bool(), oracle["edges"][index]["locked"].as_bool());
    }
    if occupied_handles {
        let existing = &before["handleNamespace"]["edges"][0];
        assert_eq!(endpoint_owners[existing["source"].as_str().unwrap()], "elsewhere");
        assert_eq!(endpoint_owners[existing["target"].as_str().unwrap()], "node-11:v0");
    }
    println!("Preserved centered placement and unique endpoint owners (occupied handles: {occupied_handles})");
}
