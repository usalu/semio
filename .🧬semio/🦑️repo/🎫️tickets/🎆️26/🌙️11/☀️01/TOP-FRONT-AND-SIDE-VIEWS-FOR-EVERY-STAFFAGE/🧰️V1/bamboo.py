from kit import *

doc = load("bamboo-clump")
r, bones = rest(doc)

# lateral position (left is +) of every culm at its base and at its top
LAT = {1: (0.25, 0.55), 2: (-0.28, -0.30), 3: (0.12, 0.10), 4: (-0.22, -0.35), 5: (0.2, 0.45), 6: (-0.1, -0.45)}
HW0, HW1 = 0.04, 0.022
# front foliage sprays: (culm, cx, cy, rx, ry)
SPRAY_F = {1: (0.55, 3.0, 0.3, 0.75), 2: (-0.3, 3.65, 0.5, 0.9), 3: (0.1, 3.2, 0.5, 0.85), 4: (-0.35, 2.65, 0.5, 0.72), 5: (0.45, 2.2, 0.4, 0.6), 6: (-0.45, 1.9, 0.4, 0.55)}
# plan foliage: (culm, cx, cy, rx, ry), x like the elevation
SPRAY_P = {1: (-1.05, 0.42, 0.5, 0.43), 6: (-0.78, -0.45, 0.45, 0.4), 2: (-0.3, 0.08, 0.55, 0.5), 3: (0.2, -0.3, 0.5, 0.5), 4: (0.75, 0.38, 0.5, 0.47), 5: (1.1, -0.2, 0.4, 0.42)}
TOPX = {n: tip(r, bones, f"culm-{n}-c")[0] for n in range(1, 7)}
BASEX = {n: r[f"culm-{n}-a"][0] for n in range(1, 7)}


def lat_at(n, y):
    top = tip(r, bones, f"culm-{n}-c")[1]
    return LAT[n][0] + (LAT[n][1] - LAT[n][0]) * min(1.0, y / top)


def culm_front(scale, n):
    out = []
    for s in "abc":
        b = f"culm-{n}-{s}"
        y0 = r[b][1]
        y1 = tip(r, bones, b)[1]
        pts = [(lat_at(n, y0 + (y1 - y0) * k / 4), y0 + (y1 - y0) * k / 4) for k in range(5)]
        h0 = HW0 + (HW1 - HW0) * y0 / tip(r, bones, f"culm-{n}-c")[1]
        h1 = HW0 + (HW1 - HW0) * y1 / tip(r, bones, f"culm-{n}-c")[1]
        if scale == 200:
            out.append(poly(f"culm-{n}-{s}", b, "body", 1.0, 0.25, pts, closed=False, region="culms", followTip=True))
        else:
            out.append(poly(f"culm-{n}-{s}", b, "body", 1.0, 0.25, ribbon(pts, [h0 + (h1 - h0) * k / 4 for k in range(5)]), region="culms", followTip=True))
    return out


def nodes_front(n, count):
    out = []
    top = tip(r, bones, f"culm-{n}-c")[1]
    for k in range(count):
        y = top * (0.12 + 0.8 * k / max(1, count - 1)) * 0.9
        seg_b = "culm-%d-a" % n if y < tip(r, bones, f"culm-{n}-a")[1] else ("culm-%d-b" % n if y < tip(r, bones, f"culm-{n}-b")[1] else "culm-%d-c" % n)
        x = lat_at(n, y)
        w = HW0 + (HW1 - HW0) * y / top
        out.append(poly(f"node-{n}-{k}", seg_b, "texture", 5.0, 0.2, [(x - w * 1.05, y), (x + w * 1.05, y)], closed=False))
    return out


def sprays(view, scale):
    src = SPRAY_F if view == "front" else SPRAY_P
    out = []
    for n, (cx, cy, rx, ry) in src.items():
        bone = f"fol-{n}"
        k = 6 if scale <= 100 else 5
        out.append(lobe(f"mass-{n}-shade", bone, cx, cy, rx, ry, k, 0.3, 0.72 if scale == 50 else 0.7, 3.0 + 0.001 * n, phase=n, seed=n + (0 if view == "front" else 20), region="leaves-shade"))
        out.append(lobe(f"mass-{n}-sun", bone, cx + rx * 0.12, cy + ry * 0.1, rx * 0.62, ry * 0.62, 5, 0.3, 0.55 if scale == 50 else 0.5, 3.1 + 0.001 * n, phase=n + 1, seed=n + 40, region="leaves-sun"))
        if scale == 50:
            out += arcs(f"leaf-{n}", bone, cx, cy, rx, ry, 4, 200 + n, 0.25, 3.2, role="detail", size=(0.18, 0.3))
    return out


