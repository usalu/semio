
use super::*;
use crate::wfc_engine::diag::Metrics;
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::topology::GraphTopologyBuilder;
use crate::wfc_engine::weights::WeightTable;

/// 🧪️ A trivial arc-less topology sized to `node_count` — every test here only needs the
/// AC-3 re-propagation hook to be a safe no-op (no neighbors to cascade to), not to exercise
/// AC-3 itself (that's `🦀️prop_ac3.rs`'s own job).
fn no_arcs_topology(node_count: usize) -> crate::wfc_engine::topology::GraphTopology {
    GraphTopologyBuilder::new(node_count).build().unwrap()
}

fn w3() -> WeightTable {
    WeightTable::new(&[1.0, 1.0, 1.0]).unwrap()
}

fn model3() -> CompiledModel {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    b.add_pattern(1.0);
    b.add_pattern(1.0);
    b.add_relation("r");
    b.compile().unwrap()
}

#[test]
fn disabled_store_records_and_watches_nothing() {
    let mut store = NogoodIndex::new(NogoodConfig { enabled: false, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
    assert_eq!(store.len(), 0);
}

#[test]
fn record_skips_empty_and_over_length_clauses() {
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, max_len: 2, max_count: 10 });
    store.record(vec![]);
    assert_eq!(store.len(), 0);
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(0)), (NodeId(2), PatternId(0))]);
    assert_eq!(store.len(), 0, "3-literal clause exceeds max_len=2");
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(0))]);
    assert_eq!(store.len(), 1);
}

#[test]
fn eviction_keeps_store_at_max_count() {
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, max_len: 8, max_count: 2 });
    store.record(vec![(NodeId(0), PatternId(0))]);
    store.record(vec![(NodeId(1), PatternId(0))]);
    store.record(vec![(NodeId(2), PatternId(0))]);
    assert_eq!(store.len(), 2);
}

#[test]
fn rewatch_unit_propagates_a_length_one_nogood_at_attempt_start() {
    // A length-1 nogood means "node=pattern alone is impossible" — rewatch should exclude it
    // immediately, before any decision.
    let model = model3();
    let topo = no_arcs_topology(2);
    let w = w3();
    let mut domains = DomainStore::new_full(2, &w);
    let mut trail = Trail::new();
    let mut queue = PropQueue::new(2);
    let mut metrics = Metrics::default();
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(1))]);

    let conflict = store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
    assert!(conflict.is_none());
    assert!(!domains.get(NodeId(0)).bits().get(PatternId(1)));
    assert_eq!(domains.get(NodeId(0)).cardinality(), 2);
}

#[test]
fn rewatch_reports_conflict_when_length_one_nogood_wipes_a_singleton_domain() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    b.add_relation("r");
    let model = b.compile().unwrap();
    let topo = no_arcs_topology(1);
    let w = WeightTable::new(&[1.0]).unwrap();
    let mut domains = DomainStore::new_full(1, &w);
    let mut trail = Trail::new();
    let mut queue = PropQueue::new(1);
    let mut metrics = Metrics::default();
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(0))]);

    let conflict = store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
    assert_eq!(conflict, Some(NodeId(0)));
}

#[test]
fn rewatch_leaves_a_fully_excluded_nogood_inert() {
    // Every literal already impossible before any decision (e.g. `fixed` pinned node 0 away
    // from pattern 0 independent of this nogood): nothing to watch, nothing to propagate.
    let model = model3();
    let topo = no_arcs_topology(2);
    let w = w3();
    let mut domains = DomainStore::new_full(2, &w);
    domains.get_mut(NodeId(0)).remove(PatternId(0), &w);
    let mut trail = Trail::new();
    let mut queue = PropQueue::new(2);
    let mut metrics = Metrics::default();
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(0))]);

    let conflict = store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
    assert!(conflict.is_none());
    assert!(domains.get(NodeId(1)).bits().get(PatternId(0)), "the other literal was never touched");
}

#[test]
fn on_decision_unit_propagates_the_partner_literal() {
    let model = model3();
    let topo = no_arcs_topology(2);
    let w = w3();
    let mut domains = DomainStore::new_full(2, &w);
    let mut trail = Trail::new();
    let mut queue = PropQueue::new(2);
    let mut metrics = Metrics::default();
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
    store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

    // Decide node0 = pattern0: the nogood's other literal (node1=pattern1) must now be forced
    // out of node1's domain, since keeping it open risks completing the known-bad combination.
    domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
    let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
    assert!(conflict.is_none());
    assert!(!domains.get(NodeId(1)).bits().get(PatternId(1)));
}

