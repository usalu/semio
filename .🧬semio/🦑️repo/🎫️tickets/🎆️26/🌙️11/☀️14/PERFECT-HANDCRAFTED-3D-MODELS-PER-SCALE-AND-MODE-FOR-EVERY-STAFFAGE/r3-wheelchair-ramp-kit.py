"""♿ Round 3 wheelchair ramp: the measured facts of the real ramp and the small solid writers every list uses.

Frames: bone `body` is the world (u ahead and up the ramp, v up, w lateral, left +); bone `handrail` starts 1.2 m ahead, so its u is world x - 1.2.
"""
import math

TH = math.atan(1 / 12)
C, S = math.cos(TH), math.sin(TH)
HALF = 0.7
CLEAR = 0.6
RAIL_Z = 0.65
RAIL_R = 0.0212
RAIL_WALL = 0.0026
RAIL_H = 0.85
BEND = 0.06
POST_R = 0.02415
POST_WALL = 0.0032
POST_TOP = 0.669
POST_FULL = 0.729
STEM_R = 0.008
PLATE = [0.12, 0.01, 0.07]
NUT_R = 0.0098
NUT_H = 0.011
KERB = 0.1
CHAMFER = 0.01
SLAB_T = 0.15
WALL_T = 0.2
POSTS = [1.6 + 1.7 * k for k in range(6)]
POST_STEP = [1.7, 1.7 / 12, 0]
RUN_LEN = 3.6 / C
JOINTS = [5.1, 6.6, 10.2]
POURS = {
    "run-1": [[1.5, 0], [5.1, 0.3], [5.1, 0]],
    "landing-middle": [[5.1, 0], [5.1, 0.3], [6.6, 0.3], [6.6, 0]],
    "run-2": [[6.6, 0], [6.6, 0.3], [10.2, 0.6], [10.2, 0]],
    "landing-top": [[10.2, 0], [10.2, 0.6], [11.7, 0.6], [11.7, 0]],
}
WEDGE = [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0]]
RAIL_KINKS = [[0.3, 0.85], [3.9, 1.15], [5.4, 1.15], [9.0, 1.45]]


def r5(v):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(v, 5)


def h(x):
    """📐 Height of the walking surface at world x."""
    if x <= 1.5:
        return 0.0
    if x <= 5.1:
        return (x - 1.5) / 12
    if x <= 6.6:
        return 0.3
    if x <= 10.2:
        return 0.3 + (x - 6.6) / 12
    return 0.6


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def qz(deg):
    """🧭 Quaternion of a turn about the lateral axis."""
    a = math.radians(deg) / 2
    return [0, 0, r5(math.sin(a)), r5(math.cos(a))]


def qsection(deg):
    """🎯 Quaternion that lays a loft section across a path running at this angle in the sagittal plane."""
    a = math.radians(deg) / 2
    k = math.sqrt(0.5)
    return [round(-math.sin(a) * k, 6), round(math.cos(a) * k, 6), round(math.sin(a) * k, 6), round(math.cos(a) * k, 6)]


