#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""W12+: migrate one stdio artifact to the standards/subsets tree, following the
hand-verified 💾️binary pilot exactly. Idempotent-ish (checks before moving), but
NOT safe to re-run after a partial glue.rs edit -- run once per artifact, verify,
then move on. Usage: python3 w12_migrate_stdio_artifact.py <dir> [<dir> ...]
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
STDIO_ART = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
GLUE = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs")
HERE = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(HERE, "w9_standards_table.json"), encoding="utf-8") as f:
    STANDARDS = json.load(f)["stdio"]

with open(os.path.join(HERE, "w9_owner_table_v2.json"), encoding="utf-8") as f:
    OWNER_V2 = json.load(f)

# kind id (owner-table key, e.g. "txt") -> dir (e.g. "📄txt")
KIND_TO_DIR = {k: v["dir"] for k, v in OWNER_V2["stdio_roster"].items()}
DIR_TO_KIND = {v: k for k, v in KIND_TO_DIR.items()}
DEPENDS = {k: v.get("depends", []) for k, v in json.load(open(os.path.join(os.path.dirname(HERE), "🧪owner-table.json"), encoding="utf-8"))["stdio_roster"].items()}


def rust_mod_from_slug(slug):
    out = []
    for ch in slug:
        out.append(ch if (ch.isalnum() and ch.isascii()) else "_")
    out = "".join(out)
    if out[0].isdigit():
        out = "v" + out
    return "v_" + out if not out.startswith("v_") and not out[0].isalpha() else ("v" + out if out[0].isdigit() else out)


def std_mod_name(slug):
    # Match the convention used for binary: "raw" -> "v_raw", "2.0" -> "v2_0", "1.0" -> "v1_0"
    s = re.sub(r"[^a-zA-Z0-9]", "_", slug)
    if s[0].isdigit():
        return "v" + s.replace("_", "_", 1) if False else "v" + s
    return "v_" + s


def artifact_name_from_root(dir_name):
    """Extract the PascalCase type prefix (e.g. 'Txt') from the artifact's root component.rs."""
    root_rs = os.path.join(STDIO_ART, dir_name, "🦀️component.rs")
    text = open(root_rs, encoding="utf-8").read()
    m = re.search(r"schema::snapshot::(\w+)Snapshot", text)
    if not m:
        raise SystemExit(f"could not find <Name>Snapshot pattern in {root_rs}")
    return m.group(1)


