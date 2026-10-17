"""📉 Compare two tsc outputs by file: counts before and after, with new files and per-code deltas."""
import collections, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt3_blocks import blocks

a = blocks(sys.argv[1]); b = blocks(sys.argv[2])
only_after = len(sys.argv) > 3 and sys.argv[3] == "increases"
ca = collections.Counter(x["file"] for x in a); cb = collections.Counter(x["file"] for x in b)
print("before", len(a), "after", len(b))
rows = []
for f in set(ca) | set(cb):
    if only_after and cb.get(f, 0) <= ca.get(f, 0): continue
    rows.append((cb.get(f, 0) - ca.get(f, 0), ca.get(f, 0), cb.get(f, 0), f))
for d, x, y, f in sorted(rows, reverse=True)[:60]:
    print(f"{d:+d} {x}->{y} {f.replace('🧰️framework/', 'FW/')[-120:]}")
