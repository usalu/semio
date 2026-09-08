mod tests {
    use super::*;

    #[test]
    fn graph_has_node_and_edge_children() {
        let g = Graph::new(vec![1, 2, 3], vec![(1, 2), (2, 3)], 2.0, Point::ZERO, Color::BLUE);
        assert_eq!(g.nodes.len(), 3);
        assert!(!g.group.children.is_empty());
    }

    #[test]
    fn digraph_uses_arrows_and_labels() {
        let dg = DiGraph::new(vec![1, 2], vec![(1, 2)], 2.0, Point::ZERO, Color::WHITE);
        assert_eq!(dg.edges.len(), 1);
        let mut positions = HashMap::new();
        positions.insert(1, Point::new(-1.0, 0.0));
        positions.insert(2, Point::new(1.0, 0.0));
        let mut labels = HashMap::new();
        labels.insert((1, 2), "edge".into());
        let labeled = dg.with_edge_labels(&labels, &positions, Color::WHITE);
        assert!(labeled.group.children.len() > 2);
    }
}
