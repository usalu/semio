#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""W15: add EXPORT-direction ComposerEntry rows to one artifact's standard-level composer.
Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO. The typed registry (W11-W14) only ever grew IMPORT
entries (each composer's own reads()) -- nothing registered "this artifact can be exported AS
format Y", because ArtifactComposer only models "produce my own snapshot". This wraps each
artifact's EXISTING 🚪️io/📤️export/🧵️serializers leaves (which already convert this artifact's
snapshot straight to target-format bytes/text) as their own ComposerEntry rows.

Pattern hand-validated + tested on 🗒️note/json (65->68 tests, all passing, genuine round-trip
through the typed registry) before this script existed -- see that file's own #[cfg(test)] mod
for the reference shape this script reproduces mechanically.

Usage: python3 w15_add_export_entries.py <plugin_dir> <artifact_dir> [<artifact_dir> ...]
Idempotent: re-running replaces the whole 🔖️ExportEntries region + entries() body rather than
duplicating it.
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
PLUGINS = os.path.join(REPO, "✏️s/🔌️plugins")
HERE = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(HERE, "w9_standards_table.json"), encoding="utf-8") as f:
    STANDARDS = json.load(f)["stdio"]
with open(os.path.join(HERE, "w9_owner_table_v2.json"), encoding="utf-8") as f:
    OWNER_V2 = json.load(f)
KIND_TO_DIR = {k: v["dir"] for k, v in OWNER_V2["stdio_roster"].items()}
DIR_TO_KIND = {v: k for k, v in KIND_TO_DIR.items()}

STD_DIR = "🔖️1"
STD_MOD = "v1"


def rust_ident_from_slug_dir(dir_name):
    m = re.match(r"^\W*([a-z0-9][a-z0-9-]*)$", dir_name)
    if not m:
        stripped = re.sub(r"^[^a-z0-9]+", "", dir_name)
        m = re.match(r"^([a-z0-9][a-z0-9-]*)$", stripped)
    if not m:
        raise SystemExit(f"could not derive rust ident from dir {dir_name!r}")
    return m.group(1).replace("-", "_")


def existing_struct_name(component_rs_path, suffix, fallback):
    """Read the REAL `pub struct X{suffix}` name off disk instead of assuming `{Name}{suffix}` --
    confirmed false for gis (GisterrainBuilder/GismapBuilder, not GisTerrainBuilder/GisMapBuilder)
    and previously for space/home (HomeBuilder vs SHomeBuilder, see w14_migrate_domain_artifact.py's
    own identically-named helper, which this mirrors)."""
    if not os.path.isfile(component_rs_path):
        return fallback
    text = open(component_rs_path, encoding="utf-8").read()
    m = re.search(r"pub struct (\w+)" + re.escape(suffix) + r"\b", text)
    return f"{m.group(1)}{suffix}" if m else fallback


def find_kind_module(plugin_dir, art_dir):
    glue = os.path.join(PLUGINS, plugin_dir, "📦️packages/🦀️rust/📦️glue.rs")
    text = open(glue, encoding="utf-8").read()
    pat = re.compile(r'pub mod (\w+) \{\s*#\[path = "\.\./\.\./🗿️artifacts/' + re.escape(art_dir) + r'/🦀️component\.rs"\]')
    m = pat.search(text)
    if not m:
        raise SystemExit(f"could not find module name for {art_dir} in {glue}")
    return m.group(1)


