use super::*;

#[test]
fn builder_produces_correct_out_and_in_arcs() {
    let mut b = GraphTopologyBuilder::new(3);
    b.arc(NodeId(0), NodeId(1), RelationId(0));
    b.arc(NodeId(1), NodeId(2), RelationId(0));
    b.arc(NodeId(0), NodeId(2), RelationId(1));
    let topo = b.build().unwrap();
    assert_eq!(topo.node_count(), 3);
    assert_eq!(topo.arc_count(), 3);

    let mut out0 = Vec::new();
    topo.for_each_out_arc(NodeId(0), |m, r| out0.push((m, r)));
    assert_eq!(out0, vec![(NodeId(1), RelationId(0)), (NodeId(2), RelationId(1))]);

    let mut in2 = Vec::new();
    topo.for_each_in_arc(NodeId(2), |m, r, _slot| in2.push((m, r)));
    assert_eq!(in2, vec![(NodeId(0), RelationId(1)), (NodeId(1), RelationId(0))]);
}

#[test]
fn self_loops_and_multiedges_are_supported() {
    let mut b = GraphTopologyBuilder::new(2);
    b.arc(NodeId(0), NodeId(0), RelationId(0)); // self-loop
    b.arc(NodeId(0), NodeId(1), RelationId(0));
    b.arc(NodeId(0), NodeId(1), RelationId(1)); // multiedge under a different relation
    let topo = b.build().unwrap();
    assert_eq!(topo.arc_count(), 3);
    assert_eq!(topo.out_degree(NodeId(0)), 3);
    let mut out0 = Vec::new();
    topo.for_each_out_arc(NodeId(0), |m, r| out0.push((m, r)));
    assert!(out0.contains(&(NodeId(0), RelationId(0))));
    assert!(out0.contains(&(NodeId(1), RelationId(0))));
    assert!(out0.contains(&(NodeId(1), RelationId(1))));
}

#[test]
fn dangling_arc_is_rejected() {
    let mut b = GraphTopologyBuilder::new(2);
    b.arc(NodeId(0), NodeId(5), RelationId(0));
    assert!(b.build().is_err());
}

#[test]
fn assembly_cursor_compiler_matches_canonical_csr_order_and_multiplicity() {
    let arcs = [(NodeId(2), NodeId(0), RelationId(1)), (NodeId(0), NodeId(1), RelationId(0)), (NodeId(0), NodeId(1), RelationId(0)), (NodeId(1), NodeId(2), RelationId(1))];
    let mut canonical = GraphTopologyBuilder::new(3);
    let mut incremental = AssemblyTopologyBuild::new(3);
    for &(from, to, relation) in &arcs {
        canonical.arc(from, to, relation);
        incremental.add_arc(from, to, relation).expect("assembly arc");
    }
    let canonical = canonical.build().expect("canonical topology");
    let incremental = loop {
        if let Some(topology) = incremental.step() {
            break topology;
        }
    };
    assert_eq!(incremental.out_starts, canonical.out_starts);
    assert_eq!(incremental.out_targets, canonical.out_targets);
    assert_eq!(incremental.out_relations, canonical.out_relations);
    assert_eq!(incremental.in_starts, canonical.in_starts);
    assert_eq!(incremental.in_sources, canonical.in_sources);
    assert_eq!(incremental.in_relations, canonical.in_relations);
    assert_eq!(incremental.regions, canonical.regions);
}

#[test]
fn in_arc_slots_are_dense_and_unique_per_node() {
    let mut b = GraphTopologyBuilder::new(3);
    b.arc(NodeId(0), NodeId(2), RelationId(0));
    b.arc(NodeId(1), NodeId(2), RelationId(0));
    let topo = b.build().unwrap();
    assert_eq!(topo.in_degree(NodeId(2)), 2);
    let mut slots = Vec::new();
    topo.for_each_in_arc(NodeId(2), |_, _, slot| slots.push(slot));
    assert_eq!(slots.len(), 2);
    assert_ne!(slots[0], slots[1]);
    for &slot in &slots {
        assert!(slot < topo.node_count() * topo.max_in_degree());
    }
    assert_eq!(topo.max_in_degree(), 2);
}

#[test]
fn regions_default_to_zero_and_are_settable() {
    let mut b = GraphTopologyBuilder::new(2);
    b.region(NodeId(1), RegionId(7));
    let topo = b.build().unwrap();
    assert_eq!(topo.region_of(NodeId(0)), RegionId(0));
    assert_eq!(topo.region_of(NodeId(1)), RegionId(7));
}

#[test]
fn graph_view_conversion_preserves_neutral_directed_and_undirected_arcs() {
    use semio_framework_graph::{Directed, Normal, Storage, Undirected};
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔍️decoding-and-graphs/🔣️.json")).unwrap();
    let row = &oracle["graph"];
    let mut directed = Storage::<Normal, Directed>::new();
    let mut undirected = Storage::<Normal, Undirected>::new();
    for id in serde_json::from_value::<Vec<u64>>(row["nodes"].clone()).unwrap() {
        directed.add_node_with_id(id, Default::default());
        undirected.add_node_with_id(id, Default::default());
    }
    for [from, to] in serde_json::from_value::<Vec<[u64; 2]>>(row["edges"].clone()).unwrap() {
        directed.add_edge(from, to);
        undirected.add_edge(from, to);
    }
    let relation = RelationId(row["relation"].as_u64().unwrap() as u32);
    for (key, topology) in [("directed", from_graph_view(&directed, |_| relation).unwrap()), ("undirected", from_graph_view(&undirected, |_| relation).unwrap())] {
        assert_eq!(topology.node_count(), row["nodes"].as_array().unwrap().len());
        let mut arcs = Vec::new();
        for from in 0..topology.node_count() {
            topology.for_each_out_arc(NodeId::from_index(from), |to, relation| arcs.push([from as u32, to.get(), relation.get()]));
        }
        arcs.sort_unstable();
        assert_eq!(serde_json::to_value(&arcs).unwrap(), row[key]);
        assert_eq!(topology.arc_count(), arcs.len());
    }
}
