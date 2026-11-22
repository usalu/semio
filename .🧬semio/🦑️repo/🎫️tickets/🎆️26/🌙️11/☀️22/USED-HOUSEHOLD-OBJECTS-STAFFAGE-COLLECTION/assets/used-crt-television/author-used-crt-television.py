"""📺 Authoritative chisel of the used CRT television: states every hand-chosen number and writes `🔣️.json` next to this file.

A 1990s 70 cm (28 inch, 4:3) picture-tube television of the German mid-range type on its swivel stand, as a reuse depot stores it: on a
folded moving blanket on the depot floor, the mains cable coiled and taped to the back (rest = default pose `stored`), carried by two
people under the bottom edges and set up again in a living room where it runs warm. World frame of the staffage: x ahead (the screen
looks ahead), y up, z left.

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE / "🔣️.json"

G = 9.80665
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}

BLANKET_T = 0.012
BASE_T = 0.016
TURN_T = 0.018
YC = BLANKET_T + BASE_T + TURN_T
XF = 0.15
H = 0.53
HW = 0.35
BEZEL_D = 0.115
XB = XF - BEZEL_D
DEPTH = 0.49
XR = XF - DEPTH
SCREEN = dict(y0=0.110, y1=0.500, hw=0.280, r=0.03, recess=0.012)
BEZEL_R = 0.02
REAR_FRONT = dict(yc=0.26, hh=0.26, hw=0.34)
REAR_BACK = dict(yc=0.17, hh=0.17, hw=0.20)
TRAY = dict(x0=XR + 0.05, x1=XB, t=0.012, hw=0.30)
COIL = dict(R=0.07, r=0.012, y=0.17, z=0.0)
BASE_R = 0.17
TURN_R = 0.15
BLANKET = dict(x0=XR - 0.11, x1=XF + 0.13, hz=0.45)
BAND_T = 0.0125
CARRY_DY = 0.674
CARRY_DX = 1.20
USE_DY = -BLANKET_T

RHO = {"crt-panel-glass": 2700.0, "crt-funnel-glass": 3000.0, "hips-anthracite": 1050.0, "abs-stand": 1050.0, "pvc-cable": 1400.0, "felt-blanket": 134.0, "steel-band": 7850.0}


def r6(v):
    """🔢 Rounds every float of a nested value to six significant places."""
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


def rounded_rect(a0, a1, b0, b1, r, steps):
    """⬭ A counter-clockwise rounded rectangle in (a, b) with corner radius `r` and `steps` segments per corner."""
    pts = []
    corners = [(a1 - r, b1 - r, 0.0), (a0 + r, b1 - r, 90.0), (a0 + r, b0 + r, 180.0), (a1 - r, b0 + r, 270.0)]
    for ca, cb, start in corners:
        for k in range(steps + 1):
            t = math.radians(start + 90.0 * k / steps)
            pts.append((ca + r * math.cos(t), cb + r * math.sin(t)))
    return pts


def polygon_area(pts):
    """📐 Shoelace area of a simple polygon."""
    s = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        s += x0 * y1 - x1 * y0
    return abs(s) / 2


def polygon_perimeter(pts):
    """📏 Perimeter of a closed polygon."""
    return sum(math.dist(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts)))


def blanket_section(steps):
    """🧺 The folded blanket in elevation (world x, y): four 3 mm layers with the rounded fold at the front edge."""
    b = BLANKET
    r = BLANKET_T / 2
    if steps == 0:
        return [(b["x0"], 0.0), (b["x1"], 0.0), (b["x1"], BLANKET_T), (b["x0"], BLANKET_T)]
    pts = [(b["x0"], 0.0)]
    for k in range(steps + 1):
        a = -math.pi / 2 + math.pi * k / steps
        pts.append((b["x1"] - r + r * math.cos(a), r + r * math.sin(a)))
    pts.append((b["x0"], BLANKET_T))
    return pts


def blanket_top(scale):
    """🧺 Upper face of the blanket: true 12 mm, a 22 mm strip at 1:500 so that it still prints at 1:1000."""
    return 0.022 if scale == 500 else BLANKET_T


# ---------------------------------------------------------------- 2D shapes

def shape(sid, bone, role, z, tone, geom, region=None, material=None, hatch=None, filled=True):
    """✏️ One 2D shape with its line-hierarchy role, paint order and region."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    return s


def rect(x0, y0, x1, y1, r=None):
    """▭ A rect geometry from two corners."""
    g = {"kind": "rect", "x": min(x0, x1), "y": min(y0, y1), "w": abs(x1 - x0), "h": abs(y1 - y0)}
    if r:
        g["r"] = r
    return g


def poly(points, closed=True):
    """⬠ A poly geometry."""
    return {"kind": "poly", "points": [list(p) for p in points], "closed": closed}


def lines(segments):
    """〰 A path of straight strokes."""
    return {"kind": "path", "d": " ".join(f"M {a[0]:.5f} {a[1]:.5f} L {b[0]:.5f} {b[1]:.5f}" for a, b in segments)}


def rear_side():
    """📐 Side profile of the tapered rear cover (cabinet-local X, Y): flat bottom, sloping top."""
    f, b = REAR_FRONT, REAR_BACK
    return [(XB, 0.0), (XB, f["yc"] + f["hh"]), (XR, b["yc"] + b["hh"]), (XR, 0.0)]


def rear_plan():
    """📐 Plan of the tapered rear cover (X, Z)."""
    f, b = REAR_FRONT, REAR_BACK
    return [(XB, -f["hw"]), (XB, f["hw"]), (XR, b["hw"]), (XR, -b["hw"])]


def slope_y(X):
    """📈 Height of the rear cover top along its centre line at cabinet-local X."""
    f, b = REAR_FRONT, REAR_BACK
    t = (X - XR) / (XB - XR)
    return (b["yc"] + b["hh"]) + t * ((f["yc"] + f["hh"]) - (b["yc"] + b["hh"]))


def vent_segments_side():
    """🌬 Vent slots on the sloping top seen from the side: short ticks normal to the slope."""
    out = []
    for k in range(7):
        X = XR + 0.06 + 0.03 * k
        y = slope_y(X)
        out.append(((X, y - 0.004), (X + 0.006, y - 0.016)))
    return out


