"""✏️ Round 3 electric fan heater: the 2D drawings of every authored scale and view, drawn by hand from the measures of the kit as the true orthographic views of that scale's body (ruling E1). Elevation: x ahead, y up; front: x = lateral z, y up; plan: x ahead, y = lateral z."""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-electric-fan-heater-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

W, YB, YT, YS = K.HALF_W, K.Y_BOTTOM, K.Y_TOP, K.Y_SEAM
MID = (YB + YT) / 2
FLAT_Y0 = MID - (MID - YB) * K.FRONT_LIP_SCALE
FLAT_Y1 = MID + (YT - MID) * K.FRONT_LIP_SCALE
FLAT_W = W * K.FRONT_LIP_SCALE
X0, X1 = K.REAR_LIP_X0, K.FRONT_LIP_X1
ABS, PC, STEEL, PP, RUBBER = "abs-cream", "pc-heat", "steel-mesh", "pp-dial", "rubber-foot"
T_CASE, T_COVER, T_GRILLE, T_INTAKE, T_DARK, T_RUBBER = 0.78, 0.7, 0.2, 0.3, 0.2, 0.15


def n(x):
    x = round(x, 5) + 0.0
    return int(x) if float(x).is_integer() else x


def shape(sid, bone, role, z, filled, tone, kind, geo, region=None, regions=None, material=None):
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    s.update(geo)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    return s


def rect(sid, x, y, w, h, tone, region=None, material=None, r=None, role="body", z=3, bone="body", filled=True, regions=None):
    geo = {"x": n(x), "y": n(y), "w": n(w), "h": n(h)}
    if r:
        geo["r"] = n(r)
    return shape(sid, bone, role, z, filled, tone, "rect", geo, region, regions, material)


def poly(sid, points, tone, region=None, material=None, role="body", z=3, bone="body", regions=None):
    return shape(sid, bone, role, z, True, tone, "poly", {"points": [[n(a), n(b)] for a, b in points], "closed": True}, region, regions, material)


def stroke(sid, a, b, tone, region=None, material=None, z=3.4, bone="body"):
    return shape(sid, bone, "detail", z, False, tone, "poly", {"points": [[n(a[0]), n(a[1])], [n(b[0]), n(b[1])]], "closed": False}, region, None, material)


def path(sid, d, tone, region=None, material=None, z=2.9, bone="body"):
    return shape(sid, bone, "body", z, True, tone, "path", {"d": d}, region, None, material)


def d(*parts):
    return " ".join(p if isinstance(p, str) else str(n(p)) for p in parts)


def side_outline(tube0, tube1):
    return [(tube0, YB), (tube1, YB), (X1, FLAT_Y0), (X1, FLAT_Y1), (tube1, YT), (tube0, YT), (X0, FLAT_Y1), (X0, FLAT_Y0)]


def plan_outline(tube0, tube1):
    return [(tube0, -W), (tube1, -W), (X1, -FLAT_W), (X1, FLAT_W), (tube1, W), (tube0, W), (X0, FLAT_W), (X0, -FLAT_W)]


def handle_side(flare, top=0.302):
    hd = K.HANDLE_DEPTH
    if flare:
        return d("M", -hd - 0.004, YT, "L", -hd, 0.279, "L", -hd, top, "C", -hd, 0.308, -0.009, 0.31, -0.005, 0.31, "L", 0.005, 0.31, "C", 0.009, 0.31, hd, 0.308, hd, top, "L", hd, 0.279, "L", hd + 0.004, YT, "Z")
    return d("M", -hd, YT, "L", -hd, top, "C", -hd, 0.308, -0.009, 0.31, -0.005, 0.31, "L", 0.005, 0.31, "C", 0.009, 0.31, hd, 0.308, hd, top, "L", hd, YT, "Z")


def handle_front(post, bar, flare):
    zo, zi = K.HANDLE_Z + post, K.HANDLE_Z - post
    under = 0.31 - 2 * bar
    f = 0.004 if flare else 0.0
    lift = 0.279 if flare else YT
    out = ["M", -zo - f, YT, "L", -zo, lift, "L", -zo, 0.292, "C", -zo, 0.304, -zo + 0.008, 0.31, -zo + 0.02, 0.31, "L", zo - 0.02, 0.31, "C", zo - 0.008, 0.31, zo, 0.304, zo, 0.292, "L", zo, lift, "L", zo + f, YT]
    out += ["L", zi - f, YT, "L", zi, lift, "L", zi, under - 0.005, "C", zi, under - 0.002, zi - 0.002, under, zi - 0.005, under, "L", -zi + 0.005, under, "C", -zi + 0.002, under, -zi, under - 0.002, -zi, under - 0.005, "L", -zi, lift, "L", -zi + f, YT, "Z"]
    return d(*out)


