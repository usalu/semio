use super::*;
use crate::standards::v1::subsets::any::schema::inferences::En1990Inference;
use crate::standards::v1::subsets::any::schema::mutations::change_resistance::ChangeResistance;
use crate::standards::v1::subsets::any::schema::snapshot::text::{parse_dsl, EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT};
use crate::{En1990Mutation, En1990Snapshot};
use protocol::Inference;

use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};

#[semio_framework_async_macros::async_test]
async fn dsl_and_pack_wire_roundtrip_default() {
    let snapshot = En1990Snapshot::default();
    let dsl = en1990_to_dsl_bytes(&snapshot);
    let reparsed = en1990_from_dsl_bytes(&dsl).expect("dsl roundtrip");
    assert_eq!(reparsed, snapshot);
    let packed = en1990_to_pack(&snapshot);
    let unpacked = en1990_from_pack(&packed).expect("pack roundtrip");
    assert_eq!(unpacked, snapshot);
}

struct En1990AnyRoundtrip;

impl SubsetRoundtripSpec for En1990AnyRoundtrip {
    type Snapshot = En1990Snapshot;
    type Mutation = En1990Mutation;
    type Inference = En1990Inference;

    async fn dialect() -> store::os_io::ArtifactDialect {
        store::os_io::ArtifactDialect { artifact_kind: "s.norm.en1990".into(), standard: "1".into(), subset: "*".into() }
    }

    async fn fidelity() -> IoFidelityClass {
        IoFidelityClass::Canonical
    }

    async fn drops() -> &'static [&'static str] {
        &[]
    }

    async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
        if let Some(text) = asset.text {
            parse_dsl(text).map_err(|error| error.to_string())
        } else {
            en1990_from_pack(asset.bytes).map_err(|error| error.to_string())
        }
    }

    async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
        Ok(en1990_to_dsl_bytes(snapshot))
    }

    async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
        en1990_from_dsl_bytes(bytes).map_err(|error| error.to_string())
    }

    async fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
        En1990Inference::infer(snapshot)
    }

    async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
        vec![En1990Mutation::ChangeResistance(ChangeResistance { new_resistance_kn: snapshot.resistance_kn + 10.0 })]
    }

    async fn validate_payload(_bytes: &[u8]) -> Result<(), Vec<String>> {
        Err(vec!["SKIP:validator not wired for en1990 yet".into()])
    }

    async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
        Err("SKIP:negative validator not wired".into())
    }
}

#[semio_framework_async_macros::async_test]
async fn high_consequence_office_subset_roundtrip() {
    let asset = ExampleAsset { bytes: EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT.as_bytes(), text: Some(EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT), provenance: "high-consequence-office.dsl.semio (EN 1990 CC3 office example)" };
    test_support::assert_subset_roundtrip::<En1990AnyRoundtrip>(&asset, None).await;
}
