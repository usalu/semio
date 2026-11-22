"""📐 Rest-pose kinematics and the 2D projections of the solids (side elevation, front elevation, plan) drawn to the same numbers."""
import math
import importlib.util
import pathlib

_g = importlib.util.spec_from_file_location("geo", pathlib.Path(__file__).with_name("author-geometry.py"))
G = importlib.util.module_from_spec(_g)
_g.loader.exec_module(G)

r6 = G.r6


def bone_rest():
    """🦴 Rest frames of every bone: origin (x, y, z) and rotation angle about world z (degrees)."""
    bx, by = G.XB, G.YB
    h = math.radians(G.HOSE_REST_DEG)
    return {
        "yard": ((0, 0, 0), 0),
        "crate": ((0, 0, 0), 0),
        "burner": ((bx, by, 0), 0),
        "hoses": ((bx + G.HOSE_ORIGIN[0], by + G.HOSE_ORIGIN[1], G.HOSE_Z), G.HOSE_REST_DEG),
        "air-damper": ((bx + G.DAMPER_ORIGIN[0], by + G.DAMPER_ORIGIN[1], 0), -90),
    }


def to_world(bone, p):
    """🌍 Bone-local (u, v, w) to world rest coordinates."""
    (ox, oy, oz), a = bone_rest()[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    u, v, w = p
    return (ox + c * u - s * v, oy + s * u + c * v, oz + w)


def solid_points(s, n=24):
    """🧮 Sample points of a solid in its bone frame (enough for convex projections)."""
    k = s["kind"]
    pts = []
    if k == "box":
        cu, cv, cw = s["transform"]["translation"]
        su, sv, sw = s["size"]
        reps = [(0, 0, 0)]
        if "repeat" in s:
            t = s["repeat"]["translation"]
            reps = [(t[0] * i, t[1] * i, t[2] * i) for i in range(s["repeat"]["count"])]
        for dx, dy, dz in reps:
            for a in (-1, 1):
                for b in (-1, 1):
                    for c in (-1, 1):
                        pts.append((cu + dx + a * su / 2, cv + dy + b * sv / 2, cw + dz + c * sw / 2))
    elif k == "extrude":
        prof = s["profile"]
        for a, b in prof:
            for t in (s["from"], s["to"]):
                if s["plane"] == "xy":
                    pts.append((a, b, t))
                elif s["plane"] == "yz":
                    pts.append((t, a, b))
                else:
                    pts.append((b, t, a))
    elif k == "revolve":
        ax = s["axis"]
        o = s["origin"]
        for a, r in s["profile"]:
            for i in range(n):
                t = 2 * math.pi * i / n
                c, d = r * math.cos(t), r * math.sin(t)
                if ax[0]:
                    pts.append((o[0] + a * ax[0], o[1] + c, o[2] + d))
                elif ax[1]:
                    pts.append((o[0] + c, o[1] + a * ax[1], o[2] + d))
                else:
                    pts.append((o[0] + c, o[1] + d, o[2] + a * ax[2]))
    elif k == "capsule":
        p0, p1, r = s["p0"], s["p1"], s["r0"]
        for p in (p0, p1):
            for i in range(n):
                t = 2 * math.pi * i / n
                for j in range(n // 2):
                    f = math.pi * j / (n // 2 - 1) - math.pi / 2
                    pts.append((p[0] + r * math.cos(f) * math.cos(t), p[1] + r * math.cos(f) * math.sin(t), p[2] + r * math.sin(f)))
    return pts


def hull(points):
    """🔺 Convex hull (Andrew's monotone chain), counter-clockwise."""
    p = sorted(set((r6(a), r6(b)) for a, b in points))
    if len(p) < 3:
        return [list(q) for q in p]

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lo, up = [], []
    for q in p:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], q) <= 1e-12:
            lo.pop()
        lo.append(q)
    for q in reversed(p):
        while len(up) >= 2 and cross(up[-2], up[-1], q) <= 1e-12:
            up.pop()
        up.append(q)
    return [list(q) for q in lo[:-1] + up[:-1]]


def common(s, view_depth, role=None):
    """🏷️ Shared shape members derived from a solid."""
    d = {"id": s["id"], "bone": s["bone"], "role": role or ("texture" if s.get("texture") else s["role"]), "z": round(view_depth, 4), "filled": True, "tone": s["tone"]}
    if s.get("region"):
        d["region"] = s["region"]
    d["material"] = s["material"]
    return d


def elevation_shape(s):
    """👁️ Side-elevation shape of a solid in its bone's (u, v) frame."""
    k = s["kind"]
    wpts = [to_world(s["bone"], p) for p in solid_points(s)]
    depth = 5 - 10 * min(p[2] for p in wpts)
    d = common(s, depth)
    if k == "box":
        cu, cv, _ = s["transform"]["translation"]
        su, sv, _ = s["size"]
        d.update({"kind": "rect", "x": r6(cu - su / 2), "y": r6(cv - sv / 2), "w": su, "h": sv})
        if s.get("cornerRadius"):
            d["r"] = s["cornerRadius"]
        return d
    if k == "extrude" and s["plane"] == "xy":
        d.update({"kind": "poly", "points": s["profile"], "closed": True})
        return d
    if k == "capsule":
        d.update({"kind": "capsule", "x0": s["p0"][0], "y0": s["p0"][1], "x1": s["p1"][0], "y1": s["p1"][1], "r0": s["r0"], "r1": s["r1"]})
        return d
    if k == "revolve" and s["axis"][0] and s["profile"][0] != s["profile"][-1]:
        o = s["origin"]
        top = [[r6(o[0] + a), r6(o[1] + r)] for a, r in s["profile"]]
        bot = [[r6(o[0] + a), r6(o[1] - r)] for a, r in reversed(s["profile"])]
        pts = []
        for q in top + bot:
            if not pts or pts[-1] != q:
                pts.append(q)
        if pts[0] == pts[-1]:
            pts.pop()
        d.update({"kind": "poly", "points": pts, "closed": True})
        return d
    loc = solid_points(s)
    h = hull([(p[0], p[1]) for p in loc])
    d.update({"kind": "poly", "points": h, "closed": True})
    return d


def front_shape(s):
    """🔭 Front-elevation shape (paper x = world z, y = world y) in the rest pose."""
    wpts = [to_world(s["bone"], p) for p in solid_points(s)]
    depth = 5 + 10 * max(p[0] for p in wpts)
    d = common(s, depth)
    d.update({"kind": "poly", "points": hull([(p[2], p[1]) for p in wpts]), "closed": True})
    return d


def plan_shape(s):
    """🗺️ Plan shape (paper x = world x, y = world z) in the rest pose."""
    wpts = [to_world(s["bone"], p) for p in solid_points(s)]
    depth = 5 + 10 * max(p[1] for p in wpts)
    d = common(s, depth)
    d.update({"kind": "poly", "points": hull([(p[0], p[2]) for p in wpts]), "closed": True})
    return d


def world_bounds(solids):
    """📏 World rest bounding box of a list of solids."""
    pts = [to_world(s["bone"], p) for s in solids if not s.get("texture") for p in solid_points(s)]
    return [min(p[i] for p in pts) for i in range(3)], [max(p[i] for p in pts) for i in range(3)]
