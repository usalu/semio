"""🏗️ Hand-stated geometry of `machinery/tower-crane` (round 3): every part of the crane as explicit numbers in its bone's rest frame, written once and turned into the 3D solids of each scale and into the 2D shapes of the three viewpoints, so the drawing and the model are one body (ruling E1).

Frames (rest pose): `base` world; `slewing` origin on the mast top (world y 40), axes world; `jib` at slewing (0, 4.4), axes world; `counter-jib` at slewing (0, 4.4) turned 180 degrees (local x towards the tail, local y down); `counterweight` at slewing (-14.9, 1.5); `trolley` at jib (3, -0.8); `hoist-block` below the trolley; `skip` at block y -1.5; `rope-n` hang from the trolley along -y.

@see ./round3-brief.md
@see ./r3-machinery-tower-crane.md
"""
import math

DEN = (50, 100, 200, 500)
FEAT = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
STEEL, CONC, MACH, GLASS, ROPE, SKIP = "steel-s355", "concrete", "machinery", "glass", "wire-rope", "skip-steel"

JIB_TOP_Y, JIB_BOT_Y, JIB_BOT_Z = 0.74, -0.74, 0.6
JIB_HEEL_TOP, JIB_HEEL_BOT, JIB_TIP = 0.56, 0.8, 55.0
JIB_PANELS = 15
JIB_P = (JIB_TIP - JIB_HEEL_BOT) / JIB_PANELS
CJ_Y, CJ_Z, CJ_HEEL, CJ_TAIL, CJ_PANELS = 0.64, 0.64, 0.785, 16.5, 5
CJ_P = (CJ_TAIL - CJ_HEEL) / CJ_PANELS
APEX = (0.0, 8.6, 0.0)
PENDANT_NODES = (6, 11)
CAB_X0, CAB_X1, CAB_Y0, CAB_Y1, CAB_Z0, CAB_Z1 = 1.45, 3.0, 1.2, 3.3, -2.25, -0.85
CAB_TOPX, CAB_SLOPE_Y1 = 3.3, 3.15
WALL = 0.033
STRAND_X, ROPE_R = 0.2, 0.007
PENDANT_R = 0.008


def r(v, n=4):
    return round(v + 0.0, n)


def P(*v):
    return [r(x) for x in v]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def norm(a):
    return math.sqrt(sum(x * x for x in a))


def unit(a):
    n = norm(a)
    return [x / n for x in a]


def front_x(y):
    return CAB_X1 + (CAB_TOPX - CAB_X1) * (y - CAB_Y0) / (CAB_SLOPE_Y1 - CAB_Y0)


CAB_PROFILE = [[CAB_X0, CAB_Y0], [CAB_X1, CAB_Y0], [CAB_TOPX, CAB_SLOPE_Y1], [CAB_TOPX, CAB_Y1], [CAB_X0, CAB_Y1]]
CAB_WINDOW = [[2.05, 1.95], [r(front_x(1.95) - 0.075), 1.95], [r(front_x(3.08) - 0.075), 3.08], [2.05, 3.08]]


def solid(sid, bone, kind, mat, role, tone, **kw):
    s = {"id": sid, "kind": kind, "bone": bone, "material": mat, "role": role, "tone": tone}
    s.update(kw)
    return s


def box(sid, bone, c, size, mat=STEEL, role="body", tone=0.5, **kw):
    return solid(sid, bone, "box", mat, role, tone, size=P(*size), transform={"translation": P(*c)}, **kw)


def cyl(sid, bone, a, b, rad, mat=STEEL, role="body", tone=0.5, slender=None, **kw):
    d = sub(b, a)
    L = norm(d)
    s = solid(sid, bone, "revolve", mat, role, tone, profile=[[0.0, r(rad, 5)], [r(L), r(rad, 5)]], axis=P(*unit(d)), origin=P(*a), **kw)
    if slender if slender is not None else L / (2 * rad) >= 20.5:
        s["slender"] = True
    return s


