#!/usr/bin/env python3
# usage: overlay.py [entry ...]   rebuilds C/🗑️generated/rcache-overlay: COPIES of the edited repo files with the edit set applied,
# the staged new files, support files, and the relative-import closure of the entry files (copies, never symlinks).
import json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine import *

OVERLAY = os.path.join(os.path.dirname(TOOLS), "🗑️generated", "rcache-overlay")
SUPPORT = [
    "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts",
    "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts",
    ".cargo/config.toml",
    "package.json",
    "bunfig.toml",
    "📋️project.json",
]
TREES = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"]
IMPORT = re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+)"(\.{1,2}/[^"]+)"''')


def copy(relative):
    source = os.path.join(ROOT, relative)
    target = os.path.join(OVERLAY, relative)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    if not os.path.exists(target):
        shutil.copy2(source, target)


def closure(entries, seen):
    pending = list(entries)
    while pending:
        relative = pending.pop()
        if relative in seen:
            continue
        seen.add(relative)
        path = os.path.join(OVERLAY if os.path.exists(os.path.join(OVERLAY, relative)) else ROOT, relative)
        if not os.path.isfile(path) or not relative.endswith((".ts", ".mjs", ".js")):
            continue
        for match in IMPORT.finditer(read(path)):
            target = os.path.normpath(os.path.join(os.path.dirname(relative), match.group(1)))
            if os.path.isfile(os.path.join(ROOT, target)) or os.path.isfile(os.path.join(STAGED, target)):
                pending.append(target)


def main():
    if os.path.isdir(OVERLAY):
        shutil.rmtree(OVERLAY)
    os.makedirs(OVERLAY)
    edits = load_edits()
    hashes = {}
    for relative in edited_files(edits):
        source = os.path.join(ROOT, relative)
        text = read(source)
        hashes[relative] = sha(text)
        for edit in [e for e in edits if e["file"] == relative]:
            text = apply_one(text, edit)
        target = os.path.join(OVERLAY, relative)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "w", encoding="utf8") as handle:
            handle.write(text)
    for relative in staged_files():
        target = os.path.join(OVERLAY, relative)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copy2(os.path.join(STAGED, relative), target)
    for relative in SUPPORT:
        copy(relative)
    for tree in TREES:
        for base, dirs, names in os.walk(os.path.join(ROOT, tree)):
            dirs[:] = [name for name in dirs if name not in ("node_modules", ".cache", "target")]
            for name in names:
                copy(os.path.relpath(os.path.join(base, name), ROOT))
    seen = set()
    entries = [e for e in sys.argv[1:]]
    closure(entries, seen)
    for relative in sorted(seen):
        if os.path.isfile(os.path.join(ROOT, relative)):
            copy(relative)
    base = os.path.join(TOOLS, "base-hashes.json")
    if not os.path.exists(base):
        with open(base, "w", encoding="utf8") as handle:
            json.dump(hashes, handle, ensure_ascii=False, indent=1)
    print(f"overlay ready: {len(edited_files(edits))} edited, {len(staged_files())} staged, {len(seen)} closure files")


main()