def artifact_name_from_root(art_root):
    for candidate in (
        os.path.join(art_root, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🧬️schema", "📸️snapshot", "🦀️component.rs"),
        os.path.join(art_root, "🦀️component.rs"),
    ):
        if os.path.isfile(candidate):
            text = open(candidate, encoding="utf-8").read()
            m = re.search(r"(\w+)Snapshot\b", text)
            if m:
                return m.group(1)
    raise SystemExit(f"could not find <Name>Snapshot pattern for {art_root}")


def detect_json_import_call(art_root):
    """The OS dispatch layer (export_os_app_instance_media_kind) deals in already-deserialized
    `serde_json::Value`, not artifact-native wire text/binary -- so rebuild_native_snapshot needs
    a JSON-dialect fallback path alongside the native-dialect one. Every domain artifact declares
    a json import target (confirmed: 54/54), so json is the universal bridge. Returns
    (fn_name, payload_kind) matching detect_export_call's shape, or None if genuinely absent."""
    base = os.path.join(art_root, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🚪️io", "📥️import", "🧩️deserializers", "🗿️artifacts", "🔣️json")
    if not os.path.isdir(base):
        return None
    tstd_dir = STANDARDS["json"]["dir"]
    leaf_path = os.path.join(base, tstd_dir, "✳️any", "🦀️component.rs")
    if not os.path.isfile(leaf_path):
        return None
    text = open(leaf_path, encoding="utf-8").read()
    for name in ("deserialize_bytes",):
        if re.search(r"pub fn " + name + r"\s*\(", text):
            return (name, "bytes")
    for name in ("deserialize_text",):
        if re.search(r"pub fn " + name + r"\s*\(", text):
            return (name, "text")
    return None


def detect_export_call(leaf_path):
    """Returns (fn_name, payload_kind) where payload_kind is 'bytes' or 'text', or None if the
    leaf has no directly-usable bytes/text producing function (typed-only leaf -- skip, matching
    the same 'skip' precedent w14's deserializer_call_style already established for imports)."""
    text = open(leaf_path, encoding="utf-8").read()
    for name in ("serialize_bytes", "export_bytes", "export"):
        if re.search(r"pub fn " + name + r"\s*\(", text):
            m = re.search(r"pub fn " + name + r"\s*\([^)]*\)\s*->\s*Result<\s*Vec<u8>", text)
            if m:
                return (name, "bytes")
    for name in ("serialize_text", "export_text"):
        if re.search(r"pub fn " + name + r"\s*\([^)]*\)\s*->\s*Result<\s*String", text):
            return (name, "text")
    return None


def scan_export_targets(art_root):
    """[(target_dir, target_kind, tstd_dir, tmod, fn_name, payload_kind), ...] for every export
    leaf this artifact has that's directly usable (bytes or text producing)."""
    base = os.path.join(art_root, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🚪️io", "📤️export", "🧵️serializers", "🗿️artifacts")
    if not os.path.isdir(base):
        return []
    out = []
    for target_dir in sorted(os.listdir(base)):
        target_kind = DIR_TO_KIND.get(target_dir)
        if target_kind is None:
            continue
        tstd_dir = STANDARDS[target_kind]["dir"]
        tmod = STANDARDS[target_kind]["rust_mod"]
        leaf_path = os.path.join(base, target_dir, tstd_dir, "✳️any", "🦀️component.rs")
        if not os.path.isfile(leaf_path):
            continue
        call = detect_export_call(leaf_path)
        if call is None:
            continue
        fn_name, payload_kind = call
        out.append((target_dir, target_kind, tstd_dir, tmod, fn_name, payload_kind))
    return out


REGION_START = "//#region 🔖️ExportEntries\n"
REGION_END = "//#endregion 🔖️ExportEntries\n"


def build_export_region(kind, Name, targets, json_import_call):
    lines = [REGION_START]
    lines.append(
        f'/// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew\n'
        f'/// IMPORT-direction entries (each composer\'s own `reads()`) -- nothing registers the REVERSE\n'
        f'/// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models\n'
        f'/// "produce my own snapshot." These entries wrap the artifact\'s EXISTING `🚪️io/📤️export/🧵️serializers`\n'
        f'/// leaves (which already convert this artifact\'s snapshot straight to target-format bytes/text) as\n'
        f'/// their own `ComposerEntry` rows: `writes` = the target format\'s dialect, `reads` = just this\n'
        f'/// artifact\'s own dialect. `register_composer_entries` already inserts BOTH an Import key (target\n'
        f'/// reads from us) and an Export key (we export to target) per entry, so no framework change was\n'
        f'/// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py\n'
        f'/// -- hand-validated pattern on note/json first (see that file\'s own tests), pilot kept as reference.\n'
    )
    lines.append(f'const {kind.upper()}_DIALECT: Dialect = Dialect {{ artifact_kind: "s.{kind}", standard: StandardId("1"), subset: SubsetId("*") }};\n')
    if json_import_call:
        lines.append(f'const {kind.upper()}_JSON_BRIDGE_DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.json", standard: StandardId("{STANDARDS["json"]["slug"]}"), subset: SubsetId("*") }};\n')
    lines.append('\n')
    json_fallback = ""
    if json_import_call:
        json_fn, json_payload_kind = json_import_call
        json_tmod = STANDARDS["json"]["rust_mod"]
        json_call = f'crate::artifacts::{kind}::io::import::deserializers::artifacts::json::{json_tmod}::any::{json_fn}'
        if json_payload_kind == "bytes":
            json_body = (
                f'    if let Some(source) = sources.iter().find(|s| s.dialect == {kind.upper()}_JSON_BRIDGE_DIALECT) {{\n'
                f'        // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-\n'
                f'        // deserialized `serde_json::Value`, not this artifact\'s own wire text/binary -- json\n'
                f'        // is the universal bridge dialect every domain artifact already imports from.\n'
                f'        let bytes: Vec<u8> = match &source.payload {{\n'
                f'            IoPayload::Text(t) => t.as_bytes().to_vec(),\n'
                f'            IoPayload::Binary(b) => b.clone(),\n'
                f'        }};\n'
                f'        return {json_call}(&bytes).map_err(|e| ComposeError {{ message: e.to_string(), diagnostics: Vec::new() }});\n'
                f'    }}\n'
            )
        else:
            json_body = (
                f'    if let Some(source) = sources.iter().find(|s| s.dialect == {kind.upper()}_JSON_BRIDGE_DIALECT) {{\n'
                f'        // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-\n'
                f'        // deserialized `serde_json::Value`, not this artifact\'s own wire text/binary -- json\n'
                f'        // is the universal bridge dialect every domain artifact already imports from.\n'
                f'        let text = match &source.payload {{\n'
                f'            IoPayload::Text(t) => t.clone(),\n'
                f'            IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),\n'
                f'        }};\n'
                f'        return {json_call}(&text).map_err(|e| ComposeError {{ message: e.to_string(), diagnostics: Vec::new() }});\n'
                f'    }}\n'
            )
        json_fallback = json_body
    lines.append(
        f'fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::{kind}::{Name}Snapshot, ComposeError> {{\n'
        f'    if let Some(source) = sources.iter().find(|s| s.dialect == {kind.upper()}_DIALECT) {{\n'
        f'        let builder = match &source.payload {{\n'
        f'            IoPayload::Text(t) => {Name}AnyBuilder::from_text(t).map_err(|e| ComposeError {{ message: e.to_string(), diagnostics: Vec::new() }})?,\n'
        f'            IoPayload::Binary(b) => {Name}AnyBuilder::from_binary(b).map_err(|e| ComposeError {{ message: e.to_string(), diagnostics: Vec::new() }})?,\n'
        f'        }};\n'
        f'        return builder.build().map_err(|diagnostics| ComposeError {{ message: "{Name}Composer export: build() failed".into(), diagnostics }});\n'
        f'    }}\n'
        f'{json_fallback}'
        f'    Err(ComposeError {{ message: "{Name}Composer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() }})\n'
        f'}}\n\n'
    )
    entry_lines = []
    for target_dir, target_kind, tstd_dir, tmod, fn_name, payload_kind in targets:
        const_name = f"EXPORT_{target_kind.upper()}_DIALECT"
        fn_ident = f"compose_export_{target_kind}"
        std_id = STANDARDS[target_kind]["slug"]
        lines.append(f'const {const_name}: Dialect = Dialect {{ artifact_kind: "s.stdio.{target_kind}", standard: StandardId("{std_id}"), subset: SubsetId("*") }};\n')
        if payload_kind == "bytes":
            wrap = "IoPayload::Binary(bytes)"
            call = f'crate::artifacts::{kind}::io::export::serializers::artifacts::{target_kind}::{tmod}::any::{fn_name}(&snapshot)'
            body = f'    let bytes = {call}.map_err(|e| ComposeError {{ message: e.to_string(), diagnostics: Vec::new() }})?;\n'
        else:
            wrap = "IoPayload::Text(text)"
            call = f'crate::artifacts::{kind}::io::export::serializers::artifacts::{target_kind}::{tmod}::any::{fn_name}(&snapshot)'
            body = f'    let text = {call}.map_err(|e| ComposeError {{ message: e.to_string(), diagnostics: Vec::new() }})?;\n'
        lines.append(
            f'fn {fn_ident}(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {{\n'
            f'    let snapshot = rebuild_native_snapshot(sources)?;\n'
            f'{body}'
            f'    Ok(ComposedArtifact {{ dialect: {const_name}, payload: {wrap}, diagnostics: Vec::new(), confidence: IoConfidence::Medium }})\n'
            f'}}\n'
        )
        entry_lines.append(f'        ComposerEntry {{ writes: {const_name}, reads: &[{kind.upper()}_DIALECT], compose: {fn_ident} }},\n')
    lines.append(REGION_END)
    return "".join(lines), "".join(entry_lines)


def patch_composer(plugin_dir, art_dir, kind, Name):
    art_root = os.path.join(PLUGINS, plugin_dir, "🗿️artifacts", art_dir)
    std_composer_path = os.path.join(art_root, "🏅️standards", STD_DIR, "🎹️composer", "🦀️component.rs")
    if not os.path.isfile(std_composer_path):
        print(f"SKIP {plugin_dir}/{art_dir}: no standard-level composer file")
        return
    targets = scan_export_targets(art_root)
    if not targets:
        print(f"SKIP {plugin_dir}/{art_dir}: no directly-usable export leaves found")
        return

    text = open(std_composer_path, encoding="utf-8").read()

    # Strip any prior generated region + prior entries() body (idempotent re-run).
    text = re.sub(re.escape(REGION_START) + r".*?" + re.escape(REGION_END), "", text, flags=re.DOTALL)

    imports_needed = "ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of"
    old_import_re = re.compile(r"use semio_framework_plugin::\{[^}]*\};\n")
    text = old_import_re.sub(f"use semio_framework_plugin::{{{imports_needed}}};\n", text, count=1)

    subset_builder_path = os.path.join(PLUGINS, plugin_dir, "🗿️artifacts", art_dir, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🏗️builder", "🦀️component.rs")
    real_builder_name = existing_struct_name(subset_builder_path, "Builder", f"{Name}Builder")
    builder_use = f"use crate::artifacts::{kind}::standards::v1::subsets::any::builder::{real_builder_name} as {Name}AnyBuilder;\n"
    existing_builder_use_re = re.compile(r"use crate::artifacts::" + re.escape(kind) + r"::standards::v1::subsets::any::builder::\w+ as " + re.escape(Name) + r"AnyBuilder;\n")
    if existing_builder_use_re.search(text):
        text = existing_builder_use_re.sub(builder_use, text, count=1)
    else:
        composer_use_re = re.compile(r"(use crate::artifacts::" + re.escape(kind) + r"::standards::v1::subsets::any::composer::\w+ as \w+;\n)")
        text = composer_use_re.sub(r"\1" + builder_use, text, count=1)

    json_import_call = detect_json_import_call(art_root)
    region_text, entry_lines = build_export_region(kind, Name, targets, json_import_call)
    static_marker = "static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();\n"
    if static_marker not in text:
        raise SystemExit(f"unexpected shape in {std_composer_path}: no ENTRIES static found")
    text = text.replace(static_marker, static_marker + "\n" + region_text, 1)

    entries_fn_re = re.compile(r"pub fn entries\(\) -> &'static \[ComposerEntry\] \{\n.*?\n\}\n", re.DOTALL)
    m = entries_fn_re.search(text)
    if not m:
        raise SystemExit(f"unexpected shape in {std_composer_path}: no entries() fn found")
    native_call_m = re.search(r"composer_entry_of::<(\w+)>\(\)", m.group(0))
    if not native_call_m:
        raise SystemExit(f"unexpected shape in {std_composer_path}: no composer_entry_of(...) call in entries()")
    native_composer_type = native_call_m.group(1)
    new_fn = (
        "pub fn entries() -> &'static [ComposerEntry] {\n"
        "    ENTRIES.get_or_init(|| vec![\n"
        f"        composer_entry_of::<{native_composer_type}>(),\n"
        f"{entry_lines}"
        "    ]).as_slice()\n"
        "}\n"
    )
    text = text[:m.start()] + new_fn + text[m.end():]

    open(std_composer_path, "w", encoding="utf-8").write(text)
    print(f"OK  {plugin_dir}/{art_dir:12s} kind={kind:12s} export_targets={','.join(t[1] for t in targets)}")


if __name__ == "__main__":
    plugin_dir = sys.argv[1]
    for art_dir in sys.argv[2:]:
        kind = find_kind_module(plugin_dir, art_dir)
        art_root = os.path.join(PLUGINS, plugin_dir, "🗿️artifacts", art_dir)
        Name = artifact_name_from_root(art_root)
        patch_composer(plugin_dir, art_dir, kind, Name)
