import math
from kit import *

doc = load("agave")
r, bones = rest(doc)

AZ = {"R1": -10, "R4": 40, "R5": 62, "R2": 85, "L5": 108, "L3": 128, "L1": 168, "L4": 220, "L2": 272, "R3": 308}
SPREAD = {"R1", "R2", "L1", "L2"}
BASE_Y = 0.3


def leaf(name):
    """(bone a, bone b or None, reach at knee, knee height, reach at tip, tip height)"""
    if name in ("R5", "L5"):
        a = f"leaf-{name}"
        t = tip(r, bones, a)
        return a, None, abs(t[0]), t[1], abs(t[0]), t[1]
    a, b = f"leaf-{name}-a", f"leaf-{name}-b"
    k = tip(r, bones, a)
    t = tip(r, bones, b)
    return a, b, abs(k[0]), k[1], abs(t[0]), t[1]


def region(name):
    return "leaves-spreading" if name in SPREAD else "leaves-erect"


def tone(name):
    return 0.45 if name in SPREAD else 0.35


def plan_pt(rad, az):
    return polar(rad, az)


def plan_leaf(scale, name, z):
    a, b, rk, yk, rt, yt = leaf(name)
    az = AZ[name]
    out = []
    hw = 0.1 if name in SPREAD else 0.075
    if b is None:
        x1, y1 = plan_pt(rt, az)
        out.append(capsule(f"leaf-{name}", a, "body", z, tone(name), 0, 0, x1, y1, hw * 0.9, 0.012, region=region(name), followTip=True))
        return out
    kx, ky = plan_pt(rk, az)
    tx, ty = plan_pt(rt, az)
    out.append(capsule(f"leaf-{name}-a", a, "body", z, tone(name), 0, 0, kx, ky, hw * 0.9, hw, region=region(name), followTip=True))
    out.append(capsule(f"leaf-{name}-b", b, "body", z + 0.005, tone(name), kx, ky, tx, ty, hw, 0.012, region=region(name), followTip=True))
    return out


def front_leaf(scale, name, z):
    a, b, rk, yk, rt, yt = leaf(name)
    az = AZ[name]
    s, c = math.sin(az * D2R), math.cos(az * D2R)
    hwid = 0.11 if name in SPREAD else 0.085
    hwid = hwid * max(abs(c), 0.2)
    out = []
    zz = z + 0.5 * c
    sx = lambda rad: rad * s
    if b is None:
        cen = bow((0.0, BASE_Y), (sx(rt), yt), 6, 0.0)
        out.append(poly(f"leaf-{name}", a, "body", zz, tone(name), ribbon(cen, prof(6, hwid * 0.55, hwid, 0.004)), region=region(name), followTip=True))
        return out
    ca = seg((0.0, BASE_Y), (sx(rk), yk), 5)
    out.append(poly(f"leaf-{name}-a", a, "body", zz, tone(name), ribbon(ca, prof(5, hwid * 0.5, hwid, hwid * 0.9, 0.6)), region=region(name), followTip=True))
    cb = seg((sx(rk), yk), (sx(rt), yt), 5)
    out.append(poly(f"leaf-{name}-b", b, "body", zz + 0.003, tone(name), ribbon(cb, prof(5, hwid * 0.9, hwid, 0.004, 0.25)), region=region(name), followTip=True))
    return out


def ribs(view, name, z):
    a, b, rk, yk, rt, yt = leaf(name)
    az = AZ[name]
    out = []
    if view == "plan":
        kx, ky = plan_pt(rk, az)
        tx, ty = plan_pt(rt, az)
        if b is None:
            out.append(poly(f"rib-{name}", a, "detail", z, 0.2, seg((0.0, 0.0), (tx * 0.9, ty * 0.9), 2), closed=False, followTip=False))
            return out
        out.append(poly(f"rib-{name}-a", a, "detail", z, 0.2, seg((0.0, 0.0), (kx, ky), 2), closed=False))
        out.append(poly(f"rib-{name}-b", b, "detail", z + 0.005, 0.2, seg((kx, ky), (tx * 0.96 + kx * 0.04, ty * 0.96 + ky * 0.04), 2), closed=False))
    else:
        s = math.sin(az * D2R)
        if b is None:
            out.append(poly(f"rib-{name}", a, "detail", z, 0.2, seg((0.0, BASE_Y + 0.05), (rt * s * 0.9, yt - 0.05), 2), closed=False, followTip=True))
            return out
        out.append(poly(f"rib-{name}-a", a, "detail", z, 0.2, seg((0.0, BASE_Y + 0.03), (rk * s, yk), 2), closed=False, followTip=True))
        out.append(poly(f"rib-{name}-b", b, "detail", z + 0.005, 0.2, seg((rk * s, yk), (rt * s * 0.94 + rk * s * 0.06, yt - 0.03), 2), closed=False, followTip=True))
    return out


