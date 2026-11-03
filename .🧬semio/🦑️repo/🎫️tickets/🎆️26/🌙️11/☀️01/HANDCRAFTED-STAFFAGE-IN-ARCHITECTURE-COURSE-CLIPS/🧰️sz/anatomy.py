"""📊 Attribute and path-data byte shares of every exported cast frame, plus the savings each compaction would give.

@see ./compare.py — pixel comparison of two exports
"""

import re
import sys
from collections import Counter
from pathlib import Path

root = Path(sys.argv[1])
total = 0
frames = 0
largest = 0
attributes: Counter[str] = Counter()
saved: Counter[str] = Counter()
for path in root.glob("*/*.svg"):
    text = path.read_text(encoding="utf-8")
    size = path.stat().st_size
    total += size
    frames += 1
    largest = max(largest, size)
    for match in re.finditer(r'\s([\w:-]+)="([^"]*)"', text):
        attributes[match.group(1)] += len(match.group(0))
    saved["linecap-round"] += text.count(' stroke-linecap="round"')*len(' stroke-linecap="round"')
    saved["linejoin-round"] += text.count(' stroke-linejoin="round"')*len(' stroke-linejoin="round"')
    saved["fill-rule"] += text.count(' fill-rule="evenodd"')*len(' fill-rule="evenodd"')
    saved["path-id"] += sum(len(m.group(0)) for m in re.finditer(r'<path id="[^"]*"', text)) - 6*len(re.findall(r'<path id="', text))
    for d in re.findall(r' d="([^"]*)"', text):
        saved["implicit-L"] += d.count("L")
        saved["leading-zero"] += len(re.findall(r"(?<![\d.])-?0\.", d))
print("frames", frames, "bytes", total, "max", largest)
print([(k, v, round(100*v/total, 1)) for k, v in attributes.most_common(14)])
print(dict(saved))
