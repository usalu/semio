"""🧵 Authoritative chisel of the used sewing machine: states every hand-chosen number and writes `🔣️.json` next to this file.

A 1980s electric free-arm household sewing machine of the German mid-range type, as a flat clearance sets it out for the buyer:
on a folded moving blanket on the hallway floor, the machine on the wooden base board of its hard carrying case, the hood lifted
off and standing behind it, the foot pedal with its cable taped in a coil in front, a scrap of fabric under the lowered presser
foot (rest = default pose `stored`). Carried shut by the hood handle by one person; back in use on a table with lamp and motor on.
World frame of the staffage: x ahead (towards the sewer), y up, z left of the asset (the handwheel end; the needle head is at -z).
Every bone rests with angle 0 and lies in the centre plane, so bone-local (u, v, w) = world (x - ox, y - oy, z).

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE / "🔣️.json"

G = 9.80665
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}
YB = 0.036
BLANKET = dict(x0=-0.40, x1=0.32, z=0.28, t=0.012)
TABLE = dict(x0=-0.44, x1=0.36, z=0.30, h=0.75, top=0.025, leg=0.045, inset=0.03, apron=0.09, rail=0.02)
HOOD = dict(x0=-0.385, x1=-0.145, z=0.225, y0=0.012, y1=0.362, wall=0.003, r=0.02)
HANDLE = dict(x0=-0.275, x1=-0.255, z=0.080, zi=0.062, h=0.032, bar=0.012)
PEDAL = dict(x0=0.150, x1=0.270, z0=-0.070, z1=0.025)
COIL = dict(x=0.21, z=-0.0225, y=0.0635, R=0.042, r=0.008)
HOOD_DX = 0.265
HOOD_DY = 0.024
CARRY_DY = 0.348 - TABLE["h"]
CLEAR = dict(dy=0.05, dx=0.7)
TRAVEL = 1.0
PEDAL_STOW = dict(dx=-0.215, dy=0.099)
PEDAL_USE = dict(dx=0.0, dy=-0.012 - TABLE["h"])
COIL_USE = dict(dx=-0.13, dy=-0.044)
NEEDLE_STROKE = 0.030
PRESSER_LIFT = 0.007
TREAD_PRESS = 3.0

ORIGIN = {
    "ground": (0.0, -TABLE["h"]), "table": (TABLE["x0"], 0.0), "blanket": (-0.40, 0.006), "carry-lift": (0.0, 0.0), "carry-travel": (0.0, 0.0), "case": (-0.12, 0.028),
    "machine": (-0.09, 0.0735), "handwheel": (-0.010, YB + 0.250), "needle-bar": (0.010, YB + 0.115), "take-up": (0.071, YB + 0.2475),
    "presser": (0.005, YB + 0.080), "fabric": (0.005, YB + 0.076), "hood-lift": (-0.12, 0.028), "hood-slide": (-0.12, 0.028), "hood": (-0.385, 0.187),
    "pedal-lift": (-0.12, 0.028), "pedal-slide": (-0.12, 0.028), "pedal": (0.15, 0.029), "tread": (0.266, 0.046),
    "cable-lift": (0.15, 0.029), "cable-slide": (0.15, 0.029), "cable": (0.21, 0.0635),
}

RHO = {"cast-aluminium": 2700.0, "chromed-zinc": 6700.0, "chromed-steel": 7850.0, "abs-ivory": 1050.0, "abs-brown": 1050.0, "pvc-cable": 1800.0,
       "vinyl-chipboard": 650.0, "rubber": 1200.0, "thread": 900.0, "cotton": 400.0, "felt-blanket": 134.0, "laminate-chipboard": 680.0, "beech-lacquered": 720.0}


def r6(v):
    """🔢 Rounds every float of a nested value to six decimals."""
    if isinstance(v, float):
        return float(f"{v:.6g}") if 0 < abs(v) < 0.01 else round(v, 6) + 0.0
    if isinstance(v, (list, tuple)):
        return [r6(x) for x in v]
    if isinstance(v, dict):
        return {k: r6(x) for k, x in v.items()}
    return v


def T(en, de):
    """🗣 A bilingual text."""
    return {"en": en, "de": de}


def loc(bone, x, y):
    """📍 World rest (x, y) to the (u, v) frame of a bone (every bone rests unrotated)."""
    ox, oy = ORIGIN[bone]
    return (x - ox, y - oy)


def arc(cx, cy, r, a0, a1, n):
    """◜ Points on a circular arc from angle a0 to a1 (degrees)."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def rrect(a0, b0, a1, b1, r, n=4, corners=(True, True, True, True)):
    """▢ A rectangle with rounded corners (counter-clockwise from the lower left); `corners` = lower left, lower right, upper right, upper left."""
    pts = []
    spec = [((a0 + r, b0 + r), 180, 270, corners[0], (a0, b0)), ((a1 - r, b0 + r), 270, 360, corners[1], (a1, b0)), ((a1 - r, b1 - r), 0, 90, corners[2], (a1, b1)), ((a0 + r, b1 - r), 90, 180, corners[3], (a0, b1))]
    for (cx, cy), s, e, rounded, sharp in spec:
        pts.extend(arc(cx, cy, r, s, e, n) if rounded and r > 0 else [sharp])
    return pts


def area(pts):
    """📐 Shoelace area of a simple polygon."""
    return abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2


def perimeter(pts):
    """📏 Perimeter of a closed polygon."""
    return sum(math.dist(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts)))


# ---------------------------------------------------------------- parts

class Part:
    """🧩 One physical part in world rest coordinates: it writes its 3D solid in the frame of its bone and its three 2D projections."""

    def __init__(self, pid, bone, kind, geo, material, region, tone, views=("elevation", "front", "plan"), hatch=None, flags=None, fp_bone=None, solid2d=False):
        self.id, self.bone, self.kind, self.geo, self.material, self.region, self.tone = pid, bone, kind, geo, material, region, tone
        self.views, self.hatch, self.flags, self.fp_bone, self.solid2d = views, hatch or {}, flags or {}, fp_bone, solid2d

    def solid(self):
        """🧊 The 3D solid in the rest frame of the bone."""
        g, b = self.geo, self.bone
        ox, oy = ORIGIN[b]
        base = {"id": self.id, "bone": b, "material": self.material, "role": "body", "tone": self.tone}
        if self.kind == "box":
            c = [((g["x0"] + g["x1"]) / 2) - ox, ((g["y0"] + g["y1"]) / 2) - oy, (g["z0"] + g["z1"]) / 2]
            base.update({"kind": "box", "size": [g["x1"] - g["x0"], g["y1"] - g["y0"], g["z1"] - g["z0"]], "transform": {"translation": c}})
        elif self.kind == "xy":
            base.update({"kind": "extrude", "plane": "xy", "profile": [[x - ox, y - oy] for x, y in g["pts"]], "from": g["z0"], "to": g["z1"]})
        elif self.kind == "zx":
            base.update({"kind": "extrude", "plane": "zx", "profile": [[z, x - ox] for x, z in g["pts"]], "from": g["y0"] - oy, "to": g["y1"] - oy})
            if g.get("holes"):
                base["holes"] = [[[z, x - ox] for x, z in h] for h in g["holes"]]
        elif self.kind == "yz":
            base.update({"kind": "extrude", "plane": "yz", "profile": [[y - oy, z] for y, z in g["pts"]], "from": g["x0"] - ox, "to": g["x1"] - ox})
        elif self.kind == "rev_y":
            base.update({"kind": "revolve", "profile": [list(p) for p in g["prof"]], "axis": [0, 1, 0], "origin": [g["xc"] - ox, -oy, g["zc"]]})
        elif self.kind == "rev_x":
            base.update({"kind": "revolve", "profile": [list(p) for p in g["prof"]], "axis": [1, 0, 0], "origin": [-ox, g["yc"] - oy, g["zc"]]})
        elif self.kind == "rev_z":
            base.update({"kind": "revolve", "profile": [list(p) for p in g["prof"]], "axis": [0, 0, 1], "origin": [g["xc"] - ox, g["yc"] - oy, 0.0]})
        elif self.kind == "torus":
            prof = [(g["y"] + g["r"] * math.sin(2 * math.pi * k / 16), g["R"] + g["r"] * math.cos(2 * math.pi * k / 16)) for k in range(16)]
            prof.append(prof[0])
            base.update({"kind": "revolve", "profile": [list(p) for p in prof], "axis": [0, 1, 0], "origin": [g["x"] - ox, -oy, g["z"]]})
        base.update(self.flags)
        return base

    def bounds(self):
        """📦 World axis-aligned bounds (x0, x1, y0, y1, z0, z1)."""
        g = self.geo
        if self.kind == "box":
            return g["x0"], g["x1"], g["y0"], g["y1"], g["z0"], g["z1"]
        if self.kind == "xy":
            xs, ys = [p[0] for p in g["pts"]], [p[1] for p in g["pts"]]
            return min(xs), max(xs), min(ys), max(ys), g["z0"], g["z1"]
        if self.kind == "zx":
            xs, zs = [p[0] for p in g["pts"]], [p[1] for p in g["pts"]]
            return min(xs), max(xs), g["y0"], g["y1"], min(zs), max(zs)
        if self.kind == "yz":
            ys, zs = [p[0] for p in g["pts"]], [p[1] for p in g["pts"]]
            return g["x0"], g["x1"], min(ys), max(ys), min(zs), max(zs)
        if self.kind in ("rev_y", "rev_x", "rev_z"):
            a = [p[0] for p in g["prof"]]
            rm = max(p[1] for p in g["prof"])
            if self.kind == "rev_y":
                return g["xc"] - rm, g["xc"] + rm, min(a), max(a), g["zc"] - rm, g["zc"] + rm
            if self.kind == "rev_x":
                return min(a), max(a), g["yc"] - rm, g["yc"] + rm, g["zc"] - rm, g["zc"] + rm
            return g["xc"] - rm, g["xc"] + rm, g["yc"] - rm, g["yc"] + rm, min(a), max(a)
        if self.kind == "torus":
            o = g["R"] + g["r"]
            return g["x"] - o, g["x"] + o, g["y"] - g["r"], g["y"] + g["r"], g["z"] - o, g["z"] + o

    def outline(self, view):
        """✏️ The filled outline of the part in one view: elevation in world (x, y), front (z, y), plan (x, z)."""
        g = self.geo
        x0, x1, y0, y1, z0, z1 = self.bounds()
        if self.kind == "box" or (self.kind == "zx" and view != "plan") or (self.kind == "yz" and view != "front") or (self.kind == "xy" and view != "elevation"):
            return {"elevation": rect_pts(x0, y0, x1, y1), "front": rect_pts(z0, y0, z1, y1), "plan": rect_pts(x0, z0, x1, z1)}[view]
        if self.kind == "xy":
            return list(g["pts"])
        if self.kind == "zx":
            return list(g["pts"])
        if self.kind == "yz":
            return [(z, y) for y, z in g["pts"]]
        if self.kind in ("rev_y", "rev_x", "rev_z"):
            prof = g["prof"]
            axis_view = {"rev_y": "plan", "rev_x": "front", "rev_z": "elevation"}[self.kind]
            if view == axis_view:
                rm = max(p[1] for p in prof)
                c = {"rev_y": (g.get("xc"), g.get("zc")), "rev_x": (g.get("zc"), g.get("yc")), "rev_z": (g.get("xc"), g.get("yc"))}[self.kind]
                return ("circle", c[0], c[1], rm)
            c = {("rev_y", "elevation"): g.get("xc"), ("rev_y", "front"): g.get("zc"), ("rev_x", "elevation"): g.get("yc"), ("rev_x", "plan"): g.get("zc"), ("rev_z", "front"): g.get("yc"), ("rev_z", "plan"): g.get("xc")}[(self.kind, view)]
            pts = mirrored(prof, c)
            if (self.kind, view) in (("rev_y", "elevation"), ("rev_y", "front"), ("rev_z", "plan")):
                return [(b, a) for a, b in pts]
            return pts
        if self.kind == "torus":
            if view == "plan":
                return ("ring", g["x"], g["z"], g["R"] + g["r"], g["R"] - g["r"])
            c = g["x"] if view == "elevation" else g["z"]
            o = g["R"] + g["r"]
            right = arc(c + g["R"], g["y"], g["r"], -90, 90, 6)
            left = arc(c - g["R"], g["y"], g["r"], 90, 270, 6)
            return right + left


def rect_pts(a0, b0, a1, b1):
    """▭ The corners of a rectangle."""
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def mirrored(prof, c):
    """🪞 A revolve profile [(axial, radial)] as the closed outline of the whole body: (axial, c + r) out and (axial, c - r) back."""
    pts = [p for p in prof if p[1] > 0]
    out = [(a, c + r) for a, r in pts] + [(a, c - r) for a, r in reversed(pts)]
    return out


# ---------------------------------------------------------------- the parts of each scale

def bed_profile(scale):
    """🛏 The bed cross-section (x, y) with its rounded front edge."""
    if scale == 50:
        return [(-0.090, YB), (0.090, YB), (0.090, YB + 0.060)] + arc(0.075, YB + 0.060, 0.015, 0, 90, 4)[1:] + [(-0.085, YB + 0.075), (-0.090, YB + 0.070)]
    if scale == 100:
        return [(-0.090, YB), (0.090, YB), (0.090, YB + 0.062), (0.077, YB + 0.075), (-0.090, YB + 0.075)]
    return [(-0.090, YB), (0.090, YB), (0.090, YB + 0.075), (-0.090, YB + 0.075)]


def arm_profile(x0, x1, y0, y1, r, scale):
    """🦾 A housing cross-section (x, y) rounded at its two upper edges."""
    if scale == 200:
        return rect_pts(x0, y0, x1, y1)
    n = 4 if scale == 50 else 2
    return [(x0, y0), (x1, y0)] + arc(x1 - r, y1 - r, r, 0, 90, n)[0:] + arc(x0 + r, y1 - r, r, 90, 180, n)


def hood_ring(inset, r, scale):
    """🧳 Plan outline (x, z) of the hood, `inset` inwards."""
    h = HOOD
    rr = max(r - inset, 0.0)
    if scale == 200:
        return rect_pts(h["x0"] + inset, -h["z"] + inset, h["x1"] - inset, h["z"] - inset)
    return rrect(h["x0"] + inset, -h["z"] + inset, h["x1"] - inset, h["z"] - inset, rr, 3 if scale == 50 else 2)


def blanket_profile(scale):
    """🧺 The folded blanket in elevation: four 3 mm layers with the rounded fold at the front edge."""
    b = BLANKET
    r = b["t"] / 2
    if scale == 200:
        return rect_pts(b["x0"], 0.0, b["x1"], b["t"])
    n = 6 if scale == 50 else 3
    return [(b["x0"], 0.0)] + arc(b["x1"] - r, r, r, -90, 90, n) + [(b["x0"], b["t"])]


