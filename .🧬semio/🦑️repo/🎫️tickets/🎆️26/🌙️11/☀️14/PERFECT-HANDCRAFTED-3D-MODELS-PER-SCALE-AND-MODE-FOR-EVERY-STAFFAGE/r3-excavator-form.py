"""📐 The hand-stated forms of the 22 t crawler excavator shared by its four scales: lateral stations, the track loop, and the side outlines of boom, stick, bucket, cab and counterweight. Every number is a dimension of the real machine class (Cat 320, Komatsu PC210LC-11) in metres; bone-local coordinates (u along the bone, v its normal, w to the left)."""
import importlib.util
import math
import os
import sys

here = os.path.dirname(os.path.abspath(__file__))


def load(name):
    key = name.replace("-", "_")
    if key in sys.modules:
        return sys.modules[key]
    spec = importlib.util.spec_from_file_location(key, os.path.join(here, name + ".py"))
    mod = importlib.util.module_from_spec(spec)
    sys.modules[key] = mod
    spec.loader.exec_module(mod)
    return mod


L = load("r3-excavator-lib")
K = load("r3-excavator-kin")

ZC = -0.08
HALF = 1.37
GAUGE = 1.19
SHOE = 0.60
SPROCKET = (-1.825, 0.47)
IDLER = (1.825, 0.45)
WRAP = 0.40
RUN = 1.42
GROUSER = 0.026
PLATE = 0.014
RAIL = 0.085
DECK_TOP = 0.28
HOOD = 1.24
CAB = {"rear": -0.38, "front": 1.44, "floor": 0.30, "top": 1.92, "right": 0.42, "left": HALF}
SLOT = (-0.58, 0.42)
TAIL = 2.95
CW_FRONT = 0.57
CW_TOP = 1.16
BOOM_W = (ZC - 0.26, ZC + 0.26)
STICK_W = (ZC - 0.165, ZC + 0.165)
BUCKET_W = (ZC - 0.60, ZC + 0.60)
CYL_Z = (ZC + 0.38, ZC - 0.38)


def tangent_from(p, c, r, lowest=True):
    d = math.dist(p, c)
    phi = math.atan2(p[1] - c[1], p[0] - c[0])
    a = math.acos(r / d)
    cands = [(c[0] + r * math.cos(phi + s * a), c[1] + r * math.sin(phi + s * a), phi + s * a) for s in (1, -1)]
    return min(cands, key=lambda t: t[1]) if lowest else max(cands, key=lambda t: t[1])


def track(offset):
    """The track loop inset by `offset` from the grouser tips: its key points, the idler and sprocket arcs (centre, radius, start and end angle, counter-clockwise) and the direction of the upper run."""
    r = WRAP - offset
    p0, p1 = (-RUN, offset), (RUN, offset)
    ib = tangent_from(p1, IDLER, r)
    sb = tangent_from(p0, SPROCKET, r)
    dx, dy = IDLER[0] - SPROCKET[0], IDLER[1] - SPROCKET[1]
    n = math.hypot(dx, dy)
    nx, ny = -dy / n, dx / n
    top = math.atan2(ny, nx)
    it = (IDLER[0] + r * nx, IDLER[1] + r * ny)
    st = (SPROCKET[0] + r * nx, SPROCKET[1] + r * ny)
    a_i0 = ib[2]
    a_s1 = sb[2] % (2 * math.pi)
    return {"p0": p0, "p1": p1, "ib": ib[:2], "it": it, "st": st, "sb": sb[:2], "r": r, "idler": (a_i0, top), "sprocket": (top, a_s1), "slope": math.degrees(math.atan2(dy, dx))}


def track_path(offset):
    t = track(offset)
    q = L.Path().M(*t["p0"]).L(*t["p1"]).L(*t["ib"])
    q.A(t["r"], t["it"][0], t["it"][1], ccw=True, large=(t["idler"][1] - t["idler"][0]) > math.pi)
    q.L(*t["st"])
    q.A(t["r"], t["sb"][0], t["sb"][1], ccw=True, large=(t["sprocket"][1] - t["sprocket"][0]) > math.pi)
    return q.Z()


def top_run_y(x, offset):
    t = track(offset)
    return t["st"][1] + (t["it"][1] - t["st"][1]) * (x - t["st"][0]) / (t["it"][0] - t["st"][0])


