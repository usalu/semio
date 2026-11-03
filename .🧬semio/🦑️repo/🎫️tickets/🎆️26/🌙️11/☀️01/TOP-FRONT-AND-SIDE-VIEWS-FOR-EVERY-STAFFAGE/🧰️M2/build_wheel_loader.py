import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️M2")
from lib import *

A = "wheel-loader"
doc = load(A)
PS, CI, EX, CH, WS, RU, GL = "paint-steel", "cast-iron", "exhaust-steel", "chrome", "wear-steel", "rubber", "glass"


def body_front(detail):
    s = [
        R("counterweight", "rear-frame", 2.05, 0.3, -1.1, 0.62, 2.2, 1.78, "counterweight", CI),
        R("hood", "rear-frame", 2, 0.45, -1.0, 0.62, 2.0, 1.78, "engine-bay", PS),
        R("exhaust-stack", "rear-frame", 2.1, 0.2, -0.99, 2.4, 0.18, 0.75, "exhaust-stack", EX),
        R("front-frame", "front-frame", 2, 0.45, -0.9, 0.62, 1.8, 0.58, "front-frame", PS),
        R("tower-l", "front-frame", 2, 0.45, 0.62, 1.2, 0.26, 1.0, "front-frame", PS),
        R("tower-r", "front-frame", 2, 0.45, -0.88, 1.2, 0.26, 1.0, "front-frame", PS),
        R("cab", "front-frame", 2.2, 0.55, -0.8, 1.45, 1.6, 1.95, "cab", PS),
        R("cab-glass-front", "front-frame", 2.4, 0.85, -0.65, 2.1, 1.3, 1.1, "cab-glazing", GL, role="glazing"),
    ]
    if detail:
        s += [R("head-lamp-l", "front-frame", 2.5, 0.9, 0.5, 0.95, 0.2, 0.2, "front-frame", GL, role="detail"),
              R("head-lamp-r", "front-frame", 2.5, 0.9, -0.7, 0.95, 0.2, 0.2, "front-frame", GL, role="detail"),
              R("mirror", "front-frame", 2.5, 0.2, -0.95, 3.0, 0.15, 0.25, "cab", PS, role="detail")]
    return s


def body_front_coarse():
    return [
        R("rear-body", "rear-frame", 2, 0.45, -1.1, 0.62, 2.2, 1.78, ["engine-bay", "counterweight"], PS),
        R("exhaust-stack", "rear-frame", 2.1, 0.2, -0.99, 2.4, 0.18, 0.75, "exhaust-stack", EX),
        R("front-body", "front-frame", 2, 0.45, -0.9, 0.62, 1.8, 0.58, ["front-frame"], PS),
        R("cab", "front-frame", 2.2, 0.55, -0.8, 1.45, 1.6, 1.95, "cab", PS),
        R("cab-glass", "front-frame", 2.4, 0.85, -0.65, 2.1, 1.3, 1.1, "cab-glazing", GL, role="glazing"),
    ]


def tyres_front(ids=("tyre-f-l", "tyre-f-r")):
    return [R(ids[0], "wheel-f", 4, 0.12, 0.8, 0, 0.6, 1.75, "tyres", RU, r=0.2),
            R(ids[1], "wheel-f", 4, 0.12, -1.4, 0, 0.6, 1.75, "tyres", RU, r=0.2)]


ARM = [(-0.249, 0.32), (1.001, 0.48), (2.001, 0.38), (2.851, 0.25), (2.851, -0.25), (2.001, -0.22), (1.001, -0.28), (-0.249, -0.32)]
BAR = lambda l, h: [(0, -h), (l, -h), (l, h), (0, h)]


def arms_front(w=0.3, z=5, tone=0.35, region="lift-arm"):
    out = []
    for side, c in (("l", 1.05), ("r", -1.05)):
        out.append(P(f"lift-arm-{side}", "lift-arm", z, tone, ARM, region, PS, solid={"kind": "extruded", "z0": n_(c - w / 2), "z1": n_(c + w / 2)}))
    return out


