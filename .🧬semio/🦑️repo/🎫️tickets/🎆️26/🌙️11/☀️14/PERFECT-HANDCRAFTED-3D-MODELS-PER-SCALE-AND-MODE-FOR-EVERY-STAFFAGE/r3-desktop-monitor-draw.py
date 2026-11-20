"""✏ Round 3, appliances/desktop-monitor: the 2D drawings of every authored scale as the true orthographic views of that scale's body (ruling E1), drawn by hand from the same section tables as the solids.

Frames (measured with r3-project2d): elevation bone-local (base u ahead from the rear edge, neck u up and v backwards, panel u up the screen and v backwards); front x = world z, y = world height; plan x = world x ahead, y = world z. The recessed cover glass is hidden by the bezel in the side elevation and is drawn only where it shows: in the front view and as the slanted strip seen from above.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_desktop_monitor_" + name, os.path.join(HERE, f"r3-desktop-monitor-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
B = _load("body")

STAND = {"region": "stand", "material": "stand-plastic"}
FEET = {"region": "stand", "material": "rubber-foot"}
FRONT = {"region": "panel-front", "material": "bezel-plastic"}
GLASS = {"region": "panel-front", "material": "display-glass"}
HOUSING = {"region": "housing", "material": "housing-plastic"}


def n(value):
    """🔢 A coordinate as written in the file."""
    return round(value + 0.0, 4)


def shape(sid, bone, z, tone, geometry, tags, role="body", filled=True, **more):
    """🧾 One authored shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(tags)
    out.update(more)
    return out


def rect(x0, y0, x1, y1, r=0.0):
    """▭ A rectangle between two corners, optionally rounded."""
    out = {"kind": "rect", "x": n(min(x0, x1)), "y": n(min(y0, y1)), "w": n(abs(x1 - x0)), "h": n(abs(y1 - y0))}
    if r:
        out["r"] = r
    return out


def poly(points, closed=True):
    """🔷 A polygon or an open polyline."""
    return {"kind": "poly", "points": [[n(x), n(y)] for x, y in points], "closed": closed}


def ellipse(cx, cy, r):
    """⚪ A circle."""
    return {"kind": "ellipse", "cx": n(cx), "cy": n(cy), "rx": r, "ry": r}


def lateral(rows, height):
    """↔ The front outline of a loft table (s, centre, ry, half width rx, n) whose s maps to a height."""
    right = [(r[3], height(r[0])) for r in rows]
    return right + [(-x, y) for x, y in right[::-1]]


def squircle(cx, cy, rx, ry, exponent, steps):
    """🟢 A plan superellipse with x ahead and y lateral."""
    return [(x, y) for y, x in K.superellipse(cy, cx, ry, rx, exponent, steps)]


def plan_x(u, v):
    """🗺 The plan x of a panel-local point."""
    return K.panel_world(u, v)[0]


def front_y(u, v):
    """🗼 The front height of a panel-local point."""
    return K.panel_world(u, v)[1]


def housing_plan(rows, front_x, fraction):
    """🗺 The plan outline of the rear cover: behind the panel edge each section reaches back to its crest over most of its width."""
    right = [(front_x, K.PANEL_W)] + [(plan_x(r[1] + r[2], r[0]), r[3] * fraction) for r in rows[1:]]
    return right + [(x, -y) for x, y in right[::-1]]


def base_elevation(rows):
    """📐 The side outline of the base plate (u ahead, v up) from its section table."""
    lo = [(r[1] - r[2], r[0]) for r in rows]
    hi = [(r[1] + r[2], r[0]) for r in rows]
    return hi + lo[::-1]


def neck_elevation(rows, start):
    """📐 The side outline of the riser (u up, v backwards) from the plate top."""
    rows = [interp(rows, start)] + [r for r in rows if r[0] > start]
    front = [(r[0], r[1] - r[2]) for r in rows]
    back = [(r[0], r[1] + r[2]) for r in rows]
    return front + back[::-1]


def interp(rows, s):
    """📏 A section of a table interpolated at s."""
    for a, b in zip(rows, rows[1:]):
        if a[0] <= s <= b[0]:
            t = (s - a[0]) / (b[0] - a[0])
            return tuple(a[k] * (1 - t) + b[k] * t for k in range(4)) + (a[4],)
    raise ValueError(s)


def housing_elevation(rows):
    """📐 The side outline of the rear cover (u up the screen, v backwards)."""
    top = [(r[1] + r[2], r[0]) for r in rows]
    bottom = [(r[1] - r[2], r[0]) for r in rows]
    return bottom[:1] + top + bottom[::-1][:-1]


