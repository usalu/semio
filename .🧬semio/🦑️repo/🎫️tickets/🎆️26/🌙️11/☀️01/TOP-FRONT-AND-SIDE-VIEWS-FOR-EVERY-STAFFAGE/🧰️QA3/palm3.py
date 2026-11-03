from kit import *

import json
doc = load("date-palm")
r, bones = rest(doc)
doc0 = json.load(open("palm.before.json", encoding="utf8"))
r0, bones0 = rest(doc0)
XC, YC = r["crown"][0], r["crown"][1]

AZ = {"l1": 100, "l2": 188, "l3": 148, "l4": 125, "r1": -80, "r2": 8, "r3": -38, "r4": 55}
SEC = [("l3", 168, 0.8), ("l2", 232, 0.85), ("r1", -60, 0.9), ("r3", -12, 0.9), ("r2", 32, 0.88), ("r4", 80, 0.8), ("l4", 105, 0.85)]


EXT = {"l2": 0.27, "r2": 0.27}


def fr(n):
    a, b = f"fr-{n}a", f"fr-{n}b"
    k, t = tip(r0, bones0, a), tip(r0, bones0, b)
    return a, b, abs(k[0] - XC), k[1], abs(t[0] - XC), t[1]


def own(n):
    a, b = f"fr-{n}a", f"fr-{n}b"
    return a, b, (r[a][0], r[a][1]), tip(r, bones, a), tip(r, bones, b)


def sec_offset(n, az):
    d = (az - AZ[n] + 180) % 360 - 180
    return 1 if d > 0 else -1


def lens(L, w0, w1, w2, peak=0.5, n=6):
    return ribbon(seg((0.0, 0.0), (L, 0.0), n), prof(n, w0, w1, w2, peak))


def plan_fronds(scale, detail):
    out = []
    z = 3.0
    for n in sorted(AZ, key=lambda k: fr(k)[5]):
        a, b, O, K, T = own(n)
        w = 0.3
        out.append(capsule(f"fr-{n}-a", a, "body", z, 0.55, O[0], 0, K[0], 0, w * 0.45, w, region="fronds", followTip=True))
        out.append(capsule(f"fr-{n}-b", b, "body", z + 0.001, 0.55, K[0], 0, T[0], 0, w, 0.03, region="fronds", followTip=True))
        if scale <= 100:
            out.append(capsule(f"rachis-{n}-a", a, "detail", z + 0.5, 0.2, O[0], 0, K[0], 0, 0.012, 0.012, followTip=True))
            out.append(capsule(f"rachis-{n}-b", b, "detail", z + 0.501, 0.2, K[0], 0, T[0], 0, 0.012, 0.012, followTip=True))
        if detail:
            for sg2 in (1, -1):
                out.append(capsule(f"leaflets-{n}-a{'lr'[sg2 < 0]}", a, "detail", z + 0.6, 0.2, O[0], sg2 * 0.03, K[0], sg2 * 0.12, 0.01, 0.01, followTip=True))
                out.append(capsule(f"leaflets-{n}-b{'lr'[sg2 < 0]}", b, "detail", z + 0.601, 0.2, K[0], sg2 * 0.12, T[0], sg2 * 0.02, 0.01, 0.01, followTip=True))
        z += 0.01
    return out


def comb(L, w, teeth, tip=0.0):
    up, lo = [], []
    for i in range(teeth):
        x0, x1 = L * i / teeth, L * (i + 0.5) / teeth
        k = 1 - (1 - tip) * i / teeth
        up += [(x0, 0.012), (x1, w * k)]
        lo += [(x0, -0.012), (x1, -w * k)]
    return up + [(L, 0.0)] + lo[::-1]


def front_fronds(scale, detail):
    out = []
    z = 3.0
    for n in sorted(AZ, key=lambda k: math.cos(AZ[k] * D2R)):
        a, b = f"fr-{n}a", f"fr-{n}b"
        c = math.cos(AZ[n] * D2R)
        w = 0.3
        sol = {"kind": "extruded", "z0": -0.05, "z1": 0.05}
        out.append(poly(f"fr-{n}-a", a, "body", z + 0.4 * c, 0.55, lens(bones[a]["lengthM"], w * 0.4, w, w * 0.95, 0.6, 5), region="fronds", solid=sol))
        out.append(poly(f"fr-{n}-b", b, "body", z + 0.4 * c + 0.001, 0.55, lens(bones[b]["lengthM"], w * 0.95, w, 0.02, 0.4, 5), region="fronds", solid=sol))
        thin = {"kind": "extruded", "z0": -0.012, "z1": 0.012}
        if scale <= 200:
            out.append(poly(f"rachis-{n}-a", a, "detail", z + 0.4 * c + 0.5, 0.2, lens(bones[a]["lengthM"], 0.02, 0.02, 0.012, 0.5, 4), region="fronds", solid=thin))
        if scale <= 100:
            out.append(poly(f"rachis-{n}-b", b, "detail", z + 0.4 * c + 0.501, 0.2, lens(bones[b]["lengthM"], 0.02, 0.02, 0.012, 0.5, 4), region="fronds", solid=thin))
        if detail:
            out.append(poly(f"leaflets-{n}-a", a, "detail", z + 0.4 * c + 0.6, 0.2, comb(bones[a]["lengthM"], 0.17, 8), region="fronds", solid=thin))
            out.append(poly(f"leaflets-{n}-b", b, "detail", z + 0.4 * c + 0.601, 0.2, comb(bones[b]["lengthM"], 0.28, 8, 0.2), region="fronds", solid=thin))
        z += 0.01
    return out


