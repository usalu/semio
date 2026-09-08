//! 🚪️ IO `s.fem.fem2d` (1/🌐️any) — `io() -> IoDeclaration`: this subset's native codecs plus every
//! foreign hop, aggregated from the typed `Serializer<Fem2dSnapshot>`/`Deserializer<Fem2dSnapshot>`
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
//! | `s.stdio.csv@rfc4180/*` | both | `Exact` | a real RFC 4180 envelope: header `payload`, one quoted cell carrying the DSL text |
//! | `s.stdio.md@commonmark/*` | both | `Exact` | a real CommonMark envelope: one fenced code block, info string `fem2d`, carrying the DSL text |
//! | `s.stdio.stl@ascii/*` | export | `Lossy` | REAL geometry — extruded `FemRegion` footprints via the meshing kernel |
//! | `s.stdio.stl@ascii/*` | import | `Lossy` | typed `Err` — a triangle soup carries no material/section/support/load case |
//! | `s.stdio.obj@3.0/*` | export | `Lossy` | REAL geometry — same kernel, `.obj` grammar |
//! | `s.stdio.obj@3.0/*` | import | `Lossy` | typed `Err` — same reason as `stl` |
//!
//! The two refusing hops stay registered on purpose: an unregistered `(from, into)` yields a bare
//! "no route" at the router, a registered one at the weakest fidelity (`Lossy`, rank 0, so the
//! router never prefers it over a real hop) hands the caller the actual reason. They live HERE
//! rather than under `📥️import/🧩️deserializers/…/{🔺️stl,🧊️obj}` because the crate root mounts no
//! import leaf for either format — the mount tree is not this packet's file to edit, and an unmounted
//! leaf would be dead code.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::Fem2dAnalyzer;
    use crate::Fem2dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.fem.fem2d", standard: StandardId("1"), subset: SubsetId("*") };

    /// 🎹️ The `ArtifactComposition` facet `derive_artifact_facets!` binds in `🧬️schema/🦀️.rs`.
    /// Native-only by design: foreign dialects are the `io_mechanism` entries in `io()` below, not
    /// composer sources.
    pub struct Fem2dComposerComposition;

    impl ArtifactComposition for Fem2dComposerComposition {
        type Snapshot = Fem2dSnapshot;
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
                    let analysis = Fem2dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Fem2dComposerComposition: no source in this artifact's own dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚫️GeometryImport
/// 🚫️ The import direction of the two geometry hops, which HONESTLY REFUSE. Both live here rather
/// than in `📥️import/🧩️deserializers/🗿️artifacts/{🔺️stl,🧊️obj}/…` because the crate root's `#[path]`
/// mount tree declares no import leaf for either format (only the two export leaves), and a file
/// nothing mounts is dead code.
pub mod geometry_import {
    use crate::Fem2dSnapshot;
    use semio_framework::io::io_mechanism::Deserializer;
    use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
    use semio_framework_plugin::{StandardId, SubsetId};

    /// 🎯️ The foreign dialects these leaves would read.
    pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };
    pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

    const REASON: &str = "a mesh carries triangles only — it has no node/element topology, no FemMaterial, no FemSection, no FemSupport, no FemLoadCase and no analysis settings, so no fem2d model can be reconstructed from it";

    /// 🧩️ `s.stdio.stl@ascii/*` → `s.fem.fem2d@1/*` — always `Err`, see this module's doc.
    pub struct StlIntoFem2d;

    impl Deserializer<Fem2dSnapshot> for StlIntoFem2d {
        const FROM: Dialect = STL_DIALECT;
        const FIDELITY: IoFidelity = IoFidelity::Lossy;
        async fn deserialize(_payload: &IoPayload) -> IoResult<Fem2dSnapshot> {
            Err(IoError { message: format!("stl import not supported for a fem2d model: {REASON}"), diagnostics: Vec::new() })
        }
    }

    /// 🧩️ `s.stdio.obj@3.0/*` → `s.fem.fem2d@1/*` — always `Err`, see this module's doc.
    pub struct ObjIntoFem2d;

    impl Deserializer<Fem2dSnapshot> for ObjIntoFem2d {
        const FROM: Dialect = OBJ_DIALECT;
        const FIDELITY: IoFidelity = IoFidelity::Lossy;
        async fn deserialize(_payload: &IoPayload) -> IoResult<Fem2dSnapshot> {
            Err(IoError { message: format!("obj import not supported for a fem2d model: {REASON}"), diagnostics: Vec::new() })
        }
    }
}
//#endregion 🚫️GeometryImport

//#region 🔖️IoDeclaration
/// 🚪️ This subset's complete io declaration — the native `LanguagePair`s and `ArtifactCodec` plus
/// the twelve typed foreign entries. `pilot_languages()` indices are fixed by that function's own
/// literal `vec![document, op, diff, pack, spr]` order, the same role→slot mapping `🗒️note`'s
/// `io()` uses.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::standards::v1::subsets::any::io::geometry_import::{ObjIntoFem2d, StlIntoFem2d};
    use crate::{Fem2dMutation, Fem2dSnapshot, FEM2D_DIALECT, FEM_2D_SCHEMA};
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
                    serializer_entry::<Fem2dSnapshot, export::txt::v_utf_8::any::Fem2dIntoTxt>(FEM2D_DIALECT),
                    deserializer_entry::<Fem2dSnapshot, import::txt::v_utf_8::any::TxtIntoFem2d>(FEM2D_DIALECT),
                    serializer_entry::<Fem2dSnapshot, export::json::v_rfc8259::any::Fem2dIntoJson>(FEM2D_DIALECT),
                    deserializer_entry::<Fem2dSnapshot, import::json::v_rfc8259::any::JsonIntoFem2d>(FEM2D_DIALECT),
                    serializer_entry::<Fem2dSnapshot, export::csv::v_rfc4180::any::Fem2dIntoCsv>(FEM2D_DIALECT),
                    deserializer_entry::<Fem2dSnapshot, import::csv::v_rfc4180::any::CsvIntoFem2d>(FEM2D_DIALECT),
                    serializer_entry::<Fem2dSnapshot, export::md::v_commonmark::any::Fem2dIntoMd>(FEM2D_DIALECT),
                    deserializer_entry::<Fem2dSnapshot, import::md::v_commonmark::any::MdIntoFem2d>(FEM2D_DIALECT),
                    serializer_entry::<Fem2dSnapshot, export::stl::v_ascii::any::Fem2dIntoStl>(FEM2D_DIALECT),
                    deserializer_entry::<Fem2dSnapshot, StlIntoFem2d>(FEM2D_DIALECT),
                    serializer_entry::<Fem2dSnapshot, export::obj::v3_0::any::Fem2dIntoObj>(FEM2D_DIALECT),
                    deserializer_entry::<Fem2dSnapshot, ObjIntoFem2d>(FEM2D_DIALECT),
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
            codec: store::ArtifactCodec::of::<Fem2dSnapshot, Fem2dMutation>(FEM_2D_SCHEMA.to_string()),
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