def draw50():
    y0, y1, half = K.OUTLET
    py0, py1, phalf = K.INTAKE_PANEL
    cx = K.CASSETTE_X1
    el = []
    for name, x in (("back", -K.FOOT_X), ("front", K.FOOT_X)):
        el.append(rect("foot-" + name, x - 0.015, 0, 0.03, K.PAD_H, T_RUBBER, "feet", RUBBER, z=2.8))
    for name, x in (("back", -K.FOOT_X), ("front", K.FOOT_X)):
        el.append(rect("boss-" + name, x - 0.017, K.PAD_H, 0.034, YB - K.PAD_H, T_CASE, "casing", ABS, z=2.85))
    el.append(path("handle", handle_side(True), 0.75, "casing", ABS))
    el.append(poly("housing", side_outline(K.TUBE_X0, K.TUBE_X1), T_CASE, "casing", ABS))
    el.append(rect("top-cover", K.TUBE_X0, YS, K.TUBE_X1 - K.TUBE_X0, YT - YS, T_COVER, "top-cover", ABS, z=3.1))
    el.append(rect("outlet-grille", X1, y0, cx - X1, y1 - y0, T_GRILLE, "grille", STEEL, z=3.2))
    slope = (K.SLAT_OUT[1] - K.SLAT_IN[1]) / (K.SLAT_OUT[0] - K.SLAT_IN[0])
    for k, c in enumerate(K.slat_centres()):
        el.append(stroke("louvre-%d" % k, (cx, c + K.SLAT_IN[1] + slope * (cx - K.SLAT_IN[0])), (K.SLAT_OUT[0], c + K.SLAT_OUT[1]), 0.2, "grille"))
    el.append(rect("inlet-grille", K.REAR_X, py0, X0 - K.REAR_X, py1 - py0, T_INTAKE, "intake", PC, z=3.2))
    el.append(rect("cord-grommet", K.REAR_X, 0.033, X0 - K.REAR_X, 0.014, T_DARK, None, PP, role="detail", z=3.2))
    el.append(stroke("highlight", (-0.066, 0.222), (-0.066, 0.17), 0.95, "casing", z=3.3))
    el.append(shape("dial", "knob", "body", 3.5, True, T_DARK, "ellipse", {"cx": 0, "cy": 0, "rx": K.KNOB_R, "ry": K.KNOB_R}, "control", None, PP))
    el.append(rect("dial-grip", -0.004, -0.02, 0.008, 0.04, 0.3, "control", PP, r=0.002, z=3.6, bone="knob"))
    el.append(stroke("dial-pointer", (0, 0.008), (0, 0.018), 0.95, "control", z=3.7, bone="knob"))
    el.append(rect("mode-switch", -0.016, K.SWITCH_Y - 0.008, 0.032, 0.016, T_DARK, "control", PP, r=0.002, z=3.4))
    el.append(stroke("mode-mark", (-0.008, K.SWITCH_Y), (0.008, K.SWITCH_Y), 0.95, "control", z=3.7))
    pl = [poly("housing", plan_outline(K.TUBE_X0, K.TUBE_X1), T_CASE, "casing", ABS)]
    pl.append(rect("top-cover", K.TUBE_X0, -W, K.TUBE_X1 - K.TUBE_X0, 2 * W, T_COVER, "top-cover", ABS, z=3.1))
    pl.append(rect("handle", -K.HANDLE_DEPTH, -K.HANDLE_Z - 0.008, 2 * K.HANDLE_DEPTH, 2 * K.HANDLE_Z + 0.016, 0.75, "casing", ABS, r=0.008, z=3.3))
    pl.append(rect("outlet-grille", X1, -half, cx - X1, 2 * half, T_GRILLE, "grille", STEEL, z=3.2))
    pl.append(rect("inlet-grille", K.REAR_X, -phalf, X0 - K.REAR_X, 2 * phalf, T_INTAKE, "intake", PC, z=3.2))
    pl.append(rect("dial", -K.KNOB_R, -K.KNOB_OUT, 2 * K.KNOB_R, K.KNOB_OUT - W, T_DARK, "control", PP, z=3.5))
    pl.append(stroke("highlight", (-0.06, 0.1), (-0.06, -0.1), 0.95, "casing", ABS, z=3.5))
    fr = []
    for name, z in (("left", K.FOOT_Z), ("right", -K.FOOT_Z)):
        fr.append(rect("foot-" + name, z - 0.015, 0, 0.03, K.PAD_H, T_RUBBER, "feet", RUBBER, z=2.8))
    for name, z in (("left", K.FOOT_Z), ("right", -K.FOOT_Z)):
        fr.append(rect("boss-" + name, z - 0.017, K.PAD_H, 0.034, YB - K.PAD_H, T_CASE, "casing", ABS, z=2.85))
    fr.append(path("handle", handle_front(0.008, 0.006, True), 0.75, "casing", ABS))
    fr.append(rect("housing", -W, YB, 2 * W, YT - YB, T_CASE, "casing", ABS, r=K.EDGE_R))
    fr.append(rect("frame-edge", -FLAT_W, FLAT_Y0, 2 * FLAT_W, FLAT_Y1 - FLAT_Y0, 0.3, "casing", ABS, r=K.EDGE_R * K.FRONT_LIP_SCALE, role="detail", z=3.1, filled=False))
    fr.append(rect("outlet-grille", -half, y0, 2 * half, y1 - y0, T_GRILLE, "grille", STEEL, r=K.OUTLET_R, z=3.2))
    bar = K.CASSETTE_BAR
    fr.append(rect("grille-opening", -half + bar, y0 + bar, 2 * (half - bar), y1 - y0 - 2 * bar, 0.5, "grille", STEEL, r=0.003, role="detail", z=3.3, filled=False))
    for k, c in enumerate(K.slat_centres()):
        fr.append(stroke("louvre-%d" % k, (-half + 0.006, c + K.SLAT_OUT[1]), (half - 0.006, c + K.SLAT_OUT[1]), 0.9, "grille", STEEL))
    fr.append(rect("dial", -K.KNOB_OUT, K.KNOB_Y - K.KNOB_R, K.KNOB_OUT - W, 2 * K.KNOB_R, T_DARK, "control", PP, z=3.5))
    return {"elevation": el, "plan": pl, "front": fr}


