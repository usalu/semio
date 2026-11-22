"""🪈 Authoritative generator of the staffage asset `used-steel-pipe-rack`.

Used black medium-series threaded steel pipes EN 10255 (DIN 2440 successor) DN 25 to DN 50 in 6 m lengths in a painted
single-sided cantilever pipe rack of three IPE 160 columns with IPE 100 arms: a strapped bundle of seven DN 50 lengths on
the lower arms (the lifted component), a loose row of DN 40, DN 32 and DN 25 lengths with fittings remnants on the upper
arms. Every hand-chosen number is stated here; running the script rewrites `🔣️.json` next to it.

@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""

import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
SLUG = "used-steel-pipe-rack"
G = 9.81
RHO = 7850.0
RHO_MI = 7300.0

PIPE_L = 6.0
PIPE_Z0, PIPE_Z1 = -PIPE_L / 2, PIPE_L / 2
EN10255 = {"DN25": (0.0337, 0.0032), "DN32": (0.0424, 0.0032), "DN40": (0.0483, 0.0032), "DN50": (0.0603, 0.0036)}
IPE160 = {"h": 0.160, "b": 0.082, "tw": 0.005, "tf": 0.0074, "r": 0.009, "A": 20.1e-4, "I": 869e-8, "kgm": 15.8}
IPE100 = {"h": 0.100, "b": 0.055, "tw": 0.0041, "tf": 0.0057, "r": 0.007, "A": 10.3e-4, "I": 171e-8, "kgm": 8.1}
L50 = {"a": 0.050, "t": 0.005, "A": 4.80e-4, "kgm": 3.77}

COL_Z = (-2.1, 0.0, 2.1)
COL_X0, COL_X1 = 0.0, IPE160["h"]
FOOT_X0, FOOT_X1 = -0.12, 1.10
FOOT_H = IPE160["h"]
COL_TOP = 1.60
BRK_X1 = COL_X1 + 0.010
ARM_X1 = BRK_X1 + 0.80
ARM_TOP = {"lower": 0.55, "upper": 1.25}
STOP_T, STOP_H = 0.010, 0.060
BRK_W, BRK_DOWN, BRK_UP = 0.120, 0.060, 0.020
TIE_Y = (0.30, 1.45)
TIE_Z = COL_Z[-1] + IPE160["b"] / 2

BUNDLE_X0 = 0.20
STOCK_X0 = 0.20
STRAP_Z = (-1.55, 1.55)
STRAP_W, STRAP_T = 0.019, 0.0006
TINE_Z = (-1.0, 1.0)
HANGER_Z = (-2.0, 2.0)
LIFT_DY, LIFT_DX = 0.08, 1.30
REIN_DY, REIN_DX = 2.25, 1.60

SOCKET_50 = (0.074, 0.050, 0.45)
SOCKET_40 = (0.062, 0.044, 0.30)
ELBOW_25 = (0.043, 0.038, 0.20)
NIPPLE_25 = 0.032


def t(en, de):
    """📝 One bilingual text, English first."""
    return {"en": en, "de": de}


def dez(x):
    """🇩🇪 A number with a decimal comma."""
    return str(x).replace(".", ",")


def r6(x):
    """🎯 Round to micrometres."""
    return round(x + 0.0, 6)


def pipe_kgm(dn):
    """⚖️ Mass per metre of a plain-end EN 10255 medium tube from its section and the steel density."""
    d, s = EN10255[dn]
    return math.pi * (d - s) * s * RHO


def pipe_inner_area(dn):
    """💧 Clear bore area of a tube."""
    d, s = EN10255[dn]
    return math.pi * (d - 2 * s) ** 2 / 4


def pipe_i(dn):
    """📐 Second moment of area of a tube."""
    d, s = EN10255[dn]
    return math.pi / 64 * (d ** 4 - (d - 2 * s) ** 4)


R50 = EN10255["DN50"][0] / 2
BUNDLE = []
for k in range(4):
    BUNDLE.append(("b" + str(k + 1), BUNDLE_X0 + R50 + 2 * R50 * k, R50))
for k in range(3):
    BUNDLE.append(("t" + str(k + 1), BUNDLE_X0 + 2 * R50 + 2 * R50 * k, R50 + 2 * R50 * math.sin(math.pi / 3)))
BUNDLE_W = 8 * R50
BUNDLE_H = BUNDLE[-1][2] + R50

STOCK = []
_x = STOCK_X0
for dn, n, gap in (("DN40", 3, 0.0), ("DN32", 3, 0.0), ("DN25", 4, 0.0)):
    rr = EN10255[dn][0] / 2
    for k in range(n):
        STOCK.append((f"{dn.lower()}-{k + 1}", dn, _x + rr, rr))
        _x += 2 * rr
    _x += gap
STOCK_X1 = _x
SOCKET_ON_BUNDLE = "b2"
SOCKET_ON_STOCK = "dn40-1"
ELBOW_ON_STOCK = "dn25-3"

ORIGIN = {"yard": (0.0, 0.0), "rack": (0.0, 0.0), "stock": (STOCK_X0, ARM_TOP["upper"]), "carrier-out": (0.0, 0.0), "carrier-up": (0.0, 0.0), "bundle": (BUNDLE_X0, ARM_TOP["lower"])}
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def arc(cp, cq, rad, a0, a1, n):
    """🌙 Points of a quarter fillet in (p, q), from angle a0 to a1 in degrees, end points included."""
    return [(cp + rad * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cq + rad * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def i_section(sec, n):
    """🏗️ The rolled I-section in (p along the web, q across the flanges) from p = 0 to h, with its root fillets in n segments."""
    h, b, tw, tf, r = sec["h"], sec["b"], sec["tw"], sec["tf"], sec["r"]
    pts = [(0, -b / 2), (0, b / 2), (tf, b / 2)]
    pts += arc(tf + r, tw / 2 + r, r, 180, 270, n)
    pts += arc(h - tf - r, tw / 2 + r, r, 270, 360, n)
    pts += [(h - tf, b / 2), (h, b / 2), (h, -b / 2), (h - tf, -b / 2)]
    pts += arc(h - tf - r, -tw / 2 - r, r, 0, 90, n)
    pts += arc(tf + r, -tw / 2 - r, r, 90, 180, n)
    pts += [(tf, -b / 2)]
    return pts


def hull(points):
    """🫧 Convex hull (monotone chain), counter-clockwise."""
    pts = sorted(set((round(x, 7), round(y, 7)) for x, y in points))

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def circles_hull(circles, grow, n=24):
    """⭕ Convex hull of circles (cx, cy, r), each grown by `grow`, sampled on circumscribed polygons."""
    pts = []
    for cx, cy, rr in circles:
        rad = (rr + grow) / math.cos(math.pi / n)
        pts += [(cx + rad * math.cos(2 * math.pi * (k + 0.5) / n), cy + rad * math.sin(2 * math.pi * (k + 0.5) / n)) for k in range(n)]
    return hull(pts)


def bundle_circles():
    """🪈 The seven DN 50 sections of the bundle in world rest coordinates."""
    return [(x, ARM_TOP["lower"] + y, R50) for _, x, y in BUNDLE]


def stock_circles():
    """🪈 The ten sections of the loose upper row in world rest coordinates."""
    return [(x, ARM_TOP["upper"] + rr, rr) for _, _, x, rr in STOCK]


class Part:
    """🧱 One physical part of one scale, authored in world rest coordinates: its 3D solid in the bone frame and its 2D shapes in the three views from the same numbers."""

    def __init__(self, pid, bone, kind, geo, material, tone, region, views=("elevation", "front", "plan"), role="body", extra=None, shape2d=None, lists=("body", "thermal", "structure")):
        self.id, self.bone, self.kind, self.geo, self.material, self.tone, self.region = pid, bone, kind, geo, material, tone, region
        self.views, self.role, self.extra, self.shape2d, self.lists = views, role, extra or {}, shape2d or {}, lists

    def solid(self):
        """🧊 The volume solid of the part in the rest frame of its bone."""
        ox, oy = ORIGIN[self.bone]
        g = self.geo
        s = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
            s.update(kind="box", size=[r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], transform={"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2)]})
        elif self.kind == "cylz":
            cx, cy, z0, z1, rr = g
            s.update(kind="revolve", profile=[[0, r6(rr)], [r6(z1 - z0), r6(rr)]], axis=[0, 0, 1], origin=[r6(cx - ox), r6(cy - oy), r6(z0)])
        elif self.kind == "cyly":
            cx, cz, y0, y1, rr = g
            s.update(kind="revolve", profile=[[0, r6(rr)], [r6(y1 - y0), r6(rr)]], axis=[0, 1, 0], origin=[r6(cx - ox), r6(y0 - oy), r6(cz)])
        elif self.kind == "ext":
            plane, prof, holes, a0, a1 = g
            s.update(kind="extrude", plane=plane, profile=[list(self._local(plane, p)) for p in prof])
            if holes:
                s["holes"] = [[list(self._local(plane, p)) for p in h] for h in holes]
            off = {"xy": 0.0, "yz": ox, "zx": oy}[plane]
            s["from"], s["to"] = r6(a0 - off), r6(a1 - off)
        for key in ("slender", "sheet", "texture", "envelope", "cornerRadius"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s

    def _local(self, plane, p):
        """🧭 A world profile point of a plane in the bone frame."""
        ox, oy = ORIGIN[self.bone]
        a, b = p
        if plane == "xy":
            return r6(a - ox), r6(b - oy)
        if plane == "yz":
            return r6(a - oy), r6(b)
        return r6(a), r6(b - ox)

    def bounds(self):
        """📦 World bounds (x0, y0, z0), (x1, y1, z1) of the part at rest."""
        g = self.geo
        if self.kind == "box":
            return g
        if self.kind == "cylz":
            cx, cy, z0, z1, rr = g
            return (cx - rr, cy - rr, z0), (cx + rr, cy + rr, z1)
        if self.kind == "cyly":
            cx, cz, y0, y1, rr = g
            return (cx - rr, y0, cz - rr), (cx + rr, y1, cz + rr)
        plane, prof, _, a0, a1 = g
        us = [p[0] for p in prof]
        vs = [p[1] for p in prof]
        if plane == "xy":
            return (min(us), min(vs), a0), (max(us), max(vs), a1)
        if plane == "yz":
            return (a0, min(us), min(vs)), (a1, max(us), max(vs))
        return (min(vs), a0, min(us)), (max(vs), a1, max(us))

    def shape(self, view):
        """✏️ The filled 2D shape of the part in one view: elevation in the bone frame, front (lateral, height) and plan (ahead, lateral) in world rest coordinates."""
        ox, oy = ORIGIN[self.bone]
        (x0, y0, z0), (x1, y1, z1) = self.bounds()
        depth = {"elevation": 10 - z0, "front": 10 + x1, "plan": 10 + y1}[view]
        s = {"id": self.id, "bone": self.bone, "role": self.role, "z": round(depth + self.shape2d.get("dz", 0), 4), "filled": True, "tone": self.tone, "material": self.material, "region": self.region}
        g = self.geo

        def rect(x, y, w, h):
            return dict(s, kind="rect", x=r6(x), y=r6(y), w=r6(w), h=r6(h))

        def polyg(pts, holes=None):
            if holes:
                d = " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ring) + " Z" for ring in [pts] + holes)
                return dict(s, kind="path", d=d)
            return dict(s, kind="poly", points=[[r6(a), r6(b)] for a, b in pts], closed=True)

        if view == "elevation":
            if self.kind == "cylz":
                cx, cy, _, _, rr = g
                return dict(s, kind="ellipse", cx=r6(cx - ox), cy=r6(cy - oy), rx=r6(rr), ry=r6(rr))
            if self.kind == "ext" and g[0] == "xy":
                return polyg([(a - ox, b - oy) for a, b in g[1]], [[(a - ox, b - oy) for a, b in h] for h in g[2]] if g[2] else None)
            return rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)
        if view == "front":
            if self.kind == "ext" and g[0] == "yz":
                return polyg([(b, a) for a, b in g[1]])
            return rect(z0, y0, z1 - z0, y1 - y0)
        if self.kind == "cyly":
            cx, cz, _, _, rr = g
            return dict(s, kind="ellipse", cx=r6(cx), cy=r6(cz), rx=r6(rr), ry=r6(rr))
        if self.kind == "ext" and g[0] == "zx":
            return polyg([(b, a) for a, b in g[1]])
        return rect(x0, z0, x1 - x0, z1 - z0)


def parts_of(scale):
    """🗂️ The hand-chosen part list of one scale: every part the real rack and its pipes have that this scale can show, generalised for its smallest feature."""
    P = []
    fine = scale == 50
    for k, zc in enumerate(COL_Z):
        b = IPE160["b"]
        if fine:
            prof = [(zc + q, COL_X0 + p) for p, q in i_section(IPE160, 3)]
            P.append(Part(f"column-{k}", "rack", "ext", ("zx", prof, None, FOOT_H, COL_TOP), "rack-steel", 0.55, "rack"))
            fprof = [(p, zc + q) for p, q in i_section(IPE160, 3)]
            P.append(Part(f"foot-{k}", "rack", "ext", ("yz", fprof, None, FOOT_X0, FOOT_X1), "rack-steel", 0.5, "rack"))
            P.append(Part(f"column-cap-{k}", "rack", "box", ((COL_X0 - 0.005, COL_TOP, zc - b / 2 - 0.004), (COL_X1 + 0.005, COL_TOP + 0.010, zc + b / 2 + 0.004)), "rack-steel", 0.6, "rack"))
        elif scale == 100:
            P.append(Part(f"column-{k}", "rack", "box", ((COL_X0, FOOT_H, zc - b / 2), (COL_X1, COL_TOP, zc + b / 2)), "rack-steel", 0.55, "rack"))
            P.append(Part(f"foot-{k}", "rack", "box", ((FOOT_X0, 0.0, zc - b / 2), (FOOT_X1, FOOT_H, zc + b / 2)), "rack-steel", 0.5, "rack"))
        else:
            frame = [(FOOT_X0, 0.0), (FOOT_X1, 0.0), (FOOT_X1, FOOT_H), (COL_X1, FOOT_H), (COL_X1, COL_TOP), (COL_X0, COL_TOP), (COL_X0, FOOT_H), (FOOT_X0, FOOT_H)]
            P.append(Part(f"frame-{k}", "rack", "ext", ("xy", frame, None, zc - b / 2, zc + b / 2), "rack-steel", 0.55, "rack"))
        for lvl, top in ARM_TOP.items():
            bot = top - IPE100["h"]
            aw = IPE100["b"] if scale != 500 else FEATURE[500]
            if fine:
                P.append(Part(f"bracket-{lvl}-{k}", "rack", "box", ((COL_X1, bot - BRK_DOWN, zc - BRK_W / 2), (BRK_X1, top + BRK_UP, zc + BRK_W / 2)), "rack-steel", 0.6, "rack"))
                aprof = [(bot + p, zc + q) for p, q in i_section(IPE100, 2)]
                P.append(Part(f"arm-{lvl}-{k}", "rack", "ext", ("yz", aprof, None, BRK_X1, ARM_X1), "rack-steel", 0.6, "rack"))
            else:
                P.append(Part(f"arm-{lvl}-{k}", "rack", "box", ((COL_X1, bot, zc - aw / 2), (ARM_X1, top, zc + aw / 2)), "rack-steel", 0.6, "rack"))
            if scale in (50, 100):
                st = STOP_T if fine else FEATURE[100]
                P.append(Part(f"stop-{lvl}-{k}", "rack", "box", ((ARM_X1 - st, top, zc - IPE100["b"] / 2), (ARM_X1, top + STOP_H, zc + IPE100["b"] / 2)), "rack-steel", 0.45, "rack"))
    for j, yt in enumerate(TIE_Y):
        a, tt = L50["a"], L50["t"]
        if fine:
            prof = [(-a, yt), (0.0, yt), (0.0, yt + a), (-tt, yt + a), (-tt, yt + tt), (-a, yt + tt)]
            P.append(Part(f"tie-{j}", "rack", "ext", ("xy", prof, None, -TIE_Z, TIE_Z), "rack-steel", 0.5, "rack", extra={"slender": True}))
        else:
            a = FEATURE[500] if scale == 500 else a
            P.append(Part(f"tie-{j}", "rack", "box", ((-a, yt, -TIE_Z), (0.0, yt + a, TIE_Z)), "rack-steel", 0.5, "rack"))
    slender = {"slender": True} if scale == 500 else {}
    for (pid, _, _), (cx, cy, rr) in zip(BUNDLE, bundle_circles()):
        P.append(Part(f"bundle-{pid}", "bundle", "cylz", (cx, cy, PIPE_Z0, PIPE_Z1, rr), "black-steel", 0.22 if pid != "b2" else 0.26, "bundle-pipes", views=("elevation", "front", "plan") if scale in (50, 100) else (), extra=slender, lists=("thermal", "structure") if scale == 500 else ("body", "thermal", "structure")))
    if scale == 500:
        P.append(Part("bundle-mass", "bundle", "ext", ("xy", circles_hull(bundle_circles(), 0.0, 8), None, PIPE_Z0, PIPE_Z1), "black-steel", 0.22, "bundle-pipes", views=(), lists=("body",)))
    for (pid, dn, _, _), (cx, cy, rr) in zip(STOCK, stock_circles()):
        P.append(Part(f"stock-{pid}", "stock", "cylz", (cx, cy, PIPE_Z0, PIPE_Z1, rr), "black-steel", 0.24, "stock-pipes", views=(), extra=slender, lists=("thermal", "structure") if scale == 500 else ("body", "thermal", "structure")))
    if scale == 500:
        P.append(Part("stock-mass", "stock", "box", ((STOCK_X0, ARM_TOP["upper"], PIPE_Z0), (STOCK_X1, ARM_TOP["upper"] + FEATURE[500], PIPE_Z1)), "black-steel", 0.24, "stock-pipes", lists=("body",)))
    if fine:
        bc = bundle_circles()
        inner = circles_hull(bc, 0.0002)
        outer = circles_hull(bc, 0.0002 + STRAP_T)
        for j, zs in enumerate(STRAP_Z):
            P.append(Part(f"strap-{j}", "bundle", "ext", ("xy", outer, [inner[::-1]], zs - STRAP_W / 2, zs + STRAP_W / 2), "strap-steel", 0.62, "bundle-pipes", extra={"slender": True}))
            top = max(p[1] for p in outer)
            sx = BUNDLE[5][1]
            P.append(Part(f"strap-seal-{j}", "bundle", "box", ((sx - 0.016, top - 0.0003, zs - 0.0125), (sx + 0.016, top + 0.0077, zs + 0.0125)), "strap-steel", 0.5, "bundle-pipes"))
    if scale in (50, 100, 200):
        cx, cy, _ = [c for (pid, _, _), c in zip(BUNDLE, bundle_circles()) if pid == SOCKET_ON_BUNDLE][0]
        od, ln, _ = SOCKET_50
        P.append(Part("bundle-socket", "bundle", "cylz", (cx, cy, PIPE_Z1 - ln / 2, PIPE_Z1 + ln / 2, od / 2), "malleable-iron", 0.4, "bundle-pipes"))
        cx, cy, _ = [c for (pid, _, _, _), c in zip(STOCK, stock_circles()) if pid == SOCKET_ON_STOCK][0]
        od, ln, _ = SOCKET_40
        P.append(Part("stock-socket", "stock", "cylz", (cx, cy, PIPE_Z1 - ln / 2, PIPE_Z1 + ln / 2, od / 2), "malleable-iron", 0.4, "stock-pipes"))
        cx, cy, rr = [c for (pid, _, _, _), c in zip(STOCK, stock_circles()) if pid == ELBOW_ON_STOCK][0]
        od, ce, _ = ELBOW_25
        zc = PIPE_Z1 + ce - od / 2
        P.append(Part("stock-elbow-run", "stock", "cylz", (cx, cy, PIPE_Z1 - 0.018, zc + od / 2, od / 2), "malleable-iron", 0.4, "stock-pipes"))
        P.append(Part("stock-elbow-leg", "stock", "cyly", (cx, zc, cy, cy + ce, od / 2), "malleable-iron", 0.4, "stock-pipes"))
        if scale in (50, 100):
            P.append(Part("stock-nipple", "stock", "cyly", (cx, zc, cy + ce - 0.012, cy + ce + NIPPLE_25, rr), "black-steel", 0.3, "stock-pipes"))
    return P


def row_outline(circles, base, n=12):
    """〰️ The outline of a row of tubes lying side by side on one arm: the upper half of every tube, closed along the arm top."""
    pts = [(circles[0][0] - circles[0][2], base)]
    for cx, cy, rr in circles:
        pts += [(cx + rr * math.cos(math.pi * (1 - k / n)), cy + rr * math.sin(math.pi * (1 - k / n))) for k in range(n + 1)]
    pts.append((circles[-1][0] + circles[-1][2], base))
    out = []
    for p in pts:
        if not out or math.dist(out[-1], p) > 1e-6:
            out.append(p)
    return out


def group_shapes(scale, view):
    """🫧 The generalised outlines of the bundle and the loose row at 1:200 and 1:500, where single tubes are below a printable size."""
    out = []
    groups = {50: [("stock-row", "stock", stock_circles(), "stock-pipes", 0.24)], 100: [("stock-row", "stock", stock_circles(), "stock-pipes", 0.24)],
              200: [("bundle-outline", "bundle", bundle_circles(), "bundle-pipes", 0.22), ("stock-outline", "stock", stock_circles(), "stock-pipes", 0.24)],
              500: [("bundle-outline", "bundle", bundle_circles(), "bundle-pipes", 0.22)]}[scale]
    for gid, bone, circles, region, tone in groups:
        ox, oy = ORIGIN[bone]
        h = row_outline(circles, ARM_TOP["upper"]) if gid == "stock-row" else circles_hull(circles, 0.0, 8 if scale == 500 else 12)
        x0, x1 = min(p[0] for p in h), max(p[0] for p in h)
        y0, y1 = min(p[1] for p in h), max(p[1] for p in h)
        base = {"id": gid, "bone": bone, "role": "body", "filled": True, "tone": tone, "material": "black-steel", "region": region}
        if view == "elevation":
            out.append(dict(base, z=round(10 - PIPE_Z0, 4), kind="poly", points=[[r6(a - ox), r6(b - oy)] for a, b in h], closed=True))
        elif view == "front":
            out.append(dict(base, z=round(10 + x1, 4), kind="rect", x=r6(PIPE_Z0), y=r6(y0), w=r6(PIPE_L), h=r6(y1 - y0)))
        else:
            out.append(dict(base, z=round(10 + y1, 4), kind="rect", x=r6(x0), y=r6(PIPE_Z0), w=r6(x1 - x0), h=r6(PIPE_L)))
    return out


def stroke(sid, bone, pts, role="texture", region=None, tone=0.2, z=30.0, closed=False, filled=False):
    """🖋️ One open or closed detail or texture stroke."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}
    if region:
        s["region"] = region
    return s


