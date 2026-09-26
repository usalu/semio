#!/usr/bin/env python3
"""✉️ LD item 2 (TS half): adds `observed: null, target: []` right after `dependencies` in every snake_case wire
envelope object literal (`mutation_id: …, dependencies: …, diff: …`) of the TS tree. One-off, ticket-local.

usage: ts-envelope-fields.py [--dry-run]
"""
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SKIP = {"🧰️framework/🔨️modules/📡️replication/🟦️.ts"}
PATTERN = re.compile(r"(dependencies: [^\n]*?,)(\s*)(diff: )")


def main() -> int:
    dry = "--dry-run" in sys.argv
    files = subprocess.run(["git", "grep", "-l", "mutation_id", "--", "*.ts", "*.tsx", ":!.tmp-ticket", ":!.🧬semio", ":!**/node_modules/**"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.split("\n")
    total = 0
    for name in filter(None, files):
        if name in SKIP:
            continue
        path = ROOT / name
        text = path.read_text(encoding="utf-8")
        edits = []
        for match in PATTERN.finditer(text):
            window = text[max(0, match.start() - 400):match.start()]
            if "mutation_id" not in window or "observed" in text[match.start():match.end() + 40]:
                continue
            edits.append(match)
        if not edits:
            continue
        for match in reversed(edits):
            separator = match.group(2)
            insertion = f"{match.group(1)}{separator}observed: null,{separator}target: [],{separator}{match.group(3)}"
            text = text[:match.start()] + insertion + text[match.end():]
        total += len(edits)
        print(f"{len(edits):3} {name}")
        if not dry:
            path.write_text(text, encoding="utf-8")
    print(f"total {total}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