def trunk_front(scale):
    out = []
    if scale == 500:
        out.append(poly("bole", "stem-1", "body", 1.0, 0.35, [(-0.55, 0.0), (-0.5, 0.5), (-0.45, 4.0), (-0.42, 9.0), (0.42, 9.0), (0.45, 4.0), (0.5, 0.5), (0.55, 0.0)], region="stem-bark"))
        out.append(poly("bunch-l", "bunch-l", "body", 2.0, 0.3, [(0.35, 8.6), (0.9, 8.5), (0.95, 7.9), (0.6, 7.3), (0.3, 7.7)], region="fruit", followTip=True))
        out.append(poly("bunch-r", "bunch-r", "body", 2.0, 0.3, [(-0.35, 8.6), (-0.9, 8.5), (-0.95, 7.9), (-0.6, 7.3), (-0.3, 7.7)], region="fruit", followTip=True))
        return out
    segs = [("stem-1", 0.0, 3.3, 0.5, 0.45), ("stem-2", 3.3, 6.0, 0.45, 0.42), ("stem-3", 6.0, 8.4, 0.42, 0.4), ("crown", 8.4, 9.0, 0.4, 0.34)]
    for b, y0, y1, w0, w1 in segs:
        pts = [(0.0, y0), (0.0, (y0 + y1) / 2), (0.0, y1)]
        shape = poly(f"bole-{b}", b, "body", 1.0, 0.35, ribbon(pts, [w0, (w0 + w1) / 2, w1]), region="stem-bark", followTip=True)
        out.append(shape)
    out.append(poly("root-bulb", "root-plate", "body", 1.1, 0.35, [(-0.7, 0.0), (-0.55, 0.35), (-0.5, 0.6), (0.5, 0.6), (0.55, 0.35), (0.7, 0.0)], region="stem-bark"))
    out.append(poly("bunch-l", "bunch-l", "body", 2.0, 0.3, [(0.35, 8.6), (0.95, 8.5), (1.0, 7.9), (0.65, 7.3), (0.3, 7.7)], region="fruit", followTip=True))
    out.append(poly("bunch-r", "bunch-r", "body", 2.0, 0.3, [(-0.35, 8.6), (-0.95, 8.5), (-1.0, 7.9), (-0.65, 7.3), (-0.3, 7.7)], region="fruit", followTip=True))
    if scale <= 100:
        n = 0
        for b, y0, y1, *_ in segs[:3]:
            y = y0 + 0.45
            while y < y1 - 0.2:
                n += 1
                w = 0.42
                out.append(poly(f"leafbase-{n}", b, "texture", 2.0, 0.2, [(-w * 0.55, y + 0.12), (0.0, y - 0.1), (w * 0.55, y + 0.12)], closed=False, followTip=True))
                y += 0.6 if scale == 50 else 0.9
    if scale == 50:
        for k, y in enumerate((1.2, 3.0, 5.0, 7.0)):
            out.append(poly(f"fibre-{k + 1}", "stem-1" if y < 3.3 else ("stem-2" if y < 6 else "stem-3"), "texture", 2.05, 0.15, [(-0.3, y), (-0.28, y + 0.5), (-0.3, y + 1.0)], closed=False, followTip=True))
        for k in range(1, 4):
            out.append(poly(f"bunch-strand-{k}", "bunch-l", "detail", 2.1, 0.2, [(0.35 + 0.12 * k, 8.4), (0.4 + 0.15 * k, 7.6)], closed=False, followTip=True))
            out.append(poly(f"bunch-strand-r{k}", "bunch-r", "detail", 2.1, 0.2, [(-0.35 - 0.12 * k, 8.4), (-0.4 - 0.15 * k, 7.6)], closed=False, followTip=True))
    return out