def details_of(scale):
    """🔍 Traces of use as detail and texture shapes: open bores of the cut ends, thread runs, rust streaks, chalk marks, paint chips and scuffs."""
    e, f, p = [], [], []
    if scale >= 200:
        return e, f, p
    for bone, circles, prefix, region, ids in (("bundle", bundle_circles(), "bundle", "bundle-pipes", [b[0] for b in BUNDLE]), ("stock", stock_circles(), "stock", "stock-pipes", [s[0] for s in STOCK])):
        ox, oy = ORIGIN[bone]
        for pid, (cx, cy, rr) in zip(ids, circles):
            dn = "DN50" if bone == "bundle" else [s[1] for s in STOCK if s[0] == pid][0]
            ri = rr - EN10255[dn][1]
            if scale == 50 or bone == "stock":
                e.append({"id": f"{prefix}-{pid}-bore", "bone": bone, "role": "detail", "z": 40.0, "filled": True, "tone": 0.06, "region": region, "kind": "ellipse", "cx": r6(cx - ox), "cy": r6(cy - oy), "rx": r6(ri), "ry": r6(ri)})
    if scale == 50:
        for k, (cx, cy, rr) in enumerate(bundle_circles()[:4]):
            for j in range(3):
                zt = PIPE_Z0 + 0.012 + 0.012 * j
                p.append(stroke(f"bundle-thread-{k}-{j}", "bundle", [(cx - rr * 0.8, zt), (cx + rr * 0.8, zt)], region="bundle-pipes", tone=0.12))
        for k, (cx, cy, rr) in enumerate(stock_circles()):
            if k % 3 == 1:
                for j in range(2):
                    zt = PIPE_Z0 + 0.010 + 0.010 * j
                    p.append(stroke(f"stock-thread-{k}-{j}", "stock", [(cx - rr * 0.8, zt), (cx + rr * 0.8, zt)], region="stock-pipes", tone=0.12))
        bt = bundle_circles()[-2]
        for k, (z0, z1) in enumerate(((-2.6, -1.9), (-0.7, 0.4), (0.9, 1.3), (2.2, 2.8))):
            f.append(stroke(f"bundle-rust-{k}", "bundle", [(z0, bt[1] + 0.004 * (k % 2)), ((z0 + z1) / 2, bt[1] - 0.006), (z1, bt[1] + 0.002)], region="bundle-pipes", tone=0.45))
        f.append(stroke("bundle-chalk-mark", "bundle", [(-0.4, bt[1] + 0.012), (-0.25, bt[1] + 0.012), (-0.2, bt[1] + 0.016)], region="bundle-pipes", tone=0.95))
        st = stock_circles()[1]
        for k, (z0, z1) in enumerate(((-2.3, -1.2), (0.3, 1.6))):
            f.append(stroke(f"stock-rust-{k}", "stock", [(z0, st[1]), ((z0 + z1) / 2, st[1] - 0.005), (z1, st[1] + 0.003)], region="stock-pipes", tone=0.45))
        for k, zc in enumerate(COL_Z):
            f.append(stroke(f"column-chip-{k}", "rack", [(zc - 0.02, 0.32 + 0.11 * k), (zc - 0.006, 0.34 + 0.11 * k), (zc + 0.012, 0.325 + 0.11 * k)], region="rack", tone=0.85))
            f.append(stroke(f"foot-scuff-{k}", "rack", [(zc - 0.03, 0.05), (zc + 0.03, 0.06)], region="rack", tone=0.8))
        e.append(stroke("foot-scuff", "rack", [(0.55, 0.03), (0.78, 0.045), (0.95, 0.04)], region="rack", tone=0.8))
        e.append(stroke("column-label", "rack", [(0.03, 1.45), (0.13, 1.45), (0.13, 1.51), (0.03, 1.51)], role="detail", region="rack", tone=0.92, closed=True, filled=True, z=35.0))
    return e, f, p


