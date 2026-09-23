use semio_framework_artifact_flow_flow::{FlowHostSnapshot, WidgetLayout};

/// 🎬️ The published `demo`: the default slider → add → preview graph, laid out left to right. Its
/// widgets live in the composed `content` child, so the asset carries them in the host grammar that
/// `FlowSnapshot::parse_dsl` caches as the child's genesis scene — a content REFERENCE alone loads an
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
    assert_eq!(crate::examples::demo::PRIMARY_TEXT, demo_text(&host), "demo asset drifted from demo_host_snapshot(); re-run `--ignored zzz_write_demo_example_asset`");
    host.retire_cold();
    let snapshot = <crate::FlowSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("demo parses");
    let scene = snapshot.to_host_snapshot();
    let ids: Vec<&str> = scene.widgets.iter().map(crate::schema::widget_id).collect();
    assert_eq!(ids, ["slider", "add", "preview"]);
    assert_eq!(scene.synapses.len(), 2);
    assert!(scene.synapses.iter().all(|synapse| ids.contains(&synapse.from.as_str()) && ids.contains(&synapse.to.as_str())));
    assert!(ids.iter().all(|id| scene.layout.get(*id).is_some()), "every demo widget carries its layout");
    scene.retire_cold();
    assert!(crate::flow_genesis_content_pack(&snapshot, "content", &snapshot.content.child_id).is_some(), "the demo's content child must have a genesis pack, or the canvas loads empty");
    eprintln!("[DEBUG] flow demo example ships 3 laid-out widgets + 2 synapses as its content genesis");
}

/// 🖊️ The ONLY way the demo asset is refreshed: `cargo test -p semio-s-artifact-flow-flow --lib --
/// --ignored zzz_write_demo_example_asset`, then re-run the law above.
#[semio_framework_async_macros::async_test]
#[ignore]
async fn zzz_write_demo_example_asset() {
    let host = demo_host_snapshot();
    let asset = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio");
    std::fs::write(&asset, demo_text(&host)).expect("write demo asset");
    host.retire_cold();
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::FlowSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&snapshot);
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::FlowInference::infer(&crate::FlowSnapshot::default()), crate::standards::v1::subsets::any::schema::inferences::FlowInference::default(),);
}
//#endregion 🧪️InferenceLaws