def geometry(scale):
    """🖼 The individually drawn elevation, front and plan of one scale."""
    s50, s100, s200, s500 = scale == 50, scale == 100, scale == 200, scale == 500
    fine = scale in (50, 100)
    E, F, P = [], [], []
    bt = blanket_top(scale)
    steps = 6 if s50 else (2 if s100 else 0)
    bl = blanket_section(steps) if not s500 else [(BLANKET["x0"], 0.0), (BLANKET["x1"], 0.0), (BLANKET["x1"], bt), (BLANKET["x0"], bt)]

    E.append(shape("blanket", "blanket", "body", 1, 0.3, poly(bl), "blanket", "felt-blanket"))
    if s50:
        E.append(shape("blanket-layers", "blanket", "texture", 1.1, 0.2, lines([((BLANKET["x0"], 0.003 * k), (BLANKET["x0"] + 0.06, 0.003 * k)) for k in (1, 2, 3)]), "blanket", filled=False))
    if fine:
        base_pts = [(-BASE_R, 0.0), (BASE_R, 0.0), (BASE_R, BASE_T - 0.004), (BASE_R - 0.004, BASE_T), (-BASE_R + 0.004, BASE_T), (-BASE_R, BASE_T - 0.004)] if s50 else [(-BASE_R, 0.0), (BASE_R, 0.0), (BASE_R, BASE_T), (-BASE_R, BASE_T)]
        E.append(shape("base", "base", "body", 2, 0.4, poly(base_pts), "stand", "abs-stand"))
        E.append(shape("turntable", "cabinet", "body", 2.1, 0.45, rect(-TURN_R, -TURN_T, TURN_R, 0.0), "stand", "abs-stand"))
    else:
        E.append(shape("stand", "base", "body", 2, 0.42, rect(-BASE_R, 0.0, BASE_R, BASE_T + TURN_T), "stand", "abs-stand"))
    E.append(shape("rear", "cabinet", "body", 3, 0.28, poly(rear_side()), "rear-cover", "hips-anthracite"))
    if s50:
        E.append(shape("tray", "cabinet", "body", 3.1, 0.22, rect(TRAY["x0"], 0.0, TRAY["x1"], TRAY["t"]), "rear-cover", "hips-anthracite"))
    if fine:
        cx0 = XR - 2 * COIL["r"]
        E.append(shape("coil", "cabinet", "body", 3.2, 0.15, rect(cx0, COIL["y"] - COIL["R"] - COIL["r"], XR, COIL["y"] + COIL["R"] + COIL["r"]), "rear-cover", "pvc-cable"))
    bz = shape("bezel", "cabinet", "body", 4, 0.2, rect(XB, 0.0, XF, H), "bezel", "hips-anthracite")
    del bz["region"]
    bz["regions"] = ["bezel", "screen"]
    E.append(bz)
    if s50:
        E.append(shape("vents", "cabinet", "texture", 3.3, 0.2, lines(vent_segments_side()), "rear-cover", filled=False))
        E.append(shape("dust", "cabinet", "texture", 3.4, 0.4, lines([((XR + 0.05 + 0.045 * k, slope_y(XR + 0.05 + 0.045 * k) + 0.002), (XR + 0.065 + 0.045 * k, slope_y(XR + 0.065 + 0.045 * k) + 0.002)) for k in range(7)]), "rear-cover", filled=False))
        E.append(shape("e-check-sticker", "cabinet", "texture", 3.5, 0.9, lines([((XR + 0.12, 0.08), (XR + 0.17, 0.08)), ((XR + 0.17, 0.08), (XR + 0.17, 0.11)), ((XR + 0.17, 0.11), (XR + 0.12, 0.11)), ((XR + 0.12, 0.11), (XR + 0.12, 0.08))]), "rear-cover", filled=False))
        E.append(shape("coil-tape", "cabinet", "texture", 3.6, 0.8, lines([((XR - 0.026, COIL["y"] - 0.015), (XR + 0.002, COIL["y"] - 0.015)), ((XR - 0.026, COIL["y"] + 0.015), (XR + 0.002, COIL["y"] + 0.015))]), "rear-cover", filled=False))
        E.append(shape("bezel-joint", "cabinet", "detail", 4.1, 0.3, lines([((XB + 0.012, 0.0), (XB + 0.012, H))]), "bezel", filled=False))
    if s100:
        E.append(shape("vents", "cabinet", "texture", 3.3, 0.2, lines(vent_segments_side()[::2]), "rear-cover", filled=False))

    fy = lambda y: y + YC
    F.append(shape("blanket", "blanket", "body", 1, 0.3, rect(-BLANKET["hz"], 0.0, BLANKET["hz"], bt), "blanket", "felt-blanket", hatch="quilt" if s50 else None))
    if fine:
        F.append(shape("base", "base", "body", 2, 0.4, rect(-BASE_R, BLANKET_T, BASE_R, BLANKET_T + BASE_T), "stand", "abs-stand"))
        F.append(shape("turntable", "cabinet", "body", 2.1, 0.45, rect(-TURN_R, BLANKET_T + BASE_T, TURN_R, YC), "stand", "abs-stand"))
    else:
        F.append(shape("stand", "base", "body", 2, 0.42, rect(-BASE_R, BLANKET_T, BASE_R, YC), "stand", "abs-stand"))
    F.append(shape("bezel", "cabinet", "body", 4, 0.2, rect(-HW, fy(0.0), HW, fy(H), r=BEZEL_R if fine else None), "bezel", "hips-anthracite"))
    F.append(shape("screen", "cabinet" if s500 else "tube", "glazing", 5, 0.12, rect(-SCREEN["hw"], fy(SCREEN["y0"]), SCREEN["hw"], fy(SCREEN["y1"]), r=SCREEN["r"] if fine else None), "screen", "crt-panel-glass"))
    if fine:
        for side, sgn in (("l", 1), ("r", -1)):
            z0, z1 = sorted((sgn * 0.11, sgn * 0.30))
            F.append(shape(f"speaker-{side}", "cabinet", "detail", 4.2, 0.14, rect(z0, fy(0.022), z1, fy(0.088)), "bezel", hatch="perforation" if s50 else None))
        F.append(shape("control-recess", "cabinet", "detail", 4.3, 0.08, rect(-0.075, fy(0.030), 0.075, fy(0.080)), "bezel"))
    if s50:
        F.append(shape("buttons", "cabinet", "texture", 4.4, 0.5, lines([((-0.06 + 0.024 * k, fy(0.046)), (-0.06 + 0.024 * k + 0.014, fy(0.046))) for k in range(6)] + [((-0.06 + 0.024 * k, fy(0.062)), (-0.06 + 0.024 * k + 0.014, fy(0.062))) for k in range(6)]), "bezel", filled=False))
        F.append(shape("burn-in", "tube", "texture", 5.1, 0.25, lines([((-0.235, fy(0.455)), (-0.175, fy(0.455))), ((-0.175, fy(0.455)), (-0.175, fy(0.478))), ((-0.175, fy(0.478)), (-0.235, fy(0.478))), ((-0.235, fy(0.478)), (-0.235, fy(0.455)))]), "screen", filled=False))
        F.append(shape("bezel-scratches", "cabinet", "texture", 4.5, 0.4, lines([((0.16, fy(0.096)), (0.24, fy(0.101))), ((0.19, fy(0.093)), (0.23, fy(0.097))), ((-0.32, fy(0.30)), (-0.315, fy(0.36)))]), "bezel", filled=False))
        F.append(shape("power-switch", "cabinet", "detail", 4.6, 0.6, rect(0.31, fy(0.040), 0.33, fy(0.070)), "bezel"))

    P.append(shape("blanket", "blanket", "body", 1, 0.3, rect(BLANKET["x0"], -BLANKET["hz"], BLANKET["x1"], BLANKET["hz"]), "blanket", "felt-blanket", hatch="quilt" if s50 else None))
    P.append(shape("rear", "cabinet", "body", 3, 0.28, poly(rear_plan()), "rear-cover", "hips-anthracite"))
    if fine:
        P.append(shape("coil", "cabinet", "body", 3.2, 0.15, rect(XR - 2 * COIL["r"], COIL["z"] - COIL["R"] - COIL["r"], XR, COIL["z"] + COIL["R"] + COIL["r"]), "rear-cover", "pvc-cable"))
    P.append(shape("bezel", "cabinet", "body", 4, 0.2, rect(XB, -HW, XF, HW), "bezel", "hips-anthracite"))
    if fine:
        vz = 0.14 if s50 else 0.12
        step = 0.03 if s50 else 0.06
        P.append(shape("vents", "cabinet", "texture", 3.3, 0.2, lines([((XR + 0.06 + step * k, -vz), (XR + 0.06 + step * k, vz)) for k in range(int(0.18 / step) + 1)]), "rear-cover", filled=False))
    if s50:
        P.append(shape("dust", "cabinet", "texture", 3.4, 0.4, lines([((XR + 0.04, 0.17), (XR + 0.30, 0.17)), ((XR + 0.04, -0.17), (XR + 0.30, -0.17)), ((XR + 0.10, 0.155), (XR + 0.22, 0.155))]), "rear-cover", filled=False))
        P.append(shape("price-label", "cabinet", "texture", 4.1, 0.9, lines([((XB + 0.02, 0.20), (XB + 0.08, 0.20)), ((XB + 0.08, 0.20), (XB + 0.08, 0.26)), ((XB + 0.08, 0.26), (XB + 0.02, 0.26)), ((XB + 0.02, 0.26), (XB + 0.02, 0.20)), ((XB + 0.03, 0.23), (XB + 0.07, 0.23))]), "bezel", filled=False))
        P.append(shape("top-scratches", "cabinet", "texture", 4.2, 0.4, lines([((XB + 0.03, -0.10), (XB + 0.09, -0.02)), ((XB + 0.05, -0.12), (XB + 0.10, -0.06))]), "bezel", filled=False))
    return {"elevation": E, "front": F, "plan": P}


# ---------------------------------------------------------------- 3D solids

def box(sid, bone, lo, hi, material, tone, role="body", **extra):
    """📦 An axis-aligned box from two local corners."""
    centre = [(lo[i] + hi[i]) / 2 for i in range(3)]
    size = [abs(hi[i] - lo[i]) for i in range(3)]
    s = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": centre}}
    s.update(extra)
    return s


def extrude(sid, bone, plane, profile, frm, to, material, tone, role="body", holes=None, **extra):
    """🧱 A prismatic solid in the frame of its bone."""
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "plane": plane, "profile": [list(p) for p in profile], "from": min(frm, to), "to": max(frm, to)}
    if holes:
        s["holes"] = [[list(p) for p in h] for h in holes]
    s.update(extra)
    return s


def revolve(sid, bone, profile, axis, origin, material, tone, role="body", **extra):
    """🌀 A solid of revolution in the frame of its bone."""
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [list(p) for p in profile], "axis": list(axis), "origin": list(origin)}
    s.update(extra)
    return s


def rear_loft(sid, n, material="hips-anthracite", tone=0.28, role="body", **extra):
    """🔻 The tapered rear cover: a loft of two rounded-rectangle sections (superellipse exponent `n`), flat bottom, sloping top."""
    rot = [0.5, 0.5, 0.5, 0.5]
    f, b = REAR_FRONT, REAR_BACK
    s = {"id": sid, "kind": "loft", "bone": "cabinet", "material": material, "role": role, "tone": tone, "sections": [
        {"at": [XR, b["yc"], 0.0], "rx": b["hh"], "ry": b["hw"], "exponent": n, "rotation": rot},
        {"at": [XB, f["yc"], 0.0], "rx": f["hh"], "ry": f["hw"], "exponent": n, "rotation": rot}], "capStart": True, "capEnd": True}
    s.update(extra)
    return s


def bezel_solid(scale):
    """🖼 The front bezel: a moulded frame around the screen opening."""
    fine = scale in (50, 100)
    steps = 4 if scale == 50 else (2 if scale == 100 else 1)
    r_out = BEZEL_R if fine else 0.0
    outer = rounded_rect(0.0, H, -HW, HW, r_out, steps) if fine else [(0.0, -HW), (H, -HW), (H, HW), (0.0, HW)]
    if scale == 500:
        return box("bezel", "cabinet", (XB, 0.0, -HW), (XF, H, HW), "hips-anthracite", 0.2)
    return extrude("bezel", "cabinet", "yz", outer, XB, XF, "hips-anthracite", 0.2, holes=[screen_profile(scale)])


def screen_profile(scale):
    """🖥 The screen opening (v = Y, w = Z), rounded at 1:50 and 1:100."""
    s = SCREEN
    if scale in (50, 100):
        return rounded_rect(s["y0"], s["y1"], -s["hw"], s["hw"], s["r"], 4 if scale == 50 else 2)
    return [(s["y0"], -s["hw"]), (s["y1"], -s["hw"]), (s["y1"], s["hw"]), (s["y0"], s["hw"])]


def screen_solid(scale):
    """🖥 The faceplate of the picture tube, recessed 12 mm behind the bezel face."""
    return extrude("screen", "tube", "yz", screen_profile(scale), XB, XF - SCREEN["recess"], "crt-panel-glass", 0.12, role="glazing")


def coil_solid(scale):
    """➰ The mains cable coiled to a ring of 0.164 m and taped to the back plate."""
    n = 16 if scale == 50 else 8
    r = COIL["r"]
    prof = [(r * math.cos(2 * math.pi * k / n), COIL["R"] + r * math.sin(2 * math.pi * k / n)) for k in range(n)]
    prof.append(prof[0])
    return revolve("coil", "cabinet", prof, [1, 0, 0], [XR - r, COIL["y"], COIL["z"]], "pvc-cable", 0.15)


def stand_solids(scale):
    """🔘 The swivel stand: base plate and turntable at 1:50 and 1:100, one puck at 1:200 and 1:500."""
    if scale == 50:
        base = [(0.0, 0.0), (0.0, BASE_R), (BASE_T - 0.004, BASE_R), (BASE_T, BASE_R - 0.004), (BASE_T, 0.0)]
        turn = [(-TURN_T, 0.0), (-TURN_T, TURN_R), (-0.004, TURN_R), (0.0, TURN_R - 0.004), (0.0, 0.0)]
        return [revolve("base", "base", base, [0, 1, 0], [0, 0, 0], "abs-stand", 0.4), revolve("turntable", "cabinet", turn, [0, 1, 0], [0, 0, 0], "abs-stand", 0.45)]
    if scale == 100:
        return [revolve("base", "base", [(0.0, 0.0), (0.0, BASE_R), (BASE_T, BASE_R), (BASE_T, 0.0)], [0, 1, 0], [0, 0, 0], "abs-stand", 0.4),
                revolve("turntable", "cabinet", [(-TURN_T, 0.0), (-TURN_T, TURN_R), (0.0, TURN_R), (0.0, 0.0)], [0, 1, 0], [0, 0, 0], "abs-stand", 0.45)]
    return [revolve("stand", "base", [(0.0, 0.0), (0.0, BASE_R), (BASE_T + TURN_T, BASE_R), (BASE_T + TURN_T, 0.0)], [0, 1, 0], [0, 0, 0], "abs-stand", 0.42)]