def bodies_and_shapes(scale):
    """🧊 The body solids and the three 2D views of one scale from one part list."""
    parts = parts_of(scale)
    geo = {"elevation": [], "front": [], "plan": []}
    for part in parts:
        for view in part.views:
            geo[view].append(part.shape(view))
    for view in geo:
        geo[view] += group_shapes(scale, view)
    e, f, p = details_of(scale)
    geo["elevation"] += e
    geo["front"] += f
    geo["plan"] += p
    return parts, [x.solid() for x in parts if "body" in x.lists], geo


def thermal_list(scale, parts):
    """🌡️ The thermal list of one scale: every exposed body solid in its region and the patch of the warm plume over the bundle."""
    out = [{"id": f"th-{x.id}", "of": x.id, "region": x.region} if "body" in x.lists else dict(x.solid(), id=f"th-{x.id}", region=x.region) for x in parts if "thermal" in x.lists]
    f = FEATURE[scale]
    out.append({"id": "th-warm-plume", "kind": "box", "bone": "bundle", "material": "black-steel", "role": "body", "tone": 0.3, "size": [r6(BUNDLE_W * 0.6), r6(f), r6(PIPE_L - 2 * f)], "transform": {"translation": [r6(CX_BUNDLE), r6(BUNDLE_H + f / 2), 0]}, "region": "bundle-pipes", "tags": ["warm-plume"]})
    return out


