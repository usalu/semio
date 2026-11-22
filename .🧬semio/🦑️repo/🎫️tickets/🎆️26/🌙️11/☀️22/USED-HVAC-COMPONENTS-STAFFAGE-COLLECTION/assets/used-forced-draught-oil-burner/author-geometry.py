"""🔥 Hand-chosen numbers and solid builders of the used forced-draught oil burner (stored in its crate).

All lengths in metres. Burner bone frame: u along the blast tube towards the boiler (world +x), v up, w left (world +z).
Crate bone frame = world frame (x ahead, y up, z left). See `author-asset.py` for the document writer.
"""
import math

XB, YB = 0.36, 0.193
HOSE_ORIGIN = (-0.20, -0.085)
HOSE_Z = -0.126
HOSE_REST_DEG = -5.2
HOSE_LEN, HOSE_R, CAP_LEN, CAP_R = 0.34, 0.008, 0.03, 0.0115
DAMPER_ORIGIN = (-0.27, 0.04)
CRATE_L, CRATE_W, SKID_H, FLOOR_T, WALL_T, WALL_TOP = 0.62, 0.42, 0.045, 0.018, 0.018, 0.175
LIFT_DY, INSTALL_DY, CRATE_ASIDE = 0.35, 0.457, -0.80


def r6(x):
    """🔢 Round to micrometres for stable JSON."""
    return round(x + 0.0, 6)


def arc(cx, cy, r, a0, a1, n):
    """🌀 Points on an arc from angle a0 to a1 (degrees), both ends included."""
    return [[r6(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n))), r6(cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n)))] for i in range(n + 1)]


def circle(cx, cy, r, n=32):
    """⭕ Closed polygon of a circle without the repeated first point."""
    return [[r6(cx + r * math.cos(2 * math.pi * i / n)), r6(cy + r * math.sin(2 * math.pi * i / n))] for i in range(n)]


def housing_profile(fine=True):
    """🏠 Side profile (u, v) of the die-cast fan housing with its rear scroll and front chamfers."""
    n = 6 if fine else 3
    pts = [[-0.035, -0.095], [-0.035, 0.08], [-0.065, 0.11], [-0.235, 0.11]]
    pts += arc(-0.235, 0.075, 0.035, 90, 180, n)[1:]
    pts += [[-0.27, -0.085]]
    pts += arc(-0.225, -0.085, 0.045, 180, 270, n)[1:]
    pts += [[-0.075, -0.13]]
    return [[r6(a), r6(b)] for a, b in pts]


def box(id, bone, material, tone, centre, size, region=None, role="body", radius=None, **extra):
    """📦 A box solid centred at `centre` (bone frame)."""
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r6(v) for v in size], "transform": {"translation": [r6(v) for v in centre]}}
    if radius:
        s["cornerRadius"] = radius
    if region:
        s["region"] = region
    s.update(extra)
    return s


def revolve(id, bone, material, tone, profile, axis, origin, region=None, role="body", **extra):
    """🔄 A revolved solid (profile [axial, radial])."""
    s = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[r6(a), r6(b)] for a, b in profile], "axis": axis, "origin": [r6(v) for v in origin]}
    if region:
        s["region"] = region
    s.update(extra)
    return s


def extrude(id, bone, material, tone, profile, plane, a, b, region=None, role="body", holes=None, **extra):
    """🧱 An extruded solid."""
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[r6(x), r6(y)] for x, y in profile], "plane": plane, "from": r6(a), "to": r6(b)}
    if holes:
        s["holes"] = [[[r6(x), r6(y)] for x, y in h] for h in holes]
    if region:
        s["region"] = region
    s.update(extra)
    return s


def capsule(id, bone, material, tone, p0, p1, r, region=None, role="body", **extra):
    """💊 A capsule solid."""
    s = {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": [r6(v) for v in p0], "p1": [r6(v) for v in p1], "r0": r, "r1": r}
    if region:
        s["region"] = region
    s.update(extra)
    return s


def rounded_rect(x0, y0, x1, y1, r, n=3):
    """▢ Rounded rectangle polygon (counter-clockwise)."""
    pts = []
    pts += arc(x1 - r, y0 + r, r, 270, 360, n)
    pts += arc(x1 - r, y1 - r, r, 0, 90, n)
    pts += arc(x0 + r, y1 - r, r, 90, 180, n)
    pts += arc(x0 + r, y0 + r, r, 180, 270, n)
    return pts


def slot(cy, cz, half_len, half_h, n=4):
    """🕳️ Rounded hand-hold slot in the (v, w) plane of an end board."""
    r = half_h
    ends = [(cz + half_len - r, -90, 90), (cz - half_len + r, 90, 270)]
    return [[r6(cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))), r6(wc + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)))] for wc, a0, a1 in ends for i in range(n + 1)]


