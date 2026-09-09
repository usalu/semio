use super::*;
use crate::wfc_engine::ids::{PatternId, RelationId};
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::topology::GraphTopologyBuilder;

fn checkerboard(n: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, RelationId) {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(n);
    for i in 0..n.saturating_sub(1) {
        tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
        tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
    }
    (model, tb.build().unwrap(), adj)
}

#[test]
fn propagation_forces_alternation_after_one_pin() {
    let (model, topo, _adj) = checkerboard(4);
    let mut domains = DomainStore::new_full(4, model.weights());
    let mut removed = PatternSet::new_empty(2);
    domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
    let mut queue = PropQueue::new(4);
    queue.push(NodeId(0));
    let mut trail = Trail::new();
    let mut metrics = Metrics::default();
    run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics).unwrap();
    assert_eq!(domains.get(NodeId(0)).singleton(), Some(PatternId(0)));
    assert_eq!(domains.get(NodeId(1)).singleton(), Some(PatternId(1)));
    assert_eq!(domains.get(NodeId(2)).singleton(), Some(PatternId(0)));
    assert_eq!(domains.get(NodeId(3)).singleton(), Some(PatternId(1)));
    assert!(metrics.propagations > 0);
}

#[test]
fn odd_cycle_pin_propagates_to_wipeout() {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();

    let mut tb = GraphTopologyBuilder::new(5);
    for i in 0..4 {
        tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
        tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
    }
    tb.arc(NodeId(4), NodeId(0), adj);
    tb.arc(NodeId(0), NodeId(4), adj);
    let topo = tb.build().unwrap();

    let mut domains = DomainStore::new_full(5, model.weights());
    let mut removed = PatternSet::new_empty(2);
    domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
    let mut queue = PropQueue::new(5);
    queue.push(NodeId(0));
    let mut trail = Trail::new();
    let mut metrics = Metrics::default();
    let result = run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
    assert!(result.is_err());
}

#[test]
fn no_dirty_nodes_means_no_op() {
    let (model, topo, _adj) = checkerboard(3);
    let mut domains = DomainStore::new_full(3, model.weights());
    let mut queue = PropQueue::new(3);
    let mut trail = Trail::new();
    let mut metrics = Metrics::default();
    run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics).unwrap();
    for (_, d) in domains.iter() {
        assert_eq!(d.cardinality(), 2);
    }
    assert_eq!(metrics.propagations, 0);
}
