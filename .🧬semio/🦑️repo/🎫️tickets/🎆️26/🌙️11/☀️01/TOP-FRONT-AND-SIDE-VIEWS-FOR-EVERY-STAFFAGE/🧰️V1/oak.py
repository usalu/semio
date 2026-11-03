import math as m
from kit import *

FRONT_LOBES = [
    ("core", "fol-core", 0.0, 9.4, 3.0, 2.87, 9, 0.08, 1, True),
    ("left-low", "fol-w-low", 4.9, 6.7, 2.6, 2.4, 8, 0.14, 2, False),
    ("right-low", "fol-e-low", -4.95, 7.5, 2.55, 2.4, 8, 0.14, 3, False),
    ("left-mid", "fol-w-mid", 4.2, 11.4, 3.0, 2.65, 8, 0.14, 4, False),
    ("right-mid", "fol-e-mid", -4.1, 12.1, 3.1, 2.65, 8, 0.14, 5, False),
    ("leader-left", "fol-leader-w", 1.9, 14.8, 2.7, 2.4, 7, 0.12, 6, False),
    ("leader-right", "fol-leader-e", -1.9, 14.9, 2.6, 2.45, 7, 0.12, 7, False),
    ("top", "fol-top", 0.0, 16.25, 2.7, 1.97, 7, 0.12, 8, False),
]
PLAN_LOBES = [
    ("core", "fol-core", 0.0, 0.0, 3.4, 3.3, 10, 0.08, 11, True),
    ("back", "fol-w-low", -5.76, 0.4, 3.34, 3.0, 9, 0.14, 12, False),
    ("front", "fol-e-low", 5.79, -0.4, 3.46, 3.1, 9, 0.14, 13, False),
    ("left", "fol-w-mid", -4.6, 4.3, 3.2, 3.2, 9, 0.14, 14, False),
    ("right", "fol-e-mid", 4.8, -4.3, 3.2, 3.2, 9, 0.14, 15, False),
    ("leader-left", "fol-leader-w", -2.3, 2.0, 2.6, 2.5, 8, 0.12, 16, False),
    ("leader-right", "fol-leader-e", 2.2, -2.0, 2.5, 2.4, 8, 0.12, 17, False),
    ("top", "fol-top", 0.2, 0.1, 3.0, 2.9, 8, 0.12, 18, False),
]
ZS = {"core": 4.4, "left-low": 4.6, "right-low": 4.6, "back": 4.6, "front": 4.6, "left-mid": 4.62, "right-mid": 4.62, "left": 4.62, "right": 4.62, "leader-left": 4.7, "leader-right": 4.7, "top": 4.8}
TONE50 = {"low": 0.55, "mid": 0.52, "leader": 0.58, "top": 0.62}


def tone_of(id_, core, scale):
    if core:
        return 0.4 if scale == 50 else 0.45
    if scale == 50:
        for k, v in TONE50.items():
            if k in id_ or (k == "low" and id_ in ("back", "front")) or (k == "mid" and id_ in ("left", "right")):
                return v
    return 0.6


def lobes(view, scale):
    src = FRONT_LOBES if view == "front" else PLAN_LOBES
    out = []
    for id_, bone, cx, cy, rx, ry, n, depth, seed, core in src:
        if core:
            kw = dict(region="leaf-shade")
            if scale in (100, 50):
                kw["hatch"] = "shade-lines"
        else:
            kw = dict(region="leaf-sun")
        nn = n if scale <= 100 else max(6, n - 2)
        out.append(lobe(id_, bone, cx, cy, rx, ry, nn, depth if scale <= 100 else depth * 0.8, tone_of(id_, core, scale), ZS[id_], phase=seed, seed=seed, **kw))
    return out


def hull_outline(view):
    src = FRONT_LOBES if view == "front" else PLAN_LOBES
    cy0 = 10.0 if view == "front" else 0.0
    pts = []
    n = 100
    for i in range(n):
        t = 2 * m.pi * i / n
        dx, dy = m.cos(t), m.sin(t)
        far = 0.0
        for k in range(1, 700):
            r = k * 0.04
            x, y = r * dx, cy0 + r * dy
            if any(((x - ex) / erx) ** 2 + ((y - ey) / ery) ** 2 <= 1 for _, _, ex, ey, erx, ery, *_ in src):
                far = r
        pts.append((far * dx, cy0 + far * dy))
    return pts


def mirror(pts, s):
    return [(x * s, y) for x, y in pts]