def table_legs():
    """🦵 The four legs of the sewing table as (id, x0, x1, z0, z1), set in from the corners of the top."""
    t = TABLE
    xs = {"f": (t["x1"] - t["inset"] - t["leg"], t["x1"] - t["inset"]), "r": (t["x0"] + t["inset"], t["x0"] + t["inset"] + t["leg"])}
    zs = {"l": (t["z"] - t["inset"] - t["leg"], t["z"] - t["inset"]), "r": (-t["z"] + t["inset"], -t["z"] + t["inset"] + t["leg"])}
    return [(f"{a}{c}", xs[a][0], xs[a][1], zs[c][0], zs[c][1]) for a in "fr" for c in "lr"]


def table_parts(scale):
    """🪵 The sewing table the machine is set out on and sewn at: laminate top, four square beech legs and the apron rails between them (30 mm rails at 1:200)."""
    t = TABLE
    rail = t["rail"] if scale != 200 else 0.030
    legs = {lid: (x0, x1, z0, z1) for lid, x0, x1, z0, z1 in table_legs()}
    P = [Part("table-top", "table", "box", dict(x0=t["x0"], x1=t["x1"], y0=-t["top"], y1=0.0, z0=-t["z"], z1=t["z"]), "laminate-chipboard", "table-top", 0.6, flags={"sheet": True} if scale == 200 else {})]
    for lid, (x0, x1, z0, z1) in legs.items():
        P.append(Part(f"table-leg-{lid}", "table", "box", dict(x0=x0, x1=x1, y0=-t["h"], y1=-t["top"], z0=z0, z1=z1), "beech-lacquered", "table-frame", 0.5))
    y0, y1 = -t["top"] - t["apron"], -t["top"]
    for side, lid in (("left", "fl"), ("right", "fr")):
        z0, z1 = legs[lid][2], legs[lid][3]
        zo = z1 if side == "left" else z0
        P.append(Part(f"table-apron-{side}", "table", "box", dict(x0=legs["rl"][1], x1=legs["fl"][0], y0=y0, y1=y1, z0=min(zo, zo - rail if side == "left" else zo + rail), z1=max(zo, zo - rail if side == "left" else zo + rail)), "beech-lacquered", "table-frame", 0.5))
    for end, lid in (("front", "fl"), ("rear", "rl")):
        x0, x1 = legs[lid][0], legs[lid][1]
        xo = x1 if end == "front" else x0
        P.append(Part(f"table-apron-{end}", "table", "box", dict(x0=min(xo, xo - rail if end == "front" else xo + rail), x1=max(xo, xo - rail if end == "front" else xo + rail), y0=y0, y1=y1, z0=legs["fr"][3], z1=legs["fl"][2]), "beech-lacquered", "table-frame", 0.5))
    return P


def parts(scale):
    """🧩 The individually authored parts of one scale (world rest coordinates, y = 0 on the table top)."""
    s50, s100, s200 = scale == 50, scale == 100, scale == 200
    P = table_parts(scale)
    b = BLANKET
    P.append(Part("blanket", "blanket", "xy", {"pts": blanket_profile(scale), "z0": -b["z"], "z1": b["z"]}, "felt-blanket", "blanket", 0.3, hatch={"plan": "quilt"} if s50 else {}, flags={} if s50 else {"sheet": True}))
    if s50:
        P.append(Part("base-board", "case", "box", dict(x0=-0.12, x1=0.12, y0=0.020, y1=YB, z0=-0.225, z1=0.225), "vinyl-chipboard", "base-board", 0.55))
        for fid, sx, sz in (("fl", 1, 1), ("fr", 1, -1), ("rl", -1, 1), ("rr", -1, -1)):
            P.append(Part(f"base-foot-{fid}", "case", "rev_y", {"xc": sx * 0.095, "zc": sz * 0.20, "prof": [(0.012, 0.0), (0.012, 0.010), (0.020, 0.010), (0.020, 0.0)]}, "rubber", "base-board", 0.15))
    else:
        P.append(Part("base-board", "case", "box", dict(x0=-0.12, x1=0.12, y0=0.012, y1=YB, z0=-0.225, z1=0.225), "vinyl-chipboard", "base-board", 0.55))
    P.append(Part("bed", "machine", "xy", {"pts": bed_profile(scale), "z0": -0.200, "z1": 0.185}, "cast-aluminium", "housing", 0.85))
    P.append(Part("pillar", "machine", "box", dict(x0=-0.080, x1=0.070, y0=YB + 0.075, y1=YB + 0.215, z0=0.060, z1=0.185), "cast-aluminium", "housing", 0.82))
    P.append(Part("arm", "machine", "xy", {"pts": arm_profile(-0.070, 0.050, YB + 0.215, YB + 0.290, 0.020, scale), "z0": -0.110, "z1": 0.185}, "cast-aluminium", "housing", 0.86))
    head_z = {50: -0.192, 100: -0.185, 200: -0.170}[scale]
    P.append(Part("head", "machine", "xy", {"pts": arm_profile(-0.075, 0.065, YB + 0.125, YB + 0.290, 0.018, scale), "z0": head_z, "z1": -0.105}, "cast-aluminium", "housing", 0.84))
    P.append(Part("face-cover", "machine", "xy", {"pts": arm_profile(-0.075, 0.065, YB + 0.125, YB + 0.290, 0.018, scale), "z0": -0.200, "z1": head_z}, "cast-aluminium", "lamp-cover", 0.9))
    if s200:
        P.append(Part("handwheel", "handwheel", "rev_z", {"xc": -0.010, "yc": YB + 0.250, "prof": [(0.175, 0.0), (0.175, 0.035), (0.205, 0.035), (0.205, 0.0)]}, "chromed-zinc", "handwheel", 0.7, fp_bone="machine"))
        P.append(Part("dials", "machine", "box", dict(x0=0.055, x1=0.085, y0=YB + 0.080, y1=YB + 0.190, z0=0.0975, z1=0.1475), "abs-brown", "dials", 0.3))
    else:
        prof = [(0.185, 0.0), (0.185, 0.030), (0.188, 0.035), (0.203, 0.035), (0.205, 0.031), (0.205, 0.0)] if s50 else [(0.185, 0.0), (0.185, 0.035), (0.205, 0.035), (0.205, 0.0)]
        P.append(Part("handwheel", "handwheel", "rev_z", {"xc": -0.010, "yc": YB + 0.250, "prof": prof}, "chromed-zinc", "handwheel", 0.7, fp_bone="machine"))
        sd = [(0.066, 0.0), (0.066, 0.025), (0.080, 0.025), (0.085, 0.020), (0.085, 0.0)] if s50 else [(0.066, 0.0), (0.066, 0.025), (0.085, 0.025), (0.085, 0.0)]
        P.append(Part("stitch-dial", "machine", "rev_x", {"yc": YB + 0.165, "zc": 0.1225, "prof": sd}, "abs-brown", "dials", 0.3))
        ld = [(0.066, 0.0), (0.066, 0.016), (0.079, 0.016), (0.082, 0.013), (0.082, 0.0)] if s50 else [(0.066, 0.0), (0.066, 0.016), (0.085, 0.016), (0.085, 0.0)]
        P.append(Part("length-dial", "machine", "rev_x", {"yc": YB + 0.105, "zc": 0.1225, "prof": ld}, "abs-brown", "dials", 0.3))
        if s50:
            P.append(Part("bobbin-winder", "machine", "rev_y", {"xc": -0.030, "zc": 0.155, "prof": [(YB + 0.290, 0.0), (YB + 0.290, 0.010), (YB + 0.302, 0.010), (YB + 0.302, 0.0)]}, "abs-brown", "dials", 0.3))
    if s50:
        sp = [(YB + 0.290, 0.0), (YB + 0.290, 0.016), (YB + 0.294, 0.016), (YB + 0.294, 0.013), (YB + 0.326, 0.013), (YB + 0.326, 0.016), (YB + 0.330, 0.016), (YB + 0.330, 0.0)]
    else:
        sp = [(YB + 0.290, 0.0), (YB + 0.290, 0.016), (YB + 0.330, 0.016), (YB + 0.330, 0.0)]
    P.append(Part("spool", "machine", "rev_y", {"xc": -0.020, "zc": 0.090, "prof": sp}, "thread", "thread-spool", 0.45))
    if s50:
        P.append(Part("needle-bar", "needle-bar", "rev_y", {"xc": 0.010, "zc": -0.150, "prof": [(YB + 0.115, 0.0), (YB + 0.115, 0.004), (YB + 0.175, 0.004), (YB + 0.175, 0.0)]}, "chromed-steel", "needle-foot", 0.75))
        P.append(Part("needle-clamp", "needle-bar", "box", dict(x0=0.004, x1=0.016, y0=YB + 0.110, y1=YB + 0.120, z0=-0.156, z1=-0.144), "chromed-steel", "needle-foot", 0.7))
        P.append(Part("presser-bar", "presser", "rev_y", {"xc": -0.006, "zc": -0.150, "prof": [(YB + 0.092, 0.0), (YB + 0.092, 0.004), (YB + 0.180, 0.004), (YB + 0.180, 0.0)]}, "chromed-steel", "needle-foot", 0.75))
        P.append(Part("presser-shank", "presser", "box", dict(x0=-0.010, x1=0.000, y0=YB + 0.084, y1=YB + 0.096, z0=-0.156, z1=-0.144), "chromed-steel", "needle-foot", 0.7))
        P.append(Part("presser-foot", "presser", "xy", {"pts": [(-0.014, YB + 0.077), (0.018, YB + 0.077), (0.026, YB + 0.084), (-0.014, YB + 0.084)], "z0": -0.160, "z1": -0.140}, "chromed-steel", "needle-foot", 0.72))
        P.append(Part("take-up-lever", "take-up", "box", dict(x0=0.065, x1=0.077, y0=YB + 0.240, y1=YB + 0.255, z0=-0.170, z1=-0.160), "chromed-steel", "needle-foot", 0.75))
    elif s100:
        P.append(Part("needle-bar", "needle-bar", "rev_y", {"xc": 0.010, "zc": -0.150, "prof": [(YB + 0.115, 0.0), (YB + 0.115, 0.0075), (YB + 0.175, 0.0075), (YB + 0.175, 0.0)]}, "chromed-steel", "needle-foot", 0.75))
        P.append(Part("presser-bar", "presser", "rev_y", {"xc": -0.006, "zc": -0.150, "prof": [(YB + 0.092, 0.0), (YB + 0.092, 0.0075), (YB + 0.180, 0.0075), (YB + 0.180, 0.0)]}, "chromed-steel", "needle-foot", 0.75))
        P.append(Part("presser-foot", "presser", "box", dict(x0=-0.014, x1=0.026, y0=YB + 0.077, y1=YB + 0.092, z0=-0.1575, z1=-0.1425), "chromed-steel", "needle-foot", 0.72))
    else:
        P.append(Part("presser-foot", "presser", "box", dict(x0=-0.015, x1=0.025, y0=YB + 0.077, y1=YB + 0.125, z0=-0.165, z1=-0.135), "chromed-steel", "needle-foot", 0.72))
    if scale in (50, 100):
        P.append(Part("needle", "needle-bar", "rev_y", {"xc": 0.010, "zc": -0.150, "prof": [(YB + 0.087, 0.0), (YB + 0.087, 0.0009), (YB + 0.125, 0.0009), (YB + 0.125, 0.0)]}, "chromed-steel", "needle-foot", 0.8, flags={"slender": True}, views=()))
    P.append(Part("fabric", "fabric", "box", dict(x0=-0.060, x1=0.070, y0=YB + 0.075, y1=YB + 0.077, z0=-0.198, z1=-0.088), "cotton", "fabric", 0.4, flags={"sheet": True}))
    h = HOOD
    P.append(Part("hood-walls", "hood", "zx", {"pts": hood_ring(0.0, h["r"], scale), "holes": [hood_ring(h["wall"], h["r"], scale)], "y0": h["y0"], "y1": h["y1"] - h["wall"]}, "abs-ivory", "hood", 0.78, views=("elevation", "front"), solid2d=True))
    P.append(Part("hood-top", "hood", "zx", {"pts": hood_ring(0.0, h["r"], scale), "y0": h["y1"] - h["wall"], "y1": h["y1"]}, "abs-ivory", "hood", 0.8, flags={"sheet": True}, solid2d=True))
    hd = HANDLE
    if s200:
        P.append(Part("hood-handle", "hood", "box", dict(x0=-0.280, x1=-0.250, y0=h["y1"], y1=h["y1"] + hd["h"], z0=-hd["z"], z1=hd["z"]), "abs-brown", "hood", 0.3, solid2d=True))
    else:
        bar = hd["bar"] if s50 else 0.015
        zi = hd["zi"] if s50 else 0.065
        prof = [(h["y1"], -hd["z"]), (h["y1"], -zi), (h["y1"] + hd["h"] - bar, -zi), (h["y1"] + hd["h"] - bar, zi), (h["y1"], zi), (h["y1"], hd["z"]), (h["y1"] + hd["h"], hd["z"]), (h["y1"] + hd["h"], -hd["z"])]
        P.append(Part("hood-handle", "hood", "yz", {"pts": prof, "x0": hd["x0"], "x1": hd["x1"]}, "abs-brown", "hood", 0.3, solid2d=True))
    pd = PEDAL
    if s50:
        P.append(Part("pedal-housing", "pedal", "xy", {"pts": [(pd["x0"], 0.012), (pd["x1"], 0.012), (pd["x1"], 0.046), (pd["x0"], 0.034)], "z0": pd["z0"], "z1": pd["z1"]}, "abs-brown", "pedal", 0.25))
        P.append(Part("tread", "tread", "xy", {"pts": [(0.154, 0.040), (0.266, 0.046), (0.268, 0.056), (0.152, 0.051)], "z0": pd["z0"] + 0.003, "z1": pd["z1"] - 0.003}, "abs-brown", "pedal", 0.35, hatch={"plan": "ribs"}))
    else:
        P.append(Part("pedal-housing", "pedal", "xy", {"pts": [(pd["x0"], 0.012), (pd["x1"], 0.012), (pd["x1"], 0.056), (pd["x0"], 0.051)], "z0": pd["z0"], "z1": pd["z1"]}, "abs-brown", "pedal", 0.28))
    c = COIL
    if s200:
        P.append(Part("cable-coil", "cable", "rev_y", {"xc": c["x"], "zc": c["z"], "prof": [(0.0555, 0.0), (0.0555, 0.050), (0.0855, 0.050), (0.0855, 0.0)]}, "pvc-cable", "cable", 0.15))
    else:
        P.append(Part("cable-coil", "cable", "torus", dict(x=c["x"], z=c["z"], y=c["y"], R=c["R"], r=c["r"]), "pvc-cable", "cable", 0.15))
    return P


