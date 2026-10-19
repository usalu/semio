use semio_framework_test_fixtures::fixture_path;

#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = std::fs::read_to_string(fixture_path!("../../🖼️assets/🗣️.dsl.semio")).expect("../../🖼️assets/🗣️.dsl.semio").leak();
    assert!(text.len() > 8);
}

/// 🌲️ The registered `demo` example (the one the shell's picker offers and announces at boot) is the
/// Concrete Forest document, and every pane handle its asset spells names a genesis scene of the bundled catalogue by
/// its content-addressed `child_id` and derives a non-empty genesis pack from it — a stale asset (fixture or conversion
/// drift re-hashes the children) would otherwise load four silently empty panes.
#[test]
fn demo_asset_is_the_concrete_forest_and_every_pane_resolves() {
    let snapshot = <semio_s_artifact_cad_cad_model::snapshot::CadSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(std::fs::read_to_string(fixture_path!("../../🖼️assets/🗣️.dsl.semio")).expect("../../🖼️assets/🗣️.dsl.semio").leak()).expect("demo asset parses");
    let forest = semio_s_artifact_cad_cad_inferences::forest_play_scene();
    for pane in semio_s_artifact_cad_cad_model::snapshot::CadPaneId::all() {
        assert_eq!(semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&snapshot, pane).map(|child| child.child_id.as_str()), semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&forest, pane).map(|child| child.child_id.as_str()), "{pane:?} pane child id");
    }
    assert_eq!(snapshot, forest);
    for pane in semio_s_artifact_cad_cad_model::snapshot::CadPaneId::all() {
        let child = semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&snapshot, pane).unwrap_or_else(|| panic!("{pane:?} pane composes a model child"));
        let scene = semio_s_artifact_cad_cad_play::cad_bundled_pane_scene(&child.child_id).unwrap_or_else(|| panic!("{pane:?} pane resolves its bundled genesis scene"));
        assert!(!semio_s_artifact_cad_cad_model::snapshot::cad_scene_pane_objects(&scene, pane).is_empty(), "{pane:?} pane carries objects");
        let pack = semio_s_artifact_cad_cad_play::cad_genesis_child_pack(&snapshot, semio_s_artifact_cad_cad_model::snapshot::cad_pane_model_slot(pane), &child.child_id).unwrap_or_else(|| panic!("{pane:?} pane derives its genesis pack"));
        let model = <semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack).expect("genesis pack decodes");
        assert_eq!(semio_s_artifact_cad_cad_model::geometry::objects_from_model_snapshot(&model), semio_s_artifact_cad_cad_model::snapshot::cad_scene_pane_objects(&scene, pane), "{pane:?} pane genesis objects survive the model bridge exactly");
    }
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = std::fs::read_to_string(fixture_path!("../../🖼️assets/🗣️.dsl.semio")).expect("../../🖼️assets/🗣️.dsl.semio").leak();
    let snapshot = <semio_s_artifact_cad_cad_model::snapshot::CadSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = semio_s_artifact_cad_cad_inferences::CadInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, semio_s_artifact_cad_cad_inferences::CadInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(semio_s_artifact_cad_cad_inferences::CadInference::infer(&semio_s_artifact_cad_cad_model::snapshot::empty_cad_snapshot()).expect("valid materialized inference fixture"), semio_s_artifact_cad_cad_inferences::CadInference::default(),);
}
//#endregion 🧪️InferenceLaws

//#region 🧪️SubsetRoundtrip
use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};

struct CadAnyRoundtrip;

impl SubsetRoundtripSpec for CadAnyRoundtrip {
    type Snapshot = semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
    type Mutation = semio_s_artifact_cad_cad_mutations::CadMutation;
    type Inference = semio_s_artifact_cad_cad_inferences::CadInference;

    async fn dialect() -> semio_framework_artifact_reference::ArtifactDialect {
        semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.cad.cad".into(), standard: "1".into(), subset: "*".into() }
    }

    async fn fidelity() -> IoFidelityClass {
        IoFidelityClass::Semantic
    }

    async fn drops() -> &'static [&'static str] {
        &[]
    }

    async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
        let text = asset.text.ok_or_else(|| "cad demo requires dsl text".to_string())?;
        semio_s_artifact_cad_cad_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).map_err(|e| e.to_string())
    }

    async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
        Ok(<Self::Snapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot))
    }

    async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
        <Self::Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
    }

    async fn infer(snapshot: &Self::Snapshot) -> Result<Self::Inference, semio_framework_value::ValueError> {
        use protocol::Inference;
        Self::Inference::infer(snapshot)
    }

    async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
        use semio_s_artifact_cad_cad_mut_nodes::rename_node::RenameNode;
        use semio_s_artifact_cad_cad_mutations::CadMutation;
        let Some(node) = snapshot.nodes.first() else {
            return Vec::new();
        };
        vec![CadMutation::RenameNode(RenameNode { node_id: node.id.clone(), new_label: "Roundtrip Renamed".into() })]
    }

    async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
        std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()]).and_then(|text| semio_s_artifact_cad_cad_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).map_err(|e| vec![e.to_string()])).map(|_| ())
    }

    async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
        Err("SKIP:owning subset has no negative fixture".into())
    }
}

#[semio_framework_async_macros::async_test]
async fn demo_subset_integrated_roundtrip() {
    let text = std::fs::read_to_string(fixture_path!("../../🖼️assets/🗣️.dsl.semio")).expect("../../🖼️assets/🗣️.dsl.semio").leak();
    let asset = ExampleAsset { bytes: text.as_bytes(), text: Some(text), provenance: "../../🖼️assets/🗣️.dsl.semio" };
    test_support::assert_subset_roundtrip::<CadAnyRoundtrip>(&asset, None).await;
}
//#endregion 🧪️SubsetRoundtrip
