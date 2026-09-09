use super::*;

fn page(id: &str, spread_id: &str, parent_page_id: Option<&str>) -> Page {
    serde_json::from_value(serde_json::json!({
        "id": id, "name": id, "spreadId": spread_id, "parentPageId": parent_page_id,
        "width": 210.0, "height": 297.0,
        "margins": { "top": 0.0, "right": 0.0, "bottom": 0.0, "left": 0.0 },
        "columns": { "count": 1, "gutter": 0.0 },
        "guides": [], "layerIds": [], "layers": [], "frames": [], "overrides": []
    }))
    .expect("valid page json")
}

fn spread(id: &str, page_ids: Vec<&str>) -> Spread {
    Spread { id: id.into(), name: id.into(), page_ids: page_ids.into_iter().map(String::from).collect() }
}

fn parent_page(id: &str) -> ParentPage {
    ParentPage { id: id.into(), name: id.into(), width: 210.0, height: 297.0, layer_ids: Vec::new(), layers: Vec::new(), frames: Vec::new() }
}

//#region 🧪️TopologyLaws
#[semio_framework_async_macros::async_test]
async fn empty_document_has_empty_topology() {
    let topology = compute_layout_topology(&[], &[], &[]);
    assert_eq!(topology, LayoutTopology::empty());
}

#[semio_framework_async_macros::async_test]
async fn a_page_dangling_its_spread_ref_still_sorts_cleanly() {
    // 🕳️ `spreadId` points at a spread that doesn't exist in this snapshot — the edge is simply
    // dropped (see `topological_sort`'s `indegree.contains_key` guard), not a cycle.
    let topology = compute_layout_topology(&[], &[], &[page("orphan", "missing-spread", None)]);
    assert!(topology.cycle_free);
    assert_eq!(topology.topo_order, vec!["orphan"]);
}

#[semio_framework_async_macros::async_test]
async fn master_and_spread_both_precede_their_page() {
    let topology = compute_layout_topology(&[parent_page("master-1")], &[spread("spread-1", vec!["page-1"])], &[page("page-1", "spread-1", Some("master-1"))]);
    assert!(topology.cycle_free);
    let page_depth = topology.depth["page-1"];
    assert!(page_depth > topology.depth["master-1"]);
    assert!(page_depth > topology.depth["spread-1"]);
}
//#endregion 🧪️TopologyLaws
