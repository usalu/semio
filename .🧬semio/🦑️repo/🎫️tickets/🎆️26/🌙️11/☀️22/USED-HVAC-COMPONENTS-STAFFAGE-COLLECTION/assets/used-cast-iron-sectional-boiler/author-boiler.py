"""🔥 Authoritative generator of the used cast-iron sectional boiler: every hand-chosen number of the 1970s six-section 45 kW boiler on its bearers, drawn in 2D and modelled in 3D per scale from one part list, written to 🔣️.json next to this file."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-cast-iron-sectional-boiler"


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


ORIGIN = {"floor": (0.0, 0.0), "bearers": (0.0, 0.0), "boiler": (0.0, 0.10)}

BEARER_Z = 0.22
BEARER_X = (-0.02, 0.98)
BEARER_S = 0.10
CHAN_X = (0.12, 0.88)
JACKET = ((0.10, 0.15, -0.31), (0.90, 0.95, 0.31))
SECTIONS = [("front", 0.74, 0.87, 40.0), ("middle", 0.26, 0.74, 132.0), ("rear", 0.13, 0.26, 38.0)]
BLOCK_Y = (0.15, 0.88)
BLOCK_Z = 0.26
WOOL = 0.025
SHEET = 0.001
PANEL_PROFILE = [(0.70, 0.95), (0.90, 0.95), (0.90, 0.99), (0.86, 1.05), (0.70, 1.05)]
PANEL_Z = 0.30
DOOR = ((0.90, 0.52, -0.20), (0.96, 0.88, 0.20))
PLATE = ((0.96, 0.58, -0.12), (0.972, 0.82, 0.12))
BOLTS = [(0.60, -0.10), (0.60, 0.10), (0.80, -0.10), (0.80, 0.10)]
BOLT_R, BOLT_X = 0.0095, (0.972, 0.98)
COVER = ((0.90, 0.20, -0.20), (0.93, 0.34, 0.20))
LUGS = [((0.92, 0.56, 0.20), (0.95, 0.60, 0.23)), ((0.92, 0.80, 0.20), (0.95, 0.84, 0.23))]
BOSS = (0.96, 0.975, 0.85, -0.15, 0.02)
VALVE = (0.90, 0.945, 0.19, -0.25, 0.012)
SPIGOT = (-0.03, 0.10, 0.78, 0.0, 0.075, 0.073)
FLOW = (0.22, 0.15, 0.95, 1.00, 0.021, 1.025, 0.026)
RETURN = (0.30, -0.15, 0.05, 0.10, 0.021, 0.025, 0.026)
EYES = [0.30, 0.62]
EYE_C, EYE_RO, EYE_RI, EYE_T = 0.983, 0.025, 0.013, 0.012

M_BEARER = 2 * BEARER_S * BEARER_S * (BEARER_X[1] - BEARER_X[0]) * 450.0
MASSES = {
    "sections": (210.0, (0.50, 0.515)),
    "jacket": (2.77 * SHEET * 7850.0, (0.50, 0.55)),
    "wool": (2.60 * WOOL * 60.0, (0.50, 0.55)),
    "doors": (22.0 + 6.0, (0.93, 0.56)),
    "plate": (0.24 * 0.24 * 0.012 * 7850.0 + 0.4, (0.966, 0.70)),
    "channels": (2 * 5.59 * (CHAN_X[1] - CHAN_X[0]), (0.50, 0.125)),
    "spigot": (1.5, (0.035, 0.78)),
    "panel": (3.0, (0.80, 1.00)),
    "stubs": (2.5, (0.17, 0.60)),
    "eyes": (0.6, (0.50, 0.98)),
    "tie-rods": (4 * 0.0001131 * 0.76 * 7850.0, (0.50, 0.515)),
}
M_BOILER = sum(m for m, _ in MASSES.values())
COM_BOILER = (sum(m * c[0] for m, c in MASSES.values()) / M_BOILER, sum(m * c[1] for m, c in MASSES.values()) / M_BOILER)
M_TOTAL = M_BEARER + M_BOILER
WATER_KG = 39.0

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}


class Part:
    """🧱 One physical part of one scale: its 3D solid in the bone frame and its three 2D projections from the same numbers."""

    def __init__(self, pid, bone, kind, geo, material, tone, region, views=("elevation", "front", "plan"), body=True, role="body", extra=None):
        self.id, self.bone, self.kind, self.geo, self.material, self.tone, self.region = pid, bone, kind, geo, material, tone, region
        self.views, self.body, self.role, self.extra = views, body, role, extra or {}

    def solid(self):
        """🧊 The volume solid of the part in the rest frame of its bone."""
        ox, oy = ORIGIN[self.bone]
        g = self.geo
        base = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
            s = dict(base, kind="box", size=[r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], transform={"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2)]})
            if self.extra.get("radius"):
                s["cornerRadius"] = self.extra["radius"]
        elif self.kind == "cylx":
            x0, x1, cy, cz, r, ri = g
            prof = [[0, ri], [0, r], [r6(x1 - x0), r], [r6(x1 - x0), ri], [0, ri]] if ri else [[0, 0], [0, r], [r6(x1 - x0), r], [r6(x1 - x0), 0]]
            s = dict(base, kind="revolve", profile=prof, axis=[1, 0, 0], origin=[r6(x0 - ox), r6(cy - oy), cz])
        elif self.kind == "cyly":
            cx, cz, y0, y1, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(y1 - y0), r], [r6(y1 - y0), 0]], axis=[0, 1, 0], origin=[r6(cx - ox), r6(y0 - oy), cz])
        elif self.kind == "extxy":
            prof, z0, z1 = g
            s = dict(base, kind="extrude", plane="xy", profile=[[r6(x - ox), r6(y - oy)] for x, y in prof], **{"from": z0, "to": z1})
        elif self.kind == "chan":
            x0, x1, prof = g
            s = dict(base, kind="extrude", plane="yz", profile=[[r6(y - oy), r6(z)] for y, z in prof], **{"from": r6(x0 - ox), "to": r6(x1 - ox)})
        elif self.kind == "ring":
            cx, cy, cz, ro, ri, t = g
            n = 20
            outer = [[r6(cx - ox + ro * math.cos(2 * math.pi * k / n)), r6(cy - oy + ro * math.sin(2 * math.pi * k / n))] for k in range(n)]
            s = dict(base, kind="extrude", plane="xy", profile=outer, **{"from": r6(cz - t / 2), "to": r6(cz + t / 2)})
            if ri:
                s["holes"] = [[[r6(cx - ox + ri * math.cos(2 * math.pi * k / n)), r6(cy - oy + ri * math.sin(2 * math.pi * k / n))] for k in range(n)][::-1]]
        for key in ("slender", "sheet", "texture", "repeat", "tags", "sectionFaces"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s

    def bounds(self):
        """📦 World bounds (x0, y0, z0), (x1, y1, z1) of the part at rest."""
        g = self.geo
        if self.kind == "box":
            return g
        if self.kind == "cylx":
            x0, x1, cy, cz, r, _ = g
            return (x0, cy - r, cz - r), (x1, cy + r, cz + r)
        if self.kind == "cyly":
            cx, cz, y0, y1, r = g
            return (cx - r, y0, cz - r), (cx + r, y1, cz + r)
        if self.kind == "extxy":
            prof, z0, z1 = g
            return (min(p[0] for p in prof), min(p[1] for p in prof), z0), (max(p[0] for p in prof), max(p[1] for p in prof), z1)
        if self.kind == "chan":
            x0, x1, prof = g
            return (x0, min(p[0] for p in prof), min(p[1] for p in prof)), (x1, max(p[0] for p in prof), max(p[1] for p in prof))
        if self.kind == "ring":
            cx, cy, cz, ro, ri, t = g
            return (cx - ro, cy - ro, cz - t / 2), (cx + ro, cy + ro, cz + t / 2)

    def shape(self, view):
        """✏ The filled 2D shape of the part in one viewpoint, in that viewpoint's frame."""
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

        def circle(cx, cy, r, ri=None):
            if ri:
                return dict(s, kind="path", d=ring_path(cx, cy, r, ri))
            return dict(s, kind="ellipse", cx=r6(cx), cy=r6(cy), rx=r6(r), ry=r6(r))

        if view == "elevation":
            if self.kind == "extxy":
                return dict(s, kind="poly", closed=True, points=[[r6(x - ox), r6(y - oy)] for x, y in g[0]])
            if self.kind == "ring":
                cx, cy, cz, ro, ri, t = g
                return circle(cx - ox, cy - oy, ro, ri)
            return rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)
        if view == "front":
            if self.kind == "cylx":
                x0_, x1_, cy, cz, r, ri = g
                return circle(cz, cy, r, ri)
            if self.kind == "chan":
                return dict(s, kind="poly", closed=True, points=[[r6(z), r6(y)] for y, z in g[2]])
            return rect(z0, y0, z1 - z0, y1 - y0)
        if self.kind == "cyly":
            cx, cz, _, _, r = g
            return circle(cx, cz, r)
        return rect(x0, z0, x1 - x0, z1 - z0)


