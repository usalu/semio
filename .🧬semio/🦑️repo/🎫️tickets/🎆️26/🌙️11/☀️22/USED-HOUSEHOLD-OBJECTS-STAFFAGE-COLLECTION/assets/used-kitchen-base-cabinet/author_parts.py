"""🗄 The hand-chosen numbers and the per-scale parts of the used kitchen base cabinet: every solid of the body, thermal and structure
lists and every 2D shape of the three views at 1:50, 1:100, 1:200 and 1:500 is stated here in world rest metres.

Rest pose = stored pose: two 600 mm base units screwed together under one 1.20 m laminate worktop, standing on their 150 mm plastic
legs on a moving blanket folded to their footprint. x ahead (front of the doors), y up, z left; unit A (sink unit, one door) left of
the centre plane, unit B (drawer over door) right of it, so the side elevation looks at the outer side panel of unit B.

@see ./author_geom.py — frames and projections
"""
import math
from author_geom import (T, Frame, R6, solid_box, solid_extrude, solid_revolve, solid_capsule, shape, g_rect, g_poly, g_ellipse, g_capsule, g_path_rings, box_views, depth_z, box_volume, poly_area, revolve_volume, circle_pts, rect_ring)

BL = 0.012
LEG = 0.150
CB = BL + LEG
CT = 0.882
WT = 0.038
WTT = CT + WT
CX = 0.560
FT = 0.019
FX = CX + FT
BD = 0.018
HALF = 0.600
MARGIN = 0.00075
FRONT_TOP = 0.8805
DOOR_B_TOP = 0.7405
DRAWER_BOTTOM = 0.742
RIM = (0.05, 0.55, -0.29, 0.57)
BOWL_OPEN = (0.155, 0.505, 0.14, 0.54)
BOWL_OUT = (0.154, 0.506, 0.139, 0.541)
BOWL_FLOOR = 0.761
CUTOUT = (0.062, 0.538, -0.278, 0.558)
WASTE = (0.33, 0.34, 0.045)
TAP = (0.10, WTT + 0.008, 0.34)
LEGS_X = (0.05, 0.47)
LEGS_Z = (-0.565, 0.035)
LEG_STEP = 0.53

FR = {
    "floor": Frame("S", 0, 0, 0),
    "blanket": Frame("S", 0, 0, 0),
    "lift": Frame("S", 0, BL, 0),
    "carcass": Frame("S", 0, BL, 0),
    "worktop": Frame("S", 0, CT, 0),
    "sink": Frame("S", RIM[0], WTT, 0),
    "tap": Frame("S", TAP[0], TAP[1], TAP[2]),
    "door-a": Frame("U", CX, CB, HALF - MARGIN),
    "door-b": Frame("U", CX, CB, -(HALF - MARGIN)),
    "drawer": Frame("S", 0.060, DRAWER_BOTTOM, -0.3),
}

DENSITY = {
    "melamine-chipboard-aged": 650, "swollen-chipboard": 560, "foil-mdf-front": 750, "hdf-back": 850, "hpl-worktop": 680,
    "stainless-steel-sink": 7900, "chrome-brass": 8500, "pp-plastic": 905, "moving-blanket": 380, "aluminium-handle": 2700,
    "stainless-bar-handle": 7900, "zinc-steel-runner": 7850, "braided-hose": 1400,
}

FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}

TONE = {"carcass": 0.86, "inner": 0.78, "front": 0.8, "worktop": 0.55, "sink": 0.62, "tap": 0.35, "plinth": 0.32, "leg": 0.12, "blanket": 0.24, "handle": 0.42, "swollen": 0.48, "back": 0.7, "runner": 0.45}


class Scale:
    """🧱 One authored scale under construction: the three volume lists, the three 2D views and the per-bone masses (1:50 only)."""

    def __init__(self, key):
        self.key = key
        self.body, self.thermal, self.structure = [], [], []
        self.views = {"elevation": [], "front": [], "plan": []}
        self.masses = []

    def add(self, sid, bone, lo, hi, material, region, tone, views=("elevation", "front", "plan"), member=True, role="body", zbias=0.0, solid_extra=None, shape_extra=None, mass_kg=None):
        fr = FR[bone]
        self.body.append(solid_box(sid, bone, fr, lo, hi, material, tone, role, **(solid_extra or {})))
        self.thermal.append({"id": f"th-{sid}", "of": sid, "region": region})
        if member:
            self.structure.append({"id": f"m-{sid}", "of": sid, "member": bone, "part": "member"})
        geo = box_views(fr, bone, lo, hi)
        for v in views:
            g, sol = geo[v]
            extra = {"region": region, "material": material}
            extra.update(shape_extra or {})
            if sol:
                extra["solid"] = sol
            self.views[v].append(shape(sid, bone, role, depth_z(v, lo, hi, zbias), True, tone, g, **extra))
        kg = mass_kg if mass_kg is not None else box_volume(lo, hi) * DENSITY[material]
        self.masses.append((bone, kg, ((lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2, (lo[2] + hi[2]) / 2)))

    def solid(self, s, region, member=True, part="member", mass=None):
        self.body.append(s)
        self.thermal.append({"id": f"th-{s['id']}", "of": s["id"], "region": region})
        if member:
            self.structure.append({"id": f"m-{s['id']}", "of": s["id"], "member": s["bone"], "part": part})
        if mass is not None:
            self.masses.append((s["bone"], mass[0], mass[1]))

    def draw(self, view, sid, bone, role, z, filled, tone, geom, **extra):
        self.views[view].append(shape(sid, bone, role, z, filled, tone, geom, **extra))

    def mass_marker(self, bone, at_world, r):
        fr = FR[bone]
        c = fr.local(*at_world)
        self.structure.append({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": "melamine-chipboard-aged", "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [R6(v) for v in c]}, "member": bone, "part": "mass"})


