from kit import *

SUN = ["leaf-sun", "spur-twig"]

# (id, bone, cx, cy, rx, ry, lobes, depth, seed, core?)
FRONT_LOBES = [
    ("core-low", "fol-core-low", 0.0, 3.82, 1.35, 1.02, 7, 0.10, 1, True),
    ("core-up", "fol-core-up", 0.05, 4.78, 1.0, 0.87, 6, 0.10, 2, True),
    ("left-low", "fol-w1", 1.85, 3.1, 1.25, 0.85, 7, 0.14, 3, False),
    ("right-low", "fol-e1", -1.85, 3.25, 1.25, 0.85, 7, 0.14, 4, False),
    ("left-up", "fol-w2", 1.2, 4.85, 1.0, 0.95, 6, 0.14, 5, False),
    ("right-up", "fol-e2", -1.15, 5.0, 1.0, 0.97, 6, 0.14, 6, False),
    ("top", "fol-top", -0.05, 5.5, 1.1, 0.85, 6, 0.12, 7, False),
]
PLAN_LOBES = [
    ("core-low", "fol-core-low", 0.0, 0.0, 1.55, 1.45, 9, 0.10, 11, True),
    ("back", "fol-w1", -2.05, 0.2, 1.35, 1.25, 8, 0.14, 12, False),
    ("front", "fol-e1", 2.05, -0.2, 1.33, 1.25, 8, 0.14, 13, False),
    ("left", "fol-w2", -1.55, 1.95, 1.15, 1.15, 7, 0.14, 14, False),
    ("right", "fol-e2", 1.5, -1.95, 1.15, 1.15, 7, 0.14, 15, False),
    ("core-up", "fol-core-up", 0.0, 0.0, 1.1, 1.05, 7, 0.12, 16, True),
    ("top", "fol-top", 0.14, 0.1, 1.3, 1.2, 8, 0.12, 17, False),
]
ZS = {"core-low": 3.0, "core-up": 3.3, "left-low": 3.6, "right-low": 3.6, "back": 3.6, "front": 3.6, "left-up": 4.2, "right-up": 4.2, "left": 4.2, "right": 4.2, "top": 4.8}


def lobes(view, scale, hatch):
    src = FRONT_LOBES if view == "front" else PLAN_LOBES
    out = []
    for id_, bone, cx, cy, rx, ry, n, depth, seed, core in src:
        if scale == 500:
            continue
        if scale == 200 and id_ == "core-up" and view == "plan":
            continue
        kw = dict(region="leaf-shade") if core else (dict(regions=SUN) if scale == 200 else dict(region="leaf-sun"))
        if core and hatch and scale <= 100:
            kw["hatch"] = hatch
        nn = n if scale <= 100 else max(5, n - 2)
        out.append(lobe(id_, bone, cx, cy, rx, ry, nn, depth if scale <= 100 else depth * 0.8, 0.45 if core else 0.6, ZS[id_], phase=seed, seed=seed, **kw))
    return out


def crown500(view):
    src = FRONT_LOBES if view == "front" else PLAN_LOBES
    xs = [(cx, cy, rx, ry) for _, _, cx, cy, rx, ry, *_ in src]
    pts = []
    import math as m
    n = 72
    for i in range(n):
        t = 2 * m.pi * i / n
        best = 0
        for cx, cy, rx, ry in xs:
            dx, dy = m.cos(t), m.sin(t)
            lo, hi = 0.0, 12.0
            for _ in range(30):
                mid = (lo + hi) / 2
                inside = False
                for ex, ey, erx, ery in xs:
                    if ((mid * dx - ex) / erx) ** 2 + ((mid * dy - ey) / ery) ** 2 <= 1:
                        inside = True
                        break
                lo, hi = (mid, hi) if inside else (lo, mid)
            best = lo
            break
        pts.append((best * m.cos(t) * 1.0, best * m.sin(t)))
    return pts


def hull_outline(view):
    """Scalloped union outline of the lobes of a view, sampled by rays from the crown centre."""
    import math as m
    src = FRONT_LOBES if view == "front" else PLAN_LOBES
    cy0 = 4.4 if view == "front" else 0.0
    cx0 = 0.0
    pts = []
    n = 90
    for i in range(n):
        t = 2 * m.pi * i / n
        dx, dy = m.cos(t), m.sin(t)
        far = 0.0
        for k in range(1, 400):
            r = k * 0.02
            x, y = cx0 + r * dx, cy0 + r * dy
            if any(((x - ex) / (erx * 1.0)) ** 2 + ((y - ey) / (ery * 1.0)) ** 2 <= 1 for _, _, ex, ey, erx, ery, *_ in src):
                far = r
        pts.append((cx0 + far * dx, cy0 + far * dy))
    return pts


