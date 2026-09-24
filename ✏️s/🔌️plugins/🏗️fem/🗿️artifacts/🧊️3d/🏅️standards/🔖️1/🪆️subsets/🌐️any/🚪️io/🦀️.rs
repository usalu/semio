//! 🚪️ IO `s.fem.fem3d` (1/🌐️any) — `io() -> IoDeclaration`: this subset's native codecs plus every
//! foreign hop, aggregated from the typed `Serializer<Fem3dSnapshot>`/`Deserializer<Fem3dSnapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Foreign io goes EXCLUSIVELY
//! through the framework's `io_mechanism` registry, reached from the sibling `🪆️subsets/🌐️any/🦀️.rs`'s
//! `io: io::io()` — the same wiring `🗒️note`/`🧱️block` use.
//!
//! The OLD `ComposerEntry`/`io_registry` export channel this file used to carry is DELETED, not
//! shimmed (ticket 26/09/06/FEM-PLUGIN-END-TO-END, W4): its `entries()` had zero callers repo-wide
//! (the only surviving mentions were this file's own doc comment and a bookkeeping note in the
//! artifact root), so nothing was ever registered on it. `import_stdio_kinds()`/`export_stdio_kinds()`
//! went with it — the live lists are `artifact_kind()`'s own fields in the artifact root.
//!
//! `derived_composition` below STAYS and is now native-only: it is the `ArtifactComposition` facet
//! `semio_framework_plugin::derive_artifact_facets!` binds in the sibling `🧬️schema/🦀️.rs`, and every
//! foreign-format branch it used to carry now lives in a typed leaf instead.
//!
//! Format coverage — IDENTICAL in `◻️2d` and `🧊️3d` (full decision table in the ticket's
//! `📓️w4-io.md`):
//!
//! | foreign dialect | direction | fidelity | behaviour |
//! |---|---|---|---|
//! | `s.stdio.txt@utf-8/*` | both | `Exact` | this subset's own `.semio` DSL snapshot text — the exact bytes `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` carries |
//! | `s.stdio.json@rfc8259/*` | both | `Exact` | the `dsl::ToValue` record tree as compact rfc8259 |
//! | `s.stdio.csv@rfc4180/*` | export | `Lossy` | the node coordinate table (`id,x,y,z`), written by stdio's own RFC 4180 codec |
//! | `s.stdio.stl@ascii/*` | export | `Lossy` | REAL geometry — tetrahedralised `FemSolid` extrusions via the meshing kernel |
//! | `s.stdio.obj@3.0/*` | export | `Lossy` | REAL geometry — same kernel, `.obj` grammar |
//!
//! No md, csv-import, stl-import or obj-import hop is declared: an envelope of the DSL in another
//! format's clothing, or a hop that can only refuse, is a stub — the document travels as txt or json.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::Fem3dAnalyzer;
    use crate::Fem3dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.fem.fem3d", standard: StandardId("1"), subset: SubsetId("*") };

    /// 🎹️ The `ArtifactComposition` facet `derive_artifact_facets!` binds in `🧬️schema/🦀️.rs`.
    /// Native-only by design: foreign dialects are the `io_mechanism` entries in `io()` below, not
    /// composer sources.
    pub struct Fem3dComposerComposition;

    impl ArtifactComposition for Fem3dComposerComposition {
        type Snapshot = Fem3dSnapshot;
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
                    let analysis = Fem3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Fem3dComposerComposition: no source in this artifact's own dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚫️GeometryImport

//#region 🔖️IoDeclaration
/// 🚪️ This subset's complete io declaration — the native `LanguagePair`s and `ArtifactCodec` plus
/// the seven typed foreign entries. `pilot_languages()` indices are fixed by that function's own
/// literal `vec![document, op, diff, pack, spr]` order, the same role→slot mapping `🗒️note`'s
/// `io()` uses.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{Fem3dMutation, Fem3dSnapshot, FEM3D_DIALECT, FEM_3D_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    /// 🎹️ One vtable row per typed leaf — the single place this subset advertises what it can and
    /// cannot convert.
    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<Fem3dSnapshot, export::txt::v_utf_8::any::Fem3dIntoTxt>(FEM3D_DIALECT),
                    deserializer_entry::<Fem3dSnapshot, import::txt::v_utf_8::any::TxtIntoFem3d>(FEM3D_DIALECT),
                    serializer_entry::<Fem3dSnapshot, export::json::v_rfc8259::any::Fem3dIntoJson>(FEM3D_DIALECT),
                    deserializer_entry::<Fem3dSnapshot, import::json::v_rfc8259::any::JsonIntoFem3d>(FEM3D_DIALECT),
                    serializer_entry::<Fem3dSnapshot, export::csv::v_rfc4180::any::Fem3dIntoCsv>(FEM3D_DIALECT),
                    serializer_entry::<Fem3dSnapshot, export::stl::v_ascii::any::Fem3dIntoStl>(FEM3D_DIALECT),
                    serializer_entry::<Fem3dSnapshot, export::obj::v3_0::any::Fem3dIntoObj>(FEM3D_DIALECT),
                ]
            })
            .as_slice()
    }

    let langs = crate::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Fem3dSnapshot, Fem3dMutation>(FEM_3D_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
