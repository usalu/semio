"""🪵 Authoritative chisel of the used coffee table: states every hand-chosen number and writes `🔣️.json` next to this file.

The asset is stored as a second-hand depot stores it: upside down on a moving blanket on a steel-framed depot table (rest = default pose
`stored`), carried by one person at the two short ends and set up again in a living room. World frame of the staffage: x ahead (the
0.60 m depth of the coffee table), y up, z left (its 1.10 m length).

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


def r6(v):
    """🔢 Rounds every float of a nested value to six decimals."""
    if isinstance(v, float):
        return float(f"{v:.6g}") if 0 < abs(v) < 0.01 else round(v, 6) + 0.0
    if isinstance(v, list):
        return [r6(x) for x in v]
    if isinstance(v, tuple):
        return [r6(x) for x in v]
    if isinstance(v, dict):
        return {k: r6(x) for k, x in v.items()}
    return v


def T(en, de):
    """🗣 A bilingual text."""
    return {"en": en, "de": de}


# Depot table, blanket and coffee table in world rest coordinates (stored).
DEPOT = dict(x=0.40, z=0.60, top0=0.715, top1=0.740, leg=0.040, legx=0.34, legz=0.54, wall=0.002, rail_h=0.040, rail_w=0.020, plug=0.008)
BLANKET = dict(x=0.36, z=0.58, y0=0.740, y1=0.752)
TOP_Y = 0.752
PIVOT_Y = TOP_Y + 0.0125
COFFEE = dict(X=0.30, Z=0.55, H=0.45, top=0.025, leg=0.045, inset=0.030, apron_h=0.070, apron_t=0.020, apron_set=0.005, shelf0=0.111, shelf1=0.130, shelfX=0.26, shelfZ=0.51)
CHIP = [(0.55, 0.272), (0.538, 0.281), (0.529, 0.292), (0.518, 0.30)]
CARRY_DX = 1.10
CARRY_DY = 0.0855
USE_DY = -(PIVOT_Y - (COFFEE["H"] - COFFEE["top"] / 2))

RHO = {"veneered-chipboard": 660.0, "oak-solid": 700.0, "melamine-chipboard": 650.0, "powder-coated-steel": 7850.0, "felt-blanket": 134.0, "glide-plastic": 950.0}


def up(Xv, Yv, Zv):
    """🔄 Upright coffee-table coordinates to world rest coordinates (inverted on the blanket)."""
    return (-Xv, TOP_Y + COFFEE["H"] - Yv, Zv)


LEG_IN_X = COFFEE["X"] - COFFEE["inset"] - COFFEE["leg"]
LEG_OUT_X = COFFEE["X"] - COFFEE["inset"]
LEG_IN_Z = COFFEE["Z"] - COFFEE["inset"] - COFFEE["leg"]
LEG_OUT_Z = COFFEE["Z"] - COFFEE["inset"]
LEG_CX = (LEG_IN_X + LEG_OUT_X) / 2
LEG_CZ = (LEG_IN_Z + LEG_OUT_Z) / 2
APRON_OUT_X = LEG_OUT_X - COFFEE["apron_set"]
APRON_OUT_Z = LEG_OUT_Z - COFFEE["apron_set"]
APRON_Y0 = COFFEE["H"] - COFFEE["top"] - COFFEE["apron_h"]
APRON_Y1 = COFFEE["H"] - COFFEE["top"]

FRAMES = {
    "depot-table": lambda x, y: (x + 0.40, y - 0.7275),
    "blanket": lambda x, y: (x + 0.36, y - 0.746),
    "table": lambda x, y: (0.30 - x, PIVOT_Y - y),
    "frame": lambda x, y: (y - (TOP_Y + COFFEE["top"]), (0.30 - 0.0525) - x),
    "shelf": lambda x, y: (0.26 - x, (TOP_Y + COFFEE["H"] - (COFFEE["shelf0"] + COFFEE["shelf1"]) / 2) - y),
}
AXES = {"depot-table": ("x", "y"), "blanket": ("x", "y"), "table": ("x", "y"), "frame": ("y", "x"), "shelf": ("x", "y")}


def loc(bone, x, y):
    """📍 World rest (x, y) to the (u, v) frame of a bone."""
    return FRAMES[bone](x, y)


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


def rect_e(bone, x0, y0, x1, y1):
    """▭ A rect geometry in the elevation frame of a bone from two world corners."""
    a = loc(bone, x0, y0)
    b = loc(bone, x1, y1)
    return {"kind": "rect", "x": min(a[0], b[0]), "y": min(a[1], b[1]), "w": abs(a[0] - b[0]), "h": abs(a[1] - b[1])}


def poly_w(points, closed=True):
    """⬠ A poly geometry from world points (front, plan)."""
    return {"kind": "poly", "points": [list(p) for p in points], "closed": closed}


def poly_e(bone, points, closed=True):
    """⬠ A poly geometry in the elevation frame of a bone from world points."""
    return {"kind": "poly", "points": [list(loc(bone, *p)) for p in points], "closed": closed}


def path_lines(segments):
    """〰 A path of straight strokes."""
    return {"kind": "path", "d": " ".join(f"M {a[0]:.5f} {a[1]:.5f} L {b[0]:.5f} {b[1]:.5f}" for a, b in segments)}


def path_lines_e(bone, segments):
    """〰 Straight strokes in the elevation frame of a bone from world points."""
    return path_lines([(loc(bone, *a), loc(bone, *b)) for a, b in segments])


def path_circles(circles):
    """⭕ A path of circles (cup rings)."""
    parts = []
    for cx, cy, r in circles:
        parts.append(f"M {cx + r:.5f} {cy:.5f} A {r:.5f} {r:.5f} 0 1 0 {cx - r:.5f} {cy:.5f} A {r:.5f} {r:.5f} 0 1 0 {cx + r:.5f} {cy:.5f}")
    return {"kind": "path", "d": " ".join(parts)}


def blanket_section(fold=True, steps=6, y0=None):
    """🧺 The folded blanket in elevation: four layers with the rounded fold at the front edge (`y0` lowers the underside of a generalised strip)."""
    b = dict(BLANKET, y0=BLANKET["y0"] if y0 is None else y0)
    r = (b["y1"] - b["y0"]) / 2
    cy = (b["y0"] + b["y1"]) / 2
    if not fold:
        return [(-b["x"], b["y0"]), (b["x"], b["y0"]), (b["x"], b["y1"]), (-b["x"], b["y1"])]
    pts = [(-b["x"], b["y0"])]
    for k in range(steps + 1):
        a = -math.pi / 2 + math.pi * k / steps
        pts.append((b["x"] - r + r * math.cos(a), cy + r * math.sin(a)))
    pts.append((-b["x"], b["y1"]))
    return pts


def chip_outline_world():
    """🔨 Plan outline of the coffee top with the broken corner, world rest (x, z)."""
    X, Z = COFFEE["X"], COFFEE["Z"]
    pts = [(X, -Z), (X, Z)]
    for (zc, xc) in CHIP:
        pts.append((-xc, zc))
    pts.append((-X, -Z))
    return pts


def chip_outline_upright():
    """🔨 The same outline in upright coordinates (X, Z)."""
    return [(-x, z) for x, z in chip_outline_world()]


def wy(Yv):
    """↕ World rest height of an upright coffee-table height."""
    return TOP_Y + COFFEE["H"] - Yv


def wx(Xv):
    """↔ World rest x of an upright coffee-table depth position."""
    return -Xv


def u_frame(o0, i0, i1, o1, y_top, y_band, y_end):
    """⊓ The outline of two legs joined by a band (an inverted U in world rest coordinates): outer and inner positions of the legs."""
    return [(o0, y_top), (o1, y_top), (o1, y_end), (i1, y_end), (i1, y_band), (i0, y_band), (i0, y_end), (o0, y_end)]


def geometry(scale):
    """🖼 The individually drawn elevation, front and plan of one scale."""
    d, b, c = DEPOT, BLANKET, COFFEE
    s50, s100, s200, s500 = scale == 50, scale == 100, scale == 200, scale == 500
    E, F, P = [], [], []
    top_t = 0.85
    legw = 0.075 if s500 else d["leg"]
    lx0, lz0 = (d["x"] - 0.02 - legw, d["z"] - 0.02 - legw) if s500 else (d["legx"], d["legz"])
    lx1, lz1 = lx0 + legw, lz0 + legw
    rail0 = d["top0"] - d["rail_h"]
    cleg = 0.075 if s500 else c["leg"]
    cx0, cz0 = LEG_OUT_X - cleg, LEG_OUT_Z - cleg
    band0 = APRON_Y1 - 0.075 if s500 else APRON_Y0
    fold = scale in (50, 100)

    E.append(shape("depot-top", "depot-table", "body", 1, top_t, rect_e("depot-table", -d["x"], d["top0"], d["x"], d["top1"]), "depot-top", "melamine-chipboard"))
    if s50:
        for side, (a0, a1) in (("rear", (-lx1, -lx0)), ("front", (lx0, lx1))):
            E.append(shape(f"depot-leg-{side}", "depot-table", "body", 1.2, 0.35, rect_e("depot-table", a0, 0.0, a1, d["top0"]), "depot-frame", "powder-coated-steel"))
        E.append(shape("depot-rail", "depot-table", "body", 1.1, 0.4, rect_e("depot-table", -lx0, rail0, lx0, d["top0"]), "depot-frame", "powder-coated-steel"))
        E.append(shape("depot-glides", "depot-table", "detail", 1.3, 0.2, path_lines_e("depot-table", [((-lx1, d["plug"]), (-lx0, d["plug"])), ((lx0, d["plug"]), (lx1, d["plug"]))]), "depot-frame", filled=False))
        E.append(shape("depot-tube-joints", "depot-table", "detail", 1.3, 0.2, path_lines_e("depot-table", [((-lx0, rail0), (-lx0, d["top0"])), ((lx0, rail0), (lx0, d["top0"]))]), "depot-frame", filled=False))
    elif s500:
        for side, (a0, a1) in (("rear", (-lx1, -lx0)), ("front", (lx0, lx1))):
            E.append(shape(f"depot-leg-{side}", "depot-table", "body", 1.2, 0.35, rect_e("depot-table", a0, 0.0, a1, d["top0"]), "depot-frame", "powder-coated-steel"))
    else:
        E.append(shape("depot-frame", "depot-table", "body", 1.1, 0.35, poly_e("depot-table", [(x, y) for x, y in u_frame(-lx1, -lx0, lx0, lx1, d["top0"], rail0, 0.0)]), "depot-frame", "powder-coated-steel"))
    E.append(shape("blanket", "blanket", "body", 2, 0.3, poly_e("blanket", blanket_section(fold=fold, steps=6 if s50 else 2, y0=blanket0_of(scale))), "blanket", "felt-blanket"))
    if s50:
        E.append(shape("blanket-layers", "blanket", "texture", 2.1, 0.2, path_lines_e("blanket", [((-b["x"], b["y0"] + 0.003 * k), (-b["x"] + 0.05, b["y0"] + 0.003 * k)) for k in (1, 2, 3)]), "blanket", filled=False))
    E.append(shape("top", "table", "body", 3, 0.62, rect_e("table", -c["X"], TOP_Y, c["X"], TOP_Y + c["top"]), "table-top", "veneered-chipboard", hatch="grain-h" if s50 else None))
    if s50:
        E.append(shape("top-scuffs", "table", "texture", 3.1, 0.3, path_lines_e("table", [((-0.25, TOP_Y + 0.003), (-0.205, TOP_Y + 0.003)), ((0.10, TOP_Y + 0.003), (0.165, TOP_Y + 0.003)), ((0.21, TOP_Y + 0.006), (0.24, TOP_Y + 0.006))]), "table-top", filled=False))
    E.append(shape("shelf", "shelf", "body", 4, 0.6, rect_e("shelf", -c["shelfX"], wy(c["shelf1"]), c["shelfX"], wy(shelf0_of(scale))), "shelf", "veneered-chipboard"))
    if s50:
        E.append(shape("apron-end", "frame", "body", 5, 0.5, rect_e("frame", -LEG_IN_X, wy(APRON_Y1), LEG_IN_X, wy(APRON_Y0)), "apron", "oak-solid", hatch="grain-h"))
        for side, sign in (("a", 1), ("b", -1)):
            E.append(shape(f"leg-{side}", "frame", "body", 5.5, 0.55, rect_e("frame", sign * LEG_IN_X, wy(APRON_Y1), sign * LEG_OUT_X, wy(0.0)), "legs", "oak-solid", hatch="grain-v"))
        E.append(shape("leg-wear", "frame", "texture", 5.6, 0.3, path_lines_e("frame", [((-LEG_OUT_X + 0.006, wy(0.012)), (-LEG_IN_X - 0.006, wy(0.012))), ((LEG_IN_X + 0.006, wy(0.012)), (LEG_OUT_X - 0.006, wy(0.012)))]), "legs", filled=False))
        E.append(shape("leg-joints", "frame", "detail", 5.7, 0.3, path_lines_e("frame", [((-LEG_OUT_X, wy(APRON_Y0)), (-LEG_IN_X, wy(APRON_Y0))), ((LEG_IN_X, wy(APRON_Y0)), (LEG_OUT_X, wy(APRON_Y0)))]), "legs", filled=False))
    else:
        fr = shape("frame", "frame", "body", 5, 0.52, poly_e("frame", [(wx_, wy(y_)) for wx_, y_ in u_frame(-LEG_OUT_X, -cx0, cx0, LEG_OUT_X, APRON_Y1, band0, 0.0)]), None, "oak-solid")
        fr["regions"] = ["legs", "apron"]
        E.append(fr)
        if s100:
            E.append(shape("frame-joints", "frame", "detail", 5.1, 0.3, path_lines_e("frame", [((-LEG_OUT_X, wy(APRON_Y0)), (-cx0, wy(APRON_Y0))), ((cx0, wy(APRON_Y0)), (LEG_OUT_X, wy(APRON_Y0)))]), "legs", filled=False))

    F.append(shape("depot-top", "depot-table", "body", 1, top_t, rect_w(-d["z"], d["top0"], d["z"], d["top1"]), "depot-top", "melamine-chipboard"))
    if s50 or s500:
        for side, (a0, a1) in (("left", (lz0, lz1)), ("right", (-lz1, -lz0))):
            F.append(shape(f"depot-leg-{side}", "depot-table", "body", 1.2, 0.35, rect_w(a0, 0.0, a1, d["top0"]), "depot-frame", "powder-coated-steel"))
        if s50:
            F.append(shape("depot-rail", "depot-table", "body", 1.1, 0.4, rect_w(-lz0, rail0, lz0, d["top0"]), "depot-frame", "powder-coated-steel"))
            F.append(shape("depot-glides", "depot-table", "detail", 1.3, 0.2, path_lines([((lz0, d["plug"]), (lz1, d["plug"])), ((-lz1, d["plug"]), (-lz0, d["plug"]))]), "depot-frame", filled=False))
            F.append(shape("depot-tube-joints", "depot-table", "detail", 1.3, 0.2, path_lines([((-lz0, rail0), (-lz0, d["top0"])), ((lz0, rail0), (lz0, d["top0"]))]), "depot-frame", filled=False))
    else:
        F.append(shape("depot-frame", "depot-table", "body", 1.1, 0.35, poly_w(u_frame(-lz1, -lz0, lz0, lz1, d["top0"], rail0, 0.0)), "depot-frame", "powder-coated-steel"))
    F.append(shape("blanket", "blanket", "body", 2, 0.3, rect_w(-b["z"], blanket0_of(scale), b["z"], b["y1"]), "blanket", "felt-blanket"))
    F.append(shape("top", "table", "body", 3, 0.62, rect_w(-c["Z"], TOP_Y, c["Z"], TOP_Y + c["top"]), "table-top", "veneered-chipboard", hatch="grain-h" if s50 else None))
    F.append(shape("shelf", "shelf", "body", 4, 0.6, rect_w(-c["shelfZ"], wy(c["shelf1"]), c["shelfZ"], wy(shelf0_of(scale))), "shelf", "veneered-chipboard"))
    if s50:
        F.append(shape("apron-side", "frame", "body", 5, 0.5, rect_w(-LEG_IN_Z, wy(APRON_Y1), LEG_IN_Z, wy(APRON_Y0)), "apron", "oak-solid", hatch="grain-h"))
        for side, sign in (("l", 1), ("r", -1)):
            F.append(shape(f"leg-{side}", "frame", "body", 5.5, 0.55, rect_w(sign * LEG_IN_Z, wy(APRON_Y1), sign * LEG_OUT_Z, wy(0.0)), "legs", "oak-solid", hatch="grain-v"))
        F.append(shape("leg-wear", "frame", "texture", 5.6, 0.3, path_lines([((-LEG_OUT_Z + 0.006, wy(0.012)), (-LEG_IN_Z - 0.006, wy(0.012))), ((LEG_IN_Z + 0.006, wy(0.012)), (LEG_OUT_Z - 0.006, wy(0.012)))]), "legs", filled=False))
    else:
        fr = shape("frame", "frame", "body", 5, 0.52, poly_w(u_frame(-LEG_OUT_Z, -cz0, cz0, LEG_OUT_Z, wy(APRON_Y1), wy(band0), wy(0.0))), None, "oak-solid")
        fr["regions"] = ["legs", "apron"]
        F.append(fr)

    P.append(shape("depot-top", "depot-table", "body", 1, top_t, rect_w(-d["x"], -d["z"], d["x"], d["z"]), "depot-top", "melamine-chipboard"))
    P.append(shape("blanket", "blanket", "body", 2, 0.3, rect_w(-b["x"], -b["z"], b["x"], b["z"]), "blanket", "felt-blanket", hatch="quilt" if s50 else None))
    if scale in (50, 100):
        P.append(shape("top", "table", "body", 3, 0.62, poly_w(chip_outline_world()), "table-top", "veneered-chipboard"))
        P.append(shape("cup-rings", "table", "texture", 3.1, 0.35, path_circles([(wx(0.10), 0.20, 0.040), (wx(-0.05), -0.25, 0.045), (wx(0.12), -0.32, 0.040), (wx(0.09), -0.29, 0.040)] if s50 else [(wx(0.10), 0.20, 0.040), (wx(-0.05), -0.25, 0.045), (wx(0.12), -0.32, 0.040)]), "table-top", filled=False))
    else:
        P.append(shape("top", "table", "body", 3, 0.62, rect_w(-c["X"], -c["Z"], c["X"], c["Z"]), "table-top", "veneered-chipboard"))
    if s50:
        P.append(shape("top-scratches", "table", "texture", 3.2, 0.35, path_lines([((wx(-0.15), 0.05), (wx(-0.05), 0.16)), ((wx(-0.12), 0.08), (wx(-0.06), 0.15)), ((wx(0.18), -0.05), (wx(0.20), 0.09))]), "table-top", filled=False))
        P.append(shape("chip-core", "table", "detail", 3.3, 0.3, path_lines([((wx(0.272), 0.55), (wx(0.28), 0.545)), ((wx(0.292), 0.529), (wx(0.3), 0.525))]), "table-top", filled=False))
    if s50:
        t = c["apron_t"]
        for sid, (x0, z0, x1, z1) in (("apron-side-f", (APRON_OUT_X - t, -LEG_IN_Z, APRON_OUT_X, LEG_IN_Z)), ("apron-side-r", (-APRON_OUT_X, -LEG_IN_Z, -APRON_OUT_X + t, LEG_IN_Z)), ("apron-end-l", (-LEG_IN_X, APRON_OUT_Z - t, LEG_IN_X, APRON_OUT_Z)), ("apron-end-r", (-LEG_IN_X, -APRON_OUT_Z, LEG_IN_X, -APRON_OUT_Z + t))):
            P.append(shape(sid, "frame", "body", 4, 0.5, rect_w(x0, z0, x1, z1), "apron", "oak-solid"))
        for sid, sx, sz in (("leg-fl", 1, 1), ("leg-fr", 1, -1), ("leg-rl", -1, 1), ("leg-rr", -1, -1)):
            P.append(shape(sid, "frame", "body", 4.2, 0.55, rect_w(sx * LEG_IN_X, sz * LEG_IN_Z, sx * LEG_OUT_X, sz * LEG_OUT_Z), "legs", "oak-solid"))
    else:
        if s500:
            oX, oZ, t = LEG_OUT_X, LEG_OUT_Z, 0.075
        else:
            oX, oZ, t = APRON_OUT_X, APRON_OUT_Z, (0.03 if s200 else c["apron_t"])
        outer = f"M {-LEG_OUT_X:.5f} {-LEG_OUT_Z:.5f} L {LEG_OUT_X:.5f} {-LEG_OUT_Z:.5f} L {LEG_OUT_X:.5f} {LEG_OUT_Z:.5f} L {-LEG_OUT_X:.5f} {LEG_OUT_Z:.5f} Z"
        hole = f"M {-oX + t:.5f} {-oZ + t:.5f} L {-oX + t:.5f} {oZ - t:.5f} L {oX - t:.5f} {oZ - t:.5f} L {oX - t:.5f} {-oZ + t:.5f} Z"
        fr = shape("frame", "frame", "body", 4, 0.52, {"kind": "path", "d": outer + " " + hole}, None, "oak-solid")
        fr["regions"] = ["legs", "apron"]
        P.append(fr)
    P.append(shape("shelf", "shelf", "body", 4.1, 0.6, poly_w(shelf_plan_world()), "shelf", "veneered-chipboard"))
    if s50:
        P.append(shape("price-label", "shelf", "texture", 4.15, 0.9, path_lines([((0.06, -0.33), (0.12, -0.33)), ((0.12, -0.33), (0.12, -0.29)), ((0.12, -0.29), (0.06, -0.29)), ((0.06, -0.29), (0.06, -0.33)), ((0.07, -0.315), (0.11, -0.315))]), "shelf", filled=False))
    return {"elevation": E, "front": F, "plan": P}


def shelf_plan_world():
    """🗄 The notched lower shelf in plan, world rest (x, z), symmetric."""
    sx, sz, ix, iz = COFFEE["shelfX"], COFFEE["shelfZ"], LEG_IN_X, LEG_IN_Z
    return [(-sx, -iz), (-sx, iz), (-ix, iz), (-ix, sz), (ix, sz), (ix, iz), (sx, iz), (sx, -iz), (ix, -iz), (ix, -sz), (-ix, -sz), (-ix, -iz)]


# ---------------------------------------------------------------- 3D solids

def to_local3(bone, x, y, z):
    """📍 World rest point to the (u, v, w) frame of a bone."""
    u, v = loc(bone, x, y)
    return (u, v, z)


def box(sid, bone, lo, hi, material, tone, role="body", **extra):
    """📦 An axis-aligned world box as a box solid in the frame of its bone."""
    a = to_local3(bone, *lo)
    b = to_local3(bone, *hi)
    centre = [(a[i] + b[i]) / 2 for i in range(3)]
    size = [abs(a[i] - b[i]) for i in range(3)]
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


def rect_ring(a0, b0, a1, b1):
    """▭ A rectangle as a profile ring."""
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def plan_profile_zx(bone, pts_xz):
    """🗺 World plan points (x, z) as a `zx` profile (w, u) of a horizontally lying bone."""
    return [(z, loc(bone, x, 0.0)[0]) for x, z in pts_xz]


def v_of(bone, y):
    """↕ The local v of a world height on a horizontally lying bone."""
    return loc(bone, 0.0, y)[1]


def depot_solids(scale):
    """🛠 Depot table solids of one scale (top, steel tube frame, glides)."""
    d = DEPOT
    out = []
    bone = "depot-table"
    out.append(box("depot-top", bone, (-d["x"], d["top0"], -d["z"]), (d["x"], d["top1"], d["z"]), "melamine-chipboard", 0.85))
    if scale == 500:
        legw = 0.075
        lx0, lz0 = d["x"] - 0.02 - legw, d["z"] - 0.02 - legw
        for sid, sx, sz in (("fl", 1, 1), ("fr", 1, -1), ("rl", -1, 1), ("rr", -1, -1)):
            xa, xb = sorted((sx * lx0, sx * (lx0 + legw)))
            za, zb = sorted((sz * lz0, sz * (lz0 + legw)))
            out.append(box(f"depot-leg-{sid}", bone, (xa, 0.0, za), (xb, d["top0"], zb), "powder-coated-steel", 0.35))
        return out
    lx0, lz0, w = d["legx"], d["legz"], d["leg"]
    hollow = scale == 50
    y_leg0 = d["plug"] if hollow else 0.0
    for sid, sx, sz in (("fl", 1, 1), ("fr", 1, -1), ("rl", -1, 1), ("rr", -1, -1)):
        xa, xb = sorted((sx * lx0, sx * (lx0 + w)))
        za, zb = sorted((sz * lz0, sz * (lz0 + w)))
        ua, ub = sorted((loc(bone, xa, 0)[0], loc(bone, xb, 0)[0]))
        prof = rect_ring(za, ua, zb, ub)
        holes = [rect_ring(za + d["wall"], ua + d["wall"], zb - d["wall"], ub - d["wall"])] if hollow else None
        out.append(extrude(f"depot-leg-{sid}", bone, "zx", prof, v_of(bone, y_leg0), v_of(bone, d["top0"]), "powder-coated-steel", 0.35, holes=holes))
        if hollow:
            out.append(box(f"depot-glide-{sid}", bone, (xa, 0.0, za), (xb, d["plug"], zb), "glide-plastic", 0.2))
    rh, rw = d["rail_h"], (0.03 if scale == 200 else d["rail_w"])
    va, vb = sorted((v_of(bone, d["top0"] - rh), v_of(bone, d["top0"])))
    if scale == 200:
        xo, zo = lx0 + w, lz0 + w
        prof = plan_profile_zx(bone, [(-xo, -zo), (xo, -zo), (xo, zo), (-xo, zo)])
        hole = plan_profile_zx(bone, [(-xo + rw, -zo + rw), (xo - rw, -zo + rw), (xo - rw, zo - rw), (-xo + rw, zo - rw)])
        out.append(extrude("depot-rails", bone, "zx", prof, va, vb, "powder-coated-steel", 0.4, holes=[hole]))
        return out
    for sid, sx in (("front", 1), ("rear", -1)):
        xa, xb = sorted((sx * (lx0 + w - rw), sx * (lx0 + w)))
        ua, ub = sorted((loc(bone, xa, 0)[0], loc(bone, xb, 0)[0]))
        prof = rect_ring(ua, va, ub, vb)
        holes = [rect_ring(ua + d["wall"], va + d["wall"], ub - d["wall"], vb - d["wall"])] if hollow else None
        out.append(extrude(f"depot-rail-{sid}", bone, "xy", prof, -lz0, lz0, "powder-coated-steel", 0.4, holes=holes))
    for sid, sz in (("left", 1), ("right", -1)):
        za, zb = sorted((sz * (lz0 + w - rw), sz * (lz0 + w)))
        prof = rect_ring(va, za, vb, zb)
        holes = [rect_ring(va + d["wall"], za + d["wall"], vb - d["wall"], zb - d["wall"])] if hollow else None
        ua, ub = sorted((loc(bone, -lx0, 0)[0], loc(bone, lx0, 0)[0]))
        out.append(extrude(f"depot-rail-{sid}", bone, "yz", prof, ua, ub, "powder-coated-steel", 0.4, holes=holes))
    return out


def shelf0_of(scale):
    """🗄 Upright height of the shelf underside: true at 1:50 to 1:200, 3 mm lower at 1:500 so the 22 mm strip prints at 1:1000."""
    return COFFEE["shelf1"] - 0.022 if scale == 500 else COFFEE["shelf0"]


def blanket0_of(scale):
    """🧺 Underside of the blanket: true 12 mm at 1:50 to 1:200, a 22 mm strip reaching 10 mm into the depot top at 1:500 so it prints at 1:1000."""
    return BLANKET["y1"] - 0.022 if scale == 500 else BLANKET["y0"]


def blanket_solid(scale):
    """🧺 The folded moving blanket of one scale."""
    prof = [loc("blanket", x, y) for x, y in blanket_section(fold=scale in (50, 100), y0=blanket0_of(scale))]
    extra = {} if scale in (50, 500) else {"sheet": True}
    return extrude("blanket", "blanket", "xy", prof, -BLANKET["z"], BLANKET["z"], "felt-blanket", 0.3, **extra)


def coffee_solids(scale):
    """🪵 Coffee table solids of one scale (top, legs, aprons, lower shelf)."""
    c = COFFEE
    out = []
    top_pts = chip_outline_world() if scale in (50, 100) else [(c["X"], -c["Z"]), (c["X"], c["Z"]), (-c["X"], c["Z"]), (-c["X"], -c["Z"])]
    out.append(extrude("top", "table", "zx", plan_profile_zx("table", top_pts), v_of("table", TOP_Y), v_of("table", TOP_Y + c["top"]), "veneered-chipboard", 0.62))
    legw = 0.075 if scale == 500 else c["leg"]
    for sid, sx, sz in (("fl", 1, 1), ("fr", 1, -1), ("rl", -1, 1), ("rr", -1, -1)):
        Xa, Xb = sorted((sx * (LEG_OUT_X - legw), sx * LEG_OUT_X))
        Za, Zb = sorted((sz * (LEG_OUT_Z - legw), sz * LEG_OUT_Z))
        out.append(box(f"leg-{sid}", "frame", (wx(Xb), wy(APRON_Y1), Za), (wx(Xa), wy(0.0), Zb), "oak-solid", 0.55))
    if scale == 500:
        fl = 0.075
        outer = [(-LEG_OUT_X, -LEG_OUT_Z), (LEG_OUT_X, -LEG_OUT_Z), (LEG_OUT_X, LEG_OUT_Z), (-LEG_OUT_X, LEG_OUT_Z)]
        inner = [(-LEG_OUT_X + fl, -LEG_OUT_Z + fl), (LEG_OUT_X - fl, -LEG_OUT_Z + fl), (LEG_OUT_X - fl, LEG_OUT_Z - fl), (-LEG_OUT_X + fl, LEG_OUT_Z - fl)]
        out.append(frame_ring("frame-band", outer, inner, APRON_Y1 - fl, APRON_Y1))
    elif scale == 200:
        t = 0.03
        outer = [(-APRON_OUT_X, -APRON_OUT_Z), (APRON_OUT_X, -APRON_OUT_Z), (APRON_OUT_X, APRON_OUT_Z), (-APRON_OUT_X, APRON_OUT_Z)]
        inner = [(-APRON_OUT_X + t, -APRON_OUT_Z + t), (APRON_OUT_X - t, -APRON_OUT_Z + t), (APRON_OUT_X - t, APRON_OUT_Z - t), (-APRON_OUT_X + t, APRON_OUT_Z - t)]
        out.append(frame_ring("aprons", outer, inner, APRON_Y0, APRON_Y1))
    else:
        t = c["apron_t"]
        for sid, sx in (("f", 1), ("r", -1)):
            Xa, Xb = sorted((sx * (APRON_OUT_X - t), sx * APRON_OUT_X))
            out.append(box(f"apron-side-{sid}", "frame", (wx(Xb), wy(APRON_Y1), -LEG_IN_Z), (wx(Xa), wy(APRON_Y0), LEG_IN_Z), "oak-solid", 0.5))
        for sid, sz in (("l", 1), ("r", -1)):
            Za, Zb = sorted((sz * (APRON_OUT_Z - t), sz * APRON_OUT_Z))
            out.append(box(f"apron-end-{sid}", "frame", (wx(LEG_IN_X), wy(APRON_Y1), Za), (wx(-LEG_IN_X), wy(APRON_Y0), Zb), "oak-solid", 0.5))
    out.append(extrude("shelf", "shelf", "zx", plan_profile_zx("shelf", shelf_plan_world()), v_of("shelf", wy(c["shelf1"])), v_of("shelf", wy(shelf0_of(scale))), "veneered-chipboard", 0.6))
    return out


def frame_ring(sid, outer_XZ, inner_XZ, Y0, Y1):
    """🔲 A horizontal ring of the frame (merged aprons) as a `yz` extrude along the frame bone."""
    prof = [(X + 0.2475, Z) for X, Z in outer_XZ]
    hole = [(X + 0.2475, Z) for X, Z in inner_XZ]
    u0, u1 = sorted((COFFEE["H"] - COFFEE["top"] - Y1, COFFEE["H"] - COFFEE["top"] - Y0))
    return extrude(sid, "frame", "yz", prof, u0, u1, "oak-solid", 0.5, holes=[hole])


def disc(sid, bone, member, anchor, origin, axis, r, h, sign=-1):
    """🛞 A support pad: a disc on the support anchor along the restraint direction."""
    a0, a1 = sorted((0.0, sign * h))
    return {"id": sid, "kind": "revolve", "bone": bone, "material": "glide-plastic", "role": "ground", "tone": 0.2, "profile": [[a0, 0.0], [a0, r], [a1, r], [a1, 0.0]], "axis": axis, "origin": list(origin), "member": member, "part": "support", "anchor": anchor}


def mass_marker(sid, bone, at, r, material):
    """⚖ A centre-of-mass marker."""
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": list(at)}, "member": bone, "part": "mass"}


# ---------------------------------------------------------------- masses and anchors

def masses():
    """⚖ Bone masses from section x density and their centres (local frames)."""
    d, b, c = DEPOT, BLANKET, COFFEE
    top_v = 2 * d["x"] * 2 * d["z"] * (d["top1"] - d["top0"])
    a_leg = d["leg"] ** 2 - (d["leg"] - 2 * d["wall"]) ** 2
    leg_v = 4 * a_leg * (d["top0"] - d["plug"])
    glide_v = 4 * d["leg"] ** 2 * d["plug"]
    a_rail = d["rail_h"] * d["rail_w"] - (d["rail_h"] - 2 * d["wall"]) * (d["rail_w"] - 2 * d["wall"])
    rail_len = 2 * (2 * d["legz"]) + 2 * (2 * d["legx"])
    rail_v = a_rail * rail_len
    m_top = top_v * RHO["melamine-chipboard"]
    m_leg = leg_v * RHO["powder-coated-steel"]
    m_glide = glide_v * RHO["glide-plastic"]
    m_rail = rail_v * RHO["powder-coated-steel"]
    m_depot = m_top + m_leg + m_glide + m_rail
    y_depot = (m_top * (d["top0"] + d["top1"]) / 2 + m_leg * (d["plug"] + d["top0"]) / 2 + m_glide * d["plug"] / 2 + m_rail * (d["top0"] - d["rail_h"] / 2)) / m_depot
    blanket_area = 2 * b["x"] * 2 * b["z"]
    m_blanket = blanket_area * (b["y1"] - b["y0"]) * RHO["felt-blanket"]
    chip_area = polygon_area(chip_outline_upright())
    m_ctop = chip_area * c["top"] * RHO["veneered-chipboard"]
    m_legs = 4 * c["leg"] ** 2 * APRON_Y1 * RHO["oak-solid"]
    v_apron = 2 * (2 * LEG_IN_Z) * c["apron_h"] * c["apron_t"] + 2 * (2 * LEG_IN_X) * c["apron_h"] * c["apron_t"]
    m_apron = v_apron * RHO["oak-solid"]
    shelf_area = polygon_area(shelf_plan_world())
    m_shelf = shelf_area * (c["shelf1"] - c["shelf0"]) * RHO["veneered-chipboard"]
    m_frame = m_legs + m_apron
    Y_frame = (m_legs * APRON_Y1 / 2 + m_apron * (APRON_Y0 + APRON_Y1) / 2) / m_frame
    return {
        "depot-table": (m_depot, (0.40, y_depot - 0.7275)),
        "blanket": (m_blanket, (0.36, 0.0)),
        "table": (m_ctop, (0.30, 0.0)),
        "frame": (m_frame, (COFFEE["H"] - COFFEE["top"] - Y_frame, 0.2475)),
        "shelf": (m_shelf, (0.26, 0.0)),
        "_parts": dict(m_top=m_top, m_leg=m_leg, m_glide=m_glide, m_rail=m_rail, m_blanket=m_blanket, m_ctop=m_ctop, m_legs=m_legs, m_apron=m_apron, m_shelf=m_shelf, chip_area=chip_area, shelf_area=shelf_area, a_leg=a_leg, a_rail=a_rail, rail_len=rail_len),
    }


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


DEPOT_FEET = {"depot-foot-fl": (1, 1), "depot-foot-fr": (1, -1), "depot-foot-rl": (-1, 1), "depot-foot-rr": (-1, -1)}
TABLE_FEET = {"table-foot-fl": (1, 1), "table-foot-fr": (1, -1), "table-foot-rl": (-1, 1), "table-foot-rr": (-1, -1)}
GRIPS = {"grip-left": 1, "grip-right": -1}
GRIP_Z = (COFFEE["Z"] + APRON_OUT_Z) / 2


def anchors():
    """🪝 Contacts of the depot table and the coffee table, the two grips of the carrier and the test point at the broken corner."""
    out = []
    names = {"fl": T("front left", "vorne links"), "fr": T("front right", "vorne rechts"), "rl": T("rear left", "hinten links"), "rr": T("rear right", "hinten rechts")}
    cx = DEPOT["legx"] + DEPOT["leg"] / 2
    cz = DEPOT["legz"] + DEPOT["leg"] / 2
    for aid, (sx, sz) in DEPOT_FEET.items():
        n = names[aid[-2:]]
        out.append({"id": aid, "bone": "depot-table", "x": 0.40 + sx * cx, "y": -0.7275, "z": sz * cz, "kind": "contact", "title": T(f"Depot table glide, {n['en']}", f"Gleiter des Lagertischs, {n['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": DEPOT["leg"], "footprintWidthM": DEPOT["leg"], "frictionCoefficient": 0.4})
    for aid, (sx, sz) in TABLE_FEET.items():
        n = names[aid[-2:]]
        out.append({"id": aid, "bone": "frame", "x": APRON_Y1, "y": 0.2475 + sx * LEG_CX, "z": sz * LEG_CZ, "kind": "contact", "title": T(f"Coffee table leg end, {n['en']}", f"Beinende des Couchtischs, {n['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": COFFEE["leg"], "footprintWidthM": COFFEE["leg"], "frictionCoefficient": 0.5})
    for aid, sz in GRIPS.items():
        side = T("left", "links") if sz > 0 else T("right", "rechts")
        out.append({"id": aid, "bone": "table", "x": 0.30, "y": -COFFEE["top"] / 2, "z": sz * GRIP_Z, "kind": "contact", "title": T(f"Carrier's hand under the top overhang, {side['en']} short end", f"Hand des Trägers unter dem Plattenüberstand, {side['de']} Stirnseite"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.035, "frictionCoefficient": 0.6})
    out.append({"id": "corner-test", "bone": "table", "x": 0.30 + 0.25, "y": COFFEE["top"] / 2, "z": 0.50, "kind": "load", "title": T("Test point 50 mm inside the broken corner", "Prüfpunkt 50 mm innerhalb der abgebrochenen Ecke")})
    return out


# ---------------------------------------------------------------- volumes per scale

def volume(scale, body):
    """🧊 Thermal and structural lists of one scale from its body list."""
    f = FEATURE[scale]
    region_of = {"depot-top": "depot-top", "blanket": "blanket", "top": "table-top", "shelf": "shelf"}
    thermal = []
    for s in body:
        sid = s["id"]
        if sid in region_of:
            reg = region_of[sid]
        elif sid.startswith("depot-"):
            reg = "depot-frame"
        elif sid.startswith("leg-"):
            reg = "legs"
        elif sid.startswith("apron") or sid == "aprons":
            reg = "apron"
        elif sid == "frame-band":
            reg = "apron"
        else:
            raise ValueError(sid)
        thermal.append({"id": f"th-{sid}", "of": sid, "region": reg})
    structure = []
    member_of = lambda sid: "depot-table" if sid.startswith("depot-") else ("blanket" if sid == "blanket" else ("table" if sid == "top" else ("shelf" if sid == "shelf" else "frame")))
    for s in body:
        if s["id"].startswith("depot-glide"):
            continue
        structure.append({"id": f"m-{s['id']}", "of": s["id"], "member": member_of(s["id"]), "part": "member"})
    for sid, sx, sz in (("fl", 1, 1), ("fr", 1, -1), ("rl", -1, 1), ("rr", -1, -1)):
        legw = 0.075 if scale == 500 else COFFEE["leg"]
        Xc, Zc = sx * (LEG_OUT_X - legw / 2), sz * (LEG_OUT_Z - legw / 2)
        jh = max(f, 0.03)
        structure.append(box(f"j-leg-top-{sid}", "frame", (wx(Xc) - legw / 2, wy(APRON_Y1), Zc - legw / 2), (wx(Xc) + legw / 2, wy(APRON_Y1 - jh), Zc + legw / 2), "powder-coated-steel", 0.3, role="detail", member="frame", part="joint"))
        jl = max(f, 0.02)
        ys = (COFFEE["shelf0"] + COFFEE["shelf1"]) / 2
        sh = max(f, COFFEE["shelf1"] - COFFEE["shelf0"])
        Xs, Zs = sx * LEG_IN_X, sz * LEG_IN_Z
        structure.append(box(f"j-shelf-{sid}", "shelf", (wx(Xs) - jl / 2, wy(ys) - sh / 2, Zs - jl / 2), (wx(Xs) + jl / 2, wy(ys) + sh / 2, Zs + jl / 2), "oak-solid", 0.3, role="detail", member="shelf", part="joint"))
        dl = 0.075 if scale == 500 else DEPOT["leg"]
        dx0 = DEPOT["x"] - 0.02 - dl if scale == 500 else DEPOT["legx"]
        dz0 = DEPOT["z"] - 0.02 - dl if scale == 500 else DEPOT["legz"]
        xa, xb = sorted((sx * dx0, sx * (dx0 + dl)))
        za, zb = sorted((sz * dz0, sz * (dz0 + dl)))
        jh = max(f, DEPOT["rail_h"])
        structure.append(box(f"j-weld-{sid}", "depot-table", (xa, DEPOT["top0"] - jh, za), (xb, DEPOT["top0"], zb), "powder-coated-steel", 0.3, role="detail", member="depot-table", part="joint"))
    r = f
    h = f
    for aid, (sx, sz) in DEPOT_FEET.items():
        a = next(x for x in ANCHORS if x["id"] == aid)
        structure.append(disc(f"support-{aid}", "depot-table", "depot-table", aid, (a["x"], a["y"], a["z"]), [0, 1, 0], r if scale < 500 else 0.0375, h, sign=1))
    M = masses()
    mr = f if scale < 500 else 0.0375
    marker_at = {"depot-table": (0.40, DEPOT["top0"] - DEPOT["rail_h"] / 2 - 0.7275), "frame": (COFFEE["H"] - COFFEE["top"] - (APRON_Y1 - 0.0375 if scale == 500 else (APRON_Y0 + APRON_Y1) / 2), 0.2475)}
    for bone in ("depot-table", "blanket", "table", "frame", "shelf"):
        kg, (u, v) = M[bone]
        if bone == "blanket":
            continue
        u, v = marker_at.get(bone, (u, v))
        structure.append(mass_marker(f"mass-{bone}", bone, (u, v, 0.0), mr, "oak-solid" if bone != "depot-table" else "powder-coated-steel"))
    return thermal, structure


ANCHORS = anchors()


def build_volumes():
    """🧊 The four individually authored volumes."""
    texts = {
        50: [T("Body, authored for 7.5 mm: the coffee table upside down on the folded moving blanket on the depot table. Coffee table: the 25 mm oak-veneered chipboard top with its corner broken off (about 30 x 30 mm through the whole thickness), four 45 mm square solid oak legs 425 mm long, 20 x 70 mm aprons set 5 mm back from the leg faces, the 19 mm lower shelf notched round the legs. Depot table: the 25 mm melamine top, four 40 x 40 x 2 mm square tube legs with their 8 mm plastic glides and the 40 x 20 x 2 mm tube rails, all hollow with their true wall; the blanket as four 3 mm layers folded to 12 mm with its rounded fold at the front edge.", "Körper für 7,5 mm: der Couchtisch kopfüber auf der gefalteten Möbeldecke auf dem Lagertisch. Couchtisch: die 25-mm-Platte aus eichenfurnierter Spanplatte mit abgebrochener Ecke (etwa 30 x 30 mm durch die ganze Dicke), vier 425 mm lange Vierkantbeine aus massiver Eiche 45 mm, Zargen 20 x 70 mm 5 mm hinter den Beinflächen, der 19-mm-Zwischenboden um die Beine ausgeklinkt. Lagertisch: die 25-mm-Melaminplatte, vier Beine aus Vierkantrohr 40 x 40 x 2 mm mit 8-mm-Kunststoffgleitern und die Rohrzargen 40 x 20 x 2 mm, alle hohl mit echter Wand; die Decke als vier 3-mm-Lagen auf 12 mm gefaltet mit gerundetem Falz an der Vorderkante."),
             T("Thermal: every body solid by reference in its region (the glides belong to the frame region). Structure: top, legs, aprons, shelf, blanket, depot top, tubes as members; corner fittings at the leg heads, dowels of the shelf at the legs and welds at the depot leg heads as joints; one pad per permanent support (the four depot glides; leg ends and hands are switched contacts of the poses and clips and carry no pad); masses at the centres of the bones.", "Thermik: jeder Körper als Verweis in seiner Region (die Gleiter gehören zur Gestellregion). Tragwerk: Platte, Beine, Zargen, Zwischenboden, Decke, Lagertischplatte und Rohre als Stäbe; Eckbeschläge an den Beinköpfen, Dübel des Zwischenbodens an den Beinen und Schweißnähte an den Beinköpfen des Lagertischs als Knoten; je ständigem Auflager eine Auflagerscheibe (die vier Lagertischgleiter; Beinenden und Hände sind geschaltete Kontakte der Posen und Clips ohne Scheibe); Massen in den Schwerpunkten der Knochen.")],
        100: [T("Body, authored anew for 15 mm: the broken corner stays (30 mm), the tubes of the depot table are solid bars of their outer section (the 2 mm wall and the 36 mm bore are below what the scale models as a hollow part with its plugs), the glides merge into the legs, the blanket is one 12 mm sheet with its rounded fold (declared sheet), aprons, legs and shelf keep their true sections.", "Körper für 15 mm neu gebaut: die abgebrochene Ecke bleibt (30 mm), die Rohre des Lagertischs sind volle Stäbe ihres Außenquerschnitts (2-mm-Wand und 36-mm-Bohrung mit ihren Stopfen modelliert der Maßstab nicht hohl), die Gleiter gehen in den Beinen auf, die Decke ist eine 12-mm-Lage mit gerundetem Falz (als Blatt deklariert), Zargen, Beine und Zwischenboden behalten ihre echten Querschnitte."),
              T("Thermal: one surface per region by reference. Structure: the same members, corner fittings, shelf dowels and welds at 15 mm, one pad per depot glide and the masses.", "Thermik: eine Oberfläche je Region als Verweis. Tragwerk: dieselben Stäbe, Eckbeschläge, Bodendübel und Schweißnähte bei 15 mm, je Lagertischgleiter eine Auflagerscheibe und die Massen.")],
        200: [T("Body, authored anew for 30 mm: square-cornered top (the 30 mm break is one feature and merges into the corner), the four aprons merged into one ring 30 mm thick with its outer faces kept, the four depot rails merged into one ring 30 x 40 mm, the blanket a flat 12 mm sheet; top, shelf and depot top keep their true 25 and 19 mm thickness as drawn (C13).", "Körper für 30 mm neu gebaut: rechteckige Platte (der 30-mm-Ausbruch ist ein Merkmal und geht in der Ecke auf), die vier Zargen zu einem 30 mm dicken Ring mit unveränderten Außenflächen verschmolzen, die vier Lagertischzargen zu einem Ring 30 x 40 mm, die Decke eine ebene 12-mm-Lage; Platte, Zwischenboden und Lagertischplatte behalten ihre echten 25 und 19 mm wie gezeichnet (C13)."),
              T("Thermal: one surface per region by reference. Structure: members by reference, fittings, dowels and welds at 30 mm, one pad per depot glide, masses.", "Thermik: eine Oberfläche je Region als Verweis. Tragwerk: Stäbe als Verweis, Beschläge, Dübel und Schweißnähte bei 30 mm, je Lagertischgleiter eine Auflagerscheibe, Massen.")],
        500: [T("Body, authored anew for 75 mm: the sign of the stack. Depot table as its 25 mm top on four 75 mm legs (the 40 mm tubes widened inwards, the rails left out), the blanket as the 12 mm sheet between the tables, the coffee table as its 25 mm top, a 75 mm frame band in which aprons and leg heads merge, four 75 mm legs and the 19 mm shelf; slabs and shelf keep their drawn thickness (C13).", "Körper für 75 mm neu gebaut: das Zeichen des Stapels. Lagertisch als 25-mm-Platte auf vier 75-mm-Beinen (die 40-mm-Rohre nach innen verbreitert, die Zargen entfallen), die Decke als 12-mm-Lage zwischen den Tischen, der Couchtisch als 25-mm-Platte, ein 75-mm-Rahmenband, in dem Zargen und Beinköpfe aufgehen, vier 75-mm-Beine und der 19-mm-Zwischenboden; Platten und Boden behalten ihre gezeichnete Dicke (C13)."),
              T("Thermal: one surface per region by reference; the frame band carries the apron region. Structure: members by reference, joints at 75 mm, one pad per depot glide, masses.", "Thermik: eine Oberfläche je Region als Verweis; das Rahmenband trägt die Zargenregion. Tragwerk: Stäbe als Verweis, Knoten bei 75 mm, je Lagertischgleiter eine Auflagerscheibe, Massen.")],
    }
    vols = {}
    for scale in (50, 100, 200, 500):
        body = depot_solids(scale) + [blanket_solid(scale)] + coffee_solids(scale)
        thermal, structure = volume(scale, body)
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": texts[scale]}
    return vols




# ---------------------------------------------------------------- document

REFS = [
    ("en-12521-2015", "EN 12521:2015 Furniture - Strength, durability and safety - Requirements for domestic tables (vertical static load on the top, stability under vertical load at the edge)."),
    ("en-1730-2012", "EN 1730:2012 Furniture - Tables - Test methods for the determination of stability, strength and durability (load points 50 mm from the edge of the top)."),
    ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: densities, self-weight, imposed loads for buildings (category A residential floors; used here only for the order of household live loads)."),
    ("en-312-2010", "EN 312:2010 Particleboards - Specifications (type P2, boards for interior fitments including furniture in dry conditions; bending strength about 11 MPa and modulus of elasticity about 1.6-1.8 GPa for 13-20 mm boards; density about 600-700 kg/m3)."),
    ("en-10219-2-2019", "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Tolerances, dimensions and sectional properties (square 40 x 40 x 2 mm about 2.9 cm2, rectangular 40 x 20 x 2 mm about 2.1 cm2 with corner radii; the model uses the sharp-cornered section)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (particleboard 600-900 kg/m3 0.14-0.18 W/mK and 1700 J/kgK, timber 700 kg/m3 0.18 W/mK and 1600 J/kgK, steel 50 W/mK and 450 J/kgK, nonwoven polyester insulation felts about 0.04-0.05 W/mK)."),
    ("wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. FPL-GTR-282, USDA Forest Service (air-dry density and elastic modulus of white oak, about 700 kg/m3 and 11-12 GPa; lacquered wood emissivity about 0.9)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total hemispherical emissivity of paints, lacquers, wood and textiles 0.85-0.95; solar absorptance of light and dark surfaces)."),
    ("neufert-2019", "Neufert, E., Neufert, P. (2019). Bauentwurfslehre, 42nd ed., Springer Vieweg (living room furniture: low tables 0.40-0.50 m high, 1.00-1.30 x 0.50-0.70 m; working tables 0.72-0.75 m)."),
    ("din-18011-1992", "DIN 18011:1992-11 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (placing areas of living room furniture including low tables beside the seating group)."),
    ("coffee-table-catalogue", "Catalogue-typical data of oak-veneered coffee tables with a lower shelf as German mid-range furniture stores sold them 1990-2010, 110 x 60 x 45 cm: veneered chipboard tops 22-28 mm with solid edging, square legs 40-50 mm, lower shelves 16-19 mm at 10-15 cm, net weight 18-25 kg (authored-estimate from listings of the type, no brand)."),
    ("depot-table-catalogue", "Catalogue-typical data of workshop and packing tables 1200 x 800 mm: 25 mm melamine-faced chipboard top on a welded frame of 40 x 40 x 2 mm square tube legs and 40 x 20 x 2 mm rails, working height 740 mm, plastic glides (authored-estimate from listings of the type, no brand)."),
    ("moving-blanket-catalogue", "Catalogue-typical data of removal blankets (Umzugsdecken, Möbeldecken) of quilted nonwoven felt, about 150 x 180 cm, 3 mm, 1.0-1.5 kg (about 0.4 kg/m2) (authored-estimate)."),
    ("reuse-depot-practice", "Practice of municipal second-hand department stores and reuse depots in Germany (Gebrauchtwarenkaufhaus, Haushaltsauflösung): small tables are stored upside down on larger tables on a moving blanket, cleaned, priced on a label and carried out by one person (authored-estimate from the practice described in reuse programmes, no single source)."),
    ("dguv-208-033", "DGUV Information 208-033 (2014) Muskel-Skelett-Belastungen - erkennen und beurteilen (manual handling: object grip height about 0.75-0.85 m for carrying in front of the body, loads up to about 25 kg for one adult)."),
]


def materials():
    """🧪 The materials with their aged surface properties."""
    return [
        {"id": "veneered-chipboard", "title": T("Oak-veneered P2 chipboard, lacquered and aged", "Eichenfurnierte Spanplatte P2, lackiert und gealtert"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": RHO["veneered-chipboard"], "conductivityWmK": 0.145, "specificHeatJKgK": 1700, "elasticModulusPa": 1.7e9, "strengthPa": 1.1e7, "source": "literature", "reference": "en-312-2010"},
        {"id": "oak-solid", "title": T("Solid oak, lacquered and aged", "Massive Eiche, lackiert und gealtert"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": RHO["oak-solid"], "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 1.1e10, "strengthPa": 4.0e7, "source": "literature", "reference": "wood-handbook-2021"},
        {"id": "melamine-chipboard", "title": T("Light grey melamine-faced chipboard", "Lichtgraue melaminbeschichtete Spanplatte"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": RHO["melamine-chipboard"], "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 2.55e9, "strengthPa": 1.5e7, "source": "literature", "reference": "en-312-2010"},
        {"id": "powder-coated-steel", "title": T("Powder-coated steel tube, grey", "Pulverbeschichtetes Stahlrohr, grau"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO["powder-coated-steel"], "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "literature", "reference": "en-10219-2-2019"},
        {"id": "felt-blanket", "title": T("Quilted nonwoven felt of a moving blanket, dark blue", "Gesteppter Vliesfilz einer Möbeldecke, dunkelblau"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": RHO["felt-blanket"], "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "glide-plastic", "title": T("Polyethylene glides and pads", "Gleiter und Polster aus Polyethylen"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO["glide-plastic"], "conductivityWmK": 0.4, "specificHeatJKgK": 1900, "source": "authored-estimate", "reference": "iso-10456-2007"},
    ]


def rig():
    """🦴 The rig: depot table and blanket stay behind, the coffee table is lifted, carried away and turned over by the carrier's hands."""
    M = masses()

    def mass(bone, src="authored-estimate", ref="coffee-table-catalogue"):
        kg, (u, v) = M[bone]
        return {"kg": kg, "comX": u, "comY": v, "distribution": "point", "source": src, "reference": ref}

    c = COFFEE
    return {"kind": "mechanical", "root": "ground", "bones": [
        {"id": "ground", "parent": None, "title": T("Floor reference", "Bodenbezug"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "depot-table", "parent": "ground", "title": T("Depot table (storage aid, stays behind)", "Lagertisch (Lagerhilfe, bleibt stehen)"), "x": -0.40, "y": 0.7275, "angleDeg": 0, "lengthM": 0.80, "joint": {"type": "fixed"}, "mass": mass("depot-table", ref="depot-table-catalogue"), "section": {"areaM2": 1.2 * 0.025, "secondMomentM4": 1.2 * 0.025 ** 3 / 12, "extremeFibreM": 0.0125, "material": "melamine-chipboard", "title": T("Melamine top 1200 x 25 mm", "Melaminplatte 1200 x 25 mm")}},
        {"id": "blanket", "parent": "depot-table", "title": T("Folded moving blanket (storage aid, stays behind)", "Gefaltete Möbeldecke (Lagerhilfe, bleibt liegen)"), "x": 0.04, "y": 0.0185, "angleDeg": 0, "lengthM": 0.72, "joint": {"type": "fixed"}, "mass": mass("blanket", ref="moving-blanket-catalogue")},
        {"id": "carry-lift", "parent": "ground", "title": T("Carrier's hands: lift", "Hände des Trägers: Heben"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.40, "maxM": 0.20}},
        {"id": "carry-travel", "parent": "carry-lift", "title": T("Carrier's hands: walking away", "Hände des Trägers: Weggehen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.20}},
        {"id": "turn", "parent": "carry-travel", "title": T("Turning over about the grip axis", "Umdrehen um die Griffachse"), "x": 0, "y": PIVOT_Y, "angleDeg": 180, "lengthM": 0, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 180}},
        {"id": "table", "parent": "turn", "title": T("Coffee table top", "Couchtischplatte"), "x": -0.30, "y": 0, "angleDeg": 0, "lengthM": 0.60, "joint": {"type": "fixed"}, "mass": mass("table"), "section": {"areaM2": 1.1 * c["top"], "secondMomentM4": 1.1 * c["top"] ** 3 / 12, "extremeFibreM": c["top"] / 2, "material": "veneered-chipboard", "title": T("Top 1100 x 25 mm", "Platte 1100 x 25 mm")}},
        {"id": "frame", "parent": "table", "title": T("Legs and aprons", "Beine und Zargen"), "x": 0.0525, "y": -0.0125, "angleDeg": -90, "lengthM": APRON_Y1, "joint": {"type": "fixed"}, "mass": mass("frame"), "section": {"areaM2": c["leg"] ** 2, "secondMomentM4": c["leg"] ** 4 / 12, "extremeFibreM": c["leg"] / 2, "material": "oak-solid", "title": T("Leg 45 x 45 mm", "Bein 45 x 45 mm")}},
        {"id": "shelf", "parent": "table", "title": T("Lower shelf", "Zwischenboden"), "x": 0.04, "y": -0.317, "angleDeg": 0, "lengthM": 2 * c["shelfX"], "joint": {"type": "fixed"}, "mass": mass("shelf"), "section": {"areaM2": 2 * c["shelfZ"] * 0.019, "secondMomentM4": 2 * c["shelfZ"] * 0.019 ** 3 / 12, "extremeFibreM": 0.0095, "material": "veneered-chipboard", "title": T("Shelf 1020 x 19 mm", "Boden 1020 x 19 mm")}},
    ]}


ENV = {"airTempC": 19, "meanRadiantTempC": 19, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 350, "diffuseFraction": 0.15, "pressurePa": 101325}


def areas():
    """📐 Outer areas of the regions from the 1:50 model (hand calculation)."""
    c, d, b = COFFEE, DEPOT, BLANKET
    top = 2 * polygon_area(chip_outline_upright()) + polygon_perimeter(chip_outline_upright()) * c["top"]
    legs = 4 * (4 * c["leg"] * APRON_Y1 + 2 * c["leg"] ** 2)
    long_a = 2 * (2 * (2 * LEG_IN_Z) * (c["apron_h"] + c["apron_t"]) + 2 * c["apron_h"] * c["apron_t"])
    short_a = 2 * (2 * (2 * LEG_IN_X) * (c["apron_h"] + c["apron_t"]) + 2 * c["apron_h"] * c["apron_t"])
    shelf = 2 * polygon_area(shelf_plan_world()) + polygon_perimeter(shelf_plan_world()) * (c["shelf1"] - c["shelf0"])
    dtop = 2 * (2 * d["x"]) * (2 * d["z"]) + 2 * (2 * d["x"] + 2 * d["z"]) * (d["top1"] - d["top0"])
    leg_one = 4 * d["leg"] * d["top0"] + d["leg"] ** 2 + masses()["_parts"]["a_leg"] - 2 * d["rail_h"] * d["rail_w"]
    rails = 2 * (d["rail_h"] + d["rail_w"]) * (2 * (2 * d["legz"]) + 2 * (2 * d["legx"]))
    frame = 4 * leg_one + rails
    blanket_pts = blanket_section(True)
    blanket = 2 * (2 * b["x"]) * (2 * b["z"]) + polygon_perimeter(blanket_pts) * 0 + 2 * polygon_area(blanket_pts) + (polygon_perimeter(blanket_pts) - 2 * 2 * b["x"]) * 2 * b["z"]
    return {"table-top": top, "legs": legs, "apron": long_a + short_a, "shelf": shelf, "depot-top": dtop, "depot-frame": frame, "blanket": blanket}


def layer(lid, title, kind, material, t, lam, rho, cp, src, ref):
    """🧶 One layer with resistance and the areal heat capacity of one hour's penetration depth."""
    depth = math.sqrt(lam / (rho * cp) * 3600)
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": t / lam, "heatCapacityJM2K": rho * cp * min(t, depth), "source": src, "reference": ref}


def region(rid, title, material, area, L, geom, svf, lay, extra=None):
    """🔥 A passive surface region with one layer."""
    r = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": L, "geometry": geom, "skyViewFactor": svf, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


def thermal_model():
    """🌡 Seven passive regions with aged surface data."""
    A = areas()
    vc = ("veneered-chipboard", 0.145, RHO["veneered-chipboard"], 1700)
    oak = ("oak-solid", 0.18, RHO["oak-solid"], 1600)
    mel = ("melamine-chipboard", 0.14, RHO["melamine-chipboard"], 1700)
    st = ("powder-coated-steel", 50.0, RHO["powder-coated-steel"], 450)
    fe = ("felt-blanket", 0.045, RHO["felt-blanket"], 1300)
    regions = [
        region("table-top", T("Oak-veneered top, ring-marked and sun-bleached", "Eichenfurnierte Platte, mit Wasserrändern und sonnengebleicht"), "veneered-chipboard", A["table-top"], 0.6, "horizontal-plate-up", 0.85, layer("top-board", T("Veneered chipboard 25 mm", "Furnierte Spanplatte 25 mm"), "timber", vc[0], 0.025, *vc[1:], "literature", "en-312-2010"), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("legs", T("Solid oak legs, scuffed ends", "Beine aus massiver Eiche, bestoßene Enden"), "oak-solid", A["legs"], 0.045, "vertical-plate", 0.3, layer("leg-wood", T("Oak, half thickness 22.5 mm", "Eiche, halbe Dicke 22,5 mm"), "timber", oak[0], 0.0225, *oak[1:], "literature", "wood-handbook-2021")),
        region("apron", T("Oak aprons", "Zargen aus Eiche"), "oak-solid", A["apron"], 0.07, "vertical-plate", 0.45, layer("apron-board", T("Oak board 20 mm", "Eichenbrett 20 mm"), "timber", oak[0], 0.02, *oak[1:], "literature", "wood-handbook-2021")),
        region("shelf", T("Lower shelf, veneered chipboard", "Zwischenboden, furnierte Spanplatte"), "veneered-chipboard", A["shelf"], 0.52, "horizontal-plate-up", 0.35, layer("shelf-board", T("Veneered chipboard 19 mm", "Furnierte Spanplatte 19 mm"), "timber", vc[0], 0.019, *vc[1:], "literature", "en-312-2010")),
        region("depot-top", T("Depot table top, melamine", "Lagertischplatte, Melamin"), "melamine-chipboard", A["depot-top"], 0.8, "horizontal-plate-up", 0.5, layer("depot-board", T("Melamine chipboard 25 mm", "Melamin-Spanplatte 25 mm"), "timber", mel[0], 0.025, *mel[1:], "literature", "en-312-2010")),
        region("depot-frame", T("Depot table steel tube frame", "Stahlrohrgestell des Lagertischs"), "powder-coated-steel", A["depot-frame"], 0.04, "vertical-plate", 0.3, layer("tube-wall", T("Steel tube wall 2 mm", "Stahlrohrwand 2 mm"), "metal", st[0], 0.002, *st[1:], "literature", "iso-10456-2007")),
        region("blanket", T("Folded moving blanket", "Gefaltete Möbeldecke"), "felt-blanket", A["blanket"], 0.72, "horizontal-plate-up", 0.5, layer("felt", T("Four layers of felt, 12 mm", "Vier Lagen Filz, 12 mm"), "polymer", fe[0], 0.012, *fe[1:], "authored-estimate", "iso-10456-2007")),
    ]
    return {"environment": ENV, "operating": [], "regions": regions, "emitters": [], "passengers": [], "assumptions": [
        T("Aged surfaces: the lacquered oak veneer of the top has bleached in window sun, its solar absorptance is taken as 0.50 instead of about 0.55 for new lacquered oak; the lacquer is worn and ring-marked but still closed, so the emissivity of lacquered wood (0.9) applies (Incropera, authored-estimate).", "Gealterte Oberflächen: das lackierte Eichenfurnier der Platte ist im Fensterlicht ausgebleicht, sein solarer Absorptionsgrad wird mit 0,50 statt etwa 0,55 für neue lackierte Eiche angesetzt; der Lack ist abgenutzt und hat Wasserränder, ist aber geschlossen, es gilt der Emissionsgrad von lackiertem Holz (0,9) (Incropera, Schätzwert)."),
        T("Areal heat capacity rho cp min(t, sqrt(alpha 3600 s)) per ISO 10456 data: the 25 mm top and the 19 mm shelf take part over one hour's penetration depth of about 22 mm.", "Flächenbezogene Wärmekapazität rho cp min(t, sqrt(alpha 3600 s)) nach ISO-10456-Daten: Platte (25 mm) und Boden (19 mm) wirken über die Eindringtiefe einer Stunde von etwa 22 mm."),
        T("Reference case: indoors at 19 degC with still air and a 350 W/m2 window sun patch; in the depot there is no sun (sunlit 0), in the living room a sun patch lies on the top. Contacts: the top lies on the blanket and the blanket on the depot table at room temperature; the carrier's hands (32 degC) touch 0.5 % of the top.", "Referenzfall: innen bei 19 degC, ruhige Luft und ein Fensterlichtfleck von 350 W/m2; im Lager scheint keine Sonne (besonnt 0), im Wohnzimmer liegt ein Sonnenfleck auf der Platte. Kontakte: die Platte liegt auf der Decke und die Decke auf dem Lagertisch bei Raumtemperatur; die Hände des Trägers (32 degC) berühren 0,5 % der Platte."),
        T("No element draws power: the table-top region states the passive rating of 0 W for the whole asset.", "Kein Element nimmt Leistung auf: die Region der Platte nennt die passive Nennleistung 0 W für das ganze Objekt."),
    ]}


def binding(kind):
    """🌬 The region binding of one pose family."""
    base = {"legs": {"exposedFraction": 1, "sunlitFraction": 0}, "apron": {"exposedFraction": 1, "sunlitFraction": 0}, "shelf": {"exposedFraction": 1, "sunlitFraction": 0}, "depot-frame": {"exposedFraction": 1, "sunlitFraction": 0}}
    if kind == "stored":
        base.update({"table-top": {"exposedFraction": 0.53, "sunlitFraction": 0, "contact": {"fraction": 0.47, "tempC": 19}}, "blanket": {"exposedFraction": 0.13, "sunlitFraction": 0, "contact": {"fraction": 0.87, "tempC": 19}}, "depot-top": {"exposedFraction": 0.59, "sunlitFraction": 0, "contact": {"fraction": 0.41, "tempC": 19}}})
    elif kind == "carried":
        base.update({"table-top": {"exposedFraction": 0.995, "sunlitFraction": 0, "contact": {"fraction": 0.005, "tempC": 32}}, "blanket": {"exposedFraction": 0.51, "sunlitFraction": 0, "contact": {"fraction": 0.49, "tempC": 19}}, "depot-top": {"exposedFraction": 0.59, "sunlitFraction": 0, "contact": {"fraction": 0.41, "tempC": 19}}})
    else:
        base.update({"table-top": {"exposedFraction": 0.94, "sunlitFraction": 0.4, "contact": {"fraction": 0.06, "tempC": 24}}, "shelf": {"exposedFraction": 0.88, "sunlitFraction": 0.1, "contact": {"fraction": 0.12, "tempC": 19}}, "legs": {"exposedFraction": 1, "sunlitFraction": 0.05}, "apron": {"exposedFraction": 1, "sunlitFraction": 0.05}, "blanket": {"exposedFraction": 0.51, "sunlitFraction": 0, "contact": {"fraction": 0.49, "tempC": 19}}, "depot-top": {"exposedFraction": 0.59, "sunlitFraction": 0, "contact": {"fraction": 0.41, "tempC": 19}}})
    return {"regions": base, "emitters": {}}


DEPOT_C = list(DEPOT_FEET)
GRIP_C = list(GRIPS)
FEET_C = list(TABLE_FEET)
TRAY = {"id": "tray", "kind": "live", "title": T("Tray with coffee pot and cups on the top (6 kg)", "Tablett mit Kaffeekanne und Tassen auf der Platte (6 kg)"), "distributed": {"bone": "table", "fromM": 0.15, "toM": 0.45, "wxNPerM": 0, "wyNPerM": -6 * G / 0.30}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-1"}
MAGS = {"id": "magazines", "kind": "live", "title": T("Magazines and books on the lower shelf (8 kg)", "Zeitschriften und Bücher auf dem Zwischenboden (8 kg)"), "distributed": {"bone": "shelf", "fromM": 0.10, "toM": 0.42, "wxNPerM": 0, "wyNPerM": -8 * G / 0.32}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-1"}
EDGE = {"id": "corner-load", "kind": "imposed", "title": T("Vertical test load 50 mm inside the broken corner (750 N)", "Senkrechte Prüflast 50 mm innerhalb der abgebrochenen Ecke (750 N)"), "anchor": "corner-test", "fxN": 0, "fyN": -750, "source": "authored-estimate", "reference": "en-12521-2015"}

CARRIED = {"carry-lift": {"dy": CARRY_DY}, "carry-travel": {"dx": CARRY_DX}, "turn": {"angleDeg": 180}}
IN_USE = {"carry-lift": {"dy": USE_DY}, "carry-travel": {"dx": CARRY_DX}, "turn": {"angleDeg": 180}}


def pose(pid, title, desc, bones, contacts, kind, loads=(), key=False):
    """🧍 One pose."""
    p = {"id": pid, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "auto", "contacts": contacts, "thermal": binding(kind), "loads": list(loads)})
    return p


def poses():
    """🧍 Stored, carried, in use and the edge test, plus the key poses of the carry."""
    return [
        pose("stored", T("Stored upside down", "Kopfüber gelagert"), T("In the reuse depot the coffee table lies upside down on a folded moving blanket on a depot table; its legs point up, the price label sits on the shelf.", "Im Gebrauchtwarenlager liegt der Couchtisch kopfüber auf einer gefalteten Möbeldecke auf einem Lagertisch; die Beine zeigen nach oben, das Preisschild klebt am Zwischenboden."), {}, DEPOT_C, "stored"),
        pose("carried", T("Carried by one person", "Von einer Person getragen"), T("Lifted off, turned over and carried away by one person who holds it in front of the body by the two short ends under the top overhang; top at 0.85 m, blanket and depot table stay behind.", "Abgehoben, umgedreht und von einer Person weggetragen, die ihn vor dem Körper an den beiden Stirnseiten unter dem Plattenüberstand hält; Platte auf 0,85 m, Decke und Lagertisch bleiben zurück."), CARRIED, DEPOT_C + GRIP_C, "carried"),
        pose("in-use", T("In use in a living room", "In Gebrauch im Wohnzimmer"), T("Set up again on its four legs: a tray with coffee pot and cups on the sun-bleached top, magazines and books on the lower shelf.", "Wieder auf seinen vier Beinen aufgestellt: ein Tablett mit Kaffeekanne und Tassen auf der sonnengebleichten Platte, Zeitschriften und Bücher auf dem Zwischenboden."), IN_USE, DEPOT_C + FEET_C, "in-use", [TRAY, MAGS]),
        pose("edge-test-load", T("Edge test load at the broken corner", "Randprüflast an der abgebrochenen Ecke"), T("Standing in use with a vertical test load of 750 N 50 mm inside the broken corner, the check that the chipped corner is still safe to lean on.", "In Gebrauch stehend mit einer senkrechten Prüflast von 750 N 50 mm innerhalb der abgebrochenen Ecke, die Prüfung, dass die bestoßene Ecke noch belastbar ist."), IN_USE, DEPOT_C + FEET_C, "in-use", [EDGE]),
        pose("lifted-key", T("Lifted off the blanket (key)", "Von der Decke abgehoben (Schlüssel)"), T("Still upside down, lifted 0.10 m off the blanket by the two short ends.", "Noch kopfüber, an beiden Stirnseiten 0,10 m von der Decke abgehoben."), {"carry-lift": {"dy": 0.10}}, DEPOT_C + GRIP_C, "carried", key=True),
        pose("cleared-key", T("Cleared of the depot table (key)", "Vom Lagertisch weg (Schlüssel)"), T("Upside down, carried 1.10 m ahead so that it can be turned over beside the depot table.", "Kopfüber 1,10 m nach vorn getragen, damit er neben dem Lagertisch umgedreht werden kann."), {"carry-lift": {"dy": 0.10}, "carry-travel": {"dx": CARRY_DX}}, DEPOT_C + GRIP_C, "carried", key=True),
        pose("turning-key", T("Turning over (key)", "Beim Umdrehen (Schlüssel)"), T("Halfway through turning over about the grip axis, top standing on edge.", "Mitten im Umdrehen um die Griffachse, die Platte steht hochkant."), {"carry-lift": {"dy": 0.15}, "carry-travel": {"dx": CARRY_DX}, "turn": {"angleDeg": 90}}, DEPOT_C + GRIP_C, "carried", key=True),
    ]


def spans(anchors_, a, b):
    """🦶 Contact spans."""
    return [{"anchor": x, "fromS": a, "toS": b} for x in anchors_]


def clips():
    """🎞 Carry, set down and an evening in use."""
    return [
        {"id": "carry", "title": T("Carry out of the depot", "Aus dem Lager tragen"), "description": T("One person lifts the upside-down table off the blanket, walks it clear of the depot table, turns it over about the axis of the two hands and holds it in front of the body.", "Eine Person hebt den kopfüber liegenden Tisch von der Decke, trägt ihn vom Lagertisch weg, dreht ihn um die Achse der beiden Hände um und hält ihn vor dem Körper."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "lifted-key", "ease": "smooth"}, {"t": 2.4, "pose": "cleared-key", "ease": "smooth"}, {"t": 3.5, "pose": "turning-key", "ease": "smooth"}, {"t": 4.6, "pose": "carried", "ease": "smooth"}, {"t": 6, "pose": "carried", "ease": "smooth"}],
         "contacts": spans(DEPOT_C, 0, 6) + spans(GRIP_C, 0, 6),
         "phases": [{"id": "lift", "fromS": 0, "toS": 1.0, "title": T("Lift off", "Abheben")}, {"id": "clear", "fromS": 1.0, "toS": 2.4, "title": T("Walk clear", "Wegtragen")}, {"id": "turn-over", "fromS": 2.4, "toS": 4.6, "title": T("Turn over", "Umdrehen")}, {"id": "hold", "fromS": 4.6, "toS": 6, "title": T("Hold", "Halten")}],
         "events": [{"t": 0.2, "kind": "motion", "title": T("Top leaves the blanket", "Platte löst sich von der Decke")}, {"t": 3.5, "kind": "motion", "title": T("Top on edge", "Platte hochkant")}],
         "tracks": [{"target": "region:table-top:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.53}, {"t": 1.0, "v": 0.995}]}, {"target": "region:blanket:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.13}, {"t": 1.0, "v": 0.51}]}]},
        {"id": "set-down", "title": T("Set down in the living room", "Im Wohnzimmer abstellen"), "description": T("The carrier lowers the table from 0.85 m until its four legs stand on the floor.", "Der Träger senkt den Tisch von 0,85 m ab, bis seine vier Beine auf dem Boden stehen."), "durationS": 3, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "carried", "ease": "smooth"}, {"t": 2.6, "pose": "in-use", "ease": "smooth"}, {"t": 3, "pose": "in-use", "ease": "smooth"}],
         "contacts": spans(DEPOT_C, 0, 3) + spans(GRIP_C, 0, 2.8) + spans(FEET_C, 2.5, 3),
         "phases": [{"id": "lower", "fromS": 0, "toS": 2.6, "title": T("Lower", "Absenken")}, {"id": "release", "fromS": 2.6, "toS": 3, "title": T("Let go", "Loslassen")}],
         "events": [{"t": 2.6, "kind": "contact-on", "title": T("Legs on the floor", "Beine auf dem Boden"), "target": "table-foot-fl"}, {"t": 2.8, "kind": "contact-off", "title": T("Hands let go", "Hände lassen los"), "target": "grip-left"}],
         "tracks": [{"target": "load:tray", "ease": "step", "keys": [{"t": 0, "v": 0}]}, {"target": "load:magazines", "ease": "step", "keys": [{"t": 0, "v": 0}]}]},
        {"id": "evening-use", "title": T("An evening in use", "Ein Abend in Gebrauch"), "description": T("Back in a living room: the evening sun patch slides off the bleached top, a tray with coffee arrives, then magazines are put on the shelf.", "Zurück in einem Wohnzimmer: der Abendsonnenfleck wandert von der gebleichten Platte, ein Tablett mit Kaffee kommt, dann werden Zeitschriften auf den Boden gelegt."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-use", "ease": "linear"}, {"t": 8, "pose": "in-use", "ease": "linear"}],
         "contacts": spans(DEPOT_C, 0, 8) + spans(FEET_C, 0, 8),
         "phases": [{"id": "serve", "fromS": 0, "toS": 4, "title": T("Coffee is served", "Kaffee wird serviert")}, {"id": "read", "fromS": 4, "toS": 8, "title": T("Reading", "Lesen")}],
         "events": [{"t": 1.5, "kind": "load-on", "title": T("Tray set down", "Tablett abgestellt"), "target": "tray"}, {"t": 4.5, "kind": "load-on", "title": T("Magazines on the shelf", "Zeitschriften auf dem Boden"), "target": "magazines"}],
         "tracks": [{"target": "load:tray", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 0}, {"t": 2, "v": 1}]}, {"target": "load:magazines", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 0}, {"t": 5, "v": 1}]}, {"target": "region:table-top:sunlitFraction", "ease": "linear", "keys": [{"t": 0, "v": 0.4}, {"t": 8, "v": 0.1}]}]},
    ]