def ring_path(cx, cy, r, ri):
    """⭕ An SVG annulus: outer circle and the bore as a hole."""
    return (f"M {r6(cx - r)} {r6(cy)} A {r} {r} 0 1 0 {r6(cx + r)} {r6(cy)} A {r} {r} 0 1 0 {r6(cx - r)} {r6(cy)} Z "
            f"M {r6(cx - ri)} {r6(cy)} A {ri} {ri} 0 1 1 {r6(cx + ri)} {r6(cy)} A {ri} {ri} 0 1 1 {r6(cx - ri)} {r6(cy)} Z")


def channel_profile(side):
    """🔩 The DIN 1026 U 50 base channel standing on its web edge: 50 mm high, 38 mm flanges of 7 mm, web 5 mm on the outer side."""
    zo = side * 0.24
    zi = side * 0.202
    zw = side * 0.235
    pts = [(0.10, zi), (0.10, zo), (0.15, zo), (0.15, zi), (0.143, zi), (0.143, zw), (0.107, zw), (0.107, zi)]
    return pts if side > 0 else pts[::-1]


def parts_of(scale):
    """🗂 The hand-chosen part list of one scale: every part the real boiler has that this scale can show, generalised for its smallest feature."""
    P = []
    j0, j1 = JACKET
    for side, name in ((-1, "right"), (1, "left")):
        zc = side * BEARER_Z
        P.append(Part(f"bearer-{name}", "bearers", "box", ((BEARER_X[0], 0.0, zc - BEARER_S / 2), (BEARER_X[1], BEARER_S, zc + BEARER_S / 2)), "softwood", 0.62, "bearers",
                      views=("elevation", "front", "plan") if name == "right" else ("front", "plan"), extra={"radius": 0.004} if scale == 50 else {}))
    if scale == 200:
        P.append(Part("base-frame", "boiler", "box", ((CHAN_X[0], 0.10, -0.24), (CHAN_X[1], 0.15, 0.24)), "steel", 0.28, "steelwork", views=("elevation", "front")))
    else:
        for side, name in ((-1, "right"), (1, "left")):
            if scale == 50:
                P.append(Part(f"channel-{name}", "boiler", "chan", (CHAN_X[0], CHAN_X[1], channel_profile(side)), "steel", 0.28, "steelwork",
                              views=("elevation", "front") if name == "right" else ("front",)))
            else:
                zs = sorted((side * 0.202, side * 0.24))
                P.append(Part(f"channel-{name}", "boiler", "box", ((CHAN_X[0], 0.10, zs[0]), (CHAN_X[1], 0.15, zs[1])), "steel", 0.28, "steelwork",
                              views=("elevation", "front") if name == "right" else ("front",)))
    P.append(Part("jacket", "boiler", "box", (j0, j1), "painted-steel", 0.74, "jacket" if scale != 200 else ["jacket", "steelwork"], extra={"radius": 0.008} if scale == 50 else {}))
    if scale == 200:
        P.append(Part("control-panel", "boiler", "box", ((0.70, 0.95, -PANEL_Z), (0.90, 1.05, PANEL_Z)), "painted-steel", 0.5, "control-panel"))
    else:
        P.append(Part("control-panel", "boiler", "extxy", (PANEL_PROFILE, -PANEL_Z, PANEL_Z), "painted-steel", 0.5, "control-panel"))
    if scale == 200:
        P.append(Part("burner-door", "boiler", "box", ((0.90, 0.52, -0.20), (0.945, 0.88, 0.20)), "cast-iron", 0.3, "doors"))
        P.append(Part("blanking-plate", "boiler", "box", ((0.945, 0.58, -0.12), (0.975, 0.82, 0.12)), "steel", 0.2, "blanking-plate"))
        P.append(Part("cleaning-cover", "boiler", "box", COVER, "cast-iron", 0.3, "doors", views=("elevation", "front")))
    else:
        P.append(Part("burner-door", "boiler", "box", DOOR, "cast-iron", 0.3, "doors", extra={"radius": 0.004} if scale == 50 else {}))
        if scale == 50:
            P.append(Part("blanking-plate", "boiler", "box", PLATE, "steel", 0.2, "blanking-plate"))
            for k, (y, z) in enumerate(BOLTS):
                P.append(Part(f"bolt-{k}", "boiler", "cylx", (BOLT_X[0], BOLT_X[1], y, z, BOLT_R, None), "steel", 0.12, "blanking-plate",
                              views=("front",) + (("elevation",) if z < 0 else ()) + (("plan",) if y > 0.7 else ())))
            for k, (lo, hi) in enumerate(LUGS):
                P.append(Part(f"hinge-lug-{k}", "boiler", "box", (lo, hi), "cast-iron", 0.3, "doors", views=("front", "plan") if k == 1 else ("front",)))
            bx0, bx1, by, bz, br = BOSS
            P.append(Part("sight-boss", "boiler", "cylx", (bx0, bx1, by, bz, br, None), "cast-iron", 0.24, "doors", views=("elevation", "front", "plan")))
        else:
            P.append(Part("blanking-plate", "boiler", "box", ((0.96, 0.58, -0.12), (0.98, 0.82, 0.12)), "steel", 0.2, "blanking-plate"))
            P.append(Part("hinge-lugs", "boiler", "box", ((0.92, 0.56, 0.20), (0.95, 0.84, 0.23)), "cast-iron", 0.3, "doors", views=("front", "plan")))
        P.append(Part("cleaning-cover", "boiler", "box", COVER, "cast-iron", 0.3, "doors", views=("elevation", "front")))
    if scale == 50:
        vx0, vx1, vy, vz, vr = VALVE
        P.append(Part("drain-valve", "boiler", "cylx", (vx0, vx1, vy, vz, vr, None), "brass", 0.42, "stubs"))
    sx0, sx1, sy, sz, sr, sri = SPIGOT
    P.append(Part("flue-spigot", "boiler", "cylx", (sx0, sx1, sy, sz, sr, sri if scale != 200 else None), "flue-steel", 0.34, "flue-spigot", views=("elevation", "plan")))
    fx, fz, fy0, fy1, fr, fyc, fcr = FLOW
    rh, rz, rx0, rx1, rr, rxc, rcr = RETURN
    if scale == 200:
        P.append(Part("flow-stub", "boiler", "cyly", (fx, fz, fy0, fyc, fcr), "steel", 0.4, "stubs", views=("elevation", "plan")))
        P.append(Part("return-stub", "boiler", "cylx", (rxc, rx1, rh, rz, rcr, None), "steel", 0.4, "stubs", views=("elevation", "plan")))
    else:
        P.append(Part("flow-stub", "boiler", "cyly", (fx, fz, fy0, fy1, fr), "steel", 0.4, "stubs", views=("elevation",)))
        P.append(Part("flow-cap", "boiler", "cyly", (fx, fz, fy1, fyc, fcr), "steel", 0.22, "stubs", views=("elevation", "plan")))
        P.append(Part("return-stub", "boiler", "cylx", (rx0, rx1, rh, rz, rr, None), "steel", 0.4, "stubs", views=("elevation", "plan")))
        P.append(Part("return-cap", "boiler", "cylx", (rxc, rx0, rh, rz, rcr, None), "steel", 0.22, "stubs", views=("elevation", "plan")))
    if scale != 200:
        for k, x in enumerate(EYES):
            if scale == 50:
                P.append(Part(f"lifting-eye-{k}", "boiler", "ring", (x, EYE_C, 0.0, EYE_RO, EYE_RI, EYE_T), "steel", 0.2, "steelwork", views=("elevation", "plan")))
            else:
                P.append(Part(f"lifting-eye-{k}", "boiler", "ring", (x, 0.9725, 0.0, 0.0225, None, 0.016), "steel", 0.2, "steelwork", views=("elevation", "plan")))
    return P


