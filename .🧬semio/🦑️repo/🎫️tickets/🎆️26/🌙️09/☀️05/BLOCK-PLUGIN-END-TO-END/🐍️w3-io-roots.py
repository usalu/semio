#!/usr/bin/env python3
"""🚪️ W3 — generates the three `🚪️io/🦀️.rs` roots and the three `🪆️subsets/✳️any/🦀️.rs` subset
roots of `✏️s/🔌️plugins/🧱️block`, wiring every typed io leaf onto the framework's `io_mechanism`
channel. Companion of `🐍️w3-io-leaves.py`. Re-runnable — files are written in full."""
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[7]
BLOCK = ROOT / "✏️s/🔌️plugins/🧱️block/🗿️artifacts"

SUBSETS = [
    dict(dir="◻️2d", dim="2d", p="Block2d", mod="block2d", schema="BLOCK_2D_SCHEMA",
         dialect="BLOCK2D_DIALECT", kind="s.block.block2d", apps="BlockApps",
         examples=["art_2d_hexagonal_cut_concrete_forest_left", "art_2d_hexagonal_cut_concrete_forest_right"],
         parity=True,
         assets=[("🎬️hexagonal-cut-concrete-forest-left", "🧪️hexagonal-cut-concrete-forest-left"),
                 ("➡️hexagonal-cut-concrete-forest-right", "🧪️hexagonal-cut-concrete-forest-right")]),
    dict(dir="🧊️3d", dim="3d", p="Block3d", mod="block3d", schema="BLOCK_3D_SCHEMA",
         dialect="BLOCK3D_DIALECT", kind="s.block.block3d", apps="BlockApps",
         examples=["art_3d_hexagonal_cut_concrete_forest_left", "art_3d_nakagin_capsule"],
         assets=[("🎬️hexagonal-cut-concrete-forest-left", "🧪️hexagonal-cut-concrete-forest-left"),
                 ("🏢️nakagin-capsule", "🧪️nakagin-capsule")]),
    dict(dir="🖐️5d", dim="5d", p="Block5d", mod="block5d", schema="BLOCK_5D_SCHEMA",
         dialect="BLOCK5D_DIALECT", kind="s.block.block5d", apps="BlockApps",
         examples=["art_5d_hexagonal_cut_concrete_forest_left", "art_5d_nakagin_capsule"],
         parity=True,
         assets=[("🎬️hexagonal-cut-concrete-forest-left", "🧪️hexagonal-cut-concrete-forest-left"),
                 ("🏢️nakagin-capsule", "🧪️nakagin-capsule")]),
]

