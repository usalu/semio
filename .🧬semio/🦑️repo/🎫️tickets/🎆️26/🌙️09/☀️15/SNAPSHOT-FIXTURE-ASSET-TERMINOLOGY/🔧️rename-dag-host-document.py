#!/usr/bin/env python3
"""🔧 Renames DagFixture host document symbols to DagHostDocument in framework DAG."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

REPLACEMENTS: list[tuple[str, str]] = [
    ("DagFixtureEdge", "DagHostDocumentEdge"),
    ("dag_document_from_fixture", "dag_document_from_host_document"),
    ("from_fixture_without_layout", "from_host_document_without_layout"),
    ("from_fixture_with", "from_host_document_with"),
    ("from_fixture", "from_host_document"),
    ("to_fixture", "to_host_document"),
    ("DagFixture", "DagHostDocument"),
    ('"dag.fixture"', '"dag.hostDocument"'),
    ("dag.fixture", "dag.hostDocument"),
    (".fixture()", ".host_document()"),
    ("fn fixture(&self)", "fn host_document(&self)"),
]


def targets() -> list[Path]:
    paths: list[Path] = []
    for pattern, glob in [
        ("DagFixture|dag\\.fixture|from_fixture", "*.rs"),
        ("DagFixture|dag\\.fixture", "*.json"),
    ]:
        try:
            out = subprocess.check_output(
                ["rg", "-l", pattern, "--glob", glob, "-g", "!.🧬semio/**", "-g", "!**/♻️mit-bestand/**"],
                cwd=ROOT,
                text=True,
            )
            paths.extend(ROOT / line.strip() for line in out.splitlines() if line.strip())
        except subprocess.CalledProcessError:
            pass
    return list(dict.fromkeys(paths))


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
    print(f"updated {changed} files for DagHostDocument")


if __name__ == "__main__":
    main()
