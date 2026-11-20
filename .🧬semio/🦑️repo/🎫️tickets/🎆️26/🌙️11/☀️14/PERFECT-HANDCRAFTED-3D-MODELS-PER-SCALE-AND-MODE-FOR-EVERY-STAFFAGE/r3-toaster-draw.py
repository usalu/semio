"""✏️ Round 3 two-slice toaster: the 2D drawings of every authored scale and view redrawn as the true orthographic views of that scale's model (ruling E1) from the same measures as the solids; a shape that existed keeps its id, bone, role, layer order, tone, region and material, new shapes (plinth, controls seen edge-on, cord bush) are stated in full."""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-toaster-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
TEXTS = os.path.join(HERE, "r3-toaster-texts.json")
GEOMETRY_KEYS = ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "points", "closed", "d", "x0", "y0", "x1", "y1", "r0", "r1")
KAPPA = 0.5523
ZF = K.HZ


def r4(v):
    return round(v, 4) + 0.0


def keep(shape):
    return {k: v for k, v in shape.items() if k not in GEOMETRY_KEYS}


def new(sid, bone, role, z, filled, tone, region=None, material=None):
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    return s


def poly(base, points, closed=True):
    s = keep(base)
    s.update({"kind": "poly", "points": [[r4(x), r4(y)] for x, y in points], "closed": closed})
    return s


def rect(base, x, y, w, h, r=0.0):
    s = keep(base)
    s.update({"kind": "rect", "x": r4(x), "y": r4(y), "w": r4(w), "h": r4(h)})
    if r:
        s["r"] = r4(r)
    return s


def ellipse(base, r):
    s = keep(base)
    s.update({"kind": "ellipse", "cx": 0, "cy": 0, "rx": r, "ry": r})
    return s


def path(base, d):
    s = keep(base)
    s.update({"kind": "path", "d": d})
    return s


def shouldered(half, low, wall, top_half, top, bottom):
    """🏠 Outline of the housing in a side view as an SVG path: `bottom` is the list of right-hand points from the lowest edge up to the wall, then the wall, the quarter-ellipse shoulder and the flat top, mirrored."""
    a, b = half - top_half, top - wall
    f = lambda v: f"{r4(v):g}"
    right = " ".join(f"L {f(x)} {f(y)}" for x, y in bottom[1:])
    left = " ".join(f"L {f(-x)} {f(y)}" for x, y in reversed(bottom))
    return (
        f"M {f(bottom[0][0])} {f(bottom[0][1])} {right} L {f(half)} {f(wall)} "
        f"C {f(half)} {f(wall + b * KAPPA)} {f(top_half + a * KAPPA)} {f(top)} {f(top_half)} {f(top)} L {f(-top_half)} {f(top)} "
        f"C {f(-top_half - a * KAPPA)} {f(top)} {f(-half)} {f(wall + b * KAPPA)} {f(-half)} {f(wall)} {left} Z"
    ).replace("  ", " ")


def band(width, ys):
    """🎩 The dark cap in a side view: the shoulder outline from the seam to the flat top, both sides."""
    right = [(width(y), y) for y in ys]
    return right + [(-x, y) for x, y in reversed(right)]


def plan_outline(a, b, per_quarter=8):
    return K.superellipse(a, b, K.N, per_quarter)


def bread(profile, dy=-K.SEAT):
    return K.mirror(profile, dy)


def foot_side(x, r0, r1, h):
    return [(x - r0, 0.0), (x + r0, 0.0), (x + r1, h), (x - r1, h)]


def by_id(shapes):
    return {s["id"]: s for s in shapes}


