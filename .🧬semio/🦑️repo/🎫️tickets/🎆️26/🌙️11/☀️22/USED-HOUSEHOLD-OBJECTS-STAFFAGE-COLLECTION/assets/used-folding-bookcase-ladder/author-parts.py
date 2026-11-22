"""🧩️ The parts of the used folding ladder shelf per authored scale: every part is a lateral extrusion of a bone-local profile, so the same profile and span give the body solid and its three drawings.

A part: `id`, `bone`, `material`, `region` (None = hardware without thermal region), `tone`, `role`, `profile` (bone-local u, v), `z` (z0, z1 world lateral), `pen` (paper mm), flags (`slender`, `sheet`, `texture`), `views` (subset of elevation, front, plan), `solid3d` (False = drawing only), `filled`.

@see ./author-geometry.py"""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("geometry", os.path.join(os.path.dirname(__file__), "author-geometry.py"))
g = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g)

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}
ARC = {50: 12, 100: 8, 200: 4, 500: 2}


def r5(points):
    """🔢️ Rounds a profile to 0.01 mm."""
    return [[round(u, 5), round(v, 5)] for u, v in points]


def front_rail_profile(scale):
    """🪜️ The front rail in ladder-local coordinates: cut level with the floor at the foot, rounded about the hinge bolt at the top."""
    h = g.RAIL_HALF
    foot = h * g.TAN
    cap = [(g.LADDER_LEN + h * math.cos(math.radians(a)), h * math.sin(math.radians(a))) for a in [-90 + 180 * k / ARC[scale] for k in range(ARC[scale] + 1)]]
    return r5([(-foot, -h)] + cap + [(foot, h)])


def rear_rail_profile(scale):
    """🧱️ The rear rail in rear-local coordinates (u down from the hinge, v forward): 34 mm deep, square foot, the top corner towards the hinge chamfered at 1:50 and 1:100."""
    v0, v1 = -(g.PIVOT_X - g.REAR_X[0]), -(g.PIVOT_X - g.REAR_X[1])
    top = g.PIVOT_Y - g.REAR_TOP
    if scale in (50, 100):
        return r5([(top, v0), (g.PIVOT_Y, v0), (g.PIVOT_Y, v1), (top + 0.006, v1), (top, v1 - 0.006)])
    return r5([(top, v0), (g.PIVOT_Y, v0), (g.PIVOT_Y, v1), (top, v1)])


def shelf_profile(i, scale, chipped=False):
    """📚️ Shelf i in shelf-local coordinates (u back from the hinge pin, v down): 18 mm board with an eased front edge, thickened to the smallest feature at 1:200 and 1:500."""
    end = g.shelf_end_u(i)
    f = -g.SHELF_FRONT
    if scale == 200:
        return r5([(f, 0.015), (end, 0.015), (end, -0.015), (f, -0.015)])
    if scale == 500:
        return r5([(f, 0.0375), (end, 0.0375), (end, -0.0375), (f, -0.0375)])
    t = g.SHELF_T / 2
    if chipped:
        return r5([(f, t), (end, t), (end, -t), (0.006, -t), (-0.004, -0.003), (f, 0.002)])
    if scale == 50:
        return r5([(f, t), (end, t), (end, -t), (f + 0.004, -t), (f, -t + 0.004)])
    return r5([(f, t), (end, t), (end, -t), (f, -t)])


def polygon_circle(cx, cy, r, n, phase=0.0):
    """⭕️ A regular n-gon about (cx, cy)."""
    return r5([(cx + r * math.cos(math.radians(phase + 360 * k / n)), cy + r * math.sin(math.radians(phase + 360 * k / n))) for k in range(n)])


def hull(points):
    """🫧️ Convex hull (counter-clockwise) of 2D points."""
    pts = sorted(set((round(x, 6), round(y, 6)) for x, y in points))
    if len(pts) <= 2:
        return pts
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


def strap_profile():
    """🔩️ The 4 mm steel hinge strap on the outer face of both rails, bolted to the rear rail and turning on the hinge bolt (rear-local)."""
    world = [(0.006, 1.738), (0.006, 1.784), (0.030, 1.738), (0.030, 1.784)] + g.arc(g.PIVOT_X, g.PIVOT_Y, 0.017, 0, 360, 16)
    return g.local_poly("rear", hull(world))


