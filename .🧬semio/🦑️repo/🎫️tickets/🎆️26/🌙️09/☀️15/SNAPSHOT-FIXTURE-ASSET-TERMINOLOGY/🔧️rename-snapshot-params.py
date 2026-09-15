#!/usr/bin/env python3
"""🔧 Renames Rust parameters `fixture: &*Snapshot` to `snapshot` and updates bare identifier uses."""
from __future__ import annotations

import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
PARAM = re.compile(r"\bfixture:\s*&(\w+Snapshot)\b")
SKIP_SUBSTR = (
    "FlowHostDocument",
    "FlowFixture",
    "to_host_document",
    "from_host_document",
    "to_fixture",
    "from_fixture",
    "set_fixture",
    "graph_from_fixture",
    "objects_from_fixture",
    "dag_document_from_fixture",
    "fixture_",
    "_fixture",
    "ReplaceFlow",
)


def list_files() -> list[Path]:
    out = subprocess.check_output(
        [
            "rg",
            r"fixture: &\w*Snapshot",
            "--glob",
            "*.rs",
            "-g",
            "!.🧬semio/**",
            "-g",
            "!**/♻️mit-bestand/**",
            "-l",
        ],
        cwd=ROOT,
        text=True,
    )
    return [ROOT / line.strip() for line in out.splitlines() if line.strip()]


def touch_identifier(line: str) -> bool:
    if any(s in line for s in SKIP_SUBSTR):
        return False
    return bool(re.search(r"(?<![\w])fixture(?![\w])", line))


def rewrite_file(path: Path) -> bool:
    text = path.read_text()
    if not PARAM.search(text):
        return False
    text = PARAM.sub(r"snapshot: &\1", text)
    lines = text.splitlines(keepends=True)
    new_lines: list[str] = []
    for line in lines:
        if touch_identifier(line):
            line = re.sub(r"(?<![\w])fixture(?![\w])", "snapshot", line)
        new_lines.append(line)
    new_text = "".join(new_lines)
    if new_text != path.read_text():
        path.write_text(new_text)
        return True
    return False


def main() -> None:
    changed = [p for p in list_files() if rewrite_file(p)]
    print(f"updated {len(changed)} files")
    for p in changed:
        print(p.relative_to(ROOT))


if __name__ == "__main__":
    main()
