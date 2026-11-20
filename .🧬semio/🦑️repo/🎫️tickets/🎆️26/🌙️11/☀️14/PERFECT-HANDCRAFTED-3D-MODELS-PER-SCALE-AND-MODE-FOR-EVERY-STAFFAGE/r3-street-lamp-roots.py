"""🌱 Round-3 root fixes of `objects/street-lamp` outside `volumes`: rig (arm and luminaire to real size), dimensions, plume anchor, luminaire references, and every 2D drawing of every scale redrawn as the true view of the modelled body (ruling E1) from the same dimensions as the solids (`r3-street-lamp-kit.py`).

Usage: `python3 r3-street-lamp-roots.py [--write]` — without `--write` the candidate goes to `🗑️generated/r3-street-lamp/🔣️.json` (check it with `r3-validate.ts --file`); with `--write` the catalog file is re-read and rewritten.
@see ./r3-street-lamp-kit.py
@see ./r3-street-lamp-model.py
@see ./r3-objects-street-lamp.md
"""
import copy
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/street-lamp/🔣️.json"


def load(name, file):
    """📦 Imports a sibling script by file name."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = load("kit", "r3-street-lamp-kit.py")
M = load("model", "r3-street-lamp-model.py")
r6 = K.r6
GEOM = ("x", "y", "w", "h", "r", "d", "points", "closed", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "followTip")
AY = K.ARM_Y
HX = K.HEAD_U


def num(v):
    """🔢 A plain number."""
    return isinstance(v, (int, float)) and not isinstance(v, bool)


def inline(v):
    """📏 Lists printed on one line: numbers, and short lists of number pairs."""
    return all(num(i) for i in v) or (all(isinstance(i, list) and all(num(x) for x in i) for i in v) and len(json.dumps(v)) <= 160)


def dump(value, depth=0):
    """🖨 The catalog's JSON style (two-space indent, number lists inline)."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(i, depth + 1) for k, i in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(i, depth + 1) for i in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def pts(points):
    """📍 Rounded point list."""
    return [[r6(a), r6(b)] for a, b in points]