def rev(sid, bone, origin, profile, axis=(0.0, 1.0, 0.0), mat=STEEL, role="body", tone=0.5, **kw):
    return solid(sid, bone, "revolve", mat, role, tone, profile=[P(*q) for q in profile], axis=P(*axis), origin=P(*origin), **kw)


def ext(sid, bone, profile, plane, a, b, mat=STEEL, role="body", tone=0.5, **kw):
    return solid(sid, bone, "extrude", mat, role, tone, profile=[P(*p) for p in profile], plane=plane, **{"from": r(a), "to": r(b)}, **kw)


class Bar:
    """🦯 One straight member: end points in the bone frame, a round (d) or rectangular (w across in z or y, h) section, and how it is drawn."""

    def __init__(self, sid, bone, a, b, d=None, rect=None, views=("elevation", "front", "plan"), region="lattice", mat=STEEL):
        self.id, self.bone, self.a, self.b, self.d, self.rect, self.views, self.region, self.mat = sid, bone, [float(x) for x in a], [float(x) for x in b], d, rect, views, region, mat

    def solid(self):
        if self.rect is None:
            return cyl(self.id, self.bone, self.a, self.b, self.d / 2)
        d = sub(self.b, self.a)
        L = norm(d)
        c = [(self.a[i] + self.b[i]) / 2 for i in range(3)]
        if abs(d[1]) < 1e-9 and abs(d[2]) < 1e-9:
            s = box(self.id, self.bone, c, [L, self.rect[1], self.rect[0]])
        elif abs(d[0]) < 1e-9 and abs(d[1]) < 1e-9:
            s = box(self.id, self.bone, c, [self.rect[0], self.rect[1], L])
        else:
            raise ValueError(self.id)
        if L / max(self.rect) >= 20.5:
            s["slender"] = True
        return s

    def width(self):
        return self.d if self.rect is None else self.rect[1]

    def shape(self, den, z=1.8, tone=0.3, role="body"):
        sec = {"kind": "circle", "d": r(self.d, 4)} if self.rect is None else {"kind": "rect", "w": r(self.rect[0], 4), "h": r(self.rect[1], 4)}
        return {"kind": "member", "from": dict(zip("xyz", P(*self.a))), "to": dict(zip("xyz", P(*self.b))), "section": sec, "id": self.id, "bone": self.bone, "role": role, "z": z, "filled": False, "tone": tone, "region": self.region, "material": self.mat, "penMm": r(min(max(self.width() * 1000 / den, 0.13), 2.0), 3)}


