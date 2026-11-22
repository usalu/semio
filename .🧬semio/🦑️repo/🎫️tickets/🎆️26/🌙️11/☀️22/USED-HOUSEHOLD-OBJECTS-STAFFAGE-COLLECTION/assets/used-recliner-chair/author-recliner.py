"""🪑 Hand-chosen geometry of the used recliner chair: rig rest frames, part profiles and per-scale level of detail.

Every number here is a design decision of the author (reference table in `../../reports/used-recliner-chair.md`).
World rest frame: x ahead (rear of the chair at 0), y up (floor 0), z left. The chair stands on a folded moving
blanket of 20 mm, so chair coordinates `yc` (glide bottom = 0) are world `y = yc + BLANKET`.
"""
import math

BLANKET = 0.02
ARM_IN = 0.26
ARM_OUT = 0.43
ARM_MID = (ARM_IN + ARM_OUT) / 2
ROLL_R = (ARM_OUT - ARM_IN) / 2
ROLL_Y = 0.535
ARM_FRONT = 0.918
ARM_REAR = 0.02
SEAT_X = 0.33
SEAT_Y = 0.36
BACK_PIVOT = (0.15, 0.40)
BACK_ANGLE = 100.0
LINK_PIVOT = (0.78, 0.12)
BOARD_TOP = (0.80, 0.335)
BOARD_LEN = 0.28
LEVER_PIVOT = (0.60, 0.36)
LEVER_ANGLE = 80.0
BACK_FRONT_TOP = 0.58
BACK_HALF = {50: 0.25, 100: 0.245, 200: 0.2575, 500: 0.2575}
GLIDES = [(0.06, ARM_MID), (0.06, -ARM_MID), (0.86, ARM_MID), (0.86, -ARM_MID)]
LINK_LEN = math.hypot(BOARD_TOP[0] - LINK_PIVOT[0], BOARD_TOP[1] - LINK_PIVOT[1])
LINK_ANGLE = math.degrees(math.atan2(BOARD_TOP[1] - LINK_PIVOT[1], BOARD_TOP[0] - LINK_PIVOT[0]))
FOOT_ANGLE = -90.0 - LINK_ANGLE


def r5(value):
    """🔢 Rounds a coordinate to 0.01 mm."""
    return round(value + 0.0, 5)


def pts(points):
    """📍 Rounds a point list."""
    return [[r5(x), r5(y)] for x, y in points]


