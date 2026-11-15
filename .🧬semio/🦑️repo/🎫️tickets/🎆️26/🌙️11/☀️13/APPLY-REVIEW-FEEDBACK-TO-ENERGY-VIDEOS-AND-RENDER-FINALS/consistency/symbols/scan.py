"""🔎 Scan the energy tree for regex patterns and print file:line:text."""
import os
import re
import sys

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy"
SKIP = {".venv", "site-packages", "__pycache__", ".ruff_cache", "node_modules"}
EXT = {".py", ".md", ".json", ".ts", ".tex", ".txt"}


def files():
    for d, ds, fs in os.walk(ROOT):
        ds[:] = [x for x in ds if x not in SKIP]
        for f in fs:
            if os.path.splitext(f)[1] in EXT:
                yield os.path.join(d, f)


def main():
    with open(sys.argv[1], encoding="utf-8") as h:
        pats = [re.compile(p) for p in h.read().split("\n") if p]
    out = []
    for path in files():
        try:
            with open(path, encoding="utf-8", newline="") as h:
                lines = h.read().split("\n")
        except Exception:
            continue
        rel = os.path.relpath(path, ROOT)
        for n, line in enumerate(lines, 1):
            for p in pats:
                if p.search(line):
                    out.append(f"{rel}:{n}: {line.strip()[:240]}")
                    break
    sys.stdout.reconfigure(encoding="utf-8")
    print("\n".join(out))
    print(f"-- {len(out)} lines", file=sys.stderr)


main()
