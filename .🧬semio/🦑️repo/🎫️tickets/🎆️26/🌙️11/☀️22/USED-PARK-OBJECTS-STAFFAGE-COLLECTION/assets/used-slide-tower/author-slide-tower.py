"""🛝 Authoritative author script of the staffage asset `used-slide-tower`: every hand-chosen number of the used municipal play tower
(1.5 m platform, gable roof, rung ladder, stainless steel slide, point foundations) is stated here in world metres, each preset scale is
its own model, and the 2D drawings are the exact orthographic projections of that scale's parts. Writes `🔣️.json` next to this file.

World frame of the archive: x ahead (towards the slide), y up (ground surface y = 0), z to the asset's left.

@see ../../brief.md — the author brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the asset contract
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
TAN40 = math.tan(math.radians(40))
COS40 = math.cos(math.radians(40))
TAN70 = math.tan(math.radians(70))
SIN70 = math.sin(math.radians(70))

PC, PS = 0.60, 0.12
POST_Y0, POST_Y1 = 0.10, 2.60
DECK = 1.50
BEAM_Y0, BEAM_Y1, BT = 1.327, 1.472, 0.045
RAIL_Y0, RAIL_Y1 = 2.155, 2.25
INFILL_Y0, INFILL_Y1 = 1.35, 2.25
OUTER = PC + BT / 2
INFILL_T = 0.022
BOARD_W, BOARD_GAP = 0.14743, 0.008
LAT_N, LAT_W = 5, 0.12
LAT_GAP = (1.08 - LAT_N * LAT_W) / (LAT_N + 1)
FOOTING_Y0, LADDER_FOOT_Y0, SLIDE_FOOT_Y0 = -0.80, -0.60, -0.70
STR_Z0, STR_Z1 = 0.25, 0.295
STR_W = 0.145 / SIN70
FOOT_Y = 0.08
RUNG_R = 0.0225
RUNGS = (0.25, 0.50, 0.75, 1.00, 1.25)
SLIDE_HALF = 0.225
SHEET = 0.002
WALL_H = 0.15
EDGE_R = 0.015
ROOF_EAVE, ROOF_OVER, ROOF_HALF = 0.80, 0.90, 0.80
TIE_Y0, TIE_Y1 = 2.60, 2.70
RAFTER_D = 0.095 / COS40
BOARD_D = 0.024 / COS40
CAP_D = 0.022 / COS40
RIDGE_LOW = TIE_Y1 + TAN40 * ROOF_EAVE
RIDGE_RAFTER_TOP = RIDGE_LOW + RAFTER_D
RIDGE_BOARD_TOP = RIDGE_RAFTER_TOP + BOARD_D
CAP_TOP = RIDGE_BOARD_TOP + CAP_D
EYE_R, EYE_ROD = 0.024, 0.008
EYE_CY = CAP_TOP + 0.008 + EYE_R + EYE_ROD
LEG_R = 0.03015
TILE_X0, TILE_X1 = 3.55, 4.55


def ladder_front(y):
    """📐 Front face x of a ladder stringer at height y (70 degrees, touching the back beam at its top edge)."""
    return -OUTER + (y - BEAM_Y1) / TAN70


def ladder_centre(y):
    """📏 Centre line x of a ladder stringer at height y."""
    return ladder_front(y) - STR_W / 2


def slide_path(step=5.0):
    """🛝 The bed surface of the slide as a polyline (x, y, tangent degrees): start section, 1.0 m transition arc, 35 degree sliding section, 1.2 m transition arc, run-out, rolled end lip."""
    pts = [(OUTER, DECK, 0.0), (1.0225, DECK, 0.0)]
    a = 0.0
    while a < 35 - 1e-9:
        a = min(35.0, a + step)
        r = math.radians(a)
        pts.append((1.0225 + 1.0 * math.sin(r), DECK - 1.0 * (1 - math.cos(r)), -a))
    x1, y1 = pts[-1][0], pts[-1][1]
    bx = x1 + (y1 - (0.30 + 1.2 * (1 - math.cos(math.radians(35))))) / math.tan(math.radians(35))
    cx = bx + 1.2 * math.sin(math.radians(35))
    a = 35.0
    while a > 1e-9:
        a = max(0.0, a - step)
        r = math.radians(a)
        pts.append((cx - 1.2 * math.sin(r), 0.30 + 1.2 * (1 - math.cos(r)), -a))
    pts.append((cx + 0.50, 0.30, 0.0))
    return pts, (bx, cx, cx + 0.50)


PATH, (SLIDE_B, SLIDE_C, SLIDE_D) = slide_path()
PATHS = {50: PATH, 100: slide_path(7.0)[0], 200: slide_path(35 / 3)[0], 500: slide_path(17.5)[0]}
LIP = [(SLIDE_D + 0.05 * math.cos(math.radians(a)), 0.25 + 0.05 * math.sin(math.radians(a))) for a in (75, 60, 45, 30)]
LEG_X = SLIDE_C


def offset_path(pts, d):
    """↕️ The polyline moved by d along its upward normal at every vertex."""
    out = []
    for x, y, t in pts:
        r = math.radians(t)
        out.append((x - d * math.sin(r), y + d * math.cos(r)))
    return out


def band(lower, upper):
    """🎗️ A closed band polygon between two polylines of equal length."""
    return [*lower, *reversed(upper)]




def rect(x0, y0, x1, y1):
    """⬛ A rectangle polygon."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def circle(cx, cy, r, n=24):
    """⭕ A circle polygon."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def area(poly):
    """📐 Signed shoelace area of a polygon."""
    return 0.5 * sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly)))


def centroid(poly):
    """🎯 The area centroid of a simple polygon."""
    a = area(poly)
    cx = sum((poly[i][0] + poly[(i + 1) % len(poly)][0]) * (poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1]) for i in range(len(poly))) / (6 * a)
    cy = sum((poly[i][1] + poly[(i + 1) % len(poly)][1]) * (poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1]) for i in range(len(poly))) / (6 * a)
    return cx, cy


def r6(v):
    """✂️ Rounds to the micrometre."""
    return round(v + 0.0, 6)


# ---------------------------------------------------------------- materials
T = lambda en, de: {"en": en, "de": de}
MATERIALS = [
    {"id": "larch-weathered", "title": T("Weathered larch, greyed", "Verwitterte Lärche, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.68, "densityKgM3": 590, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "williams-2005"},
    {"id": "larch-fresh", "title": T("Fresh larch (replaced board)", "Frische Lärche (ersetztes Brett)"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 590, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "robinia", "title": T("Robinia rungs, shoe-worn", "Robinien-Sprossen, abgetreten"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": 730, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 13600000000, "source": "literature", "reference": "din-68364"},
    {"id": "stainless-scratched", "title": T("Stainless steel 1.4301, scratched", "Edelstahl 1.4301, zerkratzt"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 7900, "conductivityWmK": 15, "specificHeatJKgK": 500, "elasticModulusPa": 200000000000, "strengthPa": 230000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "steel-galvanised-weathered", "title": T("Hot-dip galvanised steel, weathered", "Feuerverzinkter Stahl, verwittert"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "concrete", "title": T("Concrete C25/30 (footings)", "Beton C25/30 (Fundamente)"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 31000000000, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "rubber-granulate", "title": T("Bonded rubber granulate tile", "Fallschutzplatte aus Gummigranulat"), "emissivity": 0.92, "solarAbsorptivity": 0.92, "densityKgM3": 800, "conductivityWmK": 0.17, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "iso-10456-2007"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}
LW, LF, RB, SS, GV, CO, RU = "larch-weathered", "larch-fresh", "robinia", "stainless-scratched", "steel-galvanised-weathered", "concrete", "rubber-granulate"

# ---------------------------------------------------------------- rig (world rest frames: origin, angle; no abduction, no twist)
BONES = []


def bone(id, parent, x, y, z, angle, length, joint, en, de, section=None):
    """🦴 Adds a bone given in its parent's frame and computes its world rest frame."""
    if parent is None:
        o, th, oz = (x, y), angle, z
    else:
        p = next(b for b in BONES if b["id"] == parent)
        c, s = math.cos(math.radians(p["th"])), math.sin(math.radians(p["th"]))
        o, th, oz = (p["o"][0] + c * x - s * y, p["o"][1] + s * x + c * y), p["th"] + angle, p["oz"] + z
    BONES.append({"id": id, "parent": parent, "x": x, "y": y, "z": z, "angle": angle, "length": length, "joint": joint, "title": T(en, de), "section": section, "o": o, "th": th, "oz": oz})


def rect_section(b, h, mat, en, de, n=1):
    """📏 A rectangular member section of n equal bars b x h bending about the h axis."""
    return {"areaM2": r6(n * b * h), "secondMomentM4": float(f"{n * b * h ** 3 / 12:.4e}"), "extremeFibreM": r6(h / 2), "material": mat, "title": T(en, de)}


FIX = {"type": "fixed"}
bone("ground", None, 0, 0, 0, 0, 0.1, FIX, "Ground surface frame with the rubber tiles of the run-out", "Bezugsrahmen der Geländeoberfläche mit den Fallschutzplatten des Auslaufs")
for pid, sx, sz, en, de in (("post-fl", 1, 1, "front left", "vorn links"), ("post-fr", 1, -1, "front right", "vorn rechts"), ("post-bl", -1, 1, "back left", "hinten links"), ("post-br", -1, -1, "back right", "hinten rechts")):
    bone(pid, "ground", sx * PC, FOOTING_Y0, sz * PC, 90, POST_Y1 - FOOTING_Y0, FIX, f"Corner post {en} with its H-anchor and point footing (larch 120 x 120)", f"Eckpfosten {de} mit H-Anker und Punktfundament (Lärche 120 x 120)", rect_section(0.12, 0.12, LW, "Post larch 120 x 120 (EN 338 C24)", "Pfosten Lärche 120 x 120 (EN 338 C24)"))
bone("platform", "ground", -OUTER, BEAM_Y0, 0, 0, 2 * OUTER, FIX, "Platform frame and deck", "Podestrahmen und Belag", rect_section(0.045, 0.145, LW, "Two edge beams 45 x 145 (EN 338 C24)", "Zwei Randbalken 45 x 145 (EN 338 C24)", 2))
bone("barrier-left", "platform", OUTER - 0.54, INFILL_Y0 - BEAM_Y0, OUTER + INFILL_T / 2, 0, 0.15, FIX, "Left barrier (vertical boards and rail)", "Linke Brüstung (senkrechte Bretter und Handlauf)")
bone("barrier-right", "platform", OUTER - 0.54, INFILL_Y0 - BEAM_Y0, -OUTER - INFILL_T / 2, 0, 0.15, FIX, "Right barrier (vertical boards and rail)", "Rechte Brüstung (senkrechte Bretter und Handlauf)")
LADDER_O = (ladder_centre(0.0), 0.0)
bone("ladder", "ground", LADDER_O[0], 0, 0, 70, DECK / SIN70, FIX, "Rung ladder with shoes and strip footing (two larch stringers, robinia rungs)", "Sprossenleiter mit Schuhen und Streifenfundament (zwei Lärchenholme, Robiniensprossen)", rect_section(0.045, 0.145, LW, "Two stringers 45 x 145 (EN 338 C24)", "Zwei Holme 45 x 145 (EN 338 C24)", 2))
SLIDE_ANGLE = math.degrees(math.atan2(0.30 - DECK, SLIDE_C - OUTER))
bone("slide", "ground", OUTER, DECK, 0, SLIDE_ANGLE, math.hypot(SLIDE_C - OUTER, DECK - 0.30), FIX, "Stainless steel slide (bed with folded side walls)", "Edelstahlrutsche (Bett mit gekanteten Seitenwangen)", None)
bone("slide-leg", "ground", LEG_X, SLIDE_FOOT_Y0, 0, 90, 0.30 - SHEET - SLIDE_FOOT_Y0, FIX, "Run-out support leg with its footing (tube 60.3 x 2.9)", "Auslaufstütze mit Fundament (Rohr 60,3 x 2,9)", {"areaM2": 0.0005229, "secondMomentM4": 2.159e-07, "extremeFibreM": 0.03015, "material": GV, "title": T("Tube 60.3 x 2.9 (EN 10219-2)", "Rohr 60,3 x 2,9 (EN 10219-2)")})
bone("roof-carry", "platform", OUTER, TIE_Y0 - BEAM_Y0, 0, 0, 0.1, {"type": "prismatic", "axisDeg": 0, "minM": -2.6, "maxM": 0}, "Crane travel of the dismantled roof (horizontal)", "Kranweg des abgebauten Dachs (waagerecht)")
bone("roof", "roof-carry", -ROOF_EAVE, 0, 0, 0, 2 * ROOF_EAVE, {"type": "prismatic", "axisDeg": 90, "minM": -2.7, "maxM": 0.6}, "Gable roof (two gable frames, boards, ridge, lifting eyes)", "Satteldach (zwei Giebelbinder, Schalung, First, Anschlagösen)", rect_section(0.045, 0.10, LW, "Two tie beams 45 x 100 of the gable frames", "Zwei Zugbalken 45 x 100 der Giebelbinder", 2))

# bed channel section of the slide: bed 0.45 x 0.002, two walls 0.15 x 0.002
_ab, _aw = 0.45 * SHEET, WALL_H * SHEET
_yc = (2 * _aw * WALL_H / 2) / (_ab + 2 * _aw)
_I = _ab * _yc ** 2 + 2 * (SHEET * WALL_H ** 3 / 12 + _aw * (WALL_H / 2 - _yc) ** 2)
next(b for b in BONES if b["id"] == "slide")["section"] = {"areaM2": r6(_ab + 2 * _aw), "secondMomentM4": float(f"{_I:.4e}"), "extremeFibreM": r6(WALL_H - _yc), "material": SS, "title": T("Bed channel 450 x 150 x 2 (1.4301)", "Bettrinne 450 x 150 x 2 (1.4301)")}
BONE = {b["id"]: b for b in BONES}


def to_local(bid, x, y):
    """🔁 World (x, y) to the rest frame (u, v) of a bone."""
    b = BONE[bid]
    c, s = math.cos(math.radians(b["th"])), math.sin(math.radians(b["th"]))
    dx, dy = x - b["o"][0], y - b["o"][1]
    return (c * dx + s * dy, -s * dx + c * dy)


def quat(bid):
    """🌀 The rotation that turns world-aligned offsets into the bone frame (about w by minus the bone angle)."""
    th = math.radians(-BONE[bid]["th"])
    return [0.0, 0.0, r6(math.sin(th / 2)), r6(math.cos(th / 2))]


LADDER_ANCHOR = tuple(r6(c) for c in to_local("ladder", ladder_centre(FOOT_Y), LADDER_FOOT_Y0))


