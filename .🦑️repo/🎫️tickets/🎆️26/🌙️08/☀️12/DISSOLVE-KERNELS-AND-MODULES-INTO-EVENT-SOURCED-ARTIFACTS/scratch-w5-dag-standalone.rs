// Standalone extraction of the new infinite/board/ports/directed/dag DagMutation/DagDiff logic
// (byte-identical algorithm, minimal type stand-ins) — used because the real owning crate
// (semio-framework-os-infinite) cannot produce a `cargo test --lib` signal right now: its test
// binary fails to compile due to unrelated, pre-existing breakage in a sibling module
// (🌍️world/🦀️component.rs — DslValue indexing E0608 + a missing bundled .glb asset), neither of
// which this wave touched. This is scratch verification, not a substitute for the real crate's
// test suite — flagged honestly in the wave report rather than claimed as equivalent.

#[derive(Clone, Debug, Default, PartialEq)]
struct DagNodeSpec {
    id: String,
    name: String,
    abbreviation: String,
    icon: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    operator_kind: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct DagFixtureEdge {
    id: String,
    source: String,
    target: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct DagSnapshot {
    nodes: Vec<DagNodeSpec>,
    edges: Vec<DagFixtureEdge>,
}

fn split_dag_endpoint(endpoint: &str) -> (String, String) {
    if let Some((node, port)) = endpoint.rsplit_once('@') {
        return (node.to_string(), port.to_string());
    }
    (endpoint.to_string(), "out".into())
}

#[derive(Clone, Debug, PartialEq)]
enum DagMutation {
    CreateNode { node: DagNodeSpec, index: usize },
    DeleteNode { id: String },
    RenameNode { id: String, new_id: String },
    ChangeNodeName { id: String, new_name: String },
    MoveNode { id: String, x: f64, y: f64 },
    ResizeNode { id: String, width: f64, height: f64 },
    ReorderNodes { order: Vec<String> },
    ConnectNodes { id: String, source: String, target: String },
    DisconnectNodes { id: String },
}

#[derive(Clone, Debug, PartialEq)]
struct RenamedNode { id: String, new_id: String }
#[derive(Clone, Debug, PartialEq)]
struct MovedNode { id: String, x: f64, y: f64 }
#[derive(Clone, Debug, PartialEq)]
struct ResizedNode { id: String, width: f64, height: f64 }
#[derive(Clone, Debug, PartialEq)]
struct ChangedNodeName { id: String, new_name: String }
#[derive(Clone, Debug, PartialEq)]
struct RewrittenEdgeEndpoint { id: String, new_source: Option<String>, new_target: Option<String> }

#[derive(Clone, Debug, Default, PartialEq)]
struct DagDiff {
    created_node: Option<DagNodeSpec>,
    created_node_at: Option<usize>,
    deleted_node_ids: Option<Vec<String>>,
    renamed_node: Option<RenamedNode>,
    moved_node: Option<MovedNode>,
    resized_node: Option<ResizedNode>,
    changed_node_name: Option<ChangedNodeName>,
    reordered_nodes: Option<Vec<String>>,
    connected_edge: Option<DagFixtureEdge>,
    disconnected_edge_ids: Option<Vec<String>>,
    rewritten_edge_endpoints: Option<Vec<RewrittenEdgeEndpoint>>,
}

fn diff(mutation: &DagMutation, snapshot: &DagSnapshot) -> DagDiff {
    let mut diff = DagDiff::default();
    match mutation {
        DagMutation::CreateNode { node, index } => {
            diff.created_node = Some(node.clone());
            diff.created_node_at = Some(*index);
        }
        DagMutation::DeleteNode { id } => {
            if snapshot.nodes.iter().any(|node| &node.id == id) {
                diff.deleted_node_ids = Some(vec![id.clone()]);
                let severed: Vec<String> = snapshot.edges.iter().filter(|edge| &split_dag_endpoint(&edge.source).0 == id || &split_dag_endpoint(&edge.target).0 == id).map(|edge| edge.id.clone()).collect();
                if !severed.is_empty() {
                    diff.disconnected_edge_ids = Some(severed);
                }
            }
        }
        DagMutation::RenameNode { id, new_id } => {
            if snapshot.nodes.iter().any(|node| &node.id == id) {
                diff.renamed_node = Some(RenamedNode { id: id.clone(), new_id: new_id.clone() });
                let rewrites: Vec<RewrittenEdgeEndpoint> = snapshot
                    .edges
                    .iter()
                    .filter_map(|edge| {
                        let (source_node, source_port) = split_dag_endpoint(&edge.source);
                        let (target_node, target_port) = split_dag_endpoint(&edge.target);
                        let touches_source = &source_node == id;
                        let touches_target = &target_node == id;
                        if !touches_source && !touches_target {
                            return None;
                        }
                        Some(RewrittenEdgeEndpoint {
                            id: edge.id.clone(),
                            new_source: touches_source.then(|| format!("{new_id}@{source_port}")),
                            new_target: touches_target.then(|| format!("{new_id}@{target_port}")),
                        })
                    })
                    .collect();
                if !rewrites.is_empty() {
                    diff.rewritten_edge_endpoints = Some(rewrites);
                }
            }
        }
        DagMutation::ChangeNodeName { id, new_name } => {
            if snapshot.nodes.iter().any(|node| &node.id == id) {
                diff.changed_node_name = Some(ChangedNodeName { id: id.clone(), new_name: new_name.clone() });
            }
        }
        DagMutation::MoveNode { id, x, y } => {
            if snapshot.nodes.iter().any(|node| &node.id == id) {
                diff.moved_node = Some(MovedNode { id: id.clone(), x: *x, y: *y });
            }
        }
        DagMutation::ResizeNode { id, width, height } => {
            if snapshot.nodes.iter().any(|node| &node.id == id) {
                diff.resized_node = Some(ResizedNode { id: id.clone(), width: *width, height: *height });
            }
        }
        DagMutation::ReorderNodes { order } => diff.reordered_nodes = Some(order.clone()),
        DagMutation::ConnectNodes { id, source, target } => {
            diff.connected_edge = Some(DagFixtureEdge { id: id.clone(), source: source.clone(), target: target.clone() });
        }
        DagMutation::DisconnectNodes { id } => {
            if snapshot.edges.iter().any(|edge| &edge.id == id) {
                diff.disconnected_edge_ids = Some(vec![id.clone()]);
            }
        }
    }
    diff
}

fn apply(diff: &DagDiff, snapshot: &DagSnapshot) -> DagSnapshot {
    let mut next = snapshot.clone();
    if let Some(node) = &diff.created_node {
        let at = diff.created_node_at.unwrap_or(next.nodes.len()).min(next.nodes.len());
        next.nodes.insert(at, node.clone());
    }
    if let Some(ids) = &diff.deleted_node_ids {
        next.nodes.retain(|node| !ids.contains(&node.id));
    }
    if let Some(renamed) = &diff.renamed_node {
        if let Some(node) = next.nodes.iter_mut().find(|node| node.id == renamed.id) {
            node.id = renamed.new_id.clone();
        }
    }
    if let Some(moved) = &diff.moved_node {
        if let Some(node) = next.nodes.iter_mut().find(|node| node.id == moved.id) {
            node.x = moved.x;
            node.y = moved.y;
        }
    }
    if let Some(resized) = &diff.resized_node {
        if let Some(node) = next.nodes.iter_mut().find(|node| node.id == resized.id) {
            node.width = resized.width;
            node.height = resized.height;
        }
    }
    if let Some(changed) = &diff.changed_node_name {
        if let Some(node) = next.nodes.iter_mut().find(|node| node.id == changed.id) {
            node.name = changed.new_name.clone();
        }
    }
    if let Some(order) = &diff.reordered_nodes {
        let mut reordered: Vec<DagNodeSpec> = Vec::with_capacity(next.nodes.len());
        for id in order {
            if let Some(at) = next.nodes.iter().position(|node| &node.id == id) {
                reordered.push(next.nodes.remove(at));
            }
        }
        reordered.extend(next.nodes.drain(..));
        next.nodes = reordered;
    }
    if let Some(edge) = &diff.connected_edge {
        next.edges.push(edge.clone());
    }
    if let Some(ids) = &diff.disconnected_edge_ids {
        next.edges.retain(|edge| !ids.contains(&edge.id));
    }
    if let Some(rewrites) = &diff.rewritten_edge_endpoints {
        for rewrite in rewrites {
            if let Some(edge) = next.edges.iter_mut().find(|edge| edge.id == rewrite.id) {
                if let Some(source) = &rewrite.new_source {
                    edge.source = source.clone();
                }
                if let Some(target) = &rewrite.new_target {
                    edge.target = target.clone();
                }
            }
        }
    }
    next
}

fn absorb(target: &mut DagDiff, other: DagDiff) {
    if other.created_node.is_some() {
        target.created_node = other.created_node;
        target.created_node_at = other.created_node_at;
    }
    if let Some(ids) = other.deleted_node_ids {
        target.deleted_node_ids.get_or_insert_with(Vec::new).extend(ids);
    }
    if other.renamed_node.is_some() {
        target.renamed_node = other.renamed_node;
    }
    if other.moved_node.is_some() {
        target.moved_node = other.moved_node;
    }
    if other.resized_node.is_some() {
        target.resized_node = other.resized_node;
    }
    if other.changed_node_name.is_some() {
        target.changed_node_name = other.changed_node_name;
    }
    if other.reordered_nodes.is_some() {
        target.reordered_nodes = other.reordered_nodes;
    }
    if other.connected_edge.is_some() {
        target.connected_edge = other.connected_edge;
    }
    if let Some(ids) = other.disconnected_edge_ids {
        target.disconnected_edge_ids.get_or_insert_with(Vec::new).extend(ids);
    }
    if let Some(rewrites) = other.rewritten_edge_endpoints {
        target.rewritten_edge_endpoints.get_or_insert_with(Vec::new).extend(rewrites);
    }
}

fn inverse(mutation: &DagMutation, snapshot: &DagSnapshot) -> Vec<DagMutation> {
    match mutation {
        DagMutation::CreateNode { node, .. } => vec![DagMutation::DeleteNode { id: node.id.clone() }],
        DagMutation::DeleteNode { id } => {
            let Some(at) = snapshot.nodes.iter().position(|node| &node.id == id) else {
                return Vec::new();
            };
            let node = &snapshot.nodes[at];
            let mut mutations = vec![DagMutation::CreateNode { node: node.clone(), index: at }];
            for edge in snapshot.edges.iter().filter(|edge| &split_dag_endpoint(&edge.source).0 == id || &split_dag_endpoint(&edge.target).0 == id) {
                mutations.push(DagMutation::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone() });
            }
            mutations
        }
        DagMutation::RenameNode { id, new_id } => {
            if snapshot.nodes.iter().any(|node| &node.id == id) {
                vec![DagMutation::RenameNode { id: new_id.clone(), new_id: id.clone() }]
            } else {
                Vec::new()
            }
        }
        DagMutation::MoveNode { id, .. } => snapshot.nodes.iter().find(|node| &node.id == id).map(|node| vec![DagMutation::MoveNode { id: id.clone(), x: node.x, y: node.y }]).unwrap_or_default(),
        DagMutation::ResizeNode { id, .. } => {
            snapshot.nodes.iter().find(|node| &node.id == id).map(|node| vec![DagMutation::ResizeNode { id: id.clone(), width: node.width, height: node.height }]).unwrap_or_default()
        }
        DagMutation::ChangeNodeName { id, .. } => {
            snapshot.nodes.iter().find(|node| &node.id == id).map(|node| vec![DagMutation::ChangeNodeName { id: id.clone(), new_name: node.name.clone() }]).unwrap_or_default()
        }
        DagMutation::ReorderNodes { .. } => vec![DagMutation::ReorderNodes { order: snapshot.nodes.iter().map(|node| node.id.clone()).collect() }],
        DagMutation::ConnectNodes { id, .. } => vec![DagMutation::DisconnectNodes { id: id.clone() }],
        DagMutation::DisconnectNodes { id } => snapshot
            .edges
            .iter()
            .find(|edge| &edge.id == id)
            .map(|edge| vec![DagMutation::ConnectNodes { id: id.clone(), source: edge.source.clone(), target: edge.target.clone() }])
            .unwrap_or_default(),
    }
}

fn round_trip(document: &DagSnapshot, mutation: &DagMutation) -> DagSnapshot {
    let forward = apply(&diff(mutation, document), document);
    let mut restored = forward.clone();
    for back in inverse(mutation, document) {
        restored = apply(&diff(&back, &restored), &restored);
    }
    assert_eq!(&restored, document, "inverse() must exactly restore the pre-mutation document for {mutation:?}");
    forward
}

fn sample_node(id: &str) -> DagNodeSpec {
    DagNodeSpec { id: id.into(), name: id.into(), ..Default::default() }
}

fn main() {
    // create -> move -> resize -> delete round trip
    let document = DagSnapshot::default();
    let added = round_trip(&document, &DagMutation::CreateNode { node: sample_node("n1"), index: 0 });
    assert_eq!(added.nodes.len(), 1);
    let moved = round_trip(&added, &DagMutation::MoveNode { id: "n1".into(), x: 42.0, y: 7.0 });
    assert_eq!(moved.nodes[0].x, 42.0);
    assert_eq!(moved.nodes[0].y, 7.0);
    let resized = round_trip(&moved, &DagMutation::ResizeNode { id: "n1".into(), width: 200.0, height: 90.0 });
    assert_eq!(resized.nodes[0].width, 200.0);
    let removed = round_trip(&resized, &DagMutation::DeleteNode { id: "n1".into() });
    assert!(removed.nodes.is_empty());

    // rename cascades edge endpoints
    let mut with_edge = DagSnapshot::default();
    with_edge.nodes = vec![sample_node("a"), sample_node("b")];
    with_edge.edges = vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into() }];
    let renamed = round_trip(&with_edge, &DagMutation::RenameNode { id: "a".into(), new_id: "aa".into() });
    assert!(renamed.nodes.iter().any(|node| node.id == "aa"));
    assert_eq!(renamed.edges[0].source, "aa@out");
    assert_eq!(renamed.edges[0].target, "b@in");

    // delete severs and reconnects edges (cascade)
    let deleted = round_trip(&with_edge, &DagMutation::DeleteNode { id: "a".into() });
    assert!(deleted.edges.is_empty(), "severed edge must be gone from the delete diff, reconnected by inverse");

    // reorder round trips
    let mut three = DagSnapshot::default();
    three.nodes = vec![sample_node("a"), sample_node("b"), sample_node("c")];
    let reordered = round_trip(&three, &DagMutation::ReorderNodes { order: vec!["c".into(), "a".into(), "b".into()] });
    let ids: Vec<&str> = reordered.nodes.iter().map(|n| n.id.as_str()).collect();
    assert_eq!(ids, vec!["c", "a", "b"]);

    // connect / disconnect round trip
    let mut two = DagSnapshot::default();
    two.nodes = vec![sample_node("a"), sample_node("b")];
    let connected = round_trip(&two, &DagMutation::ConnectNodes { id: "e1".into(), source: "a@out".into(), target: "b@in".into() });
    assert_eq!(connected.edges.len(), 1);
    let disconnected = round_trip(&connected, &DagMutation::DisconnectNodes { id: "e1".into() });
    assert!(disconnected.edges.is_empty());

    // determinism: diff/inverse are pure functions of (payload, base)
    let base = round_trip(&DagSnapshot::default(), &DagMutation::CreateNode { node: sample_node("n1"), index: 0 });
    let m = DagMutation::MoveNode { id: "n1".into(), x: 12.0, y: 34.0 };
    assert_eq!(diff(&m, &base), diff(&m, &base));
    assert_eq!(inverse(&m, &base), inverse(&m, &base));

    // diff-consistency: diff().apply() matches the documented direct field effect
    let via_diff = apply(&diff(&m, &base), &base);
    let mut via_direct = base.clone();
    via_direct.nodes[0].x = 12.0;
    via_direct.nodes[0].y = 34.0;
    assert_eq!(via_diff, via_direct);

    // absorb law: two coalesced diffs converge to the LATER move, not the earlier one
    let mut d1 = diff(&DagMutation::MoveNode { id: "n1".into(), x: 10.0, y: 10.0 }, &base);
    let mid = apply(&d1, &base);
    let d2 = diff(&DagMutation::MoveNode { id: "n1".into(), x: 20.0, y: 30.0 }, &mid);
    absorb(&mut d1, d2);
    let absorbed = apply(&d1, &base);
    assert_eq!(absorbed.nodes[0].x, 20.0);
    assert_eq!(absorbed.nodes[0].y, 30.0);

    // missing target -> no-op diff / empty inverse
    let empty = DagSnapshot::default();
    assert_eq!(diff(&DagMutation::MoveNode { id: "ghost".into(), x: 1.0, y: 1.0 }, &empty), DagDiff::default());
    assert!(inverse(&DagMutation::MoveNode { id: "ghost".into(), x: 1.0, y: 1.0 }, &empty).is_empty());
    assert!(inverse(&DagMutation::DeleteNode { id: "ghost".into() }, &empty).is_empty());
    assert!(inverse(&DagMutation::DisconnectNodes { id: "ghost".into() }, &empty).is_empty());

    println!("ALL SCRATCH ASSERTIONS PASSED");
}