def structure_model():
    """🏗 Supports and assumptions."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": DEPOT_C, "clinical": False, "assumptions": [
        T("Stored: one rigid stack. The coffee table (22.6 kg) lies with its top on the folded blanket, the blanket on the 25 mm melamine top, which rests on the welded tube frame of the depot table and its four glides (all 52.6 kg on four contacts, friction 0.4 on the depot floor).", "Gelagert: ein starrer Stapel. Der Couchtisch (22,6 kg) liegt mit der Platte auf der gefalteten Decke, die Decke auf der 25-mm-Melaminplatte, diese auf dem geschweißten Rohrgestell des Lagertischs mit vier Gleitern (alle 52,6 kg auf vier Kontakten, Reibung 0,4 auf dem Lagerboden)."),
        T("Carried: one adult holds the table in front of the body with both hands under the 35 mm top overhang of the two short ends (grip height 0.84 m, about 11 kg per hand, within the 25 kg one person may carry close to the body per DGUV 208-033); the hands are compression contacts pushing up.", "Getragen: ein Erwachsener hält den Tisch vor dem Körper mit beiden Händen unter dem 35-mm-Plattenüberstand der Stirnseiten (Griffhöhe 0,84 m, etwa 11 kg je Hand, innerhalb der 25 kg, die eine Person nach DGUV 208-033 körpernah tragen kann); die Hände sind nach oben drückende Kontakte."),
        T("In use: the four 45 mm legs stand on the floor; tray 6 kg and magazines 8 kg (authored-estimate, below the order of household loads of EN 1991-1-1); edge test 750 N 50 mm inside the broken corner after the order of the vertical static load tests of EN 12521 / EN 1730 (magnitude an authored-estimate to be checked against the standard text). Legs tenoned into the aprons and screwed to the top through steel corner fittings; the shelf rests on dowels in the four legs.", "In Gebrauch: die vier 45-mm-Beine stehen auf dem Boden; Tablett 6 kg und Zeitschriften 8 kg (Schätzwerte, unter der Größenordnung der Haushaltslasten nach EN 1991-1-1); Randprüfung 750 N 50 mm innerhalb der abgebrochenen Ecke nach der Größenordnung der senkrechten statischen Lastprüfungen nach EN 12521 / EN 1730 (Betrag ein Schätzwert, am Normtext zu prüfen). Beine in die Zargen gezapft und über Stahl-Eckbeschläge mit der Platte verschraubt; der Boden liegt auf Dübeln in den vier Beinen."),
        T("Limitation of the solver: the whole asset is one instantaneously rigid body on springs of equal stiffness, so while carried or in use the depot table and the coffee table, which are then apart, share their reactions linearly over all active supports instead of each carrying its own weight; the stored pose, a resting stack, is exact.", "Grenze des Lösers: das ganze Objekt ist ein momentan starrer Körper auf gleich steifen Federn, getragen oder in Gebrauch teilen sich Lagertisch und Couchtisch, die dann getrennt sind, ihre Reaktionen deshalb linear über alle aktiven Auflager, statt dass jeder sein eigenes Gewicht trägt; die Lagerpose, ein ruhender Stapel, ist exakt."),
        T("Members: veneered P2 chipboard per EN 312 (E 1.7 GPa, 11 MPa), solid oak (E 11 GPa, 40 MPa), steel tube S235 per EN 10219 (sharp-cornered sections 3.04 and 2.24 cm2); masses from the modelled sections x density sum to 52.6 kg.", "Stäbe: furnierte Spanplatte P2 nach EN 312 (E 1,7 GPa, 11 MPa), massive Eiche (E 11 GPa, 40 MPa), Stahlrohr S235 nach EN 10219 (scharfkantige Querschnitte 3,04 und 2,24 cm2); die Massen aus modellierten Querschnitten x Dichte ergeben 52,6 kg."),
    ]}


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = [{"id": "grain-h", "title": T("Oak grain horizontal", "Eichenmaserung waagerecht"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13}, {"id": "grain-v", "title": T("Oak grain vertical", "Eichenmaserung senkrecht"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.5, "penMm": 0.13}, {"id": "quilt", "title": T("Quilting of the moving blanket", "Steppung der Möbeldecke"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.2, "penMm": 0.13}]


def decisions(scale):
    """📝 The drawing decisions of one scale."""
    D = {
        50: dict(
            silhouette=T("Stack 16 x 24 mm on paper: the coffee table upside down, legs up, on the folded dark blanket on the light depot table. Side elevation across the 0.60 m depth: top, end apron, the two near legs and the shelf edge between them; the depot top, its end rail and two tube legs with glides; the blanket with its rounded fold at the front. Front elevation along the 1.10 m length: the same with the long apron. Plan: the depot top, the blanket rim, the top with its broken corner, the aprons showing 5 mm beyond the shelf, the four leg ends and the notched shelf with its price label.", "Stapel 16 x 24 mm auf dem Papier: der Couchtisch kopfüber, Beine oben, auf der gefalteten dunklen Decke auf dem hellen Lagertisch. Seitenansicht über die 0,60 m Tiefe: Platte, Stirnzarge, die beiden vorderen Beine und dazwischen die Bodenkante; Lagertischplatte, Stirnzarge und zwei Rohrbeine mit Gleitern; die Decke mit gerundetem Falz vorn. Vorderansicht über die 1,10 m Länge: dasselbe mit der Längszarge. Grundriss: Lagertischplatte, Deckenrand, die Platte mit abgebrochener Ecke, die 5 mm über den Boden stehenden Zargen, die vier Beinenden und der ausgeklinkte Boden mit Preisschild."),
            retained=T("The 30 mm broken corner, the cup rings and scratches of the top, the scuffed leg ends, the 5 mm set-back of the aprons, the shelf notches, the glides, the quilting and the folded layers of the blanket.", "Die 30-mm-Ausbruchecke, die Wasserränder und Kratzer der Platte, die bestoßenen Beinenden, der 5-mm-Rücksprung der Zargen, die Ausklinkungen des Bodens, die Gleiter, die Steppung und die gefalteten Lagen der Decke."),
            merged=T("Far legs and far aprons coincide with the near ones in both elevations; the long aprons and depot rails seen end-on lie behind the legs.", "Hintere Beine und Zargen decken sich in beiden Ansichten mit den vorderen; die stirnseitig gesehenen Längszargen und Lagertischzargen liegen hinter den Beinen."),
            omitted=T("The 3 mm oak edging, the 0.6 mm veneer and the screws of the corner fittings, all under the 7.5 mm feature.", "Der 3-mm-Eichenumleimer, das 0,6-mm-Furnier und die Schrauben der Eckbeschläge, alle unter dem 7,5-mm-Merkmal."),
            thinElements=T("The 2 mm tube walls are modelled hollow and drawn as the outline of the tube; the 12 mm blanket is a filled strip of 0.24 mm.", "Die 2-mm-Rohrwände sind hohl modelliert und als Umriss des Rohrs gezeichnet; die 12-mm-Decke ist ein gefüllter Streifen von 0,24 mm."),
            hatching=T("Oak grain on the top edge, the aprons and the legs; cross quilting on the visible blanket rim.", "Eichenmaserung an der Plattenkante, den Zargen und Beinen; Kreuzsteppung am sichtbaren Deckenrand."),
            styles=T("Depot top 0.85, top 0.62, shelf 0.60, legs 0.55, aprons 0.50, depot frame 0.35, blanket 0.30.", "Lagertischplatte 0,85, Platte 0,62, Boden 0,60, Beine 0,55, Zargen 0,50, Lagertischgestell 0,35, Decke 0,30."),
            overlays=T("Every part carries its thermal region: top, legs, aprons, shelf, blanket, depot top and depot frame.", "Jedes Teil trägt seine Thermikregion: Platte, Beine, Zargen, Boden, Decke, Lagertischplatte und Lagertischgestell.")),
        100: dict(
            silhouette=T("Stack 8 x 12 mm: the same parts without texture of the edges: top with the broken corner in plan, legs, aprons, shelf, blanket with its fold, depot top, rail and legs.", "Stapel 8 x 12 mm: dieselben Teile ohne Kantentextur: Platte mit Ausbruchecke im Grundriss, Beine, Zargen, Boden, Decke mit Falz, Lagertischplatte, Zarge und Beine."),
            retained=T("The broken corner, the cup rings, the shelf notches and the aprons in plan.", "Die Ausbruchecke, die Wasserränder, die Ausklinkungen und die Zargen im Grundriss."),
            merged=T("Glides into the legs, tube walls into full bars.", "Gleiter in die Beine, Rohrwände zu vollen Stäben."),
            omitted=T("Grain and quilting hatches, scratches, scuffs, the price label and the folded layers.", "Maserungs- und Steppschraffuren, Kratzer, Bestoßungen, Preisschild und gefaltete Lagen."),
            thinElements=T("The 12 mm blanket is drawn as a 0.12 mm filled strip carried by its outline pen; it is a declared sheet in 3D.", "Die 12-mm-Decke ist ein gefüllter Streifen von 0,12 mm, den sein Umrissstrich trägt; in 3D ist sie ein deklariertes Blatt."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All seven regions.", "Alle sieben Regionen.")),
        200: dict(
            silhouette=T("Stack 4 x 6 mm: rectangular top, aprons as one ring, legs, shelf; blanket strip; depot top, rail ring and legs.", "Stapel 4 x 6 mm: rechteckige Platte, Zargen als ein Ring, Beine, Boden; Deckenstreifen; Lagertischplatte, Zargenring und Beine."),
            retained=T("Leg positions, apron band, shelf level, the blanket between the tables.", "Beinstellung, Zargenband, Bodenhöhe, die Decke zwischen den Tischen."),
            merged=T("The four aprons into a 30 mm ring, the four depot rails into a 30 mm ring.", "Die vier Zargen zu einem 30-mm-Ring, die vier Lagertischzargen zu einem 30-mm-Ring."),
            omitted=T("Broken corner, cup rings, fold rounding.", "Ausbruchecke, Wasserränder, Falzrundung."),
            thinElements=T("Top 25 mm, shelf 19 mm and depot top 25 mm stay at their true thickness and are carried by the body pen.", "Platte 25 mm, Boden 19 mm und Lagertischplatte 25 mm bleiben in echter Dicke und werden vom Körperstrich getragen."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All seven regions.", "Alle sieben Regionen.")),
        500: dict(
            silhouette=T("Stack sign 1.6 x 2.4 mm: the depot table as a top on four legs, the blanket strip, the coffee table as top, frame band with legs and shelf.", "Stapelzeichen 1,6 x 2,4 mm: der Lagertisch als Platte auf vier Beinen, der Deckenstreifen, der Couchtisch als Platte, Rahmenband mit Beinen und Boden."),
            retained=T("Heights of both tables, the inverted coffee table with legs up, shelf level.", "Höhen beider Tische, der umgedrehte Couchtisch mit Beinen nach oben, Bodenhöhe."),
            merged=T("Aprons and leg heads into one 75 mm frame band; legs widened inwards to 75 mm.", "Zargen und Beinköpfe zu einem 75-mm-Rahmenband; Beine nach innen auf 75 mm verbreitert."),
            omitted=T("Depot rails, glides, notches, every detail under 75 mm.", "Lagertischzargen, Gleiter, Ausklinkungen, jedes Detail unter 75 mm."),
            thinElements=T("The tops keep their true 25 mm; the 19 mm shelf and the 12 mm blanket are drawn and modelled as 22 mm strips (the blanket reaching into the depot top) so that with the 0.18 mm body pen they still print 0.2 mm wide at 1:1000.", "Die Platten behalten ihre echten 25 mm; der 19-mm-Boden und die 12-mm-Decke sind als 22-mm-Streifen gezeichnet und modelliert (die Decke reicht in die Lagertischplatte), damit sie mit dem 0,18-mm-Körperstrich bei 1:1000 noch 0,2 mm breit drucken."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50.", "Wie 1:50."),
            overlays=T("All seven regions.", "Alle sieben Regionen.")),
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
        "id": "used-coffee-table",
        "category": "furniture",
        "title": T("Used Coffee Table", "Gebrauchter Couchtisch"),
        "summary": T("A second-hand oak-veneered coffee table 1.10 x 0.60 x 0.45 m with a lower shelf, cup rings, a sun-bleached top and a broken-off corner, stored upside down on a moving blanket on a depot table, carried by one person at its short ends and set up again in a living room.", "Ein gebrauchter eichenfurnierter Couchtisch 1,10 x 0,60 x 0,45 m mit Zwischenboden, Wasserrändern, sonnengebleichter Platte und abgebrochener Ecke, kopfüber auf einer Möbeldecke auf einem Lagertisch gelagert, von einer Person an den Stirnseiten getragen und in einem Wohnzimmer wieder aufgestellt."),
        "tags": ["used", "household", "second-hand", "living", "table", "coffee-table", "oak", "veneer", "chipboard", "interior", "reuse-depot"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS], "notes": T(
            "Coffee table of the common German mid-range type, 1990s to 2000s: top 1100 x 600 x 25 mm oak-veneered P2 chipboard with solid edging, four 45 x 45 mm solid oak legs 425 mm long inset 30 mm, 20 x 70 mm aprons 5 mm behind the leg faces, a 19 mm veneered lower shelf at 111-130 mm notched round the legs; 22.6 kg. Traces of use: four cup rings and scratches in the lacquer, the top bleached on the window side, the front left corner broken off about 30 x 30 mm, scuffed leg ends, a depot price label on the shelf. Storage aids: a 1200 x 800 x 740 mm depot table (25 mm melamine top, welded 40 x 40 x 2 and 40 x 20 x 2 mm tube frame, 28.6 kg) and a moving blanket folded to 12 mm (1.3 kg). Masses are modelled section x density: 660 kg/m3 veneered chipboard, 700 oak, 650 melamine chipboard, 7850 steel, 134 felt.",
            "Couchtisch der gängigen deutschen Mittelklasse der 1990er- bis 2000er-Jahre: Platte 1100 x 600 x 25 mm aus eichenfurnierter Spanplatte P2 mit Massivumleimer, vier Beine aus massiver Eiche 45 x 45 mm, 425 mm lang und 30 mm eingerückt, Zargen 20 x 70 mm 5 mm hinter den Beinflächen, ein 19-mm-Zwischenboden auf 111-130 mm, um die Beine ausgeklinkt; 22,6 kg. Gebrauchsspuren: vier Wasserränder und Kratzer im Lack, die Platte auf der Fensterseite ausgebleicht, die vordere linke Ecke etwa 30 x 30 mm abgebrochen, bestoßene Beinenden, ein Preisschild des Lagers am Boden. Lagerhilfen: ein Lagertisch 1200 x 800 x 740 mm (25-mm-Melaminplatte, geschweißtes Rohrgestell 40 x 40 x 2 und 40 x 20 x 2 mm, 28,6 kg) und eine auf 12 mm gefaltete Möbeldecke (1,3 kg). Massen aus Querschnitt x Dichte: 660 kg/m3 furnierte Spanplatte, 700 Eiche, 650 Melamin-Spanplatte, 7850 Stahl, 134 Filz.")},
        "dimensions": {"heightM": TOP_Y + COFFEE["H"], "widthM": 2 * DEPOT["x"], "depthM": 2 * DEPOT["z"], "massKg": total},
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
    print("[DEBUG] areas", {k: round(v, 4) for k, v in areas().items()})
    print("[DEBUG] mass", round(sum(v[0] for k, v in masses().items() if k != "_parts"), 3))
