#!/usr/bin/env python3
"""🧱️ R10 prepared window-3 step (sessions 14b + 14c): after the host modules reach three.js and its BVH addon only through
the ui module (`verify interface-owners` PASS), the manifests follow the imports, in each `package.json` AND in that
workspace's block of the root `bun.lock`, textually (no `bun install` resolution, so `node_modules` is untouched):
  - `three` moves to `devDependencies` in the infinite world r3f package and the renderer's React target (test/oracle use only);
  - `three-mesh-bvh` leaves the r3f package (no test imports it) and joins the ui React target's `dependencies` (its interface
    owner, `DEPENDENCY_INTERFACE_OWNERS`), at the version the lock resolves (`^0.9.14`).
Idempotent; exact-once anchors; dry run by default. After `--apply`: `bun install --frozen-lockfile --dry-run` must report no
change, then `verify dependencies literal-external` (three / three-mesh-bvh oracle conflict users shrink to none).
Usage: python3 three-manifests.py [--apply]
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
R3F = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript"
RENDERER = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
UI = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"
OPERATIONS = [
    ("to-dev", R3F, "three", None),
    ("to-dev", RENDERER, "three", None),
    ("drop", R3F, "three-mesh-bvh", None),
    ("add", UI, "three-mesh-bvh", "^0.9.14"),
]


def section_span(text: str, section: str, indent: str) -> tuple[int, int]:
    start = text.index(f'{indent}"{section}": {{\n')
    return start, text.index(f"\n{indent}}}", start) + 1


def render_entries(entries: list[str], indent: str, trailing: bool) -> str:
    return "".join(f"{indent}{entry}{',' if trailing or index < len(entries) - 1 else ''}\n" for index, entry in enumerate(entries))


def edit_section(text: str, section: str, indent: str, trailing: bool, remove: str | None = None, insert: str | None = None) -> str:
    start, end = section_span(text, section, indent)
    head = text.index("\n", start) + 1
    entries = [line.strip().rstrip(",") for line in text[head:end].splitlines() if line.strip()]
    if remove is not None:
        entries = [entry for entry in entries if entry.split('"')[1] != remove]
    if insert is not None:
        key = insert.split('"')[1]
        at = next((index for index, entry in enumerate(entries) if entry.split('"')[1] > key), len(entries))
        entries.insert(at, insert)
    return text[:head] + render_entries(entries, indent + "  ", trailing) + text[end:]


def entry_of(text: str, section: str, indent: str, name: str) -> str | None:
    start, end = section_span(text, section, indent)
    for line in text[start:end].splitlines()[1:]:
        if line.strip().startswith(f'"{name}":'):
            return line.strip().rstrip(",")
    return None


def apply(text: str, operation: str, name: str, version: str | None, indent: str, trailing: bool) -> str:
    runtime = entry_of(text, "dependencies", indent, name)
    if operation == "to-dev":
        if runtime is None:
            return text
        text = edit_section(text, "dependencies", indent, trailing, remove=name)
        return edit_section(text, "devDependencies", indent, trailing, insert=runtime)
    if operation == "drop":
        return text if runtime is None else edit_section(text, "dependencies", indent, trailing, remove=name)
    return text if runtime is not None else edit_section(text, "dependencies", indent, trailing, insert=f'"{name}": "{version}"')


def workspace_block(lock: str, workspace: str) -> tuple[int, int]:
    start = lock.index(f'    "{workspace}": {{\n')
    return start, lock.index("\n    },\n", start) + 1


def main() -> int:
    write = "--apply" in sys.argv
    lock_path = ROOT / "bun.lock"
    original_lock = lock = lock_path.read_text(encoding="utf-8")
    manifests: dict[Path, str] = {}
    for operation, package, name, version in OPERATIONS:
        path = ROOT / package / "package.json"
        text = manifests.get(path, path.read_text(encoding="utf-8"))
        manifests[path] = apply(text, operation, name, version, "  ", False)
        json.loads(manifests[path])
        start, end = workspace_block(lock, package)
        lock = lock[:start] + apply(lock[start:end], operation, name, version, "      ", True) + lock[end:]
    writes = {path: text for path, text in manifests.items() if text != path.read_text(encoding="utf-8")}
    if lock != original_lock:
        writes[lock_path] = lock
    for path, text in writes.items():
        print(f"{'write' if write else 'would write'} {path.relative_to(ROOT)}")
        if write:
            path.write_text(text, encoding="utf-8")
    print("nothing to do" if not writes else "applied" if write else "dry run clean")
    return 0


if __name__ == "__main__":
    sys.exit(main())
