//! 🚪️ IO s.iso16757 (1/✳️any) — no stdio format bridges. W5a (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the five
//! degenerate leaves (csv/json/txt/xlsx/zip) that either fabricated a one-cell-CSV/raw-DSL-dump
//! shape or silently defaulted to `Iso16757Snapshot::default()` on import (an honesty bug, not a
//! real codec). Iso16757Snapshot is a compliance document (scalar fields plus a handful of nested
//! records), not a flat row/column table, so no honest whole-artifact CSV round-trip exists to
//! re-register in their place. Registration flows through 🎹️composer::register (called once from
//! ⚙️engine::register) for the native `s.iso16757` dialect only.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &[]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &[]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::iso16757::standards::v1::subsets::any::schema::Iso16757Analyzer;
    use crate::artifacts::iso16757::Iso16757Snapshot;
    use semio_framework_plugin::ArtifactAnalyzer as _;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.iso16757", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct Iso16757ComposerComposition;

    impl ArtifactComposition for Iso16757ComposerComposition {
        type Snapshot = Iso16757Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = Iso16757Analyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Iso16757ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️JsonSerializers
/// 🚪️ Whole-artifact JSON serializers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`; serialization is exactly what `🚪️io` is for.
use crate::document::NormError;

pub mod io {
    use super::*;

    pub fn catalogue_to_json(catalogue: &crate::artifacts::iso16757::part_1::Catalogue) -> Result<String, NormError> {
        serde_json::to_string_pretty(catalogue).map_err(|e| NormError::InvalidValue { field: "catalogue".into(), reason: e.to_string() })
    }

    pub fn catalogue_from_json(json: &str) -> Result<crate::artifacts::iso16757::part_1::Catalogue, NormError> {
        serde_json::from_str(json).map_err(|e| NormError::InvalidValue { field: "catalogue".into(), reason: e.to_string() })
    }

    pub fn dictionary_to_json(dictionary: &crate::artifacts::iso16757::part_4::Dictionary) -> Result<String, NormError> {
        serde_json::to_string_pretty(dictionary).map_err(|e| NormError::InvalidValue { field: "dictionary".into(), reason: e.to_string() })
    }
}

//#endregion 🚪️JsonSerializers

//#region 🚪️IoRegistry
/// 🚪️ Composer registry — relocated verbatim from the deleted `⚙️engine`; io is exactly where
/// composer dispatch belongs.
pub mod io_registry {
    use crate::artifacts::iso16757::standards::v1::subsets::any::schema::Iso16757Composer as Iso16757AnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Iso16757AnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry

//#region 🧪️JsonSerializersTests
#[cfg(test)]
mod json_serializers_tests {
    use super::*;
    use crate::artifacts::iso16757::Iso16757Snapshot;

    #[test]
    fn catalogue_json_round_trip() {
        let doc = Iso16757Snapshot::default();
        let json = io::catalogue_to_json(&doc.catalogue).expect("json");
        let restored = io::catalogue_from_json(&json).expect("restore");
        assert_eq!(restored.id, doc.catalogue.id);
    }

    #[test]
    fn dictionary_json_round_trip() {
        let doc = Iso16757Snapshot::default();
        let json = io::dictionary_to_json(&doc.dictionary).expect("json");
        assert!(json.contains("hvac-dict"));
    }
}
//#endregion 🧪️JsonSerializersTests