def geometry50(old):
    e, f, p = by_id(old["elevation"]), by_id(old["front"]), by_id(old["plan"])
    tray = new("base-tray", "base", "body", 2.9, True, 0.2, "casing", "pp-black")
    low_e = [(0.1485, 0.0305), (K.HX, 0.033)]
    low_f = [(K.HZ - 0.0015, 0.0305), (K.HZ, 0.033)]
    ys = K.RIM_Y["50"]
    crumb_top = [(x, y - K.SEAT) for x, y in K.mirror(K.CRUMB[3:])]
    elevation = [
        poly(e["foot-left"], foot_side(-K.FOOT_X, K.FOOT_R0, K.FOOT_R1, 0.013)),
        poly(e["foot-right"], foot_side(K.FOOT_X, K.FOOT_R0, K.FOOT_R1, 0.013)),
        poly(tray, [(-0.14, K.FOOT_H), (0.14, K.FOOT_H), (K.TRAY_HX, 0.016), (K.TRAY_HX, 0.031), (-K.TRAY_HX, 0.031), (-K.TRAY_HX, 0.016)]),
        poly(e["slice-rear"], bread(K.BREAD)),
        poly(e["slice-front"], bread(K.BREAD)),
        poly(e["crumb-edge"], crumb_top, closed=False),
        path(e["casing"], shouldered(K.HX, K.TRAY_TOP, K.WALL_X, K.TOP_HX, K.TOP, [(K.TRAY_HX, K.TRAY_TOP)] + low_e)),
        poly(e["top-plate"], band(K.hx, ys)),
        poly(e["seam-top"], [(-K.hx(K.SEAM), K.SEAM), (K.hx(K.SEAM), K.SEAM)], closed=False),
        rect({**e["tray-slot"], "filled": True, "tone": 0.7, "material": "chrome-steel"}, -0.095, 0.015, 0.19, 0.012, 0.002),
        poly(e["highlight"], [(-0.138, 0.06), (-0.138, 0.14)], closed=False),
        rect(e["lever-track"], K.LEVER_X - 0.01, 0.091, 0.02, 0.068, 0.01),
        rect(e["lever"], -0.015, -0.012, 0.03, 0.024, 0.006),
        ellipse(e["dial"], K.DIAL_R),
        poly(e["dial-pointer"], [(0, 0.002), (0, 0.011)], closed=False),
        rect(e["cancel-button"], K.BUTTON[0] - K.BUTTON[2], K.BUTTON[1] - K.BUTTON[3], 2 * K.BUTTON[2], 2 * K.BUTTON[3], 0.004),
    ]
    control = {"role": "body", "z": 2.7, "filled": True, "tone": 0.2, "region": "control", "material": "pp-black"}
    front = [
        poly(f["foot-left"], foot_side(K.FOOT_Z, K.FOOT_R0, K.FOOT_R1, 0.013)),
        poly(f["foot-right"], foot_side(-K.FOOT_Z, K.FOOT_R0, K.FOOT_R1, 0.013)),
        poly(tray, [(-0.066, K.FOOT_H), (0.066, K.FOOT_H), (K.TRAY_HZ, 0.016), (K.TRAY_HZ, 0.031), (-K.TRAY_HZ, 0.031), (-K.TRAY_HZ, 0.016)]),
        rect({"id": "cord-bush", "bone": "base", "role": "body", "z": 2.6, "filled": True, "tone": 0.2, "region": "casing", "material": "pp-black"}, 0.069, K.BUSH[1] - K.BUSH[2], 0.011, 2 * K.BUSH[2], 0.002),
        rect({"id": "lever", "bone": "lever", **control}, -ZF - 0.014, K.LEVER_Y - 0.012, 0.016, 0.024, 0.003),
        rect({"id": "dial", "bone": "dial", **control, "tone": 0.25}, -ZF - 0.0125, K.DIAL_Y - K.DIAL_R, 0.0145, 2 * K.DIAL_R, 0.002),
        rect({"id": "cancel-button", "bone": "base", **control}, -ZF - 0.0036, K.BUTTON[1] - K.BUTTON[3], 0.0056, 2 * K.BUTTON[3], 0.001),
        rect(f["slice-rear"], K.SLOT_Z - K.BREAD_T / 2, K.SEAT, K.BREAD_T, K.BREAD_H, 0.003),
        rect(f["slice-front"], -K.SLOT_Z - K.BREAD_T / 2, K.SEAT, K.BREAD_T, K.BREAD_H, 0.003),
        path(f["casing"], shouldered(K.HZ, K.TRAY_TOP, K.WALL_Z, K.TOP_HZ, K.TOP, [(K.TRAY_HZ, K.TRAY_TOP)] + low_f)),
        poly(f["top-plate"], band(K.hz, ys)),
        poly(f["seam-top"], [(-K.hz(K.SEAM), K.SEAM), (K.hz(K.SEAM), K.SEAM)], closed=False),
    ]
    plan = [
        rect({"id": "cord-bush", "bone": "base", "role": "body", "z": 2.6, "filled": True, "tone": 0.2, "region": "casing", "material": "pp-black"}, K.BUSH[0] - K.BUSH[2], 0.069, 2 * K.BUSH[2], 0.011, 0.002),
        rect({"id": "lever", "bone": "lever", **control}, K.LEVER_X - 0.015, -ZF - 0.014, 0.03, 0.016, 0.003),
        rect({"id": "dial", "bone": "dial", **control, "tone": 0.25}, K.DIAL_X - K.DIAL_R, -ZF - 0.0125, 2 * K.DIAL_R, 0.0145, 0.002),
        rect({"id": "cancel-button", "bone": "base", **control}, K.BUTTON[0] - K.BUTTON[2], -ZF - 0.0036, 2 * K.BUTTON[2], 0.0056, 0.001),
        poly(p["casing"], plan_outline(K.HX, K.HZ)),
        poly(p["top-plate"], plan_outline(K.hx(K.SEAM), K.hz(K.SEAM))),
        poly({**p["seam-top"], "region": "top-plate"}, plan_outline(K.TOP_HX, K.TOP_HZ)),
        rect(p["slot-1"], -K.SLOT_HX, K.SLOT_Z - K.SLOT_W / 2, 2 * K.SLOT_HX, K.SLOT_W, K.SLOT_R),
        rect(p["slot-2"], -K.SLOT_HX, -K.SLOT_Z - K.SLOT_W / 2, 2 * K.SLOT_HX, K.SLOT_W, K.SLOT_R),
        rect(p["slice-rear"], -K.BREAD_HX, K.SLOT_Z - K.BREAD_T / 2, 2 * K.BREAD_HX, K.BREAD_T, 0.003),
        rect(p["slice-front"], -K.BREAD_HX, -K.SLOT_Z - K.BREAD_T / 2, 2 * K.BREAD_HX, K.BREAD_T, 0.003),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def geometry100(old):
    e, f, p = by_id(old["elevation"]), by_id(old["front"]), by_id(old["plan"])
    tray = new("base-tray", "base", "body", 2.9, True, 0.2, "casing", "pp-black")
    control = {"role": "body", "z": 2.7, "filled": True, "tone": 0.2, "region": "control", "material": "pp-black"}
    elevation = [
        rect(e["foot-left"], -K.FOOT_X - 0.0125, 0, 0.025, 0.013),
        rect(e["foot-right"], K.FOOT_X - 0.0125, 0, 0.025, 0.013),
        rect(tray, -K.TRAY_HX, K.FOOT_H, 2 * K.TRAY_HX, 0.019),
        poly(e["slice-front"], bread(K.BREAD_100)),
        path(e["casing"], shouldered(K.HX, K.TRAY_TOP, K.WALL_X, K.TOP_HX, K.TOP, [(K.HX, K.TRAY_TOP)])),
        poly(e["top-bevel"], [(-K.hx(K.SEAM), K.SEAM), (K.hx(K.SEAM), K.SEAM)], closed=False),
        rect(e["lever-track"], K.LEVER_X - 0.01, 0.091, 0.02, 0.068, 0.01),
        rect(e["lever"], -0.015, -0.012, 0.03, 0.024),
        ellipse(e["dial"], K.DIAL_R),
    ]
    front = [
        rect(f["foot-left"], K.FOOT_Z - 0.0125, 0, 0.025, 0.013),
        rect(f["foot-right"], -K.FOOT_Z - 0.0125, 0, 0.025, 0.013),
        rect(tray, -K.TRAY_HZ, K.FOOT_H, 2 * K.TRAY_HZ, 0.019),
        rect({"id": "lever", "bone": "lever", **control}, -ZF - 0.015, K.LEVER_Y - 0.012, 0.018, 0.024),
        rect({"id": "dial", "bone": "dial", **control, "tone": 0.25}, -ZF - 0.012, K.DIAL_Y - K.DIAL_R, 0.015, 2 * K.DIAL_R),
        rect(f["slice-rear"], K.SLOT_Z - 0.0075, K.SEAT, 0.015, K.BREAD_H, 0.004),
        rect(f["slice-front"], -K.SLOT_Z - 0.0075, K.SEAT, 0.015, K.BREAD_H, 0.004),
        path(f["casing"], shouldered(K.HZ, K.TRAY_TOP, K.WALL_Z, K.TOP_HZ, K.TOP, [(K.HZ, K.TRAY_TOP)])),
        poly(f["top-bevel"], [(-K.hz(K.SEAM), K.SEAM), (K.hz(K.SEAM), K.SEAM)], closed=False),
    ]
    plan = [
        rect({"id": "lever", "bone": "lever", **control}, K.LEVER_X - 0.015, -ZF - 0.015, 0.03, 0.018),
        rect({"id": "dial", "bone": "dial", **control, "tone": 0.25}, K.DIAL_X - K.DIAL_R, -ZF - 0.012, 2 * K.DIAL_R, 0.015),
        poly(p["casing"], plan_outline(K.HX, K.HZ, 5)),
        poly(p["top-bevel"], plan_outline(K.hx(K.SEAM), K.hz(K.SEAM), 5)),
        rect(p["slice-rear"], -K.BREAD_HX, K.SLOT_Z - 0.0075, 2 * K.BREAD_HX, 0.015, 0.004),
        rect(p["slice-front"], -K.BREAD_HX, -K.SLOT_Z - 0.0075, 2 * K.BREAD_HX, 0.015, 0.004),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def geometry200(old):
    e, f, p = by_id(old["elevation"]), by_id(old["front"]), by_id(old["plan"])
    slice_e = e.get("slice") or e["slice-front"]
    slice_f = f.get("slice") or f["slice-front"]
    slice_p = p.get("slice") or p["slice-front"]
    control = {"role": "body", "z": 2.7, "filled": True, "tone": 0.2, "region": "control", "material": "pp-black"}
    elevation = [
        poly({**slice_e, "id": "slice-rear", "z": 2.4}, bread(K.BREAD_200)),
        poly({**slice_e, "id": "slice-front", "z": 2.5}, bread(K.BREAD_200)),
        path(e["casing"], shouldered(K.HX, 0.0, K.WALL_X, K.TOP_HX, K.TOP, [(K.HX, 0.0)])),
        rect({"id": "lever", "bone": "lever", **control, "z": 3.5}, -0.015, -0.015, 0.03, 0.03),
    ]
    front = [
        rect({"id": "lever", "bone": "lever", **control}, -ZF - 0.015, K.LEVER_Y - 0.015, 0.03, 0.03),
        rect({**slice_f, "id": "slice-rear", "z": 2.4}, K.SLOT_Z - 0.015, 0.1, 0.03, 0.1, 0.008),
        rect({**slice_f, "id": "slice-front", "z": 2.5}, -K.SLOT_Z - 0.015, 0.1, 0.03, 0.1, 0.008),
        path(f["casing"], shouldered(K.HZ, 0.0, K.WALL_Z, K.TOP_HZ, K.TOP, [(K.HZ, 0.0)])),
    ]
    plan = [
        rect({"id": "lever", "bone": "lever", **control}, K.LEVER_X - 0.015, -ZF - 0.015, 0.03, 0.03),
        poly(p["casing"], plan_outline(K.HX, K.HZ, 4)),
        rect({**slice_p, "id": "slice-rear", "z": 3.4}, -K.BREAD_HX, K.SLOT_Z - 0.015, 2 * K.BREAD_HX, 0.03, 0.006),
        rect({**slice_p, "id": "slice-front", "z": 3.45}, -K.BREAD_HX, -K.SLOT_Z - 0.015, 2 * K.BREAD_HX, 0.03, 0.006),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def geometry(den, old):
    return {"50": geometry50, "100": geometry100, "200": geometry200}[den](old)


def decisions(den):
    return json.load(open(TEXTS, encoding="utf-8"))["representations"][den]
