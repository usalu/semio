#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""One-off repo-wide fix: a systemic copy-paste bug from the original W6 domain fan-out wave
duplicated stdio's own internal `json <- txt` bridge leaf (`use crate::artifacts::json::...`,
`use crate::artifacts::txt::...`) into MANY domain plugins' own 📄txt target folders, where those
module paths don't exist (domain crates reach stdio types via `semio_s_plugin_stdio::artifacts::
json::...`, not `crate::artifacts::json::...`). Never compiled before (dead code, not mounted by
the old glue). Finds every instance repo-wide (migrated or not-yet-migrated shape) OUTSIDE the
stdio plugin itself, and outside any artifact actually named "json" or "txt", and replaces with an
honest stub producing the artifact's own real snapshot type.

Usage: python3 w14_fix_misplaced_json_via_txt_leaves.py [--apply]   (dry-run without --apply)
"""
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
PLUGINS = os.path.join(REPO, "✏️s/🔌️plugins")

BROKEN_MARKER = "use crate::artifacts::json::"
STUB_TEMPLATE = """//! {doc}
//! 🐛️ Pre-migration content here referenced `crate::artifacts::json`/`crate::artifacts::txt`,
//! types that don't exist in this crate (dead code, never mounted by the old glue, never
//! compiled) -- likely a copy-paste of stdio's own internal json<-txt bridge into the wrong
//! plugin's txt target folder. Left as an honest stub producing this artifact's own real
//! snapshot type, pending a real txt import/export implementation.
use crate::artifacts::{kind}::{Name}Snapshot;
pub fn register() {{}}
pub fn {fn_sig} {{
    Err("txt {direction_word} not yet implemented".into())
}}
pub fn deserialize_bytes(_bytes: &[u8]) -> Result<{Name}Snapshot, String> {{
    Err("txt import not yet implemented".into())
}}
"""


def find_kind_and_name(plugin_dir, filepath):
    """Best-effort: derive the plugin's own artifact kind + PascalCase Name from its root
    component.rs (searching both migrated and unmigrated tree shapes). The path contains a NESTED
    🗿️artifacts occurrence too (the io leaf's own `.../🚪️io/.../🗿️artifacts/<target>/...`), so use
    the OUTERMOST (leftmost, closest to the plugin root) 🗿️artifacts/<artifact-name> pair, not
    whichever is found first walking bottom-up."""
    rel = os.path.relpath(filepath, PLUGINS)
    parts = rel.split(os.sep)
    try:
        idx = parts.index("🗿️artifacts")  # first (outermost) occurrence
    except ValueError:
        return None, None
    if idx + 1 >= len(parts):
        return None, None
    art_root_dir = os.path.join(PLUGINS, *parts[: idx + 2])
    root_rs = os.path.join(art_root_dir, "🦀️component.rs")
    if not os.path.exists(root_rs):
        return None, None
    snap_rs_candidates = [
        os.path.join(art_root_dir, "🧬️schema", "📸️snapshot", "🦀️component.rs"),
    ]
    for std_dir in os.listdir(os.path.join(art_root_dir, "🏅️standards")) if os.path.isdir(os.path.join(art_root_dir, "🏅️standards")) else []:
        snap_rs_candidates.append(os.path.join(art_root_dir, "🏅️standards", std_dir, "🪆️subsets", "✳️any", "🧬️schema", "📸️snapshot", "🦀️component.rs"))
    Name = None
    for c in snap_rs_candidates:
        if os.path.exists(c):
            m = re.search(r"pub struct (\w+)Snapshot\b", open(c, encoding="utf-8").read())
            if m:
                Name = m.group(1)
                break
    if Name is None:
        return None, None
    kind = Name[0].lower() + Name[1:]
    return kind, Name


def main(apply: bool):
    hits = []
    for dirpath, _dirs, files in os.walk(PLUGINS):
        if "🗄️stdio" in dirpath.split(os.sep):
            continue  # stdio's own self-referencing leaves are legitimate
        if "🗿️artifacts" not in dirpath.split(os.sep):
            continue
        if "🦀️component.rs" not in files:
            continue
        # only leaf files sitting under a .../🗿️artifacts/<target>/... io path, target = txt
        parts = dirpath.split(os.sep)
        try:
            idx = len(parts) - 1 - parts[::-1].index("🗿️artifacts")
        except ValueError:
            continue
        if idx + 1 >= len(parts):
            continue
        target_dir = parts[idx + 1]
        if target_dir != "📄txt":
            continue
        fp = os.path.join(dirpath, "🦀️component.rs")
        text = open(fp, encoding="utf-8").read()
        if BROKEN_MARKER not in text:
            continue
        hits.append(fp)

    print(f"found {len(hits)} broken leaf(s)")
    for fp in hits:
        kind, Name = find_kind_and_name(fp, fp)
        if kind is None:
            print("  SKIP (could not derive artifact identity):", fp)
            continue
        is_deser = "📥️import" in fp or "🧩️deserializers" in fp
        fn_sig = f"deserialize(_from: &semio_s_plugin_stdio::artifacts::txt::TxtSnapshot) -> Result<{Name}Snapshot, String>" if is_deser \
            else f"serialize(_from: &{Name}Snapshot) -> Result<semio_s_plugin_stdio::artifacts::txt::TxtSnapshot, String>"
        direction_word = "import" if is_deser else "export"
        doc = f"deser {kind} via txt" if is_deser else f"ser {kind} to txt"
        content = STUB_TEMPLATE.format(doc=doc, kind=kind, Name=Name, fn_sig=fn_sig, direction_word=direction_word)
        print(f"  {'APPLY' if apply else 'WOULD FIX'} [{kind}/{Name}]: {fp}")
        if apply:
            open(fp, "w", encoding="utf-8").write(content)


if __name__ == "__main__":
    main(apply="--apply" in sys.argv)