def migrate(dir_name):
    kind = DIR_TO_KIND[dir_name]
    std = STANDARDS[kind]
    slug = std["slug"]
    std_dir = std["dir"]  # e.g. "🔖️raw"
    mod = std["rust_mod"]  # e.g. "v_raw"
    Name = artifact_name_from_root(dir_name)
    name_lower = kind  # module name used in crate::artifacts::<name_lower>

    art_root = os.path.join(STDIO_ART, dir_name)
    std_root = os.path.join(art_root, "🏅️standards", std_dir)
    subset_root = os.path.join(std_root, "🪆️subsets", "✳️any")
    os.makedirs(subset_root, exist_ok=True)

    def mv(src_rel, dst_rel):
        src = os.path.join(art_root, src_rel)
        dst = os.path.join(art_root, dst_rel)
        if not os.path.exists(src):
            return False
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        os.rename(src, dst)
        return True

    # 1. Move schema, engine, io verbatim.
    mv("🧬️schema", f"🏅️standards/{std_dir}/🪆️subsets/✳️any/🧬️schema")
    mv("⚙️engine", f"🏅️standards/{std_dir}/⚙️engine")
    had_io = mv("🚪️io", f"🏅️standards/{std_dir}/🪆️subsets/✳️any/🚪️io")

    # 2. Insert target standard/subset into every io leaf's target path
    #    (.../🗿️artifacts/<target>/component.* -> .../🗿️artifacts/<target>/🔖️<tstd>/✳️any/component.*)
    if had_io:
        io_root = os.path.join(subset_root, "🚪️io")
        for direction in ("📥️import/🧩️deserializers", "📤️export/🧵️serializers"):
            base = os.path.join(io_root, direction, "🗿️artifacts")
            if not os.path.isdir(base):
                continue
            for target_dir in os.listdir(base):
                target_path = os.path.join(base, target_dir)
                if not os.path.isdir(target_path):
                    continue
                target_kind = DIR_TO_KIND.get(target_dir)
                if target_kind is None:
                    continue
                tstd = STANDARDS[target_kind]["dir"]
                new_target_path = os.path.join(target_path, tstd, "✳️any")
                os.makedirs(new_target_path, exist_ok=True)
                for leaf in os.listdir(target_path):
                    leaf_path = os.path.join(target_path, leaf)
                    if os.path.isfile(leaf_path):
                        os.rename(leaf_path, os.path.join(new_target_path, leaf))

    # 3. Move builder content to subset level; scaffold standard+artifact facades.
    os.makedirs(os.path.join(subset_root, "🏗️builder"), exist_ok=True)
    os.makedirs(os.path.join(subset_root, "🧐️analyzer"), exist_ok=True)
    os.makedirs(os.path.join(subset_root, "🎹️composer"), exist_ok=True)
    os.makedirs(os.path.join(std_root, "🏗️builder"), exist_ok=True)
    os.makedirs(os.path.join(std_root, "🧐️analyzer"), exist_ok=True)
    os.makedirs(os.path.join(std_root, "🎹️composer"), exist_ok=True)
    os.makedirs(os.path.join(art_root, "🎹️composer"), exist_ok=True)
    os.makedirs(os.path.join(std_root, "🏗️builder"), exist_ok=True)
    os.makedirs(os.path.join(art_root, "🏗️builder"), exist_ok=True)
    os.makedirs(os.path.join(std_root, "🧐️analyzer"), exist_ok=True)
    os.makedirs(os.path.join(art_root, "🧐️analyzer"), exist_ok=True)

    # Idempotency guard: once write_facades() has run once, art_root/🏗️builder holds a FACADE
    # (not the original real content), so a naive re-run would move that facade into the subset
    # and clobber the real materializer. Only move if the subset doesn't already have real content.
    subset_builder_rs = os.path.join(subset_root, "🏗️builder", "🦀️component.rs")
    if not os.path.exists(subset_builder_rs):
        for ext in ("🦀️component.rs", "🟦️component.ts"):
            mv(f"🏗️builder/{ext}", f"🏅️standards/{std_dir}/🪆️subsets/✳️any/🏗️builder/{ext}")

    # 4. Replace decomposer with a fresh, template-generated subset analyzer (string-surgery on
    #    the old decompose() body proved too fragile across artifacts with slightly different
    #    formatting -- generating from the hand-verified binary template is far more reliable).
    import shutil
    decomposer_dir = os.path.join(art_root, "🪓️decomposer")
    analyzer_rs_new = os.path.join(subset_root, "🧐️analyzer", "🦀️component.rs")
    os.makedirs(os.path.dirname(analyzer_rs_new), exist_ok=True)
    open(analyzer_rs_new, "w", encoding="utf-8").write(
        SUBSET_ANALYZER.format(Name=Name, kind=kind, slug=slug)
    )
    if os.path.isdir(decomposer_dir):
        shutil.rmtree(decomposer_dir, ignore_errors=True)

    # 5. Write facades (builder, analyzer at standard+artifact level; composer at all 3).
    write_facades(art_root, std_root, subset_root, std_dir, mod, Name, kind, slug)

    print(f"OK  {dir_name:12s} kind={kind:10s} Name={Name:10s} standard={slug}")


