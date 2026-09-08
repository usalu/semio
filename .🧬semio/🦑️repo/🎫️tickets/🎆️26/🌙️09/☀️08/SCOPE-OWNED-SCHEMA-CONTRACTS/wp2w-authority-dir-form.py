#!/usr/bin/env python3
"""🛂️ Unifies the library's per-case schema authorities on the ONE declared shape.

`semanticDirectoryKinds.test-fixture-schema-authority` declares a DIRECTORY (`🛂️` + slug `schema`); the
flat `🛂️schema.json` file form resolves to no `fileKinds` entry at all (`fileKindIdForFilename` → null),
exactly like the retired `🧬️schema.json` it replaced. This moves every flat file to
`<case>/🛂️schema/🔣️.json` and rewrites the readers, which all live inside the library partition.

Usage: wp2w-authority-dir-form.py [--dry]
"""
import os, sys

LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
FLAT, DIRECTORY = "🛂️schema.json", "🛂️schema/🔣️.json"
SOURCE_SUFFIXES = (".ts", ".tsx", ".json", ".mjs", ".js", ".go", ".md", ".feature", ".jsonc")


def flat_files():
    found = []
    for directory, _, names in os.walk(LIB):
        if FLAT in names:
            found.append(os.path.join(directory, FLAT))
    return sorted(found)


def sources():
    found = []
    for directory, _, names in os.walk(LIB):
        for name in names:
            if name.endswith(SOURCE_SUFFIXES):
                found.append(os.path.join(directory, name))
    return sorted(found)


def main():
    dry = "--dry" in sys.argv
    moved = 0
    for path in flat_files():
        target = os.path.join(os.path.dirname(path), DIRECTORY)
        print(f"mv {path} -> {target}")
        moved += 1
        if not dry:
            os.makedirs(os.path.dirname(target), exist_ok=True)
            os.rename(path, target)
    touched = 0
    for path in sources():
        if not os.path.exists(path):
            continue
        before = open(path, encoding="utf-8").read()
        after = before.replace(FLAT, DIRECTORY)
        if after != before:
            touched += 1
            print(f"edit {path} ({before.count(FLAT)})")
            if not dry:
                open(path, "w", encoding="utf-8").write(after)
    print(f"# {moved} authorities moved, {touched} files rewritten")


main()