def blanket_solid(scale):
    """🧺 The folded moving blanket of one scale."""
    steps = 6 if scale == 50 else (2 if scale == 100 else 0)
    prof = blanket_section(steps) if scale != 500 else [(BLANKET["x0"], 0.0), (BLANKET["x1"], 0.0), (BLANKET["x1"], 0.022), (BLANKET["x0"], 0.022)]
    extra = {"sheet": True} if scale in (100, 200) else {}
    return extrude("blanket", "blanket", "xy", prof, -BLANKET["hz"], BLANKET["hz"], "felt-blanket", 0.3, **extra)


def rear_hw(X):
    """↔ Half width of the rear cover at cabinet-local X."""
    t = (X - XR) / (XB - XR)
    return REAR_BACK["hw"] + t * (REAR_FRONT["hw"] - REAR_BACK["hw"])


def tray_plan():
    """🗺 The bottom tray in plan as a `zx` profile (w, u): it follows the taper of the rear cover 30 to 40 mm inside its edge."""
    return [(-(rear_hw(TRAY["x0"]) - 0.03), TRAY["x0"]), (-(rear_hw(TRAY["x1"]) - 0.04), TRAY["x1"]), (rear_hw(TRAY["x1"]) - 0.04, TRAY["x1"]), (rear_hw(TRAY["x0"]) - 0.03, TRAY["x0"])]


def body_solids(scale):
    """🧊 The body list of one scale."""
    out = [blanket_solid(scale)] + stand_solids(scale)
    out.append(rear_loft("rear", {50: 8, 100: 8, 200: 6, 500: 4}[scale]))
    if scale == 50:
        out.append(extrude("tray", "cabinet", "zx", tray_plan(), 0.0, TRAY["t"], "hips-anthracite", 0.22))
    out.append(bezel_solid(scale))
    if scale != 500:
        out.append(screen_solid(scale))
    if scale in (50, 100):
        out.append(coil_solid(scale))
    return out


# ---------------------------------------------------------------- masses

def masses():
    """⚖ Bone masses from section x density and their centres (local frames)."""
    s = SCREEN
    face_w, face_h = 2 * s["hw"] + 0.03, s["y1"] - s["y0"] + 0.06
    m_face = face_w * face_h * 0.0135 * RHO["crt-panel-glass"]
    skirt_len = 2 * (face_w + face_h)
    m_skirt = skirt_len * 0.075 * 0.011 * RHO["crt-panel-glass"]
    funnel_len = 0.33
    neck_d = 0.036
    slant = math.sqrt(funnel_len ** 2 + ((face_w + face_h) / 4 - neck_d / 2) ** 2)
    funnel_area = (skirt_len + math.pi * neck_d) / 2 * slant
    m_funnel = funnel_area * 0.006 * RHO["crt-funnel-glass"]
    m_neck = math.pi * 0.029 * 0.16 * 0.0026 * RHO["crt-funnel-glass"] + 0.05
    m_band = skirt_len * 0.025 * 0.0015 * RHO["steel-band"]
    m_yoke = 0.90
    m_degauss = 0.35
    xs = XF - s["recess"]
    yc = (s["y0"] + s["y1"]) / 2
    tube_parts = [(m_face, xs - 0.007), (m_skirt, xs - 0.05), (m_funnel, xs - 0.075 - funnel_len / 3), (m_neck, xs - 0.075 - funnel_len - 0.06), (m_band, xs - 0.06), (m_yoke, xs - 0.075 - funnel_len + 0.02), (m_degauss, xs - 0.13)]
    m_tube = sum(m for m, _ in tube_parts)
    x_tube = sum(m * x for m, x in tube_parts) / m_tube

    bezel_area = (2 * HW * H - polygon_area(screen_profile(50))) + 2 * (2 * HW + H) * BEZEL_D
    m_bezel = bezel_area * 0.003 * RHO["hips-anthracite"]
    f, b = REAR_FRONT, REAR_BACK
    per_f, per_b = 2 * (2 * f["hw"] + 2 * f["hh"]), 2 * (2 * b["hw"] + 2 * b["hh"])
    rear_area = (per_f + per_b) / 2 * (XB - XR) + 4 * b["hw"] * b["hh"]
    m_rear = rear_area * 0.0025 * RHO["hips-anthracite"]
    m_tray = polygon_area(tray_plan()) * 0.003 * RHO["hips-anthracite"]
    m_chassis = 3.20
    m_speakers = 0.90
    cable_len = 2 * math.pi * COIL["R"] * 3 + 0.4
    m_cable = cable_len * 0.07
    turn_vol = math.pi * (TURN_R ** 2 - (TURN_R - 0.003) ** 2) * TURN_T + math.pi * TURN_R ** 2 * 0.003 + 8 * 0.002 * TURN_R * (TURN_T - 0.003)
    m_turn = turn_vol * RHO["abs-stand"] + 0.20
    cab_parts = [(m_bezel, XF - BEZEL_D / 2, H / 2), (m_rear, XB - (XB - XR) * 0.42, 0.24), (m_tray, (TRAY["x0"] + TRAY["x1"]) / 2, 0.006), (m_chassis, XB - 0.16, 0.05), (m_speakers, XF - 0.05, 0.055), (m_cable, XR - COIL["r"], COIL["y"]), (m_turn, 0.0, -TURN_T / 2)]
    m_cab = sum(p[0] for p in cab_parts)
    x_cab = sum(p[0] * p[1] for p in cab_parts) / m_cab
    y_cab = sum(p[0] * p[2] for p in cab_parts) / m_cab
    base_vol = math.pi * (BASE_R ** 2 - (BASE_R - 0.003) ** 2) * BASE_T + math.pi * BASE_R ** 2 * 0.003 + 8 * 0.002 * BASE_R * (BASE_T - 0.003)
    m_base = base_vol * RHO["abs-stand"] + 0.30
    blanket_area = polygon_area(blanket_section(6))
    m_blanket = blanket_area * 2 * BLANKET["hz"] * RHO["felt-blanket"]
    return {
        "blanket": (m_blanket, ((BLANKET["x0"] + BLANKET["x1"]) / 2, BLANKET_T / 2)),
        "base": (m_base, (0.0, BASE_T / 2)),
        "cabinet": (m_cab, (x_cab, y_cab)),
        "tube": (m_tube, (x_tube, yc)),
        "_parts": dict(m_face=m_face, m_skirt=m_skirt, m_funnel=m_funnel, m_neck=m_neck, m_band=m_band, m_yoke=m_yoke, m_degauss=m_degauss, m_bezel=m_bezel, m_rear=m_rear, m_tray=m_tray, m_chassis=m_chassis, m_speakers=m_speakers, m_cable=m_cable, m_turn=m_turn, m_base=m_base, bezel_area=bezel_area, rear_area=rear_area, funnel_area=funnel_area),
    }


def tv_com():
    """⚖ Centre of mass of the television (stand, cabinet, tube) in cabinet-local x and world height."""
    M = masses()
    parts = [(M["base"][0], 0.0, BLANKET_T + M["base"][1][1]), (M["cabinet"][0], M["cabinet"][1][0], YC + M["cabinet"][1][1]), (M["tube"][0], M["tube"][1][0], YC + M["tube"][1][1])]
    m = sum(p[0] for p in parts)
    return m, sum(p[0] * p[1] for p in parts) / m, sum(p[0] * p[2] for p in parts) / m


# ---------------------------------------------------------------- anchors

GRIP_X = 0.0
GRIP_Z = TRAY["hw"] - 0.02


