"""🍳 Hand-chosen numbers and the 3D solids of the used 50 cm electric cooker on its half pallet, one handcrafted body list per authored scale.

Frames: carcass, drawer and tray bones share the carcass frame (x ahead, y up from the foot soles, z left); the door bone has u up the door from its hinge, v backwards into the oven, w left; the pallet bone has x from its rear edge.

See also author-used-electric-cooker.py (the authoritative script) and ../../reports/used-electric-cooker.md.
"""
import math


def T(en, de):
    """🔤 One bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯 Rounds a number to the micrometre for the document."""
    return round(v, 6)


def pts(seq):
    """📍 Rounds a list of 2D points."""
    return [[r6(a), r6(b)] for a, b in seq]


PALLET_H = 0.144
PALLET_X0 = -0.02
PALLET_L = 0.60
PALLET_Z = 0.40
BOARD_T = 0.022
BLOCK_Y1 = 0.100
STRINGER_Y1 = 0.122
ROW_Z = [(-0.40, -0.30), (-0.05, 0.05), (0.30, 0.40)]
BLOCK_X = [(0.0, 0.10), (0.25, 0.35), (0.50, 0.60)]
TOP_BOARDS = [(0.0, 0.145), (0.185, 0.28), (0.32, 0.415), (0.455, 0.60)]
TOP_BOARDS_100 = [(0.0, 0.145), (0.2275, 0.3725), (0.455, 0.60)]

ZO, ZI = 0.25, 0.2493
X_SIDE = 0.560
FOOT_H = 0.015
BASE_Y1 = 0.016
TOP_Y0, TOP_Y1 = 0.849, 0.850
TOP_X1 = 0.570
PLATE_TOP = 0.857
FASCIA_X0, FASCIA_X1, FASCIA_Y0 = 0.548, 0.568, 0.725
FRAME_X0 = 0.559
HINGE_X, HINGE_Y = 0.600, 0.195
DOOR_H, DOOR_T, DOOR_Z = 0.520, 0.040, 0.247
WIN_W, WIN_U0, WIN_U1 = 0.160, 0.160, 0.400
HANDLE_U, HANDLE_V, HANDLE_R, HANDLE_Z, STANDOFF_Z = 0.470, -0.035, 0.010, 0.200, 0.190
DRAWER_X0, DRAWER_X1, DRAWER_Y0, DRAWER_Y1, DRAWER_Z = 0.560, 0.580, 0.017, 0.1935, 0.249
BOX_X0, BOX_Y0, BOX_Y1, BOX_Z = 0.060, 0.030, 0.150, 0.235
CAV_X0, CAV_Y0, CAV_Y1, CAV_Z = 0.130, 0.285, 0.625, 0.195
INS_X0, INS_Y0, INS_Y1, INS_Z = 0.090, 0.245, 0.665, 0.235
RAIL_LEVELS = [0.370, 0.450, 0.530]
TRAY_X0, TRAY_X1, TRAY_Z, TRAY_Y0, TRAY_H = 0.155, 0.535, 0.194, 0.450, 0.025
KNOB_Y = 0.787
KNOB_ZS = [-0.185, -0.115, -0.045, 0.045, 0.115, 0.185]
REPLACED_KNOB = -0.115
LAMP_ZS = [-0.224, 0.224]
FEET = [(0.045, -0.215), (0.515, -0.215), (0.045, 0.215), (0.515, 0.215)]
FOOT_R = 0.016
COIL = (-0.012, 0.400, 0.100)
COIL_R, COIL_T = 0.075, 0.012
PLATES = [
    ("fl", 0.41, 0.12, 0.0885, 0.0995, 2000, T("front left, 180 mm, 2.0 kW", "vorn links, 180 mm, 2,0 kW")),
    ("fr", 0.41, -0.12, 0.0705, 0.0815, 1000, T("front right, 145 mm, 1.0 kW", "vorn rechts, 145 mm, 1,0 kW")),
    ("rl", 0.16, 0.12, 0.0705, 0.0815, 1500, T("rear left, 145 mm, 1.5 kW rapid plate with red dot", "hinten links, 145 mm, 1,5-kW-Blitzkochplatte mit rotem Punkt")),
    ("rr", 0.16, -0.12, 0.0885, 0.0995, 1500, T("rear right, 180 mm, 1.5 kW", "hinten rechts, 180 mm, 1,5 kW")),
]
STEEL, CAST_IRON, GLASS, SPRUCE, WOOL, PP = 7850.0, 7200.0, 2500.0, 450.0, 60.0, 900.0


