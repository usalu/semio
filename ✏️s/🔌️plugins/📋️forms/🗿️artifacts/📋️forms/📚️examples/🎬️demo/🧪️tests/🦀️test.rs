#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::artifacts::forms::FormsSnapshot;
    use protocol::Inference;

    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo asset parses as a forms snapshot");
    let inference = crate::artifacts::forms::standards::v1::subsets::any::schema::inferences::FormsInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::forms::standards::v1::subsets::any::schema::inferences::FormsInference::infer(&snapshot));

    let expected_nodes: u32 = snapshot.steps.iter().map(|step| 1 + step.blocks.len() as u32).sum();
    assert_eq!(inference.topology.node_count, expected_nodes);
    assert_eq!(inference.topology.topo_order.len() as u32, expected_nodes);
    assert!(inference.topology.cycle_free, "the demo document has no cyclic block conditions");
}

#[test]
fn inference_default_law() {
    use crate::artifacts::forms::FormsSnapshot;
    use crate::artifacts::forms::standards::v1::subsets::any::schema::inferences::FormsInference;
    use protocol::Inference;

    assert_eq!(FormsInference::infer(&FormsSnapshot::default()), FormsInference::default());
}
//#endregion 💡️InferenceLaws
