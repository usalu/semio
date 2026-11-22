"""🧸 Hand-chosen numbers and the part model of the used beech baby cot: rig frames, one part list per scale, and the projection of every part into 3D solids and the three 2D views.

Every part is stated once in world rest coordinates (x ahead along the cot, y up, z to the cot's left, metres); the solids are written in the
rest frame of their bone and the 2D shapes are the exact orthographic outlines of the same numbers, so drawing and model agree by construction.
See `author-cot.py` for the document around it and `../../reports/used-baby-cot.md` for the reference table these numbers come from.
"""
import math

L, W, H = 1.29, 0.69, 0.88
POST = 0.04
POST_Y0 = 0.07
RAIL_T = 0.028
LONG_TOP = (0.815, 0.86)
END_TOP = (0.805, 0.86)
BOTTOM = (0.145, 0.18)
ZS = 0.325
XH, XF = 0.02, 1.27
BAR_R = 0.010
BAR_Y = (0.165, 0.83)
END_BAR_Y = (0.165, 0.82)
LONG_GAP = (1.21 - 15 * 2 * BAR_R) / 16
END_GAP = (0.61 - 7 * 2 * BAR_R) / 8
LONG_X = [round(POST + LONG_GAP + BAR_R + k * (LONG_GAP + 2 * BAR_R), 5) for k in range(15)]
END_Z = [round(-0.305 + END_GAP + BAR_R + k * (END_GAP + 2 * BAR_R), 5) for k in range(7)]
SLIP = (6, 7, 8)
SLIP_SHIFT = (round((LONG_GAP + 2 * BAR_R) / 2, 5), 0.02)
BASE_TOP = 0.47
BASE_MID, BASE_LOW = -0.115, -0.23
BASE_X = (0.0475, 1.2425)
BASE_RAIL_Z = (0.2755, 0.2975)
BASE_RAIL_Y = (BASE_TOP - 0.052, BASE_TOP - 0.012)
SLAT_Y = (BASE_TOP - 0.012, BASE_TOP)
SLAT_X = [round(0.0775 + k * (1.135 / 12), 4) for k in range(13)]
SLAT_W = 0.06
REPLACED_SLAT = 7
MAT_X = (0.045, 1.245)
MAT_Y = (BASE_TOP, BASE_TOP + 0.10)
MAT_Z = 0.30
CASTOR_TRAIL = 0.010
WHEEL_R = 0.025
CASTORS = {"hl": (XH, ZS, 1), "hr": (XH, -ZS, 1), "fl": (XF, ZS, -1), "fr": (XF, -ZS, -1)}

BONES = [
    ("ground", None, 0, 0, 0, 0, 0),
    ("frame", "ground", 0, 0, 0, 0, L),
    ("bars-right", "frame", XH + 0.02, BAR_Y[0], -ZS, 90, BAR_Y[1] - BAR_Y[0]),
    ("bars-left", "frame", XH + 0.02, BAR_Y[0], ZS, 90, BAR_Y[1] - BAR_Y[0]),
    ("bars-head", "frame", XH, END_BAR_Y[0], 0, 90, END_BAR_Y[1] - END_BAR_Y[0]),
    ("bars-foot", "frame", XF, END_BAR_Y[0], 0, 90, END_BAR_Y[1] - END_BAR_Y[0]),
    ("slip-carrier", "frame", LONG_X[7], BOTTOM[1] + BAR_R - SLIP_SHIFT[1], -ZS, 0, 0.02),
    ("slip-bars", "slip-carrier", 0, 0, 0, 90, round(BAR_Y[1] - BAR_Y[0], 4)),
    ("base", "frame", 0.045, BASE_RAIL_Y[0], 0, 0, 1.2),
    ("mattress", "base", 0, MAT_Y[0] - BASE_RAIL_Y[0], 0, 0, 1.2),
]
for key, (xc, zc, inward) in CASTORS.items():
    BONES.append((f"castor-{key}", "frame", xc, POST_Y0, zc, -90, 0.03))
    BONES.append((f"wheel-{key}", f"castor-{key}", POST_Y0 - WHEEL_R, inward * CASTOR_TRAIL, 0, 0, 0.0))


