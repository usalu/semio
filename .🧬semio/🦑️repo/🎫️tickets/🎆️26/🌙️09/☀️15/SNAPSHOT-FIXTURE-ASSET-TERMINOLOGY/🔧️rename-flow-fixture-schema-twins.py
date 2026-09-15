#!/usr/bin/env python3
"""Align generation2d/3d GraphQL, proto, and TS schema twins with Rust FlowHostSnapshot / host_snapshot."""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
BASES = [
    ROOT / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
    ROOT / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
]
SUFFIXES = (".graphql", ".proto", ".ts")

REPLACEMENTS = [
    ("FlowFixtureLayoutEntry", "FlowHostSnapshotLayoutEntry"),
    ("parseFlowFixture", "parseFlowHostSnapshot"),
    ("FlowFixture", "FlowHostSnapshot"),
    ('row["fixture"]', 'row["hostSnapshot"]'),
    ("`${at}.fixture`", "`${at}.hostSnapshot`"),
    ("optional FlowHostSnapshot fixture", "optional FlowHostSnapshot host_snapshot"),
    ("FlowHostSnapshot fixture", "FlowHostSnapshot host_snapshot"),
    ("fixture: FlowHostSnapshot", "hostSnapshot: FlowHostSnapshot"),
    ("fixture?: FlowHostSnapshot", "hostSnapshot?: FlowHostSnapshot"),
]


def rewrite(text: str) -> str:
    if "FlowFixture" not in text and "parseFlowFixture" not in text and "fixture: FlowHostSnapshot" not in text:
        if 'fixture: FlowHostSnapshot' not in text and 'row["fixture"]' not in text:
            return text
    out = text
    for old, new in REPLACEMENTS:
        out = out.replace(old, new)
    return out


def main() -> None:
    changed: list[Path] = []
    for base in BASES:
        if not base.is_dir():
            continue
        for path in base.rglob("*"):
            if path.suffix not in SUFFIXES:
                continue
            source = path.read_text(encoding="utf-8")
            updated = rewrite(source)
            if updated != source:
                path.write_text(updated, encoding="utf-8")
                changed.append(path)
    print(f"updated {len(changed)} files")
    for p in changed:
        print(p.relative_to(ROOT))


if __name__ == "__main__":
    main()