def neck_front(rows, start, head):
    """🗼 The front outline of the riser with the hinge barrel at its head."""
    rows = [interp(rows, start)] + [r for r in rows if r[0] > start]
    right = [(r[3], K.NECK_Y + r[0]) for r in rows if r[0] < 0.24] + head
    return right + [(-x, y) for x, y in right[::-1]]


def vents_plan():
    """🌬 The band of the four vent slots seen from above: their 3 mm pitch and the 11 mm band are below what 1:50 separates, so the band is one detail stroke across the slope of the hump along its middle."""
    (um, vm), _, _, _ = B.VENT
    x = plan_x(um, vm)
    return [(x, -0.10), (x, 0.10)]


def scale50():
    """🖊 1:50."""
    elevation = [
        shape("foot-front", "base", 3, 0.15, rect(0.174, 0, 0.196, K.FOOT_H), FEET),
        shape("foot-rear", "base", 3, 0.15, rect(0.014, 0, 0.036, K.FOOT_H), FEET),
        shape("base-plate", "base", 3.1, 0.5, poly(base_elevation(B.BASE_50)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(neck_elevation(B.NECK_50, -0.001)), STAND),
        shape("mount-plate", "panel", 3.15, 0.42, rect(-0.05, -0.0055, 0.04, 0.0025, 0.002), STAND),
        shape("housing", "panel", 3.4, 0.55, poly(housing_elevation([r for r in B.HOUSING_50 if r[0] not in (-0.029, -0.0045)])), HOUSING),
        shape("panel-slab", "panel", 3.6, 0.3, rect(K.PANEL_U0, K.FRONT_V, K.PANEL_U1, K.MODULE_V, 0.0015), FRONT),
    ]
    bottom, top = front_y(K.PANEL_U0, K.MODULE_V), front_y(K.PANEL_U1, K.FRONT_V)
    front = [
        shape("foot-left", "base", 3, 0.15, rect(0.084, 0, 0.106, K.FOOT_H), FEET),
        shape("foot-right", "base", 3, 0.15, rect(-0.106, 0, -0.084, K.FOOT_H), FEET),
        shape("base-plate", "base", 3.1, 0.5, poly(lateral(B.BASE_50, lambda v: v)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(neck_front(B.NECK_50, -0.001, [(0.0305, 0.245), (0.0305, 0.261), (0.026, 0.265)])), STAND),
        shape("panel-slab", "panel", 3.6, 0.3, rect(-K.PANEL_W, bottom, K.PANEL_W, top, 0.005), FRONT, followTip=True),
        shape("screen-glass", "panel", 3.7, 0.1, rect(-K.GLASS_W, front_y(K.GLASS_U0, K.GLASS_V), K.GLASS_W, front_y(K.GLASS_U1, K.GLASS_V), 0.0015), GLASS, role="glazing", followTip=True),
        shape("power-led", "panel", 3.8, 0.9, ellipse(-0.24, front_y(-0.1605, K.FRONT_V - 0.0015), 0.003), FRONT, role="detail"),
    ]
    plan = [
        shape("base-plate", "base", 3.1, 0.5, poly(squircle(K.BASE_X0 + 0.105, 0.0, 0.105, 0.125, 6, 40)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(squircle(K.NECK_X - 0.010, 0.0, 0.018, 0.040, 4, 20)), STAND),
        shape("housing", "panel", 3.4, 0.55, poly(housing_plan([r for r in B.HOUSING_50 if r[0] not in (-0.029, -0.0045)], -0.040, 0.93)), HOUSING),
        shape("vents", "panel", 3.5, 0.3, poly(vents_plan(), closed=False), HOUSING, role="detail", filled=False),
        shape("mount-plate", "panel", 3.45, 0.42, rect(-0.066, -0.04, -0.054, 0.04), STAND),
        shape("panel-slab", "panel", 3.6, 0.3, rect(plan_x(K.PANEL_U1, K.MODULE_V), -K.PANEL_W, plan_x(K.PANEL_U0, K.FRONT_V), K.PANEL_W, 0.002), FRONT),
        shape("screen-glass", "panel", 3.7, 0.1, rect(plan_x(K.GLASS_U1, K.GLASS_V), -K.GLASS_W, plan_x(K.GLASS_U0, K.GLASS_V), K.GLASS_W), GLASS, role="glazing"),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def scale100():
    """🖊 1:100."""
    gu0, gu1, gw, gv = -0.148, 0.140, 0.255, -0.0392
    bottom, top = front_y(K.PANEL_U0, -0.0295), front_y(K.PANEL_U1, K.FRONT_V)
    elevation = [
        shape("base-plate", "base", 3.1, 0.5, poly(base_elevation(B.BASE_100)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(neck_elevation(B.NECK_100, -0.001)), STAND),
        shape("housing", "panel", 3.4, 0.55, poly(housing_elevation(B.HOUSING_100)), HOUSING),
        shape("panel-slab", "panel", 3.6, 0.3, rect(K.PANEL_U0, K.FRONT_V, K.PANEL_U1, -0.0295), FRONT),
    ]
    front = [
        shape("base-plate", "base", 3.1, 0.5, poly(lateral(B.BASE_100, lambda v: v)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(neck_front(B.NECK_100, -0.001, [(0.0305, 0.252), (0.02, 0.264)])), STAND),
        shape("panel-slab", "panel", 3.6, 0.3, rect(-K.PANEL_W, bottom, K.PANEL_W, top, 0.005), FRONT, followTip=True),
        shape("screen-glass", "panel", 3.7, 0.1, rect(-gw, front_y(gu0, gv), gw, front_y(gu1, gv), 0.002), GLASS, role="glazing", followTip=True),
    ]
    plan = [
        shape("base-plate", "base", 3.1, 0.5, poly(squircle(K.BASE_X0 + 0.105, 0.0, 0.105, 0.125, 6, 28)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(squircle(K.NECK_X - 0.010, 0.0, 0.018, 0.040, 4, 16)), STAND),
        shape("housing", "panel", 3.4, 0.55, poly(housing_plan(B.HOUSING_100, -0.040, 0.93)), HOUSING),
        shape("panel-slab", "panel", 3.6, 0.3, rect(plan_x(K.PANEL_U1, -0.0295), -K.PANEL_W, plan_x(K.PANEL_U0, K.FRONT_V), K.PANEL_W), FRONT),
        shape("screen-glass", "panel", 3.7, 0.1, rect(plan_x(gu1, gv), -gw, plan_x(gu0, gv), gw), GLASS, role="glazing"),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def panel_plan200():
    """🗺 The 1:200 panel from above: the slab from its lower front edge to its upper back edge over the full width, then the hump narrowing backwards."""
    right = [(plan_x(K.PANEL_U0, K.FRONT_V), K.PANEL_W), (plan_x(K.PANEL_U1, B.SLAB_200[1][0]), K.PANEL_W)] + [(plan_x(r[1] + r[2], r[0]), r[3] * 0.93) for r in B.HOUSING_200]
    return right + [(x, -y) for x, y in right[::-1]]


def scale200():
    """🖊 1:200."""
    rows = B.SLAB_200 + B.HOUSING_200
    merged = {"regions": ["panel-front", "housing"], "material": "bezel-plastic"}
    bottom, top = front_y(K.PANEL_U0, -0.0275), front_y(K.PANEL_U1, K.FRONT_V)
    elevation = [
        shape("base-plate", "base", 3.1, 0.5, poly(base_elevation(B.BASE_200)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(neck_elevation(B.NECK_200, -0.001)), STAND),
        shape("panel", "panel", 3.4, 0.4, poly(housing_elevation(rows)), merged),
    ]
    front = [
        shape("base-plate", "base", 3.1, 0.5, poly(lateral(B.BASE_200, lambda v: v)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(neck_front(B.NECK_200, -0.001, [(0.026, 0.262)])), STAND),
        shape("panel", "panel", 3.4, 0.4, rect(-K.PANEL_W, bottom, K.PANEL_W, top, 0.008), merged, followTip=True),
    ]
    plan = [
        shape("base-plate", "base", 3.1, 0.5, poly(squircle(K.BASE_X0 + 0.105, 0.0, 0.105, 0.125, 6, 20)), STAND),
        shape("neck", "neck", 3.2, 0.45, poly(squircle(K.NECK_X - 0.012, 0.0, 0.019, 0.038, 4, 12)), STAND),
        shape("panel", "panel", 3.4, 0.4, poly(panel_plan200()), merged),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def apply(asset):
    """🖊 Replaces the geometry of every authored scale."""
    for scale, draw in (("50", scale50), ("100", scale100), ("200", scale200)):
        asset["representations"][scale]["geometry"] = draw()
