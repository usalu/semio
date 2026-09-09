use super::*;
use protocol::Inference;

//#region 🧸️Fixtures
fn snapshot_with_master_and_spread() -> LayoutSnapshot {
    let json = serde_json::json!({
        "schema": "semio.layout/v1",
        "name": "doc",
        "grid": { "baselineGrid": 12.0, "baselineOffset": 0.0, "snapToBaseline": false },
        "paragraphStyles": [],
        "characterStyles": [],
        "stories": [],
        "links": [],
        "parentPages": [
            { "id": "master-1", "name": "Master", "width": 210.0, "height": 297.0, "layerIds": [], "layers": [], "frames": [] }
        ],
        "spreads": [
            { "id": "spread-1", "name": "Spread 1", "pageIds": ["page-1"] }
        ],
        "pages": [
            {
                "id": "page-1", "name": "Page 1", "spreadId": "spread-1", "parentPageId": "master-1",
                "width": 210.0, "height": 297.0,
                "margins": { "top": 10.0, "right": 10.0, "bottom": 10.0, "left": 10.0 },
                "columns": { "count": 1, "gutter": 0.0 },
                "guides": [], "layerIds": [], "layers": [], "frames": [], "overrides": []
            }
        ],
        "printTarget": null
    });
    dsl::os_pack::from_json_str(&json.to_string()).expect("valid layout snapshot json")
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = snapshot_with_master_and_spread();
    assert_eq!(LayoutInference::infer(&snapshot), LayoutInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    let empty = dsl::os_pack::from_json_str::<LayoutSnapshot>(
        &(serde_json::json!({
            "schema": "semio.layout/v1",
            "name": "",
            "grid": { "baselineGrid": 12.0, "baselineOffset": 0.0, "snapToBaseline": false },
            "paragraphStyles": [], "characterStyles": [], "stories": [], "links": [],
            "parentPages": [], "spreads": [], "pages": [], "printTarget": null
        }))
        .to_string(),
    )
    .expect("valid empty layout snapshot json");
    assert_eq!(LayoutInference::infer(&empty), LayoutInference::default());
}

#[semio_framework_async_macros::async_test]
async fn page_topologically_follows_its_master_and_spread() {
    let snapshot = snapshot_with_master_and_spread();
    let inferred = LayoutInference::infer(&snapshot);
    let master_index = inferred.topology.topo_order.iter().position(|id| id == "master-1").unwrap();
    let spread_index = inferred.topology.topo_order.iter().position(|id| id == "spread-1").unwrap();
    let page_index = inferred.topology.topo_order.iter().position(|id| id == "page-1").unwrap();
    assert!(master_index < page_index);
    assert!(spread_index < page_index);
    assert!(inferred.topology.cycle_free);
    assert_eq!(inferred.topology.node_count, 3);
}
//#endregion 🧪️InferenceLaws
