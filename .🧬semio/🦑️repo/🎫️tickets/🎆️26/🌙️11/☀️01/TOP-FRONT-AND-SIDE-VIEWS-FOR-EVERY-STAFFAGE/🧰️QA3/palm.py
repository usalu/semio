from kit import *

doc = load("date-palm")
r, bones = rest(doc)
XC, YC = r["crown"][0], r["crown"][1]

AZ = {"l1": 100, "l2": 188, "l3": 148, "l4": 125, "r1": -80, "r2": 8, "r3": -38, "r4": 55}
SEC = [("l3", 168, 0.8), ("l2", 232, 0.85), ("r1", -60, 0.9), ("r3", -12, 0.9), ("r2", 32, 0.88), ("r4", 80, 0.8), ("l4", 105, 0.85)]


EXT = {"l2": 0.27, "r2": 0.27}


def fr(n):
    a, b = f"fr-{n}a", f"fr-{n}b"
    k, t = tip(r, bones, a), tip(r, bones, b)
    return a, b, abs(k[0] - XC), k[1], abs(t[0] - XC), t[1]


def plan_fronds(scale, detail):
    out = []
    items = [(n, AZ[n], 1.0, "") for n in AZ] + [(n, az, f, "s") for n, az, f in SEC]
    items.sort(key=lambda it: fr(it[0])[5] - (0.5 if it[3] else 0))
    z = 3.0
    for n, az, f, tag in items:
        a, b, rk, yk, rt, yt = fr(n)
        rk, rt = rk * f, rt * f
        if not tag:
            rk, rt = rk + EXT.get(n, 0) * 0.4, rt + EXT.get(n, 0)
        kx, ky = polar(rk, az)
        tx, ty = polar(rt, az)
        kx += XC
        tx += XC
        w = 0.34 * (0.85 if tag else 1.0)
        if scale == 200:
            out.append(capsule(f"fr-{n}{tag}-a", a, "body", z, 0.55, XC, 0, kx, ky, w * 0.5, w, region="fronds", followTip=True))
            out.append(capsule(f"fr-{n}{tag}-b", b, "body", z + 0.001, 0.55, kx, ky, tx, ty, w, 0.03, region="fronds", followTip=True))
        else:
            out.append(capsule(f"fr-{n}{tag}-a", a, "body", z, 0.55, XC, 0, kx, ky, w * 0.45, w, region="fronds", followTip=True))
            out.append(capsule(f"fr-{n}{tag}-b", b, "body", z + 0.001, 0.55, kx, ky, tx, ty, w, 0.03, region="fronds", followTip=True))
            out.append(poly(f"rachis-{n}{tag}-a", a, "detail", z + 0.5, 0.2, [(XC, 0.0), (kx, ky)], closed=False))
            out.append(poly(f"rachis-{n}{tag}-b", b, "detail", z + 0.501, 0.2, [(kx, ky), (tx, ty)], closed=False))
        if detail:
            for seg, bb, p0, p1 in (("a", a, (XC, 0.0), (kx, ky)), ("b", b, (kx, ky), (tx, ty))):
                L = math.hypot(p1[0] - p0[0], p1[1] - p0[1])
                ux, uy = (p1[0] - p0[0]) / L, (p1[1] - p0[1]) / L
                nx, ny = -uy, ux
                zig = []
                steps = 8
                for i in range(steps):
                    t0, t1 = i / steps, (i + 0.6) / steps
                    wd = w * (0.5 if seg == "a" else 0.9) * (1 - (0 if seg == "a" else t0 * 0.9))
                    c0 = (p0[0] + ux * L * t0, p0[1] + uy * L * t0)
                    c1 = (p0[0] + ux * L * t1, p0[1] + uy * L * t1)
                    zig += [(c0[0] + nx * wd, c0[1] + ny * wd), c1, (c0[0] - nx * wd, c0[1] - ny * wd), c1]
                out.append(poly(f"leaflets-{n}{tag}-{seg}", bb, "texture", z + 0.6, 0.2, zig, closed=False))
        z += 0.01
    return out