def geom(shape, kind, **fields):
    """✏ Replaces the geometry of a shape, keeping id, bone, role, z, tone, region, material, pen and hatch; `keep` lists geometry members kept (followTip)."""
    keep = fields.pop("keep", ())
    for k in GEOM:
        if k not in keep:
            shape.pop(k, None)
    shape.pop("d", None)
    shape["kind"] = kind
    for k, v in fields.items():
        shape[k] = pts(v) if k == "points" else (r6(v) if num(v) else v)
    if kind == "poly":
        shape.setdefault("closed", True)
    order = [k for k in shape if k not in GEOM and k != "kind"]
    rest = {k: shape[k] for k in shape if k in GEOM}
    out = {k: shape[k] for k in order}
    out["kind"] = kind
    for k in ("x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "points", "closed", "followTip"):
        if k in rest:
            out[k] = rest[k]
    shape.clear()
    shape.update(out)


def mirror(profile, flip=False):
    """🪞 A closed outline from an (axial, radial) half profile: the -r side forward and the +r side back."""
    lo = [(a, -r) for a, r in profile]
    hi = [(a, r) for a, r in reversed(profile)]
    out = lo + hi
    return [(b, a) for a, b in out] if flip else out


def drop(shapes, *ids):
    """🗑 Removes shapes by id."""
    shapes[:] = [s for s in shapes if s["id"] not in ids]


def find(shapes, sid):
    """🔍 The shape with this id."""
    for s in shapes:
        if s["id"] == sid:
            return s
    raise KeyError(sid)


def add_after(shapes, after, shape):
    """➕ Inserts a shape after another (or replaces one with the same id)."""
    drop(shapes, shape["id"])
    i = next(k for k, s in enumerate(shapes) if s["id"] == after)
    shapes.insert(i + 1, shape)


def ring(points, tol):
    """✂ Simplified closed outline."""
    return K.simplify([(r6(a), r6(b)) for a, b in points], tol)


def head_views(rows_n, plan_rows, side_rows, grow, tol):
    """🔦 Elevation (head-local), front (world z, y) and plan (world x, lateral) outlines of a luminaire built from lofts."""
    side = ring(K.side_outline(side_rows), tol)
    front = ring([(z, AY + v) for z, v in K.front_outline(rows_n)], tol)
    plan = ring(K.plan_outline(plan_rows, HX, grow), tol)
    return side, front, plan


def circle(r, n=24, cx=0.0, cy=0.0):
    """⚪ Circle points."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def clamp_plan():
    """🗜 Plan of the clamp sleeve (circle r 50 mm) with its bolting ear at the back."""
    e = math.asin(0.014 / K.CLAMP_R)
    arc = [(K.CLAMP_R * math.cos(math.pi + e + (2 * math.pi - 2 * e) * k / 22), K.CLAMP_R * math.sin(math.pi + e + (2 * math.pi - 2 * e) * k / 22)) for k in range(23)]
    return arc + [(-0.068, 0.014), (-0.068, -0.014)]


POLE_50 = [(0, K.POLE_R0), (K.TUBE_TOP, K.pole_r(K.TUBE_TOP))]
POLE_100 = [(0, K.POLE_R0), (5.858, K.pole_r(5.858)), (5.872, 0.034), (5.878, 0.022), (5.88, 0.0)]
POLE_200 = [(0, K.POLE_R0), (5.865, K.pole_r(5.865)), (5.88, 0.026)]
POLE_500 = [(0, K.POLE_R0), (5.88, K.POLE_R1)]
CAP_HALF = K.CAP[1:-1] + [K.CAP[-1]]
ARM_50 = [(0.035, -K.ARM_R), (0.040, -0.038), (0.080, -0.038), (0.092, -0.0322), (K.ARM_L, -K.ARM_R), (K.ARM_L, K.ARM_R), (0.092, 0.0322), (0.080, 0.038), (0.040, 0.038), (0.035, K.ARM_R)]
ARM_100 = [(0.04, -K.ARM_R), (K.ARM_L, -K.ARM_R), (K.ARM_L, K.ARM_R), (0.04, K.ARM_R)]
CLAMP_SIDE = [(-0.05, -0.05), (0.05, -0.05), (0.05, 0.05), (-0.05, 0.05), (-0.05, 0.04), (-0.068, 0.04), (-0.068, -0.04), (-0.05, -0.04)]
LENS_V = K.LENS_Y0


def cap_outline(dy=0.0, front=False):
    """🎩 Side (pole-local u, v) or front (z, y) outline of the push-in dome cap."""
    half = [(a, r) for a, r in K.CAP[1:]]
    out = [(a, -r) for a, r in half] + [(a, r) for a, r in reversed(half[:-1])]
    return [(b, a + dy) for a, b in out] if front else out


def pole_outline(profile, dy=0.0, front=False):
    """🗼 Side (u, v) or front (z, y) outline of the conical column from its half profile (the apex point on the axis kept once)."""
    lo = [(a, -r) for a, r in profile]
    hi = [(a, r) for a, r in reversed(profile) if r > 0]
    out = lo + hi
    if profile[-1][1] == 0:
        out = [p for p in out if not (p[1] == 0 and p is not lo[-1])]
    return [(b, a + dy) for a, b in out] if front else out


def nut_side(c, half):
    """🔩 A nut with washer and bolt end seen from the side."""
    return [(c - half, K.NUT_Y0), (c + half, K.NUT_Y0), (c + half, K.NUT_Y1), (c + K.STUB_R, K.NUT_Y1), (c + K.STUB_R, K.STUB_Y1), (c - K.STUB_R, K.STUB_Y1), (c - K.STUB_R, K.NUT_Y1), (c - half, K.NUT_Y1)]


def plan_order(pl):
    """🪜 Plan layer order by real height: the arm (5.8 m) and its clamp lie over the column, band, shroud and plinth."""
    for s in pl:
        if s["id"] == "arm":
            s["z"] = 3.6
        if s["id"] == "arm-clamp":
            s["z"] = 3.7
    pl.sort(key=lambda s: s["z"])


def redraw_50(rep):
    """✒ 1:50: every part as built, details redrawn where the form moved."""
    g = rep["geometry"]
    el, fr, pl = g["elevation"], g["front"], g["plan"]
    tol = 0.0018
    side, front, plan = head_views([(K.cover_rows(), K.COVER_N), (K.tray_rows(), K.TRAY_N)], K.HEAD, K.HEAD, K.TRAY_GROW, tol)
    drop(el, "plinth-joint", "pole-seam-0", "pole-seam-1", "lens-reflector")
    drop(fr, "plinth-joint", "pole-seam-0", "pole-seam-1", "head-spigot", "head-fin-1")
    geom(find(el, "base-plate"), "rect", x=-0.15, y=0.1, w=0.3, h=0.02, r=0.004)
    geom(find(el, "base-nut-0"), "poly", points=nut_side(-K.BOLT_C, K.NUT_AF / 2))
    geom(find(el, "base-nut-1"), "poly", points=nut_side(K.BOLT_C, K.NUT_AF / 2))
    geom(find(el, "base-cover"), "poly", points=mirror(M.COVER_PROFILE, True))
    geom(find(el, "pole"), "poly", points=pole_outline(POLE_50))
    geom(find(el, "pole-collar"), "rect", x=K.BAND_U0, y=-K.BAND_R, w=K.BAND_U1 - K.BAND_U0, h=2 * K.BAND_R)
    geom(find(el, "pole-cap"), "poly", points=cap_outline())
    geom(find(el, "pole-hatch"), "rect", x=K.DOOR_U0, y=-K.DOOR_HALF, w=K.DOOR_U1 - K.DOOR_U0, h=2 * K.DOOR_HALF, r=0.008)
    geom(find(el, "pole-highlight"), "poly", points=[(0.4, 0.062), (5.6, 0.033)], closed=False)
    geom(find(el, "arm"), "poly", points=ARM_50)
    geom(find(el, "arm-brace"), "capsule", x0=K.BRACE_FOOT[0], y0=K.BRACE_FOOT[1], x1=K.BRACE_TOP[0], y1=K.BRACE_TOP[1], r0=K.BRACE_R, r1=K.BRACE_R)
    geom(find(el, "arm-brace-joint"), "poly", points=M.LUG)
    geom(find(el, "arm-clamp"), "poly", points=CLAMP_SIDE)
    lug = find(el, "arm-brace-joint")
    add_after(el, "arm-brace-joint", dict({k: v for k, v in lug.items() if k not in GEOM and k != "kind"}, id="arm-brace-foot", region="pole-galv", material="galv-steel", kind="poly", points=pts(K.FOOT_LUG), closed=True))
    bolt = {"bone": "arm", "role": "detail", "z": 2.25, "filled": True, "tone": 0.2, "region": "arm-alu", "material": "galv-steel", "penMm": 0.13, "kind": "poly"}
    hexa = K.hexagon(K.EAR_BOLT_AF)
    for i, v in enumerate((K.EAR_BOLT_V, -K.EAR_BOLT_V)):
        add_after(el, "arm-clamp", dict(bolt, id=f"arm-clamp-bolt-{i}", points=pts([(K.EAR_X + a, v + b) for a, b in hexa]), closed=True))
    geom(find(el, "arm-seam"), "poly", points=[(0.11, 0.013), (0.70, 0.013)], closed=False)
    geom(find(el, "head-housing"), "poly", points=side)
    geom(find(el, "head-spigot"), "rect", x=-0.03, y=-0.038, w=0.042, h=0.076, r=0.004)
    t17 = K.head_at(0.17)[1]
    geom(find(el, "head-driver"), "poly", points=[(0.17, t17), (0.17, K.RIM_V)], closed=False)
    for i, u in enumerate(K.FIN_U):
        t = K.head_at(u)[1]
        geom(find(el, f"head-fin-{i}"), "poly", points=[(u, t), (u, K.fin_top(u, 0.0))], closed=False)
        find(el, f"head-fin-{i}")["penMm"] = 0.13
    geom(find(el, "head-rim"), "poly", points=[(0.012, K.RIM_V), (0.497, K.RIM_V)], closed=False)
    lens = find(el, "lens")
    lens["filled"] = False
    geom(lens, "poly", points=[(K.LENS_U0, LENS_V), (K.LENS_U1, LENS_V)], closed=False)

    geom(find(fr, "base-plate"), "rect", x=-0.15, y=0.1, w=0.3, h=0.02, r=0.004)
    corner = K.NUT_AF / math.sqrt(3)
    geom(find(fr, "base-nut-0"), "poly", points=nut_side(-K.BOLT_C, corner))
    geom(find(fr, "base-nut-1"), "poly", points=nut_side(K.BOLT_C, corner))
    geom(find(fr, "base-cover"), "poly", points=mirror(M.COVER_PROFILE, True))
    geom(find(fr, "pole"), "poly", points=pole_outline(POLE_50, K.PLATE_TOP, True), followTip=True, keep=())
    geom(find(fr, "pole-highlight"), "poly", points=[(0.056, 0.52), (0.034, 5.72)], closed=False)
    geom(find(fr, "pole-collar"), "rect", x=-K.BAND_R, y=K.PLATE_TOP + K.BAND_U0, w=2 * K.BAND_R, h=K.BAND_U1 - K.BAND_U0)
    geom(find(fr, "pole-cap"), "poly", points=cap_outline(K.PLATE_TOP, True))
    geom(find(fr, "arm"), "ellipse", cx=0.0, cy=AY, rx=K.ARM_R, ry=K.ARM_R)
    geom(find(fr, "arm-clamp"), "rect", x=-K.CLAMP_R, y=AY - K.CLAMP_H / 2, w=2 * K.CLAMP_R, h=K.CLAMP_H)
    geom(find(fr, "arm-brace"), "capsule", x0=0.0, y0=AY + K.BRACE_FOOT[1], x1=0.0, y1=AY + K.BRACE_TOP[1], r0=K.BRACE_R, r1=K.BRACE_R)
    geom(find(fr, "head-housing"), "poly", points=front)
    prof = K.fin_profile(K.PLUME_U)
    fin_front = [(K.FIN_Z, AY + K.head_at(K.PLUME_U)[1] - 0.012)] + [(w, AY + v) for v, w in prof[1:-1]] + [(-K.FIN_Z, AY + K.head_at(K.PLUME_U)[1] - 0.012)]
    geom(find(fr, "head-fin-0"), "poly", points=fin_front, closed=False)
    lensf = find(fr, "lens")
    lensf["filled"] = False
    geom(lensf, "poly", points=[(-K.LENS_Z, AY + LENS_V), (K.LENS_Z, AY + LENS_V)], closed=False)

    geom(find(pl, "base-plate"), "rect", x=-0.15, y=-0.15, w=0.3, h=0.3, r=0.004)
    hexa = K.hexagon(K.NUT_AF)
    for i, (cx, cy) in enumerate(((-K.BOLT_C, -K.BOLT_C), (K.BOLT_C, -K.BOLT_C), (-K.BOLT_C, K.BOLT_C), (K.BOLT_C, K.BOLT_C))):
        geom(find(pl, f"base-nut-{i}"), "poly", points=[(cx + b, cy + a) for a, b in hexa])
    pc = find(pl, "pole-collar")
    pc["bone"] = "pole"
    geom(pc, "ellipse", cx=0.0, cy=0.0, rx=K.BAND_R, ry=K.BAND_R)
    geom(find(pl, "arm"), "poly", points=ARM_50)
    geom(find(pl, "arm-clamp"), "poly", points=clamp_plan())
    half = K.EAR_BOLT_AF / math.sqrt(3)
    plan_order(pl)
    add_after(pl, "arm-clamp", dict({k: v for k, v in pc.items() if k not in GEOM and k != "kind"}, id="pole-cap", role="body", z=3.8, kind="ellipse", cx=0.0, cy=0.0, rx=0.044, ry=0.044))
    add_after(pl, "arm-clamp", dict(bolt, id="arm-clamp-bolt", z=3.65, points=pts([(K.EAR_X - half, -K.EAR_BOLT_Z), (K.EAR_X + half, -K.EAR_BOLT_Z), (K.EAR_X + half, K.EAR_BOLT_Z), (K.EAR_X - half, K.EAR_BOLT_Z)]), closed=True))
    geom(find(pl, "head-housing"), "poly", points=plan)
    geom(find(pl, "head-spigot"), "rect", x=HX - 0.03, y=-0.038, w=0.042, h=0.076, r=0.004)
    w17 = K.head_at(0.17)[0] + K.TRAY_GROW
    geom(find(pl, "head-driver"), "poly", points=[(HX + 0.17, -w17 + 0.004), (HX + 0.17, w17 - 0.004)], closed=False)
    for i, u in enumerate(K.FIN_U):
        geom(find(pl, f"head-fin-{i}"), "poly", points=[(HX + u, -K.FIN_Z), (HX + u, K.FIN_Z)], closed=False)
    drop(pl, "head-rim")


def redraw_100(rep):
    """✒ 1:100: plate in the plinth, solid column with dome, clamp sleeve, arm, strut stroke, housing and lens line."""
    g = rep["geometry"]
    el, fr, pl = g["elevation"], g["front"], g["plan"]
    tol = 0.0035
    rows = [K.HEAD[i] for i in M.SUB_100]
    side, front, plan = head_views([(K.cover_rows(rows), 2.6), (K.tray_rows(rows), K.TRAY_N)], rows, rows, K.TRAY_GROW, tol)
    drop(el, "pole-collar")
    drop(fr, "pole-collar")
    drop(pl, "pole-collar", "head-rim")
    geom(find(el, "base-cover"), "poly", points=mirror(M.COVER_100, True))
    geom(find(el, "pole"), "poly", points=pole_outline(POLE_100))
    geom(find(el, "arm"), "poly", points=ARM_100)
    geom(find(el, "arm-brace"), "poly", points=[K.BRACE_FOOT, K.BRACE_TOP], closed=False)
    clamp = {"id": "arm-clamp", "bone": "arm", "role": "detail", "z": 2.2, "filled": True, "tone": 0.3, "region": "arm-alu", "material": "aluminium-anodised"}
    add_after(el, "arm-brace", dict(clamp, kind="rect", x=-0.05, y=-0.05, w=0.1, h=0.1))
    geom(find(el, "head-housing"), "poly", points=side)
    lens = find(el, "lens")
    lens["filled"] = False
    geom(lens, "poly", points=[(K.LENS_U0, LENS_V), (K.LENS_U1, LENS_V)], closed=False)
    geom(find(fr, "base-cover"), "poly", points=mirror(M.COVER_100, True))
    geom(find(fr, "pole"), "poly", points=pole_outline(POLE_100, K.PLATE_TOP, True), followTip=True)
    geom(find(fr, "arm"), "ellipse", cx=0.0, cy=AY, rx=K.ARM_R, ry=K.ARM_R)
    geom(find(fr, "arm-brace"), "poly", points=[(0.0, AY + K.BRACE_FOOT[1]), (0.0, AY + K.BRACE_TOP[1])], closed=False)
    add_after(fr, "arm-brace", dict(clamp, kind="rect", x=-0.05, y=AY - 0.05, w=0.1, h=0.1))
    geom(find(fr, "head-housing"), "poly", points=front)
    lensf = find(fr, "lens")
    lensf["filled"] = False
    geom(lensf, "poly", points=[(-K.LENS_Z, AY + LENS_V), (K.LENS_Z, AY + LENS_V)], closed=False)
    geom(find(pl, "arm"), "poly", points=ARM_100)
    add_after(pl, "arm", dict(clamp, kind="ellipse", cx=0.0, cy=0.0, rx=K.CLAMP_R, ry=K.CLAMP_R))
    plan_order(pl)
    geom(find(pl, "head-housing"), "poly", points=plan)


def redraw_200(rep):
    """✒ 1:200: plinth, solid column, arm and strut strokes, one housing outline, lens stroke."""
    g = rep["geometry"]
    el, fr, pl = g["elevation"], g["front"], g["plan"]
    tol = 0.007
    rows = [K.HEAD[i] for i in M.SUB_200]
    side, front, plan = head_views([(K.single_rows(rows), 3.4)], rows, rows, K.TRAY_GROW, tol)
    geom(find(el, "pole"), "poly", points=pole_outline(POLE_200))
    geom(find(el, "arm"), "poly", points=[(0.0, 0.0), (K.ARM_L, 0.0)], closed=False)
    geom(find(el, "arm-brace"), "poly", points=[K.BRACE_FOOT, K.BRACE_TOP], closed=False)
    geom(find(el, "head-housing"), "poly", points=side)
    geom(find(el, "lens"), "poly", points=[(K.LENS_U0, -0.054), (K.LENS_U1, -0.054)], closed=False)
    geom(find(fr, "pole"), "poly", points=pole_outline(POLE_200, K.PLATE_TOP, True), followTip=True)
    geom(find(fr, "arm"), "ellipse", cx=0.0, cy=AY, rx=K.ARM_R, ry=K.ARM_R)
    geom(find(fr, "arm-brace"), "poly", points=[(0.0, AY + K.BRACE_FOOT[1]), (0.0, AY + K.BRACE_TOP[1])], closed=False)
    geom(find(fr, "head-housing"), "poly", points=front)
    geom(find(fr, "lens"), "poly", points=[(-K.LENS_Z, AY - 0.054), (K.LENS_Z, AY - 0.054)], closed=False)
    geom(find(pl, "pole"), "capsule", x0=0.0, y0=0.0, x1=0.0, y1=0.0, r0=K.POLE_R0, r1=K.POLE_R1, followTip=True)
    geom(find(pl, "arm"), "poly", points=[(0.0, 0.0), (K.ARM_L, 0.0)], closed=False)
    geom(find(pl, "head-housing"), "poly", points=plan)
    plan_order(pl)
    drop(pl, "lens")


def redraw_500(rep):
    """✒ 1:500: plinth, true cone, arm stroke, the luminaire as one flat head."""
    g = rep["geometry"]
    el, fr, pl = g["elevation"], g["front"], g["plan"]
    tol = 0.012
    side, front, plan = head_views([(K.single_rows(M.HEAD_500, 0.004), 3.2)], M.HEAD_500, M.HEAD_500, 0.004, tol)
    geom(find(el, "pole"), "poly", points=pole_outline(POLE_500))
    geom(find(el, "head"), "poly", points=side)
    geom(find(fr, "pole"), "poly", points=pole_outline(POLE_500, K.PLATE_TOP, True), followTip=True)
    geom(find(fr, "head"), "poly", points=front)
    geom(find(pl, "pole"), "capsule", x0=0.0, y0=0.0, x1=0.0, y1=0.0, r0=K.POLE_R0, r1=K.POLE_R1, followTip=True)
    geom(find(pl, "head"), "poly", points=plan)
    plan_order(pl)


def t(en, de):
    """🌐 Bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": {
        "silhouette": t("Side elevation 27.5 x 120 mm: concrete plinth, base plate with two of the four M20 nuts and bolt ends, the flared base shroud, the straight 5.87 m cone (3.4 mm at the foot, 1.6 mm at the top) with inspection door, brace band and push-in dome cap, the 60 mm aluminium bracket arm in its clamp sleeve (bolting ear with two M10 bolts at the back, socket), the 27 mm strut between its foot lug on the band and its gusset lug under the arm, and the 0.505 m side-entry LED luminaire: domed cover over the flat tray, spigot collar, parting line, gear-hatch joint, six cooling fins and the flat lens line underneath. Front view: plinth, plate, nuts, shroud, the tapering column with highlight, band and cap, the arm end-on in its clamp, the strut end-on, the luminaire section with its fin edge and lens line. Plan view: stippled plinth, plate with four hexagon nuts, shroud, column, brace band, clamp sleeve with ear and bolt, the cap, arm with socket, and the luminaire outline with spigot collar, gear-hatch joint and six fins; arm and head follow the sway.", "Seitenansicht 27,5 x 120 mm: Betonsockel, Fußplatte mit zwei der vier Muttern M20 und Schraubenenden, die ausgestellte Fußmanschette, der gerade Kegel von 5,87 m (3,4 mm am Fuß, 1,6 mm oben) mit Revisionstür, Strebenring und eingesteckter Kuppelkappe, der Aluminiumausleger von 60 mm in seiner Klemmhülse (Schraublasche mit zwei Schrauben M10 hinten, Muffe), die Strebe von 27 mm zwischen Fußlasche am Ring und Knotenblech unter dem Arm und die LED-Leuchte von 0,505 m mit seitlicher Einführung: gewölbte Haube über der flachen Wanne, Zapfenkragen, Teilungsfuge, Fuge der Geräteklappe, sechs Kühlrippen und die flache Abdeckungslinie unten. Vorderansicht: Sockel, Platte, Muttern, Manschette, der verjüngte Mast mit Glanzlinie, Ring und Kappe, der Arm stirnseitig in seiner Klemme, die Strebe stirnseitig, der Leuchtenquerschnitt mit Rippenkante und Abdeckungslinie. Draufsicht: gepunkteter Sockel, Platte mit vier Sechskantmuttern, Manschette, Mast, Strebenring, Klemmhülse mit Lasche und Schraube, die Kappe, Arm mit Muffe und der Leuchtenumriss mit Zapfenkragen, Klappenfuge und sechs Rippen; Arm und Kopf folgen dem Wiegen."),
        "retained": t("Every assembly of the real column: plinth, plate, nuts and bolt ends, shroud, cone, door, band, cap, clamp sleeve with ear and bolts, socket, arm, strut, both lugs, luminaire cover and tray, spigot collar, parting line, hatch joint, fins and lens. Front view: nuts, highlight, band, cap, fin edge, lens line. Plan view: nuts, band, clamp ear, socket, hatch joint, six fins.", "Jede Baugruppe des echten Masts: Sockel, Platte, Muttern und Schraubenenden, Manschette, Kegel, Tür, Ring, Kappe, Klemmhülse mit Lasche und Schrauben, Muffe, Arm, Strebe, beide Laschen, Haube und Wanne der Leuchte, Zapfenkragen, Teilungsfuge, Klappenfuge, Rippen und Abdeckung. Vorderansicht: Muttern, Glanzlinie, Ring, Kappe, Rippenkante, Abdeckungslinie. Draufsicht: Muttern, Ring, Klemmlasche, Muffe, Klappenfuge, sechs Rippen."),
        "merged": t("Washer and nut are one outline; the door is a line on the cone (it is flush).", "Scheibe und Mutter sind ein Umriss; die Tür ist eine Linie auf dem Kegel (sie liegt bündig)."),
        "omitted": t("Door lock (below the minimum feature); the cable and the anchor bolts inside the plinth; the column has no circumferential seams (a 6 m cone is one piece). Front view: the five rear fins hide behind the first fin edge, the spigot collar behind the housing.", "Türschloss (unter dem Mindestmaß); das Kabel und die Ankerschrauben im Sockel; der Mast hat keine Rundnähte (ein Kegel von 6 m ist ein Stück). Vorderansicht: die fünf hinteren Rippen verschwinden hinter der ersten Rippenkante, der Zapfenkragen hinter dem Gehäuse."),
        "thinElements": t("Highlight, parting line, hatch joint, fin marks and the lens line are single 0.13-0.25 mm strokes (the flat lens stands 4.5 mm proud, 0.09 mm on paper); the 27 mm strut is a 0.54 mm capsule; nuts are 0.6 x 0.6 mm outlines with their bolt ends.", "Glanzlinie, Teilungsfuge, Klappenfuge, Rippenmarken und die Abdeckungslinie sind einzelne Striche von 0,13-0,25 mm (die flache Abdeckung steht 4,5 mm vor, 0,09 mm auf dem Papier); die Strebe von 27 mm ist eine Kapsel von 0,54 mm; Muttern sind Umrisse von 0,6 x 0,6 mm mit ihren Schraubenenden."),
    },
    "100": {
        "silhouette": t("Side elevation 13.75 x 60 mm: plinth with the plate merged, the flared shroud, the straight cone (1.7 mm to 0.8 mm) with its dome, the clamp sleeve, the 60 mm arm, the strut as a stroke, the luminaire (cover and tray) with its lens line. Front view: plinth, shroud, tapering column, clamp sleeve, the arm end-on, the strut as a vertical stroke, the luminaire section with the lens line. Plan view: plinth square, round shroud, the column, clamp sleeve, arm and the luminaire outline.", "Seitenansicht 13,75 x 60 mm: Sockel mit eingeschmolzener Platte, die ausgestellte Manschette, der gerade Kegel (1,7 mm auf 0,8 mm) mit Kuppel, die Klemmhülse, der Arm von 60 mm, die Strebe als Strich, die Leuchte (Haube und Wanne) mit Abdeckungslinie. Vorderansicht: Sockel, Manschette, verjüngter Mast, Klemmhülse, der Arm stirnseitig, die Strebe als senkrechter Strich, der Leuchtenquerschnitt mit Abdeckungslinie. Draufsicht: Sockelquadrat, runde Manschette, der Mast, Klemmhülse, Arm und der Leuchtenumriss."),
        "retained": t("Plinth, shroud, cone with dome, clamp sleeve, arm, strut, luminaire outline, lens line. Front view: clamp, strut, lens line. Plan view: clamp sleeve.", "Sockel, Manschette, Kegel mit Kuppel, Klemmhülse, Arm, Strebe, Leuchtenumriss, Abdeckungslinie. Vorderansicht: Klemme, Strebe, Abdeckungslinie. Draufsicht: Klemmhülse."),
        "merged": t("Base plate, nuts and bolt ends merge into the plinth top (0.2 mm); cap and cone are one outline; the socket merges into the arm, spigot collar and parting line into the housing outline.", "Fußplatte, Muttern und Schraubenenden gehen in der Sockeloberkante auf (0,2 mm); Kappe und Kegel sind ein Umriss; die Muffe geht im Arm auf, Zapfenkragen und Teilungsfuge im Gehäuseumriss."),
        "omitted": t("Door, highlight, brace band (8 mm proud), clamp ear, gusset lug, fins and hatch joint (below 15 mm). Front and plan view: plate, nuts, fins.", "Tür, Glanzlinie, Strebenring (8 mm Überstand), Klemmlasche, Knotenblech, Rippen und Klappenfuge (unter 15 mm). Vorder- und Draufsicht: Platte, Muttern, Rippen."),
        "thinElements": t("The 27 mm strut is a single 0.18 mm stroke; the flat lens a glazing stroke under the tray; the 60 mm arm a 0.6 mm solid band.", "Die Strebe von 27 mm ist ein einzelner Strich von 0,18 mm; die flache Abdeckung ein Glasstrich unter der Wanne; der Arm von 60 mm ein Band von 0,6 mm."),
    },
    "200": {
        "silhouette": t("Side elevation 6.9 x 30 mm: a plinth block, the straight cone (0.85 mm to 0.4 mm) with a rounded top, the arm stroke, the strut hairline, the flat luminaire with a lens stroke under it. Front view: plinth and tapering column, the arm end-on as a dot, the strut as a vertical line, the luminaire section with its lens line. Plan view: plinth, the column, the arm line and the luminaire outline.", "Seitenansicht 6,9 x 30 mm: ein Sockelblock, der gerade Kegel (0,85 mm auf 0,4 mm) mit gerundetem Kopf, der Armstrich, die Strebenhaarlinie, die flache Leuchte mit einem Abdeckungsstrich darunter. Vorderansicht: Sockel und verjüngter Mast, der Arm stirnseitig als Punkt, die Strebe als senkrechte Linie, der Leuchtenquerschnitt mit Abdeckungslinie. Draufsicht: Sockel, der Mast, die Armlinie und der Leuchtenumriss."),
        "retained": t("Plinth, taper of the column, the arm and its strut, the flat luminaire and its lens line; overall height 6.0 m and width 1.375 m. Front view: strut, lens line.", "Sockel, Verjüngung des Masts, der Arm und seine Strebe, die flache Leuchte und ihre Abdeckungslinie; Gesamthöhe 6,0 m und Breite 1,375 m. Vorderansicht: Strebe, Abdeckungslinie."),
        "merged": t("Base plate, shroud, band and cap merge into the column outline; clamp and socket into the arm stroke; cover, tray and spigot collar into one luminaire outline.", "Fußplatte, Manschette, Ring und Kappe gehen im Mastumriss auf; Klemme und Muffe im Armstrich; Haube, Wanne und Zapfenkragen in einem Leuchtenumriss."),
        "omitted": t("Door, highlight, nuts, fins, hatch joint, parting line. Plan view: the lens lies under the housing and is not seen.", "Tür, Glanzlinie, Muttern, Rippen, Klappenfuge, Teilungsfuge. Draufsicht: die Abdeckung liegt unter dem Gehäuse und ist nicht zu sehen."),
        "thinElements": t("The 60 mm arm is a single 0.25 mm stroke (50 mm real weight); the 27 mm strut a 0.13 mm stroke; the lens a 0.13 mm glazing stroke; the column top is drawn true (80 mm = 0.4 mm).", "Der Arm von 60 mm ist ein einzelner Strich von 0,25 mm (50 mm reale Stärke); die Strebe von 27 mm ein Strich von 0,13 mm; die Abdeckung ein Glasstrich von 0,13 mm; das Mastende ist wahr gezeichnet (80 mm = 0,4 mm)."),
    },
    "500": {
        "silhouette": t("Composed lamp sign 2.75 x 12 mm: a plinth, the true cone (0.34 to 0.16 mm), the arm stroke and the flat luminaire head at its end. Front view: the tapering column over its full height, the plinth and the head seen end-on as a flat rounded section. Plan view: plinth square, the column, the arm line and the head outline; arm and head follow the sway.", "Zusammengesetztes Leuchtenzeichen 2,75 x 12 mm: ein Sockel, der wahre Kegel (0,34 auf 0,16 mm), der Armstrich und der flache Leuchtenkopf an seinem Ende. Vorderansicht: der verjüngte Mast über die ganze Höhe, der Sockel und der Kopf stirnseitig als flacher gerundeter Querschnitt. Draufsicht: Sockelquadrat, der Mast, die Armlinie und der Kopfumriss; Arm und Kopf folgen dem Wiegen."),
        "retained": t("Height, plinth width, taper of the column, the outreach of 1.225 m, the luminaire as a flat side-entry head. Front and plan view: plinth, column, arm and head.", "Höhe, Sockelbreite, Verjüngung des Masts, die Ausladung von 1,225 m, die Leuchte als flacher Kopf mit seitlicher Einführung. Vorder- und Draufsicht: Sockel, Mast, Arm und Kopf."),
        "merged": t("Housing, lens, spigot and fins become one head (1.0 x 0.2 mm) listing lamp-housing and lamp-lens; base plate, shroud, band and cap merge into the column; clamp, strut and arm form one stroke.", "Gehäuse, Abdeckung, Zapfen und Rippen werden ein Kopf (1,0 x 0,2 mm) mit lamp-housing und lamp-lens; Fußplatte, Manschette, Ring und Kappe gehen im Mast auf; Klemme, Strebe und Arm bilden einen Strich."),
        "omitted": t("Strut (27 mm), door, nuts, fins - all below 0.075 m. Front view: the arm is hidden end-on behind the head.", "Strebe (27 mm), Tür, Muttern, Rippen - alle unter 0,075 m. Vorderansicht: der Arm verschwindet stirnseitig hinter dem Kopf."),
        "thinElements": t("The column is drawn true, 0.17 m at the foot to 0.08 m at the top (0.34-0.16 mm); the 60 mm arm is a single 0.18 mm stroke (90 mm).", "Der Mast ist wahr gezeichnet, 0,17 m am Fuß bis 0,08 m oben (0,34-0,16 mm); der Arm von 60 mm ist ein einzelner Strich von 0,18 mm (90 mm)."),
    },
}

