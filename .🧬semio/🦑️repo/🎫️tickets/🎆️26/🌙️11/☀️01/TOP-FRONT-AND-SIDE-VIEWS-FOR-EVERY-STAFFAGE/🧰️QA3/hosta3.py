from kit import *

import json
doc = load("hosta-clump")
r, bones = rest(doc)
doc0 = json.load(open("hosta.before.json", encoding="utf8"))
r0, bones0 = rest(doc0)

AZ = {"R1": -12, "R2": 84, "R3": -52, "R4": 38, "C1": 0, "L1": 168, "L2": -96, "L3": 128, "L4": 205}
OUTER = ("R1", "R2", "L1", "L2")
Y0 = bones["petiole-R1"]["y"]


def bl(n):
    pb, bb = f"petiole-{n}", f"blade-{n}"
    ob = r0[bb]
    tp = tip(r0, bones0, bb)
    return pb, bb, abs(ob[0]), ob[1], abs(tp[0]), tp[1]


def reg(n):
    return "blades-outer" if n in OUTER else "blades-inner"


def tn(n):
    return 0.4 if n in OUTER else 0.55


def own(n):
    pb, bb = f"petiole-{n}", f"blade-{n}"
    O = (r[bb][0], r[bb][1])
    T = tip(r, bones, bb)
    return pb, bb, (r[pb][0], r[pb][1]), O, T


def plan_blades(scale):
    out = []
    for n in sorted(AZ, key=lambda k: bl(k)[5]):
        pb, bb, P, O, T = own(n)
        wid = 0.065 if n in OUTER else 0.055
        rr = 0.03
        k = list(AZ).index(n)
        if n == "C1":
            out.append(capsule("blade-C1", bb, "body", 2.08, tn(n), O[0], 0, T[0], 0, 0.11, 0.05, region=reg(n), followTip=True))
        else:
            d = 1 if T[0] >= O[0] else -1
            out.append(capsule(f"blade-{n}", bb, "body", 2.0 + 0.01 * k, tn(n), O[0] + d * wid * 0.4, 0, T[0] - d * rr, 0, wid, rr, region=reg(n), followTip=True))
        if scale <= 100:
            out.insert(0, capsule(f"petiole-{n}", pb, "body", 1.0 + 0.01 * k, 0.35, P[0], 0, O[0], 0, 0.02, 0.015, region="petioles", followTip=True))
        if scale == 50:
            out.append(capsule(f"midrib-{n}", bb, "detail", 5.0 + 0.01 * k, 0.3, O[0], 0, T[0], 0, 0.006, 0.006, region=reg(n), followTip=True))
    return out


def lens(L, w0, w1, w2, peak=0.5, n=6):
    return ribbon(seg((0.0, 0.0), (L, 0.0), n), prof(n, w0, w1, w2, peak))


def front_blades(scale):
    out = []
    for n in sorted(AZ, key=lambda k: -math.cos(AZ[k] * D2R)):
        pb, bb, P, O, T = own(n)
        hw = 0.098 if n in OUTER else 0.085
        k = list(AZ).index(n)
        zz = 0.03 if n == "C1" else (0.09 if n in OUTER else 0.075)
        out.append(poly(f"blade-{n}", bb, "body", 2.0 + 0.5 * math.cos(AZ[n] * D2R) + 0.01 * k, tn(n), lens(bones[bb]["lengthM"], hw * 0.7, hw, 0.004, 0.3), region=reg(n), solid={"kind": "extruded", "z0": -zz, "z1": zz}))
        if scale <= 100:
            out.insert(0, poly(f"petiole-{n}", pb, "body", 1.0 + 0.01 * k, 0.35, [(x + 0.6 * bones[pb]["lengthM"], y) for x, y in lens(0.4 * bones[pb]["lengthM"], 0.016, 0.014, 0.012, 0.5, 3)], region="petioles", solid={"kind": "extruded", "z0": -0.012, "z1": 0.012}))
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
        plan.append(poly("mass-outer-right", "root-plate", "body", 1.0, 0.4, sector(-55, 80, 0.46, 0.1, 3, 0.04, 1), region="blades-outer"))
        plan.append(poly("mass-outer-left", "root-plate", "body", 1.0, 0.4, sector(100, 258, 0.46, 0.1, 3, 0.04, 2), region="blades-outer"))
        plan.append(lobe("mass-inner", "root-plate", 0.0, 0.0, 0.3, 0.29, 7, 0.14, 0.55, 2.0, phase=1, seed=3, region="blades-inner"))
        front.append(lobe("mass-outer-left", "root-plate", 0.235, 0.19, 0.235, 0.2, 5, 0.12, 0.4, 1.0, seed=4, region="blades-outer"))
        front.append(lobe("mass-outer-right", "root-plate", -0.235, 0.19, 0.235, 0.2, 5, 0.12, 0.4, 1.0, seed=5, region="blades-outer"))
        front.append(lobe("mass-inner", "root-plate", 0.0, 0.4, 0.26, 0.22, 6, 0.12, 0.55, 2.0, seed=6, region="blades-inner"))
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

def rebind200():
    d = load("hosta-clump")
    for sh in d["asset"]["representations"]["200"]["geometry"]["elevation"]:
        if sh["bone"].startswith("blade-"):
            sh["points"] = [[r4(x), r4(y)] for x, y in world_poly(r0, sh)]
            sh["bone"] = "root-plate"
    save("hosta-clump", d)


if __name__ == "__main__":
    rebind200()
    views = {sc: build(sc) for sc in (200, 100, 50)}
    install("hosta-clump", views, NOTES)
    extents("hosta-clump")
