import re
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
PAT = re.compile(r"\\mathrm\{[^{}]*(?:\{[^{}]*\}[^{}]*)*\}")
SKIP = {".venv", "site-packages", "__pycache__", ".ruff_cache"}
EXT = {".py", ".json", ".md"}


def files():
    for p in ROOT.rglob("*"):
        if p.is_file() and p.suffix in EXT and not (set(p.parts) & SKIP):
            yield p


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "slash"
    for p in files():
        rel = p.relative_to(ROOT)
        try:
            lines = p.read_text(encoding="utf-8").splitlines()
        except Exception:
            continue
        for i, line in enumerate(lines, 1):
            if mode == "slash":
                hits = [m.group(0) for m in PAT.finditer(line) if "/" in m.group(0)]
                if hits:
                    print(f"{rel}:{i}: {hits}")
            elif mode == "brk":
                if re.search(r"\\left\[|\\,\[\\mathrm|\[(?:W|kWh|K|m³/h|W/K|W/\(m²·?K\))\]", line):
                    print(f"{rel}:{i}: {line.strip()[:160]}")
            elif mode == "glue":
                if re.search(r"\d\\mathrm\{|\\mathrm\{W\}\d", line):
                    print(f"{rel}:{i}: {line.strip()[:160]}")


main()
