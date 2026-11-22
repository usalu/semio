#!/usr/bin/env python3
"""🪑 Writes the asset document of the used backrest park bench (`🔣️.json` next to this file) from hand-chosen numbers.

The bench stands with its length along x (slat ends at 0 and 1.80 m), the sitters face -z (the side elevation shows the bench front), the
one-piece cast end frames stand at x = 0.12 and 1.68 m on two concrete point footings. Every part is authored in world metres with s = -z
(towards the sitter's front); the 2D drawings of a scale are the projections of the parts of that scale, the 3D lists are the same parts as
solids, so drawing and model agree by construction. Each scale has its own parts (`parts(scale)`).

@see ../../brief.md — the brief of the wave
@see ../../reports/used-backrest-park-bench.md — reference table, measurements and decisions
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the contract
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")


def T(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🔢 Rounds a coordinate to 0.01 mm."""
    return round(v + 0.0, 5)


K5 = math.tan(math.radians(5.0))
T15 = math.tan(math.radians(15.0))
C15, S15 = math.cos(math.radians(15.0)), math.sin(math.radians(15.0))
C5, S5 = math.cos(math.radians(5.0)), math.sin(math.radians(5.0))

FRAME_X = (0.12, 1.68)
LENGTH = 1.80
Y_TOP_FOOTING = 0.025
FOOTING = {"s0": -0.3625, "s1": 0.315, "y0": -0.40, "y1": Y_TOP_FOOTING, "half": 0.11}
FOOT_S = {"front": 0.2225, "rear": -0.27}
RAIL_FRONT = (0.345, 0.423)
POST_BASE = (-0.175, 0.376)
POST_TOP_Y = 0.835
DOWEL_OUT = 0.020
ARM_RIB_AREA = 0.050 * 0.008 + 2 * 0.032 * 0.008
DENSITY = {"hardwood-greyed": 900.0, "robinia-fresh": 740.0, "cast-iron-painted": 7200.0, "cast-iron-rusted": 7200.0, "concrete-c20-25": 2400.0, "steel-galvanised": 7850.0}

FRAME_PARAMS = {
    50: {"yb": 0.045, "fl": (0.200, 0.245), "ap": (0.305, 0.345), "post": 0.045, "leg": 0.045, "rail": 0.040, "arm": (0.625, 0.665), "arm_merged": False, "stretcher": (0.140, 0.175), "t": 0.022, "arm_w": 0.050},
    100: {"yb": 0.045, "fl": (0.200, 0.245), "ap": (0.305, 0.345), "post": 0.045, "leg": 0.045, "rail": 0.040, "arm": (0.625, 0.665), "arm_merged": False, "stretcher": (0.140, 0.175), "t": 0.022, "arm_w": 0.050},
    200: {"yb": 0.055, "fl": (0.200, 0.245), "ap": (0.305, 0.345), "post": 0.045, "leg": 0.045, "rail": 0.040, "arm": (0.625, 0.665), "arm_merged": False, "stretcher": (0.140, 0.175), "t": 0.030, "arm_w": 0.050},
    500: {"yb": 0.100, "fl": (0.170, 0.245), "ap": (0.295, 0.370), "post": 0.075, "leg": 0.075, "rail": 0.075, "arm": (0.590, 0.665), "arm_merged": True, "arm_front": 0.375, "stretcher": None, "t": 0.075, "arm_w": 0.075},
}
PADS = {50: (0.085, 0.020, 0.060), 100: (0.085, 0.020, 0.060), 200: (0.090, 0.030, 0.060), 500: (0.075, 0.075, 0.075)}
SEAT = {50: {"t": 0.045, "front": 0.295, "r": 0.004, "merge": False}, 100: {"t": 0.045, "front": 0.295, "r": 0.0, "merge": False}, 200: {"t": 0.045, "front": 0.305, "r": 0.0, "merge": True}, 500: {"t": 0.075, "front": 0.295, "r": 0.0, "merge": True}}
BACK = {50: {"t": 0.040, "post": 0.045, "r": 0.004, "slats": ((0.48, 0.130), (0.70, 0.130))}, 100: {"t": 0.040, "post": 0.045, "r": 0.0, "slats": ((0.48, 0.130), (0.70, 0.130))}, 200: {"t": 0.040, "post": 0.045, "r": 0.0, "slats": ((0.48, 0.130), (0.70, 0.130))}, 500: {"t": 0.075, "post": 0.075, "r": 0.0, "slats": ((0.47, 0.100), (0.70, 0.120))}}
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}
SCALES = (50, 100, 200, 500)


def rail_top(s):
    """📐 Height of the raked seat rail top at forward position s (5 degrees down to the rear)."""
    return RAIL_FRONT[1] - (RAIL_FRONT[0] - s) * K5


def post_centre(y):
    """📏 Forward position of the back post centre line at height y (raked 15 degrees back)."""
    return POST_BASE[0] - (y - POST_BASE[1]) * T15


def cross(p, d, q, e):
    """✖️ Intersection of the lines p + t d and q + u e in the (s, y) plane."""
    den = d[0] * e[1] - d[1] * e[0]
    t = ((q[0] - p[0]) * e[1] - (q[1] - p[1]) * e[0]) / den
    return (p[0] + t * d[0], p[1] + t * d[1])


def offset_line(a, b, dist):
    """↔️ The line through a and b moved by dist along (dy, -ds)/|d|, towards +s for a line rising to +y."""
    d = (b[0] - a[0], b[1] - a[1])
    n = math.hypot(*d)
    return (a[0] + d[1] / n * dist, a[1] - d[0] / n * dist), d


