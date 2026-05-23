//! ↔️ Normal undirected graph: node-to-node edges without port ordering.

pub use mathematical_graph::*;
pub use mathematical_core::{Directedness, Normal, Undirected};

/// ↔️ Node graph engine without ports; endpoints are unordered node pairs.
pub type UndirectedGraphEngine = GraphEngine<Normal, Undirected>;

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undirected_graph_engine_alias() {
        let mut g = UndirectedGraphEngine::new();
        g.create_node(1, 0.0, 0.0, 20.0, true);
        g.create_node(2, 50.0, 0.0, 20.0, true);
        g.create_edge(9, 2, 1);
        let e = g.edges.get(&9).unwrap();
        assert_eq!(e.source, 1);
        assert_eq!(e.target, 2);
    }
}
// #endregion 🔖Tests