def structure_list(scale, parts):
    """🏗️ The structure list of one scale: columns, feet, arms, ties and tubes as members, brackets, caps, stops, straps and fittings as joints."""
    out = []
    for x in parts:
        joint = any(x.id.startswith(k) for k in ("bracket", "column-cap", "stop", "strap", "bundle-socket", "stock-socket", "stock-elbow", "stock-nipple"))
        if "structure" not in x.lists:
            continue
        ref = {"of": x.id} if "body" in x.lists else x.solid()
        out.append(dict(ref, id=f"st-{x.id}", member=x.bone, part="joint" if joint else "member"))
    return out


def mass_of(parts, bone):
    """⚖️ Mass and centre (bone frame) of the parts of one bone from section x length x density, fittings by catalogue mass."""
    ox, oy = ORIGIN[bone]
    m, mx, my = 0.0, 0.0, 0.0
    for x in parts:
        if x.bone != bone:
            continue
        (x0, y0, z0), (x1, y1, z1) = x.bounds()
        if x.id.startswith("column-") and not x.id.startswith("column-cap"):
            kg = IPE160["kgm"] * (y1 - y0)
        elif x.id.startswith("frame-"):
            kg = IPE160["kgm"] * (COL_TOP - FOOT_H + FOOT_X1 - FOOT_X0)
        elif x.id.startswith("foot-"):
            kg = IPE160["kgm"] * (x1 - x0)
        elif x.id.startswith("arm-"):
            kg = IPE100["kgm"] * (x1 - x0)
        elif x.id.startswith("tie-"):
            kg = L50["kgm"] * (z1 - z0)
        elif x.id.startswith("bundle-b") or x.id.startswith("bundle-t"):
            kg = pipe_kgm("DN50") * (z1 - z0)
        elif x.id.startswith("stock-dn"):
            kg = pipe_kgm(x.id.split("-")[1].upper()) * (z1 - z0)
        elif x.id == "bundle-socket":
            kg = SOCKET_50[2]
        elif x.id == "stock-socket":
            kg = SOCKET_40[2]
        elif x.id == "stock-elbow-run":
            kg = ELBOW_25[2]
        elif x.id == "stock-elbow-leg":
            kg = 0.0
        elif x.id == "stock-nipple":
            kg = pipe_kgm("DN25") * NIPPLE_25
        elif x.id.startswith("strap-seal"):
            kg = 0.012
        elif x.id.startswith("strap-"):
            per = sum(math.dist(a, b) for a, b in zip(x.geo[1], x.geo[1][1:] + x.geo[1][:1]))
            kg = per * STRAP_W * STRAP_T * RHO
        else:
            kg = (x1 - x0) * (y1 - y0) * (z1 - z0) * RHO
        m += kg
        mx += kg * ((x0 + x1) / 2 - ox)
        my += kg * ((y0 + y1) / 2 - oy)
    return m, mx / m, my / m


FINE_PARTS = parts_of(50)
M_RACK, CX_RACK, CY_RACK = mass_of(FINE_PARTS, "rack")
M_STOCK, CX_STOCK, CY_STOCK = mass_of(FINE_PARTS, "stock")
M_BUNDLE, CX_BUNDLE, CY_BUNDLE = mass_of(FINE_PARTS, "bundle")
MASS = round(M_RACK + M_STOCK + M_BUNDLE, 2)
WATER_KG = 7 * PIPE_L * pipe_inner_area("DN50") * 1000.0
BUNDLE_A = 7 * math.pi * (EN10255["DN50"][0] - EN10255["DN50"][1]) * EN10255["DN50"][1]
STOCK_A = sum(math.pi * (EN10255[dn][0] - EN10255[dn][1]) * EN10255[dn][1] for _, dn, _, _ in STOCK)


def world_bounds():
    """📏 Overall rest extents of the 1:50 part list."""
    bb = [x.bounds() for x in FINE_PARTS]
    return [min(b[0][i] for b in bb) for i in range(3)], [max(b[1][i] for b in bb) for i in range(3)]


LO, HI = world_bounds()
HEIGHT, WIDTH, DEPTH = r6(HI[1] - LO[1]), r6(HI[0] - LO[0]), r6(HI[2] - LO[2])

