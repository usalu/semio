//! 🦘️ foreign hop, aggregated from the typed `Serializer<Fem3dSnapshot>`/`Deserializer<Fem3dSnapshot>`
//! 🚪️ IO `s.fem.fem3d` (1/🌐️any) — `io() -> IoDeclaration`: this subset's native codecs plus every
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
//! | `s.stdio.csv@rfc4180/*` | export | `Lossy` | the node coordinate table (`id,x,y,z`), written by stdio's own RFC 4180 codec |
//! | `s.stdio.stl@ascii/*` | export | `Lossy` | REAL geometry — tetrahedralised `FemSolid` extrusions via the meshing kernel |
//! | `s.stdio.obj@3.0/*` | export | `Lossy` | REAL geometry — same kernel, `.obj` grammar |
//!
//! No md, csv-import, stl-import or obj-import hop is declared: an envelope of the DSL in another
//! format's clothing, or a hop that can only refuse, is a stub — the document travels as txt or json.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::Fem3dAnalyzer;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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
                    if let Some(value) = analysis.parts.snapshot {
                        let snapshot = Fem3dSnapshot::from_reference(value).map_err(|error| ComposeError { message: error.to_string(), diagnostics: Vec::new() })?;
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
    use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
    use crate::standards::v1::subsets::any::io::FEM3D_DIALECT;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA;
    use semio_framework_os_kernel::io::io_mechanism::deserializer_entry;
    use semio_framework_os_kernel::io::io_mechanism::serializer_entry;
    use semio_framework_os_kernel::io::io_mechanism::IoEntry;
    use semio_framework_plugin::app::declarations::IoDeclaration;
    use semio_framework_plugin::app::declarations::LanguagePair;
    use semio_framework_plugin::app::declarations::NativeCodecs;
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

    let langs = crate::standards::v1::subsets::any::io::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::native_artifact_codec::<Fem3dSnapshot>(FEM_3D_SCHEMA.to_string()),
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
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
    use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
    use semio_framework_plugin::ArtifactBuilder;

    /// 🏗️ The builder accumulates the explicit authoring rows and receives them into persistent ownership once, when the snapshot is built.
    #[derive(Clone, Debug, Default)]
    pub struct Fem3dBuilderConstruction {
        rows: Fem3dSnapshotValue,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Fem3dBuilderConstruction {
        type Snapshot = Fem3dSnapshot;
        type Mutation = Fem3dMutation;
        type Diff = Fem3dDiff;
        fn empty() -> Self {
            Self::default()
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { rows: snapshot.to_reference(), diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Fem3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::ColdMutation<Fem3dSnapshotValue>>::diff(&mutation, &self.rows);
            match protocol::apply_diff_cold(outcome.diff(), &self.rows) {
                Ok(rows) => self.rows = rows,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.rows = protocol::apply_diff_cold(&diff, &self.rows)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if !self.diagnostics.is_empty() {
                return Err(self.diagnostics);
            }
            Fem3dSnapshot::from_reference(self.rows).map_err(|error| vec![semio_framework_diagnostic::Diagnostic::error("build.receive", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())])
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Fem3dParts {
        pub snapshot: Option<Fem3dSnapshotValue>,
    }

    pub struct Fem3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Fem3dAnalyzerAnalysis {
        type Parts = Fem3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.fem.fem3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Fem3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot.to_reference()),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Fem3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot.to_reference()),
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
    pub spec Fem3dBuilderFacets {
        construction: Fem3dBuilderConstruction,
        analysis: Fem3dAnalyzerAnalysis,
        composition: Fem3dComposerComposition,
    }
    builder: Fem3dBuilder,
    analyzer: Fem3dAnalyzer,
    composer: Fem3dComposer,
);

//#region 🪪️ArtifactKind
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA;

/// 🪪️ W2 packet P7 (26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET): the canonical `ArtifactEditor`/
/// `ArtifactViewer::DIALECT` for this artifact — `artifact_kind` is the 3-part schema id
/// (`#[artifact_schema(id = "s.fem.fem3d")]` on `Fem3dSnapshot`), NOT the 2-part
/// `ArtifactIdentity::parse("s.fem3d")` string `definition()` below uses, and NOT the module-private
/// `FEM3D_DIALECT` in this subset's own `🚪️io/🦀️.rs` (an older, unrelated 2-part io/composer
/// dialect — different file, different scope, no collision). Lives at the ARTIFACT root so a viewer
/// file can read it without ever importing through the sibling editor module.
pub const FEM3D_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.fem.fem3d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

/// 🪪️ The fem 3D model document kind — the id `fem3d_io`'s `ArtifactPresentation` already names (`3d.fem`),
/// with the ONE schema the hub's codec rows, document-open targets and genesis key on (`FEM_3D_SCHEMA`). Declared
/// beside [`computation_artifact_kind`], which stays the results kind `results:out` pins itself to.
pub fn document_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.fem".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("FEM 3D Model", "FEM-3D-Modell"),
        source_format: FEM_3D_SCHEMA.into(),
        component_kind: "fem3d".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Any },
        schema: FEM_3D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into()],
    }
}

/// 🏷️ The `computation.fem3d` artifact kind — every load case/combination's solved
/// `semio_s_fem_model::StaticResult`, pinned to this kind by the `results:out` media port (see
/// `semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::fem3d_results_out_port`) and produced by
/// `semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::Fem3dPlayApp::export_media`. Lifted verbatim out of the old ui crate's
/// `create_fem3d_app`'s inline `.artifact_kind(...)` call so the app's manifest can reference it by name.
pub fn computation_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "computation.fem3d".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("FEM 3D Results", "FEM-3D-Ergebnisse"),
        source_format: "computation.fem3d".into(),
        component_kind: "fem3d-results".into(),
        dimension: "computation".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Value },
        schema: "computation.fem3d".into(),
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
                    id: "fem.fem3d",
                    extension: Some("fem3d"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("fem.fem3d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "fem.fem3d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("fem.fem3d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "fem.fem3d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("fem.fem3d.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "fem3d.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("fem3d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "fem3d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("fem3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️ArtifactKind

//#region 🚀️BootDocument
/// 🚀️ The document every `fem3d` surface boots with — the bundled `concrete-forest` (Betonwald) example
/// so Entwerfen-mit-Bestand surfaces and the demonstrator Statik pane paint the reuse story on first
/// frame. Shared by `Fem3dPlayApp::reference_birth` and `Fem3dViewer::reference_birth` (the viewer
/// must never import through the sibling editor module, so the shared boot document lives here). The
/// document is received into persistent ownership through the reference wallet; a fixture that does
/// not load is a typed error, never a substitute document.
pub fn fem3d_boot_snapshot() -> Result<semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot, semio_framework_diagnostic::TextError> {
    semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT)
}
//#endregion 🚀️BootDocument

//#region 🌱️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example). Per `📓️taxonomy.md`, `SetSnapshot` is banned outright with NO replacement
/// mutation: whole-document replace is not expressible as an in-history `Mutation` at all. Every
/// former "replace the whole document" gesture in this package (`import_media`'s `"artifact:in"`,
/// `commands::set_active_example`) builds this effect instead of an `Emit::mutations([...])`.
/// The spr is a fresh, edit-free op-log for `scene` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(scene);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("fem3d", semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🌱️ResetDocument
