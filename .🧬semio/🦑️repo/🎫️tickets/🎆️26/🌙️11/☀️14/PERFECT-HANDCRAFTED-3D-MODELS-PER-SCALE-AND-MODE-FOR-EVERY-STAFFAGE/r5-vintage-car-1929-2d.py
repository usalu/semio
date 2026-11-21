"""✏️ The 2D drawings of the round 5 vintage saloon (round 3 drawings; the rear springs leave the front views because the rear wings hide them down to a sliver, the 1:500 underbody band reaches down to the axle line) as the true orthographic views of each scale's body: every shape stated from the same design lines as the solids. Executed by r3-vintage-car-1929-root.py after the kit."""
GEOM = {"kind", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d", "points", "closed", "followTip"}
n3 = lambda v: round(v + 0.0, 4)


def g_rect(x, y, w, h, r=None):
    d = {"kind": "rect", "x": n3(x), "y": n3(y), "w": n3(w), "h": n3(h)}
    if r:
        d["r"] = n3(r)
    return d


def g_cap(x0, y0, x1, y1, r0, r1=None):
    return {"kind": "capsule", "x0": n3(x0), "y0": n3(y0), "x1": n3(x1), "y1": n3(y1), "r0": n3(r0), "r1": n3(r0 if r1 is None else r1)}


def g_ell(cx, cy, rx, ry=None):
    return {"kind": "ellipse", "cx": n3(cx), "cy": n3(cy), "rx": n3(rx), "ry": n3(rx if ry is None else ry)}


def g_path(pts, close=True):
    return {"kind": "path", "d": pathd(pts, close)}


class Sheet2D:
    """One view of one scale: shapes keep the metadata the drawing had under the same id; new parts state theirs."""

    def __init__(self, old):
        self.old = {s["id"]: s for s in old}
        self.out = []

    def add(self, id, geom, **meta):
        o = self.old.get(id)
        base = {k: v for k, v in (o or {}).items() if k not in GEOM}
        if o is None:
            assert {"bone", "role", "z", "tone"} <= set(meta), id
            base = {"id": id, "filled": True}
        if "region" in meta:
            base.pop("regions", None)
        if "regions" in meta:
            base.pop("region", None)
        base.update(meta)
        if geom["kind"] == "path" and not geom["d"].endswith("Z"):
            base["filled"] = False
        d = {"id": id}
        for k in ("bone", "role", "z", "filled", "tone", "region", "regions", "material", "penMm"):
            if k in base:
                d[k] = base[k]
        d.update(geom)
        self.out.append(d)
        return d


NEW = dict(bone=CH, role="detail", z=1, tone=0.18, region="body-panels", material=EX)
flat = lambda half: [[z, y] for y, z in ([[y, -z] for y, z in half] + [[y, z] for y, z in half[::-1][1:]])]
rail_side = lambda: [[x, yc + h / 2] for x, yc, z, h in RAIL] + [[x, yc - h / 2] for x, yc, z, h in RAIL][::-1]
rail_plan = lambda sg: [[x, sg * (z + 0.0225)] for x, yc, z, h in RAIL] + [[x, sg * (z - 0.0225)] for x, yc, z, h in RAIL][::-1]
roof_side = lambda rows: [[x, yc + ry] for x, yc, rx, ry in rows] + [[x, yc - ry] for x, yc, rx, ry in rows][::-1]
roof_plan = lambda rows: [[x, rx] for x, yc, rx, ry in rows] + [[x, -rx] for x, yc, rx, ry in rows][::-1]
wing_plan = lambda line, zc, rx, sg: [[p[0], sg * (z + r)] for p, z, r in zip(line, zc, rx)] + [[p[0], sg * (z - r)] for p, z, r in zip(line, zc, rx)][::-1]
bowl_side = lambda x0, c: [[x0 + a, c + r] for a, r in BOWL[1:-1]] + [[x0 + a, c - r] for a, r in BOWL[1:-1]][::-1]
bump_plan = lambda sg: [[sg * (XBUMP - 0.006 + dx + 0.006), z] for z, dx in zip(BUMP_Z, BUMP_DX)] + [[sg * (XBUMP - 0.006 + dx - 0.006), z] for z, dx in zip(BUMP_Z, BUMP_DX)][::-1]
tail_side = lambda: band([p[:2] for p in EXH_TAIL[:-1]] + [[-2.122, 0.4155]], [0.025] * len(EXH_TAIL))


def superellipse(cx, cy, rx, ry, e, n):
    out = []
    for k in range(4 * n):
        a = 2 * math.pi * k / (4 * n)
        c, s_ = math.cos(a), math.sin(a)
        out.append([cx + rx * math.copysign(abs(c) ** (2 / e), c), cy + ry * math.copysign(abs(s_) ** (2 / e), s_)])
    return out


def se_top(z, cy, rx, ry, e):
    return cy + ry * max(0.0, 1 - abs(z / rx) ** e) ** (1 / e) if abs(z) < rx else None


def hood_front(zs):
    """Front view of the bonnet at the scuttle: flat sides and the rounded shoulders of its rear section."""
    half = [[z, se_top(z, 1.185 - HOOD_RY, HOOD_ZR, HOOD_RY, 4) or 1.185 - HOOD_RY] for z in zs]
    return [[HOOD_ZR, YSILL]] + half + [[-z, y] for z, y in half[::-1] if z > 0] + [[-HOOD_ZR, YSILL]]


HOOD_ZS = {50: (0.50, 0.49, 0.46, 0.40, 0.30, 0.15, 0.0), 100: (0.50, 0.48, 0.42, 0.25, 0.0)}


def ring_d(r0, r1):
    f = lambda v: ("%.3f" % v).rstrip("0").rstrip(".")
    c = lambda r: f"M {f(r)},0 A {f(r)},{f(r)} 0 1,0 -{f(r)},0 A {f(r)},{f(r)} 0 1,0 {f(r)},0 Z"
    return {"kind": "path", "d": c(r1) + " " + c(r0)}


def wheels2d(v, old, drum):
    """Wheel-bone shapes of the side view: kept as drawn, the rim as the true ring so that the gaps between the spokes are open down to the drum."""
    for s in old:
        if s["bone"] in (WF, WR):
            t = s["id"].split("-")[0]
            if t == "rim":
                v.add(s["id"], ring_d(0.222, 0.256))
            elif t == "drum":
                v.add(s["id"], g_ell(0, 0, drum))
            elif t == "bead":
                v.add(s["id"], ring_d(0.256, 0.30))
            elif t == "tyre":
                v.add(s["id"], ring_d(0.245, RW))
            else:
                v.out.append(s)


def elevation50(old):
    v = Sheet2D(old)
    v.add("frame", g_path(rail_side()))
    for k, (x, y0, y1) in enumerate(((0.45, 0.385, 0.42), (-0.50, 0.42, 0.435), (-1.55, 0.52, 0.55))):
        v.add(f"exhaust-hanger-{k + 1}", g_rect(x - 0.015, y0 - 0.01, 0.03, y1 - y0 + 0.02))
    for k, x in enumerate((-0.75, 0.0, 0.75)):
        v.add(f"running-bracket-{k + 1}", g_rect(x - 0.02, 0.39, 0.04, 0.03))
    v.add("running-board", g_rect(-1.02, 0.42, 2.05, 0.035, 0.012))
    v.add("sill-apron", g_rect(-1.07, 0.44, 1.60, 0.085), bone=CH, role="body", z=1.6, tone=0.5, region="body-panels", material=P)
    v.add("spring-front", g_path(band(SPRING_F, SPRING_T)), **dict(NEW, z=0.75, tone=0.25))
    v.add("spring-rear", g_path(band(SPRING_R, SPRING_T)), **dict(NEW, z=0.75, tone=0.25))
    v.add("wing-stay", g_path([[1.96, 0.50], [1.96, 0.755]], False), **dict(NEW, z=0.75, tone=0.25, material=P))
    v.add("propeller-shaft", g_cap(XR + 0.2, RW, 0.50, 0.46, 0.03), **dict(NEW, z=0.7, tone=0.25))
    v.add("fuel-tank", g_rect(-1.945, 0.435, 0.20, 0.19, 0.06), **dict(NEW, z=0.7, tone=0.2, material=P))
    v.add("wheelhouse", g_path(arc(XR, RW, ARCH_R, 25.5, 154.5, 10)), bone=CH, role="detail", z=1.9, tone=0.3, region="body-panels", material=P)
    v.add("tub", g_path(TUB))
    v.add("cabin", g_path(CABIN))
    v.add("cowl", g_path([[0.50, YSILL], [0.74, YSILL]] + [[x, top] for x, top, hw, e in COWL][::-1]), bone=CH, role="body", z=2.2, tone=0.6, region="body-panels", material=P)
    v.add("roof-slab", g_path(roof_side(ROOF)))
    v.add("hood", g_path([[0.72, YSILL], [1.69, YSILL], [1.69, 1.185], [0.72, 1.185]]))
    v.add("fender-front", g_path(band(WING_F, WING_F_RY)))
    v.add("fender-rear", g_path(band(WING_B, WING_B_RY)))
    v.add("radiator-shell", g_rect(1.68, 0.52, 0.12, 0.68))
    v.add("radiator-neck", g_cap(1.74, 1.20, 1.74, 1.222, 0.028))
    v.add("radiator-cap", g_ell(1.74, 1.241, 0.045, 0.016))
    for i, x in enumerate(LOUVRES):
        v.add(f"louvre-{i + 1}", g_cap(x, 0.767, x, 0.973, 0.007), bone=CH, role="detail", z=2.5, tone=0.12, region="engine-bay", material=EM)
    v.add("headlamp-bowl", g_path(bowl_side(LAMP[0], LAMP[1])))
    v.add("headlamp-lens", g_ell(LAMP[0] + 0.173, LAMP[1], 0.011, 0.105))
    v.add("lamp-stalk", g_cap(1.83, 0.862, 1.845, 0.895, 0.016))
    v.add("lamp-bar", g_ell(1.83, 0.862, 0.014), bone=CH, role="detail", z=3.15, tone=0.74, region="fenders", material=NI)
    v.add("fender-lamp", g_cap(1.63, 0.888, 1.665, 0.888, 0.03))
    for nm, w in (("front", WIN_F), ("rear", WIN_R), ("quarter", WIN_Q)):
        v.add(f"window-{nm}", g_path(fillet(w, 0.03, 2)))
    v.add("windscreen", g_path([[0.511, YBELT], [0.471, YCANT]], False))
    arch_y = RW + math.sqrt(ARCH_R ** 2 - (X_C - XR) ** 2) + 0.01
    v.add("door-hinge-gap", g_path([[X_A, YBELT], [X_A, 0.56]], False))
    v.add("door-gap-b", g_path([[X_B, YBELT], [X_B, 0.56]], False))
    v.add("door-gap-rear", g_path([[X_C, YBELT], [X_C, arch_y]], False))
    v.add("belt-line", g_path([[-1.90, YBELT], [0.52, YBELT]], False))
    v.add("handle-front", g_cap(X_B + 0.065, 1.06, X_B + 0.155, 1.06, 0.009))
    v.add("handle-rear", g_cap(X_B - 0.155, 1.06, X_B - 0.065, 1.06, 0.009))
    v.add("seat-front", g_path([[-0.27, 1.225], [-0.2045, 1.225], [-0.227, 1.386], [-0.27, 1.38]]))
    v.add("seat-rear", g_path([[-1.6245, 1.225], [-1.655, 1.3975], [-1.729, 1.384], [-1.766, 1.225]]))
    v.add("steering-wheel", g_cap(0.2445, 1.225, 0.372, 1.371, 0.013))
    v.add("drag-link", g_path([[1.07, 0.40], [XF - 0.10, RW - 0.03]], False), **dict(NEW, z=0.72, tone=0.25))
    v.add("number-plate", g_rect(-2.002, 0.765, 0.012, 0.11))
    v.add("tail-lamp", g_rect(-2.03, 0.918, 0.045, 0.064, 0.015))
    for end, sg in (("front", 1), ("rear", -1)):
        x = XBUMP - 0.087 if sg > 0 else -XBUMP
        for nm, y in zip(("lower", "upper"), BUMP_Y):
            v.add(f"bumper-{end}-{nm}", g_rect(x, y - 0.0225, 0.087, 0.045, 0.006), bone=CH, role="body", z=3.4, tone=0.8, region="body-panels", material=NI)
        v.add(f"bumper-{end}-clamp", g_rect(sg * (XBUMP - 0.012) - 0.01, 0.445, 0.02, 0.15, 0.006), bone=CH, role="detail", z=3.45, tone=0.8, region="body-panels", material=NI)
    v.add("bracket-front", g_cap(2.07, 0.505, 2.209, 0.52, 0.014))
    v.add("bracket-rear", g_cap(-1.95, 0.56, -2.209, 0.52, 0.014))
    v.add("exhaust-manifold", g_cap(0.98, 0.78, 1.50, 0.78, 0.03))
    v.add("exhaust-downpipe", g_path(band([p[:2] for p in EXH[:3]], [0.027] * 3)))
    v.add("exhaust-pipe", g_cap(EXH[2][0], 0.36, EXH[3][0], 0.36, 0.025))
    v.add("silencer", g_path([[-0.13, 0.33], [-0.13, 0.39], [-0.16, 0.43], [-0.83, 0.43], [-0.86, 0.39], [-0.86, 0.33], [-0.83, 0.29], [-0.16, 0.29]]))
    v.add("tail-pipe", g_path(tail_side()))
    v.add("spare-tyre", g_ell(SPARE[0], SPARE[1], RW))
    v.add("spare-ring", g_ell(SPARE[0], SPARE[1], 0.30))
    v.add("spare-cover", g_ell(SPARE[0], SPARE[1], 0.24))
    v.add("spare-boss", g_ell(SPARE[0], SPARE[1], 0.04))
    wheels2d(v, old, 0.16)
    return v.out


def front50(old):
    v = Sheet2D(old)
    for t, bone in (("r", WR), ("f", WF)):
        for sd, sg in (("l", 1), ("r", -1)):
            x = ZW - TW / 2 if sg > 0 else -ZW - TW / 2
            v.add(f"tyre-{t}{sd}", g_rect(x, 0, TW, 2 * RW, 0.05))
            if t == "f":
                v.add(f"drum-{t}{sd}", g_rect(0.588 if sg > 0 else -0.635, 0.21, 0.047, 0.32), bone=bone, role="detail", z=3.95, tone=0.3, region="brakes", material=CI)
                v.add(f"hub-{t}{sd}", g_rect(0.765 if sg > 0 else -0.775, 0.32, 0.01, 0.10, 0.004))
    v.add("frame", g_rect(-0.4425, 0.42, 0.885, 0.10))
    zs = [-0.59, -0.52, -0.44, -0.38, 0.38, 0.44, 0.52, 0.59]
    ys = [0.0, -0.005, -0.035, -0.06, -0.06, -0.035, -0.005, 0.0]
    v.add("axle-f", g_path([[z, RW + y + 0.03] for z, y in zip(zs, ys)] + [[z, RW + y - 0.03] for z, y in zip(zs, ys)][::-1]))
    v.add("axle-r", g_rect(-0.59, RW - 0.035, 1.18, 0.07), bone=AR, role="detail", z=0.8, tone=0.3, material=CI)
    v.add("differential", g_ell(0, RW, 0.14), bone=AR, role="detail", z=0.85, tone=0.3, material=CI)
    v.add("silencer", g_ell(-0.30, 0.36, 0.07))
    for sd, sg in (("l", 1), ("r", -1)):
        m = lambda x, w: x if sg > 0 else -x - w
        v.add(f"spring-front-{sd}", g_rect(m(0.3175, 0.045), 0.33, 0.045, 0.14), **dict(NEW, tone=0.25))
        v.add(f"running-bracket-{sd}", g_rect(m(0.42, 0.40), 0.39, 0.40, 0.03), **NEW)
        v.add(f"running-board-{sd}", g_rect(m(0.62, 0.21), 0.42, 0.21, 0.035, 0.012))
        v.add(f"sill-apron-{sd}", g_rect(m(0.592, 0.03), 0.44, 0.03, 0.085), bone=CH, role="body", z=1.6, tone=0.5, region="body-panels", material=P)
        v.add(f"louvre-{sd}", g_rect(m(0.337, 0.123), 0.76, 0.123, 0.22, 0.01))
        v.add(f"fender-{sd}", {"kind": "poly", "closed": True, "points": [[n3(sg * z), y] for z, y in ((0.42, 0.47), (0.42, 0.434), (0.56, 0.434), (0.57, 0.40), (0.63, 0.322), (0.78, 0.322), (0.84, 0.40), (0.85, 0.48), (0.85, 0.835), (0.825, 0.86), (0.585, 0.86), (0.56, 0.835), (0.56, 0.52), (0.47, 0.52))]})
        v.add(f"lamp-stalk-{sd}", g_rect(m(0.414, 0.032), 0.86, 0.032, 0.035))
        v.add(f"headlamp-bowl-{sd}", g_ell(sg * LAMP[2], LAMP[1], 0.118))
        v.add(f"headlamp-lens-{sd}", g_ell(sg * LAMP[2], LAMP[1], 0.105))
        v.add(f"fender-lamp-{sd}", g_ell(sg * 0.72, 0.888, 0.03), bone=CH, role="detail", z=3.2, tone=0.85, region="fenders", material=NI)
    v.add("tub", g_rect(-ZB, YSILL, 2 * ZB, YBELT - YSILL, 0.01))
    k = (YHEAD - YBELT) / 0.46 * 0.04
    v.add("cabin", g_path([[-ZB, YBELT], [ZB, YBELT], [ZB - k, YHEAD], [-ZB + k, YHEAD]]))
    v.add("roof-slab", g_path(superellipse(0, 1.6525, 0.595, 0.0475, ROOF_E, 5)))
    for sd, sg in (("l", 1), ("r", -1)):
        v.add(f"handle-{sd}", g_rect(ZB if sg > 0 else -ZB - 0.031, 1.051, 0.031, 0.018), bone=CH, role="detail", z=7.2, tone=0.8, region="interior", material=NI)
    v.add("hood", g_path(hood_front(HOOD_ZS[50])))
    v.add("windscreen", g_path([[-0.54, 1.215], [0.54, 1.215], [0.51, 1.578], [-0.51, 1.578]]))
    v.add("radiator-shell", g_path(flat(SHELL)))
    v.add("radiator-core", g_path(flat(CORE)), bone=CH, role="detail", z=2.65, tone=0.15, region="radiator", material=EM)
    v.add("radiator-neck", g_rect(-0.028, 1.19, 0.056, 0.04))
    v.add("radiator-cap", g_rect(-0.045, 1.225, 0.09, 0.032, 0.012))
    v.add("spare-tyre", g_rect(SPARE[2] - TW / 2, SPARE[1] - RW, TW, 2 * RW, 0.05))
    v.add("lamp-bar", g_rect(-0.60, 0.848, 1.20, 0.028), bone=CH, role="detail", z=3.15, tone=0.74, region="fenders", material=NI)
    for nm, y in zip(("lower", "upper"), BUMP_Y):
        v.add(f"bumper-front-{nm}", g_rect(-0.74, y - 0.0225, 1.48, 0.045, 0.015))
    for cn, cz in (("r", -0.34), ("c", 0.0), ("l", 0.34)):
        v.add(f"bumper-front-clamp-{cn}", g_rect(cz - 0.015, 0.445, 0.03, 0.15, 0.006), bone=CH, role="detail", z=3.45, tone=0.8, region="body-panels", material=NI)
    return v.out


def plan50(old):
    v = Sheet2D(old)
    for sd, sg in (("l", 1), ("r", -1)):
        v.add(f"frame-{sd}", g_path(rail_plan(sg)))
    for nm, x, hw in (("front", 1.97, 0.34), ("radiator", 1.74, 0.34), ("gearbox", 0.62, 0.41), ("mid", -0.45, 0.42), ("kick", -1.10, 0.42), ("rear", -1.93, 0.42)):
        v.add(f"crossmember-{nm}", g_rect(x - 0.025, -hw, 0.05, 2 * hw), **dict(NEW, role="body"))
    v.add("axle-f-l", g_rect(XF - 0.022, 0.34, 0.044, 0.25))
    v.add("axle-f-r", g_rect(XF - 0.022, -0.59, 0.044, 0.25))
    v.add("axle-f-mid", g_rect(XF - 0.022, -0.34, 0.044, 0.68))
    v.add("axle-r-l", g_rect(XR - 0.035, 0.14, 0.07, 0.45))
    v.add("axle-r-r", g_rect(XR - 0.035, -0.59, 0.07, 0.45))
    v.add("axle-r-mid", g_rect(XR - 0.14, -0.14, 0.36, 0.28, 0.08))
    for sd, sg in (("l", 1), ("r", -1)):
        m = lambda z, w: z if sg > 0 else -z - w
        for k, x in enumerate((-0.75, 0.0, 0.75)):
            v.add(f"running-bracket-{k + 1}-{sd}", g_rect(x - 0.02, m(0.42, 0.40), 0.04, 0.40))
        for k, x in enumerate((0.98, 1.50)):
            v.add(f"engine-bearer-{k + 1}-{sd}", g_rect(x - 0.035, m(0.10, 0.27), 0.07, 0.27))
        v.add(f"running-board-{sd}", g_rect(-1.02, m(0.62, 0.21), 2.05, 0.21, 0.012))
    v.add("spare-mount", g_cap(0.80, -0.41, SPARE[0], SPARE[2] + 0.02, 0.02))
    v.add("tail-pipe", g_cap(-1.72, -0.35, -2.10, -0.35, 0.025))
    v.add("drag-link", g_path([[1.07, -0.345], [XF - 0.10, -0.50]], False), **dict(NEW, z=0.72, tone=0.25))
    v.add("tub", {"kind": "poly", "closed": True, "points": [[-1.99, -ZB], [0.52, -ZB], [0.52, ZB], [-1.99, ZB]]})
    v.add("cowl", g_path([[x, hw] for x, top, hw, e in COWL] + [[x, -hw] for x, top, hw, e in COWL][::-1]), bone=CH, role="body", z=2.2, tone=0.6, region="body-panels", material=P)
    v.add("hood", g_path([[0.72, -HOOD_ZR], [1.69, -HOOD_ZF], [1.69, HOOD_ZF], [0.72, HOOD_ZR]]))
    v.add("roof-slab", g_path(roof_plan(ROOF)))
    v.add("radiator-shell", g_rect(1.68, -0.297, 0.12, 0.594, 0.02))
    v.add("radiator-cap", g_ell(1.74, 0, 0.045))
    v.add("spare-tyre", g_rect(SPARE[0] - RW, SPARE[2] - TW / 2, 2 * RW, TW, 0.05))
    for sd, sg in (("l", 1), ("r", -1)):
        v.add(f"fender-f{sd}", g_path(wing_plan(WING_F, WING_F_Z, WING_F_RX, sg)))
        v.add(f"fender-r{sd}", g_path(wing_plan(WING_B, WING_B_Z, WING_B_RX, sg)))
        v.add(f"headlamp-bowl-{sd}", g_path(bowl_side(LAMP[0], sg * LAMP[2])))
        v.add(f"headlamp-lens-{sd}", g_ell(LAMP[0] + 0.173, sg * LAMP[2], 0.011, 0.105))
        v.add(f"fender-lamp-{sd}", g_cap(1.63, sg * 0.72, 1.665, sg * 0.72, 0.03), bone=CH, role="detail", z=3.2, tone=0.85, region="fenders", material=NI)
        v.add(f"bracket-front-{sd}", g_cap(2.07, sg * 0.34, 2.209, sg * 0.34, 0.014))
        v.add(f"bracket-rear-{sd}", g_cap(-1.95, sg * 0.42, -2.209, sg * 0.42, 0.014))
    v.add("lamp-bar", g_rect(1.816, -0.60, 0.028, 1.20), bone=CH, role="detail", z=3.15, tone=0.74, region="fenders", material=NI)
    v.add("bumper-front", g_path(bump_plan(1)))
    v.add("bumper-rear", g_path(bump_plan(-1)))
    v.add("number-plate", g_rect(-2.002, -0.2, 0.012, 0.4))
    v.add("tail-lamp", g_rect(-2.03, 0.388, 0.045, 0.064, 0.01))
    return v.out


# ---------------------------------------------------------------- 1:100 and 1:200
BODY = dict(bone=CH, role="body", z=2.2, tone=0.6, region="body-panels", material=P)
NICKEL = dict(bone=CH, role="body", z=3.4, tone=0.8, region="body-panels", material=NI)
bump_plan_n = lambda sg, t: [[sg * (XBUMP - t + dx + t), z] for z, dx in zip(*BUMP100)] + [[sg * (XBUMP - t + dx - t), z] for z, dx in zip(*BUMP100)][::-1]
rails_side = lambda rows: [[x, yc + h / 2] for x, yc, z, h in rows] + [[x, yc - h / 2] for x, yc, z, h in rows][::-1]
rails_plan = lambda rows, sg: [[x, sg * (z + 0.0225)] for x, yc, z, h in rows] + [[x, sg * (z - 0.0225)] for x, yc, z, h in rows][::-1]
BOWLS = {100: [[0.03, 0.08], [0.10, 0.112], [0.165, 0.118]], 200: [[0.04, 0.09], [0.165, 0.118]]}
bowl_n = lambda den, c, tip: [[LAMP[0], c]] + [[LAMP[0] + a, c + r] for a, r in BOWLS[den]] + ([[LAMP[0] + 0.184, c]] if tip else []) + [[LAMP[0] + a, c - r] for a, r in BOWLS[den]][::-1]


def elevation_n(old, den):
    v = Sheet2D(old)
    fine = den == 100
    v.add("frame", g_path(rails_side(RAILS[den])))
    if fine:
        for sd in ("l", "r"):
            for k, x in enumerate((-0.55, 0.55)):
                v.add(f"running-bracket-{k + 1}-{sd}", g_rect(x - 0.02, 0.39, 0.04, 0.03))
        v.add("spring-front", g_path(band([SPRING_F[0], SPRING_F[3], SPRING_F[6]], [0.01, 0.03, 0.01])), **dict(NEW, z=0.75, tone=0.25))
        v.add("spring-rear", g_path(band([SPRING_R[0], SPRING_R[3], SPRING_R[6]], [0.01, 0.03, 0.01])), **dict(NEW, z=0.75, tone=0.25))
        v.add("fuel-tank", g_rect(-1.945, 0.435, 0.20, 0.19, 0.06), **dict(NEW, z=0.7, tone=0.2, material=P))
    else:
        v.add("running-bracket-l", g_rect(-0.03, 0.39, 0.06, 0.03))
    v.add("running-board", g_rect(-1.02, 0.42, 2.05, 0.035))
    v.add("sill-apron", g_rect(-1.07, 0.44, 1.60, 0.085), bone=CH, role="body", z=1.6, tone=0.5, region="body-panels", material=P)
    v.add("propeller-shaft", g_cap(XR + 0.1, RW, 0.50 if fine else 0.72, 0.46 if fine else 0.47, 0.03), **dict(NEW, z=0.7, tone=0.25))
    v.add("wheelhouse", g_path(arc(XR, RW, ARCH_R, 25.5, 154.5, ARCHN[den])), bone=CH, role="detail", z=1.9, tone=0.3, region="body-panels", material=P)
    v.add("tub", g_path(tub_of(den)))
    v.add("cabin", g_path(CABIN))
    v.add("cowl", g_path([[0.50, YSILL], [0.74, YSILL]] + [[x, top] for x, top, hw, e in COWLS[den]][::-1][:-1] + [[0.50, YBELT]]), **BODY)
    v.add("roof-slab", g_path(roof_side(ROOFS[den])))
    v.add("hood", g_path([[0.72, YSILL], [1.69, YSILL], [1.69, 1.185], [0.72, 1.185]]))
    v.add("fender-front", g_path(band(WINGF[den][0], WINGF[den][3])))
    v.add("fender-rear", g_path(band(WINGB[den][0], WINGB[den][3])))
    v.add("radiator-shell", g_rect(1.68, 0.52, 0.12, 0.68))
    if fine:
        v.add("radiator-neck", g_cap(1.74, 1.205, 1.74, 1.21, 0.03))
        v.add("radiator-cap", g_ell(1.74, 1.24, 0.045, 0.015))
        for i, x in enumerate(LOUVRES100):
            v.add(f"louvre-{i + 1}", g_cap(x, 0.775, x, 0.965, 0.015))
        v.add("headlamp-bowl", g_path(bowl_n(100, LAMP[1], False)))
        v.add("headlamp-lens", g_ell(LAMP[0] + 0.173, LAMP[1], 0.011, 0.105))
        v.add("lamp-stalk", g_cap(1.83, 0.862, 1.845, 0.895, 0.016))
        v.add("lamp-bar", g_ell(1.83, 0.862, 0.015), bone=CH, role="detail", z=3.15, tone=0.74, region="fenders", material=NI)
    else:
        v.add("radiator-cap", g_path([[1.705, 1.19], [1.775, 1.19], [1.775, 1.24], [1.74, 1.255], [1.705, 1.24]]))
        v.add("louvre-band", g_rect(0.90, 0.76, 0.60, 0.22))
        v.add("headlamp-bowl", g_path(bowl_n(200, LAMP[1], True)))
        v.add("lamp-stalk", g_cap(1.86, 0.53, 1.85, 0.88, 0.016), bone=CH, role="detail", z=3.1, tone=0.5, region="fenders", material=NI)
    for nm, w in zip(("front", "rear", "quarter"), WINS):
        v.add(f"window-{nm}", g_path(w))
    v.add("windscreen", g_path([[0.511, YBELT], [0.471, YCANT]], False))
    if fine:
        arch_y = RW + math.sqrt(ARCH_R ** 2 - (X_C - XR) ** 2) + 0.01
        v.add("door-hinge-gap", g_path([[X_A, YBELT], [X_A, 0.56]], False))
        v.add("door-gap-b", g_path([[X_B, YBELT], [X_B, 0.56]], False))
        v.add("door-gap-rear", g_path([[X_C, YBELT], [X_C, arch_y]], False))
        v.add("belt-line", g_path([[-1.90, YBELT], [0.52, YBELT]], False))
        v.add("tail-lamp", g_rect(-2.03, 0.918, 0.045, 0.064))
    else:
        v.add("tail-lamp", g_rect(-2.03, 0.915, 0.05, 0.07))
    t = 0.0075 if fine else 0.015
    for end, sg in (("front", 1), ("rear", -1)):
        w = 0.075 + 2 * t
        x = XBUMP - w if sg > 0 else -XBUMP
        for nm, y in zip(("lower", "upper"), BUMP_Y):
            v.add(f"bumper-{end}-{nm}", g_rect(x, y - 0.0225, w, 0.045, 0.006), **NICKEL)
    r = max(0.014, t)
    v.add("bracket-front" if fine else "bracket-front-l", g_cap(2.07, 0.505, XBUMP - t - 0.004, 0.52, r))
    v.add("bracket-rear" if fine else "bracket-rear-l", g_cap(-1.95, 0.56, -XBUMP + t + 0.004, 0.52, r))
    if fine:
        v.add("exhaust-downpipe", g_cap(0.98, 0.78, EXH[2][0], 0.36, 0.027))
        v.add("exhaust-pipe", g_cap(EXH[2][0], 0.36, EXH[3][0], 0.36, 0.025))
        v.add("silencer", g_path([[-0.13, 0.29], [-0.13, 0.43], [-0.86, 0.43], [-0.86, 0.29]]))
    else:
        v.add("exhaust-pipe", g_cap(0.95, 0.40, EXH[3][0], 0.36, 0.025))
        v.add("silencer", g_cap(-0.20, 0.36, -0.80, 0.36, 0.07))
    tl = [EXH_TAIL[0][:2], EXH_TAIL[2][:2], EXH_TAIL[3][:2], [-2.122, 0.4155]]
    v.add("tail-pipe", g_path(band(tl, [0.025] * 4)))
    v.add("spare-tyre", g_ell(SPARE[0], SPARE[1], RW))
    v.add("spare-cover", g_ell(SPARE[0], SPARE[1], 0.256))
    if fine:
        wheels2d(v, old, 0.16)
    else:
        for s in old:
            if s["bone"] in (WF, WR):
                v.out.append(s)
    return v.out


def front_n(old, den):
    v = Sheet2D(old)
    fine = den == 100
    for t, bone in (("r", WR), ("f", WF)):
        for sd, sg in (("l", 1), ("r", -1)):
            x = ZW - TW / 2 if sg > 0 else -ZW - TW / 2
            v.add(f"tyre-{t}{sd}", g_rect(x, 0, TW, 2 * RW, 0.05))
            if t == "f" and fine:
                v.add(f"drum-{t}{sd}", g_rect(0.60 if sg > 0 else -0.635, 0.21, 0.035, 0.32), bone=bone, role="detail", z=3.95, tone=0.3, region="brakes", material=CI)
                v.add(f"hub-{t}{sd}", g_rect(0.765 if sg > 0 else -0.775, 0.32, 0.01, 0.10, 0.004))
    if not fine:
        for sd, sg in (("l", 1), ("r", -1)):
            v.add(f"hub-{sd}", g_rect(0.765 if sg > 0 else -0.775, 0.32, 0.01, 0.10))
    v.add("frame", g_rect(-0.4425, 0.42, 0.885, 0.10))
    if fine:
        zs, ys = [-0.59, -0.40, 0.40, 0.59], [0.0, -0.06, -0.06, 0.0]
        v.add("axle-f", g_path([[z, RW + y + 0.03] for z, y in zip(zs, ys)] + [[z, RW + y - 0.03] for z, y in zip(zs, ys)][::-1]))
        v.add("silencer", g_ell(-0.30, 0.36, 0.07))
    else:
        v.add("axle-f", g_rect(-0.60, RW - 0.075, 1.20, 0.06))
        v.add("exhaust", g_ell(-0.30, 0.36, 0.07))
    v.add("axle-r", g_rect(-0.59 if fine else -0.63, RW - 0.035, 1.18 if fine else 1.26, 0.07), bone=AR, role="detail", z=0.8, tone=0.3, material=CI)
    v.add("differential", g_ell(0, RW, 0.13), bone=AR, role="detail", z=0.85, tone=0.3, material=CI)
    for sd, sg in (("l", 1), ("r", -1)):
        m = lambda x, w: x if sg > 0 else -x - w
        if fine:
            v.add(f"spring-front-{sd}", g_rect(m(0.3175, 0.045), 0.33, 0.045, 0.14), **dict(NEW, tone=0.25))
        v.add(f"running-bracket-{sd}", g_rect(m(0.42, 0.40), 0.39, 0.40, 0.03), **NEW)
        v.add(f"running-board-{sd}", g_rect(m(0.62, 0.21), 0.42, 0.21, 0.035))
        v.add(f"sill-apron-{sd}", g_rect(m(0.592, 0.03), 0.44, 0.03, 0.085), bone=CH, role="body", z=1.6, tone=0.5, region="body-panels", material=P)
        v.add(f"fender-{sd}", {"kind": "poly", "closed": True, "points": [[n3(sg * z), y] for z, y in ((0.42, 0.47), (0.42, 0.434), (0.56, 0.434), (0.57, 0.40), (0.63, 0.322), (0.78, 0.322), (0.84, 0.40), (0.85, 0.48), (0.85, 0.835), (0.825, 0.86), (0.585, 0.86), (0.56, 0.835), (0.56, 0.52), (0.47, 0.52))]})
        lamp = dict(bone=CH, role="body", z=3.2, tone=0.74, region="fenders", material=NI)
        if fine:
            v.add(f"headlamp-{sd}", g_ell(sg * LAMP[2], LAMP[1], 0.118), **lamp)
            v.add(f"lamp-stalk-{sd}", g_rect(m(0.414, 0.032), 0.86, 0.032, 0.035), bone=CH, role="detail", z=3.1, tone=0.5, region="fenders", material=NI)
        else:
            v.add(f"headlamp-bowl-{sd}", g_ell(sg * LAMP[2], LAMP[1], 0.118), **lamp)
            v.add(f"lamp-stalk-{sd}", g_cap(sg * 0.36, 0.53, sg * 0.43, 0.88, 0.016), bone=CH, role="detail", z=3.1, tone=0.5, region="fenders", material=NI)
    k = (YHEAD - YBELT) / 0.46 * 0.04
    if fine:
        v.add("tub", g_rect(-ZB, YSILL, 2 * ZB, YBELT - YSILL))
        v.add("cabin", g_path([[-ZB, YBELT], [ZB, YBELT], [ZB - k, YHEAD], [-ZB + k, YHEAD]]))
        v.add("roof-slab", g_path(superellipse(0, 1.6525, 0.595, 0.0475, ROOF_E, 3)))
        v.add("hood", g_path(hood_front(HOOD_ZS[100])))
        v.add("radiator-shell", g_path(flat(SHELLS[100])))
        v.add("radiator-core", g_path(flat(CORE100)), bone=CH, role="detail", z=2.65, tone=0.15, region="radiator", material=EM)
        v.add("radiator-cap", g_path([[-0.03, 1.19], [0.03, 1.19], [0.045, 1.225], [0.03, 1.25], [-0.03, 1.25], [-0.045, 1.225]]), bone=CH, role="detail", z=2.8, tone=0.78, region="radiator", material=NI)
        v.add("lamp-bar", g_rect(-0.62, 0.847, 1.24, 0.03), bone=CH, role="detail", z=3.15, tone=0.74, region="fenders", material=NI)
    else:
        half = [(ZB, YSILL), (ZB, YBELT), (ZB - k, YHEAD), (0.585, 1.63), (0.593, 1.6525), (0.57, 1.675), (0.50, 1.6875), (0.30, 1.698), (0.0, 1.70)]
        v.add("body", {"kind": "poly", "closed": True, "points": [[z, y] for z, y in half] + [[-z, y] for z, y in half[::-1][1:]]})
        hh = [(HOOD_ZR, YSILL), (HOOD_ZR, 1.10), (0.48, 1.16), (0.42, 1.185), (0.20, 1.185), (0.12, 1.195), (0.035, 1.20), (0.035, 1.24), (0.0, 1.255)]
        v.add("hood-radiator", g_path([[z, y] for z, y in hh] + [[-z, y] for z, y in hh[::-1][1:]]))
    v.add("windscreen", g_path([[-0.54, 1.215], [0.54, 1.215], [0.51, 1.578], [-0.51, 1.578]]))
    v.add("spare-tyre", g_rect(SPARE[2] - TW / 2, SPARE[1] - RW, TW, 2 * RW, 0.05))
    for nm, y in zip(("lower", "upper"), BUMP_Y):
        v.add(f"bumper-front-{nm}", g_rect(-0.74, y - 0.0225, 1.48, 0.045, 0.015), **NICKEL)
    return v.out


def plan_n(old, den):
    v = Sheet2D(old)
    fine = den == 100
    for sd, sg in (("l", 1), ("r", -1)):
        v.add(f"frame-{sd}", g_path(rails_plan(RAILS[den], sg)), **dict(NEW, role="body"))
    for nm, x, hw in (("front", 1.97, 0.34), ("mid", -0.45, 0.42), ("rear", -1.93, 0.42)):
        if fine or nm != "mid":
            v.add(f"crossmember-{nm}", g_rect(x - 0.025, -hw, 0.05, 2 * hw), **NEW)
    w = 0.022 if fine else 0.03
    v.add("axle-f-l", g_rect(XF - w, 0.34, 2 * w, 0.25 if fine else 0.29))
    v.add("axle-f-r", g_rect(XF - w, -0.59 if fine else -0.63, 2 * w, 0.25 if fine else 0.29))
    v.add("axle-f-mid", g_rect(XF - w, -0.34, 2 * w, 0.68))
    v.add("axle-r-l", g_rect(XR - 0.035, 0.14, 0.07, 0.45 if fine else 0.49))
    v.add("axle-r-r", g_rect(XR - 0.035, -0.59 if fine else -0.63, 0.07, 0.45 if fine else 0.49))
    v.add("axle-r-mid", g_rect(XR - 0.13, -0.14, 0.26, 0.28, 0.08))
    for sd, sg in (("l", 1), ("r", -1)):
        m = lambda z, w_: z if sg > 0 else -z - w_
        if fine:
            for k_, x in enumerate((-0.55, 0.55)):
                v.add(f"running-bracket-{k_ + 1}-{sd}", g_rect(x - 0.02, m(0.42, 0.40), 0.04, 0.40))
        else:
            v.add(f"running-bracket-{sd}", g_rect(-0.03, m(0.42, 0.40), 0.06, 0.40))
        v.add(f"running-board-{sd}", g_rect(-1.02, m(0.62, 0.21), 2.05, 0.21))
        v.add(f"fender-f{sd}", g_path(wing_plan(WINGF[den][0], WINGF[den][1], WINGF[den][2], sg)))
        v.add(f"fender-r{sd}", g_path(wing_plan(WINGB[den][0], WINGB[den][1], WINGB[den][2], sg)))
        lamp = dict(bone=CH, role="body", z=3.2, tone=0.74, region="fenders", material=NI)
        v.add(f"headlamp-{sd}" if fine else f"headlamp-bowl-{sd}", g_path(bowl_n(den, sg * LAMP[2], True)), **lamp)
        t = 0.0075 if fine else 0.015
        v.add(f"bracket-front-{sd}", g_cap(2.07, sg * 0.34, XBUMP - t - 0.004, sg * 0.34, max(0.014, t)))
        v.add(f"bracket-rear-{sd}", g_cap(-1.95, sg * 0.42, -XBUMP + t + 0.004, sg * 0.42, max(0.014, t)))
    v.add("spare-mount", g_cap(0.80, -0.41, SPARE[0], SPARE[2] + 0.02, 0.02))
    v.add("tail-pipe", g_cap(-1.72, -0.35, -2.10, -0.35, 0.025))
    v.add("tub", {"kind": "poly", "closed": True, "points": [[-1.99, -ZB], [0.52, -ZB], [0.52, ZB], [-1.99, ZB]]})
    v.add("cowl", g_path([[x, hw] for x, top, hw, e in COWLS[den]] + [[x, -hw] for x, top, hw, e in COWLS[den]][::-1]), **BODY)
    v.add("hood", g_path([[0.72, -HOOD_ZR], [1.69, -HOOD_ZF], [1.69, HOOD_ZF], [0.72, HOOD_ZR]]))
    v.add("roof-slab", g_path(roof_plan(ROOFS[den])))
    v.add("radiator-shell", g_rect(1.68, -0.297, 0.12, 0.594), bone=CH, role="body", z=2.6, tone=0.78, region="radiator", material=NI)
    v.add("spare-tyre", g_rect(SPARE[0] - RW, SPARE[2] - TW / 2, 2 * RW, TW, 0.05))
    if fine:
        v.add("lamp-bar", g_rect(1.815, -0.62, 0.03, 1.24), bone=CH, role="detail", z=3.15, tone=0.74, region="fenders", material=NI)
        v.add("tail-lamp", g_rect(-2.03, 0.388, 0.045, 0.064))
    else:
        v.add("tail-lamp", g_rect(-2.03, 0.385, 0.05, 0.07))
    t = 0.0075 if fine else 0.015
    v.add("bumper-front", g_path(bump_plan_n(1, t)))
    v.add("bumper-rear", g_path(bump_plan_n(-1, t)))
    return v.out


elevation100 = lambda old: elevation_n(old, 100)
front100 = lambda old: front_n(old, 100)
plan100 = lambda old: plan_n(old, 100)
elevation200 = lambda old: elevation_n(old, 200)
front200 = lambda old: front_n(old, 200)
plan200 = lambda old: plan_n(old, 200)


# ---------------------------------------------------------------- 1:500
HULL = dict(bone=CH, role="body", z=2, tone=0.55, regions=["body-panels", "hood", "radiator", "engine-bay"], material=P)
WING5 = dict(bone=CH, role="body", z=3, tone=0.5, region="fenders", material=P)
ROOF5 = dict(bone=CH, role="body", z=2.4, tone=0.78, region="roof", material=FB)


def elevation500(old):
    v = Sheet2D(old)
    v.add("underbody", g_path([[-2.15, 0.40], [2.15, 0.40], [2.15, 0.52], [-2.15, 0.52]]))
    hull = [[1.80, YSILL], [1.80, 1.20], [1.68, 1.20], [1.68, 1.185], [0.74, 1.185], [0.60, 1.174], [0.52, YBELT], CABIN[1], CABIN[2], [-1.90, YBELT]] + TAILS[500][::-1] + arc(XR, RW, ARCH_R, _a0, 180 - _a0, ARCHN[500])[::-1]
    v.add("wheelhouse", g_path(arc(XR, RW, ARCH_R, _a0, 180 - _a0, ARCHN[500])), bone=CH, role="body", z=1.5, tone=0.3, region="body-panels", material=P)
    v.add("silhouette", g_path(hull), **HULL)
    v.add("roof-slab", g_path(roof_side(ROOFS[500])), **ROOF5)
    v.add("wing-line", g_path(band(WINGLINE[0], WINGLINE[3])), **WING5)
    v.add("windows", g_path([[0.444, 1.225], [0.415, 1.565], [-1.692, 1.565], [-1.766, 1.225]]))
    for end, sg in (("front", 1), ("rear", -1)):
        v.add(f"bumper-{end}", g_rect(XBUMP - 0.075 if sg > 0 else -XBUMP, 0.4525, 0.075, 0.135), **NICKEL)
    for s in old:
        if s["bone"] in (WF, WR):
            v.out.append(s)
    return v.out


def front500(old):
    v = Sheet2D(old)
    for t in ("r", "f"):
        for sd, sg in (("l", 1), ("r", -1)):
            v.add(f"tyre-{t}{sd}", g_rect(ZW - TW / 2 if sg > 0 else -ZW - TW / 2, 0, TW, 2 * RW, 0.04))
    v.add("underbody", g_rect(-0.35, 0.37, 0.70, 0.15))
    v.add("axles", g_rect(-0.635, RW - 0.0375, 1.27, 0.075), bone=AF, role="body", z=0.5, tone=0.3, material=CI)
    k = (YHEAD - YBELT) / 0.46 * 0.04
    half = [(0.56, YSILL), (0.56, 0.3325), (0.85, 0.3325), (0.85, 0.87), (0.83, 0.8975), (ZB, 0.8975), (ZB, YBELT), (ZB - k, YHEAD), (0.595, 1.66), (0.58, 1.70)]
    v.add("silhouette", {"kind": "poly", "closed": True, "points": [[z, y] for z, y in half] + [[-z, y] for z, y in half[::-1]]}, regions=["body-panels", "hood", "fenders", "radiator", "engine-bay", "roof"])
    v.add("windows", g_path([[-0.54, 1.215], [0.54, 1.215], [0.51, 1.578], [-0.51, 1.578]]))
    v.add("bumper-front", g_rect(-0.74, 0.4525, 1.48, 0.135), **NICKEL)
    return v.out


def plan500(old):
    v = Sheet2D(old)
    v.add("axle-f", g_rect(XF - 0.0375, -0.635, 0.075, 1.27), bone=AF, role="body", z=0.5, tone=0.3, material=CI)
    v.add("underbody", g_rect(-2.15, -0.35, 4.30, 0.70))
    half = [(1.80, 0.0), (1.80, 0.297), (1.68, 0.285), (0.74, 0.505), (0.60, 0.58), (1.02, 0.57), (1.20, 0.53), (1.32, 0.56), (2.19, 0.56), (2.2275, 0.60), (2.2275, 0.80), (2.19, 0.85), (-1.96, 0.85), (-2.0275, 0.80)]
    v.add("silhouette", {"kind": "poly", "closed": True, "points": [[x, z] for x, z in half] + [[x, -z] for x, z in half[::-1]]}, regions=["body-panels", "hood", "fenders", "radiator", "engine-bay"])
    v.add("roof-slab", g_path(roof_plan(ROOFS[500])), **dict(ROOF5, role="body"))
    for end, sg in (("front", 1), ("rear", -1)):
        v.add(f"bumper-{end}", g_rect(XBUMP - 0.075 if sg > 0 else -XBUMP, -0.74, 0.075, 1.48))
    return v.out