SUBSET_ANALYZER = """//! 🧐️ {Name}Analyzer ({slug}/✳️any) — read-only analysis, successor to the pre-migration
//! {Name}Decomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource}};
use crate::artifacts::{kind}::{Name}Snapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.{kind}` parts.
#[derive(Clone, Debug, Default)]
pub struct {Name}Parts {{
    pub snapshot: Option<{Name}Snapshot>,
}}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.{kind}` ({slug}/✳️any) sources.
pub struct {Name}Analyzer;

impl ArtifactAnalyzer for {Name}Analyzer {{
    type Parts = {Name}Parts;
    const DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.{kind}", standard: StandardId("{slug}"), subset: SubsetId("*") }};

    fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {{
        IoConfidence::Medium
    }}

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {{
        let mut parts = {Name}Parts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {{
            match source {{
                AnalyzeSource::Text(text) => match <{Name}Snapshot as store::DocumentDsl>::parse_dsl(text) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.text",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }}
                }},
                AnalyzeSource::Binary(bytes) => match <{Name}Snapshot as store::DocumentPack>::decode_pack(bytes) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.binary",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }}
                }},
            }}
        }}
        Analysis {{ parts, dialect: Self::DIALECT, confidence, diagnostics }}
    }}
}}
//#endregion 🔖️Analyzer
"""

BUILDER_FACADE = """//! 🏗️ {Name}Builder ({level}) — delegates to {target}.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::{kind}::{{{Name}Diff, {Name}Mutation, {Name}Snapshot}};
use {target_path}::{Name}Builder as {SourceAlias};

#[derive(Clone, Debug, Default)]
pub struct {Name}Builder({SourceAlias});

impl ArtifactBuilder for {Name}Builder {{
    type Snapshot = {Name}Snapshot;
    type Mutation = {Name}Mutation;
    type Diff = {Name}Diff;
    fn empty() -> Self {{ Self({SourceAlias}::empty()) }}
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {{ Self({SourceAlias}::from_snapshot(snapshot)) }}
    fn from_text(text: &str) -> Result<Self, store::TextError> {{ Ok(Self({SourceAlias}::from_text(text)?)) }}
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {{ Ok(Self({SourceAlias}::from_binary(bytes)?)) }}
    fn mutate(self, mutation: Self::Mutation) -> Self {{ Self(self.0.mutate(mutation)) }}
    fn absorb(self, diff: Self::Diff) -> Self {{ Self(self.0.absorb(diff)) }}
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {{ self.0.build() }}
}}
"""

ANALYZER_FACADE = """//! 🧐️ {Name}Analyzer ({level}) — delegates to {target}.

use semio_framework_plugin::{{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource}};
use {target_path}::{Name}Analyzer as {SourceAlias};
pub use {target_path}::{Name}Parts;

const DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.{kind}", standard: StandardId("{slug}"), subset: SubsetId("*") }};

pub struct {Name}Analyzer;

impl ArtifactAnalyzer for {Name}Analyzer {{
    type Parts = {Name}Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {{ {SourceAlias}::sniff(source) }}
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {{ {SourceAlias}::analyze(sources) }}
}}
"""

SUBSET_COMPOSER = """//! 🎹️ {Name}Composer (raw/✳️any at {slug}) — analyzer + builder glued. Reads native
//! `stdio.{kind}` sources{extra_reads_doc}, writes one `stdio.{kind}` ({slug}/✳️any) snapshot.

use semio_framework_plugin::{{ArtifactComposer, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource}};
use crate::artifacts::{kind}::{Name}Snapshot;
use crate::artifacts::{kind}::standards::{mod}::subsets::any::analyzer::{Name}Analyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.{kind}", standard: StandardId("{slug}"), subset: SubsetId("*") }};
{reads_consts}

pub struct {Name}Composer;

impl ArtifactComposer for {Name}Composer {{
    type Snapshot = {Name}Snapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {{
        &[{reads_list}]
    }}

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {{
        // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
        // analyzer already round-trips through `store::Document{{Dsl,Pack}}` -- including bytes
        // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
        // like binary) that payload IS the same byte/text shape `analyze` already accepts.
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| {reads_filter})
            .map(|s| match &s.payload {{
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            }})
            .collect();
        if native.is_empty() {{
            return Err(ComposeError {{ message: "{Name}Composer: no source in a known read dialect".into(), diagnostics: Vec::new() }});
        }}
        let analysis = {Name}Analyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {{
            message: "{Name}Composer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        }})?;
        Ok(Composition {{ snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics }})
    }}
}}
"""

