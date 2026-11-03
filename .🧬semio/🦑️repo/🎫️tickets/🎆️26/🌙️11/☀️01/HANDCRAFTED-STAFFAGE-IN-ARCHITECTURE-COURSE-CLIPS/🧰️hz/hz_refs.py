"""🔎 List file:line references to each literal given on the command line across architecture clip scripts."""

import sys
from pathlib import Path

ARCH = Path("C:/git/semio/🎓️teaching/🏛️architecture")
needles = sys.argv[1:]
files = []
for p in ARCH.rglob("🐍️.py"):
    if ".venv" in p.parts or "node_modules" in p.parts:
        continue
    if "🎬️clip" not in p.parts and "🧪️tests" not in p.parts:
        continue
    files.append(p)
for needle in needles:
    print(f"## {needle}")
    for p in sorted(files):
        for ln, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
            if needle in line:
                print(f"{p.relative_to(ARCH).as_posix()}:{ln}: {line.strip()[:170]}")
