#!/usr/bin/env python3
"""Rename dag_*fixture* helpers to dag_*host_snapshot* across framework + plugins."""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
SCAN = [ROOT / "🧰️framework", ROOT / "✏️s/🔌️plugins"]
SKIP = ("🧫️fixtures/", ".🧬semio/🦑️repo/🎫️tickets/")

REPLACEMENTS = [
    ("dag_fixture_to_wire_literal", "dag_host_snapshot_to_wire_literal"),
    ("dag_fixture_execution_rows", "dag_host_snapshot_execution_rows"),
    ("dag_fixture_from_document", "dag_host_snapshot_from_document"),
    ("validate_dag_fixture_node_kinds", "validate_dag_host_snapshot_node_kinds"),
    ("build_dag_fixture", "build_dag_host_snapshot"),
]


def ok(path: Path) -> bool:
    s = str(path)
    return not any(x in s for x in SKIP)


def main() -> None:
    n = 0
    for base in SCAN:
        if not base.is_dir():
            continue
        for path in base.rglob("*.rs"):
            if not ok(path):
                continue
            text = path.read_text(encoding="utf-8")
            orig = text
            for old, new in REPLACEMENTS:
                text = text.replace(old, new)
            if text != orig:
                path.write_text(text, encoding="utf-8")
                n += 1
    print(f"updated {n} rust files")


if __name__ == "__main__":
    main()
