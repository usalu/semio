"""Author 61: set areaM2 of one thermal region of a catalog asset (text-level). Usage: author-61-area.py <slug> <region> <old> <new>"""
import re, sys
slug, region, old, new = sys.argv[1:5]
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/%s/🔣️.json" % slug
t = open(P).read()
i = t.index('"thermal"'); j = t.index('"id": "%s"' % region, i)
m = re.compile(r'"areaM2":\s*([0-9.]+)').search(t, j)
assert m and float(m.group(1)) == float(old), (m.group(1) if m else None)
t = t[:m.start(1)] + new + t[m.end(1):]
open(P, "w").write(t)
print(region, old, "->", new)
