"""🧥 Authoritative author script of the staffage asset `used-coat-stand`: states every hand-chosen number of the used Viennese bentwood coat stand on its folded moving blanket and writes `🔣️.json` next to this file.

Run: `python3 author-coat-stand.py` (writes the candidate and prints the reference measurements).

@see ./author_geometry.py — centrelines, profiles and the geometric helpers
@see ../../brief.md — the brief this asset follows
"""
import json
import math
import os

from author_geometry import (BLANKET_SIDE, BLANKET_TOP, BLANKET_X, BLANKET_Z, BROKEN_PHI, FOOT, FOOT_PHI, FOOT_R, G, GRIP_LOWER_U, GRIP_UPPER_U, LABEL,
                             LOWER, LOWER_HANG, LOWER_PHI, LOWER_R, POLE_PROFILE, RING_H, RING_R, RING_T, STAND_BASE, STEEL_ARM, STEEL_ARM_R, STEEL_BALL,
                             STEEL_HANG, STEEL_PLATE, STUMP, UPPER, UPPER_HANG, UPPER_PHI, UPPER_R, catmull, circle_path, offset_band, path_length, pick,
                             polygon_area, polygon_perimeter, revolve_area_volume, rnd, rod_area_volume, self_intersects, to_local, tube_loft, world_x, world_z)

HERE = os.path.dirname(os.path.abspath(__file__))
SCALES = [50, 100, 200, 500]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
RHO_BEECH = 720.0
RHO_ZINC = 6700.0
BLANKET_KG = 1.2
CARRY_LIFT = 0.22
CARRY_TILT = -40.0
C45 = math.cos(math.radians(45))


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def d(value, digits):
    """🇩🇪 A number with a decimal comma for the German texts."""
    return f"{value:.{digits}f}".replace(".", ",")


POLE = {
    50: POLE_PROFILE,
    100: [(0.075, 0.0), (0.075, 0.012), (0.085, 0.021), (0.100, 0.025), (0.125, 0.0225), (0.300, 0.0235), (1.300, 0.0225), (1.497, 0.022), (1.503, 0.027),
          (1.547, 0.027), (1.553, 0.022), (1.652, 0.022), (1.660, 0.028), (1.720, 0.028), (1.728, 0.021), (1.744, 0.016), (1.758, 0.027), (1.776, 0.038),
          (1.800, 0.041), (1.822, 0.035), (1.840, 0.020), (1.850, 0.0)],
    200: [(0.075, 0.0), (0.075, 0.020), (0.100, 0.025), (0.130, 0.023), (1.300, 0.0225), (1.500, 0.027), (1.550, 0.027), (1.560, 0.022), (1.655, 0.028),
          (1.722, 0.028), (1.745, 0.017), (1.770, 0.036), (1.800, 0.041), (1.835, 0.025), (1.850, 0.0)],
    500: [(0.075, 0.0), (0.075, 0.024), (1.500, 0.024), (1.505, 0.028), (1.725, 0.028), (1.748, 0.024), (1.775, 0.040), (1.815, 0.040), (1.850, 0.0)],
}
FOOT_PTS = {50: catmull(FOOT, 3), 100: catmull(FOOT, 2), 200: pick(FOOT, [0, 2, 4, 6, 8, 10, 12, 13, 15, 17]), 500: pick(FOOT, [0, 3, 6, 9, 12, 15, 17])}
UPPER_PTS = {50: catmull(UPPER, 3), 100: catmull(UPPER, 2), 200: pick(UPPER, [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]), 500: pick(UPPER, [0, 2, 4, 6, 8, 10, 12, 14])}
LOWER_PTS = {50: catmull(LOWER, 3), 100: catmull(LOWER, 2), 200: pick(LOWER, [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13])}
RING_LOOP = {50: 20, 100: 14, 200: 10, 500: 8}
RING_TUBE = {50: RING_T, 100: RING_T, 200: RING_T, 500: RING_T}
TAPER = {50: True, 100: True, 200: True, 500: False}


def pole_elev(profile):
    """🏛️ Elevation outline (u, v) of the turned pole: right half up, left half down."""
    right = [[h, -r] for h, r in profile]
    left = [[h, r] for h, r in reversed(profile[1:-1])]
    return rnd(right + left)


def pole_front(profile):
    """🏛️ Front outline (lateral, height) of the turned pole."""
    right = [[r, h + STAND_BASE] for h, r in profile]
    left = [[-r, h + STAND_BASE] for h, r in reversed(profile[1:-1])]
    return rnd(right + left)


def foot_projection(points, phi, view):
    """🦶 The projected centreline of a rod at azimuth phi in a view: elevation (u, v) local, front (z, y) world."""
    if view == "elevation":
        return [(h, -world_x(r, phi)) for r, h in points]
    return [(world_z(r, phi), h + STAND_BASE) for r, h in points]


def radial_band(points, radius, phi, start_cap=True):
    """🛤️ The plan picture of a rod in a radial plane: a straight band from its innermost to its outermost radius."""
    rs = [r for r, _ in points]
    r0, r1 = min(rs), max(rs)
    a = math.radians(phi)
    cen = [(r0 * math.cos(a), r0 * math.sin(a)), (r1 * math.cos(a), r1 * math.sin(a))]
    return offset_band(cen, radius, cap_start=start_cap)


def check_ring(ring, label):
    """🔍 Refuses a self-crossing outline."""
    if self_intersects(ring):
        raise SystemExit(f"self-intersecting outline: {label}")
    return ring