def details_of(scale):
    """🖋 The open and closed detail and texture strokes of one scale: seams, type plate, dial holes, rust, grain and the sawn and capped ends."""
    e, f, p = [], [], []
    if scale == 200:
        return e, f, p
    oy = 0.10

    def line(lst, sid, bone, pts, role="detail", region=None, tone=0.25, z=9.0, closed=False, filled=False):
        s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}
        if region:
            s["region"] = region
        lst.append(s)

    line(e, "jacket-seam-front", "boiler", [(0.86, 0.15 - oy), (0.86, 0.95 - oy)], region="jacket")
    line(e, "jacket-seam-top", "boiler", [(0.10, 0.92 - oy), (0.70, 0.92 - oy)], region="jacket")
    line(e, "door-gap", "boiler", [(0.905, 0.52 - oy), (0.905, 0.88 - oy)], region="doors", z=8.5)
    if scale == 50:
        line(e, "bearer-grain-0", "bearers", [(-0.02, 0.035), (0.42, 0.036), (0.98, 0.033)], role="texture", region="bearers", z=6.0)
        line(e, "bearer-grain-1", "bearers", [(-0.02, 0.068), (0.55, 0.066), (0.98, 0.069)], role="texture", region="bearers", z=6.0)
        line(e, "type-plate", "boiler", [(0.16, 0.80 - oy), (0.30, 0.80 - oy), (0.30, 0.88 - oy), (0.16, 0.88 - oy)], region="jacket", tone=0.35, closed=True, filled=True, z=8.0)
        line(e, "type-plate-text", "boiler", [(0.18, 0.84 - oy), (0.28, 0.84 - oy)], role="texture", region="jacket", z=8.1)
        for k, x in enumerate((0.24, 0.47, 0.63, 0.81)):
            line(e, f"rust-streak-{k}", "boiler", [(x, 0.15 - oy), (x + 0.004, 0.24 - oy + 0.03 * (k % 2))], role="texture", region="jacket", tone=0.45, z=8.2)
        line(e, "channel-flange-top", "boiler", [(CHAN_X[0], 0.143 - oy), (CHAN_X[1], 0.143 - oy)], region="steelwork", z=8.0)
        line(e, "channel-flange-bottom", "boiler", [(CHAN_X[0], 0.107 - oy), (CHAN_X[1], 0.107 - oy)], region="steelwork", z=8.0)
        line(e, "spigot-soot", "boiler", [(-0.02, 0.712 - oy), (-0.02, 0.848 - oy)], role="texture", region="flue-spigot", z=8.0)
        line(e, "cover-gap", "boiler", [(0.905, 0.20 - oy), (0.905, 0.34 - oy)], region="doors", z=8.5)
        line(e, "flow-thread", "boiler", [(0.199, 0.99 - oy), (0.241, 0.99 - oy)], role="texture", region="stubs", z=8.0)
        line(e, "return-thread", "boiler", [(0.06, 0.279 - oy), (0.06, 0.321 - oy)], role="texture", region="stubs", z=8.0)
        line(f, "panel-dial-hole-left", "boiler", [(0.10 + 0.018 * math.cos(a), 1.00 + 0.018 * math.sin(a)) for a in [2 * math.pi * k / 16 for k in range(16)]], region="control-panel", tone=0.08, closed=True, filled=True, z=7.0)
        line(f, "panel-dial-hole-right", "boiler", [(-0.10 + 0.018 * math.cos(a), 1.00 + 0.018 * math.sin(a)) for a in [2 * math.pi * k / 16 for k in range(16)]], region="control-panel", tone=0.08, closed=True, filled=True, z=7.0)
        line(f, "plate-bolt-circle", "boiler", [(-0.10, 0.60), (0.10, 0.60), (0.10, 0.80), (-0.10, 0.80)], role="texture", region="blanking-plate", closed=True, z=7.5)
        line(p, "panel-slope", "boiler", [(0.86, -PANEL_Z), (0.86, PANEL_Z)], region="control-panel", z=7.0)
        line(p, "jacket-seam-plan", "boiler", [(0.86, -0.31), (0.86, -PANEL_Z)], region="jacket", z=7.0)
    line(f, "jacket-seam-door", "boiler", [(-0.31, 0.92), (0.31, 0.92)], region="jacket", z=5.95)
    line(f, "rust-front-0", "boiler", [(-0.27, 0.15), (-0.268, 0.21)], role="texture" if scale == 50 else "detail", region="jacket", tone=0.45, z=5.95)
    line(p, "panel-front-edge", "boiler", [(0.90, -PANEL_Z), (0.90, PANEL_Z)], region="control-panel", z=7.0)
    return e, f, p