# ---------------------------------------------------------------- 2D

def shape(sid, bone, role, z, tone, geom, region=None, material=None, hatch=None, filled=True, solid=None):
    """✏️ One 2D shape with its line-hierarchy role, paint order and region."""
    s = {"id": sid, "bone": bone, "role": role, "z": round(z, 4), "filled": filled, "tone": tone}
    s.update(geom)
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if solid:
        s["solid"] = solid
    return s


def poly(points, closed=True):
    """⬠ A poly geometry."""
    return {"kind": "poly", "points": [[float(a), float(b)] for a, b in points], "closed": closed}


def lines(segments):
    """〰 A path of straight strokes."""
    return {"kind": "path", "d": " ".join(f"M {a[0]:.5f} {a[1]:.5f} L {b[0]:.5f} {b[1]:.5f}" for a, b in segments)}


def polyline_path(chains):
    """〰 A path of open polylines."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{p[0]:.5f} {p[1]:.5f}" for p in ch) for ch in chains)}


def ring_path(cx, cy, ro, ri):
    """⭕ An annulus as one even-odd path."""
    def circ(r):
        return f"M {cx + r:.5f} {cy:.5f} A {r:.5f} {r:.5f} 0 1 0 {cx - r:.5f} {cy:.5f} A {r:.5f} {r:.5f} 0 1 0 {cx + r:.5f} {cy:.5f} Z"
    return {"kind": "path", "d": circ(ro) + " " + circ(ri)}


def nearness(part, view):
    """📷 How near a part is to the viewer of a view (larger is nearer)."""
    x0, x1, y0, y1, z0, z1 = part.bounds()
    return {"elevation": -z0, "front": x1, "plan": y1}[view]


def part_shape(part, view, z):
    """✏️ The filled shape of a part in one view."""
    o = part.outline(view)
    bone = part.bone if (view == "elevation" or part.fp_bone is None) else part.fp_bone
    hatch = part.hatch.get(view)
    if view == "elevation":
        ox, oy = ORIGIN[part.bone]
        if isinstance(o, tuple) and o[0] == "circle":
            geom = {"kind": "ellipse", "cx": o[1] - ox, "cy": o[2] - oy, "rx": o[3], "ry": o[3]}
        else:
            geom = poly([(x - ox, y - oy) for x, y in o])
        return shape(part.id, bone, "body", z, part.tone, geom, part.region, part.material, hatch)
    if part.solid2d:
        ox, oy = ORIGIN[part.bone]
        x0, x1, y0, y1, z0, z1 = part.bounds()
        geom = poly([(x - ox, y - oy) for x, y in rect_pts(x0, y0, x1, y1)])
        return shape(part.id, bone, "body", z, part.tone, geom, part.region, part.material, hatch, solid={"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1})
    if view == "front":
        o = ("circle", o[1], o[2] + TABLE["h"], o[3]) if isinstance(o, tuple) else [(a, b + TABLE["h"]) for a, b in o]
    if isinstance(o, tuple) and o[0] == "circle":
        geom = {"kind": "ellipse", "cx": o[1], "cy": o[2], "rx": o[3], "ry": o[3]}
    elif isinstance(o, tuple) and o[0] == "ring":
        geom = ring_path(o[1], o[2], o[3], o[4])
    else:
        geom = poly(o)
    return shape(part.id, bone, "body", z, part.tone, geom, part.region, part.material, hatch)


def details(scale):
    """🖊 The detail and texture strokes of one scale per view: traces of use, seams, controls (coordinates with y = 0 on the table top; elevation converted per bone, front lifted to the floor datum)."""
    E, F, Pl = [], [], []
    s50, s100 = scale == 50, scale == 100

    def e(sid, bone, role, z, tone, segs, region=None, chains=False):
        ox, oy = ORIGIN[bone]
        if chains:
            g = polyline_path([[(x - ox, y - oy) for x, y in ch] for ch in segs])
        else:
            g = lines([((a[0] - ox, a[1] - oy), (b[0] - ox, b[1] - oy)) for a, b in segs])
        E.append(shape(sid, bone, role, z, tone, g, region, filled=False))

    def f(sid, bone, role, z, tone, segs, region=None, chains=False):
        segs = [[(a, b + TABLE["h"]) for a, b in seg] for seg in segs]
        F.append(shape(sid, bone, role, z, tone, polyline_path(segs) if chains else lines(segs), region, filled=False))

    def p(sid, bone, role, z, tone, segs, region=None, chains=False):
        Pl.append(shape(sid, bone, role, z, tone, polyline_path(segs) if chains else lines(segs), region, filled=False))

    nz = -0.150
    if s50 or s100:
        e("needle-line", "needle-bar", "detail", 0.1505, 0.2, [((0.010, YB + 0.087), (0.010, YB + 0.125))], "needle-foot")
        f("needle-line", "needle-bar", "detail", 0.0115, 0.2, [((nz, YB + 0.087), (nz, YB + 0.125))], "needle-foot")
        e("thread-path", "machine", "texture" if s50 else "detail", 0.165, 0.3, [[(-0.020, YB + 0.318), (-0.040, YB + 0.300), (0.071, YB + 0.300), (0.071, YB + 0.262), (0.068, YB + 0.230), (0.012, YB + 0.128)]], "thread-spool", chains=True)
        f("bed-seam", "machine", "detail", 0.091, 0.3, [((-0.040, YB + 0.004), (-0.040, YB + 0.074))], "housing")
        p("bed-seam", "machine", "detail", YB + 0.0751, 0.3, [((-0.088, -0.040), (0.088, -0.040))], "housing")
        e("hood-latches", "hood", "detail", 0.226, 0.3, [((-0.385, 0.040), (-0.375, 0.040)), ((-0.155, 0.040), (-0.145, 0.040))], "hood")
        f("hood-crack", "hood", "detail", -0.1449, 0.3, [[(-0.225, 0.300), (-0.205, 0.318), (-0.212, 0.334), (-0.196, 0.352)]], "hood", chains=True)
        f("base-edge", "case", "detail", 0.1201, 0.3, [((-0.225, 0.028), (0.225, 0.028))], "base-board")
    if s50:
        e("face-cover-screw", "machine", "detail", 0.2001, 0.3, [((-0.006, YB + 0.270), (0.006, YB + 0.270)), ((0.0, YB + 0.264), (0.0, YB + 0.276))], "lamp-cover")
        e("face-cover-hinge", "machine", "detail", 0.2001, 0.3, [((-0.068, YB + 0.140), (-0.068, YB + 0.275))], "lamp-cover")
        e("lamp-window", "machine", "detail", 0.2001, 0.3, [((-0.030, YB + 0.131), (0.030, YB + 0.131))], "lamp-cover")
        e("yellowing", "machine", "texture", 0.2002, 0.3, [((-0.060, YB + 0.282 - 0.008 * k), (0.050, YB + 0.282 - 0.008 * k)) for k in range(3)], "lamp-cover")
        e("blanket-layers", "blanket", "texture", 0.2801, 0.2, [((BLANKET["x0"], 0.003 * k), (BLANKET["x0"] + 0.06, 0.003 * k)) for k in (1, 2, 3)], "blanket")
        e("price-tag", "hood", "texture", 0.2261, 0.9, [((-0.330, 0.200), (-0.290, 0.200)), ((-0.290, 0.200), (-0.290, 0.240)), ((-0.290, 0.240), (-0.330, 0.240)), ((-0.330, 0.240), (-0.330, 0.200)), ((-0.325, 0.220), (-0.295, 0.220))], "hood")
        e("tread-ribs", "tread", "texture", 0.0701, 0.3, [((0.165 + 0.02 * k, 0.0505 + 0.00085 * k + 0.0012), (0.170 + 0.02 * k, 0.0510 + 0.00085 * k + 0.0012)) for k in range(5)], "pedal")
        e("coil-tape", "cable", "texture", 0.0726, 0.9, [((0.170, 0.0555), (0.170, 0.0715)), ((0.174, 0.0555), (0.174, 0.0715))], "cable")
        f("worn-paint", "machine", "texture", 0.0901, 0.6, [((-0.150 + 0.008 * k, YB + 0.064), (-0.144 + 0.008 * k, YB + 0.071)) for k in range(12)], "housing")
        f("scratches", "machine", "texture", 0.0901, 0.5, [((0.010, YB + 0.030), (0.060, YB + 0.038)), ((0.030, YB + 0.020), (0.070, YB + 0.026))], "housing")
        f("reverse-lever", "machine", "detail", 0.0702, 0.3, [((0.095, YB + 0.130), (0.150, YB + 0.130))], "dials")
        f("dial-marks", "machine", "texture", 0.0851, 0.9, [((0.1225, YB + 0.165), (0.1225, YB + 0.185)), ((0.1225, YB + 0.105), (0.1225, YB + 0.118))], "dials")
        f("take-up-slot", "machine", "detail", 0.0651, 0.3, [((-0.165, YB + 0.200), (-0.165, YB + 0.270))], "housing")
        f("arm-seam", "machine", "detail", 0.0501, 0.3, [((-0.105, YB + 0.218), (0.060, YB + 0.218))], "housing")
        f("handwheel-knurl", "machine", "texture", 0.0251, 0.3, [((0.188 + 0.004 * k, YB + 0.218), (0.188 + 0.004 * k, YB + 0.282)) for k in range(5)], "handwheel")
        f("sticker", "hood", "texture", -0.1448, 0.95, [((0.120, 0.333), (0.180, 0.333)), ((0.180, 0.333), (0.180, 0.355)), ((0.180, 0.355), (0.120, 0.355)), ((0.120, 0.355), (0.120, 0.333)), ((0.128, 0.344), (0.172, 0.344))], "hood")
        f("blanket-fold", "blanket", "texture", 0.3201, 0.2, [((-0.28, 0.003 * k), (0.28, 0.003 * k)) for k in (1, 2, 3)], "blanket")
        f("coil-turns", "cable", "texture", 0.2521, 0.3, [((-0.060, 0.0600), (0.015, 0.0600)), ((-0.060, 0.0670), (0.015, 0.0670))], "cable")
        p("arm-seam", "machine", "detail", YB + 0.2901, 0.3, [((0.030, -0.105), (0.030, 0.180))], "housing")
        p("pedal-cable-entry", "pedal", "detail", 0.0461, 0.3, [((0.270, -0.030), (0.285, -0.030))], "pedal")
        p("scuffs", "case", "texture", YB + 0.0001, 0.4, [((0.100, 0.205), (0.115, 0.215)), ((-0.110, -0.215), (-0.095, -0.205))], "base-board")
        p("coil-tape", "cable", "texture", 0.0716, 0.9, [((0.162, -0.035), (0.180, -0.035)), ((0.162, -0.028), (0.180, -0.028))], "cable")
        p("hood-sticker", "hood", "texture", 0.3621, 0.95, [((-0.340, 0.120), (-0.300, 0.120)), ((-0.300, 0.120), (-0.300, 0.180)), ((-0.300, 0.180), (-0.340, 0.180)), ((-0.340, 0.180), (-0.340, 0.120))], "hood")
    return E, F, Pl


def geometry(scale):
    """🖼 The individually drawn elevation, front and plan of one scale."""
    out = {"elevation": [], "front": [], "plan": []}
    for part in parts(scale):
        for view in part.views:
            out[view].append(part_shape(part, view, nearness(part, view)))
    E, F, Pl = details(scale)
    out["elevation"] += E
    out["front"] += F
    out["plan"] += Pl
    for view in out:
        out[view].sort(key=lambda s: s["z"])
    return out


# ---------------------------------------------------------------- 3D lists

MEMBER_OF = {"blanket": "blanket", "base-board": "case", "bed": "machine", "pillar": "machine", "arm": "machine", "head": "machine", "hood-walls": "hood", "hood-top": "hood", "hood-handle": "hood", "pedal-housing": "pedal"}


def mass_marker(sid, bone, at, r, material="chromed-steel"):
    """⚖ A centre-of-mass marker (world rest point)."""
    ox, oy = ORIGIN[bone]
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [at[0] - ox, at[1] - oy, at[2]]}, "member": bone, "part": "mass"}


def joint_box(sid, bone, member, c, s, material="chromed-steel"):
    """🔩 A joint solid: a cube of side `s` at the world rest point `c`."""
    ox, oy = ORIGIN[bone]
    return {"id": sid, "kind": "box", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "size": [s, s, s], "transform": {"translation": [c[0] - ox, c[1] - oy, c[2]]}, "member": member, "part": "joint"}


def support_disc(aid, scale):
    """🛞 A support pad on a floor anchor under a leg of the table."""
    a = next(x for x in ANCHORS if x["id"] == aid)
    f = FEATURE[scale]
    return {"id": f"support-{aid}", "kind": "revolve", "bone": "table", "material": "rubber", "role": "ground", "tone": 0.2, "profile": [[0.0, 0.0], [0.0, f], [f, f], [f, 0.0]], "axis": [0, 1, 0], "origin": [a["x"], a["y"], a["z"]], "member": "table", "part": "support", "anchor": aid}


def vent_patch(scale):
    """💨 The outlet patch of the motor vent on the rear of the pillar (emitter solid, D2)."""
    t = {50: 0.008, 100: 0.015, 200: 0.030}[scale]
    w = max(0.030, FEATURE[scale])
    x0 = -0.080 - t / 2
    ox, oy = ORIGIN["machine"]
    return {"id": "motor-vent-outlet", "kind": "box", "bone": "machine", "material": "cast-aluminium", "role": "detail", "tone": 0.4, "size": [t, w, w], "transform": {"translation": [x0 + t / 2 - ox, YB + 0.150 - oy, 0.120]}, "region": "housing", "tags": ["emitter", "motor-vent"]}


def lists(scale):
    """🧊 Body, thermal and structure lists of one scale."""
    body = [p.solid() for p in parts(scale)]
    thermal = [{"id": f"th-{s['id']}", "of": s["id"], "region": p.region} for s, p in zip(body, parts(scale))]
    thermal.append(vent_patch(scale))
    f = FEATURE[scale]
    structure = []
    for s in body:
        if s["id"] in MEMBER_OF or s["id"].startswith(("base-foot", "table-")):
            structure.append({"id": f"m-{s['id']}", "of": s["id"], "member": MEMBER_OF.get(s["id"], "table" if s["id"].startswith("table-") else "case"), "part": "member"})
    js = max(f, 0.0075)
    structure.append(joint_box("j-machine-base-l", "machine", "machine", (0.0, YB + js / 2, 0.15), js))
    structure.append(joint_box("j-machine-base-r", "machine", "machine", (0.0, YB + js / 2, -0.15), js))
    hs = max(f, 0.0075)
    structure.append(joint_box("j-handle-l", "hood", "hood", (-0.265, HOOD["y1"] + hs / 2, 0.071), hs))
    structure.append(joint_box("j-handle-r", "hood", "hood", (-0.265, HOOD["y1"] + hs / 2, -0.071), hs))
    structure.append(joint_box("j-tread-hinge", "pedal", "pedal", (0.262, 0.046 - js / 2, -0.0225), js))
    for aid in FLOOR:
        structure.append(support_disc(aid, scale))
    mr = f / 2 if scale == 200 else f
    for bone, at in (("table", (-0.04, -0.105, 0.0)), ("blanket", (-0.04, 0.006, 0.26)), ("case", (0.0, 0.028 if scale == 50 else 0.024, 0.0)), ("machine", (-0.005, YB + 0.13, 0.12)), ("hood", (-0.265, 0.100, 0.0)), ("pedal", (0.21, 0.025, -0.0225))):
        structure.append(mass_marker(f"mass-{bone}", bone, at, mr))
    return body, thermal, structure


VOL_DECISIONS = {
    50: [T("Body, authored for 7.5 mm: the folded moving blanket (four 3 mm layers with the rounded fold); the base board of the case (16 mm vinyl-faced chipboard on four 20 mm rubber feet); the machine as its cast C-frame: the bed with its rounded front edge, the pillar, the arm and the head rounded along their upper edges, the 8 mm face cover over the lamp, the handwheel with its chamfered rim, the stitch selector and stitch length dials, the bobbin winder, the thread spool with its flanges, the 8 mm needle bar with its clamp, the 1.8 mm needle (slender), the presser bar, shank and foot, the take-up lever in its slot and the scrap of fabric under the foot (1 mm, declared sheet); the hood as a hollow 3 mm ABS shell with rounded corners and its 3 mm top (declared sheet) and the arched handle; the foot pedal as a wedge housing with its hinged tread and the taped cable coil (8 mm tube) on top.",
           "Körper für 7,5 mm: die gefaltete Möbeldecke (vier 3-mm-Lagen mit gerundetem Falz); das Bodenbrett des Koffers (16 mm Spanplatte mit Vinylfolie auf vier 20-mm-Gummifüßen); die Maschine als gegossener C-Rahmen: das Bett mit gerundeter Vorderkante, Säule, Arm und Kopf mit gerundeten Oberkanten, die 8-mm-Kopfdeckel über der Lampe, das Handrad mit gefastem Rand, Stichwahl- und Stichlängenregler, der Spuler, die Garnrolle mit Bördeln, die 8-mm-Nadelstange mit Klemme, die 1,8-mm-Nadel (schlank), Stoffdrückerstange, Schaft und Fuß, der Fadenhebel im Schlitz und der Stoffrest unter dem Fuß (1 mm, als Blatt deklariert); die Haube als hohle 3-mm-ABS-Schale mit gerundeten Ecken, ihr 3-mm-Deckel (Blatt) und der Bügelgriff; der Fußanlasser als Keilgehäuse mit Trittplatte und die aufgeklebte Kabelrolle (8-mm-Schlauch) obenauf."),
           T("Thermal: every body solid by reference in its region, plus the outlet patch of the motor vent on the rear of the pillar (emitter). Structure: blanket, base board with feet, the C-frame (bed, pillar, arm, head), the hood shell and handle and the pedal housing as members by reference; the two clamps of the machine on the base board, the two handle posts and the tread hinge as joints; one pad per floor contact of the blanket; masses at the centres of the six bones that carry mass.",
             "Thermik: jeder Körper als Verweis in seiner Region, dazu die Austrittsfläche der Motorlüftung an der Säulenrückseite (Emitter). Tragwerk: Decke, Bodenbrett mit Füßen, der C-Rahmen (Bett, Säule, Arm, Kopf), Haubenschale und Griff sowie das Anlassergehäuse als Stäbe per Verweis; die zwei Klammern der Maschine auf dem Bodenbrett, die zwei Griffstützen und das Trittgelenk als Knoten; je Bodenkontakt der Decke eine Auflagerscheibe; Massen in den Schwerpunkten der sechs massetragenden Knochen.")],
    100: [T("Body, authored anew for 15 mm: the rubber feet merge into a 24 mm base board, the face cover is 15 mm, needle bar and presser bar are 15 mm rods and shank and foot one 15 mm block, the needle stays at its true 1.8 mm (slender); the needle clamp, the take-up lever and the bobbin winder (all under 15 mm) are left out, the length dial stands 15 mm proud; spool without flanges, handwheel without chamfer, bed with a chamfer instead of the round; tread and pedal housing are one wedge; the blanket is one 12 mm declared sheet with its fold; the handle bar is 15 mm.",
            "Körper für 15 mm neu gebaut: die Gummifüße gehen in einem 24-mm-Bodenbrett auf, der Kopfdeckel ist 15 mm, Nadel- und Drückerstange sind 15-mm-Stäbe und Schaft und Fuß ein 15-mm-Block, die Nadel bleibt bei echten 1,8 mm (schlank); Nadelklemme, Fadenhebel und Spuler (alle unter 15 mm) entfallen, der Stichlängenregler steht 15 mm vor; Garnrolle ohne Bördel, Handrad ohne Fase, Bett mit Fase statt Rundung; Trittplatte und Anlassergehäuse sind ein Keil; die Decke ist ein 12-mm-Blatt mit Falz; der Griffbügel ist 15 mm."),
          T("Thermal: one surface per region by reference plus the vent outlet. Structure: the same members by reference, joints at 15 mm, one pad per floor contact, masses.",
            "Thermik: eine Oberfläche je Region per Verweis und die Lüftungsöffnung. Tragwerk: dieselben Stäbe per Verweis, Knoten bei 15 mm, je Bodenkontakt eine Auflagerscheibe, Massen.")],
    200: [T("Body, authored anew for 30 mm: the machine as five boxes of its C-frame (bed, pillar, arm, head, a 30 mm face cover), the handwheel as a 30 mm disc that reaches 10 mm into the arm end, the two dials as one 30 mm control block on the pillar, needle, bars and foot as one 30 mm column under the head; the spool keeps its 32 by 40 mm; the hood shell keeps its true 3 mm wall (a hollow part judged by its extents) with its top as a declared sheet and the handle as a 30 mm block; the pedal is one wedge, the coil a 30 mm puck on it; base board (24 mm) and blanket (12 mm, sheet) keep their drawn thickness (C13).",
            "Körper für 30 mm neu gebaut: die Maschine als fünf Quader ihres C-Rahmens (Bett, Säule, Arm, Kopf, ein 30-mm-Kopfdeckel), das Handrad als 30-mm-Scheibe, die 10 mm in das Armende reicht, die zwei Regler als ein 30-mm-Bedienblock an der Säule, Nadel, Stangen und Fuß als eine 30-mm-Säule unter dem Kopf; die Garnrolle behält 32 x 40 mm; die Haubenschale behält ihre echte 3-mm-Wand (ein Hohlteil, nach seinen Abmessungen beurteilt), ihr Deckel ist ein Blatt und der Griff ein 30-mm-Block; der Anlasser ist ein Keil, die Kabelrolle ein 30-mm-Puck darauf; Bodenbrett (24 mm) und Decke (12 mm, Blatt) behalten ihre gezeichnete Dicke (C13)."),
          T("Thermal: one surface per region by reference plus the vent outlet at 30 mm. Structure: members by reference, joints at 30 mm, one pad per floor contact, masses.",
            "Thermik: eine Oberfläche je Region per Verweis und die Lüftungsöffnung bei 30 mm. Tragwerk: Stäbe per Verweis, Knoten bei 30 mm, je Bodenkontakt eine Auflagerscheibe, Massen.")],
}


TABLE_DECISION = {
    50: T("Sewing table, authored for 7.5 mm: the 25 mm laminate top 0.80 x 0.60 m, four 45 mm square beech legs set 30 mm in from the corners and four apron rails 20 x 90 mm between them; in the thermal list the top and the frame are two regions, in the structure list the table is one member that stands on four floor supports under the legs and carries the blanket and everything on it.", "Nähtisch, für 7,5 mm modelliert: die 25-mm-Schichtstoffplatte 0,80 x 0,60 m, vier quadratische 45-mm-Buchenbeine, 30 mm von den Ecken eingerückt, und vier Zargen 20 x 90 mm dazwischen; in der Thermikliste sind Platte und Gestell zwei Regionen, in der Tragwerksliste ist der Tisch ein Stab, der auf vier Bodenauflagern unter den Beinen steht und die Decke mit allem darauf trägt."),
    100: T("Sewing table, authored for 15 mm: the same top, legs and 20 mm apron rails, all above the feature.", "Nähtisch, für 15 mm modelliert: dieselbe Platte, Beine und 20-mm-Zargen, alle über dem kleinsten Merkmal."),
    200: T("Sewing table, authored for 30 mm: the 25 mm top stays at its real thickness as a sheet, the four 45 mm legs stay and the apron rails are typified at 30 mm.", "Nähtisch, für 30 mm modelliert: die 25-mm-Platte bleibt als Scheibe in echter Dicke, die vier 45-mm-Beine bleiben und die Zargen sind auf 30 mm typisiert."),
}


def build_volumes():
    """🧊 The three individually authored volumes and the unavailable 1:500."""
    vols = {}
    for scale in (50, 100, 200):
        body, thermal, structure = lists(scale)
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOL_DECISIONS[scale] + [TABLE_DECISION[scale]]}
    vols["500"] = {"status": "unavailable", "denominator": 500}
    return vols


# ---------------------------------------------------------------- masses, rig, anchors

def masses():
    """⚖ Bone masses (kg) and centres (world rest)."""
    b = BLANKET
    blanket = (b["x1"] - b["x0"]) * 2 * b["z"] * b["t"] * RHO["felt-blanket"]
    board = 0.24 * 0.45 * 0.016 * RHO["vinyl-chipboard"]
    feet = 4 * math.pi * 0.010 ** 2 * 0.008 * RHO["rubber"]
    outer = hood_ring(0.0, HOOD["r"], 50)
    inner = hood_ring(HOOD["wall"], HOOD["r"], 50)
    h_wall = (area(outer) - area(inner)) * (HOOD["y1"] - HOOD["wall"] - HOOD["y0"])
    h_top = area(outer) * HOOD["wall"]
    hd = HANDLE
    h_handle = (2 * (hd["z"] - hd["zi"]) * (hd["h"] - hd["bar"]) + 2 * hd["z"] * hd["bar"]) * (hd["x1"] - hd["x0"])
    hood = (h_wall + h_top + h_handle) * RHO["abs-ivory"]
    cable = 2 * math.pi * COIL["R"] * math.pi * COIL["r"] ** 2 * RHO["pvc-cable"]
    t = TABLE
    top = (t["x1"] - t["x0"]) * 2 * t["z"] * t["top"] * RHO["laminate-chipboard"]
    legs = 4 * t["leg"] ** 2 * (t["h"] - t["top"]) * RHO["beech-lacquered"]
    span_x = t["x1"] - t["x0"] - 2 * (t["inset"] + t["leg"])
    span_z = 2 * (t["z"] - t["inset"] - t["leg"])
    aprons = 2 * (span_x + span_z) * t["rail"] * t["apron"] * RHO["beech-lacquered"]
    table_y = (top * -t["top"] / 2 + legs * -(t["h"] + t["top"]) / 2 + aprons * -(t["top"] + t["apron"] / 2)) / (top + legs + aprons)
    return {
        "table": (top + legs + aprons, ((t["x0"] + t["x1"]) / 2, table_y)),
        "blanket": (blanket, (-0.04, 0.006)),
        "case": (board + feet, (0.0, 0.0275)),
        "machine": (8.0, (-0.005, YB + 0.130)),
        "hood": (hood, (-0.265, 0.192)),
        "pedal": (0.45, (0.21, 0.030)),
        "cable": (cable, (COIL["x"], COIL["y"])),
        "_parts": dict(table_top=top, table_legs=legs, table_aprons=aprons, blanket=blanket, board=board, feet=feet, hood_wall=h_wall * RHO["abs-ivory"], hood_top=h_top * RHO["abs-ivory"], hood_handle=h_handle * RHO["abs-ivory"], cable=cable),
    }


FLOOR = {f"floor-{lid}": ((x0 + x1) / 2, (z0 + z1) / 2) for lid, x0, x1, z0, z1 in table_legs()}
CASE_FEET = {"case-foot-fl": (0.095, 0.20), "case-foot-fr": (0.095, -0.20), "case-foot-rl": (-0.095, 0.20), "case-foot-rr": (-0.095, -0.20)}
PEDAL_FEET = {"pedal-foot-fl": (0.16, 0.015), "pedal-foot-fr": (0.16, -0.060), "pedal-foot-rl": (0.26, 0.015), "pedal-foot-rr": (0.26, -0.060)}
NAMES = {"fl": T("front left", "vorne links"), "fr": T("front right", "vorne rechts"), "rl": T("rear left", "hinten links"), "rr": T("rear right", "hinten rechts")}


def anchors():
    """🪝 Floor contacts of the four table legs, the four feet of the base board, the hand on the handle, the four corners of the pedal, the motor vent and the two load points."""
    out = []
    for aid, (x, z) in FLOOR.items():
        n = NAMES[aid[-2:]]
        u, v = loc("table", x, -TABLE["h"])
        out.append({"id": aid, "bone": "table", "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"Floor under the table leg, {n['en']}", f"Boden unter dem Tischbein, {n['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": TABLE["leg"], "footprintWidthM": TABLE["leg"], "frictionCoefficient": 0.4})
    for aid, (x, z) in CASE_FEET.items():
        n = NAMES[aid[-2:]]
        u, v = loc("case", x, 0.012)
        out.append({"id": aid, "bone": "case", "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"Rubber foot of the base board, {n['en']}", f"Gummifuß des Bodenbretts, {n['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.02, "frictionCoefficient": 0.7})
    u, v = loc("hood", -0.265, HOOD["y1"] + HANDLE["h"] - HANDLE["bar"] / 2)
    out.append({"id": "grip", "bone": "hood", "x": u, "y": v, "z": 0.0, "kind": "contact", "title": T("The carrier's hand round the handle bar", "Die Hand des Trägers um den Griffbügel"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.09, "frictionCoefficient": 0.6})
    for aid, (x, z) in PEDAL_FEET.items():
        n = NAMES[aid[-2:]]
        u, v = loc("pedal", x, 0.012)
        out.append({"id": aid, "bone": "pedal", "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"Pedal underside, {n['en']}", f"Unterseite des Anlassers, {n['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.02, "frictionCoefficient": 0.6})
    u, v = loc("machine", -0.084, YB + 0.150)
    out.append({"id": "motor-vent", "bone": "machine", "x": u, "y": v, "z": 0.12, "kind": "emitter", "title": T("Cooling air leaving the motor vent at the rear of the pillar", "Kühlluft aus der Motorlüftung an der Säulenrückseite")})
    u, v = loc("tread", 0.21, 0.054)
    out.append({"id": "foot-on-tread", "bone": "tread", "x": u, "y": v, "z": -0.0225, "kind": "load", "title": T("The sewer's right foot on the tread", "Der rechte Fuß der Nähenden auf der Trittplatte")})
    u, v = loc("machine", 0.040, YB + 0.075)
    out.append({"id": "hands-on-bed", "bone": "machine", "x": u, "y": v, "z": -0.120, "kind": "load", "title": T("Hands guiding the fabric on the bed", "Hände, die den Stoff auf dem Bett führen")})
    return out


ANCHORS = anchors()


def bone(bid, parent, title, joint, length, mass=None, section=None):
    """🦴 One bone at its world rest origin, unrotated."""
    ox, oy = ORIGIN[bid]
    px, py = ORIGIN[parent] if parent else ORIGIN[bid]
    b = {"id": bid, "parent": parent, "title": title, "x": ox - px, "y": oy - py, "angleDeg": 0, "lengthM": length, "joint": joint}
    if mass:
        b["mass"] = mass
    if section:
        b["section"] = section
    return b


def rig():
    """🦴 The rig: the sewing table stands on the floor and the folded blanket lies on it, both stay behind; the case (base board, machine, hood, pedal and cable) is lifted and carried by the handle; the hood, the pedal and the coil move on their own sliders; handwheel, needle bar, take-up lever, presser foot, fabric and tread move in use."""
    M = masses()

    def mass(b, ref, src="authored-estimate"):
        kg, (x, y) = M[b]
        u, v = loc(b, x, y)
        return {"kg": kg, "comX": u, "comY": v, "distribution": "point", "source": src, "reference": ref}

    fixed = {"type": "fixed"}
    hood_I = 2 * (0.45 * 0.003 * 0.175 ** 2) + 2 * (0.003 * 0.35 ** 3 / 12)
    bed_I = 2 * 0.385 * 0.0035 * 0.0375 ** 2 + 2 * 0.0035 * 0.075 ** 3 / 12
    ped_I = 2 * 0.095 * 0.0025 * 0.014 ** 2 + 2 * 0.0025 * 0.028 ** 3 / 12
    return {"kind": "mechanical", "root": "ground", "bones": [
        bone("ground", None, T("Floor reference", "Bodenbezug"), fixed, 0),
        bone("table", "ground", T("Sewing table (stays behind when the case is carried)", "Nähtisch (bleibt stehen, wenn der Koffer getragen wird)"), fixed, TABLE["x1"] - TABLE["x0"], mass("table", "kitchen-table-catalogue"), {"areaM2": 4 * TABLE["leg"] ** 2, "secondMomentM4": 4 * TABLE["leg"] ** 4 / 12, "extremeFibreM": TABLE["leg"] / 2, "material": "beech-lacquered", "title": T("Four legs 45 x 45 mm", "Vier Beine 45 x 45 mm")}),
        bone("blanket", "table", T("Folded moving blanket under the machine (stays on the table)", "Gefaltete Möbeldecke unter der Maschine (bleibt auf dem Tisch)"), fixed, 0.72, mass("blanket", "moving-blanket-catalogue")),
        bone("carry-lift", "ground", T("Carrier's hand: lift", "Hand des Trägers: Heben"), {"type": "prismatic", "axisDeg": 90, "minM": -0.45, "maxM": 0.8}, 0),
        bone("carry-travel", "carry-lift", T("Carrier's hand: walking away", "Hand des Trägers: Weggehen"), {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.2}, 0),
        bone("case", "carry-travel", T("Base board of the carrying case", "Bodenbrett des Tragekoffers"), fixed, 0.24, mass("case", "sewing-case-catalogue"), {"areaM2": 0.45 * 0.016, "secondMomentM4": 0.45 * 0.016 ** 3 / 12, "extremeFibreM": 0.008, "material": "vinyl-chipboard", "title": T("Board 450 x 16 mm", "Brett 450 x 16 mm")}),
        bone("machine", "case", T("Sewing machine: cast C-frame of bed, pillar, arm and head", "Nähmaschine: gegossener C-Rahmen aus Bett, Säule, Arm und Kopf"), fixed, 0.18, mass("machine", "sewing-machine-catalogue"), {"areaM2": 2 * (0.385 + 0.075) * 0.0035, "secondMomentM4": bed_I, "extremeFibreM": 0.0375, "material": "cast-aluminium", "title": T("Bed casting 385 x 75 mm, wall 3.5 mm", "Bettguss 385 x 75 mm, Wand 3,5 mm")}),
        bone("handwheel", "machine", T("Handwheel on the main shaft", "Handrad auf der Hauptwelle"), {"type": "revolute", "minDeg": 0, "maxDeg": 4320}, 0.010),
        bone("needle-bar", "machine", T("Needle bar with clamp and needle", "Nadelstange mit Klemme und Nadel"), {"type": "prismatic", "axisDeg": 90, "minM": -NEEDLE_STROKE, "maxM": 0}, 0.008),
        bone("take-up", "machine", T("Thread take-up lever", "Fadenhebel"), {"type": "prismatic", "axisDeg": 90, "minM": -NEEDLE_STROKE, "maxM": 0}, 0.008),
        bone("presser", "machine", T("Presser bar and foot", "Stoffdrückerstange und Nähfuß"), {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": PRESSER_LIFT}, 0.008),
        bone("fabric", "machine", T("Scrap of fabric under the foot", "Stoffrest unter dem Fuß"), {"type": "prismatic", "axisDeg": 0, "minM": -0.04, "maxM": 0}, 0.008),
        bone("hood-lift", "case", T("Hood: lift", "Haube: Heben"), {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.40}, 0),
        bone("hood-slide", "hood-lift", T("Hood: slide over the machine", "Haube: über die Maschine schieben"), {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.30}, 0),
        bone("hood", "hood-slide", T("Hood of the carrying case with handle", "Haube des Tragekoffers mit Griff"), fixed, 0.24, mass("hood", "sewing-case-catalogue"), {"areaM2": 2 * (0.45 + 0.35) * 0.003, "secondMomentM4": hood_I, "extremeFibreM": 0.175, "material": "abs-ivory", "title": T("ABS shell 450 x 350 mm, 3 mm", "ABS-Schale 450 x 350 mm, 3 mm")}),
        bone("pedal-lift", "case", T("Pedal: lift", "Anlasser: Heben"), {"type": "prismatic", "axisDeg": 90, "minM": -0.80, "maxM": 0.15}, 0),
        bone("pedal-slide", "pedal-lift", T("Pedal: slide", "Anlasser: Schieben"), {"type": "prismatic", "axisDeg": 0, "minM": -0.25, "maxM": 0.30}, 0),
        bone("pedal", "pedal-slide", T("Foot pedal housing", "Gehäuse des Fußanlassers"), fixed, 0.12, mass("pedal", "sewing-machine-catalogue"), {"areaM2": 2 * (0.095 + 0.028) * 0.0025, "secondMomentM4": ped_I, "extremeFibreM": 0.014, "material": "abs-brown", "title": T("ABS housing 95 x 28 mm, 2.5 mm", "ABS-Gehäuse 95 x 28 mm, 2,5 mm")}),
        bone("tread", "pedal", T("Tread of the pedal, hinged at the heel", "Trittplatte des Anlassers, an der Ferse gelenkig"), {"type": "revolute", "minDeg": 0, "maxDeg": 3.5}, 0.010),
        bone("cable-lift", "pedal", T("Cable coil: drop", "Kabelrolle: Absenken"), {"type": "prismatic", "axisDeg": 90, "minM": -0.06, "maxM": 0}, 0),
        bone("cable-slide", "cable-lift", T("Cable coil: slide", "Kabelrolle: Schieben"), {"type": "prismatic", "axisDeg": 0, "minM": -0.20, "maxM": 0}, 0),
        bone("cable", "cable-slide", T("Taped coil of the mains and pedal cable", "Abgeklebte Rolle aus Netz- und Anlasserkabel"), fixed, 0.008, mass("cable", "sewing-machine-catalogue")),
    ]}


# ---------------------------------------------------------------- materials, thermal

REFS = [
    ("iec-60335-2-28", "IEC 60335-2-28 Household and similar electrical appliances - Safety - Part 2-28: Particular requirements for sewing machines (household sewing machines with motor and lamp; rated input on the nameplate)."),
    ("din-en-60598-1", "DIN EN 60598-1 Luminaires - General requirements and tests (surface temperature limits of lamp covers; used for the order of the lamp cover temperature)."),
    ("sewing-machine-catalogue", "Catalogue-typical data of electric free-arm household sewing machines of German mail-order and department-store mid-range types of the 1980s and 1990s (no brand): 390-410 x 180-200 x 290-300 mm, 7.5-9.5 kg with die-cast aluminium arm, motor 60-75 W and lamp 15 W at 230 V (nameplate total 75-90 W), 600-850 stitches per minute, needle bar stroke 30-32 mm, presser foot lift 6-7 mm, foot controller about 120 x 95 x 50 mm, 0.4-0.5 kg (authored-estimate from listings of the type)."),
    ("sewing-case-catalogue", "Catalogue-typical data of hard carrying cases of portable sewing machines of the same period: base board of 15-16 mm vinyl-faced chipboard about 450 x 240 mm on rubber feet, ABS hood 3 mm about 450 x 240 x 350 mm with a moulded handle and two side latches, 2.5-3.5 kg (authored-estimate)."),
    ("kitchen-table-catalogue", "Catalogue-typical data of small German kitchen and work tables of the 1960s to 1980s (types, not brands): top 80 x 60 cm of 25 mm chipboard faced with decorative laminate (Resopal type), frame of lacquered solid beech with four square legs of about 45 mm and apron rails of about 20 x 90 mm, height 75 cm, about 14-16 kg (authored-estimate)."),
    ("en-338-2016", "EN 338:2016 Structural timber - Strength classes (beech of class D35: characteristic bending strength 35 MPa, mean modulus of elasticity 12 GPa, mean density about 650-720 kg/m3)."),
    ("moving-blanket-catalogue", "Catalogue-typical data of removal blankets (Umzugsdecken) of quilted nonwoven felt, 3 mm, about 0.4 kg/m2, folded fourfold (authored-estimate)."),
    ("en-1706-2020", "EN 1706:2020 Aluminium and aluminium alloys - Castings - Chemical composition and mechanical properties (EN AC-46000 AlSi9Cu3(Fe) die castings: tensile strength about 240 MPa, 0.2 % proof strength about 140 MPa, E about 70-75 GPa, density 2700 kg/m3)."),
    ("en-312-2010", "EN 312:2010 Particleboards - Specifications (type P2, about 650 kg/m3, bending strength about 11 MPa, E about 1.6 GPa)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (particleboard, ABS and PVC plastics, rubber, aluminium, steel, felt)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of enamels and lacquers 0.85-0.95, of polished chromium 0.07-0.15; solar absorptance of light and dark paints; free convection)."),
    ("din-18011-1992", "DIN 18011:1992-11 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (worktop and table height 0.72-0.75 m used for the table in use)."),
    ("dguv-208-033", "DGUV Information 208-033 (2014) Muskel-Skelett-Belastungen - erkennen und beurteilen (one-handed carrying of loads beside the body with a hanging arm, hand height about 0.75-0.80 m; loads of about 10-12 kg acceptable for short distances)."),
    ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: imposed loads on buildings (used only for the order of the resting loads of hand and foot)."),
    ("reuse-depot-practice", "Practice of flat clearances and municipal second-hand department stores in Germany (Haushaltsauflösung, Gebrauchtwarenkaufhaus): sewing machines are set out in their case with the hood beside them for inspection, presser foot lowered on a fabric scrap, cable wound and taped, then carried out shut by the handle (authored-estimate from described practice, no single source)."),
]


def materials():
    """🧪 The materials with their aged surface properties."""
    def m(mid, en, de, eps, alpha, k, cp, src, ref, E=None, f=None):
        d = {"id": mid, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": RHO[mid], "conductivityWmK": k, "specificHeatJKgK": cp, "source": src, "reference": ref}
        if E:
            d["elasticModulusPa"], d["strengthPa"] = E, f
        return d
    return [
        m("cast-aluminium", "Die-cast aluminium AlSi9Cu3 with ivory enamel, yellowed and worn", "Aluminium-Druckguss AlSi9Cu3 mit elfenbeinfarbenem Lack, vergilbt und abgegriffen", 0.9, 0.42, 110, 900, "literature", "en-1706-2020", 7.2e10, 1.4e8),
        m("chromed-zinc", "Chromed zinc die casting of the handwheel, dulled", "Verchromter Zinkdruckguss des Handrads, matt geworden", 0.15, 0.4, 110, 390, "authored-estimate", "incropera-2018"),
        m("chromed-steel", "Chromed steel of needle bar, presser foot and needle", "Verchromter Stahl von Nadelstange, Nähfuß und Nadel", 0.12, 0.45, 50, 450, "literature", "incropera-2018", 2.1e11, 3.5e8),
        m("abs-ivory", "Ivory ABS of the hood, sun-yellowed", "Elfenbeinfarbenes ABS der Haube, sonnenvergilbt", 0.9, 0.45, 0.17, 1400, "authored-estimate", "iso-10456-2007", 2.2e9, 4.0e7),
        m("abs-brown", "Brown ABS of pedal, dials and handle", "Braunes ABS von Anlasser, Reglern und Griff", 0.9, 0.85, 0.17, 1400, "authored-estimate", "iso-10456-2007", 2.2e9, 4.0e7),
        m("pvc-cable", "PVC-sheathed two-core cable, coiled bundle", "PVC-ummanteltes zweiadriges Kabel, als Bund gerollt", 0.9, 0.7, 0.3, 1000, "authored-estimate", "iso-10456-2007"),
        m("vinyl-chipboard", "P2 chipboard faced with brown wood-grain vinyl foil", "Spanplatte P2 mit brauner Holzdekor-Vinylfolie", 0.9, 0.7, 0.14, 1700, "literature", "en-312-2010", 1.6e9, 1.1e7),
        m("rubber", "Rubber of feet and pads", "Gummi von Füßen und Polstern", 0.9, 0.9, 0.16, 1500, "authored-estimate", "iso-10456-2007"),
        m("thread", "Polyester thread wound on a plastic spool", "Polyestergarn auf einer Kunststoffrolle", 0.9, 0.6, 0.1, 1200, "authored-estimate", "incropera-2018"),
        m("cotton", "Cotton scrap fabric, blue", "Baumwoll-Stoffrest, blau", 0.9, 0.65, 0.06, 1300, "authored-estimate", "incropera-2018"),
        m("laminate-chipboard", "P2 chipboard faced with grey decorative laminate, scratched and dulled", "Spanplatte P2 mit grauem Schichtstoff, zerkratzt und matt geworden", 0.9, 0.6, 0.14, 1700, "literature", "en-312-2010", 1.6e9, 1.1e7),
        m("beech-lacquered", "Solid beech with clear lacquer, yellowed and scuffed at the feet", "Massive Buche mit Klarlack, vergilbt und an den Füßen bestoßen", 0.9, 0.55, 0.16, 1600, "literature", "en-338-2016", 1.2e10, 3.5e7),
        m("felt-blanket", "Quilted nonwoven felt of a moving blanket, dark blue", "Gesteppter Vliesfilz einer Möbeldecke, dunkelblau", 0.9, 0.8, 0.045, 1300, "authored-estimate", "iso-10456-2007"),
    ]


AREAS = {"housing": 0.473, "lamp-cover": 0.051, "handwheel": 0.0124, "dials": 0.0105, "needle-foot": 0.008, "thread-spool": 0.0058, "fabric": 0.029, "base-board": 0.23, "hood": 0.58, "pedal": 0.058, "cable": 0.013, "blanket": 0.83, "table-top": 1.03, "table-frame": 0.97}


def layer(lid, title, kind, material, t, src, ref):
    """🧶 One layer with resistance and the areal heat capacity of one hour's penetration depth."""
    mat = {x["id"]: x for x in materials()}[material]
    lam, rho, cp = mat["conductivityWmK"], mat["densityKgM3"], mat["specificHeatJKgK"]
    depth = math.sqrt(lam / (rho * cp) * 3600)
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": t / lam, "heatCapacityJM2K": rho * cp * min(t, depth), "source": src, "reference": ref}