def shape(id, bone, role, z, filled, tone, geometry, region=None, material=None, pen=None, regions=None):
    """✏️ One 2D shape of a representation."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, **geometry}
    if region is not None:
        out["region"] = region
    if regions is not None:
        out["regions"] = regions
    if material is not None:
        out["material"] = material
    if pen is not None:
        out["penMm"] = pen
    return out


def poly(points, closed=True):
    """🔷 A poly geometry."""
    return {"kind": "poly", "points": rnd(points), "closed": closed}


def steel_plate_corners():
    """🔩 The eight corners (r-direction, tangential, height) of the screwed-on replacement hook plate, in world rest coordinates (x, y, z)."""
    p = STEEL_PLATE
    a = math.radians(BROKEN_PHI)
    radial = (math.cos(a), math.sin(a))
    tang = (-math.sin(a), math.cos(a))
    out = []
    for dr in (p["r_in"], p["r_in"] + p["thick"]):
        for dt in (-p["wide"] / 2, p["wide"] / 2):
            for dh in (-p["len"] / 2, p["len"] / 2):
                x = radial[0] * dr + tang[0] * dt
                z = radial[1] * dr + tang[1] * dt
                out.append((x, p["h"] + dh, z))
    return out


def hull(points):
    """🫧 Convex hull (monotone chain) of 2D points, counter-clockwise."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) <= 2:
        return pts

    def cr(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cr(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cr(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def plate_view(view):
    """🔩 The outline of the replacement hook plate in a view."""
    c = steel_plate_corners()
    if view == "elevation":
        return hull([(y, -x) for x, y, z in c])
    if view == "front":
        return hull([(z, y + STAND_BASE) for x, y, z in c])
    return hull([(x, z) for x, y, z in c])


def ball_centre():
    """⚪ Centre (r, h) of the end ball of the replacement hook, one ball radius beyond the last arm point."""
    (r0, h0), (r1, h1) = STEEL_ARM[-2], STEEL_ARM[-1]
    ln = math.hypot(r1 - r0, h1 - h0)
    return (r1 + (r1 - r0) / ln * STEEL_BALL["r"] * 0.6, h1 + (h1 - h0) / ln * STEEL_BALL["r"] * 0.6)


PENS = {
    50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7},
    100: {"body": 0.35, "detail": 0.18, "texture": None, "glazing": None, "ground": 0.5},
    200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
    500: {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.35},
}
ROD_PEN = {50: 0.25, 100: 0.18, 200: 0.18, 500: None}
TONE = {"wood": 0.4, "worn": 0.62, "chrome": 0.85, "blanket": 0.66, "label": 0.95}


def rod_shapes(view, s, prefix, points, radius, phis, z, region, tip=True, near=None):
    """🥢 The bands of a family of rods at the given azimuths in one view; rods that project onto the same band are drawn once."""
    out = []
    seen = set()
    pen = ROD_PEN[s]
    for phi in phis:
        if view == "plan":
            ring = check_ring(radial_band(points, radius, phi), f"{prefix}-{phi}-plan")
            out.append(shape(f"{prefix}-{int(phi)}", "stand", "body", rnd(z + 0.02 * phis.index(phi), 3), True, TONE["wood"], poly(ring), region, "beech-stained", pen))
            continue
        key = round(world_x(1, phi), 3) if view == "elevation" else round(world_z(1, phi), 3)
        if abs(key) < 1e-6 or key in seen:
            continue
        seen.add(key)
        cen = foot_projection(points, phi, view)
        ring = check_ring(offset_band(cen, radius, cap_end=tip), f"{prefix}-{phi}-{view}")
        twins = [q for q in phis if abs((round(world_x(1, q), 3) if view == "elevation" else round(world_z(1, q), 3)) - key) < 1e-6]
        zz = z if near is None else (z + 1.5 if any(near(q, view) for q in twins) else z - 1.8)
        out.append(shape(f"{prefix}-{int(phi)}", "stand", "body", rnd(zz + 0.02 * phis.index(phi), 3), True, TONE["wood"], poly(ring), region, "beech-stained", pen))
    return out


def near_lower(phi, view):
    """👁️ Whether a lower hook lies on the near half of a view (elevation from the right, front from ahead)."""
    if view == "elevation":
        return world_z(1, phi) < 0
    return world_x(1, phi) > 0


def blanket_shapes(view, s):
    """🧺 The folded moving blanket in one view."""
    if view == "elevation":
        prof = BLANKET_SIDE if s in (50, 100) else [(0.0, 0.012), (0.012, 0.0), (0.688, 0.0), (0.700, 0.012), (0.700, 0.033), (0.688, 0.045), (0.012, 0.045), (0.0, 0.033)]
        return [shape("blanket", "blanket", "body", 1, True, TONE["blanket"], poly(prof), "blanket", "blanket-polyester")]
    if view == "front":
        g = {"kind": "rect", "x": -BLANKET_Z / 2, "y": 0.0, "w": BLANKET_Z, "h": BLANKET_TOP, "r": 0.008 if s < 500 else 0.0}
        return [shape("blanket", "blanket", "body", 1, True, TONE["blanket"], g, "blanket", "blanket-polyester")]
    g = {"kind": "rect", "x": -BLANKET_X / 2, "y": -BLANKET_Z / 2, "w": BLANKET_X, "h": BLANKET_Z, "r": 0.02 if s < 500 else 0.0}
    return [shape("blanket", "blanket", "body", 1, True, TONE["blanket"], g, "blanket", "blanket-polyester")]


def details(view, s):
    """🔎 Detail and texture strokes: collar grooves, the scuffed foot tips, worn hook tips, the glue line, the lot label and the quilting."""
    out = []
    if s not in (50, 100):
        return out
    for h, r in ((1.525, 0.027), (1.690, 0.028)):
        if view == "elevation":
            out.append(shape(f"groove-{int(h * 1000)}", "stand", "detail", 9, False, 0.3, poly([[h, -r], [h, r]], False), "pole", "beech-stained"))
        elif view == "front":
            out.append(shape(f"groove-{int(h * 1000)}", "stand", "detail", 9, False, 0.3, poly([[-r, h + STAND_BASE], [r, h + STAND_BASE]], False), "pole", "beech-stained"))
    if s != 50:
        return out
    if view == "elevation":
        for sign in (-1, 1):
            out.append(shape(f"scuff-{'front' if sign < 0 else 'rear'}", "stand", "texture", 9, False, TONE["worn"], poly([[0.004, sign * 0.190], [0.002, sign * 0.205], [0.004, sign * 0.218]], False), "feet", "beech-worn"))
        out.append(shape("glue-line", "stand", "detail", 9, False, 0.3, poly([[0.330, 0.1265], [0.300, 0.1395]], False), "feet", "beech-stained"))
        for phi in (0.0, 180.0):
            (r1, h1) = UPPER[-1]
            out.append(shape(f"worn-tip-{int(phi)}", "stand", "texture", 9, False, TONE["worn"], poly([[h1 - 0.004, -world_x(r1 - 0.006, phi)], [h1 + 0.008, -world_x(r1 - 0.003, phi)]], False), "hooks", "beech-worn"))
        out.append(shape("quilting-side", "blanket", "texture", 2, False, 0.5, poly([[0.03, 0.0225], [0.67, 0.0225]], False), "blanket", "blanket-polyester"))
    elif view == "front":
        lab = {"kind": "rect", "x": -LABEL["w"] / 2, "y": LABEL["h"] - LABEL["hgt"] / 2 + STAND_BASE, "w": LABEL["w"], "h": LABEL["hgt"], "r": 0.002}
        out.append(shape("lot-label", "stand", "detail", 9, True, TONE["label"], lab, "pole", "paper-label", 0.13))
        for sign in (-1, 1):
            out.append(shape(f"scuff-{'left' if sign > 0 else 'right'}", "stand", "texture", 9, False, TONE["worn"], poly([[sign * 0.190, STAND_BASE + 0.004], [sign * 0.205, STAND_BASE + 0.002], [sign * 0.218, STAND_BASE + 0.004]], False), "feet", "beech-worn"))
        out.append(shape("quilting-front", "blanket", "texture", 2, False, 0.5, poly([[-0.26, 0.0225], [0.26, 0.0225]], False), "blanket", "blanket-polyester"))
    else:
        for k in range(-3, 4):
            x0 = k * 0.09
            out.append(shape(f"quilting-{k + 3}", "blanket", "texture", 1.5, False, 0.5, poly([[x0 - 0.06, -0.26], [x0 + 0.06, 0.26]], False), "blanket", "blanket-polyester"))
    return out


def steel_shapes(view, s):
    """🪝 The screwed-on chromed replacement hook and the broken bentwood stump in one view."""
    out = []
    near = {"elevation": True, "front": False, "plan": True}[view]
    zs = 8 if near else 3
    if s in (50, 100):
        if view == "plan":
            stump = radial_band(STUMP, LOWER_R, BROKEN_PHI)
        else:
            stump = offset_band(foot_projection(STUMP, BROKEN_PHI, view), LOWER_R, cap_end=False)
        out.append(shape("stump", "stand", "body", zs - 0.5, True, TONE["worn"], poly(check_ring(stump, "stump")), "hooks", "beech-worn", ROD_PEN[s]))
    if s == 50:
        out.append(shape("steel-plate", "stand", "body", zs, True, TONE["chrome"], poly(plate_view(view)), "steel-hook", "chrome-zinc", 0.18))
    if s in (50, 100):
        arm_r = STEEL_ARM_R if s == 50 else 0.0065
        arm = [(0.021 if s == 100 else STEEL_ARM[0][0], STEEL_ARM[0][1])] + STEEL_ARM[1:]
        if view == "plan":
            ring = radial_band(arm, arm_r if s == 50 else 0.0062, BROKEN_PHI)
        else:
            ring = offset_band(foot_projection(arm, BROKEN_PHI, view), arm_r if s == 50 else 0.0062)
        out.append(shape("steel-arm", "stand", "body", zs + 0.2, True, TONE["chrome"], poly(check_ring(ring, "steel-arm")), "steel-hook", "chrome-zinc", 0.13 if s == 50 else 0.18))
    if s == 50:
        rb, hb = ball_centre()
        if view == "elevation":
            g = {"kind": "ellipse", "cx": rnd(hb), "cy": rnd(-world_x(rb, BROKEN_PHI)), "rx": STEEL_BALL["r"], "ry": STEEL_BALL["r"]}
        elif view == "front":
            g = {"kind": "ellipse", "cx": rnd(world_z(rb, BROKEN_PHI)), "cy": rnd(hb + STAND_BASE), "rx": STEEL_BALL["r"], "ry": STEEL_BALL["r"]}
        else:
            g = {"kind": "ellipse", "cx": rnd(world_x(rb, BROKEN_PHI)), "cy": rnd(world_z(rb, BROKEN_PHI)), "rx": STEEL_BALL["r"], "ry": STEEL_BALL["r"]}
        out.append(shape("steel-ball", "stand", "body", zs + 0.3, True, TONE["chrome"], g, "steel-hook", "chrome-zinc", 0.13))
    if s == 200:
        arm = [(0.020, 1.417), (0.090, 1.410)]
        if view == "plan":
            ring = radial_band(arm, 0.009, BROKEN_PHI)
        else:
            ring = offset_band(foot_projection(arm, BROKEN_PHI, view), 0.009)
        out.append(shape("steel-hook", "stand", "body", zs, True, TONE["chrome"], poly(check_ring(ring, "steel-200")), "steel-hook", "chrome-zinc", 0.18))
    return out


def geometry(s):
    """🖼️ The three views of one scale, every part drawn individually for that scale."""
    views = {}
    for view in ("elevation", "front", "plan"):
        out = []
        out += blanket_shapes(view, s)
        prof = POLE[s]
        pole_regions = ["pole", "steel-hook"] if s == 500 else None
        pole_region = None if s == 500 else "pole"
        if view == "elevation":
            out.append(shape("pole", "stand", "body", 4, True, TONE["wood"], poly(pole_elev(prof)), pole_region, "beech-stained", None, pole_regions))
        elif view == "front":
            out.append(shape("pole", "stand", "body", 4, True, TONE["wood"], poly(pole_front(prof)), pole_region, "beech-stained", None, pole_regions))
        else:
            rmax = max(r for _, r in prof)
            out.append(shape("pole", "stand", "body", 7, True, TONE["wood"], {"kind": "ellipse", "cx": 0.0, "cy": 0.0, "rx": rmax, "ry": rmax}, pole_region, "beech-stained", None, pole_regions))
        top = 0 if s < 500 else 4
        rt = RING_TUBE[s]
        out += rod_shapes(view, s, "foot", FOOT_PTS[s], FOOT_R, FOOT_PHI, (3 if view != "plan" else 2) + top, "feet", tip=True)
        out += rod_shapes(view, s, "hook-upper", UPPER_PTS[s], UPPER_R, UPPER_PHI, (3.5 if view != "plan" else 5) + top, "hooks", tip=True)
        if s in LOWER_PTS:
            out += rod_shapes(view, s, "hook-lower", LOWER_PTS[s], LOWER_R, LOWER_PHI, 5 if view != "plan" else 4, "hooks", tip=True, near=None if view == "plan" else near_lower)
        if view == "plan":
            out.append(shape("ring", "stand", "body", 3, True, TONE["wood"], {"kind": "path", "d": circle_path(0, 0, RING_R + rt) + " " + circle_path(0, 0, RING_R - rt)}, "ring", "beech-stained", ROD_PEN[s]))
        elif view == "elevation":
            out.append(shape("ring", "stand", "body", 5 + top, True, TONE["wood"], {"kind": "capsule", "x0": RING_H, "y0": -RING_R, "x1": RING_H, "y1": RING_R, "r0": rt, "r1": rt}, "ring", "beech-stained", ROD_PEN[s]))
        else:
            out.append(shape("ring", "stand", "body", 5 + top, True, TONE["wood"], {"kind": "capsule", "x0": -RING_R, "y0": RING_H + STAND_BASE, "x1": RING_R, "y1": RING_H + STAND_BASE, "r0": rt, "r1": rt}, "ring", "beech-stained", ROD_PEN[s]))
        if s < 500:
            out += steel_shapes(view, s)
        out += details(view, s)
        views[view] = out
    return views


def revolve(id, bone, material, tone, profile, **extra):
    """🏺 A revolve solid about the bone axis u."""
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": rnd([[h, r] for h, r in profile]), "axis": [1.0, 0.0, 0.0], "origin": [0.0, 0.0, 0.0], **extra}


def loft(id, sections, material, tone, **extra):
    """🥢 A loft solid on the stand bone."""
    return {"id": id, "kind": "loft", "bone": "stand", "material": material, "role": "body", "tone": tone, "sections": sections, "capStart": True, "capEnd": True, **extra}


def ring_profile(n):
    """⭕ The closed circular section loop of the umbrella ring, [axial, radial]."""
    t = RING_TUBE[{20: 50, 14: 100, 10: 200, 8: 500}[n]]
    pts = [[RING_H + t * math.cos(2 * math.pi * k / n), RING_R + t * math.sin(2 * math.pi * k / n)] for k in range(n)]
    return pts + [pts[0]]


def blanket_solid(s):
    """🧺 The folded moving blanket: its folded side profile pushed across its width."""
    if s in (50, 100):
        prof = BLANKET_SIDE if s == 50 else [(0.0, 0.016), (0.010, 0.002), (0.030, 0.0), (0.670, 0.0), (0.690, 0.002), (0.700, 0.016), (0.700, 0.030), (0.690, 0.043), (0.670, 0.045), (0.030, 0.045), (0.010, 0.043), (0.0, 0.030)]
        return {"id": "blanket", "kind": "extrude", "bone": "blanket", "material": "blanket-polyester", "role": "body", "tone": TONE["blanket"], "profile": rnd([list(p) for p in prof]), "plane": "xy", "from": -BLANKET_Z / 2, "to": BLANKET_Z / 2}
    return {"id": "blanket", "kind": "box", "bone": "blanket", "material": "blanket-polyester", "role": "body", "tone": TONE["blanket"], "size": [BLANKET_X, BLANKET_TOP, BLANKET_Z], "cornerRadius": 0.012 if s == 200 else 0.0, "transform": {"translation": [BLANKET_X / 2, BLANKET_TOP / 2, 0.0]}}


def body(s):
    """🧊 The handcrafted body list of one scale."""
    out = [revolve("pole", "stand", "beech-stained", TONE["wood"], POLE[s])]
    slender = {}
    for phi in FOOT_PHI:
        out.append(loft(f"foot-{int(phi)}", tube_loft(FOOT_PTS[s], FOOT_R, phi, taper_tip=TAPER[s], taper_start=TAPER[s]), "beech-stained", TONE["wood"], **slender))
    for phi in UPPER_PHI:
        out.append(loft(f"hook-upper-{int(phi)}", tube_loft(UPPER_PTS[s], UPPER_R, phi, taper_tip=TAPER[s]), "beech-stained", TONE["wood"], **slender))
    if s in LOWER_PTS:
        for phi in LOWER_PHI:
            out.append(loft(f"hook-lower-{int(phi)}", tube_loft(LOWER_PTS[s], LOWER_R, phi, taper_tip=TAPER[s]), "beech-stained", TONE["wood"]))
    if s in (50, 100):
        out.append(loft("stump", tube_loft(STUMP if s == 50 else [STUMP[0], STUMP[2]], LOWER_R, BROKEN_PHI, taper_tip=False), "beech-worn", TONE["worn"]))
    out.append({"id": "ring", "kind": "revolve", "bone": "stand", "material": "beech-stained", "role": "body", "tone": TONE["wood"], "profile": rnd(ring_profile(RING_LOOP[s])), "axis": [1.0, 0.0, 0.0], "origin": [0.0, 0.0, 0.0], **slender})
    if s == 50:
        p = STEEL_PLATE
        a = math.radians(BROKEN_PHI)
        rc = p["r_in"] + p["thick"] / 2
        _, v, w = to_local(rc, BROKEN_PHI, p["h"])
        half = math.radians(-45.0) / 2
        out.append({"id": "steel-plate", "kind": "box", "bone": "stand", "material": "chrome-zinc", "role": "body", "tone": TONE["chrome"], "size": [p["len"], p["thick"], p["wide"]], "cornerRadius": 0.002, "transform": {"rotation": rnd([math.sin(half), 0.0, 0.0, math.cos(half)], 6), "translation": rnd([p["h"], v, w])}})
        out.append(loft("steel-arm", tube_loft(STEEL_ARM, STEEL_ARM_R, BROKEN_PHI, taper_tip=False), "chrome-zinc", TONE["chrome"]))
        rb, hb = ball_centre()
        out.append({"id": "steel-ball", "kind": "ellipsoid", "bone": "stand", "material": "chrome-zinc", "role": "body", "tone": TONE["chrome"], "radii": [STEEL_BALL["r"]] * 3, "transform": {"translation": rnd(list(to_local(rb, BROKEN_PHI, hb)))}})
    elif s == 100:
        arm = [(0.021, STEEL_ARM[0][1])] + STEEL_ARM[1:]
        out.append(loft("steel-arm", tube_loft(arm, 0.0065, BROKEN_PHI, taper_tip=False), "chrome-zinc", TONE["chrome"]))
    elif s == 200:
        out.append({"id": "steel-hook", "kind": "capsule", "bone": "stand", "material": "chrome-zinc", "role": "body", "tone": TONE["chrome"], "p0": rnd(list(to_local(0.020, BROKEN_PHI, 1.417))), "p1": rnd(list(to_local(0.090, BROKEN_PHI, 1.410))), "r0": 0.009, "r1": 0.009})
    out.append(blanket_solid(s))
    return out


def region_of(solid_id):
    """🌡️ The thermal region of a body solid."""
    if solid_id == "pole":
        return "pole"
    if solid_id.startswith("foot"):
        return "feet"
    if solid_id.startswith("hook") or solid_id == "stump":
        return "hooks"
    if solid_id == "ring":
        return "ring"
    if solid_id.startswith("steel"):
        return "steel-hook"
    return "blanket"


def thermal(s, bodies):
    """🔥 The thermal list of one scale: every body surface is its region's surface; the replacement hook gets a thin stand-in at 1:500."""
    out = [{"id": f"th-{b['id']}", "of": b["id"], "region": region_of(b["id"])} for b in bodies]
    if s == 500:
        out.append({"id": "th-steel-hook", "kind": "capsule", "bone": "stand", "material": "chrome-zinc", "role": "body", "tone": TONE["chrome"], "p0": rnd(list(to_local(0.020, BROKEN_PHI, 1.417))), "p1": rnd(list(to_local(0.125, BROKEN_PHI, 1.410))), "r0": 0.0025, "r1": 0.0025, "slender": True, "region": "steel-hook"})
    return out


def ellipsoid(id, bone, r, at, material, part, member):
    """⚫ A compact joint or mass marker of the structure list."""
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.4, "radii": [r, r, r], "transform": {"translation": rnd(list(at))}, "member": member, "part": part}


def structure(s, bodies, com_u):
    """🏗️ The structure list of one scale: the pole as a slender member at its true Ø47 section, feet, ring and hooks as the members they are, joints at the screwed and dowelled connections, the masses at their centres and the blanket as the member that carries the stand into the floor when stored."""
    ids = {b["id"] for b in bodies}
    ends = {50: (0.095, 1.745), 100: (0.10, 1.74), 200: (0.11, 1.73), 500: (0.12, 1.72)}[s]
    out = [{"id": "m-pole", "kind": "capsule", "bone": "stand", "material": "beech-stained", "role": "body", "tone": 0.5, "p0": [ends[0], 0.0, 0.0], "p1": [ends[1], 0.0, 0.0], "r0": 0.0225, "r1": 0.0225, "slender": True, "member": "stand", "part": "member"}]
    for i in sorted(ids):
        if i.startswith("foot") or i.startswith("hook") or i in ("ring", "steel-arm", "steel-hook", "stump"):
            out.append({"id": f"m-{i}", "of": i, "member": "stand", "part": "member"})
    if s in (50, 100, 200):
        jr = {50: 0.016, 100: 0.016, 200: 0.016}[s]
        for phi in FOOT_PHI:
            out.append(ellipsoid(f"j-foot-{int(phi)}", "stand", jr, to_local(0.030, phi, 0.62), "beech-stained", "joint", "stand"))
    if s in (50, 100):
        for phi in FOOT_PHI:
            out.append(ellipsoid(f"j-ring-{int(phi)}", "stand", 0.012 if s == 50 else 0.015, to_local(RING_R - RING_T, phi, RING_H), "beech-stained", "joint", "stand"))
    if s == 50:
        out.append({"id": "j-steel-plate", "of": "steel-plate", "member": "stand", "part": "joint"})
        out.append(ellipsoid("j-collar-upper", "stand", 0.03, (1.69, 0.0, 0.0), "beech-stained", "joint", "stand"))
        out.append(ellipsoid("j-collar-lower", "stand", 0.028, (1.525, 0.0, 0.0), "beech-stained", "joint", "stand"))
    mr = FEATURE[s] / 2 if s > 50 else 0.02
    out.append(ellipsoid("mass-stand", "stand", max(mr, 0.02), (com_u, 0.0, 0.0), "beech-stained", "mass", "stand"))
    out.append({"id": "m-blanket", "of": "blanket", "member": "blanket", "part": "member"})
    out.append(ellipsoid("mass-blanket", "blanket", 0.0225 if s < 500 else 0.0225, (BLANKET_X / 2, BLANKET_TOP / 2, 0.0), "blanket-polyester", "mass", "blanket"))
    return out


def masses():
    """⚖️ Mass and centre of mass of every part of the stand from section x density, and the reference measurements of the 1:50 model."""
    parts = []
    pa, pv = revolve_area_volume(POLE_PROFILE)
    mh = 0.0
    for (h0, r0), (h1, r1) in zip(POLE_PROFILE, POLE_PROFILE[1:]):
        mh += math.pi * (h1 - h0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3 * (h0 + h1) / 2
    parts.append(("pole and finial", pv * RHO_BEECH, mh / pv, pa))

    def rod(name, pts, rad, count, rho=RHO_BEECH):
        a, v, ln = rod_area_volume(pts, rad)
        hc = sum((pts[i][1] + pts[i + 1][1]) / 2 * math.dist(pts[i], pts[i + 1]) for i in range(len(pts) - 1)) / ln
        parts.append((name, v * rho * count, hc, a * count))
    rod("four feet", FOOT_PTS[50], FOOT_R, 4)
    rod("four upper hooks", UPPER_PTS[50], UPPER_R, 4)
    rod("three lower hooks", LOWER_PTS[50], LOWER_R, 3)
    rod("broken stump", STUMP, LOWER_R, 1)
    parts.append(("umbrella ring", 2 * math.pi ** 2 * RING_R * RING_T ** 2 * RHO_BEECH, RING_H, 4 * math.pi ** 2 * RING_R * RING_T))
    p = STEEL_PLATE
    plv = p["len"] * p["thick"] * p["wide"]
    pla = 2 * (p["len"] * p["thick"] + p["len"] * p["wide"] + p["thick"] * p["wide"])
    aa, av, _ = rod_area_volume(STEEL_ARM, STEEL_ARM_R)
    bv = 4 / 3 * math.pi * STEEL_BALL["r"] ** 3
    ba = 4 * math.pi * STEEL_BALL["r"] ** 2
    parts.append(("replacement steel hook", (plv + av + bv) * RHO_ZINC, 1.415, pla + aa + ba))
    return parts


def main():
    """🧥 Builds and writes the asset document."""
    parts = masses()
    stand_kg = sum(p[1] for p in parts)
    com_u = sum(p[1] * p[2] for p in parts) / stand_kg
    total_kg = stand_kg + BLANKET_KG
    area = {
        "pole": parts[0][3],
        "feet": parts[1][3],
        "hooks": parts[2][3] + parts[3][3] + parts[4][3],
        "ring": parts[5][3],
        "steel-hook": parts[6][3],
        "blanket": 2 * polygon_area(BLANKET_SIDE) + polygon_perimeter(BLANKET_SIDE) * BLANKET_Z,
    }
    W = stand_kg * G
    edge = world_x(0.290, 45.0)
    hook_x = UPPER_HANG[0]
    low_x = world_x(LOWER_HANG[0], 45.0)
    tip_front_kg = 7.5
    tip_bag_kg = 3.0
    m_resist = W * edge + tip_bag_kg * G * (edge - low_x)
    m_vert = tip_front_kg * G * (hook_x - edge)
    h_hook = UPPER_HANG[1]
    h_crit = (m_resist - m_vert) / h_hook
    tug = 4.0
    safety = m_resist / (m_vert + tug * h_hook)
    in_use = [("coat-winter", "hook-front", 2.0, T("Winter coat on the front upper hook (2.0 kg)", "Wintermantel am vorderen oberen Haken (2,0 kg)")),
              ("jacket", "hook-left", 1.2, T("Jacket on the left upper hook (1.2 kg)", "Jacke am linken oberen Haken (1,2 kg)")),
              ("parka", "hook-rear", 2.4, T("Parka on the rear upper hook (2.4 kg)", "Parka am hinteren oberen Haken (2,4 kg)")),
              ("raincoat", "hook-right", 0.9, T("Raincoat on the right upper hook (0.9 kg)", "Regenmantel am rechten oberen Haken (0,9 kg)")),
              ("scarf", "hook-lower-front-left", 0.3, T("Scarf on the front-left lower hook (0.3 kg)", "Schal am vorderen linken unteren Haken (0,3 kg)")),
              ("bag", "hook-lower-rear-left", 1.5, T("Shoulder bag on the rear-left lower hook (1.5 kg)", "Umhängetasche am hinteren linken unteren Haken (1,5 kg)")),
              ("cap", "hook-lower-front-right", 0.2, T("Cap on the front-right lower hook (0.2 kg)", "Mütze am vorderen rechten unteren Haken (0,2 kg)")),
              ("leash", "hook-steel", 0.3, T("Dog leash on the replacement steel hook (0.3 kg)", "Hundeleine am Ersatzhaken aus Stahl (0,3 kg)"))]
    coats_kg = sum(x[2] for x in in_use)

    anchors = []
    for phi in FOOT_PHI:
        tag = {45.0: "fl", 135.0: "rl", 225.0: "rr", 315.0: "fr"}[phi]
        names = {"fl": ("front left", "vorn links"), "rl": ("rear left", "hinten links"), "rr": ("rear right", "hinten rechts"), "fr": ("front right", "vorn rechts")}[tag]
        u, v, w = to_local(0.290, phi, 0.0)
        anchors.append({"id": f"blanket-under-{tag}", "bone": "blanket", "x": rnd(world_x(0.290, phi) + BLANKET_X / 2), "y": 0.0, "z": rnd(world_z(0.290, phi)), "kind": "contact", "title": T(f"Blanket on the floor under the foot {names[0]}", f"Decke auf dem Boden unter dem Fuß {names[1]}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.08, "footprintWidthM": 0.06, "frictionCoefficient": 0.5})
    anchors.append({"id": "grip-lower", "bone": "hand-lower", "x": 0.0, "y": 0.0, "kind": "support", "title": T("Lower hand round the pole (holds and steers)", "Untere Hand um die Stange (hält und führt)"), "support": "pin"})
    anchors.append({"id": "grip-upper", "bone": "hand-upper", "x": 0.0, "y": 0.0, "kind": "support", "title": T("Upper hand under the pole (lifts)", "Obere Hand unter der Stange (hebt)"), "support": "roller", "normalDeg": 90})
    hook_anchor = {"hook-front": (UPPER_HANG, 0.0, T("Coat seat of the front upper hook", "Mantelauflage des vorderen oberen Hakens")),
                   "hook-left": (UPPER_HANG, 90.0, T("Coat seat of the left upper hook", "Mantelauflage des linken oberen Hakens")),
                   "hook-rear": (UPPER_HANG, 180.0, T("Coat seat of the rear upper hook", "Mantelauflage des hinteren oberen Hakens")),
                   "hook-right": (UPPER_HANG, 270.0, T("Coat seat of the right upper hook", "Mantelauflage des rechten oberen Hakens")),
                   "hook-lower-front-left": (LOWER_HANG, 45.0, T("Seat of the front-left lower hook", "Auflage des vorderen linken unteren Hakens")),
                   "hook-lower-rear-left": (LOWER_HANG, 135.0, T("Seat of the rear-left lower hook", "Auflage des hinteren linken unteren Hakens")),
                   "hook-lower-front-right": (LOWER_HANG, 315.0, T("Seat of the front-right lower hook", "Auflage des vorderen rechten unteren Hakens")),
                   "hook-steel": (STEEL_HANG, BROKEN_PHI, T("Seat of the screwed-on replacement hook", "Auflage des angeschraubten Ersatzhakens"))}
    for aid, ((r, h), phi, title) in hook_anchor.items():
        u, v, w = to_local(r, phi, h)
        anchors.append({"id": aid, "bone": "stand", "x": rnd(u), "y": rnd(v), "z": rnd(w), "kind": "load", "title": title})
    u, v, w = to_local(RING_R - RING_T, 0.0, RING_H)
    anchors.append({"id": "ring-front", "bone": "stand", "x": rnd(u), "y": rnd(v), "z": rnd(w), "kind": "load", "title": T("Inside of the umbrella ring, front", "Innenseite des Schirmrings, vorn")})

    regions = list(area)
    exposed = lambda over=None: {r: {"exposedFraction": (over or {}).get(r, 1.0), "sunlitFraction": 0.0} for r in regions}
    floor_share = round(BLANKET_X * BLANKET_Z / area["blanket"], 3)

    def binding(over=None, contact=None):
        out = exposed(over)
        for r, c in (contact or {}).items():
            out[r]["contact"] = c
        return {"regions": out, "emitters": {}}
    stored_th = binding(contact={"blanket": {"fraction": floor_share, "tempC": 17.0}})
    carried_th = binding(contact={"blanket": {"fraction": floor_share, "tempC": 17.0}, "pole": {"fraction": 0.04, "tempC": 32.0}})
    setup_th = binding(contact={"blanket": {"fraction": floor_share, "tempC": 19.0}})
    inuse_th = binding({"hooks": 0.55, "pole": 0.8, "steel-hook": 0.6}, {"blanket": {"fraction": floor_share, "tempC": 19.0}})
    tip_th = binding({"hooks": 0.75, "pole": 0.9}, {"blanket": {"fraction": floor_share, "tempC": 19.0}})

    def load(id, anchor, kg, title, kind="live"):
        return {"id": id, "kind": kind, "title": title, "anchor": anchor, "fxN": 0.0, "fyN": rnd(-kg * G, 3), "source": "authored-estimate", "reference": "semio-used-household"}
    umbrella = {"id": "umbrella", "kind": "live", "title": T("Umbrella leaning against the inside of the ring (0.86 N outward; its 0.45 kg stand on the floor)", "Schirm lehnt innen am Ring (0,86 N nach außen; seine 0,45 kg stehen auf dem Boden)"), "anchor": "ring-front", "fxN": 0.86, "fyN": 0.0, "source": "authored-estimate", "reference": "semio-used-household"}
    inuse_loads = [load(i, a, kg, t) for i, a, kg, t in in_use] + [umbrella]
    tip_loads = [load("coats-one-sided", "hook-front", tip_front_kg, T("Three heavy coats on the front upper hook only (7.5 kg)", "Drei schwere Mäntel nur am vorderen oberen Haken (7,5 kg)"), "imposed"),
                 load("bag-front", "hook-lower-front-left", tip_bag_kg, T("Full bag on the front-left lower hook (3.0 kg)", "Volle Tasche am vorderen linken unteren Haken (3,0 kg)"), "imposed"),
                 {"id": "tug", "kind": "imposed", "title": T(f"Forward tug at the front hook while a coat is taken off ({tug:.0f} N, {tug / h_crit * 100:.0f} % of the {h_crit:.1f} N that tips it)", f"Zug nach vorn am vorderen Haken beim Abnehmen eines Mantels ({d(tug, 0)} N, {d(tug / h_crit * 100, 0)} % der {d(h_crit, 1)} N, die ihn kippen)"), "anchor": "hook-front", "fxN": tug, "fyN": 0.0, "source": "authored-estimate", "reference": "en-14749-2016"}]
    under = ["blanket-under-fl", "blanket-under-fr", "blanket-under-rl", "blanket-under-rr"]
    grips = ["grip-lower", "grip-upper"]
    setup_bones = {}
    poses = [
        {"id": "stored", "title": T("Stored on a moving blanket", "Gelagert auf einer Möbeldecke"), "description": T("In the reuse depot: the emptied, wiped coat stand stands upright on a moving blanket folded nine-fold, its feet pressed 3 mm into it; a lot label is taped to the pole.", "Im Wiederverwendungslager: der geleerte, abgewischte Garderobenständer steht aufrecht auf einer neunfach gefalteten Möbeldecke, die Füße 3 mm eingedrückt; an der Stange klebt ein Losetikett."), "bones": {}, "ground": "authored", "contacts": under, "thermal": stored_th, "loads": []},
        {"id": "grip-key", "title": T("Gripped (key)", "Gegriffen (Schlüssel)"), "description": T("Both hands round the pole, the stand still on its blanket.", "Beide Hände um die Stange, der Ständer noch auf seiner Decke."), "key": True, "bones": {}, "ground": "authored", "contacts": under + grips, "thermal": carried_th, "loads": []},
        {"id": "lifted-key", "title": T("Lifted off the blanket (key)", "Von der Decke gehoben (Schlüssel)"), "description": T("Lifted straight up by 60 mm, the blanket left on the floor.", "Senkrecht um 60 mm angehoben, die Decke bleibt am Boden."), "key": True, "bones": {"lift": {"dy": 0.06}}, "ground": "authored", "contacts": grips, "thermal": carried_th, "loads": []},
        {"id": "carried", "title": T("Carried by one person", "Von einer Person getragen"), "description": T("Tilted 40 degrees head first and carried under the arm: the lower hand round the pole above the feet holds it, the upper hand lifts it at 1.20 m along the pole; the feet swing 0.10 m clear of the floor and the blanket stays behind.", "Um 40 Grad mit dem Kopf voran geneigt unter dem Arm getragen: die untere Hand hält die Stange über den Füßen, die obere hebt sie bei 1,20 m entlang der Stange; die Füße schweben 0,10 m über dem Boden und die Decke bleibt liegen."), "bones": {"lift": {"dy": CARRY_LIFT}, "stand": {"angleDeg": CARRY_TILT}}, "ground": "authored", "contacts": grips, "thermal": carried_th, "loads": []},
        {"id": "set-up", "title": T("Set up in a hallway", "Im Flur aufgestellt"), "description": T("Back in a home: the folded moving blanket stays under its feet as a mat that takes the drips of wet umbrellas; no coats yet.", "Wieder in einer Wohnung: die gefaltete Möbeldecke bleibt als Matte für das Tropfwasser nasser Schirme unter seinen Füßen; noch keine Mäntel."), "bones": setup_bones, "ground": "authored", "contacts": under, "thermal": setup_th, "loads": []},
        {"id": "in-use", "title": T("In use with coats", "In Gebrauch mit Mänteln"), "description": T(f"Four coats on the upper hooks, a scarf, a bag and a cap on the lower hooks, a leash on the replacement hook and an umbrella in the ring: {coats_kg:.1f} kg hung on the stand.", f"Vier Mäntel an den oberen Haken, Schal, Tasche und Mütze an den unteren Haken, eine Leine am Ersatzhaken und ein Schirm im Ring: {d(coats_kg, 1)} kg hängen am Ständer."), "bones": setup_bones, "ground": "authored", "contacts": under, "thermal": inuse_th, "loads": inuse_loads},
        {"id": "tipping-check", "title": T("Tipping check, loaded on one side", "Kippprüfung, einseitig belastet"), "description": T(f"Everything on the front: 7.5 kg of coats on the front upper hook, a 3 kg bag on a front lower hook and a {tug:.0f} N tug; the stand stays on its four feet on the blanket with a safety of {safety:.2f} against tipping over its front feet.", f"Alles vorn: 7,5 kg Mäntel am vorderen oberen Haken, 3 kg Tasche an einem vorderen unteren Haken und {d(tug, 0)} N Zug; der Ständer bleibt auf seinen vier Füßen auf der Decke mit einer Sicherheit von {d(safety, 2)} gegen Kippen über die vorderen Füße."), "bones": setup_bones, "ground": "authored", "contacts": under, "thermal": tip_th, "loads": tip_loads},
    ]
    clips = [
        {"id": "carry", "title": T("Lift off the blanket and carry", "Von der Decke heben und tragen"), "description": T("Both hands grip the pole, lift the stand 60 mm straight off its blanket, tilt it 40 degrees head first while raising it 0.22 m and carry it under the arm; the blanket stays on the floor.", "Beide Hände greifen die Stange, heben den Ständer 60 mm senkrecht von seiner Decke, neigen ihn um 40 Grad mit dem Kopf voran, heben ihn dabei auf 0,22 m und tragen ihn unter dem Arm; die Decke bleibt am Boden."),
         "durationS": 4.0, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0.0, "pose": "stored", "ease": "linear"}, {"t": 1.0, "pose": "grip-key", "ease": "in"}, {"t": 2.0, "pose": "lifted-key", "ease": "out"}, {"t": 3.5, "pose": "carried", "ease": "linear"}, {"t": 4.0, "pose": "carried", "ease": "linear"}],
         "contacts": [{"anchor": a, "fromS": 0.0, "toS": 1.2} for a in under] + [{"anchor": a, "fromS": 0.8, "toS": 4.0} for a in grips],
         "phases": [{"id": "grip", "fromS": 0.0, "toS": 1.0, "title": T("Grip", "Greifen")}, {"id": "lift", "fromS": 1.0, "toS": 2.0, "title": T("Lift", "Anheben")}, {"id": "tilt", "fromS": 2.0, "toS": 3.5, "title": T("Tilt and raise", "Neigen und heben")}, {"id": "carry", "fromS": 3.5, "toS": 4.0, "title": T("Carry", "Tragen")}],
         "events": [{"t": 0.8, "kind": "contact-on", "title": T("Hands take the pole", "Hände fassen die Stange"), "target": "grip-lower"}, {"t": 1.2, "kind": "contact-off", "title": T("Feet leave the blanket", "Füße verlassen die Decke"), "target": "blanket-under-fl"}, {"t": 3.5, "kind": "motion", "title": T("Carrying position reached", "Trageposition erreicht")}],
         "tracks": []},
        {"id": "hang-coats", "title": T("Coats hung up one by one", "Mäntel nacheinander aufgehängt"), "description": T("In the hallway the household hangs its coats, a scarf, a bag, a cap and a leash on the hooks and leans an umbrella into the ring; the loads grow hook by hook and the coats cover the hooks.", "Im Flur hängt der Haushalt Mäntel, Schal, Tasche, Mütze und Leine an die Haken und stellt einen Schirm in den Ring; die Lasten wachsen Haken für Haken und die Mäntel bedecken die Haken."),
         "durationS": 10.0, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0.0, "pose": "set-up", "ease": "linear"}, {"t": 10.0, "pose": "in-use", "ease": "linear"}],
         "contacts": [{"anchor": a, "fromS": 0.0, "toS": 10.0} for a in under],
         "phases": [{"id": "hang", "fromS": 0.0, "toS": 10.0, "title": T("Hang up", "Aufhängen")}],
         "events": [],
         "tracks": []},
    ]
    times = [0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5]
    for (lid, *_), t0 in zip(in_use + [("umbrella",)], times):
        clips[1]["tracks"].append({"target": f"load:{lid}", "ease": "smooth", "keys": [{"t": 0.0, "v": 0.0}, {"t": t0, "v": 0.0}, {"t": t0 + 0.6, "v": 1.0}, {"t": 10.0, "v": 1.0}]})
        clips[1]["events"].append({"t": rnd(t0 + 0.6, 2), "kind": "load-on", "title": T(f"{lid.replace('-', ' ').capitalize()} hung up", f"{ {'coat-winter': 'Wintermantel', 'jacket': 'Jacke', 'parka': 'Parka', 'raincoat': 'Regenmantel', 'scarf': 'Schal', 'bag': 'Tasche', 'cap': 'Mütze', 'leash': 'Leine', 'umbrella': 'Schirm'}[lid]} aufgehängt"), "target": lid})
    clips[1]["tracks"].append({"target": "region:hooks:exposedFraction", "ease": "smooth", "keys": [{"t": 0.0, "v": 1.0}, {"t": 0.5, "v": 1.0}, {"t": 4.1, "v": 0.6}, {"t": 10.0, "v": 0.55}]})

    vols = {}
    reps = {}
    for s in SCALES:
        b = body(s)
        vols[str(s)] = {"status": "authored", "denominator": s, "body": b, "thermal": thermal(s, b), "structure": structure(s, b, com_u), "decisions": VOLUME_DECISIONS[s]}
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": {50: 20, 100: 75, 200: 150, 500: 350}[s], "maxDenominator": {50: 75, 100: 150, 200: 350, 500: 1000}[s]},
                        "pens": PENS[s], "hatches": [], "geometry": geometry(s), "decisions": REP_DECISIONS[s]}

    wood_layer = lambda rid, t: [{"id": f"{rid}-wood", "title": T(f"Beech {t * 1000:.1f} mm (volume over surface)", f"Buche {d(t * 1000, 1)} mm (Volumen durch Oberfläche)"), "kind": "timber", "material": "beech-stained", "thicknessM": rnd(t), "resistanceM2KW": rnd(t / 0.18), "heatCapacityJM2K": round(RHO_BEECH * 1600 * t), "source": "literature", "reference": "iso-10456-2007"}]
    vol_over_area = {"pole": parts[0][1] / RHO_BEECH / area["pole"], "feet": parts[1][1] / RHO_BEECH / area["feet"], "hooks": (parts[2][1] + parts[3][1] + parts[4][1]) / RHO_BEECH / area["hooks"], "ring": parts[5][1] / RHO_BEECH / area["ring"]}
    def wood_region(rid, title, cl, geom, svf, extra=None):
        t = vol_over_area[rid]
        r = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": "beech-stained", "areaM2": rnd(area[rid], 4), "characteristicLengthM": cl, "geometry": geom, "skyViewFactor": svf, "layers": wood_layer(rid, t), "heatCapacityJM2K": round(RHO_BEECH * 1600 * t), "source": "authored-estimate", "reference": "iso-10456-2007"}
        r.update(extra or {})
        return r
    steel_t = parts[6][1] / RHO_ZINC / area["steel-hook"]
    blanket_t = BLANKET_TOP
    thermal_model = {
        "environment": {"airTempC": 18.0, "meanRadiantTempC": 18.0, "relativeHumidity": 0.5, "windMS": 0.05, "windDirectionDeg": 0.0, "solarWM2": 0.0, "diffuseFraction": 1.0, "pressurePa": 101325.0},
        "passengers": [], "operating": [],
        "regions": [
            wood_region("pole", T("Turned pole, collars and finial (stained beech, lacquer faded)", "Gedrechselte Stange, Bunde und Knauf (gebeizte Buche, Lack verblasst)"), 0.047, "cylinder", 0.5, {"ratedPowerW": 0.0, "ratedPowerBasis": "passive"}),
            wood_region("feet", T("Four bentwood feet (tips scuffed to bare wood)", "Vier Bugholzfüße (Spitzen bis aufs Holz abgestoßen)"), 0.026, "cylinder", 0.4),
            wood_region("hooks", T("Seven bentwood hooks and the broken stump", "Sieben Bugholzhaken und der abgebrochene Stumpf"), 0.022, "cylinder", 0.5),
            wood_region("ring", T("Umbrella ring", "Schirmring"), 0.018, "cylinder", 0.4),
            {"id": "steel-hook", "title": T("Screwed-on chromed zinc replacement hook", "Angeschraubter verchromter Zinkdruckguss-Ersatzhaken"), "kind": "surface", "model": "passive", "material": "chrome-zinc", "areaM2": rnd(area["steel-hook"], 5), "characteristicLengthM": 0.009, "geometry": "cylinder", "skyViewFactor": 0.4,
             "layers": [{"id": "steel-hook-metal", "title": T("Zinc die casting", "Zinkdruckguss"), "kind": "metal", "material": "chrome-zinc", "thicknessM": rnd(steel_t), "resistanceM2KW": rnd(steel_t / 110, 8), "heatCapacityJM2K": round(RHO_ZINC * 420 * steel_t), "source": "literature", "reference": "incropera-2018"}],
             "heatCapacityJM2K": round(RHO_ZINC * 420 * steel_t), "source": "authored-estimate", "reference": "incropera-2018"},
            {"id": "blanket", "title": T("Folded moving blanket", "Gefaltete Möbeldecke"), "kind": "surface", "model": "passive", "material": "blanket-polyester", "areaM2": rnd(area["blanket"], 4), "characteristicLengthM": 0.62, "geometry": "horizontal-plate-up", "skyViewFactor": 0.6,
             "layers": [{"id": "blanket-pad", "title": T("Nine layers of quilted polyester pad, 45 mm", "Neun Lagen gesteppte Polyesterwatte, 45 mm"), "kind": "insulation", "material": "blanket-polyester", "thicknessM": blanket_t, "resistanceM2KW": rnd(blanket_t / 0.045), "heatCapacityJM2K": round(68 * 1300 * blanket_t), "source": "authored-estimate", "reference": "iso-10456-2007"}],
             "heatCapacityJM2K": round(68 * 1300 * blanket_t), "source": "authored-estimate", "reference": "iso-10456-2007"},
        ],
        "emitters": [],
        "assumptions": [
            T("All parts are inert and passive: radiation, convection and contact conduction with heat capacity, no internal heat; the stand is furniture without any electrical part (0 W, stated on the pole region).", "Alle Teile sind träge und passiv: Strahlung, Konvektion und Kontaktleitung mit Wärmekapazität, keine innere Wärme; der Ständer ist ein Möbel ohne elektrisches Teil (0 W, an der Region der Stange angegeben)."),
            T("Aged surfaces: the walnut-stained nitrocellulose lacquer of the beech is matt and faded, so the emissivity is 0.90 and the solar absorptance 0.70 instead of about 0.80 for a fresh dark stain; the scuffed bare beech of the foot and hook tips reads 0.92 and 0.55 (authored estimates after Incropera Table A.11).", "Gealterte Oberflächen: der nussbaumgebeizte Nitrolack der Buche ist matt und verblasst, Emissionsgrad daher 0,90 und solare Absorption 0,70 statt etwa 0,80 bei frischer dunkler Beize; das abgestoßene blanke Buchenholz der Fuß- und Hakenspitzen 0,92 und 0,55 (Schätzwerte nach Incropera Tabelle A.11)."),
            T("Each wooden region has one timber layer as thick as its volume over its surface (solid round sections), with beech 720 kg/m3, 0.18 W/(m K) and 1600 J/(kg K) of ISO 10456; the chromed zinc hook (0.07, 110 W/(m K)) and the folded polyester blanket (0.045 W/(m K), 68 kg/m3 bulk) likewise.", "Jede Holzregion hat eine Holzschicht so dick wie ihr Volumen durch ihre Oberfläche (volle Rundquerschnitte), mit Buche 720 kg/m3, 0,18 W/(m K) und 1600 J/(kg K) nach ISO 10456; der verchromte Zinkhaken (0,07, 110 W/(m K)) und die gefaltete Polyesterdecke (0,045 W/(m K), 68 kg/m3 Schüttdichte) ebenso."),
            T("Indoors (depot and hallway at 18 degC), no sun; the blanket lies with its underside on the floor (contact share 0.45 at 17 to 19 degC), the hands of the carrier warm 4 % of the pole to 32 degC, hung coats cover 45 % of the hooks.", "Innenraum (Lager und Flur bei 18 degC), keine Sonne; die Decke liegt mit ihrer Unterseite auf dem Boden (Kontaktanteil 0,45 bei 17 bis 19 degC), die Hände der tragenden Person wärmen 4 % der Stange auf 32 degC, aufgehängte Mäntel bedecken 45 % der Haken."),
        ],
    }
    stand_section = {"areaM2": rnd(math.pi * 0.047 ** 2 / 4, 7), "secondMomentM4": rnd(math.pi * 0.047 ** 4 / 64, 10), "extremeFibreM": 0.0235, "material": "beech-stained", "title": T("Turned beech pole Ø47 mm", "Gedrechselte Buchenstange Ø47 mm")}
    rig = {"kind": "mechanical", "root": "ground", "bones": [
        {"id": "ground", "parent": None, "title": T("Floor marker under the pole axis", "Bodenmarke unter der Stangenachse"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "blanket", "parent": "ground", "title": T("Folded moving blanket (stays on the floor when the stand is carried; a drip mat under it when set up)", "Gefaltete Möbeldecke (bleibt beim Tragen am Boden liegen; aufgestellt eine Tropfmatte darunter)"), "x": -BLANKET_X / 2, "y": 0, "angleDeg": 0, "lengthM": BLANKET_X, "joint": {"type": "fixed"}, "mass": {"kg": BLANKET_KG, "comX": BLANKET_X / 2, "comY": BLANKET_TOP / 2, "distribution": "point", "source": "authored-estimate", "reference": "semio-used-household"}},
        {"id": "lift", "parent": "ground", "title": T("Lift of the stand from the blanket top up into the carrying height", "Hub des Ständers von der Deckenoberseite hinauf in die Tragehöhe"), "x": 0, "y": STAND_BASE, "angleDeg": 90, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0.0, "maxM": 0.40}},
        {"id": "stand", "parent": "lift", "title": T("Coat stand: turned pole with feet, ring and hooks (tilts about the foot of its axis when carried)", "Garderobenständer: gedrechselte Stange mit Füßen, Ring und Haken (neigt sich beim Tragen um den Fuß seiner Achse)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 1.85, "joint": {"type": "revolute", "minDeg": -60, "maxDeg": 15}, "mass": {"kg": rnd(stand_kg, 3), "comX": rnd(com_u, 4), "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "wagenfuehr-holzatlas"}, "section": stand_section},
        {"id": "hand-lower", "parent": "stand", "title": T("Lower hand of the carrier round the pole", "Untere Hand der tragenden Person um die Stange"), "x": GRIP_LOWER_U, "y": 0, "angleDeg": -90, "lengthM": 0.08, "joint": {"type": "fixed"}},
        {"id": "hand-upper", "parent": "stand", "title": T("Upper hand of the carrier under the pole", "Obere Hand der tragenden Person unter der Stange"), "x": GRIP_UPPER_U, "y": 0, "angleDeg": -90, "lengthM": 0.08, "joint": {"type": "fixed"}},
    ]}
    structure_model = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
        T(f"Stored: the stand ({stand_kg:.2f} kg) stands with its four foot tips on the blanket ({BLANKET_KG:.1f} kg), which carries it into the floor through four contacts under the feet on the diagonals at 0.29 m from the axis (friction 0.5 of the woven cover on the floor); set up in the hallway the blanket stays under the feet as a drip mat, so the same load path holds in use and in the tipping check.", f"Gelagert: der Ständer ({d(stand_kg, 2)} kg) steht mit seinen vier Fußspitzen auf der Decke ({d(BLANKET_KG, 1)} kg), die ihn über vier Kontakte unter den Füßen auf den Diagonalen 0,29 m von der Achse in den Boden trägt (Reibung 0,5 der Gewebehülle auf dem Boden); im Flur aufgestellt bleibt die Decke als Tropfmatte unter den Füßen, derselbe Lastpfad gilt daher in Gebrauch und bei der Kippprüfung."),
        T(f"Carried: the lower hand is a pin round the pole at {GRIP_LOWER_U:.2f} m, the upper hand a vertical roller under it at {GRIP_UPPER_U:.2f} m; with the stand tilted 40 degrees they carry its weight and balance the overhang of its {com_u:.2f} m high centre of mass. The structural model is one rigid body, so in the carried pose the 1.2 kg blanket that stays on the floor is carried by the hands as well (11.8 N of the grip reactions; stated plainly, not hidden).", f"Getragen: die untere Hand ist ein Gelenklager um die Stange bei {d(GRIP_LOWER_U, 2)} m, die obere ein senkrechtes Rollenlager darunter bei {d(GRIP_UPPER_U, 2)} m; bei 40 Grad Neigung tragen sie das Gewicht und halten den Überhang des {d(com_u, 2)} m hohen Schwerpunkts. Das Tragwerksmodell ist ein starrer Körper, daher tragen in der Tragepose die Hände auch die 1,2 kg der am Boden liegenden Decke (11,8 N der Griffreaktionen; offen angegeben)."),
        T("Members: the turned pole Ø47 mm (A 1.73e-3 m2, I 2.40e-7 m4) of steam-bent beech, strength class D35 of EN 338 assigned to beech by EN 1912 (f_m,k 35 MPa, E0,mean 12 GPa); the feet Ø26, upper hooks Ø22, lower hooks Ø18 and the ring Ø18 are solid round bentwood rods screwed and dowelled into the pole; the replacement hook is a die-cast zinc hook on two wood screws. Masses are section x density: beech 720 kg/m3, zinc 6700 kg/m3.", "Stäbe: die gedrechselte Stange Ø47 mm (A 1,73e-3 m2, I 2,40e-7 m4) aus dampfgebogener Buche, Festigkeitsklasse D35 nach EN 338, der Buche durch EN 1912 zugeordnet (f_m,k 35 MPa, E0,mean 12 GPa); Füße Ø26, obere Haken Ø22, untere Haken Ø18 und der Ring Ø18 sind volle Bugholzstäbe, in die Stange geschraubt und gedübelt; der Ersatzhaken ist ein Zinkdruckgusshaken an zwei Holzschrauben. Massen aus Querschnitt mal Dichte: Buche 720 kg/m3, Zink 6700 kg/m3."),
        T(f"Tipping check after the principle of the overturning tests of EN 14749 (most unfavourable one-sided loading): the overturning edge through the two front feet lies {edge:.3f} m ahead of the axis, the coat seat of the front upper hook {hook_x:.3f} m ahead at {h_hook:.2f} m height. Resisting moment {m_resist:.2f} N m (stand weight {W:.1f} N on {edge:.3f} m plus the 3 kg bag inside the edge), overturning moment of 7.5 kg of coats {m_vert:.2f} N m: a forward pull of {h_crit:.1f} N at the hook tips it; the pose applies {tug:.0f} N, safety {safety:.2f}. Bentwood coat stands are notoriously easy to tip; the in-use loads on all sides stay far from it.", f"Kippprüfung nach dem Grundsatz der Kippversuche der EN 14749 (ungünstigste einseitige Belastung): die Kippkante durch die beiden vorderen Füße liegt {d(edge, 3)} m vor der Achse, die Mantelauflage des vorderen oberen Hakens {d(hook_x, 3)} m davor in {d(h_hook, 2)} m Höhe. Standmoment {d(m_resist, 2)} N m (Eigengewicht {d(W, 1)} N auf {d(edge, 3)} m und die 3-kg-Tasche innerhalb der Kante), Kippmoment der 7,5 kg Mäntel {d(m_vert, 2)} N m: ein Zug von {d(h_crit, 1)} N nach vorn am Haken kippt ihn; die Pose setzt {d(tug, 0)} N an, Sicherheit {d(safety, 2)}. Bugholzständer kippen bekanntlich leicht; die allseitigen Lasten in Gebrauch bleiben weit davon entfernt."),
    ], "clinical": False}
    materials = [
        {"id": "beech-stained", "title": T("Steam-bent beech, walnut stain, faded nitrocellulose lacquer", "Dampfgebogene Buche, Nussbaumbeize, verblasster Nitrolack"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": RHO_BEECH, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 12e9, "strengthPa": 35e6, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "beech-worn", "title": T("Bare beech worn through the lacquer", "Blanke Buche, durch den Lack abgenutzt"), "emissivity": 0.92, "solarAbsorptivity": 0.55, "densityKgM3": RHO_BEECH, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "chrome-zinc", "title": T("Chromed zinc die casting", "Verchromter Zinkdruckguss"), "emissivity": 0.07, "solarAbsorptivity": 0.4, "densityKgM3": RHO_ZINC, "conductivityWmK": 110, "specificHeatJKgK": 420, "elasticModulusPa": 96e9, "strengthPa": 280e6, "source": "literature", "reference": "incropera-2018"},
        {"id": "blanket-polyester", "title": T("Moving blanket: quilted polyester pad in a woven cover", "Möbeldecke: gesteppte Polyesterwatte in Gewebehülle"), "emissivity": 0.88, "solarAbsorptivity": 0.75, "densityKgM3": 68, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "paper-label", "title": T("Paper lot label on masking tape", "Papier-Losetikett auf Kreppband"), "emissivity": 0.92, "solarAbsorptivity": 0.25, "densityKgM3": 800, "conductivityWmK": 0.12, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
    ]
    references = [
        {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values (hardwood 700 kg/m3: 0.18 W/(m K), 1600 J/(kg K); polyester and mineral fibre insulation 0.035-0.045 W/(m K))."},
        {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (hardwood class D35: characteristic bending strength 35 N/mm2, mean modulus of elasticity parallel to grain 12 kN/mm2)."},
        {"id": "en-1912", "citation": "EN 1912:2012 Structural timber - Strength classes - Assignment of visual grades and species (beech, Fagus sylvatica, assigned to hardwood D classes)."},
        {"id": "wagenfuehr-holzatlas", "citation": "Wagenführ, R. (2007). Holzatlas, 6th ed., Fachbuchverlag Leipzig im Carl Hanser Verlag (Rotbuche/European beech: density at 12 % moisture about 0.69-0.72 g/cm3; steam bending)."},
        {"id": "incropera-2018", "citation": "Bergman, T. L., Lavine, A. S., Incropera, F. P., DeWitt, D. P. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11 total emissivities of wood, lacquer, polished chromium; Table A.1 properties of zinc)."},
        {"id": "en-14749-2016", "citation": "EN 14749:2016 Domestic and kitchen storage units and worktops - Safety requirements and test methods (stability: overturning of free-standing units under the most unfavourable loading; used here as the principle of the tipping check, not as a test result)."},
        {"id": "din-33402-2", "citation": "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (stature and reach of adults; upper hooks at 1.47-1.71 m and the finial at 1.85 m stay within the reach of the 5th percentile woman)."},
        {"id": "semio-used-household", "citation": "Semio staffage content, used household objects wave 2026-11-22: dimensions, masses and loads authored from catalogue-typical second-hand Viennese bentwood coat stands (8 hooks in two tiers, 1.85-1.95 m, hook span 0.60-0.70 m, 5-8 kg), moving blankets (1.5 x 1.8 m, 1-2 kg) and household clothing masses; tagged authored-estimate, nothing is measured."},
    ]
    summary = T(f"A used Viennese bentwood coat stand of 1.85 m from a flat clearance: a turned beech pole Ø47 mm with a finial, four steam-bent feet on a 0.58 m footprint, an umbrella ring Ø0.40 m at 0.40 m, four large upper hooks spanning 0.60 m and three small lower hooks; the fourth lower hook broke off and a chromed zinc hook is screwed on below its stump, the foot tips are scuffed to bare wood and the walnut lacquer is faded. It is stored on a folded moving blanket, carried tilted under the arm, set up in a hallway with {coats_kg:.1f} kg of coats and checked for tipping when loaded on one side.",
                f"Ein gebrauchter Wiener Bugholz-Garderobenständer von 1,85 m aus einer Haushaltsauflösung: eine gedrechselte Buchenstange Ø47 mm mit Knauf, vier dampfgebogene Füße auf 0,58 m Aufstandsfläche, ein Schirmring Ø0,40 m auf 0,40 m, vier große obere Haken über 0,60 m und drei kleine untere Haken; der vierte untere Haken ist abgebrochen und unter seinem Stumpf ein verchromter Zinkhaken angeschraubt, die Fußspitzen sind bis aufs Holz abgestoßen und der Nussbaumlack ist verblasst. Er lagert auf einer gefalteten Möbeldecke, wird geneigt unter dem Arm getragen, im Flur mit {d(coats_kg, 1)} kg Mänteln aufgestellt und einseitig belastet auf Kippen geprüft.")
    asset = {
        "id": "used-coat-stand", "category": "furniture", "title": T("Used Coat Stand", "Gebrauchter Garderobenständer"), "summary": summary,
        "tags": ["used", "household", "second-hand", "hallway", "coat-stand", "bentwood", "beech", "turned", "moving-blanket", "interior"],
        "provenance": {"author": "Semio staffage content (used household objects wave, author 17)", "method": "handcrafted", "created": "2026-11-22", "references": references,
                       "notes": T(f"Modelled to a second-hand Viennese bentwood coat stand of the classic eight-hook type: stand 1.850 m from the underside of the feet, upper hook span 0.600 m, foot tips on a 0.58 m circle, total depth over the blanket 0.70 m. Stand {stand_kg:.2f} kg from section x density (pole {parts[0][1]:.2f} kg, feet {parts[1][1]:.2f} kg, hooks {parts[2][1] + parts[3][1] + parts[4][1]:.2f} kg, ring {parts[5][1]:.2f} kg, replacement hook {parts[6][1]:.3f} kg), blanket {BLANKET_KG:.1f} kg.", f"Nach einem gebrauchten Wiener Bugholz-Garderobenständer der klassischen Bauart mit acht Haken modelliert: Ständer 1,850 m ab Fußunterseite, Spannweite der oberen Haken 0,600 m, Fußspitzen auf einem Kreis von 0,58 m, Gesamttiefe über die Decke 0,70 m. Ständer {d(stand_kg, 2)} kg aus Querschnitt mal Dichte (Stange {d(parts[0][1], 2)} kg, Füße {d(parts[1][1], 2)} kg, Haken {d(parts[2][1] + parts[3][1] + parts[4][1], 2)} kg, Ring {d(parts[5][1], 2)} kg, Ersatzhaken {d(parts[6][1], 3)} kg), Decke {d(BLANKET_KG, 1)} kg.")},
        "dimensions": {"heightM": rnd(STAND_BASE + 1.85, 3), "widthM": BLANKET_X, "depthM": rnd(2 * (max(r for r, _ in UPPER) + UPPER_R), 3), "massKg": rnd(total_kg, 3)},
        "materials": materials, "rig": rig, "anchors": anchors, "representations": reps, "symbols": [], "poses": poses, "clips": clips,
        "thermal": thermal_model, "structure": structure_model, "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print(f"[DEBUG] stand {stand_kg:.3f} kg, com {com_u:.3f} m, total {total_kg:.3f} kg")
    for p in parts:
        print(f"[DEBUG] part {p[0]}: {p[1]:.3f} kg at h {p[2]:.3f}, area {p[3]:.4f} m2")
    print(f"[DEBUG] areas {json.dumps({k: round(v, 4) for k, v in area.items()})}")
    print(f"[DEBUG] tipping: W {W:.1f} N edge {edge:.3f} resist {m_resist:.2f} vert {m_vert:.2f} Hcrit {h_crit:.2f} N safety {safety:.2f}")
    print(f"[DEBUG] slender: foot {path_length(FOOT_PTS[50]) / 0.026:.1f} upper {path_length(UPPER_PTS[50]) / 0.022:.1f} lower {path_length(LOWER_PTS[50]) / 0.018:.1f}")


from author_decisions import REP_DECISIONS, VOLUME_DECISIONS

if __name__ == "__main__":
    main()