IO_ROOT = '''//! 🚪️ IO `{kind}` (1/✳️any) — `io() -> IoDeclaration`: this subset's native codecs plus every
//! foreign hop, aggregated from the typed `Serializer<{p}Snapshot>`/`Deserializer<{p}Snapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Foreign io goes EXCLUSIVELY
//! through the framework's `io_mechanism` registry, reached from the sibling `🪆️subsets/✳️any/🦀️.rs`'s
//! `io: io::io()` — the same wiring `🗒️note`/`🖍️draw` use.
//!
//! The OLD `ComposerEntry`/`io_registry` export channel this file used to carry is DELETED, not
//! shimmed (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): nothing in the repo ever called its
//! `entries()`, and the `serialize_bytes` free functions it dispatched to handed back this subset's
//! DSL text mislabelled as zip/png/stl/obj bytes. `import_stdio_kinds()`/`export_stdio_kinds()` went
//! with it — the live lists are `artifact_kind()`'s own fields in the artifact root.
//!
//! `derived_composition` below STAYS and is now native-only: it is the `ArtifactComposition` facet
//! `semio_framework_plugin::derive_artifact_facets!` requires in the sibling `🧬️schema/🦀️.rs`, and
//! every foreign-format branch it used to carry now lives in a typed leaf instead.
//!
//! Format coverage — IDENTICAL in `◻️2d`, `🧊️3d` and `🖐️5d` (full decision table in the ticket's
//! `📓️w3-io.md`):
//!
//! | foreign dialect | direction | fidelity | behaviour |
//! |---|---|---|---|
//! | `s.stdio.txt@utf-8/*` | both | `Exact` | this subset's own `.semio` DSL snapshot text — the exact bytes `📚️examples/**/🗣️.dsl.semio` carry |
//! | `s.stdio.json@rfc8259/*` | both | `Exact` | the `dsl::ToValue` record tree as compact rfc8259 |
//! | `s.stdio.zip@2.0/*` | both | `Exact` | a real zip 2.0 container: `snapshot.{ext}.semio` + `snapshot.json` |
//! | `s.stdio.stl@ascii/*` | both | `Lossy` | typed `Err` — the schema carries no triangle geometry |
//! | `s.stdio.obj@3.0/*` | both | `Lossy` | typed `Err` — the schema carries no vertex/face geometry |
//! | `s.stdio.png@1.2/*` | both | `Lossy` | typed `Err` — no raster in the schema, no rasterizer here |
//!
//! The three refusing hops stay registered on purpose: an unregistered hop yields a bare "no route",
//! a registered one hands the caller the actual reason.

//#region 🎹️DerivedComposition
pub mod derived_composition {{
    use crate::artifacts::{mod}::standards::v1::subsets::any::schema::{p}Analyzer;
    use crate::artifacts::{mod}::{p}Snapshot;
    use semio_framework_plugin::{{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId}};

    const DIALECT: Dialect = Dialect {{ artifact_kind: "{kind}", standard: StandardId("1"), subset: SubsetId("*") }};

    /// 🎹️ The `ArtifactComposition` facet `derive_artifact_facets!` binds in `🧬️schema/🦀️.rs`.
    /// Native-only by design: foreign dialects are the `io_mechanism` entries in `io()` below, not
    /// composer sources.
    pub struct {p}ComposerComposition;

    impl ArtifactComposition for {p}ComposerComposition {{
        type Snapshot = {p}Snapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {{
            &[DIALECT]
        }}

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {{
            for source in sources {{
                if source.dialect == DIALECT {{
                    let native = match &source.payload {{
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    }};
                    let analysis = {p}Analyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {{
                        return Ok(Composition {{ snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics }});
                    }}
                }}
            }}
            Err(ComposeError {{ message: "{p}ComposerComposition: no source in this artifact's own dialect".into(), diagnostics: Vec::new() }})
        }}
    }}
}}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️IoDeclaration
/// 🚪️ This subset's complete io declaration — the native `LanguagePair`s and `ArtifactCodec` plus
/// the twelve typed foreign entries. `pilot_languages()` indices are fixed by that function's own
/// literal `vec![document, op, diff, pack, spr]` order, the same role→slot mapping `🗒️note`'s
/// `io()` uses.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {{
    use crate::artifacts::{mod}::io::export::serializers::artifacts as export;
    use crate::artifacts::{mod}::io::import::deserializers::artifacts as import;
    use crate::artifacts::{mod}::{{{p}Mutation, {p}Snapshot, {dialect}, {schema}}};
    use semio_framework::io::io_mechanism::{{deserializer_entry, serializer_entry, IoEntry}};
    use semio_framework_plugin::app::declarations::{{IoDeclaration, LanguagePair, NativeCodecs}};
    use std::sync::OnceLock;

    /// 🎹️ One vtable row per typed leaf, both directions per format — the single place this subset
    /// advertises what it can and cannot convert.
    fn entries() -> &'static [IoEntry] {{
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {{
                vec![
                    serializer_entry::<{p}Snapshot, export::txt::v_utf_8::any::{p}IntoTxt>({dialect}),
                    deserializer_entry::<{p}Snapshot, import::txt::v_utf_8::any::TxtInto{p}>({dialect}),
                    serializer_entry::<{p}Snapshot, export::json::v_rfc8259::any::{p}IntoJson>({dialect}),
                    deserializer_entry::<{p}Snapshot, import::json::v_rfc8259::any::JsonInto{p}>({dialect}),
                    serializer_entry::<{p}Snapshot, export::zip::v2_0::any::{p}IntoZip>({dialect}),
                    deserializer_entry::<{p}Snapshot, import::zip::v2_0::any::ZipInto{p}>({dialect}),
                    serializer_entry::<{p}Snapshot, export::stl::v_ascii::any::{p}IntoStl>({dialect}),
                    deserializer_entry::<{p}Snapshot, import::stl::v_ascii::any::StlInto{p}>({dialect}),
                    serializer_entry::<{p}Snapshot, export::obj::v3_0::any::{p}IntoObj>({dialect}),
                    deserializer_entry::<{p}Snapshot, import::obj::v3_0::any::ObjInto{p}>({dialect}),
                    serializer_entry::<{p}Snapshot, export::png::v1_2::any::{p}IntoPng>({dialect}),
                    deserializer_entry::<{p}Snapshot, import::png::v1_2::any::PngInto{p}>({dialect}),
                ]
            }})
            .as_slice()
    }}

    let langs = crate::artifacts::{mod}::pilot_languages();
    IoDeclaration {{
        native: NativeCodecs {{
            snapshot: LanguagePair {{ text: Some(&langs[0]), binary: Some(&langs[3]) }},
            diff: LanguagePair {{ text: Some(&langs[2]), binary: None }},
            mutations: LanguagePair {{ text: Some(&langs[1]), binary: Some(&langs[4]) }},
            inferences: None,
            codec: store::ArtifactCodec::of::<{p}Snapshot, {p}Mutation>({schema}.to_string()),
        }},
        entries: entries(),
    }}
}}
//#endregion 🔖️IoDeclaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {{
    use crate::artifacts::{mod}::io::export::serializers::artifacts::json::v_rfc8259::any::json_text;
    use crate::artifacts::{mod}::io::export::serializers::artifacts::obj::v3_0::any::{p}IntoObj;
    use crate::artifacts::{mod}::io::export::serializers::artifacts::png::v1_2::any::{p}IntoPng;
    use crate::artifacts::{mod}::io::export::serializers::artifacts::stl::v_ascii::any::{p}IntoStl;
    use crate::artifacts::{mod}::io::export::serializers::artifacts::txt::v_utf_8::any::dsl_text;
    use crate::artifacts::{mod}::io::export::serializers::artifacts::zip::v2_0::any::{p}IntoZip;
    use crate::artifacts::{mod}::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text;
    use crate::artifacts::{mod}::io::import::deserializers::artifacts::obj::v3_0::any::ObjInto{p};
    use crate::artifacts::{mod}::io::import::deserializers::artifacts::png::v1_2::any::PngInto{p};
    use crate::artifacts::{mod}::io::import::deserializers::artifacts::stl::v_ascii::any::StlInto{p};
    use crate::artifacts::{mod}::io::import::deserializers::artifacts::txt::v_utf_8::any::from_dsl_text;
    use crate::artifacts::{mod}::io::import::deserializers::artifacts::zip::v2_0::any::{{from_zip_bytes, ZIP_MAGIC}};
    use crate::artifacts::{mod}::{p}Snapshot;
    use semio_framework::io::io_mechanism::{{Deserializer, Serializer}};
    use semio_framework::io_schema::IoPayload;

    /// 📄️ Every handcrafted `.semio` DSL example asset of this subset — the language-agnostic
    /// fixtures the TypeScript mirror's own test reads too.
    const EXAMPLES: &[(&str, &str)] = &[
{examples_rs}    ];

    #[semio_framework_async_macros::async_test]
    async fn txt_round_trips_every_example() {{
        for (id, text) in EXAMPLES {{
            let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{{id}}: {{error:?}}"));
            let printed = dsl_text(&snapshot);
            assert_eq!(printed.trim_end_matches('\\n'), text.trim_end_matches('\\n'), "{{id}}: txt export must reproduce the example asset");
            assert_eq!(from_dsl_text(&printed).unwrap(), snapshot, "{{id}}: txt is not a fixed point");
        }}
    }}

    #[semio_framework_async_macros::async_test]
    async fn json_round_trips_every_example() {{
        for (id, text) in EXAMPLES {{
            let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{{id}}: {{error:?}}"));
            let json = json_text(&snapshot);
            assert_eq!(from_json_text(&json).unwrap_or_else(|error| panic!("{{id}}: {{error:?}}")), snapshot, "{{id}}: json is not a lossless round trip");
        }}
    }}

    #[semio_framework_async_macros::async_test]
    async fn zip_round_trips_every_example_as_a_real_archive() {{
        for (id, text) in EXAMPLES {{
            let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{{id}}: {{error:?}}"));
            let IoPayload::Binary(bytes) = {p}IntoZip::serialize(&snapshot).await.unwrap().value else {{
                panic!("{{id}}: zip export must be a binary payload");
            }};
            assert!(bytes.starts_with(ZIP_MAGIC), "{{id}}: zip export must be a real zip 2.0 container");
            assert_eq!(from_zip_bytes(&bytes).unwrap_or_else(|error| panic!("{{id}}: {{error:?}}")), snapshot, "{{id}}: zip is not a lossless round trip");
        }}
    }}

    #[semio_framework_async_macros::async_test]
    async fn geometry_and_raster_hops_refuse_with_a_reason() {{
        let snapshot = {p}Snapshot::default();
        let empty_text = IoPayload::Text(String::new());
        let empty_binary = IoPayload::Binary(Vec::new());
        for message in [
            {p}IntoStl::serialize(&snapshot).await.expect_err("stl export must refuse").message,
            {p}IntoObj::serialize(&snapshot).await.expect_err("obj export must refuse").message,
            {p}IntoPng::serialize(&snapshot).await.expect_err("png export must refuse").message,
            StlInto{p}::deserialize(&empty_text).await.expect_err("stl import must refuse").message,
            ObjInto{p}::deserialize(&empty_text).await.expect_err("obj import must refuse").message,
            PngInto{p}::deserialize(&empty_binary).await.expect_err("png import must refuse").message,
        ] {{
            assert!(message.contains("not supported for"), "every refusing hop must name the reason, got: {{message}}");
        }}
    }}

{parity_test}
    #[semio_framework_async_macros::async_test]
    async fn io_declaration_registers_both_directions_of_all_six_formats() {{
        let declaration = super::io();
        assert_eq!(declaration.entries.len(), 12, "six formats x two directions");
        let own = "{kind}";
        let mut foreign: Vec<&str> = Vec::new();
        for entry in declaration.entries {{
            assert!(entry.from.artifact_kind == own || entry.into.artifact_kind == own, "every entry must touch this subset's own dialect");
            foreign.push(if entry.from.artifact_kind == own {{ entry.into.artifact_kind }} else {{ entry.from.artifact_kind }});
        }}
        foreign.sort_unstable();
        foreign.dedup();
        assert_eq!(foreign, vec!["s.stdio.json", "s.stdio.obj", "s.stdio.png", "s.stdio.stl", "s.stdio.txt", "s.stdio.zip"]);
    }}
}}
//#endregion 🧪️Tests
'''

