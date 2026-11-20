"""✏️ Round 3 electric jug kettle: the 2D drawings of every authored scale and view redrawn as the true orthographic views of that scale's model (ruling E1) from the same measures as the solids; every shape keeps its id, bone, role, layer order, tone, region and material, only its geometry changes, plus the new skirt line and switch at 1:50."""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-electric-kettle-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
TEXTS = os.path.join(HERE, "r3-electric-kettle-texts.json")
BODY_Y = 0.03
LID_Y = BODY_Y + K.RIM_V
HANDLE_V = K.HANDLE_Y


def r4(p):
    return [round(p[0], 4) + 0.0, round(p[1], 4) + 0.0]


def mirror(half):
    """🪞 A closed symmetric outline from its right half given as (x, y) from bottom to top."""
    right = [r4(p) for p in half]
    left = [r4((-x, y)) for x, y in reversed(half) if abs(x) > 1e-9]
    return right + left


def profile_half(profile, lo=-1.0, dy=0.0):
    """📐 The right half of a revolve's outline as (radius, height + dy), dropping the axis points and everything below lo."""
    return [(r, a + dy) for a, r in profile if r > 1e-9 and a >= lo - 1e-9]


def poly(shape, points, **extra):
    s = {k: v for k, v in shape.items() if k not in ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "points", "closed")}
    s.update({"kind": "poly", "points": [r4(p) for p in points], "closed": True})
    s.update(extra)
    return s


def rect(shape, x, y, w, h, r=None):
    s = {k: v for k, v in shape.items() if k not in ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "points", "closed")}
    s.update({"kind": "rect", "x": round(x, 4), "y": round(y, 4), "w": round(w, 4), "h": round(h, 4)})
    if r:
        s["r"] = r
    return s


def circle(shape, r):
    s = {k: v for k, v in shape.items() if k not in ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "points", "closed")}
    s.update({"kind": "ellipse", "cx": 0, "cy": 0, "rx": r, "ry": r})
    return s


def edges(path):
    left, right = K.section_points(path)
    return left, right


def outside_wall(edge, wall=K.r_out):
    """✂️ The part of a spout edge (from root to tip) that lies outside the jug wall, starting at the crossing."""
    out = []
    for k in range(1, len(edge)):
        a, b = edge[k - 1], edge[k]
        fa, fb = -a[0] - wall(a[1]), -b[0] - wall(b[1])
        if fa < 0 <= fb:
            t = -fa / (fb - fa)
            out.append((a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])))
        if fb >= 0:
            out.append(b)
    return out


def spout_elevation(rows):
    upper, lower = edges(rows)
    if upper[-1][1] < lower[-1][1]:
        upper, lower = lower, upper
    upper = [(2 * upper[0][0] - upper[1][0], 2 * upper[0][1] - upper[1][1])] + upper
    lower = [(2 * lower[0][0] - lower[1][0], 2 * lower[0][1] - lower[1][1])] + lower
    up, lo = outside_wall(upper), outside_wall(lower)
    tip = rows[-1]
    d = (rows[-1][0] - rows[-2][0], rows[-1][1] - rows[-2][1])
    n = math.hypot(*d)
    cap = (tip[0] + d[0] / n * tip[2] * 0.6, tip[1] + d[1] / n * tip[2] * 0.6)
    return lo + [cap] + list(reversed(up))


def handle_elevation(path):
    outer, inner = edges(path)
    return outer + list(reversed(inner))


def spout_front(rows, dy):
    """🔭 Front view of the spout: hull of its sections projected on (lateral, height)."""
    pts = []
    for k, (u, v, rx, ry) in enumerate(rows):
        a, b = rows[max(k - 1, 0)], rows[min(k + 1, len(rows) - 1)]
        tx, ty = b[0] - a[0], b[1] - a[1]
        n = math.hypot(tx, ty)
        ny = abs(tx / n)
        for j in range(24):
            t = 2 * math.pi * j / 24
            pts.append((ry * math.cos(t), v + dy + rx * ny * math.sin(t)))
    return hull(pts)


def spout_plan(rows, root_x):
    """🔭 Plan of the spout: its lateral half widths from the wall to the tip, tip rounded by the tilted last section."""
    r0 = rows[0]
    r1 = rows[1]
    t = (root_x - r0[0]) / (r1[0] - r0[0])
    half = [(root_x, r0[3] + t * (r1[3] - r0[3]))] + [(u, ry) for u, v, rx, ry in rows[1:]]
    tip = rows[-1]
    end = tip[0] - tip[2] * 0.6
    side = half + [(end, tip[3] * 0.5)]
    return [(x, -y) for x, y in side] + [(x, y) for x, y in reversed(side)]


