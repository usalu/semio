"""🔥 Authoritative chisel of the used oil-filled radiator: states every hand-chosen number and writes `🔣️.json` next to this file.

A second-hand 2 kW oil-filled electric radiator with nine pressed-steel fins on two castor feet, as a reuse depot offers it: standing on
its castors on a folded furniture pad (rest = default pose `stored`), lifted and carried away by one person at the top of the control
housing and of the far end fin, and back in a living room heating at full power. World frame of the staffage: x ahead (the 150 mm depth
of the fins), y up, z left (the 0.445 m length of the fin stack with the control housing on the right end).

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
BL = 0.012

PAD = dict(x=0.225, z=0.30, y0=0.0, y1=BL)
CASTOR = dict(x=0.100, r=0.020, w=0.018, fork_x=0.016, fork_y0=0.030, fork_y1=0.052, fork_w=0.026)
FOOT = dict(x=0.125, y0=0.052, y1=0.072, w=0.030)
FIN = dict(x=0.075, y0=0.072, y1=0.640, t=0.022, pitch=0.042, n=9, r_top=0.040, r_bot=0.020, rim=0.006)
HUB = dict(r=0.018, y_low=0.112, y_high=0.600)
ZMIN = -0.2225
Z_FACE = ZMIN + 0.022
HOUSING = dict(x=0.060, y0=0.095, y1=0.620, t=0.065, r_top=0.020, r_bot=0.008)
Z_FIN0 = Z_FACE + HOUSING["t"]
Z_FIN1 = Z_FIN0 + (FIN["n"] - 1) * FIN["pitch"] + FIN["t"]
KNOB = dict(y=0.540, r=0.022, proud=0.020)
SWITCH = dict(x=0.025, y0=0.455, y1=0.485, proud=0.008)
CORD = dict(c_low=0.187, c_high=0.313, r_out=0.032, r_in=0.016, z_out=Z_FACE - 0.022, z_in=Z_FACE - 0.008)
HOOKS = dict(x=0.012, low=(0.171, 0.195), high=(0.305, 0.329))
FEET_K = (1, 7)
Z_STACK = (Z_FIN0 + Z_FIN1) / 2
Z_HOUSING = Z_FACE + HOUSING["t"] / 2
Z_KNOB = Z_FACE - KNOB["proud"] / 2
CARRY_DY = 0.15
CARRY_DX = 0.70
USE_DY = -BL
OIL_KG = 1.6
ELEMENT_KG = 0.35
CORD_M = 1.8
CORD_KG_M = 0.065
T_SHELL = 0.0007
OPERATING = {"fins": 1940.0, "housing": 60.0}

RHO = {"enamelled-steel": 7850.0, "housing-plastic": 1050.0, "knob-plastic": 1050.0, "pvc-cord": 1380.0, "painted-steel": 7850.0, "castor-polyamide": 1140.0, "castor-rubber": 1200.0, "felt-pad": 134.0}


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


def yw(Y):
    """↕ World rest height of a radiator height measured from the castor treads."""
    return BL + Y


def fin_z(k, t=None, n=None):
    """🪟 Lateral span (z0, z1) of fin k of a row of n fins of thickness t filling the true stack length."""
    t = FIN["t"] if t is None else t
    n = FIN["n"] if n is None else n
    pitch = (Z_FIN1 - Z_FIN0 - t) / (n - 1)
    z0 = Z_FIN0 + k * pitch
    return z0, z0 + t


FOOT_Z = [sum(fin_z(k)) / 2 for k in FEET_K]

BONES_WORLD = {
    "ground": ((0.0, 0.0, 0.0), 0.0),
    "blanket": ((-PAD["x"], BL / 2, 0.0), 0.0),
    "carry-lift": ((0.0, 0.0, 0.0), 0.0),
    "carry-travel": ((0.0, 0.0, 0.0), 0.0),
    "fins": ((0.0, yw(FIN["y0"]), Z_STACK), 90.0),
    "housing": ((0.0, yw(HOUSING["y0"]), Z_HOUSING), 90.0),
    "knob": ((0.0, yw(KNOB["y"]), Z_KNOB), 0.0),
    "feet": ((-FOOT["x"], yw((FOOT["y0"] + FOOT["y1"]) / 2), Z_STACK), 0.0),
    "castors": ((-CASTOR["x"], yw(CASTOR["r"]), Z_STACK), 0.0),
}
PARENT = {"ground": None, "blanket": "ground", "carry-lift": "ground", "carry-travel": "carry-lift", "fins": "carry-travel", "housing": "fins", "knob": "housing", "feet": "fins", "castors": "feet"}


def loc(bone, x, y):
    """📍 World rest (x, y) to the (u, v) frame of a bone."""
    (ox, oy, _), a = BONES_WORLD[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy = x - ox, y - oy
    return (dx * c + dy * s, -dx * s + dy * c)


def loc3(bone, x, y, z):
    """📍 World rest point to the (u, v, w) frame of a bone."""
    u, v = loc(bone, x, y)
    return (u, v, z - BONES_WORLD[bone][0][2])


def wz(bone, z):
    """↔ The local w of a world lateral position on a bone."""
    return z - BONES_WORLD[bone][0][2]


def rest_offset(bone):
    """🦴 Rest placement (x, y, z, angleDeg) of a bone in its parent frame from the world rest frames."""
    p = PARENT[bone]
    (ox, oy, oz), a = BONES_WORLD[bone]
    if p is None:
        return ox, oy, oz, a
    (px, py, pz), pa = BONES_WORLD[p]
    c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
    dx, dy = ox - px, oy - py
    return dx * c + dy * s, -dx * s + dy * c, oz - pz, a - pa


# ---------------------------------------------------------------- outlines

def rrect(x0, y0, x1, y1, rb, rt, n=4):
    """▢ A rectangle with bottom corner radius rb and top corner radius rt as a polygon."""
    pts = []
    for cx, cy, r, a0 in ((x1 - rb, y0 + rb, rb, -90), (x1 - rt, y1 - rt, rt, 0), (x0 + rt, y1 - rt, rt, 90), (x0 + rb, y0 + rb, rb, 180)):
        if r <= 0:
            pts.append((cx, cy))
            continue
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return pts


def racetrack(c0, c1, r, n=8):
    """🏟 A vertical stadium of radius r between the centres (0, c0) and (0, c1), counter-clockwise."""
    pts = []
    for k in range(n + 1):
        a = math.radians(-180 + 180 * k / n)
        pts.append((r * math.cos(a), c0 + r * math.sin(a)))
    for k in range(n + 1):
        a = math.radians(180 * k / n)
        pts.append((r * math.cos(a), c1 + r * math.sin(a)))
    return pts


def fin_outline(n=4):
    """🪟 Side profile of one fin in world rest (x, y)."""
    return rrect(-FIN["x"], yw(FIN["y0"]), FIN["x"], yw(FIN["y1"]), FIN["r_bot"], FIN["r_top"], n)


def housing_outline(n=3):
    """🎛 Side profile of the control housing in world rest (x, y)."""
    h = HOUSING
    return rrect(-h["x"], yw(h["y0"]), h["x"], yw(h["y1"]), h["r_bot"], h["r_top"], n)


def pad_section(fold=True, steps=6):
    """🧺 The folded pad in elevation: four layers with the rounded fold at the front edge."""
    p = PAD
    r = (p["y1"] - p["y0"]) / 2
    cy = (p["y0"] + p["y1"]) / 2
    if not fold:
        return [(-p["x"], p["y0"]), (p["x"], p["y0"]), (p["x"], p["y1"]), (-p["x"], p["y1"])]
    pts = [(-p["x"], p["y0"])]
    for k in range(steps + 1):
        a = -math.pi / 2 + math.pi * k / steps
        pts.append((p["x"] - r + r * math.cos(a), cy + r * math.sin(a)))
    pts.append((-p["x"], p["y1"]))
    return pts


def polygon_area(pts):
    """📐 Shoelace area of a simple polygon."""
    return abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2


def polygon_perimeter(pts):
    """📏 Perimeter of a closed polygon."""
    return sum(math.dist(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts)))


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


def rect_w(x0, y0, x1, y1):
    """▭ A rect geometry from two world corners (front, plan)."""
    return {"kind": "rect", "x": min(x0, x1), "y": min(y0, y1), "w": abs(x1 - x0), "h": abs(y1 - y0)}


def poly_w(points, closed=True):
    """⬠ A poly geometry from world points (front, plan)."""
    return {"kind": "poly", "points": [list(p) for p in points], "closed": closed}


def poly_e(bone, points, closed=True):
    """⬠ A poly geometry in the elevation frame of a bone from world points."""
    return {"kind": "poly", "points": [list(loc(bone, *p)) for p in points], "closed": closed}


def ellipse_e(bone, cx, cy, r):
    """⭕ A circle in the elevation frame of a bone from a world centre."""
    u, v = loc(bone, cx, cy)
    return {"kind": "ellipse", "cx": u, "cy": v, "rx": r, "ry": r}


def ring_path_e(bone, outer, inner):
    """◎ A filled ring with its hole as one path in the elevation frame of a bone."""
    def sub(pts):
        q = [loc(bone, *p) for p in pts]
        return "M " + " L ".join(f"{a:.5f} {b:.5f}" for a, b in q) + " Z"
    return {"kind": "path", "d": sub(outer) + " " + sub(list(reversed(inner)))}


def rects_path(rects):
    """▥ A row of separate rectangles as one filled path, one subpath per rectangle."""
    return {"kind": "path", "d": " ".join(f"M {x0:.5f} {y0:.5f} L {x1:.5f} {y0:.5f} L {x1:.5f} {y1:.5f} L {x0:.5f} {y1:.5f} Z" for x0, y0, x1, y1 in rects)}


def path_lines(segments):
    """〰 A path of straight strokes."""
    return {"kind": "path", "d": " ".join(f"M {a[0]:.5f} {a[1]:.5f} L {b[0]:.5f} {b[1]:.5f}" for a, b in segments)}


def path_lines_e(bone, segments):
    """〰 Straight strokes in the elevation frame of a bone from world points."""
    return path_lines([(loc(bone, *a), loc(bone, *b)) for a, b in segments])


def path_poly_e(bone, pts, closed=False):
    """〰 One polyline stroke in the elevation frame of a bone from world points."""
    q = [loc(bone, *p) for p in pts]
    return {"kind": "path", "d": "M " + " L ".join(f"{a:.5f} {b:.5f}" for a, b in q) + (" Z" if closed else "")}


def path_circles_e(bone, circles):
    """⭕ Circles as one stroked path in the elevation frame of a bone."""
    parts = []
    for cx, cy, r in circles:
        u, v = loc(bone, cx, cy)
        parts.append(f"M {u + r:.5f} {v:.5f} A {r:.5f} {r:.5f} 0 1 0 {u - r:.5f} {v:.5f} A {r:.5f} {r:.5f} 0 1 0 {u + r:.5f} {v:.5f}")
    return {"kind": "path", "d": " ".join(parts)}


def scale_parts(scale):
    """📏 The generalised part sizes of one scale."""
    if scale == 200:
        return dict(n=7, t=0.030, foot_y0=0.042, castor_block=True, knob_proud=0.030, cord_z=(Z_FACE - 0.030, Z_FACE), cord_ring=False, hooks=False, switch=False, corner_n=2)
    if scale == 100:
        return dict(n=FIN["n"], t=FIN["t"], foot_y0=FOOT["y0"], castor_block=False, knob_proud=KNOB["proud"], cord_z=(Z_FACE - 0.016, Z_FACE), cord_ring=True, hooks=False, switch=False, corner_n=3)
    return dict(n=FIN["n"], t=FIN["t"], foot_y0=FOOT["y0"], castor_block=False, knob_proud=KNOB["proud"], cord_z=(CORD["z_out"], CORD["z_in"]), cord_ring=True, hooks=True, switch=True, corner_n=4)


def castor_tone(i):
    """🛞 Tone of castor i: the front castor of the right foot is a black replacement."""
    return 0.15 if i == 0 else 0.55


CASTORS = [(1, 0), (-1, 0), (1, 1), (-1, 1)]


def geometry(scale):
    """🖼 The individually drawn elevation, front and plan of one scale."""
    s50, s100, s200 = scale == 50, scale == 100, scale == 200
    P_ = scale_parts(scale)
    E, F, P = [], [], []
    c, f, h, k = CASTOR, FOOT, HOUSING, KNOB
    fy0 = P_["foot_y0"]

    E.append(shape("pad", "blanket", "body", 1, 0.3, poly_e("blanket", pad_section(fold=s50)), "pad", "felt-pad"))
    if s50:
        E.append(shape("pad-layers", "blanket", "texture", 1.1, 0.2, path_lines_e("blanket", [((-PAD["x"], 0.003 * j), (-PAD["x"] + 0.06, 0.003 * j)) for j in (1, 2, 3)]), "pad", filled=False))
    E.append(shape("fin", "fins", "body", 2, 0.82, poly_e("fins", fin_outline(P_["corner_n"])), "fins", "enamelled-steel"))
    if s50 or s100:
        rim = FIN["rim"]
        inner = rrect(-FIN["x"] + rim, yw(FIN["y0"]) + rim, FIN["x"] - rim, yw(FIN["y1"]) - rim, FIN["r_bot"] - rim / 2, FIN["r_top"] - rim, 3)
        E.append(shape("fin-rim", "fins", "detail", 2.1, 0.6, path_poly_e("fins", inner, closed=True), "fins", filled=False))
    if s50:
        E.append(shape("fin-rust", "fins", "texture", 2.2, 0.4, path_lines_e("fins", [((-0.072, yw(0.080)), (-0.066, yw(0.086))), ((-0.071, yw(0.092)), (-0.066, yw(0.090))), ((0.066, yw(0.079)), (0.072, yw(0.084))), ((0.064, yw(0.612)), (0.070, yw(0.606)))]), "fins", filled=False))
    E.append(shape("foot", "feet", "body", 3, 0.45, poly_e("feet", [(-f["x"], yw(fy0)), (f["x"], yw(fy0)), (f["x"], yw(f["y1"])), (-f["x"], yw(f["y1"]))]), "feet", "painted-steel"))
    for i, sx in enumerate((1, -1)):
        side = "front" if sx > 0 else "rear"
        cx = sx * c["x"]
        if P_["castor_block"]:
            E.append(shape(f"castor-{side}", "castors", "body", 3.5, castor_tone(i), poly_e("castors", [(cx - 0.015, yw(0.0)), (cx + 0.015, yw(0.0)), (cx + 0.015, yw(fy0)), (cx - 0.015, yw(fy0))]), "castors", "castor-polyamide"))
        else:
            E.append(shape(f"castor-fork-{side}", "castors", "body", 3.5, 0.5, poly_e("castors", [(cx - c["fork_x"], yw(c["fork_y0"])), (cx + c["fork_x"], yw(c["fork_y0"])), (cx + c["fork_x"], yw(c["fork_y1"])), (cx - c["fork_x"], yw(c["fork_y1"]))]), "castors", "castor-polyamide"))
            E.append(shape(f"castor-wheel-{side}", "castors", "body", 3.6, castor_tone(i), ellipse_e("castors", cx, yw(c["r"]), c["r"]), "castors", "castor-rubber" if i == 0 else "castor-polyamide"))
    if s50:
        E.append(shape("castor-axles", "castors", "detail", 3.7, 0.3, path_circles_e("castors", [(sx * c["x"], yw(c["r"]), 0.004) for sx in (1, -1)]), "castors", filled=False))
    E.append(shape("housing", "housing", "body", 4, 0.88, poly_e("housing", housing_outline(P_["corner_n"] - 1)), "housing", "housing-plastic"))
    if s50 or s100:
        E.append(shape("housing-lip", "housing", "detail", 4.1, 0.6, path_lines_e("housing", [((-0.045, yw(0.600)), (0.045, yw(0.600))), ((-h["x"], yw(0.110)), (h["x"], yw(0.110)))]), "housing", filled=False))
    if s50:
        E.append(shape("switch-panel", "housing", "body", 4.2, 0.65, poly_e("housing", [(-SWITCH["x"], yw(SWITCH["y0"])), (SWITCH["x"], yw(SWITCH["y0"])), (SWITCH["x"], yw(SWITCH["y1"])), (-SWITCH["x"], yw(SWITCH["y1"]))]), "housing", "housing-plastic"))
        E.append(shape("switch-rockers", "housing", "detail", 4.3, 0.4, path_lines_e("housing", [((-0.002, yw(SWITCH["y0"])), (-0.002, yw(SWITCH["y1"]))), ((-0.018, yw(0.470)), (-0.008, yw(0.470))), ((0.008, yw(0.470)), (0.018, yw(0.470)))]), "housing", filled=False))
        E.append(shape("labels", "housing", "texture", 4.4, 0.5, path_lines_e("housing", [((-0.045, yw(0.380)), (-0.015, yw(0.380))), ((-0.015, yw(0.380)), (-0.015, yw(0.410))), ((-0.015, yw(0.410)), (-0.045, yw(0.410))), ((-0.045, yw(0.410)), (-0.045, yw(0.380))), ((0.010, yw(0.385)), (0.048, yw(0.385))), ((0.048, yw(0.385)), (0.048, yw(0.410))), ((0.048, yw(0.410)), (0.010, yw(0.410))), ((0.010, yw(0.410)), (0.010, yw(0.385))), ((0.016, yw(0.397)), (0.042, yw(0.397)))]), "housing", filled=False))
        E.append(shape("knob-scale", "housing", "texture", 4.5, 0.4, path_lines_e("housing", [((0.027 * math.cos(math.radians(a)), yw(k["y"]) + 0.027 * math.sin(math.radians(a))), (0.032 * math.cos(math.radians(a)), yw(k["y"]) + 0.032 * math.sin(math.radians(a)))) for a in (225, 180, 135, 90, 45, 0, -45)]), "housing", filled=False))
    if s100:
        E.append(shape("switch-outline", "housing", "detail", 4.25, 0.5, path_poly_e("housing", [(-SWITCH["x"], yw(SWITCH["y0"])), (SWITCH["x"], yw(SWITCH["y0"])), (SWITCH["x"], yw(SWITCH["y1"])), (-SWITCH["x"], yw(SWITCH["y1"]))], closed=True), "housing", filled=False))
    if P_["hooks"]:
        for hid, (a, b) in (("hook-low", HOOKS["low"]), ("hook-high", HOOKS["high"])):
            E.append(shape(hid, "housing", "body", 4.6, 0.7, poly_e("housing", [(-HOOKS["x"], yw(a)), (HOOKS["x"], yw(a)), (HOOKS["x"], yw(b)), (-HOOKS["x"], yw(b))]), "housing", "housing-plastic"))
    outer = [(x, yw(y)) for x, y in racetrack(CORD["c_low"], CORD["c_high"], CORD["r_out"], 8 if s50 else 4)]
    if P_["cord_ring"]:
        inner = [(x, yw(y)) for x, y in racetrack(CORD["c_low"], CORD["c_high"], CORD["r_in"], 8 if s50 else 4)]
        E.append(shape("cord", "housing", "body", 5, 0.25, ring_path_e("housing", outer, inner), "cord", "pvc-cord"))
    else:
        E.append(shape("cord", "housing", "body", 5, 0.25, poly_e("housing", outer), "cord", "pvc-cord"))
    if s50:
        mid = [(x, yw(y)) for x, y in racetrack(CORD["c_low"], CORD["c_high"], (CORD["r_out"] + CORD["r_in"]) / 2, 8)]
        E.append(shape("cord-turns", "housing", "texture", 5.1, 0.5, path_poly_e("housing", mid, closed=True), "cord", filled=False))
        E.append(shape("cord-tape", "housing", "texture", 5.2, 0.9, path_lines_e("housing", [((0.016, yw(0.240)), (0.032, yw(0.240))), ((0.016, yw(0.246)), (0.032, yw(0.246))), ((0.016, yw(0.252)), (0.032, yw(0.252)))]), "cord", filled=False))
    E.append(shape("knob", "knob", "body", 6, 0.3, ellipse_e("knob", 0.0, yw(k["y"]), k["r"]), "knob", "knob-plastic"))
    if s50 or s100:
        E.append(shape("knob-pointer", "knob", "detail", 6.1, 0.9, path_lines_e("knob", [((0.0, yw(k["y"])), (0.0, yw(k["y"]) + k["r"] - 0.004))]), "knob", filled=False))

    fins = [fin_z(j, P_["t"], P_["n"]) for j in range(P_["n"])]
    F.append(shape("pad", "blanket", "body", 1, 0.3, rect_w(-PAD["z"], PAD["y0"], PAD["z"], PAD["y1"]), "pad", "felt-pad"))
    for nm, yc in (("hub-low", HUB["y_low"]), ("hub-high", HUB["y_high"])):
        F.append(shape(nm, "fins", "body", 1.5, 0.6, rect_w(Z_FIN0, yw(yc - HUB["r"]), Z_FIN1, yw(yc + HUB["r"])), "fins", "enamelled-steel"))
    F.append(shape("fin-row", "fins", "body", 2, 0.82, rects_path([(z0, yw(FIN["y0"]), z1, yw(FIN["y1"])) for z0, z1 in fins]), "fins", "enamelled-steel"))
    if s50:
        F.append(shape("fin-seams", "fins", "detail", 2.1, 0.6, path_lines([((sum(zz) / 2, yw(FIN["y0"]) + 0.01), (sum(zz) / 2, yw(FIN["y1"]) - 0.01)) for zz in fins]), "fins", filled=False))
        F.append(shape("fin-scratches", "fins", "texture", 2.2, 0.4, path_lines([((fins[2][0] + 0.004, yw(0.30)), (fins[2][0] + 0.018, yw(0.34))), ((fins[5][0] + 0.003, yw(0.45)), (fins[5][0] + 0.016, yw(0.43))), ((fins[8][0] + 0.004, yw(0.10)), (fins[8][0] + 0.018, yw(0.12)))]), "fins", filled=False))
    for j, zc in enumerate(FOOT_Z):
        F.append(shape(f"foot-{j + 1}", "feet", "body", 3, 0.45, rect_w(zc - f["w"] / 2, yw(fy0), zc + f["w"] / 2, yw(f["y1"])), "feet", "painted-steel"))
        if P_["castor_block"]:
            F.append(shape(f"castor-{j + 1}", "castors", "body", 3.5, castor_tone(j), rect_w(zc - 0.015, yw(0.0), zc + 0.015, yw(fy0)), "castors", "castor-polyamide"))
        else:
            F.append(shape(f"castor-fork-{j + 1}", "castors", "body", 3.5, 0.5, rect_w(zc - c["fork_w"] / 2, yw(c["fork_y0"]), zc + c["fork_w"] / 2, yw(c["fork_y1"])), "castors", "castor-polyamide"))
            F.append(shape(f"castor-wheel-{j + 1}", "castors", "body", 3.6, castor_tone(j), rect_w(zc - c["w"] / 2, yw(0.0), zc + c["w"] / 2, yw(2 * c["r"])), "castors", "castor-polyamide"))
    F.append(shape("housing", "housing", "body", 4, 0.88, rect_w(Z_FACE, yw(h["y0"]), Z_FACE + h["t"], yw(h["y1"])), "housing", "housing-plastic"))
    if s50:
        F.append(shape("switch-panel", "housing", "body", 4.2, 0.65, rect_w(Z_FACE - SWITCH["proud"], yw(SWITCH["y0"]), Z_FACE, yw(SWITCH["y1"])), "housing", "housing-plastic"))
    if P_["hooks"]:
        for hid, (a, b) in (("hook-low", HOOKS["low"]), ("hook-high", HOOKS["high"])):
            F.append(shape(hid, "housing", "body", 4.6, 0.7, rect_w(CORD["z_out"], yw(a), Z_FACE, yw(b)), "housing", "housing-plastic"))
    cz0, cz1 = P_["cord_z"]
    F.append(shape("cord", "housing", "body", 5, 0.25, rect_w(cz0, yw(CORD["c_low"] - CORD["r_out"]), cz1, yw(CORD["c_high"] + CORD["r_out"])), "cord", "pvc-cord"))
    kp = P_["knob_proud"]
    F.append(shape("knob-side", "housing", "body", 6, 0.3, rect_w(Z_FACE - kp, yw(k["y"] - k["r"]), Z_FACE, yw(k["y"] + k["r"])), "knob", "knob-plastic"))

    P.append(shape("pad", "blanket", "body", 1, 0.3, rect_w(-PAD["x"], -PAD["z"], PAD["x"], PAD["z"]), "pad", "felt-pad", hatch="quilt" if s50 else None))
    for j, zc in enumerate(FOOT_Z):
        P.append(shape(f"foot-{j + 1}", "feet", "body", 2, 0.45, rect_w(-f["x"], zc - f["w"] / 2, f["x"], zc + f["w"] / 2), "feet", "painted-steel"))
    P.append(shape("hub-high", "fins", "body", 2.5, 0.6, rect_w(-HUB["r"], Z_FIN0, HUB["r"], Z_FIN1), "fins", "enamelled-steel"))
    P.append(shape("fin-row", "fins", "body", 3, 0.82, rects_path([(-FIN["x"], z0, FIN["x"], z1) for z0, z1 in fins]), "fins", "enamelled-steel"))
    if s50:
        P.append(shape("fin-seams", "fins", "detail", 3.1, 0.6, path_lines([((-FIN["x"] + 0.004, sum(zz) / 2), (FIN["x"] - 0.004, sum(zz) / 2)) for zz in fins]), "fins", filled=False))
        P.append(shape("fin-dust", "fins", "texture", 3.2, 0.5, path_lines([((-0.05, fins[3][1] + 0.006), (0.03, fins[3][1] + 0.006)), ((-0.02, fins[6][1] + 0.008), (0.05, fins[6][1] + 0.008))]), "fins", filled=False))
    P.append(shape("housing", "housing", "body", 4, 0.88, rect_w(-h["x"], Z_FACE, h["x"], Z_FACE + h["t"]), "housing", "housing-plastic"))
    if s50 or s100:
        P.append(shape("housing-grip", "housing", "detail", 4.1, 0.6, path_lines([((-0.045, Z_FACE + 0.012), (0.045, Z_FACE + 0.012))]), "housing", filled=False))
    if s50:
        P.append(shape("switch-panel", "housing", "body", 4.2, 0.65, rect_w(-SWITCH["x"], Z_FACE - SWITCH["proud"], SWITCH["x"], Z_FACE), "housing", "housing-plastic"))
    if P_["hooks"]:
        P.append(shape("hook-high", "housing", "body", 4.6, 0.7, rect_w(-HOOKS["x"], CORD["z_out"], HOOKS["x"], Z_FACE), "housing", "housing-plastic"))
    P.append(shape("cord", "housing", "body", 5, 0.25, rect_w(-CORD["r_out"], cz0, CORD["r_out"], cz1), "cord", "pvc-cord"))
    P.append(shape("knob-side", "housing", "body", 6, 0.3, rect_w(-k["r"], Z_FACE - kp, k["r"], Z_FACE), "knob", "knob-plastic"))
    return {"elevation": E, "front": F, "plan": P}


# ---------------------------------------------------------------- 3D solids

def box(sid, bone, lo, hi, material, tone, role="body", **extra):
    """📦 An axis-aligned world box as a box solid in the frame of its bone."""
    a = loc3(bone, *lo)
    b = loc3(bone, *hi)
    centre = [(a[i] + b[i]) / 2 for i in range(3)]
    size = [abs(a[i] - b[i]) for i in range(3)]
    s = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": centre}}
    s.update(extra)
    return s


def extrude_xy(sid, bone, world_pts, z0, z1, material, tone, role="body", holes=None, **extra):
    """🧱 A prism of a world side profile swept laterally from z0 to z1 in the frame of its bone."""
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "plane": "xy", "profile": [list(loc(bone, *p)) for p in world_pts], "from": wz(bone, min(z0, z1)), "to": wz(bone, max(z0, z1))}
    if holes:
        s["holes"] = [[list(loc(bone, *p)) for p in hole] for hole in holes]
    s.update(extra)
    return s


def cylinder_w(sid, bone, cx, cy, r, z0, z1, material, tone, role="body", **extra):
    """🛞 A cylinder about the lateral axis through the world point (cx, cy) from z0 to z1."""
    u, v = loc(bone, cx, cy)
    a0, a1 = wz(bone, min(z0, z1)), wz(bone, max(z0, z1))
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[a0, 0.0], [a0, r], [a1, r], [a1, 0.0]], "axis": [0, 0, 1], "origin": [u, v, 0.0]}
    s.update(extra)
    return s


def body_solids(scale):
    """🧊 The body of one scale."""
    P_ = scale_parts(scale)
    c, f, h, k = CASTOR, FOOT, HOUSING, KNOB
    out = []
    pad = [(x, y) for x, y in pad_section(fold=scale == 50)]
    extra = {} if scale == 50 else {"sheet": True}
    out.append(extrude_xy("pad", "blanket", pad, -PAD["z"], PAD["z"], "felt-pad", 0.3, **extra))
    z0, z1 = fin_z(0, P_["t"], P_["n"])
    pitch = fin_z(1, P_["t"], P_["n"])[0] - z0
    fin_extra = {"repeat": {"count": P_["n"], "translation": [0, 0, pitch]}}
    out.append(extrude_xy("fins", "fins", fin_outline(P_["corner_n"]), z0, z1, "enamelled-steel", 0.82, **fin_extra))
    for nm, yc in (("hub-low", HUB["y_low"]), ("hub-high", HUB["y_high"])):
        out.append(cylinder_w(nm, "fins", 0.0, yw(yc), HUB["r"], Z_FIN0, Z_FIN1, "enamelled-steel", 0.6))
    fy0 = P_["foot_y0"]
    for j, zc in enumerate(FOOT_Z):
        out.append(box(f"foot-{j + 1}", "feet", (-f["x"], yw(fy0), zc - f["w"] / 2), (f["x"], yw(f["y1"]), zc + f["w"] / 2), "painted-steel", 0.45))
    i = 0
    for j, zc in enumerate(FOOT_Z):
        for sx in (1, -1):
            cx = sx * c["x"]
            tag = f"{'f' if sx > 0 else 'r'}{j + 1}"
            tone = castor_tone(i)
            mat = "castor-rubber" if i == 0 else "castor-polyamide"
            if P_["castor_block"]:
                out.append(box(f"castor-{tag}", "castors", (cx - 0.015, yw(0.0), zc - 0.015), (cx + 0.015, yw(fy0), zc + 0.015), mat, tone))
            else:
                out.append(box(f"castor-fork-{tag}", "castors", (cx - c["fork_x"], yw(c["fork_y0"]), zc - c["fork_w"] / 2), (cx + c["fork_x"], yw(c["fork_y1"]), zc + c["fork_w"] / 2), "castor-polyamide", 0.5))
                out.append(cylinder_w(f"castor-wheel-{tag}", "castors", cx, yw(c["r"]), c["r"], zc - c["w"] / 2, zc + c["w"] / 2, mat, tone))
            i += 1
    out.append(extrude_xy("housing", "housing", housing_outline(P_["corner_n"] - 1), Z_FACE, Z_FACE + h["t"], "housing-plastic", 0.88))
    if P_["switch"]:
        out.append(box("switch-panel", "housing", (-SWITCH["x"], yw(SWITCH["y0"]), Z_FACE - SWITCH["proud"]), (SWITCH["x"], yw(SWITCH["y1"]), Z_FACE), "housing-plastic", 0.65))
    if P_["hooks"]:
        for hid, (a, b) in (("hook-low", HOOKS["low"]), ("hook-high", HOOKS["high"])):
            out.append(box(hid, "housing", (-HOOKS["x"], yw(a), CORD["z_out"]), (HOOKS["x"], yw(b), Z_FACE), "housing-plastic", 0.7))
    cz0, cz1 = P_["cord_z"]
    outer = [(x, yw(y)) for x, y in racetrack(CORD["c_low"], CORD["c_high"], CORD["r_out"], 8 if scale == 50 else 4)]
    holes = [[(x, yw(y)) for x, y in racetrack(CORD["c_low"], CORD["c_high"], CORD["r_in"], 8 if scale == 50 else 4)]] if P_["cord_ring"] else None
    out.append(extrude_xy("cord", "housing", outer, cz0, cz1, "pvc-cord", 0.25, holes=holes))
    kp = P_["knob_proud"]
    out.append(cylinder_w("knob", "knob", 0.0, yw(k["y"]), k["r"], Z_FACE - kp, Z_FACE, "knob-plastic", 0.3))
    return out


REGION_OF = {"pad": "pad", "fins": "fins", "hub-low": "fins", "hub-high": "fins", "housing": "housing", "switch-panel": "housing", "hook-low": "housing", "hook-high": "housing", "cord": "cord", "knob": "knob"}
MEMBER_OF = {"pad": "blanket", "fins": "fins", "hub-low": "fins", "hub-high": "fins", "housing": "housing", "switch-panel": "housing", "hook-low": "housing", "hook-high": "housing", "cord": "housing", "knob": "knob"}


def region_of(sid):
    """🌡 The thermal region of a body solid."""
    if sid in REGION_OF:
        return REGION_OF[sid]
    if sid.startswith("foot-"):
        return "feet"
    if sid.startswith("castor-"):
        return "castors"
    raise ValueError(sid)


def member_of(sid):
    """🏗 The member bone of a body solid."""
    if sid in MEMBER_OF:
        return MEMBER_OF[sid]
    if sid.startswith("foot-"):
        return "feet"
    if sid.startswith("castor-"):
        return "castors"
    raise ValueError(sid)


def disc(sid, bone, member, anchor, origin, r, h):
    """🛞 A support pad on a support anchor, rising from the floor."""
    return {"id": sid, "kind": "revolve", "bone": bone, "material": "felt-pad", "role": "ground", "tone": 0.2, "profile": [[0.0, 0.0], [0.0, r], [h, r], [h, 0.0]], "axis": [0, 1, 0], "origin": list(origin), "member": member, "part": "support", "anchor": anchor}


def mass_marker(sid, bone, at, r, material):
    """⚖ A centre-of-mass marker."""
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": list(at)}, "member": bone, "part": "mass"}


def volume_lists(scale, body):
    """🧊 Thermal and structural lists of one scale from its body list."""
    fe = FEATURE[scale]
    P_ = scale_parts(scale)
    thermal = [{"id": f"th-{s['id']}", "of": s["id"], "region": region_of(s["id"])} for s in body]
    thermal.append(cylinder_w("th-plume", "fins", 0.0, yw(HUB["y_high"]), {50: 0.012, 100: 0.014, 200: 0.016}[scale], Z_FIN0 + 0.005, Z_FIN1 - 0.005, "enamelled-steel", 0.6, region="fins", tags=["plume"]))
    structure = [{"id": f"m-{s['id']}", "of": s["id"], "member": member_of(s["id"]), "part": "member"} for s in body]
    j = max(fe, 0.02)
    for n, zc in enumerate(FOOT_Z):
        structure.append(box(f"j-clamp-{n + 1}", "feet", (-0.015, yw(FOOT["y1"]) - 0.004, zc - 0.015), (0.015, yw(FOOT["y1"]) - 0.004 + j, zc + 0.015), "painted-steel", 0.3, role="detail", member="feet", part="joint"))
        for sx in (1, -1):
            cx = sx * CASTOR["x"]
            y1 = yw(P_["foot_y0"]) + 0.004
            structure.append(box(f"j-stem-{'f' if sx > 0 else 'r'}{n + 1}", "castors", (cx - j / 2, y1 - j, zc - j / 2), (cx + j / 2, y1, zc + j / 2), "painted-steel", 0.3, role="detail", member="castors", part="joint"))
    structure.append(box("j-housing-screws", "housing", (-0.015, yw(0.330), Z_FIN0 - j / 2), (0.015, yw(0.330) + max(j, 0.03), Z_FIN0 + j / 2), "painted-steel", 0.3, role="detail", member="housing", part="joint"))
    kj = max(fe, 0.015)
    structure.append(box("j-knob-spindle", "knob", (-kj / 2, yw(KNOB["y"]) - kj / 2, Z_FACE - kj / 2), (kj / 2, yw(KNOB["y"]) + kj / 2, Z_FACE + kj / 2), "painted-steel", 0.3, role="detail", member="knob", part="joint"))
    for a in ANCHORS:
        if a["id"].startswith("pad-floor-"):
            structure.append(disc(f"support-{a['id']}", "blanket", "blanket", a["id"], (a["x"], a["y"], a["z"]), fe, fe if scale == 50 else 0.012))
    M = masses()
    mr = fe
    for bone in ("fins", "housing", "feet"):
        kg, (u, v) = M[bone]
        if bone == "feet":
            v = loc("feet", 0.0, yw(FIN["y0"]) + mr + 0.002)[1]
        structure.append(mass_marker(f"mass-{bone}", bone, (u, v, 0.0), mr, "enamelled-steel" if bone != "housing" else "housing-plastic"))
    return thermal, structure


VOLUME_TEXTS = {
    50: [T("Body, authored for 7.5 mm: nine pressed fins 150 x 568 mm and 22 mm thick at a pitch of 42 mm (one extrusion of the true rounded fin profile, repeated), the upper and lower oil hubs of 36 mm running through the stack, the control housing 120 x 525 x 65 mm with its rounded top, the switch panel 8 mm proud, the replacement thermostat knob 44 mm across and 20 mm proud, the cord wound into a 16 x 14 mm bundle on two moulded hooks 8 mm off the end face, two pressed steel foot bars 250 x 30 x 20 mm under fins 2 and 8 with four swivel castors (forks and 40 mm wheels, the front right one a black replacement), and the furniture pad folded in four with its rounded fold.",
           "Körper für 7,5 mm: neun gepresste Rippen 150 x 568 mm, 22 mm dick, im Abstand von 42 mm (eine Extrusion des wahren gerundeten Rippenprofils, wiederholt), die obere und untere Ölnabe von 36 mm durch das Paket, das Bediengehäuse 120 x 525 x 65 mm mit gerundetem Kopf, das Schalterfeld 8 mm vorstehend, der Ersatz-Thermostatknopf mit 44 mm Durchmesser und 20 mm Überstand, das zu einem 16 x 14 mm starken Bündel auf zwei angeformte Haken 8 mm vor der Stirnseite aufgewickelte Kabel, zwei Fußbügel aus gepresstem Stahl 250 x 30 x 20 mm unter Rippe 2 und 8 mit vier Lenkrollen (Gabeln und 40-mm-Räder, die vordere rechte ein schwarzer Ersatz) und das viermal gefaltete Möbelpolster mit gerundetem Falz."),
         T("Thermal: every body solid by reference in its region (hubs to the fins, hooks and switch panel to the housing). Structure: every body solid as a member of its bone; the U-bolt clamps of the feet, the castor stems, the housing screws and the knob spindle as joints; one pad per permanent support (the four corners of the furniture pad on the floor; castor treads and hands are switched contacts of the poses and clips and carry no pad); masses at the centres of fins and housing and, for the two foot bars whose common centre lies in the open space between them, in the bottom of the middle fin above it.",
           "Thermik: jeder Körper als Verweis in seiner Region (Naben zu den Rippen, Haken und Schalterfeld zum Gehäuse). Tragwerk: jeder Körper als Stab seines Knochens; die Bügelschrauben der Füße, die Rollenzapfen, die Gehäuseschrauben und die Knopfachse als Knoten; je ständigem Auflager eine Scheibe (die vier Ecken des Polsters auf dem Boden; Laufflächen der Rollen und Hände sind geschaltete Kontakte der Posen und Clips ohne Scheibe); Massen in den Schwerpunkten von Rippen und Gehäuse und, für die beiden Fußbügel, deren gemeinsamer Schwerpunkt im freien Raum zwischen ihnen liegt, im Fuß der mittleren Rippe darüber.")],
    100: [T("Body, authored anew for 15 mm: the nine fins keep their true 22 mm and the 20 mm gaps, hubs, housing, foot bars and castors keep their sections; the 8 mm switch panel goes into the housing face, the cord bundle lies flush on the end face as a 16 mm ring with the hooks merged into it, the knob stays 20 mm proud, the pad is one flat 12 mm sheet (declared sheet), the fin corners are traced with fewer points.",
            "Körper für 15 mm neu gebaut: die neun Rippen behalten ihre wahren 22 mm und die 20-mm-Zwischenräume, Naben, Gehäuse, Fußbügel und Rollen behalten ihre Querschnitte; das 8-mm-Schalterfeld geht in der Gehäusestirn auf, das Kabelbündel liegt als 16-mm-Ring bündig auf der Stirnseite, die Haken gehen darin auf, der Knopf bleibt 20 mm vorstehend, das Polster ist eine ebene 12-mm-Lage (deklariertes Blatt), die Rippenecken sind mit weniger Punkten gezogen."),
          T("Thermal: one surface per region by reference. Structure: the same members, clamps, stems, screws and spindle at 15 mm, one pad per pad corner, the masses.",
            "Thermik: eine Oberfläche je Region als Verweis. Tragwerk: dieselben Stäbe, Bügel, Zapfen, Schrauben und Achse bei 15 mm, je Polsterecke eine Auflagerscheibe, die Massen.")],
    200: [T("Body, authored anew for 30 mm: the row of nine 22 mm fins is typified as seven fins of 30 mm over the true stack length (fewer, thicker fins on the fins bone; the gaps stay open at 25 mm, so the row keeps its outer area and its volume within the cross-scale band without an envelope declaration), the hubs keep their 36 mm, each foot bar is 30 mm high and each castor one 30 mm block under it, the knob a 44 mm disc 30 mm proud, the cord a solid 64 x 190 mm plate 30 mm proud, the pad a 12 mm sheet.",
            "Körper für 30 mm neu gebaut: die Reihe der neun 22-mm-Rippen ist als sieben Rippen zu 30 mm über die wahre Paketlänge typisiert (weniger, dickere Rippen am Rippenknochen; die Zwischenräume bleiben mit 25 mm offen, sodass die Reihe ihre Außenfläche und ihr Volumen ohne Hüllendeklaration im Band zwischen den Maßstäben hält), die Naben behalten 36 mm, jeder Fußbügel ist 30 mm hoch und jede Rolle ein 30-mm-Block darunter, der Knopf eine 44-mm-Scheibe mit 30 mm Überstand, das Kabel eine volle Platte 64 x 190 mm mit 30 mm Überstand, das Polster eine 12-mm-Lage."),
          T("Thermal: one surface per region by reference; the seven typified fins carry the fin region (about 18 % less outer area than the nine true fins, inside the 30 % of a coarse scale). Structure: members by reference, joints at 30 mm, one pad per pad corner, masses.",
            "Thermik: eine Oberfläche je Region als Verweis; die sieben typisierten Rippen tragen die Rippenregion (etwa 18 % weniger Außenfläche als die neun wahren Rippen, innerhalb der 30 % eines groben Maßstabs). Tragwerk: Stäbe als Verweis, Knoten bei 30 mm, je Polsterecke eine Auflagerscheibe, Massen.")],
}


def build_volumes():
    """🧊 The three individually authored volumes and the unavailable 1:500."""
    vols = {}
    for scale in (50, 100, 200):
        body = body_solids(scale)
        thermal, structure = volume_lists(scale, body)
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOLUME_TEXTS[scale]}
    vols["500"] = {"status": "unavailable", "denominator": 500}
    return vols


# ---------------------------------------------------------------- masses and anchors

def masses():
    """⚖ Bone masses from section x density and their centres (local frames)."""
    a_fin = polygon_area(fin_outline(8))
    m_shell = FIN["n"] * 2 * a_fin * T_SHELL * RHO["enamelled-steel"]
    m_hub = 2 * (FIN["n"] - 1) * math.pi * 2 * HUB["r"] * (FIN["pitch"] - FIN["t"]) * T_SHELL * RHO["enamelled-steel"]
    h_fin = FIN["y1"] - FIN["y0"]
    u_shell = h_fin / 2
    u_oil = 0.85 * h_fin / 2
    u_el = HUB["y_low"] - FIN["y0"]
    m_fins = m_shell + m_hub + OIL_KG + ELEMENT_KG
    u_fins = (m_shell * u_shell + m_hub * h_fin / 2 + OIL_KG * u_oil + ELEMENT_KG * u_el) / m_fins
    a_h = polygon_area(housing_outline(8))
    p_h = polygon_perimeter(housing_outline(8))
    m_hshell = (2 * a_h + p_h * HOUSING["t"]) * 0.0025 * RHO["housing-plastic"]
    m_electrics = 0.18
    m_cord = CORD_M * CORD_KG_M
    m_hooks = 0.03
    m_housing = m_hshell + m_electrics + m_cord + m_hooks
    u_housing = (m_hshell * (HOUSING["y1"] - HOUSING["y0"]) / 2 + m_electrics * (0.47 - HOUSING["y0"]) + (m_cord + m_hooks) * (0.25 - HOUSING["y0"])) / m_housing
    a_channel = (FOOT["w"] + 2 * (FOOT["y1"] - FOOT["y0"] - 0.0012)) * 0.0012
    m_feet = 2 * a_channel * 2 * FOOT["x"] * RHO["painted-steel"] + 2 * 0.04
    m_castors = 4 * 0.045
    m_knob = 0.015
    m_pad = 2 * PAD["x"] * 2 * PAD["z"] * 4 * 0.4
    return {
        "blanket": (m_pad, (PAD["x"], 0.0)),
        "fins": (m_fins, (u_fins, 0.0)),
        "housing": (m_housing, (u_housing, 0.0)),
        "knob": (m_knob, (0.0, 0.0)),
        "feet": (m_feet, (FOOT["x"], 0.0)),
        "castors": (m_castors, (CASTOR["x"], 0.0)),
        "_parts": dict(a_fin=a_fin, m_shell=m_shell, m_hub=m_hub, m_hshell=m_hshell, m_cord=m_cord, a_channel=a_channel),
    }


PAD_CORNERS = {"pad-floor-fl": (1, 1), "pad-floor-fr": (1, -1), "pad-floor-rl": (-1, 1), "pad-floor-rr": (-1, -1)}
CASTOR_IDS = {"castor-fl": (1, 1), "castor-fr": (1, 0), "castor-rl": (-1, 1), "castor-rr": (-1, 0)}
GRIPS = ["grip-housing", "grip-end-fin"]
NAMES = {"fl": T("front left", "vorne links"), "fr": T("front right", "vorne rechts"), "rl": T("rear left", "hinten links"), "rr": T("rear right", "hinten rechts")}


def anchors():
    """🪝 Pad corners on the floor, castor treads, the carrier's two hands and the plume source."""
    out = []
    for aid, (sx, sz) in PAD_CORNERS.items():
        n = NAMES[aid[-2:]]
        u, v, w = loc3("blanket", sx * 0.18, 0.0, sz * 0.25)
        out.append({"id": aid, "bone": "blanket", "x": u, "y": v, "z": w, "kind": "contact", "title": T(f"Corner of the folded pad on the floor, {n['en']}", f"Ecke des gefalteten Polsters auf dem Boden, {n['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.12, "frictionCoefficient": 0.5})
    for aid, (sx, j) in CASTOR_IDS.items():
        n = NAMES[aid[-2:]]
        u, v, w = loc3("castors", sx * CASTOR["x"], yw(0.0), FOOT_Z[j])
        out.append({"id": aid, "bone": "castors", "x": u, "y": v, "z": w, "kind": "contact", "title": T(f"Castor tread, {n['en']}", f"Lauffläche der Rolle, {n['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.008, "footprintWidthM": CASTOR["w"], "frictionCoefficient": 0.05})
    u, v, w = loc3("housing", 0.0, yw(HOUSING["y1"]), Z_HOUSING)
    out.append({"id": "grip-housing", "bone": "housing", "x": u, "y": v, "z": w, "kind": "contact", "title": T("Carrier's right hand under the top lip of the control housing", "Rechte Hand des Trägers unter der oberen Kante des Bediengehäuses"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.03, "frictionCoefficient": 0.6})
    zc = sum(fin_z(FIN["n"] - 1)) / 2
    u, v, w = loc3("fins", 0.0, yw(FIN["y1"]), zc)
    out.append({"id": "grip-end-fin", "bone": "fins", "x": u, "y": v, "z": w, "kind": "contact", "title": T("Carrier's left hand round the top of the far end fin", "Linke Hand des Trägers um den Kopf der äußeren Endrippe"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.03, "frictionCoefficient": 0.6})
    u, v, w = loc3("fins", 0.0, yw(FIN["y1"]), Z_STACK)
    out.append({"id": "plume-top", "bone": "fins", "x": u, "y": v, "z": w, "kind": "emitter", "title": T("Warm air rising from the top of the fin stack", "Warme Luft steigt über dem Rippenpaket auf")})
    return out


ANCHORS = anchors()


# ---------------------------------------------------------------- document parts

REFS = [
    ("iec-60335-2-30", "IEC 60335-2-30:2009+A1:2016 Household and similar electrical appliances - Safety - Part 2-30: Particular requirements for room heaters (liquid-filled radiators, surface temperature limits, marking 'do not cover', stability and tip-over switch requirements)."),
    ("en-60335-1", "EN 60335-1:2012 Household and similar electrical appliances - Safety - Part 1: General requirements (supply cords, cord storage, stability of appliances on castors)."),
    ("din-vde-0701-0702", "DIN VDE 0701-0702:2008 Prüfung nach Instandsetzung, Änderung elektrischer Geräte - Wiederholungsprüfung elektrischer Geräte (the electrical safety test of second-hand appliances before resale, with its test sticker)."),
    ("dguv-v3", "DGUV Vorschrift 3 Elektrische Anlagen und Betriebsmittel (2005): periodic testing of portable electrical equipment, the test label with its next due date."),
    ("en-10130-2006", "EN 10130:2006 Cold rolled low carbon steel flat products for cold forming (DC01, the deep-drawing sheet of pressed radiator fins, yield strength up to 280 MPa)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel 50 W/mK and 450 J/kgK, plastics 0.17-0.25 W/mK, nonwoven felts about 0.04-0.05 W/mK)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of enamel and paints 0.85-0.95, solar absorptance of white and yellowed paints, free convection of vertical plates and plumes)."),
    ("vdi-waermeatlas-2013", "VDI-Wärmeatlas, 11th ed. (2013), Springer Vieweg (specific heat and density of mineral heat-transfer oils, about 1.9 kJ/kgK and 870 kg/m3 at 20 degC)."),
    ("din-en-50525-2-11", "DIN EN 50525-2-11:2012 Electric cables - Low voltage energy cables - Cables for general applications - Flexible cables with thermoplastic PVC insulation (H05VV-F 3G0.75: about 6.4 mm diameter, about 60-70 g/m)."),
    ("oil-radiator-catalogue", "Catalogue-typical data of oil-filled electric radiators as German DIY stores and department stores sold them from the 1990s to the 2010s, 9 fins, 2000 W in three stages (800 / 1200 / 2000 W), mechanical thermostat: about 64 x 43 x 25 cm over the castor feet, fins 15 cm deep, 9.5-12.5 kg, 1.5-1.9 m cord with a cord winder (authored-estimate from listings of the type, no brand)."),
    ("castor-catalogue", "Catalogue-typical data of the swivel castors of radiator feet: polyamide twin or single wheels of 35-45 mm on a pressed fork with a push-in stem, about 40-50 g each (authored-estimate)."),
    ("moving-blanket-catalogue", "Catalogue-typical data of quilted furniture pads (Möbeldecken, Umzugsdecken) of nonwoven felt, 120 x 90 cm, 3 mm, about 0.4 kg/m2 (authored-estimate)."),
    ("reuse-depot-practice", "Practice of municipal second-hand department stores and reuse depots in Germany (Gebrauchtwarenkaufhaus, Haushaltsauflösung): small electrical appliances are cleaned, tested to DIN VDE 0701-0702, labelled with test sticker and price, the cord wound up, and stood on a pad or cardboard in the electrical department (authored-estimate from the practice described in reuse programmes, no single source)."),
    ("dguv-208-033", "DGUV Information 208-033 (2014) Muskel-Skelett-Belastungen - erkennen und beurteilen (manual handling: loads up to about 15 kg carried with both hands close to the body by one adult without restriction)."),
    ("din-18011-1992", "DIN 18011:1992-11 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (placing areas in living rooms; a mobile heater stands in front of a wall or beside the seating group)."),
]


def materials():
    """🧪 The materials with their aged surface properties."""
    return [
        {"id": "enamelled-steel", "title": T("Pressed DC01 sheet steel with cream-white stove enamel, yellowed", "Gepresstes Stahlblech DC01 mit cremeweißer Einbrennlackierung, vergilbt"), "emissivity": 0.92, "solarAbsorptivity": 0.36, "densityKgM3": RHO["enamelled-steel"], "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 2.1e11, "strengthPa": 2.1e8, "source": "literature", "reference": "en-10130-2006"},
        {"id": "housing-plastic", "title": T("White polypropylene housing, yellowed by heat", "Weißes Polypropylen-Gehäuse, durch Wärme vergilbt"), "emissivity": 0.92, "solarAbsorptivity": 0.40, "densityKgM3": RHO["housing-plastic"], "conductivityWmK": 0.2, "specificHeatJKgK": 1700, "elasticModulusPa": 1.5e9, "strengthPa": 3.0e7, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "knob-plastic", "title": T("Dark grey ABS of the replacement thermostat knob", "Dunkelgraues ABS des Ersatz-Thermostatknopfs"), "emissivity": 0.92, "solarAbsorptivity": 0.85, "densityKgM3": RHO["knob-plastic"], "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "pvc-cord", "title": T("White PVC-sheathed cord H05VV-F 3G0.75, wound", "Weiße PVC-Leitung H05VV-F 3G0,75, aufgewickelt"), "emissivity": 0.92, "solarAbsorptivity": 0.40, "densityKgM3": RHO["pvc-cord"], "conductivityWmK": 0.16, "specificHeatJKgK": 1000, "source": "literature", "reference": "din-en-50525-2-11"},
        {"id": "painted-steel", "title": T("Grey painted pressed steel of the foot bars", "Grau lackierter gepresster Stahl der Fußbügel"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO["painted-steel"], "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "literature", "reference": "en-10130-2006"},
        {"id": "castor-polyamide", "title": T("Grey polyamide castors", "Graue Polyamid-Lenkrollen"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO["castor-polyamide"], "conductivityWmK": 0.25, "specificHeatJKgK": 1600, "source": "authored-estimate", "reference": "castor-catalogue"},
        {"id": "castor-rubber", "title": T("Black rubber-tyred replacement castor", "Schwarze gummibereifte Ersatzrolle"), "emissivity": 0.94, "solarAbsorptivity": 0.9, "densityKgM3": RHO["castor-rubber"], "conductivityWmK": 0.2, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "castor-catalogue"},
        {"id": "felt-pad", "title": T("Quilted nonwoven felt of a furniture pad, dark blue", "Gesteppter Vliesfilz eines Möbelpolsters, dunkelblau"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": RHO["felt-pad"], "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "moving-blanket-catalogue"},
    ]


def section(area, inertia, fibre, material, en, de):
    """📏 A member cross-section."""
    return {"areaM2": area, "secondMomentM4": inertia, "extremeFibreM": fibre, "material": material, "title": T(en, de)}


def rig():
    """🦴 The rig: the pad stays behind, the radiator is lifted and carried away by the carrier's two hands, the knob turns."""
    M = masses()

    def mass(bone, ref, src="authored-estimate"):
        kg, (u, v) = M[bone]
        return {"kg": kg, "comX": u, "comY": v, "distribution": "point", "source": src, "reference": ref}

    def place(bone):
        x, y, z, a = rest_offset(bone)
        out = {"x": x, "y": y}
        if abs(z) > 1e-12:
            out["z"] = z
        out["angleDeg"] = a
        return out

    fin_area = FIN["n"] * (2 * 0.150 + 2 * FIN["t"]) * T_SHELL
    fin_i = FIN["n"] * 2 * T_SHELL * 0.150 ** 3 / 12
    h_area = 2 * (0.12 + HOUSING["t"]) * 0.0025
    h_i = (HOUSING["t"] * 0.12 ** 3 - (HOUSING["t"] - 0.005) * 0.115 ** 3) / 12
    ch = M["_parts"]["a_channel"]
    bones = [
        {"id": "ground", "parent": None, "title": T("Floor reference", "Bodenbezug"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "blanket", "parent": "ground", "title": T("Folded furniture pad (storage aid, stays behind)", "Gefaltetes Möbelpolster (Lagerhilfe, bleibt liegen)"), **place("blanket"), "lengthM": 2 * PAD["x"], "joint": {"type": "fixed"}, "mass": mass("blanket", "moving-blanket-catalogue")},
        {"id": "carry-lift", "parent": "ground", "title": T("Carrier's hands: lift", "Hände des Trägers: Heben"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 0.30}},
        {"id": "carry-travel", "parent": "carry-lift", "title": T("Carrier's hands: walking away", "Hände des Trägers: Weggehen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.0}},
        {"id": "fins", "parent": "carry-travel", "title": T("Oil-filled fin block with hubs and heating element", "Ölgefüllter Rippenblock mit Naben und Heizstab"), **place("fins"), "lengthM": FIN["y1"] - FIN["y0"], "joint": {"type": "fixed"}, "mass": mass("fins", "oil-radiator-catalogue"), "section": section(fin_area, fin_i, 0.075, "enamelled-steel", "Nine fins, two 0.7 mm shells 150 mm deep", "Neun Rippen, zwei Schalen 0,7 mm, 150 mm tief")},
        {"id": "housing", "parent": "fins", "title": T("Control housing with switches, thermostat and wound cord", "Bediengehäuse mit Schaltern, Thermostat und aufgewickeltem Kabel"), **place("housing"), "lengthM": HOUSING["y1"] - HOUSING["y0"], "joint": {"type": "fixed"}, "mass": mass("housing", "oil-radiator-catalogue"), "section": section(h_area, h_i, 0.06, "housing-plastic", "Housing shell 120 x 65 x 2.5 mm", "Gehäuseschale 120 x 65 x 2,5 mm")},
        {"id": "knob", "parent": "housing", "title": T("Replacement thermostat knob", "Ersatz-Thermostatknopf"), **place("knob"), "lengthM": KNOB["r"], "joint": {"type": "revolute", "minDeg": -270, "maxDeg": 0}, "mass": mass("knob", "oil-radiator-catalogue")},
        {"id": "feet", "parent": "fins", "title": T("Two pressed steel foot bars", "Zwei Fußbügel aus gepresstem Stahl"), **place("feet"), "lengthM": 2 * FOOT["x"], "joint": {"type": "fixed"}, "mass": mass("feet", "oil-radiator-catalogue"), "section": section(ch, 3.34e-9, 0.0138, "painted-steel", "U channel 30 x 20 x 1.2 mm", "U-Profil 30 x 20 x 1,2 mm")},
        {"id": "castors", "parent": "feet", "title": T("Four swivel castors", "Vier Lenkrollen"), **place("castors"), "lengthM": 2 * CASTOR["x"], "joint": {"type": "fixed"}, "mass": mass("castors", "castor-catalogue")},
    ]
    return {"kind": "mechanical", "root": "ground", "bones": bones}


ENV = {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.4, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325, "skyTempC": 20, "groundTempC": 20}


def areas():
    """📐 Outer areas of the regions from the 1:50 model (hand calculation)."""
    a_fin = polygon_area(fin_outline(8))
    p_fin = polygon_perimeter(fin_outline(8))
    gap = FIN["pitch"] - FIN["t"]
    fins = FIN["n"] * (2 * a_fin + p_fin * FIN["t"]) + 2 * (FIN["n"] - 1) * (math.pi * 2 * HUB["r"] * gap - 2 * math.pi * HUB["r"] ** 2)
    a_h = polygon_area(housing_outline(8))
    p_h = polygon_perimeter(housing_outline(8))
    hook = 2 * (2 * (2 * HOOKS["x"] + 0.024) * 0.022 + 2 * HOOKS["x"] * 0.024 - 2 * HOOKS["x"] * 0.024) + 2 * (2 * HOOKS["x"] * 0.024)
    switch = 2 * (2 * SWITCH["x"] + (SWITCH["y1"] - SWITCH["y0"])) * SWITCH["proud"]
    housing = 2 * a_h + p_h * HOUSING["t"] + hook + switch
    knob = 2 * math.pi * KNOB["r"] ** 2 + 2 * math.pi * KNOB["r"] * KNOB["proud"]
    o = racetrack(CORD["c_low"], CORD["c_high"], CORD["r_out"], 32)
    i = racetrack(CORD["c_low"], CORD["c_high"], CORD["r_in"], 32)
    cord = 2 * (polygon_area(o) - polygon_area(i)) + (polygon_perimeter(o) + polygon_perimeter(i)) * (CORD["z_in"] - CORD["z_out"])
    bar = 2 * (2 * FOOT["x"] * (FOOT["y1"] - FOOT["y0"]) + 2 * FOOT["x"] * FOOT["w"] + (FOOT["y1"] - FOOT["y0"]) * FOOT["w"])
    feet = 2 * bar
    c = CASTOR
    wheel = 2 * math.pi * c["r"] * c["w"] + 2 * math.pi * c["r"] ** 2
    fork = 2 * (2 * c["fork_x"] * (c["fork_y1"] - c["fork_y0"]) + 2 * c["fork_x"] * c["fork_w"] + (c["fork_y1"] - c["fork_y0"]) * c["fork_w"])
    castors = 4 * (wheel + fork - 2 * (2 * c["fork_x"]) * c["w"] * 0.5)
    pad_pts = pad_section(True)
    pad = 2 * polygon_area(pad_pts) + polygon_perimeter(pad_pts) * 2 * PAD["z"] - 0 * 1.0
    return {"fins": fins, "housing": housing, "knob": knob, "cord": cord, "feet": feet, "castors": castors, "pad": pad}


def layer(lid, title, kind, material, t, lam, rho, cp, src, ref):
    """🧶 One layer with resistance and the areal heat capacity of one hour's penetration depth."""
    depth = math.sqrt(lam / (rho * cp) * 3600)
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": t / lam, "heatCapacityJM2K": rho * cp * min(t, depth), "source": src, "reference": ref}


def region(rid, title, kind, model, material, area, L, geom, svf, lay, cap=None, extra=None, ref="iso-10456-2007"):
    """🔥 A surface region with one layer."""
    r = {"id": rid, "title": title, "kind": kind, "model": model, "material": material, "areaM2": area, "characteristicLengthM": L, "geometry": geom, "skyViewFactor": svf, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"] if cap is None else cap, "source": "authored-estimate", "reference": ref}
    if extra:
        r.update(extra)
    return r


def thermal_model():
    """🌡 Seven regions: the oil-filled fins and the housing heated by the 2 kW element, the other parts passive."""
    A = areas()
    steel = layer("shell", T("Pressed steel shell 0.7 mm with enamel", "Gepresste Stahlschale 0,7 mm mit Lack"), "metal", "enamelled-steel", T_SHELL, 50.0, RHO["enamelled-steel"], 460, "literature", "en-10130-2006")
    oil_cap = (OIL_KG * 1900 + ELEMENT_KG * 460) / A["fins"]
    regions = [
        region("fins", T("Oil-filled fins and hubs, yellowed enamel with rust spots at the lower rims", "Ölgefüllte Rippen und Naben, vergilbter Lack mit Rostpunkten an den unteren Rändern"), "heater", "operating", "enamelled-steel", A["fins"], 0.568, "vertical-plate", 0.5, steel, cap=steel["heatCapacityJM2K"] + oil_cap, extra={"operating": {"key": "power", "heatW": OPERATING["fins"]}, "ratedPowerW": 2000, "ratedPowerBasis": "electrical-input"}, ref="iec-60335-2-30"),
        region("housing", T("Control housing, yellowed white plastic with test sticker and price label", "Bediengehäuse, vergilbter weißer Kunststoff mit Prüfplakette und Preisschild"), "surface", "operating", "housing-plastic", A["housing"], 0.525, "vertical-plate", 0.5, layer("housing-wall", T("Polypropylene wall 2.5 mm", "Polypropylenwand 2,5 mm"), "polymer", "housing-plastic", 0.0025, 0.2, RHO["housing-plastic"], 1700, "literature", "iso-10456-2007"), extra={"operating": {"key": "power", "heatW": OPERATING["housing"]}}, ref="iec-60335-2-30"),
        region("knob", T("Replacement thermostat knob, dark grey", "Ersatz-Thermostatknopf, dunkelgrau"), "surface", "passive", "knob-plastic", A["knob"], 0.044, "vertical-plate", 0.5, layer("knob-wall", T("ABS 3 mm", "ABS 3 mm"), "polymer", "knob-plastic", 0.003, 0.17, RHO["knob-plastic"], 1400, "literature", "iso-10456-2007")),
        region("cord", T("Wound supply cord with a repair of insulating tape", "Aufgewickeltes Netzkabel mit einer Reparatur aus Isolierband"), "surface", "passive", "pvc-cord", A["cord"], 0.064, "vertical-plate", 0.5, layer("cord-sheath", T("PVC sheath and insulation, 3 mm to the conductors", "PVC-Mantel und Isolierung, 3 mm bis zu den Leitern"), "polymer", "pvc-cord", 0.003, 0.16, RHO["pvc-cord"], 1000, "literature", "din-en-50525-2-11")),
        region("feet", T("Grey painted foot bars, scuffed", "Grau lackierte Fußbügel, abgestoßen"), "surface", "passive", "painted-steel", A["feet"], 0.03, "horizontal-plate-up", 0.4, layer("foot-steel", T("Steel channel 1.2 mm", "Stahlprofil 1,2 mm"), "metal", "painted-steel", 0.0012, 50.0, RHO["painted-steel"], 460, "literature", "iso-10456-2007")),
        region("castors", T("Swivel castors, one black replacement", "Lenkrollen, eine schwarzer Ersatz"), "surface", "passive", "castor-polyamide", A["castors"], 0.04, "cylinder", 0.2, layer("castor-wall", T("Polyamide 5 mm", "Polyamid 5 mm"), "polymer", "castor-polyamide", 0.005, 0.25, RHO["castor-polyamide"], 1600, "authored-estimate", "castor-catalogue")),
        region("pad", T("Folded furniture pad", "Gefaltetes Möbelpolster"), "surface", "passive", "felt-pad", A["pad"], 0.45, "horizontal-plate-up", 0.5, layer("felt", T("Four layers of felt, 12 mm", "Vier Lagen Filz, 12 mm"), "polymer", "felt-pad", 0.012, 0.045, RHO["felt-pad"], 1300, "authored-estimate", "iso-10456-2007")),
    ]
    emitters = [{"id": "plume", "kind": "plume", "title": T("Free convection plume over the warm fins", "Freie Konvektionsfahne über den warmen Rippen"), "anchor": "plume-top", "directionDeg": 90, "diameterM": 0.26, "region": "fins", "source": "authored-estimate", "reference": "incropera-2018"}]
    operating = [{"id": "power", "title": T("Heating stages under the thermostat", "Heizstufen unter dem Thermostat"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.4, "title": T("Stage I (800 W)", "Stufe I (800 W)")}, {"v": 0.6, "title": T("Stage II (1200 W)", "Stufe II (1200 W)")}, {"v": 1, "title": T("Stages I and II (2000 W)", "Stufen I und II (2000 W)")}]}]
    return {"environment": ENV, "operating": operating, "regions": regions, "emitters": emitters, "passengers": [], "assumptions": [
        T("The 2000 W tubular element lies in the lower hub inside the oil and is not a region: at full power 1940 W leave through the fins and hubs and 60 W through the control housing, which the end fin and the thermostat warm; the asset's nameplate of 2000 W (electrical input) is stated once on the fin region. The three stages are 800, 1200 and 2000 W; the thermostat cycling is not modelled.",
          "Der 2000-W-Rohrheizkörper liegt in der unteren Nabe im Öl und ist keine Region: bei voller Leistung verlassen 1940 W das Gerät über Rippen und Naben und 60 W über das Bediengehäuse, das die Endrippe und der Thermostat erwärmen; die Nennleistung von 2000 W (elektrische Aufnahme) steht einmal an der Rippenregion. Die drei Stufen sind 800, 1200 und 2000 W; das Takten des Thermostats ist nicht modelliert."),
        T("Heat capacity of the fins: the 0.7 mm steel shells (about 2.5 kJ/m2K) plus 1.6 kg of mineral heat-transfer oil at 1.9 kJ/kgK and the 0.35 kg element spread over the outer fin area, about 4.3 kJ/m2K in all, so the fins take about a quarter of an hour to warm up (VDI-Wärmeatlas, authored-estimate).",
          "Wärmekapazität der Rippen: die 0,7-mm-Stahlschalen (etwa 2,5 kJ/m2K) und 1,6 kg mineralisches Wärmeträgeröl mit 1,9 kJ/kgK und der 0,35-kg-Heizstab auf die äußere Rippenfläche verteilt, zusammen etwa 4,3 kJ/m2K, sodass die Rippen etwa eine Viertelstunde zum Aufwärmen brauchen (VDI-Wärmeatlas, Schätzwert)."),
        T("Aged surfaces: the cream-white stove enamel has yellowed over years of heating, its solar absorptance is taken as 0.36 instead of about 0.25 for new white enamel, the emissivity of paint (0.92) is unchanged; the polypropylene housing yellowed likewise (0.40 instead of about 0.30); the rust spots at the lower fin rims are too small to change the region values (Incropera, authored-estimate).",
          "Gealterte Oberflächen: die cremeweiße Einbrennlackierung ist über Jahre des Heizens vergilbt, ihr solarer Absorptionsgrad wird mit 0,36 statt etwa 0,25 für neuen weißen Lack angesetzt, der Emissionsgrad des Lacks (0,92) bleibt; das Polypropylen-Gehäuse ist ebenso vergilbt (0,40 statt etwa 0,30); die Rostpunkte an den unteren Rippenrändern sind zu klein, um die Regionswerte zu ändern (Incropera, Schätzwert)."),
        T("Reference case: indoors at 20 degC with still air (0.05 m/s) and no sun, in the depot as in the living room. The fins face each other across 20 mm gaps, so a good part of their radiation stays in the stack; the warm air rises between the fins and leaves the top as the plume.",
          "Referenzfall: innen bei 20 degC mit ruhiger Luft (0,05 m/s) und ohne Sonne, im Lager wie im Wohnzimmer. Die Rippen sehen einander über 20-mm-Spalte, ein guter Teil ihrer Strahlung bleibt im Paket; die warme Luft steigt zwischen den Rippen auf und verlässt den Kopf als Konvektionsfahne."),
    ]}


def binding(kind):
    """🌬 The region binding of one pose family."""
    base = {"fins": {"exposedFraction": 1, "sunlitFraction": 0}, "housing": {"exposedFraction": 1, "sunlitFraction": 0}, "knob": {"exposedFraction": 1, "sunlitFraction": 0}, "cord": {"exposedFraction": 1, "sunlitFraction": 0}, "feet": {"exposedFraction": 1, "sunlitFraction": 0}}
    if kind == "stored":
        base.update({"castors": {"exposedFraction": 0.95, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 20}}, "pad": {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 20}}})
    elif kind == "carried":
        base.update({"fins": {"exposedFraction": 0.99, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 32}}, "housing": {"exposedFraction": 0.96, "sunlitFraction": 0, "contact": {"fraction": 0.04, "tempC": 32}}, "castors": {"exposedFraction": 1, "sunlitFraction": 0}, "pad": {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 20}}})
    else:
        base.update({"castors": {"exposedFraction": 0.95, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 20}}, "pad": {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 20}}})
    return base


PAD_C = list(PAD_CORNERS)
CASTOR_C = list(CASTOR_IDS)
CARRIED = {"carry-lift": {"dy": CARRY_DY}, "carry-travel": {"dx": CARRY_DX}}
IN_USE = {"carry-lift": {"dy": USE_DY}, "carry-travel": {"dx": CARRY_DX}}


def pose(pid, title, desc, bones, contacts, kind, level, plume, key=False):
    """🧍 One pose."""
    p = {"id": pid, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "auto", "contacts": contacts, "thermal": {"operating": {"power": level}, "regions": binding(kind), "emitters": {"plume": plume}}, "loads": []})
    return p


def poses():
    """🧍 Stored, carried, in use at full and at low power, and the key poses of the clips."""
    return [
        pose("stored", T("Stored on a folded pad", "Auf einem gefalteten Polster gelagert"), T("In the electrical department of the reuse depot the radiator stands cold on its four castors on a folded furniture pad, its cord wound on the hooks and taped, a test sticker and a price label on the control housing.", "In der Elektroabteilung des Gebrauchtwarenkaufhauses steht der Radiator kalt auf seinen vier Rollen auf einem gefalteten Möbelpolster, das Kabel auf die Haken gewickelt und abgeklebt, Prüfplakette und Preisschild am Bediengehäuse."), {}, PAD_C, "stored", 0, 0),
        pose("carried", T("Carried by one person", "Von einer Person getragen"), T("Lifted 0.15 m off the pad and carried 0.70 m away by one person who holds it in front of the body by the top lip of the control housing and the top of the far end fin; it hangs plumb from the two hands, the pad stays behind.", "0,15 m vom Polster gehoben und von einer Person 0,70 m weggetragen, die ihn vor dem Körper an der oberen Kante des Bediengehäuses und am Kopf der äußeren Endrippe hält; er hängt lotrecht an beiden Händen, das Polster bleibt liegen."), CARRIED, PAD_C + GRIPS, "carried", 0, 0),
        pose("in-use", T("Heating at full power in a living room", "Heizt mit voller Leistung im Wohnzimmer"), T("Standing on its castors on the living room floor, both stages on (2000 W), the thermostat turned almost to the end; the fins are hot and the warm air rises over the stack.", "Auf seinen Rollen auf dem Wohnzimmerboden, beide Stufen eingeschaltet (2000 W), der Thermostat fast bis zum Anschlag gedreht; die Rippen sind heiß, die warme Luft steigt über dem Paket auf."), {**IN_USE, "knob": {"angleDeg": -240}}, PAD_C + CASTOR_C, "in-use", 1, 1),
        pose("low-setting", T("Heating on stage I", "Heizt auf Stufe I"), T("In use with only the first stage on (800 W) and the thermostat at the middle, the background heating of a bedroom or a cold evening.", "In Gebrauch mit nur der ersten Stufe (800 W) und dem Thermostat in der Mitte, die Grundheizung eines Schlafzimmers oder eines kühlen Abends."), {**IN_USE, "knob": {"angleDeg": -135}}, PAD_C + CASTOR_C, "in-use", 0.4, 0.4),
        pose("lifted-key", T("Lifted off the pad (key)", "Vom Polster gehoben (Schlüssel)"), T("Held 0.15 m above the pad by the two hands, castors free.", "Mit beiden Händen 0,15 m über dem Polster gehalten, Rollen frei."), {"carry-lift": {"dy": CARRY_DY}}, PAD_C + GRIPS, "carried", 0, 0, key=True),
        pose("off-key", T("Set up, still cold (key)", "Aufgestellt, noch kalt (Schlüssel)"), T("Standing on its castors in the living room, plugged in, switches off and the thermostat at zero.", "Auf seinen Rollen im Wohnzimmer, eingesteckt, Schalter aus und Thermostat auf null."), IN_USE, PAD_C + CASTOR_C, "in-use", 0, 0, key=True),
    ]


def spans(anchors_, a, b):
    """🦶 Contact spans."""
    return [{"anchor": x, "fromS": a, "toS": b} for x in anchors_]


def clips():
    """🎞 Carry out of the depot and heating up from cold."""
    return [
        {"id": "carry", "title": T("Carry out of the depot", "Aus dem Lager tragen"), "description": T("One person takes the radiator by the top of the control housing and of the far end fin, lifts it off the pad and carries it away hanging plumb in front of the body.", "Eine Person fasst den Radiator am Kopf des Bediengehäuses und der äußeren Endrippe, hebt ihn vom Polster und trägt ihn lotrecht hängend vor dem Körper davon."), "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.2, "pose": "lifted-key", "ease": "smooth"}, {"t": 4.0, "pose": "carried", "ease": "smooth"}, {"t": 5, "pose": "carried", "ease": "smooth"}],
         "contacts": spans(PAD_C, 0, 5) + spans(GRIPS, 0, 5),
         "phases": [{"id": "lift", "fromS": 0, "toS": 1.2, "title": T("Lift off", "Abheben")}, {"id": "walk", "fromS": 1.2, "toS": 4.0, "title": T("Walk away", "Weggehen")}, {"id": "hold", "fromS": 4.0, "toS": 5, "title": T("Hold", "Halten")}],
         "events": [{"t": 0.2, "kind": "motion", "title": T("Castors leave the pad", "Rollen lösen sich vom Polster")}],
         "tracks": [{"target": "region:castors:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.95}, {"t": 0.3, "v": 1}]}]},
        {"id": "heat-up", "title": T("Switching on and heating up", "Einschalten und Aufheizen"), "description": T("In the living room the thermostat is turned up and both stages switched on; over twenty minutes the oil and the fins warm up from room temperature and the plume of warm air over the stack grows.", "Im Wohnzimmer wird der Thermostat aufgedreht und beide Stufen eingeschaltet; über zwanzig Minuten erwärmen sich Öl und Rippen von Raumtemperatur, und die Fahne warmer Luft über dem Paket wächst."), "durationS": 1200, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "off-key", "ease": "smooth"}, {"t": 2, "pose": "off-key", "ease": "smooth"}, {"t": 5, "pose": "in-use", "ease": "step"}, {"t": 1200, "pose": "in-use", "ease": "step"}],
         "contacts": spans(PAD_C, 0, 1200) + spans(CASTOR_C, 0, 1200),
         "phases": [{"id": "switch-on", "fromS": 0, "toS": 5, "title": T("Turn up and switch on", "Aufdrehen und einschalten")}, {"id": "warm-up", "fromS": 5, "toS": 1200, "title": T("Warm up", "Aufheizen")}],
         "events": [{"t": 5, "kind": "operating", "title": T("Both stages on", "Beide Stufen ein"), "target": "power"}],
         "tracks": [{"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 1}]}, {"target": "emitter:plume", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 1}]}]},
    ]


def structure_model():
    """🏗 Supports and assumptions."""
    M = masses()
    rad = sum(M[b][0] for b in ("fins", "housing", "knob", "feet", "castors"))
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": PAD_C, "clinical": False, "assumptions": [
        T(f"Stored: one rigid stack. The radiator ({rad:.1f} kg) stands on its four castors on the folded pad, which lies on the depot floor; the pad corners are the permanent supports (friction 0.5).",
          f"Gelagert: ein starrer Stapel. Der Radiator ({rad:.1f} kg) steht auf seinen vier Rollen auf dem gefalteten Polster, das auf dem Lagerboden liegt; die Polsterecken sind die ständigen Auflager (Reibung 0,5).".replace(f"({rad:.1f} kg)", f"({rad:.1f} kg)".replace(".", ","))),
        T("Carried: one adult holds the radiator in front of the body with the right hand under the top lip of the control housing and the left hand round the top of the far end fin, 0.38 m apart; its centre of mass lies in the plane of the two hands, so it hangs plumb and each hand carries about half (about 6 kg, within the 15 kg one person carries with both hands close to the body per DGUV 208-033). The hands are compression contacts pushing up.",
          "Getragen: ein Erwachsener hält den Radiator vor dem Körper mit der rechten Hand unter der oberen Kante des Bediengehäuses und der linken Hand um den Kopf der äußeren Endrippe, 0,38 m auseinander; sein Schwerpunkt liegt in der Ebene der beiden Hände, er hängt also lotrecht und jede Hand trägt etwa die Hälfte (etwa 6 kg, innerhalb der 15 kg, die eine Person nach DGUV 208-033 beidhändig körpernah trägt). Die Hände sind nach oben drückende Kontakte."),
        T("In use: the four castors stand on the floor under the two foot bars (wheel base 0.20 x 0.25 m around the centre of mass, which lies 27 mm towards the fin side of the stack centre because of the housing); self weight only, nothing hangs on the fins (IEC 60335-2-30 marks the radiator 'do not cover').",
          "In Gebrauch: die vier Rollen stehen unter den beiden Fußbügeln auf dem Boden (Radstand 0,20 x 0,25 m um den Schwerpunkt, der wegen des Gehäuses 27 mm zur Rippenseite der Paketmitte liegt); nur Eigengewicht, nichts hängt an den Rippen (IEC 60335-2-30 kennzeichnet den Radiator mit 'nicht abdecken')."),
        T("Limitation of the solver: the whole asset is one instantaneously rigid body on springs of equal stiffness, so while carried or in use the pad, which then lies apart, shares the reactions linearly with the hands or castors instead of carrying only its own 0.4 kg; the stored pose, a resting stack, is exact (requests.md, used-coffee-table).",
          "Grenze des Lösers: das ganze Objekt ist ein momentan starrer Körper auf gleich steifen Federn, getragen oder in Gebrauch teilt das dann getrennt liegende Polster die Reaktionen deshalb linear mit den Händen oder Rollen, statt nur seine eigenen 0,4 kg zu tragen; die Lagerpose, ein ruhender Stapel, ist exakt (requests.md, used-coffee-table)."),
        T("Members: the fin block of nine fins of two 0.7 mm DC01 shells (EN 10130, E 210 GPa) welded at the rims and joined by two hubs, the polypropylene housing screwed to the first fin, two pressed U channels 30 x 20 x 1.2 mm clamped under fins 2 and 8 with U-bolts and wing nuts, four push-in castors; masses from the modelled sections x density and the oil fill sum to dimensions.massKg.",
          "Stäbe: der Rippenblock aus neun Rippen aus je zwei 0,7-mm-Schalen DC01 (EN 10130, E 210 GPa), an den Rändern verschweißt und über zwei Naben verbunden, das an die erste Rippe geschraubte Polypropylen-Gehäuse, zwei gepresste U-Profile 30 x 20 x 1,2 mm, mit Bügelschrauben und Flügelmuttern unter Rippe 2 und 8 geklemmt, vier eingesteckte Rollen; die Massen aus modellierten Querschnitten x Dichte und der Ölfüllung ergeben dimensions.massKg."),
    ]}


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350)}
HATCHES = [{"id": "quilt", "title": T("Quilting of the furniture pad", "Steppung des Möbelpolsters"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.2, "penMm": 0.13}]


def decisions(scale):
    """📝 The drawing decisions of one scale."""
    D = {
        50: dict(
            silhouette=T("Radiator 13 x 9 mm on paper on its 0.24 mm pad. Side elevation from the right end: the control housing with the replacement knob, the switch panel and the cord wound on two hooks stands in front of the rounded end fin, which shows its rim seam and the rust spots at the lower edge; below it the foot bar with two castors, the front one a black replacement. Front elevation: nine fins with their weld seams and the two hubs in the gaps, the housing at the right end with knob, switch panel, hooks and cord bundle, the two foot bars end-on with their castors. Plan: the pad, the foot bars showing beyond the fins, nine fins over the upper hub, the housing with the cord bundle and the knob.",
                       "Radiator 13 x 9 mm auf dem Papier auf seinem 0,24-mm-Polster. Seitenansicht vom rechten Ende: das Bediengehäuse mit dem Ersatzknopf, dem Schalterfeld und dem auf zwei Haken gewickelten Kabel steht vor der gerundeten Endrippe, die ihre Randnaht und die Rostpunkte an der Unterkante zeigt; darunter der Fußbügel mit zwei Rollen, die vordere ein schwarzer Ersatz. Vorderansicht: neun Rippen mit ihren Schweißnähten und die zwei Naben in den Zwischenräumen, das Gehäuse am rechten Ende mit Knopf, Schalterfeld, Haken und Kabelbündel, die beiden Fußbügel stirnseitig mit ihren Rollen. Draufsicht: das Polster, die über die Rippen hinausragenden Fußbügel, neun Rippen über der oberen Nabe, das Gehäuse mit Kabelbündel und Knopf."),
            retained=T("Every fin and gap, the fin rim seams, the hubs, the rust spots, scratches and dust lines, the pressed foot bars, forks, wheels and axles, the mismatched castor, the switch rockers, the knob pointer and scale, the test sticker and price label, the turns of the cord and its tape repair, the folded layers and the quilting of the pad.",
                     "Jede Rippe und jeder Zwischenraum, die Randnähte, die Naben, Rostpunkte, Kratzer und Staubstreifen, die gepressten Fußbügel, Gabeln, Räder und Achsen, die nicht passende Rolle, die Wippen der Schalter, Zeiger und Skala des Knopfs, Prüfplakette und Preisschild, die Windungen des Kabels und seine Klebebandreparatur, die gefalteten Lagen und die Steppung des Polsters."),
            merged=T("All fins coincide in the side elevation, the far foot bar and its castors lie behind the near one; the lower hub lies behind the fins in plan.",
                     "Alle Rippen decken sich in der Seitenansicht, der hintere Fußbügel und seine Rollen liegen hinter dem vorderen; die untere Nabe liegt in der Draufsicht unter den Rippen."),
            omitted=T("The 3 mm weld flange as a body (it is drawn as the rim line), the U-bolts and wing nuts of the feet, the neon pilot lamp, the screws of the housing, the plug tucked into the cord bundle, all under the 7.5 mm feature or hidden.",
                      "Der 3-mm-Schweißflansch als Körper (er ist als Randlinie gezeichnet), Bügelschrauben und Flügelmuttern der Füße, die Glimmlampe, die Schrauben des Gehäuses, der in das Kabelbündel gesteckte Stecker, alle unter dem 7,5-mm-Merkmal oder verdeckt."),
            thinElements=T("Fins of 22 mm print 0.44 mm and their 20 mm gaps join with the 0.5 mm body pen into ribs from 1:40 on; the 12 mm pad is a filled strip of 0.24 mm.",
                           "Rippen von 22 mm drucken 0,44 mm, ihre 20-mm-Zwischenräume schließen sich ab 1:40 mit dem 0,5-mm-Körperstrich zu Rippen; das 12-mm-Polster ist ein gefüllter Streifen von 0,24 mm."),
            hatching=T("Cross quilting on the pad in plan.", "Kreuzsteppung auf dem Polster in der Draufsicht."),
            styles=T("Housing 0.88, fins 0.82, switch panel 0.65, hooks 0.70, hubs 0.60, castors 0.55 (replacement 0.15), foot bars 0.45, knob 0.30, pad 0.30, cord 0.25.",
                     "Gehäuse 0,88, Rippen 0,82, Schalterfeld 0,65, Haken 0,70, Naben 0,60, Rollen 0,55 (Ersatz 0,15), Fußbügel 0,45, Knopf 0,30, Polster 0,30, Kabel 0,25."),
            overlays=T("Every part carries its thermal region: fins (with hubs), housing (with switch panel and hooks), knob, cord, feet, castors and pad.",
                       "Jedes Teil trägt seine Thermikregion: Rippen (mit Naben), Gehäuse (mit Schalterfeld und Haken), Knopf, Kabel, Füße, Rollen und Polster.")),
        100: dict(
            silhouette=T("Radiator 6.5 x 4.5 mm: the same parts without texture: rounded end fin with its rim line, housing with knob, switch outline and the cord ring, foot bar and castors; nine fins and hubs in front and plan.",
                         "Radiator 6,5 x 4,5 mm: dieselben Teile ohne Textur: gerundete Endrippe mit Randlinie, Gehäuse mit Knopf, Schalterumriss und Kabelring, Fußbügel und Rollen; neun Rippen und Naben in Vorderansicht und Draufsicht."),
            retained=T("Nine fins and their gaps, hubs, rim line, castors with forks and wheels, the knob pointer, the cord ring.", "Neun Rippen und ihre Zwischenräume, Naben, Randlinie, Rollen mit Gabeln und Rädern, der Knopfzeiger, der Kabelring."),
            merged=T("The switch panel into the housing face, the hooks into the cord ring lying flush on the end face.", "Das Schalterfeld in die Gehäusestirn, die Haken in den bündig auf der Stirnseite liegenden Kabelring."),
            omitted=T("Seams of the fins in front and plan, rust, scratches, dust, stickers, knob scale, cord turns and tape, pad layers and quilting.", "Rippennähte in Vorderansicht und Draufsicht, Rost, Kratzer, Staub, Aufkleber, Knopfskala, Kabelwindungen und Klebeband, Polsterlagen und Steppung."),
            thinElements=T("The 22 mm fins print 0.22 mm and join over their gaps with the 0.35 mm pen; the 12 mm pad is a 0.12 mm strip carried by its outline pen and a declared sheet in 3D.", "Die 22-mm-Rippen drucken 0,22 mm und schließen sich mit dem 0,35-mm-Strich über ihre Zwischenräume; das 12-mm-Polster ist ein Streifen von 0,12 mm, den sein Umrissstrich trägt, in 3D ein deklariertes Blatt."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All seven regions.", "Alle sieben Regionen.")),
        200: dict(
            silhouette=T("Radiator 3.3 x 2.2 mm: the rounded end fin with housing, knob disc and cord plate, foot bar and castor blocks in elevation; seven typified fins with the hubs, the housing, cord and knob in front and plan; the pad strip.", "Radiator 3,3 x 2,2 mm: die gerundete Endrippe mit Gehäuse, Knopfscheibe und Kabelplatte, Fußbügel und Rollenblöcke in der Seitenansicht; sieben typisierte Rippen mit den Naben, das Gehäuse, Kabel und Knopf in Vorderansicht und Draufsicht; der Polsterstreifen."),
            retained=T("The fin row as a row, the hubs, the housing at the right end, the castor feet, the pad.", "Die Rippenreihe als Reihe, die Naben, das Gehäuse am rechten Ende, die Rollenfüße, das Polster."),
            merged=T("Nine 22 mm fins typified as seven 30 mm fins over the true stack length; fork and wheel of each castor into one 30 mm block; cord, hooks and winder into one plate.", "Neun 22-mm-Rippen typisiert als sieben 30-mm-Rippen über die wahre Paketlänge; Gabel und Rad jeder Rolle zu einem 30-mm-Block; Kabel, Haken und Wickel zu einer Platte."),
            omitted=T("Rim lines, switch panel, knob pointer, every detail under 30 mm.", "Randlinien, Schalterfeld, Knopfzeiger, jedes Detail unter 30 mm."),
            thinElements=T("The typified fins print 0.15 mm and join over their 25 mm gaps with the 0.25 mm pen into one ribbed block at 1:150 to 1:350; the pad stays a 12 mm strip carried by the body pen (0.28 mm at 1:350).", "Die typisierten Rippen drucken 0,15 mm und schließen sich mit dem 0,25-mm-Strich von 1:150 bis 1:350 über ihre 25-mm-Zwischenräume zu einem gerippten Block; das Polster bleibt ein 12-mm-Streifen, den der Körperstrich trägt (0,28 mm bei 1:350)."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All seven regions.", "Alle sieben Regionen.")),
    }
    return D[scale]


def representations():
    """🖨 The three authored drawings and the unavailable 1:500."""
    out = {}
    for scale in (50, 100, 200):
        lo, hi = BANDS[scale]
        out[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES if scale == 50 else [], "geometry": geometry(scale), "decisions": decisions(scale)}
    out["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T(
        "At 1:500 the radiator is 1.3 by 0.9 mm on paper, under the 3 mm a recognisable drawing needs; its 22 mm fins would be 0.04 mm and their 20 mm gaps closed by any pen, a fifth of the 0.2 mm office-laser minimum feature; draw it at 1:200.",
        "Bei 1:500 misst der Radiator 1,3 mal 0,9 mm auf dem Papier, unter den 3 mm, die eine erkennbare Zeichnung braucht; seine 22-mm-Rippen wären 0,04 mm und ihre 20-mm-Zwischenräume von jedem Strich geschlossen, ein Fünftel des Mindestmerkmals von 0,2 mm des Büro-Laserdrucks; bei 1:200 zeichnen."), "limitingFeature": T("Fins of 22 mm pressed steel", "Rippen aus 22 mm gepresstem Stahl"), "limitingSizeM": FIN["t"], "alternatives": [{"kind": "scale", "denominator": 200}]}
    return out


def document():
    """📄 The whole asset document."""
    M = masses()
    total = sum(v[0] for k, v in M.items() if k != "_parts")
    asset = {
        "id": "used-oil-filled-radiator",
        "category": "appliances",
        "title": T("Used Oil-Filled Radiator", "Gebrauchter Ölradiator"),
        "summary": T("A second-hand 2 kW oil-filled electric radiator with nine pressed-steel fins on castor feet, 0.64 x 0.45 x 0.25 m and RADKG kg, with yellowed enamel, rust spots at the lower fin rims, a replacement thermostat knob, one mismatched castor and a taped cord on its winder, stored on a folded furniture pad in a reuse depot, carried by one person at the top of the housing and the far end fin and heating a living room at full power.",
                   "Ein gebrauchter elektrischer Ölradiator mit 2 kW und neun gepressten Stahlrippen auf Rollenfüßen, 0,64 x 0,45 x 0,25 m und RADKGDE kg, mit vergilbtem Lack, Rostpunkten an den unteren Rippenrändern, einem Ersatz-Thermostatknopf, einer nicht passenden Rolle und einem abgeklebten Kabel auf seinem Wickel, im Gebrauchtwarenkaufhaus auf einem gefalteten Möbelpolster gelagert, von einer Person am Kopf des Gehäuses und der äußeren Endrippe getragen und mit voller Leistung im Wohnzimmer heizend."),
        "tags": ["used", "household", "second-hand", "living", "bedroom", "heater", "oil-filled-radiator", "electric-heating", "steel", "enamel", "castors", "reuse-depot"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS], "notes": T(
            "Oil-filled radiator of the common type sold in German DIY and department stores from the 1990s to the 2010s: nine fins of two pressed 0.7 mm DC01 shells, 150 x 568 x 22 mm at a pitch of 42 mm, joined by an upper and a lower hub, about 1.6 kg of mineral oil with the 2000 W element in the lower hub (800 / 1200 / 2000 W), a polypropylene control housing 120 x 525 x 65 mm on the right end with two rocker switches, a mechanical thermostat and a cord winder, two pressed steel foot bars with four swivel castors; 0.64 m high, 0.445 m long, 0.25 m deep over the feet, RADKG kg. Traces of use: enamel yellowed by years of heating, rust spots at the lower fin rims, scratches and dust lines, a dark grey replacement thermostat knob, the front right castor a black replacement, the cord repaired with insulating tape, a test sticker of the DIN VDE 0701-0702 check and a price label of the depot. Storage aid: a furniture pad folded to 0.45 x 0.60 m and 12 mm (0.4 kg). Every parameter is literature or authored-estimate; nothing is measured.",
            "Ölradiator der gängigen Bauart, wie ihn deutsche Bau- und Kaufhäuser von den 1990er- bis in die 2010er-Jahre verkauften: neun Rippen aus je zwei gepressten 0,7-mm-Schalen DC01, 150 x 568 x 22 mm im Abstand von 42 mm, über eine obere und eine untere Nabe verbunden, etwa 1,6 kg Mineralöl mit dem 2000-W-Heizstab in der unteren Nabe (800 / 1200 / 2000 W), ein Bediengehäuse aus Polypropylen 120 x 525 x 65 mm am rechten Ende mit zwei Wippschaltern, mechanischem Thermostat und Kabelwickel, zwei Fußbügel aus gepresstem Stahl mit vier Lenkrollen; 0,64 m hoch, 0,445 m lang, über den Füßen 0,25 m tief, RADKGDE kg. Gebrauchsspuren: durch Jahre des Heizens vergilbter Lack, Rostpunkte an den unteren Rippenrändern, Kratzer und Staubstreifen, ein dunkelgrauer Ersatz-Thermostatknopf, die vordere rechte Rolle ein schwarzer Ersatz, das Kabel mit Isolierband repariert, eine Prüfplakette der Prüfung nach DIN VDE 0701-0702 und ein Preisschild des Lagers. Lagerhilfe: ein auf 0,45 x 0,60 m und 12 mm gefaltetes Möbelpolster (0,4 kg). Jeder Parameter ist Literatur oder Schätzwert; nichts ist gemessen.")},
        "dimensions": {"heightM": yw(FIN["y1"]), "widthM": 2 * PAD["x"], "depthM": 2 * PAD["z"], "massKg": total},
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
    rad = f"{total - M['blanket'][0]:.1f}"
    text = json.dumps(r6(asset), ensure_ascii=False).replace("RADKGDE", rad.replace(".", ",")).replace("RADKG", rad)
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": json.loads(text)}


if __name__ == "__main__":
    OUT.write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    M = masses()
    print("[DEBUG] areas", {k: round(v, 4) for k, v in areas().items()})
    print("[DEBUG] masses", {k: round(v[0], 3) for k, v in M.items() if k != "_parts"}, "total", round(sum(v[0] for k, v in M.items() if k != "_parts"), 3))
    print("[DEBUG] z", round(ZMIN, 4), round(Z_FACE, 4), round(Z_FIN0, 4), round(Z_FIN1, 4), "feet", [round(z, 4) for z in FOOT_Z])
