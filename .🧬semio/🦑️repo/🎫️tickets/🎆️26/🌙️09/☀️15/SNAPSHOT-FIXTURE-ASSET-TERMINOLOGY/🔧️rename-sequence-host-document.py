#!/usr/bin/env python3
"""🔧 Renames SequenceFixture host document symbols to SequenceHostDocument."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
SCOPE = ROOT / "✏️s/🔌️plugins/🎬️sequence"

REPLACEMENTS: list[tuple[str, str]] = [
    ("sequence_fixture_from_children", "sequence_host_document_from_children"),
    ("host_from_fixture", "host_from_document"),
    ("try_to_fixture", "try_to_host_document"),
    ("SequenceSnapshot::from_fixture", "SequenceSnapshot::from_host_document"),
    ("from_fixture", "from_host_document"),
    ("to_fixture", "to_host_document"),
    ("SequenceFixture", "SequenceHostDocument"),
    ("//#endregion 🔖️Fixture", "//#endregion 🔖️HostDocument"),
    ("//#region 🔖️Fixture", "//#region 🔖️HostDocument"),
    ("fixture: &SequenceHostDocument", "host_document: &SequenceHostDocument"),
    ("fixture: &SequenceSnapshot", "snapshot: &SequenceSnapshot"),
]


def sequence_files() -> list[Path]:
    paths: list[Path] = []
    for glob in ("*.rs", "*.json", "*.ts", "*.tsx"):
        try:
            out = subprocess.check_output(
                ["rg", "-l", "SequenceFixture|from_fixture|to_fixture|sequence\\.fixture", "--glob", glob, str(SCOPE)],
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
    changed = sum(1 for path in sequence_files() if rewrite(path))
    print(f"updated {changed} files under sequence plugin")


if __name__ == "__main__":
    main()
