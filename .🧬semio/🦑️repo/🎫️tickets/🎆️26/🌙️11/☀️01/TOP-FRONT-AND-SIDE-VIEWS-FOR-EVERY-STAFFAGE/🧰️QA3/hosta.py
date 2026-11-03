from kit import *

doc = load("hosta-clump")
r, bones = rest(doc)

AZ = {"R1": -12, "R2": 84, "R3": -52, "R4": 38, "C1": 0, "L1": 168, "L2": -96, "L3": 128, "L4": 205}
OUTER = ("R1", "R2", "L1", "L2")
Y0 = bones["petiole-R1"]["y"]


def bl(n):
    pb, bb = f"petiole-{n}", f"blade-{n}"
    ob = r[bb]
    tp = tip(r, bones, bb)
    return pb, bb, abs(ob[0]), ob[1], abs(tp[0]), tp[1]


def reg(n):
    return "blades-outer" if n in OUTER else "blades-inner"


def tn(n):
    return 0.4 if n in OUTER else 0.55


def plan_blades(scale):
    out = []
    for n in sorted(AZ, key=lambda k: bl(k)[5]):
        pb, bb, rb, yb, rt, yt = bl(n)
        az = AZ[n]
        wid = 0.12 if n in OUTER else 0.1
        rr = 0.05
        if n == "C1":
            out.append(ellipse("blade-C1", bb, "body", 2.08, tn(n), 0.0, 0.0, 0.11, 0.12, region=reg(n)))
            continue
        ux, uy = math.cos(az * D2R), math.sin(az * D2R)
        bx, by = polar(rb, az)
        ex, ey = polar(rt - rr, az)
        z = 2.0 + 0.01 * list(AZ).index(n)
        out.append(capsule(f"blade-{n}", bb, "body", z, tn(n), bx + ux * wid * 0.4, by + uy * wid * 0.4, ex, ey, wid, rr, region=reg(n), followTip=True))
        if scale <= 100:
            out.insert(0, capsule(f"petiole-{n}", pb, "body", 1.0 + 0.01 * list(AZ).index(n), 0.35, 0.0, 0.0, bx, by, 0.02, 0.015, region="petioles", followTip=True))
        if scale == 50:
            sx, sy = polar(rb + 0.05, az)
            tx, ty = polar(rt - 0.03, az)
            out.append(poly(f"midrib-{n}", bb, "detail", 5.0 + 0.01 * list(AZ).index(n), 0.3, [(sx, sy), (tx, ty)], closed=False))
            for k, f in enumerate((0.45, 0.7)):
                mx, my = polar(lerp(rb, rt, f), az)
                for sgn in (1, -1):
                    out.append(poly(f"vein-{n}-{k + 1}{'lr'[sgn < 0]}", bb, "texture", 5.1, 0.25, [(mx, my), (mx + (ux * 0.04 - uy * sgn * wid * 0.8), my + (uy * 0.04 + ux * sgn * wid * 0.8))], closed=False))
    return out


def front_blades(scale):
    out = []
    for n in sorted(AZ, key=lambda k: -math.cos(AZ[k] * D2R)):
        pb, bb, rb, yb, rt, yt = bl(n)
        az = AZ[n]
        s, c = math.sin(az * D2R), math.cos(az * D2R)
        hw = (0.13 if n in OUTER else 0.11) * max(abs(c), 0.22)
        cen = [(lerp(rb * s, rt * s, k / 6), lerp(yb, yt, k / 6) + 0.02 * math.sin(math.pi * k / 6)) for k in range(7)]
        z = 2.0 + 0.5 * c + 0.01 * list(AZ).index(n)
        out.append(poly(f"blade-{n}", bb, "body", z, tn(n), ribbon(cen, prof(6, hw * 0.7, hw, 0.004, 0.3)), region=reg(n), followTip=True))
        if scale <= 100:
            pc = [(lerp(0.0, rb * s, k / 3), lerp(Y0, yb, k / 3)) for k in range(4)]
            out.insert(0, poly(f"petiole-{n}", pb, "body", 1.0 + 0.01 * list(AZ).index(n), 0.35, ribbon(pc, [0.016, 0.014, 0.013, 0.012]), region="petioles", followTip=True))
        if scale == 50:
            out.append(poly(f"midrib-{n}", bb, "detail", 5.0 + 0.01 * list(AZ).index(n), 0.3, [(rb * s + (rt - rb) * s * 0.1, yb + (yt - yb) * 0.1), (rt * s * 0.92 + rb * s * 0.08, yt - (yt - yb) * 0.08)], closed=False, followTip=True))
    return out


