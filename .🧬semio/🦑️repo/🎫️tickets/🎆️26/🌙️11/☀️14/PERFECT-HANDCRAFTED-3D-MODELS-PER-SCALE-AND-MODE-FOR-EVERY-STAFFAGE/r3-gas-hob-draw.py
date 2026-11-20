"""✏️ Round 3 domino gas hob: the 2D drawings of 1:50, 1:100 and 1:200 redrawn as the true orthographic views of that scale's model (ruling E1) from the measures of `r3-gas-hob-kit.py`; the flame tongues come from the projected loft outlines in `r3-gas-hob-flames.json`. Every kept shape keeps its id, bone, role, layer, tone, region and material; new shapes: feet, plan well, knob at 1:100, pan support in the 1:200 plan."""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-gas-hob-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
FLAMES = json.load(open(os.path.join(HERE, "r3-gas-hob-flames.json"), encoding="utf-8"))
TEXTS = os.path.join(HERE, "r3-gas-hob-texts.json")
GEOMETRY_KEYS = ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "points", "closed", "d")
BZ = K.BURNER_Z


def r4(v):
    return round(v, 4) + 0.0


def keep(shape):
    return {k: v for k, v in shape.items() if k not in GEOMETRY_KEYS}


def poly(shape, points):
    s = keep(shape)
    s.update({"kind": "poly", "points": [[r4(x), r4(y)] for x, y in points], "closed": True})
    return s


def rect(shape, x, y, w, h, r=None):
    s = keep(shape)
    s.update({"kind": "rect", "x": r4(x), "y": r4(y), "w": r4(w), "h": r4(h)})
    if r:
        s["r"] = r
    return s