def blanket_profile():
    """🛏 Side section of the folded blanket: a flat quilt of 12 mm from x 0 to 0.58 with the two folds rounded at 6 mm, counter-clockwise."""
    r = BL / 2
    front = [(0.580 - r + r * math.cos(-math.pi / 2 + math.pi * k / 6), r + r * math.sin(-math.pi / 2 + math.pi * k / 6)) for k in range(7)]
    back = [(r + r * math.cos(math.pi / 2 + math.pi * k / 6), r + r * math.sin(math.pi / 2 + math.pi * k / 6)) for k in range(7)]
    return front + back


def worktop_end(nose):
    """🪵 The worktop end profile (bone-local u, v): 38 mm laminate board, the postformed front with radius `nose` (0 = square)."""
    if nose <= 0:
        return [(0, 0), (0.600, 0), (0.600, WT), (0, WT)]
    arc = [(0.600 - nose + nose * math.cos(math.pi / 2 * k / 4), WT - nose + nose * math.sin(math.pi / 2 * k / 4)) for k in range(5)]
    return [(0, 0), (0.597, 0), (0.600, 0.003)] + arc + [(0, WT)]


def nose_profile():
    """🪵 The postformed nose strip (u 0.590..0.600) with its 10 mm top radius and the 3 mm lower chamfer."""
    arc = [(0.590 + 0.010 * math.cos(math.pi / 2 * k / 4), 0.028 + 0.010 * math.sin(math.pi / 2 * k / 4)) for k in range(5)]
    return [(0.590, 0), (0.597, 0), (0.600, 0.003)] + arc


def swelling_profile():
    """💧 The water-swollen lip under the worktop front over the sink unit (u, v below the underside)."""
    return [(0.584, 0.0), (0.600, 0.0), (0.600, -0.004), (0.597, -0.0075), (0.590, -0.0085), (0.586, -0.006)]


def leg_profile(detail):
    """🦵 Adjustable plastic leg (axial v up from the blanket, radial): foot Ø50, shaft Ø28, head plate Ø44 at 1:50; a plain Ø30 cylinder coarser."""
    if detail == "50":
        return [(0, 0), (0, 0.025), (0.010, 0.025), (0.014, 0.014), (0.140, 0.014), (0.144, 0.022), (LEG, 0.022), (LEG, 0)]
    return [(0, 0), (0, 0.015), (LEG, 0.015), (LEG, 0)]


def tap_profile(detail):
    """🚰 The single-lever mixer body (axial v up from the 1:50 sink deck top, radial): rosette Ø56, body Ø45, chamfered head; coarser scales start 7 mm lower on the flat sink deck that replaces the rolled rim."""
    if detail == "50":
        return [(0, 0), (0, 0.028), (0.005, 0.028), (0.005, 0.0225), (0.104, 0.0225), (0.112, 0.016), (0.112, 0)]
    if detail == "100":
        return [(-0.007, 0), (-0.007, 0.0225), (0.112, 0.0225), (0.112, 0)]
    if detail == "200":
        return [(-0.007, 0), (-0.007, 0.0225), (0.135, 0.0225), (0.135, 0)]
    return [(-0.007, 0), (-0.007, 0.0375), (1.071 - TAP[1], 0.0375), (1.071 - TAP[1], 0)]


def mirrored(profile):
    """🪞 The full outline of a revolved profile seen side-on (radial mirrored about the axis), in (radial, axial)."""
    right = [(r, a) for a, r in profile if r > 0]
    left = [(-r, a) for r, a in reversed(right)]
    return right + left


def legs(sc, detail):
    """🦵 The eight legs: four repeat solids of two legs each, drawn in the elevation (the nearer row); front and plan see them only behind the plinth and under the carcass."""
    prof = leg_profile(detail)
    vol = revolve_volume(prof)
    k = 0
    for x in LEGS_X:
        for z in LEGS_Z:
            k += 1
            sid = f"legs-{k}"
            s = solid_revolve(sid, "carcass", prof, [0, 1, 0], [x, 0, z], "pp-plastic", TONE["leg"], repeat={"count": 2, "translation": [0, 0, LEG_STEP]})
            sc.solid(s, "plinth", mass=(2 * vol * DENSITY["pp-plastic"], (x, BL + LEG / 2, z + LEG_STEP / 2)))
    outline = mirrored(prof)
    for x in LEGS_X:
        sc.draw("elevation", f"leg-near-{int(x*100)}", "carcass", "body", 7.0, True, TONE["leg"], g_poly([(x + r, a) for r, a in outline]), region="plinth", material="pp-plastic")


