//! 🚪️ IO s.vdi3805 (1/✳️any) — no stdio format bridges. W5a (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the five
//! degenerate leaves (csv/json/txt/xlsx/zip) that either fabricated a one-cell-CSV/raw-DSL-dump
//! shape or silently defaulted to `Vdi3805Snapshot::default()` on import (an honesty bug, not a
//! real codec). Vdi3805Snapshot is a compliance document (scalar fields plus a handful of nested
//! records), not a flat row/column table, so no honest whole-artifact CSV round-trip exists to
//! re-register in their place. Registration flows through 🎹️composer::register (called once from
//! ⚙️engine::register) for the native `s.vdi3805` dialect only.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &[]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &[]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::Vdi3805Analyzer;
    use crate::artifacts::vdi3805::Vdi3805Snapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct Vdi3805ComposerComposition;

    impl ArtifactComposition for Vdi3805ComposerComposition {
        type Snapshot = Vdi3805Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = Vdi3805Analyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Vdi3805ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️JsonSerializers
/// 🚪️ Whole-artifact JSON (de)serializers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — relocated verbatim from the deleted `⚙️engine`; serialization is exactly what `🚪️io` is for.
use crate::artifacts::vdi3805::{ManufacturerCatalog, Vdi3805Snapshot};
use crate::document::NormError;

/// 📤️ JSON round-trip for manufacturer catalogues.
pub fn catalog_to_json(catalog: &ManufacturerCatalog) -> Result<String, NormError> {
    Ok(pack::json::to_string_pretty(&pack::json::from_dsl_value(&dsl::ToValue::to_value(catalog))))
}

pub fn catalog_from_json(json: &str) -> Result<ManufacturerCatalog, NormError> {
    pack::json::from_json_str(json).map_err(|e| NormError::InvalidValue { field: "json".into(), reason: e.to_string() })
}

pub fn document_to_json(document: &Vdi3805Snapshot) -> Result<String, NormError> {
    Ok(pack::json::to_string_pretty(&pack::json::from_dsl_value(&dsl::ToValue::to_value(document))))
}

pub fn document_from_json(json: &str) -> Result<Vdi3805Snapshot, NormError> {
    pack::json::from_json_str(json).map_err(|e| NormError::InvalidValue { field: "json".into(), reason: e.to_string() })
}

//#endregion 🚪️JsonSerializers

//#region 🚪️IoRegistry
/// 🚪️ Composer registry — relocated verbatim from the deleted `⚙️engine`; io is exactly where
/// composer dispatch belongs.
pub mod io_registry {
    use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::Vdi3805Composer as Vdi3805AnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Vdi3805AnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry

//#region 🧪️JsonSerializersTests
#[cfg(test)]
mod json_serializers_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn catalog_and_document_json_round_trip() {
        let doc = Vdi3805Snapshot::default();
        let json = catalog_to_json(&doc.catalog).expect("to_json");
        let restored = catalog_from_json(&json).expect("from_json");
        assert_eq!(restored.products.len(), doc.catalog.products.len());
        assert!(catalog_from_json("not json").is_err());

        let doc_json = document_to_json(&doc).expect("doc to_json");
        let restored_doc = document_from_json(&doc_json).expect("doc from_json");
        assert_eq!(restored_doc.strict_mode, doc.strict_mode);
        assert!(document_from_json("not json").is_err());
    }
}
//#endregion 🧪️JsonSerializersTests