def region(rid, title, kind, model, material, L, geom, svf, lay, extra=None):
    """🔥 One region with one layer."""
    r = {"id": rid, "title": title, "kind": kind, "model": model, "material": material, "areaM2": AREAS[rid], "characteristicLengthM": L, "geometry": geom, "skyViewFactor": svf, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


ENV = {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.5, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325, "skyTempC": 20, "groundTempC": 19}


def thermal_model():
    """🌡 Fourteen regions: the motor-warmed housing, the lamp-heated face cover, the pedal resistor, and the passive parts and storage aids."""
    regions = [
        region("housing", T("Housing of bed, pillar, arm and head (ivory enamel on die-cast aluminium, yellowed)", "Gehäuse aus Bett, Säule, Arm und Kopf (elfenbeinfarbener Lack auf Aluminiumguss, vergilbt)"), "surface", "operating", "cast-aluminium", 0.2, "vertical-plate", 0.5,
               layer("casting", T("Die-cast aluminium wall 3.5 mm with enamel", "Aluminiumgusswand 3,5 mm mit Lack"), "metal", "cast-aluminium", 0.0035, "literature", "en-1706-2020"),
               {"operating": {"key": "motor", "heatW": 42}, "ratedPowerW": 70, "ratedPowerBasis": "electrical-input", "reference": "iec-60335-2-28"}),
        region("lamp-cover", T("Face cover of the head over the 15 W lamp", "Kopfdeckel über der 15-W-Lampe"), "lamp", "operating", "cast-aluminium", 0.14, "vertical-plate", 0.5,
               layer("cover", T("Die-cast aluminium cover 3 mm with enamel", "Aluminiumgussdeckel 3 mm mit Lack"), "metal", "cast-aluminium", 0.003, "literature", "en-1706-2020"),
               {"operating": {"key": "lamp", "heatW": 15}, "ratedPowerW": 15, "ratedPowerBasis": "electrical-input", "reference": "iec-60335-2-28"}),
        region("handwheel", T("Chromed handwheel", "Verchromtes Handrad"), "surface", "passive", "chromed-zinc", 0.07, "cylinder", 0.5,
               layer("rim", T("Zinc die casting 4 mm", "Zinkdruckguss 4 mm"), "metal", "chromed-zinc", 0.004, "authored-estimate", "incropera-2018")),
        region("dials", T("Stitch selector, stitch length dial and bobbin winder", "Stichwahl, Stichlängenregler und Spuler"), "surface", "passive", "abs-brown", 0.05, "cylinder", 0.5,
               layer("knob", T("ABS 3 mm", "ABS 3 mm"), "polymer", "abs-brown", 0.003, "authored-estimate", "iso-10456-2007")),
        region("needle-foot", T("Needle bar, needle, presser foot and take-up lever (chromed steel)", "Nadelstange, Nadel, Nähfuß und Fadenhebel (verchromter Stahl)"), "surface", "passive", "chromed-steel", 0.008, "cylinder", 0.3,
               layer("steel", T("Steel, half section 4 mm", "Stahl, halber Querschnitt 4 mm"), "metal", "chromed-steel", 0.004, "literature", "incropera-2018")),
        region("thread-spool", T("Spool of thread on the spool pin", "Garnrolle auf dem Garnrollenstift"), "surface", "passive", "thread", 0.032, "cylinder", 0.8,
               layer("thread", T("Wound polyester thread 13 mm", "Gewickeltes Polyestergarn 13 mm"), "polymer", "thread", 0.013, "authored-estimate", "incropera-2018")),
        region("fabric", T("Scrap of cotton fabric under the presser foot", "Baumwoll-Stoffrest unter dem Nähfuß"), "surface", "passive", "cotton", 0.11, "horizontal-plate-up", 0.6,
               layer("cloth", T("Cotton cloth folded double, 2 mm", "Baumwolltuch doppelt gelegt, 2 mm"), "clothing", "cotton", 0.002, "authored-estimate", "incropera-2018")),
        region("base-board", T("Base board of the case on rubber feet, scuffed", "Bodenbrett des Koffers auf Gummifüßen, bestoßen"), "surface", "passive", "vinyl-chipboard", 0.24, "horizontal-plate-up", 0.4,
               layer("board", T("Vinyl-faced chipboard 16 mm", "Spanplatte mit Vinylfolie 16 mm"), "timber", "vinyl-chipboard", 0.016, "literature", "en-312-2010")),
        region("hood", T("Hood of the case with handle, yellowed, cracked corner and an old sticker", "Haube des Koffers mit Griff, vergilbt, mit gesprungener Ecke und altem Aufkleber"), "surface", "passive", "abs-ivory", 0.35, "vertical-plate", 0.5,
               layer("shell", T("ABS shell 3 mm", "ABS-Schale 3 mm"), "polymer", "abs-ivory", 0.003, "authored-estimate", "iso-10456-2007")),
        region("pedal", T("Foot pedal with its speed resistor", "Fußanlasser mit Vorwiderstand"), "surface", "operating", "abs-brown", 0.1, "horizontal-plate-up", 0.5,
               layer("housing", T("ABS housing 2.5 mm", "ABS-Gehäuse 2,5 mm"), "polymer", "abs-brown", 0.0025, "authored-estimate", "iso-10456-2007"),
               {"operating": {"key": "motor", "heatW": 6}, "reference": "sewing-machine-catalogue"}),
        region("cable", T("Taped coil of mains and pedal cable", "Abgeklebte Rolle aus Netz- und Anlasserkabel"), "surface", "passive", "pvc-cable", 0.016, "cylinder", 0.5,
               layer("sheath", T("PVC sheath and cores, half bundle 8 mm", "PVC-Mantel und Adern, halber Bund 8 mm"), "polymer", "pvc-cable", 0.008, "authored-estimate", "iso-10456-2007")),
        region("blanket", T("Folded moving blanket", "Gefaltete Möbeldecke"), "surface", "passive", "felt-blanket", 0.56, "horizontal-plate-up", 0.4,
               layer("felt", T("Four layers of felt, 12 mm", "Vier Lagen Filz, 12 mm"), "polymer", "felt-blanket", 0.012, "authored-estimate", "iso-10456-2007")),
        region("table-top", T("Laminate top of the sewing table", "Schichtstoffplatte des Nähtischs"), "surface", "passive", "laminate-chipboard", 0.6, "horizontal-plate-up", 0.5,
               layer("top", T("Laminate-faced chipboard 25 mm", "Schichtstoffbeschichtete Spanplatte 25 mm"), "timber", "laminate-chipboard", 0.025, "literature", "en-312-2010")),
        region("table-frame", T("Beech legs and apron rails of the sewing table", "Buchenbeine und Zargen des Nähtischs"), "surface", "passive", "beech-lacquered", 0.725, "vertical-plate", 0.4,
               layer("beech", T("Solid beech 45 mm", "Massive Buche 45 mm"), "timber", "beech-lacquered", 0.045, "literature", "en-338-2016")),
    ]
    emitters = [{"id": "motor-vent", "kind": "vent", "title": T("Warm cooling air from the motor vent", "Warme Kühlluft aus der Motorlüftung"), "anchor": "motor-vent", "directionDeg": 180, "diameterM": 0.03, "operatingKey": "motor", "volumeFlowM3S": 0.0015, "temperatureRiseK": 12, "source": "authored-estimate", "reference": "sewing-machine-catalogue"}]
    operating = [
        {"id": "motor", "title": T("Sewing motor (pedal speed)", "Nähmotor (Pedaldrehzahl)"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.6, "title": T("Even sewing, about 500 stitches per minute", "Gleichmäßiges Nähen, etwa 500 Stiche pro Minute")}, {"v": 1, "title": T("Full speed (70 W)", "Volle Drehzahl (70 W)")}]},
        {"id": "lamp", "title": T("Sewing lamp", "Nählampe"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("On (15 W)", "An (15 W)")}]},
    ]
    return {"environment": ENV, "operating": operating, "passengers": [], "regions": regions, "emitters": emitters, "assumptions": [
        T("Nameplate 230 V, 85 W: motor 70 W (stated on the housing region) and lamp 15 W (stated on the face cover region), both electrical input. At full speed the 70 W of the motor leave as about 22 W in the cooling air of the vent (1.5 l/s warmed 12 K), 42 W through the housing and 6 W in the series resistor of the pedal; at the sewing level 0.6 they scale with the level. The 15 W incandescent lamp heats its face cover.", "Typenschild 230 V, 85 W: Motor 70 W (an der Gehäuseregion angegeben) und Lampe 15 W (an der Kopfdeckelregion), beide elektrische Aufnahme. Bei voller Drehzahl verlassen die 70 W des Motors die Maschine als etwa 22 W in der Kühlluft der Lüftung (1,5 l/s um 12 K erwärmt), 42 W über das Gehäuse und 6 W im Vorwiderstand des Anlassers; bei der Nähstufe 0,6 skalieren sie mit der Stufe. Die 15-W-Glühlampe erwärmt ihren Kopfdeckel."),
        T("Aged surfaces: the ivory enamel of the housing has yellowed and its solar absorptance is taken as 0.42 instead of about 0.30 for new ivory enamel; the ABS of the hood is sun-yellowed (0.45 instead of 0.35); the chrome of the handwheel is dulled (emissivity 0.15 instead of 0.07 for polished chromium) (Incropera, authored-estimate).", "Gealterte Oberflächen: der Elfenbeinlack des Gehäuses ist vergilbt, sein solarer Absorptionsgrad wird mit 0,42 statt etwa 0,30 für neuen Elfenbeinlack angesetzt; das ABS der Haube ist sonnenvergilbt (0,45 statt 0,35); das Chrom des Handrads ist matt (Emissionsgrad 0,15 statt 0,07 für poliertes Chrom) (Incropera, Schätzwert)."),
        T("Areas are the outer areas of the 1:50 model in the stored pose: the hood stands open side down on the blanket, so only its outside counts; while carried shut the machine regions face only the air inside the case (exposed 0).", "Flächen sind die Außenflächen des 1:50-Modells in der Lagerpose: die Haube steht mit der offenen Seite auf der Decke, es zählt nur ihre Außenseite; geschlossen getragen sehen die Maschinenregionen nur die Luft im Koffer (freiliegend 0)."),
        T("Reference case: indoors at 20 degC, still air, no sun; the table stands on a floor of 19 degC.", "Referenzfall: innen bei 20 degC, ruhige Luft, keine Sonne; der Tisch steht auf einem Boden von 19 degC."),
    ]}