def front_fronds(scale, detail):
    out = []
    items = [(n, AZ[n], 1.0, "") for n in AZ] + [(n, az, f, "s") for n, az, f in SEC]
    items.sort(key=lambda it: math.cos(it[1] * D2R))
    z = 3.0
    for n, az, f, tag in items:
        a, b, rk, yk, rt, yt = fr(n)
        s, c = math.sin(az * D2R), math.cos(az * D2R)
        yk = YC + (yk - YC) * f
        yt = YC + (yt - YC) * f
        rk, rt = rk * f, rt * f
        w = (0.4 * abs(c) + 0.12) * (0.85 if tag else 1.0)
        ca = [(rk * s * k / 4, YC + (yk - YC) * k / 4) for k in range(5)]
        cb = [(lerp(rk * s, rt * s, k / 4), lerp(yk, yt, k / 4)) for k in range(5)]
        out.append(poly(f"fr-{n}{tag}-a", a, "body", z + 0.4 * c, 0.55, ribbon(ca, [w * 0.4, w * 0.8, w, w, w * 0.95]), region="fronds", followTip=True))
        out.append(poly(f"fr-{n}{tag}-b", b, "body", z + 0.4 * c + 0.001, 0.55, ribbon(cb, [w * 0.95, w, w * 0.8, w * 0.45, 0.02]), region="fronds", followTip=True))
        if scale <= 100:
            out.append(poly(f"rachis-{n}{tag}-a", a, "detail", z + 0.4 * c + 0.5, 0.2, ca, closed=False, followTip=True))
            out.append(poly(f"rachis-{n}{tag}-b", b, "detail", z + 0.4 * c + 0.501, 0.2, cb, closed=False, followTip=True))
        if detail:
            for seg, bb, cc, k0 in (("a", a, ca, 0.5), ("b", b, cb, 0.9)):
                zig = []
                for i in range(len(cc) - 1):
                    for j in range(2):
                        t = (j + 0.5) / 2
                        px, py = lerp(cc[i][0], cc[i + 1][0], t), lerp(cc[i][1], cc[i + 1][1], t)
                        zig += [(px, py + w * k0 * 0.5 * (1 - 0.2 * i)), (px + 0.02 * (j - 0.5), py), (px, py - w * k0 * 0.45), (px, py)]
                out.append(poly(f"leaflets-{n}{tag}-{seg}", bb, "texture", z + 0.4 * c + 0.6, 0.2, zig, closed=False, followTip=True))
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
        items = [(n, AZ[n], 1.0) for n in AZ] + [(n, az, f) for n, az, f in SEC]
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
            pl += [polar(rk * 0.55, az - 14), polar(rt, az), polar(rk * 0.55, az + 14)]
        plan.append(poly("crown", "crown", "body", 3.0, 0.55, [(XC + x, y) for x, y in pl], region="fronds"))
        return front, plan
    front += trunk_front(scale)
    front += front_fronds(scale, scale == 50)
    plan += plan_fronds(scale, scale == 50)
    plan.append(ellipse("crown-bud", "crown", "body", 8.0, 0.35, XC, 0.0, 0.42 if scale <= 100 else 0.48, 0.42 if scale <= 100 else 0.48, region="fronds"))
    if scale <= 100:
        plan.append(ellipse("crown-heart", "crown", "detail", 8.1, 0.2, XC, 0.0, 0.2, 0.2, region="fronds"))
    return front, plan


NOTES = {
    500: ("Front: the bole with both bunches and one crown outline of fifteen frond tips; plan: one star of fifteen fronds around the crown centre, bole hidden.",
          "Vorderansicht: der Schaft mit beiden Fruchtständen und ein Kronenumriss aus fünfzehn Wedelspitzen; Draufsicht: ein Stern aus fünfzehn Wedeln um die Kronenmitte, Schaft verdeckt."),
    200: ("Front: bole with three rings of leaf bases left out, both bunches, fifteen fronds in two segments seen from the side. Plan: fifteen fronds as tapering blades with a bud circle; the eight main fronds follow the bones, seven short ones fill the gaps.",
          "Vorderansicht: Schaft, beide Fruchtstände, fünfzehn Wedel in zwei Segmenten von der Seite. Draufsicht: fünfzehn Wedel als verjüngte Klingen mit Herzkreis; die acht Hauptwedel folgen den Knochen, sieben kurze füllen die Lücken."),
    100: ("Front adds root bulb, chevron leaf bases and the frond rachis. Plan adds the rachis of every frond and the heart ring.",
          "Die Vorderansicht ergänzt Wurzelballen, Winkelmuster der Blattbasen und die Wedelspindeln. Die Draufsicht ergänzt die Spindel jedes Wedels und den Herzring."),
    50: ("Front and plan add pinnate leaflets along every frond, fibre lines and bunch strands.",
         "Vorder- und Draufsicht ergänzen gefiederte Fiedern an jedem Wedel, Faserlinien und Fruchtstrang-Linien."),
}

if __name__ == "__main__":
    views = {sc: build(sc) for sc in (500, 200, 100, 50)}
    install("date-palm", views, NOTES)
    extents("date-palm")