STANDARD_COMPOSER = """//! 🎹️ {Name}Composer ({slug} standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{{ComposerEntry, composer_entry_of}};
use crate::artifacts::{kind}::standards::{mod}::subsets::any::composer::{Name}Composer as {Name}RawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {{
    ENTRIES.get_or_init(|| vec![composer_entry_of::<{Name}RawAnyComposer>()]).as_slice()
}}
"""

ARTIFACT_COMPOSER = """//! 🎹️ {Name}Composer (final, artifact-level) — union over every standard's composer entries.

use std::sync::OnceLock;
use semio_framework_plugin::{{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries}};
use crate::artifacts::{kind}::standards::{mod}::composer as {mod};

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [&'static ComposerEntry] {{
    ENTRIES.get_or_init(|| {mod}::entries().iter().collect()).as_slice()
}}

pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {{
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError {{ message: format!("{Name}Composer: no entry writes {{:?}}", target), diagnostics: Vec::new() }})?;
    (entry.compose)(sources)
}}

pub fn register() {{
    register_composer_entries({mod}::entries());
}}
"""

TS_META = """/** {emoji} {Name}{facet} ({level}) meta. */
export const meta = {{
  artifactKind: "s.stdio.{kind}",
  standard: "{slug}",
  subset: "*",
}} as const;
"""


