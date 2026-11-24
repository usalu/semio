"""🧮️ Summary of a matrix output folder: `python3 cells.py <dir>` prints the cells by status, and per failing check the cells by scale entry and view with the worst delta and its subject."""
import json, sys, collections
cells = json.load(open(sys.argv[1] + "/matrix.json"))["cells"]
print("status", dict(collections.Counter(c["status"] for c in cells)))
by = collections.defaultdict(lambda: collections.defaultdict(list))
for c in cells:
    for check, f in (c.get("failures") or {}).items():
        by[check][(c["scale"], c["view"])].append((c["mode"], f))
for check, groups in sorted(by.items()):
    print(check, sum(len(v) for v in groups.values()), "cells")
    for (scale, view), items in sorted(groups.items()):
        mode, f = items[0]
        worst = f.get("worst") or {}
        first = (f.get("first") or [{}])[0]
        print(f"   {scale:>14} {view:<9} ×{len(items)} modes n={f.get('count')} worst {worst.get('deltaMm', 0):.0f} mm {worst.get('pair', '')} @ {worst.get('subject', first.get('subject', ''))} :: {first.get('message', '')[:150]}")