MATERIALS = [
    {"id": "black-steel", "title": t("Black steel tube S195T EN 10255, mill scale with light surface rust", "Schwarzes Stahlrohr S195T EN 10255, Walzhaut mit leichtem Flugrost"), "emissivity": 0.8, "solarAbsorptivity": 0.8, "densityKgM3": RHO, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 195e6, "source": "literature", "reference": "en-10255"},
    {"id": "rack-steel", "title": t("Structural steel S235, painted gentian blue, scuffed", "Baustahl S235, enzianblau lackiert, verschrammt"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": RHO, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-10025-2"},
    {"id": "malleable-iron", "title": t("Black malleable cast iron fitting EN 10242", "Schwarzer Temperguss-Fitting EN 10242"), "emissivity": 0.8, "solarAbsorptivity": 0.8, "densityKgM3": RHO_MI, "conductivityWmK": 40, "specificHeatJKgK": 500, "elasticModulusPa": 170e9, "strengthPa": 200e6, "source": "literature", "reference": "en-10242"},
    {"id": "strap-steel", "title": t("Blued high-tensile steel strapping 19 x 0.6 mm", "Gebläutes hochfestes Stahlband 19 x 0,6 mm"), "emissivity": 0.7, "solarAbsorptivity": 0.7, "densityKgM3": RHO, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 205e9, "strengthPa": 830e6, "source": "literature", "reference": "en-13246"},
]

REFERENCES = [
    {"id": "en-10255", "citation": "EN 10255:2004+A1:2007 Non-alloy steel tubes suitable for welding and threading, technical delivery conditions (medium series M: DN 25 33.7 x 3.2 mm 2.41 kg/m, DN 32 42.4 x 3.2 mm 3.10 kg/m, DN 40 48.3 x 3.2 mm 3.56 kg/m, DN 50 60.3 x 3.6 mm 5.03 kg/m plain end; steel S195T; random lengths about 6 m). Successor of DIN 2440 (Gewinderohr mittelschwer)."},
    {"id": "din-2440", "citation": "DIN 2440:1978 Steel tubes, medium-weight, suitable for screwing (withdrawn, replaced by EN 10255 series M; same outside diameters and wall thicknesses)."},
    {"id": "en-10226-1", "citation": "EN 10226-1:2004 Pipe threads where pressure tight joints are made on the threads, taper external threads R (thread length of R 2 about 24 mm, R 1 about 17 mm)."},
    {"id": "en-10242", "citation": "EN 10242:1994+A2:2003 Threaded pipe fittings in malleable cast iron (socket M2 and elbow A1; centre-to-end of an A1 elbow size 1 about 38 mm; masses from manufacturer tables, socket 2 about 0.45 kg, socket 1 1/2 about 0.30 kg, elbow 1 about 0.20 kg)."},
    {"id": "en-10365", "citation": "EN 10365:2017 Hot rolled steel channels, I and H sections, dimensions and masses (IPE 160: 160 x 82 x 5.0 x 7.4 mm, r 9 mm, 20.1 cm2, 15.8 kg/m, Iy 869 cm4; IPE 100: 100 x 55 x 4.1 x 5.7 mm, r 7 mm, 10.3 cm2, 8.1 kg/m, Iy 171 cm4)."},
    {"id": "en-10056-1", "citation": "EN 10056-1:2017 Structural steel equal and unequal leg angles, dimensions (L 50 x 50 x 5: 4.80 cm2, 3.77 kg/m)."},
    {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy structural steels (S235JR: yield strength 235 MPa)."},
    {"id": "en-15635", "citation": "EN 15635:2008 Steel static storage systems, application and maintenance of storage equipment (inspection of cantilever racking, arm deflection, end stops)."},
    {"id": "fem-10-2-09", "citation": "FEM 10.2.09:2012 Design of cantilever racking (single-sided cantilever racks of I-section columns with welded feet and bolted arms inclined 0 to 2 degrees; arm loads of 300 to 1000 kg on light racks); dimensions here are typical of a light rack (authored-estimate)."},
    {"id": "en-13246", "citation": "EN 13246:2001 Packaging, determination of the strength of steel strapping (high-tensile blued strap 19 x 0.6 mm, about 830 MPa)."},
    {"id": "iso-2328", "citation": "ISO 2328:2011 Fork-lift trucks, hook-on type fork arms and fork arm carriages (class 2A fork arms 100 x 40 mm; tine spread of 2 m on a side loader used for the lifted pose)."},
    {"id": "din-en-12828", "citation": "DIN EN 12828:2014 Heating systems in buildings, design for water-based heating systems (flow and return temperatures 70/55 degC used for the reinstalled distribution run)."},
    {"id": "recknagel-2019", "citation": "Recknagel, Sprenger, Albers: Taschenbuch fuer Heizung und Klimatechnik, 79th ed. (2019), pipe heat emission of uninsulated steel tubes (about 70 W/m for DN 50 at 50 K over room air)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, Table 5 emissivities: oxidised steel 0.79, rough painted steel 0.9; new rolled steel with bright mill scale 0.6 to 0.7; weathered values differ from new metal."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (steel 50 W/mK, 450 J/kgK, 7850 kg/m3; cast iron 40 to 50 W/mK, 500 J/kgK)."},
]

BONES = [
    {"id": "yard", "parent": None, "title": t("Yard floor (frame of the rack)", "Lagerplatz (Bezugsrahmen des Regals)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.01, "joint": {"type": "fixed"}},
    {"id": "rack", "parent": "yard", "title": t("Cantilever rack: three IPE 160 columns on IPE 160 feet, six IPE 100 arms on bolted plates, two L 50 ties (stays standing when the bundle is taken out)", "Kragarmregal: drei Stützen IPE 160 auf Füßen IPE 160, sechs Kragarme IPE 100 an geschraubten Platten, zwei Riegel L 50 (bleibt stehen, wenn das Bündel entnommen wird)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": r6(FOOT_X1), "joint": {"type": "fixed"},
     "mass": {"kg": round(M_RACK, 3), "comX": r6(CX_RACK), "comY": r6(CY_RACK), "distribution": "uniform", "source": "literature", "reference": "en-10365"},
     "section": {"areaM2": IPE160["A"], "secondMomentM4": IPE160["I"], "extremeFibreM": IPE160["h"] / 2, "material": "rack-steel", "title": t("Column IPE 160", "Stütze IPE 160")}},
    {"id": "stock", "parent": "rack", "title": t("Loose row on the upper arms: three DN 40, three DN 32 and four DN 25 lengths with a socket, an elbow and a nipple stub left on", "Lose Lage auf den oberen Kragarmen: drei Rohre DN 40, drei DN 32 und vier DN 25 mit einer Muffe, einem Winkel und einem Nippelrest"), "x": STOCK_X0, "y": ARM_TOP["upper"], "angleDeg": 0, "lengthM": r6(STOCK_X1 - STOCK_X0), "joint": {"type": "fixed"},
     "mass": {"kg": round(M_STOCK, 3), "comX": r6(CX_STOCK), "comY": r6(CY_STOCK), "distribution": "uniform", "source": "literature", "reference": "en-10255"},
     "section": {"areaM2": r6(STOCK_A), "secondMomentM4": float(f"{sum(pipe_i(dn) for _, dn, _, _ in STOCK):.4e}"), "extremeFibreM": EN10255["DN40"][0] / 2, "material": "black-steel", "title": t("Ten tubes DN 25 to DN 40 side by side, each acting alone", "Zehn Rohre DN 25 bis DN 40 nebeneinander, jedes für sich tragend")}},
    {"id": "carrier-out", "parent": "yard", "title": t("Side loader travel (massless, carries the bundle out of the rack)", "Fahrweg des Seitenstaplers (masselos, fährt das Bündel aus dem Regal)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.01, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.0}},
    {"id": "carrier-up", "parent": "carrier-out", "title": t("Fork lift (massless, raises the bundle off the arms)", "Gabelhub (masselos, hebt das Bündel von den Kragarmen)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.01, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 2.5}},
    {"id": "bundle", "parent": "carrier-up", "title": t("Strapped bundle of seven DN 50 lengths, four below and three nested above, with two steel straps and a socket left on one end", "Umreiftes Bündel aus sieben Rohren DN 50, vier unten und drei darüber eingebettet, mit zwei Stahlbändern und einer Muffe an einem Ende"), "x": BUNDLE_X0, "y": ARM_TOP["lower"], "angleDeg": 0, "lengthM": r6(BUNDLE_W), "joint": {"type": "fixed"},
     "mass": {"kg": round(M_BUNDLE, 3), "comX": r6(CX_BUNDLE), "comY": r6(CY_BUNDLE), "distribution": "uniform", "source": "literature", "reference": "en-10255"},
     "section": {"areaM2": r6(BUNDLE_A), "secondMomentM4": float(f"{7 * pipe_i('DN50'):.4e}"), "extremeFibreM": R50, "material": "black-steel", "title": t("Seven tubes DN 50 60.3 x 3.6 mm, each acting alone", "Sieben Rohre DN 50 60,3 x 3,6 mm, jedes für sich tragend")}},
]


def anchor(id_, bone, x, y, z, kind, en, de, **kw):
    """📍 One anchor in the bone frame."""
    a = {"id": id_, "bone": bone, "x": r6(x), "y": r6(y), "z": r6(z), "kind": kind, "title": t(en, de)}
    a.update(kw)
    return a


SUP = {"support": "contact", "normalDeg": 90, "frictionCoefficient": 0.4}
ANCHORS = []
for k, zc in enumerate(COL_Z):
    for end, x, en, de in (("rear", FOOT_X0 + 0.05, "rear", "hinteres"), ("front", FOOT_X1 - 0.05, "front", "vorderes")):
        ANCHORS.append(anchor(f"foot-{k}-{end}", "rack", x, 0, zc, "support", f"Ground under the {en} end of foot {k + 1}", f"Boden unter dem {de} Ende von Fuß {k + 1}", footprintLengthM=0.1, footprintWidthM=IPE160["b"], **SUP))
for j, zt in enumerate(TINE_Z):
    ANCHORS.append(anchor(f"tine-{j}", "bundle", CX_BUNDLE, 0, zt, "support", f"Fork tine {j + 1} under the bundle", f"Gabelzinke {j + 1} unter dem Bündel", footprintLengthM=BUNDLE_W, footprintWidthM=0.1, **SUP))
for j, zh in enumerate(HANGER_Z):
    ANCHORS.append(anchor(f"hanger-{j}", "bundle", CX_BUNDLE, 0, zh, "support", f"Trapeze hanger {j + 1} under the reinstalled run", f"Trapezhänger {j + 1} unter dem wieder eingebauten Strang", footprintLengthM=BUNDLE_W, footprintWidthM=0.05, **SUP))
ANCHORS.append(anchor("water-centre", "bundle", CX_BUNDLE, CY_BUNDLE, 0, "load", "Centre of the heating water in the seven tubes", "Schwerpunkt des Heizwassers in den sieben Rohren"))
ANCHORS.append(anchor("bundle-top", "bundle", CX_BUNDLE, BUNDLE_H, 0, "emitter", "Top of the bundle, where the warm plume rises", "Oberseite des Bündels, wo die warme Fahne aufsteigt"))

FOOT_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("foot-")]
TINE_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("tine-")]
HANGER_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("hanger-")]

PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DECISIONS = {
    50: {
        "silhouette": t("Side elevation seen from the right along the pipes: the cantilever profile of the rack, column IPE 160 on its 1220 mm foot, the two IPE 100 arms 800 mm long on their bolted plates with end stops, the L 50 ties behind the column, and the open cut ends of the seven DN 50 tubes of the bundle and of the ten DN 40, DN 32 and DN 25 tubes of the loose row, with the elbow and nipple standing up at the far end. Front view: the three columns 2.1 m apart and the 6 m tubes as long bands. Plan: the I-sections of the columns, the arms and feet and the tubes from above.", "Seitenansicht von rechts entlang der Rohre: das Kragarmprofil des Regals, Stütze IPE 160 auf ihrem 1220 mm langen Fuß, die beiden Kragarme IPE 100 von 800 mm an ihren geschraubten Platten mit Endanschlägen, die Riegel L 50 hinter der Stütze und die offenen Schnittenden der sieben Rohre DN 50 des Bündels und der zehn Rohre DN 40, DN 32 und DN 25 der losen Lage, mit dem am fernen Ende aufragenden Winkel und Nippel. Vorderansicht: die drei Stützen im Abstand von 2,1 m und die 6 m langen Rohre als lange Bänder. Draufsicht: die I-Querschnitte der Stützen, Arme und Füße und die Rohre von oben."),
        "retained": t("True I-sections with root fillets in plan and front, the 3.2 and 3.6 mm walls as the open bores of the cut ends, the two straps with their seals, thread runs at the cut ends, rust streaks, a chalk size mark, paint chips and forklift scuffs on the rack, the rack load label.", "Wahre I-Querschnitte mit Ausrundungen in Draufsicht und Vorderansicht, die Wände von 3,2 und 3,6 mm als offene Bohrungen der Schnittenden, die beiden Bänder mit ihren Plomben, Gewindeauslauf an den Schnittenden, Roststreifen, eine Kreidemarkierung der Nennweite, Lackabplatzungen und Staplerschrammen am Regal, das Traglastschild."),
        "merged": t("Bolts of the arm plates and the levelling shims under the feet are part of plates and feet.", "Schrauben der Armplatten und Unterlegbleche unter den Füßen sind Teil von Platten und Füßen."),
        "omitted": t("Single thread turns and the weld seams (below 7.5 mm).", "Einzelne Gewindegänge und die Schweißnähte (unter 7,5 mm)."),
        "thinElements": t("The straps are 0.6 mm sheet drawn at their true thickness; the ties are L 50 x 5 angles.", "Die Bänder sind Blech von 0,6 mm in wahrer Dicke; die Riegel sind Winkel L 50 x 5."),
        "hatching": t("No hatch: a view of painted rolled sections and black tube; tone separates the blue rack, the dark tubes, the open bores and the fittings.", "Keine Schraffur: Ansicht von lackierten Walzprofilen und schwarzem Rohr; der Ton trennt das blaue Regal, die dunklen Rohre, die offenen Bohrungen und die Fittings."),
        "styles": t("Outline 0.35 mm, bores, label and straps 0.18 mm, threads, rust, chalk and scuffs 0.1 mm.", "Kontur 0,35 mm, Bohrungen, Schild und Bänder 0,18 mm, Gewinde, Rost, Kreide und Schrammen 0,1 mm."),
        "overlays": t("Rack, bundle and loose row are three regions; every tube is its own cell.", "Regal, Bündel und lose Lage sind drei Regionen; jedes Rohr ist eine eigene Zelle."),
    },
    100: {
        "silhouette": t("The same rack and tubes at 1:100: columns, feet and arms as their outer rectangles, every tube as a circle in the side elevation and a band in front and plan, the sockets and the elbow at the far ends.", "Dasselbe Regal mit Rohren in 1:100: Stützen, Füße und Arme als ihre äußeren Rechtecke, jedes Rohr als Kreis in der Seitenansicht und als Band in Vorderansicht und Draufsicht, Muffen und Winkel an den fernen Enden."),
        "retained": t("Every tube on its own, the end stops (drawn 15 mm thick), the rust streaks and the chalk mark.", "Jedes Rohr für sich, die Endanschläge (15 mm dick gezeichnet), die Roststreifen und die Kreidemarkierung."),
        "merged": t("Arm plates, column caps and the flanges and webs of the I-sections are merged into the outer rectangles of the sections.", "Armplatten, Stützenkappen sowie Flansche und Stege der I-Profile gehen in den äußeren Rechtecken der Profile auf."),
        "omitted": t("Straps (19 mm, 0.19 mm on paper), bores, threads, chips and the label.", "Bänder (19 mm, 0,19 mm auf Papier), Bohrungen, Gewinde, Abplatzungen und Schild."),
        "thinElements": t("None below the 15 mm feature.", "Keine unter der kleinsten Einzelheit von 15 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm, texture 0.1 mm.", "Kontur 0,25 mm, Textur 0,1 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    200: {
        "silhouette": t("Rack outline with columns, feet, arms and ties; the bundle drawn as its convex outline 241 x 112 mm and the loose row as its outline 427 x 48 mm, because a single DN 25 tube is 0.17 mm on paper here.", "Regalkontur mit Stützen, Füßen, Armen und Riegeln; das Bündel als seine konvexe Kontur 241 x 112 mm und die lose Lage als ihre Kontur 427 x 48 mm gezeichnet, weil ein einzelnes Rohr DN 25 hier 0,17 mm auf Papier misst."),
        "retained": t("The two arm levels, the 2.1 m column spacing and the 0.9 m overhang of the tubes beyond the outer columns.", "Die beiden Armebenen, der Stützenabstand von 2,1 m und der Überstand der Rohre von 0,9 m über die äußeren Stützen."),
        "merged": t("The single tubes into the two group outlines.", "Die einzelnen Rohre in die beiden Gruppenkonturen."),
        "omitted": t("End stops (10 mm) and all traces of use.", "Endanschläge (10 mm) und alle Gebrauchsspuren."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm.", "Kontur 0,18 mm."),
        "overlays": t("Each group outline is one cell of its region.", "Jede Gruppenkontur ist eine Zelle ihrer Region."),
    },
    500: {
        "silhouette": t("Three columns on their feet, the two arm levels and the two tube groups as outlines; front and plan the 6 m long groups over the three columns.", "Drei Stützen auf ihren Füßen, die beiden Armebenen und die beiden Rohrgruppen als Konturen; Vorderansicht und Draufsicht die 6 m langen Gruppen über den drei Stützen."),
        "retained": t("Overall size of rack and stock and the two levels.", "Gesamtgröße von Regal und Lager und die beiden Ebenen."),
        "merged": t("Ties drawn as bars of the 0.13 mm pen; arms drawn 75 mm wide.", "Riegel als Striche der Feder 0,13 mm; Arme 75 mm breit gezeichnet."),
        "omitted": t("Fittings and all traces of use.", "Fittings und alle Gebrauchsspuren."),
        "thinElements": t("The tube groups keep their true height.", "Die Rohrgruppen behalten ihre wahre Höhe."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": t("As at 1:200.", "Wie 1:200."),
    },
}

VOLUME_DECISIONS = {
    50: [
        t("Body: columns, feet and arms as extrudes of the true IPE 160 and IPE 100 sections with their root fillets, bolted arm plates 10 mm, end stops 10 mm, column caps, two L 50 x 5 ties as slender angle extrudes; every tube as a revolve of its true outside diameter 6 m long; the two 0.6 mm straps as declared sheets around the convex outline of the bundle with their crimp seals; a 2 inch socket on one DN 50 end, a 1 1/2 inch socket on a DN 40 end and a 1 inch elbow with a cut nipple on a DN 25 end.", "Körper: Stützen, Füße und Arme als Extrusionen der wahren Querschnitte IPE 160 und IPE 100 mit ihren Ausrundungen, geschraubte Armplatten 10 mm, Endanschläge 10 mm, Stützenkappen, zwei Riegel L 50 x 5 als schlanke Winkelextrusionen; jedes Rohr als Drehkörper seines wahren Außendurchmessers, 6 m lang; die beiden Bänder 0,6 mm als erklärte Bleche um die konvexe Kontur des Bündels mit ihren Plomben; eine Muffe 2 Zoll an einem Ende DN 50, eine Muffe 1 1/2 Zoll an einem Ende DN 40 und ein Winkel 1 Zoll mit abgesägtem Nippel an einem Ende DN 25."),
        t("Thermal: every body solid in its region (rack, bundle, loose row); the tubes are modelled as closed rods because their 6 m long bores are cavities that exchange almost nothing with the room.", "Thermisch: jeder Körper in seiner Region (Regal, Bündel, lose Lage); die Rohre sind als geschlossene Stäbe modelliert, weil ihre 6 m langen Bohrungen Hohlräume sind, die mit dem Raum fast nichts austauschen."),
        t("Structure: columns, feet, arms, ties and tubes are members of their bones, plates, caps, stops, straps and fittings are joints.", "Tragwerk: Stützen, Füße, Arme, Riegel und Rohre sind Stäbe ihrer Knochen, Platten, Kappen, Anschläge, Bänder und Fittings sind Knoten."),
    ],
    100: [
        t("Body: columns, feet and arms as boxes of the outer section (82 x 160 and 55 x 100 mm), ties as 50 x 50 mm bars, end stops 15 mm thick; every tube on its own; sockets, elbow and nipple kept; straps, seals, plates and caps left out (under 15 mm).", "Körper: Stützen, Füße und Arme als Quader des äußeren Querschnitts (82 x 160 und 55 x 100 mm), Riegel als Stäbe 50 x 50 mm, Endanschläge 15 mm dick; jedes Rohr für sich; Muffen, Winkel und Nippel bleiben; Bänder, Plomben, Platten und Kappen entfallen (unter 15 mm)."),
        t("Thermal: every body solid in its region.", "Thermisch: jeder Körper in seiner Region."),
        t("Structure: as 1:50 without plates, caps and straps.", "Tragwerk: wie 1:50 ohne Platten, Kappen und Bänder."),
    ],
    200: [
        t("Body: rack boxes as at 1:100 without end stops; every tube kept at its true diameter (DN 25 is 33.7 mm, above the 30 mm feature); the two sockets and the elbow kept, the nipple left out.", "Körper: Regalquader wie 1:100 ohne Endanschläge; jedes Rohr in seinem wahren Durchmesser (DN 25 misst 33,7 mm, über der kleinsten Einzelheit von 30 mm); die beiden Muffen und der Winkel bleiben, der Nippel entfällt."),
        t("Thermal: every body solid in its region.", "Thermisch: jeder Körper in seiner Region."),
        t("Structure: as 1:100 without end stops and nipple.", "Tragwerk: wie 1:100 ohne Endanschläge und Nippel."),
    ],
    500: [
        t("Body: columns and feet as boxes, arms widened from 55 to 75 mm (the smallest feature), ties as slender 50 mm bars; the seventeen tubes keep their true sections as slender rods (100 to 180 times as long as thick); fittings left out.", "Körper: Stützen und Füße als Quader, Arme von 55 auf 75 mm verbreitert (die kleinste Einzelheit), Riegel als schlanke Stäbe 50 mm; die siebzehn Rohre behalten als schlanke Stäbe ihre wahren Querschnitte (100- bis 180-mal so lang wie dick); Fittings entfallen."),
        t("Thermal: every body solid in its region, so the tube surface stays the true one.", "Thermisch: jeder Körper in seiner Region, so bleibt die Rohroberfläche die wahre."),
        t("Structure: every body solid as member of its bone.", "Tragwerk: jeder Körper als Stab seines Knochens."),
    ],
}


def representation(scale, geo):
    """🖨️ The authored representation of one scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": geo, "decisions": DECISIONS[scale]}


def volume(scale, parts, body):
    """🧊 The authored volume of one scale."""
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": VOLUME_DECISIONS[scale]}


def bind(rack, stock, bundle, bundle_contact=None):
    """🌬️ Region bindings of one pose (exposed, sunlit)."""
    regions = {"rack": {"exposedFraction": rack, "sunlitFraction": 0}, "stock-pipes": {"exposedFraction": stock, "sunlitFraction": 0}, "bundle-pipes": {"exposedFraction": bundle, "sunlitFraction": 0}}
    if bundle_contact:
        regions["bundle-pipes"]["contact"] = bundle_contact
    return regions


WATER_LOAD = [{"id": "heating-water", "kind": "carried", "title": t(f"Heating water in the seven DN 50 tubes, {WATER_KG:.1f} kg", f"Heizwasser in den sieben Rohren DN 50, {dez(f'{WATER_KG:.1f}')} kg"), "anchor": "water-centre", "fxN": 0, "fyN": r6(-WATER_KG * G), "source": "literature", "reference": "en-10255"}]
RAISED = {"carrier-up": {"dy": LIFT_DY}}
LIFTED = {"carrier-up": {"dy": LIFT_DY}, "carrier-out": {"dx": LIFT_DX}}
REIN = {"carrier-up": {"dy": REIN_DY}, "carrier-out": {"dx": REIN_DX}}

POSES = [
    {"id": "stored", "title": t("Stored in the cantilever rack", "Im Kragarmregal gelagert"), "description": t("The strapped DN 50 bundle lies on the lower arms, the loose DN 40, DN 32 and DN 25 lengths on the upper arms, all drained and dry, in the covered yard of the component exchange.", "Das umreifte Bündel DN 50 liegt auf den unteren Kragarmen, die losen Rohre DN 40, DN 32 und DN 25 auf den oberen, alle entleert und trocken, im überdachten Lager der Bauteilbörse."), "bones": {}, "ground": "authored", "contacts": FOOT_CONTACTS, "thermal": {"operating": {"heating": 0}, "regions": bind(0.95, 0.85, 0.85), "emitters": {"warm-plume": 0}}, "loads": []},
    {"id": "raised", "title": t("Raised off the arms", "Von den Kragarmen abgehoben"), "description": t("The side loader has run its tines under the bundle between the columns and raised it 80 mm, clear of the end stops.", "Der Seitenstapler hat seine Zinken zwischen den Stützen unter das Bündel gefahren und es 80 mm angehoben, frei von den Endanschlägen."), "bones": RAISED, "ground": "authored", "contacts": FOOT_CONTACTS + TINE_CONTACTS, "thermal": {"operating": {"heating": 0}, "regions": bind(0.95, 0.85, 0.9), "emitters": {"warm-plume": 0}}, "loads": []},
    {"id": "lifted", "title": t("Lifted out on the forks", "Auf der Gabel ausgelagert"), "description": t("The bundle on the two tines, carried 1.3 m out of the rack; the rack and the loose row stay behind.", "Das Bündel auf den beiden Zinken, 1,3 m aus dem Regal gefahren; Regal und lose Lage bleiben zurück."), "bones": LIFTED, "ground": "authored", "contacts": FOOT_CONTACTS + TINE_CONTACTS, "thermal": {"operating": {"heating": 0}, "regions": bind(0.95, 0.85, 0.9), "emitters": {"warm-plume": 0}}, "loads": []},
    {"id": "reinstalled-off", "title": t("Reinstalled, filled and cold", "Wieder eingebaut, befüllt und kalt"), "description": t("The seven lengths, threaded and coupled again, lie as the parallel mains of a plant-room distribution run on two trapeze hangers at 2.8 m, filled with water and not yet heated.", "Die sieben Längen, neu geschnitten und wieder verbunden, liegen als parallele Hauptleitungen eines Verteilstrangs im Technikraum auf zwei Trapezhängern in 2,8 m Höhe, mit Wasser gefüllt und noch nicht beheizt."), "bones": REIN, "ground": "authored", "contacts": FOOT_CONTACTS + HANGER_CONTACTS, "thermal": {"operating": {"heating": 0}, "regions": bind(0.95, 0.85, 0.9), "emitters": {"warm-plume": 0}}, "loads": WATER_LOAD},
    {"id": "reinstalled", "title": t("Reinstalled and heating, 70/55 degC", "Wieder eingebaut und heizend, 70/55 °C"), "description": t("The run in service: heating water at 70 degC flow and 55 degC return in the uninsulated DN 50 mains, about 70 W per metre of tube rising as a warm plume.", "Der Strang in Betrieb: Heizwasser mit 70 °C Vorlauf und 55 °C Rücklauf in den ungedämmten Hauptleitungen DN 50, etwa 70 W je Meter Rohr steigen als warme Fahne auf."), "bones": REIN, "ground": "authored", "contacts": FOOT_CONTACTS + HANGER_CONTACTS, "thermal": {"operating": {"heating": 1}, "regions": bind(0.95, 0.85, 0.9), "emitters": {"warm-plume": 1}}, "loads": WATER_LOAD},
]

CLIPS = [
    {"id": "lift", "title": t("Take the bundle out of the rack", "Bündel aus dem Regal nehmen"), "description": t("The side loader raises the bundle 80 mm off the lower arms in two seconds and carries it 1.3 m out of the rack in four; rack and loose row stay where they are.", "Der Seitenstapler hebt das Bündel in zwei Sekunden 80 mm von den unteren Kragarmen und fährt es in vier Sekunden 1,3 m aus dem Regal; Regal und lose Lage bleiben, wo sie sind."), "durationS": 8, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1, "pose": "stored", "ease": "smooth"}, {"t": 3, "pose": "raised", "ease": "smooth"}, {"t": 7, "pose": "lifted", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in FOOT_CONTACTS] + [{"anchor": a, "fromS": 1, "toS": 8} for a in TINE_CONTACTS],
     "phases": [{"id": "engage", "fromS": 0, "toS": 1, "title": t("Tines engage", "Zinken greifen")}, {"id": "raise", "fromS": 1, "toS": 3, "title": t("Raise", "Anheben")}, {"id": "carry-out", "fromS": 3, "toS": 7, "title": t("Carry out", "Ausfahren")}, {"id": "hold", "fromS": 7, "toS": 8, "title": t("Hold", "Halten")}],
     "events": [{"t": 1.2, "kind": "contact-off", "title": t("Bundle leaves the lower arms", "Bündel verlässt die unteren Kragarme"), "target": "bundle"}], "tracks": []},
    {"id": "heat-up", "title": t("Heating up after reinstallation", "Aufheizen nach dem Wiedereinbau"), "description": t("The circuit pump starts, the flow temperature rises from cold to 70 degC in 15 minutes and the warm plume over the run grows with it.", "Die Heizkreispumpe läuft an, die Vorlauftemperatur steigt in 15 Minuten von kalt auf 70 °C, und die warme Fahne über dem Strang wächst mit."), "durationS": 1200, "fps": 1, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 60, "pose": "reinstalled", "ease": "step"}, {"t": 1200, "pose": "reinstalled", "ease": "step"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 1200} for a in FOOT_CONTACTS + HANGER_CONTACTS],
     "phases": [{"id": "cold", "fromS": 0, "toS": 60, "title": t("Cold", "Kalt")}, {"id": "warm-up", "fromS": 60, "toS": 960, "title": t("Warming up", "Aufheizen")}, {"id": "design", "fromS": 960, "toS": 1200, "title": t("Design temperatures", "Auslegungstemperaturen")}],
     "events": [{"t": 60, "kind": "operating", "title": t("Circuit pump starts", "Heizkreispumpe läuft an"), "target": "heating"}, {"t": 960, "kind": "operating", "title": t("Flow reaches 70 degC", "Vorlauf erreicht 70 °C"), "target": "heating"}],
     "tracks": [{"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 960, "v": 1}, {"t": 1200, "v": 1}]}, {"target": "emitter:warm-plume", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 960, "v": 1}, {"t": 1200, "v": 1}]}]},
]

AREAS_FILE = HERE / "author-areas.json"
AREAS = json.loads(AREAS_FILE.read_text()) if AREAS_FILE.exists() else {"rack": 9.0, "bundle-pipes": 7.0, "stock-pipes": 6.5}


def wall_layer(dn, en, de):
    """🧱 The steel wall of a tube as one thermal layer."""
    s = EN10255[dn][1]
    return {"id": f"wall-{dn.lower()}", "title": t(en, de), "kind": "metal", "material": "black-steel", "thicknessM": s, "resistanceM2KW": r6(s / 50), "heatCapacityJM2K": round(s * RHO * 450, 1), "source": "literature", "reference": "en-10255"}


L_BUNDLE = [wall_layer("DN50", "Tube wall DN 50, 3.6 mm", "Rohrwand DN 50, 3,6 mm")]
L_STOCK = [wall_layer("DN40", "Tube wall DN 25 to DN 40, 3.2 mm", "Rohrwand DN 25 bis DN 40, 3,2 mm")]
L_RACK = [{"id": "flange", "title": t("Painted flange of the rolled sections, 7.4 mm", "Lackierter Flansch der Walzprofile, 7,4 mm"), "kind": "metal", "material": "rack-steel", "thicknessM": IPE160["tf"], "resistanceM2KW": r6(IPE160["tf"] / 50), "heatCapacityJM2K": round(IPE160["tf"] * RHO * 450, 1), "source": "literature", "reference": "en-10365"}]

THERMAL = {
    "environment": {"airTempC": 12, "meanRadiantTempC": 11, "relativeHumidity": 0.7, "windMS": 0.3, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [{"id": "heating", "title": t("Heating water through the reinstalled DN 50 mains", "Heizwasser durch die wieder eingebauten Hauptleitungen DN 50"), "levels": [{"v": 0, "title": t("Drained, or filled and cold", "Entleert oder befüllt und kalt")}, {"v": 0.5, "title": t("Reduced, 55/45 degC", "Abgesenkt, 55/45 °C")}, {"v": 1, "title": t("Design, 70/55 degC", "Auslegung, 70/55 °C")}]}],
    "regions": [
        {"id": "bundle-pipes", "title": t("Bundle of seven DN 50 black tubes with straps and socket", "Bündel aus sieben schwarzen Rohren DN 50 mit Bändern und Muffe"), "kind": "surface", "model": "hydronic", "material": "black-steel", "areaM2": AREAS["bundle-pipes"], "characteristicLengthM": EN10255["DN50"][0], "geometry": "cylinder", "skyViewFactor": 0.1, "layers": L_BUNDLE, "heatCapacityJM2K": L_BUNDLE[0]["heatCapacityJM2K"], "source": "authored-estimate", "reference": "recknagel-2019",
         "hydronic": {"key": "heating", "rows": [{"v": 0.5, "supplyTempC": 55, "returnTempC": 45}, {"v": 1, "supplyTempC": 70, "returnTempC": 55}], "flowPosition": 0.5, "filmM2KW": 0.0005}, "ratedPowerW": 0, "ratedPowerBasis": "passive"},
        {"id": "stock-pipes", "title": t("Loose row of DN 40, DN 32 and DN 25 black tubes with fittings", "Lose Lage schwarzer Rohre DN 40, DN 32 und DN 25 mit Fittings"), "kind": "surface", "model": "passive", "material": "black-steel", "areaM2": AREAS["stock-pipes"], "characteristicLengthM": EN10255["DN32"][0], "geometry": "cylinder", "skyViewFactor": 0.1, "layers": L_STOCK, "heatCapacityJM2K": L_STOCK[0]["heatCapacityJM2K"], "source": "authored-estimate", "reference": "en-10255"},
        {"id": "rack", "title": t("Painted cantilever rack", "Lackiertes Kragarmregal"), "kind": "surface", "model": "passive", "material": "rack-steel", "areaM2": AREAS["rack"], "characteristicLengthM": IPE160["h"], "geometry": "vertical-plate", "skyViewFactor": 0.1, "layers": L_RACK, "heatCapacityJM2K": L_RACK[0]["heatCapacityJM2K"], "source": "authored-estimate", "reference": "en-10365"},
    ],
    "emitters": [{"id": "warm-plume", "kind": "plume", "title": t("Warm convection plume over the heated DN 50 mains", "Warme Konvektionsfahne über den beheizten Hauptleitungen DN 50"), "anchor": "bundle-top", "directionDeg": 90, "diameterM": r6(BUNDLE_W), "region": "bundle-pipes", "source": "authored-estimate", "reference": "recknagel-2019"}],
    "assumptions": [
        t("All regions are passive (0 W, stated on the bundle region): tubes, fittings and rack draw no power; the reinstalled bundle is fed from a boiler elsewhere at 70/55 degC (DIN EN 12828).", "Alle Regionen sind passiv (0 W, an der Bündelregion angegeben): Rohre, Fittings und Regal nehmen keine Leistung auf; das wieder eingebaute Bündel wird von einem Kessel anderswo mit 70/55 °C gespeist (DIN EN 12828)."),
        t("The tubes are modelled as closed rods: their bores are 6 m long cavities of 27 to 53 mm whose exchange with the room is negligible, so the region area is the outer tube surface.", "Die Rohre sind als geschlossene Stäbe modelliert: ihre Bohrungen sind 6 m lange Hohlräume von 27 bis 53 mm, deren Austausch mit dem Raum vernachlässigbar ist, daher ist die Regionsfläche die äußere Rohroberfläche."),
        t("Weathered surfaces: black tube with mill scale and light rust has an emissivity of about 0.8 (bright new mill scale 0.6 to 0.7), the scuffed gentian-blue paint of the rack 0.9; the yard is covered, so no sun is bound.", "Bewitterte Oberflächen: schwarzes Rohr mit Walzhaut und Flugrost hat einen Emissionsgrad von etwa 0,8 (blanke neue Walzhaut 0,6 bis 0,7), der verschrammte enzianblaue Lack des Regals 0,9; das Lager ist überdacht, daher ist keine Sonne gebunden."),
        t("Reference case: covered yard and unheated plant room at 12 degC air; an uninsulated DN 50 tube at 62.5 degC mean water temperature gives off about 70 W per metre (Recknagel), about 2.9 kW for the seven 6 m lengths.", "Referenzfall: überdachtes Lager und unbeheizter Technikraum mit 12 °C Luft; ein ungedämmtes Rohr DN 50 mit 62,5 °C mittlerer Wassertemperatur gibt etwa 70 W je Meter ab (Recknagel), rund 2,9 kW für die sieben Längen von 6 m."),
    ],
}

STRUCTURE = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
    t("Supports per pose: stored, the six ground contacts under the front and rear ends of the three feet; raised and lifted, the same six plus the two fork tines under the bundle (ISO 2328 class 2A, 2 m apart); reinstalled, the six plus two trapeze hangers 4 m apart under the run. The asset is solved as one rigid body per pose, so the rack and the bundle share the contacts.", "Auflager je Pose: gelagert die sechs Bodenkontakte unter den vorderen und hinteren Enden der drei Füße; angehoben und ausgelagert dieselben sechs und die beiden Gabelzinken unter dem Bündel (ISO 2328 Klasse 2A, 2 m Abstand); eingebaut die sechs und zwei Trapezhänger im Abstand von 4 m unter dem Strang. Das Asset wird je Pose als ein starrer Körper gelöst, Regal und Bündel teilen sich daher die Kontakte."),
    t(f"Masses from section x length x density (EN 10255, EN 10365, EN 10056-1) and catalogue masses of the fittings: rack {M_RACK:.1f} kg, loose row {M_STOCK:.1f} kg, bundle {M_BUNDLE:.1f} kg; total {MASS} kg.", f"Massen aus Querschnitt x Länge x Dichte (EN 10255, EN 10365, EN 10056-1) und Katalogmassen der Fittings: Regal {dez(f'{M_RACK:.1f}')} kg, lose Lage {dez(f'{M_STOCK:.1f}')} kg, Bündel {dez(f'{M_BUNDLE:.1f}')} kg; zusammen {dez(MASS)} kg."),
    t(f"Arm load stored: the bundle of {M_BUNDLE:.0f} kg on three lower arms, about 70 kg per arm, far below the 300 kg of a light cantilever arm (FEM 10.2.09). Reinstalled, the seven tubes carry {WATER_KG:.0f} kg of heating water, drawn as one load at the centre of the bundle.", f"Armlast gelagert: das Bündel von {M_BUNDLE:.0f} kg auf drei unteren Kragarmen, etwa 70 kg je Arm, weit unter den 300 kg eines leichten Kragarms (FEM 10.2.09). Eingebaut tragen die sieben Rohre {WATER_KG:.0f} kg Heizwasser, dargestellt als eine Last im Schwerpunkt des Bündels."),
], "clinical": False}


def build():
    """🏁 Assemble the asset document."""
    reps, vols = {}, {}
    for s in (50, 100, 200, 500):
        parts, body, geo = bodies_and_shapes(s)
        reps[str(s)] = representation(s, geo)
        vols[str(s)] = volume(s, parts, body)
    asset = {
        "id": SLUG,
        "category": "components",
        "title": t("Used Threaded Steel Pipe Rack", "Gebrauchte Gewinderohre im Rohrgestell"),
        "summary": t(f"Used black medium-series threaded steel tubes EN 10255 in 6 m lengths in a painted cantilever rack of three IPE 160 columns with IPE 100 arms: a strapped bundle of seven DN 50 tubes on the lower arms and a loose row of DN 40, DN 32 and DN 25 tubes with cut ends, thread runs and a socket, an elbow and a nipple left on, on the upper arms; {MASS:.0f} kg in all. The bundle is lifted out on forks and reinstalled as the mains of a plant-room run heating at 70/55 degC.", f"Gebrauchte schwarze mittelschwere Gewinderohre EN 10255 in Längen von 6 m in einem lackierten Kragarmregal aus drei Stützen IPE 160 mit Kragarmen IPE 100: ein umreiftes Bündel aus sieben Rohren DN 50 auf den unteren Armen und eine lose Lage aus Rohren DN 40, DN 32 und DN 25 mit Schnittenden, Gewindeausläufen und einer Muffe, einem Winkel und einem Nippel auf den oberen Armen; zusammen {dez(f'{MASS:.0f}')} kg. Das Bündel wird mit der Gabel ausgelagert und als Hauptleitungen eines Technikraumstrangs mit 70/55 °C heizend wieder eingebaut."),
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "plumbing", "steel", "pipe", "threaded-pipe", "black-steel", "cantilever-rack", "storage", "rolled-section"],
        "provenance": {"author": "Semio staffage component authors (used HVAC wave, author 17)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
            f"Tube sizes and masses from EN 10255 series M (DIN 2440 successor); rack sections from EN 10365 and EN 10056-1; rack layout (three columns 2.1 m apart, arms 800 mm at 550 and 1250 mm) typical of a light single-sided cantilever rack (authored-estimate). Overall {WIDTH} x {HEIGHT} x {DEPTH} m, {MASS} kg. Nothing is measured.",
            f"Rohrmaße und Massen nach EN 10255 Reihe M (Nachfolger der DIN 2440); Regalprofile nach EN 10365 und EN 10056-1; Regalaufbau (drei Stützen im Abstand von 2,1 m, Arme 800 mm in 550 und 1250 mm Höhe) typisch für ein leichtes einseitiges Kragarmregal (authored-estimate). Insgesamt {dez(WIDTH)} x {dez(HEIGHT)} x {dez(DEPTH)} m, {dez(MASS)} kg. Nichts ist gemessen.")},
        "dimensions": {"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH, "massKg": MASS},
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "yard", "bones": BONES},
        "anchors": ANCHORS,
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": THERMAL,
        "structure": STRUCTURE,
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(build(), ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"[DEBUG] wrote {SLUG}: mass {MASS} kg (rack {M_RACK:.2f}, stock {M_STOCK:.2f}, bundle {M_BUNDLE:.2f}), water {WATER_KG:.1f} kg, h {HEIGHT} w {WIDTH} d {DEPTH}")
