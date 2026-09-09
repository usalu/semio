#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../../🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::Generation3dInference;
    use protocol::Inference;

    let text = include_str!("../../🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio");
    let snapshot = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Generation3dInference::infer(&snapshot), Generation3dInference::infer(&snapshot));
    snapshot.retire_cold();
}

/// 💡️ The example's own topology is REAL — this bundled fixture is a non-trivial acyclic graph, so
/// the inference must see its nodes and edges rather than fall back on a zero value.
#[test]
fn inference_of_the_example_is_a_non_trivial_acyclic_topology() {
    use crate::standards::v1::subsets::any::schema::inferences::Generation3dInference;
    use protocol::Inference;

    let text = include_str!("../../🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio");
    let snapshot = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example dsl parses");
    let inferred = Generation3dInference::infer(&snapshot);
    assert_eq!(inferred.topology.node_count as usize, snapshot.fixture.widgets.len());
    assert_eq!(inferred.topology.edge_count as usize, snapshot.fixture.synapses.len());
    assert!(inferred.topology.cycle_free, "a bundled example must be an acyclic graph");
    assert_eq!(inferred.topology.topo_order.len(), snapshot.fixture.widgets.len());
    snapshot.retire_cold();
}
//#endregion 🧪️InferenceLaws