# ---------------------------------------------------------------- parts
class Part:
    """🧩 One physical part in world coordinates, with the data its solid and its three drawings need."""

    def __init__(self, id, bone, kind, mat, region, tone, role="body", views=("elevation", "front", "plan"), hatch=None, sheet=False, slender=False, envelope=False, fill=1.0, draw=True, build=True, **g):
        self.fill, self.draw, self.build = fill, draw, build
        self.id, self.bone, self.kind, self.mat, self.region, self.tone, self.role, self.views, self.hatch = id, bone, kind, mat, region, tone, role, views, hatch
        self.sheet, self.slender, self.envelope, self.g = sheet, slender, envelope, g

    def bounds(self):
        """📦 World bounding box (x0, x1, y0, y1, z0, z1)."""
        g = self.g
        if self.kind == "box":
            (cx, cy, cz), (sx, sy, sz) = g["c"], g["s"]
            return cx - sx / 2, cx + sx / 2, cy - sy / 2, cy + sy / 2, cz - sz / 2, cz + sz / 2
        if self.kind == "zp":
            xs, ys = [p[0] for p in g["prof"]], [p[1] for p in g["prof"]]
            return min(xs), max(xs), min(ys), max(ys), g["z0"], g["z1"]
        if self.kind == "rbox":
            (cx, cy, cz), (sx, sy, sz), a = g["c"], g["s"], math.radians(g["angle"])
            ex, ey = abs(math.cos(a)) * sx / 2 + abs(math.sin(a)) * sy / 2, abs(math.sin(a)) * sx / 2 + abs(math.cos(a)) * sy / 2
            return cx - ex, cx + ex, cy - ey, cy + ey, cz - sz / 2, cz + sz / 2
        if self.kind == "xp":
            zs, ys = [p[0] for p in g["prof"]], [p[1] for p in g["prof"]]
            return g["x0"], g["x1"], min(ys), max(ys), min(zs), max(zs)
        if self.kind == "yp":
            xs, zs = [p[0] for p in g["prof"]], [p[1] for p in g["prof"]]
            return min(xs), max(xs), g["y0"], g["y1"], min(zs), max(zs)
        if self.kind == "cyl":
            (x, y, z), L, r, ax = g["c"], g["L"], g["r"], g["axis"]
            lo = [x - r, y - r, z - r]
            hi = [x + r, y + r, z + r]
            i = "xyz".index(ax)
            lo[i], hi[i] = (x, y, z)[i], (x, y, z)[i] + L
            return lo[0], hi[0], lo[1], hi[1], lo[2], hi[2]
        if self.kind == "cap":
            p0, p1, r = g["p0"], g["p1"], g["r"]
            return min(p0[0], p1[0]) - r, max(p0[0], p1[0]) + r, min(p0[1], p1[1]) - r, max(p0[1], p1[1]) + r, min(p0[2], p1[2]) - r, max(p0[2], p1[2]) + r
        if self.kind == "torus":
            (x, y, z), R, r = g["c"], g["R"], g["r"]
            return x - R - r, x + R + r, y - R - r, y + R + r, z - r, z + r
        if self.kind == "loft":
            pts, r = g["pts"], g["r"]
            return min(p[0] for p in pts) - r, max(p[0] for p in pts) + r, min(p[1] for p in pts) - r, max(p[1] for p in pts) + r, g["z"] - r, g["z"] + r
        raise ValueError(self.kind)

    def volume(self):
        """⚖️ Volume (m3) and world centroid of the part."""
        g = self.g
        if self.kind in ("box", "rbox"):
            sx, sy, sz = g["s"]
            return sx * sy * sz, g["c"]
        if self.kind == "zp":
            a = abs(area(g["prof"]))
            cx, cy = centroid(g["prof"])
            return a * (g["z1"] - g["z0"]), (cx, cy, (g["z0"] + g["z1"]) / 2)
        if self.kind == "xp":
            a = abs(area(g["prof"]))
            cz, cy = centroid(g["prof"])
            return a * (g["x1"] - g["x0"]), ((g["x0"] + g["x1"]) / 2, cy, cz)
        if self.kind == "yp":
            a = abs(area(g["prof"]))
            cx, cz = centroid(g["prof"])
            return a * (g["y1"] - g["y0"]), (cx, (g["y0"] + g["y1"]) / 2, cz)
        if self.kind == "cyl":
            c, L, r, ax = list(g["c"]), g["L"], g["r"], g["axis"]
            c["xyz".index(ax)] += L / 2
            return math.pi * r * r * L, tuple(c)
        if self.kind == "cap":
            p0, p1, r = g["p0"], g["p1"], g["r"]
            L = math.dist(p0, p1)
            return math.pi * r * r * L + 4 / 3 * math.pi * r ** 3, tuple((a + b) / 2 for a, b in zip(p0, p1))
        if self.kind == "torus":
            return 2 * math.pi ** 2 * g["R"] * g["r"] ** 2, g["c"]
        if self.kind == "loft":
            pts, r = g["pts"], g["r"]
            L = sum(math.dist(pts[i][:2], pts[i + 1][:2]) for i in range(len(pts) - 1))
            mid = pts[len(pts) // 2]
            return math.pi * r * r * L, (mid[0], mid[1], g["z"])
        raise ValueError(self.kind)


def box(id, bone, x0, x1, y0, y1, z0, z1, mat, region, tone, **k):
    """📦 An axis-aligned box part from its world extents."""
    return Part(id, bone, "box", mat, region, tone, c=((x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2), s=(x1 - x0, y1 - y0, z1 - z0), **k)


def zp(id, bone, prof, z0, z1, mat, region, tone, **k):
    """🗂️ A prism of a side-elevation profile swept across z."""
    return Part(id, bone, "zp", mat, region, tone, prof=prof, z0=z0, z1=z1, **k)


def cyl(id, bone, axis, c, L, r, mat, region, tone, **k):
    """🥢 A cylinder along a world axis from its start point."""
    return Part(id, bone, "cyl", mat, region, tone, axis=axis, c=c, L=L, r=r, **k)


def cap(id, bone, p0, p1, r, mat, region, tone, **k):
    """💊 A capsule between two world points."""
    return Part(id, bone, "cap", mat, region, tone, p0=p0, p1=p1, r=r, **k)


def yp(id, bone, prof, y0, y1, mat, region, tone, **k):
    """🧱 A prism of a plan profile (x, z) swept along y."""
    return Part(id, bone, "yp", mat, region, tone, prof=prof, y0=y0, y1=y1, **k)


DECK_CROSS = [(-0.54, -OUTER), (0.54, -OUTER), (0.54, -0.54), (OUTER, -0.54), (OUTER, 0.54), (0.54, 0.54), (0.54, OUTER), (-0.54, OUTER), (-0.54, 0.54), (-OUTER, 0.54), (-OUTER, -0.54), (-0.54, -0.54)]
POSTS = (("fl", 1, 1), ("fr", 1, -1), ("bl", -1, 1), ("br", -1, -1))
TONE = {"grey": 0.56, "grey-dark": 0.46, "fresh": 0.84, "steel": 0.62, "stainless": 0.86, "concrete": 0.5, "rubber": 0.16, "robinia": 0.5, "roof": 0.44, "moss": 0.36}


def rafter_profile(side):
    """🏠 The side-elevation profile of one rafter (plumb cuts at eave and ridge)."""
    s = side
    return [(s * ROOF_EAVE, TIE_Y1), (s * ROOF_EAVE, TIE_Y1 + RAFTER_D), (0.0, RIDGE_RAFTER_TOP), (0.0, RIDGE_LOW)]


def board_profile(side, depth=BOARD_D, base=RIDGE_RAFTER_TOP):
    """🪵 The side-elevation profile of one roof slope from the ridge to the eave overhang."""
    s = side
    return [(0.0, base), (s * ROOF_OVER, base - TAN40 * ROOF_OVER), (s * ROOF_OVER, base - TAN40 * ROOF_OVER + depth), (0.0, base + depth)]


def stringer_profile():
    """🪜 The side-elevation profile of a ladder stringer (horizontal cuts at the foot and at deck level)."""
    return [(ladder_front(FOOT_Y), FOOT_Y), (ladder_front(DECK), DECK), (ladder_front(DECK) - STR_W, DECK), (ladder_front(FOOT_Y) - STR_W, FOOT_Y)]


def parts(scale):
    """🏗️ The parts of the model of one preset scale; each scale is decided on its own (see the decisions of the document)."""
    P = []
    fine, s100, s200, s500 = scale == 50, scale == 100, scale == 200, scale == 500
    for key, sx, sz in POSTS:
        x, z = sx * PC, sz * PC
        P.append(box(f"post-{key}", f"post-{key}", x - 0.06, x + 0.06, POST_Y0, POST_Y1, z - 0.06, z + 0.06, LW, "timber-frame", TONE["grey"] - (0.04 if sx < 0 else 0)))
        P.append(box(f"footing-{key}", f"post-{key}", x - 0.2, x + 0.2, FOOTING_Y0, -0.20, z - 0.2, z + 0.2, CO, "footings", TONE["concrete"], hatch="concrete", views=("elevation", "front")))
        if fine:
            for side, zz in (("o", z + sz * 0.064), ("i", z - sz * 0.064)):
                P.append(box(f"anchor-{key}-plate-{side}", f"post-{key}", x - 0.05, x + 0.05, -0.50, 0.30, zz - 0.004, zz + 0.004, GV, "galvanised-steel", TONE["steel"], role="detail"))
            P.append(box(f"anchor-{key}-web", f"post-{key}", x - 0.05, x + 0.05, 0.092, POST_Y0, z - 0.06, z + 0.06, GV, "galvanised-steel", TONE["steel"], role="detail", views=("elevation", "front")))
            for k, yb in enumerate((0.16, 0.25)):
                P.append(cyl(f"anchor-{key}-nut-{k + 1}", f"post-{key}", "z", (x, yb, z + sz * 0.068 - (0.010 if sz < 0 else 0)), 0.010, 0.011, GV, "galvanised-steel", TONE["steel"] - 0.1, role="detail", views=("elevation", "front", "plan")))
        else:
            P.append(box(f"anchor-{key}", f"post-{key}", x - 0.05, x + 0.05, -0.50, POST_Y0, z - 0.068, z + 0.068, GV, "galvanised-steel", TONE["steel"]))
    if fine or s100:
        for side, sz in (("left", 1), ("right", -1)):
            P.append(box(f"beam-{side}", "platform", -0.54, 0.54, BEAM_Y0, BEAM_Y1, sz * PC - BT / 2, sz * PC + BT / 2, LW, "timber-frame", TONE["grey-dark"]))
        for side, sx in (("front", 1), ("back", -1)):
            P.append(box(f"beam-{side}", "platform", sx * PC - BT / 2, sx * PC + BT / 2, BEAM_Y0, BEAM_Y1, -0.54, 0.54, LW, "timber-frame", TONE["grey-dark"]))
        P.append(box("joist", "platform", -PC + BT / 2, PC - BT / 2, BEAM_Y0, BEAM_Y1, -0.0225, 0.0225, LW, "timber-frame", TONE["grey-dark"], views=("elevation", "front")))
    if fine:
        for k in range(7):
            x0 = -0.54 + k * (BOARD_W + BOARD_GAP)
            fresh = k == 2
            P.append(box(f"deck-board-{k + 1}", "platform", x0, x0 + BOARD_W, BEAM_Y1, DECK, -OUTER, OUTER, LF if fresh else LW, "deck-boards", TONE["fresh"] if fresh else TONE["grey"], views=("elevation", "front")))
        for side, sx in (("front", 1), ("back", -1)):
            P.append(box(f"deck-nosing-{side}", "platform", min(sx * 0.548, sx * OUTER), max(sx * 0.548, sx * OUTER), BEAM_Y1, DECK, -0.54, 0.54, LW, "deck-boards", TONE["grey"], views=("elevation", "front")))
    elif s100:
        P.append(yp("deck", "platform", DECK_CROSS, BEAM_Y1, DECK, LW, "deck-boards", TONE["grey"]))
    else:
        P.append(yp("deck", "platform", DECK_CROSS, BEAM_Y0, DECK, LW, "deck-boards", TONE["grey"]))
    for side, sz, bid in (("left", 1, "barrier-left"), ("right", -1, "barrier-right")):
        z0, z1 = sorted((sz * OUTER, sz * (OUTER + INFILL_T)))
        if s500:
            P.append(box(f"barrier-{side}", bid, -0.54, 0.54, INFILL_Y0, INFILL_Y1, *sorted((sz * (PC - BT / 2), sz * (OUTER + INFILL_T))), LW, "timber-frame", TONE["grey"], envelope=True))
            continue
        if s200:
            w200 = (1.08 - 2 * LAT_GAP - 2 * 0.15) / 3
            for k in range(3):
                x0 = -0.54 + LAT_GAP + k * (w200 + 0.15)
                P.append(box(f"infill-{side}-{k + 1}", bid, x0, x0 + w200, INFILL_Y0, INFILL_Y1, z0, z1, LW, "timber-frame", TONE["grey"] + (0.03 if k % 2 else 0), envelope=True))
        else:
            for k in range(LAT_N):
                x0 = -0.54 + LAT_GAP + k * (LAT_W + LAT_GAP)
                P.append(box(f"infill-{side}-{k + 1}", bid, x0, x0 + LAT_W, INFILL_Y0, INFILL_Y1, z0, z1, LW, "timber-frame", TONE["grey"] + (0.03 if k % 2 else 0)))
        P.append(box(f"rail-{side}", bid, -0.54, 0.54, RAIL_Y0, RAIL_Y1, sz * PC - BT / 2, sz * PC + BT / 2, LW, "timber-frame", TONE["grey-dark"], envelope=s200))
    for side, sx, w, za in (("front", 1, 0.145, 0.275), ("back", -1, 0.125, 0.295)):
        x0, x1 = sorted((sx * OUTER, sx * (OUTER + INFILL_T)))
        for hand, sz in (("left", 1), ("right", -1)):
            post = f"post-{side[0]}{hand[0]}"
            zc = sz * (za + (0.54 - za) / 2)
            if s500:
                P.append(box(f"infill-{side}-{hand}", post, *sorted((sx * (PC - BT / 2), sx * (OUTER + INFILL_T))), INFILL_Y0, INFILL_Y1, *sorted((sz * za, sz * 0.54)), LW, "timber-frame", TONE["grey"]))
            else:
                P.append(box(f"infill-{side}-{hand}", post, x0, x1, INFILL_Y0, INFILL_Y1, zc - w / 2, zc + w / 2, LW, "timber-frame", TONE["grey"]))
            if side == "back" and not s500:
                P.append(box(f"rail-back-{hand}", post, -PC - BT / 2, -PC + BT / 2, RAIL_Y0, RAIL_Y1, *sorted((sz * za, sz * 0.54)), LW, "timber-frame", TONE["grey-dark"]))
        if side == "front":
            if s500:
                P.append(box("rail-front", "post-fl", PC - BT / 2, OUTER + INFILL_T, RAIL_Y0, RAIL_Y1, -0.275, 0.275, LW, "timber-frame", TONE["grey-dark"]))
            else:
                P.append(box("rail-front", "post-fl", PC - BT / 2, PC + BT / 2, RAIL_Y0, RAIL_Y1, -0.54, 0.54, LW, "timber-frame", TONE["grey-dark"]))
    if fine or s100:
        for hand, sz in (("left", 1), ("right", -1)):
            post = f"post-b{hand[0]}"
            xg = -OUTER - INFILL_T - 0.040
            P.append(cap(f"grab-{hand}", post, (xg, 1.60, sz * 0.4175), (xg, 2.15, sz * 0.4175), 0.01685, GV, "galvanised-steel", TONE["steel"], role="detail"))
            for k, yy in enumerate((1.65, 2.10)):
                P.append(cyl(f"grab-{hand}-stay-{k + 1}", post, "x", (xg, yy, sz * 0.4175), -OUTER - INFILL_T - xg, 0.012, GV, "galvanised-steel", TONE["steel"], role="detail", views=("elevation", "plan")))
    for side, sz in (("left", 1), ("right", -1)):
        P.append(zp(f"stringer-{side}", "ladder", stringer_profile(), *sorted((sz * STR_Z0, sz * STR_Z1)), LW, "timber-frame", TONE["grey"] - 0.02))
    xc0 = ladder_centre(FOOT_Y)
    if fine:
        for side, sz in (("left", 1), ("right", -1)):
            for face, zz in (("o", sz * (STR_Z1 + 0.004)), ("i", sz * (STR_Z0 - 0.004))):
                P.append(box(f"shoe-{side}-{face}", "ladder", xc0 - 0.05, xc0 + 0.05, -0.30, 0.25, zz - 0.004, zz + 0.004, GV, "galvanised-steel", TONE["steel"], role="detail"))
            P.append(box(f"shoe-{side}-web", "ladder", xc0 - 0.0775, xc0 + 0.0775, FOOT_Y - 0.008, FOOT_Y, *sorted((sz * (STR_Z0 - 0.008), sz * (STR_Z1 + 0.008))), GV, "galvanised-steel", TONE["steel"], role="detail", views=("elevation", "front")))
            P.append(box(f"ladder-angle-{side}", "ladder", ladder_front(DECK) - 0.06, -OUTER, BEAM_Y1 - 0.07, BEAM_Y1 - 0.005, *sorted((sz * (STR_Z0 - 0.006), sz * STR_Z0)), GV, "galvanised-steel", TONE["steel"] - 0.05, role="detail", views=("elevation", "plan")))
    else:
        for side, sz in (("left", 1), ("right", -1)):
            P.append(box(f"shoe-{side}", "ladder", xc0 - 0.0775, xc0 + 0.0775, -0.30, FOOT_Y, *sorted((sz * (STR_Z0 - 0.008), sz * (STR_Z1 + 0.008))), GV, "galvanised-steel", TONE["steel"]))
    P.append(box("footing-ladder", "ladder", xc0 - 0.15, xc0 + 0.15, LADDER_FOOT_Y0, -0.20, -0.35, 0.35, CO, "footings", TONE["concrete"], hatch="concrete"))
    for k, y in enumerate(RUNGS):
        P.append(cyl(f"rung-{k + 1}", "ladder", "z", (ladder_centre(y), y, -STR_Z1), 2 * STR_Z1, RUNG_R, RB, "ladder-rungs", TONE["robinia"]))
    path = PATHS[scale]
    lip = {50: [LIP[3]], 100: [LIP[3]], 200: [LIP[3]], 500: []}[scale]
    surface = [(p[0], p[1]) for p in path] + lip
    P.append(zp("slide-bed", "slide", band([(x, y - SHEET) for x, y in surface], surface), -SLIDE_HALF - SHEET, SLIDE_HALF + SHEET, SS, "slide-stainless", TONE["stainless"], build=False))
    for k in range(len(surface) - 1):
        (xa, ya), (xb, yb) = surface[k], surface[k + 1]
        ang = math.degrees(math.atan2(yb - ya, xb - xa))
        L = math.hypot(xb - xa, yb - ya)
        nx, ny = math.sin(math.radians(ang)) * SHEET / 2, -math.cos(math.radians(ang)) * SHEET / 2
        P.append(Part(f"slide-bed-{k + 1}", "slide", "rbox", SS, "slide-stainless", TONE["stainless"], draw=False, sheet=True, c=((xa + xb) / 2 + nx, (ya + yb) / 2 + ny, 0.0), s=(L, SHEET, 2 * (SLIDE_HALF + SHEET)), angle=ang))
    for side, sz in (("left", 1), ("right", -1)):
        top = WALL_H + (EDGE_R if scale in (200, 500) else 0)
        P.append(zp(f"slide-wall-{side}", "slide", band(offset_path(path, -SHEET), offset_path(path, top)), *sorted((sz * SLIDE_HALF, sz * (SLIDE_HALF + SHEET))), SS, "slide-stainless", TONE["stainless"] - 0.06, sheet=True, views=("elevation",) if s500 else ("elevation", "front", "plan")))
        zc = sz * (SLIDE_HALF + SHEET / 2)
        if fine:
            P.append(Part(f"slide-edge-{side}", "slide", "loft", SS, "slide-stainless", TONE["stainless"] - 0.12, role="detail", fill=(math.pi * 2 * EDGE_R * SHEET) / (math.pi * EDGE_R ** 2), pts=[(*q, p[2]) for q, p in zip(offset_path(path, WALL_H), path)], z=zc, r=EDGE_R, band=band(offset_path(path, WALL_H - EDGE_R), offset_path(path, WALL_H + EDGE_R))))
        elif s100:
            P.append(zp(f"slide-edge-{side}", "slide", band(offset_path(path, WALL_H - EDGE_R), offset_path(path, WALL_H + EDGE_R)), zc - EDGE_R, zc + EDGE_R, SS, "slide-stainless", TONE["stainless"] - 0.12, role="detail"))
    if fine or s100:
        P.append(box("slide-flange", "slide", OUTER, OUTER + 0.003, 1.36, DECK - SHEET, -SLIDE_HALF - SHEET - EDGE_R, SLIDE_HALF + SHEET + EDGE_R, SS, "slide-stainless", TONE["stainless"] - 0.1, role="detail", sheet=True, views=("elevation", "front")))
    leg_top = 0.30 - SHEET - (0.010 if fine else 0.0)
    P.append(cyl("slide-leg-tube", "slide-leg", "y", (LEG_X, -0.50, 0.0), leg_top + 0.50, LEG_R, GV, "galvanised-steel", TONE["steel"]))
    if fine:
        P.append(box("slide-saddle", "slide-leg", LEG_X - 0.03, LEG_X + 0.03, leg_top, 0.30 - SHEET, -SLIDE_HALF - SHEET, SLIDE_HALF + SHEET, GV, "galvanised-steel", TONE["steel"] - 0.05, role="detail"))
    P.append(box("footing-slide", "slide-leg", LEG_X - 0.15, LEG_X + 0.15, SLIDE_FOOT_Y0, -0.20, -0.15, 0.15, CO, "footings", TONE["concrete"], hatch="concrete"))
    if fine or s100:
        for i in range(2):
            for j in range(2):
                P.append(box(f"tile-{i + 1}{j + 1}", "ground", TILE_X0 + 0.5 * i, TILE_X0 + 0.5 * (i + 1), 0.0, 0.045, -0.5 + 0.5 * j, -0.5 + 0.5 * (j + 1), RU, "impact-tiles", TONE["rubber"] + (0.04 if (i + j) % 2 else 0), hatch="rubber" if fine else None))
    else:
        P.append(box("tiles", "ground", TILE_X0, TILE_X1, 0.0, 0.045, -0.5, 0.5, RU, "impact-tiles", TONE["rubber"]))
    for side, sz in (("left", 1), ("right", -1)):
        z0, z1 = sorted((sz * (PC - BT / 2), sz * (PC + BT / 2)))
        P.append(box(f"tie-{side}", "roof", -ROOF_EAVE, ROOF_EAVE, TIE_Y0, TIE_Y1, z0, z1, LW, "timber-frame", TONE["grey-dark"]))
        if not s500:
            for end, sx in (("front", 1), ("back", -1)):
                P.append(zp(f"rafter-{side}-{end}", "roof", rafter_profile(sx), z0, z1, LW, "timber-frame", TONE["grey-dark"]))
        P.append(box(f"king-post-{side}", "roof", -0.0475, 0.0475, TIE_Y1, RIDGE_LOW - TAN40 * 0.0475, z0, z1, LW, "timber-frame", TONE["grey-dark"]))
    if s500:
        for end, sx in (("front", 1), ("back", -1)):
            P.append(zp(f"roof-{end}", "roof", [(0.0, RIDGE_LOW), (sx * ROOF_EAVE, TIE_Y1), (sx * ROOF_OVER, TIE_Y1 - TAN40 * (ROOF_OVER - ROOF_EAVE) + RAFTER_D), (sx * ROOF_OVER, RIDGE_BOARD_TOP - TAN40 * ROOF_OVER), (sx * 0.08, RIDGE_BOARD_TOP - TAN40 * 0.08 + CAP_D), (0.0, CAP_TOP)], -ROOF_HALF, ROOF_HALF, LW, "roof-boards", TONE["roof"] if sx > 0 else TONE["moss"]))
    else:
        for end, sx in (("front", 1), ("back", -1)):
            P.append(zp(f"roof-{end}", "roof", board_profile(sx), -ROOF_HALF, ROOF_HALF, LW, "roof-boards", TONE["roof"] if sx > 0 else TONE["moss"]))
        if s200:
            P.append(zp("ridge", "roof", [(-0.08, RIDGE_BOARD_TOP - TAN40 * 0.08 + CAP_D), (0.0, CAP_TOP), (0.08, RIDGE_BOARD_TOP - TAN40 * 0.08 + CAP_D), (0.08, RIDGE_BOARD_TOP - TAN40 * 0.08), (0.05, RIDGE_BOARD_TOP - TAN40 * 0.05), (0.05, RIDGE_LOW - 0.04), (-0.05, RIDGE_LOW - 0.04), (-0.05, RIDGE_BOARD_TOP - TAN40 * 0.05), (-0.08, RIDGE_BOARD_TOP - TAN40 * 0.08)], -ROOF_HALF, ROOF_HALF, LW, "roof-boards", TONE["grey-dark"]))
        else:
            P.append(zp("ridge-beam", "roof", [(-0.05, RIDGE_RAFTER_TOP - TAN40 * 0.05), (0.0, RIDGE_RAFTER_TOP), (0.05, RIDGE_RAFTER_TOP - TAN40 * 0.05), (0.05, RIDGE_LOW - 0.04), (-0.05, RIDGE_LOW - 0.04)], -ROOF_HALF, ROOF_HALF, LW, "timber-frame", TONE["grey-dark"], views=("elevation",)))
            P.append(zp("ridge-cap", "roof", [(-0.08, RIDGE_BOARD_TOP - TAN40 * 0.08), (0.0, RIDGE_BOARD_TOP), (0.08, RIDGE_BOARD_TOP - TAN40 * 0.08), (0.08, RIDGE_BOARD_TOP - TAN40 * 0.08 + CAP_D), (0.0, CAP_TOP), (-0.08, RIDGE_BOARD_TOP - TAN40 * 0.08 + CAP_D)], -ROOF_HALF, ROOF_HALF, LW, "roof-boards", TONE["grey-dark"]))
            for side, sz in (("left", 1), ("right", -1)):
                P.append(Part(f"eye-{side}", "roof", "torus", GV, "galvanised-steel", TONE["steel"], role="detail", c=(0.0, EYE_CY if fine else CAP_TOP + EYE_R + EYE_ROD - 0.002, sz * PC), R=EYE_R, r=EYE_ROD))
                if fine:
                    P.append(cyl(f"eye-{side}-collar", "roof", "y", (0.0, CAP_TOP - 0.004, sz * PC), 0.012, 0.018, GV, "galvanised-steel", TONE["steel"], role="detail"))
    return P


# ---------------------------------------------------------------- 3D solids
def solid(p, list_id=None):
    """🧊 The volume solid of a part in its bone frame (world-aligned offsets turned by the bone angle)."""
    b = BONE[p.bone]
    ox, oy, oz = b["o"][0], b["o"][1], b["oz"]
    rot = abs(b["th"]) > 1e-12
    g = p.g
    s = {"id": p.id, "bone": p.bone, "material": p.mat, "role": p.role, "tone": round(p.tone, 3)}
    if p.kind == "box":
        c = (g["c"][0] - ox, g["c"][1] - oy, g["c"][2] - oz)
        u, v = to_local(p.bone, g["c"][0], g["c"][1])
        s.update(kind="box", size=[r6(x) for x in g["s"]], transform={"translation": [r6(u), r6(v), r6(c[2])], **({"rotation": quat(p.bone)} if rot else {})})
    elif p.kind == "rbox":
        u, v = to_local(p.bone, g["c"][0], g["c"][1])
        th = math.radians(g["angle"] - b["th"])
        s.update(kind="box", size=[r6(x) for x in g["s"]], transform={"translation": [r6(u), r6(v), r6(g["c"][2] - oz)], "rotation": [0.0, 0.0, r6(math.sin(th / 2)), r6(math.cos(th / 2))]})
    elif p.kind == "zp":
        s.update(kind="extrude", plane="xy", profile=[[r6(x - ox), r6(y - oy)] for x, y in g["prof"]], **{"from": r6(g["z0"] - oz), "to": r6(g["z1"] - oz)})
        if rot:
            s["transform"] = {"rotation": quat(p.bone)}
    elif p.kind == "yp":
        s.update(kind="extrude", plane="zx", profile=[[r6(z - oz), r6(x - ox)] for x, z in g["prof"]], **{"from": r6(g["y0"] - oy), "to": r6(g["y1"] - oy)})
        if rot:
            s["transform"] = {"rotation": quat(p.bone)}
    elif p.kind == "cyl":
        axis = {"x": [1.0, 0.0, 0.0], "y": [0.0, 1.0, 0.0], "z": [0.0, 0.0, 1.0]}[g["axis"]]
        L = g["L"]
        c = list(g["c"])
        if L < 0:
            c["xyz".index(g["axis"])] += L
            L = -L
        s.update(kind="revolve", profile=[[0.0, 0.0], [0.0, r6(g["r"])], [r6(L), r6(g["r"])], [r6(L), 0.0]], axis=axis, origin=[r6(c[0] - ox), r6(c[1] - oy), r6(c[2] - oz)])
        if rot:
            s["transform"] = {"rotation": quat(p.bone)}
    elif p.kind == "cap":
        s.update(kind="capsule", p0=[r6(g["p0"][0] - ox), r6(g["p0"][1] - oy), r6(g["p0"][2] - oz)], p1=[r6(g["p1"][0] - ox), r6(g["p1"][1] - oy), r6(g["p1"][2] - oz)], r0=r6(g["r"]), r1=r6(g["r"]))
        if rot:
            s["transform"] = {"rotation": quat(p.bone)}
    elif p.kind == "torus":
        n = 16
        prof = [[r6(g["r"] * math.cos(2 * math.pi * k / n)), r6(g["R"] + g["r"] * math.sin(2 * math.pi * k / n))] for k in range(n)]
        prof.append(prof[0])
        s.update(kind="revolve", profile=prof, axis=[0.0, 0.0, 1.0], origin=[r6(g["c"][0] - ox), r6(g["c"][1] - oy), r6(g["c"][2] - oz)])
        if rot:
            s["transform"] = {"rotation": quat(p.bone)}
    elif p.kind == "loft":
        secs = []
        for x, y, t in g["pts"]:
            u, v = to_local(p.bone, x, y)
            phi = math.radians(t - b["th"])
            qw = (0.0, 0.0, math.sin(phi / 2), math.cos(phi / 2))
            q0 = (0.5, 0.5, 0.5, 0.5)
            x1, y1, z1, w1 = qw
            x2, y2, z2, w2 = q0
            q = (w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2, w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2, w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2, w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2)
            secs.append({"at": [r6(u), r6(v), r6(g["z"] - oz)], "rx": r6(g["r"]), "ry": r6(g["r"]), "rotation": [r6(c) for c in q]})
        s.update(kind="loft", sections=secs, capStart=True, capEnd=True)
    for flag in ("sheet", "slender", "envelope"):
        if getattr(p, flag):
            s[flag] = True
    return s


# ---------------------------------------------------------------- 2D shapes
def proj(p, view):
    """🖼️ The exact orthographic projection of a part in one view as (kind, data) in world view coordinates."""
    x0, x1, y0, y1, z0, z1 = p.bounds()
    g = p.g
    if view == "elevation":
        if p.kind == "zp":
            return "poly", g["prof"]
        if p.kind == "cyl" and g["axis"] == "z" or p.kind == "torus":
            return ("ring" if p.kind == "torus" else "circle"), (g["c"][0], g["c"][1], g["R"] + g["r"] if p.kind == "torus" else g["r"], g.get("R", 0) - g.get("r", 0))
        if p.kind == "cap":
            return "capsule", ((g["p0"][0], g["p0"][1]), (g["p1"][0], g["p1"][1]), g["r"])
        if p.kind == "loft":
            return "poly", g["band"]
        return "poly", rect(x0, y0, x1, y1)
    if view == "front":
        if p.kind == "cyl" and g["axis"] == "x":
            return "circle", (g["c"][2], g["c"][1], g["r"], 0)
        if p.kind == "cap":
            return "capsule", ((g["p0"][2], g["p0"][1]), (g["p1"][2], g["p1"][1]), g["r"])
        return "poly", rect(z0, y0, z1, y1)
    if p.kind == "yp":
        return "poly", g["prof"]
    if p.kind == "cyl" and g["axis"] == "y":
        return "circle", (g["c"][0], g["c"][2], g["r"], 0)
    if p.kind == "cap":
        return "capsule", ((g["p0"][0], g["p0"][2]), (g["p1"][0], g["p1"][2]), g["r"])
    return "poly", rect(x0, z0, x1, z1)


def depth_key(p, view):
    """🔢 Paint order: nearer to the viewer paints later (elevation seen from -z, front from +x, plan from +y)."""
    x0, x1, y0, y1, z0, z1 = p.bounds()
    return {"elevation": -z0, "front": x1, "plan": y1}[view]


def shape_of(p, view, pens):
    """✏️ The drawn shape of a part in one view (elevation in its bone frame, front and plan in world rest coordinates)."""
    kind, d = proj(p, view)
    base = {"id": p.id, "bone": p.bone, "role": p.role, "z": round(2 + depth_key(p, view), 4), "filled": True, "tone": round(p.tone, 3), "region": p.region, "material": p.mat}
    if p.hatch:
        base["hatch"] = p.hatch
    loc = (lambda x, y: to_local(p.bone, x, y)) if view == "elevation" else (lambda x, y: (x, y))
    if kind == "poly":
        base.update(kind="poly", points=[[r6(a), r6(b)] for a, b in (loc(*q) for q in d)], closed=True)
    elif kind == "circle":
        cx, cy = loc(d[0], d[1])
        base.update(kind="ellipse", cx=r6(cx), cy=r6(cy), rx=r6(d[2]), ry=r6(d[2]))
    elif kind == "ring":
        cx, cy = loc(d[0], d[1])
        ro, ri = d[2], d[3]
        base.update(kind="path", d=f"M {cx + ro:.6f} {cy:.6f} A {ro:.6f} {ro:.6f} 0 1 0 {cx - ro:.6f} {cy:.6f} A {ro:.6f} {ro:.6f} 0 1 0 {cx + ro:.6f} {cy:.6f} Z M {cx + ri:.6f} {cy:.6f} A {ri:.6f} {ri:.6f} 0 1 1 {cx - ri:.6f} {cy:.6f} A {ri:.6f} {ri:.6f} 0 1 1 {cx + ri:.6f} {cy:.6f} Z")
    elif kind == "capsule":
        (a0, b0), (a1, b1), r = d
        u0, v0 = loc(a0, b0)
        u1, v1 = loc(a1, b1)
        base.update(kind="capsule", x0=r6(u0), y0=r6(v0), x1=r6(u1), y1=r6(v1), r0=r6(r), r1=r6(r))
    return base


def texture(id, bone, view, pts, region, closed=False, tone=0.3):
    """〰️ A texture stroke (traces of use and weather) in world view coordinates; elevation points are carried into the bone frame."""
    loc = (lambda x, y: to_local(bone, x, y)) if view == "elevation" else (lambda x, y: (x, y))
    return {"id": id, "bone": bone, "role": "texture", "z": 4.0, "filled": False, "tone": tone, "region": region, "kind": "poly", "points": [[r6(a), r6(b)] for a, b in (loc(*q) for q in pts)], "closed": closed}


def traces(scale):
    """🧽 The traces of use and weather drawn as texture at 1:50 and 1:100 (checks, grain, graffiti, gum, scratches, moss, worn rungs, worn tile)."""
    out = {"elevation": [], "front": [], "plan": []}
    if scale not in (50, 100):
        return out
    fine = scale == 50
    for key, sx, sz in POSTS:
        x, z = sx * PC, sz * PC
        if sz < 0:
            for k, (dx, ya, yb) in enumerate(((-0.02, 0.45, 1.05), (0.025, 1.6, 2.2), (0.0, 0.18, 0.4)) if fine else ((-0.02, 0.45, 1.05),)):
                out["elevation"].append(texture(f"check-{key}-{k + 1}", f"post-{key}", "elevation", [(x + dx, ya), (x + dx + 0.004, (ya + yb) / 2), (x + dx - 0.002, yb)], "timber-frame"))
        if sx > 0:
            out["front"].append(texture(f"check-front-{key}", f"post-{key}", "front", [(z - 0.015 * sz, 0.6), (z - 0.012 * sz, 1.0), (z - 0.016 * sz, 1.25)], "timber-frame"))
    if fine:
        out["elevation"].append(texture("algae-post-br", "post-br", "elevation", [(-0.66, 0.10), (-0.63, 0.22), (-0.60, 0.15), (-0.57, 0.26), (-0.54, 0.10)], "timber-frame", closed=True, tone=0.36))
        out["elevation"].append(texture("rust-anchor-fr", "post-fr", "elevation", [(0.57, 0.30), (0.575, 0.16), (0.58, 0.05)], "galvanised-steel", tone=0.3))
        out["elevation"].append(texture("graffiti-right", "barrier-right", "elevation", [(-0.30, 1.75), (-0.26, 1.92), (-0.22, 1.74), (-0.17, 1.95), (-0.12, 1.78), (-0.06, 1.88), (0.02, 1.70), (0.08, 1.93)], "timber-frame", tone=0.2))
        out["elevation"].append(texture("sticker-right", "barrier-right", "elevation", [(0.26, 1.98), (0.33, 1.98), (0.33, 2.04), (0.26, 2.04)], "timber-frame", closed=True, tone=0.75))
        out["elevation"].append(texture("slide-sticker", "slide", "elevation", [(2.10, 1.03), (2.18, 0.975), (2.205, 1.01), (2.125, 1.065)], "slide-stainless", closed=True, tone=0.7))
        for k, y in enumerate(RUNGS):
            out["front"].append(texture(f"rung-wear-{k + 1}", "ladder", "front", [(-0.17, y + RUNG_R), (0.17, y + RUNG_R)], "ladder-rungs", tone=0.75))
        for k in range(6):
            x0 = -0.54 + (k + 1) * (BOARD_W + BOARD_GAP) - BOARD_GAP / 2
            out["plan"].append(texture(f"deck-joint-{k + 1}", "platform", "plan", [(x0, -OUTER), (x0, OUTER)], "deck-boards", tone=0.25))
        out["plan"].append(texture("deck-joint-front", "platform", "plan", [(0.544, -0.54), (0.544, 0.54)], "deck-boards", tone=0.25))
        out["plan"].append(texture("deck-joint-back", "platform", "plan", [(-0.544, -0.54), (-0.544, 0.54)], "deck-boards", tone=0.25))
        for k, (gx, gz) in enumerate(((0.38, 0.12), (0.44, -0.08), (0.20, 0.30))):
            out["plan"].append(texture(f"gum-{k + 1}", "platform", "plan", circle(gx, gz, 0.012, 8), "deck-boards", closed=True, tone=0.15))
        for k, (zz, xa, xb) in enumerate(((0.06, 1.2, 2.9), (-0.08, 1.05, 2.6), (0.15, 1.7, 3.4), (-0.15, 2.2, 3.8))):
            out["plan"].append(texture(f"slide-scratch-{k + 1}", "slide", "plan", [(xa, zz), (xb, zz + 0.01)], "slide-stainless", tone=0.55))
        out["plan"].append(texture("tile-hollow", "ground", "plan", [(4.02, -0.16), (4.22, -0.20), (4.32, 0.0), (4.22, 0.20), (4.02, 0.16), (3.94, 0.0)], "impact-tiles", closed=True, tone=0.3))
    for k, (mx, mz) in enumerate(((-0.35, 0.40), (-0.62, -0.20), (-0.20, -0.55)) if fine else ((-0.40, 0.30),)):
        out["plan"].append(texture(f"moss-{k + 1}", "roof", "plan", [(mx - 0.08, mz), (mx - 0.03, mz + 0.07), (mx + 0.06, mz + 0.05), (mx + 0.09, mz - 0.03), (mx + 0.02, mz - 0.08), (mx - 0.06, mz - 0.06)], "roof-boards", closed=True, tone=0.3))
    out["plan"].append(texture("slide-track", "slide", "plan", [(1.0, -0.12), (3.9, -0.12), (3.9, 0.12), (1.0, 0.12)], "slide-stainless", closed=True, tone=0.92))
    return out


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {
    50: [{"id": "concrete", "title": T("Concrete in the ground (cut)", "Beton im Erdreich (geschnitten)"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 1.2, "penMm": 0.13}, {"id": "rubber", "title": T("Rubber granulate", "Gummigranulat"), "pattern": "dots", "angleDeg": 0, "spacingMm": 1.0, "penMm": 0.13}],
    100: [{"id": "concrete", "title": T("Concrete in the ground (cut)", "Beton im Erdreich (geschnitten)"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 1.0, "penMm": 0.13}],
    200: [{"id": "concrete", "title": T("Concrete in the ground (cut)", "Beton im Erdreich (geschnitten)"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1}],
    500: [{"id": "concrete", "title": T("Concrete in the ground (cut)", "Beton im Erdreich (geschnitten)"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
}


def representation(scale, P, decisions):
    """🗺️ The authored representation of one scale: the three exact projections of its parts plus its traces of use."""
    geo = {v: [shape_of(p, v, PENS[scale]) for p in P if p.draw and v in p.views] for v in ("elevation", "front", "plan")}
    tr = traces(scale)
    for v in geo:
        geo[v].extend(tr[v])
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": geo, "decisions": decisions}


# ---------------------------------------------------------------- masses (from the 1:50 parts)
def masses():
    """⚖️ Bone masses and centres of mass from the 1:50 parts (volume x density), in each bone's frame."""
    out = {}
    for p in parts(50):
        if not p.build:
            continue
        v, c = p.volume()
        m = v * DENSITY[p.mat] * p.fill
        e = out.setdefault(p.bone, [0.0, 0.0, 0.0, 0.0])
        e[0] += m
        e[1] += m * c[0]
        e[2] += m * c[1]
        e[3] += m * c[2]
    res = {}
    for bid, (m, mx, my, mz) in out.items():
        u, v = to_local(bid, mx / m, my / m)
        res[bid] = (m, u, v, mz / m)
    return res


MASS = masses()
TOTAL_MASS = round(sum(m for m, *_ in MASS.values()), 1)

# ---------------------------------------------------------------- structure lists
SUPPORTS = ["support-fl", "support-fr", "support-bl", "support-br", "ladder-foot", "slide-foot", "tiles-ground"]


def ellipsoid(id, bone, at, radii, mat, member, part, tone=0.2):
    """🥚 A compact marker solid at a bone-local point."""
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": mat, "role": "detail", "tone": tone, "radii": [r6(r) for r in radii], "transform": {"translation": [r6(a) for a in at]}, "member": member, "part": part}


def structure(scale, P):
    """🏗️ The structural idealisation of one scale: members along their bones with the real sections, supports at the support anchors, masses at the centres."""
    f = FEATURE[scale]
    ids = {p.id for p in P}
    S = []
    for key, sx, sz in POSTS:
        for sid in [f"post-{key}", f"footing-{key}", f"anchor-{key}", f"anchor-{key}-plate-o", f"anchor-{key}-plate-i", f"anchor-{key}-web"]:
            if sid in ids:
                S.append({"id": f"member-{sid}", "of": sid, "member": f"post-{key}", "part": "member"})
    if "beam-left" in ids:
        for b in ("beam-left", "beam-right", "beam-front", "beam-back"):
            S.append({"id": f"member-{b}", "of": b, "member": "platform", "part": "member"})
    else:
        for side, sz in (("left", 1), ("right", -1)):
            S.append({"id": f"member-beam-{side}", "kind": "box", "bone": "platform", "material": LW, "role": "body", "tone": 0.5, "size": [1.08, 0.145, max(0.045, 1.07 * f)], "transform": {"translation": [OUTER, 0.0725, sz * PC]}, "member": "platform", "part": "member"})
        for side, sx in (("front", 1), ("back", -1)):
            S.append({"id": f"member-beam-{side}", "kind": "box", "bone": "platform", "material": LW, "role": "body", "tone": 0.5, "size": [max(0.045, 1.07 * f), 0.145, 1.08], "transform": {"translation": [OUTER + sx * PC, 0.0725, 0.0]}, "member": "platform", "part": "member"})
    for side in ("left", "right"):
        S.append({"id": f"member-stringer-{side}", "of": f"stringer-{side}", "member": "ladder", "part": "member"})
    for sid in ("shoe-left", "shoe-right", "shoe-left-o", "shoe-left-i", "shoe-left-web", "shoe-right-o", "shoe-right-i", "shoe-right-web", "footing-ladder"):
        if sid in ids:
            S.append({"id": f"member-{sid}", "of": sid, "member": "ladder", "part": "member"})
    for sid in [p.id for p in P if p.id.startswith("slide-bed-")] + ["slide-wall-left", "slide-wall-right"]:
        S.append({"id": f"member-{sid}", "of": sid, "member": "slide", "part": "member"})
    S.append({"id": "member-slide-leg", "of": "slide-leg-tube", "member": "slide-leg", "part": "member"})
    S.append({"id": "member-footing-slide", "of": "footing-slide", "member": "slide-leg", "part": "member"})
    for side in ("left", "right"):
        S.append({"id": f"member-tie-{side}", "of": f"tie-{side}", "member": "roof", "part": "member"})
        S.append({"id": f"member-king-post-{side}", "of": f"king-post-{side}", "member": "roof", "part": "member"})
        if f"rafter-{side}-front" in ids:
            for end in ("front", "back"):
                S.append({"id": f"member-rafter-{side}-{end}", "of": f"rafter-{side}-{end}", "member": "roof", "part": "member"})
    if "roof-front" in ids and scale == 500:
        for end in ("front", "back"):
            S.append({"id": f"member-roof-{end}", "of": f"roof-{end}", "member": "roof", "part": "member"})
    if "slide-flange" in ids:
        S.append({"id": "joint-slide-flange", "of": "slide-flange", "member": "slide", "part": "joint"})
    else:
        S.append({"id": "joint-slide-fixing", "kind": "box", "bone": "slide", "material": SS, "role": "detail", "tone": 0.3, "size": [r6(max(0.06, 1.05 * f)), r6(max(0.06, 1.05 * f)), 0.40], "transform": {"translation": [*[r6(c) for c in to_local("slide", 0.605, 1.47)], 0.0], "rotation": quat("slide")}, "member": "slide", "part": "joint"})
    if "slide-saddle" in ids:
        S.append({"id": "joint-slide-saddle", "of": "slide-saddle", "member": "slide-leg", "part": "joint"})
    sz_ = 0.05 + 1.2 * f
    for key, sx, szz in POSTS:
        S.append({"id": f"support-{key}", "kind": "box", "bone": f"post-{key}", "material": GV, "role": "body", "tone": 0.15, "size": [sz_, 0.16, 0.16], "transform": {"translation": [0.0, 0.0, 0.0]}, "member": f"post-{key}", "part": "support", "anchor": f"support-{key}"})
    w = 0.05 + 1.2 * f
    S.append({"id": "support-ladder-foot", "kind": "extrude", "bone": "ladder", "material": GV, "role": "body", "tone": 0.15, "profile": [[0.0, 0.0], [r6(w / 2), r6(-w * 0.8)], [r6(-w / 2), r6(-w * 0.8)]], "plane": "xy", "from": -0.30, "to": 0.30, "transform": {"translation": [LADDER_ANCHOR[0], LADDER_ANCHOR[1], 0.0], "rotation": quat("ladder")}, "member": "ladder", "part": "support", "anchor": "ladder-foot"})
    S.append({"id": "support-slide-foot", "kind": "extrude", "bone": "slide-leg", "material": GV, "role": "body", "tone": 0.15, "profile": [[0.0, 0.0], [r6(w / 2), r6(-w * 0.8)], [r6(-w / 2), r6(-w * 0.8)]], "plane": "xy", "from": -0.08, "to": 0.08, "transform": {"rotation": quat("slide-leg")}, "member": "slide-leg", "part": "support", "anchor": "slide-foot"})
    S.append({"id": "support-tiles-ground", "kind": "box", "bone": "ground", "material": RU, "role": "body", "tone": 0.15, "size": [0.30, r6(0.02 + f), 0.30], "transform": {"translation": [0.5 * (TILE_X0 + TILE_X1), 0.0, 0.0]}, "member": "ground", "part": "support", "anchor": "tiles-ground"})
    rr = 0.025 + 0.55 * f
    for bid, (m, u, v, zz) in MASS.items():
        S.append(ellipsoid(f"mass-{bid}", bid, (u, v, 0.0), (rr, rr, rr), GV, bid if bid in MEMBER_BONES else nearest_member(bid), "mass"))
    return S


MEMBER_BONES = {"post-fl", "post-fr", "post-bl", "post-br", "platform", "ladder", "slide", "slide-leg", "roof"}


def nearest_member(bid):
    """🔗 The member bone a non-structural bone's mass is reported on (its nearest structural ancestor or the post it stands on)."""
    return bid


# ---------------------------------------------------------------- thermal
REGION_AREA = {}


def thermal_list(P):
    """🌡️ The thermal list of a scale: every body solid by reference in its region."""
    return [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in P]


def layer(id, en, de, kind, mat, t, rho, cp, k, ref):
    """🧶 One layer with its resistance and areal heat capacity rho*cp*min(t, sqrt(alpha*3600 s))."""
    d = min(t, math.sqrt(k / (rho * cp) * 3600))
    return {"id": id, "title": T(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": float(f"{t / k:.4g}"), "heatCapacityJM2K": round(rho * cp * d), "source": "literature", "reference": ref}


def region(id, en, de, kind, mat, area_m2, L, geom, sky, lay, ref, src="authored-estimate"):
    """🔥 A passive thermal region of one material with one layer."""
    return {"id": id, "title": T(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": area_m2, "characteristicLengthM": L, "geometry": geom, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": src, "reference": ref}


AREAS = json.load(open(os.path.join(HERE, "author-areas.json"))) if os.path.exists(os.path.join(HERE, "author-areas.json")) else {}
REGIONS = [
    region("timber-frame", "Greyed larch frame: posts, beams, rails, barrier boards, ladder stringers, gable frames", "Vergrautes Lärchengerüst: Pfosten, Balken, Handläufe, Brüstungsbretter, Leiterholme, Giebelbinder", "surface", LW, AREAS.get("timber-frame", 14.0), 0.12, "vertical-plate", 0.5, layer("larch", "Weathered larch, 22 to 120 mm (active depth)", "Verwitterte Lärche, 22 bis 120 mm (aktive Tiefe)", "timber", LW, 0.06, 590, 1600, 0.13, "iso-10456-2007"), "williams-2005"),
    region("deck-boards", "Deck boards under the roof (one replaced in fresh larch)", "Belagbretter unter dem Dach (eines in frischer Lärche ersetzt)", "surface", LW, AREAS.get("deck-boards", 3.4), 0.6, "horizontal-plate-up", 0.25, layer("deck", "Larch deck board 28 mm", "Lärchenbelag 28 mm", "timber", LW, 0.028, 590, 1600, 0.13, "iso-10456-2007"), "williams-2005"),
    region("roof-boards", "Roof boarding, moss on the north slope", "Dachschalung, Moos auf der Nordseite", "surface", LW, AREAS.get("roof-boards", 7.8), 1.2, "horizontal-plate-up", 0.9, layer("roof", "Larch roof boards 24 mm", "Lärchen-Dachschalung 24 mm", "timber", LW, 0.024, 590, 1600, 0.13, "iso-10456-2007"), "williams-2005"),
    region("slide-stainless", "Stainless steel slide bed and side walls (the hot surface in sun)", "Edelstahl-Rutschbett und Seitenwangen (die heiße Fläche in der Sonne)", "surface", SS, AREAS.get("slide-stainless", 5.6), 0.45, "horizontal-plate-up", 0.9, layer("sheet", "Stainless sheet 2 mm (both faces exposed: 1 mm per face)", "Edelstahlblech 2 mm (beidseitig frei: 1 mm je Seite)", "metal", SS, 0.001, 7900, 500, 15, "en-10088-2"), "incropera-2018"),
    region("galvanised-steel", "Galvanised H-anchors, ladder shoes, run-out leg, grab bars, lifting eyes", "Verzinkte H-Anker, Leiterschuhe, Auslaufstütze, Haltegriffe, Anschlagösen", "surface", GV, AREAS.get("galvanised-steel", 1.6), 0.06, "cylinder", 0.5, layer("zinc-steel", "Galvanised steel 3 to 8 mm", "Verzinkter Stahl 3 bis 8 mm", "metal", GV, 0.004, 7850, 450, 50, "en-iso-1461"), "en-iso-1461"),
    region("ladder-rungs", "Robinia rungs, worn by shoes", "Robiniensprossen, abgetreten", "surface", RB, AREAS.get("ladder-rungs", 0.33), 0.045, "cylinder", 0.5, layer("robinia", "Robinia round 45 mm (active depth)", "Robinie rund 45 mm (aktive Tiefe)", "timber", RB, 0.0225, 730, 1600, 0.18, "din-68364"), "din-68364"),
    region("footings", "Concrete point and strip footings in the ground", "Beton-Punkt- und Streifenfundamente im Erdreich", "surface", CO, AREAS.get("footings", 7.0), 0.4, "vertical-plate", 0.0, layer("concrete", "Concrete C25/30", "Beton C25/30", "masonry", CO, 0.2, 2400, 1000, 2.0, "iso-10456-2007"), "iso-10456-2007"),
    region("impact-tiles", "Rubber impact tiles at the run-out, worn hollow at the exit", "Fallschutzplatten am Auslauf, ausgetretene Mulde am Ausstieg", "surface", RU, AREAS.get("impact-tiles", 2.2), 0.5, "horizontal-plate-up", 1.0, layer("granulate", "Rubber granulate 45 mm", "Gummigranulat 45 mm", "polymer", RU, 0.045, 800, 1500, 0.17, "iso-10456-2007"), "iso-10456-2007"),
]
REGION_IDS = [r["id"] for r in REGIONS]


def binding(sun, extra=None):
    """☀️ The region binding of a pose (sunlit shares for the summer reference case)."""
    b = {
        "timber-frame": {"exposedFraction": 1.0, "sunlitFraction": 0.45 * sun},
        "deck-boards": {"exposedFraction": 1.0, "sunlitFraction": 0.15 * sun},
        "roof-boards": {"exposedFraction": 1.0, "sunlitFraction": 0.55 * sun},
        "slide-stainless": {"exposedFraction": 1.0, "sunlitFraction": 0.85 * sun},
        "galvanised-steel": {"exposedFraction": 0.7, "sunlitFraction": 0.4 * sun},
        "ladder-rungs": {"exposedFraction": 1.0, "sunlitFraction": 0.5 * sun},
        "footings": {"exposedFraction": 0.0, "sunlitFraction": 0.0, "contact": {"fraction": 1.0, "tempC": 14}},
        "impact-tiles": {"exposedFraction": 0.5, "sunlitFraction": 0.5 * sun, "contact": {"fraction": 0.5, "tempC": 24}},
    }
    for k, v in (extra or {}).items():
        b[k] = {**b[k], **v}
    return {"regions": {k: {kk: (round(vv, 3) if isinstance(vv, float) else vv) for kk, vv in v.items()} for k, v in b.items()}, "emitters": {}}


# ---------------------------------------------------------------- anchors, loads, poses, clips
def bone_point(bid, x, y):
    """📍 World (x, y) as bone-local (x, y) rounded."""
    u, v = to_local(bid, x, y)
    return r6(u), r6(v)


ANCHORS = []
for key, sx, sz in POSTS:
    ANCHORS.append({"id": f"support-{key}", "bone": f"post-{key}", "x": 0, "y": 0, "kind": "support", "title": T(f"Point footing of post {key} on the soil, clamped by the earth around it (bottom of the footing)", f"Punktfundament des Pfostens {key} auf dem Baugrund, vom umgebenden Erdreich eingespannt (Fundamentsohle)"), "support": "fixed", "footprintLengthM": 0.4, "footprintWidthM": 0.4, "frictionCoefficient": 0.6})
ANCHORS.append({"id": "ladder-foot", "bone": "ladder", "x": LADDER_ANCHOR[0], "y": LADDER_ANCHOR[1], "kind": "support", "title": T("Strip footing of the ladder on the soil (bottom of the footing)", "Streifenfundament der Leiter auf dem Baugrund (Fundamentsohle)"), "support": "pin", "footprintLengthM": 0.3, "footprintWidthM": 0.7, "frictionCoefficient": 0.6})
ANCHORS.append({"id": "slide-foot", "bone": "slide-leg", "x": 0, "y": 0, "kind": "support", "title": T("Footing of the run-out leg on the soil (bottom of the footing)", "Fundament der Auslaufstütze auf dem Baugrund (Fundamentsohle)"), "support": "pin", "footprintLengthM": 0.3, "footprintWidthM": 0.3, "frictionCoefficient": 0.6})
ANCHORS.append({"id": "tiles-ground", "bone": "ground", "x": r6(0.5 * (TILE_X0 + TILE_X1)), "y": 0, "kind": "support", "title": T("Rubber tiles lying loose on the sand", "Fallschutzplatten lose auf dem Sand"), "support": "contact", "footprintLengthM": 1.0, "footprintWidthM": 1.0, "frictionCoefficient": 0.7})
for end, ux in (("front", 2 * ROOF_EAVE), ("back", 0.0)):
    for side, sz in (("left", 1), ("right", -1)):
        ANCHORS.append({"id": f"roof-rest-{end}-{side}", "bone": "roof", "x": ux, "y": 0, "z": sz * PC, "kind": "load", "title": T(f"Tie beam end {end} {side}: lifting point and resting point of the dismantled roof", f"Zugbalkenende {'vorn' if end == 'front' else 'hinten'} {'links' if side == 'left' else 'rechts'}: Anschlag- und Ablagepunkt des abgebauten Dachs")})
ROOF_RESTS = [a["id"] for a in ANCHORS if a["id"].startswith("roof-rest")]
G = 9.80665
CHILD = 30.0
MU = 0.25


def slide_point(xw):
    """🧒 The bed surface point, tangent and slide-local position of a seated child at world x."""
    for i in range(len(PATH) - 1):
        a, b = PATH[i], PATH[i + 1]
        if a[0] <= xw <= b[0]:
            t = (xw - a[0]) / (b[0] - a[0])
            return a[1] + t * (b[1] - a[1]), a[2] + t * (b[2] - a[2])
    return PATH[-1][1], 0.0


def child_on_slide(xw, sliding):
    """🛝 The load a child of 30 kg puts on the bed at world x: the normal pressure and, while sliding, the friction it drags along the bed (mu 0.25)."""
    y, t = slide_point(xw)
    r = math.radians(t)
    n = CHILD * G * math.cos(r)
    nx, ny = -math.sin(r) * n, -math.cos(r) * n
    if sliding:
        fx, fy = nx + MU * n * math.cos(r), ny + MU * n * math.sin(r)
    else:
        fx, fy = 0.0, -CHILD * G
    u, v = bone_point("slide", xw, y + 0.10)
    return {"id": "child-sliding", "kind": "occupant", "title": T("Child of 30 kg on the slide", "Kind mit 30 kg auf der Rutsche"), "bone": "slide", "x": u, "y": v, "fxN": round(fx, 1), "fyN": round(fy, 1), "source": "authored-estimate", "reference": "en-1176-3"}


POSE_CONTACTS = list(SUPPORTS)
ROOF_CARRY = [{"id": a.replace("roof-rest", "roof-carry"), "kind": "equipment", "title": T("A quarter of the dismantled roof carried at this tie beam end, first by the crane sling, then by the lawn", "Ein Viertel des abgebauten Dachs an diesem Zugbalkenende getragen, erst vom Krangehänge, dann vom Rasen"), "anchor": a, "fxN": 0, "fyN": round(round(MASS["roof"][0], 2) * G / 4, 1), "source": "authored-estimate", "reference": "en-1176-7"} for a in ROOF_RESTS]
PLATFORM_CHILD = [{"id": f"child-platform-{k + 1}", "kind": "occupant", "title": T(f"Child {k + 1} on the platform (30 kg)", f"Kind {k + 1} auf dem Podest (30 kg)"), "bone": "platform", "x": r6(OUTER + dx), "y": r6(DECK - BEAM_Y0), "z": dz, "fxN": 0, "fyN": round(-CHILD * G, 1), "source": "authored-estimate", "reference": "en-1176-1"} for k, (dx, dz) in enumerate(((-0.2, 0.25), (0.1, -0.2)))]
LADDER_CHILD = {"id": "child-ladder", "kind": "occupant", "title": T("Child climbing the ladder (30 kg on rung 3)", "Kind auf der Leiter (30 kg auf Sprosse 3)"), "bone": "ladder", "x": r6(0.75 / SIN70), "y": 0.0, "fxN": 0, "fyN": round(-CHILD * G, 1), "source": "authored-estimate", "reference": "en-1176-1"}
POSES = [
    {"id": "in-service", "title": T("In service", "Im Betrieb"), "description": T("Standing in the park on its four H-anchors in point footings, the ladder shoes on a strip footing and the run-out leg in its own footing, the sand at y = 0 over the footings; empty on a summer afternoon.", "Im Park stehend auf vier H-Ankern in Punktfundamenten, die Leiterschuhe auf einem Streifenfundament, die Auslaufstütze in einem eigenen Fundament, der Sand bei y = 0 über den Fundamenten; leer an einem Sommernachmittag."), "bones": {}, "ground": "authored", "contacts": POSE_CONTACTS, "thermal": binding(1.0), "loads": []},
    {"id": "maintenance", "title": T("Roof dismantled", "Dach abgebaut"), "description": T("The depot has unbolted the gable roof from the post heads and its loader crane has set it down behind the ladder on the ends of its two tie beams for new boards; posts, platform, ladder, slide and footings stay in place.", "Der Bauhof hat das Satteldach von den Pfostenköpfen gelöst und der Ladekran hat es hinter der Leiter auf den Enden seiner zwei Zugbalken abgesetzt, um die Schalung zu erneuern; Pfosten, Podest, Leiter, Rutsche und Fundamente bleiben stehen."), "bones": {"roof-carry": {"dx": -2.45}, "roof": {"dy": -TIE_Y0}}, "ground": "authored", "contacts": POSE_CONTACTS, "thermal": binding(1.0, {"deck-boards": {"sunlitFraction": 0.9}}), "loads": ROOF_CARRY},
    {"id": "in-use", "title": T("In use", "In Benutzung"), "description": T("Afternoon use: two children on the platform, one sitting at the slide start under the entry bar, one on the third rung of the ladder (30 kg each).", "Nachmittags in Benutzung: zwei Kinder auf dem Podest, eines sitzt am Rutschenstart unter der Einstiegsstange, eines auf der dritten Sprosse der Leiter (je 30 kg)."), "bones": {}, "ground": "authored", "contacts": POSE_CONTACTS, "thermal": binding(1.0, {"slide-stainless": {"contact": {"fraction": 0.03, "tempC": 33}}}), "loads": [child_on_slide(0.85, False), *PLATFORM_CHILD, LADDER_CHILD]},
]
POSES.append({"id": "roof-on-crane", "key": True, "title": T("Roof on the crane", "Dach am Kran"), "description": T("Key of the dismantling clip: the crane sling carries the unbolted roof at its four tie beam ends.", "Schlüssel des Abbau-Clips: das Krangehänge trägt das gelöste Dach an seinen vier Zugbalkenenden."), "bones": {}, "ground": "authored", "contacts": POSE_CONTACTS, "thermal": binding(1.0), "loads": ROOF_CARRY})
SLIDE_KEYS = [("slide-seated", 0.85, False, "Seated at the start", "Am Start sitzend"), ("slide-arc", 1.45, True, "Into the transition", "In den Übergangsbogen"), ("slide-slope", 2.30, True, "On the 35 degree section", "Auf dem 35-Grad-Abschnitt"), ("slide-runout", 3.55, True, "Braking on the run-out", "Bremsen im Auslauf"), ("slide-stopped", 3.85, False, "Stopped at the end", "Am Ende angehalten")]
for pid, xw, sliding, en, de in SLIDE_KEYS:
    POSES.append({"id": pid, "title": T(en, de), "description": T(f"Key of the sliding clip: the child at x = {xw:.2f} m.", f"Schlüssel des Rutschclips: das Kind bei x = {xw:.2f} m."), "key": True, "bones": {}, "ground": "authored", "contacts": POSE_CONTACTS, "thermal": binding(1.0, {"slide-stainless": {"contact": {"fraction": 0.03, "tempC": 33}}}), "loads": [child_on_slide(xw, sliding)]})
CLIPS = [
    {"id": "child-sliding", "title": T("A child slides down", "Ein Kind rutscht hinunter"), "description": T("A 30 kg child sits at the start, pushes off, slides through the transition arc and the 35 degree section and brakes on the run-out; the load moves along the bed with the normal pressure and the friction drag (mu 0.25) it puts on the stainless steel.", "Ein Kind mit 30 kg sitzt am Start, stößt sich ab, rutscht durch den Übergangsbogen und den 35-Grad-Abschnitt und bremst im Auslauf; die Last wandert mit dem Normaldruck und dem Reibungszug (mu 0,25), die es auf den Edelstahl bringt, das Bett entlang."), "durationS": 3.2, "fps": 25, "loop": False, "rootMotion": "none", "keys": [{"t": 0.0, "pose": "slide-seated", "ease": "smooth"}, {"t": 0.8, "pose": "slide-seated", "ease": "in"}, {"t": 1.35, "pose": "slide-arc", "ease": "linear"}, {"t": 1.85, "pose": "slide-slope", "ease": "linear"}, {"t": 2.45, "pose": "slide-runout", "ease": "out"}, {"t": 3.2, "pose": "slide-stopped", "ease": "smooth"}], "contacts": [{"anchor": a, "fromS": 0, "toS": 3.2} for a in SUPPORTS], "phases": [{"id": "seated", "fromS": 0.0, "toS": 0.8, "title": T("Seated", "Sitzen")}, {"id": "sliding", "fromS": 0.8, "toS": 2.45, "title": T("Sliding", "Rutschen")}, {"id": "braking", "fromS": 2.45, "toS": 3.2, "title": T("Braking on the run-out", "Bremsen im Auslauf")}], "events": [{"t": 0.8, "kind": "motion", "title": T("Push-off", "Abstoßen")}, {"t": 2.45, "kind": "motion", "title": T("Run-out reached", "Auslauf erreicht")}], "tracks": []},
    {"id": "dismantle-roof", "title": T("Dismantling the roof", "Dach abbauen"), "description": T("Depot maintenance: the loader crane takes the unbolted roof by its two lifting eyes, lifts it 0.4 m off the post heads, swings it 2.45 m back over the ladder and sets it down on the lawn on the ends of its tie beams.", "Wartung durch den Bauhof: der Ladekran nimmt das gelöste Dach an seinen zwei Anschlagösen, hebt es 0,4 m von den Pfostenköpfen, schwenkt es 2,45 m über die Leiter zurück und setzt es auf den Enden seiner Zugbalken auf dem Rasen ab."), "durationS": 16.0, "fps": 25, "loop": False, "rootMotion": "none", "keys": [{"t": 0.0, "pose": "in-service", "ease": "linear"}, {"t": 1.0, "pose": "roof-on-crane", "ease": "linear"}, {"t": 3.0, "pose": "roof-on-crane", "ease": "linear", "bones": {"roof": {"dy": 0.4}}}, {"t": 9.0, "pose": "roof-on-crane", "ease": "linear", "bones": {"roof-carry": {"dx": -2.45}, "roof": {"dy": 0.4}}}, {"t": 15.0, "pose": "maintenance", "ease": "linear"}, {"t": 16.0, "pose": "maintenance", "ease": "linear"}], "contacts": [*({"anchor": a, "fromS": 0, "toS": 16.0} for a in SUPPORTS)], "phases": [{"id": "lift", "fromS": 0.0, "toS": 3.0, "title": T("Lift off the post heads", "Von den Pfostenköpfen abheben")}, {"id": "travel", "fromS": 3.0, "toS": 9.0, "title": T("Travel back over the ladder", "Über die Leiter zurückschwenken")}, {"id": "set-down", "fromS": 9.0, "toS": 16.0, "title": T("Set down on the lawn", "Auf dem Rasen absetzen")}], "events": [{"t": 0.0, "kind": "motion", "title": T("Roof taken by its lifting eyes", "Dach an den Anschlagösen angeschlagen")}, {"t": 1.0, "kind": "load-on", "title": T("Crane carries the roof", "Kran trägt das Dach"), "target": "roof-carry-front-left"}, {"t": 15.0, "kind": "motion", "title": T("Roof set down", "Dach abgesetzt")}], "tracks": []},
]


# ---------------------------------------------------------------- texts
def rep_decisions(scale):
    """📝 The drawing decisions of one scale."""
    if scale == 50:
        return {
            "silhouette": T("Side elevation 1:50 of the whole used tower seen from its right: the two right posts on their H-anchors with the footings cut in the ground, the platform frame, the left barrier boards behind and the right ones in front, the 70 degree rung ladder on its shoes and strip footing, the gable frame (tie beam, rafters, king post) under the roof boards and ridge cap with the two lifting eyes, the stainless slide with its folded walls and rolled edges from the 0.4 m start section down the 35 degree section to the 0.5 m run-out on its leg and footing, and the four rubber tiles under the exit. Front and plan are the exact projections of the same parts.", "Seitenansicht 1:50 des ganzen gebrauchten Turms von rechts: die beiden rechten Pfosten auf ihren H-Ankern mit den im Erdreich geschnittenen Fundamenten, der Podestrahmen, die linken Brüstungsbretter dahinter und die rechten davor, die 70-Grad-Sprossenleiter auf Schuhen und Streifenfundament, der Giebelbinder (Zugbalken, Sparren, Hängesäule) unter Schalung und Firstbrett mit den zwei Anschlagösen, die Edelstahlrutsche mit gekanteten Wangen und Rollkanten vom 0,4 m Startabschnitt über den 35-Grad-Abschnitt bis zum 0,5 m Auslauf auf Stütze und Fundament und die vier Fallschutzplatten unter dem Ausstieg. Vorderansicht und Draufsicht sind die genauen Projektionen derselben Teile."),
            "retained": T("Every board of the barriers, the seven deck boards with the replaced fresh one, both nosings, the H-anchor plates with web and nuts, the ladder shoes and angles, five rungs, the grab bars with their stays, flange and saddle of the slide, the lifting eyes with collars and every tile.", "Jedes Brüstungsbrett, die sieben Belagbretter mit dem frisch ersetzten, beide Stirnbretter, die H-Ankerbleche mit Steg und Muttern, Leiterschuhe und Winkel, fünf Sprossen, die Haltegriffe mit ihren Stützen, Flansch und Sattel der Rutsche, die Anschlagösen mit Bunden und jede Platte."),
            "merged": T("Bolts through the posts are shown by their nuts only; concealed joist hangers are inside the beam ends; the rolled edge of the slide wall is drawn as the band it covers in the side elevation.", "Die Bolzen durch die Pfosten erscheinen nur mit ihren Muttern; verdeckte Balkenschuhe liegen in den Balkenenden; die Rollkante der Rutschwange ist in der Seitenansicht als das Band gezeichnet, das sie deckt."),
            "omitted": T("The loose sand of the impact area (the ground surface y = 0 itself), screws and nails, the zinc coating thickness.", "Der lose Sand der Aufprallfläche (die Geländeoberfläche y = 0 selbst), Schrauben und Nägel, die Dicke der Verzinkung."),
            "thinElements": T("The 2 mm slide sheets and the 3 mm flange keep their true thickness as declared sheets; the 8 mm deck joints are texture lines in plan because they are under the 11 mm white gap that prints at 1:75.", "Die 2-mm-Rutschbleche und der 3-mm-Flansch behalten als erklärte Bleche ihre wahre Dicke; die 8-mm-Belagfugen sind in der Draufsicht Texturlinien, weil sie unter der 11-mm-Weißlücke liegen, die bei 1:75 druckt."),
            "hatching": T("Stipple for the concrete footings cut in the ground, dots for the rubber granulate tiles.", "Punktraster für die im Erdreich geschnittenen Betonfundamente, Punkte für die Fallschutzplatten aus Gummigranulat."),
            "styles": T("Greyed larch in a mid grey, the north posts and frame members darker, the replaced deck board light, the stainless slide almost white with a lighter polished track, galvanised parts light grey, tiles black.", "Vergraute Lärche in mittlerem Grau, die Nordpfosten und Rahmenteile dunkler, das ersetzte Belagbrett hell, die Edelstahlrutsche fast weiß mit hellerer Polierspur, verzinkte Teile hellgrau, Platten schwarz."),
            "overlays": T("Traces of use as texture: drying checks in the posts, algae at the foot of the north-east post, rust at an anchor, a graffiti tag and a sticker on the right barrier, chewing gum on the deck, scratches and the polished track on the slide bed, a sticker on the slide wall, worn rung tops, moss on the north roof slope and the worn hollow in the exit tile.", "Gebrauchsspuren als Textur: Trockenrisse in den Pfosten, Algen am Fuß des nordöstlichen Pfostens, Rost an einem Anker, ein Graffiti-Tag und ein Aufkleber an der rechten Brüstung, Kaugummi auf dem Belag, Kratzer und die polierte Spur im Rutschbett, ein Aufkleber an der Rutschwange, abgetretene Sprossen, Moos auf der Nordseite des Dachs und die ausgetretene Mulde in der Ausstiegsplatte."),
        }
    if scale == 100:
        return {
            "silhouette": T("Side elevation 1:100 of the same tower drawn from its own model: posts on merged anchor blocks, the platform with beams, joist and one deck plate, every barrier board, the ladder with stringers, shoes and five rungs, the gable frames, roof boards, ridge cap and eyes, the slide with walls, square rolled edges and flange, the leg and the four tiles.", "Seitenansicht 1:100 desselben Turms aus seinem eigenen Modell: Pfosten auf zusammengefassten Ankerblöcken, das Podest mit Balken, Wechsel und einer Belagplatte, jedes Brüstungsbrett, die Leiter mit Holmen, Schuhen und fünf Sprossen, die Giebelbinder, Schalung, Firstbrett und Ösen, die Rutsche mit Wangen, rechteckigen Rollkanten und Flansch, die Stütze und die vier Platten."),
            "retained": T("Barrier boards and their 73 mm gaps, the rungs, the grab bars, the eyes, the tile joints.", "Die Brüstungsbretter und ihre 73-mm-Lücken, die Sprossen, die Haltegriffe, die Ösen, die Plattenfugen."),
            "merged": T("Anchor plates, web and nuts into one 100 x 136 mm block per post; the seven deck boards and the nosings into one plate around the post corners; the saddle into the leg.", "Ankerbleche, Steg und Muttern zu einem Block 100 x 136 mm je Pfosten; die sieben Belagbretter und die Stirnbretter zu einer Platte um die Pfostenecken; der Sattel in die Stütze."),
            "omitted": T("Ladder angles, the eye collars, the fresh board as a separate part.", "Leiterwinkel, die Ösenbunde, das frische Brett als eigenes Teil."),
            "thinElements": T("Slide sheets and flange stay declared sheets at their true 2 and 3 mm.", "Rutschbleche und Flansch bleiben erklärte Bleche mit ihren wahren 2 und 3 mm."),
            "hatching": T("Stipple for the concrete footings.", "Punktraster für die Betonfundamente."),
            "styles": T("The tones of 1:50, without the light tone of the replaced board.", "Die Töne von 1:50, ohne den hellen Ton des ersetzten Bretts."),
            "overlays": T("Fewer traces: one check per right post, the post checks of the front, moss on the north slope and the polished track on the slide.", "Weniger Spuren: ein Riss je rechtem Pfosten, die Pfostenrisse der Vorderansicht, Moos auf der Nordseite und die polierte Spur auf der Rutsche."),
        }
    if scale == 200:
        return {
            "silhouette": T("Side elevation 1:200 from the model of this scale: posts and anchor blocks, the platform as one 173 mm deck mass, every barrier board, ladder with shoes and rungs, gable frames with the ridge merged into one beam under the boards, slide with walls carrying the rolled edge, the leg and one tile mat.", "Seitenansicht 1:200 aus dem Modell dieses Maßstabs: Pfosten und Ankerblöcke, das Podest als eine 173-mm-Belagmasse, jedes Brüstungsbrett, Leiter mit Schuhen und Sprossen, Giebelbinder mit dem zu einem Balken unter der Schalung zusammengefassten First, Rutsche mit Wangen, die die Rollkante tragen, die Stütze und eine Plattenmatte."),
            "retained": T("The barrier boards with their gaps (0.36 mm on paper), the five rungs, the king posts, the start and run-out sections of the slide.", "Die Brüstungsbretter mit ihren Lücken (0,36 mm auf dem Papier), die fünf Sprossen, die Hängesäulen, Start- und Auslaufabschnitt der Rutsche."),
            "merged": T("Beams, joist and deck into one mass; ridge beam and cap into one ridge; the rolled edges into the wall bands; the four tiles into one mat.", "Balken, Wechsel und Belag zu einer Masse; Firstpfette und Firstbrett zu einem First; die Rollkanten in die Wangenbänder; die vier Platten zu einer Matte."),
            "omitted": T("Grab bars, lifting eyes, flange and saddle, every texture.", "Haltegriffe, Anschlagösen, Flansch und Sattel, jede Textur."),
            "thinElements": T("The 22 mm barrier boards are drawn at their true thickness in front and plan, so their solids keep it; the slide walls and bed stay declared sheets.", "Die 22-mm-Brüstungsbretter sind in Vorderansicht und Draufsicht in ihrer wahren Dicke gezeichnet, ihre Körper behalten sie daher; Rutschwangen und Bett bleiben erklärte Bleche."),
            "hatching": T("Stipple for the footings.", "Punktraster für die Fundamente."),
            "styles": T("Tones only, no texture pen at this scale.", "Nur Töne, kein Texturstift in diesem Maßstab."),
            "overlays": T("None.", "Keine."),
        }
    return {
        "silhouette": T("Side elevation 1:500 from its own model: posts on anchor blocks, the deck mass, each barrier as one typified panel, the ladder with its stringers and rungs, the roof as two slope masses of boards and rafters over the tie beams and king posts, the slide as bed and two wall bands, the leg, the tile mat and the footings.", "Seitenansicht 1:500 aus dem eigenen Modell: Pfosten auf Ankerblöcken, die Belagmasse, jede Brüstung als eine typisierte Tafel, die Leiter mit Holmen und Sprossen, das Dach als zwei Dachflächenmassen aus Schalung und Sparren über Zugbalken und Hängesäulen, die Rutsche als Bett und zwei Wangenbänder, die Stütze, die Plattenmatte und die Fundamente."),
        "retained": T("The rungs and the open triangles of the gable frames, the run-out height.", "Die Sprossen und die offenen Dreiecke der Giebelbinder, die Auslaufhöhe."),
        "merged": T("Boards, rails and gaps of each lateral barrier into one envelope panel (decision of typification); the front and back infill with the rail into short panels; rafters with the boards into each slope.", "Bretter, Handläufe und Lücken jeder seitlichen Brüstung zu einer Hüllentafel (Typisierung); vordere und hintere Füllung mit dem Handlauf zu kurzen Tafeln; Sparren mit der Schalung zu je einer Dachfläche."),
        "omitted": T("Shoes detail, grab bars, eyes, ridge cap, flange, saddle, joints, texture.", "Schuhdetail, Haltegriffe, Ösen, Firstbrett, Flansch, Sattel, Fugen, Textur."),
        "thinElements": T("The 45 mm stringers and rungs, the 60 mm leg and the 45 mm tile mat are drawn at their true size (0.09 to 0.12 mm plus the pen), the slide sheets stay declared sheets.", "Die 45-mm-Holme und -Sprossen, die 60-mm-Stütze und die 45-mm-Plattenmatte sind in wahrer Größe gezeichnet (0,09 bis 0,12 mm plus Stift), die Rutschbleche bleiben erklärte Bleche."),
        "hatching": T("Stipple for the footings.", "Punktraster für die Fundamente."),
        "styles": T("Body pen only, tones for timber, steel, stainless, rubber and concrete.", "Nur Körperstift, Töne für Holz, Stahl, Edelstahl, Gummi und Beton."),
        "overlays": T("None.", "Keine."),
    }


def vol_decisions(scale):
    """🧾 The volume decisions of one scale (body, thermal, structure)."""
    body = {
        50: T("Body 1:50, the used tower part by part: four 120 mm larch posts from 0.10 m on H-anchors (two 100 x 8 mm plates reaching 0.30 m into the footing top, web, nuts), 400 x 400 x 600 mm point footings with their top 200 mm under the sand, 45 x 145 edge and front/back beams and a joist, seven 28 x 147 deck boards with 8 mm joints (the third replaced in fresh larch) and two nosings, six 22 x 95 boards per lateral barrier with 73 mm gaps, front and back infill boards, 45 x 95 rails, two grab bars with stays, a 70 degree ladder of two 45 x 145 stringers in shoes with angles, five robinia rungs of 45 mm, strip footing, the stainless slide of a 2 mm bed with the rolled end lip, two 2 mm folded walls, round rolled edges lofted along the path, 3 mm flange on the front beam, the 60.3 leg with saddle and footing, four 45 mm rubber tiles, and the gable roof: two gable frames (tie beam 45 x 100, rafters 45 x 95, king post), ridge beam, two 24 mm board slopes, ridge cap and two lifting eyes with collars.", "Körper 1:50, der gebrauchte Turm Teil für Teil: vier Lärchenpfosten 120 mm ab 0,10 m auf H-Ankern (zwei Bleche 100 x 8 mm, die 0,30 m in den Fundamentkopf reichen, Steg, Muttern), Punktfundamente 400 x 400 x 600 mm mit der Oberkante 200 mm unter dem Sand, Rand- und Stirnbalken 45 x 145 und ein Wechsel, sieben Belagbretter 28 x 147 mit 8-mm-Fugen (das dritte in frischer Lärche ersetzt) und zwei Stirnbretter, sechs Bretter 22 x 95 je Seitenbrüstung mit 73-mm-Lücken, vordere und hintere Füllbretter, Handläufe 45 x 95, zwei Haltegriffe mit Stützen, eine 70-Grad-Leiter aus zwei Holmen 45 x 145 in Schuhen mit Winkeln, fünf Robiniensprossen 45 mm, Streifenfundament, die Edelstahlrutsche aus 2-mm-Bett mit gerollter Endlippe, zwei gekanteten 2-mm-Wangen, runden Rollkanten als Loft entlang der Bahn, 3-mm-Flansch am Stirnbalken, die Stütze 60,3 mit Sattel und Fundament, vier Fallschutzplatten 45 mm und das Satteldach: zwei Giebelbinder (Zugbalken 45 x 100, Sparren 45 x 95, Hängesäule), Firstpfette, zwei Schalungsflächen 24 mm, Firstbrett und zwei Anschlagösen mit Bunden."),
        100: T("Body 1:100, authored anew for 15 mm: anchors as one 100 x 136 mm block per post, the deck as one 28 mm plate around the post corners, rolled edges as square 30 mm bands, ladder shoes as blocks; ladder angles, nuts, saddle and eye collars left out; every barrier board, rung, grab bar and tile kept.", "Körper 1:100, neu für 15 mm erstellt: Anker als ein Block 100 x 136 mm je Pfosten, der Belag als eine 28-mm-Platte um die Pfostenecken, Rollkanten als rechteckige 30-mm-Bänder, Leiterschuhe als Blöcke; Leiterwinkel, Muttern, Sattel und Ösenbunde entfallen; jedes Brüstungsbrett, jede Sprosse, jeder Haltegriff und jede Platte bleiben."),
        200: T("Body 1:200, authored anew for 30 mm: the platform is one 173 mm deck mass, ridge beam and cap one ridge, the rolled edges are part of the 165 mm wall sheets, the tiles one mat; grab bars, eyes and flange left out; the 22 mm barrier boards keep their drawn thickness.", "Körper 1:200, neu für 30 mm erstellt: das Podest ist eine Belagmasse von 173 mm, Firstpfette und Firstbrett ein First, die Rollkanten gehören zu den 165-mm-Wangenblechen, die Platten sind eine Matte; Haltegriffe, Ösen und Flansch entfallen; die 22-mm-Brüstungsbretter behalten ihre gezeichnete Dicke."),
        500: T("Body 1:500, authored anew for 75 mm: each lateral barrier is one typified envelope panel (envelope: true) over the span of its boards, the front and back infill short panels with the rail, each roof slope one mass of boards and rafters, the slide bed and two wall bands as declared sheets; ladder stringers and rungs stay at their drawn true size.", "Körper 1:500, neu für 75 mm erstellt: jede Seitenbrüstung ist eine typisierte Hüllentafel (envelope: true) über die Spannweite ihrer Bretter, die vordere und hintere Füllung kurze Tafeln mit dem Handlauf, jede Dachfläche eine Masse aus Schalung und Sparren, Rutschbett und zwei Wangenbänder als erklärte Bleche; Leiterholme und Sprossen bleiben in ihrer gezeichneten wahren Größe."),
    }[scale]
    thermal = T("Thermal: every body solid by reference in one of eight single-layer passive regions (greyed larch frame, deck, roof boards, stainless slide, galvanised steel, robinia rungs, concrete footings, rubber tiles); no emitters, no hidden regions, no electrical rating.", "Thermik: jeder Körper als Referenz in einer von acht einschichtigen passiven Zonen (vergrautes Lärchengerüst, Belag, Dachschalung, Edelstahlrutsche, verzinkter Stahl, Robiniensprossen, Betonfundamente, Fallschutzplatten); keine Emitter, keine verborgenen Zonen, keine elektrische Leistung.")
    structure = T("Structure: the four posts as 120 x 120 members from the ground to the post heads with fixed supports at the anchors, the platform frame as its four beams, the ladder as its stringers with a pin at the shoes, the slide channel (bed and walls) with the leg and a pin in its footing, the roof as its gable frames on the post heads; masses at the computed centres of every bone.", "Tragwerk: die vier Pfosten als Stäbe 120 x 120 vom Boden bis zu den Pfostenköpfen mit Einspannungen an den Ankern, der Podestrahmen als seine vier Balken, die Leiter als ihre Holme mit einem Gelenk an den Schuhen, die Rutschrinne (Bett und Wangen) mit der Stütze und einem Gelenk im Fundament, das Dach als seine Giebelbinder auf den Pfostenköpfen; Massen in den berechneten Schwerpunkten jedes Knochens.")
    return [body, thermal, structure]


REFERENCES = [
    {"id": "en-1176-1", "citation": "DIN EN 1176-1:2017-12 Playground equipment and surfacing - Part 1: General safety requirements and test methods (protection against falling: barriers at least 700 mm high on platforms above 1.0 m free height of fall, openings under 89 mm against head entrapment, foundations covered by at least 200 mm of loose surfacing; free height of fall and impact area)."},
    {"id": "en-1176-3", "citation": "DIN EN 1176-3:2017-12 Playground equipment and surfacing - Part 3: Additional specific safety requirements and test methods for slides (start section, sliding section inclination at most 40 degrees, side protection, run-out of at least 0.5 m with an end at most 0.35 m high for slides longer than 1.5 m)."},
    {"id": "en-1176-7", "citation": "DIN EN 1176-7:2020-07 Playground equipment and surfacing - Part 7: Guidance on installation, inspection, maintenance and operation (routine visual, operational and annual main inspection, replacement of worn parts by the operator)."},
    {"id": "en-1177", "citation": "DIN EN 1177:2018-03 Impact attenuating playground surfacing - Methods of test for determination of impact attenuation (critical fall height of loose sand and of bonded rubber tiles)."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber - Strength classes (C24: mean modulus of elasticity parallel 11 000 N/mm2, bending strength 24 N/mm2)."},
    {"id": "din-68364", "citation": "DIN 68364:2003-05 Kennwerte von Holzarten - Rohdichte, Elastizitätsmodul und Festigkeiten (European larch about 590 kg/m3, robinia about 730 kg/m3 at 12 % moisture content)."},
    {"id": "din-68800-2", "citation": "DIN 68800-2:2022-02 Holzschutz - Teil 2: Vorbeugende bauliche Maßnahmen im Hochbau (structural wood protection: timber held clear of the ground and of standing water on steel anchors)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads for buildings (plain concrete 24 kN/m3)."},
    {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (tube 60.3 x 2.9 mm: A = 5.23 cm2, I = 21.6 cm4)."},
    {"id": "en-10088-2", "citation": "EN 10088-2:2014 Stainless steels - Part 2: Technical delivery conditions for sheet/plate and strip of corrosion resisting steels for general purposes (1.4301, density 7.9 kg/dm3)."},
    {"id": "en-iso-1461", "citation": "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles - Specifications and test methods."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, density and specific heat of timber, concrete, rubber and metals)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity and solar absorptivity of stainless steel, galvanised steel and dark polymers)."},
    {"id": "williams-2005", "citation": "Williams, R. S. (2005). Weathering of wood. In: Rowell, R. M. (ed.), Handbook of Wood Chemistry and Wood Composites, CRC Press, chapter 7 (greying, checking and erosion of exposed softwood; a weathered grey surface absorbs more solar radiation than fresh wood)."},
]


def document():
    """📄 The whole asset document."""
    reps, vols = {}, {}
    for scale in (500, 200, 100, 50):
        P = parts(scale)
        ids = [p.id for p in P]
        assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]
        reps[str(scale)] = representation(scale, P, rep_decisions(scale))
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": [solid(p) for p in P if p.build], "thermal": thermal_list([p for p in P if p.build]), "structure": structure(scale, P), "decisions": vol_decisions(scale)}
    P = [p for p in parts(50) if p.build]
    bb = [p.bounds() for p in P]
    dims = {"heightM": r6(max(b[3] for b in bb) - min(b[2] for b in bb)), "widthM": r6(max(b[1] for b in bb) - min(b[0] for b in bb)), "depthM": r6(max(b[5] for b in bb) - min(b[4] for b in bb)), "massKg": TOTAL_MASS}
    rig = []
    for b in BONES:
        e = {"id": b["id"], "parent": b["parent"], "title": b["title"], "x": r6(b["x"]), "y": r6(b["y"])}
        if b["z"]:
            e["z"] = r6(b["z"])
        e.update(angleDeg=r6(b["angle"]), lengthM=r6(b["length"]), joint=b["joint"])
        if b["id"] in MASS:
            m, u, v, _ = MASS[b["id"]]
            e["mass"] = {"kg": round(m, 2), "comX": r6(u), "comY": r6(v), "distribution": "point", "source": "literature", "reference": "din-68364" if b["id"] not in ("footings", "slide", "slide-leg") else ("en-1991-1-1" if b["id"] == "footings" else "en-10088-2")}
        if b["section"]:
            e["section"] = b["section"]
        rig.append(e)
    asset = {
        "id": "used-slide-tower",
        "category": "objects",
        "title": T("Used Slide Tower", "Gebrauchter Rutschenturm"),
        "summary": T(
            f"A municipal play tower after years in a German park: four greyed 120 mm larch posts on galvanised H-anchors in point footings, a 1.2 m square platform at 1.50 m with 0.75 m board barriers, a 70 degree rung ladder with shoe-worn robinia rungs, a gable roof with moss on its north slope and a 2 mm stainless steel slide with a 35 degree section and a 0.5 m run-out at 0.30 m, its bed scratched and polished by use; {dims['widthM']:.2f} m long, {TOTAL_MASS:.0f} kg with its footings. The depot lifts the roof off with a crane; children climb, sit at the start and slide; in summer sun the stainless bed is the hot surface.",
            f"Ein kommunaler Spielturm nach Jahren in einem deutschen Park: vier vergraute Lärchenpfosten 120 mm auf verzinkten H-Ankern in Punktfundamenten, ein 1,2 m großes Podest auf 1,50 m mit 0,75 m hohen Bretterbrüstungen, eine 70-Grad-Sprossenleiter mit abgetretenen Robiniensprossen, ein Satteldach mit Moos auf der Nordseite und eine Edelstahlrutsche aus 2-mm-Blech mit 35-Grad-Abschnitt und 0,5 m Auslauf auf 0,30 m, ihr Bett vom Gebrauch zerkratzt und poliert; {dims['widthM']:.2f} m lang, {TOTAL_MASS:.0f} kg mit Fundamenten. Der Bauhof hebt das Dach mit dem Kran ab; Kinder klettern, sitzen am Start und rutschen; in der Sommersonne ist das Edelstahlbett die heiße Fläche.".replace(".", ",", 0)),
        "tags": ["used", "park", "public-space", "play", "playground", "slide", "tower", "timber", "larch", "stainless-steel", "galvanised-steel", "footing"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T(
            "Catalogue-typical municipal play tower (type, no maker): posts 120 x 120 larch C24 at 1.20 m centres, platform 1.50 m (free height of fall 1.5 m, EN 1176-1), barriers 0.75 m above the deck with vertical boards and 73 mm gaps (under 89 mm), ladder 70 degrees with rungs every 0.25 m, slide after EN 1176-3 with a 0.40 m start section, a 1.0 m and a 1.2 m transition arc, a 35 degree sliding section and a 0.50 m run-out at 0.30 m, bed 450 mm wide with 150 mm walls; point footings 400 x 400 x 600 mm with their top 200 mm under the sand (EN 1176-1). Traces of use and weather are authored: greyed and checked larch, one replaced deck board, moss on the north roof slope, algae at a north post foot, rust at an anchor, graffiti and stickers, chewing gum, shoe-worn rungs, a scratched and polished slide bed and a worn hollow in the exit tile. The child loads are authored estimates of real use (30 kg each), not the design user loads of EN 1176-1. Every mass is section x density of the 1:50 parts; nothing is measured.",
            "Katalogtypischer kommunaler Spielturm (Typ, kein Hersteller): Pfosten 120 x 120 Lärche C24 im Achsmaß 1,20 m, Podest 1,50 m (freie Fallhöhe 1,5 m, EN 1176-1), Brüstungen 0,75 m über dem Belag mit senkrechten Brettern und 73-mm-Lücken (unter 89 mm), Leiter 70 Grad mit Sprossen alle 0,25 m, Rutsche nach EN 1176-3 mit 0,40 m Startabschnitt, einem 1,0-m- und einem 1,2-m-Übergangsbogen, 35-Grad-Rutschabschnitt und 0,50 m Auslauf auf 0,30 m, Bett 450 mm breit mit 150-mm-Wangen; Punktfundamente 400 x 400 x 600 mm mit der Oberkante 200 mm unter dem Sand (EN 1176-1). Gebrauchs- und Witterungsspuren sind festgelegt: vergraute und gerissene Lärche, ein ersetztes Belagbrett, Moos auf der Nordseite des Dachs, Algen an einem Nordpfostenfuß, Rost an einem Anker, Graffiti und Aufkleber, Kaugummi, abgetretene Sprossen, ein zerkratztes und poliertes Rutschbett und eine ausgetretene Mulde in der Ausstiegsplatte. Die Kinderlasten sind Schätzungen der wirklichen Nutzung (je 30 kg), nicht die Bemessungs-Nutzerlasten der EN 1176-1. Jede Masse ist Querschnitt x Dichte der 1:50-Teile; nichts ist gemessen.")},
        "dimensions": dims,
        "materials": MATERIALS,
        "rig": {"kind": "structural", "root": "ground", "bones": rig},
        "anchors": ANCHORS,
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {
            "environment": {"airTempC": 32, "meanRadiantTempC": 36, "skyTempC": 21, "groundTempC": 50, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 850, "diffuseFraction": 0.12, "pressurePa": 101325},
            "referenceEnvironment": "summer-hot-sun",
            "passengers": [],
            "operating": [],
            "regions": REGIONS,
            "emitters": [],
            "assumptions": [
                T("Hot summer afternoon (summer-hot-sun: 32 degC air, 850 W/m2 sun, 1.5 m/s wind). The stainless slide in full sun (85 % sunlit) with absorptance 0.5 and a low emissivity of 0.3 for a scratched, dulled 1.4301 surface sheds little heat by radiation, so its thin 2 mm bed follows the sun quickly and is the hot surface of the tower; the black rubber tiles (absorptance 0.92) heat as well, the deck under the roof stays near air temperature.", "Heißer Sommernachmittag (summer-hot-sun: 32 degC Luft, 850 W/m2 Sonne, 1,5 m/s Wind). Die Edelstahlrutsche in voller Sonne (85 % besonnt) mit Absorptionsgrad 0,5 und dem niedrigen Emissionsgrad 0,3 einer zerkratzten, stumpfen 1.4301-Oberfläche gibt wenig Wärme durch Strahlung ab, ihr dünnes 2-mm-Bett folgt der Sonne daher schnell und ist die heiße Fläche des Turms; die schwarzen Fallschutzplatten (Absorptionsgrad 0,92) erwärmen sich ebenfalls, der Belag unter dem Dach bleibt nahe der Lufttemperatur."),
                T("Weathered surfaces differ from new ones: greyed larch absorbs about 0.68 of the sun against about 0.55 for the fresh replaced board (Williams 2005), weathered galvanising has lost its bright emissivity of about 0.05 and is taken at 0.28 with absorptance 0.65 (authored estimates after Incropera 2018).", "Verwitterte Oberflächen unterscheiden sich von neuen: vergraute Lärche absorbiert etwa 0,68 der Sonne gegen etwa 0,55 beim frisch ersetzten Brett (Williams 2005), verwitterte Verzinkung hat ihren blanken Emissionsgrad von etwa 0,05 verloren und wird mit 0,28 bei Absorptionsgrad 0,65 angesetzt (Schätzungen nach Incropera 2018)."),
                T("The footings are in the ground: exposed fraction 0, in contact with soil at 14 degC. Heat capacities rho*cp*min(thickness, sqrt(alpha*3600 s)) with ISO 10456 values; the slide sheet is exposed on both faces, so 1 mm of it is counted per face.", "Die Fundamente liegen im Erdreich: freier Anteil 0, in Kontakt mit Boden von 14 degC. Wärmekapazitäten rho*cp*min(Dicke, sqrt(alpha*3600 s)) mit ISO-10456-Werten; das Rutschblech ist beidseitig frei, daher zählt je Seite 1 mm."),
            ],
        },
        "structure": {
            "gravityMS2": G,
            "dynamics": "quasi-static",
            "supports": SUPPORTS,
            "assumptions": [
                T("The four posts are fixed at ground level in their H-anchors and point footings (fixed supports), the ladder shoes and the run-out leg are pins in their footings; the tower is solved as one rigid body on these six supports with the member cuts of each bone.", "Die vier Pfosten sind in Geländehöhe in ihren H-Ankern und Punktfundamenten eingespannt, Leiterschuhe und Auslaufstütze sind Gelenke in ihren Fundamenten; der Turm wird als ein starrer Körper auf diesen sechs Auflagern mit den Stabschnitten jedes Knochens gelöst."),
                T("In use the children are rigid point loads of 30 kg (authored estimate); a sliding child presses the bed with m g cos(alpha) and drags it with the friction mu = 0.25 of clothing on stainless steel; the design user loads, dynamic factors and wind loads of EN 1176-1 and EN 1991-1-4 are not applied.", "In Benutzung sind die Kinder starre Punktlasten von 30 kg (Schätzung); ein rutschendes Kind drückt das Bett mit m g cos(alpha) und zieht es mit der Reibung mu = 0,25 von Kleidung auf Edelstahl; die Bemessungs-Nutzerlasten, dynamischen Beiwerte und Windlasten der EN 1176-1 und EN 1991-1-4 werden nicht angesetzt."),
                T("Dismantled, the roof rests on the ends of its two tie beams on the lawn ; the crane sling and then the lawn carry it at these four points, applied as four carrying forces of a quarter of the roof weight each (0.23 kN, together 0.92 kN), because supports of the archive are springs of equal stiffness that would share the weight of the whole tower with the roof rests; the crane itself is not part of the asset.", "Abgebaut liegt das Dach auf den Enden seiner zwei Zugbalken auf dem Rasen ; das Krangehänge und danach der Rasen tragen es an diesen vier Punkten, angesetzt als vier Tragkräfte von je einem Viertel des Dachgewichts (0,23 kN, zusammen 0,92 kN), weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht des ganzen Turms mit den Dachauflagen teilen würden; der Kran selbst gehört nicht zum Objekt."),
            ],
            "clinical": False,
        },
        "volumes": vols,
        "presentation": {"defaultPose": "in-service", "defaultClip": "child-sliding", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    a = doc["asset"]
    print("[DEBUG] dimensions", a["dimensions"])
    for bid, (m, u, v, z) in sorted(MASS.items(), key=lambda kv: -kv[1][0]):
        print(f"[DEBUG] mass {bid:14s} {m:8.2f} kg com ({u:.3f}, {v:.3f})")
    for s in ("50", "100", "200", "500"):
        vv = a["volumes"][s]
        print(f"[DEBUG] 1:{s} body {len(vv['body'])} thermal {len(vv['thermal'])} structure {len(vv['structure'])} shapes", {k: len(x) for k, x in a["representations"][s]["geometry"].items()})