CAP100 = 0.015
PLATE100 = 0.0925
SLATS100 = (4, 0.04, 0.085)


def draw100():
    y0, y1, half = K.OUTLET
    py0, py1, phalf = K.INTAKE_PANEL
    t0, t1 = X0 + CAP100, X1 - CAP100
    count, pitch, first = SLATS100
    el = []
    for name, x in (("back", -K.FOOT_X), ("front", K.FOOT_X)):
        el.append(rect("foot-" + name, x - 0.016, 0, 0.032, YB, T_RUBBER, "feet", RUBBER, z=2.8))
    el.append(path("handle", handle_side(False, 0.303), 0.75, "casing", ABS))
    el.append(poly("housing", side_outline(t0, t1), T_CASE, "casing", ABS))
    el.append(stroke("top-cover-edge", (t0, YS), (t1, YS), 0.3, "top-cover", z=3.1))
    el.append(rect("outlet-grille", X1, y0, PLATE100 - X1, y1 - y0, T_GRILLE, "grille", STEEL, z=3.2))
    for k in range(count):
        c = first + pitch * k
        el.append(stroke("louvre-%d" % k, (PLATE100, c + 0.003), (K.FRONT_X, c - 0.003), 0.2, "grille"))
    el.append(rect("inlet-grille", K.REAR_X, py0, X0 - K.REAR_X, py1 - py0, T_INTAKE, "intake", PC, z=3.2))
    el.append(shape("dial", "knob", "body", 3.5, True, T_DARK, "ellipse", {"cx": 0, "cy": 0, "rx": K.KNOB_R, "ry": K.KNOB_R}, "control", None, PP))
    el.append(stroke("dial-pointer", (0, 0.006), (0, 0.018), 0.95, "control", z=3.7, bone="knob"))
    pl = [poly("housing", plan_outline(t0, t1), T_CASE, "casing", ABS)]
    pl.append(rect("handle", -K.HANDLE_DEPTH, -K.HANDLE_Z - 0.0085, 2 * K.HANDLE_DEPTH, 2 * K.HANDLE_Z + 0.017, 0.75, "casing", ABS, z=3.3))
    pl.append(rect("outlet-grille", X1, -half, PLATE100 - X1, 2 * half, T_GRILLE, "grille", STEEL, z=3.2))
    pl.append(rect("inlet-grille", K.REAR_X, -phalf, X0 - K.REAR_X, 2 * phalf, T_INTAKE, "intake", PC, z=3.2))
    pl.append(rect("dial", -K.KNOB_R, -K.KNOB_OUT, 2 * K.KNOB_R, K.KNOB_OUT - W, T_DARK, "control", PP, z=3.5))
    fr = []
    for name, z in (("left", K.FOOT_Z), ("right", -K.FOOT_Z)):
        fr.append(rect("foot-" + name, z - 0.016, 0, 0.032, YB, T_RUBBER, "feet", RUBBER, z=2.8))
    fr.append(path("handle", handle_front(0.0085, 0.0075, False), 0.75, "casing", ABS))
    fr.append(rect("housing", -W, YB, 2 * W, YT - YB, T_CASE, "casing", ABS, r=K.EDGE_R))
    fr.append(rect("outlet-grille", -half, y0, 2 * half, y1 - y0, T_GRILLE, "grille", STEEL, r=0.004, z=3.2))
    for k in range(count):
        c = first + pitch * k
        fr.append(stroke("louvre-%d" % k, (-half + 0.006, c - 0.003), (half - 0.006, c - 0.003), 0.9, "grille", STEEL))
    fr.append(rect("dial", -K.KNOB_OUT, K.KNOB_Y - K.KNOB_R, K.KNOB_OUT - W, 2 * K.KNOB_R, T_DARK, "control", PP, z=3.5))
    return {"elevation": el, "plan": pl, "front": fr}


