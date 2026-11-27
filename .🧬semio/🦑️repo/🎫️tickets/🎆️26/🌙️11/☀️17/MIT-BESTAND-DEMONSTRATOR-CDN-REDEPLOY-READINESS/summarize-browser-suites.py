"""📊 Summarize bs-baseline.log and bs-behaviours.log by test title: python3 -I summarize-browser-suites.py"""
import re
from collections import Counter
from pathlib import Path

G = Path(__file__).resolve().parent / "🗑️generated"
line = re.compile(r"^\s+([✓✘-])\s+\d+ \[(\w+)\] › \S+ › (.*?)(?: \([\d.]+m?s\))?$")
for suite in ("baseline", "behaviours"):
    log = G / f"bs-{suite}.log"
    if not log.is_file():
        continue
    count = Counter()
    for text in log.read_text(encoding="utf-8", errors="replace").splitlines():
        found = line.match(text)
        if found:
            count[found.group(1)] += 1
            if found.group(1) != "✓":
                print(f"{suite}: {found.group(1)} [{found.group(2)}] {found.group(3)}")
    print(suite, dict(count))
