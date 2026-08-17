#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""W14: migrate one domain artifact to the standards/subsets tree (standard 🔖️1, subset ✳️any
uniformly). Generalizes the proven stdio recipe; the composer differs because domain artifacts
convert real cross-format payloads (their io leaves already contain real conversion logic, just
untyped) rather than reusing a single-hop own-codec trick.

Usage: python3 w14_migrate_domain_artifact.py <plugin_dir> <artifact_dir>
"""
import json
import os
import re
import subprocess
import sys
import shutil

REPO = "/Users/ueli/Documents/semio"
PLUGINS = os.path.join(REPO, "✏️s/🔌️plugins")
# 🚨️ Same pinned-commit rationale as w14_rewrite_glue.py: auto-commit can snapshot this session's
# own in-progress (sometimes still-buggy) edits, so `HEAD`/on-disk state is not a safe source for
# "the true pre-migration original" once a file has already been touched once this session.
PRE_SESSION_COMMIT = "678a50d6c5"


def original_file_at_pre_session(rel_path):
    """Best-effort: the file's content at the pinned pre-session commit, or None if it didn't
    exist there (e.g. a path that only exists post-migration)."""
    try:
        return subprocess.run(
            ["git", "show", f"{PRE_SESSION_COMMIT}:{rel_path}"],
            cwd=REPO, capture_output=True, text=True, check=True,
        ).stdout
    except subprocess.CalledProcessError:
        return None
HERE = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(HERE, "w9_standards_table.json"), encoding="utf-8") as f:
    STANDARDS = json.load(f)["stdio"]
with open(os.path.join(HERE, "w9_owner_table_v2.json"), encoding="utf-8") as f:
    OWNER_V2 = json.load(f)
KIND_TO_DIR = {k: v["dir"] for k, v in OWNER_V2["stdio_roster"].items()}
DIR_TO_KIND = {v: k for k, v in KIND_TO_DIR.items()}

STD_DIR = "🔖️1"
STD_MOD = "v1"


def fix_include_depth(moved_root, extra_levels=None):
    """Relative include_str!/include_bytes! paths that reach OUTSIDE their own moved subtree (e.g.
    `../../../📚️examples/...` from a schema/snapshot/text file reaching back to the artifact's
    examples dir) need more `../` prepended -- different facets (schema vs engine) moved different
    numbers of levels deeper, so try a range (1..6) rather than one fixed guess. Self-correcting
    (checks the path actually resolves on disk) rather than gated on "did a move just happen", so
    it's safe to call on every run regardless of mv() idempotency no-ops."""
    tries = [extra_levels] if extra_levels else list(range(1, 7))
    pattern = re.compile(r'(include_(?:str|bytes)!\(")(\.\./[^"]*)(")')
    for dirpath, _dirs, files in os.walk(moved_root):
        for fn in files:
            if not fn.endswith(".rs"):
                continue
            fp = os.path.join(dirpath, fn)
            text = open(fp, encoding="utf-8").read()

            def repl(m):
                rel = m.group(2)
                if os.path.exists(os.path.normpath(os.path.join(dirpath, rel))):
                    return m.group(0)  # already resolves correctly, leave alone
                for n in tries:
                    fixed = ("../" * n) + rel
                    if os.path.exists(os.path.normpath(os.path.join(dirpath, fixed))):
                        return m.group(1) + fixed + m.group(3)
                return m.group(0)  # neither resolves -- leave alone, not this bug

            new_text = pattern.sub(repl, text)
            if new_text != text:
                open(fp, "w", encoding="utf-8").write(new_text)


SKIP_FN_NAMES = {"register", "import_stdio_kinds", "export_stdio_kinds"}


def preserve_extra_fns(text):
    """Extract every top-level `pub fn <name>(...) { ... }` in `text` except the ones this script
    itself regenerates, and return them concatenated verbatim (brace-matched, so multi-line bodies
    survive intact). Self-stable across re-runs: re-extracting from already-migrated content just
    re-copies the same preserved functions forward unchanged."""
    out = []
    for m in re.finditer(r"pub fn (\w+)\s*[<(]", text):
        name = m.group(1)
        if name in SKIP_FN_NAMES:
            continue
        brace = text.find("{", m.start())
        if brace == -1:
            continue
        depth = 0
        end = None
        for idx in range(brace, len(text)):
            if text[idx] == "{":
                depth += 1
            elif text[idx] == "}":
                depth -= 1
                if depth == 0:
                    end = idx + 1
                    break
        if end is None:
            continue
        out.append(text[m.start():end] + "\n")
    return "".join(out)


