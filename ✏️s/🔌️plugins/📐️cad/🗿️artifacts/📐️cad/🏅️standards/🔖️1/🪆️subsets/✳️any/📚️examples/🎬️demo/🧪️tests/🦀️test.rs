#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::cad::CadSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference::infer(&crate::artifacts::cad::empty_cad_snapshot()),
        crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference::default(),
    );
}
//#endregion 🧪️InferenceLaws

//#region 🧪️SubsetRoundtrip
use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};

struct CadAnyRoundtrip;

impl SubsetRoundtripSpec for CadAnyRoundtrip {
    type Snapshot = crate::artifacts::cad::CadSnapshot;
    type Mutation = crate::artifacts::cad::CadMutation;
    type Inference = crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::CadInference;

    async fn dialect() -> store::os_io::ArtifactDialect {
        store::os_io::ArtifactDialect { artifact_kind: "s.cad".into(), standard: "1".into(), subset: "*".into() }
    }

    async fn fidelity() -> IoFidelityClass {
        IoFidelityClass::Semantic
    }

    async fn drops() -> &'static [&'static str] {
        &[]
    }

    async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
        let text = asset.text.ok_or_else(|| "cad demo requires dsl text".to_string())?;
        crate::artifacts::cad::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).map_err(|e| e.to_string())
    }

    async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
        Ok(<Self::Snapshot as store::ArtifactPack>::encode_pack(snapshot))
    }

    async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
        <Self::Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
    }

    async fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
        use protocol::Inference;
        Self::Inference::infer(snapshot)
    }

    async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `RenameObject` is retired
        // — object fields now live inside composed `s.stdio.semio.model` CHILD documents, which
        // this demo fixture's `CadSnapshot` no longer carries inline. `RenameNode` is real and
        // unaffected (node data was never part of the deleted inline object list) and exercises the
        // identical sample-mutation-roundtrip law this spec is for.
        use crate::artifacts::cad::mutations::rename_node::mutation::RenameNode;
        use crate::artifacts::cad::CadMutation;
        let Some(node) = snapshot.nodes.first() else {
            return Vec::new();
        };
        vec![CadMutation::RenameNode(RenameNode { node_id: node.id.clone(), new_label: "Roundtrip Renamed".into() })]
    }

    async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
        std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()]).and_then(|text| crate::artifacts::cad::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).map_err(|e| vec![e.to_string()])).map(|_| ())
    }

    async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
        Err("SKIP:owning subset has no negative fixture".into())
    }
}

#[semio_framework_async_macros::async_test]
async fn demo_subset_integrated_roundtrip() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let asset = ExampleAsset { bytes: text.as_bytes(), text: Some(text), provenance: "../../🖼️assets/🗣️example.dsl.semio" };
    test_support::assert_subset_roundtrip::<CadAnyRoundtrip>(&asset, None).await;
}
//#endregion 🧪️SubsetRoundtrip
