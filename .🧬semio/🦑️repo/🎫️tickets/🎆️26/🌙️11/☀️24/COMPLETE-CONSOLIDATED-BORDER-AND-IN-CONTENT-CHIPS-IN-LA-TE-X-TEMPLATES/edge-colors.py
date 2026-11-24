"""🎨 Audits edge colours in PDFs: every horizontal rule that spans a frame (at least
`min_width` pt) must be drawn in the neutral border colour, the most common colour
among those rules. Prints offending rules and a per-document verdict.

usage: python edge-colors.py <min_width> <pdf> [<pdf> ...]
"""
import sys
from collections import Counter
import pymupdf

minimum = float(sys.argv[1])
failed = False
for path in sys.argv[2:]:
    rules = []
    for number, page in enumerate(pymupdf.open(path), 1):
        for drawing in page.get_drawings():
            box, color = drawing["rect"], drawing.get("fill") or drawing.get("color")
            if color and box.height < 1.5 and box.width >= minimum:
                rules.append((number, round(box.y0, 2), round(box.x0, 2), round(box.width, 2), tuple(round(c, 3) for c in color)))
    if not rules:
        print(path, "no rules")
        continue
    neutral = Counter(rule[4] for rule in rules).most_common(1)[0][0]
    canvas = (1.0, 1.0, 1.0)
    odd = [rule for rule in rules if rule[4] not in (neutral, canvas)]
    failed |= bool(odd)
    print(path, f"{len(rules)} frame-wide horizontal rules, neutral {neutral}, non-neutral {len(odd)}")
    for rule in odd[:20]:
        print("  page", *rule)
sys.exit(1 if failed else 0)