PARITY_TEST = '''
    /// 🧫️ The exact json bytes the TypeScript mirror (`🚪️io/🧪️tests/🟦️.ts`) writes for the same
    /// example assets — a disagreement fails HERE as well as in `bun test`, so neither
    /// implementation can drift silently.
    const JSON_PARITY_FIXTURES: &[(&str, &str)] = &[
{fixtures_rs}    ];

    #[semio_framework_async_macros::async_test]
    async fn json_matches_the_typescript_parity_fixture() {{
        for ((id, text), (fixture_id, fixture)) in EXAMPLES.iter().zip(JSON_PARITY_FIXTURES) {{
            assert_eq!(id, fixture_id, "EXAMPLES and JSON_PARITY_FIXTURES must list the same assets in the same order");
            let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{{id}}: {{error:?}}"));
            assert_eq!(json_text(&snapshot).as_str(), *fixture, "{{id}}: the Rust json must match the TypeScript parity fixture byte for byte");
        }}
    }}
'''

SUBSET_ROOT = '''//! 🪆️ Subset root for `{kind}@1/*` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
//! `descriptor-prep`, following `🔱️trinity`'s `fleet-trinity-recipe`). Exports
//! `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/
//! `📚️examples` children — `crate::editor::{mod}`/`crate::viewer::{mod}` stay mounted at the
//! plugin's top-level `editor`/`viewer` modules, not here.
//!
//! 🚪️ `io: io::io()` matches the `🗒️note`/`🖍️draw`/`🔱️trinity` template exactly: the local
//! `io_declaration()` this file used to carry (with `entries: &[]` and a DEVIATION note explaining
//! that the six foreign formats stayed unregistered on the `io_mechanism` channel) is gone — ticket
//! 26/09/05/BLOCK-PLUGIN-END-TO-END, W3 hand-authored the twelve typed
//! `Serializer<{p}Snapshot>`/`Deserializer<{p}Snapshot>` leaves that gap called for and relocated the
//! declaration into `🚪️io/🦀️.rs` as `io()`. See that file's own module doc for the per-format
//! fidelity table.

use crate::artifacts::{mod}::standards::v1::subsets::any::{{io, schema}};
use crate::artifacts::{mod}::{dialect};
use crate::editor::{mod} as editor;
use crate::viewer::{mod} as viewer;
use semio_framework_plugin::app::declarations::{{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration}};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {{
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![{examples_calls}]).as_slice()
}}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {{
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::{mod}_artifact_inference_descriptor()]).as_slice()
}}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::{apps}> {{
    SubsetDeclaration {{
        dialect: {dialect},
        schema: SchemaDeclaration {{ descriptor: schema::{mod}_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() }},
        io: io::io(),
        viewer: viewer_surface::<viewer::{p}Viewer, crate::{apps}>(viewer::create_{mod}_viewer()),
        editor: editor_surface::<editor::{p}PlayApp, crate::{apps}>(editor::create_{mod}_app()),
        examples: examples(),
    }}
}}
'''