def star(names, extra_valley=0.32):
    pts = [(0.0, 0.0)]
    items = sorted(names, key=lambda n: AZ[n])
    for i, n in enumerate(items):
        a, b, rk, yk, rt, yt = leaf(n)
        az = AZ[n]
        if i > 0:
            pn = items[i - 1]
            ra = leaf(pn)[4]
            mid = (AZ[pn] + az) / 2
            pts.append(polar(extra_valley * min(ra, rt) + 0.15, mid))
        pts.append(polar(rt * 0.55, az - 4))
        pts.append(polar(rt, az))
        pts.append(polar(rt * 0.55, az + 4))
    return pts


def heart_plan(scale):
    if scale >= 200:
        return [ellipse("heart", "leaf-S", "body", 3.5, 0.3, 0.0, 0.0, 0.16, 0.16, region="leaves-erect")]
    return [ellipse("heart", "leaf-S", "body", 3.5, 0.3, 0.0, 0.0, 0.14, 0.14, region="leaves-erect")]


def build(scale):
    names = list(AZ)
    front, plan = [], []
    if scale == 500:
        Rn = [n for n in names if n.startswith("R")]
        Ln = [n for n in names if n.startswith("L")]
        plan.append(poly("rosette-right", "leaf-R2-a", "body", 2.0, 0.4, star(Rn), regions=["leaves-spreading", "leaves-erect"]))
        plan.append(poly("rosette-left", "leaf-L2-a", "body", 2.0, 0.4, star(Ln), regions=["leaves-spreading", "leaves-erect"]))
        plan.append(ellipse("heart", "leaf-S", "body", 3.0, 0.35, 0.0, 0.0, 0.2, 0.2, region="leaves-erect"))
        front.append(poly("caudex", "caudex", "body", 1.0, 0.3, [(-0.12, 0.0), (0.12, 0.0), (0.1, 0.42), (-0.1, 0.42)], region="caudex"))
        fan = [(0.0, 0.3)]
        for n in sorted(AZ, key=lambda k: math.sin(AZ[k] * D2R)):
            a, b, rk, yk, rt, yt = leaf(n)
            s = math.sin(AZ[n] * D2R)
            if n in ("R5", "L5"):
                continue
            fan.append((rk * s * 0.6, yk + 0.02))
            fan.append((rt * s, yt))
        pts_r = sorted([(p[0], p[1]) for p in fan[1:]], key=lambda p: p[0])
        left_names = [n for n in names if math.sin(AZ[n] * D2R) >= 0.0]
        right_names = [n for n in names if math.sin(AZ[n] * D2R) < 0.0]
        for lab, grp, bone in (("left", left_names, "leaf-L2-a"), ("right", right_names, "leaf-R2-a")):
            pts = [(0.0, 0.3)]
            ordered = sorted(grp, key=lambda k: math.sin(AZ[k] * D2R))
            if lab == "right":
                ordered = ordered[::-1]
            tips = []
            for n in ordered:
                a, b, rk, yk, rt, yt = leaf(n)
                s = math.sin(AZ[n] * D2R)
                tips.append((rt * s, yt, rk * s, yk))
            tips.sort(key=lambda t: -abs(t[0]))
            outline = [(0.0, 0.3)]
            lo = sorted(tips, key=lambda t: t[0])
            for t in lo:
                outline.append((t[2], t[3] + 0.08))
                outline.append((t[0], t[1]))
            outline.append((0.0, 0.3))
            front.append(poly(f"rosette-{lab}", bone, "body", 2.0, 0.4, [(x, y) for x, y in outline if True], regions=["leaves-spreading", "leaves-erect"]))
        front.append(poly("heart", "leaf-S", "body", 3.0, 0.35, [(-0.08, 0.35), (0.08, 0.35), (0.05, 1.0), (0.0, 1.45), (-0.05, 1.0)], region="leaves-erect"))
        return front, plan

    front.append(poly("caudex", "caudex", "body", 1.0, 0.3, [(-0.13, 0.0), (0.13, 0.0), (0.11, 0.42), (-0.11, 0.42)], region="caudex"))
    if scale == 50:
        for i, yy in enumerate((0.1, 0.2, 0.3)):
            front.append(poly(f"caudex-scar-{i + 1}", "caudex", "texture", 1.2, 0.15, [(-0.1, yy), (0.0, yy - 0.03), (0.1, yy)], closed=False))
    order = sorted(names, key=lambda n: -math.cos(AZ[n] * D2R))
    z = 2.0
    for n in order:
        front += front_leaf(scale, n, z)
        z += 0.02
    zr = 2.0
    for n in sorted(names, key=lambda n: leaf(n)[5] + (0.5 if n in ("R5", "L5") else 0)):
        plan += plan_leaf(scale, n, zr)
        zr += 0.02
    plan += heart_plan(scale)
    front.append(poly("leaf-S", "leaf-S", "body", 2.6, 0.35, ribbon(seg((0.0, BASE_Y), (0.0, 1.45), 6), prof(6, 0.045, 0.085, 0.004)), region="leaves-erect", followTip=True))
    if scale == 50:
        for n in names:
            front += ribs("front", n, 3.0 + 0.5 * math.cos(AZ[n] * D2R))
            plan += ribs("plan", n, 3.2)
        front.append(poly("rib-S", "leaf-S", "detail", 3.0, 0.2, seg((0.0, 0.4), (0.0, 1.35), 2), closed=False, followTip=True))
        plan.append(ellipse("heart-core", "leaf-S", "detail", 3.6, 0.2, 0.0, 0.0, 0.06, 0.06, region="leaves-erect"))
    return front, plan


