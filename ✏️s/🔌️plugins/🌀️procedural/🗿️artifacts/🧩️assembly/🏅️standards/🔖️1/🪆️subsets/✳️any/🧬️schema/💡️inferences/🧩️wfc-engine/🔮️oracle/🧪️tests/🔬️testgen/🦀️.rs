
use super::*;
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::weights::WeightTable;

/// 🧪️ A self-contained tiny instance: a compiled model, its node count, arcs, and per-node
/// initial domains — everything [`super::enumerate`] and a real solver both need.
pub struct Fixture {
    pub model: CompiledModel,
    pub node_count: usize,
    pub arcs: Vec<ArcSpec>,
    pub init_domains: Vec<PatternSet>,
}

/// 🧪️ Two patterns (black/white) that must differ across every edge of a path graph
/// `0 - 1 - ... - (n-1)`. Always satisfiable (paths are bipartite).
pub fn checkerboard_path(n: usize) -> Fixture {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();
    let mut arcs = Vec::new();
    for i in 0..n.saturating_sub(1) {
        arcs.push(ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(i + 1), relation: adj });
        arcs.push(ArcSpec { from: NodeId::from_index(i + 1), to: NodeId::from_index(i), relation: adj });
    }
    let init_domains = vec![model.full_domain(); n];
    Fixture { model, node_count: n, arcs, init_domains }
}

/// 🧪️ Two patterns that must differ across every edge of an odd cycle `0-1-...-(n-1)-0` with
/// `n` odd — unsatisfiable, since odd cycles are not bipartite. `n` must be odd and >= 3.
pub fn unsat_odd_cycle(n: usize) -> Fixture {
    assert!(n >= 3 && n % 2 == 1, "unsat_odd_cycle requires an odd n >= 3");
    let mut fx = checkerboard_path(n);
    let adj = RelationId(0);
    fx.arcs.push(ArcSpec { from: NodeId::from_index(n - 1), to: NodeId::from_index(0), relation: adj });
    fx.arcs.push(ArcSpec { from: NodeId::from_index(0), to: NodeId::from_index(n - 1), relation: adj });
    fx
}

/// 🧪️ A complete graph `K_n` over `k` patterns that must all differ pairwise — a proper
/// `k`-coloring of `K_n`, satisfiable iff `k >= n`.
pub fn complete_graph_coloring(n: usize, k: usize) -> Fixture {
    let mut b = ModelBuilder::new();
    let patterns: Vec<_> = (0..k).map(|_| b.add_pattern(1.0)).collect();
    let ne = b.add_relation("not_equal");
    for &a in &patterns {
        for &c in &patterns {
            if a != c {
                b.allow(ne, a, c);
            }
        }
    }
    let model = b.compile().unwrap();
    let mut arcs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            arcs.push(ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(j), relation: ne });
            arcs.push(ArcSpec { from: NodeId::from_index(j), to: NodeId::from_index(i), relation: ne });
        }
    }
    let init_domains = vec![model.full_domain(); n];
    Fixture { model, node_count: n, arcs, init_domains }
}

/// 🧪️ A uniformly-random tiny compiled model: `pattern_count` patterns each with a random
/// weight in `[1, 5]`, one relation whose compatibility pairs are each independently kept with
/// probability `density`.
pub fn random_model(rng: &mut semio_framework_geometry::random::Rng, pattern_count: usize, density: f64) -> (CompiledModel, RelationId) {
    let mut b = ModelBuilder::new();
    let patterns: Vec<_> = (0..pattern_count).map(|_| b.add_pattern(1.0 + rng.next_range(0, 5) as f64)).collect();
    let r = b.add_relation("r");
    for &a in &patterns {
        for &c in &patterns {
            if rng.next_bool(density) {
                b.allow(r, a, c);
            }
        }
    }
    let model = b.compile().unwrap();
    (model, r)
}

/// 🧪️ A random small connected graph over `node_count` nodes (a random spanning tree plus a
/// few extra random edges), with both directions registered under `relation`.
pub fn random_arcs(rng: &mut semio_framework_geometry::random::Rng, node_count: usize, relation: RelationId) -> Vec<ArcSpec> {
    let mut arcs = Vec::new();
    for i in 1..node_count {
        let j = rng.next_range(0, i as u64) as usize;
        arcs.push(ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(j), relation });
        arcs.push(ArcSpec { from: NodeId::from_index(j), to: NodeId::from_index(i), relation });
    }
    let extra = rng.next_range(0, node_count as u64) as usize;
    for _ in 0..extra {
        if node_count < 2 {
            break;
        }
        let i = rng.next_range(0, node_count as u64) as usize;
        let j = rng.next_range(0, node_count as u64) as usize;
        if i != j {
            arcs.push(ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(j), relation });
            arcs.push(ArcSpec { from: NodeId::from_index(j), to: NodeId::from_index(i), relation });
        }
    }
    arcs
}

#[allow(dead_code)]
pub fn full_domains(model: &CompiledModel, node_count: usize) -> Vec<PatternSet> {
    vec![model.full_domain(); node_count]
}

#[allow(dead_code)]
pub fn weight_table_of(model: &CompiledModel) -> &WeightTable {
    model.weights()
}
