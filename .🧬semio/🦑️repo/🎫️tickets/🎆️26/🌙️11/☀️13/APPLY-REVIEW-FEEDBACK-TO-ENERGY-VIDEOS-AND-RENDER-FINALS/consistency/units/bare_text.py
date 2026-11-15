import re
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
SKIP = {".venv", "site-packages", "__pycache__", "🔊️speech", "🧪️tests"}
PAT = re.compile(r"\d[\s\u00a0]*[×·*][\s\u00a0]*[\d−\-]")


def main():
    for p in ROOT.rglob("*"):
        if not p.is_file() or p.suffix not in {".json", ".md"} or set(p.parts) & SKIP:
            continue
        for i, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
            if PAT.search(line):
                print(f"{p.relative_to(ROOT)}:{i}: {line.strip()[:300]}")


main()
