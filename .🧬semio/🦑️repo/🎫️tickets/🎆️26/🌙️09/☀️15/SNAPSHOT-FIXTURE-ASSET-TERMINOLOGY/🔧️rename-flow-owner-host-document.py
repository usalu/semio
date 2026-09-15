#!/usr/bin/env python3
"""🔧 Renames FlowOwner::Fixture and related flow diff variants to HostDocument."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

REPLACEMENTS: list[tuple[str, str]] = [
    ("FlowOwner::Fixture", "FlowOwner::HostDocument"),
    ("Self::Fixture(", "Self::HostDocument("),
    ("FlowOwner::HostDocument(value.fixture)", "FlowOwner::HostDocument(value.host_document)"),
    ("value.fixture)", "value.host_document)"),
    ("ReplaceFlowHostDocument(value) => self.frontier.push(FlowOwner::HostDocument(value.host_document))", "ReplaceFlowHostDocument(value) => self.frontier.push(FlowOwner::HostDocument(value.host_document))"),
]

ENUM_REPLACEMENTS: list[tuple[str, str]] = [
    ("Fixture(FlowHostDocument)", "HostDocument(FlowHostDocument)"),
    ("    Fixture,", "    HostDocument,"),
]


def flow_files() -> list[Path]:
    out = subprocess.check_output(
        ["rg", "-l", "FlowOwner::Fixture|FlowOwner::HostDocument|Self::Fixture", "--glob", "*.rs", "-g", "!.🧬semio/**"],
        cwd=ROOT,
        text=True,
    )
    return [ROOT / line.strip() for line in out.splitlines() if line.strip()]


def rewrite(path: Path) -> bool:
    text = path.read_text()
    original = text
    for old, new in REPLACEMENTS:
        text = text.replace(old, new)
    for old, new in ENUM_REPLACEMENTS:
        text = text.replace(old, new)
    if text != original:
        path.write_text(text)
        return True
    return False


def main() -> None:
    changed = sum(1 for path in flow_files() if rewrite(path))
    print(f"updated {changed} files for FlowOwner::HostDocument")


if __name__ == "__main__":
    main()