def star500(view):
    items = [(n, AZ[n], 1.0) for n in AZ] + [(n, az, f) for n, az, f in SEC]
    items.sort(key=lambda it: it[1] % 360)
    pts = []
    for i, (n, az, f) in enumerate(items):
        rt = fr(n)[4] * f + (EXT.get(n, 0) if f == 1.0 else 0)
        a0 = az
        pts.append(polar(rt * 0.6, a0 - 9))
        pts.append(polar(rt, a0))
        pts.append(polar(rt * 0.6, a0 + 9))
        nxt = items[(i + 1) % len(items)]
        mid = (az + (nxt[1] % 360 if nxt[1] % 360 > az % 360 else nxt[1] % 360 + 360)) / 2 if False else None
    return [(XC + x, y) for x, y in pts]


def build(scale):
    front, plan = [], []
    if scale == 500:
        front += trunk_front(scale)
        items = [(n, AZ[n], 1.0) for n in AZ]
        tips = []
        for n, az, f in items:
            a, b, rk, yk, rt, yt = fr(n)
            s = math.sin(az * D2R)
            tips.append((rt * f * s, YC + (yt - YC) * f, rk * f * s, YC + (yk - YC) * f))
        tips.sort(key=lambda t: t[0])
        outline = [(0.0, YC)]
        for t in tips:
            outline += [(t[2] * 0.9, t[3] + 0.15), (t[0], t[1])]
        outline.append((0.0, YC))
        front.append(poly("crown", "crown", "body", 3.0, 0.55, outline, region="fronds"))
        pl = []
        its = sorted(items, key=lambda it: it[1] % 360)
        for n, az, f in its:
            rt = fr(n)[4] * f + (EXT.get(n, 0) if f == 1.0 else 0)
            rk = fr(n)[2] * f
            pl += [polar(rk * 0.55, az - 10), polar(rt, az), polar(rk * 0.55, az + 10)]
        plan.append(poly("crown", "crown", "body", 3.0, 0.55, [(XC + 0.917 * x, y) for x, y in pl], region="fronds"))
        return front, plan
    front += trunk_front(scale)
    front += front_fronds(scale, scale == 50)
    plan += plan_fronds(scale, scale == 50)
    plan.append(ellipse("crown-bud", "crown", "body", 8.0, 0.35, XC, 0.0, 0.42 if scale <= 100 else 0.48, 0.42 if scale <= 100 else 0.48, region="fronds"))
    if scale <= 100:
        plan.append(ellipse("crown-heart", "crown", "detail", 8.1, 0.2, XC, 0.0, 0.2, 0.2, region="fronds"))
    return front, plan


NOTES = {
    500: ("Front: the bole with both bunches and one crown outline of the eight frond tips; plan: one star of eight fronds around the crown centre, bole hidden.",
          "Vorderansicht: der Schaft mit beiden Fruchtständen und ein Kronenumriss aus den acht Wedelspitzen; Draufsicht: ein Stern aus acht Wedeln um die Kronenmitte, Schaft verdeckt."),
    200: ("Front: bole with three rings of leaf bases left out, both bunches, the eight bone fronds in two segments as slabs seen from their own azimuth. Plan: the eight fronds as tapering blades with a bud circle.",
          "Vorderansicht: Schaft, beide Fruchtstände, die acht Knochenwedel in zwei Segmenten als Platten aus ihrer eigenen Richtung gesehen. Draufsicht: die acht Wedel als verjüngte Klingen mit Herzkreis."),
    100: ("Front adds root bulb, chevron leaf bases and the frond rachis as thin solids. Plan adds the rachis of every frond and the heart ring.",
          "Die Vorderansicht ergänzt Wurzelballen und Winkelmuster der Blattbasen. Die Draufsicht ergänzt die Spindel jedes Wedels und den Herzring."),
    50: ("Front adds pinnate leaflets as comb solids along every frond, fibre lines and bunch strands; plan adds leaflet margin strokes along every frond.",
         "Die Vorderansicht ergänzt gefiederte Fiedern als Kammkörper an jedem Wedel, Faserlinien und Fruchtstrang-Linien; die Draufsicht ergänzt Fiederrandstriche an jedem Wedel."),
}

def rescale500():
    import re
    d = load("date-palm")
    for sh in d["asset"]["representations"]["500"]["geometry"]["elevation"]:
        if sh["id"] == "crown" and sh["kind"] == "path":
            nums = iter(re.findall(r"-?\d*\.?\d+", sh["d"]))
            letters = re.findall(r"[A-Za-z]", sh["d"])
            parts = re.split(r"([A-Za-z])", sh["d"])
            out = []
            for part in parts:
                if re.fullmatch(r"[A-Za-z]", part) or not part.strip():
                    out.append(part)
                    continue
                v = [float(x) for x in re.findall(r"-?\d*\.?\d+", part)]
                out.append(" ".join(f"{v[i]}" if i % 2 == 0 else f"{round(v[i] * 0.917, 4)}" for i in range(len(v))))
            sh["d"] = "".join(out)
    save("date-palm", d)


if __name__ == "__main__":
    rescale500()
    views = {sc: build(sc) for sc in (500, 200, 100, 50)}
    install("date-palm", views, NOTES)
    extents("date-palm")