def jib_bars(level):
    """🏗️ The triangular jib: one 140 mm top chord on the axis, two 120 x 140 mm rectangular bottom chords at +-0.6 m that carry the trolley, Warren diagonals in both inclined faces, horizontals and a Z bracing in the bottom plane, heel and tip frames. level 50 / 100 / 200 drops the bottom diagonals (100) and the bottom horizontals and heel details (200)."""
    B = "jib"
    tops = [JIB_HEEL_BOT + k * JIB_P for k in range(JIB_PANELS + 1)]
    mids = [t + JIB_P / 2 for t in tops[:-1]]
    yt, yb, zb = JIB_TOP_Y, JIB_BOT_Y, JIB_BOT_Z
    yi, zi = -0.70, 0.56
    dd = 0.08 if level < 200 else 0.08
    bars = [Bar("jib-chord", B, [JIB_HEEL_TOP, yt, 0], [JIB_TIP, yt, 0], d=0.14)]
    for s, z in (("l", zb), ("r", -zb)):
        bars.append(Bar(f"jib-rail-{s}", B, [JIB_HEEL_BOT, yb, z], [JIB_TIP, yb, z], rect=(0.12, 0.14), views=("elevation", "front", "plan") if s == "r" else ("front", "plan")))
    for k, t in enumerate(tops[:-1]):
        m = mids[k]
        for s, sg in (("l", 1), ("r", -1)):
            v = ("elevation", "plan") if s == "r" else ("plan",)
            bars.append(Bar(f"jib-diag-down-{s}-{k + 1}", B, [t, yt, 0], [m, yi, sg * zi], d=dd, views=v))
            bars.append(Bar(f"jib-diag-up-{s}-{k + 1}", B, [m, yi, sg * zi], [tops[k + 1], yt, 0], d=dd, views=v))
    if level <= 100:
        for k, m in enumerate(mids):
            bars.append(Bar(f"jib-bottom-{k + 1}", B, [m, yb, -0.54], [m, yb, 0.54], d=0.05, views=("plan",)))
    if level == 50:
        for k in range(len(mids) - 1):
            sg = 1 if k % 2 == 0 else -1
            bars.append(Bar(f"jib-bottom-diag-{k + 1}", B, [mids[k], yb, -0.54 * sg], [mids[k + 1], yb, 0.54 * sg], d=0.05, views=("plan",)))
    for tag, x, xt in (("heel", JIB_HEEL_BOT, JIB_HEEL_BOT), ("tip", JIB_TIP - 0.03, JIB_TIP - 0.03)):
        for s, sg in (("l", 1), ("r", -1)):
            bars.append(Bar(f"jib-{tag}-post-{s}", B, [xt, yt, 0], [x, yi, sg * zi], d=0.07, views=("front",) if s == "l" else ("elevation", "front")))
        bars.append(Bar(f"jib-{tag}-cross", B, [x, yb, -0.54], [x, yb, 0.54], d=0.06, views=("front", "plan")))
    if level <= 100:
        bars.append(Bar("jib-heel-pin", B, [JIB_HEEL_TOP, yt, -0.44], [JIB_HEEL_TOP, yt, 0.44], d=0.08, views=("front", "plan")))
    return bars


def counter_bars(level):
    """🧱 The counter-jib: a flat box lattice of four 120 mm chords 1.28 m apart and 1.28 m deep, Warren diagonals in both side faces, horizontals and Z bracing in the top plane (the walkway level), horizontals in the bottom plane, heel posts at the rear tower-head legs and a tail frame that takes the counter-jib pendant. level 100 drops the top Z bracing, 200 also the bottom horizontals."""
    B = "counter-jib"
    nodes = [CJ_HEEL + k * CJ_P for k in range(CJ_PANELS + 1)]
    mids = [n + CJ_P / 2 for n in nodes[:-1]]
    yt, yb, zc = -CJ_Y, CJ_Y, CJ_Z
    bars = []
    for s, z in (("l", zc), ("r", -zc)):
        bars.append(Bar(f"counter-chord-top-{s}", B, [CJ_HEEL, yt, z], [CJ_TAIL, yt, z], d=0.12, views=("elevation", "front", "plan") if s == "r" else ("front", "plan")))
        bars.append(Bar(f"counter-chord-bottom-{s}", B, [CJ_HEEL, yb, z], [CJ_TAIL, yb, z], d=0.12, views=("elevation", "front") if s == "r" else ("front",)))
    for k, n in enumerate(nodes[:-1]):
        for s, z in (("l", zc), ("r", -zc)):
            v = ("elevation",) if s == "r" else ()
            bars.append(Bar(f"counter-diag-down-{s}-{k + 1}", B, [n, yt, z], [mids[k], yb, z], d=0.07, views=v))
            bars.append(Bar(f"counter-diag-up-{s}-{k + 1}", B, [mids[k], yb, z], [nodes[k + 1], yt, z], d=0.07, views=v))
    for k, n in enumerate(nodes[1:-1]):
        bars.append(Bar(f"counter-top-{k + 1}", B, [n, yt, -zc], [n, yt, zc], d=0.06, views=("plan",)))
    if level == 50:
        for k in range(CJ_PANELS):
            sg = 1 if k % 2 == 0 else -1
            bars.append(Bar(f"counter-brace-{k + 1}", B, [nodes[k], yt, -zc * sg], [nodes[k + 1], yt, zc * sg], d=0.06, views=("plan",)))
    if level <= 100:
        for k, m in enumerate(mids):
            bars.append(Bar(f"counter-bottom-{k + 1}", B, [m, yb, -zc], [m, yb, zc], d=0.06, views=("plan",)))
    for tag, x in (("heel", CJ_HEEL + 0.03), ("tail", CJ_TAIL - 0.03)):
        for s, z in (("l", zc), ("r", -zc)):
            bars.append(Bar(f"counter-{tag}-post-{s}", B, [x, yt, z], [x, yb, z], d=0.08, views=("elevation", "front") if s == "r" else ("front",)))
    bars.append(Bar("counter-tail-top", B, [CJ_TAIL - 0.03, yt, -zc], [CJ_TAIL - 0.03, yt, zc], d=0.1, views=("front", "plan")))
    bars.append(Bar("counter-tail-bottom", B, [CJ_TAIL - 0.03, yb, -zc], [CJ_TAIL - 0.03, yb, zc], d=0.08, views=("front", "plan")))
    return bars