def ellipse(shape, cx, cy, r):
    s = keep(shape)
    s.update({"kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(r), "ry": r4(r)})
    return s


def path(shape, rings, closed=True):
    s = keep(shape)
    d = []
    for ring in rings:
        d.append("M " + " L ".join("%s %s" % (r4(x), r4(y)) for x, y in ring) + (" Z" if closed else ""))
    s.update({"kind": "path", "d": " ".join(d)})
    return s


def mirror(half):
    """🪞 A closed symmetric outline from its right half (x >= 0, bottom to top)."""
    return list(half) + [(-x, y) for x, y in reversed(half) if abs(x) > 1e-9]


def shifted(points, dx, dy):
    return [(x + dx, y + dy) for x, y in points]


CUP_HALF = [(0.041, -0.004), (0.043, 0.002), (0.045, 0.002), (0.045, 0.0122)]
CAP_HALF = [(0.0465, 0.0122), (0.048, 0.0135), (0.048, 0.0172), (0.0462, 0.019), (0.03, 0.0196), (0.0, 0.02)]
RAIL_50 = [(-0.135, 0.048), (-0.135, 0.04), (-0.12, 0.04), (-0.066, 0.036), (-0.0635, 0.0362), (-0.0622, 0.038), (-0.062, 0.04), (-0.004, 0.04), (-0.004, 0.036), (0.004, 0.036), (0.004, 0.04), (0.062, 0.04), (0.0622, 0.038), (0.0635, 0.0362), (0.066, 0.036), (0.12, 0.04), (0.135, 0.04), (0.135, 0.048)]


def front_rail_50():
    """🛞 The pan support seen from the side (+x): the square frame makes it the elevation outline moved to the burner axis."""
    return shifted(RAIL_50, BZ, K.TRIVET_Y)


def tongue_star(r_tip=0.0585, r_root=0.0505, half=4.5):
    """⭐ The flame crown seen from above: 24 tongue lobes on the ring of inner cones."""
    pts = []
    for k in range(24):
        t = 75.0 - 15.0 * k
        for a, r in ((t + 7.5, r_root), (t + half, r_tip), (t - half, r_tip)):
            pts.append((r * math.cos(math.radians(a)), BZ + r * math.sin(math.radians(a))))
    return pts


def rounded(x0, y0, x1, y1, r, steps=3):
    """⬜ Rounded rectangle (x, y) counter-clockwise."""
    return [(p[1], p[0]) for p in K.rounded_rect(x0, x1, y0, y1, r, steps)]


def plan_trivet_50(side):
    """🛞 One bar of the pan support with its finger, seen from above."""
    h, b = K.TRIVET_HALF, K.TRIVET_BAR
    s0, s1 = K.FINGER_TIP, h - b
    if side == "front":
        return [(-h + 0.004, BZ - h), (h - 0.004, BZ - h), (h, BZ - h + 0.004), (h, BZ - h + b), (0.004, BZ - h + b), (0.004, BZ - s0 - 0.0005), (0.0, BZ - s0), (-0.004, BZ - s0 - 0.0005), (-0.004, BZ - h + b), (-h, BZ - h + b), (-h, BZ - h + 0.004)]
    if side == "back":
        return [(x, 2 * BZ - y) for x, y in reversed(plan_trivet_50("front"))]
    if side == "right":
        return [(h - b, BZ - h + b), (h, BZ - h + b), (h, BZ + h - b), (h - b, BZ + h - b), (h - b, BZ + 0.004), (s0 + 0.0005, BZ + 0.004), (s0, BZ), (s0 + 0.0005, BZ - 0.004), (h - b, BZ - 0.004)]
    return [(-x, y) for x, y in reversed(plan_trivet_50("right"))]


def vent_strokes(xs, y0=0.012, y1=0.02):
    return [[(x, y0), (x, y1)] for x in xs]


def port_strokes():
    out = []
    for k in range(K.PORT_N):
        t = math.radians(-360.0 * k / K.PORT_N)
        if math.sin(t) < -0.05 and abs(K.CUP[3][1] * math.cos(t)) < 0.041:
            x = K.CUP[3][1] * math.cos(t)
            out.append([(x, 0.0045), (x, 0.0105)])
    return out


def new(base, **kw):
    s = dict(base)
    s.update(kw)
    return s


def elevation50(old):
    by = {s["id"]: s for s in old}
    fl = FLAMES["50"]["elevation"]
    feet = new({k: v for k, v in by["housing"].items() if k not in ("region",)}, id="feet", z=3.52, tone=0.15, material="rubber")
    knob_pointer = path(by["knob-pointer"], [[(-0.00375, -0.011), (0.00375, -0.011), (0.00375, 0.011), (-0.00375, 0.011), (-0.00375, -0.011)], [(0.0, 0.0045), (0.0, 0.0095)]], closed=False)
    out = []
    for s in old:
        i = s["id"]
        if i in ("flame-left", "flame-right", "flame-centre"):
            out.append(path(s, fl[i]))
        elif i == "burner-cap":
            out.append(poly(s, mirror(CAP_HALF)))
        elif i == "burner-ring":
            out.append(poly(s, mirror(CUP_HALF)))
        elif i == "plate":
            out.append(rect(s, -K.PLATE_X, K.PLATE_Y0, 2 * K.PLATE_X, K.PLATE_Y1 - K.PLATE_Y0))
        elif i == "housing":
            out.append(rect(s, -K.HOUSING_X, K.FOOT_H, 2 * K.HOUSING_X, K.PLATE_Y0 - K.FOOT_H))
            out.append(path(feet, [rounded(-K.FEET_X - K.FOOT_R, 0.0, -K.FEET_X + K.FOOT_R, K.FOOT_H, 0.0015, 1), rounded(K.FEET_X - K.FOOT_R, 0.0, K.FEET_X + K.FOOT_R, K.FOOT_H, 0.0015, 1)]))
        elif i == "plate-lip":
            continue
        elif i == "knob":
            out.append(knob_solid(ellipse(s, 0, 0, 0.013)))
        elif i == "knob-pointer":
            out.append(knob_pointer)
        elif i == "trivet-leg-left":
            out.append(rect(s, -K.TRIVET_HALF, 0.0, K.TRIVET_LEG, 0.0405))
        elif i == "trivet-leg-right":
            out.append(rect(s, K.TRIVET_HALF - K.TRIVET_LEG, 0.0, K.TRIVET_LEG, 0.0405))
        elif i == "trivet-rail":
            out.append(poly(s, RAIL_50))
        elif i == "vent-slots":
            out.append(path(s, vent_strokes([-0.11 + 0.02 * k for k in range(9)]), closed=False))
        elif i == "front-panel-seam":
            out.append(path(s, [[(-K.HOUSING_X + 0.004, K.BEAD_Y), (K.HOUSING_X - 0.004, K.BEAD_Y)]], closed=False))
        elif i == "burner-ports":
            out.append(path(s, port_strokes(), closed=False))
        else:
            raise SystemExit("unhandled elevation shape " + i)
    return out


def front50(old):
    by = {s["id"]: s for s in old}
    fl = FLAMES["50"]["front"]
    feet = new({k: v for k, v in by["housing"].items() if k not in ("region",)}, id="feet", z=3.52, tone=0.15, material="rubber")
    out = []
    for s in old:
        i = s["id"]
        if i in ("flame-left", "flame-right", "flame-centre"):
            out.append(path(s, fl[i]))
        elif i == "burner-cap":
            out.append(poly(s, shifted(mirror(CAP_HALF), BZ, K.BURNER_Y)))
        elif i == "burner-ring":
            out.append(poly(s, shifted(mirror(CUP_HALF), BZ, K.BURNER_Y)))
        elif i == "plate":
            out.append(rect(s, K.PLATE_Z0, K.PLATE_Y0, K.PLATE_Z1 - K.PLATE_Z0, K.PLATE_Y1 - K.PLATE_Y0))
        elif i == "housing":
            out.append(rect(s, K.HOUSING_Z0, K.FOOT_H, K.HOUSING_Z1 - K.HOUSING_Z0, K.PLATE_Y0 - K.FOOT_H))
            out.append(path(feet, [rounded(z - K.FOOT_R, 0.0, z + K.FOOT_R, K.FOOT_H, 0.0015, 1) for z in K.FEET_Z]))
        elif i == "plate-lip":
            continue
        elif i == "knob":
            out.append(knob_solid(ellipse(s, 0, 0, 0.013)))
        elif i == "trivet-leg-left":
            out.append(rect(s, BZ + K.TRIVET_HALF - K.TRIVET_LEG, K.TRIVET_Y, K.TRIVET_LEG, 0.0405))
        elif i == "trivet-leg-right":
            out.append(rect(s, BZ - K.TRIVET_HALF, K.TRIVET_Y, K.TRIVET_LEG, 0.0405))
        elif i == "trivet-rail":
            out.append(poly(s, front_rail_50()))
        elif i == "vent-slots":
            out.append(path(s, vent_strokes([-0.2 + 0.04 * k for k in range(11)]), closed=False))
        elif i == "front-panel-seam":
            out.append(path(s, [[(K.HOUSING_Z0 + 0.004, K.BEAD_Y), (K.HOUSING_Z1 - 0.004, K.BEAD_Y)]], closed=False))
        else:
            raise SystemExit("unhandled front shape " + i)
    return out


def plan50(old):
    by = {s["id"]: s for s in old}
    out = []
    for s in old:
        i = s["id"]
        if i == "plan-plate":
            out.append(rect(s, -K.PLATE_X, K.PLATE_Z0, 2 * K.PLATE_X, K.PLATE_Z1 - K.PLATE_Z0, K.PLATE_R))
            well = {"id": "plan-plate-well", "bone": "hob", "role": "detail", "z": 1.5, "filled": False, "tone": 0.3, "region": "plate", "material": "steel-enamel"}
            out.append(ellipse(well, 0.0, BZ, K.WELL_R))
        elif i == "plan-burner-ring":
            out.append(ellipse(s, 0.0, BZ, 0.045))
        elif i == "plan-flame-ring":
            out.append(poly(s, tongue_star()))
        elif i == "plan-burner-cap":
            out.append(ellipse(s, 0.0, BZ, 0.048))
        elif i.startswith("plan-trivet-"):
            out.append(poly(s, plan_trivet_50(i.split("-")[-1])))
        elif i == "plan-knob":
            out.append(rect(s, K.KNOB_X - 0.013, -0.2573, 0.026, 0.2573 + K.HOUSING_Z0))
        elif i == "plan-knob-pointer":
            p = keep(s)
            p.update({"kind": "poly", "points": [[r4(K.KNOB_X - 0.0105), -0.2495], [r4(K.KNOB_X + 0.0105), -0.2495]], "closed": False})
            out.append(p)
        else:
            raise SystemExit("unhandled plan shape " + i)
    return out


CROWN_100 = [(-0.059, -0.003), (0.059, -0.003), (0.059, 0.015), (0.051, 0.0235), (0.04, 0.0125), (0.028, 0.0235), (0.014, 0.012), (0.0, 0.0235), (-0.014, 0.012), (-0.028, 0.0235), (-0.04, 0.0125), (-0.051, 0.0235), (-0.059, 0.015)]


def knob_solid(shape, front=-0.2575):
    """🎛️ The knob as a laterally extruded solid from the fascia to its ridge: front and plan follow its turned face in every pose."""
    s = dict(shape)
    s.pop("solid", None)
    s["solid"] = {"kind": "extruded", "sweep": "w", "z0": front, "z1": K.HOUSING_Z0}
    return s


def knob100(base):
    return {"id": "knob", "bone": "knob", "role": "body", "z": 3.8, "filled": True, "tone": 0.6, "region": "knob", "material": "bakelite"}


def elevation100(old):
    fl = [CROWN_100]
    out = []
    for s in old:
        i = s["id"]
        if i == "flame-crown":
            out.append(path(s, fl))
        elif i == "burner":
            out.append(poly(s, mirror([(0.046, -0.002), (0.047, 0.0165), (0.03, 0.0195), (0.0, 0.02)])))
        elif i == "plate":
            p = rect(s, -K.PLATE_X, 0.0, 2 * K.PLATE_X, K.PLATE_Y1)
            p.pop("regions", None)
            p["region"] = "plate"
            out.append(p)
            out.append(knob_solid(ellipse(knob100(s), 0, 0, 0.013)))
        elif i == "trivet-leg-left":
            out.append(rect(s, -K.TRIVET_HALF, 0.0, 0.01, 0.0365))
        elif i == "trivet-leg-right":
            out.append(rect(s, K.TRIVET_HALF - 0.01, 0.0, 0.01, 0.0365))
        elif i == "trivet-rail":
            out.append(rect(s, -K.TRIVET_HALF, 0.036, 2 * K.TRIVET_HALF, 0.012))
        else:
            raise SystemExit("unhandled 1:100 elevation shape " + i)
    return out


def front100(old):
    fl = [shifted(CROWN_100, BZ, K.FLAME_Y)]
    out = []
    for s in old:
        i = s["id"]
        if i == "flame-crown":
            out.append(path(s, fl))
        elif i == "burner":
            out.append(poly(s, shifted(mirror([(0.046, -0.002), (0.047, 0.0165), (0.03, 0.0195), (0.0, 0.02)]), BZ, K.BURNER_Y)))
        elif i == "plate":
            p = rect(s, K.PLATE_Z0, 0.0, K.PLATE_Z1 - K.PLATE_Z0, K.PLATE_Y1)
            p.pop("regions", None)
            p["region"] = "plate"
            out.append(p)
            out.append(knob_solid(ellipse(knob100(s), 0, 0, 0.013)))
        elif i == "trivet-leg-left":
            out.append(rect(s, BZ + K.TRIVET_HALF - 0.01, K.TRIVET_Y, 0.01, 0.0365))
        elif i == "trivet-leg-right":
            out.append(rect(s, BZ - K.TRIVET_HALF, K.TRIVET_Y, 0.01, 0.0365))
        elif i == "trivet-rail":
            out.append(rect(s, BZ - K.TRIVET_HALF, K.TRIVET_Y + 0.036, 2 * K.TRIVET_HALF, 0.012))
        else:
            raise SystemExit("unhandled 1:100 front shape " + i)
    return out


def plan100(old):
    h = K.TRIVET_HALF
    out = []
    for s in old:
        i = s["id"]
        if i == "plan-plate":
            p = rect(s, -K.PLATE_X, K.PLATE_Z0, 2 * K.PLATE_X, K.PLATE_Z1 - K.PLATE_Z0, K.PLATE_R)
            p.pop("regions", None)
            p["region"] = "plate"
            out.append(p)
        elif i == "plan-burner":
            out.append(ellipse(s, 0.0, BZ, 0.0585))
        elif i == "plan-trivet":
            outer = rounded(-h, BZ - h, h, BZ + h, 0.013)
            a, f, w = h - 0.01, 0.0638, 0.005
            inner = [(-a, BZ - a), (-w, BZ - a), (-w, BZ - f), (w, BZ - f), (w, BZ - a), (a, BZ - a), (a, BZ - w), (f, BZ - w), (f, BZ + w), (a, BZ + w), (a, BZ + a), (w, BZ + a), (w, BZ + f), (-w, BZ + f), (-w, BZ + a), (-a, BZ + a), (-a, BZ + w), (-f, BZ + w), (-f, BZ - w), (-a, BZ - w)]
            p = path(s, [outer, list(reversed(inner))])
            p["filled"] = True
            out.append(p)
            k = {"id": "plan-knob", "bone": "hob", "role": "body", "z": 6, "filled": True, "tone": 0.6, "region": "knob", "material": "bakelite"}
            out.append(rect(k, K.KNOB_X - 0.013, -0.2575, 0.026, 0.2575 + K.HOUSING_Z0))
        else:
            raise SystemExit("unhandled 1:100 plan shape " + i)
    return out


CROWN_200 = [(0.062, 0.0), (0.062, 0.01), (0.052, 0.019), (0.034, 0.0255), (0.0, 0.028)]
BURNER_200 = [(0.05, -0.002), (0.05, 0.022), (0.04, 0.03), (0.0, 0.031)]


def plate_with_knob(r, steps=2):
    """⬜ The 1:200 plate seen from above with the knob merged as a 30 mm step at the front edge (counter-clockwise from the front left corner)."""
    x0, x1, y0, y1 = -K.PLATE_X, K.PLATE_X, K.PLATE_Z0, K.PLATE_Z1
    def arc(cx, cy, a0):
        return [(cx + r * math.cos(math.radians(a0 + 90.0 * k / steps)), cy + r * math.sin(math.radians(a0 + 90.0 * k / steps))) for k in range(steps + 1)]
    bump = [(K.KNOB_X - 0.015, y0), (K.KNOB_X - 0.015, -0.26), (K.KNOB_X + 0.015, -0.26), (K.KNOB_X + 0.015, y0)]
    return arc(x0 + r, y0 + r, 180) + bump + arc(x1 - r, y0 + r, 270) + arc(x1 - r, y1 - r, 0) + arc(x0 + r, y1 - r, 90)


def elevation200(old):
    h = K.TRIVET_HALF
    out = []
    for s in old:
        i = s["id"]
        if i == "flame-crown":
            out.append(path(s, [mirror(CROWN_200)]))
        elif i == "burner":
            out.append(poly(s, mirror(BURNER_200)))
        elif i == "plate":
            out.append(rect(s, -K.PLATE_X, 0.0, 2 * K.PLATE_X, K.PLATE_Y1))
        elif i == "trivet":
            out.append(rect(s, -h, 0.033, 2 * h, 0.015))
            leg = {k: v for k, v in s.items()}
            out.append(rect(dict(leg, id="trivet-leg-left"), -h, 0.0, 0.015, 0.0335))
            out.append(rect(dict(leg, id="trivet-leg-right"), h - 0.015, 0.0, 0.015, 0.0335))
        else:
            raise SystemExit("unhandled 1:200 elevation shape " + i)
    return out


def front200(old):
    h = K.TRIVET_HALF
    ty = K.TRIVET_Y
    out = []
    for s in old:
        i = s["id"]
        if i == "plate":
            out.append(poly(s, [(K.PLATE_Z0, 0.0), (K.PLATE_Z1, 0.0), (K.PLATE_Z1, K.PLATE_Y1), (K.PLATE_Z0, K.PLATE_Y1), (K.PLATE_Z0, K.KNOB_Y + 0.015), (-0.26, K.KNOB_Y + 0.015), (-0.26, K.KNOB_Y - 0.015), (K.PLATE_Z0, K.KNOB_Y - 0.015)]))
        elif i == "burner-block":
            out.append(poly(s, [(BZ - h, ty + 0.048), (BZ - h, ty), (BZ - h + 0.015, ty), (BZ - h + 0.015, ty + 0.033), (BZ - 0.062, ty + 0.033), (BZ - 0.062, K.FLAME_Y), (BZ - 0.05, K.FLAME_Y), (BZ - 0.05, ty), (BZ + 0.05, ty), (BZ + 0.05, K.FLAME_Y), (BZ + 0.062, K.FLAME_Y), (BZ + 0.062, ty + 0.033), (BZ + h - 0.015, ty + 0.033), (BZ + h - 0.015, ty), (BZ + h, ty), (BZ + h, ty + 0.048)]))
        else:
            raise SystemExit("unhandled 1:200 front shape " + i)
    return out


def plan200(old):
    h = K.TRIVET_HALF
    out = []
    for s in old:
        i = s["id"]
        if i == "plan-plate":
            out.append(poly(s, plate_with_knob(0.012)))
        elif i == "plan-burner":
            p = ellipse(s, 0.0, BZ, 0.062)
            p["regions"] = ["burner", "flame"]
            out.append(p)
            t = {"id": "plan-trivet", "bone": "hob", "role": "body", "z": 3, "filled": True, "tone": 0.45, "region": "trivet", "material": "cast-iron"}
            out.append(path(t, [rounded(-h, BZ - h, h, BZ + h, 0.018), list(reversed(rounded(-h + 0.015, BZ - h + 0.015, h - 0.015, BZ + h - 0.015, 0.003, 1)))]))
        else:
            raise SystemExit("unhandled 1:200 plan shape " + i)
    return out


ADDED = {"50": {"elevation": ("feet",), "front": ("feet",), "plan": ("plan-plate-well",)}, "100": {"elevation": ("knob",), "front": ("knob",), "plan": ("plan-knob",)}, "200": {"elevation": ("trivet-leg-left", "trivet-leg-right"), "plan": ("plan-trivet",)}}
VIEWS = {"50": (elevation50, front50, plan50), "100": (elevation100, front100, plan100), "200": (elevation200, front200, plan200)}


def geometry(den, old):
    """🖊️ The redrawn geometry of one scale: every view from its own function, in the order of the old list."""
    e, f, p = VIEWS[den]
    fresh = {k: [s for s in v if s["id"] not in ADDED[den].get(k, ())] for k, v in old.items()}
    return {"elevation": e(fresh["elevation"]), "plan": p(fresh["plan"]), "front": f(fresh["front"])}


def decisions(den, old):
    texts = json.load(open(TEXTS, encoding="utf-8"))["drawings"][den]
    return {k: texts.get(k, v) for k, v in old.items()}