def crate_solids(scale):
    """🪵 Body solids of the open pine crate with two skids, its floor boards, walls, posts and hand-hold slots."""
    W = CRATE_W / 2
    s = []
    if scale == 500:
        s.append(box("crate", "crate", "pine", 0.6, (CRATE_L / 2, WALL_TOP / 2, 0), (CRATE_L, WALL_TOP, CRATE_W), region="crate"))
        return s
    if scale == 200:
        sh, ft, wt = 0.033, 0.030, 0.030
        s.append(box("skids", "crate", "pine", 0.5, (CRATE_L / 2, sh / 2, 0.1725), (CRATE_L, sh, 0.075), region="crate", repeat={"count": 2, "translation": [0, 0, -0.345]}))
        outer = [[0, sh], [CRATE_L, sh], [CRATE_L, WALL_TOP], [CRATE_L - wt, WALL_TOP], [CRATE_L - wt, sh + ft], [wt, sh + ft], [wt, WALL_TOP], [0, WALL_TOP]]
        s.append(extrude("crate-tray", "crate", "pine", 0.62, outer, "xy", -W, W, region="crate"))
        s.append(box("side-wall-left", "crate", "pine", 0.64, (CRATE_L / 2, (sh + ft + WALL_TOP) / 2, W - wt / 2), (CRATE_L - 2 * wt, WALL_TOP - sh - ft, wt), region="crate"))
        s.append(box("side-wall-right", "crate", "pine", 0.64, (CRATE_L / 2, (sh + ft + WALL_TOP) / 2, -W + wt / 2), (CRATE_L - 2 * wt, WALL_TOP - sh - ft, wt), region="crate"))
        return s
    s.append(box("skid-left", "crate", "pine", 0.48, (CRATE_L / 2, SKID_H / 2, 0.1725), (CRATE_L, SKID_H, 0.075), region="crate"))
    s.append(box("skid-right", "crate", "pine", 0.5, (CRATE_L / 2, SKID_H / 2, -0.1725), (CRATE_L, SKID_H, 0.075), region="crate"))
    yf = SKID_H + FLOOR_T / 2
    if scale == 50:
        s.append(box("floor-boards", "crate", "pine", 0.66, (CRATE_L / 2, yf, -0.144), (CRATE_L - 2 * WALL_T, FLOOR_T, 0.096), region="crate", repeat={"count": 4, "translation": [0, 0, 0.096]}))
    else:
        s.append(box("floor-boards", "crate", "pine", 0.66, (CRATE_L / 2, yf, 0), (CRATE_L - 2 * WALL_T, FLOOR_T, 2 * (W - WALL_T)), region="crate"))
    hw = (WALL_TOP - SKID_H) / 2 + SKID_H
    s.append(box("side-wall-left", "crate", "pine", 0.62, (CRATE_L / 2, hw, W - WALL_T / 2), (CRATE_L, WALL_TOP - SKID_H, WALL_T), region="crate"))
    s.append(box("side-wall-right", "crate", "pine", 0.6, (CRATE_L / 2, hw, -W + WALL_T / 2), (CRATE_L, WALL_TOP - SKID_H, WALL_T), region="crate"))
    end = [[SKID_H, -(W - WALL_T)], [SKID_H, W - WALL_T], [WALL_TOP, W - WALL_T], [WALL_TOP, -(W - WALL_T)]]
    hole = [slot(0.1425, 0.0, 0.05, 0.0125)]
    s.append(extrude("end-wall-rear", "crate", "pine", 0.58, end, "yz", 0, WALL_T, region="crate", holes=hole))
    s.append(extrude("end-wall-front", "crate", "pine", 0.58, end, "yz", CRATE_L - WALL_T, CRATE_L, region="crate", holes=hole))
    if scale == 50:
        py = (SKID_H + FLOOR_T + WALL_TOP) / 2
        ph = WALL_TOP - SKID_H - FLOOR_T
        for i, (x, z) in enumerate([(0.0355, 0.1745), (0.5845, 0.1745), (0.0355, -0.1745), (0.5845, -0.1745)]):
            s.append(box(f"corner-post-{i + 1}", "crate", "pine", 0.55, (x, py, z), (0.035, ph, 0.035), region="crate"))
    return s


