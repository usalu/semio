//! ➡️ Normal directed graph: node-to-node relationships (mindmaps).

pub use mathematical_graph::*;
pub use mathematical_core::{Directed, Normal};

/// ➡️ Node graph engine without ports; relationships are directed node pairs.
pub type DirectedGraphEngine = GraphEngine<Normal, Directed>;

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directed_graph_engine_alias() {
        let mut g = DirectedGraphEngine::new();
        g.create_node(1, 0.0, 0.0, 20.0, true);
        g.create_node(2, 80.0, 0.0, 20.0, true);
        g.create_edge(9, 1, 2);
        let e = g.edges.get(&9).unwrap();
        assert_eq!(e.source, 1);
        assert_eq!(e.target, 2);
    }
}
// #endregion 🔖Tests