def frames():
    """🧭 World rest frames (origin, u, v, w) of every bone of the sagittal rig."""
    out = {}
    for bid, parent, x, y, z, ang, _ in BONES:
        if parent is None:
            out[bid] = ((0.0, 0.0, 0.0), 0.0)
            continue
        (po, pa) = out[parent]
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        o = (po[0] + c * x - s * y, po[1] + s * x + c * y, po[2] + z)
        out[bid] = (o, pa + ang)
    return out


FR = frames()


def to_local(bone, p):
    """📐 World rest point to (u, v, w) of the bone."""
    (o, a) = FR[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy, dz = p[0] - o[0], p[1] - o[1], p[2] - o[2]
    return (c * dx + s * dy, -s * dx + c * dy, dz)


def axis_local(bone, axis):
    """🧲 World axis letter to the local axis vector of the bone (90-degree frames only)."""
    vec = {"x": (1, 0, 0), "y": (0, 1, 0), "z": (0, 0, 1)}[axis]
    (o, a) = FR[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    u = c * vec[0] + s * vec[1]
    v = -s * vec[0] + c * vec[1]
    return [round(u), round(v), vec[2]]


def r6(v):
    """🔢 Six-decimal rounding for the document."""
    return round(v + 0.0, 6)


class Part:
    """🧱 One physical part: an axis-aligned box, a cylinder along a world axis, or a prism with a profile in the side plane."""

    def __init__(self, pid, bone, kind, material, region, tone, **k):
        self.id, self.bone, self.kind, self.material, self.region, self.tone = pid, bone, kind, material, region, tone
        self.k = k
        self.role = k.get("role", "body")
        self.hatch = k.get("hatch")
        self.flags = {f: True for f in ("slender", "sheet", "envelope", "texture") if k.get(f)}
        self.views = k.get("views", ("elevation", "front", "plan"))
        self.drawn = k.get("drawn", True)

    def bounds(self):
        """📦 World box of the part."""
        k = self.k
        if self.kind == "box":
            return k["x"], k["y"], k["z"]
        if self.kind == "cyl":
            c, r, ax, span = k["c"], k["r"], k["axis"], k["span"]
            b = [(c[i] - r, c[i] + r) for i in range(3)]
            b["xyz".index(ax)] = span
            return tuple(b)
        if self.kind == "prof":
            xs = [p[0] for p in k["profile"]]
            ys = [p[1] for p in k["profile"]]
            return (min(xs), max(xs)), (min(ys), max(ys)), k["z"]
        raise ValueError(self.kind)

    def mass(self, density):
        """⚖️ Real mass: a stated mass (a sheet-metal fitting generalised to a block) or volume x density."""
        return self.k.get("mass_kg") or self.volume() * density

    def volume(self):
        """⚖️ Real volume in cubic metres."""
        if self.kind == "box":
            (x0, x1), (y0, y1), (z0, z1) = self.bounds()
            return (x1 - x0) * (y1 - y0) * (z1 - z0)
        if self.kind == "cyl":
            return math.pi * self.k["r"] ** 2 * (self.k["span"][1] - self.k["span"][0])
        pts = self.k["profile"]
        a = 0.5 * abs(sum(pts[i][0] * pts[i - 1][1] - pts[i - 1][0] * pts[i][1] for i in range(len(pts))))
        return a * (self.k["z"][1] - self.k["z"][0])

    def centre(self):
        """🎯 World centre of the bounds."""
        return tuple((a + b) / 2 for a, b in self.bounds())

    def solid(self, sid=None):
        """🧊 The volume solid of the part in its bone frame."""
        base = {"id": sid or self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        base.update(self.flags)
        k = self.k
        if self.kind == "box":
            (x0, x1), (y0, y1), (z0, z1) = self.bounds()
            corners = [to_local(self.bone, (x, y, z)) for x in (x0, x1) for y in (y0, y1) for z in (z0, z1)]
            lo = [min(c[i] for c in corners) for i in range(3)]
            hi = [max(c[i] for c in corners) for i in range(3)]
            s = {**base, "kind": "box", "size": [r6(hi[i] - lo[i]) for i in range(3)], "transform": {"translation": [r6((hi[i] + lo[i]) / 2) for i in range(3)]}}
            if k.get("radius"):
                s["cornerRadius"] = k["radius"]
            return s
        if self.kind == "cyl":
            ax = k["axis"]
            c = list(k["c"])
            c["xyz".index(ax)] = k["span"][0]
            o = to_local(self.bone, c)
            length = k["span"][1] - k["span"][0]
            if k.get("capsule"):
                e = list(c)
                e["xyz".index(ax)] = k["span"][1]
                q = to_local(self.bone, e)
                r = k["r"]
                al = axis_local(self.bone, ax)
                p0 = [r6(o[i] + al[i] * r) for i in range(3)]
                p1 = [r6(q[i] - al[i] * r) for i in range(3)]
                return {**base, "kind": "capsule", "p0": p0, "p1": p1, "r0": r, "r1": r}
            return {**base, "kind": "revolve", "profile": [[0, 0], [0, k["r"]], [r6(length), k["r"]], [r6(length), 0]], "axis": axis_local(self.bone, ax), "origin": [r6(v) for v in o]}
        if self.kind == "prof":
            loc = [to_local(self.bone, (p[0], p[1], 0)) for p in k["profile"]]
            oz = FR[self.bone][0][2]
            return {**base, "kind": "extrude", "profile": [[r6(p[0]), r6(p[1])] for p in loc], "plane": "xy", "from": r6(k["z"][0] - oz), "to": r6(k["z"][1] - oz)}
        raise ValueError(self.kind)

    def shape_base(self, view, z):
        """🖋️ The common members of one 2D shape of the part."""
        d = {"id": self.id, "bone": self.bone, "role": self.role, "z": round(z, 3), "filled": self.role != "texture", "tone": self.tone, "region": self.region, "material": self.material}
        if self.hatch and view in self.hatch:
            d["hatch"] = self.hatch[view]
        return d

    def shapes(self, view):
        """✏️ The exact orthographic outline of the part in one view (elevation in bone frame, front and plan in world rest)."""
        if view not in self.views or not self.drawn:
            return []
        (x0, x1), (y0, y1), (z0, z1) = self.bounds()
        order = self.k.get("order", {}).get(view, {"elevation": -z0, "front": x1, "plan": y1}[view] * 10)
        b = self.shape_base(view, order)
        if self.k.get("solid_u"):
            return [self.solid_u_shape(b)]
        rr = self.k.get("radius")
        if view == "elevation":
            if self.kind == "prof":
                pts = [to_local(self.bone, (p[0], p[1], 0)) for p in self.k["profile"]]
                return [{**b, "kind": "poly", "closed": True, "points": [[r6(p[0]), r6(p[1])] for p in pts]}]
            if self.kind == "cyl" and self.k["axis"] == "z":
                c = to_local(self.bone, self.k["c"])
                return [{**b, "kind": "ellipse", "cx": r6(c[0]), "cy": r6(c[1]), "rx": self.k["r"], "ry": self.k["r"]}]
            corners = [to_local(self.bone, (x, y, 0)) for x in (x0, x1) for y in (y0, y1)]
            u0, u1 = min(c[0] for c in corners), max(c[0] for c in corners)
            v0, v1 = min(c[1] for c in corners), max(c[1] for c in corners)
            return [rect(b, u0, v0, u1 - u0, v1 - v0, rr)]
        if view == "front":
            if self.kind == "cyl" and self.k["axis"] == "x":
                c = self.k["c"]
                return [{**b, "kind": "ellipse", "cx": r6(c[2]), "cy": r6(c[1]), "rx": self.k["r"], "ry": self.k["r"]}]
            return [rect(b, z0, y0, z1 - z0, y1 - y0, rr)]
        if self.kind == "cyl" and self.k["axis"] == "y":
            c = self.k["c"]
            return [{**b, "kind": "ellipse", "cx": r6(c[0]), "cy": r6(c[2]), "rx": self.k["r"], "ry": self.k["r"]}]
        return [rect(b, x0, z0, x1 - x0, z1 - z0, rr)]

    def solid_u_shape(self, b):
        """🪵 A bar along its bone drawn as a solid swept along u, so every view follows the bone through any spread."""
        c = to_local(self.bone, self.k["c"])
        lo = to_local(self.bone, tuple(self.k["c"][i] if i != 1 else self.k["span"][0] for i in range(3)))
        hi = to_local(self.bone, tuple(self.k["c"][i] if i != 1 else self.k["span"][1] for i in range(3)))
        r = self.k["r"]
        ring = [[r6(c[1] + r * math.cos(2 * math.pi * i / 16)), r6(c[2] + r * math.sin(2 * math.pi * i / 16))] for i in range(16)]
        return {**b, "kind": "poly", "closed": True, "points": ring, "solid": {"kind": "extruded", "sweep": "u", "z0": r6(lo[0]), "z1": r6(hi[0])}}


def rect(b, x, y, w, h, r=None):
    """▭ A rect shape."""
    s = {**b, "kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}
    if r:
        s["r"] = r
    return s


def box(pid, bone, mat, reg, tone, x, y, z, **k):
    """📦 Box part from world extents."""
    return Part(pid, bone, "box", mat, reg, tone, x=x, y=y, z=z, **k)


def cyl(pid, bone, mat, reg, tone, c, r, axis, span, **k):
    """🥢 Cylinder part along a world axis."""
    return Part(pid, bone, "cyl", mat, reg, tone, c=c, r=r, axis=axis, span=span, **k)


def prof(pid, bone, mat, reg, tone, profile, z, **k):
    """✂️ Prism part with a side-plane profile."""
    return Part(pid, bone, "prof", mat, reg, tone, profile=profile, z=z, **k)


def gnawed_rail_profile():
    """🦷 Side profile of the right long top rail with the teeth marks: a 3 mm deep chewed run of 0.29 m in its middle."""
    x0, x1 = 0.04, 1.25
    y0, y1 = LONG_TOP
    pts = [(x0, y0), (x1, y0), (x1, y1)]
    a, b = 0.79, 0.50
    n = 29
    for i in range(n + 1):
        x = a - (a - b) * i / n
        depth = 0.0 if i in (0, n) else (0.003 if i % 2 else 0.0015) * math.sin(math.pi * i / n) ** 0.5
        pts.append((round(x, 4), round(y1 - depth, 5)))
    pts.append((x0, y1))
    return pts


def sagging_mattress_profile(sag):
    """🛏️ Side profile of the mattress: rounded ends of 20 mm and a sag of the top where the baby lay."""
    x0, x1 = MAT_X
    y0, y1 = MAT_Y
    r = 0.02
    pts = [(x0 + r, y0), (x1 - r, y0)]
    for i in range(1, 4):
        t = math.pi / 2 * i / 4
        pts.append((x1 - r + r * math.sin(t), y0 + r - r * math.cos(t)))
    pts.append((x1, y0 + r))
    pts.append((x1, y1 - r))
    for i in range(1, 4):
        t = math.pi / 2 * i / 4
        pts.append((x1 - r + r * math.cos(t), y1 - r + r * math.sin(t)))
    pts.append((x1 - r, y1))
    for i in range(1, 12):
        x = x1 - r - (x1 - x0 - 2 * r) * i / 12
        t = (x - 0.33) / (0.96 - 0.33)
        dip = sag * math.sin(math.pi * t) ** 2 if 0 < t < 1 else 0.0
        pts.append((x, y1 - dip))
    pts.append((x0 + r, y1))
    for i in range(1, 4):
        t = math.pi / 2 * i / 4
        pts.append((x0 + r - r * math.sin(t), y1 - r + r * math.cos(t)))
    pts.append((x0, y1 - r))
    pts.append((x0, y0 + r))
    for i in range(1, 4):
        t = math.pi / 2 * i / 4
        pts.append((x0 + r - r * math.cos(t), y0 + r - r * math.sin(t)))
    return [(round(p[0], 5), round(p[1], 5)) for p in pts]


WOOD = "beech-lacquered-aged"
SLIP_ORDER = {"elevation": 3.3, "front": 7.7, "plan": 2.0}
HG = {"elevation": "grain-v", "front": "grain-v"}
HH = {"elevation": "grain-h", "front": "grain-h"}


def frame_parts(scale):
    """🪵 Posts and rails of the frame bone at one scale."""
    rad = 0.006 if scale == 50 else None
    hatch_v = HG if scale == 50 else None
    hatch_h = HH if scale == 50 else None
    out = []
    for xn, x in (("head", (0, POST)), ("foot", (L - POST, L))):
        for zn, z in (("l", (ZS - 0.02, ZS + 0.02)), ("r", (-ZS - 0.02, -ZS + 0.02))):
            out.append(box(f"post-{xn}-{zn}", "frame", WOOD, "beech-frame", 0.5, x, (POST_Y0, H), z, radius=rad, hatch=hatch_v))
    for zn, zc in (("l", ZS), ("r", -ZS)):
        z = (zc - RAIL_T / 2, zc + RAIL_T / 2)
        if scale == 50 and zn == "r":
            out.append(prof("rail-top-r", "frame", WOOD, "beech-frame", 0.55, gnawed_rail_profile(), z, hatch=hatch_h))
        else:
            out.append(box(f"rail-top-{zn}", "frame", WOOD, "beech-frame", 0.55, (POST, L - POST), LONG_TOP, z, radius=rad, hatch=hatch_h))
        out.append(box(f"rail-bottom-{zn}", "frame", WOOD, "beech-frame", 0.55, (POST, L - POST), BOTTOM, z, radius=rad, hatch=hatch_h))
    for xn, xc in (("head", XH), ("foot", XF)):
        x = (xc - RAIL_T / 2, xc + RAIL_T / 2)
        out.append(box(f"end-top-{xn}", "frame", WOOD, "beech-frame", 0.55, x, END_TOP, (-ZS + 0.02, ZS - 0.02), radius=rad, hatch=hatch_h))
        out.append(box(f"end-bottom-{xn}", "frame", WOOD, "beech-frame", 0.55, x, BOTTOM, (-ZS + 0.02, ZS - 0.02), radius=rad, hatch=hatch_h))
    return out


def typified(lo, hi, n, d):
    """🧮 Centres of n bars of diameter d spread over the hull lo..hi of a finer row."""
    return [round(lo + d / 2 + (hi - lo - d) * i / (n - 1), 4) for i in range(n)]


def bar_parts(scale):
    """🎋 The four bar rows and the three removable slip bars at one scale (typified at 1:200 and 1:500)."""
    out = []
    cap = False
    if scale in (50, 100):
        for side, zc, bone in (("r", -ZS, "bars-right"), ("l", ZS, "bars-left")):
            for k, x in enumerate(LONG_X):
                if side == "r" and k in SLIP:
                    continue
                out.append(cyl(f"bar-{side}-{k:02d}", bone, WOOD, "beech-frame", 0.6, (x, 0, zc), BAR_R, "y", BAR_Y, capsule=cap))
        for end, xc, bone in (("head", XH, "bars-head"), ("foot", XF, "bars-foot")):
            for k, z in enumerate(END_Z):
                out.append(cyl(f"bar-{end}-{k}", bone, WOOD, "beech-frame", 0.6, (xc, 0, z), BAR_R, "y", END_BAR_Y, capsule=cap))
        for k in SLIP:
            out.append(cyl(f"slip-bar-{k:02d}", "slip-bars", WOOD, "beech-frame", 0.62, (LONG_X[k], 0, -ZS), BAR_R, "y", BAR_Y, capsule=cap, solid_u=True, order=SLIP_ORDER))
        return out
    if scale == 200:
        d = 0.03
        right = typified(LONG_X[0] - BAR_R, LONG_X[5] + BAR_R, 3, d) + typified(LONG_X[9] - BAR_R, LONG_X[14] + BAR_R, 3, d)
        left = right[:3] + [LONG_X[7]] + right[3:]
        for side, zc, bone, xs in (("r", -ZS, "bars-right", right), ("l", ZS, "bars-left", left)):
            for i, x in enumerate(xs):
                out.append(cyl(f"bar-{side}-{i}", bone, WOOD, "beech-frame", 0.6, (x, 0, zc), d / 2, "y", BAR_Y, envelope=True))
        for end, xc, bone in (("head", XH, "bars-head"), ("foot", XF, "bars-foot")):
            for i, z in enumerate(typified(END_Z[0] - BAR_R, END_Z[-1] + BAR_R, 3, d)):
                out.append(cyl(f"bar-{end}-{i}", bone, WOOD, "beech-frame", 0.6, (xc, 0, z), d / 2, "y", END_BAR_Y, envelope=True))
        ds = 0.028
        for i, x in enumerate(typified(LONG_X[SLIP[0]] - BAR_R, LONG_X[SLIP[-1]] + BAR_R, 2, ds)):
            out.append(cyl(f"slip-bar-{i}", "slip-bars", WOOD, "beech-frame", 0.62, (x, 0, -ZS), ds / 2, "y", BAR_Y, envelope=True, solid_u=True, order=SLIP_ORDER))
        return out
    t = 2 * BAR_R
    rows = [("bars-right", "row-r-head", (LONG_X[0] - BAR_R, LONG_X[5] + BAR_R), -ZS), ("bars-right", "row-r-foot", (LONG_X[9] - BAR_R, LONG_X[14] + BAR_R), -ZS), ("bars-left", "row-l", (LONG_X[0] - BAR_R, LONG_X[14] + BAR_R), ZS)]
    for bone, rid, xs, zc in rows:
        out.append(box(rid, bone, WOOD, "beech-frame", 0.6, xs, (BOTTOM[1], LONG_TOP[0]), (zc - t / 2, zc + t / 2), envelope=True, drawn=False))
    for bone, rid, xc in (("bars-head", "row-head", XH), ("bars-foot", "row-foot", XF)):
        out.append(box(rid, bone, WOOD, "beech-frame", 0.6, (xc - t / 2, xc + t / 2), (BOTTOM[1], END_TOP[0]), (END_Z[0] - BAR_R, END_Z[-1] + BAR_R), envelope=True, drawn=False))
    out.append(box("row-slip", "slip-bars", WOOD, "beech-frame", 0.62, (LONG_X[SLIP[0]] - BAR_R, LONG_X[SLIP[-1]] + BAR_R), (BOTTOM[1], LONG_TOP[0]), (-ZS - t / 2, -ZS + t / 2), envelope=True, drawn=False))
    return out


def true_bars(scale):
    """🎋 The bars at their true 22 mm section, kept slender in the thermal and structure lists of the typified scales."""
    out = []
    for side, zc, bone in (("r", -ZS, "bars-right"), ("l", ZS, "bars-left")):
        for k, x in enumerate(LONG_X):
            if side == "r" and k in SLIP:
                continue
            out.append(cyl(f"t-bar-{side}-{k:02d}", bone, WOOD, "beech-frame", 0.6, (x, 0, zc), BAR_R, "y", BAR_Y, slender=True))
    for end, xc, bone in (("head", XH, "bars-head"), ("foot", XF, "bars-foot")):
        for k, z in enumerate(END_Z):
            out.append(cyl(f"t-bar-{end}-{k}", bone, WOOD, "beech-frame", 0.6, (xc, 0, z), BAR_R, "y", END_BAR_Y, slender=True))
    for k in SLIP:
        out.append(cyl(f"t-slip-bar-{k:02d}", "slip-bars", WOOD, "beech-frame", 0.62, (LONG_X[k], 0, -ZS), BAR_R, "y", BAR_Y, slender=True))
    return out


def base_parts(scale):
    """🪜 The slatted base on its height-adjustable bone and its four steel hanger brackets."""
    out = []
    if scale == 50:
        for zn, sgn in (("l", 1), ("r", -1)):
            z = tuple(sorted((sgn * BASE_RAIL_Z[0], sgn * BASE_RAIL_Z[1])))
            out.append(box(f"base-rail-{zn}", "base", "beech-slat", "slatted-base", 0.5, BASE_X, BASE_RAIL_Y, z))
        for xn, x in (("head", (BASE_X[0], BASE_X[0] + 0.022)), ("foot", (BASE_X[1] - 0.022, BASE_X[1]))):
            out.append(box(f"base-end-{xn}", "base", "beech-slat", "slatted-base", 0.5, x, (BASE_RAIL_Y[0] + 0.01, BASE_RAIL_Y[1]), (-BASE_RAIL_Z[0], BASE_RAIL_Z[0])))
        for k, x in enumerate(SLAT_X):
            mat, tone = ("pine-slat-replacement", 0.72) if k == REPLACED_SLAT else ("beech-slat", 0.62)
            out.append(box(f"slat-{k:02d}", "base", mat, "slatted-base", tone, (x - SLAT_W / 2, x + SLAT_W / 2), SLAT_Y, (-BASE_RAIL_Z[1], BASE_RAIL_Z[1])))
    else:
        out.append(box("base-tray", "base", "beech-slat", "slatted-base", 0.55, BASE_X, (BASE_RAIL_Y[0], SLAT_Y[1]), (-BASE_RAIL_Z[1], BASE_RAIL_Z[1])))
    if True:
        for xn, x in (("head", (POST, 0.075)), ("foot", (L - 0.075, L - POST))):
            for zn, sgn in (("l", 1), ("r", -1)):
                z = tuple(sorted((sgn * 0.27, sgn * 0.32)))
                out.append(box(f"bracket-{xn}-{zn}", "base", "steel-zinc", "brackets", 0.35, x, (BASE_RAIL_Y[0] + 0.005, BASE_RAIL_Y[0] + 0.035), z, mass_kg=0.05))
    return out


def mattress_parts(scale):
    """🛏️ The 60 x 120 x 10 cm foam mattress on its own bone, sagging 12 mm where the baby lay (1:50)."""
    z = (-MAT_Z, MAT_Z)
    if scale == 50:
        return [prof("mattress", "mattress", "mattress-cover", "mattress", 0.85, sagging_mattress_profile(0.012), z)]
    return [box("mattress", "mattress", "mattress-cover", "mattress", 0.85, MAT_X, MAT_Y, z)]


def castor_parts(scale):
    """🛞 Four swivel castors with 50 mm wheels; the head pair has brake pedals, the right foot castor is a black replacement without brake."""
    out = []
    for key, (xc, zc, inward) in CASTORS.items():
        bone, wb = f"castor-{key}", f"wheel-{key}"
        replaced = key == "fr"
        fork_mat = "castor-polyamide-black" if replaced else "castor-polyamide"
        tone = 0.12 if replaced else 0.3
        wx = xc + inward * CASTOR_TRAIL
        if scale == 500:
            out.append(box(f"castor-{key}", bone, fork_mat, "castors", tone, (xc - 0.02, xc + 0.02), (0.0, POST_Y0), (zc - 0.02, zc + 0.02), drawn=False))
            continue
        w = 0.02 if scale in (50, 100) else 0.03
        if scale == 50:
            out.append(box(f"socket-{key}", bone, fork_mat, "castors", tone, (xc - 0.018, xc + 0.018), (POST_Y0 - 0.008, POST_Y0), (zc - 0.018, zc + 0.018)))
            out.append(box(f"fork-{key}", bone, fork_mat, "castors", tone, (min(xc, wx) - 0.015, max(xc, wx) + 0.015), (WHEEL_R - 0.004, POST_Y0 - 0.008), (zc - 0.015, zc + 0.015)))
            if not replaced and xc < 0.5:
                out.append(box(f"brake-{key}", bone, fork_mat, "castors", 0.22, (wx + 0.012, wx + 0.034), (0.040, 0.048), (zc - 0.01, zc + 0.01)))
        else:
            out.append(box(f"fork-{key}", bone, fork_mat, "castors", tone, (min(xc, wx) - 0.015, max(xc, wx) + 0.015), (WHEEL_R, POST_Y0), (zc - 0.015, zc + 0.015)))
        out.append(cyl(f"wheel-{key}", wb, "castor-rubber", "castors", 0.18 if replaced else 0.28, (wx, WHEEL_R, zc), WHEEL_R, "z", (zc - w / 2, zc + w / 2)))
    return out


def parts(scale):
    """🧾 Every body part of one scale."""
    return frame_parts(scale) + bar_parts(scale) + base_parts(scale) + mattress_parts(scale) + castor_parts(scale)