def representation(scale):
    """🖨 The authored representation of one scale: the projected part shapes, the detail strokes, pens, hatches and decisions."""
    parts = parts_of(scale)
    geo = {"elevation": [], "front": [], "plan": []}
    for part in parts:
        for view in part.views:
            sh = part.shape(view)
            if scale == 50 and view == "front" and part.id.startswith("bearer-"):
                sh["hatch"] = "end-grain"
            geo[view].append(sh)
    e, f, p = details_of(scale)
    geo["elevation"] += e
    geo["front"] += f
    geo["plan"] += p
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    hatches = [{"id": "end-grain", "title": T("Sawn end grain of the softwood bearers", "Gesägtes Hirnholz der Nadelholz-Kanthölzer"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}] if scale == 50 else []
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geo, "decisions": DECISIONS_2D[scale]}


def thermal_list(scale, parts):
    """🌡 The thermal list of one scale: jacket shells (nested layers at 1:50), the hidden cast-iron block, every visible region by reference and the two emitter outlets."""
    out = []
    (x0, y0, z0), (x1, y1, z1) = JACKET
    if scale == 50:
        def plate(pid, lo, hi, layer):
            out.append({"id": pid, "kind": "box", "bone": "boiler", "material": "painted-steel" if layer == "jacket" else "mineral-wool", "role": "body", "tone": 0.74 if layer == "jacket" else 0.9,
                        "size": [r6(hi[0] - lo[0]), r6(hi[1] - lo[1]), r6(hi[2] - lo[2])], "transform": {"translation": [r6((lo[0] + hi[0]) / 2), r6((lo[1] + hi[1]) / 2 - 0.10), r6((lo[2] + hi[2]) / 2)]},
                        "region": "jacket", "layer": layer})
        t = SHEET
        plate("th-jacket-top", (x0, y1 - t, z0), (x1, y1, z1), "jacket")
        plate("th-jacket-front", (x1 - t, y0, z0), (x1, y1 - t, z1), "jacket")
        plate("th-jacket-rear", (x0, y0, z0), (x0 + t, y1 - t, z1), "jacket")
        plate("th-jacket-left", (x0 + t, y0, z1 - t), (x1 - t, y1 - t, z1), "jacket")
        plate("th-jacket-right", (x0 + t, y0, z0), (x1 - t, y1 - t, z0 + t), "jacket")
        w = WOOL
        a0, a1, b0, b1 = x0 + t, x1 - t, z0 + t, z1 - t
        plate("th-wool-top", (a0, y1 - t - w, b0), (a1, y1 - t, b1), "insulation")
        plate("th-wool-front", (a1 - w, y0, b0 + w), (a1, y1 - t - w, b1 - w), "insulation")
        plate("th-wool-rear", (a0, y0, b0 + w), (a0 + w, y1 - t - w, b1 - w), "insulation")
        plate("th-wool-left", (a0, y0, b1 - w), (a1, y1 - t - w, b1), "insulation")
        plate("th-wool-right", (a0, y0, b0), (a1, y1 - t - w, b0 + w), "insulation")
    else:
        out.append({"id": "th-jacket", "of": "jacket", "region": "jacket"})
    for name, sx0, sx1, _ in SECTIONS:
        if name == "middle":
            out.append({"id": "th-section-middle", "kind": "box", "bone": "boiler", "material": "cast-iron", "role": "body", "tone": 0.25, "size": [0.12, r6(BLOCK_Y[1] - BLOCK_Y[0]), 2 * BLOCK_Z],
                        "transform": {"translation": [0.32, r6((BLOCK_Y[0] + BLOCK_Y[1]) / 2 - 0.10), 0]}, "repeat": {"count": 4, "translation": [0.12, 0, 0]}, "region": "block"})
        else:
            out.append({"id": f"th-section-{name}", "kind": "box", "bone": "boiler", "material": "cast-iron", "role": "body", "tone": 0.25, "size": [r6(sx1 - sx0), r6(BLOCK_Y[1] - BLOCK_Y[0]), 2 * BLOCK_Z],
                        "transform": {"translation": [r6((sx0 + sx1) / 2), r6((BLOCK_Y[0] + BLOCK_Y[1]) / 2 - 0.10), 0]}, "region": "block"})
    for part in parts:
        if part.id == "jacket":
            continue
        region = part.region if isinstance(part.region, str) else part.region[0]
        out.append({"id": f"th-{part.id}", "of": part.id, "region": region})
    sx0, sx1, sy, sz, sr, sri = SPIGOT
    depth = max(FEATURE[scale], 0.008)
    if scale == 200:
        out.append({"id": "th-flue-outlet", "kind": "revolve", "bone": "boiler", "material": "flue-steel", "role": "detail", "tone": 0.2, "profile": [[0, 0], [0, 0.06], [depth, 0.06], [depth, 0]],
                    "axis": [-1, 0, 0], "origin": [r6(sx0), r6(sy - 0.10), 0], "region": "flue-spigot", "tags": ["flue-gas"]})
    else:
        out.append({"id": "th-flue-outlet", "kind": "revolve", "bone": "boiler", "material": "flue-steel", "role": "detail", "tone": 0.2, "profile": [[0, 0], [0, r6(sri - 0.001)], [depth, r6(sri - 0.001)], [depth, 0]],
                    "axis": [1, 0, 0], "origin": [r6(sx0), r6(sy - 0.10), 0], "region": "flue-spigot", "tags": ["flue-gas"]})
    pt = max(FEATURE[scale], 0.004)
    out.append({"id": "th-jacket-plume", "kind": "box", "bone": "boiler", "material": "painted-steel", "role": "detail", "tone": 0.74, "size": [0.36, pt, 0.40],
                "transform": {"translation": [0.45, r6(0.95 + pt / 2 - 0.10), 0]}, "region": "jacket", "tags": ["jacket-plume"]})
    return out