def trunk_front(scale):
    out = []
    pts = [(-0.55, 0.0), (-0.42, 0.25), (-0.37, 0.8), (-0.35, 1.6), (-0.3, 2.2), (-0.2, 2.9), (0.2, 2.9), (0.3, 2.2), (0.35, 1.6), (0.37, 0.8), (0.42, 0.25), (0.55, 0.0)]
    lo = [(-0.55, 0.0), (-0.42, 0.25), (-0.37, 0.8), (-0.35, 1.6), (0.35, 1.6), (0.37, 0.8), (0.42, 0.25), (0.55, 0.0)]
    out.append(poly("stem", "trunk", "body", 1.0, 0.36, lo, region="trunk-bark"))
    if scale >= 500:
        pass
    return out


def build(scale, hatch):
    front, plan = [], []
    if scale == 500:
        front.append(poly("stem", "trunk", "body", 1.0, 0.36, [(-0.5, 0.0), (-0.36, 0.5), (-0.34, 2.7), (0.34, 2.7), (0.36, 0.5), (0.5, 0.0)], region="trunk-bark"))
        front.append(poly("crown", "ld", "body", 3.0, 0.55, hull_outline("front"), regions=["leaf-sun", "leaf-shade", "limb-bark", "spur-twig", "fruit"]))
        plan.append(poly("crown", "ld", "body", 3.0, 0.55, hull_outline("plan"), regions=["leaf-sun", "leaf-shade", "limb-bark", "spur-twig", "fruit"]))
        return front, plan
    # trunk and limbs, front
    front.append(poly("stem", "trunk", "body", 1.0, 0.36, [(-0.55, 0.0), (-0.42, 0.25), (-0.37, 0.8), (-0.35, 1.6), (0.35, 1.6), (0.37, 0.8), (0.42, 0.25), (0.55, 0.0)], region="trunk-bark"))
    front.append(poly("stem-fork", "ld", "body", 1.5, 0.38, [(-0.35, 1.5), (0.35, 1.5), (0.22, 3.8), (0.15, 5.3), (-0.15, 5.3), (-0.22, 3.8)], region="limb-bark", followTip=True))
    if scale <= 100:
        # scaffolds seen as they leave the stem: left on the +x side
        sc = [
            ("sc-w1a", "sc-w1a", 1, [(0.0, 1.5), (0.9, 2.1), (1.55, 2.75)], 0.12, 0.08),
            ("sc-e1a", "sc-e1a", -1, [(0.0, 1.55), (-0.95, 2.15), (-1.6, 2.8)], 0.12, 0.08),
            ("sc-w2a", "sc-w2a", 1, [(0.0, 2.1), (0.6, 3.0), (1.1, 4.1)], 0.1, 0.06),
            ("sc-e2a", "sc-e2a", -1, [(0.0, 2.3), (-0.55, 3.3), (-1.05, 4.4)], 0.1, 0.06),
        ]
        for id_, bone, s, c, w0, w1 in sc:
            front.append(poly(f"wood-{id_}", bone, "body", 2.2, 0.38, ribbon(c, [w0, (w0 + w1) / 2, w1]), region="limb-bark", followTip=True))
        front.append(poly("wood-ld-b", "ld-b", "body", 2.3, 0.38, ribbon([(-0.02, 3.8), (-0.04, 4.6), (-0.05, 5.3)], [0.1, 0.08, 0.05]), region="limb-bark", followTip=True))
    front += lobes("front", scale, hatch)
    plan += lobes("plan", scale, hatch)
    fruit_f = [("fol-w1", 2.5, 2.7), ("fol-e1", -2.55, 2.85), ("fol-w2", 1.65, 4.4), ("fol-e2", -1.55, 4.6), ("fol-top", 0.55, 5.1)]
    fruit_p = [("fol-w1", -2.4, 0.7), ("fol-e1", 2.6, -0.6), ("fol-w2", -1.7, 2.4), ("fol-e2", 1.7, -2.3), ("fol-top", 0.6, 0.55)]
    if scale == 50:
        fruit_f += [("fol-w1", 1.3, 2.9), ("fol-e1", -1.4, 3.1), ("fol-e2", -0.7, 4.3), ("fol-top", 0.9, 5.2)]
        fruit_p += [("fol-w1", -1.5, -0.6), ("fol-e1", 1.4, 0.8), ("fol-e2", 0.9, -1.2), ("fol-w2", -2.2, 1.4)]
    if scale <= 100:
        for i, (b, x, y) in enumerate(fruit_f):
            front.append(ellipse(f"{b}-fruit{i + 1}", b, "detail", 9.0, 0.25, x, y, 0.09, 0.09, region="fruit"))
        for i, (b, x, y) in enumerate(fruit_p):
            plan.append(ellipse(f"{b}-fruit{i + 1}", b, "detail", 9.0, 0.25, x, y, 0.09, 0.09, region="fruit"))
    if scale <= 100:
        for i, (id_, b, cx, cy, rx, ry, *_r) in enumerate(FRONT_LOBES):
            if id_ in ("core-up",):
                continue
            front += arcs(f"{id_}-arc", b, cx, cy, rx, ry, 2 if scale == 100 else 3, 20 + i, 0.2, 5.0 + 0.01 * i, role="detail" if scale == 100 else "texture")
        for i, (id_, b, cx, cy, rx, ry, *_r) in enumerate(PLAN_LOBES):
            if id_ in ("core-up",):
                continue
            plan += arcs(f"{id_}-arc", b, cx, cy, rx, ry, 2 if scale == 100 else 3, 40 + i, 0.2, 5.0 + 0.01 * i, role="detail" if scale == 100 else "texture")
    if scale == 50:
        for i, (x0, y0, x1, y1) in enumerate([(-0.18, 0.4, -0.15, 0.85), (0.14, 0.55, 0.17, 1.05), (-0.12, 1.05, -0.1, 1.5), (0.02, 1.3, 0.04, 1.75)]):
            front.append(poly(f"bark-scale-{i + 1}", "trunk", "texture", 1.2, 0.15, [(x0, y0), ((x0 + x1) / 2 + 0.02, (y0 + y1) / 2), (x1, y1)], closed=False))
        front.append(poly("bark-knot-1", "trunk", "detail", 1.3, 0.2, ring_poly(0.05, 0.95, 0.04, 0.06, 8), closed=False))
        # branch structure hint seen through the crown
        scaf = [("sc-w1a", [(0.0, 0.0), (-1.1, 0.1), (-2.1, 0.25)]), ("sc-e1a", [(0.0, 0.0), (1.1, -0.1), (2.1, -0.25)]), ("sc-w2a", [(0.0, 0.0), (-0.9, 1.0), (-1.55, 1.9)]), ("sc-e2a", [(0.0, 0.0), (0.9, -1.0), (1.5, -1.9)])]
        for b, pts in scaf:
            plan.append(poly(f"hint-{b}", b, "detail", 6.0, 0.25, pts, closed=False, region="limb-bark"))
        for i, (b, pts) in enumerate([("spur-w1", [(-2.1, 0.25), (-2.55, 0.5)]), ("spur-e1", [(2.1, -0.25), (2.6, -0.5)]), ("spur-w2", [(-1.55, 1.9), (-1.8, 2.5)]), ("spur-e2", [(1.5, -1.9), (1.75, -2.5)])]):
            plan.append(poly(f"twig-{b}", b, "texture", 6.1, 0.2, pts, closed=False, region="spur-twig"))
    return front, plan