def build(scale):
    front, plan = [], []
    CR = ["leaf-sun", "leaf-shade"]
    if scale == 500:
        front.append(poly("limb-w", "limb-w-low", "body", 1.0, 0.45, ribbon([(0, 3.0), (1.8, 3.4), (3.6, 4.2)], [0.5, 0.35, 0.2]), region="limb-bark", followTip=True))
        front.append(poly("limb-e", "limb-e-low", "body", 1.0, 0.45, ribbon([(0, 3.3), (-1.8, 3.9), (-3.8, 4.8)], [0.5, 0.35, 0.2]), region="limb-bark", followTip=True))
        front.append(poly("bole", "trunk", "body", 2.0, 0.35, [(-1.15, 0), (-0.6, 0.6), (-0.5, 3.0), (-0.4, 6.6), (0.4, 6.6), (0.5, 3.0), (0.6, 0.6), (1.15, 0)], region="trunk-bark"))
        front.append(poly("crown", "leader", "body", 3.0, 0.65, hull_outline("front"), regions=CR))
        plan.append(poly("crown", "leader", "body", 3.0, 0.65, hull_outline("plan"), regions=CR))
        return front, plan
    # trunk, bole with root flare and leader
    flare = [(-1.2, 0.0), (-0.8, 0.3), (-0.55, 0.9), (-0.5, 3.0), (-0.42, 6.0), (0.42, 6.0), (0.5, 3.0), (0.55, 0.9), (0.8, 0.3), (1.2, 0.0)]
    front.append(poly("trunk", "trunk", "body", 3.0, 0.26 if scale == 50 else 0.35, flare, region="trunk-bark"))
    front.append(poly("leader", "leader", "body", 3.2, 0.28 if scale == 50 else 0.35, ribbon([(0.0, 5.9), (-0.1, 8.0), (-0.3, 10.4), (0.15, 13.7)], [0.42, 0.36, 0.3, 0.12]), region="trunk-bark", followTip=True))
    limbs_f = [
        ("limb-w-low", "limb-w-low", [(0.0, 2.9), (1.9, 3.5), (3.7, 4.3)], [0.5, 0.36, 0.2]),
        ("limb-e-low", "limb-e-low", [(0.0, 3.2), (-1.9, 3.9), (-3.9, 4.9)], [0.5, 0.36, 0.2]),
        ("limb-w-mid", "limb-w-mid", [(0.1, 6.0), (1.5, 7.6), (3.0, 9.2)], [0.4, 0.3, 0.17]),
        ("limb-e-mid", "limb-e-mid", [(0.1, 6.1), (-1.4, 7.8), (-2.9, 9.7)], [0.4, 0.3, 0.17]),
    ]
    for id_, b, c, w in limbs_f:
        front.append(poly(id_, b, "body", 2.4, 0.4 if scale > 50 else 0.27, ribbon(c, w), region="limb-bark", followTip=True))
    if scale <= 100:
        for id_, b, c, w in [
            ("br-leader-w", "br-leader-w", [(-0.3, 10.4), (0.9, 11.8), (1.8, 13.0)], [0.2, 0.15, 0.09]),
            ("br-leader-e", "br-leader-e", [(-0.3, 10.4), (-0.9, 11.8), (-1.6, 13.2)], [0.2, 0.15, 0.09]),
        ]:
            front.append(poly(id_, b, "body", 2.5, 0.4 if scale > 50 else 0.28, ribbon(c, w), region="limb-bark", followTip=True))
    if scale == 50:
        for id_, b, c, w in [
            ("br-w-low", "br-w-low", [(3.7, 4.3), (4.8, 4.5), (6.0, 5.3)], [0.2, 0.14, 0.07]),
            ("br-e-low", "br-e-low", [(-3.9, 4.9), (-5.0, 5.3), (-6.2, 6.1)], [0.2, 0.14, 0.07]),
            ("br-w-mid", "br-w-mid", [(3.0, 9.2), (4.2, 9.6), (5.4, 10.6)], [0.17, 0.12, 0.06]),
            ("br-e-mid", "br-e-mid", [(-2.9, 9.7), (-4.0, 10.3), (-5.3, 11.3)], [0.17, 0.12, 0.06]),
        ]:
            front.append(poly(id_, b, "body", 2.5, 0.28, ribbon(c, w), region="limb-bark", followTip=True))
    front += lobes("front", scale)
    plan += lobes("plan", scale)
    if scale <= 100:
        for i, (id_, b, cx, cy, rx, ry, *_r) in enumerate(FRONT_LOBES):
            cnt = 2 if scale == 100 else 3
            front += arcs(f"{id_}-arc", b, cx, cy, rx, ry, cnt, 60 + i, 0.3, 9.0 + 0.01 * i, role="detail")
        for i, (id_, b, cx, cy, rx, ry, *_r) in enumerate(PLAN_LOBES):
            cnt = 2 if scale == 100 else 3
            plan += arcs(f"{id_}-arc", b, cx, cy, rx, ry, cnt, 80 + i, 0.3, 9.0 + 0.01 * i, role="detail")
        front += [poly(f"bole-fissure-{i + 1}", "trunk", "texture", 3.0, 0.2, pts, closed=False) for i, pts in enumerate([[(0.2, 0.7), (0.17, 1.8), (0.12, 2.9)], [(-0.2, 0.9), (-0.17, 2.0), (-0.12, 3.1)]])]
    if scale == 50:
        for i, (id_, b, cx, cy, rx, ry, *_r) in enumerate(FRONT_LOBES):
            front += arcs(f"{id_}-leaves", b, cx, cy, rx, ry, 4, 100 + i, 0.3, 9.1 + 0.01 * i, role="texture", size=(0.1, 0.2))
        for i, (id_, b, cx, cy, rx, ry, *_r) in enumerate(PLAN_LOBES):
            plan += arcs(f"{id_}-leaves", b, cx, cy, rx, ry, 4, 120 + i, 0.3, 9.1 + 0.01 * i, role="texture", size=(0.1, 0.2))
        front.append(ellipse("bark-knot-1", "trunk", "detail", 3.07, 0.15, 0.2, 1.45, 0.13, 0.07))
        front.append(ellipse("bark-knot-2", "trunk", "detail", 3.07, 0.15, -0.25, 2.25, 0.1, 0.06))
        front.append(poly("bark-upper", "trunk-upper", "texture", 3.15, 0.15, [(-0.2, 3.7), (-0.15, 4.8), (-0.18, 5.7)], closed=False))
        hints = [
            ("limb-w-low", [(0.0, 0.0), (-2.0, 0.6), (-4.2, 1.0)]),
            ("limb-e-low", [(0.0, 0.0), (2.2, -0.5), (4.5, -1.0)]),
            ("limb-w-mid", [(0.0, 0.0), (-1.6, 1.8), (-3.1, 3.6)]),
            ("limb-e-mid", [(0.0, 0.0), (1.7, -1.8), (3.3, -3.6)]),
            ("br-w-low", [(-4.2, 1.0), (-6.0, 1.2), (-7.6, 1.0)]),
            ("br-e-low", [(4.5, -1.0), (6.2, -1.2), (8.1, -0.9)]),
            ("br-w-mid", [(-3.1, 3.6), (-4.4, 5.0), (-5.4, 6.3)]),
            ("br-e-mid", [(3.3, -3.6), (4.8, -5.0), (5.9, -6.4)]),
        ]
        for b, pts in hints:
            plan.append(poly(f"hint-{b}", b, "detail", 7.0, 0.25, pts, closed=False, region="limb-bark"))
    return front, plan


