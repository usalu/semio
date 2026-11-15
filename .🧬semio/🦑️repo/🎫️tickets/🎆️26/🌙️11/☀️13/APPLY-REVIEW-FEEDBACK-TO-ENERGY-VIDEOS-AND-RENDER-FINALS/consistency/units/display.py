import re
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
SKIP = {".venv", "site-packages", "__pycache__", ".ruff_cache", "🔊️speech"}
UNIT = re.compile(r"\\mathrm\{\{?[^{}]*?/[^{}]*?\}\}?(?:\^\{?\d\}?)?")


def candidates():
    for p in ROOT.rglob("*"):
        if not p.is_file() or p.suffix not in {".py", ".json", ".md"} or set(p.parts) & SKIP:
            continue
        try:
            lines = p.read_text(encoding="utf-8").splitlines()
        except Exception:
            continue
        for i, line in enumerate(lines, 1):
            for m in re.finditer(r"\\mathrm\{(?:[^{}]|\{[^{}]*\})*\}", line):
                if "/" not in m.group(0):
                    continue
                if line[: m.start()].count("$") % 2 == 1:
                    continue
                yield p, i, line, m


def main():
    for p, i, line, m in candidates():
        print(f"{p.relative_to(ROOT)}:{i}: {m.group(0)} || {line.strip()[:200]}")


main()
