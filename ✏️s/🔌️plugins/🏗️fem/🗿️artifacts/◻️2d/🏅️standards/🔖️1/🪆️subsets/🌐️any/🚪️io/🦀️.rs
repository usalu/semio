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
//! | `s.stdio.json@rfc8259/*` | both | `Exact` | the `semio_framework_value::ToValue` record tree as compact rfc8259 |
//! | `s.stdio.csv@rfc4180/*` | export | `Lossy` | the node coordinate table (`id,x,y`), written by stdio's own RFC 4180 codec |
//! | `s.stdio.stl@ascii/*` | export | `Lossy` | REAL geometry — extruded `FemRegion` footprints via the meshing kernel |
//! | `s.stdio.obj@3.0/*` | export | `Lossy` | REAL geometry — same kernel, `.obj` grammar |
//!
//! No md, csv-import, stl-import or obj-import hop is declared: an envelope of the DSL in another
//! format's clothing, or a hop that can only refuse, is a stub — the document travels as txt or json.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::Fem2dAnalyzer;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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

/// 🪪️ W2 packet P7 (26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET): the canonical `ArtifactEditor`/
/// `ArtifactViewer::DIALECT` and composer dialect of this artifact — `artifact_kind` is the 3-part schema id
/// (`#[artifact_schema(id = "s.fem.fem2d")]` on `Fem2dSnapshot`), NOT the 2-part `ArtifactIdentity::parse("s.fem2d")`
/// string the artifact `definition()` uses. Lives in io, the lowest layer every editor, viewer and composer names.
pub const FEM2D_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.fem.fem2d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

/// 🪪️ The fem 2D model document kind — the id `fem2d_io`'s `ArtifactPresentation` already names (`2d.fem`),
/// with the ONE schema the hub's codec rows, document-open targets and genesis key on (`FEM_2D_SCHEMA`). Declared
/// beside [`computation_artifact_kind`], which stays the results kind `results:out` pins itself to.
pub fn document_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "2d.fem".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("FEM 2D Model", "FEM-2D-Modell"),
        source_format: FEM_2D_SCHEMA.into(),
        component_kind: "fem2d".into(),
        dimension: "2d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        schema: FEM_2D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into()],
    }
}

/// 🔌️ The computed-results output artifact kind (`results:out`'s `kind_id`, see
/// `crate::editor::fem2d::fem2d_io`) — the OS-catalog-level resource descriptor for
/// `computation.fem2d`; deliberately a different `media_type` (`Computation`×`Value`) than the PORT's
/// wire-level `Data`×`Value` (see WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE's
/// port recipe). Lifted verbatim out of the pre-migration `fem2d_ui::create_fem2d_app`'s
/// `.artifact_kind(...)` call so the app's manifest can call this instead of inlining the literal.
pub fn computation_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "computation.fem2d".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("FEM 2D Results", "FEM-2D-Ergebnisse"),
        source_format: "computation.fem2d".into(),
        component_kind: "fem2d-results".into(),
        dimension: "computation".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Value },
        schema: "computation.fem2d".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into()],
    }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `🗒️note`'s own `pilot_languages()` convention.
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "fem.fem2d",
                    extension: Some("fem2d"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("fem.fem2d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "fem.fem2d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("fem.fem2d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "fem.fem2d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("fem.fem2d.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "fem2d.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("fem2d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "fem2d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("fem2d.spr"),
                },
            ]
        })
        .as_slice()
}

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example). Per `📓️taxonomy.md`, `SetSnapshot` is banned outright with NO replacement
/// mutation: whole-document replace is not expressible as an in-history `Mutation` at all. Every
/// former "replace the whole document" gesture in this package (`import_media`'s `"artifact:in"`,
/// `commands::set_active_example`) builds this effect instead of an `Emit::mutations([...])`.
/// The spr is a fresh, edit-free op-log for `scene` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(scene);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("fem2d", semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️IoDeclaration
/// 🚪️ This subset's complete io declaration — the native `LanguagePair`s and `ArtifactCodec` plus
/// the seven typed foreign entries. `pilot_languages()` indices are fixed by that function's own
/// literal `vec![document, op, diff, pack, spr]` order, the same role→slot mapping `🗒️note`'s
/// `io()` uses.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
    use crate::standards::v1::subsets::any::io::FEM2D_DIALECT;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA;
    use semio_framework_os_kernel::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
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
                    serializer_entry::<Fem2dSnapshot, export::stl::v_ascii::any::Fem2dIntoStl>(FEM2D_DIALECT),
                    serializer_entry::<Fem2dSnapshot, export::obj::v3_0::any::Fem2dIntoObj>(FEM2D_DIALECT),
                ]
            })
            .as_slice()
    }

    let langs = crate::standards::v1::subsets::any::io::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::bare_artifact_codec::<Fem2dSnapshot, Fem2dMutation>(FEM_2D_SCHEMA.to_string()),
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

pub mod derived_construction {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
    use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Fem2dBuilderConstruction {
        snapshot: Fem2dSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Fem2dBuilderConstruction {
        type Snapshot = Fem2dSnapshot;
        type Mutation = Fem2dMutation;
        type Diff = Fem2dDiff;
        fn empty() -> Self {
            Self { snapshot: Fem2dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self { snapshot: <Fem2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?, diagnostics: Vec::new() })
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <Fem2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?, diagnostics: Vec::new() })
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Fem2dParts {
        pub snapshot: Option<Fem2dSnapshot>,
    }

    pub struct Fem2dAnalyzerAnalysis;

    impl ArtifactAnalysis for Fem2dAnalyzerAnalysis {
        type Parts = Fem2dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.fem.fem2d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Fem2dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Fem2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Fem2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec Fem2dBuilderFacets {
        construction: Fem2dBuilderConstruction,
        analysis: Fem2dAnalyzerAnalysis,
        composition: Fem2dComposerComposition,
    }
    builder: Fem2dBuilder,
    analyzer: Fem2dAnalyzer,
    composer: Fem2dComposer,
);