def frame_outline(p):
    """🦴 The one-piece cast end frame (outer ring and holes) in (s, y) for the member widths of a scale."""
    yb = p["yb"]
    fl0, fl1 = p["fl"]
    ap0, ap1 = p["ap"]
    hp = p["post"] / 2 / C15
    dR = p["rail"] / C5
    rb = lambda s: rail_top(s) - dR
    up = (-S15, C15)
    pf = ((post_centre(POST_BASE[1]) + hp, POST_BASE[1]), up)
    pr = ((post_centre(POST_BASE[1]) - hp, POST_BASE[1]), up)
    rt_line = (RAIL_FRONT, (-C5, -S5))
    rb_line = ((RAIL_FRONT[0], RAIL_FRONT[1] - dR), (-C5, -S5))
    foot = (FOOT_S["rear"], yb)
    rlf = offset_line(foot, POST_BASE, p["leg"] / 2)
    rlr = offset_line(foot, POST_BASE, -p["leg"] / 2)
    at = lambda line, y: cross(line[0], line[1], (0.0, y), (1.0, 0.0))
    yA0, yA1, yT = p["arm"][0], p["arm"][1], POST_TOP_Y
    outer = [(fl1, yb), (fl1, rb(fl1)), (ap1, rb(ap1))]
    holes = []
    if p["arm_merged"]:
        outer += [(ap1, yA0), (p["arm_front"], yA0), (p["arm_front"], yA1), at(pf, yA1)]
        holes.append([(ap0, rail_top(ap0)), cross(*rt_line, *pf), at(pf, yA0), (ap0, yA0)])
    else:
        outer += [(ap1, yA0), (ap0, yA0), (ap0, rail_top(ap0)), cross(*rt_line, *pf)]
    outer += [at(pf, yT), at(pr, yT), cross(*pr, *rlr), at(rlr, yb), at(rlf, yb)]
    if p["stretcher"] is not None:
        st0, st1 = p["stretcher"]
        outer += [at(rlf, st0), (fl0, st0), (fl0, yb)]
        holes.append([(fl0, st1), (fl0, rb(fl0)), cross(*rb_line, *rlf), at(rlf, st1)])
    else:
        outer += [cross(*rb_line, *rlf), (fl0, rb(fl0)), (fl0, yb)]
    return outer, holes


def rotated_rect(origin, along, normal, w, h, r=0.0, n_arc=3):
    """▭ A rectangle with one corner at origin, w along `along`, h along `normal`, corners eased by radius r."""
    if r <= 0:
        local = [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)]
    else:
        local = []
        for (cx, cy), a0 in zip([(r, r), (w - r, r), (w - r, h - r), (r, h - r)], [180, 270, 0, 90]):
            for k in range(n_arc + 1):
                a = math.radians(a0 + 90 * k / n_arc)
                local.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return [(origin[0] + a * along[0] + b * normal[0], origin[1] + a * along[1] + b * normal[1]) for a, b in local]


E_SEAT, N_SEAT = (-C5, -S5), (-S5, C5)
D_BACK, M_BACK = (-S15, C15), (C15, S15)


def seat_sections(scale):
    """🪵 Seat slat sections (s, y) on the raked rail, front (replaced robinia) first; coarse scales merge the two greyed rear slats."""
    c = SEAT[scale]
    w, g, t = 0.130, 0.015, c["t"]
    p0 = (c["front"], rail_top(c["front"]))
    step = lambda k: (p0[0] + k * E_SEAT[0], p0[1] + k * E_SEAT[1])
    if not c["merge"]:
        return [rotated_rect(step((w + g) * i), E_SEAT, N_SEAT, w, t, c["r"]) for i in range(3)]
    rear = 2 * w + g + (c["front"] - 0.295) / C5
    return [rotated_rect(p0, E_SEAT, N_SEAT, w, t), rotated_rect(step(w), E_SEAT, N_SEAT, rear, t)]


def seat_frames(scale):
    """📍 Corner frame (origin on the rail top, width, thickness) of each seat slat section, for bolts, gum and textures."""
    c = SEAT[scale]
    out = []
    for i in range(3 if not c["merge"] else 2):
        k = (0.145 * i) if not c["merge"] else (0.0 if i == 0 else 0.130)
        o = (c["front"] + k * E_SEAT[0], rail_top(c["front"]) + k * E_SEAT[1])
        out.append((o, 0.130, c["t"]))
    return out


def back_sections(scale):
    """🪵 Back slat sections (s, y) against the front face of the raked post."""
    c = BACK[scale]
    hp = c["post"] / 2 / C15
    return [rotated_rect((post_centre(yb) + hp, yb), D_BACK, M_BACK, w, c["t"], c["r"]) for yb, w in c["slats"]]


def back_frames(scale):
    """📍 Corner frame (origin on the post face, width, thickness) of each back slat."""
    c = BACK[scale]
    hp = c["post"] / 2 / C15
    return [((post_centre(yb) + hp, yb), w, c["t"]) for yb, w in c["slats"]]


def armrest_profile(scale):
    """💪 Armrest side profile (s, y): from 12 mm inside the post to the rounded front knuckle (square at 1:200)."""
    y0, y1, front, rad = 0.625, 0.665, 0.355, 0.020
    hp = 0.045 / 2 / C15
    rear0, rear1 = post_centre(y0) + hp - 0.012, post_centre(y1) + hp - 0.012
    if scale == 200:
        return [(rear0, y0), (front + rad, y0), (front + rad, y1), (rear1, y1)]
    n = 6 if scale == 50 else 3
    arc = [(front + rad * math.cos(math.radians(-90 + 180 * k / n)), (y0 + y1) / 2 + rad * math.sin(math.radians(-90 + 180 * k / n))) for k in range(n + 1)]
    return [(rear0, y0)] + arc + [(rear1, y1)]


def ring_area(pts):
    """📐 Signed shoelace area of a ring."""
    return 0.5 * sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))


def ring_perimeter(pts):
    """📏 Perimeter of a closed ring."""
    return sum(math.hypot(pts[(i + 1) % len(pts)][0] - pts[i][0], pts[(i + 1) % len(pts)][1] - pts[i][1]) for i in range(len(pts)))


def ring_centroid(pts):
    """🎯 Area centroid of a ring."""
    a = ring_area(pts)
    f = lambda i: pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]
    cx = sum((pts[i][0] + pts[(i + 1) % len(pts)][0]) * f(i) for i in range(len(pts))) / (6 * a)
    cy = sum((pts[i][1] + pts[(i + 1) % len(pts)][1]) * f(i) for i in range(len(pts))) / (6 * a)
    return cx, cy


def section_props(outer, holes=()):
    """🧮 Area, perimeter and centroid (s, y) of a ring with holes."""
    a0, c0 = abs(ring_area(outer)), ring_centroid(outer)
    area, mx, my, per = a0, a0 * c0[0], a0 * c0[1], ring_perimeter(outer)
    for h in holes:
        ah, ch = abs(ring_area(h)), ring_centroid(h)
        area, mx, my, per = area - ah, mx - ah * ch[0], my - ah * ch[1], per + ring_perimeter(h)
    return area, per, (mx / area, my / area)