def poly_area(p):
    """📐 Shoelace area of a simple polygon."""
    return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2


def poly_centroid(p):
    """⚖️ Area centroid of a simple polygon."""
    a = cx = cy = 0.0
    for i in range(len(p)):
        x0, y0 = p[i]
        x1, y1 = p[(i + 1) % len(p)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    return cx / (3 * a), cy / (3 * a)


def revolve_volume(profile):
    """🌀 Pappus volume of an [axial, radial] profile closed through the axis (open profile) or a closed loop."""
    closed = profile[0] == profile[-1]
    ring = profile[:-1] if closed else [(profile[0][0], 0.0)] + list(profile) + [(profile[-1][0], 0.0)]
    area = poly_area(ring)
    _, rc = poly_centroid(ring)
    return 2 * math.pi * abs(rc) * area


def rect_ring(a0, a1, b0, b1):
    """⬜ A rectangle as a counter-clockwise ring."""
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def circle_ring(ca, cb, r, n=12):
    """⭕ A circle as a polygon ring of n points."""
    return [(ca + r * math.cos(2 * math.pi * k / n), cb + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def box(sid, bone, mat, tone, x0, x1, y0, y1, z0, z1, role="body", kg=None, **kw):
    """📦 A box solid from its bone-local extents; `kg` overrides section x density for assemblies."""
    s = {"id": sid, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": [r6((x0 + x1) / 2), r6((y0 + y1) / 2), r6((z0 + z1) / 2)]}}
    s.update(kw)
    s["_vol"] = (x1 - x0) * (y1 - y0) * (z1 - z0)
    s["_com"] = ((x0 + x1) / 2, (y0 + y1) / 2)
    if kg is not None:
        s["_kg"] = kg
    return s


def extrude(sid, bone, mat, tone, profile, plane, f, t, holes=None, role="body", kg=None, com=None, **kw):
    """🧱 An extrude solid; volume = (profile - holes) x length, centre of mass from the profile centroid."""
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": pts(profile), "plane": plane, "from": r6(f), "to": r6(t)}
    area = poly_area(profile)
    if holes:
        s["holes"] = [pts(h) for h in holes]
        area -= sum(poly_area(h) for h in holes)
    s.update(kw)
    s["_vol"] = area * (t - f)
    a, b = poly_centroid(profile)
    m = (f + t) / 2
    s["_com"] = com or {"xy": (a, b), "yz": (m, a), "zx": (b, m)}[plane]
    if kg is not None:
        s["_kg"] = kg
    return s


def revolve(sid, bone, mat, tone, profile, axis, origin, role="body", kg=None, **kw):
    """🍄 A revolve solid about `axis` through `origin`."""
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": pts(profile), "axis": axis, "origin": [r6(c) for c in origin]}
    s.update(kw)
    s["_vol"] = revolve_volume(profile)
    s["_com"] = (origin[0], origin[1])
    if kg is not None:
        s["_kg"] = kg
    return s


def capsule(sid, bone, mat, tone, p0, p1, r, role="body", kg=None, **kw):
    """💊 A capsule solid between two points."""
    s = {"id": sid, "kind": "capsule", "bone": bone, "material": mat, "role": role, "tone": tone, "p0": [r6(c) for c in p0], "p1": [r6(c) for c in p1], "r0": r6(r), "r1": r6(r)}
    s.update(kw)
    length = math.dist(p0, p1)
    s["_vol"] = math.pi * r * r * length + 4 / 3 * math.pi * r ** 3
    s["_com"] = ((p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2)
    if kg is not None:
        s["_kg"] = kg
    return s


def plate_profile(rp, n):
    """🔘 Half section [axial up from the hob surface, radius] of a cast-iron hotplate: 5 mm below the hob, 7 mm proud, chamfered rim, 2 mm dished unheated centre."""
    if n == 50:
        return [(-0.005, 0.0), (-0.005, rp - 0.004), (-0.002, rp), (0.006, rp), (0.007, rp - 0.003), (0.007, 0.012), (0.005, 0.010), (0.005, 0.0)]
    return [(-0.015, 0.0), (-0.015, rp + 0.011), (0.0015, rp + 0.011), (0.006, rp), (0.007, rp - 0.004), (0.007, 0.0)]


def ring_profile(rp, ro):
    """💍 Closed-loop section of the pressed stainless trim ring that holds a hotplate in the hob."""
    return [(0.0, rp), (0.006, rp), (0.006, rp + 0.002), (0.0018, rp + 0.0035), (0.0012, ro), (0.0, ro), (0.0, rp)]


KNOB = [(0.0, 0.019), (0.004, 0.019), (0.006, 0.018), (0.022, 0.017), (0.026, 0.015), (0.027, 0.0)]
KNOB_OLD = [(0.0, 0.017), (0.023, 0.017), (0.024, 0.016), (0.0245, 0.0)]
KNOB_100 = [(0.0, 0.019), (0.022, 0.017), (0.027, 0.0)]
FOOT = [(0.0, 0.0), (0.0, 0.013), (0.004, FOOT_R), (FOOT_H, FOOT_R), (FOOT_H, 0.0)]


def coil_profile(n=10):
    """➰ Closed-loop circle of the cable bundle section around the coil radius."""
    p = [(COIL_T * math.cos(2 * math.pi * k / n), COIL_R + COIL_T * math.sin(2 * math.pi * k / n)) for k in range(n)]
    return p + [p[0]]


def pallet_50():
    """🪵 Half pallet 600 x 800 x 144 mm at 1:50: three bottom boards, nine blocks (three repeated rows), three stringer boards, four top boards."""
    b = "pallet"
    out = []
    for i, (z0, z1) in enumerate(ROW_Z):
        out.append(box(f"pallet-bottom-{i + 1}", b, "spruce-pallet", 0.55, 0.0, PALLET_L, 0.0, BOARD_T, z0, z1, cornerRadius=0.002))
        out.append(box(f"pallet-blocks-{i + 1}", b, "spruce-pallet", 0.45, 0.0, 0.10, BOARD_T, BLOCK_Y1, z0, z1, repeat={"count": 3, "translation": [0.25, 0.0, 0.0]}))
        out.append(box(f"pallet-stringer-{i + 1}", b, "spruce-pallet", 0.55, 0.0, PALLET_L, BLOCK_Y1, STRINGER_Y1, z0, z1, cornerRadius=0.002))
    for i, (x0, x1) in enumerate(TOP_BOARDS):
        out.append(box(f"pallet-top-{i + 1}", "pallet-deck", "spruce-pallet", 0.6, x0, x1, STRINGER_Y1, PALLET_H, -PALLET_Z, PALLET_Z, cornerRadius=0.002))
    for s in out:
        if "repeat" in s:
            s["_vol"] *= 3
            s["_com"] = (0.30, s["_com"][1])
    return out


def pallet_100():
    """🪵 Half pallet at 1:100: bottom boards, blocks and stringers as built, the four top boards typified as three with gaps that still print, sharp edges."""
    b = "pallet"
    out = []
    for i, (z0, z1) in enumerate(ROW_Z):
        out.append(box(f"pallet-bottom-{i + 1}", b, "spruce-pallet", 0.55, 0.0, PALLET_L, 0.0, BOARD_T, z0, z1))
        out.append(box(f"pallet-blocks-{i + 1}", b, "spruce-pallet", 0.45, 0.0, 0.10, BOARD_T, BLOCK_Y1, z0, z1, repeat={"count": 3, "translation": [0.25, 0.0, 0.0]}))
        out.append(box(f"pallet-stringer-{i + 1}", b, "spruce-pallet", 0.55, 0.0, PALLET_L, BLOCK_Y1, STRINGER_Y1, z0, z1))
    for i, (x0, x1) in enumerate(TOP_BOARDS_100):
        out.append(box(f"pallet-top-{i + 1}", "pallet-deck", "spruce-pallet", 0.6, x0, x1, STRINGER_Y1, PALLET_H, -PALLET_Z, PALLET_Z, envelope=True))
    return out


def pallet_200():
    """🪵 Half pallet at 1:200: the 22 mm boards are under 30 mm, so the top deck and the stringers are one 44 mm deck and each block reaches the floor through its bottom board."""
    b = "pallet"
    out = [box("pallet-deck", "pallet-deck", "spruce-pallet", 0.6, 0.0, PALLET_L, BLOCK_Y1, PALLET_H, -PALLET_Z, PALLET_Z)]
    for i, (z0, z1) in enumerate(ROW_Z):
        out.append(box(f"pallet-blocks-{i + 1}", b, "spruce-pallet", 0.45, 0.0, 0.10, 0.0, BLOCK_Y1, z0, z1, repeat={"count": 3, "translation": [0.25, 0.0, 0.0]}))
    return out


def door_50():
    """🚪 Oven door at 1:50: pressed enamel frame with the window opening, outer and inner glass pane, bar handle on two stand-offs."""
    d = "door"
    frame = rect_ring(-DOOR_Z, DOOR_Z, 0.0, DOOR_H)
    window = rect_ring(-WIN_W, WIN_W, WIN_U0, WIN_U1)
    return [
        extrude("door-frame", d, "enamel-white-aged", 0.9, frame, "zx", 0.0, DOOR_T, holes=[window], kg=3.07, com=(DOOR_H / 2, DOOR_T / 2)),
        box("glass-outer", d, "door-glass", 0.25, WIN_U0, WIN_U1, 0.002, 0.006, -WIN_W, WIN_W, role="glazing", sheet=True),
        box("glass-inner", d, "door-glass", 0.3, WIN_U0, WIN_U1, 0.032, 0.036, -WIN_W, WIN_W, role="glazing", sheet=True),
        box("standoff-r", d, "chrome-steel", 0.7, HANDLE_U - 0.010, HANDLE_U + 0.010, HANDLE_V, 0.0, -STANDOFF_Z - 0.008, -STANDOFF_Z + 0.008, role="detail", kg=0.05),
        box("standoff-l", d, "chrome-steel", 0.7, HANDLE_U - 0.010, HANDLE_U + 0.010, HANDLE_V, 0.0, STANDOFF_Z - 0.008, STANDOFF_Z + 0.008, role="detail", kg=0.05),
        capsule("handle-bar", d, "chrome-steel", 0.75, (HANDLE_U, HANDLE_V, -HANDLE_Z), (HANDLE_U, HANDLE_V, HANDLE_Z), HANDLE_R, slender=True, kg=0.21),
    ]


def door_100():
    """🚪 Oven door at 1:100: frame with the window opening, one glass pane, handle bar on two stand-offs."""
    d = "door"
    return [
        extrude("door-frame", d, "enamel-white-aged", 0.9, rect_ring(-DOOR_Z, DOOR_Z, 0.0, DOOR_H), "zx", 0.0, DOOR_T, holes=[rect_ring(-WIN_W, WIN_W, WIN_U0, WIN_U1)]),
        box("glass-outer", d, "door-glass", 0.25, WIN_U0, WIN_U1, 0.002, 0.006, -WIN_W, WIN_W, role="glazing", sheet=True),
        box("standoff-r", d, "chrome-steel", 0.7, HANDLE_U - 0.010, HANDLE_U + 0.010, HANDLE_V, 0.0, -STANDOFF_Z - 0.008, -STANDOFF_Z + 0.008, role="detail"),
        box("standoff-l", d, "chrome-steel", 0.7, HANDLE_U - 0.010, HANDLE_U + 0.010, HANDLE_V, 0.0, STANDOFF_Z - 0.008, STANDOFF_Z + 0.008, role="detail"),
        capsule("handle-bar", d, "chrome-steel", 0.75, (HANDLE_U, HANDLE_V, -HANDLE_Z), (HANDLE_U, HANDLE_V, HANDLE_Z), HANDLE_R, slender=True),
    ]


def door_200():
    """🚪 Oven door at 1:200: one 40 mm slab with the glass sheet in its face, the handle as one 30 x 45 mm grip block."""
    d = "door"
    return [
        box("door-slab", d, "enamel-white-aged", 0.9, 0.0, DOOR_H, 0.004, DOOR_T, -DOOR_Z, DOOR_Z),
        box("glass-outer", d, "door-glass", 0.25, WIN_U0, WIN_U1, 0.0, 0.004, -WIN_W, WIN_W, role="glazing", sheet=True),
        extrude("door-face", d, "enamel-white-aged", 0.9, rect_ring(-DOOR_Z, DOOR_Z, 0.0, DOOR_H), "zx", 0.0, 0.004, holes=[rect_ring(-WIN_W, WIN_W, WIN_U0, WIN_U1)], sheet=True),
        box("handle-block", d, "chrome-steel", 0.75, HANDLE_U - 0.015, HANDLE_U + 0.015, -0.045, 0.0, -0.21, 0.21),
    ]


def plates_50():
    """🔥 Four cast-iron hotplates in the hob holes and their pressed stainless trim rings."""
    out = []
    for key, x, z, rp, ro, _, _ in PLATES:
        out.append(revolve(f"plate-{key}", "carcass", "cast-iron-aged", 0.12, plate_profile(rp, 50), [0, 1, 0], (x, TOP_Y1, z)))
        out.append(revolve(f"ring-{key}", "carcass", "stainless-dulled", 0.8, ring_profile(rp, ro), [0, 1, 0], (x, TOP_Y1, z), role="detail", slender=True))
    return out


def knobs_50():
    """🎛️ Six control knobs on the fascia: five original white ones and the black replacement on the second hotplate."""
    out = []
    for i, z in enumerate(KNOB_ZS):
        old = abs(z - REPLACED_KNOB) < 1e-9
        out.append(revolve(f"knob-{i + 1}", "carcass", "pp-black" if old else "abs-knob-aged", 0.12 if old else 0.85, KNOB_OLD if old else KNOB, [1, 0, 0], (FASCIA_X1, KNOB_Y, z), role="detail", kg=0.025))
    return out


def body_50():
    """🔍 Body 1:50: the cooker as built in true sheet gauges, the oven muffle in its wool jacket, drawer, tray, door, hotplates, knobs, cable coil, on the half pallet."""
    c = "carcass"
    hob_holes = [circle_ring(z, x, rp, 16) for _, x, z, rp, _, _, _ in PLATES]
    frame = rect_ring(-ZI, ZI, HINGE_Y, FASCIA_Y0)
    out = pallet_50()
    out += [revolve(f"foot-{i + 1}", c, "pp-black", 0.15, FOOT, [0, 1, 0], (x, 0.0, z)) for i, (x, z) in enumerate(FEET)]
    out += [
        box("side-r", c, "enamel-white-aged", 0.88, 0.0, X_SIDE, FOOT_H, TOP_Y0, -ZO, -ZI, sheet=True),
        box("side-l", c, "enamel-white-aged", 0.88, 0.0, X_SIDE, FOOT_H, TOP_Y0, ZI, ZO, sheet=True),
        box("back", c, "steel-galvanised", 0.7, 0.0, 0.0007, FOOT_H, TOP_Y0, -ZI, ZI, sheet=True),
        box("base", c, "steel-galvanised", 0.6, 0.0007, 0.5595, FOOT_H, BASE_Y1, -ZI, ZI, sheet=True),
        extrude("top-plate", c, "enamel-hob-stained", 0.82, rect_ring(-ZO, ZO, 0.0, TOP_X1), "zx", TOP_Y0, TOP_Y1, holes=hob_holes, sheet=True),
        box("fascia", c, "enamel-white-aged", 0.86, FASCIA_X0, FASCIA_X1, FASCIA_Y0, TOP_Y0, -ZI, ZI, kg=0.76),
        extrude("front-frame", c, "enamel-white-aged", 0.8, [(v, w) for w, v in frame], "yz", FRAME_X0, X_SIDE, holes=[[(v, w) for w, v in rect_ring(-CAV_Z, CAV_Z, CAV_Y0, CAV_Y1)]], sheet=True),
        extrude("muffle", c, "enamel-oven-dark", 0.2, [(v, w) for w, v in rect_ring(-CAV_Z - 0.001, CAV_Z + 0.001, CAV_Y0 - 0.001, CAV_Y1 + 0.001)], "yz", CAV_X0, FRAME_X0, holes=[[(v, w) for w, v in rect_ring(-CAV_Z, CAV_Z, CAV_Y0, CAV_Y1)]]),
        box("muffle-back", c, "enamel-oven-dark", 0.2, CAV_X0 - 0.001, CAV_X0, CAV_Y0 - 0.001, CAV_Y1 + 0.001, -CAV_Z - 0.001, CAV_Z + 0.001, sheet=True),
        extrude("insulation", c, "mineral-wool", 0.7, [(v, w) for w, v in rect_ring(-INS_Z, INS_Z, INS_Y0, INS_Y1)], "yz", INS_X0, FRAME_X0, holes=[[(v, w) for w, v in rect_ring(-CAV_Z - 0.001, CAV_Z + 0.001, CAV_Y0 - 0.001, CAV_Y1 + 0.001)]], role="detail"),
        box("insulation-back", c, "mineral-wool", 0.7, INS_X0, CAV_X0 - 0.001, INS_Y0, INS_Y1, -INS_Z, INS_Z, role="detail"),
        box("controls", c, "switchgear", 0.4, 0.44, FASCIA_X0, 0.70, 0.84, -0.22, 0.22, role="detail", kg=2.0),
        capsule("element-r", c, "tubular-element", 0.3, (CAV_X0, 0.61, -0.15), (0.52, 0.61, -0.15), 0.004, role="detail", slender=True, kg=0.12),
        capsule("element-l", c, "tubular-element", 0.3, (CAV_X0, 0.61, 0.15), (0.52, 0.61, 0.15), 0.004, role="detail", slender=True, kg=0.12),
        capsule("element-front", c, "tubular-element", 0.3, (0.52, 0.61, -0.15), (0.52, 0.61, 0.15), 0.004, role="detail", slender=True, kg=0.06),
        revolve("cable-coil", c, "pvc-cable", 0.3, coil_profile(), [1, 0, 0], COIL, role="detail"),
    ]
    for i, y in enumerate(RAIL_LEVELS):
        for side, sgn in (("r", -1), ("l", 1)):
            z0, z1 = sorted((sgn * (CAV_Z - 0.008), sgn * CAV_Z))
            out.append(box(f"rail-{i + 1}-{side}", c, "enamel-oven-dark", 0.25, 0.15, 0.54, y - 0.008, y, z0, z1, role="detail", kg=0.0))
    for side, sgn in (("r", -1), ("l", 1)):
        z0, z1 = sorted((sgn * 0.200, sgn * 0.210))
        out.append(box(f"runner-{side}", c, "steel-galvanised", 0.5, BOX_X0, 0.55, BASE_Y1, BOX_Y0, z0, z1, role="detail", kg=0.1))
    out += plates_50() + knobs_50()
    dr = "drawer"
    out += [
        box("drawer-front", dr, "enamel-white-aged", 0.86, DRAWER_X0, DRAWER_X1, DRAWER_Y0, DRAWER_Y1, -DRAWER_Z, DRAWER_Z, kg=0.68),
        box("drawer-bottom", dr, "steel-galvanised", 0.65, BOX_X0 + 0.0006, DRAWER_X0, BOX_Y0, BOX_Y0 + 0.0006, -BOX_Z + 0.0006, BOX_Z - 0.0006, sheet=True),
        box("drawer-side-r", dr, "steel-galvanised", 0.65, BOX_X0, DRAWER_X0, BOX_Y0, BOX_Y1, -BOX_Z, -BOX_Z + 0.0006, sheet=True),
        box("drawer-side-l", dr, "steel-galvanised", 0.65, BOX_X0, DRAWER_X0, BOX_Y0, BOX_Y1, BOX_Z - 0.0006, BOX_Z, sheet=True),
        box("drawer-back", dr, "steel-galvanised", 0.65, BOX_X0, BOX_X0 + 0.0006, BOX_Y0, BOX_Y1, -BOX_Z + 0.0006, BOX_Z - 0.0006, sheet=True),
    ]
    tr = "tray"
    out += [
        box("tray-bottom", tr, "enamel-oven-dark", 0.25, TRAY_X0, TRAY_X1, TRAY_Y0, TRAY_Y0 + 0.0008, -TRAY_Z, TRAY_Z, sheet=True),
        extrude("tray-rim", tr, "enamel-oven-dark", 0.25, rect_ring(-TRAY_Z, TRAY_Z, TRAY_X0, TRAY_X1), "zx", TRAY_Y0 + 0.0008, TRAY_Y0 + TRAY_H, holes=[rect_ring(-TRAY_Z + 0.0008, TRAY_Z - 0.0008, TRAY_X0 + 0.0008, TRAY_X1 - 0.0008)]),
    ]
    return out + door_50()


def body_100():
    """🔎 Body 1:100: the sheet casing becomes solid masses around the true oven cavity and a hollow rear shell where the real cooker is empty behind the muffle; plates carry their rings, knobs are three-point profiles, the drawer is one closed box."""
    c = "carcass"
    hob_holes = [circle_ring(z, x, ro, 16) for _, x, z, rp, ro, _, _ in PLATES]
    u_bay = [(FOOT_H, -ZO), (FOOT_H, ZO), (HINGE_Y, ZO), (HINGE_Y, 0.234), (0.031, 0.234), (0.031, -0.234), (HINGE_Y, -0.234), (HINGE_Y, -ZO)]
    out = pallet_100()
    out += [box(f"foot-{i + 1}", c, "pp-black", 0.15, x - 0.015, x + 0.015, 0.0, FOOT_H, z - 0.015, z + 0.015) for i, (x, z) in enumerate(FEET)]
    out += [
        box("carcass-back", c, "enamel-white-aged", 0.88, 0.0, 0.015, FOOT_H, 0.835, -ZO, ZO),
        extrude("carcass-rear", c, "enamel-white-aged", 0.88, [(v, w) for w, v in rect_ring(-ZO, ZO, FOOT_H, 0.835)], "yz", 0.015, CAV_X0, holes=[[(v, w) for w, v in rect_ring(-0.235, 0.235, 0.030, 0.820)]]),
        extrude("carcass-bay", c, "enamel-white-aged", 0.88, u_bay, "yz", CAV_X0, X_SIDE),
        extrude("carcass-oven", c, "enamel-white-aged", 0.88, [(v, w) for w, v in rect_ring(-ZO, ZO, HINGE_Y, FASCIA_Y0)], "yz", CAV_X0, X_SIDE, holes=[[(v, w) for w, v in rect_ring(-CAV_Z, CAV_Z, CAV_Y0, CAV_Y1)]]),
        box("carcass-upper", c, "enamel-white-aged", 0.88, CAV_X0, FASCIA_X0, FASCIA_Y0, 0.835, -ZO, ZO),
        box("fascia", c, "enamel-white-aged", 0.86, FASCIA_X0, FASCIA_X1, FASCIA_Y0, 0.835, -ZO, ZO),
        extrude("hob-slab", c, "enamel-hob-stained", 0.82, rect_ring(-ZO, ZO, 0.0, TOP_X1), "zx", 0.835, TOP_Y1, holes=hob_holes),
        box("rail-r", c, "enamel-oven-dark", 0.25, 0.15, 0.54, 0.435, TRAY_Y0, -CAV_Z, -CAV_Z + 0.015, role="detail"),
        box("rail-l", c, "enamel-oven-dark", 0.25, 0.15, 0.54, 0.435, TRAY_Y0, CAV_Z - 0.015, CAV_Z, role="detail"),
        revolve("cable-coil", c, "pvc-cable", 0.3, coil_profile(8), [1, 0, 0], COIL, role="detail"),
    ]
    for key, x, z, rp, ro, _, _ in PLATES:
        out.append(revolve(f"plate-{key}", c, "cast-iron-aged", 0.12, plate_profile(rp, 100), [0, 1, 0], (x, TOP_Y1, z)))
    for i, z in enumerate(KNOB_ZS):
        old = abs(z - REPLACED_KNOB) < 1e-9
        out.append(revolve(f"knob-{i + 1}", c, "pp-black" if old else "abs-knob-aged", 0.12 if old else 0.85, [(0.0, 0.017), (0.024, 0.017), (0.0245, 0.0)] if old else KNOB_100, [1, 0, 0], (FASCIA_X1, KNOB_Y, z), role="detail"))
    out += [
        box("drawer-front", "drawer", "enamel-white-aged", 0.86, DRAWER_X0, DRAWER_X1, DRAWER_Y0, DRAWER_Y1, -DRAWER_Z, DRAWER_Z),
        box("drawer-box", "drawer", "steel-galvanised", 0.65, BOX_X0, DRAWER_X0, 0.031, BOX_Y1, -0.234, 0.234),
        box("tray", "tray", "enamel-oven-dark", 0.25, TRAY_X0, TRAY_X1, TRAY_Y0, TRAY_Y0 + 0.016, -CAV_Z + 0.015, CAV_Z - 0.015),
    ]
    return out + door_100()


def body_200():
    """🗺️ Body 1:200: carcass of four masses around the oven cavity, drawer block, slab door with grip block, hotplates as flat discs in the hob slab; knobs, feet, cable and runners are under 30 mm."""
    c = "carcass"
    hob_holes = [circle_ring(z, x, ro, 12) for _, x, z, rp, ro, _, _ in PLATES]
    u_low = [(0.0, -ZO), (0.0, ZO), (HINGE_Y, ZO), (HINGE_Y, 0.22), (0.03, 0.22), (0.03, -0.22), (HINGE_Y, -0.22), (HINGE_Y, -ZO)]
    out = pallet_200()
    out += [
        extrude("carcass-low", c, "enamel-white-aged", 0.88, u_low, "yz", 0.0, 0.55),
        box("carcass-back", c, "enamel-white-aged", 0.88, 0.0, CAV_X0, HINGE_Y, 0.82, -ZO, ZO),
        extrude("carcass-oven", c, "enamel-white-aged", 0.88, [(v, w) for w, v in rect_ring(-ZO, ZO, HINGE_Y, FASCIA_Y0)], "yz", CAV_X0, X_SIDE, holes=[[(v, w) for w, v in rect_ring(-CAV_Z, CAV_Z, CAV_Y0, CAV_Y1)]]),
        box("carcass-upper", c, "enamel-white-aged", 0.88, CAV_X0, 0.538, FASCIA_Y0, 0.82, -ZO, ZO),
        box("fascia", c, "enamel-white-aged", 0.86, 0.538, FASCIA_X1, FASCIA_Y0, 0.82, -ZO, ZO),
        extrude("hob-slab", c, "enamel-hob-stained", 0.82, rect_ring(-ZO, ZO, 0.0, TOP_X1), "zx", 0.82, TOP_Y1, holes=hob_holes),
    ]
    for key, x, z, rp, ro, _, _ in PLATES:
        out.append(extrude(f"plate-{key}", c, "cast-iron-aged", 0.12, circle_ring(z, x, ro, 12), "zx", 0.82, PLATE_TOP))
    out += [
        box("drawer-block", "drawer", "enamel-white-aged", 0.86, 0.0, DRAWER_X1, 0.03, DRAWER_Y1, -0.22, 0.22),
        box("drawer-front", "drawer", "enamel-white-aged", 0.86, 0.55, DRAWER_X1, 0.03, DRAWER_Y1, -DRAWER_Z, DRAWER_Z),
        box("tray", "tray", "enamel-oven-dark", 0.25, TRAY_X0, TRAY_X1, 0.435, 0.465, -CAV_Z, CAV_Z),
    ]
    return out + door_200()


def clean(lst):
    """🧽 Strips the private helper keys."""
    return [{k: v for k, v in s.items() if not k.startswith("_")} for s in lst]


BODY = {"50": body_50(), "100": body_100(), "200": body_200()}
DENSITY = {"spruce-pallet": SPRUCE, "enamel-white-aged": STEEL, "enamel-hob-stained": STEEL, "enamel-oven-dark": STEEL, "steel-galvanised": STEEL, "cast-iron-aged": CAST_IRON, "stainless-dulled": 7900.0, "door-glass": GLASS, "chrome-steel": STEEL, "pp-black": PP, "abs-knob-aged": 1050.0, "mineral-wool": WOOL, "switchgear": 1000.0, "tubular-element": 5500.0, "pvc-cable": 1000.0}


def masses():
    """⚖️ Mass and centre of mass per bone from the 1:50 solids: section x density, or the stated assembly mass of a hollow pressed part."""
    out = {}
    for s in BODY["50"]:
        m = s.get("_kg", s["_vol"] * DENSITY[s["material"]])
        cx, cy = s["_com"]
        e = out.setdefault(s["bone"], [0.0, 0.0, 0.0])
        e[0] += m
        e[1] += m * cx
        e[2] += m * cy
    return {b: (v[0], v[1] / v[0], v[2] / v[0]) for b, v in out.items()}


MASS = masses()
COOKER_KG = sum(v[0] for b, v in MASS.items() if not b.startswith("pallet"))
TOTAL = sum(v[0] for v in MASS.values())