def artifact_name_from_root(art_root):
    """The PascalCase type prefix (e.g. 'Note', 'Writer'). Root component.rs re-export ordering
    varies across plugins (`schema::snapshot::XSnapshot` vs a legacy-shim `snapshot::schema::
    XSnapshot`), so read the struct declaration straight from the schema snapshot file instead --
    the one place it's guaranteed present, regardless of how it gets re-exported upward."""
    snap_rs = os.path.join(art_root, "🧬️schema", "📸️snapshot", "🦀️component.rs")
    if os.path.exists(snap_rs):
        text = open(snap_rs, encoding="utf-8").read()
        m = re.search(r"pub struct (\w+)Snapshot\b", text)
        if m:
            return m.group(1)
    root_rs = os.path.join(art_root, "🦀️component.rs")
    text = open(root_rs, encoding="utf-8").read()
    m = re.search(r"(\w+)Snapshot\b", text)
    if not m:
        raise SystemExit(f"could not find <Name>Snapshot pattern for {art_root}")
    return m.group(1)


def mutation_type_name(art_root, Name):
    """The Mutation enum's REAL Rust type name -- do NOT assume it's always `{Name}Mutation`.
    Confirmed false for vcs: the real type is `VcsDemoMutation`, and hardcoding `{Name}Mutation`
    in BUILDER_FACADE produced an unresolved-import (E0432) that only surfaced once something
    finally tried to import `crate::artifacts::vcs::VcsMutation` (see cad/vcs incidents, this
    ticket's STATUS.md). Find the real name by reading which type implements
    `Mutation<{Name}Snapshot>` in the mutations component.rs -- that trait impl is the one place
    the artifact's own code commits to a specific type, regardless of naming convention. Falls
    back to the `{Name}Mutation` convention if the impl isn't found (keeps existing behavior for
    every artifact that already follows it -- most do)."""
    mut_rs = os.path.join(art_root, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🧬️schema", "🧬️mutations", "🦀️component.rs")
    if not os.path.isfile(mut_rs):
        mut_rs = os.path.join(art_root, "🧬️schema", "🧬️mutations", "🦀️component.rs")
    if os.path.isfile(mut_rs):
        text = open(mut_rs, encoding="utf-8").read()
        m = re.search(r"impl (?:\w+::)*Mutation<" + re.escape(Name) + r"Snapshot>\s+for\s+(\w+)", text)
        if m:
            return m.group(1)
    return f"{Name}Mutation"


def existing_struct_name(component_rs_path, suffix, fallback):
    """Read the REAL `pub struct X{suffix}` name from an already-existing (hand-authored,
    mv()-preserved) component.rs, instead of assuming it follows the `{Name}{suffix}` convention.
    Confirmed false for space/home: schema types are `SHomeSnapshot`/`SHomeDiff`/`SHomeMutation`
    (Name="SHome") but the pre-existing subset-level builder struct is `HomeBuilder` (no S-prefix)
    -- a plugin-internal naming inconsistency between the schema facet and the builder/analyzer
    facet. Returns `fallback` (the `{Name}{suffix}` convention) if the file doesn't exist yet or
    the pattern isn't found, which keeps existing behavior for every artifact where the two
    conventions already agree (most do)."""
    if not os.path.isfile(component_rs_path):
        return fallback
    text = open(component_rs_path, encoding="utf-8").read()
    m = re.search(r"pub struct (\w+)" + re.escape(suffix) + r"\b", text)
    return f"{m.group(1)}{suffix}" if m else fallback


def find_kind_module(plugin_dir, art_dir):
    """The Rust module name (crate::artifacts::<kind>) -- read from the plugin glue.rs mapping of
    artifact dir -> mod name, by grepping the #[path] leaf for this artifact's root component.rs."""
    glue = glue_path(plugin_dir)
    text = open(glue, encoding="utf-8").read()
    pat = re.compile(r'pub mod (\w+) \{\s*#\[path = "\.\./\.\./🗿️artifacts/' + re.escape(art_dir) + r'/🦀️component\.rs"\]')
    m = pat.search(text)
    if not m:
        raise SystemExit(f"could not find module name for {art_dir} in {glue}")
    return m.group(1)


def glue_path(plugin_dir):
    return os.path.join(PLUGINS, plugin_dir, "📦️packages/🦀️rust/📦️glue.rs")


def stdio_targets_of(art_root):
    """Scan disk for the (import_targets, export_targets, union) stdio kinds this artifact's io
    facet references."""
    import_targets, export_targets = set(), set()
    for direction, child, bucket in (
        ("📥️import", "🧩️deserializers", import_targets),
        ("📤️export", "🧵️serializers", export_targets),
    ):
        base = os.path.join(art_root, "🚪️io", direction, child, "🗿️artifacts")
        if os.path.isdir(base):
            for d in os.listdir(base):
                if os.path.isdir(os.path.join(base, d)) and d in DIR_TO_KIND:
                    bucket.add(DIR_TO_KIND[d])
    return sorted(import_targets), sorted(export_targets), sorted(import_targets | export_targets)


def migrate(plugin_dir, art_dir):
    art_root = os.path.join(PLUGINS, plugin_dir, "🗿️artifacts", art_dir)
    kind = find_kind_module(plugin_dir, art_dir)
    Name = artifact_name_from_root(art_root)
    std_root = os.path.join(art_root, "🏅️standards", STD_DIR)
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

    schema_moved = mv("🧬️schema", "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")
    mv("⚙️engine", "🏅️standards/🔖️1/⚙️engine")
    mv("🚪️io", "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io")
    had_io = os.path.isdir(os.path.join(subset_root, "🚪️io"))

    import_targets, export_targets, targets = stdio_targets_of(subset_root) if had_io else stdio_targets_of(art_root)

    # Insert target std/subset into every io leaf's target path.
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

    for d in (subset_root, std_root, art_root):
        os.makedirs(os.path.join(d, "🏗️builder"), exist_ok=True)
        os.makedirs(os.path.join(d, "🧐️analyzer"), exist_ok=True)
        os.makedirs(os.path.join(d, "🎹️composer"), exist_ok=True)

    subset_builder_rs = os.path.join(subset_root, "🏗️builder", "🦀️component.rs")
    if not os.path.exists(subset_builder_rs):
        for ext in ("🦀️component.rs", "🟦️component.ts"):
            mv(f"🏗️builder/{ext}", f"🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/{ext}")

    decomposer_dir = os.path.join(art_root, "🪓️decomposer")
    analyzer_rs_new = os.path.join(subset_root, "🧐️analyzer", "🦀️component.rs")
    os.makedirs(os.path.dirname(analyzer_rs_new), exist_ok=True)
    open(analyzer_rs_new, "w", encoding="utf-8").write(
        SUBSET_ANALYZER.format(Name=Name, kind=kind)
    )
    if os.path.isdir(decomposer_dir):
        shutil.rmtree(decomposer_dir, ignore_errors=True)

    fix_include_depth(std_root)
    write_facades(art_root, std_root, subset_root, kind, Name, targets, import_targets, export_targets)
    ensure_root_reexports(art_root, kind, Name, mutation_type_name(art_root, Name))
    print(f"OK  {plugin_dir}/{art_dir:12s} kind={kind:12s} Name={Name}  targets={','.join(targets)}")


def ensure_root_reexports(art_root, kind, Name, MutationName):
    """The artifact-root 🦀️component.rs is hand-authored and left untouched by this script, but
    both the final artifact-level AND standard-level 🏗️builder facades (written by write_facades)
    require `crate::artifacts::{kind}::{{Name}}Diff` and `crate::artifacts::{kind}::{MutationName}`
    to resolve as bare siblings there -- confirmed missing for BOTH cad and vcs (only
    `{Name}Snapshot` was re-exported originally; Diff/Mutation were never needed pre-migration
    because the builder lived directly beside the schema, not three levels away through a facade
    chain). Idempotent: only appends what's missing, right after the existing Snapshot
    re-export line if found, using the shim modules (always pure aliases, see
    w14_rewrite_glue.py's CORE_TRIAD_FACETS handling) rather than any specific convention -- those
    resolve regardless of which pre-existing convention (`schema::snapshot::X` vs
    `snapshot::schema::X`) the plugin's Snapshot line already uses."""
    root_rs = os.path.join(art_root, "🦀️component.rs")
    text = open(root_rs, encoding="utf-8").read()
    has_diff = re.search(r"\b" + re.escape(Name) + r"Diff\b", text) is not None
    has_mutation = re.search(r"\b" + re.escape(MutationName) + r"\b", text) is not None
    if has_diff and has_mutation:
        return
    new_lines = []
    if not has_diff:
        new_lines.append(f"pub use crate::artifacts::{kind}::diff::{Name}Diff;")
    if not has_mutation:
        new_lines.append(f"pub use crate::artifacts::{kind}::mutations::{MutationName};")
    snap_match = re.search(r"^pub use crate::artifacts::" + re.escape(kind) + r"::[\w:]*" + re.escape(Name) + r"Snapshot;\s*$", text, re.MULTILINE)
    insertion = "\n".join(new_lines) + "\n"
    if snap_match:
        insert_at = snap_match.end()
        text = text[:insert_at] + "\n" + insertion.rstrip("\n") + text[insert_at:]
    else:
        # No Snapshot re-export line found to anchor on (unusual) -- prepend after the module doc
        # comment block instead so it's still near the top, not buried mid-file.
        text = insertion + text
    open(root_rs, "w", encoding="utf-8").write(text)


SUBSET_ANALYZER = """//! 🧐️ {Name}Analyzer (1/✳️any) — read-only analysis, successor to the pre-migration
//! {Name}Decomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource}};
use crate::artifacts::{kind}::{Name}Snapshot;

#[derive(Clone, Debug, Default)]
pub struct {Name}Parts {{
    pub snapshot: Option<{Name}Snapshot>,
}}

pub struct {Name}Analyzer;

impl ArtifactAnalyzer for {Name}Analyzer {{
    type Parts = {Name}Parts;
    const DIALECT: Dialect = Dialect {{ artifact_kind: "s.{kind}", standard: StandardId("1"), subset: SubsetId("*") }};

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
                        diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }}
                }},
                AnalyzeSource::Binary(bytes) => match <{Name}Snapshot as store::DocumentPack>::decode_pack(bytes) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }}
                }},
            }}
        }}
        Analysis {{ parts, dialect: Self::DIALECT, confidence, diagnostics }}
    }}
}}
"""

BUILDER_FACADE = """//! 🏗️ {Name}Builder ({level}) — delegates to {target}.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::{kind}::{{{Name}Diff, {MutationName}, {Name}Snapshot}};
use {target_path}::{SourceStructName} as {SourceAlias};

#[derive(Clone, Debug)]
pub struct {Name}Builder({SourceAlias});

impl ArtifactBuilder for {Name}Builder {{
    type Snapshot = {Name}Snapshot;
    type Mutation = {MutationName};
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
use {target_path}::{SourceStructName} as {SourceAlias};
pub use {target_path}::{SourcePartsName};

const DIALECT: Dialect = Dialect {{ artifact_kind: "s.{kind}", standard: StandardId("1"), subset: SubsetId("*") }};

pub struct {Name}Analyzer;

impl ArtifactAnalyzer for {Name}Analyzer {{
    type Parts = {Name}Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {{ {SourceAlias}::sniff(source) }}
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {{ {SourceAlias}::analyze(sources) }}
}}
"""

# Domain subset composer: native dialect uses the artifact's own analyzer; each stdio target
# dialect decodes via that stdio kind's own FINAL builder facade (uniform across all 29 migrated
# stdio artifacts), then calls the existing (just-wrapped) deserializer leaf function.
SUBSET_COMPOSER_HEAD = """//! 🎹️ {Name}Composer (1/✳️any) — analyzer + builder glued. Reads native `s.{kind}` sources
//! plus any of: {targets_doc}. Writes one `s.{kind}` (1/✳️any) snapshot.

use semio_framework_plugin::{{ArtifactComposer, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource}};
use crate::artifacts::{kind}::{Name}Snapshot;
use crate::artifacts::{kind}::standards::v1::subsets::any::analyzer::{Name}Analyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect {{ artifact_kind: "s.{kind}", standard: StandardId("1"), subset: SubsetId("*") }};
{dep_consts}

pub struct {Name}Composer;

impl ArtifactComposer for {Name}Composer {{
    type Snapshot = {Name}Snapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {{
        &[DIALECT{dep_list}]
    }}

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {{
        for source in sources {{
            if source.dialect == DIALECT {{
                let native = match &source.payload {{
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                }};
                let analysis = {Name}Analyzer::analyze(&[native]);
                if let Some(snapshot) = analysis.parts.snapshot {{
                    return Ok(Composition {{ snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics }});
                }}
            }}
{dep_arms}
        }}
        Err(ComposeError {{ message: "{Name}Composer: no source in a known read dialect".into(), diagnostics: Vec::new() }})
    }}
}}
"""


# Every io leaf's deserializer facet, regardless of its typed `deserialize(&Typed)` fn (present on
# some but not all leaves), always ALSO defines `deserialize_bytes(&[u8])` -- the uniform entry
# point every leaf provides. Composing from a dependency dialect calls that directly; no need to
# round-trip through the dependency's own typed stdio snapshot first.
DEP_ARM_BYTES = """            if source.dialect == DEP_{KIND_UPPER} {{
                let bytes: Vec<u8> = match &source.payload {{
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                }};
                if let Ok(snapshot) = crate::artifacts::{art_kind}::io::import::deserializers::artifacts::{kind}::{tmod}::any::deserialize_bytes(&bytes) {{
                    return Ok(Composition {{ snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() }});
                }}
            }}
"""

# Fallback for leaves whose convention is `deserialize_text(&str)` (no `deserialize_bytes`) --
# binary payloads are decoded as UTF-8 first, matching what `deserialize_bytes` would do anyway
# in the plugins that DO have it (see writer/layout: real conversion logic keyed on text).
DEP_ARM_TEXT = """            if source.dialect == DEP_{KIND_UPPER} {{
                let text: Option<String> = match &source.payload {{
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                }};
                if let Some(text) = text {{
                    if let Ok(snapshot) = crate::artifacts::{art_kind}::io::import::deserializers::artifacts::{kind}::{tmod}::any::deserialize_text(&text) {{
                        return Ok(Composition {{ snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() }});
                    }}
                }}
            }}
"""


def deserializer_call_style(art_root, target_kind, tstd_dir):
    """Inspect the actual leaf file -- conventions vary: some have `deserialize_bytes(&[u8])`,
    others only `deserialize_text(&str)`, others only a typed `deserialize(&Typed)` with neither
    convenience wrapper. Prefer bytes, then text; "skip" means don't wire this dependency into the
    composer's dispatch at all (documented simplification) rather than generate a call to a
    function that doesn't exist."""
    leaf = os.path.join(
        art_root, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🚪️io",
        "📥️import", "🧩️deserializers", "🗿️artifacts", KIND_TO_DIR[target_kind], tstd_dir, "✳️any", "🦀️component.rs",
    )
    if not os.path.exists(leaf):
        return "skip"
    text = open(leaf, encoding="utf-8").read()
    if re.search(r"pub fn deserialize_bytes\s*\(", text):
        return "bytes"
    if re.search(r"pub fn deserialize_text\s*\(", text):
        return "text"
    return "skip"

STANDARD_COMPOSER = """//! 🎹️ {Name}Composer (1 standard) — aggregates its subsets' composer entries value-level.

use std::sync::OnceLock;
use semio_framework_plugin::{{ComposerEntry, composer_entry_of}};
use crate::artifacts::{kind}::standards::v1::subsets::any::composer::{Name}Composer as {Name}AnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {{
    ENTRIES.get_or_init(|| vec![composer_entry_of::<{Name}AnyComposer>()]).as_slice()
}}
"""

ARTIFACT_COMPOSER = """//! 🎹️ {Name}Composer (final, artifact-level) — union over every standard's composer entries.

use std::sync::OnceLock;
use semio_framework_plugin::{{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries}};
use crate::artifacts::{kind}::standards::v1::composer as v1;

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [&'static ComposerEntry] {{
    ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
}}

pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {{
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError {{ message: format!("{Name}Composer: no entry writes {{:?}}", target), diagnostics: Vec::new() }})?;
    (entry.compose)(sources)
}}

pub fn register() {{
    register_composer_entries(v1::entries());
}}
"""

TS_META = """/** {emoji} {Name}{facet} ({level}) meta. */
export const meta = {{
  artifactKind: "s.{kind}",
  standard: "1",
  subset: "*",
}} as const;
"""


def write_facades(art_root, std_root, subset_root, kind, Name, targets, import_targets, export_targets):
    art_path = f"crate::artifacts::{kind}"
    std_path = f"{art_path}::standards::v1"
    subset_path = f"{std_path}::subsets::any"
    MutationName = mutation_type_name(art_root, Name)
    # Subset-level builder is hand-authored/preserved (mv() only, never regenerated by this
    # script) -- its struct name can diverge from the `{Name}Builder` convention (see space/home).
    # Subset-level analyzer IS regenerated below by SUBSET_ANALYZER, always as `{Name}Analyzer`/
    # `{Name}Parts` -- no divergence possible there, so no lookup needed for it.
    subset_builder_source = existing_struct_name(
        os.path.join(subset_root, "🏗️builder", "🦀️component.rs"), "Builder", f"{Name}Builder")

    open(os.path.join(std_root, "🏗️builder", "🦀️component.rs"), "w", encoding="utf-8").write(
        BUILDER_FACADE.format(Name=Name, MutationName=MutationName, kind=kind, level="1 standard", target="its ✳️any subset",
                               target_path=f"{subset_path}::builder", SourceStructName=subset_builder_source, SourceAlias=f"{Name}AnyBuilder")
    )
    open(os.path.join(art_root, "🏗️builder", "🦀️component.rs"), "w", encoding="utf-8").write(
        BUILDER_FACADE.format(Name=Name, MutationName=MutationName, kind=kind, level="final, artifact-level", target="the 1 standard",
                               target_path=f"{std_path}::builder", SourceStructName=f"{Name}Builder", SourceAlias=f"{Name}RawBuilder")
    )
    open(os.path.join(std_root, "🧐️analyzer", "🦀️component.rs"), "w", encoding="utf-8").write(
        ANALYZER_FACADE.format(Name=Name, kind=kind, level="1 standard", target="its ✳️any subset",
                                target_path=f"{subset_path}::analyzer", SourceStructName=f"{Name}Analyzer", SourcePartsName=f"{Name}Parts", SourceAlias=f"{Name}AnyAnalyzer")
    )
    open(os.path.join(art_root, "🧐️analyzer", "🦀️component.rs"), "w", encoding="utf-8").write(
        ANALYZER_FACADE.format(Name=Name, kind=kind, level="final, artifact-level", target="the 1 standard",
                                target_path=f"{std_path}::analyzer", SourceStructName=f"{Name}Analyzer", SourcePartsName=f"{Name}Parts", SourceAlias=f"{Name}RawAnalyzer")
    )

    dep_consts = ""
    dep_list = ""
    dep_arms = ""
    skipped_targets = []
    for t in targets:
        std_name = STANDARDS[t]  # {slug, dir, rust_mod}
        style = deserializer_call_style(art_root, t, std_name["dir"])
        if style == "skip":
            skipped_targets.append(t)
            continue
        upper = t.upper()
        dep_consts += (
            f'const DEP_{upper}: Dialect = Dialect {{ artifact_kind: "s.stdio.{t}", '
            f'standard: StandardId("{std_name["slug"]}"), subset: SubsetId("*") }};\n'
        )
        dep_list += f", DEP_{upper}"
        std_name_pascal = t[0].upper() + t[1:]
        template = DEP_ARM_BYTES if style == "bytes" else DEP_ARM_TEXT
        dep_arms += template.format(KIND_UPPER=upper, kind=t, StdName=std_name_pascal, art_kind=kind, tmod=std_name["rust_mod"])
    if skipped_targets:
        print(f"    (compose() skips {','.join(skipped_targets)}: leaf has neither deserialize_bytes nor deserialize_text)")

    targets_doc = ", ".join(f"stdio.{t}" for t in targets) if targets else "(none)"
    open(os.path.join(subset_root, "🎹️composer", "🦀️component.rs"), "w", encoding="utf-8").write(
        SUBSET_COMPOSER_HEAD.format(Name=Name, kind=kind, dep_consts=dep_consts, dep_list=dep_list,
                                     dep_arms=dep_arms, targets_doc=targets_doc)
    )
    open(os.path.join(std_root, "🎹️composer", "🦀️component.rs"), "w", encoding="utf-8").write(
        STANDARD_COMPOSER.format(Name=Name, kind=kind)
    )
    open(os.path.join(art_root, "🎹️composer", "🦀️component.rs"), "w", encoding="utf-8").write(
        ARTIFACT_COMPOSER.format(Name=Name, kind=kind)
    )

    for path, level in [
        (os.path.join(std_root, "🏗️builder", "🟦️component.ts"), "1 standard"),
        (os.path.join(art_root, "🏗️builder", "🟦️component.ts"), "final"),
        (os.path.join(subset_root, "🏗️builder", "🟦️component.ts"), "✳️any subset"),
        (os.path.join(std_root, "🧐️analyzer", "🟦️component.ts"), "1 standard"),
        (os.path.join(art_root, "🧐️analyzer", "🟦️component.ts"), "final"),
        (os.path.join(subset_root, "🧐️analyzer", "🟦️component.ts"), "✳️any subset"),
        (os.path.join(std_root, "🎹️composer", "🟦️component.ts"), "1 standard"),
        (os.path.join(art_root, "🎹️composer", "🟦️component.ts"), "final"),
        (os.path.join(subset_root, "🎹️composer", "🟦️component.ts"), "✳️any subset"),
    ]:
        facet = "Builder" if "🏗️builder" in path else ("Analyzer" if "🧐️analyzer" in path else "Composer")
        emoji = {"Builder": "🏗️", "Analyzer": "🧐️", "Composer": "🎹️"}[facet]
        open(path, "w", encoding="utf-8").write(TS_META.format(Name=Name, kind=kind, facet=facet, emoji=emoji, level=level))

    engine_rs = os.path.join(std_root, "⚙️engine", "🦀️component.rs")
    if os.path.exists(engine_rs):
        text = open(engine_rs, encoding="utf-8").read()
        old_call = f"crate::artifacts::{kind}::io::register();"
        new_call = f"crate::artifacts::{kind}::composer::register();"
        if old_call in text:
            open(engine_rs, "w", encoding="utf-8").write(text.replace(old_call, new_call))

    io_root_rs = os.path.join(subset_root, "🚪️io", "🦀️component.rs")
    if os.path.exists(io_root_rs):
        import_list = ", ".join(f'"stdio.{t}"' for t in import_targets)
        export_list = ", ".join(f'"stdio.{t}"' for t in export_targets)
        # The io facet root can carry REAL helper functions beyond register()/*_stdio_kinds()
        # (e.g. dag's dag_to_wire/dag_from_wire, used by its own io leaves) -- preserve any such
        # function verbatim. Prefer the pinned pre-session original (the on-disk file may already
        # be a placeholder from an earlier run of this same script, before this preservation
        # logic existed -- reading THAT would silently preserve nothing).
        original_io_rel = os.path.relpath(art_root, REPO) + "/🚪️io/🦀️component.rs"
        source_text = original_file_at_pre_session(original_io_rel) or open(io_root_rs, encoding="utf-8").read()
        preserved = preserve_extra_fns(source_text)
        open(io_root_rs, "w", encoding="utf-8").write(
            f"//! 🚪️ IO s.{kind} (1/✳️any) — registration now flows through 🎹️composer::register\n"
            f"//! (called once from ⚙️engine::register), not per-leaf register().\n"
            f'pub fn import_stdio_kinds() -> &\'static [&\'static str] {{ &[{import_list}] }}\n'
            f'pub fn export_stdio_kinds() -> &\'static [&\'static str] {{ &[{export_list}] }}\n'
            + preserved
        )


if __name__ == "__main__":
    migrate(sys.argv[1], sys.argv[2])