def masses():
    """⚖️ Masses and centres (x, y, s) of the four loaded bones from the 1:50 parts and densities."""
    p = FRAME_PARAMS[50]
    outer, holes = frame_outline(p)
    a_f, _, (sf, yf) = section_props(outer, holes)
    a_a, _, (sa, ya) = section_props(armrest_profile(50))
    pl, ph, pw = PADS[50]
    m_plate = a_f * p["t"] * DENSITY["cast-iron-painted"]
    m_arm = a_a / 0.040 * ARM_RIB_AREA * DENSITY["cast-iron-painted"]
    m_pad = pl * ph * pw * DENSITY["cast-iron-rusted"]
    pads = [(FOOT_S["front"], Y_TOP_FOOTING + ph / 2), (FOOT_S["rear"], Y_TOP_FOOTING + ph / 2)]
    m_frame = m_plate + m_arm + 2 * m_pad
    s_frame = (m_plate * sf + m_arm * sa + m_pad * sum(q[0] for q in pads)) / m_frame
    y_frame = (m_plate * yf + m_arm * ya + m_pad * sum(q[1] for q in pads)) / m_frame
    seat = seat_sections(50)
    bolt = 0.068
    ms, ss, ys = 0.0, 0.0, 0.0
    for i, sec in enumerate(seat):
        a, _, (cs, cy) = section_props(sec)
        m = a * LENGTH * DENSITY["robinia-fresh" if i == 0 else "hardwood-greyed"]
        ms, ss, ys = ms + m, ss + m * cs, ys + m * cy
    ms_bolts = 6 * bolt
    seat_m = ms + ms_bolts
    out = {
        "frame": {"kg": m_frame, "s": s_frame, "y": y_frame, "plate": m_plate, "arm": m_arm, "pad": m_pad},
        "seat": {"kg": seat_m, "s": ss / ms, "y": ys / ms, "slats": ms},
    }
    for name, sec in zip(("back-lower", "back-upper"), back_sections(50)):
        a, _, (cs, cy) = section_props(sec)
        m = a * LENGTH * DENSITY["hardwood-greyed"]
        out[name] = {"kg": m + 2 * bolt, "s": cs, "y": cy, "slats": m}
    return out


M = masses()
TOTAL_KG = 2 * M["frame"]["kg"] + M["seat"]["kg"] + M["back-lower"]["kg"] + M["back-upper"]["kg"]
Z_COM = (2 * M["frame"]["kg"] * -M["frame"]["s"] + sum(M[k]["kg"] * -M[k]["s"] for k in ("seat", "back-lower", "back-upper"))) / TOTAL_KG
BACK_Y = {"back-lower": 0.548, "back-upper": 0.768}