NOTES = {
    500: ("Front is one fan of the seen leaf tips with the spear; plan is a two-half star of the leaf tips around a heart, no bases or ribs.",
          "Die Vorderansicht ist ein Fächer der sichtbaren Blattspitzen mit dem Herzblatt; die Draufsicht ein zweiteiliger Stern der Blattspitzen um ein Herz, ohne Basen und Rippen."),
    200: ("Front: the eleven leaves seen from the side as arching blades, near leaves over far ones, caudex in front of the spear. Plan: the eleven leaves as tapered blades around the heart, spiral azimuths, upright leaves on top.",
          "Vorderansicht: die elf Blätter von der Seite als bogige Klingen, nahe Blätter über fernen, der Caudex vor dem Herzblatt. Draufsicht: die elf Blätter als verjüngte Klingen um das Herz in Spiralrichtungen, aufrechte Blätter oben."),
    100: ("Front: the leaves are drawn with base and tip segments, foreshortened by their direction, widths by the view angle. Plan: every leaf in two segments from the heart, so the knee shows and spread leaves lie under the erect ones.",
          "Vorderansicht: die Blätter mit Basis- und Spitzensegment, nach Richtung verkürzt, Breiten nach Blickwinkel. Draufsicht: jedes Blatt in zwei Segmenten ab Herz, sodass der Knick sichtbar ist und ausgebreitete Blätter unter den aufrechten liegen."),
    50: ("Front and plan as at 1:100 with leaf midribs, three leaf-base scars on the caudex and the heart circle; midribs follow the same bones as the blades.",
         "Vorder- und Draufsicht wie bei 1:100, dazu Blattmittelrippen, drei Blattnarben am Caudex und der Herzkreis; die Rippen folgen denselben Knochen wie die Klingen."),
}

if __name__ == "__main__":
    views = {sc: build(sc) for sc in (500, 200, 100, 50)}
    install("agave", views, NOTES)
    extents("agave")