NOTES = {
    500: ("Front: the stem and one scalloped crown of seven lobes, widest at the lower side lobes. Plan: one scalloped crown outline from above; the stem is hidden.",
          "Vorderansicht: Stamm und eine gewellte Krone aus sieben Lappen, am breitesten an den unteren Seitenlappen. Draufsicht: ein gewellter Kronenumriss von oben; der Stamm ist verdeckt."),
    200: ("Front: stem with the leader, seven foliage masses (left and right pairs at two heights, core and top). Plan: eight lobes around the shaded core, so the lobed crown shows; no stem.",
          "Vorderansicht: Stamm mit Mittelast, sieben Laubmassen (linke und rechte Paare in zwei Höhen, Kern und Wipfel). Draufsicht: acht Lappen um den beschatteten Kern, damit die gelappte Krone erscheint; kein Stamm."),
    100: ("Front: stem, four scaffolds and the leader, seven masses with leaf arcs and fruit; the core stays hatched. Plan: lobes in sun and shade, leaf arcs and fruit; the stem is under the crown.",
          "Vorderansicht: Stamm, vier Leitäste und Mittelast, sieben Massen mit Blattbögen und Früchten; der Kern bleibt schraffiert. Draufsicht: Lappen in Sonne und Schatten, Blattbögen und Früchte; der Stamm liegt unter der Krone."),
    50: ("Front as at 1:100 with bark scales and knot, three arc strokes per mass and nine fruits. Plan with the four scaffolds as branch hints over the crown, spur twigs at the lobe tips, arcs and eight fruits.",
         "Vorderansicht wie bei 1:100 mit Rindenschuppen und Astloch, drei Bogenstrichen je Masse und neun Früchten. Draufsicht mit den vier Leitästen als Astandeutung über der Krone, Kurztrieben an den Lappenspitzen, Bögen und acht Früchten."),
}

if __name__ == "__main__":
    d = load("apple-tree")
    views = {}
    for sc in (500, 200, 100, 50):
        hatch = "shade-lines" if sc in (100, 50) else None
        views[sc] = build(sc, hatch)
    install("apple-tree", views, NOTES)
    extents("apple-tree")
