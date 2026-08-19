//! 🚪️ IO s.en1990 (1/✳️any) — universal semio DSL/pack import+export for the native `s.en1990`
//! dialect. Registration flows through 🎹️composer::register (called once from ⚙️engine::register).

use crate::artifacts::en1990::En1990Snapshot;

pub async fn import_stdio_kinds() -> &'static [&'static str] {
    &["s.en1990"]
}
pub async fn export_stdio_kinds() -> &'static [&'static str] {
    &["s.en1990"]
}

/// 📖️ Parses `.en1990` DSL bytes into a snapshot.
pub async fn en1990_from_dsl_bytes(bytes: &[u8]) -> Result<En1990Snapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))?;
    crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text)
}

/// 🖨️ Prints a snapshot to `.en1990` DSL bytes.
pub async fn en1990_to_dsl_bytes(snapshot: &En1990Snapshot) -> Vec<u8> {
    crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::text::print_dsl(snapshot).into_bytes()
}

/// 📦️ Decodes a semio pack into a snapshot.
pub async fn en1990_from_pack(bytes: &[u8]) -> Result<En1990Snapshot, store::PackError> {
    <En1990Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

/// 📦️ Encodes a snapshot as a semio pack.
pub async fn en1990_to_pack(snapshot: &En1990Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::en1990::standards::v1::subsets::any::schema::En1990Analyzer;
    use crate::artifacts::en1990::En1990Snapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.en1990", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct En1990ComposerComposition;

    impl ArtifactComposition for En1990ComposerComposition {
        type Snapshot = En1990Snapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = En1990Analyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "En1990ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
/// 🚪️ Composer registry (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated
/// verbatim from the deleted `⚙️engine`; io is exactly where composer dispatch belongs.
pub mod io_registry {
    use crate::artifacts::en1990::standards::v1::subsets::any::schema::En1990Composer as En1990AnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub async fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<En1990AnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1990::standards::v1::subsets::any::schema::inferences::En1990Inference;
    use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::change_resistance::mutation::ChangeResistance;
    use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::text::{parse_dsl, EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT};
    use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};
    use protocol::Inference;
    use semio_framework_plugin::{Dialect, StandardId, SubsetId};
    use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};
    use store::{ArtifactDsl, ArtifactPack};

    #[test]
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
            store::os_io::ArtifactDialect { artifact_kind: "s.en1990".into(), standard: "1".into(), subset: "*".into() }
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

    #[test]
    async fn high_consequence_office_subset_roundtrip() {
        let asset = ExampleAsset { bytes: EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT.as_bytes(), text: Some(EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT), provenance: "high-consequence-office.dsl.semio (EN 1990 CC3 office example)" };
        test_support::assert_subset_roundtrip::<En1990AnyRoundtrip>(&asset, None);
    }
}
