
use super::*;

#[test]
fn adjacency_view_exposes_neighbors_and_regions() {
    let view = AdjacencyView::new(vec![vec![NodeId(1)], vec![NodeId(0), NodeId(2)], vec![NodeId(1)]], vec![RegionId(0), RegionId(1), RegionId(0)]);
    assert_eq!(view.node_count(), 3);
    assert_eq!(view.neighbors(NodeId(1)), &[NodeId(0), NodeId(2)]);
    assert_eq!(view.region_of(NodeId(1)), RegionId(1));
}