REGION_IDS = ["housing", "lamp-cover", "handwheel", "dials", "needle-foot", "thread-spool", "fabric", "base-board", "hood", "pedal", "cable", "blanket", "table-top", "table-frame"]


def binding(kind, motor, lamp):
    """🌬 The thermal binding of one pose family."""
    reg = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGION_IDS}
    reg["blanket"] = {"exposedFraction": 0.4, "sunlitFraction": 0, "contact": {"fraction": 0.6, "tempC": 20}}
    reg["table-top"] = {"exposedFraction": 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.4, "tempC": 20}}
    reg["table-frame"] = {"exposedFraction": 0.99, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 19}}
    reg["fabric"] = {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 20}}
    if kind == "stored":
        reg["base-board"] = {"exposedFraction": 0.7, "sunlitFraction": 0, "contact": {"fraction": 0.3, "tempC": 20}}
        reg["pedal"] = {"exposedFraction": 0.75, "sunlitFraction": 0, "contact": {"fraction": 0.25, "tempC": 20}}
    elif kind == "carried":
        for r in ("housing", "lamp-cover", "handwheel", "dials", "needle-foot", "thread-spool", "fabric", "pedal", "cable"):
            reg[r] = {"exposedFraction": 0, "sunlitFraction": 0}
        reg["base-board"] = {"exposedFraction": 0.5, "sunlitFraction": 0}
        reg["hood"] = {"exposedFraction": 0.99, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 32}}
        reg["blanket"] = {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 20}}
    else:
        reg["base-board"] = {"exposedFraction": 0.7, "sunlitFraction": 0, "contact": {"fraction": 0.3, "tempC": 20}}
        reg["pedal"] = {"exposedFraction": 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.4, "tempC": 25 if motor > 0 else 19}}
    return {"operating": {"motor": motor, "lamp": lamp}, "regions": reg, "emitters": {"motor-vent": motor}}


