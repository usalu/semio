#!/usr/bin/env python3
"""🔧 Renames fixture_to_workflow to dag_host_document_to_workflow in product code."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

REPLACEMENTS: list[tuple[str, str]] = [
    ("fixture_to_workflow", "dag_host_document_to_workflow"),
    ("fn dag_host_document_to_workflow(fixture:", "fn dag_host_document_to_workflow(host_document:"),
    ("dag_host_document_to_workflow(&host.dag.host_document)", "dag_host_document_to_workflow(&host.dag.host_document)"),
    ("with_host(fixture,", "with_host(snapshot,"),
    ("with_host(&fixture,", "with_host(&snapshot,"),
]


def targets() -> list[Path]:
    out = subprocess.check_output(
        ["rg", "-l", "fixture_to_workflow", "--glob", "*.rs", "✏️s"],
        cwd=ROOT,
        text=True,
    )
    return [ROOT / line.strip() for line in out.splitlines() if line.strip()]


def rewrite(path: Path) -> bool:
    text = path.read_text()
    original = text
    for old, new in REPLACEMENTS:
        text = text.replace(old, new)
    if text != original:
        path.write_text(text)
        return True
    return False


def main() -> None:
    changed = sum(1 for path in targets() if rewrite(path))
    print(f"updated {changed} files for dag_host_document_to_workflow")


if __name__ == "__main__":
    main()
