use semio_framework_artifact_flow_flow::{FlowHostSnapshot, WidgetLayout};

/// 🎬️ The published `demo`: the default slider → add → preview graph, laid out left to right. Its
/// widgets live in the composed `content` child, so the asset carries them in the host grammar that
/// the explicit `examples::demo::snapshot_from_text` boundary caches as the child's genesis scene — a content REFERENCE alone loads an
/// empty canvas (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, `📓️flow.md` §8).
fn demo_host_snapshot() -> FlowHostSnapshot {
    let mut host = FlowHostSnapshot::default();
    for (id, x) in [("slider", 40.0), ("add", 120.0), ("preview", 240.0)] {
        host.layout.insert(id.to_string(), WidgetLayout { x, y: -40.0 });
    }
    host
}

fn demo_text(host: &FlowHostSnapshot) -> String {
    store::ArtifactDsl::print_dsl(host)
}

#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// ⚖️ LAW: the shipped demo is exactly the writer's output and loads a VISIBLE graph — three laid-out
/// widgets, two resolvable synapses, and a genesis pack for its composed `content` child.
#[semio_framework_async_macros::async_test]
async fn demo_example_ships_the_laid_out_default_graph_as_its_content_genesis() {
    let host = demo_host_snapshot();
    assert_eq!(crate::examples::demo::PRIMARY_TEXT, demo_text(&host), "handcrafted demo asset differs from its canonical host graph");
    host.retire_cold();
    let snapshot = crate::examples::demo::snapshot_from_text(crate::examples::demo::PRIMARY_TEXT).expect("demo parses");
    let scene = snapshot.to_host_snapshot();
    let ids: Vec<&str> = scene.widgets.iter().map(crate::schema::widget_id).collect();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral demo graph");
    assert_eq!(crate::FLOW_DOCUMENT_SCHEMA, fixture["parentSchema"].as_str().unwrap());
    assert_eq!(serde_json::to_value(&ids).expect("independent widget array"), fixture["widgetIds"]);
    assert_eq!(scene.synapses.len() as u64, fixture["synapseCount"].as_u64().unwrap());
    let actual_layout: serde_json::Value = scene.layout.iter().map(|(id, position)| (id.clone(), serde_json::json!({"x": position.x, "y": position.y}))).collect();
    assert_eq!(actual_layout, fixture["layout"]);
    assert!(scene.synapses.iter().all(|synapse| ids.contains(&synapse.from.as_str()) && ids.contains(&synapse.to.as_str())));
    assert!(ids.iter().all(|id| scene.layout.get(*id).is_some()), "every demo widget carries its layout");
    println!("[DEBUG] Flow handcrafted demo matches independent serde_json: widgets={} synapses={} layouts={}", ids.len(), scene.synapses.len(), scene.layout.len());
    scene.retire_cold();
    assert!(crate::flow_genesis_content_pack(&snapshot, "content", &snapshot.content.child_id).is_some(), "the demo's content child must have a genesis pack, or the canvas loads empty");
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = crate::examples::demo::snapshot_from_text(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&crate::FlowSnapshot::default()).expect("valid materialized inference fixture"), crate::standards::v1::subsets::any::schema::inferences::FlowInference::default(),);
}
//#endregion 🧪️InferenceLaws
