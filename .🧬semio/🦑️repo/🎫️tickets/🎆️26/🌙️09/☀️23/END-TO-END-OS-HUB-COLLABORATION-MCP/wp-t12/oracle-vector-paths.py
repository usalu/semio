#!/usr/bin/env python3
"""🧭️ Re-points Python case oracles that still read the retired `shared://🧬️mutations/<leaf>/🧪️tests/<vector>`
layout, or a vector directory whose emoji the relocation changed, at the vector each feature actually plans,
`shared://🧬️mutations/<leaf>/<emoji><vector>` (the owner's
`🧫️fixtures/🧬️mutations/<leaf>/<emoji><vector>` bundle). A vector is rewritten only when exactly one bundle on disk
carries that identity AND the case's own feature names it; anything else is reported and left alone.
Usage: oracle-vector-paths.py [--write]"""
import re
import subprocess
import sys
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
write = "--write" in sys.argv
retired = re.compile(r'\{_ROOT\}/([^/"{}]+)/(?:🧪️tests/)?([^/"{}]+)"')
adapters = subprocess.run(["git", "ls-files", "*/🧪️tests/*/🐍️.py"], cwd=root, capture_output=True, text=True, check=True).stdout.split()
for relative in adapters:
    adapter = root / relative
    source = adapter.read_text(encoding="utf-8")
    if not retired.search(source):
        continue
    case = adapter.parent
    subset = case.parent.parent
    feature = (case / "🥒️.feature").read_text(encoding="utf-8")
    problems, rewritten = [], 0

    def settle(match):
        global rewritten
        leaf, vector = match.group(1), re.sub(r"^[^a-z0-9]+", "", match.group(2))
        if "/🧪️tests/" not in match.group(0) and (subset / "🧫️fixtures/🧬️mutations" / leaf / match.group(2)).is_dir():
            return match.group(0)
        bundles = [entry.name for entry in (subset / "🧫️fixtures/🧬️mutations" / leaf).glob("*") if entry.is_dir() and re.sub(r"^[^a-z0-9]+", "", entry.name) == vector]
        if len(bundles) != 1:
            problems.append(f"{leaf}/{vector}: {len(bundles)} bundle(s)")
            return match.group(0)
        if f"{leaf}/{bundles[0]}" not in feature and not re.search(rf"\|\s*{re.escape(leaf)}\s*\|\s*{re.escape(bundles[0])}\s*\|", feature):
            problems.append(f"{leaf}/{bundles[0]}: not planned by the feature")
            return match.group(0)
        rewritten += 1
        return f'{{_ROOT}}/{leaf}/{bundles[0]}"'

    settled = retired.sub(settle, source)
    print(f"{relative.split('/🧪️tests/')[-1]}: {rewritten} rewritten, problems {problems}")
    if write and rewritten:
        adapter.write_text(settled, encoding="utf-8")
