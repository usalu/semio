"""🛁 Authoritative generator of the used chrome towel radiator: every hand-chosen number of the 500 x 1200 mm ladder radiator with D-section uprights, its fixings and its thermostatic valve body, leaning in a steel A-frame rack with EPDM edge strips, drawn in 2D and modelled in 3D per scale from one part list, written to 🔣️.json next to this file."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-towel-radiator"


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


LEAN = 10.0
TH = math.radians(LEAN)
TAN, COS, SIN = math.tan(TH), math.cos(TH), math.sin(TH)
YS = 0.20
APEX_Y = 1.26
SHS = 0.04
SHS_H = SHS / 2 / COS
PAD = 0.015
RAIL_Y = (0.78, 0.82)
UP_D = 0.030
UP_W = 0.035
UP_R = UP_W / 2
UP_BACK = -UP_D / 2
UP_Z = 0.2325
UP_Y = (-0.05, 1.14)
PLUG_TOP = 1.15
RAIL_R = 0.011
RAIL_PITCH = 0.055
RAIL_GROUPS = (0.011, 0.416, 0.821)
RAILS = [g + k * RAIL_PITCH for g in RAIL_GROUPS for k in range(6)]
BRACKET_Y = (0.15, 1.0135)
BRACKET_X = (-0.065, -0.055, -0.020)
VALVE_Z = UP_Z
SILL_X = 0.03
SILL_Z = 0.18
STRIP = 0.015
FOOT_X = 0.45
FRAME_Z = 0.40
INNER_Z = FRAME_Z - SHS / 2


def leg_centre(y):
    """📐 The x of the centre line of the front A-frame leg at height y (apex over the origin at 1.46 m)."""
    return (APEX_Y - y) * TAN


XP = (leg_centre(RAIL_Y[1]) + SHS_H + PAD) + (RAIL_Y[1] - YS) * TAN + (-UP_BACK) / COS


def back_plane(y):
    """📏 The x of the flat back face of the leaning uprights at height y in the stored pose."""
    return XP - (y - YS) * TAN - (-UP_BACK) / COS


ORIGIN = {"yard": (0.0, 0.0), "rack": (0.0, 0.0), "carrier": (XP, YS), "radiator": (XP, YS), "rails": (XP, YS)}

SHS_KGM = 2.31
LEG_L = (APEX_Y - SHS) / COS
RACK_TUBE_M = 4 * LEG_L + (2 * FRAME_Z + SHS) + 2 * (2 * FOOT_X) + 2 * (2 * INNER_Z) + 2 * (2 * INNER_Z)
M_RACK_STEEL = SHS_KGM * RACK_TUBE_M
M_SILLS = 2 * (2 * SILL_X) * (YS - STRIP - SHS) * (2 * SILL_Z) * 450.0
PAD_AREA = 0.5 * ((PAD + 0.0) + (PAD + (RAIL_Y[1] - RAIL_Y[0]) * TAN)) * (RAIL_Y[1] - RAIL_Y[0])
M_STRIPS = 2 * PAD_AREA * 2 * INNER_Z * 1150.0 + 2 * (2 * SILL_X) * STRIP * (2 * SILL_Z) * 1150.0
M_RACK = M_RACK_STEEL + M_SILLS + M_STRIPS

UP_AREA = UP_W * (UP_D - UP_R) + math.pi * UP_R ** 2 / 2
UP_PERIM = UP_W + 2 * (UP_D - UP_R) + math.pi * UP_R
UP_KGM = UP_PERIM * 0.0012 * 7850.0
RAIL_KGM = math.pi * (2 * RAIL_R - 0.0012) * 0.0012 * 7850.0
RAIL_LEN = 2 * (UP_Z - UP_R) + 0.010
M_UPRIGHTS = 2 * UP_KGM * (UP_Y[1] - UP_Y[0])
M_RAILS = len(RAILS) * RAIL_KGM * RAIL_LEN
RADIATOR_PARTS = {
    "uprights": (M_UPRIGHTS, (0.0, (UP_Y[0] + UP_Y[1]) / 2)),
    "plugs": (0.12, (0.0, 0.62)),
    "valve": (0.34, (-0.02, -0.09)),
    "brackets": (4 * 0.11, (-0.045, (BRACKET_Y[0] + BRACKET_Y[1]) / 2)),
}
M_RAD_BODY = sum(m for m, _ in RADIATOR_PARTS.values())
COM_RAD = (sum(m * c[0] for m, c in RADIATOR_PARTS.values()) / M_RAD_BODY, sum(m * c[1] for m, c in RADIATOR_PARTS.values()) / M_RAD_BODY)
COM_RAILS = (0.0, sum(RAILS) / len(RAILS))
M_RADIATOR = M_RAD_BODY + M_RAILS
M_TOTAL = M_RACK + M_RADIATOR
WATER_L = (len(RAILS) * math.pi * (RAIL_R - 0.0012) ** 2 * RAIL_LEN + 2 * (UP_AREA - UP_PERIM * 0.0012) * (UP_Y[1] - UP_Y[0])) * 1000.0

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}
RAILS_OF = {50: (RAILS, RAIL_R), 100: ([g + k * 0.1375 for g in RAIL_GROUPS for k in range(3)], RAIL_R), 200: ([0.015 + k * (1.077 / 4) for k in range(5)], 0.015)}
LEANING = ("radiator", "rails")


def lean(x, y):
    """↗ A point of the upright radiator (world coordinates with the uprights vertical) carried to the leaning rest: turned by the lean about the sill bearing."""
    lx, ly = x - XP, y - YS
    return XP + lx * COS - ly * SIN, YS + lx * SIN + ly * COS


def hull(points):
    """🪢 The convex hull of 2D points, counter-clockwise (monotone chain)."""
    pts = sorted(set((round(a, 7), round(b, 7)) for a, b in points))
    if len(pts) < 3:
        return pts

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for q in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], q) <= 0:
            lower.pop()
        lower.append(q)
    for q in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], q) <= 0:
            upper.pop()
        upper.append(q)
    return lower[:-1] + upper[:-1]


def ring(n=16):
    """⭕ Unit circle samples."""
    return [(math.cos(2 * math.pi * k / n), math.sin(2 * math.pi * k / n)) for k in range(n)]


class Part:
    """🧱 One physical part of one scale: its 3D solid in the bone frame and its three 2D projections from the same numbers (world rest coordinates)."""

    def __init__(self, pid, bone, kind, geo, material, tone, region, views=("elevation", "front", "plan"), role="body", extra=None, struct=None):
        self.id, self.bone, self.kind, self.geo, self.material, self.tone, self.region = pid, bone, kind, geo, material, tone, region
        self.views, self.role, self.extra, self.struct = views, role, extra or {}, struct

    def solid(self):
        """🧊 The volume solid of the part in the rest frame of its bone."""
        ox, oy = ORIGIN[self.bone]
        g = self.geo
        base = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
            s = dict(base, kind="box", size=[r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], transform={"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2)]})
        elif self.kind == "cylx":
            x0, x1, cy, cz, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(x1 - x0), r], [r6(x1 - x0), 0]], axis=[1, 0, 0], origin=[r6(x0 - ox), r6(cy - oy), r6(cz)])
        elif self.kind == "cyly":
            cx, cz, y0, y1, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(y1 - y0), r], [r6(y1 - y0), 0]], axis=[0, 1, 0], origin=[r6(cx - ox), r6(y0 - oy), r6(cz)])
        elif self.kind == "cylz":
            cx, cy, z0, z1, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(z1 - z0), r], [r6(z1 - z0), 0]], axis=[0, 0, 1], origin=[r6(cx - ox), r6(cy - oy), r6(z0)])
        elif self.kind == "extxy":
            prof, z0, z1 = g
            s = dict(base, kind="extrude", plane="xy", profile=[[r6(x - ox), r6(y - oy)] for x, y in prof], **{"from": r6(z0), "to": r6(z1)})
        elif self.kind == "extzx":
            prof, y0, y1 = g
            s = dict(base, kind="extrude", plane="zx", profile=[[r6(z), r6(x - ox)] for x, z in prof], **{"from": r6(y0 - oy), "to": r6(y1 - oy)})
        for key in ("slender", "sheet", "texture", "tags", "envelope"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s

    def bounds(self):
        """📦 World bounds (x0, y0, z0), (x1, y1, z1) of the part at rest."""
        g = self.geo
        if self.kind == "box":
            return g
        if self.kind == "cylx":
            x0, x1, cy, cz, r = g
            return (x0, cy - r, cz - r), (x1, cy + r, cz + r)
        if self.kind == "cyly":
            cx, cz, y0, y1, r = g
            return (cx - r, y0, cz - r), (cx + r, y1, cz + r)
        if self.kind == "cylz":
            cx, cy, z0, z1, r = g
            return (cx - r, cy - r, z0), (cx + r, cy + r, z1)
        if self.kind == "extxy":
            prof, z0, z1 = g
            return (min(p[0] for p in prof), min(p[1] for p in prof), z0), (max(p[0] for p in prof), max(p[1] for p in prof), z1)
        if self.kind == "extzx":
            prof, y0, y1 = g
            return (min(p[0] for p in prof), y0, min(p[1] for p in prof)), (max(p[0] for p in prof), y1, max(p[1] for p in prof))

    def cloud(self):
        """☁ Points on the surface of the part (corners, end circles, profile rings) in the upright world frame, whose convex hull is its projection."""
        g = self.geo
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
            return [(x, y, z) for x in (x0, x1) for y in (y0, y1) for z in (z0, z1)]
        if self.kind == "cylx":
            x0, x1, cy, cz, r = g
            return [(x, cy + r * a, cz + r * b) for x in (x0, x1) for a, b in ring()]
        if self.kind == "cyly":
            cx, cz, y0, y1, r = g
            return [(cx + r * a, y, cz + r * b) for y in (y0, y1) for a, b in ring()]
        if self.kind == "cylz":
            cx, cy, z0, z1, r = g
            return [(cx + r * a, cy + r * b, z) for z in (z0, z1) for a, b in ring()]
        if self.kind == "extzx":
            prof, y0, y1 = g
            return [(x, y, z) for y in (y0, y1) for x, z in prof]
        prof, z0, z1 = g
        return [(x, y, z) for z in (z0, z1) for x, y in prof]

    def shape(self, view):
        """✏ The filled 2D shape of the part in one viewpoint: elevation in the bone frame, front and plan in the world rest frame."""
        ox, oy = ORIGIN[self.bone]
        (x0, y0, z0), (x1, y1, z1) = self.bounds()
        depth = {"elevation": 5 - z0, "front": 5 + x1, "plan": 5 + y1}[view]
        s = {"id": self.id, "bone": self.bone, "role": self.role, "z": round(depth, 4), "filled": True, "tone": self.tone, "material": self.material}
        if isinstance(self.region, list):
            s["regions"] = self.region
        else:
            s["region"] = self.region
        g = self.geo

        def rect(x, y, w, h):
            return dict(s, kind="rect", x=r6(x), y=r6(y), w=r6(w), h=r6(h))

        def circle(cx, cy, r):
            return dict(s, kind="ellipse", cx=r6(cx), cy=r6(cy), rx=r6(r), ry=r6(r))

        if self.bone in LEANING and view != "elevation":
            cloud = []
            for x, y, z in self.cloud():
                xw, yw = lean(x, y)
                cloud.append((xw, yw, z))
            s["z"] = round(5 + max(c[0] for c in cloud), 4) if view == "front" else round(5 + max(c[1] for c in cloud), 4)
            pts = hull([(c[2], c[1]) for c in cloud] if view == "front" else [(c[0], c[2]) for c in cloud])
            return dict(s, kind="poly", closed=True, points=[[r6(a), r6(b)] for a, b in pts])
        if view == "elevation":
            if self.kind == "extxy":
                return dict(s, kind="poly", closed=True, points=[[r6(x - ox), r6(y - oy)] for x, y in g[0]])
            if self.kind == "cylz":
                return circle(g[0] - ox, g[1] - oy, g[4])
            return rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)
        if view == "front":
            if self.kind == "cylx":
                return circle(g[3], g[2], g[4])
            return rect(z0, y0, z1 - z0, y1 - y0)
        if self.kind == "cyly":
            return circle(g[0], g[1], g[4])
        if self.kind == "extzx":
            return dict(s, kind="poly", closed=True, points=[[r6(x), r6(z)] for x, z in g[0]])
        return rect(x0, z0, x1 - x0, z1 - z0)


def d_profile(zc, n=10):
    """🅳 The D section of an upright 35 mm wide and 30 mm deep around its centre zc: flat back to the wall (-x), half-round front; points as (x, z)."""
    xc = UP_BACK + (UP_D - UP_R)
    pts = [(UP_BACK, zc - UP_R), (xc, zc - UP_R)]
    for k in range(1, n):
        a = -math.pi / 2 + math.pi * k / n
        pts.append((xc + UP_R * math.cos(a), zc + UP_R * math.sin(a)))
    pts += [(xc, zc + UP_R), (UP_BACK, zc + UP_R)]
    return [(XP + x, z) for x, z in pts]


def leg_profile(side, y0=SHS, y1=APEX_Y):
    """🔺 The inclined SHS 40 leg of an A-frame end frame (side +1 front face, -1 back face) as a parallelogram in the x-y plane."""
    pts = [(leg_centre(y0) - SHS_H, y0), (leg_centre(y0) + SHS_H, y0), (leg_centre(y1) + SHS_H, y1), (leg_centre(y1) - SHS_H, y1)]
    return [(side * x, y) for x, y in pts] if side > 0 else [(side * x, y) for x, y in pts][::-1]


def rail_front():
    """📍 The x of the front face of the upper SHS rail of the front face (flush with the leg front at the rail top)."""
    return leg_centre(RAIL_Y[1]) + SHS_H


def pad_profile(side, merged=False):
    """🩹 The EPDM edge strip on the upper rail, its front face in the plane of the leaning uprights (side +1 front, -1 back); merged adds the rail behind it."""
    xf = rail_front()
    x_back = xf - SHS if merged else xf
    pts = [(x_back, RAIL_Y[0]), (back_plane(RAIL_Y[0]), RAIL_Y[0]), (back_plane(RAIL_Y[1]), RAIL_Y[1]), (x_back, RAIL_Y[1])]
    return [(side * x, y) for x, y in pts] if side > 0 else [(side * x, y) for x, y in pts][::-1]


def parts_of(scale):
    """🗂 The hand-chosen part list of one scale: every part the radiator and its rack have that this scale can show, generalised for its smallest feature."""
    P = []
    f = FEATURE[scale]
    xf = rail_front()
    for side, name in ((-1, "right"), (1, "left")):
        zc = side * FRAME_Z
        ev = ("elevation", "front", "plan") if name == "right" else ("front", "plan")
        P.append(Part(f"foot-{name}", "rack", "box", ((-FOOT_X, 0.0, zc - SHS / 2), (FOOT_X, SHS, zc + SHS / 2)), "painted-steel", 0.45, "rack", views=ev, struct="rack"))
        for face, fname in ((1, "front"), (-1, "back")):
            P.append(Part(f"leg-{fname}-{name}", "rack", "extxy", (leg_profile(face), zc - SHS / 2, zc + SHS / 2), "painted-steel", 0.45, "rack", views=ev, struct="rack"))
    P.append(Part("ridge", "rack", "box", ((-SHS / 2, APEX_Y, -FRAME_Z - SHS / 2), (SHS / 2, APEX_Y + SHS, FRAME_Z + SHS / 2)), "painted-steel", 0.45, "rack", struct="rack"))
    for face, fname in ((1, "front"), (-1, "back")):
        xc = face * XP
        P.append(Part(f"cross-tube-{fname}", "rack", "box", ((xc - SHS / 2, 0.0, -INNER_Z), (xc + SHS / 2, SHS, INNER_Z)), "painted-steel", 0.45, "rack", views=("front", "plan"), struct="rack"))
        lo, hi = sorted((face * (xf - SHS), face * xf))
        if scale == 200:
            P.append(Part(f"rail-{fname}", "rack", "extxy", (pad_profile(face, True), -INNER_Z, INNER_Z), "epdm", 0.15, ["edge-strips", "rack"], struct="rack"))
            P.append(Part(f"sill-{fname}", "rack", "box", ((xc - SILL_X, SHS, -SILL_Z), (xc + SILL_X, YS, SILL_Z)), "softwood", 0.62, ["sills", "edge-strips"], struct="rack"))
        else:
            P.append(Part(f"rail-{fname}", "rack", "box", ((lo, RAIL_Y[0], -INNER_Z), (hi, RAIL_Y[1], INNER_Z)), "painted-steel", 0.45, "rack", struct="rack"))
            P.append(Part(f"edge-strip-{fname}", "rack", "extxy", (pad_profile(face), -INNER_Z, INNER_Z), "epdm", 0.15, "edge-strips", struct="rack"))
            P.append(Part(f"sill-{fname}", "rack", "box", ((xc - SILL_X, SHS, -SILL_Z), (xc + SILL_X, YS - STRIP, SILL_Z)), "softwood", 0.62, "sills", struct="rack"))
            P.append(Part(f"sill-strip-{fname}", "rack", "box", ((xc - SILL_X, YS - STRIP, -SILL_Z), (xc + SILL_X, YS, SILL_Z)), "epdm", 0.15, "edge-strips", struct="rack"))
    y_top = PLUG_TOP if scale != 50 else UP_Y[1]
    for side, name, region in ((-1, "right", "upright-return"), (1, "left", "upright-flow")):
        zc = side * UP_Z
        P.append(Part(f"upright-{name}", "radiator", "extzx", (d_profile(zc), YS + UP_Y[0], YS + y_top), "chrome-steel", 0.86, region,
                      extra={"slender": True}, struct="radiator"))
    rails, rr = RAILS_OF[scale]
    for k, yc in enumerate(rails):
        extra = {"slender": True} if scale == 50 else ({"slender": True, "envelope": True} if scale == 100 else {"envelope": True})
        P.append(Part(f"rail-{k:02d}", "rails", "cylz", (XP, YS + yc, -UP_Z, UP_Z, rr), "chrome-steel", 0.82, "rails", extra=extra, struct="rails"))
    bx0, bpin, bcol = BRACKET_X
    for j, by in enumerate(BRACKET_Y):
        for side, name in ((-1, "right"), (1, "left")):
            zc = side * UP_Z
            ev = ("elevation", "front", "plan") if name == "right" else ("front", "plan")
            pid = f"bracket-{'lower' if j == 0 else 'upper'}-{name}"
            if scale == 50:
                P.append(Part(f"{pid}-collar", "radiator", "box", ((XP - 0.020, YS + by - 0.010, zc - 0.0225), (XP + 0.020, YS + by + 0.010, zc + 0.0225)), "chrome-steel", 0.78, "brackets", views=ev))
                P.append(Part(f"{pid}-pin", "radiator", "cylx", (XP + bpin, XP + bcol, YS + by, zc, 0.008), "chrome-steel", 0.7, "brackets", views=ev))
                P.append(Part(f"{pid}-rosette", "radiator", "cylx", (XP + bx0, XP + bpin, YS + by, zc, 0.025), "chrome-steel", 0.74, "brackets", views=ev))
            else:
                hy = 0.0125 if scale == 100 else 0.015
                P.append(Part(pid, "radiator", "box", ((XP + bx0, YS + by - hy, zc - 0.0225), (XP + UP_BACK, YS + by + hy, zc + 0.0225)), "chrome-steel", 0.74, "brackets", views=ev))
    vz = VALVE_Z
    if scale == 50:
        P.append(Part("plug-top-left", "radiator", "cyly", (XP, UP_Z, YS + UP_Y[1], YS + PLUG_TOP, 0.012), "chrome-brass", 0.8, "upright-flow", views=("front", "plan")))
        P.append(Part("plug-top-right", "radiator", "cyly", (XP, -UP_Z, YS + UP_Y[1], YS + PLUG_TOP, 0.012), "chrome-brass", 0.8, "upright-return"))
        P.append(Part("blanking-plug", "radiator", "cyly", (XP, -UP_Z, YS - 0.062, YS + UP_Y[0], 0.015), "chrome-brass", 0.8, "upright-return"))
        P.append(Part("valve-nut", "radiator", "cyly", (XP, vz, YS - 0.065, YS + UP_Y[0], 0.016), "chrome-brass", 0.8, "valve"))
        P.append(Part("valve-body", "radiator", "cyly", (XP, vz, YS - 0.115, YS - 0.065, 0.013), "chrome-brass", 0.76, "valve"))
        P.append(Part("valve-outlet", "radiator", "cylx", (XP - 0.055, XP - 0.012, YS - 0.10, vz, 0.013), "chrome-brass", 0.78, "valve"))
        P.append(Part("copper-stub", "radiator", "cylx", (XP - 0.085, XP - 0.055, YS - 0.10, vz, 0.0075), "copper", 0.5, "valve"))
        P.append(Part("valve-bonnet", "radiator", "cylz", (XP, YS - 0.10, vz + 0.012, vz + 0.033, 0.012), "chrome-brass", 0.8, "valve", views=("front", "plan")))
        P.append(Part("protection-cap", "radiator", "cylz", (XP, YS - 0.10, vz + 0.033, vz + 0.048, 0.015), "abs-plastic", 0.95, "valve", views=("front", "plan")))
    elif scale == 100:
        P.append(Part("valve-body", "radiator", "cyly", (XP, vz, YS - 0.115, YS + UP_Y[0], 0.016), "chrome-brass", 0.78, "valve"))
        P.append(Part("valve-outlet", "radiator", "cylx", (XP - 0.085, XP - 0.012, YS - 0.10, vz, 0.011), "chrome-brass", 0.76, "valve"))
        P.append(Part("valve-bonnet", "radiator", "cylz", (XP, YS - 0.10, vz + 0.012, vz + 0.048, 0.015), "chrome-brass", 0.85, "valve", views=("front", "plan")))
    else:
        P.append(Part("valve-body", "radiator", "box", ((XP - 0.045, YS - 0.115, vz - 0.015), (XP + 0.015, YS + UP_Y[0], vz + 0.048)), "chrome-brass", 0.78, "valve"))
    return P


def details_of(scale):
    """🖋 The detail and texture strokes of one scale: chrome highlights and dull wear spots, the cut copper end, welds, EPDM ribs and the sill grain."""
    e, f, p = [], [], []
    if scale == 200:
        return e, f, p

    def line(lst, sid, bone, pts, role="detail", region=None, tone=0.3, z=9.0, closed=False, filled=False):
        s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}
        if region:
            s["region"] = region
        lst.append(s)

    def spot(lst, sid, bone, cx, cy, rx, ry, region, z, front=False):
        pts = [(cx + rx * math.cos(2 * math.pi * k / 8), cy + ry * math.sin(2 * math.pi * k / 8)) for k in range(8)]
        if front:
            pts = [(a, lean(XP + 0.011, YS + b)[1]) for a, b in pts]
        line(lst, sid, bone, pts, role="texture", region=region, tone=0.55, z=z, closed=True, filled=True)

    line(e, "upright-highlight", "radiator", [(0.006, UP_Y[0] + 0.02), (0.006, UP_Y[1] - 0.02)], role="texture" if scale == 50 else "detail", region="upright-return", tone=0.98, z=8.0)
    def fr(zz, yl, xl=0.0):
        return (zz, lean(XP + xl, YS + yl)[1])

    line(f, "upright-highlight-right", "radiator", [fr(-UP_Z + 0.004, UP_Y[0] + 0.02, 0.015), fr(-UP_Z + 0.004, UP_Y[1] - 0.02, 0.015)], role="texture" if scale == 50 else "detail", region="upright-return", tone=0.98, z=9.6)
    line(f, "upright-highlight-left", "radiator", [fr(UP_Z - 0.004, UP_Y[0] + 0.02, 0.015), fr(UP_Z - 0.004, UP_Y[1] - 0.02, 0.015)], role="texture" if scale == 50 else "detail", region="upright-flow", tone=0.98, z=9.6)
    if scale == 50:
        for k, (yy, h) in enumerate(((0.24, 0.05), (0.43, 0.03), (0.71, 0.04))):
            spot(e, f"wear-spot-elevation-{k}", "radiator", 0.008, yy, 0.004, h / 2, "upright-return", 8.2)
        for k, (zz, yy) in enumerate(((-0.12, 0.066), (0.05, 0.121), (0.14, 0.471), (-0.06, 0.526), (0.09, 0.876), (-0.15, 0.931))):
            spot(f, f"wear-spot-rail-{k}", "rails", zz, yy, 0.022, 0.0085, "rails", 9.7, True)
        spot(f, "wear-spot-upright-right", "radiator", -UP_Z, 0.36, 0.009, 0.03, "upright-return", 9.7, True)
        spot(f, "wear-spot-upright-left", "radiator", UP_Z, 0.64, 0.009, 0.022, "upright-flow", 9.7, True)
        line(e, "copper-cut-face", "radiator", [(-0.085, -0.1075), (-0.085, -0.0925)], region="valve", tone=0.2, z=8.3)
        line(e, "nut-flats", "radiator", [(-0.016, -0.0575), (0.016, -0.0575)], region="valve", tone=0.4, z=8.3)
        for k in range(3):
            line(e, f"edge-strip-rib-{k}", "rack", [(back_plane(RAIL_Y[0] + 0.01 * (k + 1)) - 0.004, RAIL_Y[0] + 0.01 * (k + 1)), (rail_front() + 0.002, RAIL_Y[0] + 0.01 * (k + 1))], role="texture", region="edge-strips", tone=0.3, z=4.85)
        line(e, "sill-grain", "rack", [(XP - 0.02, SHS + 0.02), (XP - 0.018, YS - STRIP - 0.02)], role="texture", region="sills", tone=0.45, z=4.85)
        line(e, "foot-weld-front", "rack", [(leg_centre(SHS) - 0.026, SHS), (leg_centre(SHS) + 0.026, SHS)], region="rack", tone=0.3, z=4.7)
        line(e, "foot-weld-back", "rack", [(-leg_centre(SHS) - 0.026, SHS), (-leg_centre(SHS) + 0.026, SHS)], region="rack", tone=0.3, z=4.7)
        line(f, "sill-grain-front", "rack", [(-SILL_Z + 0.02, SHS + 0.06), (SILL_Z - 0.02, SHS + 0.064)], role="texture", region="sills", tone=0.45, z=5.2)
        line(f, "rosette-screw-holes", "radiator", [fr(-UP_Z - 0.03, BRACKET_Y[0], -0.06), fr(-UP_Z - 0.025, BRACKET_Y[0], -0.06)], role="texture", region="brackets", tone=0.2, z=5.25)
        line(p, "ridge-seam", "rack", [(0.0, -FRAME_Z), (0.0, FRAME_Z)], region="rack", tone=0.3, z=6.52)
    line(f, "rail-weld-line", "rack", [(-INNER_Z, RAIL_Y[1] - 0.004), (INNER_Z, RAIL_Y[1] - 0.004)], region="rack", tone=0.3, z=5.1)
    return e, f, p


exec(open(os.path.join(HERE, "author-towel-radiator-texts.py"), encoding="utf8").read())


def representation(scale):
    """🖨 The authored representation of one scale: the projected part shapes, the detail strokes, pens, hatches and decisions."""
    geo = {"elevation": [], "front": [], "plan": []}
    for part in parts_of(scale):
        for view in part.views:
            sh = part.shape(view)
            if scale == 50 and view == "elevation" and part.id.startswith("sill-front"):
                sh["hatch"] = "end-grain"
            geo[view].append(sh)
    e, f, p = details_of(scale)
    geo["elevation"] += e
    geo["front"] += f
    geo["plan"] += p
    pens = {50: {"body": 0.25, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    hatches = [{"id": "end-grain", "title": T("Sawn end grain of the softwood sill block", "Gesägtes Hirnholz des Nadelholz-Auflageklotzes"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}] if scale == 50 else []
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geo, "decisions": DECISIONS_2D[scale]}


def thermal_list(scale, parts):
    """🌡 The thermal list of one scale: every body solid by reference under its region and the plume patch over the top rail."""
    out = []
    for part in parts:
        region = part.region if isinstance(part.region, str) else part.region[0]
        out.append({"id": f"th-{part.id}", "of": part.id, "region": region})
    pt = max(FEATURE[scale], 0.004)
    rails, rr = RAILS_OF[scale]
    out.append({"id": "th-radiator-plume", "kind": "box", "bone": "rails", "material": "chrome-steel", "role": "detail", "tone": 0.82, "size": [max(2 * rr, 0.022), pt, r6(2 * (UP_Z - UP_R))],
                "transform": {"translation": [0.0, r6(rails[-1] + rr + pt / 2), 0.0]}, "region": "rails", "tags": ["radiator-plume"]})
    return out


def structure_list(scale, parts):
    """🏗 The structural idealisation of one scale: uprights, rails and rack tubes as members, the sill bearing and the two lean contacts as joints, and the centres of mass."""
    out = []
    for part in parts:
        if part.struct:
            out.append({"id": f"member-{part.id}", "of": part.id, "member": part.struct, "part": "member"})
    f = FEATURE[scale]
    out.append({"id": "joint-sill-bearing", "kind": "box", "bone": "radiator", "material": "chrome-steel", "role": "detail", "tone": 0.5, "size": [max(0.02, 2 * f), max(0.01, f), 0.30],
                "transform": {"translation": [0.0, 0.0, 0.0]}, "member": "radiator", "part": "joint"})
    yl = (0.5 * (RAIL_Y[0] + RAIL_Y[1]) - YS - UP_BACK * SIN) / COS
    for side, name in ((-1, "right"), (1, "left")):
        out.append({"id": f"joint-lean-{name}", "kind": "box", "bone": "radiator", "material": "chrome-steel", "role": "detail", "tone": 0.5, "size": [max(0.01, f), 0.04, UP_W],
                    "transform": {"translation": [r6(UP_BACK), r6(yl), r6(side * UP_Z)]}, "member": "radiator", "part": "joint"})
    rails, rr = RAILS_OF[scale]
    mr = max(rr, f / 2)
    near = lambda y: min(rails, key=lambda q: abs(q - y))
    y_rad = near(COM_RAD[1])
    y_rails = near(COM_RAILS[1]) if near(COM_RAILS[1]) != y_rad else sorted(rails, key=lambda q: abs(q - COM_RAILS[1]))[1]
    out.append({"id": "mass-radiator", "kind": "ellipsoid", "bone": "radiator", "material": "chrome-steel", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [0.0, r6(y_rad), 0]}, "member": "radiator", "part": "mass"})
    out.append({"id": "mass-rails", "kind": "ellipsoid", "bone": "rails", "material": "chrome-steel", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [0.0, r6(y_rails), 0]}, "member": "rails", "part": "mass"})
    rm = max(0.02, f / 2)
    out.append({"id": "mass-rack", "kind": "ellipsoid", "bone": "rack", "material": "painted-steel", "role": "detail", "tone": 0.5, "radii": [rm, rm, rm],
                "transform": {"translation": [0.0, rm, 0.0]}, "member": "rack", "part": "mass"})
    return out


def volume(scale):
    """🧊 The authored volume of one scale: body from the part list, thermal and structure lists, and the decisions."""
    parts = parts_of(scale)
    body = [p.solid() for p in parts]
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": DECISIONS_3D[scale]}


def binding(regions, wall=False):
    """🌬 The region binding of a pose: everything indoors and out of the sun; the rack feet on the floor, the rosettes on the bathroom wall when reinstalled."""
    out = {}
    for r in regions:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "rack":
            b = {"exposedFraction": 0.97, "sunlitFraction": 0, "contact": {"fraction": 0.03, "tempC": 20}}
        if r == "brackets" and wall:
            b = {"exposedFraction": 0.8, "sunlitFraction": 0, "contact": {"fraction": 0.2, "tempC": 20}}
        out[r] = b
    return out


def document():
    """📄 The whole asset document."""
    regions = [r["id"] for r in THERMAL["regions"]]
    rig = {"kind": "mechanical", "root": "yard", "bones": [
        {"id": "yard", "parent": None, "title": T("Store floor frame (massless, fixed root)", "Bezug Lagerboden (masselos, feste Wurzel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}},
        {"id": "rack", "parent": "yard", "title": T("Steel A-frame rack with sill blocks and EPDM edge strips (stays when the radiator is lifted)", "Stahl-A-Bock mit Auflageklötzen und EPDM-Kantenschutz (bleibt beim Anheben stehen)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.45, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.5, "maxM": 0},
         "mass": {"kg": r6(M_RACK), "comX": 0.0, "comY": 0.55, "distribution": "point", "source": "literature", "reference": "en-10219-2"},
         "section": {"areaM2": 0.000294, "secondMomentM4": 6.94e-8, "extremeFibreM": 0.02, "material": "painted-steel", "title": T("Square hollow section 40 x 40 x 2 mm", "Quadratrohr 40 x 40 x 2 mm")}},
        {"id": "carrier", "parent": "yard", "title": T("Lifting carrier at the sill bearing of the bottom rail (massless)", "Hubträger am Auflager der untersten Sprosse (masselos)"),
         "x": r6(XP), "y": YS, "angleDeg": 0, "lengthM": 0.015, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.5}},
        {"id": "radiator", "parent": "carrier", "title": T("Towel radiator uprights with valve body, plugs and wall brackets, tilting about the bottom rail on the sill", "Badheizkörper-Sammelrohre mit Ventilkörper, Stopfen und Wandkonsolen, um die unterste Sprosse auf dem Klotz kippend"),
         "x": 0, "y": 0, "angleDeg": LEAN, "lengthM": 0.015, "joint": {"type": "revolute", "minDeg": -12, "maxDeg": 0},
         "mass": {"kg": r6(M_RAD_BODY), "comX": r6(COM_RAD[0]), "comY": r6(COM_RAD[1]), "distribution": "point", "source": "authored-estimate", "reference": "towel-radiator-catalogues"}},
        {"id": "rails", "parent": "radiator", "title": T("Eighteen round towel rails 22 mm welded between the uprights", "Achtzehn runde Handtuchsprossen 22 mm, zwischen die Sammelrohre geschweißt"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.022, "joint": {"type": "fixed"},
         "mass": {"kg": r6(M_RAILS), "comX": 0.0, "comY": r6(COM_RAILS[1]), "distribution": "point", "source": "literature", "reference": "en-10305-3"}},
    ]}
    anchors = []
    for k, (x, z) in enumerate(((-0.42, -FRAME_Z), (-0.42, FRAME_Z), (0.42, -FRAME_Z), (0.42, FRAME_Z))):
        anchors.append({"id": f"rack-foot-{k}", "bone": "rack", "x": x, "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.06, "footprintWidthM": 0.04, "frictionCoefficient": 0.4,
                        "title": T(f"A-frame foot {'right' if z < 0 else 'left'} {'back' if x < 0 else 'front'} end on the store floor", f"A-Bock-Fuß {'rechts' if z < 0 else 'links'} {'hinten' if x < 0 else 'vorn'} auf dem Lagerboden")})
    anchors.append({"id": "sill-bearing", "bone": "radiator", "x": 0, "y": 0, "z": 0, "kind": "contact", "support": "contact", "footprintLengthM": 0.01, "footprintWidthM": 0.36, "frictionCoefficient": 0.6,
                    "title": T("Bottom rail standing on the EPDM strip of the sill block", "Unterste Sprosse auf dem EPDM-Streifen des Auflageklotzes")})
    for k, z in enumerate((-0.19, 0.19)):
        anchors.append({"id": f"sling-{k}", "bone": "rails", "x": 0, "y": r6(RAILS[-1] + RAIL_R), "z": z, "kind": "support", "support": "pin",
                        "title": T(f"Round sling choked round the top rail at the {'right' if z < 0 else 'left'} upright", f"Rundschlinge um die oberste Sprosse am {'rechten' if z < 0 else 'linken'} Sammelrohr")})
    for j, by in enumerate(BRACKET_Y):
        for side in (-1, 1):
            anchors.append({"id": f"wall-{'lower' if j == 0 else 'upper'}-{'right' if side < 0 else 'left'}", "bone": "radiator", "x": BRACKET_X[0], "y": by, "z": r6(side * UP_Z), "kind": "support", "support": "pin",
                            "title": T(f"{'Lower' if j == 0 else 'Upper'} {'right' if side < 0 else 'left'} bracket rosette screwed to the bathroom wall", f"{'Untere' if j == 0 else 'Obere'} {'rechte' if side < 0 else 'linke'} Konsolrosette an der Badwand verschraubt")})
    anchors.append({"id": "radiator-top", "bone": "rails", "x": 0, "y": r6(RAILS[-1] + RAIL_R), "z": 0, "kind": "emitter", "title": T("Middle of the top rail", "Mitte der obersten Sprosse")})
    anchors.append({"id": "towel-rails", "bone": "rails", "x": 0, "y": r6(RAILS[14]), "z": 0, "kind": "load", "title": T("Upper rail group where the towels hang", "Obere Sprossengruppe, an der die Handtücher hängen")})
    feet = [f"rack-foot-{k}" for k in range(4)]
    walls = ["wall-lower-right", "wall-lower-left", "wall-upper-right", "wall-upper-left"]
    service = [
        {"id": "radiator-water", "kind": "carried", "title": T(f"Heating water {WATER_L:.1f} l in rails and uprights", f"Heizwasser {WATER_L:.1f} l in Sprossen und Sammelrohren"), "bone": "rails", "x": 0, "y": r6(COM_RAILS[1]), "z": 0, "fxN": 0, "fyN": r6(-WATER_L * 9.81), "source": "authored-estimate", "reference": "towel-radiator-catalogues"},
        {"id": "wet-towels", "kind": "live", "title": T("Two wet bath towels, 3 kg, over the upper rails", "Zwei nasse Badetücher, 3 kg, über den oberen Sprossen"), "anchor": "towel-rails", "fxN": 0, "fyN": -29.43, "source": "authored-estimate"},
    ]
    off = {"heating": 0}
    zero_em = {"radiator-plume": 0}
    stored_desc = T(
        f"Drained and dry, the radiator stands with its bottom rail on the EPDM strip of the sill block of a steel A-frame and leans {LEAN:.0f} degrees back with the flat backs of its D-section uprights against the EPDM edge strip of the upper rail; valve body, plugs and the four brackets stay on it.",
        f"Entleert und trocken steht der Heizkörper mit seiner untersten Sprosse auf dem EPDM-Streifen des Auflageklotzes eines Stahl-A-Bocks und lehnt um {LEAN:.0f} Grad zurück mit den flachen Rückseiten seiner D-Sammelrohre am EPDM-Kantenschutz der oberen Traverse; Ventilkörper, Stopfen und die vier Konsolen bleiben daran.")
    poses = [
        {"id": "stored", "title": T("Stored leaning in the A-frame", "Im A-Bock angelehnt gelagert"), "description": stored_desc,
         "bones": {}, "ground": "authored", "contacts": feet, "thermal": {"operating": off, "regions": binding(regions), "emitters": zero_em}, "loads": []},
        {"id": "upright", "key": True, "title": T("Tilted upright on the sill", "Auf dem Klotz aufgerichtet"), "description": T(
            "In-between key of the lift: the radiator stands vertical on its bottom rail on the sill block, held by the round sling, clear of the edge strip.",
            "Zwischenschlüssel des Anhebens: der Heizkörper steht senkrecht mit seiner untersten Sprosse auf dem Auflageklotz, von der Rundschlinge gehalten, frei vom Kantenschutz."),
         "bones": {"radiator": {"angleDeg": -LEAN}}, "ground": "authored", "contacts": feet, "thermal": {"operating": off, "regions": binding(regions), "emitters": zero_em}, "loads": []},
        {"id": "lifted", "title": T("Lifted on a round sling", "An einer Rundschlinge angehoben"), "description": T(
            f"A textile round sling choked round the top rail at both uprights hangs the {M_RADIATOR:.1f} kg radiator vertical, 0.30 m above the sill block; the A-frame with its sills and edge strips stays on the floor.",
            f"Eine textile Rundschlinge, an beiden Sammelrohren um die oberste Sprosse geschnürt, hält den {M_RADIATOR:.1f} kg schweren Heizkörper senkrecht 0,30 m über dem Auflageklotz; der A-Bock mit Klötzen und Kantenschutz bleibt am Boden."),
         "bones": {"carrier": {"dy": 0.30}, "radiator": {"angleDeg": -LEAN}}, "ground": "authored", "contacts": ["sling-0", "sling-1"] + feet, "thermal": {"operating": off, "regions": binding(regions), "emitters": zero_em}, "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled, heating at the rated point", "Wieder eingebaut, Nennbetrieb"), "description": T(
            "Hung on a bathroom wall by its four bracket rosettes, the bottom of the uprights 150 mm above the floor, refilled and fed through the thermostatic valve body at 75/65 degC in a 20 degC room (EN 442 rating point); towels hang on the upper rails and a plume rises over the top rail. The emptied A-frame stands 1.2 m aside.",
            "An seinen vier Konsolrosetten an einer Badwand aufgehängt, Unterkante der Sammelrohre 150 mm über dem Boden, wieder befüllt und über den Thermostatventilkörper mit 75/65 °C in einem 20 °C warmen Raum versorgt (Normpunkt EN 442); Handtücher hängen an den oberen Sprossen, und über der obersten Sprosse steigt eine Fahne auf. Der geleerte A-Bock steht 1,2 m daneben."),
         "bones": {"rack": {"dx": -1.2}, "radiator": {"angleDeg": -LEAN}}, "ground": "authored", "contacts": walls + feet, "thermal": {"operating": {"heating": 1}, "regions": binding(regions, True), "emitters": {"radiator-plume": 1}}, "loads": service},
        {"id": "reinstalled-comfort", "title": T("Reinstalled, comfort setting", "Wieder eingebaut, Komfortstufe"), "description": T(
            "The same wall position with the valve throttled to the comfort level: 65/55 degC, dT 40 K.",
            "Dieselbe Wandlage mit auf die Komfortstufe gedrosseltem Ventil: 65/55 °C, dT 40 K."),
         "bones": {"rack": {"dx": -1.2}, "radiator": {"angleDeg": -LEAN}}, "ground": "authored", "contacts": walls + feet, "thermal": {"operating": {"heating": 0.748}, "regions": binding(regions, True), "emitters": {"radiator-plume": 0.748}}, "loads": service},
        {"id": "reinstalled-off", "title": T("Reinstalled, filled and cold", "Wieder eingebaut, befüllt und kalt"), "description": T(
            "The same wall position with the valve closed: water and towels load the brackets, everything is at the 20 degC of the room.",
            "Dieselbe Wandlage mit geschlossenem Ventil: Wasser und Handtücher belasten die Konsolen, alles hat die 20 °C des Raums."),
         "bones": {"rack": {"dx": -1.2}, "radiator": {"angleDeg": -LEAN}}, "ground": "authored", "contacts": walls + feet, "thermal": {"operating": off, "regions": binding(regions, True), "emitters": zero_em}, "loads": service},
    ]
    clips = [
        {"id": "lift", "title": T("Tilt up and lift out of the A-frame", "Aufrichten und aus dem A-Bock heben"), "description": T(
            "The sling is choked round the top rail; the radiator is tilted upright on its bottom rail in three seconds, then hoisted 0.30 m clear of the sill block in four seconds.",
            "Die Schlinge wird um die oberste Sprosse geschnürt; der Heizkörper wird in drei Sekunden auf seiner untersten Sprosse aufgerichtet und dann in vier Sekunden 0,30 m über den Auflageklotz gehoben."),
         "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "stored", "ease": "smooth"}, {"t": 5, "pose": "upright", "ease": "smooth"}, {"t": 9, "pose": "lifted", "ease": "smooth"}, {"t": 10, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 10} for a in feet] + [{"anchor": "sill-bearing", "fromS": 0, "toS": 5.2}, {"anchor": "sling-0", "fromS": 2, "toS": 10}, {"anchor": "sling-1", "fromS": 2, "toS": 10}],
         "phases": [{"id": "sling-on", "fromS": 0, "toS": 2, "title": T("Slinging", "Anschlagen")}, {"id": "tilt-up", "fromS": 2, "toS": 5, "title": T("Tilting upright", "Aufrichten")}, {"id": "hoist", "fromS": 5, "toS": 9, "title": T("Hoisting", "Heben")}, {"id": "hold", "fromS": 9, "toS": 10, "title": T("Holding", "Halten")}],
         "events": [{"t": 2.4, "kind": "contact-off", "title": T("Uprights leave the edge strip", "Sammelrohre lösen sich vom Kantenschutz"), "target": "radiator"}, {"t": 5.2, "kind": "contact-off", "title": T("Bottom rail leaves the sill block", "Unterste Sprosse verlässt den Auflageklotz"), "target": "radiator"}],
         "tracks": []},
        {"id": "heat-up", "title": T("Valve opens after reinstallation", "Ventil öffnet nach dem Wiedereinbau"), "description": T(
            "The refilled radiator starts cold; the valve opens after thirty seconds and the hot water climbs the flow upright, fills the rails and reaches the rated 75/65 degC after ten minutes, while the plume over the top rail builds up.",
            "Der wieder befüllte Heizkörper startet kalt; nach dreißig Sekunden öffnet das Ventil, das heiße Wasser steigt im Vorlauf-Sammelrohr auf, füllt die Sprossen und erreicht nach zehn Minuten die Nenntemperaturen 75/65 °C, während sich über der obersten Sprosse die Fahne aufbaut."),
         "durationS": 900, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 30, "pose": "reinstalled", "ease": "step"}, {"t": 900, "pose": "reinstalled", "ease": "step"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 900} for a in walls + feet],
         "phases": [{"id": "cold", "fromS": 0, "toS": 30, "title": T("Cold", "Kalt")}, {"id": "warm-up", "fromS": 30, "toS": 600, "title": T("Warming up", "Aufheizen")}, {"id": "rated", "fromS": 600, "toS": 900, "title": T("Rated temperatures", "Nenntemperaturen")}],
         "events": [{"t": 30, "kind": "operating", "title": T("Valve opens", "Ventil öffnet"), "target": "heating"}, {"t": 600, "kind": "operating", "title": T("Flow reaches 75 degC", "Vorlauf erreicht 75 °C"), "target": "heating"}],
         "tracks": [{"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 600, "v": 1}, {"t": 900, "v": 1}]},
                    {"target": "emitter:radiator-plume", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 600, "v": 1}, {"t": 900, "v": 1}]}]},
    ]
    unavailable = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T(
        "At 1:500 the radiator is 1.0 mm wide and 2.4 mm high on paper, under the 3 mm a recognisable drawing needs, its 22 mm rails and 35 mm uprights shrink to 0.04 and 0.07 mm and its 55 mm rail pitch to 0.11 mm, so the ladder cannot be told from a grey bar; draw it at 1:200 or larger.",
        "Bei 1:500 ist der Heizkörper auf Papier 1,0 mm breit und 2,4 mm hoch, unter den 3 mm, die eine erkennbare Zeichnung braucht, seine 22 mm Sprossen und 35 mm Sammelrohre schrumpfen auf 0,04 und 0,07 mm und der Sprossenabstand von 55 mm auf 0,11 mm, sodass die Leiter nicht von einem grauen Balken zu unterscheiden ist; bei 1:200 oder größer zeichnen."),
        "limitingFeature": T("Uprights 35 mm wide", "Sammelrohre 35 mm breit"), "limitingSizeM": 0.035, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}
    asset = {
        "id": SLUG, "category": "components", "title": T("Used Tubular Towel Radiator", "Gebrauchter Badheizkörper"), "summary": SUMMARY,
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "radiator", "towel-radiator", "chrome", "steel", "brass", "thermostatic-valve", "a-frame", "operating", "convection", "thermal"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": r6(max(APEX_Y + SHS, lean(XP + 0.012, YS + PLUG_TOP)[1], lean(XP + UP_D / 2, YS + UP_Y[1])[1])), "widthM": r6(2 * FOOT_X), "depthM": r6(2 * FRAME_Z + SHS), "massKg": r6(M_TOTAL)},
        "materials": MATERIALS, "rig": rig,
        "volumes": {"50": volume(50), "100": volume(100), "200": volume(200), "500": {"status": "unavailable", "denominator": 500}},
        "anchors": anchors,
        "representations": {"50": representation(50), "100": representation(100), "200": representation(200), "500": unavailable},
        "symbols": [], "poses": poses, "clips": clips, "thermal": THERMAL, "structure": STRUCTURE,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print(f"[DEBUG] xp {XP:.4f}; radiator {M_RADIATOR:.3f} kg (body {M_RAD_BODY:.3f}, rails {M_RAILS:.3f}) com {COM_RAD}; rack {M_RACK:.3f} kg (steel {M_RACK_STEEL:.3f}, sills {M_SILLS:.3f}, strips {M_STRIPS:.3f}); total {M_TOTAL:.3f} kg; water {WATER_L:.2f} l; tube {RACK_TUBE_M:.3f} m")