BOOM = [(0.0, 0.20), (2.60, 0.90, 0.70), (5.70, 0.20), (5.70, -0.20), (2.55, 0.17, 0.90), (0.0, -0.20)]
BOOM_BOX = [(0.0, 0.20), (2.60, 0.90, 0.70), (5.30, 0.2903), (5.30, -0.153), (2.55, 0.17, 0.90), (0.0, -0.20)]
BOOM_CHEEK = [(5.0, 0.358), (5.70, 0.20), (5.70, -0.20), (5.0, -0.118)]
STICK = [(-0.90, 0.47), (-0.75, 0.585), (2.90, 0.13), (2.90, -0.13), (0.30, -0.22, 0.30), (-0.12, -0.17, 0.10), (-0.85, 0.35)]
STICK_BOX = [(-0.20, 0.517), (2.90, 0.13), (2.90, -0.13), (0.30, -0.22, 0.30), (-0.12, -0.17, 0.10), (-0.20, -0.07)]
STICK_TAIL = [(-0.90, 0.47), (-0.75, 0.585), (-0.10, 0.505), (-0.10, -0.184), (-0.85, 0.35)]
BUCKET = [(1.305, 0.145), (0.90, 0.50, 0.30), (0.42, 0.74, 0.25), (-0.12, 0.56, 0.14), (-0.19, 0.26, 0.08), (-0.13, -0.09, 0.05), (0.06, -0.11, 0.03), (1.24, -0.03, 0.03)]
BUCKET_SHELL = [(1.305, 0.145), (0.90, 0.50, 0.30), (0.42, 0.74, 0.25), (-0.12, 0.56, 0.14), (-0.19, 0.26, 0.08), (-0.13, -0.09, 0.05), (0.06, -0.11)]
TOOTH = [(1.286, 0.215), (1.47, 0.0), (1.234, 0.155)]
LIP = [(1.333, 0.147), (1.107, 0.3445), (1.086, 0.3205), (1.312, 0.123)]
CAB_SIDE = [(-0.38, 0.30, 0.03), (1.44, 0.30, 0.05), (1.44, 1.02, 0.03), (1.30, 1.90, 0.10), (-0.38, 1.92, 0.10)]
HOUSE_PLAN = [(-2.38, -HALF), (0.75, -HALF, 0.08), (0.75, SLOT[0], 0.05), (-0.38, SLOT[0]), (-0.38, HALF, 0.08), (-2.38, HALF)]
DECK_PLAN = [(-2.38, -HALF), (1.44, -HALF, 0.10), (1.44, SLOT[0]), (0.42, SLOT[0]), (0.42, SLOT[1]), (1.44, SLOT[1]), (1.44, HALF, 0.10), (-2.38, HALF)]
TOOLBOX_PLAN = [(0.75, -HALF), (1.42, -HALF, 0.08), (1.42, SLOT[0], 0.04), (0.75, SLOT[0])]


def cw_plan(inset=0.0):
    """The counterweight in plan (u ahead from its rear point, w): the rear face is the arc of the tail swing radius about the swing axis, the front face is flat against the house."""
    r = TAIL - inset
    half = HALF - inset
    w = half - 0.08
    u = TAIL - math.sqrt(r * r - w * w)
    return L.Path().M(CW_FRONT, -half).L(u + 0.09, -half).A(0.10, u, -w, ccw=False).A(r, u, w, ccw=False).A(0.10, u + 0.09, half, ccw=False).L(CW_FRONT, half).Z()


def band(points, t0, t1):
    """The part of a closed convex-ish outline ring between two inward offsets, as a ring with one hole (outer, hole)."""
    area = sum(points[i - 1][0] * points[i][1] - points[i][0] * points[i - 1][1] for i in range(len(points)))
    sign = 1 if area > 0 else -1

    def inset(t):
        n = len(points)
        out = []
        for i in range(n):
            a, b = points[i - 1], points[(i + 1) % n]
            dx, dy = b[0] - a[0], b[1] - a[1]
            l = math.hypot(dx, dy) or 1
            out.append((points[i][0] - dy / l * t * sign, points[i][1] + dx / l * t * sign))
        return out

    return inset(t0), inset(t1)