def write_facades(art_root, std_root, subset_root, std_dir, mod, Name, kind, slug):
    art_path = f"crate::artifacts::{kind}"
    std_path = f"{art_path}::standards::{mod}"
    subset_path = f"{std_path}::subsets::any"

    # Builder facades
    open(os.path.join(std_root, "🏗️builder", "🦀️component.rs"), "w", encoding="utf-8").write(
        BUILDER_FACADE.format(Name=Name, kind=kind, level=f"{slug} standard", target=f"its ✳️any subset",
                               target_path=f"{subset_path}::builder", SourceAlias=f"{Name}RawAnyBuilder")
    )
    open(os.path.join(art_root, "🏗️builder", "🦀️component.rs"), "w", encoding="utf-8").write(
        BUILDER_FACADE.format(Name=Name, kind=kind, level="final, artifact-level", target=f"the {slug} standard",
                               target_path=f"{std_path}::builder", SourceAlias=f"{Name}RawBuilder")
    )
    # Analyzer facades
    open(os.path.join(std_root, "🧐️analyzer", "🦀️component.rs"), "w", encoding="utf-8").write(
        ANALYZER_FACADE.format(Name=Name, kind=kind, slug=slug, level=f"{slug} standard", target="its ✳️any subset",
                                target_path=f"{subset_path}::analyzer", SourceAlias=f"{Name}RawAnyAnalyzer")
    )
    open(os.path.join(art_root, "🧐️analyzer", "🦀️component.rs"), "w", encoding="utf-8").write(
        ANALYZER_FACADE.format(Name=Name, kind=kind, slug=slug, level="final, artifact-level", target=f"the {slug} standard",
                                target_path=f"{std_path}::analyzer", SourceAlias=f"{Name}RawAnalyzer")
    )

    # Composers: reads() includes own dialect + each direct dependency's final dialect.
    deps = [d for d in DEPENDS.get(kind, [])]
    reads_list_parts = ["DIALECT"]
    reads_filter_parts = ["s.dialect == DIALECT"]
    reads_consts = ""
    for dep in deps:
        dep_kind = dep
        dep_const = f"DEP_{dep_kind.upper()}"
        reads_consts += (
            f'const {dep_const}: Dialect = Dialect {{ artifact_kind: "s.stdio.{dep_kind}", '
            f'standard: StandardId("{STANDARDS.get(dep_kind, {"slug": "1"})["slug"] if dep_kind in STANDARDS else "1"}"), subset: SubsetId("*") }};\n'
        )
        reads_list_parts.append(dep_const)
        reads_filter_parts.append(f"s.dialect == {dep_const}")
    reads_list = ", ".join(reads_list_parts)
    reads_filter = " || ".join(reads_filter_parts)
    extra_reads_doc = f" (plus its DAG dependencies: {', '.join(deps)})" if deps else ""

    open(os.path.join(subset_root, "🎹️composer", "🦀️component.rs"), "w", encoding="utf-8").write(
        SUBSET_COMPOSER.format(Name=Name, kind=kind, slug=slug, mod=mod, reads_list=reads_list,
                                reads_filter=reads_filter, reads_consts=reads_consts, extra_reads_doc=extra_reads_doc)
    )
    open(os.path.join(std_root, "🎹️composer", "🦀️component.rs"), "w", encoding="utf-8").write(
        STANDARD_COMPOSER.format(Name=Name, kind=kind, slug=slug, mod=mod)
    )
    open(os.path.join(art_root, "🎹️composer", "🦀️component.rs"), "w", encoding="utf-8").write(
        ARTIFACT_COMPOSER.format(Name=Name, kind=kind, mod=mod)
    )

    # TS twins
    for path, facet, emoji, level in [
        (os.path.join(std_root, "🏗️builder", "🟦️component.ts"), "Builder", "🏗️", f"{slug} standard"),
        (os.path.join(art_root, "🏗️builder", "🟦️component.ts"), "Builder", "🏗️", "final"),
        (os.path.join(subset_root, "🏗️builder", "🟦️component.ts"), "Builder", "🏗️", "✳️any subset"),
        (os.path.join(std_root, "🧐️analyzer", "🟦️component.ts"), "Analyzer", "🧐️", f"{slug} standard"),
        (os.path.join(art_root, "🧐️analyzer", "🟦️component.ts"), "Analyzer", "🧐️", "final"),
        (os.path.join(subset_root, "🧐️analyzer", "🟦️component.ts"), "Analyzer", "🧐️", "✳️any subset"),
        (os.path.join(std_root, "🎹️composer", "🟦️component.ts"), "Composer", "🎹️", f"{slug} standard"),
        (os.path.join(art_root, "🎹️composer", "🟦️component.ts"), "Composer", "🎹️", "final"),
        (os.path.join(subset_root, "🎹️composer", "🟦️component.ts"), "Composer", "🎹️", "✳️any subset"),
    ]:
        open(path, "w", encoding="utf-8").write(TS_META.format(Name=Name, kind=kind, slug=slug, facet=facet, emoji=emoji, level=level))

    # Engine register() rewire: io::register() -> composer::register()
    engine_rs = os.path.join(std_root, "⚙️engine", "🦀️component.rs")
    if os.path.exists(engine_rs):
        text = open(engine_rs, encoding="utf-8").read()
        old_call = f"crate::artifacts::{kind}::io::register();"
        new_call = f"crate::artifacts::{kind}::composer::register();"
        if old_call in text:
            text = text.replace(old_call, new_call)
            open(engine_rs, "w", encoding="utf-8").write(text)

    # io facet root: replace register() chain with a doc-only placeholder (no-op, matches binary).
    io_root_rs = os.path.join(subset_root, "🚪️io", "🦀️component.rs")
    if os.path.exists(io_root_rs):
        open(io_root_rs, "w", encoding="utf-8").write(
            f"//! 🚪️ IO stdio.{kind} ({slug}/✳️any) — registration now flows through 🎹️composer::register\n"
            f"//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().\n"
        )


if __name__ == "__main__":
    for d in sys.argv[1:]:
        migrate(d)