def hull(pts):
    pts = sorted(set((round(x, 5), round(y, 5)) for x, y in pts))

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def body_with_spout(den):
    """🫖 Elevation outline of jug and spout in one (1:100, 1:200): right wall, rim, spout over the left rim, left wall."""
    half = K.SHELL_OUTLINE[den]
    right = [(r, v) for v, r in half]
    rim = right[-1]
    sp = spout_elevation(K.SPOUTS[den])
    left_wall = [(-r, v) for r, v in reversed(right) if v < sp[0][1]]
    return right + [(-rim[0], rim[1])] + list(reversed(sp)) + left_wall


def handle_extent(path):
    outer, inner = edges(path)
    ys = [p[1] for p in outer + inner]
    w = max(p[3] for p in path)
    xs = [p[0] for p in outer + inner]
    return min(ys), max(ys), w, max(xs)


def switch_points():
    u, v, hu, hv, _ = K.SWITCH
    return [(u - hu, v - hv), (u + hu, v - hv), (u + hu, v + hv), (u - hu, v + hv)]


def draw50(geo):
    by = {view: {s["id"]: s for s in shapes} for view, shapes in geo.items()}
    h = K.water_height()
    lo = K.FLOOR_TOP - K.WATER_Y
    water = [(K.r_in(K.FLOOR_TOP), lo), (K.r_in(K.FLOOR_TOP + h), lo + h)]
    body_half = [(r, v) for v, r in K.SHELL_OUTLINE["50"]]
    hy0, hy1, hw, hx = handle_extent(K.HANDLE["50"])
    knob = [(r, a) for a, r in K.KNOB if r > 1e-9 and a >= 0.0135]
    knob = [(0.0085, 0.0135)] + knob[1:]
    e, f, p = by["elevation"], by["front"], by["plan"]
    skirt_line = {"id": "skirt-line", "bone": "body", "role": "detail", "z": 3.45, "filled": False, "tone": 0.3, "kind": "poly", "points": [r4((-0.0746, K.SKIRT_TOP)), r4((0.0746, K.SKIRT_TOP))], "closed": False, "region": "shell", "material": "pp-plastic"}
    switch = {"id": "switch", "bone": "handle", "role": "detail", "z": 3.55, "filled": True, "tone": 0.65, "kind": "poly", "points": [r4(q) for q in switch_points()], "closed": True, "region": "handle", "material": "pp-plastic"}
    elevation = [
        poly(e["water"], mirror(water)),
        poly(e["spout"], spout_elevation(K.SPOUTS["50"])),
        poly(e["body"], mirror(body_half)),
        skirt_line,
        rect(e["gauge"], -0.014, 0.02, 0.028, 0.12, 0.0075),
        poly(e["handle"], handle_elevation(K.HANDLE["50"])),
        switch,
        poly(e["lid"], mirror(profile_half(K.LID["50"], lo=-0.006))),
        poly(e["lid-button"], mirror(knob)),
        poly(e["base"], mirror(profile_half(K.BASE["50"])[:-2])),
    ]
    sw = switch_points()
    sy = [q[1] + BODY_Y + HANDLE_V for q in sw]
    front_line = dict(skirt_line, points=[r4((-0.0746, BODY_Y + K.SKIRT_TOP)), r4((0.0746, BODY_Y + K.SKIRT_TOP))])
    front = [
        poly(f["base"], mirror(profile_half(K.BASE["50"])[:-2])),
        poly(f["body"], mirror([(x, y + BODY_Y) for x, y in body_half])),
        poly(f["spout"], spout_front(K.SPOUTS["50"], BODY_Y)),
        front_line,
        rect(f["handle"], -hw, hy0 + BODY_Y + HANDLE_V, 2 * hw, hy1 - hy0, 0.01),
        poly(f["handle-grip"], [(0, hy0 + BODY_Y + HANDLE_V + 0.016), (0, hy1 + BODY_Y + HANDLE_V - 0.016)], closed=False),
        rect(dict(switch, z=3.56), -K.SWITCH[4], min(sy), 2 * K.SWITCH[4], max(sy) - min(sy), 0.002),
        poly(f["lid"], mirror(profile_half(K.LID["50"], lo=-0.006, dy=LID_Y))),
        poly(f["lid-button"], mirror([(x, y + LID_Y) for x, y in knob])),
    ]
    plan = [
        p["base"],
        p["body"],
        circle(p["body-shoulder"], 0.0699),
        poly(p["spout"], spout_plan(K.SPOUTS["50"], -0.066)),
        rect(p["handle"], 0.068, -hw, K.HANDLE_X + hx - 0.068, 2 * hw, 0.012),
        p["lid"],
        circle(p["lid-button"], 0.0125),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def draw_coarse(den, geo):
    by = {view: {s["id"]: s for s in shapes} for view, shapes in geo.items()}
    hy0, hy1, hw, hx = handle_extent(K.HANDLE[den])
    out = {}
    e, f, p = by["elevation"], by["front"], by["plan"]
    lid_half = profile_half(K.LID[den], lo=-0.004)
    base_half = profile_half(K.BASE[den])
    body_half = [(r, v) for v, r in K.SHELL_OUTLINE[den]]
    elevation = []
    if den == "100":
        h = K.water_height()
        lo = K.FLOOR_TOP - K.WATER_Y
        elevation.append(poly(e["water"], mirror([(K.r_in(K.FLOOR_TOP), lo), (K.r_in(K.FLOOR_TOP + h), lo + h)])))
    elevation += [
        poly(e["body"], body_with_spout(den)),
        poly(e["handle"], handle_elevation(K.HANDLE[den])),
        poly(e["lid"], mirror(lid_half)),
        poly(e["base"], mirror(base_half)),
    ]
    out["elevation"] = elevation
    plan_handle = rect(p["handle"], 0.068, -hw, K.HANDLE_X + hx - 0.068, 2 * hw, 0.012) if "handle" in p else None
    if den == "100":
        out["plan"] = [p["base"], circle(p["body"], 0.075), poly(p["spout"], spout_plan(K.SPOUTS[den], -0.066)), plan_handle, circle(p["lid"], 0.066)]
        out["front"] = [
            poly(f["base"], mirror(base_half)),
            poly(f["body"], mirror([(x, y + BODY_Y) for x, y in body_half])),
            rect(f["handle"], -hw, hy0 + BODY_Y + HANDLE_V, 2 * hw, hy1 - hy0, 0.01),
            poly(f["lid"], mirror([(x, y + LID_Y) for x, y in lid_half])),
        ]
    else:
        sp = spout_plan(K.SPOUTS[den], -0.06)
        hd = [(0.068, -hw), (K.HANDLE_X + hx, -hw), (K.HANDLE_X + hx, hw), (0.068, hw)]
        out["plan"] = [p["base"], poly(p["jug"], union_plan(sp, hd))]
        kettle = base_half[:-1] + [(0.0778, 0.04)] + [(x, y + BODY_Y) for x, y in body_half if y + BODY_Y > 0.045]
        out["front"] = [
            poly(f["kettle"], mirror(kettle)),
            rect(f["handle"], -hw, hy0 + BODY_Y + HANDLE_V, 2 * hw, hy1 - hy0, 0.01),
            poly(f["lid"], mirror([(x, y + LID_Y) for x, y in lid_half])),
        ]
    return out


def union_plan(spout, handle, r=0.078):
    """🧩 One plan outline of jug, spout and handle (1:200): the circle walked counter-clockwise from the bottom, the handle bar spliced in on the right, the spout wedge on the left."""
    sl = max(abs(y) for x, y in spout if x > -0.07)
    hl = max(abs(y) for x, y in handle)
    xr, xs = math.sqrt(r * r - hl * hl), math.sqrt(r * r - sl * sl)
    tip = [q for q in reversed(spout) if q[0] < -xs]
    out = []
    for a in range(-90, 270, 10):
        x, y = r * math.cos(math.radians(a)), r * math.sin(math.radians(a))
        if (x > 0 and abs(y) <= hl) or (x < 0 and abs(y) <= sl):
            continue
        out.append((x, y))
    res = []
    for k, (x, y) in enumerate(out):
        res.append((x, y))
        nxt = out[(k + 1) % len(out)]
        if x > 0 and y < 0 and nxt[1] > 0:
            res += [(xr, -hl), (handle[1][0], -hl), (handle[1][0], hl), (xr, hl)]
        if x < 0 and y > 0 and nxt[1] < 0:
            res += [(-xs, sl)] + tip + [(-xs, -sl)]
    return res


def geometry(den, old):
    return draw50(old) if den == "50" else draw_coarse(den, old)


def decisions(den, old):
    texts = json.load(open(TEXTS, encoding="utf-8")).get("drawings", {})
    return texts.get(den, old)