def pendant_points():
    tops = [JIB_HEEL_BOT + k * JIB_P for k in range(JIB_PANELS + 1)]
    jy = 4.4 + JIB_TOP_Y + 0.07
    return [("pendant-fore-1", (tops[PENDANT_NODES[0]], jy, 0.0)), ("pendant-fore-2", (tops[PENDANT_NODES[1]], jy, 0.0)), ("pendant-back", (-(CJ_TAIL - 0.03), 4.4 + CJ_Y + 0.05, 0.0))]


def pendant_bars():
    return [Bar(pid, "slewing", APEX, end, d=2 * PENDANT_R, region="pendants", mat=ROPE) for pid, end in pendant_points()]


MAST_L, MAST_C = 39.0, 0.92
MAST_LEVELS = [2.0 * k for k in range(20)] + [MAST_L]


def mast_bars():
    """🗼 The lattice mast: four 160 mm square chords 1.84 m apart (2.0 m over all), 90 mm horizontals every 2 m on all four faces and at the top, 80 mm Warren diagonals in every 2 m panel (the last panel 1 m), the same pattern on all faces; bone frame u up, v towards world -x, w lateral."""
    B = "mast"
    c = MAST_C
    bars = []
    for sy, sz, tag in ((-1, -1, "pxnz"), (-1, 1, "pxpz"), (1, -1, "nxnz"), (1, 1, "nxpz")):
        views = tuple(v for v, ok in (("elevation", sz < 0), ("front", sy < 0)) if ok)
        bars.append(Bar(f"mast-chord-{tag}", B, [0, sy * c, sz * c], [MAST_L, sy * c, sz * c], rect=(0.16, 0.16), views=views))
    faces = (("px", lambda t: [None, -c, t], ("front",)), ("nx", lambda t: [None, c, t], ()), ("nz", lambda t: [None, t, -c], ("elevation",)), ("pz", lambda t: [None, t, c], ()))
    for name, at, views in faces:
        for i, u in enumerate(MAST_LEVELS):
            a, b = at(-c), at(c)
            a[0] = b[0] = u
            bars.append(Bar(f"mast-tie-{name}-{i + 1}", B, a, b, d=0.09, views=views))
        for k in range(len(MAST_LEVELS) - 1):
            u0, u1 = MAST_LEVELS[k], MAST_LEVELS[k + 1]
            s = -c if k % 2 == 0 else c
            a, b = at(s), at(-s)
            a[0], b[0] = u0, u1
            bars.append(Bar(f"mast-diag-{name}-{k + 1}", B, a, b, d=0.08, views=views))
    return bars
