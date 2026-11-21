"""📐 Round 5: measures the girth (perimeter of the superellipse section), breadth and depth of hand-stated loft sections of a volumes source at given stations along the bone, for the model-vs-reference tables.
Usage: python3 r5-girth.py <volumes json> <scale> <solid id>:<s> [...]"""
import json, math, sys

def section(solid, s):
    secs = solid["sections"]
    for a, b in zip(secs, secs[1:]):
        if a["at"][0] <= s <= b["at"][0] or a["at"][0] >= s >= b["at"][0]:
            t = 0 if b["at"][0] == a["at"][0] else (s - a["at"][0]) / (b["at"][0] - a["at"][0])
            return [a[k] + (b[k] - a[k]) * t for k in ("rx", "ry", "exponent")]
    raise SystemExit(f"{solid['id']}: s {s} outside")

def perimeter(rx, ry, e, n=720):
    pts = []
    for i in range(n):
        a = 2 * math.pi * i / n
        c, d = math.cos(a), math.sin(a)
        pts.append((rx * math.copysign(abs(c) ** (2 / e), c), ry * math.copysign(abs(d) ** (2 / e), d)))
    return sum(math.dist(pts[i], pts[(i + 1) % n]) for i in range(n))

vol = json.load(open(sys.argv[1]))["volumes"][sys.argv[2]]["body"]
by = {s["id"]: s for s in vol}
for arg in sys.argv[3:]:
    sid, s = arg.rsplit(":", 1)
    rx, ry, e = section(by[sid], float(s))
    print(f"{sid} s={s}: breadth {2*rx:.3f} depth {2*ry:.3f} girth {perimeter(rx, ry, e):.3f}")