def build(scale):
    front, plan = [], []
    if scale == 500:
        front.append(poly("stems", "culm-3-a", "body", 2.0, 0.3, [(-0.5, 0.0), (0.5, 0.0), (0.55, 2.5), (-0.55, 2.5)], region="culms"))
        af = [(0.3, 3.2, 0.55, 1.35), (0.2, 2.5, 0.6, 0.9)]
        bf = [(-0.4, 2.7, 0.45, 1.0), (-0.3, 2.1, 0.5, 0.6)]
        front.append(poly("crown-A", "fol-1", "body", 3.0, 0.55, union_outline(af, 0.3, 3.0, 60, 0.01), regions=["leaves-sun", "leaves-shade"]))
        front.append(poly("crown-B", "fol-4", "body", 3.0, 0.55, union_outline(bf, -0.4, 2.7, 60, 0.01), regions=["leaves-sun", "leaves-shade"]))
        pa = [(-1.05, 0.42, 0.5, 0.43), (-0.78, -0.45, 0.45, 0.4), (-0.3, 0.08, 0.55, 0.5)]
        pb = [(0.2, -0.3, 0.5, 0.5), (0.75, 0.38, 0.5, 0.47), (1.1, -0.2, 0.4, 0.42)]
        plan.append(poly("crown-A", "fol-1", "body", 3.0, 0.55, union_outline(pa, -0.7, 0.0, 60, 0.01), regions=["leaves-sun", "leaves-shade"]))
        plan.append(poly("crown-B", "fol-4", "body", 3.0, 0.55, union_outline(pb, 0.7, 0.0, 60, 0.01), regions=["leaves-sun", "leaves-shade"]))
        return front, plan
    for n in range(1, 7):
        front += culm_front(scale, n)
        if scale <= 100:
            front += nodes_front(n, 4 if scale == 100 else 6)
    front += sprays("front", scale)
    plan += sprays("plan", scale)
    if scale <= 100:
        for n in range(1, 7):
            top = tip(r, bones, f"culm-{n}-c")
            plan.append(ellipse(f"culm-{n}-top", f"culm-{n}-c", "detail" if scale == 100 else "body", 6.0, 0.25, TOPX[n], LAT[n][1] * 0.8, 0.04, 0.04, region="culms"))
    if scale == 50:
        for n in range(1, 7):
            plan.append(ellipse(f"culm-{n}-base", f"culm-{n}-a", "detail", 6.1, 0.2, BASEX[n], LAT[n][0], 0.03, 0.03, region="culms"))
    return front, plan


NOTES = {
    500: ("Front: one bundle of culms and two crown masses seen from the side, leaning left and right. Plan: two crown masses from above, six culms hidden.",
          "Vorderansicht: ein Halmbündel und zwei Kronenmassen von der Seite, nach links und rechts geneigt. Draufsicht: zwei Kronenmassen von oben, die sechs Halme verdeckt."),
    200: ("Front: six culms as lines with their lateral lean and six leaf masses in shade and sun at the culm tops. Plan: six leaf masses around the clump centre, culms hidden.",
          "Vorderansicht: sechs Halme als Linien mit ihrer seitlichen Neigung und sechs Blattmassen in Schatten und Sonne an den Halmspitzen. Draufsicht: sechs Blattmassen um die Horstmitte, Halme verdeckt."),
    100: ("Front: six filled culms in three segments with node ticks and a leaf spray on each culm. Plan: six sprays and the six culm tops as rings in the foliage.",
          "Vorderansicht: sechs gefüllte Halme in drei Segmenten mit Knotenstrichen und je ein Blattbüschel pro Halm. Draufsicht: sechs Büschel und die sechs Halmspitzen als Ringe im Laub."),
    50: ("Front and plan as at 1:100 with six node ticks per culm, four leaf strokes per spray and in plan the six culm bases as small circles.",
         "Vorder- und Draufsicht wie bei 1:100 mit sechs Knotenstrichen je Halm, vier Blattstrichen je Büschel und in der Draufsicht den sechs Halmfüßen als kleinen Kreisen."),
}

if __name__ == "__main__":
    views = {sc: build(sc) for sc in (500, 200, 100, 50)}
    install("bamboo-clump", views, NOTES)
    extents("bamboo-clump")
