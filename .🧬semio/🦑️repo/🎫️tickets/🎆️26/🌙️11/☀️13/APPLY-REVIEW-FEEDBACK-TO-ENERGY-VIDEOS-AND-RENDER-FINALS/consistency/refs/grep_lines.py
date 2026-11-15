import re
import sys
from pathlib import Path

ENERGY = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
pattern = re.compile(sys.argv[1])
suffixes = sys.argv[2].split(",") if len(sys.argv) > 2 else [".py"]
skip = {".venv", "__pycache__", "site-packages", "📚️references", "🗑️generated", "🔊️speech"}
for path in sorted(ENERGY.rglob("*")):
    if not path.is_file() or path.suffix not in suffixes or skip & set(path.parts):
        continue
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if pattern.search(line):
            print(f"{path.relative_to(ENERGY).as_posix()}:{number}: {line.strip()[:400]}")
