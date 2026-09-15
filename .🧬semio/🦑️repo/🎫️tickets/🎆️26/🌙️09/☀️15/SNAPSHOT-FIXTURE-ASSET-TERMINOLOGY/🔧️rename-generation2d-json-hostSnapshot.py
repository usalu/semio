#!/usr/bin/env python3
"""Rename persisted generation2d JSON field fixture → hostSnapshot in schemas and 🧫️fixtures oracles."""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
BASE = ROOT / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any"

REPLACEMENTS = [
    ('"fixture": null', '"hostSnapshot": null'),
    ('"fixture": {', '"hostSnapshot": {'),
    ('"fixture":', '"hostSnapshot":'),
    ('"fixture",', '"hostSnapshot",'),
    ("reads: &[\"fixture\"]", 'reads: &["hostSnapshot"]'),
]


def rewrite(text: str) -> str:
    if "fixture" not in text and "hostSnapshot" in text:
        return text
    out = text
    for old, new in REPLACEMENTS:
        out = out.replace(old, new)
    return out


def main() -> None:
    changed: list[Path] = []
    for path in BASE.rglob("*"):
        if path.suffix not in (".json", ".rs"):
            continue
        if "🧫️fixtures/🧩️example" in str(path):
            continue
        source = path.read_text(encoding="utf-8")
        if "fixture" not in source and "FlowFixture" not in source:
            continue
        updated = rewrite(source)
        if updated != source:
            path.write_text(updated, encoding="utf-8")
            changed.append(path)
    print(f"updated {len(changed)} files")


if __name__ == "__main__":
    main()