#[test]
fn on_decision_detects_conflict_when_the_other_watch_was_already_resolved_true() {
    // Models the realistic conflict path: node1 reaches singleton=pattern1 via ordinary
    // propagation the store never reacts to (per its documented scope — only explicit
    // decisions trigger `on_decision`), leaving its watch stale-but-unprocessed. When node0 is
    // then genuinely decided to pattern0, the fresh re-check of the (unmoved) partner watch
    // must discover it's already false too — the exact combination this nogood forbids.
    let model = model3();
    let topo = no_arcs_topology(2);
    let w = w3();
    let mut domains = DomainStore::new_full(2, &w);
    let mut trail = Trail::new();
    let mut queue = PropQueue::new(2);
    let mut metrics = Metrics::default();
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
    store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

    let mut singleton_p1 = crate::wfc_engine::bitset::PatternSet::new_empty(3);
    singleton_p1.set(PatternId(1), true);
    domains.get_mut(NodeId(1)).restrict(&singleton_p1, &w); // propagation-forced, not a decision

    domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
    let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
    assert_eq!(conflict, Some(NodeId(0)));
}

#[test]
fn on_decision_finds_a_third_literal_as_replacement_watch_instead_of_propagating() {
    // A 3-literal nogood: deciding node0=pattern0 (one watch) must find node2's still-open
    // literal as a replacement watch rather than prematurely forcing node1's exclusion.
    let model = model3();
    let topo = no_arcs_topology(3);
    let w = w3();
    let mut domains = DomainStore::new_full(3, &w);
    let mut trail = Trail::new();
    let mut queue = PropQueue::new(3);
    let mut metrics = Metrics::default();
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1)), (NodeId(2), PatternId(2))]);
    store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

    domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
    let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
    assert!(conflict.is_none());
    assert!(domains.get(NodeId(1)).bits().get(PatternId(1)), "node1's literal must not be forced yet — node2's is still a valid watch");
}

#[test]
fn on_decision_ignores_a_watch_whose_partner_is_already_stale() {
    // The partner literal was excluded by something outside this store's notice (simulated by
    // directly removing it); on_decision must re-check liveness live and do nothing, not act
    // on a stale cached assumption.
    let model = model3();
    let topo = no_arcs_topology(2);
    let w = w3();
    let mut domains = DomainStore::new_full(2, &w);
    let mut trail = Trail::new();
    let mut queue = PropQueue::new(2);
    let mut metrics = Metrics::default();
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
    store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

    domains.get_mut(NodeId(1)).remove(PatternId(1), &w); // partner literal silently goes false
    domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
    let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
    assert!(conflict.is_none());
}

#[test]
fn on_decision_forced_exclusion_cascades_through_ac3_to_a_third_node() {
    // node0 --r--> node1 --r--> node2, where `r` only allows equal patterns (so excluding a
    // pattern at node1 must cascade to node2 too). The nogood forbids node0=0 AND node1=1
    // simultaneously; deciding node0=0 forces node1 away from pattern1 — and that exclusion
    // must cascade via AC-3 to remove pattern1 from node2 as well, not just node1.
    let mut b = ModelBuilder::new();
    let p0 = b.add_pattern(1.0);
    let p1 = b.add_pattern(1.0);
    let r = b.add_relation("eq");
    b.allow_mirrored(r, p0, p0);
    b.allow_mirrored(r, p1, p1);
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(3);
    tb.arc(NodeId(1), NodeId(2), r);
    tb.arc(NodeId(2), NodeId(1), r);
    let topo = tb.build().unwrap();

    let w = model.weights().clone();
    let mut domains = DomainStore::new_full(3, &w);
    let mut trail = Trail::new();
    let mut queue = PropQueue::new(3);
    let mut metrics = Metrics::default();
    let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
    store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
    store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

    domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
    let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
    assert!(conflict.is_none());
    assert!(!domains.get(NodeId(1)).bits().get(PatternId(1)), "node1's forced exclusion");
    assert!(!domains.get(NodeId(2)).bits().get(PatternId(1)), "the exclusion must cascade to node2 via AC-3, or node2 could later be wrongly decided to pattern1");
}
