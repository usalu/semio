"""📊 Tally strict findings by kind over the layout reports of both capture tickets (en and de, current frames only).

Usage: python tally-strict.py [kind-prefix-to-list]
"""
import re
import sys
from collections import Counter
from pathlib import Path

DAY = Path(__file__).resolve().parent.parent
LIST = sys.argv[1] if len(sys.argv) > 1 else ""
KINDS = ("outside frame", "near frame edge", "unreadable text", "in caption band", "text over text", "text crosses chip", "stroke through chip",
         "chips overlap", "stroke through", "filled shape under", "caption has")


def kind(line):
    return next((k for k in KINDS if line.startswith(k)), "other")


for ticket in ("CONCRETE-OPTIMIZATION-CLIP-EXAMPLES", "CONCRETE-BIM-CLIP-EXAMPLES"):
    for report in sorted((DAY / ticket / "🗑️generated/frames").glob("*/*-layout.md")):
        if "until" in report.name:
            continue
        text = report.read_text(encoding="utf-8")
        if "### strict" not in text:
            continue
        frames = re.split(r"^## ", text, flags=re.M)[1:]
        counts, shared, empty = Counter(), 0, 0
        for frame in frames:
            head, _, strict = frame.partition("### strict")
            shared += sum(1 for l in head.splitlines() if l.startswith("- ") and l != "- none")
            lines = [l[2:] for l in strict.splitlines() if l.startswith("- ") and l != "- none"]
            empty += not lines
            for line in lines:
                counts[kind(line)] += 1
                if LIST and kind(line) == LIST:
                    print(f"  [{report.name} {frame.splitlines()[0]}] {line[:230]}")
        print(f"{report.name}: frames={len(frames)} clean={empty} shared={shared} strict={sum(counts.values())} {dict(counts)}")
