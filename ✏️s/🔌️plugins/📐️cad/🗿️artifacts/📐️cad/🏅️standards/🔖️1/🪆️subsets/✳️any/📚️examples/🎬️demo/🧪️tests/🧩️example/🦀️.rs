#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🌲️ The registered `demo` example (the one the shell's picker offers and announces at boot) is the
/// Concrete Forest document, and every pane handle its asset spells names a genesis scene of the bundled catalogue by
/// its content-addressed `child_id` and derives a non-empty genesis pack from it — a stale asset (fixture or conversion
/// drift re-hashes the children) would otherwise load four silently empty panes.
#[test]
fn demo_asset_is_the_concrete_forest_and_every_pane_resolves() {
    let snapshot = <crate::CadSnapshot as store::ArtifactDsl>::parse_dsl(include_str!("../../🖼️assets/🗣️.dsl.semio")).expect("demo asset parses");
    let forest = crate::standards::v1::subsets::any::schema::inferences::forest_play_scene();
    for pane in crate::CadPaneId::all() {
        assert_eq!(crate::cad_pane_model(&snapshot, pane).map(|child| child.child_id.as_str()), crate::cad_pane_model(&forest, pane).map(|child| child.child_id.as_str()), "{pane:?} pane child id");
    }
    assert_eq!(snapshot, forest);
    for pane in crate::CadPaneId::all() {
        let child = crate::cad_pane_model(&snapshot, pane).unwrap_or_else(|| panic!("{pane:?} pane composes a model child"));
        let scene = crate::cad_bundled_pane_scene(&child.child_id).unwrap_or_else(|| panic!("{pane:?} pane resolves its bundled genesis scene"));
        assert!(!crate::cad_scene_pane_objects(&scene, pane).is_empty(), "{pane:?} pane carries objects");
        let pack = crate::cad_genesis_child_pack(&snapshot, crate::cad_pane_model_slot(pane), &child.child_id).unwrap_or_else(|| panic!("{pane:?} pane derives its genesis pack"));
        let model = <semio_s_artifact_stdio_semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("genesis pack decodes");
        assert_eq!(crate::standards::v1::subsets::any::io::geometry_import::objects_from_model_snapshot(&model), crate::cad_scene_pane_objects(&scene, pane), "{pane:?} pane genesis objects survive the model bridge exactly");
    }
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    let snapshot = <crate::CadSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::CadInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::CadInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::CadInference::infer(&crate::empty_cad_snapshot()).expect("valid materialized inference fixture"), crate::standards::v1::subsets::any::schema::inferences::CadInference::default(),);
}
//#endregion 🧪️InferenceLaws

//#region 🧪️SubsetRoundtrip
use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};

struct CadAnyRoundtrip;

impl SubsetRoundtripSpec for CadAnyRoundtrip {
    type Snapshot = crate::CadSnapshot;
    type Mutation = crate::CadMutation;
    type Inference = crate::standards::v1::subsets::any::schema::inferences::CadInference;

    async fn dialect() -> store::os_io::ArtifactDialect {
        store::os_io::ArtifactDialect { artifact_kind: "s.cad.cad".into(), standard: "1".into(), subset: "*".into() }
    }

    async fn fidelity() -> IoFidelityClass {
        IoFidelityClass::Semantic
    }

    async fn drops() -> &'static [&'static str] {
        &[]
    }

    async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
        let text = asset.text.ok_or_else(|| "cad demo requires dsl text".to_string())?;
        crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).map_err(|e| e.to_string())
    }

    async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
        Ok(<Self::Snapshot as store::ArtifactPack>::encode_pack(snapshot))
    }

    async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
        <Self::Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
    }

    async fn infer(snapshot: &Self::Snapshot) -> Result<Self::Inference, semio_framework_value::ValueError> {
        use protocol::Inference;
        Self::Inference::infer(snapshot)
    }

    async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `RenameObject` is retired
        // — object fields now live inside composed `s.stdio.semio.model` CHILD documents, which
        // this demo fixture's `CadSnapshot` no longer carries inline. `RenameNode` is real and
        // unaffected (node data was never part of the deleted inline object list) and exercises the
        // identical sample-mutation-roundtrip law this spec is for.
        use crate::mutations::rename_node::RenameNode;
        use crate::CadMutation;
        let Some(node) = snapshot.nodes.first() else {
            return Vec::new();
        };
        vec![CadMutation::RenameNode(RenameNode { node_id: node.id.clone(), new_label: "Roundtrip Renamed".into() })]
    }

    async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
        std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()]).and_then(|text| crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).map_err(|e| vec![e.to_string()])).map(|_| ())
    }

    async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
        Err("SKIP:owning subset has no negative fixture".into())
    }
}

#[semio_framework_async_macros::async_test]
async fn demo_subset_integrated_roundtrip() {
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    let asset = ExampleAsset { bytes: text.as_bytes(), text: Some(text), provenance: "../../🖼️assets/🗣️.dsl.semio" };
    test_support::assert_subset_roundtrip::<CadAnyRoundtrip>(&asset, None).await;
}
//#endregion 🧪️SubsetRoundtrip
