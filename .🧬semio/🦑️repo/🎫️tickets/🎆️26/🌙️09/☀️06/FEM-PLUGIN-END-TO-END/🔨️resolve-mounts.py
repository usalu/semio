#!/usr/bin/env python3
"""🧭 Resolve every `#[path = "…"]` / `include_str!("…")` literal in the fem crate entry.

Leaf `#[path]` literals in this crate are written relative to the crate-entry *directory*
(grouping modules use `#[path = "."]`), so every literal is resolved against
`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/`.

Usage:
    python3 🔨️resolve-mounts.py [--suggest] [--fix] [<entry.rs> ...]

`--suggest` lists, for each unresolved leaf, the sibling directories of its parent so the
truncated on-disk name can be matched by leading emoji + prefix.

`--fix` rewrites each dangling leaf literal to the unambiguous on-disk sibling that actually
carries the referenced file. A leaf is only rewritten when exactly one candidate sibling
qualifies (either the sole sibling, or the sole one sharing the literal's longest common
prefix); ambiguous leaves are reported and left untouched.
"""

import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 7)))
DEFAULT_ENTRY = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "📦️packages", "🦀️rust", "🦀️.rs")

PATH_RE = re.compile(r'#\[path\s*=\s*"([^"]+)"\s*\]')
INCLUDE_RE = re.compile(r'include_str!\(\s*"([^"]+)"\s*\)')


def literals(entry):
    with open(entry, encoding="utf-8") as handle:
        for lineno, line in enumerate(handle, 1):
            for match in PATH_RE.finditer(line):
                yield lineno, "path", match.group(1)
            for match in INCLUDE_RE.finditer(line):
                yield lineno, "include_str", match.group(1)


def suggest(target):
    parent = os.path.dirname(os.path.dirname(target))
    if not os.path.isdir(parent):
        return []
    return sorted(os.listdir(parent))


def candidates(target):
    tests_dir = os.path.dirname(os.path.dirname(target))
    leaf = os.path.basename(os.path.dirname(target))
    filename = os.path.basename(target)
    if not os.path.isdir(tests_dir):
        return leaf, []
    found = [
        name
        for name in sorted(os.listdir(tests_dir))
        if os.path.exists(os.path.join(tests_dir, name, filename))
    ]
    if len(found) > 1:
        best = max(len(os.path.commonprefix([leaf, name])) for name in found)
        found = [name for name in found if len(os.path.commonprefix([leaf, name])) == best]
    return leaf, found


def fix(entry, unresolved):
    with open(entry, encoding="utf-8") as handle:
        source = handle.read()
    mapping = []
    ambiguous = []
    for _, _, literal, target in unresolved:
        leaf, found = candidates(target)
        if len(found) != 1:
            ambiguous.append((literal, found))
            continue
        replacement = literal.replace("/" + leaf + "/", "/" + found[0] + "/")
        if replacement == literal or source.count('"' + literal + '"') != 1:
            ambiguous.append((literal, found))
            continue
        source = source.replace('"' + literal + '"', '"' + replacement + '"')
        mapping.append((leaf, found[0]))
    with open(entry, "w", encoding="utf-8") as handle:
        handle.write(source)
    print(f"   rewrote:    {len(mapping)}")
    for old, new in mapping:
        print(f"        {old}  →  {new}")
    for literal, found in ambiguous:
        print(f"   AMBIGUOUS: {literal} → {found}")


def audit(entry, want_suggest, want_fix=False):
    base = os.path.dirname(os.path.abspath(entry))
    total = 0
    unresolved = []
    for lineno, kind, literal in literals(entry):
        if literal in (".", ".."):
            continue
        total += 1
        target = os.path.normpath(os.path.join(base, literal))
        if not os.path.exists(target):
            unresolved.append((lineno, kind, literal, target))
    rel = os.path.relpath(entry, REPO)
    print(f"── {rel}")
    print(f"   checked:    {total}")
    print(f"   resolved:   {total - len(unresolved)}")
    print(f"   unresolved: {len(unresolved)}")
    for lineno, kind, literal, target in unresolved:
        print(f"   [{lineno}] {kind}: {literal}")
        if want_suggest:
            for name in suggest(target):
                print(f"        · {name}")
    if want_fix and unresolved:
        fix(entry, unresolved)
    return len(unresolved)


def main():
    flags = {"--suggest", "--fix"}
    args = [a for a in sys.argv[1:] if a not in flags]
    want_suggest = "--suggest" in sys.argv[1:]
    want_fix = "--fix" in sys.argv[1:]
    entries = args or [DEFAULT_ENTRY]
    bad = 0
    for entry in entries:
        bad += audit(entry, want_suggest, want_fix)
    print(f"TOTAL unresolved: {bad}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