def tap_parts(sc, detail):
    """🚰 The cut-off single-lever mixer on the back ledge of the sink: body, lever and swivel spout; at 1:50 and 1:100 also the shank and the two braided hoses cut through below the sink."""
    prof = tap_profile(detail)
    body = solid_revolve("tap-body", "tap", prof, [0, 1, 0], [0, 0, 0], "chrome-brass", TONE["tap"])
    hollow = revolve_volume(prof) - revolve_volume([(a, max(r - 0.003, 0)) for a, r in prof])
    sc.solid(body, "tap", mass=(hollow * DENSITY["chrome-brass"] + 0.25, (TAP[0], TAP[1] + 0.05, TAP[2])))
    outline = mirrored(prof)
    sc.draw("elevation", "tap-body", "tap", "body", 9.0, True, TONE["tap"], g_poly([(r, a) for r, a in outline]), region="tap", material="chrome-brass")
    sc.draw("front", "tap-body", "tap", "body", 9.0, True, TONE["tap"], g_poly([(TAP[2] + r, TAP[1] + a) for r, a in outline]), region="tap", material="chrome-brass")
    rmax = max(r for _, r in prof)
    sc.draw("plan", "tap-body", "tap", "body", 9.0, True, TONE["tap"], g_ellipse(TAP[0], TAP[2], rmax, rmax), region="tap", material="chrome-brass")
    if detail in ("50", "100"):
        lever = solid_capsule("tap-lever", "tap", (0.0, 0.108, 0), (-0.070, 0.135, 0), 0.010 if detail == "50" else 0.0085, 0.007 if detail == "50" else 0.0085, "chrome-brass", TONE["tap"])
        sc.solid(lever, "tap", mass=(0.12, (TAP[0] - 0.035, TAP[1] + 0.12, TAP[2])))
        r0, r1 = lever["r0"], lever["r1"]
        sc.draw("elevation", "tap-lever", "tap", "body", 9.1, True, TONE["tap"], g_capsule(0.0, 0.108, -0.070, 0.135, r0, r1), region="tap", material="chrome-brass")
        sc.draw("front", "tap-lever", "tap", "body", 9.1, True, TONE["tap"], g_capsule(TAP[2], TAP[1] + 0.108, TAP[2], TAP[1] + 0.135, r0, r1), region="tap", material="chrome-brass")
        sc.draw("plan", "tap-lever", "tap", "body", 9.2, True, TONE["tap"], g_capsule(TAP[0], TAP[2], TAP[0] - 0.070, TAP[2], r0, r1), region="tap", material="chrome-brass")
    if detail != "500":
        rs = 0.010 if detail in ("50", "100") else 0.015
        y0, y1 = (0.072, 0.058) if detail != "200" else (0.080, 0.066)
        spout = solid_capsule("tap-spout", "tap", (0.0, y0, 0), (0.205, y1, 0), rs, rs * 0.9 if detail == "50" else rs, "chrome-brass", TONE["tap"])
        sc.solid(spout, "tap", mass=(0.30, (TAP[0] + 0.1, TAP[1] + 0.065, TAP[2])))
        sc.draw("elevation", "tap-spout", "tap", "body", 9.2, True, TONE["tap"], g_capsule(0.0, y0, 0.205, y1, spout["r0"], spout["r1"]), region="tap", material="chrome-brass")
        sc.draw("front", "tap-spout", "tap", "body", 9.3, True, TONE["tap"], g_capsule(TAP[2], TAP[1] + y1, TAP[2], TAP[1] + y0, spout["r1"], spout["r0"]), region="tap", material="chrome-brass")
        sc.draw("plan", "tap-spout", "tap", "body", 9.1, True, TONE["tap"], g_capsule(TAP[0], TAP[2], TAP[0] + 0.205, TAP[2], spout["r0"], spout["r1"]), region="tap", material="chrome-brass")
    if detail in ("50", "100"):
        top = 0.0 if detail == "50" else -0.007
        shank = solid_revolve("tap-shank", "tap", [(-0.076, 0), (-0.076, 0.015), (top, 0.015), (top, 0)], [0, 1, 0], [0, 0, 0], "chrome-brass", TONE["tap"])
        sc.solid(shank, "tap", mass=(0.20, (TAP[0], TAP[1] - 0.038, TAP[2])))
        for side, w in (("r", -1), ("l", 1)):
            h = solid_capsule(f"hose-{side}", "tap", (-0.004, -0.076, 0.006 * w), (0.0, -0.300, 0.022 * w), 0.005, 0.005, "braided-hose", 0.5, slender=True)
            sc.solid(h, "tap", member=False, mass=(0.06, (TAP[0], TAP[1] - 0.19, TAP[2] + 0.014 * w)))