BONES = [
    {"id": "ground", "parent": None, "title": T("Ground frame", "Bezugsrahmen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}, "o": (0.0, 0.0, 0.0), "a": 0},
    {"id": "footing-r", "parent": "ground", "title": T("Right point footing (sitter's right)", "Rechtes Punktfundament (rechts vom Sitzenden)"), "x": FRAME_X[0], "y": Y_TOP_FOOTING, "angleDeg": -90, "lengthM": 0.425, "joint": {"type": "fixed"}, "o": (FRAME_X[0], Y_TOP_FOOTING, 0.0), "a": -90},
    {"id": "footing-l", "parent": "ground", "title": T("Left point footing (sitter's left)", "Linkes Punktfundament (links vom Sitzenden)"), "x": FRAME_X[1], "y": Y_TOP_FOOTING, "angleDeg": -90, "lengthM": 0.425, "joint": {"type": "fixed"}, "o": (FRAME_X[1], Y_TOP_FOOTING, 0.0), "a": -90},
    {"id": "bench", "parent": "ground", "title": T("Bench body (lifted out by the depot)", "Bankkörper (vom Bauhof ausgehoben)"), "x": 0, "y": Y_TOP_FOOTING, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6}, "o": (0.0, Y_TOP_FOOTING, 0.0), "a": 0},
]
for bid, x, title in (("end-r", FRAME_X[0], T("Right cast end frame", "Rechtes Gussseitenteil")), ("end-l", FRAME_X[1], T("Left cast end frame", "Linkes Gussseitenteil"))):
    zc = -M["frame"]["s"]
    BONES.append({"id": bid, "parent": "bench", "title": title, "x": x, "y": 0, "z": r6(zc), "angleDeg": 90, "lengthM": 0.81, "joint": {"type": "fixed"}, "o": (x, Y_TOP_FOOTING, zc), "a": 90,
                  "mass": {"kg": round(M["frame"]["kg"], 2), "comX": r6(M["frame"]["y"] - Y_TOP_FOOTING), "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "en-1561-2011"},
                  "section": {"areaM2": r6(2 * 0.045 * 0.022), "secondMomentM4": float("%.4g" % (2 * 0.045 * 0.022 ** 3 / 12)), "extremeFibreM": 0.011, "material": "cast-iron-painted", "title": T("Front and rear leg, 45 x 22 mm cast section each", "Vorder- und Hinterbein, je 45 x 22 mm Gussquerschnitt")}})
BONES.append({"id": "seat", "parent": "bench", "title": T("Seat slats", "Sitzlatten"), "x": 0, "y": 0.40, "z": r6(-M["seat"]["s"]), "angleDeg": 0, "lengthM": LENGTH, "joint": {"type": "fixed"}, "o": (0.0, 0.425, -M["seat"]["s"]), "a": 0,
              "mass": {"kg": round(M["seat"]["kg"], 2), "comX": 0.9, "comY": r6(M["seat"]["y"] - 0.425), "distribution": "uniform", "source": "authored-estimate", "reference": "wagenfuehr-holzatlas-2007"},
              "section": {"areaM2": r6(3 * 0.130 * 0.045), "secondMomentM4": float("%.4g" % (3 * 0.130 * 0.045 ** 3 / 12)), "extremeFibreM": 0.0225, "material": "hardwood-greyed", "title": T("Three seat slats 130 x 45 mm, hardwood D60 (one robinia D40)", "Drei Sitzlatten 130 x 45 mm, Laubholz D60 (eine Robinie D40)")}})
for bid, title in (("back-lower", T("Lower back slat", "Untere Rückenlatte")), ("back-upper", T("Upper back slat", "Obere Rückenlatte"))):
    BONES.append({"id": bid, "parent": "bench", "title": title, "x": 0, "y": r6(BACK_Y[bid] - Y_TOP_FOOTING), "z": r6(-M[bid]["s"]), "angleDeg": 0, "lengthM": LENGTH, "joint": {"type": "fixed"}, "o": (0.0, BACK_Y[bid], -M[bid]["s"]), "a": 0,
                  "mass": {"kg": round(M[bid]["kg"], 2), "comX": 0.9, "comY": r6(M[bid]["y"] - BACK_Y[bid]), "distribution": "uniform", "source": "authored-estimate", "reference": "wagenfuehr-holzatlas-2007"},
                  "section": {"areaM2": r6(0.130 * 0.040), "secondMomentM4": float("%.4g" % (0.130 * 0.040 ** 3 / 12)), "extremeFibreM": 0.020, "material": "hardwood-greyed", "title": T("Back slat 130 x 40 mm, hardwood D60", "Rückenlatte 130 x 40 mm, Laubholz D60")}})
BONE = {b["id"]: b for b in BONES}


def to_local(bone, x, y, z):
    """🔁 World point to the (u, v, w) rest frame of a bone (angles 0, 90 or -90 about the lateral axis)."""
    ox, oy, oz = BONE[bone]["o"]
    a = BONE[bone]["a"]
    if a == 0:
        return (x - ox, y - oy, z - oz)
    if a == 90:
        return (y - oy, -(x - ox), z - oz)
    return (-(y - oy), x - ox, z - oz)


def local_axis(bone, dx, dy, dz):
    """🧭 World direction in the bone frame."""
    a = BONE[bone]["a"]
    if a == 0:
        return (dx, dy, dz)
    if a == 90:
        return (dy, -dx, dz)
    return (-dy, dx, dz)


def to_elev(bone, x, y):
    """📐 World (x, y) to the (u, v) elevation coordinates of a bone."""
    u, v, _ = to_local(bone, x, y, 0.0)
    return (r6(u), r6(v))


def ccw(pts):
    """🔄 The ring counter-clockwise."""
    return pts if ring_area(pts) > 0 else list(reversed(pts))


def cw(pts):
    """🔃 The ring clockwise."""
    return list(reversed(ccw(pts)))


def part(pid, bone, kind, material, region, tone, layers, **geo):
    """🧩 One physical part in world metres: `xprism` (an (s, y) profile pushed from x0 to x1), `box` or `ycyl` (vertical cylinder)."""
    return {"id": pid, "bone": bone, "kind": kind, "material": material, "region": region, "tone": tone, "layers": layers, **geo}


def parts(scale):
    """🪑 Every physical part of one scale."""
    fp = FRAME_PARAMS[scale]
    out = []
    outer, holes = frame_outline(fp)
    for fid, bone, x in (("frame-r", "end-r", FRAME_X[0]), ("frame-l", "end-l", FRAME_X[1])):
        side = fid[-1]
        out.append(part(fid, bone, "xprism", "cast-iron-painted", "cast-frames", 0.30, {"elevation": 2.0, "front": 3.0, "plan": 3.0}, prof=outer, holes=holes, x0=x - fp["t"] / 2, x1=x + fp["t"] / 2, front=side == "l", hatch={}))
        if not fp["arm_merged"]:
            out.append(part(f"armrest-{side}", bone, "xprism", "cast-iron-painted", "cast-frames", 0.26, {"elevation": 4.6, "front": 3.5, "plan": 6.0}, prof=armrest_profile(scale), holes=[], x0=x - fp["arm_w"] / 2, x1=x + fp["arm_w"] / 2, front=side == "l", hatch={}))
        pl, ph, pw = PADS[scale]
        for foot, s in FOOT_S.items():
            out.append(part(f"pad-{foot}-{side}", bone, "box", "cast-iron-rusted", "feet-rusted", 0.18, {"elevation": 2.5 if foot == "front" else 2.4, "front": 3.2, "plan": 2.0},
                            x0=x - pw / 2, x1=x + pw / 2, y0=Y_TOP_FOOTING, y1=Y_TOP_FOOTING + ph, s0=s - pl / 2, s1=s + pl / 2, front=side == "l", elevation=foot == "front"))
        fbone = "footing-" + side
        out.append(part(f"footing-{side}", fbone, "box", "concrete-c20-25", "footings", 0.78, {"elevation": 1.0, "front": 1.0, "plan": 1.0},
                        x0=x - FOOTING["half"], x1=x + FOOTING["half"], y0=FOOTING["y0"], y1=FOOTING["y1"], s0=FOOTING["s0"], s1=FOOTING["s1"], front=side == "l", hatch={"elevation": "concrete", "front": "concrete", "plan": "concrete"} if scale in (50, 100) else {}))
        if scale == 50:
            xd = x - DOWEL_OUT if side == "r" else x + DOWEL_OUT
            for foot, s in FOOT_S.items():
                out.append(part(f"dowel-{foot}-{side}", fbone, "ycyl", "steel-galvanised", "feet-rusted", 0.08, {"elevation": 2.6 if foot == "front" else 2.55, "front": 3.3, "plan": 2.5}, x=xd, s=s, r=0.006, y0=-0.125, y1=Y_TOP_FOOTING + PADS[50][1] + 0.010, front=side == "l", elevation=foot == "front", role="detail", thermal=False))
    seat = seat_sections(scale)
    for i, sec in enumerate(seat):
        replaced = i == 0
        out.append(part("seat-slat-replaced" if replaced else (f"seat-slat-{i + 1}" if len(seat) == 3 else "seat-slats-rear"), "seat", "xprism", "robinia-fresh" if replaced else "hardwood-greyed",
                        "seat-slat-replaced" if replaced else "seat-slats-greyed", 0.86 if replaced else 0.60 - 0.04 * i, {"elevation": 4.2 - 0.1 * i, "front": 5.0 + 0.01 * i, "plan": 5.0 + 0.01 * i},
                        prof=sec, holes=[], x0=0.0, x1=LENGTH, front=True, hatch=({"elevation": "grain-fresh" if replaced else "grain", "plan": "grain-fresh" if replaced else "grain"} if scale == 50 else {})))
    for i, sec in enumerate(back_sections(scale)):
        out.append(part(f"back-slat-{'lower' if i == 0 else 'upper'}", "back-lower" if i == 0 else "back-upper", "xprism", "hardwood-greyed", "back-slats-greyed", 0.55 if i == 0 else 0.52, {"elevation": 3.0 + 0.1 * i, "front": 5.1 + 0.01 * i, "plan": 5.5 if i == 0 else 7.0},
                        prof=sec, holes=[], x0=0.0, x1=LENGTH, front=True, hatch=({"elevation": "grain", "plan": "grain"} if scale == 50 else {})))
    if fp["arm_merged"]:
        pass
    return out


def textures3d(scale):
    """🧽 The traces that are only surface at every scale (graffiti, sticker residue, moss, chewing gum, carriage-bolt heads), as texture solids of 1:50 and 1:100."""
    if scale not in (50, 100):
        return []
    out = []
    (q_lo, w_lo, t_lo), (q_up, w_up, t_up) = back_frames(scale)
    face = lambda q, w, t, f0, f1, inset=0.0005, thick=0.0015: rotated_rect((q[0] + (t - inset) * M_BACK[0] + f0 * w * D_BACK[0], q[1] + (t - inset) * M_BACK[1] + f0 * w * D_BACK[1]), D_BACK, M_BACK, (f1 - f0) * w, thick)
    out.append({"id": "graffiti-tag", "bone": "back-upper", "kind": "xprism", "material": "spray-paint", "tone": 0.14, "prof": face(q_up, w_up, t_up, 0.18, 0.82), "x0": 0.70, "x1": 1.06})
    out.append({"id": "moss-top-edge", "bone": "back-upper", "kind": "xprism", "material": "moss-algae", "tone": 0.32, "prof": rotated_rect((q_up[0] + (w_up - 0.0005) * D_BACK[0], q_up[1] + (w_up - 0.0005) * D_BACK[1]), M_BACK, D_BACK, t_up, 0.0015), "x0": 0.25, "x1": 1.55})
    if scale == 50:
        out.append({"id": "sticker-residue", "bone": "back-lower", "kind": "xprism", "material": "sticker-residue", "tone": 0.9, "prof": face(q_lo, w_lo, t_lo, 0.30, 0.62), "x0": 1.30, "x1": 1.36})
    return out


def ground_lines(scale):
    """🟫 The water-bound gravel surface around the footings with the hollow the sitters' heels have worn in front of the seat (role ground, not part of the body)."""
    dip = [(0.315, 0.0), (0.36, -0.006), (0.42, -0.024), (0.50, -0.036), (0.60, -0.036), (0.68, -0.022), (0.74, -0.006), (0.80, 0.0), (0.95, 0.0)]
    if scale in (200, 500):
        dip = [(0.315, 0.0), (0.42, -0.03), (0.68, -0.03), (0.80, 0.0), (0.95, 0.0)]
    rear = [(-0.70, 0.0), (FOOTING["s0"], 0.0)]
    return dip, rear


def shape(sid, bone, role, z, filled, tone, geom, region=None, material=None, hatch=None, pen=None):
    """✏️ One 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": round(z, 4), "filled": filled, "tone": tone, **geom}
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen:
        s["penMm"] = pen
    return s


def rect_geom(x0, y0, x1, y1, r=None):
    """▭ A rect geometry from two corners."""
    g = {"kind": "rect", "x": r6(min(x0, x1)), "y": r6(min(y0, y1)), "w": r6(abs(x1 - x0)), "h": r6(abs(y1 - y0))}
    if r:
        g["r"] = r
    return g


def poly_geom(pts, closed=True):
    """〰️ A polyline geometry."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}


def path_geom(rings):
    """🛤️ A path of closed rings (even-odd)."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ring) + " Z" for ring in rings)}


def elev_rect(bone, x0, x1, y0, y1):
    """📐 A world elevation rectangle in the bone's (u, v) frame."""
    a, b = to_elev(bone, x0, y0), to_elev(bone, x1, y1)
    return rect_geom(a[0], a[1], b[0], b[1])


def world_box(p):
    """📦 World bounds (x0, x1, y0, y1, s0, s1) of a part."""
    if p["kind"] == "box":
        return p["x0"], p["x1"], p["y0"], p["y1"], p["s0"], p["s1"]
    if p["kind"] == "ycyl":
        return p["x"] - p["r"], p["x"] + p["r"], p["y0"], p["y1"], p["s"] - p["r"], p["s"] + p["r"]
    ys = [q[1] for q in p["prof"]]
    ss = [q[0] for q in p["prof"]]
    return p["x0"], p["x1"], min(ys), max(ys), min(ss), max(ss)


class Counter:
    """🔢 Unique paint depths per view."""

    def __init__(self):
        self.n = {"elevation": 0, "front": 0, "plan": 0}

    def z(self, view, layer):
        self.n[view] += 1
        return layer + self.n[view] * 0.0001


def drawings(scale):
    """🖼️ The three views of one scale: projections of the parts plus the 2D-only traces."""
    ps = parts(scale)
    c = Counter()
    views = {"elevation": [], "front": [], "plan": []}
    pen_seat = {50: 0.35, 100: None, 200: None, 500: None}[scale]
    for p in ps:
        x0, x1, y0, y1, s0, s1 = world_box(p)
        role = p.get("role", "body")
        hatch = p.get("hatch", {})
        pen = pen_seat if p["bone"] in ("seat", "back-lower", "back-upper") else None
        if p.get("elevation", True):
            if p["kind"] == "xprism" and p["bone"] in ("end-r", "end-l") and p["id"].startswith("frame"):
                views["elevation"].append(shape(p["id"], p["bone"], role, c.z("elevation", p["layers"]["elevation"]), True, p["tone"], elev_rect(p["bone"], x0, x1, y0, y1), p["region"], p["material"]))
                ap0 = FRAME_PARAMS[scale]["ap"][0]
                views["elevation"].append(shape(p["id"] + "-arm-post", p["bone"], role, c.z("elevation", 4.5), True, p["tone"] + 0.04, elev_rect(p["bone"], x0, x1, rail_top(ap0), FRAME_PARAMS[scale]["arm"][0]), p["region"], p["material"]))
            else:
                views["elevation"].append(shape(p["id"], p["bone"], role, c.z("elevation", p["layers"]["elevation"]), True, p["tone"], elev_rect(p["bone"], x0, x1, y0, y1), p["region"], p["material"], hatch.get("elevation"), pen))
        if p.get("front", True):
            if p["kind"] == "xprism":
                rings = [ccw([(-s, y) for s, y in p["prof"]])] + [cw([(-s, y) for s, y in h]) for h in p["holes"]]
                g = path_geom(rings) if p["holes"] else poly_geom(rings[0])
            else:
                g = rect_geom(-s1, y0, -s0, y1)
            views["front"].append(shape(p["id"], p["bone"], role, c.z("front", p["layers"]["front"]), True, p["tone"], g, p["region"], p["material"], hatch.get("front"), pen))
        if p["kind"] == "ycyl":
            g = {"kind": "ellipse", "cx": r6(p["x"]), "cy": r6(-p["s"]), "rx": p["r"], "ry": p["r"]}
        else:
            g = rect_geom(x0, -s1, x1, -s0)
        views["plan"].append(shape(p["id"], p["bone"], role, c.z("plan", p["layers"]["plan"]), True, p["tone"], g, p["region"], p["material"], hatch.get("plan"), pen))
    dip, rear = ground_lines(scale)
    views["front"].append(shape("ground-front", "footing-l", "ground", c.z("front", 0.5), False, 0.5, poly_geom([(-s, y) for s, y in dip], False)))
    views["front"].append(shape("ground-rear", "footing-l", "ground", c.z("front", 0.5), False, 0.5, poly_geom([(-s, y) for s, y in rear], False)))
    views["elevation"].append(shape("ground-line", "footing-r", "ground", c.z("elevation", 0.5), False, 0.5, poly_geom([to_elev("footing-r", -0.15, 0.0), to_elev("footing-r", FRAME_X[0] - FOOTING["half"], 0.0)], False)))
    views["elevation"].append(shape("ground-line-mid", "footing-r", "ground", c.z("elevation", 0.5), False, 0.5, poly_geom([to_elev("footing-r", FRAME_X[0] + FOOTING["half"], 0.0), to_elev("footing-r", FRAME_X[1] - FOOTING["half"], 0.0)], False)))
    views["elevation"].append(shape("ground-line-end", "footing-l", "ground", c.z("elevation", 0.5), False, 0.5, poly_geom([to_elev("footing-l", FRAME_X[1] + FOOTING["half"], 0.0), to_elev("footing-l", 1.95, 0.0)], False)))
    hollow = [(0.30 + 1.20 * (0.5 + 0.5 * math.cos(math.radians(a))), -0.55 + 0.20 * math.sin(math.radians(a)) * (1.0 + 0.08 * math.cos(math.radians(3 * a)))) for a in range(0, 360, 20 if scale in (50, 100) else 45)]
    views["plan"].append(shape("worn-hollow", "footing-r", "ground", c.z("plan", 0.2), True, 0.92, poly_geom(hollow), pen={50: 0.18, 100: 0.13, 200: 0.13, 500: 0.13}[scale]))
    traces(scale, views, c)
    return views


def traces(scale, views, c):
    """🩹 The 2D-only traces of use and weather of one scale: checks, graffiti, sticker residue, moss, chewing gum, rust and flaking paint, bolt heads and dowel ends."""
    if scale not in (50, 100):
        return
    (q_lo, w_lo, t_lo), (q_up, w_up, t_up) = back_frames(scale)
    y_lo = (q_lo[1], q_lo[1] + w_lo * C15 + t_lo * S15)
    y_up = (q_up[1], q_up[1] + w_up * C15 + t_up * S15)
    be = lambda x, y, bone="back-upper": (r6(x), r6(y - BACK_Y[bone]))
    if scale == 50:
        tag = [(0.72, 0.745), (0.76, 0.79), (0.79, 0.742), (0.83, 0.80), (0.87, 0.75), (0.90, 0.78), (0.95, 0.744), (0.99, 0.795), (1.04, 0.752)]
        views["elevation"].append(shape("graffiti-tag", "back-upper", "texture", c.z("elevation", 7.0), False, 0.1, poly_geom([be(*q) for q in tag], False), "back-slats-greyed", "spray-paint", pen=0.25))
        views["elevation"].append(shape("graffiti-tag-underline", "back-upper", "texture", c.z("elevation", 7.0), False, 0.1, poly_geom([be(0.73, 0.733), be(1.03, 0.738)], False), "back-slats-greyed", "spray-paint"))
        views["elevation"].append(shape("sticker-residue", "back-lower", "texture", c.z("elevation", 7.0), True, 0.92, rect_geom(*be(1.30, y_lo[0] + 0.045, "back-lower"), *be(1.36, y_lo[0] + 0.085, "back-lower")), "back-slats-greyed", "sticker-residue"))
        for k, (xa, xb, f) in enumerate(((0.30, 0.64, 0.35), (1.02, 1.44, 0.62), (0.18, 0.46, 0.55))):
            ys, bb = (y_lo, "back-lower") if k < 2 else (y_up, "back-upper")
            y = ys[0] + f * (ys[1] - ys[0])
            views["elevation"].append(shape(f"check-back-{k + 1}", bb, "texture", c.z("elevation", 6.8), False, 0.3, poly_geom([be(xa, y, bb), be((xa + xb) / 2, y + 0.003, bb), be(xb, y - 0.002, bb)], False), "back-slats-greyed"))
        for side, x in (("r", FRAME_X[0]), ("l", FRAME_X[1])):
            for k, (q, w, t) in enumerate(back_frames(50)):
                cs = q[0] + 0.5 * w * D_BACK[0] + t * M_BACK[0]
                cy = q[1] + 0.5 * w * D_BACK[1] + t * M_BACK[1]
                bb = ("back-lower", "back-upper")[k]
                views["elevation"].append(shape(f"back-bolt-{k + 1}-{side}", bb, "detail", c.z("elevation", 6.0), False, 0.15, {"kind": "ellipse", "cx": r6(x), "cy": r6(cy - BACK_Y[bb]), "rx": 0.0095, "ry": 0.0095}, "back-slats-greyed", "steel-galvanised"))
            fb = "end-" + side
            yb = FRAME_PARAMS[50]["yb"]
            views["elevation"].append(shape(f"rust-streak-{side}", fb, "texture", c.z("elevation", 6.5), True, 0.16, elev_rect(fb, x - 0.011, x + 0.011, yb, yb + 0.05), "feet-rusted", "cast-iron-rusted"))
        for k, (s_a, ys) in enumerate(((0.245, 0.045), (-0.247, 0.045))):
            pts = [(s_a, ys), (s_a, ys + 0.06), (s_a - 0.012, ys + 0.045), (s_a - 0.025, ys + 0.07), (s_a - 0.045, ys + 0.04), (s_a - 0.045, ys)]
            views["front"].append(shape(f"rust-foot-{k + 1}", "end-l", "texture", c.z("front", 4.0), True, 0.15, poly_geom([(-s, y) for s, y in pts]), "feet-rusted", "cast-iron-rusted"))
        for k, (s_c, y_c) in enumerate(((post_centre(0.55), 0.55), (post_centre(0.78), 0.78), (0.0, rail_top(0.0) - 0.02), (0.325, 0.52))):
            pts = [(s_c - 0.008, y_c - 0.006), (s_c + 0.006, y_c - 0.009), (s_c + 0.010, y_c + 0.004), (s_c - 0.004, y_c + 0.010)]
            views["front"].append(shape(f"paint-flake-{k + 1}", "end-l", "texture", c.z("front", 4.1), True, 0.55, poly_geom([(-s, y) for s, y in pts]), "cast-frames", "cast-iron-painted"))
        for side, x in (("r", FRAME_X[0]), ("l", FRAME_X[1])):
            for k, (o, w, t) in enumerate(seat_frames(50)):
                cs = o[0] + 0.5 * w * E_SEAT[0] + t * N_SEAT[0]
                views["plan"].append(shape(f"seat-bolt-{k + 1}-{side}", "seat", "detail", c.z("plan", 9.0), False, 0.15, {"kind": "ellipse", "cx": r6(x), "cy": r6(-cs), "rx": 0.0095, "ry": 0.0095}, "seat-slat-replaced" if k == 0 else "seat-slats-greyed", "steel-galvanised"))
        for k, (o, w, t) in enumerate(seat_frames(50)[1:]):
            s_mid = o[0] + 0.5 * w * E_SEAT[0]
            for j, (xa, xb) in enumerate(((0.32, 0.71), (1.05, 1.47))):
                ds = (0.03 if j == 0 else -0.025) * (1 if k == 0 else -1)
                views["plan"].append(shape(f"check-seat-{k + 2}-{j + 1}", "seat", "texture", c.z("plan", 8.0), False, 0.3, poly_geom([(xa, -(s_mid + ds)), ((xa + xb) / 2, -(s_mid + ds + 0.004)), (xb, -(s_mid + ds - 0.002))], False), "seat-slats-greyed"))
        o2 = seat_frames(50)[1][0]
        for k, (x, f) in enumerate(((0.52, 0.3), (0.97, 0.7), (1.21, 0.45))):
            s = o2[0] + f * 0.130 * E_SEAT[0]
            views["plan"].append(shape(f"gum-{k + 1}", "seat", "texture", c.z("plan", 8.5), True, 0.2, {"kind": "ellipse", "cx": r6(x), "cy": r6(-s), "rx": 0.0085, "ry": 0.0075}, "seat-slats-greyed", "chewing-gum"))
    else:
        tag = [(0.72, 0.745), (0.80, 0.79), (0.88, 0.745), (0.96, 0.79), (1.04, 0.75)]
        views["elevation"].append(shape("graffiti-tag", "back-upper", "texture", c.z("elevation", 7.0), False, 0.1, poly_geom([be(*q) for q in tag], False), "back-slats-greyed", "spray-paint", pen=0.18))
        pts = [(0.245, 0.045), (0.245, 0.10), (0.215, 0.075), (0.200, 0.10), (0.200, 0.045)]
        views["front"].append(shape("rust-foot-1", "end-l", "texture", c.z("front", 4.0), True, 0.15, poly_geom([(-s, y) for s, y in pts]), "feet-rusted", "cast-iron-rusted"))
    moss_s = (q_up[0] + w_up * D_BACK[0], q_up[0] + w_up * D_BACK[0] + t_up * M_BACK[0])
    views["plan"].append(shape("moss-top-edge", "back-upper", "texture", c.z("plan", 8.0), True, 0.3, rect_geom(0.25, -max(moss_s), 1.55, -min(moss_s)), "back-slats-greyed", "moss-algae", "moss" if scale == 50 else None))


def solid_of(p, scale):
    """🧊 A part as a solid of its bone."""
    bone = p["bone"]
    base = {"id": p["id"], "bone": bone, "material": p["material"], "role": p.get("role", "body"), "tone": p["tone"]}
    if p.get("texture"):
        base["texture"] = True
    if p["kind"] == "xprism":
        a = BONE[bone]["a"]
        oz = BONE[bone]["o"][2]
        if a == 0:
            prof = [(r6(y - BONE[bone]["o"][1]), r6(-s - oz)) for s, y in p["prof"]]
            holes = [[(r6(y - BONE[bone]["o"][1]), r6(-s - oz)) for s, y in h] for h in p.get("holes", [])]
            solid = {"kind": "extrude", "plane": "yz", "profile": [list(q) for q in ccw(prof)], "from": r6(p["x0"] - BONE[bone]["o"][0]), "to": r6(p["x1"] - BONE[bone]["o"][0])}
        else:
            prof = [(r6(-s - oz), r6(y - BONE[bone]["o"][1])) for s, y in p["prof"]]
            holes = [[(r6(-s - oz), r6(y - BONE[bone]["o"][1])) for s, y in h] for h in p.get("holes", [])]
            solid = {"kind": "extrude", "plane": "zx", "profile": [list(q) for q in ccw(prof)], "from": r6(-(p["x1"] - BONE[bone]["o"][0])), "to": r6(-(p["x0"] - BONE[bone]["o"][0]))}
        if holes:
            solid["holes"] = [[list(q) for q in cw(h)] for h in holes]
        return {**base, **solid}
    if p["kind"] == "box":
        x0, x1, y0, y1, s0, s1 = world_box(p)
        cx, cy, cz = (x0 + x1) / 2, (y0 + y1) / 2, -(s0 + s1) / 2
        u, v, w = to_local(bone, cx, cy, cz)
        d = (x1 - x0, y1 - y0, s1 - s0)
        size = d if BONE[bone]["a"] == 0 else (d[1], d[0], d[2])
        return {**base, "kind": "box", "size": [r6(q) for q in size], "transform": {"translation": [r6(u), r6(v), r6(w)]}}
    u, v, w = to_local(bone, p["x"], p["y0"], -p["s"])
    ax = local_axis(bone, 0, 1, 0)
    return {**base, "kind": "revolve", "profile": [[0, p["r"]], [r6(p["y1"] - p["y0"]), p["r"]]], "axis": list(ax), "origin": [r6(u), r6(v), r6(w)]}


def bolt_solids(scale):
    """🔩 Carriage-bolt heads (texture, 1:50) and bolt shafts (structure joints) at every slat and frame crossing."""
    body, joints = [], []
    r_joint = {50: 0.005, 100: 0.0075, 200: 0.015, 500: 0.0375}[scale]
    for k, (o, w, t) in enumerate(seat_frames(scale)):
        top = (o[0] + 0.5 * w * E_SEAT[0] + t * N_SEAT[0], o[1] + 0.5 * w * E_SEAT[1] + t * N_SEAT[1])
        low = (top[0] - (t + FRAME_PARAMS[scale]["rail"] * 0.8) * N_SEAT[0], top[1] - (t + FRAME_PARAMS[scale]["rail"] * 0.8) * N_SEAT[1])
        a = to_local("seat", FRAME_X[0], top[1], -top[0])
        b = to_local("seat", FRAME_X[0], low[1], -low[0])
        joints.append({"id": f"bolt-seat-{k + 1}", "kind": "capsule", "bone": "seat", "material": "steel-galvanised", "role": "detail", "tone": 0.25, "p0": [r6(q) for q in a], "p1": [r6(q) for q in b], "r0": r_joint, "r1": r_joint, "repeat": {"count": 2, "translation": [FRAME_X[1] - FRAME_X[0], 0, 0]}, "member": "seat", "part": "joint"})
        if scale == 50:
            ax = local_axis("seat", 0, N_SEAT[1], -N_SEAT[0])
            org = to_local("seat", FRAME_X[0], top[1] - 0.0005 * N_SEAT[1], -(top[0] - 0.0005 * N_SEAT[0]))
            body.append({"id": f"seat-bolt-head-{k + 1}", "kind": "revolve", "bone": "seat", "material": "steel-galvanised", "role": "detail", "tone": 0.15, "texture": True, "profile": [[0, 0.0095], [0.002, 0.009], [0.0035, 0.0068], [0.0048, 0.0038], [0.0053, 0.0]], "axis": [r6(q) for q in ax], "origin": [r6(q) for q in org], "repeat": {"count": 2, "translation": [FRAME_X[1] - FRAME_X[0], 0, 0]}})
    post = BACK[scale]["post"]
    for k, (q, w, t) in enumerate(back_frames(scale)):
        front = (q[0] + 0.5 * w * D_BACK[0] + t * M_BACK[0], q[1] + 0.5 * w * D_BACK[1] + t * M_BACK[1])
        rear = (front[0] - (t + post * 0.8) * M_BACK[0], front[1] - (t + post * 0.8) * M_BACK[1])
        bb = ("back-lower", "back-upper")[k]
        a = to_local(bb, FRAME_X[0], front[1], -front[0])
        b = to_local(bb, FRAME_X[0], rear[1], -rear[0])
        joints.append({"id": f"bolt-back-{k + 1}", "kind": "capsule", "bone": bb, "material": "steel-galvanised", "role": "detail", "tone": 0.25, "p0": [r6(v) for v in a], "p1": [r6(v) for v in b], "r0": r_joint, "r1": r_joint, "repeat": {"count": 2, "translation": [FRAME_X[1] - FRAME_X[0], 0, 0]}, "member": bb, "part": "joint"})
        if scale == 50:
            ax = local_axis(bb, 0, M_BACK[1], -M_BACK[0])
            org = to_local(bb, FRAME_X[0], front[1] - 0.0005 * M_BACK[1], -(front[0] - 0.0005 * M_BACK[0]))
            body.append({"id": f"back-bolt-head-{k + 1}", "kind": "revolve", "bone": bb, "material": "steel-galvanised", "role": "detail", "tone": 0.15, "texture": True, "profile": [[0, 0.0095], [0.002, 0.009], [0.0035, 0.0068], [0.0048, 0.0038], [0.0053, 0.0]], "axis": [r6(v) for v in ax], "origin": [r6(v) for v in org], "repeat": {"count": 2, "translation": [FRAME_X[1] - FRAME_X[0], 0, 0]}})
    return body, joints


def gum_solids():
    """🍬 Three chewing-gum spots on the middle seat slat (1:50 texture)."""
    o, w, t = seat_frames(50)[1]
    out = []
    ax = local_axis("seat", 0, N_SEAT[1], -N_SEAT[0])
    for k, (x, f) in enumerate(((0.52, 0.3), (0.97, 0.7), (1.21, 0.45))):
        p = (o[0] + f * w * E_SEAT[0] + (t - 0.0005) * N_SEAT[0], o[1] + f * w * E_SEAT[1] + (t - 0.0005) * N_SEAT[1])
        org = to_local("seat", x, p[1], -p[0])
        out.append({"id": f"gum-{k + 1}", "kind": "revolve", "bone": "seat", "material": "chewing-gum", "role": "texture", "tone": 0.2, "texture": True, "profile": [[0, 0.008], [0.002, 0.007], [0.0025, 0.0]], "axis": [r6(q) for q in ax], "origin": [r6(q) for q in org]})
    return out


def joint_dowels(scale):
    """📌 The M12 anchor dowels of the four feet as structure joints of the footings."""
    r = {50: 0.006, 100: 0.0075, 200: 0.015, 500: 0.0375}[scale]
    out = []
    for side, x in (("r", FRAME_X[0]), ("l", FRAME_X[1])):
        xd = x - DOWEL_OUT if side == "r" else x + DOWEL_OUT
        if scale == 500:
            xd = x
        bone = "footing-" + side
        top = Y_TOP_FOOTING + PADS[scale][1] + (0.010 if scale == 50 else 0.0)
        a = to_local(bone, xd, top, -FOOT_S["front"])
        b = to_local(bone, xd, -0.125, -FOOT_S["front"])
        out.append({"id": f"dowel-joint-{side}", "kind": "capsule", "bone": bone, "material": "steel-galvanised", "role": "detail", "tone": 0.25, "p0": [r6(q) for q in a], "p1": [r6(q) for q in b], "r0": r, "r1": r, "repeat": {"count": 2, "translation": [0, 0, r6(FOOT_S["front"] - FOOT_S["rear"])]}, "member": bone, "part": "joint"})
    return out


def mass_solids(scale):
    """⚖️ Mass markers at the centres of the four loaded bones."""
    r = max(0.02, FEATURE[scale] / 2 + 0.002)
    out = []
    for bid in ("end-r", "end-l", "seat", "back-lower", "back-upper"):
        b = BONE[bid]
        m = b["mass"]
        out.append({"id": f"mass-{bid}", "kind": "ellipsoid", "bone": bid, "material": "cast-iron-painted" if bid.startswith("end") else "hardwood-greyed", "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [m["comX"], m["comY"], 0.0]}, "member": bid, "part": "mass"})
    return out


def volumes(scale):
    """🧊 The body, thermal and structure lists of one scale."""
    ps = parts(scale)
    body = []
    for p in ps:
        body.append(solid_of(p, scale))
    for t in textures3d(scale):
        body.append(solid_of({**t, "texture": True, "role": "texture"}, scale))
    heads, joints = bolt_solids(scale)
    body += heads
    if scale == 50:
        body += gum_solids()
    thermal = [{"id": "th-" + p["id"], "of": p["id"], "region": p["region"]} for p in ps if p.get("thermal", True)]
    structure = [{"id": "m-" + p["id"], "of": p["id"], "member": p["bone"], "part": "member"} for p in ps if p.get("role", "body") == "body"]
    structure += joints + joint_dowels(scale) + mass_solids(scale)
    return body, thermal, structure


def write():
    """💾 Builds the document and writes it."""
    from importlib import util
    spec = util.spec_from_file_location("text", os.path.join(HERE, "author-text.py"))
    text = util.module_from_spec(spec)
    spec.loader.exec_module(text)
    doc = text.document(globals())
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("[DEBUG] wrote", OUT, "mass", round(TOTAL_KG, 2), "z_com", round(Z_COM, 4))


if __name__ == "__main__":
    write()