def cleat_profile(i):
    """🪵️ The beech cleat under the rear end of shelf i on the inner face of a rear rail (rear-local)."""
    y1 = g.SHELF_TOPS[i] - g.SHELF_T
    y0 = y1 - g.CLEAT_H
    return g.local_poly("rear", [(g.REAR_X[0], y0), (g.REAR_X[1], y0), (g.REAR_X[1], y1), (g.REAR_X[0], y1)])


def blanket_profile(scale):
    """🧺️ The folded moving blanket (blanket-local: u ahead, v up), its folded ends rounded at 1:50 and 1:100."""
    L, t = g.BLANKET[0], g.BLANKET[1]
    if scale in (50, 100):
        n = 6 if scale == 50 else 4
        right = [(L - t / 2 + t / 2 * math.cos(math.radians(-90 + 180 * k / n)), t / 2 + t / 2 * math.sin(math.radians(-90 + 180 * k / n))) for k in range(n + 1)]
        left = [(t / 2 + t / 2 * math.cos(math.radians(90 + 180 * k / n)), t / 2 + t / 2 * math.sin(math.radians(90 + 180 * k / n))) for k in range(n + 1)]
        return r5(right + left)
    return r5([(0, 0), (L, 0), (L, t), (0, t)])


def anchor_profile():
    """🧱️ The pine wall-anchor block (anchor-local: u along its 60 mm, v away from the wall face)."""
    return r5([(0, -g.ANCHOR[1]), (g.ANCHOR[0], -g.ANCHOR[1]), (g.ANCHOR[0], 0), (0, 0)])


def mirror(z):
    """🪞️ The span mirrored to the right side."""
    return (-z[1], -z[0])


