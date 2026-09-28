#!/usr/bin/env python3
"""🧹️ T14 item 6: removes the last per-app selection fallback wrapper (a compat shim). The shared core
`semio_framework_plugin::selection_ids` reads only the `ids` array; puzzle2d kept a dead `selection_ids` wrapper that also
accepted a singular `id` (0 callers on the live tree 2026-09-28; sequence, trinity, procedural and mindmap wrappers are gone —
procedural's `transforms::selection_ids(ids, fallback)` is the current-selection default, not a key fallback). The core's doc
drops the "keep their own fallback wrapper for now" sentence.
usage: fallback-wrappers.py [--write] [--root <tree>]   (default: dry run on the live tree)"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
EDITS = [
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
     """    /// 🎯️ Parses a selection-action's `ids` array arg into a plain `Vec<String>` — the shape used by the
    /// majority of duplicate copies (`layout`, `gis`, `presentation`, …). A handful of apps additionally
    /// fall back to a singular `id`/`nodeId`/`nodeIds` key (`puzzle`, `sequence`, `trinity`, `procedural`,
    /// `mindmap`); those apps keep their own fallback wrapper around this shared core for now.
""",
     """    /// 🎯️ Parses a selection-action's `ids` array arg into a plain `Vec<String>` — the one selection-argument shape
    /// every app's selection actions accept.
"""),
    ("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
     """/// 🎯️ `semio_framework_plugin::selection_ids`'s "ids" array plus a singular "id" fallback —
/// this app's actions accept either shape depending on the caller.
pub fn selection_ids(args: Option<&Value>) -> Vec<String> {
    let dsl_args = args.map(dsl::DslValue::from);
    let ids = semio_framework_plugin::selection_ids(dsl_args.as_ref());
    if !ids.is_empty() {
        return ids;
    }
    args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()]).unwrap_or_default()
}

""", ""),
]
problems, changed = [], {}
for rel, old, new in EDITS:
    path = ROOT / rel
    text = changed.get(path) or path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        if (new and new in text) or (not new and old not in text and "fn selection_ids(args: Option<&Value>)" not in text):
            continue
        problems.append(f"{rel}: anchor found {text.count(old)} times")
        continue
    changed[path] = text.replace(old, new)
for path in changed:
    print(("write " if WRITE else "dry-run ") + str(path.relative_to(ROOT)))
if not changed and not problems:
    print("nothing to do (applied)")
for problem in problems:
    print("problem:", problem)
print(f"{len(changed)} files, {len(problems)} problems")
if problems:
    sys.exit(1)
if WRITE:
    for path, text in changed.items():
        path.write_text(text, encoding="utf-8")