BASE200 = (0.073, 0.09)
MERGED = ["casing", "top-cover", "intake", "control", "feet"]


def draw200():
    y0, y1, half = K.OUTLET
    bx, bz = BASE200
    el = [poly("block", [(-bx, 0), (bx, 0), (bx, YB), (X1, YB), (X1, YT), (0.015, YT), (0.015, 0.31), (-0.015, 0.31), (-0.015, YT), (X0, YT), (X0, YB), (-bx, YB)], T_CASE, None, ABS, regions=MERGED)]
    el.append(rect("outlet-grille", X1, y0, K.FRONT_X - X1, y1 - y0, T_GRILLE, "grille", STEEL, z=3.2))
    pl = [rect("block", X0, -W, X1 - X0, 2 * W, T_CASE, None, ABS, regions=MERGED)]
    pl.append(rect("outlet-grille", X1, -half, K.FRONT_X - X1, 2 * half, T_GRILLE, "grille", STEEL, z=3.2))
    fr = [poly("block", [(-bz, 0), (bz, 0), (bz, YB), (W - 0.022, YB), (W - 0.0065, YB + 0.0065), (W, YB + 0.022), (W, YT - 0.022), (W - 0.0065, YT - 0.0065), (W - 0.022, YT), (0.066, YT), (0.066, 0.298), (0.054, 0.31), (-0.054, 0.31), (-0.066, 0.298), (-0.066, YT), (-W + 0.022, YT), (-W + 0.0065, YT - 0.0065), (-W, YT - 0.022), (-W, YB + 0.022), (-W + 0.0065, YB + 0.0065), (-W + 0.022, YB), (-bz, YB)], T_CASE, None, ABS, regions=MERGED)]
    fr.append(rect("outlet-grille", -half, y0, 2 * half, y1 - y0, T_GRILLE, "grille", STEEL, z=3.2))
    return {"elevation": el, "plan": pl, "front": fr}


def geometry(den, old):
    drawn = {"50": draw50, "100": draw100, "200": draw200}[str(den)]()
    return {view: drawn[view] for view in old}


def decisions(den, old):
    tp = os.path.join(HERE, "r3-electric-fan-heater-texts.json")
    if not os.path.exists(tp):
        return old
    new = json.load(open(tp, encoding="utf-8")).get("representations", {}).get(str(den))
    if not new:
        return old
    return {key: new.get(key, old[key]) for key in old}