def parts(scale):
    """📋️ Every part of the asset at one scale."""
    out = []
    rail_pen = {50: 0.35, 100: 0.25, 200: 0.18, 500: 0.18}[scale]
    shelf_pen = {50: 0.25, 100: 0.18, 200: 0.15, 500: 0.18}[scale]
    slender_rails = scale in (200, 500)

    def add(**kw):
        kw.setdefault("role", "body")
        kw.setdefault("views", ("elevation", "front", "plan"))
        kw.setdefault("solid3d", True)
        kw.setdefault("filled", True)
        kw.setdefault("flags", {})
        out.append(kw)

    fr = front_rail_profile(scale)
    for side, z in (("l", g.RAIL_W), ("r", mirror(g.RAIL_W))):
        add(id=f"front-rail-{side}", bone="ladder", material="pine-aged", region="front-rails", tone=0.62, profile=fr, z=z, pen=rail_pen, views=("elevation", "front", "plan") if side == "r" else ("front", "plan"), flags={"slender": True} if slender_rails else {})
    rr = rear_rail_profile(scale)
    for side, z in (("l", g.RAIL_W), ("r", mirror(g.RAIL_W))):
        add(id=f"rear-rail-{side}", bone="rear", material="pine-aged", region="rear-rails", tone=0.5, profile=rr, z=z, pen=rail_pen, views=("elevation", "front", "plan") if side == "r" else ("front", "plan"), flags={"slender": True} if slender_rails else {})
    zs = {50: (-g.SHELF_Z, g.SHELF_Z), 100: (-0.272, 0.272), 200: (-0.272, 0.272), 500: (-0.272, 0.272)}[scale]
    for i in range(5):
        tone = [0.72, 0.72, 0.74, 0.8, 0.84][i]
        if scale == 50 and i == 1:
            add(id="shelf-2", bone="shelf-2", material="pine-board", region="shelves", tone=tone, profile=shelf_profile(i, scale), z=(-0.243, zs[1]), pen=shelf_pen)
            add(id="shelf-2-chip", bone="shelf-2", material="pine-board", region="shelves", tone=tone, profile=shelf_profile(i, scale, chipped=True), z=(zs[0], -0.243), pen=shelf_pen)
        else:
            add(id=f"shelf-{i + 1}", bone=f"shelf-{i + 1}", material="pine-board", region="shelves", tone=tone, profile=shelf_profile(i, scale), z=zs, pen=shelf_pen)
    if scale in (50, 100):
        for i in range(5):
            for side, z in (("l", g.CLEAT_Z), ("r", mirror(g.CLEAT_Z))):
                add(id=f"cleat-{i + 1}-{side}", bone="rear", material="beech", region="rear-rails", tone=0.42, profile=cleat_profile(i), z=z, pen=0.18, views=("front", "plan"))
    if scale == 50:
        for i in range(5):
            for side, z in (("l", (0.262, 0.278)), ("r", (-0.278, -0.262))):
                add(id=f"pin-{i + 1}-{side}", bone=f"shelf-{i + 1}", material="steel-zinc", region=None, tone=0.28, profile=polygon_circle(0, 0, g.PIN_R, 12), z=z, pen=0.13, views=())
        add(id="strap-l", bone="rear", material="steel-zinc", region=None, tone=0.3, profile=strap_profile(), z=(0.300, 0.304), pen=0.18, views=(), flags={"texture": True})
        add(id="strap-r", bone="rear", material="steel-zinc", region=None, tone=0.3, profile=strap_profile(), z=(-0.304, -0.300), pen=0.18, role="detail", views=("elevation",), filled=False, flags={"texture": True})
        add(id="bolt-l", bone="ladder", material="steel-zinc", region="front-rails", tone=0.3, profile=polygon_circle(g.LADDER_LEN, 0, 0.010, 16), z=(0.304, 0.312), pen=0.18, views=("front", "plan"))
        add(id="bolt-r", bone="ladder", material="steel-zinc", region="front-rails", tone=0.2, profile=polygon_circle(g.LADDER_LEN, 0, 0.013, 6, 30), z=(-0.312, -0.304), pen=0.18)
        foot = g.RAIL_HALF * g.TAN
        add(id="scuff-r", bone="ladder", material="pine-aged", region="front-rails", tone=0.35, profile=r5([(-foot, -g.RAIL_HALF), (0.05, -g.RAIL_HALF), (0.05, g.RAIL_HALF), (foot, g.RAIL_HALF)]), z=mirror(g.RAIL_W), pen=0.13, role="detail", solid3d=False)
        add(id="scuff-l", bone="ladder", material="pine-aged", region="front-rails", tone=0.35, profile=r5([(-foot, -g.RAIL_HALF), (0.05, -g.RAIL_HALF), (0.05, g.RAIL_HALF), (foot, g.RAIL_HALF)]), z=g.RAIL_W, pen=0.13, role="detail", solid3d=False, views=("front",))
        add(id="sticker", bone="ladder", material="pine-aged", region="front-rails", tone=0.95, profile=r5([(1.10, -0.014), (1.145, -0.014), (1.145, 0.014), (1.10, 0.014)]), z=mirror(g.RAIL_W), pen=0.13, role="texture", solid3d=False, views=("elevation",), filled=False)
    add(id="blanket", bone="blanket", material="felt-blanket", region="blanket", tone=0.3, profile=blanket_profile(scale), z=(-g.BLANKET[2], g.BLANKET[2]), pen={50: 0.35, 100: 0.25, 200: 0.18, 500: 0.18}[scale], flags={"sheet": True})
    if scale in (50, 100):
        add(id="wall-anchor", bone="wall-anchor", material="pine-raw", region="rear-rails", tone=0.55, profile=anchor_profile(), z=g.ANCHOR[2], pen={50: 0.25, 100: 0.18}[scale])
    return out


def volume_of(part):
    """📦️ Volume (m³) and bone-local centroid of the 3D part."""
    area, (cu, cv) = g.poly_area_centroid(part["profile"])
    return area * (part["z"][1] - part["z"][0]), (cu, cv, (part["z"][0] + part["z"][1]) / 2)


def outer_area(part):
    """🌫️ Full surface of the prism: both end faces and the mantle."""
    area, _ = g.poly_area_centroid(part["profile"])
    return 2 * area + g.perimeter(part["profile"]) * (part["z"][1] - part["z"][0])
