"""📏️ Compares two outputs of compare.ts line by line: the exact IoU before and after and their difference; ends with the largest difference of the views both versions computed.
python3 diff.py <before.tsv> <after.tsv>"""
import sys

rows = [[line.rstrip("\n").split("\t") for line in open(p, encoding="utf-8")] for p in sys.argv[1:3]]
worst = 0.0
for a, b in zip(*rows):
    assert a[:3] == b[:3]
    x, y = (float(r[3].split()[1]) if r[3].startswith("exact") else None for r in (a, b))
    d = None if x is None or y is None else abs(x - y)
    worst = max(worst, d or 0.0)
    owned = b[3].split()[3] if y is not None else "-"
    print("| `%s` | 1:%s | %s | %s | %s | %s | %s |" % (a[0], a[1].split(":")[1], a[2], "throws" if x is None else "%.9f" % x, "throws" if y is None else "%.9f" % y, "-" if d is None else "%.1e" % d, owned))
print("largest difference %.3e" % worst)
