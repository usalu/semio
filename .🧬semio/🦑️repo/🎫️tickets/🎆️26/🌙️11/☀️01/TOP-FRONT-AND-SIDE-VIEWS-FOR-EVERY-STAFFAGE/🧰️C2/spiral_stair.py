from lib import *

A = "spiral-stair"
doc = load(A)
COL = "column"
WALK_R = 0.6


def turn_bone():
    return {"id": "walker-turn", "parent": COL, "title": {"en": "Walker azimuth round the column (massless)", "de": "Personenazimut um die Säule (masselos)"}, "x": 0.0, "y": 0.0, "angleDeg": 0, "lengthM": 0.0,
            "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 360}}


def azimuth_offsets(theta, dy):
    return {"walker-turn": {"twistDeg": round(theta, 5)}, "walker-x": {"dy": -WALK_R}, "walker-y": {"dy": round(dy, 5)}}


def patch_walker(asset):
    bones = asset["rig"]["bones"]
    i = next(k for k, b in enumerate(bones) if b["id"] == "walker-x")
    bones.insert(i, turn_bone())
    wx = next(b for b in bones if b["id"] == "walker-x")
    wx["parent"] = "walker-turn"
    wx["joint"] = {"type": "prismatic", "axisDeg": 90, "minM": -0.7, "maxM": 0.7}
    for p in asset["poses"]:
        if p["id"] == "walker-on-tread-8":
            p["bones"] = azimuth_offsets(168.75, 1.49333)
        if p["id"] == "point-load-top-tread":
            p["bones"] = azimuth_offsets(326.25, 2.8)
    for c in asset["clips"]:
        if c["id"] == "climb":
            for k, key in enumerate(c["keys"]):
                key["bones"] = azimuth_offsets(11.25 + 22.5 * k, RISE * (k + 1))
RISE = 0.186667
RI, RO, RB = 0.0842, 0.8, 0.775
STEP = 22.5
BAL_H = 0.933333
PIT = {}


def el(sc, id):
    for s in doc["asset"]["representations"][str(sc)]["geometry"]["elevation"]:
        if s["id"] == id:
            return s


def sector(i, n=4):
    a, b = STEP * (i - 1), STEP * i
    outer = circ(0, 0, RO, a, b, n)
    inner = circ(0, 0, RI, b, a, 1)
    return outer + inner


def rng(pts, k):
    v = [p[k] for p in pts]
    return min(v), max(v)


def th(i):
    return math.radians(11.25 + STEP * (i - 1))


def hr_pts(proj):
    pts = []
    n = 60
    for k in range(n + 1):
        t = 11.25 + (326.25 - 11.25) * k / n
        h = 1.12 + (t - 11.25) / STEP * RISE
        a = math.radians(t)
        pts.append((RB * math.sin(a), h) if proj == "front" else (RB * math.cos(a), RB * math.sin(a)))
    return pts


def walk_pts(a0=8, a1=330, r=0.6):
    return circ(0, 0, r, a0, a1, 40)


def arrow_head(a1=330, r=0.6):
    a = math.radians(a1)
    e = (r * math.cos(a), r * math.sin(a))
    t = (-math.sin(a), math.cos(a))
    n = (math.cos(a), math.sin(a))
    return [(e[0] - 0.12 * t[0] + 0.05 * n[0], e[1] - 0.12 * t[1] + 0.05 * n[1]), e, (e[0] - 0.12 * t[0] - 0.05 * n[0], e[1] - 0.12 * t[1] - 0.05 * n[1])]


def front(sc):
    hpen = el(sc, "handrail")["penMm"]
    if sc == 500:
        right, left = [(RI, 0.02)], [(-RI, 0.02)]
        rr, ll = [], []
        for i in range(1, 16):
            lo, hi = rng(sector(i, 8), 1)
            h = RISE * i
            rr += [(RI, h - 0.06), (max(hi, RI), h - 0.06), (max(hi, RI), h), (RI, h)]
            ll += [(-RI, h - 0.06), (min(lo, -RI), h - 0.06), (min(lo, -RI), h), (-RI, h)]
        pts = [(-0.2, 0), (0.2, 0), (0.2, 0.02)] + rr + [(RI, 2.8)] + [(-RI, 2.8)] + ll[::-1] + [(-0.2, 0.02)]
        pts = [(-0.2, 0), (0.2, 0), (0.2, 0.02), (RI, 0.02)] + rr + [(-RI, 2.8)] + ll[::-1] + [(-RI, 0.02), (-0.2, 0.02)]
        return [P("stair-mass", COL, "body", 1, pts, tone=0.55, regions=["treads", "column-steel"], material="steel"),
                P("handrail", "balustrade", "body", 3, hr_pts("front"), closed=False, filled=False, tone=0.3, region="balustrade-steel", material="steel", pen=hpen),
                P("ground", COL, "ground", 0, [(-0.8, 0), (0.8, 0)], closed=False, filled=False, tone=0.5)]
    s = [P("ground", COL, "ground", 0, [(-0.8, 0), (0.8, 0)], closed=False, filled=False, tone=0.5),
         R("column", COL, "body", 5, -RI, 0.02, 2 * RI, 2.78, tone=0.5, region="column-steel", material="steel"),
         R("base-plate", COL, "body", 1, -0.2, 0, 0.4, 0.02, tone=0.5, region="column-steel", material="steel")]
    for i in range(1, 16):
        lo, hi = rng(sector(i, 8), 1)
        h = RISE * i
        zc = 5 + math.cos(math.radians(STEP * (i - 0.5)))
        pen = el(sc, "tread-%d" % i)["penMm"]
        s.append(R("tread-%d" % i, "tread-%d" % i, "body", zc, lo, h - 0.06, hi - lo, 0.06, tone=0.5, region="treads", material="steel", pen=pen))
        if sc == 50:
            s.append(line("nosing-%d" % i, "tread-%d" % i, "texture", zc + 0.01, [(lo, h - 0.012), (hi, h - 0.012)], tone=0.2, region="treads", pen=0.1))
            s.append(line("sleeve-%d" % i, COL, "texture", 5.5, [(-RI, h - 0.06), (RI, h - 0.06)], tone=0.2, region="column-steel", pen=0.1))
        if sc in (100, 50):
            a = th(i)
            bp = el(sc, "baluster-%d" % i)["penMm"]
            s.append(line("baluster-%d" % i, "balustrade", "detail", 5 + math.cos(a) + 0.1, [(RB * math.sin(a), h), (RB * math.sin(a), h + BAL_H)], region="balustrade-steel", material="steel", pen=bp))
    s.append(P("handrail", "balustrade", "detail", 8, hr_pts("front"), closed=False, filled=False, tone=0.3, region="balustrade-steel", material="steel", pen=hpen))
    return s


def plan(sc):
    hpen = el(sc, "handrail")["penMm"]
    s = []
    if sc != 500:
        s.append(E("base-plate", COL, "body", 0.5, 0, 0, 0.2, tone=0.5, region="column-steel", material="steel"))
    for i in range(1, 16):
        pen = None if sc == 500 else el(sc, "tread-%d" % i)["penMm"]
        s.append(P("tread-%d" % i, "tread-%d" % i, "body", 2, sector(i, 6), tone=0.5, region="treads", material="steel", pen=pen))
    s.append(E("column", COL, "body", 3, 0, 0, RI, tone=0.5, region="column-steel", material="steel"))
    s.append(P("handrail", "balustrade", "body" if sc == 500 else "detail", 6, circ(0, 0, RB, 11.25, 326.25, 40), closed=False, filled=False, tone=0.3, region="balustrade-steel", material="steel", pen=hpen))
    if sc == 500:
        return s
    if sc in (100, 50):
        for i in range(1, 16):
            a = th(i)
            s.append(E("baluster-%d" % i, "balustrade", "detail", 6.5, RB * math.cos(a), RB * math.sin(a), 0.014, tone=0.3, region="balustrade-steel", material="steel"))
    s.append(P("walking-line", COL, "detail", 4, walk_pts(), closed=False, filled=False, tone=0.2, region="treads"))
    s.append(P("walking-arrow", COL, "detail", 4, arrow_head(), closed=False, filled=False, tone=0.2, region="treads"))
    s.append(E("walking-start", COL, "detail", 4, 0.6 * math.cos(math.radians(8)), 0.6 * math.sin(math.radians(8)), 0.04, tone=0.2, region="treads"))
    if sc in (200, 100):
        for i in range(1, 16):
            a = math.radians(STEP * (i - 1))
            s.append(P("nosing-%d" % i, "tread-%d" % i, "detail", 2.5, [(RI * math.cos(a), RI * math.sin(a)), (RO * math.cos(a), RO * math.sin(a))], closed=False, filled=False, tone=0.5, region="treads"))
    if sc == 50:
        s.append(E("column-bore", COL, "detail", 3.5, 0, 0, 0.0779, tone=0.2, region="column-steel", filled=False))
        for i in range(1, 16):
            a = math.radians(STEP * (i - 1))
            s.append(P("nosing-%d" % i, "tread-%d" % i, "texture", 2.5, [(RI * math.cos(a), RI * math.sin(a)), (RO * math.cos(a), RO * math.sin(a))], closed=False, filled=False, tone=0.2, region="treads", pen=0.1))
    return s


EN = {
    500: ("Front and plan: the front is the stair as a saw-tooth silhouette (each tread sticks out sideways where it turns), the plan is the fifteen pie treads around the column with the handrail circle; the free sector is the landing opening.",
          "Vorder- und Draufsicht: Die Vorderansicht ist die Treppe als Sägezahnsilhouette (jede Stufe ragt dort seitlich heraus, wo sie sich dreht), die Draufsicht sind die fünfzehn Kreisringstufen um die Säule mit dem Handlaufkreis; der freie Sektor ist die Deckenöffnung."),
    200: ("Front and plan: the front shows the fifteen treads at their true heights with the column and the helical handrail projected sideways; the plan shows the fifteen pie treads of 22.5 degrees, the column, the handrail circle and the walking line at 0.6 m radius with arrow upward (DIN 1356-1).",
          "Vorder- und Draufsicht: Die Vorderansicht zeigt die fünfzehn Stufen in wahren Höhen mit der Säule und dem Wendelhandlauf seitlich projiziert; die Draufsicht zeigt die fünfzehn Kreisringstufen zu je 22,5 Grad, die Säule, den Handlaufkreis und die Lauflinie bei 0,6 m Radius mit Pfeil nach oben (DIN 1356-1)."),
    100: ("Front and plan: the front adds the 15 balusters, drawn from the tread to 0.93 m above it, in front of or behind the column according to their turn; the plan adds the base plate under the open sector, the 15 baluster tubes on the handrail circle and the walking line with start dot.",
          "Vorder- und Draufsicht: Die Vorderansicht ergänzt die 15 Stäbe von der Stufe bis 0,93 m darüber, je nach Drehung vor oder hinter der Säule; die Draufsicht ergänzt die Fußplatte unter dem freien Sektor, die 15 Stabrohre auf dem Handlaufkreis und die Lauflinie mit Startpunkt."),
    50: ("Front and plan: the front adds nosing lines on every tread and the column sleeves at the underside of each tread; the plan adds the column bore and a radial nosing line on each tread, so the 15 steps count from above.",
         "Vorder- und Draufsicht: Die Vorderansicht ergänzt Trittkantenlinien auf jeder Stufe und die Säulenhülsen an der Unterseite jeder Stufe; die Draufsicht ergänzt die Säulenbohrung und eine radiale Trittkantenlinie auf jeder Stufe, sodass man die 15 Stufen von oben zählen kann."),
}
patch_walker(doc["asset"])
for sc in (500, 200, 100, 50):
    install(A, sc, front(sc), plan(sc), doc, *EN[sc])
finish(A, doc)
print("ok", A)