def sector(a0, a1, r_out, r_in, n_scallop, depth, seed):
    pts = []
    steps = n_scallop * 4
    for i in range(steps + 1):
        a = a0 + (a1 - a0) * i / steps
        k = 1 - depth * (0.5 - 0.5 * math.cos(2 * math.pi * i / 4))
        pts.append(polar(r_out * k, a))
    for i in range(steps, -1, -1):
        a = a0 + (a1 - a0) * i / steps
        pts.append(polar(r_in, a))
    return pts


def build(scale):
    front, plan = [], []
    if scale == 200:
        plan.append(poly("mass-outer-right", "blade-R1", "body", 1.0, 0.4, sector(-55, 80, 0.445, 0.1, 3, 0.04, 1), region="blades-outer"))
        plan.append(poly("mass-outer-left", "blade-L1", "body", 1.0, 0.4, sector(100, 258, 0.445, 0.1, 3, 0.04, 2), region="blades-outer"))
        plan.append(lobe("mass-inner", "blade-R3", 0.0, 0.0, 0.3, 0.29, 7, 0.14, 0.55, 2.0, phase=1, seed=3, region="blades-inner"))
        front.append(lobe("mass-outer-left", "blade-L1", 0.22, 0.19, 0.22, 0.2, 5, 0.12, 0.4, 1.0, seed=4, region="blades-outer"))
        front.append(lobe("mass-outer-right", "blade-R1", -0.22, 0.19, 0.22, 0.2, 5, 0.12, 0.4, 1.0, seed=5, region="blades-outer"))
        front.append(lobe("mass-inner", "blade-R3", 0.0, 0.4, 0.26, 0.22, 6, 0.12, 0.55, 2.0, seed=6, region="blades-inner"))
        return front, plan
    front = front_blades(scale)
    plan = plan_blades(scale)
    return front, plan


NOTES = {
    200: ("Front: two outer masses left and right and the inner mound, seen from the side. Plan: two outer sectors of blades around a lobed inner mound; petioles are covered.",
          "Vorderansicht: zwei äußere Massen links und rechts und der innere Hügel von der Seite. Draufsicht: zwei äußere Spreitensektoren um einen gelappten inneren Hügel; die Stiele sind verdeckt."),
    100: ("Front: nine blades as foreshortened leaf shapes with their petioles, near blades over far ones. Plan: nine broad blades on spiral azimuths over thin petiole strokes; inner blades lie over the outer ones.",
          "Vorderansicht: neun Spreiten als verkürzte Blattformen mit ihren Stielen, nahe Spreiten über fernen. Draufsicht: neun breite Spreiten in Spiralrichtungen über dünnen Stielstrichen; innere Spreiten liegen über den äußeren."),
    50: ("As at 1:100 with midribs on every blade in both views and two vein pairs per blade in plan.",
         "Wie bei 1:100 mit Mittelrippen auf jeder Spreite in beiden Ansichten und zwei Aderpaaren je Spreite in der Draufsicht."),
}

if __name__ == "__main__":
    views = {sc: build(sc) for sc in (200, 100, 50)}
    install("hosta-clump", views, NOTES)
    extents("hosta-clump")