REFERENCES = [
    {"id": "luminaire-philips-505", "citation": "Signify (Philips), product data sheet 910925864077: compact LED road luminaire, side entry 48-60 mm, overall length 505 mm, width 270 mm, net weight 4.5 kg (assets.lighting.philips.com/is/content/PhilipsLighting/fp910925864077-pss-en_gb)."},
    {"id": "luminaire-thorn-isaro-s", "citation": "Thorn Lighting, Isaro small narrow road optic 96636141: 550 x 238 x 157 mm, 4.1 kg, spigot 60 mm (thornlighting.co.uk)."},
    {"id": "luminaire-dialux-602", "citation": "DIALux product data sheet cPthucZZTuWLhRwU72u6AA: LED street luminaire 52 W, post top on 60 mm spigot or side entry 49-60 mm, 602 x 282 x 195 mm, 5.32 kg (luminaires.dialux.com)."},
    {"id": "en-40", "citation": "EN 40 Lighting columns (parts 2, 3 and 5): dimensions and tolerances, design and verification of steel lighting columns (conical columns, flange plates, door openings, bracket spigots)."},
]


def roots(doc):
    """🌱 Applies every root fix to a document in place."""
    a = doc["asset"]
    refs = a["provenance"]["references"]
    known = {r["id"] for r in refs}
    refs += [r for r in REFERENCES if r["id"] not in known]
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones["arm"]["lengthM"] = K.ARM_L
    bones["arm"]["mass"]["comX"] = 0.41
    bones["arm"]["section"] = {"areaM2": r6(math.pi * (K.ARM_R ** 2 - (K.ARM_R - K.ARM_WALL) ** 2)), "secondMomentM4": float(f"{math.pi / 4 * (K.ARM_R ** 4 - (K.ARM_R - K.ARM_WALL) ** 4):.4e}"), "extremeFibreM": K.ARM_R, "material": "aluminium-anodised", "title": t("Aluminium tube 60.4 x 3 mm", "Aluminiumrohr 60,4 x 3 mm")}
    bones["pole"]["section"] = {"areaM2": r6(math.pi * (K.POLE_R0 ** 2 - (K.POLE_R0 - K.POLE_WALL) ** 2)), "secondMomentM4": float(f"{math.pi / 4 * (K.POLE_R0 ** 4 - (K.POLE_R0 - K.POLE_WALL) ** 4):.4e}"), "extremeFibreM": K.POLE_R0, "material": "galv-steel", "title": t("Steel tube 170 x 4 mm at the foot (critical section)", "Stahlrohr 170 x 4 mm am Fuß (maßgebender Schnitt)")}
    head = bones["head"]
    head["lengthM"] = K.HEAD_L
    head["mass"] = {"kg": 4.5, "comX": 0.24, "comY": -0.005, "distribution": "point", "source": "literature", "reference": "luminaire-philips-505"}
    a["dimensions"]["widthM"] = r6(0.15 + K.HEAD_U + K.HEAD_L)
    a["dimensions"]["massKg"] = r6(sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b))
    for an in a["anchors"]:
        if an["id"] == "lamp-plume":
            an["x"], an["y"] = K.PLUME_U, r6(M.PLUME_Y)
    for reg in a["thermal"]["regions"]:
        if reg["id"] == "arm-alu":
            reg["areaM2"] = 0.27
        if reg["id"] == "lamp-housing":
            reg["layers"][0]["title"] = t("Die-cast aluminium housing with fins, 4.5 kg luminaire", "Aluminium-Druckgussgehäuse mit Rippen, Leuchte 4,5 kg")
    reps = a["representations"]
    for den, fn in (("50", redraw_50), ("100", redraw_100), ("200", redraw_200), ("500", redraw_500)):
        fn(reps[den])
        reps[den]["decisions"].update(DECISIONS[den])
    return doc


def main():
    """🌱 Builds the candidate or writes the catalog."""
    with open(CATALOG, encoding="utf-8") as f:
        doc = json.load(f)
    roots(doc)
    text = dump(doc) + "\n"
    if "--write" in sys.argv:
        with open(CATALOG, encoding="utf-8") as f:
            fresh = json.load(f)
        roots(fresh)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(dump(fresh) + "\n")
        print("wrote", CATALOG)
    else:
        out = os.path.join(HERE, "🗑️generated", "r3-street-lamp", "🔣️.json")
        with open(out, "w", encoding="utf-8") as f:
            f.write(text)
        print("candidate", out)


if __name__ == "__main__":
    main()