NOTES = {
    500: ("Front: the bole with two low limbs and one crown outline of eight lobes seen from the front. Plan: one crown outline from above; bole and limbs are hidden.",
          "Vorderansicht: der Schaft mit zwei unteren Starkästen und ein Kronenumriss aus acht Lappen von vorn. Draufsicht: ein Kronenumriss von oben; Schaft und Starkäste sind verdeckt."),
    200: ("Front: bole with root flare, leader and four limbs, eight foliage masses in pairs; the core stays shaded behind. Plan: eight lobes around the shaded core; no bole.",
          "Vorderansicht: Schaft mit Wurzelanlauf, Mittelast und vier Starkästen, acht Laubmassen in Paaren; der Kern bleibt dahinter beschattet. Draufsicht: acht Lappen um den beschatteten Kern; kein Schaft."),
    100: ("Front adds the two leader branches, leaf arcs and two bole fissures; the core is hatched. Plan: lobes with leaf arcs, hatched core; bole and limbs hidden.",
          "Die Vorderansicht ergänzt die zwei Mittelastzweige, Blattbögen und zwei Schaftrisse; der Kern ist schraffiert. Draufsicht: Lappen mit Blattbögen, schraffierter Kern; Schaft und Starkäste verdeckt."),
    50: ("Front adds the four secondary branches, bark texture with two knots and leaf strokes in every mass. Plan adds the eight limbs and branches as hints over the crown and leaf strokes.",
         "Die Vorderansicht ergänzt die vier Nebenäste, Rindentextur mit zwei Astlöchern und Blattstriche in jeder Masse. Die Draufsicht ergänzt die acht Starkäste und Äste als Andeutung über der Krone sowie Blattstriche."),
}

if __name__ == "__main__":
    views = {sc: build(sc) for sc in (500, 200, 100, 50)}
    install("english-oak", views, NOTES)
    extents("english-oak")