def anchors():
    """🪝 Floor contacts of the blanket and the stand, the two carriers' hands, the warm vent of the rear cover."""
    bx = (BLANKET["x0"] + BLANKET["x1"]) / 2
    out = [
        {"id": "blanket-floor", "bone": "blanket", "x": bx, "y": 0.0, "kind": "contact", "title": T("Folded blanket on the depot floor", "Gefaltete Decke auf dem Lagerboden"), "support": "contact", "normalDeg": 90, "footprintLengthM": BLANKET["x1"] - BLANKET["x0"], "footprintWidthM": 2 * BLANKET["hz"], "frictionCoefficient": 0.5},
        {"id": "stand-foot", "bone": "base", "x": 0.0, "y": 0.0, "kind": "contact", "title": T("Swivel stand base plate on the floor", "Fußplatte des Drehfußes auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.30, "footprintWidthM": 0.30, "frictionCoefficient": 0.5},
    ]
    for aid, sz in (("grip-left", 1), ("grip-right", -1)):
        side = T("left", "links") if sz > 0 else T("right", "rechts")
        out.append({"id": aid, "bone": "cabinet", "x": GRIP_X, "y": 0.0, "z": sz * GRIP_Z, "kind": "contact", "title": T(f"Hands of the {side['en']} carrier under the bottom edge", f"Hände des Trägers {side['de']} unter der Bodenkante"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 0.04, "frictionCoefficient": 0.6})
    xv = XR + 0.12
    out.append({"id": "vent-top", "bone": "cabinet", "x": xv, "y": slope_y(xv), "kind": "emitter", "title": T("Vent slots on the sloping top of the rear cover", "Lüftungsschlitze auf der schrägen Oberseite der Rückwand")})
    return out


ANCHORS = anchors()


# ---------------------------------------------------------------- volumes per scale

def mass_marker(sid, bone, at, r, material):
    """⚖ A centre-of-mass marker."""
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": list(at)}, "member": bone, "part": "mass"}


def band_ring(scale):
    """⭕ The skirt of the tube faceplate with its steel implosion band, behind the bezel: the structural frame of the tube."""
    s = SCREEN
    t = {50: BAND_T, 100: BAND_T, 200: 0.015, 500: 0.02}[scale]
    steps = 4 if scale == 50 else 2
    outer = rounded_rect(s["y0"] - t, s["y1"] + t, -s["hw"] - t, s["hw"] + t, s["r"] + t, steps) if scale < 200 else [(s["y0"] - t, -s["hw"] - t), (s["y1"] + t, -s["hw"] - t), (s["y1"] + t, s["hw"] + t), (s["y0"] - t, s["hw"] + t)]
    hole = screen_profile(scale) if scale < 200 else [(s["y0"], -s["hw"]), (s["y1"], -s["hw"]), (s["y1"], s["hw"]), (s["y0"], s["hw"])]
    depth = max(0.025, FEATURE[scale])
    return extrude("m-tube-skirt", "tube", "yz", outer, XB - depth, XB, "crt-panel-glass", 0.3, role="detail", holes=[hole], member="tube", part="member")


def volume(scale, body):
    """🧊 Thermal and structural lists of one scale from its body list."""
    f = FEATURE[scale]
    region_of = {"blanket": "blanket", "base": "stand", "turntable": "stand", "stand": "stand", "rear": "rear-cover", "tray": "rear-cover", "coil": "rear-cover", "bezel": "bezel", "screen": "screen"}
    thermal = [{"id": f"th-{s['id']}", "of": s["id"], "region": region_of[s["id"]]} for s in body]
    xv = XR + 0.12
    ang = math.atan2((REAR_FRONT["yc"] + REAR_FRONT["hh"]) - (REAR_BACK["yc"] + REAR_BACK["hh"]), XB - XR)
    q = [0.0, 0.0, math.sin(ang / 2), math.cos(ang / 2)]
    if scale == 500:
        thermal.append(box("th-screen-face", "cabinet", (XF - 0.0135, SCREEN["y0"], -SCREEN["hw"]), (XF, SCREEN["y1"], SCREEN["hw"]), "crt-panel-glass", 0.12, role="glazing", region="screen", layer="panel-glass"))
    pl = {50: 0.12, 100: 0.125, 200: 0.13, 500: 0.15}[scale]
    thermal.append({"id": "th-vent-plume", "kind": "box", "bone": "cabinet", "material": "hips-anthracite", "role": "body", "tone": 0.28, "sheet": True, "size": [pl, 0.0015, 2 * pl], "transform": {"translation": [xv + 0.03, slope_y(xv + 0.03) + 0.0008, 0.0], "rotation": q}, "region": "rear-cover", "tags": ["vent-plume"]})

    structure = []
    ids = {s["id"] for s in body}
    structure.append({"id": "m-blanket", "of": "blanket", "member": "blanket", "part": "member"})
    if "base" in ids:
        structure.append({"id": "m-base", "of": "base", "member": "base", "part": "member"})
        structure.append({"id": "m-turntable", "of": "turntable", "member": "cabinet", "part": "member"})
    else:
        structure.append({"id": "m-stand", "of": "stand", "member": "base", "part": "member"})
    if "tray" in ids:
        structure.append({"id": "m-tray", "of": "tray", "member": "cabinet", "part": "member"})
    else:
        tb = max(f, TRAY["t"])
        structure.append(extrude("m-bottom", "cabinet", "zx", tray_plan(), 0.0, tb, "hips-anthracite", 0.3, role="detail", member="cabinet", part="member"))
    structure.append({"id": "m-bezel", "of": "bezel", "member": "cabinet", "part": "member"})
    structure.append(band_ring(scale))
    js = max(f, 0.015)
    ears = (("tl", SCREEN["y1"], 1), ("tr", SCREEN["y1"], -1), ("bl", SCREEN["y0"], 1), ("br", SCREEN["y0"], -1)) if scale < 500 else (("l", (SCREEN["y0"] + SCREEN["y1"]) / 2, 1), ("r", (SCREEN["y0"] + SCREEN["y1"]) / 2, -1))
    for sid, yy, zz in ears:
        zc = zz * (SCREEN["hw"] + BAND_T / 2)
        yc = yy if scale == 500 else yy + (BAND_T / 2 if yy > 0.3 else -BAND_T / 2)
        structure.append(box(f"j-ear-{sid}", "tube", (XB - js, yc - js / 2, zc - js / 2), (XB, yc + js / 2, zc + js / 2), "steel-band", 0.3, role="detail", member="tube", part="joint"))
    pr = max(f, 0.02)
    jh = min(max(f, 0.012), BASE_T + TURN_T)
    j0 = max(0.0, BASE_T - jh / 2)
    structure.append(revolve("j-swivel", "base", [(j0, 0.0), (j0, pr), (j0 + jh, pr), (j0 + jh, 0.0)], [0, 1, 0], [0, 0, 0], "steel-band", 0.3, role="detail", member="base", part="joint"))
    a = next(x for x in ANCHORS if x["id"] == "blanket-floor")
    rr = f if scale < 500 else 0.0375
    structure.append({"id": "support-blanket-floor", "kind": "revolve", "bone": "blanket", "material": "felt-blanket", "role": "ground", "tone": 0.2, "profile": [[0.0, 0.0], [0.0, rr], [min(f, BLANKET_T * 0.75), rr], [min(f, BLANKET_T * 0.75), 0.0]], "axis": [0, 1, 0], "origin": [a["x"], a["y"], 0.0], "member": "blanket", "part": "support", "anchor": "blanket-floor"})
    M = masses()
    mr = f if scale < 500 else 0.0375
    for bone, mat in (("base", "abs-stand"), ("cabinet", "hips-anthracite"), ("tube", "crt-panel-glass")):
        kg, (u, v) = M[bone]
        if bone == "base":
            v = (BASE_T + TURN_T) / 2 if scale >= 200 else BASE_T / 2
            structure.append(mass_marker(f"mass-{bone}", bone, (u, v, 0.0), {50: 0.006, 100: 0.0075, 200: 0.015, 500: 0.017}[scale], mat))
        else:
            structure.append(mass_marker(f"mass-{bone}", bone, (u, v, 0.0), mr, mat))
    return thermal, structure


def build_volumes():
    """🧊 The four individually authored volumes."""
    texts = {
        50: [T("Body, authored for 7.5 mm: the television on its swivel stand on the folded moving blanket. Front bezel as a 115 mm deep moulded frame with rounded corners round the 560 x 390 mm screen opening, the faceplate of the tube 12 mm behind the bezel face, the tapered rear cover as a loft from 680 x 520 mm behind the bezel to the 400 x 340 mm back plate with rounded moulded edges, the 12 mm bottom tray under it, the mains cable coiled to a ring of 164 mm and taped to the back plate; the stand as a 340 x 16 mm base plate and a 300 x 18 mm turntable with chamfered edges; the blanket as four 3 mm layers folded to 12 mm with its rounded fold at the front.", "Körper für 7,5 mm: der Fernseher auf seinem Drehfuß auf der gefalteten Möbeldecke. Frontrahmen als 115 mm tiefer Spritzgussrahmen mit gerundeten Ecken um die Bildöffnung 560 x 390 mm, der Bildschirm der Röhre 12 mm hinter der Rahmenfläche, die verjüngte Rückwand als Loft von 680 x 520 mm hinter dem Rahmen auf die Rückplatte 400 x 340 mm mit gerundeten Kanten, darunter die 12-mm-Bodenwanne, das Netzkabel zu einem Ring von 164 mm aufgewickelt und an die Rückplatte geklebt; der Fuß als Fußplatte 340 x 16 mm und Drehteller 300 x 18 mm mit gefasten Kanten; die Decke als vier 3-mm-Lagen auf 12 mm gefaltet mit gerundetem Falz vorn."),
             T("Thermal: every body solid by reference in its region (tray and cable coil in the rear-cover region) and the outlet patch of the vent plume flush on the sloping top. Structure: blanket, base plate, turntable, bottom tray and bezel as members, the faceplate skirt with its implosion band as the frame of the tube; the four mounting ears of the band at the bezel and the swivel pivot as joints; one pad on the depot floor under the blanket; masses at the centres of stand, cabinet and tube.", "Thermik: jeder Körper als Verweis in seiner Region (Bodenwanne und Kabelring in der Region der Rückwand) und die Austrittsfläche der Warmluftfahne bündig auf der schrägen Oberseite. Tragwerk: Decke, Fußplatte, Drehteller, Bodenwanne und Frontrahmen als Stäbe, der Schirmmantel mit dem Spannband als Rahmen der Röhre; die vier Befestigungslaschen des Bandes am Rahmen und der Drehzapfen als Knoten; ein Auflager auf dem Lagerboden unter der Decke; Massen in den Schwerpunkten von Fuß, Gehäuse und Röhre.")],
        100: [T("Body, authored anew for 15 mm: the bottom tray (12 mm) merges into the rear cover, the corners of bezel and screen are drawn with fewer segments, stand plates without chamfers (16 and 18 mm stay true), the cable ring kept with a coarser section, the blanket one 12 mm declared sheet with a two-step fold.", "Körper für 15 mm neu gebaut: die Bodenwanne (12 mm) geht in der Rückwand auf, die Ecken von Rahmen und Bildschirm mit weniger Segmenten, Fußplatten ohne Fase (16 und 18 mm bleiben echt), der Kabelring mit gröberem Querschnitt erhalten, die Decke als eine deklarierte 12-mm-Lage mit zweistufigem Falz."),
              T("Thermal: one surface per region by reference plus the vent patch. Structure: the same members with a 15 mm bottom member inside the rear cover, ears, pivot, the floor pad and the masses.", "Thermik: eine Oberfläche je Region als Verweis und die Lüftungsfläche. Tragwerk: dieselben Stäbe mit einem 15-mm-Bodenstab in der Rückwand, Laschen, Drehzapfen, das Bodenauflager und die Massen.")],
        200: [T("Body, authored anew for 30 mm: base plate and turntable merged into one 34 mm stand puck, bezel and screen with square corners, the rear cover more rounded (exponent 6), the cable ring (24 mm) left out and the blanket a flat 12 mm declared sheet.", "Körper für 30 mm neu gebaut: Fußplatte und Drehteller zu einem 34-mm-Fuß verschmolzen, Rahmen und Bildschirm mit rechtwinkligen Ecken, die Rückwand stärker gerundet (Exponent 6), der Kabelring (24 mm) entfällt und die Decke ist eine ebene deklarierte 12-mm-Lage."),
              T("Thermal: one surface per region by reference plus the vent patch. Structure: members by reference, a 30 mm bottom member, the skirt ring at 30 mm, ears and pivot at 30 mm, the floor pad, masses.", "Thermik: eine Oberfläche je Region als Verweis und die Lüftungsfläche. Tragwerk: Stäbe als Verweis, ein 30-mm-Bodenstab, der Mantelring bei 30 mm, Laschen und Drehzapfen bei 30 mm, das Bodenauflager, Massen.")],
        500: [T("Body, authored anew for 75 mm: the sign of the television on its stand: the bezel as one block with the screen flush in its face (the 12 mm recess and the 30 mm upper border are under the feature; the faceplate is a 13.5 mm glass layer of the thermal list on the block face), the rear cover as a strongly rounded taper (exponent 4), the 34 mm stand puck and the blanket as a 22 mm strip so that both still print at 1:1000 (C13).", "Körper für 75 mm neu gebaut: das Zeichen des Fernsehers auf seinem Fuß: der Frontrahmen als ein Block mit bündigem Bildschirm (die 12-mm-Vertiefung und der 30-mm-Oberrand liegen unter dem Merkmal; der Bildschirm ist eine 13,5-mm-Glasschicht der Thermikliste auf der Blockfläche), die Rückwand als stark gerundete Verjüngung (Exponent 4), der 34-mm-Fuß und die Decke als 22-mm-Streifen, damit beide bei 1:1000 noch drucken (C13)."),
              T("Thermal: one surface per region by reference, the faceplate as its glass layer and the vent patch. Structure: members by reference, a 75 mm bottom member, the skirt ring, two side ears and the pivot at 75 mm, the floor pad, masses.", "Thermik: eine Oberfläche je Region als Verweis, der Bildschirm als seine Glasschicht und die Lüftungsfläche. Tragwerk: Stäbe als Verweis, ein 75-mm-Bodenstab, Mantelring, zwei seitliche Laschen und Drehzapfen bei 75 mm, das Bodenauflager, Massen.")],
    }
    vols = {}
    for scale in (50, 100, 200, 500):
        body = body_solids(scale)
        thermal, structure = volume(scale, body)
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": texts[scale]}
    return vols


# ---------------------------------------------------------------- document

REFS = [
    ("crt-tv-catalogue", "Catalogue-typical data of 70 cm (28 inch, 4:3, 100 Hz) picture-tube television sets of the German mid-range 1994-2003 (types as Grundig, Loewe, Philips, Telefunken, Nokia sold them; no brand): cabinet 690-740 mm wide, 530-570 mm high, 470-520 mm deep, net weight 32-40 kg, on-mode power 85-130 W, standby 1-8 W, optional swivel stand 300-350 mm diameter, 30-40 mm high (authored-estimate from listings and manuals of the type)."),
    ("iec-62087-2011", "IEC 62087:2011 Methods of measurement for the power consumption of audio, video and related equipment (on-mode power with a broadcast video signal, standby power; the rating basis of the nameplate)."),
    ("en-60065-2014", "EN 60065:2014 Audio, video and similar electronic apparatus - Safety requirements (accessible surface temperatures, ventilation openings, mechanical strength and implosion protection of picture tubes)."),
    ("andreola-2005", "Andreola, F., Barbieri, L., Corradi, A., Lancellotti, I., Falcone, R., Hreglich, S. (2005). Glass-ceramics obtained by the recycling of end of life cathode ray tubes glasses. Waste Management 25, 183-189 (panel glass about two thirds and funnel glass about one third of the tube glass; barium-strontium panel glass about 2.7 g/cm3, lead funnel glass about 3.0 g/cm3)."),
    ("herat-2008", "Herat, S. (2008). Recycling of cathode ray tubes (CRTs) in electronic waste. CLEAN - Soil, Air, Water 36, 19-24 (the tube is about two thirds of the mass of a CRT television; composition of panel, funnel, neck and frit)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (glass 2500 kg/m3 1.0 W/mK 750 J/kgK, plastics about 0.17-0.25 W/mK and 1300-1500 J/kgK, nonwoven polyester felts about 0.04-0.05 W/mK)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total hemispherical emissivity of glass about 0.92 and of plastics and paints 0.9-0.95; natural convection from vertical and inclined plates)."),
    ("plastics-handbook", "Domininghaus, H., Elsner, P., Eyerer, P., Hirth, T. (eds.) (2012). Kunststoffe - Eigenschaften und Anwendungen, 8th ed., Springer (HIPS density 1.04-1.06 g/cm3, tensile modulus 1.6-2.2 GPa, tensile strength 20-30 MPa; ABS 1.04-1.07 g/cm3, 2.2-2.6 GPa; photo-oxidative ageing of styrenics: yellowing and chalking of the surface under daylight)."),
    ("din-18011-1992", "DIN 18011:1992-11 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (placing area of a television set in the living room)."),
    ("dguv-208-033", "DGUV Information 208-033 (2014) Muskel-Skelett-Belastungen - erkennen und beurteilen (manual handling: bulky loads above about 25 kg are carried by two persons; grip height about 0.7-0.8 m with hanging arms)."),
    ("moving-blanket-catalogue", "Catalogue-typical data of removal blankets (Umzugsdecken, Möbeldecken) of quilted nonwoven felt, about 150 x 180 cm, 3 mm, 1.0-1.5 kg (authored-estimate)."),
    ("reuse-depot-practice", "Practice of municipal second-hand department stores and reuse depots in Germany (Gebrauchtwarenkaufhaus, Haushaltsauflösung): picture-tube televisions are checked (E-Check sticker), stored on blankets on the floor because they are too heavy for shelving, the mains cable coiled and taped to the back, carried by two people (authored-estimate from the practice described in reuse programmes, no single source)."),
]


def materials():
    """🧪 The materials with their aged surface properties."""
    return [
        {"id": "crt-panel-glass", "title": T("Barium-strontium panel glass of the picture tube, tinted grey, with a dust film", "Barium-Strontium-Schirmglas der Bildröhre, grau getönt, mit Staubfilm"), "emissivity": 0.92, "solarAbsorptivity": 0.55, "densityKgM3": RHO["crt-panel-glass"], "conductivityWmK": 1.0, "specificHeatJKgK": 840, "elasticModulusPa": 7.0e10, "strengthPa": 4.0e7, "source": "literature", "reference": "andreola-2005"},
        {"id": "hips-anthracite", "title": T("Anthracite high-impact polystyrene, faded and scratched", "Anthrazitfarbenes schlagfestes Polystyrol, verblasst und verkratzt"), "emissivity": 0.92, "solarAbsorptivity": 0.82, "densityKgM3": RHO["hips-anthracite"], "conductivityWmK": 0.17, "specificHeatJKgK": 1300, "elasticModulusPa": 1.9e9, "strengthPa": 2.5e7, "source": "literature", "reference": "plastics-handbook"},
        {"id": "abs-stand", "title": T("Dark grey ABS of the swivel stand", "Dunkelgraues ABS des Drehfußes"), "emissivity": 0.92, "solarAbsorptivity": 0.8, "densityKgM3": RHO["abs-stand"], "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "elasticModulusPa": 2.4e9, "strengthPa": 4.0e7, "source": "literature", "reference": "plastics-handbook"},
        {"id": "pvc-cable", "title": T("PVC-sheathed mains cable H03VVH2-F", "PVC-ummanteltes Netzkabel H03VVH2-F"), "emissivity": 0.92, "solarAbsorptivity": 0.85, "densityKgM3": RHO["pvc-cable"], "conductivityWmK": 0.19, "specificHeatJKgK": 1000, "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "felt-blanket", "title": T("Quilted nonwoven felt of a moving blanket, dark blue", "Gesteppter Vliesfilz einer Möbeldecke, dunkelblau"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": RHO["felt-blanket"], "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "steel-band", "title": T("Steel implosion band and pivot", "Stahl-Spannband und Drehzapfen"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": RHO["steel-band"], "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "literature", "reference": "iso-10456-2007"},
    ]


def rig():
    """🦴 The rig: the blanket stays behind, the television is lifted and carried away by two pairs of hands, its cabinet turns on the swivel stand."""
    M = masses()

    def mass(bone, ref):
        kg, (u, v) = M[bone]
        return {"kg": kg, "comX": u, "comY": v, "distribution": "point", "source": "authored-estimate", "reference": ref}

    s = SCREEN
    ring_area = 2 * (2 * s["hw"] + s["y1"] - s["y0"]) * 0.011
    return {"kind": "mechanical", "root": "ground", "bones": [
        {"id": "ground", "parent": None, "title": T("Floor reference", "Bodenbezug"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "blanket", "parent": "ground", "title": T("Folded moving blanket (storage aid, stays behind)", "Gefaltete Möbeldecke (Lagerhilfe, bleibt liegen)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": BLANKET["x1"], "joint": {"type": "fixed"}, "mass": mass("blanket", "moving-blanket-catalogue")},
        {"id": "carry-lift", "parent": "ground", "title": T("Carriers' hands: lift", "Hände der Träger: Heben"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 0.80}},
        {"id": "carry-travel", "parent": "carry-lift", "title": T("Carriers' hands: walking away", "Hände der Träger: Weggehen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.50}},
        {"id": "base", "parent": "carry-travel", "title": T("Base plate of the swivel stand", "Fußplatte des Drehfußes"), "x": 0, "y": BLANKET_T, "angleDeg": 0, "lengthM": BASE_R, "joint": {"type": "fixed"}, "mass": mass("base", "crt-tv-catalogue"), "section": {"areaM2": 2 * BASE_R * 0.006, "secondMomentM4": 2 * BASE_R * BASE_T ** 3 / 12 * 0.35, "extremeFibreM": BASE_T / 2, "material": "abs-stand", "title": T("Ribbed ABS plate 340 x 16 mm", "Verrippte ABS-Platte 340 x 16 mm")}},
        {"id": "swivel", "parent": "base", "title": T("Swivel pivot (vertical axis)", "Drehzapfen (senkrechte Achse)"), "x": 0, "y": BASE_T, "angleDeg": 90, "lengthM": TURN_T, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -40, "twistMaxDeg": 40}},
        {"id": "cabinet", "parent": "swivel", "title": T("Cabinet: bezel, rear cover, chassis, turntable", "Gehäuse: Frontrahmen, Rückwand, Chassis, Drehteller"), "x": TURN_T, "y": 0, "angleDeg": -90, "lengthM": XF, "joint": {"type": "fixed"}, "mass": mass("cabinet", "crt-tv-catalogue"), "section": {"areaM2": 2 * TRAY["hw"] * 0.003, "secondMomentM4": 2 * TRAY["hw"] * 0.012 ** 3 / 12 * 0.4, "extremeFibreM": 0.006, "material": "hips-anthracite", "title": T("Ribbed bottom tray 600 x 12 mm", "Verrippte Bodenwanne 600 x 12 mm")}},
        {"id": "tube", "parent": "cabinet", "title": T("Picture tube with implosion band and yoke", "Bildröhre mit Spannband und Ablenkeinheit"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": XF - SCREEN["recess"], "joint": {"type": "fixed"}, "mass": mass("tube", "herat-2008"), "section": {"areaM2": ring_area, "secondMomentM4": 0.011 * 0.075 ** 3 / 12 * 2, "extremeFibreM": 0.0375, "material": "crt-panel-glass", "title": T("Faceplate skirt 11 mm glass, 75 mm deep", "Schirmmantel 11 mm Glas, 75 mm tief")}},
    ]}


ENV = {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.45, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325}

AREAS = {"screen": 0.625, "bezel": 0.797, "rear-cover": 1.314, "stand": 0.207, "blanket": 1.348}


def layer(lid, title, kind, material, t, lam, rho, cp, src, ref):
    """🧶 One layer with resistance and the areal heat capacity of one hour's penetration depth."""
    depth = math.sqrt(lam / (rho * cp) * 3600)
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": t / lam, "heatCapacityJM2K": rho * cp * min(t, depth), "source": src, "reference": ref}


def region(rid, title, material, L, geom, lay, extra=None, model="passive"):
    """🔥 A surface region with one layer."""
    r = {"id": rid, "title": title, "kind": "surface", "model": model, "material": material, "areaM2": AREAS[rid], "characteristicLengthM": L, "geometry": geom, "skyViewFactor": 0.5, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "crt-tv-catalogue"}
    if extra:
        r.update(extra)
    return r


HEAT = {"screen": 15.0, "bezel": 8.0, "rear-cover": 77.0}


def thermal_model():
    """🌡 Five regions: three heated by the running set, stand and blanket passive."""
    regions = [
        region("screen", T("Faceplate of the picture tube, dusty", "Bildschirm der Bildröhre, verstaubt"), "crt-panel-glass", 0.39, "vertical-plate", layer("panel-glass", T("Panel glass 13.5 mm", "Schirmglas 13,5 mm"), "glass", "crt-panel-glass", 0.0135, 1.0, RHO["crt-panel-glass"], 840, "literature", "andreola-2005"), {"operating": {"key": "power", "heatW": HEAT["screen"]}}, model="operating"),
        region("bezel", T("Front bezel, scratched, with speaker grilles and a missing control flap", "Frontrahmen, verkratzt, mit Lautsprechergittern und fehlender Bedienklappe"), "hips-anthracite", 0.53, "vertical-plate", layer("bezel-shell", T("HIPS shell 3 mm", "HIPS-Schale 3 mm"), "polymer", "hips-anthracite", 0.003, 0.17, RHO["hips-anthracite"], 1300, "literature", "plastics-handbook"), {"operating": {"key": "power", "heatW": HEAT["bezel"]}}, model="operating"),
        region("rear-cover", T("Rear cover with dusty vent slots, bottom tray and taped cable coil", "Rückwand mit verstaubten Lüftungsschlitzen, Bodenwanne und angeklebtem Kabelring"), "hips-anthracite", 0.40, "vertical-plate", layer("rear-shell", T("HIPS shell 2.5 mm", "HIPS-Schale 2,5 mm"), "polymer", "hips-anthracite", 0.0025, 0.17, RHO["hips-anthracite"], 1300, "literature", "plastics-handbook"), {"operating": {"key": "power", "heatW": HEAT["rear-cover"]}, "ratedPowerW": 110, "ratedPowerBasis": "electrical-input"}, model="operating"),
        region("stand", T("Swivel stand, base plate and turntable", "Drehfuß, Fußplatte und Drehteller"), "abs-stand", 0.30, "horizontal-plate-up", layer("stand-shell", T("Ribbed ABS 3 mm", "Verripptes ABS 3 mm"), "polymer", "abs-stand", 0.003, 0.17, RHO["abs-stand"], 1400, "literature", "plastics-handbook"), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("blanket", T("Folded moving blanket", "Gefaltete Möbeldecke"), "felt-blanket", 0.73, "horizontal-plate-up", layer("felt", T("Four layers of felt, 12 mm", "Vier Lagen Filz, 12 mm"), "polymer", "felt-blanket", BLANKET_T, 0.045, RHO["felt-blanket"], 1300, "authored-estimate", "iso-10456-2007")),
    ]
    emitters = [{"id": "vent-plume", "kind": "plume", "title": T("Warm air rising from the vent slots of the rear cover", "Warme Luft aus den Lüftungsschlitzen der Rückwand"), "anchor": "vent-top", "directionDeg": 90, "diameterM": 0.24, "region": "rear-cover", "operatingKey": "power", "source": "authored-estimate", "reference": "incropera-2018"}]
    operating = [{"id": "power", "title": T("Electrical power", "Elektrische Leistung"), "levels": [{"v": 0, "title": T("Off, unplugged", "Aus, Stecker gezogen")}, {"v": 0.05, "title": T("Standby (5 W)", "Bereitschaft (5 W)")}, {"v": 1, "title": T("On, picture (100 W)", "An, Bild (100 W)")}]}]
    return {"environment": ENV, "operating": operating, "regions": regions, "emitters": emitters, "passengers": [], "assumptions": [
        T("Every watt ends as heat in the room: at level 1 (100 W on-mode, IEC 62087 type value of a 70 cm 100 Hz set) the rear cover with deflection, line output stage and power supply behind it releases 77 W, the bezel 8 W and the faceplate 15 W; the nameplate states 110 W electrical input on the rear cover, the element that carries the rating plate. Standby is 5 W, typical of the 1990s.", "Jedes Watt endet als Wärme im Raum: bei Stufe 1 (100 W im Betrieb, IEC-62087-Typwert eines 70-cm-100-Hz-Geräts) gibt die Rückwand mit Ablenkung, Zeilenendstufe und Netzteil dahinter 77 W ab, der Frontrahmen 8 W und der Bildschirm 15 W; das Typenschild nennt 110 W elektrische Aufnahme an der Rückwand, die das Schild trägt. Bereitschaft 5 W, typisch für die 1990er-Jahre."),
        T("Aged surfaces: the anthracite HIPS has faded and chalked on the window side, its solar absorptance is taken as 0.82 instead of about 0.88 when new; dust on the faceplate and in the vent slots keeps the emissivity of glass and plastics (0.92) but narrows the free vent area, which the model leaves to the plume diameter (Incropera, plastics handbook, authored-estimate).", "Gealterte Oberflächen: das anthrazitfarbene HIPS ist auf der Fensterseite verblasst und kreidet, sein solarer Absorptionsgrad wird mit 0,82 statt etwa 0,88 im Neuzustand angesetzt; Staub auf dem Bildschirm und in den Lüftungsschlitzen ändert den Emissionsgrad von Glas und Kunststoff (0,92) nicht, verengt aber die freie Lüftungsfläche, was das Modell dem Fahnendurchmesser überlässt (Incropera, Kunststoffhandbuch, Schätzwert)."),
        T("Reference case: indoors at 20 degC with still air and no sun, in the depot unplugged, in the living room running. Contacts: the stand on the blanket and the blanket on the floor at room temperature; the carriers' hands (32 degC) touch 1 % of the rear cover.", "Referenzfall: innen bei 20 degC, ruhige Luft, keine Sonne, im Lager ohne Strom, im Wohnzimmer in Betrieb. Kontakte: der Fuß auf der Decke und die Decke auf dem Boden bei Raumtemperatur; die Hände der Träger (32 degC) berühren 1 % der Rückwand."),
        T("The cable coil and the bottom tray belong to the rear-cover region: they share its temperature; the stand states the passive rating 0 W.", "Kabelring und Bodenwanne gehören zur Region der Rückwand und teilen ihre Temperatur; der Fuß nennt die passive Nennleistung 0 W."),
    ]}


def binding(kind):
    """🌬 The region binding of one pose family."""
    on = kind in ("in-use", "standby")
    reg = {"screen": {"exposedFraction": 0.38, "sunlitFraction": 0}, "bezel": {"exposedFraction": 0.61, "sunlitFraction": 0}, "rear-cover": {"exposedFraction": 0.66, "sunlitFraction": 0}}
    if kind == "stored":
        reg["stand"] = {"exposedFraction": 0.26, "sunlitFraction": 0, "contact": {"fraction": 0.44, "tempC": 20}}
        reg["blanket"] = {"exposedFraction": 0.45, "sunlitFraction": 0, "contact": {"fraction": 0.55, "tempC": 20}}
    elif kind == "carried":
        reg["rear-cover"] = {"exposedFraction": 0.65, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 32}}
        reg["stand"] = {"exposedFraction": 0.7, "sunlitFraction": 0}
        reg["blanket"] = {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 20}}
    else:
        reg["stand"] = {"exposedFraction": 0.26, "sunlitFraction": 0, "contact": {"fraction": 0.44, "tempC": 20}}
        reg["blanket"] = {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 20}}
    level = 1.0 if kind == "in-use" else (0.05 if kind == "standby" else 0.0)
    return {"regions": reg, "emitters": {"vent-plume": level}, "operating": {"power": level}}


PLANT = {"id": "plant-on-doily", "kind": "live", "title": T("Pot plant on a crocheted doily on the top (2.5 kg)", "Topfpflanze auf einem Häkeldeckchen auf der Oberseite (2,5 kg)"), "distributed": {"bone": "cabinet", "fromM": XB - 0.12, "toM": XB + 0.06, "wxNPerM": 0, "wyNPerM": -2.5 * G / 0.18}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "reuse-depot-practice"}

CARRIED = {"carry-lift": {"dy": CARRY_DY}, "carry-travel": {"dx": CARRY_DX}}
IN_USE = {"carry-lift": {"dy": USE_DY}, "carry-travel": {"dx": CARRY_DX}}


def pose(pid, title, desc, bones, contacts, kind, loads=(), key=False):
    """🧍 One pose."""
    p = {"id": pid, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "auto", "contacts": contacts, "thermal": binding(kind), "loads": list(loads)})
    return p


def poses():
    """🧍 Stored, carried, in use, standby and the key poses of carry and set-down."""
    B, Gp, S = ["blanket-floor"], ["grip-left", "grip-right"], ["stand-foot"]
    return [
        pose("stored", T("Stored on a blanket", "Auf einer Decke gelagert"), T("In the reuse depot the television stands on its swivel stand on a folded moving blanket on the floor, unplugged, the mains cable coiled and taped to the back, an E-Check sticker on the side and a price label on the top.", "Im Gebrauchtwarenlager steht der Fernseher auf seinem Drehfuß auf einer gefalteten Möbeldecke am Boden, ohne Strom, das Netzkabel aufgewickelt und hinten angeklebt, ein E-Check-Aufkleber an der Seite und ein Preisschild oben."), {}, B, "stored"),
        pose("carried", T("Carried by two people", "Von zwei Personen getragen"), T("Two people, one at each side, hold it under the bottom edges near the centre of mass and carry it level with hanging arms, bottom at 0.72 m, 1.2 m away from the blanket, which stays behind.", "Zwei Personen, je eine an jeder Seite, halten ihn unter den Bodenkanten nahe dem Schwerpunkt und tragen ihn waagerecht mit hängenden Armen, Unterkante auf 0,72 m, 1,2 m von der Decke weg, die liegen bleibt."), CARRIED, B + Gp, "carried"),
        pose("in-use", T("Running in a living room", "In Betrieb im Wohnzimmer"), T("Set down on its swivel stand on the floor of a living room, plugged in and showing a picture: 100 W, the rear cover runs warm and a plume of warm air rises from its dusty vent slots; a pot plant on a doily stands on the top.", "Auf seinem Drehfuß auf den Boden eines Wohnzimmers gestellt, eingesteckt und mit Bild: 100 W, die Rückwand wird warm und aus ihren verstaubten Lüftungsschlitzen steigt eine Warmluftfahne; oben steht eine Topfpflanze auf einem Deckchen."), IN_USE, B + S, "in-use", [PLANT]),
        pose("standby", T("In standby", "Im Bereitschaftsbetrieb"), T("Switched off with the remote control, the red standby lamp lit: 5 W, a few tenths of a kelvin above the room.", "Mit der Fernbedienung ausgeschaltet, die rote Bereitschaftslampe leuchtet: 5 W, wenige Zehntel Kelvin über dem Raum."), IN_USE, B + S, "standby", [PLANT]),
        pose("lifted-key", T("Lifted off the blanket (key)", "Von der Decke abgehoben (Schlüssel)"), T("Lifted 0.10 m off the blanket by the two carriers.", "Von den beiden Trägern 0,10 m von der Decke abgehoben."), {"carry-lift": {"dy": 0.10}}, B + Gp, "carried", key=True),
        pose("raised-key", T("Raised to carrying height (key)", "Auf Traghöhe angehoben (Schlüssel)"), T("Raised to carrying height above the blanket before walking off.", "Über der Decke auf Traghöhe angehoben, bevor die Träger losgehen."), {"carry-lift": {"dy": CARRY_DY}}, B + Gp, "carried", key=True),
        pose("set-down-key", T("Set down, not yet plugged in (key)", "Abgestellt, noch nicht eingesteckt (Schlüssel)"), T("Standing on the floor of the living room, still unplugged.", "Steht auf dem Boden des Wohnzimmers, noch ohne Strom."), IN_USE, B + S, "stored-at-home", key=True),
    ]


def spans(anchors_, a, b):
    """🦶 Contact spans."""
    return [{"anchor": x, "fromS": a, "toS": b} for x in anchors_]


def clips():
    """🎞 Carry out of the depot, set down, and switching on and warming up."""
    B, Gp, S = ["blanket-floor"], ["grip-left", "grip-right"], ["stand-foot"]
    return [
        {"id": "carry", "title": T("Carry out of the depot", "Aus dem Lager tragen"), "description": T("Two people lift the television off the blanket, raise it to carrying height with hanging arms and walk it 1.2 m away.", "Zwei Personen heben den Fernseher von der Decke, bringen ihn mit hängenden Armen auf Traghöhe und tragen ihn 1,2 m weg."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "lifted-key", "ease": "smooth"}, {"t": 2.4, "pose": "raised-key", "ease": "smooth"}, {"t": 5.4, "pose": "carried", "ease": "smooth"}, {"t": 6, "pose": "carried", "ease": "smooth"}],
         "contacts": spans(B, 0, 6) + spans(Gp, 0, 6),
         "phases": [{"id": "lift", "fromS": 0, "toS": 2.4, "title": T("Lift", "Heben")}, {"id": "walk", "fromS": 2.4, "toS": 5.4, "title": T("Walk", "Gehen")}, {"id": "hold", "fromS": 5.4, "toS": 6, "title": T("Hold", "Halten")}],
         "events": [{"t": 0.2, "kind": "motion", "title": T("Stand leaves the blanket", "Fuß löst sich von der Decke")}, {"t": 2.4, "kind": "motion", "title": T("Carriers walk off", "Träger gehen los")}],
         "tracks": [{"target": "region:stand:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.26}, {"t": 1.0, "v": 0.7}]}, {"target": "region:blanket:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.45}, {"t": 1.0, "v": 0.5}]}]},
        {"id": "set-down", "title": T("Set down in the living room", "Im Wohnzimmer abstellen"), "description": T("The carriers lower the television from carrying height until the stand rests on the floor and let go.", "Die Träger senken den Fernseher von der Traghöhe ab, bis der Fuß auf dem Boden steht, und lassen los."), "durationS": 3, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "carried", "ease": "smooth"}, {"t": 2.6, "pose": "set-down-key", "ease": "smooth"}, {"t": 3, "pose": "set-down-key", "ease": "smooth"}],
         "contacts": spans(B, 0, 3) + spans(Gp, 0, 2.8) + spans(S, 2.5, 3),
         "phases": [{"id": "lower", "fromS": 0, "toS": 2.6, "title": T("Lower", "Absenken")}, {"id": "release", "fromS": 2.6, "toS": 3, "title": T("Let go", "Loslassen")}],
         "events": [{"t": 2.6, "kind": "contact-on", "title": T("Stand on the floor", "Fuß auf dem Boden"), "target": "stand-foot"}, {"t": 2.8, "kind": "contact-off", "title": T("Hands let go", "Hände lassen los"), "target": "grip-left"}],
         "tracks": []},
        {"id": "switch-on-warm-up", "title": T("Switch on and warm up", "Einschalten und aufwärmen"), "description": T("Woken from standby by the remote control after five seconds, the picture tube lights up and the cabinet warms up over fifteen minutes; the warm plume over the rear vents builds up.", "Nach fünf Sekunden mit der Fernbedienung aus der Bereitschaft geweckt, leuchtet die Bildröhre auf und das Gehäuse erwärmt sich über fünfzehn Minuten; über den hinteren Lüftungsschlitzen baut sich die Warmluftfahne auf."), "durationS": 900, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "standby", "ease": "step"}, {"t": 5, "pose": "in-use", "ease": "step"}, {"t": 900, "pose": "in-use", "ease": "step"}],
         "contacts": spans(B, 0, 900) + spans(S, 0, 900),
         "phases": [{"id": "standby", "fromS": 0, "toS": 5, "title": T("Standby", "Bereitschaft")}, {"id": "warm-up", "fromS": 5, "toS": 900, "title": T("On, warming up", "An, wärmt auf")}],
         "events": [{"t": 5, "kind": "operating", "title": T("Switched on", "Eingeschaltet"), "target": "power"}],
         "tracks": [{"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 0.05}, {"t": 5, "v": 1}]}, {"target": "emitter:vent-plume", "ease": "step", "keys": [{"t": 0, "v": 0.05}, {"t": 5, "v": 1}]}]},
    ]


def structure_model():
    """🏗 Supports and assumptions."""
    m, xc, yc = tv_com()
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": ["blanket-floor"], "clinical": False, "assumptions": [
        T(f"Stored: one rigid stack. The television ({m:.1f} kg, centre of mass {abs(xc) * 1000:.0f} mm {'ahead of' if xc > 0 else 'behind'} the swivel axis at {yc:.2f} m) stands with its ribbed base plate on the folded blanket, the blanket on the depot floor (friction 0.5).", f"Gelagert: ein starrer Stapel. Der Fernseher ({m:.1f} kg, Schwerpunkt {abs(xc) * 1000:.0f} mm {'vor' if xc > 0 else 'hinter'} der Drehachse auf {yc:.2f} m) steht mit seiner verrippten Fußplatte auf der gefalteten Decke, die Decke auf dem Lagerboden (Reibung 0,5)."),
        T(f"Carried: two people, one at each side, hold the cabinet under the bottom edges at the swivel axis with hanging arms (grip height 0.72 m, about {m / 2:.0f} kg per person, above the 25 kg that one person may carry and therefore a two-person load per DGUV 208-033); the hands are compression contacts pushing up.", f"Getragen: zwei Personen, je eine an jeder Seite, halten das Gehäuse unter den Bodenkanten an der Drehachse mit hängenden Armen (Griffhöhe 0,72 m, etwa {m / 2:.0f} kg je Person, über den 25 kg, die eine Person tragen darf, deshalb eine Last für zwei nach DGUV 208-033); die Hände sind nach oben drückende Kontakte."),
        T("In use: the stand rests on the floor; a pot plant on a doily on the top, 2.5 kg (authored-estimate). Load path: tube faceplate skirt with its implosion band screwed at four ears to the bezel, the bezel and the ribbed bottom tray into the turntable, the steel pivot and the base plate; the rear cover carries only itself.", "In Gebrauch: der Fuß steht auf dem Boden; eine Topfpflanze auf einem Deckchen oben, 2,5 kg (Schätzwert). Lastweg: Schirmmantel der Röhre mit Spannband an vier Laschen mit dem Frontrahmen verschraubt, Frontrahmen und verrippte Bodenwanne in den Drehteller, den Stahlzapfen und die Fußplatte; die Rückwand trägt nur sich selbst."),
        T("Limitation of the solver: the whole asset is one instantaneously rigid body on springs of equal stiffness, so while carried or in use the blanket left behind and the television share their reactions linearly over all active supports instead of each carrying its own weight; the stored pose, a resting stack, is exact.", "Grenze des Lösers: das ganze Objekt ist ein momentan starrer Körper auf gleich steifen Federn, getragen oder in Gebrauch teilen sich die zurückgelassene Decke und der Fernseher ihre Reaktionen deshalb linear über alle aktiven Auflager, statt dass jedes sein eigenes Gewicht trägt; die Lagerpose, ein ruhender Stapel, ist exakt."),
        T("Masses from sections x density: panel glass 2700 kg/m3 (faceplate 13.5 mm, skirt 11 mm), funnel lead glass 3000 kg/m3 at 6 mm, steel band, yoke 0.9 kg and degaussing coil 0.35 kg, HIPS 1050 kg/m3 shells of 3 and 2.5 mm, chassis 3.2 kg and speakers 0.9 kg as catalogue-typical masses; the tube is about two thirds of the set (Herat 2008, Andreola 2005).", "Massen aus Querschnitt x Dichte: Schirmglas 2700 kg/m3 (Schirm 13,5 mm, Mantel 11 mm), Bleiglas des Konus 3000 kg/m3 bei 6 mm, Stahlband, Ablenkeinheit 0,9 kg und Entmagnetisierungsspule 0,35 kg, HIPS-Schalen 1050 kg/m3 mit 3 und 2,5 mm, Chassis 3,2 kg und Lautsprecher 0,9 kg als katalogtypische Massen; die Röhre ist etwa zwei Drittel des Geräts (Herat 2008, Andreola 2005)."),
    ]}


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": 0.13, "ground": 0.25}}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = [{"id": "quilt", "title": T("Quilting of the moving blanket", "Steppung der Möbeldecke"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.2, "penMm": 0.13}, {"id": "perforation", "title": T("Perforated speaker grille", "Gelochtes Lautsprechergitter"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.13}]


def decisions(scale):
    """📝 The drawing decisions of one scale."""
    D = {
        50: dict(
            silhouette=T("Television 14 x 11 mm on paper on its stand and blanket. Side elevation: the 115 mm bezel, the rear cover tapering from 520 to 340 mm with its flat bottom, the bottom tray, the cable ring taped to the back, the two plates of the swivel stand and the folded blanket. Front elevation: the bezel with rounded corners, the screen 560 x 390 mm, two perforated speaker grilles, the open control recess, the power switch, the stand and the blanket. Plan: bezel, the rear cover tapering to 400 mm, the cable ring, vent slots and the blanket.", "Fernseher 14 x 11 mm auf dem Papier auf Fuß und Decke. Seitenansicht: der 115-mm-Frontrahmen, die von 520 auf 340 mm verjüngte Rückwand mit flachem Boden, die Bodenwanne, der hinten angeklebte Kabelring, die zwei Platten des Drehfußes und die gefaltete Decke. Vorderansicht: der Rahmen mit gerundeten Ecken, der Bildschirm 560 x 390 mm, zwei gelochte Lautsprechergitter, die offene Bedienmulde, der Netzschalter, Fuß und Decke. Grundriss: Rahmen, die auf 400 mm verjüngte Rückwand, Kabelring, Lüftungsschlitze und Decke."),
            retained=T("Traces of use: the missing control flap (open recess with the bare buttons), scratches on the bezel and the top, the ghost of a channel logo burnt into the phosphor, dust lines in and beside the vent slots, the E-Check sticker on the side, the price label on the top, the taped cable; the chamfers of the stand, the folded layers and quilting of the blanket.", "Gebrauchsspuren: die fehlende Bedienklappe (offene Mulde mit den blanken Tasten), Kratzer an Rahmen und Oberseite, das eingebrannte Senderlogo im Leuchtstoff, Staublinien in und neben den Lüftungsschlitzen, der E-Check-Aufkleber an der Seite, das Preisschild oben, das angeklebte Kabel; die Fasen des Fußes, gefaltete Lagen und Steppung der Decke."),
            merged=T("The far side of every part coincides with the near one in the elevations; the rear cover and the cable ring are hidden behind the bezel in the front elevation, the stand under the cabinet in plan; in the side elevation the bezel stands also for the faceplate recessed inside it (regions bezel and screen).", "Die ferne Seite jedes Teils deckt sich in den Ansichten mit der nahen; Rückwand und Kabelring verdeckt der Rahmen in der Vorderansicht, den Fuß das Gehäuse im Grundriss; in der Seitenansicht steht der Rahmen auch für den darin vertieften Bildschirm (Regionen Rahmen und Bildschirm)."),
            omitted=T("Screws, the 5 mm vent slots as openings (drawn as strokes), the convexity of the faceplate (under 7.5 mm), the sockets of the back plate.", "Schrauben, die 5-mm-Lüftungsschlitze als Öffnungen (als Striche gezeichnet), die Wölbung des Schirms (unter 7,5 mm), die Buchsen der Rückplatte."),
            thinElements=T("The 12 mm blanket and tray are filled strips of 0.24 mm; the screen recess of 12 mm shows in the plan of the 3D model only.", "Decke und Bodenwanne (12 mm) sind gefüllte Streifen von 0,24 mm; die 12-mm-Vertiefung des Bildschirms zeigt nur das 3D-Modell."),
            hatching=T("Dots on the speaker grilles, cross quilting on the blanket.", "Punkte auf den Lautsprechergittern, Kreuzsteppung auf der Decke."),
            styles=T("Bezel 0.20, screen 0.12 (glazing), rear cover 0.28, stand 0.40-0.45, blanket 0.30, cable 0.15.", "Rahmen 0,20, Bildschirm 0,12 (Verglasung), Rückwand 0,28, Fuß 0,40-0,45, Decke 0,30, Kabel 0,15."),
            overlays=T("Every part carries its thermal region: screen, bezel, rear cover, stand, blanket.", "Jedes Teil trägt seine Thermikregion: Bildschirm, Rahmen, Rückwand, Fuß, Decke.")),
        100: dict(
            silhouette=T("Television 7 x 5.5 mm: bezel, screen, tapered rear cover, cable ring, both stand plates, blanket with its fold.", "Fernseher 7 x 5,5 mm: Rahmen, Bildschirm, verjüngte Rückwand, Kabelring, beide Fußplatten, Decke mit Falz."),
            retained=T("Speaker grilles and the open control recess as outlines, the cable ring, every second vent slot.", "Lautsprechergitter und offene Bedienmulde als Umrisse, der Kabelring, jeder zweite Lüftungsschlitz."),
            merged=T("Bottom tray into the rear cover.", "Bodenwanne in die Rückwand."),
            omitted=T("Hatches, scratches, burn-in, stickers, dust, chamfers, the folded layers.", "Schraffuren, Kratzer, Einbrennung, Aufkleber, Staub, Fasen, gefaltete Lagen."),
            thinElements=T("The 12 mm blanket is a 0.12 mm filled strip carried by its outline pen; a declared sheet in 3D.", "Die 12-mm-Decke ist ein gefüllter Streifen von 0,12 mm, den sein Umrissstrich trägt; in 3D ein deklariertes Blatt."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All five regions.", "Alle fünf Regionen.")),
        200: dict(
            silhouette=T("Television 3.5 x 2.7 mm: bezel, screen, rear cover, one stand, blanket strip.", "Fernseher 3,5 x 2,7 mm: Rahmen, Bildschirm, Rückwand, ein Fuß, Deckenstreifen."),
            retained=T("The taper of the rear cover, the screen inside the bezel, the stand.", "Die Verjüngung der Rückwand, der Bildschirm im Rahmen, der Fuß."),
            merged=T("Base plate and turntable into one 34 mm stand.", "Fußplatte und Drehteller zu einem 34-mm-Fuß."),
            omitted=T("Cable ring, grilles, recess, vents, rounded corners.", "Kabelring, Gitter, Mulde, Lüftung, gerundete Ecken."),
            thinElements=T("The 12 mm blanket stays at its true thickness, carried by the body pen.", "Die 12-mm-Decke bleibt in echter Dicke und wird vom Körperstrich getragen."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All five regions.", "Alle fünf Regionen.")),
        500: dict(
            silhouette=T("Television sign 1.4 x 1.1 mm: bezel with screen, the tapered rear cover, the stand and the blanket strip.", "Fernsehzeichen 1,4 x 1,1 mm: Rahmen mit Bildschirm, die verjüngte Rückwand, der Fuß und der Deckenstreifen."),
            retained=T("Height, width and depth, the taper of the back, the screen.", "Höhe, Breite und Tiefe, die Verjüngung hinten, der Bildschirm."),
            merged=T("Stand plates into one puck, screen flush with the bezel.", "Fußplatten zu einem Fuß, Bildschirm bündig mit dem Rahmen."),
            omitted=T("Every detail under 75 mm.", "Jedes Detail unter 75 mm."),
            thinElements=T("The blanket is drawn and modelled as a 22 mm strip and the stand as 34 mm so that with the 0.18 mm body pen they still print 0.2 mm wide at 1:1000.", "Die Decke ist als 22-mm-Streifen und der Fuß mit 34 mm gezeichnet und modelliert, damit sie mit dem 0,18-mm-Körperstrich bei 1:1000 noch 0,2 mm breit drucken."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All five regions.", "Alle fünf Regionen.")),
    }
    return D[scale]


def representations():
    """🖨 The four authored drawings."""
    out = {}
    for scale in (50, 100, 200, 500):
        lo, hi = BANDS[scale]
        out[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES if scale == 50 else [], "geometry": geometry(scale), "decisions": decisions(scale)}
    return out


def document():
    """📄 The whole asset document."""
    M = masses()
    total = sum(v[0] for k, v in M.items() if k != "_parts")
    asset = {
        "id": "used-crt-television",
        "category": "appliances",
        "title": T("Used CRT Television", "Gebrauchter Röhrenfernseher"),
        "summary": T("A second-hand 1990s 70 cm picture-tube television 0.70 x 0.53 x 0.49 m on a swivel stand, 34 kg, with a missing control flap, scratches, a burnt-in channel logo and dusty vents, stored on a moving blanket with its cable taped to the back, carried by two people and running warm again in a living room at 100 W.", "Ein gebrauchter 70-cm-Röhrenfernseher der 1990er-Jahre, 0,70 x 0,53 x 0,49 m, auf einem Drehfuß, 34 kg, mit fehlender Bedienklappe, Kratzern, eingebranntem Senderlogo und verstaubten Lüftungsschlitzen, auf einer Möbeldecke gelagert, das Kabel hinten angeklebt, von zwei Personen getragen und im Wohnzimmer mit 100 W wieder warm in Betrieb."),
        "tags": ["used", "household", "second-hand", "living", "television", "crt", "picture-tube", "electrical", "heat-source", "internal-gain", "plastic", "glass", "reuse-depot"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS], "notes": T(
            "Picture-tube television of the German mid-range type 1994-2003: 70 cm tube (66 cm visible, 4:3), cabinet 700 x 530 x 490 mm of anthracite HIPS: a 115 mm deep bezel and a rear cover tapering to a 400 x 340 mm back plate; on a 340 mm swivel stand 34 mm high. Traces of use: the control flap broken off, scratches on bezel and top, a burnt-in channel logo, dust in the vent slots, an E-Check sticker and a price label, the mains cable coiled and taped to the back. Storage aid: a moving blanket folded to 12 mm. Masses section x density: tube 24.4 kg, cabinet with turntable 8.9 kg, base plate 0.7 kg (television 34.0 kg), blanket 1.1 kg.",
            "Röhrenfernseher der deutschen Mittelklasse 1994-2003: 70-cm-Röhre (66 cm sichtbar, 4:3), Gehäuse 700 x 530 x 490 mm aus anthrazitfarbenem HIPS: ein 115 mm tiefer Frontrahmen und eine auf die Rückplatte 400 x 340 mm verjüngte Rückwand; auf einem Drehfuß 340 mm, 34 mm hoch. Gebrauchsspuren: die Bedienklappe abgebrochen, Kratzer an Rahmen und Oberseite, ein eingebranntes Senderlogo, Staub in den Lüftungsschlitzen, ein E-Check-Aufkleber und ein Preisschild, das Netzkabel aufgewickelt und hinten angeklebt. Lagerhilfe: eine auf 12 mm gefaltete Möbeldecke. Massen aus Querschnitt x Dichte: Röhre 24,4 kg, Gehäuse mit Drehteller 8,9 kg, Fußplatte 0,7 kg (Fernseher 34,0 kg), Decke 1,1 kg.")},
        "dimensions": {"heightM": YC + H, "widthM": BLANKET["x1"] - BLANKET["x0"], "depthM": 2 * BLANKET["hz"], "massKg": total},
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
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": r6(asset)}


if __name__ == "__main__":
    OUT.write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    M = masses()
    print("[DEBUG] masses", {k: round(v[0], 3) for k, v in M.items() if k != "_parts"}, "total", round(sum(v[0] for k, v in M.items() if k != "_parts"), 3))
    print("[DEBUG] parts", {k: round(v, 3) for k, v in M["_parts"].items()})
    print("[DEBUG] tv com", [round(x, 4) for x in tv_com()])
