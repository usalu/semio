#!/usr/bin/env python3
"""🩹 Applies literal edits to LaTeX packages from a JSON plan, retrying while a concurrent
tectonic run holds the file open.

Plan shape: {"<file.sty>": [{"old": "...", "new": "...", "count": 1}, ...]}
`old` is matched literally (never as a regular expression, so backslashes survive), and the
expected occurrence count is checked before anything is written. An entry with `"old": null`
and an `"after"` anchor inserts `new` on the line following the anchor line.
Usage: python 🔧️polish-apply.py <latex-dir> <plan.json>
"""
import json
import sys
import time
from pathlib import Path


def write_retry(path: Path, text: str) -> None:
    for attempt in range(240):
        try:
            path.write_text(text, encoding="utf-8", newline="\n")
            return
        except OSError:
            time.sleep(0.5)
    raise SystemExit(f"{path.name}: still locked after two minutes")


def main() -> None:
    root = Path(sys.argv[1])
    plan = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
    for name, edits in plan.items():
        path = root / name
        text = path.read_text(encoding="utf-8")
        for edit in edits:
            old = edit["old"]
            want = edit.get("count", 1)
            found = text.count(old)
            if found != want:
                raise SystemExit(f"{name}: expected {want} occurrence(s), found {found}: {old[:90]!r}")
            text = text.replace(old, edit["new"])
        write_retry(path, text)
        print(f"{name}: {len(edits)} edits")


main()
