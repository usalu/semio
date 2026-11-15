"""Author 28: root repair of the cyclist 2D elevation drawings. The thin unfilled hoops (tyres at 1:200 and 1:500) and the open fender arcs (1:100) are drawn as filled annuli / bands like at 1:50 and 1:100 (even-odd paths), because the catalog validator measures an unfilled stroke by the stripe of the body along the stroke normal, which runs along the fork or seat tube where the stroke crosses it (stroke width 0.45-0.76 m). Run: python3 author-28-2d-cyclist.py"""
import glob, json, math, re

P = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*humans/cyclist/🔣️.json")[0]
lines = open(P).read().split("\n")


def num(v):
    return f"{v:.5f}".rstrip("0").rstrip(".")


def circle(r):
    k = 0.5522847498 * r
    n = num
    return f"M {n(r)} 0 C {n(r)} {n(k)} {n(k)} {n(r)} 0 {n(r)} C {n(-k)} {n(r)} {n(-r)} {n(k)} {n(-r)} 0 C {n(-r)} {n(-k)} {n(-k)} {n(-r)} 0 {n(-r)} C {n(k)} {n(-r)} {n(r)} {n(-k)} {n(r)} 0 Z"


def band(d, c, gap):
    nums = [float(x) for x in re.findall(r"-?\d+\.?\d*", d)]
    pts = [(nums[i], nums[i + 1]) for i in range(0, len(nums), 2)]
    ends = [pts[0]] + [pts[i] for i in range(3, len(pts), 3)]
    inner = []
    for x, y in ends:
        r = math.hypot(x - c[0], y - c[1])
        k = (r - gap) / r
        inner.append((c[0] + (x - c[0]) * k, c[1] + (y - c[1]) * k))
    return "M " + " L ".join(f"{x:.4f} {y:.4f}" for x, y in ends + inner[::-1]) + " Z"


def section(scale):
    s = next(i for i, l in enumerate(lines) if l == f'      "{scale}": {{' and i > 1117)
    e = next(i for i in range(s, len(lines)) if lines[i] == '          "front": [')
    st = next(i for i in range(s, e) if lines[i] == '          "elevation": [')
    return st, e


def edit(scale, ident, fn):
    st, e = section(scale)
    i = next(i for i in range(st, e) if lines[i] == f'              "id": "{ident}",')
    j = next(k for k in range(i, i + 8) if lines[k].startswith('              "d": '))
    f = next(k for k in range(i, i + 12) if lines[k].startswith('              "filled": '))
    d = json.loads("{" + lines[j].strip().rstrip(",") + "}")["d"]
    lines[j] = '              "d": ' + json.dumps(fn(d)) + ","
    lines[f] = '              "filled": true,'


HALF = {"200": 0.018, "500": 0.0386}
for sc, h in HALF.items():
    for tid in ("tyre-rear", "tyre-front"):
        edit(sc, tid, lambda d, h=h: circle(0.34) + " " + circle(0.34 - 2 * h))
edit("100", "fender-rear", lambda d: band(d, (0.0, 0.34), 0.016) if "C" in d else d)
edit("100", "fender-front", lambda d: band(d, (1.05, 0.34), 0.016) if "C" in d else d)
open(P, "w").write("\n".join(lines))
print("written", P)