def base(sid, kind, bone, material, tone, role="body"):
    """🧱 The common head of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def extrude(sid, bone, material, tone, profile, plane, lo, hi, **more):
    """🧊 A prism."""
    out = base(sid, "extrude", bone, material, tone)
    out.update({"profile": [[r5(a), r5(b)] for a, b in profile], "plane": plane, "from": r5(lo), "to": r5(hi)})
    out.update(more)
    return out


def box(sid, bone, material, tone, size, at, **more):
    """📦 A box at a place."""
    out = base(sid, "box", bone, material, tone)
    out.update({"size": [r5(v) for v in size], "transform": {"translation": [r5(v) for v in at]}})
    out.update(more)
    return out


def revolve(sid, bone, material, tone, profile, axis, origin, **more):
    """🏺 A turned part."""
    out = base(sid, "revolve", bone, material, tone)
    out.update({"profile": [[r5(a), r5(b)] for a, b in profile], "axis": [round(v, 6) for v in axis], "origin": [r5(v) for v in origin]})
    out.update(more)
    return out


def tube(length, outer, wall):
    """⭕ The loop profile of a hollow tube wall."""
    return [[0, outer - wall], [0, outer], [length, outer], [length, outer - wall], [0, outer - wall]]


def rod(length, radius):
    """➖ The open profile of a solid rod."""
    return [[0, 0], [0, radius], [length, radius], [length, 0]]


def loft(sid, bone, material, tone, sections, **more):
    """🌀 A skin over circular sections with stated directions."""
    out = base(sid, "loft", bone, material, tone)
    out["sections"] = [{"at": [r5(x), r5(y), r5(z)], "rx": r, "ry": r, "rotation": qsection(deg)} for x, y, z, r, deg in sections]
    out.update({"capStart": True, "capEnd": True})
    out.update(more)
    return out


def bend_start(z, radius=RAIL_R, tail=0.0):
    """↩️ Sections of the lower rail end: capped leg, 90 degree bend of 60 mm radius, start of the level extension."""
    cx, cy = RAIL_R + BEND, RAIL_H - BEND
    out = [[RAIL_R, 0.75, z, radius, 90], [RAIL_R, cy, z, radius, 90]]
    for a in (165, 150, 135, 120, 105):
        out.append([cx + BEND * math.cos(math.radians(a)), cy + BEND * math.sin(math.radians(a)), z, radius, a - 90])
    out.append([cx, RAIL_H, z, radius, 0])
    if tail:
        out.append([cx + tail, RAIL_H, z, radius, 0])
    return out


def bend_end(z, radius=RAIL_R, tail=0.0):
    """↪️ Sections of the upper rail end: end of the level extension, bend, capped leg."""
    cx, cy = 9.3 - RAIL_R - BEND, 1.45 - BEND
    out = [[cx - tail, 1.45, z, radius, 0]] if tail else []
    out.append([cx, 1.45, z, radius, 0])
    for a in (75, 60, 45, 30, 15):
        out.append([cx + BEND * math.cos(math.radians(a)), cy + BEND * math.sin(math.radians(a)), z, radius, a - 90])
    out += [[9.3 - RAIL_R, cy, z, radius, -90], [9.3 - RAIL_R, 1.35, z, radius, -90]]
    return out


def kinks(z, radius=RAIL_R):
    """📈 Mitre sections of the rail at the four slope changes."""
    return [[x, y, z, radius, math.degrees(TH) / 2] for x, y in RAIL_KINKS]


def kerb_section(mid):
    """🔲 Cross-section (v, w) of the 100 x 100 mm wheel guard with 10 mm chamfered arrises, centred on w = mid."""
    a, b = mid - KERB / 2, mid + KERB / 2
    return [[0, a], [0, b], [KERB - CHAMFER, b], [KERB, b - CHAMFER], [KERB, a + CHAMFER], [KERB - CHAMFER, a]]


def hexagon(radius):
    """🔩 Hexagon (w, u) of a nut."""
    return [[radius * math.cos(math.radians(60 * k)), radius * math.sin(math.radians(60 * k))] for k in range(6)]


def polygon(points):
    """📏 Area and centroid of a polygon."""
    area = cx = cy = 0.0
    for i, (x0, y0) in enumerate(points):
        x1, y1 = points[(i + 1) % len(points)]
        cross = x0 * y1 - x1 * y0
        area += cross
        cx += (x0 + x1) * cross
        cy += (y0 + y1) * cross
    area /= 2
    return abs(area), cx / (6 * area), cy / (6 * area)


def body_mass():
    """⚖️ Mass and centre of the concrete body: the gross wedge at 25 kN/m3 (walls, slab and retained fill lumped, EN 1991-1-1) plus the two kerbs."""
    area, cx, cy = polygon(WEDGE)
    parts = [(area * 2 * HALF * 2500, cx, cy)]
    section = KERB * KERB - CHAMFER * CHAMFER
    for length, x, y in ((RUN_LEN, 3.3, 0.2), (1.5, 5.85, 0.35), (RUN_LEN, 8.4, 0.5), (1.5, 10.95, 0.65)):
        parts.append((2 * section * length * 2500, x, y))
    kg = sum(p[0] for p in parts)
    return kg, sum(p[0] * p[1] for p in parts) / kg, sum(p[0] * p[2] for p in parts) / kg


def rail_length():
    """🧵 Centre-line length of one handrail with both bends and legs."""
    return 0.04 + math.pi * BEND / 2 + (0.3 - RAIL_R - BEND) + RUN_LEN + 1.5 + RUN_LEN + (0.3 - RAIL_R - BEND) + math.pi * BEND / 2 + 0.04


def handrail_mass():
    """🔧 Mass and centre (handrail frame) of both handrails: tube 42.4 x 2.6 at 2.55 kg/m, posts 48.3 x 3.2 at 3.56 kg/m, stems, plates, anchors."""
    post_y = sum(h(x) for x in POSTS) / 6 + KERB
    parts = [
        (2 * 2.55 * rail_length(), 4.65, 1.15),
        (12 * 3.56 * POST_TOP, 4.65, post_y + POST_TOP / 2),
        (12 * math.pi * STEM_R**2 * 0.06 * 7850, 4.65, post_y + POST_TOP + 0.03),
        (12 * PLATE[0] * PLATE[1] * PLATE[2] * 7850, 4.65, post_y),
        (24 * 0.03, 4.65, post_y),
    ]
    kg = sum(p[0] for p in parts)
    return kg, sum(p[0] * p[1] for p in parts) / kg, sum(p[0] * p[2] for p in parts) / kg, parts
