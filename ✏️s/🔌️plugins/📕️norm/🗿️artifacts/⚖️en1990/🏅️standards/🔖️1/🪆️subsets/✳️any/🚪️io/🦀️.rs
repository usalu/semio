//! 🚪️ IO s.norm.en1990 (1/✳️any) — universal semio DSL/pack import+export for the native `s.norm.en1990`
//! dialect. Registration flows through 🎹️composer::register (called once from ⚙️engine::register).

use crate::En1990Snapshot;

pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["s.norm.en1990"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["s.norm.en1990"]
}

/// 📖️ Parses `.en1990` DSL bytes into a snapshot.
pub fn en1990_from_dsl_bytes(bytes: &[u8]) -> Result<En1990Snapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))?;
    crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text)
}

/// 🖨️ Prints a snapshot to `.en1990` DSL bytes.
pub fn en1990_to_dsl_bytes(snapshot: &En1990Snapshot) -> Vec<u8> {
    crate::standards::v1::subsets::any::schema::snapshot::text::print_dsl(snapshot).into_bytes()
}

/// 📦️ Decodes a semio pack into a snapshot.
pub fn en1990_from_pack(bytes: &[u8]) -> Result<En1990Snapshot, store::PackError> {
    <En1990Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

/// 📦️ Encodes a snapshot as a semio pack.
pub fn en1990_to_pack(snapshot: &En1990Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::En1990Analyzer;
    use crate::En1990Snapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1990", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct En1990ComposerComposition;

    impl ArtifactComposition for En1990ComposerComposition {
        type Snapshot = En1990Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
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
    use crate::standards::v1::subsets::any::schema::En1990Composer as En1990AnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<En1990AnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
