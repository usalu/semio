#!/usr/bin/env python3
"""Fix generation3d Rust bodies that still reference `fixture` after host_snapshot param rename."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
BASE = ROOT / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d"

SKIP_LINE = re.compile(
    r"let\s+fixture\b|fixture_json|Fixture|store_fixture|preview_fixture|"
    r"fixture-drop|decode_fixture|KEYBOARD_|FIXTURE_JSON|ExampleGeometry|"
    r"ctx\.fixture|/fixture|three-widget import fixture|load-bearing assertion in the fixture|"
    r"host_snapshot:\s*fixture\b|Generation3dSnapshot \{ host_snapshot: fixture"
)

BODY = [
    (re.compile(r"\bsnapshot\.nodes\b"), "host_snapshot.nodes"),
    (re.compile(r"\bsnapshot\.edges\b"), "host_snapshot.edges"),
    (re.compile(r"\bfixture\."), "host_snapshot."),
    (re.compile(r"\bfixture,"), "host_snapshot,"),
    (re.compile(r"\(fixture,"), "(host_snapshot,"),
    (re.compile(r", fixture\)"), ", host_snapshot)"),
    (re.compile(r"\(fixture\)"), "(host_snapshot)"),
    (re.compile(r"&fixture\b"), "&host_snapshot"),
    (re.compile(r"\bfixture\b(?=\s*\))"), "host_snapshot"),
    (re.compile(r"\bfixture\b(?=\s*,)"), "host_snapshot"),
]


def rewrite_line(line: str) -> str:
    if SKIP_LINE.search(line):
        return line
    out = line
    for pattern, repl in BODY:
        out = pattern.sub(repl, out)
    return out


def rewrite(source: str) -> str:
    if "FlowHostSnapshot" not in source and "DagHostSnapshot" not in source:
        return source
    lines = source.splitlines(keepends=True)
    return "".join(rewrite_line(line) for line in lines)


def main() -> None:
    changed: list[Path] = []
    for path in BASE.rglob("*.rs"):
        if "🧫️fixtures" in str(path) and "/🧪️" not in str(path):
            continue
        source = path.read_text(encoding="utf-8")
        updated = rewrite(source)
        if updated != source:
            path.write_text(updated, encoding="utf-8")
            changed.append(path)
    print(f"updated {len(changed)} files")


if __name__ == "__main__":
    main()