def arc(cx, cy, r, a0, a1, n):
    """🌙 Points on a circular arc from a0 to a1 degrees (both ends included)."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def rounded_rect(x0, y0, x1, y1, r, n=4):
    """⬜ A rectangle with rounded corners as a counter-clockwise polygon."""
    if r <= 0:
        return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
    return arc(x1 - r, y0 + r, r, -90, 0, n) + arc(x1 - r, y1 - r, r, 0, 90, n) + arc(x0 + r, y1 - r, r, 90, 180, n) + arc(x0 + r, y0 + r, r, 180, 270, n)


BONES = {
    "ground": {"parent": None, "x": 0.0, "y": 0.0, "angle": 0.0},
    "blanket": {"parent": "ground", "x": -0.04, "y": 0.0, "angle": 0.0},
    "lift": {"parent": "ground", "x": 0.0, "y": BLANKET, "angle": 0.0},
    "tilt": {"parent": "lift", "x": 0.0, "y": 0.0, "angle": 0.0},
    "frame": {"parent": "tilt", "x": 0.0, "y": 0.0, "angle": 0.0},
    "glides": {"parent": "frame", "x": 0.0, "y": 0.0, "angle": 0.0},
    "seat": {"parent": "frame", "x": SEAT_X, "y": SEAT_Y, "angle": 0.0},
    "back": {"parent": "frame", "x": BACK_PIVOT[0], "y": BACK_PIVOT[1], "angle": BACK_ANGLE},
    "link": {"parent": "frame", "x": LINK_PIVOT[0], "y": LINK_PIVOT[1], "angle": LINK_ANGLE},
    "footrest": {"parent": "link", "x": LINK_LEN, "y": 0.0, "angle": FOOT_ANGLE},
    "lever": {"parent": "frame", "x": LEVER_PIVOT[0], "y": LEVER_PIVOT[1], "angle": LEVER_ANGLE},
}


def frame_of(bone):
    """🧭 World rest origin and angle (degrees) of a bone."""
    b = BONES[bone]
    if b["parent"] is None:
        return (b["x"], b["y"], b["angle"])
    px, py, pa = frame_of(b["parent"])
    c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
    return (px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"], pa + b["angle"])


def to_world(bone, points):
    """🌍 Bone-local (u, v) points to world rest (x, y)."""
    ox, oy, a = frame_of(bone)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [(ox + c * u - s * v, oy + s * u + c * v) for u, v in points]


def to_local(bone, points):
    """🦴 World rest (x, y) points to bone-local (u, v)."""
    ox, oy, a = frame_of(bone)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [(c * (x - ox) + s * (y - oy), -s * (x - ox) + c * (y - oy)) for x, y in points]


def chair(points):
    """🪑 Chair coordinates (glide bottom 0) to world rest coordinates."""
    return [(x, y + BLANKET) for x, y in points]


def arm_side(scale):
    """🛋 Side profile of the lower arm box in chair coordinates (to the roll centre line)."""
    bottom = 0.0 if scale >= 200 else 0.03
    return [(ARM_REAR, bottom), (0.90, bottom), (0.912, bottom + 0.03), (ARM_FRONT, bottom + 0.09), (ARM_FRONT, ROLL_Y), (ARM_REAR, ROLL_Y)]


def arm_outline(scale):
    """🛋 Side silhouette of the whole arm (lower box and padded roll) in chair coordinates."""
    bottom = 0.0 if scale >= 200 else 0.03
    n = 8 if scale <= 100 else 5
    front = arc(ARM_FRONT - ROLL_R, ROLL_Y, ROLL_R, 0, 90, n)
    rear = arc(ARM_REAR + ROLL_R, ROLL_Y, ROLL_R, 90, 180, n)
    return [(ARM_REAR, bottom), (0.90, bottom), (0.912, bottom + 0.03), (ARM_FRONT, bottom + 0.09)] + front + rear


def seat_profile(scale):
    """💺 Side profile of the sagging seat cushion in seat-bone coordinates."""
    world = [(0.88, 0.36), (0.897, 0.37), (0.905, 0.39), (0.905, 0.44), (0.899, 0.462), (0.886, 0.472), (0.862, 0.476), (0.80, 0.474), (0.72, 0.465), (0.64, 0.452), (0.56, 0.445), (0.48, 0.447), (0.41, 0.454), (0.36, 0.458), (0.34, 0.452), (0.332, 0.44), (0.33, 0.42), (0.33, 0.36)]
    if scale >= 200:
        world = [(0.88, 0.36), (0.905, 0.385), (0.905, 0.45), (0.875, 0.476), (0.72, 0.466), (0.56, 0.447), (0.40, 0.455), (0.34, 0.452), (0.33, 0.36)]
    if scale >= 500:
        world = [(0.905, 0.36), (0.905, 0.476), (0.56, 0.447), (0.33, 0.456), (0.33, 0.36)]
    return [(x - SEAT_X, y - SEAT_Y) for x, y in world]


def back_profile(scale):
    """🛋 Side profile of the waterfall back cushion in back-bone coordinates (v < 0 is the sitter's side)."""
    if scale >= 500:
        return [(-0.02, 0.04), (-0.02, -0.14), (0.10, -0.20), (0.50, -0.205), (0.603, -0.10), (0.60, 0.04)]
    if scale >= 200:
        return [(-0.02, 0.04), (-0.02, -0.13), (0.05, -0.19), (0.20, -0.18), (0.30, -0.205), (0.40, -0.185), (0.50, -0.205), (0.585, -0.15), (0.603, -0.06), (0.585, 0.04)]
    return [(-0.02, 0.04), (-0.02, -0.12), (0.0, -0.16), (0.05, -0.19), (0.10, -0.20), (0.15, -0.195), (0.20, -0.17), (0.25, -0.196), (0.30, -0.206), (0.35, -0.199), (0.40, -0.172), (0.45, -0.192), (0.50, -0.198), (0.55, -0.186), (0.585, -0.15), (0.60, -0.10), (0.603, -0.04), (0.60, 0.0), (0.59, 0.03), (0.57, 0.04)]


def back_front_outline(scale, half):
    """🛋 Front outline of the back cushion in (u, w) of the back bone, rounded at the top corners."""
    r = 0.09 if scale <= 200 else 0.0
    n = 6 if scale <= 100 else 3
    top = BACK_FRONT_TOP
    if r == 0:
        return [(-0.02, -half), (top, -half), (top, half), (-0.02, half)]
    return [(-0.02, -half)] + arc(top - r, -half + r, r, -90, 0, n) + arc(top - r, half - r, r, 0, 90, n) + [(-0.02, half)]


def footrest_board():
    """🦶 Plywood footrest board in footrest-bone coordinates (u down the stowed board, v forward)."""
    return [(0.0, 0.0), (BOARD_LEN, 0.0), (BOARD_LEN, 0.018), (0.0, 0.018)]


def footrest_pad(scale):
    """🦶 Padded footrest cushion profile on the board in footrest-bone coordinates."""
    if scale >= 200:
        return [(0.0, 0.0), (BOARD_LEN, 0.0), (BOARD_LEN, 0.07), (BOARD_LEN - 0.03, 0.09), (0.03, 0.09), (0.0, 0.07)]
    return [(0.005, 0.018), (BOARD_LEN - 0.005, 0.018), (BOARD_LEN, 0.03), (BOARD_LEN - 0.004, 0.065), (BOARD_LEN - 0.02, 0.084), (BOARD_LEN - 0.05, 0.09), (0.05, 0.09), (0.02, 0.084), (0.004, 0.065), (0.0, 0.03)]


def link_bar(width):
    """🔩 Rounded steel link bar along the link bone from the pivot to the footrest hinge."""
    h = width / 2
    return arc(LINK_LEN, 0.0, h, -90, 90, 6) + arc(0.0, 0.0, h, 90, 270, 6)


def lever_handle(scale):
    """🎚 Lever handle in lever-bone coordinates: a flat bar with a rounded grip knob."""
    if scale >= 100:
        return [(-0.02, -0.012), (0.15, -0.015), (0.175, -0.012), (0.18, 0.0), (0.175, 0.014), (0.15, 0.016), (-0.02, 0.012)]
    return [(-0.02, -0.012), (0.13, -0.012), (0.145, -0.017), (0.168, -0.016), (0.18, -0.006), (0.18, 0.008), (0.17, 0.016), (0.145, 0.018), (0.13, 0.012), (-0.02, 0.012)]


def blanket_profile(scale):
    """🧺 Side profile of the folded moving blanket in blanket-bone coordinates (rounded folds front and rear)."""
    if scale >= 200:
        return [(0.0, 0.0), (1.0, 0.0), (1.0, BLANKET), (0.0, BLANKET)]
    return [(0.008, 0.0), (0.992, 0.0)] + arc(0.99, 0.01, 0.01, -90, 90, 4)[1:-1] + [(0.992, BLANKET), (0.008, BLANKET)] + arc(0.01, 0.01, 0.01, 90, 270, 4)[1:-1]


def glide_profile():
    """⚪ Half profile [axial, radial] of a turned polypropylene glide 50 mm across and 30 mm high."""
    return [[0.0, 0.0], [0.0, 0.025], [0.004, 0.025], [0.03, 0.021], [0.03, 0.0]]


def T(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


MATERIALS = [
    {"id": "pvc-faux-leather", "title": T("Aged PVC faux leather on knitted backing", "Gealtertes PVC-Kunstleder auf Wirkträger"), "emissivity": 0.93, "solarAbsorptivity": 0.82, "densityKgM3": 1390, "conductivityWmK": 0.17, "specificHeatJKgK": 900, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "pu-foam", "title": T("Flexible PU foam 35 kg/m3, aged", "PU-Weichschaum 35 kg/m3, gealtert"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 35, "conductivityWmK": 0.04, "specificHeatJKgK": 1400, "elasticModulusPa": 25000.0, "strengthPa": 4000.0, "source": "authored-estimate", "reference": "iso-3386-1"},
    {"id": "polyester-fibre", "title": T("Polyester fibre fill of the back cushion", "Polyesterfaserfüllung des Rückenkissens"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 20, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "chipboard-p2", "title": T("Chipboard P2 (EN 312), black covered", "Spanplatte P2 (EN 312), schwarz bezogen"), "emissivity": 0.9, "solarAbsorptivity": 0.85, "densityKgM3": 650, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 1800000000.0, "strengthPa": 11500000.0, "source": "literature", "reference": "en-312-2010"},
    {"id": "spruce-c24", "title": T("Spruce frame timber C24 (EN 338)", "Fichten-Rahmenholz C24 (EN 338)"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 420, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "literature", "reference": "en-338-2016"},
    {"id": "birch-plywood", "title": T("Birch plywood 18 mm", "Birkensperrholz 18 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 680, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 9000000000.0, "strengthPa": 40000000.0, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "steel-black", "title": T("Black-lacquered steel S235, scratched", "Schwarz lackierter Stahl S235, zerkratzt"), "emissivity": 0.88, "solarAbsorptivity": 0.9, "densityKgM3": 7850, "conductivityWmK": 50.0, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "literature", "reference": "en-1993-1-1"},
    {"id": "polypropylene", "title": T("Polypropylene (glides, lever handle)", "Polypropylen (Gleiter, Hebelgriff)"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 905, "conductivityWmK": 0.22, "specificHeatJKgK": 1700, "elasticModulusPa": 1300000000.0, "strengthPa": 30000000.0, "source": "literature", "reference": "ashby-2011"},
    {"id": "moving-blanket", "title": T("Moving blanket, recycled cotton felt, folded fourfold", "Möbeldecke aus Recycling-Baumwollvlies, vierfach gefaltet"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 90, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "elasticModulusPa": 50000.0, "strengthPa": 20000.0, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "parcel-tape", "title": T("Brown polypropylene parcel tape", "Braunes Polypropylen-Packband"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 905, "conductivityWmK": 0.22, "specificHeatJKgK": 1700, "source": "literature", "reference": "ashby-2011"},
]

MASS = {
    "blanket": (1.8, 0.5, 0.01),
    "frame": (30.52, 0.46, 0.29),
    "seat": (1.7, 0.29, 0.05),
    "back": (3.9, 0.29, -0.07),
    "link": (1.14, LINK_LEN / 2, 0.0),
    "footrest": (2.14, BOARD_LEN / 2, 0.04),
    "lever": (0.13, 0.06, 0.0),
}

REGION_OF = {"blanket": "blanket", "glide": "plastic-trim", "lever": "plastic-trim", "arm": "arm-covers", "seat-frame": "base-frame", "rear-rail": "base-frame", "pivot-tube": "mechanism", "hinge-bracket": "mechanism", "link": "mechanism", "seat-cushion": "seat-cushion", "back-cushion": "back-cushion", "footrest": "footrest"}


def region_of(solid_id):
    """🌡 The thermal region of a body solid, by the prefix of its id."""
    for prefix in sorted(REGION_OF, key=len, reverse=True):
        if solid_id.startswith(prefix):
            return REGION_OF[prefix]
    raise KeyError(solid_id)


def solid(sid, bone, material, tone, kind, role="body", **fields):
    """🧊 One volume solid."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def ext(sid, bone, material, tone, profile, z0, z1, role="body", **flags):
    """📐 An extrusion of a bone-plane (u, v) profile along w from z0 to z1."""
    return solid(sid, bone, material, tone, "extrude", role, profile=pts(profile), plane="xy", **{"from": r5(z0), "to": r5(z1)}, **flags)


def box(sid, bone, material, tone, x0, y0, z0, x1, y1, z1, role="body", **flags):
    """📦 An axis-aligned box in bone coordinates given by two corners."""
    return solid(sid, bone, material, tone, "box", role, size=[r5(x1 - x0), r5(y1 - y0), r5(z1 - z0)], transform={"translation": [r5((x0 + x1) / 2), r5((y0 + y1) / 2), r5((z0 + z1) / 2)]}, **flags)


def cap(sid, bone, material, tone, p0, p1, r, role="body", **flags):
    """💊 A capsule between two bone-local points."""
    return solid(sid, bone, material, tone, "capsule", role, p0=[r5(c) for c in p0], p1=[r5(c) for c in p1], r0=r5(r), r1=r5(r), **flags)


def glide(sid, x, z, scale):
    """⚪ A turned glide under an arm box."""
    profile = glide_profile() if scale == 50 else [[0.0, 0.0], [0.0, 0.025], [0.03, 0.022], [0.03, 0.0]]
    return solid(sid, "glides", "polypropylene", 0.15, "revolve", profile=profile, axis=[0.0, 1.0, 0.0], origin=[r5(x), 0.0, r5(z)])


def tape(scale):
    """🩹 Brown parcel tape wrapped over the crack on the top of the right arm roll (a texture band)."""
    ring = []
    for i in range(9):
        p = -0.38 + 0.76 * i / 8
        ring.append((ROLL_Y + (ROLL_R + 0.0005) * math.cos(p), -ARM_MID + (ROLL_R + 0.0005) * math.sin(p)))
    for i in range(9):
        p = 0.38 - 0.76 * i / 8
        ring.append((ROLL_Y + (ROLL_R - 0.0015) * math.cos(p), -ARM_MID + (ROLL_R - 0.0015) * math.sin(p)))
    return solid("repair-tape", "frame", "parcel-tape", 0.62, "extrude", "texture", profile=pts(ring), plane="yz", **{"from": 0.70, "to": 0.80}, texture=True)


def body_solids(scale):
    """🧱 The body list of one scale: the chair as the depot stores it, each scale modelled for its own smallest feature."""
    f = scale * 0.00015
    out = []
    out.append(ext("blanket", "blanket", "moving-blanket", 0.78, blanket_profile(scale), -0.45, 0.45, **({"sheet": True} if scale >= 200 else {})))
    for name, (x, z) in zip(["glide-rear-left", "glide-rear-right", "glide-front-left", "glide-front-right"], GLIDES):
        out.append(glide(name, x, z, scale))
    for side, (z0, z1) in (("right", (-ARM_OUT, -ARM_IN)), ("left", (ARM_IN, ARM_OUT))):
        zc = (z0 + z1) / 2
        if scale == 500:
            out.append(ext(f"arm-{side}", "frame", "pvc-faux-leather", 0.38, arm_outline(500), z0, z1))
        else:
            out.append(ext(f"arm-{side}-box", "frame", "pvc-faux-leather", 0.38, arm_side(scale), z0, z1))
            out.append(cap(f"arm-{side}-roll", "frame", "pvc-faux-leather", 0.36, (ARM_REAR + ROLL_R, ROLL_Y, zc), (ARM_FRONT - ROLL_R, ROLL_Y, zc), ROLL_R))
    out.append(box("seat-frame", "frame", "chipboard-p2", 0.2, 0.20, 0.22, -ARM_IN, 0.74, 0.36, ARM_IN))
    if scale <= 100:
        out.append(box("rear-rail", "frame", "spruce-c24", 0.22, 0.02, 0.06, -ARM_IN, 0.05, 0.20, ARM_IN))
    out.append(cap("pivot-tube", "frame", "steel-black", 0.25, (LINK_PIVOT[0], LINK_PIVOT[1], -ARM_IN), (LINK_PIVOT[0], LINK_PIVOT[1], ARM_IN), 0.0125, **({"slender": True} if scale >= 200 else {})))
    if scale <= 100:
        inner = 0.25 if scale == 50 else 0.245
        outer = 0.2595 if scale == 50 else ARM_IN
        for side, (z0, z1) in (("left", (inner, outer)), ("right", (-outer, -inner))):
            out.append(box(f"hinge-bracket-{side}", "frame", "steel-black", 0.25, 0.09, 0.36, z0, 0.21, 0.46, z1))
    out.append(ext("seat-cushion", "seat", "pvc-faux-leather", 0.42, seat_profile(scale), -0.258, 0.258))
    half = BACK_HALF[scale]
    out.append(ext("back-cushion", "back", "pvc-faux-leather", 0.44, back_profile(scale), -half, half))
    if scale <= 100:
        out.append(ext("footrest-board", "footrest", "birch-plywood", 0.3, footrest_board(), -0.245, 0.245))
        out.append(ext("footrest-pad", "footrest", "pvc-faux-leather", 0.42, footrest_pad(scale), -0.24, 0.24))
        lz = (0.246, 0.254) if scale == 50 else (0.245, 0.26)
        out.append(solid("link-plates", "link", "steel-black", 0.25, "extrude", profile=pts(link_bar(0.036)), plane="xy", **{"from": lz[0], "to": lz[1]}, repeat={"count": 2, "translation": [0.0, 0.0, r5(-(lz[0] + lz[1]))]}))
        out.append(ext("lever-handle", "lever", "polypropylene", 0.18, lever_handle(scale), -0.445, -0.43))
    else:
        out.append(ext("footrest-pad", "footrest", "pvc-faux-leather", 0.42, footrest_pad(scale), -0.257, 0.257))
    if scale == 50:
        out.append(solid("lever-boss", "lever", "steel-black", 0.25, "extrude", profile=pts(arc(0.0, 0.0, 0.02, 0, 360, 16)[:-1]), plane="xy", **{"from": -0.45, "to": -0.43}))
    if scale <= 100:
        out.append(tape(scale))
    return out


def thermal_solids(scale, body):
    """🌡 The thermal list: every body surface as a reference bound to its region (the tape is a texture of the arm cover)."""
    return [{"id": f"th-{s['id']}", "of": s["id"], "region": region_of(s["id"])} for s in body if not s.get("texture")]


def structure_solids(scale, body):
    """🏗 The structure list: the real load path of the chair (frame panels and rails, seat, back frame, linkage, footrest board, lever), joints, supports and masses."""
    f = scale * 0.00015
    ids = {s["id"] for s in body}
    out = [{"id": "m-blanket", "of": "blanket", "member": "blanket", "part": "member"}]
    for s in body:
        if s["id"].startswith("glide-"):
            out.append({"id": f"m-{s['id']}", "of": s["id"], "member": "glides", "part": "member"})
    for side, sign in (("left", 1), ("right", -1)):
        if scale <= 100:
            z = sorted([sign * ARM_IN, sign * (ARM_IN + 0.012)])
            out.append(dict(box(f"m-arm-panel-{side}", "frame", "chipboard-p2", 0.3, 0.03, 0.03, z[0], 0.90, 0.52, z[1], **({"sheet": True} if scale >= 100 else {})), member="frame", part="member"))
            zr = sorted([sign * ARM_IN, sign * ARM_OUT])
            out.append(dict(box(f"m-arm-base-rail-{side}", "frame", "spruce-c24", 0.3, 0.02, 0.03, zr[0], 0.90, 0.052, zr[1]), member="frame", part="member"))
            zt = sorted([sign * (ARM_IN + 0.012), sign * (ARM_IN + 0.052)])
            out.append(dict(box(f"m-arm-top-rail-{side}", "frame", "spruce-c24", 0.3, 0.03, 0.46, zt[0], 0.90, 0.52, zt[1]), member="frame", part="member"))
        else:
            t = 0.04 if scale == 200 else 0.08
            z = sorted([sign * ARM_IN, sign * (ARM_IN + t)])
            out.append(dict(box(f"m-arm-frame-{side}", "frame", "chipboard-p2", 0.3, 0.02, 0.0, z[0], 0.90, 0.52, z[1]), member="frame", part="member"))
    out.append({"id": "m-seat-frame", "of": "seat-frame", "member": "frame", "part": "member"})
    if "rear-rail" in ids:
        out.append({"id": "m-rear-rail", "of": "rear-rail", "member": "frame", "part": "member"})
    out.append({"id": "m-pivot-tube", "of": "pivot-tube", "member": "frame", "part": "member"})
    out.append({"id": "m-seat", "of": "seat-cushion", "member": "seat", "part": "member"})
    st = max(0.04, f) if scale < 500 else 0.04
    stile = dict(slender=True) if scale == 500 else {}
    for side, sign in (("left", 1), ("right", -1)):
        zs = sorted([sign * 0.21, sign * 0.25])
        out.append(dict(box(f"m-back-stile-{side}", "back", "spruce-c24", 0.3, -0.02, 0.0, zs[0], 0.56, st, zs[1], **stile), member="back", part="member"))
    if scale <= 200:
        out.append(dict(box("m-back-top-rail", "back", "spruce-c24", 0.3, 0.52, 0.0, -0.21, 0.56, st, 0.21), member="back", part="member"))
        out.append(dict(box("m-back-bottom-rail", "back", "spruce-c24", 0.3, -0.02, 0.0, -0.21, 0.02, st, 0.21), member="back", part="member"))
    out.append(dict(solid("m-link-plates", "link", "steel-black", 0.25, "extrude", profile=pts(link_bar(0.036)), plane="xy", **{"from": 0.246, "to": 0.254}, repeat={"count": 2, "translation": [0.0, 0.0, -0.5]}, **({"slender": True} if scale >= 100 else {})), member="link", part="member"))
    if scale <= 100:
        out.append({"id": "m-footrest-board", "of": "footrest-board", "member": "footrest", "part": "member"})
        out.append({"id": "m-lever", "of": "lever-handle", "member": "lever", "part": "member"})
    else:
        out.append({"id": "m-footrest", "of": "footrest-pad", "member": "footrest", "part": "member"})
        out.append(dict(ext("m-lever", "lever", "polypropylene", 0.18, lever_handle(100), -0.445, -0.43, slender=True), member="lever", part="member"))
    jr = max(0.008, f / 2)
    out.append(dict(cap("j-back-pivot", "back", "steel-black", 0.4, (0.0, 0.0, -0.2595), (0.0, 0.0, 0.2595), jr, role="detail"), member="back", part="joint"))
    out.append(dict(cap("j-link-pivot", "link", "steel-black", 0.4, (0.0, 0.0, -0.255), (0.0, 0.0, 0.255), jr, role="detail"), member="link", part="joint"))
    out.append(dict(cap("j-footrest-hinge", "footrest", "steel-black", 0.4, (0.0, 0.0, -0.255), (0.0, 0.0, 0.255), jr, role="detail"), member="footrest", part="joint"))
    out.append(dict(cap("j-lever-shaft", "lever", "steel-black", 0.4, (0.0, 0.0, -0.445 if scale <= 100 else -ARM_OUT), (0.0, 0.0, -0.27), jr, role="detail"), member="lever", part="joint"))
    sj = max(f, 0.01)
    out.append(dict(box("j-seat-on-frame", "seat", "pu-foam", 0.4, 0.10, -sj, -0.2, 0.40, 0.0, 0.2, role="detail"), member="seat", part="joint"))
    s = max(f, 0.015)
    for a in ANCHORS:
        if a["id"] in STRUCTURE_SUPPORTS:
            x, y, z = a["x"], a["y"], a.get("z", 0.0)
            dy = s / 2
            out.append(dict(box(f"support-{a['id']}", a["bone"], "polypropylene", 0.15, x - s / 2, y + dy - s / 2, z - s / 2, x + s / 2, y + dy + s / 2, z + s / 2, role="ground"), member=a["bone"] if a["bone"] != "glides" else "glides", part="support", anchor=a["id"]))
    mr = max(f / 2, 0.012)
    for bone, (kg, cx, cy) in MASS.items():
        out.append(dict(solid(f"mass-{bone}", bone, "steel-black", 0.4, "ellipsoid", "detail", radii=[r5(mr), r5(mr), r5(mr)], transform={"translation": [r5(cx), r5(cy), 0.0]}), member=bone, part="mass"))
    return out


def anchor(aid, bone, world_xy_or_local, z, kind, en, de, local=False, **extra):
    """📍 An anchor; points are given in world rest coordinates unless `local`."""
    u, v = world_xy_or_local if local else to_local(bone, [world_xy_or_local])[0]
    out = {"id": aid, "bone": bone, "x": r5(u), "y": r5(v), "z": r5(z), "kind": kind, "title": T(en, de)}
    out.update(extra)
    return out


ANCHORS = []
for name, (x, z), en, de in zip(["rear-left", "rear-right", "front-left", "front-right"], GLIDES, ["rear left", "rear right", "front left", "front right"], ["hinten links", "hinten rechts", "vorn links", "vorn rechts"]):
    ANCHORS.append(anchor(f"blanket-{name}", "blanket", (x, 0.0), z, "contact", f"Blanket on the floor under the {en} glide", f"Decke auf dem Boden unter dem Gleiter {de}", support="contact", normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.05, frictionCoefficient=0.45))
for name, (x, z), en, de in zip(["rear-left", "rear-right", "front-left", "front-right"], GLIDES, ["rear left", "rear right", "front left", "front right"], ["hinten links", "hinten rechts", "vorn links", "vorn rechts"]):
    ANCHORS.append(anchor(f"glide-{name}", "glides", (x, 0.0), z, "contact", f"Glide {en} on the floor", f"Gleiter {de} auf dem Boden", local=True, support="contact", normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.05, frictionCoefficient=0.3))
for name, x, z, en, de in (("grip-rear-left", 0.08, ARM_MID, "rear left", "hinten links"), ("grip-rear-right", 0.08, -ARM_MID, "rear right", "hinten rechts"), ("grip-front-left", 0.84, ARM_MID, "front left", "vorn links"), ("grip-front-right", 0.84, -ARM_MID, "front right", "vorn rechts")):
    ANCHORS.append(anchor(name, "frame", (x, 0.03), z, "contact", f"Carrier's hand under the {en} arm box", f"Hand eines Trägers unter dem Armkasten {de}", local=True, support="contact", normalDeg=90, footprintLengthM=0.09, footprintWidthM=0.08, frictionCoefficient=0.6))
ANCHORS.append(anchor("seat-load", "seat", (0.25, 0.088), 0.0, "load", "Sitting point on the sagging seat", "Sitzpunkt auf dem durchgesessenen Sitz", local=True))
ANCHORS.append(anchor("back-load", "back", (0.27, -0.205), 0.0, "load", "Shoulder and lumbar contact on the back cushion", "Schulter- und Lendenkontakt am Rückenkissen", local=True))
ANCHORS.append(anchor("legs-load", "footrest", (0.17, 0.09), 0.0, "load", "Calves on the footrest", "Waden auf der Fußstütze", local=True))
SUPPORTS = [a["id"] for a in ANCHORS if a.get("support")]
STRUCTURE_SUPPORTS = []


def shape(sid, bone, role, z, geom, tone, filled=True, region=None, material=None, **extra):
    """✏️ One 2D shape of a representation."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    if region:
        out["region"] = region
    if material:
        out["material"] = material
    out.update(extra)
    return out


def poly(points, closed=True):
    """🔷 A poly geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rect_poly(x0, y0, x1, y1, r=0.0, n=4):
    """⬛ A (rounded) rectangle as a poly geometry."""
    return poly(rounded_rect(x0, y0, x1, y1, r, n))


def sweep_w(z0, z1):
    """🌀 An extruded solid placement swept laterally from z0 to z1 (rest asset coordinates)."""
    return {"kind": "extruded", "sweep": "w", "z0": r5(z0), "z1": r5(z1)}


def sweep_v(v0, v1):
    """🌀 An extruded solid placement swept along the bone normal from v0 to v1."""
    return {"kind": "extruded", "sweep": "v", "z0": r5(v0), "z1": r5(v1)}


def crack_lines(scale):
    """⚡ Craquelure of the dried-out faux leather on the front of the right arm roll, as (x, y) chair polylines seen from the right."""
    if scale == 50:
        return [[(0.705, 0.608), (0.73, 0.602), (0.752, 0.609), (0.78, 0.600), (0.81, 0.606), (0.84, 0.598), (0.87, 0.590)], [(0.82, 0.612), (0.85, 0.604), (0.875, 0.598), (0.895, 0.585)]]
    return [[(0.705, 0.606), (0.76, 0.600), (0.82, 0.604), (0.88, 0.590)]]


def crack_plan(scale):
    """⚡ Craquelure on the top of the right arm roll in plan (x, z)."""
    if scale == 50:
        return [[(0.72, -0.36), (0.745, -0.332), (0.77, -0.352), (0.80, -0.326), (0.83, -0.348), (0.86, -0.33), (0.885, -0.345)], [(0.79, -0.37), (0.815, -0.352), (0.845, -0.372), (0.87, -0.36)]]
    return [[(0.72, -0.355), (0.79, -0.33), (0.86, -0.35)]]


def geometry(scale):
    """🖼 The three authored views of one scale, drawn to the same numbers as the solids of that scale."""
    fine = scale <= 100
    elev, front, plan = [], [], []
    w = lambda pts_: [(x, y + BLANKET) for x, y in pts_]
    elev.append(shape("blanket", "blanket", "body", 1, poly(blanket_profile(scale)), 0.78, region="blanket", material="moving-blanket"))
    for x, z in GLIDES:
        if z < 0:
            elev.append(shape(f"glide-{'rear' if x < 0.5 else 'front'}", "glides", "body", 2, poly([(x - 0.025, 0.0), (x + 0.025, 0.0), (x + 0.021, 0.03), (x - 0.021, 0.03)]), 0.15, region="plastic-trim", material="polypropylene"))
    elev.append(shape("seat-frame", "frame", "body", 2.2, rect_poly(0.20, 0.22, 0.74, 0.36), 0.2, region="base-frame", material="chipboard-p2"))
    if fine:
        elev.append(shape("rear-rail", "frame", "body", 2.3, rect_poly(0.02, 0.06, 0.05, 0.20), 0.22, region="base-frame", material="spruce-c24"))
    elev.append(shape("pivot-tube", "frame", "body", 2.4, poly(arc(LINK_PIVOT[0], LINK_PIVOT[1], 0.0125, 0, 360, 12)[:-1]), 0.25, region="mechanism", material="steel-black"))
    elev.append(shape("back-cushion", "back", "body", 3, poly(back_profile(scale)), 0.44, region="back-cushion", material="pvc-faux-leather"))
    if scale <= 200:
        for i, u in enumerate((0.20, 0.40)):
            elev.append(shape(f"back-seam-{i + 1}", "back", "detail", 3.5, poly([(u, -0.168), (u, 0.035)], False), 0.3, filled=False, region="back-cushion"))
    elev.append(shape("seat-cushion", "seat", "body", 4, poly(seat_profile(scale)), 0.42, region="seat-cushion", material="pvc-faux-leather"))
    if fine:
        elev.append(shape("link-plates", "link", "body", 4.2, poly(link_bar(0.036)), 0.25, region="mechanism", material="steel-black"))
        elev.append(shape("footrest-board", "footrest", "body", 4.4, poly(footrest_board()), 0.3, region="footrest", material="birch-plywood"))
    elev.append(shape("footrest-pad", "footrest", "body", 4.5, poly(footrest_pad(scale)), 0.42, region="footrest", material="pvc-faux-leather"))
    elev.append(shape("arm-right", "frame", "body", 6, poly(arm_outline(scale)), 0.38, region="arm-covers", material="pvc-faux-leather"))
    if scale <= 200:
        elev.append(shape("arm-piping", "frame", "detail", 6.5, poly([(ARM_REAR + 0.05, 0.47), (ARM_FRONT - 0.05, 0.47)], False), 0.3, filled=False, region="arm-covers"))
        elev.append(shape("arm-front-seam", "frame", "detail", 6.5, poly([(0.888, 0.12), (0.888, 0.45)], False), 0.3, filled=False, region="arm-covers"))
    if fine:
        for i, line in enumerate(crack_lines(scale)):
            elev.append(shape(f"arm-crack-{i + 1}", "frame", "texture", 7, poly(line, False), 0.2, filled=False, region="arm-covers"))
    if scale == 50:
        elev.append(shape("repair-tape", "frame", "texture", 7.5, rect_poly(0.70, ROLL_Y + ROLL_R * math.cos(0.38), 0.80, ROLL_Y + ROLL_R + 0.0005), 0.62, region="arm-covers", material="parcel-tape"))
    if fine:
        elev.append(shape("lever-handle", "lever", "body", 8, poly(lever_handle(scale)), 0.18, region="plastic-trim", material="polypropylene"))
    if scale == 50:
        elev.append(shape("lever-boss", "lever", "body", 8.5, poly(arc(0.0, 0.0, 0.02, 0, 360, 16)[:-1]), 0.25, region="mechanism", material="steel-black"))

    front.append(shape("blanket", "blanket", "body", 1, rect_poly(-0.45, 0.0, 0.45, BLANKET), 0.78, region="blanket", material="moving-blanket"))
    for x, z in GLIDES:
        if x > 0.5:
            front.append(shape(f"glide-{'left' if z > 0 else 'right'}", "glides", "body", 2, poly([(z - 0.025, BLANKET), (z + 0.025, BLANKET), (z + 0.021, BLANKET + 0.03), (z - 0.021, BLANKET + 0.03)]), 0.15, region="plastic-trim", material="polypropylene"))
    front.append(shape("seat-frame", "frame", "body", 2.5, rect_poly(-ARM_IN, 0.24, ARM_IN, 0.38), 0.2, region="base-frame", material="chipboard-p2"))
    if fine:
        front.append(shape("pivot-tube", "frame", "body", 2.6, rect_poly(-ARM_IN, LINK_PIVOT[1] + BLANKET - 0.0125, ARM_IN, LINK_PIVOT[1] + BLANKET + 0.0125), 0.25, region="mechanism", material="steel-black"))
    half = BACK_HALF[scale]
    front.append(shape("back-cushion", "back", "body", 3, poly(back_front_outline(scale, half)), 0.44, region="back-cushion", material="pvc-faux-leather", solid=sweep_v(-0.206, 0.04)))
    if scale <= 200:
        for i, u in enumerate((0.20, 0.40)):
            a, b = to_world("back", [(u, -0.17), (u, -0.17)])
            front.append(shape(f"back-seam-{i + 1}", "back", "detail", 3.5, poly([(-half + 0.03, a[1]), (half - 0.03, b[1])], False), 0.3, filled=False, region="back-cushion"))
    front.append(shape("seat-cushion", "seat", "body", 4, rect_poly(-ARM_IN, 0.38, ARM_IN, 0.496, 0.03 if fine else 0.0), 0.42, region="seat-cushion", material="pvc-faux-leather"))
    if scale <= 200:
        front.append(shape("seat-piping", "seat", "detail", 4.1, poly([(-0.24, 0.47), (0.24, 0.47)], False), 0.3, filled=False, region="seat-cushion"))
    if fine:
        front.append(shape("footrest-board", "footrest", "body", 4.4, rect_poly(0.0, -0.245, BOARD_LEN, 0.245), 0.3, region="footrest", material="birch-plywood", solid=sweep_v(0.0, 0.018)))
        front.append(shape("footrest-pad", "footrest", "body", 4.5, rect_poly(0.005, -0.24, BOARD_LEN - 0.005, 0.24, 0.03), 0.42, region="footrest", material="pvc-faux-leather", solid=sweep_v(0.018, 0.09)))
        for side, (z0, z1) in (("left", (0.245, 0.26)), ("right", (-0.26, -0.245))):
            front.append(shape(f"link-{side}", "link", "body", 4.6, poly(link_bar(0.036)), 0.25, region="mechanism", material="steel-black", solid=sweep_w(z0, z1)))
    else:
        front.append(shape("footrest-pad", "footrest", "body", 4.5, rect_poly(0.0, -0.257, BOARD_LEN, 0.257), 0.42, region="footrest", material="pvc-faux-leather", solid=sweep_v(0.0, 0.09)))
    for side, (z0, z1) in (("right", (-ARM_OUT, -ARM_IN)), ("left", (ARM_IN, ARM_OUT))):
        zc = (z0 + z1) / 2
        bottom = BLANKET if scale >= 200 else BLANKET + 0.03
        n = 10 if fine else 5
        if scale == 500:
            outline = [(z0, bottom), (z1, bottom), (z1, ROLL_Y + ROLL_R + BLANKET), (z0, ROLL_Y + ROLL_R + BLANKET)]
        else:
            outline = [(z0, bottom), (z1, bottom)] + arc(zc, ROLL_Y + BLANKET, ROLL_R, 0, 180, n)
        front.append(shape(f"arm-{side}", "frame", "body", 6, poly(outline), 0.38, region="arm-covers", material="pvc-faux-leather"))
        if scale <= 200:
            front.append(shape(f"arm-piping-{side}", "frame", "detail", 6.5, poly([(z0 + 0.012, 0.49), (z1 - 0.012, 0.49)], False), 0.3, filled=False, region="arm-covers"))
    if fine:
        front.append(shape("lever-handle", "lever", "body", 8, poly(lever_handle(scale)), 0.18, region="plastic-trim", material="polypropylene", solid=sweep_w(-0.445, -0.43)))
    if scale == 50:
        front.append(shape("lever-boss", "lever", "body", 8.5, poly(arc(0.0, 0.0, 0.02, 0, 360, 16)[:-1]), 0.25, region="mechanism", material="steel-black", solid=sweep_w(-0.45, -0.43)))

    hatch = {"hatch": "quilting"} if fine else {}
    plan.append(shape("blanket", "blanket", "body", 1, rect_poly(-0.04, -0.45, 0.96, 0.45), 0.78, region="blanket", material="moving-blanket", **hatch))
    if fine:
        plan.append(shape("footrest-board", "footrest", "body", 2, poly(footrest_board()), 0.3, region="footrest", material="birch-plywood", solid=sweep_w(-0.245, 0.245)))
        plan.append(shape("footrest-pad", "footrest", "body", 2.1, poly(footrest_pad(scale)), 0.42, region="footrest", material="pvc-faux-leather", solid=sweep_w(-0.24, 0.24)))
        for side, (z0, z1) in (("left", (0.246, 0.254)), ("right", (-0.254, -0.246))):
            plan.append(shape(f"link-{side}", "link", "body", 2.2, poly(link_bar(0.036)), 0.25, region="mechanism", material="steel-black", solid=sweep_w(z0, z1)))
    else:
        plan.append(shape("footrest-pad", "footrest", "body", 2.1, poly(footrest_pad(scale)), 0.42, region="footrest", material="pvc-faux-leather", solid=sweep_w(-0.257, 0.257)))
    plan.append(shape("seat-cushion", "seat", "body", 3, poly([(SEAT_X, -ARM_IN)] + arc(0.905 - 0.03, -ARM_IN + 0.03, 0.03, -90, 0, 4) + arc(0.905 - 0.03, ARM_IN - 0.03, 0.03, 0, 90, 4) + [(SEAT_X, ARM_IN)]) if fine else rect_poly(SEAT_X, -ARM_IN, 0.905, ARM_IN), 0.42, region="seat-cushion", material="pvc-faux-leather"))
    if scale <= 200:
        plan.append(shape("seat-piping", "seat", "detail", 3.1, poly([(0.88, -0.24), (0.88, 0.24)], False), 0.3, filled=False, region="seat-cushion"))
    plan.append(shape("back-cushion", "back", "body", 4, poly(back_profile(scale)), 0.44, region="back-cushion", material="pvc-faux-leather", solid=sweep_w(-half, half)))
    for side, (z0, z1) in (("right", (-ARM_OUT, -ARM_IN)), ("left", (ARM_IN, ARM_OUT))):
        plan.append(shape(f"arm-{side}", "frame", "body", 5, rect_poly(ARM_REAR, z0, ARM_FRONT, z1), 0.38, region="arm-covers", material="pvc-faux-leather"))
    if fine:
        for i, line in enumerate(crack_plan(scale)):
            plan.append(shape(f"arm-crack-{i + 1}", "frame", "texture", 6, poly(line, False), 0.2, filled=False, region="arm-covers"))
        plan.append(shape("repair-tape", "frame", "texture", 6.5, rect_poly(0.70, -ARM_MID - ROLL_R * math.sin(0.38), 0.80, -ARM_MID + ROLL_R * math.sin(0.38)), 0.62, region="arm-covers", material="parcel-tape"))
        plan.append(shape("lever-handle", "lever", "body", 7, poly(lever_handle(scale)), 0.18, region="plastic-trim", material="polypropylene", solid=sweep_w(-0.445, -0.43)))
    return {"elevation": elev, "front": front, "plan": plan}


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {50: [{"id": "quilting", "title": T("Quilted stitching of the moving blanket", "Steppnähte der Möbeldecke"), "pattern": "cross", "angleDeg": 45, "spacingMm": 2.0, "penMm": 0.13}], 100: [{"id": "quilting", "title": T("Quilted stitching of the moving blanket", "Steppnähte der Möbeldecke"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.0, "penMm": 0.13}], 200: [], 500: []}


def layer(lid, en, de, kind, material, resistance, capacity, thickness=None, source="authored-estimate", reference="iso-10456-2007"):
    """🧶 One thermal layer."""
    out = {"id": lid, "title": T(en, de), "kind": kind, "material": material, "resistanceM2KW": resistance, "heatCapacityJM2K": capacity, "source": source, "reference": reference}
    if thickness is not None:
        out["thicknessM"] = thickness
    return out


def region(rid, en, de, kind, material, area, length, geometry_, sky, layers, capacity, **extra):
    """🔥 One passive thermal region."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": length, "geometry": geometry_, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": capacity, "source": "authored-estimate", "reference": "iso-10456-2007"}
    out.update(extra)
    return out


AREAS = {"blanket": 1.86, "plastic-trim": 0.050, "arm-covers": 2.95, "base-frame": 1.04, "mechanism": 0.139, "seat-cushion": 0.80, "back-cushion": 1.10, "footrest": 0.394}

REGIONS = [
    region("arm-covers", "Faux-leather arm boxes with the cracked, taped right roll", "Kunstleder-Armkästen mit der rissigen, geklebten rechten Rolle", "surface", "pvc-faux-leather", AREAS["arm-covers"], 0.55, "vertical-plate", 0.4,
           [layer("arm-upholstery", "Faux leather 1 mm over 30 mm PU foam", "Kunstleder 1 mm über 30 mm PU-Schaum", "polymer", "pvc-faux-leather", 0.756, 2510)], 2510, ratedPowerW=0, ratedPowerBasis="passive"),
    region("seat-cushion", "Sagging seat cushion", "Durchgesessenes Sitzkissen", "surface", "pvc-faux-leather", AREAS["seat-cushion"], 0.55, "horizontal-plate-up", 0.6,
           [layer("seat-upholstery", "Faux leather 1 mm over 100 mm PU foam, compressed by 25 mm of use", "Kunstleder 1 mm über 100 mm PU-Schaum, um 25 mm durchgesessen", "polymer", "pvc-faux-leather", 1.882, 2730)], 2730),
    region("back-cushion", "Waterfall back cushion with the faded headrest pillow", "Wasserfall-Rückenkissen mit ausgeblichenem Kopfkissen", "surface", "pvc-faux-leather", AREAS["back-cushion"], 0.6, "vertical-plate", 0.5,
           [layer("back-upholstery", "Faux leather 1 mm over polyester fibre fill", "Kunstleder 1 mm über Polyesterfaserfüllung", "polymer", "pvc-faux-leather", 2.006, 2340)], 2340),
    region("footrest", "Padded footrest on its plywood board", "Gepolsterte Fußstütze auf ihrem Sperrholzbrett", "surface", "pvc-faux-leather", AREAS["footrest"], 0.28, "vertical-plate", 0.4,
           [layer("footrest-upholstery", "Faux leather 1 mm over 70 mm PU foam", "Kunstleder 1 mm über 70 mm PU-Schaum", "polymer", "pvc-faux-leather", 1.756, 2580)], 2580),
    region("base-frame", "Seat box and rear rail, black-covered chipboard and spruce", "Sitzkasten und hintere Zarge, schwarz bezogene Spanplatte und Fichte", "surface", "chipboard-p2", AREAS["base-frame"], 0.5, "vertical-plate", 0.1,
           [layer("chipboard", "Chipboard 18 mm", "Spanplatte 18 mm", "timber", "chipboard-p2", 0.129, 19890, 0.018, "literature", "en-312-2010")], 19890),
    region("mechanism", "Steel linkage, pivot tube and hinge brackets", "Stahlgestänge, Drehrohr und Gelenkwinkel", "surface", "steel-black", AREAS["mechanism"], 0.036, "cylinder", 0.2,
           [layer("steel", "Steel 4 mm, black lacquer", "Stahl 4 mm, schwarz lackiert", "metal", "steel-black", 0.00008, 14130, 0.004, "literature", "en-1993-1-1")], 14130),
    region("plastic-trim", "Polypropylene glides and lever handle", "Polypropylen-Gleiter und Hebelgriff", "surface", "polypropylene", AREAS["plastic-trim"], 0.05, "cylinder", 0.2,
           [layer("pp", "Polypropylene 5 mm", "Polypropylen 5 mm", "polymer", "polypropylene", 0.023, 7690, 0.005, "literature", "ashby-2011")], 7690),
    region("blanket", "Folded moving blanket on the depot floor", "Gefaltete Möbeldecke auf dem Lagerboden", "surface", "moving-blanket", AREAS["blanket"], 0.9, "horizontal-plate-up", 0.7,
           [layer("felt", "Four layers of 5 mm cotton felt", "Vier Lagen Baumwollvlies zu 5 mm", "insulation", "moving-blanket", 0.444, 2340, 0.02)], 2340),
]

REGION_IDS = [r["id"] for r in REGIONS]


def bind(exposed=None, contact=None):
    """🌬 The region binding of a pose: exposed fractions and conductive contacts (indoors, no sun)."""
    exposed = exposed or {}
    contact = contact or {}
    out = {}
    for rid in REGION_IDS:
        b = {"exposedFraction": exposed.get(rid, 1.0), "sunlitFraction": 0.0}
        if rid in contact:
            b["contact"] = {"fraction": contact[rid][0], "tempC": contact[rid][1]}
        out[rid] = b
    return {"regions": out, "emitters": {}}


STORED_EXPOSURE = {"blanket": 0.55, "plastic-trim": 0.6, "base-frame": 0.6, "mechanism": 0.5, "footrest": 0.85}
USE_EXPOSURE = {"blanket": 0.55, "plastic-trim": 0.6, "base-frame": 0.6, "mechanism": 0.5, "footrest": 0.85}
OCCUPIED = {"seat-cushion": (0.35, 32.0), "back-cushion": (0.25, 32.0), "arm-covers": (0.05, 31.0)}
RECLINED = {"seat-cushion": (0.32, 32.0), "back-cushion": (0.3, 32.0), "arm-covers": (0.05, 31.0), "footrest": (0.3, 31.0)}

BLANKET_CONTACTS = [a for a in SUPPORTS if a.startswith("blanket")]
GLIDE_CONTACTS = [a for a in SUPPORTS if a.startswith("glide")]
GRIP_CONTACTS = [a for a in SUPPORTS if a.startswith("grip")]

G = 9.80665
OCCUPANT_KG = 75.0


def load(lid, en, de, anchor_, fx, fy):
    """🧲 An occupant load at a load anchor (EN 1991-1-1 / EN 12520 sitting case, authored split)."""
    return {"id": lid, "kind": "occupant", "title": T(en, de), "anchor": anchor_, "fxN": round(fx, 1), "fyN": round(fy, 1), "source": "authored-estimate", "reference": "en-12520-2015"}


def occupant(seat_n, back_n, back_deg, legs_n):
    """🧍 The three occupant loads of a seated 75 kg adult: seat, back (normal to the back cushion) and calves."""
    bx, by = back_n * math.cos(math.radians(back_deg)), back_n * math.sin(math.radians(back_deg))
    return [load("occupant-seat", "Seated adult on the seat (75 kg person)", "Sitzende erwachsene Person auf dem Sitz (75 kg)", "seat-load", 0.0, -seat_n),
            load("occupant-back", "Upper body leaning on the back", "Oberkörper an der Lehne", "back-load", bx, by),
            load("occupant-legs", "Calves on the footrest", "Waden auf der Fußstütze", "legs-load", 0.0, -legs_n)]


IN_USE_BONES = {"blanket": {"dx": -1.15}, "lift": {"dy": -BLANKET}, "seat": {"dy": -0.03}}
FOOT_OUT = {"link": {"angleDeg": -35.0}, "footrest": {"angleDeg": 130.0}}


def pose(pid, en_t, de_t, en_d, de_d, bones, contacts, thermal, loads, key=False, ground="authored"):
    """📥 One pose."""
    out = {"id": pid, "title": T(en_t, de_t), "description": T(en_d, de_d)}
    if key:
        out["key"] = True
    out.update({"bones": bones, "ground": ground, "contacts": contacts, "thermal": thermal, "loads": loads})
    return out


POSES = [
    pose("stored", "Stored on a moving blanket", "Gelagert auf einer Möbeldecke",
         "As the second-hand store keeps it: back upright, footrest stowed, standing on its four glides on a folded moving blanket on the depot floor, emptied and wiped, the cracked right arm roll taped over.",
         "Wie das Gebrauchtwarenkaufhaus ihn lagert: Lehne aufrecht, Fußstütze eingeklappt, auf seinen vier Gleitern auf einer gefalteten Möbeldecke auf dem Lagerboden, geleert und abgewischt, die rissige rechte Armrolle mit Packband überklebt.",
         {}, BLANKET_CONTACTS, bind(STORED_EXPOSURE), []),
    pose("lift-off-key", "Lifted off the blanket", "Von der Decke gehoben", "Two carriers have taken the chair under the arm boxes and lift it 10 mm off the blanket.", "Zwei Träger haben den Sessel unter den Armkästen gefasst und heben ihn 10 mm von der Decke.",
         {"lift": {"dy": 0.01}}, GRIP_CONTACTS, bind(STORED_EXPOSURE), [], key=True),
    pose("carried", "Carried by two people", "Von zwei Personen getragen",
         "Two carriers hold the chair under the front and rear ends of both arm boxes, lifted 0.22 m and tipped 6 degrees back so the back cushion leans into the rear carrier; the moving blanket stays on the floor.",
         "Zwei Träger halten den Sessel unter den vorderen und hinteren Enden beider Armkästen, 0,22 m angehoben und 6 Grad nach hinten gekippt, sodass das Rückenkissen zum hinteren Träger lehnt; die Möbeldecke bleibt auf dem Boden.",
         {"lift": {"dy": 0.22}, "tilt": {"angleDeg": 6.0}}, GRIP_CONTACTS, bind(STORED_EXPOSURE), []),
    pose("in-use", "In use, upright", "In Gebrauch, aufrecht",
         "Back in a living room on its glides: an adult sits upright, the worn seat sinks another 30 mm, the moving blanket lies folded behind the chair.",
         "Wieder in einem Wohnzimmer auf seinen Gleitern: eine erwachsene Person sitzt aufrecht, der abgenutzte Sitz sinkt weitere 30 mm ein, die Möbeldecke liegt gefaltet hinter dem Sessel.",
         IN_USE_BONES, GLIDE_CONTACTS, bind(USE_EXPOSURE, OCCUPIED), occupant(0.75 * OCCUPANT_KG * G, 110.0, 190.0, 0.0)),
    pose("lever-pulled-key", "Lever pulled", "Hebel gezogen", "The sitter pulls the release lever back by 25 degrees; the footrest is still stowed.", "Die sitzende Person zieht den Auslösehebel um 25 Grad zurück; die Fußstütze ist noch eingeklappt.",
         dict(IN_USE_BONES, lever={"angleDeg": 25.0}), GLIDE_CONTACTS, bind(USE_EXPOSURE, OCCUPIED), occupant(0.75 * OCCUPANT_KG * G, 110.0, 190.0, 0.0), key=True),
    pose("footrest-up", "Footrest up (TV position)", "Fußstütze hoch (Fernsehposition)",
         "The linkage has swung the footrest out and up to 0.40 m, the back is still upright, the calves rest on the footrest.",
         "Das Gestänge hat die Fußstütze nach vorn und oben auf 0,40 m geschwenkt, die Lehne steht noch aufrecht, die Waden liegen auf der Fußstütze.",
         dict(IN_USE_BONES, **FOOT_OUT), GLIDE_CONTACTS, bind(USE_EXPOSURE, RECLINED), occupant(0.62 * OCCUPANT_KG * G, 120.0, 190.0, 0.14 * OCCUPANT_KG * G)),
    pose("reclined", "Fully reclined", "Ganz zurückgelehnt",
         "Footrest up and the back reclined by 35 degrees about its hinge: the sitter's upper body now rests on the back, the chair is 1.50 m long.",
         "Fußstütze hoch und die Lehne um 35 Grad um ihr Gelenk zurückgelehnt: der Oberkörper liegt nun auf der Lehne, der Sessel ist 1,50 m lang.",
         dict(IN_USE_BONES, back={"angleDeg": 35.0}, **FOOT_OUT), GLIDE_CONTACTS, bind(USE_EXPOSURE, RECLINED), occupant(0.55 * OCCUPANT_KG * G, 240.0, 225.0, 0.15 * OCCUPANT_KG * G)),
]

CLIPS = [
    {"id": "carry", "title": T("Carry out of the depot", "Aus dem Lager tragen"),
     "description": T("Two carriers grip the arm boxes from below, lift the chair off its moving blanket, tip it back by 6 degrees and hold it 0.22 m above the floor; the blanket stays behind.", "Zwei Träger fassen die Armkästen von unten, heben den Sessel von seiner Möbeldecke, kippen ihn um 6 Grad nach hinten und halten ihn 0,22 m über dem Boden; die Decke bleibt liegen."),
     "durationS": 4.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0.0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "lift-off-key", "ease": "smooth"}, {"t": 2.5, "pose": "carried", "ease": "smooth"}, {"t": 4.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0.0, "toS": 1.0} for a in BLANKET_CONTACTS] + [{"anchor": a, "fromS": 0.0, "toS": 4.0} for a in GRIP_CONTACTS],
     "phases": [{"id": "grip", "fromS": 0.0, "toS": 1.0, "title": T("Gripping under the arm boxes", "Unter die Armkästen greifen")}, {"id": "lift", "fromS": 1.0, "toS": 2.5, "title": T("Lifting and tipping back", "Anheben und nach hinten kippen")}, {"id": "carry", "fromS": 2.5, "toS": 4.0, "title": T("Carrying", "Tragen")}],
     "events": [{"t": 1.0, "kind": "contact-off", "title": T("The glides leave the blanket", "Die Gleiter verlassen die Decke")}, {"t": 2.5, "kind": "motion", "title": T("Chair carried level with the knees", "Sessel auf Kniehöhe getragen")}],
     "tracks": []},
    {"id": "recline", "title": T("Pull the lever and recline", "Hebel ziehen und zurücklehnen"),
     "description": T("The sitter pulls the release lever, the linkage throws the footrest out and up, and the back reclines by 35 degrees as the sitter leans back.", "Die sitzende Person zieht den Auslösehebel, das Gestänge klappt die Fußstütze nach vorn und oben, und die Lehne neigt sich um 35 Grad, während die Person sich zurücklehnt."),
     "durationS": 4.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0.0, "pose": "in-use", "ease": "smooth"}, {"t": 0.6, "pose": "lever-pulled-key", "ease": "out"}, {"t": 1.6, "pose": "footrest-up", "ease": "smooth", "bones": {"lever": {"angleDeg": 25.0}}}, {"t": 2.2, "pose": "footrest-up", "ease": "smooth"}, {"t": 4.0, "pose": "reclined", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0.0, "toS": 4.0} for a in GLIDE_CONTACTS],
     "phases": [{"id": "release", "fromS": 0.0, "toS": 0.6, "title": T("Pulling the lever", "Hebel ziehen")}, {"id": "footrest", "fromS": 0.6, "toS": 1.6, "title": T("Footrest swings out", "Fußstütze schwenkt aus")}, {"id": "recline", "fromS": 2.2, "toS": 4.0, "title": T("Leaning back", "Zurücklehnen")}],
     "events": [{"t": 0.6, "kind": "motion", "title": T("Release lever pulled", "Auslösehebel gezogen")}, {"t": 1.6, "kind": "motion", "title": T("Footrest locks out", "Fußstütze rastet aus")}, {"t": 4.0, "kind": "motion", "title": T("Back fully reclined", "Lehne ganz zurückgelehnt")}],
     "tracks": [{"target": "load:occupant-legs", "ease": "smooth", "keys": [{"t": 0.0, "v": 0.0}, {"t": 0.6, "v": 0.0}, {"t": 1.6, "v": 1.0}, {"t": 4.0, "v": 1.0}]}]},
]


BONE_INFO = {
    "ground": (T("Depot floor reference", "Bezug Lagerboden"), 0.0, {"type": "fixed"}, None),
    "blanket": (T("Folded moving blanket (stays on the floor)", "Gefaltete Möbeldecke (bleibt am Boden)"), 1.0, {"type": "prismatic", "axisDeg": 0, "minM": -1.3, "maxM": 0.0}, None),
    "lift": (T("Lift of the chair by its carriers", "Anheben des Sessels durch die Träger"), 0.0, {"type": "prismatic", "axisDeg": 90, "minM": -BLANKET, "maxM": 0.4}, None),
    "tilt": (T("Tilt about the rear glides", "Kippen um die hinteren Gleiter"), 0.0, {"type": "revolute", "minDeg": -10, "maxDeg": 15}, None),
    "frame": (T("Chair frame: arm boxes, seat box, rails and pivot tube", "Sesselgestell: Armkästen, Sitzkasten, Zargen und Drehrohr"), 0.92, {"type": "fixed"}, {"areaM2": 0.01176, "secondMomentM4": 0.000235, "extremeFibreM": 0.245, "material": "chipboard-p2", "title": T("Two 12 mm chipboard inner arm panels on edge, 490 mm deep", "Zwei hochkant stehende Arm-Innenwangen aus 12 mm Spanplatte, 490 mm hoch")}),
    "glides": (T("Four polypropylene glides", "Vier Polypropylen-Gleiter"), 0.0, {"type": "fixed"}, None),
    "seat": (T("Seat cushion on the seat box (sinks under load)", "Sitzkissen auf dem Sitzkasten (sinkt unter Last ein)"), 0.575, {"type": "prismatic", "axisDeg": 90, "minM": -0.06, "maxM": 0.0}, None),
    "back": (T("Back cushion on its spruce frame, hinged at the seat", "Rückenkissen auf seinem Fichtenrahmen, am Sitz angelenkt"), 0.60, {"type": "revolute", "minDeg": 0, "maxDeg": 40}, {"areaM2": 0.0032, "secondMomentM4": 4.27e-7, "extremeFibreM": 0.02, "material": "spruce-c24", "title": T("Two spruce stiles 40 x 40 mm", "Zwei Fichtenholme 40 x 40 mm")}),
    "link": (T("Footrest link pair, two riveted 4 mm steel stampings each side", "Fußstützen-Lenkerpaar, je Seite zwei vernietete 4-mm-Stahlstanzteile"), r5(LINK_LEN), {"type": "revolute", "minDeg": -35, "maxDeg": 0}, {"areaM2": 0.000576, "secondMomentM4": 6.22e-8, "extremeFibreM": 0.018, "material": "steel-black", "title": T("Two flat links 36 x 8 mm", "Zwei Flachlenker 36 x 8 mm")}),
    "footrest": (T("Padded footrest board", "Gepolstertes Fußstützenbrett"), BOARD_LEN, {"type": "revolute", "minDeg": 0, "maxDeg": 130}, {"areaM2": 0.00882, "secondMomentM4": 2.38e-7, "extremeFibreM": 0.009, "material": "birch-plywood", "title": T("Birch plywood 490 x 18 mm", "Birkensperrholz 490 x 18 mm")}),
    "lever": (T("Release lever on the right arm", "Auslösehebel an der rechten Armlehne"), 0.17, {"type": "revolute", "minDeg": 0, "maxDeg": 30}, {"areaM2": 0.00036, "secondMomentM4": 1.73e-8, "extremeFibreM": 0.012, "material": "polypropylene", "title": T("Polypropylene handle 24 x 15 mm", "Polypropylengriff 24 x 15 mm")}),
}


def rig():
    """🦴 The mechanical rig: floor, blanket, carrier lift and tilt, frame with glides, seat, back, footrest linkage and lever."""
    bones = []
    for bid, b in BONES.items():
        title, length, joint, section = BONE_INFO[bid]
        out = {"id": bid, "parent": b["parent"], "title": title, "x": r5(b["x"]), "y": r5(b["y"]), "angleDeg": round(b["angle"], 4), "lengthM": length, "joint": joint}
        if bid in MASS:
            kg, cx, cy = MASS[bid]
            out["mass"] = {"kg": kg, "comX": r5(cx), "comY": r5(cy), "distribution": "point", "source": "authored-estimate", "reference": "en-1728-2012"}
        if section:
            out["section"] = section
        bones.append(out)
    return {"kind": "mechanical", "root": "ground", "bones": bones}


REFERENCES = [
    {"id": "en-12520-2015", "citation": "EN 12520:2015 Furniture - Strength, durability and safety - Requirements for domestic seating (seat and back static loads, footrest and leg rest loads, durability cycles)."},
    {"id": "en-1728-2012", "citation": "EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability (load pads, seat and back static load tests)."},
    {"id": "en-1022-2018", "citation": "EN 1022:2018 Furniture - Seating - Determination of stability (rearward overturning of reclining chairs)."},
    {"id": "en-1021-1-2014", "citation": "EN 1021-1:2014 Furniture - Assessment of the ignitability of upholstered furniture - Part 1: Ignition source smouldering cigarette."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: densities, self-weight, imposed loads for buildings."},
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (popliteal height, buttock-knee length, sitting shoulder height of German adults)."},
    {"id": "din-18011-2006", "citation": "DIN 18011:2006 Room sizes and movement areas in residential buildings (furniture space of an armchair 0.85 x 0.85 m and its movement area)."},
    {"id": "en-312-2010", "citation": "EN 312:2010 Particleboards - Specifications (type P2 for interior fitments, bending strength 11.5 MPa and modulus 1600-1800 MPa at 13-20 mm, density about 650 kg/m3)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24: bending strength 24 MPa, mean modulus 11 GPa, mean density 420 kg/m3)."},
    {"id": "en-1993-1-1", "citation": "EN 1993-1-1:2005 Eurocode 3: Design of steel structures - General rules (S235, E = 210 GPa, 7850 kg/m3)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, density and specific heat of PVC, wood-based panels, timber, steel, foams and textiles)."},
    {"id": "iso-3386-1", "citation": "ISO 3386-1:1986 Polymeric materials, cellular flexible - Determination of stress-strain characteristic in compression (CLD 40 of seating foams 3.5-5 kPa)."},
    {"id": "en-15618-2009", "citation": "EN 15618:2009 Rubber- or plastics-coated fabrics - Upholstery fabrics - Specification (PVC-coated knitted fabrics of 450-750 g/m2 for furniture)."},
    {"id": "wypych-2017", "citation": "Wypych, G. (2017). Handbook of Plasticizers, 3rd ed., ChemTec Publishing (plasticiser migration and loss make PVC coatings stiff, glossy-dull and cracked on flexed edges such as arm rolls; emissivity of aged PVC coatings 0.92-0.94)."},
    {"id": "ashby-2011", "citation": "Ashby, M. F. (2011). Materials Selection in Mechanical Design, 4th ed., Butterworth-Heinemann (polypropylene density, modulus and strength)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties, convection correlations)."},
    {"id": "recliner-catalogue-typical", "citation": "Catalogue-typical data of manual German TV armchairs with lever recline of the 1995-2010 mid-range market (Fernsehsessel mit Hebelmechanik): 0.80-0.90 m wide, 0.90-0.95 m deep upright and 1.45-1.65 m reclined, 1.00-1.08 m high, seat height 0.45-0.48 m, arm height 0.60-0.65 m, 35-45 kg; brand-neutral type data, authored-estimate."},
]

PROVENANCE = {
    "author": "Semio staffage content (used household objects wave)",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": REFERENCES,
    "notes": T(
        "A manual lever recliner (TV armchair) of the German mid-range market around 2000, as a second-hand department store takes it in from a flat clearance: dark brown PVC faux leather whose plasticiser has migrated out, so the cover is dull and cracked on the front of the right arm roll, where the previous owner rested a forearm and somebody taped the crack over with brown parcel tape; the seat foam is compressed by 25 mm and sags in the middle; the top pillow of the waterfall back is sun-faded where a head rested. Side elevation from the right, so the release lever on the right arm is seen. Stored on a folded moving blanket, carried by two people under the arm boxes, used again upright, with the footrest up and fully reclined. Dimensions after the catalogue-typical data, DIN 33402-2 and DIN 18011; loads after EN 12520 and EN 1728; masses from the sections and densities of the reference table of the report.",
        "Ein Fernsehsessel mit Hebelmechanik aus dem deutschen Mittelklassemarkt um 2000, wie ein Gebrauchtwarenkaufhaus ihn aus einer Haushaltsauflösung übernimmt: dunkelbraunes PVC-Kunstleder, dessen Weichmacher ausgewandert ist, sodass der Bezug stumpf ist und vorn auf der rechten Armrolle aufgerissen, wo der Vorbesitzer den Unterarm aufgelegt hatte und jemand den Riss mit braunem Packband überklebt hat; der Sitzschaum ist um 25 mm zusammengedrückt und hängt in der Mitte durch; das obere Kissen der Wasserfall-Lehne ist dort ausgeblichen, wo ein Kopf lag. Seitenansicht von rechts, damit der Auslösehebel an der rechten Armlehne zu sehen ist. Gelagert auf einer gefalteten Möbeldecke, von zwei Personen unter den Armkästen getragen, wieder benutzt aufrecht, mit hochgestellter Fußstütze und ganz zurückgelehnt. Maße nach den katalogtypischen Daten, DIN 33402-2 und DIN 18011; Lasten nach EN 12520 und EN 1728; Massen aus den Querschnitten und Dichten der Referenztabelle des Berichts."),
}

THERMAL_ASSUMPTIONS = [
    T("Indoor reference case of a heated depot or living room in winter (21 degC air, 19 degC radiant, still air, no sun); every surface is passive with one effective upholstery or material layer and its heat capacity; there are no emitters and no electrical parts: the lever recliner draws 0 W, stated on the arm covers as passive.", "Innenraum-Referenzfall eines beheizten Lagers oder Wohnzimmers im Winter (21 degC Luft, 19 degC Strahlung, ruhige Luft, keine Sonne); jede Fläche ist passiv mit einer wirksamen Polster- oder Materialschicht und ihrer Wärmekapazität; es gibt keine Emitter und keine elektrischen Teile: der Hebelsessel nimmt 0 W auf, als passiv an den Armbezügen angegeben."),
    T("Aged PVC faux leather: plasticiser loss raises the emissivity of the dulled coating from about 0.90 to 0.93 and the solar absorptance of the dark brown cover is 0.82 (Wypych 2017; ISO 10456 for density, conductivity and heat capacity of PVC). Upholstery resistances are R = 0.001/0.17 + t/0.040 for the foam thickness t of each part (arm 30 mm, seat 75 mm after 25 mm of compression, footrest 70 mm) and t/0.045 for the 90 mm fibre fill of the back.", "Gealtertes PVC-Kunstleder: der Weichmacherverlust hebt den Emissionsgrad der stumpf gewordenen Beschichtung von etwa 0,90 auf 0,93, der solare Absorptionsgrad des dunkelbraunen Bezugs ist 0,82 (Wypych 2017; ISO 10456 für Dichte, Leitfähigkeit und Wärmekapazität von PVC). Die Polsterwiderstände sind R = 0,001/0,17 + t/0,040 für die Schaumdicke t jedes Teils (Arm 30 mm, Sitz 75 mm nach 25 mm Stauchung, Fußstütze 70 mm) und t/0,045 für die 90 mm Faserfüllung der Lehne."),
    T("In use the seated adult warms the seat (35 % of its area in contact at 32 degC), the back (25 %, 30 % reclined), the arm rolls under the forearms (5 % at 31 degC) and, with the footrest up, the footrest under the calves (30 %).", "In Gebrauch wärmt die sitzende Person den Sitz (35 % seiner Fläche in Kontakt mit 32 degC), die Lehne (25 %, zurückgelehnt 30 %), die Armrollen unter den Unterarmen (5 % mit 31 degC) und bei hochgestellter Fußstütze die Fußstütze unter den Waden (30 %)."),
    T("Heat capacities per area are rho cp min(thickness, sqrt(alpha 3600 s)) of the outer layer and the first centimetres of the core; the moving blanket is four layers of 5 mm recycled cotton felt (R = 0.02/0.045).", "Wärmekapazitäten je Fläche sind rho cp min(Dicke, sqrt(alpha 3600 s)) der äußeren Schicht und der ersten Zentimeter des Kerns; die Möbeldecke besteht aus vier Lagen Recycling-Baumwollvlies zu 5 mm (R = 0,02/0,045)."),
]

STRUCTURE_ASSUMPTIONS = [
    T("Stored, the chair stands on four glides on the folded blanket, which carries it to the depot floor (four contacts under the glides); carried, the four hands of two carriers under the front and rear ends of the arm boxes are the supports (contacts with 90 x 80 mm palms) and the blanket keeps only its own weight; in use the four glides stand on the floor.", "Gelagert steht der Sessel auf vier Gleitern auf der gefalteten Decke, die ihn auf den Lagerboden abträgt (vier Kontakte unter den Gleitern); getragen sind die vier Hände der zwei Träger unter den vorderen und hinteren Enden der Armkästen die Auflager (Kontakte mit 90 x 80 mm Handflächen) und die Decke trägt nur ihr Eigengewicht; in Gebrauch stehen die vier Gleiter auf dem Boden."),
    T("Load path: occupant on the seat cushion into the seat box (chipboard ring with zig-zag springs and the hidden scissor mechanism of 6 kg), the seat box and the rear rail into the 12 mm chipboard inner arm panels with their spruce base and top rails, the back frame (two spruce stiles 40 x 40 mm) through its steel hinge brackets into the arm panels, the footrest board through the riveted link pair into the 25 mm pivot tube, and the arm boxes through the glides to the floor.", "Lastpfad: Person auf dem Sitzkissen in den Sitzkasten (Spanplattenrahmen mit Wellenfedern und der verdeckten Scherenmechanik von 6 kg), Sitzkasten und hintere Zarge in die 12-mm-Spanplatten-Innenwangen der Armkästen mit ihren Fichten-Fuß- und Oberzargen, der Lehnenrahmen (zwei Fichtenholme 40 x 40 mm) über seine Stahl-Gelenkwinkel in die Armwangen, das Fußstützenbrett über das vernietete Lenkerpaar in das 25-mm-Drehrohr und die Armkästen über die Gleiter in den Boden."),
    T("Occupant: a 75 kg adult (735 N, EN 1991-1-1 imposed load as a sitting case) split into seat, back and calves: upright 552 N on the seat and 110 N against the back (feet on the floor), footrest up 456 N seat, 120 N back and 103 N on the footrest, reclined 405 N seat, 240 N normal to the back and 110 N on the footrest; EN 12520 / EN 1728 test the domestic seat with 1300 N and the back with 410 N, the footrest with 750 N (authored values to be checked against the standard texts).", "Person: eine erwachsene Person von 75 kg (735 N, Nutzlast nach EN 1991-1-1 als Sitzfall) aufgeteilt auf Sitz, Lehne und Waden: aufrecht 552 N auf dem Sitz und 110 N gegen die Lehne (Füße am Boden), Fußstütze hoch 456 N Sitz, 120 N Lehne und 103 N auf der Fußstütze, zurückgelehnt 405 N Sitz, 240 N senkrecht zur Lehne und 110 N auf der Fußstütze; EN 12520 / EN 1728 prüfen den Wohnsitz mit 1300 N und die Lehne mit 410 N, die Fußstütze mit 750 N (Schätzwerte, am Normtext zu prüfen)."),
    T("Rearward stability (EN 1022): fully reclined with the occupant, the centre of mass of chair and person lies about 0.25 m in front of the rear glides, so the recliner does not overturn backwards; the wear has loosened nothing that bears load, the chair is offered for reuse.", "Standsicherheit nach hinten (EN 1022): ganz zurückgelehnt mit der Person liegt der Schwerpunkt von Sessel und Person etwa 0,25 m vor den hinteren Gleitern, der Sessel kippt also nicht nach hinten; die Abnutzung hat nichts Tragendes gelockert, der Sessel wird zur Wiederverwendung angeboten."),
]


REP_DECISIONS = {
    50: {
        "silhouette": T("Side elevation from the right, 18.3 x 20.4 mm: the near arm box with its padded roll and front scroll, the four-pillow waterfall back above it, the release lever with its boss on the right arm, two glides on the folded moving blanket; the stowed footrest, the seat cushion and the linkage lie behind the arm and appear when the footrest swings out. Front view: both arm boxes with their round tops, the seat cushion between them, the padded footrest stowed under it with the link plates beside it, the back above, the lever beside the right arm. Plan: blanket, arms, seat, back and the lever.", "Seitenansicht von rechts, 18,3 x 20,4 mm: der nahe Armkasten mit gepolsterter Rolle und vorderer Rundung, darüber die Wasserfall-Lehne mit vier Kissen, der Auslösehebel mit seiner Nabe an der rechten Armlehne, zwei Gleiter auf der gefalteten Möbeldecke; die eingeklappte Fußstütze, das Sitzkissen und das Gestänge liegen hinter dem Arm und erscheinen, wenn die Fußstütze ausschwenkt. Vorderansicht: beide Armkästen mit runder Oberseite, das Sitzkissen dazwischen, darunter die eingeklappte gepolsterte Fußstütze mit den Lenkerblechen daneben, darüber die Lehne, der Hebel neben dem rechten Arm. Grundriss: Decke, Arme, Sitz, Lehne und Hebel."),
        "retained": T("Glides, arm boxes with piping and front seam, waterfall seams of the back, seat piping, footrest board and pad, link plates, pivot tube, seat box, lever handle and boss, the craquelure and the parcel tape on the right arm roll, the quilting of the blanket.", "Gleiter, Armkästen mit Keder und vorderer Naht, Wasserfallnähte der Lehne, Sitzkeder, Fußstützenbrett und Polster, Lenkerbleche, Drehrohr, Sitzkasten, Hebelgriff und Nabe, die Risse und das Packband auf der rechten Armrolle, die Steppung der Decke."),
        "merged": T("Arm box and roll are one outline in the side elevation; the far arm coincides with the near one.", "Armkasten und Rolle sind in der Seitenansicht ein Umriss; der ferne Arm fällt mit dem nahen zusammen."),
        "omitted": T("Hidden scissor mechanism inside the seat box, staples and the springs under the seat.", "Verdeckte Scherenmechanik im Sitzkasten, Klammern und die Federn unter dem Sitz."),
        "thinElements": T("The link plates are drawn in the front view up to the arm faces so the 6 mm air gap does not print as a sliver; seams and cracks are strokes.", "Die Lenkerbleche sind in der Vorderansicht bis an die Armflächen gezeichnet, damit der Luftspalt von 6 mm nicht als Splitter druckt; Nähte und Risse sind Striche."),
        "hatching": T("Quilting of the moving blanket as a cross hatch of 100 mm; no section hatches.", "Steppung der Möbeldecke als Kreuzschraffur von 100 mm; keine Schnittschraffuren."),
        "styles": T("Dark faux leather in mid greys, black seat box and steel, pale blanket.", "Dunkles Kunstleder in mittleren Grautönen, schwarzer Sitzkasten und Stahl, helle Decke."),
        "overlays": T("Thermal regions and structural members as in the model; no annotations.", "Thermische Regionen und Tragglieder wie im Modell; keine Beschriftungen."),
    },
    100: {
        "silhouette": T("The same views at 1:100, 9.2 x 10.2 mm: arm, back, lever, glides and blanket in the side elevation, arms, seat, footrest, links and back in the front view, the plan with the lever.", "Dieselben Ansichten in 1:100, 9,2 x 10,2 mm: Arm, Lehne, Hebel, Gleiter und Decke in der Seitenansicht, Arme, Sitz, Fußstütze, Lenker und Lehne in der Vorderansicht, der Grundriss mit dem Hebel."),
        "retained": T("Glides, piping and seams, footrest board and pad, link plates, lever, a single crack line and the tape, blanket quilting.", "Gleiter, Keder und Nähte, Fußstützenbrett und Polster, Lenkerbleche, Hebel, eine einzelne Risslinie und das Klebeband, Steppung der Decke."),
        "merged": T("The lever boss is merged into the handle; the link plates fill the gap to the arms.", "Die Hebelnabe ist in den Griff eingeschmolzen; die Lenkerbleche füllen den Spalt bis zu den Armen."),
        "omitted": T("Hinge brackets, the second crack line, the hidden mechanism.", "Gelenkwinkel, die zweite Risslinie, die verdeckte Mechanik."),
        "thinElements": T("The 25 mm pivot tube and the 15 mm link pack stay drawn at their size.", "Das 25-mm-Drehrohr und das 15-mm-Lenkerpaket bleiben in ihrer Größe gezeichnet."),
        "hatching": T("Blanket quilting at 100 mm.", "Steppung der Decke mit 100 mm."),
        "styles": T("As at 1:50 with lighter pens.", "Wie in 1:50 mit leichteren Stiften."),
        "overlays": T("None.", "Keine."),
    },
    200: {
        "silhouette": T("4.6 x 5.1 mm: arm boxes standing on the blanket, back with its two seams, seat, footrest as one padded block, glides marked under the arms.", "4,6 x 5,1 mm: Armkästen auf der Decke stehend, Lehne mit ihren zwei Nähten, Sitz, Fußstütze als ein gepolsterter Block, Gleiter unter den Armen angedeutet."),
        "retained": T("Arm piping, back seams, seat piping, glides.", "Armkeder, Lehnennähte, Sitzkeder, Gleiter."),
        "merged": T("The 30 mm glide gap is closed: the arm boxes reach the blanket; board and pad of the footrest are one block that reaches the arms.", "Der Gleiterspalt von 30 mm ist geschlossen: die Armkästen reichen bis zur Decke; Brett und Polster der Fußstütze sind ein Block bis an die Arme."),
        "omitted": T("Lever, link plates, pivot tube, cracks, tape, quilting.", "Hebel, Lenkerbleche, Drehrohr, Risse, Klebeband, Steppung."),
        "thinElements": T("The 20 mm blanket is kept as a thin band under the chair.", "Die 20-mm-Decke bleibt als dünnes Band unter dem Sessel."),
        "hatching": T("None.", "Keine."),
        "styles": T("Grey tones of 1:100, detail pen 0.13 mm.", "Grautöne wie 1:100, Detailstift 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
    500: {
        "silhouette": T("1.8 x 2.0 mm: the block of the two arm boxes, the back, the seat and the footrest on the blanket band.", "1,8 x 2,0 mm: der Block der zwei Armkästen, die Lehne, der Sitz und die Fußstütze auf dem Deckenband."),
        "retained": T("The silhouettes of arms, back, seat, footrest, blanket and the glides inside the arm base.", "Die Umrisse von Armen, Lehne, Sitz, Fußstütze, Decke und die Gleiter im Armsockel."),
        "merged": T("Arm box and roll are one square-topped box; the waterfall back is one pillow.", "Armkasten und Rolle sind ein Kasten mit gerader Oberseite; die Wasserfall-Lehne ist ein Kissen."),
        "omitted": T("Every seam, the lever, the linkage, cracks and tape.", "Alle Nähte, der Hebel, das Gestänge, Risse und Klebeband."),
        "thinElements": T("None.", "Keine."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.18 mm only.", "Nur Körperstift 0,18 mm."),
        "overlays": T("None.", "Keine."),
    },
}

VOL_DECISIONS = {
    50: [T("Body modelled from the real chair: four turned glides, two arm boxes as side profiles extruded over 170 mm with a capsule roll of 85 mm radius on top, the seat box and the rear rail, the 25 mm pivot tube, two 9.5 mm hinge brackets that tie the back to the arm panels, the sagging seat cushion and the four-pillow waterfall back as extruded profiles, the plywood footrest board with its pad, the pair of riveted link plates and the lever with its boss on the right arm; the parcel tape is a texture band on the roll.", "Körper nach dem wirklichen Sessel: vier gedrehte Gleiter, zwei Armkästen als Seitenprofile über 170 mm gezogen mit einer Kapselrolle von 85 mm Radius obenauf, Sitzkasten und hintere Zarge, das 25-mm-Drehrohr, zwei 9,5-mm-Gelenkwinkel, die die Lehne mit den Armwangen verbinden, das durchgesessene Sitzkissen und die Wasserfall-Lehne mit vier Kissen als gezogene Profile, das Sperrholzbrett der Fußstütze mit Polster, das Paar vernieteter Lenkerbleche und der Hebel mit Nabe an der rechten Armlehne; das Packband ist ein Texturband auf der Rolle."),
         T("Thermal: every body surface referenced into its region. Structure: the 12 mm chipboard inner arm panels with base and top rails, seat box, rear rail, pivot tube, seat cushion, back frame of two stiles and two rails, link plates, footrest board, lever; pins at the four rig joints, the seat bearing on the seat box and a mass marker per bone; the twelve contact anchors (blanket, glides, hands) are switched by the poses and clips, so the structure has no permanent support solid.", "Thermik: jede Körperfläche in ihre Region verwiesen. Tragwerk: die 12-mm-Spanplatten-Innenwangen mit Fuß- und Oberzargen, Sitzkasten, hintere Zarge, Drehrohr, Sitzkissen, Lehnenrahmen aus zwei Holmen und zwei Riegeln, Lenkerbleche, Fußstützenbrett, Hebel; Bolzen an den vier Gelenken, das Sitzlager auf dem Sitzkasten und eine Massenmarke je Knochen; die zwölf Kontaktanker (Decke, Gleiter, Hände) schalten Posen und Clips, daher hat das Tragwerk keinen ständigen Auflagerkörper.")],
    100: [T("Modelled for 15 mm: hinge brackets and lever boss dropped, the back widened to 515 mm so it bears on the arm panels, the link plates as one 15 mm pack per side that fills the gap between footrest and arm, simplified glides and the same arm, seat, back and footrest profiles.", "Für 15 mm modelliert: Gelenkwinkel und Hebelnabe entfallen, die Lehne auf 515 mm verbreitert, sodass sie an den Armwangen anliegt, die Lenkerbleche als ein 15-mm-Paket je Seite, das den Spalt zwischen Fußstütze und Arm füllt, vereinfachte Gleiter und dieselben Profile für Arm, Sitz, Lehne und Fußstütze."),
          T("Structure as at 1:50 with the 12 mm arm panels and the link plates declared as sheet and slender at their true section.", "Tragwerk wie in 1:50, die 12-mm-Armwangen und die Lenkerbleche als Platte und schlank in ihrem wahren Querschnitt erklärt.")],
    200: [T("Modelled for 30 mm: arm boxes reach down to the blanket (the glides stay inside their base as small turned parts), simplified seat and back profiles, the footrest as one padded block 514 mm wide, the blanket declared as a 20 mm sheet, the pivot tube slender; rear rail, link plates and lever are below the feature and left out of the body.", "Für 30 mm modelliert: die Armkästen reichen bis zur Decke (die Gleiter bleiben als kleine Drehteile in ihrem Sockel), vereinfachte Sitz- und Lehnenprofile, die Fußstütze als ein gepolsterter Block von 514 mm Breite, die Decke als 20-mm-Platte erklärt, das Drehrohr schlank; hintere Zarge, Lenkerbleche und Hebel liegen unter dem Merkmal und fehlen im Körper."),
          T("Structure: each arm frame as one 40 mm chipboard wall, back frame, link plates and lever kept as slender members at their true sections.", "Tragwerk: jeder Armrahmen als eine 40-mm-Spanplattenwand, Lehnenrahmen, Lenkerbleche und Hebel als schlanke Stäbe in ihrem wahren Querschnitt.")],
    500: [T("Modelled for 75 mm: each arm is one extruded box with the scroll in its side profile, the back one pillow, seat, seat box, footrest block, blanket sheet and pivot tube; the glides stay as small parts inside the arm base.", "Für 75 mm modelliert: jeder Arm ist ein gezogener Kasten mit der Rundung im Seitenprofil, die Lehne ein Kissen, Sitz, Sitzkasten, Fußstützenblock, Deckenplatte und Drehrohr; die Gleiter bleiben als kleine Teile im Armsockel."),
          T("Structure: arm frames as 80 mm walls, the back stiles slender, seat box, link plates and lever slender, masses of the scale's feature.", "Tragwerk: Armrahmen als 80-mm-Wände, die Lehnenholme schlank, Sitzkasten, Lenkerbleche und Hebel schlank, Massen in der Größe des Merkmals.")],
}


def representation(scale):
    """🗺 One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": geometry(scale), "decisions": REP_DECISIONS[scale]}


def volume(scale):
    """🧊 One authored volume."""
    body = body_solids(scale)
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_solids(scale, body), "structure": structure_solids(scale, body), "decisions": VOL_DECISIONS[scale]}


def extents():
    """📏 Overall rest extents (height, x, z) from the 1:50 drawings."""
    g = geometry(50)
    ys, xs, zs = [], [], []
    for s in g["elevation"]:
        if s["role"] == "body":
            for p in to_world(s["bone"], s["points"]):
                xs.append(p[0])
                ys.append(p[1])
    for s in g["front"]:
        if s["role"] == "body" and "solid" not in s:
            zs.extend(p[0] for p in s["points"])
    return max(ys), max(xs) - min(xs), max(zs) - min(zs)


def document():
    """📄 The whole asset document."""
    h, wx, dz = extents()
    mass = round(sum(m[0] for m in MASS.values()), 2)
    asset = {
        "id": "used-recliner-chair",
        "category": "furniture",
        "title": T("Used Recliner Chair", "Gebrauchter Fernsehsessel"),
        "summary": T("A second-hand manual lever recliner in dark brown PVC faux leather, 0.89 m wide over the lever, 0.91 m deep and 1.01 m high, 39.5 kg: two arm boxes with padded rolls, a sagging seat cushion, a four-pillow waterfall back hinged at the seat, a padded footrest on a riveted steel linkage and the release lever on the right arm; the plasticiser-starved cover has cracked on the right arm roll and is taped over with parcel tape. Stored on a folded moving blanket, carried by two people under the arm boxes, and used again upright, with the footrest up and reclined by 35 degrees.",
                     "Ein gebrauchter Fernsehsessel mit Hebelmechanik in dunkelbraunem PVC-Kunstleder, 0,89 m breit über dem Hebel, 0,91 m tief und 1,01 m hoch, 39,5 kg: zwei Armkästen mit gepolsterten Rollen, ein durchgesessenes Sitzkissen, eine am Sitz angelenkte Wasserfall-Lehne mit vier Kissen, eine gepolsterte Fußstütze an einem vernieteten Stahlgestänge und der Auslösehebel an der rechten Armlehne; der weichmacherarme Bezug ist auf der rechten Armrolle gerissen und mit Packband überklebt. Gelagert auf einer gefalteten Möbeldecke, von zwei Personen unter den Armkästen getragen und wieder benutzt aufrecht, mit hochgestellter Fußstütze und um 35 Grad zurückgelehnt."),
        "tags": ["used", "household", "second-hand", "living", "recliner", "armchair", "seating", "faux-leather", "upholstery", "mechanism", "moving-blanket"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": round(h, 4), "widthM": round(wx, 4), "depthM": round(dz, 4), "massKg": mass},
        "materials": MATERIALS,
        "rig": rig(),
        "anchors": ANCHORS,
        "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {"environment": {"airTempC": 21, "meanRadiantTempC": 19, "relativeHumidity": 0.35, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}, "referenceEnvironment": "indoor-heated-winter", "passengers": [], "operating": [], "regions": REGIONS, "emitters": [], "assumptions": THERMAL_ASSUMPTIONS},
        "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": STRUCTURE_SUPPORTS, "assumptions": STRUCTURE_ASSUMPTIONS, "clinical": False},
        "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
        "presentation": {"defaultPose": "stored", "defaultClip": "recline", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    import json
    import os
    here = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(here, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(document(), fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    d = document()["asset"]["dimensions"]
    print(f"[DEBUG] written: height {d['heightM']} m, x {d['widthM']} m, z {d['depthM']} m, mass {d['massKg']} kg")