def main() -> None:
    for s in SUBSETS:
        base = BLOCK / s["dir"] / "🏅️standards/🔖️1/🪆️subsets/✳️any"
        examples_rs = "".join(
            f'        ("{asset.removeprefix("🧪️")}", include_str!("../📚️examples/{ex}/🖼️assets/{asset}/🗣️.dsl.semio")),\n'
            for ex, asset in s["assets"])
        fixtures_rs = "".join(
            f'        ("{asset.removeprefix("🧪️")}", include_str!("🧪️tests/🧫️fixtures/{asset.removeprefix("🧪️")}.json")),\n'
            for _, asset in s["assets"])
        parity_test = PARITY_TEST.format(fixtures_rs=fixtures_rs) if s.get("parity") else ""
        io_root = IO_ROOT.format(kind=s["kind"], p=s["p"], mod=s["mod"], ext=s["mod"],
                                 dialect=s["dialect"], schema=s["schema"], examples_rs=examples_rs,
                                 parity_test=parity_test)
        (base / "🚪️io/🦀️.rs").write_text(io_root, encoding="utf-8")
        print(f"wrote {(base / '🚪️io/🦀️.rs').relative_to(ROOT)}")

        subset_root = SUBSET_ROOT.format(
            kind=s["kind"], p=s["p"], mod=s["mod"], dialect=s["dialect"], apps=s["apps"],
            examples_calls=", ".join(f"crate::examples::{e}::source()" for e in s["examples"]))
        (base / "🦀️.rs").write_text(subset_root, encoding="utf-8")
        print(f"wrote {(base / '🦀️.rs').relative_to(ROOT)}")


if __name__ == "__main__":
    main()