def burner_solids(scale):
    """🔥 Body solids of the burner bone: housing, motor, pump, controls, flange, blast tube and dismantling traces."""
    s = []
    b = "burner"
    if scale == 500:
        s.append(extrude("housing", b, "housing-alu", 0.5, [[-0.27, -0.13], [0.008, -0.13], [0.008, 0.11], [-0.27, 0.11]], "xy", -0.152, 0.182, region="housing"))
        s.append(box("control-box", b, "polyamide", 0.25, (-0.205, 0.1375, 0), (0.12, 0.055, 0.13), region="controls"))
        s.append(revolve("blast-tube", b, "stainless-tube", 0.4, [[0, 0], [0, 0.0445], [0.142, 0.0445], [0.142, 0]], [1, 0, 0], (0.008, 0, 0), region="blast-tube"))
        return s
    s.append(extrude("housing", b, "housing-alu", 0.55, housing_profile(scale == 50), "xy", -0.097, 0.097, region="housing"))
    s.append(revolve("motor", b, "motor-steel", 0.35, [[0, 0], [0, 0.055], [0.068, 0.055], [0.076, 0.05], [0.082, 0.04], [0.085, 0]], [0, 0, 1], (-0.20, -0.02, 0.097), region="motor"))
    s.append(box("pump", b, "pump-alu", 0.7, (-0.20, -0.03, -0.1245), (0.07, 0.09, 0.055), region="pump", radius=0.006 if scale == 50 else None))
    s.append(box("control-box", b, "polyamide", 0.22, (-0.205, 0.1375, 0), (0.12, 0.055, 0.13), region="controls", radius=0.005 if scale == 50 else None))
    s.append(box("ignition-transformer", b, "polyamide", 0.3, (-0.085, 0.13, 0), (0.08, 0.04, 0.10), region="controls", radius=0.004 if scale == 50 else None))
    if scale == 200:
        s.append(revolve("flange", b, "flange-steel", 0.2, [[0, 0], [0, 0.085], [0.043, 0.085], [0.043, 0]], [1, 0, 0], (-0.035, 0, 0), region="flange"))
        s.append(revolve("blast-tube", b, "stainless-tube", 0.42, [[0, 0], [0, 0.0445], [0.142, 0.0445], [0.142, 0]], [1, 0, 0], (0.008, 0, 0), region="blast-tube"))
        s.append(revolve("solenoid-coil", b, "polyamide", 0.2, [[0, 0], [0, 0.016], [0.035, 0.016], [0.035, 0]], [0, 1, 0], (-0.20, 0.015, -0.1245), region="pump"))
        return s
    s.append(revolve("capacitor", b, "motor-steel", 0.45, [[0, 0], [0, 0.0175], [0.06, 0.0175], [0.06, 0]], [0, 0, 1], (-0.20, 0.0525, 0.107), region="motor"))
    s.append(revolve("solenoid-coil", b, "polyamide", 0.2, [[0, 0], [0, 0.016], [0.035, 0.016], [0.035, 0]], [0, 1, 0], (-0.20, 0.015, -0.1245), region="pump"))
    s.append(box("plug", b, "polyamide", 0.15, (-0.20, 0.135, -0.087), (0.03, 0.04, 0.044), region="controls"))
    if scale == 50:
        s.append(revolve("reset-button", b, "polyamide", 0.9, [[0, 0], [0, 0.009], [0.008, 0.009], [0.008, 0]], [0, 1, 0], (-0.235, 0.165, 0.03), region="controls"))
        s.append(box("cable-stub", b, "polyamide", 0.1, (-0.20, 0.0875, -0.103), (0.01, 0.055, 0.01), region="controls"))
        s.append(revolve("clamp-collar", b, "flange-steel", 0.25, [[0, 0], [0, 0.052], [0.027, 0.052], [0.027, 0]], [1, 0, 0], (-0.035, 0, 0), region="flange"))
        s.append(box("clamp-screw", b, "flange-steel", 0.15, (-0.0215, 0.059, 0), (0.016, 0.014, 0.016), region="flange"))
        bolt = [circle(0.075 * math.cos(math.radians(a)), 0.075 * math.sin(math.radians(a)), 0.0055, 12) for a in (45, 135, 225, 315)]
        disc = circle(0, 0, 0.085, 48)
        bore = circle(0, 0, 0.0445, 32)
        s.append(extrude("flange", b, "flange-steel", 0.18, disc, "yz", -0.008, 0.0, region="flange", holes=[bore] + bolt))
        s.append(extrude("gasket", b, "ceramic-fibre", 0.85, disc, "yz", 0.0, 0.008, region="gasket", holes=[bore] + bolt))
        s.append(revolve("blast-tube", b, "stainless-tube", 0.5, [[0, 0.0425], [0, 0.0445], [0.145, 0.0445], [0.145, 0.0425], [0, 0.0425]], [1, 0, 0], (-0.035, 0, 0), region="blast-tube"))
        s.append(revolve("blast-tube-tip", b, "stainless-tube", 0.32, [[0, 0.0425], [0, 0.0445], [0.04, 0.0445], [0.04, 0.0425], [0, 0.0425]], [1, 0, 0], (0.11, 0, 0), region="blast-tube"))
        s.append(revolve("combustion-head", b, "head-steel", 0.12, [[0, 0], [0, 0.0425], [0.008, 0.0425], [0.008, 0]], [1, 0, 0], (0.13, 0, 0), region="combustion-head"))
        s.append(box("type-plate", b, "housing-alu", 0.85, (-0.08, -0.065, -0.0975), (0.06, 0.04, 0.001), region="housing", texture=True))
        for i, w in enumerate((-0.115, -0.137)):
            s.append(revolve(f"nipple-{'ab'[i]}", b, "brass", 0.55, [[0, 0], [0, 0.008], [0.013, 0.008], [0.013, 0]], [0, -1, 0], (-0.20, -0.075, w), region="hose-fittings"))
    else:
        s.append(revolve("flange", b, "flange-steel", 0.2, [[0, 0], [0, 0.052], [0.027, 0.052], [0.027, 0.085], [0.043, 0.085], [0.043, 0]], [1, 0, 0], (-0.035, 0, 0), region="flange"))
        s.append(revolve("blast-tube", b, "stainless-tube", 0.45, [[0, 0], [0, 0.0445], [0.142, 0.0445], [0.142, 0]], [1, 0, 0], (0.008, 0, 0), region="blast-tube"))
        s.append(revolve("combustion-head", b, "head-steel", 0.12, [[0, 0], [0, 0.04], [0.002, 0.04], [0.002, 0]], [1, 0, 0], (0.15, 0, 0), region="combustion-head"))
        for i, w in enumerate((-0.115, -0.137)):
            s.append(revolve(f"nipple-{'ab'[i]}", b, "brass", 0.55, [[0, 0], [0, 0.008], [0.016, 0.008], [0.016, 0]], [0, -1, 0], (-0.20, -0.075, w), region="hose-fittings"))
    return s