FLOOR_C = list(FLOOR)
CASE_C = list(CASE_FEET)
PEDAL_C = list(PEDAL_FEET)
FOOT = {"id": "foot", "kind": "occupant", "title": T("Resting right foot on the tread (30 N)", "Ruhender rechter Fuß auf der Trittplatte (30 N)"), "anchor": "foot-on-tread", "fxN": 0, "fyN": -30, "source": "authored-estimate", "reference": "en-1991-1-1"}
HANDS = {"id": "hands", "kind": "occupant", "title": T("Hands guiding the fabric (10 N)", "Hände, die den Stoff führen (10 N)"), "anchor": "hands-on-bed", "fxN": 0, "fyN": -10, "source": "authored-estimate", "reference": "en-1991-1-1"}

STOWED = {"pedal-lift": {"dy": PEDAL_STOW["dy"]}, "pedal-slide": {"dx": PEDAL_STOW["dx"]}}
CLOSED = dict(STOWED, **{"hood-lift": {"dy": HOOD_DY}, "hood-slide": {"dx": HOOD_DX}})
CARRIED = dict(CLOSED, **{"carry-lift": {"dy": CARRY_DY}, "carry-travel": {"dx": TRAVEL}})
IN_USE = {"pedal-lift": {"dy": PEDAL_USE["dy"]}, "pedal-slide": {"dx": PEDAL_USE["dx"]}, "cable-lift": {"dy": COIL_USE["dy"]}, "cable-slide": {"dx": COIL_USE["dx"]}, "tread": {"angleDeg": TREAD_PRESS}}
THREADING = dict(IN_USE, **{"tread": {"angleDeg": 0}, "presser": {"dy": PRESSER_LIFT}})


