//! ↔️ Port undirected graph: handle-to-handle edges without direction.

pub use mathematical_core::{Ported, Undirected};
pub use mathematical_graph_port::*;

/// ↔️ Port graph engine; handle endpoints are unordered pairs.
pub type UndirectedPortGraphEngine = GraphEngine<Ported, Undirected>;

/// 🪢 Port edge with handle endpoints (unordered).
pub type Edge = GraphEdge<HandleId>;

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undirected_port_graph_engine_alias() {
        let mut g = UndirectedPortGraphEngine::new();
        g.create_node(1, 0.0, 0.0, 40.0, true);
        g.create_handle(10, 1, 0.0);
        g.create_handle(11, 1, 1.0);
        g.create_edge(9, 11, 10);
        let e = g.edges.get(&9).unwrap();
        assert_eq!(e.source, 10);
        assert_eq!(e.target, 11);
    }
}
// #endregion 🔖Tests