def cyl_front(rod=True, z=4.9, tone=0.5):
    out = []
    for side, c in (("l", 0.6), ("r", -0.6)):
        out.append(P(f"lift-cyl-{side}", "lift-cyl", z, tone, BAR(1.1, 0.12), "hydraulics", PS, solid={"kind": "extruded", "z0": n_(c - 0.12), "z1": n_(c + 0.12)}))
        if rod:
            out.append(P(f"lift-rod-{side}", "lift-rod", z + 0.05, 0.8, BAR(0.4574, 0.05), "hydraulics", CH, solid={"kind": "extruded", "z0": n_(c - 0.05), "z1": n_(c + 0.05)}))
    out.append(P("tilt-cyl", "tilt-cyl", z + 0.4, tone, BAR(0.9, 0.1), "hydraulics", PS, solid={"kind": "extruded", "z0": -0.1, "z1": 0.1}))
    if rod:
        out.append(P("tilt-rod", "tilt-rod", z + 0.45, 0.8, BAR(0.2689, 0.045), "hydraulics", CH, solid={"kind": "extruded", "z0": -0.045, "z1": 0.045}))
    return out


def bucket_front(edge):
    out = [P("bucket", "bucket", 6.2, 0.25, BUCKET, "bucket", WS, solid={"kind": "extruded", "z0": -1.475, "z1": 1.475})]
    if edge:
        out.append(P("bucket-edge", "bucket", 6.4, 0.1, LIP, "bucket", WS, role="detail", solid={"kind": "extruded", "z0": -1.475, "z1": 1.475}))
    return out


def body_plan(detail, coarse=False):
    if coarse:
        s = [R("rear-body", "rear-frame", 2, 0.45, -3.5, -1.1, 2.6, 2.2, ["engine-bay", "counterweight"], PS),
             R("front-body", "front-frame", 2, 0.45, -0.9, -0.9, 3.25, 1.8, ["front-frame"], PS),
             R("cab", "front-frame", 2.2, 0.55, -0.9, -0.8, 1.9, 1.6, "cab", PS),
             E("exhaust-stack", "rear-frame", 2.1, 0.2, -1.23, -0.9, 0.09, 0.09, "exhaust-stack", EX)]
        return s
    s = [R("counterweight", "rear-frame", 2.05, 0.3, -3.5, -1.1, 0.9, 2.2, "counterweight", CI),
         R("hood", "rear-frame", 2, 0.45, -3.0, -1.0, 2.1, 2.0, "engine-bay", PS),
         E("exhaust-stack", "rear-frame", 2.1, 0.2, -1.23, -0.9, 0.09, 0.09, "exhaust-stack", EX),
         R("front-frame", "front-frame", 2, 0.45, -0.9, -0.9, 3.25, 1.8, "front-frame", PS),
         R("cab", "front-frame", 2.2, 0.55, -0.9, -0.8, 1.9, 1.6, "cab", PS)]
    if detail:
        s += [P("handrail", "rear-frame", 2.3, 0.3, [(-2.9, -0.95), (-2.9, 0.95), (-1.4, 0.95), (-1.4, -0.95)], "engine-bay", PS, role="detail", filled=False),
              P("hood-louvres", "rear-frame", 2.3, 0.3, [(-2.3, 0.3), (-1.2, 0.3)], "engine-bay", PS, role="detail", filled=False, closed=False),
              P("hood-louvres-2", "rear-frame", 2.3, 0.3, [(-2.3, -0.3), (-1.2, -0.3)], "engine-bay", PS, role="detail", filled=False, closed=False),
              R("head-lamp-l", "front-frame", 2.5, 0.9, 2.3, 0.5, 0.05, 0.2, "front-frame", GL, role="detail"),
              R("head-lamp-r", "front-frame", 2.5, 0.9, 2.3, -0.7, 0.05, 0.2, "front-frame", GL, role="detail"),
              R("access-step", "front-frame", 1.9, 0.3, -0.9, -1.0, 0.4, 0.2, "front-frame", PS, role="detail")]
    return s


def tyres_plan():
    out = []
    for w, x in (("f", 1.7), ("r", -1.7)):
        for side, lo in (("l", 0.8), ("r", -1.4)):
            out.append(R(f"tyre-{w}-{side}", f"wheel-{w}", 1.5, 0.12, x - 0.875, lo, 1.75, 0.6, "tyres", RU, r=0.2))
    return out


def arms_plan(r=0.15):
    return arms_front(0.3, 3, 0.35, "lift-arm")


def cyl_plan(rod=True):
    return cyl_front(rod, 2.9, 0.5)


BUCKET = [(-0.449, -0.6), (1.501, -0.8), (1.621, -0.62), (1.051, 0.05), (0.151, 0.6), (-0.349, 0.55), (-0.549, 0)]
LIP = [(1.301, -0.77), (1.621, -0.62), (1.501, -0.8)]