def pose(pid, title, desc, bones, contacts, kind, motor=0, lamp=0, loads=(), key=False):
    """🧍 One pose."""
    p = {"id": pid, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "authored", "root": {"x": 0, "y": 0, "angleDeg": 0}, "contacts": contacts, "thermal": binding(kind, motor, lamp), "loads": list(loads)})
    return p


def poses():
    """🧍 Stored, carried, in use, threading and the key poses of the carry."""
    return [
        pose("stored", T("Set out at the flat clearance", "Bei der Haushaltsauflösung bereitgestellt"), T("On its sewing table in the hallway, a folded moving blanket under it: the machine on the base board of its case, the hood lifted off and standing behind it, the pedal with its taped cable coil in front, presser foot lowered on a scrap of fabric, needle up.", "Auf ihrem Nähtisch im Flur, darunter eine gefaltete Möbeldecke: die Maschine auf dem Bodenbrett ihres Koffers, die Haube abgenommen und dahinter abgestellt, der Anlasser mit seiner abgeklebten Kabelrolle davor, der Nähfuß auf einem Stoffrest abgesenkt, Nadel oben."), {}, FLOOR_C, "stored"),
        pose("carried", T("Carried shut by the handle", "Geschlossen am Griff getragen"), T("Pedal and coil stowed in the throat under the arm, the hood put over the machine and latched to the base board; one person carries the 11.5 kg case beside the body by the handle with a hanging arm, hand at 0.76 m; the table and the blanket stay behind.", "Anlasser und Kabelrolle im Durchgang unter dem Arm verstaut, die Haube über die Maschine gesetzt und am Bodenbrett verriegelt; eine Person trägt den 11,5-kg-Koffer seitlich am Körper mit hängendem Arm am Griff, Hand auf 0,76 m; Tisch und Decke bleiben zurück."), CARRIED, FLOOR_C + ["grip"], "carried"),
        pose("in-use", T("Sewing at a table", "Nähen am Tisch"), T("Back in a home at its 0.75 m table, the folded blanket under the machine damping it: hood off and standing behind, lamp on, the pedal on the floor under the table pressed by the right foot so the motor runs at the sewing level, the coil of slack cable beside it, hands guiding the fabric.", "Wieder in einer Wohnung an ihrem 0,75 m hohen Tisch, die gefaltete Decke dämpfend unter der Maschine: Haube abgenommen und dahinter abgestellt, Lampe an, der Anlasser auf dem Boden unter dem Tisch vom rechten Fuß gedrückt, sodass der Motor in der Nähstufe läuft, die Rolle des überschüssigen Kabels daneben, die Hände führen den Stoff."), IN_USE, FLOOR_C + PEDAL_C, "in-use", 0.6, 1, [FOOT, HANDS]),
        pose("threading", T("Threading with the presser foot up", "Einfädeln mit angehobenem Nähfuß"), T("On the table, lamp on, motor off: the presser foot is lifted 7 mm to thread the needle and slide the fabric in.", "Auf dem Tisch, Lampe an, Motor aus: der Nähfuß ist 7 mm angehoben, um die Nadel einzufädeln und den Stoff einzulegen."), THREADING, FLOOR_C + PEDAL_C, "in-use", 0, 1),
        pose("pedal-raised-key", T("Pedal lifted (key)", "Anlasser angehoben (Schlüssel)"), T("The pedal with its coil lifted 0.11 m off the blanket.", "Der Anlasser mit Kabelrolle 0,11 m von der Decke angehoben."), {"pedal-lift": {"dy": 0.11}}, FLOOR_C, "stored", key=True),
        pose("pedal-stowed-key", T("Pedal stowed (key)", "Anlasser verstaut (Schlüssel)"), T("The pedal lies with its coil on the bed in the throat under the arm.", "Der Anlasser liegt mit seiner Kabelrolle auf dem Bett im Durchgang unter dem Arm."), STOWED, FLOOR_C, "stored", key=True),
        pose("hood-raised-key", T("Hood lifted (key)", "Haube angehoben (Schlüssel)"), T("The hood lifted 0.37 m behind the machine, clear of the spool.", "Die Haube hinter der Maschine 0,37 m angehoben, über der Garnrolle."), dict(STOWED, **{"hood-lift": {"dy": 0.37}}), FLOOR_C, "stored", key=True),
        pose("hood-over-key", T("Hood over the machine (key)", "Haube über der Maschine (Schlüssel)"), T("The hood held above the machine before it is lowered.", "Die Haube über der Maschine gehalten, bevor sie abgesenkt wird."), dict(STOWED, **{"hood-lift": {"dy": 0.37}, "hood-slide": {"dx": HOOD_DX}}), FLOOR_C, "stored", key=True),
        pose("closed-key", T("Case closed (key)", "Koffer geschlossen (Schlüssel)"), T("The hood sits on the base board and is latched.", "Die Haube sitzt auf dem Bodenbrett und ist verriegelt."), CLOSED, FLOOR_C, "stored", key=True),
        pose("lifted-key", T("Case lifted (key)", "Koffer angehoben (Schlüssel)"), T("The closed case lifted 50 mm off the blanket by the handle.", "Der geschlossene Koffer am Griff 50 mm von der Decke abgehoben."), dict(CLOSED, **{"carry-lift": {"dy": CLEAR["dy"]}}), FLOOR_C + ["grip"], "carried", key=True),
        pose("clear-key", T("Case clear of the table (key)", "Koffer vom Tisch frei (Schlüssel)"), T("The closed case carried level past the front edge of the table before the arm is let hang.", "Der geschlossene Koffer waagrecht über die Vorderkante des Tischs hinausgetragen, bevor der Arm hängen gelassen wird."), dict(CLOSED, **{"carry-lift": {"dy": CLEAR["dy"]}, "carry-travel": {"dx": CLEAR["dx"]}}), FLOOR_C + ["grip"], "carried", key=True),
    ]


def spans(anchors_, a, b):
    """🦶 Contact spans."""
    return [{"anchor": x, "fromS": a, "toS": b} for x in anchors_]


def stitch_keys():
    """🪡 Twelve stitches at five per second: needle and take-up lever down and up, handwheel half a turn per key, fabric fed 2.5 mm per stitch."""
    keys = []
    for k in range(25):
        t = round(0.1 * k, 3)
        down = k % 2 == 1
        keys.append({"t": t, "pose": "in-use", "ease": "smooth", "bones": {"needle-bar": {"dy": -NEEDLE_STROKE if down else 0.0}, "take-up": {"dy": -NEEDLE_STROKE if down else 0.0}, "handwheel": {"angleDeg": 180.0 * k}, "fabric": {"dx": -0.00125 * k}}})
    return keys


def clips():
    """🎞 Carry out shut, and twelve stitches at the table."""
    return [
        {"id": "carry", "title": T("Pack up and carry out", "Einpacken und hinaustragen"), "description": T("The pedal with its coil is put into the throat under the arm, the hood is lifted from behind the machine, set over it and latched, then the case is lifted by its handle and carried away past the table edge; the table and the blanket stay behind.", "Der Anlasser mit Kabelrolle wird in den Durchgang unter dem Arm gelegt, die Haube hinter der Maschine angehoben, darübergesetzt und verriegelt, dann wird der Koffer am Griff angehoben und über die Tischkante hinaus weggetragen; Tisch und Decke bleiben zurück."), "durationS": 10, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.2, "pose": "pedal-raised-key", "ease": "smooth"}, {"t": 2.4, "pose": "pedal-stowed-key", "ease": "smooth"}, {"t": 3.6, "pose": "hood-raised-key", "ease": "smooth"}, {"t": 5.0, "pose": "hood-over-key", "ease": "smooth"}, {"t": 6.0, "pose": "closed-key", "ease": "smooth"}, {"t": 7.2, "pose": "lifted-key", "ease": "smooth"}, {"t": 8.6, "pose": "clear-key", "ease": "smooth"}, {"t": 10, "pose": "carried", "ease": "smooth"}],
         "contacts": spans(FLOOR_C, 0, 10) + spans(["grip"], 6.0, 10),
         "phases": [{"id": "stow-pedal", "fromS": 0, "toS": 2.4, "title": T("Stow the pedal", "Anlasser verstauen")}, {"id": "close-hood", "fromS": 2.4, "toS": 6.0, "title": T("Put the hood on", "Haube aufsetzen")}, {"id": "lift", "fromS": 6.0, "toS": 7.2, "title": T("Lift by the handle", "Am Griff anheben")}, {"id": "carry-away", "fromS": 7.2, "toS": 10, "title": T("Carry away", "Wegtragen")}],
         "events": [{"t": 2.4, "kind": "motion", "title": T("Pedal in the throat", "Anlasser im Durchgang")}, {"t": 6.0, "kind": "contact-on", "title": T("Hand takes the handle", "Hand fasst den Griff"), "target": "grip"}, {"t": 7.2, "kind": "motion", "title": T("Case off the blanket", "Koffer von der Decke gelöst")}],
         "tracks": [{"target": "region:housing:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 5.0, "v": 1}, {"t": 6.0, "v": 0}]}, {"target": "region:blanket:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.4}, {"t": 7.2, "v": 0.4}, {"t": 8.0, "v": 0.5}]}]},
        {"id": "stitching", "title": T("Twelve stitches", "Zwölf Stiche"), "description": T("At the table with lamp on and the pedal pressed: the needle bar and the take-up lever go down and up 30 mm five times a second, the handwheel turns once per stitch and the feed moves the fabric 2.5 mm back with every stitch.", "Am Tisch mit eingeschalteter Lampe und gedrücktem Anlasser: Nadelstange und Fadenhebel gehen fünfmal pro Sekunde 30 mm auf und ab, das Handrad dreht sich je Stich einmal und der Transporteur schiebt den Stoff mit jedem Stich 2,5 mm nach hinten."), "durationS": 2.4, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": stitch_keys(),
         "contacts": spans(FLOOR_C + PEDAL_C, 0, 2.4),
         "phases": [{"id": "sew", "fromS": 0, "toS": 2.4, "title": T("Sewing a seam", "Eine Naht nähen")}],
         "events": [{"t": 0.1, "kind": "motion", "title": T("First stitch: the needle pierces the fabric", "Erster Stich: die Nadel durchsticht den Stoff")}, {"t": 2.3, "kind": "motion", "title": T("Twelfth stitch", "Zwölfter Stich")}],
         "tracks": [{"target": "operating:motor", "ease": "linear", "keys": [{"t": 0, "v": 0.6}, {"t": 2.4, "v": 0.6}]}]},
    ]


