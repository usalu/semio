import math, json, re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_34_lib import sector
F = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/spiral-stair/🔣️.json"
T_ = 0.075
PLATE = 0.075
RISE = 0.186667


def outline(bands, col, plate_h, top, plate_r=0.2):
    ys = sorted({0, plate_h, top, *[b[0] for b in bands], *[b[1] for b in bands]})
    rows = []
    for i in range(len(ys) - 1):
        a, b = ys[i], ys[i + 1]
        m = (a + b) / 2
        lo, hi = (-plate_r, plate_r) if m < plate_h else (-col, col)
        for y0, y1, l, h in bands:
            if y0 <= m <= y1:
                lo, hi = min(lo, l), max(hi, h)
        rows.append((a, b, lo, hi))
    right = []
    for a, b, lo, hi in rows:
        right += [(hi, a), (hi, b)]
    left = []
    for a, b, lo, hi in rows:
        left += [(lo, a), (lo, b)]
    pts = right + left[::-1]
    out = []
    for p in pts:
        p = (round(p[0], 5), round(p[1], 5))
        if not out or out[-1] != p:
            out.append(p)
    ch = True
    while ch:
        ch = False
        for i in range(len(out)):
            a, b, c = out[i - 1], out[i], out[(i + 1) % len(out)]
            if (a[0] == b[0] == c[0]) or (a[1] == b[1] == c[1]):
                out.pop(i)
                ch = True
                break
    return out


def bands(kind):
    res = []
    for k in range(1, 16):
        P = sector(k, 6, r_in=0.0842)
        zs = [p[0] for p in P]
        xs = [p[1] for p in P]
        h = RISE * k
        if kind == "front":
            res.append((h - T_, h, min(zs), max(zs)))
        else:
            res.append((h - T_, h, -max(xs), -min(xs)))
    return res


def fmt_points(pts):
    L = ['              "points": [']
    for i, p in enumerate(pts):
        L.append("                [")
        L.append("                  %s," % json.dumps(p[0]))
        L.append("                  %s" % json.dumps(p[1]))
        L.append("                ]" + ("," if i < len(pts) - 1 else ""))
    L.append("              ],")
    return L


def main():
    t = open(F, encoding="utf8").read().split("\n")
    idxs = [i for i, l in enumerate(t) if l == '              "id": "stair-mass",']
    assert len(idxs) == 2, idxs
    new = {}
    fr = outline(bands("front"), 0.0842, PLATE, 2.8)
    el_h = outline(bands("elevation"), 0.0842, PLATE, 2.8)
    el = [(y, h) for (h, y) in el_h]
    for i, pts in zip(idxs, (el, fr)):
        s = next(j for j in range(i, i + 40) if t[j] == '              "points": [')
        e = next(j for j in range(s, s + 2000) if t[j].startswith("              ]"))
        new[s] = (e, fmt_points([[a, b] for a, b in pts]))
    for s in sorted(new, reverse=True):
        e, L = new[s]
        t[s:e + 1] = L
    txt = "\n".join(t)
    rep = {
        "The 15 treads (0.06 m thick, 0.13 m gaps = 0.27 mm) and the column are one filled mass listing treads and column; the notches between treads close because the bars are thinner than 0.2 mm.": "The 15 treads (0.06 m thick, drawn at the 0.075 m smallest feature = 0.15 mm, with 0.11 m gaps = 0.22 mm) and the column are one filled outline listing treads and column; the saw-tooth notches between the treads stay open in elevation and front alike, because the gaps are wider than 0.2 mm.",
        "Die 15 Stufen (0,06 m dick, 0,13 m Lücken = 0,27 mm) und die Säule sind eine gefüllte Masse mit den Regionen Stufen und Säule; die Einkerbungen zwischen den Stufen schließen sich, weil die Balken dünner als 0,2 mm wären.": "Die 15 Stufen (0,06 m dick, gezeichnet mit dem kleinsten Merkmal 0,075 m = 0,15 mm, Lücken 0,11 m = 0,22 mm) und die Säule sind eine gefüllte Kontur mit den Regionen Stufen und Säule; die Sägezahnkerben zwischen den Stufen bleiben in Seiten- und Vorderansicht offen, weil die Lücken breiter als 0,2 mm sind.",
        "One twisting mass: the envelope through the tread tips (x = R cos(phi), left and right) around the column, a base plate foot, and one helical handrail line above it.": "One saw-tooth outline: the column with a 0.075 m base plate foot and fifteen tread bars of 0.075 m whose projected lengths follow the tips (x = R cos(phi), left and right), and one helical handrail line above it.",
        "Eine sich windende Masse: die Hüllkurve durch die Stufenspitzen (x = R cos(phi), links und rechts) um die Säule, ein Fußplattensockel und darüber eine Wendelhandlauflinie.": "Eine Sägezahnkontur: die Säule mit 0,075 m dickem Fußplattensockel und fünfzehn Stufenbalken von 0,075 m, deren Projektionslängen den Stufenspitzen folgen (x = R cos(phi), links und rechts), und darüber eine Wendelhandlauflinie.",
    }
    for a, b in rep.items():
        assert txt.count(a) == 1, a[:40]
        txt = txt.replace(a, b)
    json.loads(txt)
    open(F, "w", encoding="utf8").write(txt)
    print("elevation", len(el), "front", len(fr))


main()