def hose_solids(scale):
    """🐍 The two cut and capped braided oil hoses on their swinging bone."""
    s = []
    for i, w in enumerate((0.011, -0.011)):
        n = "ab"[i]
        if scale >= 200:
            s.append(capsule(f"hose-{n}", "hoses", "braided-hose", 0.35, (0, 0, w), (HOSE_LEN + CAP_LEN - HOSE_R, 0, w), HOSE_R, region="oil-hoses", slender=True))
        else:
            s.append(capsule(f"hose-{n}", "hoses", "braided-hose", 0.35, (0, 0, w), (HOSE_LEN, 0, w), HOSE_R, region="oil-hoses"))
            s.append(revolve(f"hose-cap-{n}", "hoses", "brass", 0.6, [[0, 0], [0, CAP_R], [CAP_LEN, CAP_R], [CAP_LEN, 0]], [1, 0, 0], (HOSE_LEN, 0, w), region="hose-fittings"))
    return s


def damper_solids(scale):
    """🌬️ The moulded air-intake shutter on its hinge at the rear face."""
    if scale >= 200:
        return []
    return [box("air-shutter", "air-damper", "housing-alu", 0.42, (0.045, -0.0075, 0), (0.09, 0.015, 0.11), region="housing", radius=0.003 if scale == 50 else None)]


def body(scale):
    """🧊 All body solids of one scale."""
    return crate_solids(scale) + burner_solids(scale) + hose_solids(scale) + damper_solids(scale)