def structure_model():
    """🏗 Supports and assumptions."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": FLOOR_C, "clinical": False, "assumptions": [
        T("Stored: one resting group on the folded blanket: the machine (8.0 kg) on the base board (1.1 kg) on four rubber feet, the empty hood (1.9 kg) on its rim, the pedal (0.45 kg) with its coil; the blanket lies on the 15.2 kg sewing table, whose four 45 mm beech legs stand on the hallway floor (four floor contacts, friction 0.4).", "Gelagert: eine ruhende Gruppe auf der gefalteten Decke: die Maschine (8,0 kg) auf dem Bodenbrett (1,1 kg) auf vier Gummifüßen, die leere Haube (1,9 kg) auf ihrem Rand, der Anlasser (0,45 kg) mit Kabelrolle; die Decke liegt auf dem 15,2 kg schweren Nähtisch, dessen vier 45-mm-Buchenbeine auf dem Flurboden stehen (vier Bodenkontakte, Reibung 0,4)."),
        T("Carried: the hood is latched to the base board, so the 11.5 kg of the closed case hang from the handle in one hand beside the body (DGUV 208-033: short carrying of 10-12 kg acceptable); the handle posts and the two latches carry the weight of machine and base board.", "Getragen: die Haube ist am Bodenbrett verriegelt, die 11,5 kg des geschlossenen Koffers hängen am Griff in einer Hand seitlich am Körper (DGUV 208-033: kurzes Tragen von 10-12 kg zumutbar); Griffstützen und die zwei Riegel tragen das Gewicht von Maschine und Bodenbrett."),
        T("In use: the machine stays on the folded blanket on its own 0.75 m table, which carries it to the floor through its four legs (table height after DIN 18011); the pedal stands on the floor under a resting foot (30 N) and the hands rest on the bed with 10 N (authored-estimates of the order of household loads).", "In Gebrauch: die Maschine bleibt auf der gefalteten Decke auf ihrem eigenen 0,75 m hohen Tisch, der sie über seine vier Beine auf den Boden abträgt (Tischhöhe nach DIN 18011); der Anlasser steht auf dem Boden unter einem ruhenden Fuß (30 N), die Hände liegen mit 10 N auf dem Bett (Schätzwerte in der Größenordnung von Haushaltslasten)."),
        T("Limitation of the solver: the asset is one instantaneously rigid body on springs of equal stiffness, so in the carried and in-use poses the blanket, the case and the pedal, which are then apart, share their reactions linearly over all active contacts instead of each carrying its own weight; the stored pose is exact.", "Grenze des Lösers: das Objekt ist ein momentan starrer Körper auf gleich steifen Federn, getragen und in Gebrauch teilen sich Decke, Koffer und Anlasser, die dann getrennt sind, ihre Reaktionen deshalb linear über alle aktiven Kontakte, statt dass jeder sein eigenes Gewicht trägt; die Lagerpose ist exakt."),
        T("Members: die-cast AlSi9Cu3 C-frame (EN 1706, E 72 GPa, 140 MPa proof), P2 chipboard base board (EN 312), ABS hood and pedal shells; masses: blanket, board and hood from section x density, machine 8.0 kg and pedal 0.45 kg catalogue-typical (shell, motor and mechanism are not modelled one by one).", "Stäbe: C-Rahmen aus Druckguss AlSi9Cu3 (EN 1706, E 72 GPa, 140 MPa Dehngrenze), Bodenbrett aus Spanplatte P2 (EN 312), Haube und Anlasser als ABS-Schalen; Massen: Decke, Brett und Haube aus Querschnitt x Dichte, Maschine 8,0 kg und Anlasser 0,45 kg katalogtypisch (Schale, Motor und Mechanik sind nicht einzeln modelliert)."),
    ]}


PENS = {50: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, 100: {"body": 0.2, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, 200: {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350)}
HATCHES = [{"id": "quilt", "title": T("Quilting of the moving blanket", "Steppung der Möbeldecke"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1}, {"id": "ribs", "title": T("Ribs of the pedal tread", "Rippen der Trittplatte"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.45, "penMm": 0.1}]


def decisions(scale):
    """📝 The drawing decisions of one scale."""
    D = {
        50: dict(
            silhouette=T("The set-out group on its table 16 x 23 mm on paper. Side elevation from the needle end: the table with its 25 mm top, two legs and the apron rail, on it the folded blanket with its rounded fold, the hood standing behind, the base board on its feet, the head with its face cover, the needle bar, needle, presser foot and fabric under it, the spool on top, the stitch dial and the pillar seen through the throat, the pedal with tread and coil in front. Front elevation from the sewer: the C of bed, pillar, arm and head with the handwheel on the right, the two dials, the spool, the needle over the fabric, the hood behind, the pedal and its coil in front. Plan: blanket, hood top with handle and sticker, base board, the bed with needle plate and seam, arm, head, spool, dials, handwheel, pedal with ribbed tread and the coil ring.", "Die bereitgestellte Gruppe auf ihrem Tisch 16 x 23 mm auf dem Papier. Seitenansicht von der Nadelseite: der Tisch mit 25-mm-Platte, zwei Beinen und Zarge, darauf die gefaltete Decke mit gerundetem Falz, die Haube dahinter, das Bodenbrett auf seinen Füßen, der Kopf mit Kopfdeckel, Nadelstange, Nadel, Nähfuß und Stoff darunter, die Garnrolle oben, Stichwahl und Säule durch den Durchgang gesehen, vorn der Anlasser mit Trittplatte und Kabelrolle. Vorderansicht von der Nähenden: das C aus Bett, Säule, Arm und Kopf mit dem Handrad rechts, die zwei Regler, die Garnrolle, die Nadel über dem Stoff, die Haube dahinter, vorn Anlasser und Kabelrolle. Grundriss: Decke, Haubendeckel mit Griff und Aufkleber, Bodenbrett, das Bett mit Stichplatte und Fuge, Arm, Kopf, Garnrolle, Regler, Handrad, Anlasser mit gerippter Trittplatte und der Ring der Kabelrolle."),
            retained=T("Traces of use: the paint worn through on the bed front where the hands rest, scratches on the bed, the yellowed face cover, the crack at the upper corner of the hood and an old sticker on it, the depot price tag on the hood side, the scuffed base board corners, the tape round the cable coil; the thread path from spool to needle.", "Gebrauchsspuren: der an der Bettvorderkante durchgeriebene Lack, wo die Hände liegen, Kratzer auf dem Bett, der vergilbte Kopfdeckel, der Sprung an der oberen Haubenecke und ein alter Aufkleber darauf, das Preisschild an der Haubenseite, die bestoßenen Ecken des Bodenbretts, das Klebeband um die Kabelrolle; der Fadenlauf von der Garnrolle zur Nadel."),
            merged=T("Arm and handwheel lie behind the head in the side elevation; the far pillar shows through the throat.", "Arm und Handrad liegen in der Seitenansicht hinter dem Kopf; die hintere Säule zeigt sich durch den Durchgang."),
            omitted=T("The feed dogs, the bobbin case, the screws of the covers and the 2 mm wall of the hood rim, all under 7.5 mm or inside.", "Transporteur, Spulenkapsel, Deckelschrauben und die 2-mm-Wand des Haubenrands, alle unter 7,5 mm oder innen."),
            thinElements=T("The 1.8 mm needle is an open stroke; the 1 mm fabric and the 3 mm hood top are filled strips carried by the body pen.", "Die 1,8-mm-Nadel ist ein offener Strich; der 1-mm-Stoff und der 3-mm-Haubendeckel sind gefüllte Streifen, die der Körperstrich trägt."),
            hatching=T("Cross quilting on the blanket in plan, ribs on the pedal tread.", "Kreuzsteppung der Decke im Grundriss, Rippen auf der Trittplatte."),
            styles=T("Housing 0.85, face cover 0.9, hood 0.78, handwheel 0.7, base board 0.55, spool 0.45, fabric 0.4, dials and pedal 0.25 to 0.35, blanket 0.3, coil 0.15.", "Gehäuse 0,85, Kopfdeckel 0,9, Haube 0,78, Handrad 0,7, Bodenbrett 0,55, Garnrolle 0,45, Stoff 0,4, Regler und Anlasser 0,25 bis 0,35, Decke 0,3, Kabelrolle 0,15."),
            overlays=T("Every part carries its thermal region: housing, lamp cover, handwheel, dials, needle and foot, spool, fabric, base board, hood, pedal, cable, blanket, table top and table frame.", "Jedes Teil trägt seine Thermikregion: Gehäuse, Lampendeckel, Handrad, Regler, Nadel und Fuß, Garnrolle, Stoff, Bodenbrett, Haube, Anlasser, Kabel, Decke, Tischplatte und Tischgestell.")),
        100: dict(
            silhouette=T("The group on its table 8 x 11 mm: the table top, legs and apron rails and the same parts without the small controls: C-frame, face cover, handwheel, stitch dial, spool, needle column and foot, fabric, base board, hood with handle, pedal wedge, coil, blanket with fold.", "Die Gruppe auf ihrem Tisch 8 x 11 mm: Tischplatte, Beine und Zargen und dieselben Teile ohne die kleinen Bedienteile: C-Rahmen, Kopfdeckel, Handrad, Stichwahl, Garnrolle, Nadelsäule und Fuß, Stoff, Bodenbrett, Haube mit Griff, Anlasserkeil, Kabelrolle, Decke mit Falz."),
            retained=T("The thread path, the crack of the hood, the bed seam and the needle plate.", "Der Fadenlauf, der Haubensprung, die Bettfuge und die Stichplatte."),
            merged=T("Feet into the base board, tread into the pedal, shank into the foot.", "Füße ins Bodenbrett, Trittplatte in den Anlasser, Schaft in den Fuß."),
            omitted=T("Bobbin winder, take-up lever, needle clamp, worn paint, scratches, stickers, price tag and tape.", "Spuler, Fadenhebel, Nadelklemme, abgeriebener Lack, Kratzer, Aufkleber, Preisschild und Klebeband."),
            thinElements=T("Needle as an open stroke; fabric, hood top and blanket as strips carried by the 0.2 mm body pen.", "Nadel als offener Strich; Stoff, Haubendeckel und Decke als Streifen, die der 0,2-mm-Körperstrich trägt."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All fourteen regions.", "Alle vierzehn Regionen.")),
        200: dict(
            silhouette=T("The group on its table 4 x 5.7 mm: the table as top strip, four legs and 30 mm apron rails; the C-frame as bed, pillar, arm and head boxes with the face cover, the handwheel disc, one control block, the spool, the needle column, the hood with a block handle, the pedal wedge with the coil puck, base board and blanket strips.", "Die Gruppe auf ihrem Tisch 4 x 5,7 mm: der Tisch als Plattenstreifen, vier Beine und 30-mm-Zargen; der C-Rahmen als Quader aus Bett, Säule, Arm und Kopf mit Kopfdeckel, die Handradscheibe, ein Bedienblock, die Garnrolle, die Nadelsäule, die Haube mit Blockgriff, der Anlasserkeil mit dem Puck der Kabelrolle, Bodenbrett- und Deckenstreifen."),
            retained=T("The throat of the C under the arm, the hood standing behind and the pedal in front.", "Der Durchgang des C unter dem Arm, die Haube dahinter und der Anlasser davor."),
            merged=T("Both dials into one block, needle bar, presser bar and foot into one column, coil into a puck.", "Beide Regler zu einem Block, Nadelstange, Drückerstange und Fuß zu einer Säule, Kabelrolle zu einem Puck."),
            omitted=T("Every detail stroke and trace of use.", "Jeder Detailstrich und jede Gebrauchsspur."),
            thinElements=T("Fabric, hood top, base board and blanket stay at their real thickness, carried by the 0.25 mm body pen so they print at 1:350.", "Stoff, Haubendeckel, Bodenbrett und Decke bleiben in echter Dicke und werden vom 0,25-mm-Körperstrich getragen, damit sie bei 1:350 drucken."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All fourteen regions.", "Alle vierzehn Regionen.")),
    }
    return D[scale]


def representations():
    """🖨 Three authored drawings and the unavailable 1:500."""
    out = {}
    for scale in (50, 100, 200):
        lo, hi = BANDS[scale]
        out[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES if scale == 50 else [], "geometry": geometry(scale), "decisions": decisions(scale)}
    out["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T(
        "At 1:500 the set-out group on its table is 1.6 by 2.3 mm on paper, the group without the table 1.4 by 0.8 mm and the machine alone 0.8 by 0.6 mm, under the 3 mm a recognisable drawing needs; its 75 mm bed would be 0.15 mm and the throat under the arm, which makes it a sewing machine, 0.3 mm; draw it at 1:200.",
        "Bei 1:500 misst die bereitgestellte Gruppe auf ihrem Tisch 1,6 mal 2,3 mm auf dem Papier, die Gruppe ohne Tisch 1,4 mal 0,8 mm und die Maschine allein 0,8 mal 0,6 mm, unter den 3 mm, die eine erkennbare Zeichnung braucht; ihr 75-mm-Bett wäre 0,15 mm und der Durchgang unter dem Arm, der sie zur Nähmaschine macht, 0,3 mm; bei 1:200 zeichnen."),
        "limitingFeature": T("The 75 mm bed and arm of the C-frame", "Das 75-mm-Bett und der Arm des C-Rahmens"), "limitingSizeM": 0.075, "alternatives": [{"kind": "scale", "denominator": 200}]}
    return out


def document():
    """📄 The whole asset document."""
    M = masses()
    total = sum(v[0] for k, v in M.items() if k != "_parts")
    asset = {
        "id": "used-sewing-machine",
        "category": "appliances",
        "title": T("Used Sewing Machine", "Gebrauchte Nähmaschine"),
        "summary": T("A second-hand electric free-arm sewing machine of the 1980s, 0.40 x 0.18 x 0.30 m and 8 kg with 70 W motor and 15 W lamp, its ivory enamel yellowed and worn through at the bed edge, set out at a flat clearance on its 0.80 x 0.60 x 0.75 m sewing table on a folded moving blanket with the hood of its hard case beside it and the pedal with its taped cable in front, carried shut by the handle and sewing again at a table.",
                     "Eine gebrauchte elektrische Freiarm-Nähmaschine der 1980er-Jahre, 0,40 x 0,18 x 0,30 m und 8 kg mit 70-W-Motor und 15-W-Lampe, ihr Elfenbeinlack vergilbt und an der Bettkante durchgerieben, bei einer Haushaltsauflösung auf ihrem 0,80 x 0,60 x 0,75 m großen Nähtisch auf einer gefalteten Möbeldecke bereitgestellt, die Haube ihres Hartschalenkoffers daneben und der Anlasser mit abgeklebtem Kabel davor, geschlossen am Griff getragen und am Tisch wieder nähend."),
        "tags": ["used", "household", "second-hand", "utility", "living", "appliance", "sewing-machine", "sewing-table", "electric", "internal-gain", "die-cast-aluminium", "carrying-case", "flat-clearance"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS], "notes": T(
            "Electric free-arm household sewing machine of the German mail-order and department-store mid-range of the 1980s (no brand): 405 x 180 x 295 mm without spool, 8.0 kg, die-cast aluminium C-frame of bed (75 mm), pillar, arm (75 mm) and head with ivory enamel, throat 160 x 140 mm under the arm, handwheel 70 mm, stitch selector 50 mm, needle bar stroke 30 mm, presser lift 7 mm, nameplate 230 V 85 W (motor 70 W, lamp 15 W). Hard case: base board 450 x 240 x 16 mm vinyl-faced chipboard on 8 mm rubber feet, ABS hood 450 x 240 x 350 mm, 3 mm, with handle, 1.9 kg. Foot pedal 120 x 95 x 44 mm, 0.45 kg, with the 3 m mains and pedal cable taped in a coil. Traces of use: enamel yellowed and worn through at the bed front edge, scratches, a cracked hood corner, an old sticker and a price tag, scuffed board corners. Storage aid: a moving blanket 720 x 560 mm folded to 12 mm (0.65 kg).",
            "Elektrische Freiarm-Haushaltsnähmaschine der deutschen Versandhaus- und Kaufhaus-Mittelklasse der 1980er-Jahre (ohne Marke): 405 x 180 x 295 mm ohne Garnrolle, 8,0 kg, C-Rahmen aus Aluminiumdruckguss aus Bett (75 mm), Säule, Arm (75 mm) und Kopf mit Elfenbeinlack, Durchgang 160 x 140 mm unter dem Arm, Handrad 70 mm, Stichwahl 50 mm, Nadelstangenhub 30 mm, Nähfußhub 7 mm, Typenschild 230 V 85 W (Motor 70 W, Lampe 15 W). Hartschalenkoffer: Bodenbrett 450 x 240 x 16 mm Spanplatte mit Vinylfolie auf 8-mm-Gummifüßen, ABS-Haube 450 x 240 x 350 mm, 3 mm, mit Griff, 1,9 kg. Fußanlasser 120 x 95 x 44 mm, 0,45 kg, mit dem 3 m langen Netz- und Anlasserkabel als Rolle abgeklebt. Gebrauchsspuren: Lack vergilbt und an der Bettvorderkante durchgerieben, Kratzer, eine gesprungene Haubenecke, ein alter Aufkleber und ein Preisschild, bestoßene Brettecken. Lagerhilfe: eine Möbeldecke 720 x 560 mm, auf 12 mm gefaltet (0,65 kg).")},
        "dimensions": {"heightM": TABLE["h"] + HOOD["y1"] + HANDLE["h"], "widthM": TABLE["x1"] - TABLE["x0"], "depthM": 2 * TABLE["z"], "massKg": total},
        "materials": materials(),
        "rig": rig(),
        "anchors": ANCHORS,
        "representations": representations(),
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal_model(),
        "structure": structure_model(),
        "volumes": build_volumes(),
        "presentation": {"defaultPose": "stored", "defaultClip": "stitching", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": r6(asset)}


if __name__ == "__main__":
    OUT.write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    M = masses()
    print("[DEBUG] mass", round(sum(v[0] for k, v in M.items() if k != "_parts"), 3), {k: round(v, 3) for k, v in M["_parts"].items()})