def structure_list(scale, parts):
    """🏗 The structural idealisation of one scale: the two bearers, the base channels and the cast-iron block with its tie rods as members, the bearings of the channels on the bearers as joints, and the centres of mass."""
    out = []
    names = {p.id for p in parts}
    for pid in ("bearer-right", "bearer-left"):
        out.append({"id": f"member-{pid}", "of": pid, "member": "bearers", "part": "member"})
    for pid in ("channel-right", "channel-left", "base-frame"):
        if pid in names:
            out.append({"id": f"member-{pid}", "of": pid, "member": "boiler", "part": "member"})
    if scale == 50:
        for name, sx0, sx1, _ in SECTIONS:
            if name == "middle":
                out.append({"id": "member-section-middle", "kind": "box", "bone": "boiler", "material": "cast-iron", "role": "body", "tone": 0.3, "size": [0.12, r6(BLOCK_Y[1] - BLOCK_Y[0]), 2 * BLOCK_Z],
                            "transform": {"translation": [0.32, r6((BLOCK_Y[0] + BLOCK_Y[1]) / 2 - 0.10), 0]}, "repeat": {"count": 4, "translation": [0.12, 0, 0]}, "member": "boiler", "part": "member"})
            else:
                out.append({"id": f"member-section-{name}", "kind": "box", "bone": "boiler", "material": "cast-iron", "role": "body", "tone": 0.3, "size": [r6(sx1 - sx0), r6(BLOCK_Y[1] - BLOCK_Y[0]), 2 * BLOCK_Z],
                            "transform": {"translation": [r6((sx0 + sx1) / 2), r6((BLOCK_Y[0] + BLOCK_Y[1]) / 2 - 0.10), 0]}, "member": "boiler", "part": "member"})
        for k, (y, z) in enumerate(((0.20, -0.266), (0.20, 0.266), (0.83, -0.266), (0.83, 0.266))):
            out.append({"id": f"member-tie-rod-{k}", "kind": "revolve", "bone": "boiler", "material": "steel", "role": "detail", "tone": 0.2, "profile": [[0, 0], [0, 0.006], [0.76, 0.006], [0.76, 0]],
                        "axis": [1, 0, 0], "origin": [0.12, r6(y - 0.10), z], "slender": True, "member": "boiler", "part": "member"})
    else:
        out.append({"id": "member-block", "kind": "box", "bone": "boiler", "material": "cast-iron", "role": "body", "tone": 0.3, "size": [0.74, r6(BLOCK_Y[1] - BLOCK_Y[0]), 2 * BLOCK_Z],
                    "transform": {"translation": [0.50, r6((BLOCK_Y[0] + BLOCK_Y[1]) / 2 - 0.10), 0]}, "member": "boiler", "part": "member"})
    pad = max(FEATURE[scale], 0.008)
    for k, (x, z) in enumerate(((0.16, -0.221), (0.16, 0.221), (0.84, -0.221), (0.84, 0.221))):
        out.append({"id": f"joint-bearing-{k}", "kind": "box", "bone": "boiler", "material": "steel", "role": "detail", "tone": 0.5, "size": [max(0.04, 2 * pad), pad, max(0.038, 2 * pad)],
                    "transform": {"translation": [x, 0.0, z]}, "member": "boiler", "part": "joint"})
    mr = max(0.03, FEATURE[scale])
    out.append({"id": "mass-boiler", "kind": "ellipsoid", "bone": "boiler", "material": "cast-iron", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [r6(COM_BOILER[0]), r6(COM_BOILER[1] - 0.10), 0]}, "member": "boiler", "part": "mass"})
    out.append({"id": "mass-bearers", "kind": "ellipsoid", "bone": "bearers", "material": "softwood", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [0.48, 0.05, -BEARER_Z]}, "member": "bearers", "part": "mass"})
    return out