def bucket_plan(n, lip=False):
    w = 1.475
    out = [P("bucket", "bucket", 3.2, 0.25, BUCKET, "bucket", WS, solid={"kind": "extruded", "z0": -w, "z1": w})]
    for i in range(1, n):
        c = -w + 2 * w * i / n
        out.append(P(f"bucket-rib-{i}", "bucket", 3.3, 0.25, BUCKET, "bucket", WS, role="detail", filled=False, penMm=0.18, solid={"kind": "extruded", "z0": n_(c - 0.01), "z1": n_(c + 0.01)}))
    if lip:
        out.append(P("bucket-edge", "bucket", 3.4, 0.1, LIP, "bucket", WS, role="detail", solid={"kind": "extruded", "z0": -w, "z1": w}))
    return out


def n_(v):
    return round(v, 4)


front50 = [*body_front(True), *tyres_front(), *arms_front(), *cyl_front(), *bucket_front(True)]
plan50 = [*body_plan(True), *tyres_plan(), *arms_plan(), *cyl_plan(), *bucket_plan(5, True)]
put(doc, 50, front50, plan50,
    "front: the bucket, a pair of lift arms, three cylinders, the cab with its glass, mirror and head lamps, the hood and counterweight behind and the two front tyres; each arm, cylinder and the bucket stretches between its pivot and its tip, so lifting and tilting move them. Top: hood with handrail and louvres, cab roof, tyres, the two arms and cylinders as stretching capsules and the bucket as a true top view, its outline extruded across the width with the cutting edge and, as unfilled detail strokes, four cross ribs; all follow lifting and tilting.",
    "vorn: die Schaufel, zwei Hubarme, drei Zylinder, die Kabine mit Glas, Spiegel und Scheinwerfern, dahinter Haube und Gegengewicht sowie die beiden Vorderreifen; jeder Arm, jeder Zylinder und die Schaufel strecken sich zwischen Drehpunkt und Spitze, sodass Heben und Kippen sie bewegen. Oben: Haube mit Handlauf und Lüftungsschlitzen, Kabinendach, Reifen, die beiden Arme und Zylinder als sich streckende Kapseln und die Schaufel als echte Draufsicht, seine über die Breite gezogene Kontur mit der Schneide und, als ungefüllte Detailstriche, vier Querrippen; alles folgt Heben und Kippen.")

front100 = [*body_front(False), *tyres_front(), *arms_front(), *cyl_front(), *bucket_front(True)]
plan100 = [*body_plan(True), *tyres_plan(), *arms_plan(), *cyl_plan(), *bucket_plan(4, True)]
put(doc, 100, front100, plan100,
    "as at 1:50 without lamps and mirror; the bucket from above is its outline with the cutting edge and three cross rib strokes.",
    "wie bei 1:50 ohne Leuchten und Spiegel; die Schaufel besteht von oben aus ihrer Kontur mit der Schneide und drei Querrippenstrichen.")

front200 = [*body_front(False), *tyres_front(), *arms_front(0.3), *cyl_front(False), *bucket_front(False)]
plan200 = [*body_plan(False), *tyres_plan(), *arms_plan(), *cyl_plan(False), *bucket_plan(3)]
put(doc, 200, front200, plan200,
    "main masses only: bucket, lift arms, cylinders, cab with glass, hood with counterweight and two tyres in front; hood, cab, tyres, arms, cylinders and the bucket outline with two cross rib strokes from above.",
    "nur Hauptmassen: Schaufel, Hubarme, Zylinder, Kabine mit Glas, Haube mit Gegengewicht und zwei Reifen von vorn; Haube, Kabine, Reifen, Arme, Zylinder und die Schaufelkontur mit zwei Querrippenstrichen von oben.")

front500 = [*body_front_coarse(), *tyres_front(), *arms_front(0.3, 5, 0.35, ["lift-arm", "hydraulics"]), *bucket_front(False)]
plan500 = [*body_plan(False, True), *tyres_plan(), *arms_front(0.3, 3, 0.35, ["lift-arm", "hydraulics"]), *bucket_plan(1)]
put(doc, 500, front500, plan500,
    "a composed silhouette: from the front the bucket, two arm strips, cab, bodies and the front tyres; from above bodies, cab, tyres, two arm capsules and the bucket outline.",
    "eine zusammengesetzte Silhouette: von vorn Schaufel, zwei Armleisten, Kabine, Rümpfe und die Vorderreifen; von oben Rümpfe, Kabine, Reifen, zwei Armkapseln und die Schaufelkontur.")
save(A, doc)
print("ok")
