#!/usr/bin/env python3
"""🔍️ Static check of the invariant `validate_mutation_leaf_source` enforces at compile time.

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:649` requires, for every leaf an aggregate
mounts, that the leaf descriptor's `owner` be an IMMEDIATE CHILD of the aggregate's own mutation
root, and that the descriptor and source paths equal `owner` plus the canonical filename. A leaf
mounted with a `#[path]` that climbs out into a sibling subset can never satisfy that, and the derive
fails with `E0080` — which only surfaces once the whole dependency tree compiles, so this repo's
in-flight framework breakage hides it.

Checks every `🧬️mutations/🦀️.rs` aggregate in the tree and reports:
  ESCAPES   — a `#[path]` mount that leaves the aggregate's own mutation root.
  OWNER     — a leaf `🔣️.json` whose `owner` is not the leaf's real directory.
  MISSING   — a `#[path]` mount whose target file does not exist.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
PATH_ATTR = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
SKIP = {"target", "node_modules", ".git", "dist", "build", "storybook-static", ".nx"}


def aggregates() -> list[Path]:
    found = []
    stack = [ROOT]
    while stack:
        current = stack.pop()
        try:
            entries = list(current.iterdir())
        except OSError:
            continue
        for entry in entries:
            if not entry.is_dir() or entry.is_symlink() or entry.name in SKIP:
                continue
            if entry.name == "🧬️mutations":
                if (entry / "🦀️.rs").is_file():
                    found.append(entry / "🦀️.rs")
                continue
            stack.append(entry)
    return sorted(found)


def main() -> int:
    problems: list[tuple[str, str, str]] = []
    checked = 0
    for aggregate in aggregates():
        root = aggregate.parent
        text = aggregate.read_text(encoding="utf-8", errors="replace")
        for mount in PATH_ATTR.findall(text):
            if mount == ".":
                continue
            checked += 1
            target = (root / mount).resolve()
            leaf = target.parent
            if ".." in mount.split("/"):
                problems.append(("ESCAPES", str(aggregate.relative_to(ROOT)), mount))
                continue
            try:
                relative = leaf.relative_to(root)
            except ValueError:
                problems.append(("ESCAPES", str(aggregate.relative_to(ROOT)), mount))
                continue
            if len(relative.parts) != 1:
                continue
            if not target.is_file():
                problems.append(("MISSING", str(aggregate.relative_to(ROOT)), mount))
                continue
            descriptor = leaf / "🔣️.json"
            if not descriptor.is_file():
                continue
            try:
                owner = json.loads(descriptor.read_text(encoding="utf-8")).get("owner")
            except json.JSONDecodeError:
                problems.append(("BAD-JSON", str(descriptor.relative_to(ROOT)), ""))
                continue
            if owner is None:
                continue
            expected = str(leaf.relative_to(ROOT))
            if owner != expected:
                problems.append(("OWNER", str(descriptor.relative_to(ROOT)), f"{owner!r} != {expected!r}"))

    by_kind: dict[str, int] = {}
    for kind, where, detail in problems:
        by_kind[kind] = by_kind.get(kind, 0) + 1
    print(f"aggregates={len(aggregates())} mounts={checked} problems={len(problems)}")
    for kind, count in sorted(by_kind.items(), key=lambda kv: -kv[1]):
        print(f"  {kind}: {count}")
    for kind, where, detail in problems[:60]:
        print(f"{kind:14s} {where}  {detail}")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