def volume(scale):
    """🧊 The authored volume of one scale: body from the part list, thermal and structure lists, and the decisions."""
    parts = parts_of(scale)
    body = [p.solid() for p in parts if p.body]
    if scale == 50:
        body.append({"id": "type-plate", "kind": "box", "bone": "boiler", "material": "aluminium", "role": "detail", "tone": 0.35, "size": [0.14, 0.08, 0.0015],
                     "transform": {"translation": [0.23, 0.74, -0.31075]}, "texture": True, "sheet": True})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": DECISIONS_3D[scale]}


exec(open(os.path.join(HERE, "author-boiler-texts.py"), encoding="utf8").read())


def binding(regions, contact_ground=False, stored=True):
    """🌬 The region binding of a pose: everything indoors, nothing sunlit, the timber and the channels on their bearing faces."""
    out = {}
    for r in regions:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "bearers":
            b = {"exposedFraction": 0.83, "sunlitFraction": 0, "contact": {"fraction": 0.17, "tempC": 15}}
        if r == "steelwork" and contact_ground:
            b = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 15}}
        out[r] = b
    return out


def document():
    """📄 The whole asset document."""
    regions = [r["id"] for r in THERMAL["regions"]]
    rig = {"kind": "mechanical", "root": "floor", "bones": [
        {"id": "floor", "parent": None, "title": T("Yard floor frame (massless, fixed root)", "Hallenboden-Bezug (masselos, feste Wurzel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}},
        {"id": "bearers", "parent": "floor", "title": T("Two squared softwood bearers 100 x 100 x 1000 mm (stay on the floor when the boiler is lifted)", "Zwei Nadelholz-Kanthölzer 100 x 100 x 1000 mm (bleiben beim Anheben am Boden liegen)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.98, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.2},
         "mass": {"kg": r6(M_BEARER), "comX": 0.48, "comY": 0.05, "distribution": "uniform", "source": "literature", "reference": "en-338"},
         "section": {"areaM2": 0.02, "secondMomentM4": r6(2 * BEARER_S ** 4 / 12), "extremeFibreM": 0.05, "material": "softwood", "title": T("Two squared timbers 100 x 100 mm", "Zwei Kanthölzer 100 x 100 mm")}},
        {"id": "boiler", "parent": "floor", "title": T("Cast-iron sectional boiler with jacket, doors, base channels and stubs (lifted by its two eye bolts)", "Guss-Gliederkessel mit Mantel, Türen, Grundrahmen und Stutzen (an seinen zwei Ringschrauben gehoben)"),
         "x": 0, "y": 0.10, "angleDeg": 0, "lengthM": 0.90, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.10, "maxM": 0.60},
         "mass": {"kg": r6(M_BOILER), "comX": r6(COM_BOILER[0]), "comY": r6(COM_BOILER[1] - 0.10), "distribution": "point", "source": "authored-estimate", "reference": "sectional-boiler-catalogues"}},
    ]}
    anchors = []
    for k, (x, z) in enumerate(((0.03, -BEARER_Z), (0.03, BEARER_Z), (0.93, -BEARER_Z), (0.93, BEARER_Z))):
        anchors.append({"id": f"bearer-ground-{k}", "bone": "bearers", "x": x, "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.10, "footprintWidthM": 0.10, "frictionCoefficient": 0.5,
                        "title": T(f"Bearer {'right' if z < 0 else 'left'} {'rear' if x < 0.5 else 'front'} end on the yard floor", f"Kantholz {'rechts' if z < 0 else 'links'} {'hinten' if x < 0.5 else 'vorn'} auf dem Hallenboden")})
    for k, (x, z) in enumerate(((0.16, -0.221), (0.16, 0.221), (0.84, -0.221), (0.84, 0.221))):
        anchors.append({"id": f"boiler-foot-{k}", "bone": "boiler", "x": x, "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.08, "footprintWidthM": 0.038, "frictionCoefficient": 0.4,
                        "title": T(f"Base channel {'right' if z < 0 else 'left'} {'rear' if x < 0.5 else 'front'} bearing face", f"Grundschiene {'rechts' if z < 0 else 'links'} {'hinten' if x < 0.5 else 'vorn'}, Auflagerfläche")})
    for k, x in enumerate(EYES):
        anchors.append({"id": f"eye-{k}", "bone": "boiler", "x": x, "y": r6(EYE_C + EYE_RI - 0.10), "z": 0, "kind": "support", "support": "pin",
                        "title": T(f"{'Rear' if k == 0 else 'Front'} M16 eye bolt: chain sling hook", f"{'Hintere' if k == 0 else 'Vordere'} Ringschraube M16: Haken des Kettengehänges")})
    anchors.append({"id": "flue-outlet", "bone": "boiler", "x": SPIGOT[0], "y": r6(SPIGOT[2] - 0.10), "z": 0, "kind": "emitter", "title": T("Open sawn end of the flue spigot", "Offenes gesägtes Ende des Abgasstutzens")})
    anchors.append({"id": "jacket-top", "bone": "boiler", "x": 0.45, "y": 0.85, "z": 0, "kind": "emitter", "title": T("Middle of the jacket top", "Mitte der Manteloberseite")})
    anchors.append({"id": "burner-flange", "bone": "boiler", "x": 0.972, "y": 0.60, "z": 0, "kind": "load", "title": T("Centre of the blanking plate where the replacement burner is bolted", "Mitte der Blindplatte, an der der Ersatzbrenner angeschraubt wird")})
    anchors.append({"id": "flow-connection", "bone": "boiler", "x": FLOW[0], "y": r6(FLOW[5] - 0.10), "z": FLOW[1], "kind": "load", "title": T("Flow stub, where the heating flow pipe is connected", "Vorlaufstutzen, an dem das Vorlaufrohr angeschlossen wird")})
    anchors.append({"id": "return-connection", "bone": "boiler", "x": RETURN[5], "y": r6(RETURN[0] - 0.10), "z": RETURN[1], "kind": "load", "title": T("Return stub, where the heating return pipe is connected", "Rücklaufstutzen, an dem das Rücklaufrohr angeschlossen wird")})
    stored_regions = binding(regions)
    feet = [f"boiler-foot-{k}" for k in range(4)]
    ground = [f"bearer-ground-{k}" for k in range(4)]
    service_loads = [
        {"id": "boiler-water", "kind": "carried", "title": T("Boiler water 39 l in the six sections", "Kesselwasser 39 l in den sechs Gliedern"), "bone": "boiler", "x": 0.50, "y": 0.415, "z": 0, "fxN": 0, "fyN": r6(-WATER_KG * 9.81), "source": "authored-estimate", "reference": "sectional-boiler-catalogues"},
        {"id": "replacement-burner", "kind": "equipment", "title": T("Replacement forced-draught burner 18 kg on the burner door", "Ersatz-Gebläsebrenner 18 kg an der Brennertür"), "anchor": "burner-flange", "fxN": 0, "fyN": r6(-18 * 9.81), "momentNm": r6(-18 * 9.81 * 0.17), "source": "authored-estimate", "reference": "sectional-boiler-catalogues"},
        {"id": "flow-pipe", "kind": "imposed", "title": T("Flow pipework hanging on the flow stub, 0.12 kN", "Vorlaufleitung am Vorlaufstutzen, 0,12 kN"), "anchor": "flow-connection", "fxN": 0, "fyN": -120, "source": "authored-estimate"},
        {"id": "return-pipe", "kind": "imposed", "title": T("Return pipework on the return stub, 0.12 kN", "Rücklaufleitung am Rücklaufstutzen, 0,12 kN"), "anchor": "return-connection", "fxN": 0, "fyN": -120, "source": "authored-estimate"},
    ]
    off = {"burner": 0, "heating": 0}
    poses = [
        {"id": "stored", "title": T("Stored on bearers", "Auf Kanthölzern gelagert"), "description": T(
            "Drained and capped, the boiler stands with its two base channels on two squared timbers in the hall of the component exchange; the burner is removed and its opening closed by the blanking plate.",
            "Entleert und verschlossen steht der Kessel mit seinen zwei Grundschienen auf zwei Kanthölzern in der Halle der Bauteilbörse; der Brenner ist abgebaut und seine Öffnung mit der Blindplatte verschlossen."),
         "bones": {}, "ground": "authored", "contacts": ground, "thermal": {"operating": off, "regions": stored_regions, "emitters": {"flue-gas": 0, "jacket-plume": 0}}, "loads": []},
        {"id": "lifted", "title": T("Lifted on the chain sling", "Am Kettengehänge angehoben"), "description": T(
            f"A two-leg chain sling hangs the {M_BOILER:.0f} kg boiler 0.40 m above its bearers by the two M16 eye bolts screwed into the top of the front and rear sections; the bearers stay on the floor.",
            f"Ein zweisträngiges Kettengehänge hält den {M_BOILER:.0f} kg schweren Kessel an den zwei Ringschrauben M16 im Kopf des vorderen und hinteren Glieds 0,40 m über seinen Kanthölzern; die Kanthölzer bleiben am Boden."),
         "bones": {"boiler": {"dy": 0.40}}, "ground": "authored", "contacts": ["eye-0", "eye-1"], "thermal": {"operating": off, "regions": binding(regions), "emitters": {"flue-gas": 0, "jacket-plume": 0}}, "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled, firing at design", "Wieder eingebaut, Nennbetrieb"), "description": T(
            "Set down on the boiler room floor, refilled and fired at 50 kW heat input by a replacement burner on the burner door: flow 90 degC, return 70 degC, flue gas leaves the spigot at about 200 degC; the bearers lie in front of it.",
            "Auf den Boden des Heizraums gesetzt, wieder befüllt und von einem Ersatzbrenner an der Brennertür mit 50 kW Wärmebelastung befeuert: Vorlauf 90 °C, Rücklauf 70 °C, das Abgas verlässt den Stutzen mit etwa 200 °C; die Kanthölzer liegen davor."),
         "bones": {"boiler": {"dy": -0.10}, "bearers": {"dx": 1.10}}, "ground": "authored", "contacts": feet, "thermal": {"operating": {"burner": 1, "heating": 1}, "regions": binding(regions, True), "emitters": {"flue-gas": 1, "jacket-plume": 1}}, "loads": service_loads},
        {"id": "reinstalled-off", "title": T("Reinstalled, filled and cold", "Wieder eingebaut, befüllt und kalt"), "description": T(
            "The same boiler room state with the burner off: the 39 l of water and the pipework load the sections, everything is at room temperature.",
            "Derselbe Zustand im Heizraum bei abgeschaltetem Brenner: die 39 l Wasser und die Leitungen belasten die Glieder, alles hat Raumtemperatur."),
         "bones": {"boiler": {"dy": -0.10}, "bearers": {"dx": 1.10}}, "ground": "authored", "contacts": feet, "thermal": {"operating": off, "regions": binding(regions, True), "emitters": {"flue-gas": 0, "jacket-plume": 0}}, "loads": service_loads},
    ]
    clips = [
        {"id": "lift", "title": T("Lift off the bearers", "Von den Kanthölzern abheben"), "description": T(
            "The chain sling takes the weight after two seconds and the crane lifts the boiler 0.40 m clear of its bearers in six seconds.",
            "Das Kettengehänge übernimmt nach zwei Sekunden das Gewicht, und der Kran hebt den Kessel in sechs Sekunden 0,40 m über seine Kanthölzer."),
         "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "stored", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}, {"t": 10, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 2.2} for a in ground] + [{"anchor": "eye-0", "fromS": 2, "toS": 10}, {"anchor": "eye-1", "fromS": 2, "toS": 10}],
         "phases": [{"id": "sling-on", "fromS": 0, "toS": 2, "title": T("Slinging", "Anschlagen")}, {"id": "hoist", "fromS": 2, "toS": 8, "title": T("Hoisting", "Heben")}, {"id": "hold", "fromS": 8, "toS": 10, "title": T("Holding", "Halten")}],
         "events": [{"t": 2.2, "kind": "contact-off", "title": T("Base channels leave the bearers", "Grundschienen verlassen die Kanthölzer"), "target": "boiler"}],
         "tracks": []},
        {"id": "heat-up", "title": T("Firing up after reinstallation", "Anfahren nach dem Wiedereinbau"), "description": T(
            "The refilled boiler starts cold; the burner lights after one minute and the sections, the jacket and the flue spigot warm up until the boiler water reaches 90/70 degC after 25 minutes.",
            "Der wieder befüllte Kessel startet kalt; der Brenner zündet nach einer Minute, und Glieder, Mantel und Abgasstutzen erwärmen sich, bis das Kesselwasser nach 25 Minuten 90/70 °C erreicht."),
         "durationS": 1800, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 60, "pose": "reinstalled", "ease": "step"}, {"t": 1800, "pose": "reinstalled", "ease": "step"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 1800} for a in feet],
         "phases": [{"id": "cold", "fromS": 0, "toS": 60, "title": T("Cold start", "Kaltstart")}, {"id": "warm-up", "fromS": 60, "toS": 1560, "title": T("Warming up", "Aufheizen")}, {"id": "design", "fromS": 1560, "toS": 1800, "title": T("Design temperatures", "Auslegungstemperaturen")}],
         "events": [{"t": 60, "kind": "ignition", "title": T("Burner lights", "Brenner zündet"), "target": "burner"}, {"t": 1560, "kind": "operating", "title": T("Flow reaches 90 degC", "Vorlauf erreicht 90 °C"), "target": "heating"}],
         "tracks": [{"target": "operating:burner", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 1}, {"t": 1800, "v": 1}]},
                    {"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 1560, "v": 1}, {"t": 1800, "v": 1}]},
                    {"target": "emitter:flue-gas", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 1}, {"t": 1800, "v": 1}]},
                    {"target": "emitter:jacket-plume", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 1560, "v": 1}, {"t": 1800, "v": 1}]}]},
    ]
    unavailable = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T(
        "At 1:500 the boiler on its bearers is 2.0 mm long, 1.2 mm wide and 2.1 mm high on paper, under the 3 mm a recognisable drawing needs, and the 60 mm burner door, the 50 mm base channels and the 52 mm capped stubs shrink to about 0.1 mm, below the printable feature; draw it at 1:200 or larger.",
        "Bei 1:500 ist der Kessel auf seinen Kanthölzern auf Papier 2,0 mm lang, 1,2 mm breit und 2,1 mm hoch, unter den 3 mm, die eine erkennbare Zeichnung braucht, und die 60 mm tiefe Brennertür, die 50 mm hohen Grundschienen und die 52 mm großen Verschlusskappen schrumpfen auf etwa 0,1 mm, unter das druckbare Merkmal; bei 1:200 oder größer zeichnen."),
        "limitingFeature": T("Burner door 60 mm deep", "Brennertür 60 mm tief"), "limitingSizeM": 0.06, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}
    asset = {
        "id": SLUG, "category": "components", "title": T("Used Cast-Iron Sectional Boiler", "Gebrauchter Guss-Gliederkessel"), "summary": SUMMARY,
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "boiler", "cast-iron", "sectional-boiler", "oil", "gas", "sheet-steel", "timber", "operating", "thermal"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": 1.05, "widthM": 1.01, "depthM": 0.62, "massKg": r6(M_TOTAL)},
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
    print(f"[DEBUG] boiler {M_BOILER:.2f} kg com {COM_BOILER[0]:.4f} {COM_BOILER[1]:.4f}; bearers {M_BEARER:.2f} kg; total {M_TOTAL:.2f} kg")
