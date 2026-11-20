"""🧰️ Hand tools and the hand-stated design lines of the round 3 vintage saloon (vehicles/vintage-car-1929): quaternions, mirrored solids, tilted panels, bands along a drawn centre line and every profile of the car in metres (x ahead, y up, z lateral, the right side of the car at -z)."""
import os, sys, math, json

exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-44-lib.py")).read())

SLUG = "vintage-car-1929"
P, FB, GL, RU, RM, NI, CI, EX, EM, UP = "paint", "fabric-roof", "glass", "rubber", "steel-rim", "nickel-plate", "cast-iron", "exhaust-steel", "engine-metal", "upholstery"
CH, AF, AR, WF, WR = "chassis", "axle-f", "axle-r", "wheel-f", "wheel-r"
XF, XR, RW, ZW, TW = 1.70, -1.50, 0.37, 0.67, 0.13
YSILL, YBELT, YCANT, YTOP = 0.52, 1.16, 1.62, 1.70
ZB = 0.62
XBUMP = 2.225


def Q(a, b):
    return [r6(c) for c in qmul(a, b)]


def qpath(theta):
    return Q(Qz(theta), Qy(math.pi / 2))


def mirq(q):
    return [r6(-q[0]), r6(-q[1]), r6(q[2]), r6(q[3])]


def mir(d, id=None):
    c = json.loads(json.dumps(d))
    if id:
        c["id"] = id
    k = c["kind"]
    if k == "capsule":
        c["p0"][2] = r6(-c["p0"][2])
        c["p1"][2] = r6(-c["p1"][2])
    elif k == "revolve":
        c["origin"][2] = r6(-c["origin"][2])
        c["axis"] = [c["axis"][0], c["axis"][1], r6(-c["axis"][2])]
    elif k == "extrude":
        if c["plane"] == "xy":
            c["from"], c["to"] = r6(-c["to"]), r6(-c["from"])
        elif c["plane"] == "yz":
            c["profile"] = [[p[0], r6(-p[1])] for p in c["profile"]]
            if "holes" in c:
                c["holes"] = [[[p[0], r6(-p[1])] for p in h] for h in c["holes"]]
        else:
            c["profile"] = [[r6(-p[0]), p[1]] for p in c["profile"]]
            if "holes" in c:
                c["holes"] = [[[r6(-p[0]), p[1]] for p in h] for h in c["holes"]]
    elif k == "loft":
        for s in c["sections"]:
            s["at"][2] = r6(-s["at"][2])
            if "rotation" in s:
                s["rotation"] = mirq(s["rotation"])
    t = c.get("transform")
    if t:
        if "translation" in t:
            t["translation"][2] = r6(-t["translation"][2])
        if "rotation" in t:
            t["rotation"] = mirq(t["rotation"])
    rp = c.get("repeat")
    if rp:
        if "translation" in rp:
            rp["translation"][2] = r6(-rp["translation"][2])
        if "origin" in rp:
            rp["origin"][2] = r6(-rp["origin"][2])
        if "rotationAxis" in rp:
            a = rp["rotationAxis"]
            rp["rotationAxis"] = [r6(-a[0]), r6(-a[1]), a[2]]
    return c


def pair(d, stem=None):
    """Left solid (id ends with -l) and its mirror (-r)."""
    assert d["id"].endswith("-l"), d["id"]
    return [d, mir(d, d["id"][:-2] + "-r")]


def rotv(q, v):
    x, y, z, w = q
    vx, vy, vz = v
    tx, ty, tz = 2 * (y * vz - z * vy), 2 * (z * vx - x * vz), 2 * (x * vy - y * vx)
    return [vx + w * tx + (y * tz - z * ty), vy + w * ty + (z * tx - x * tz), vz + w * tz + (x * ty - y * tx)]


def turn(d, q, pivot):
    """Turns a solid by the quaternion q about the point pivot."""
    rp = rotv(q, pivot)
    d["transform"] = {"translation": [r6(pivot[i] - rp[i]) for i in range(3)], "rotation": [r6(c) for c in q]}
    return d


def fillet(pts, r, n=3):
    out = []
    m = len(pts)
    for i in range(m):
        a, b, c = pts[i - 1], pts[i], pts[(i + 1) % m]
        ux, uy = a[0] - b[0], a[1] - b[1]
        vx, vy = c[0] - b[0], c[1] - b[1]
        lu, lv = math.hypot(ux, uy), math.hypot(vx, vy)
        ux, uy, vx, vy = ux / lu, uy / lu, vx / lv, vy / lv
        half = math.acos(max(-1, min(1, ux * vx + uy * vy))) / 2
        t = min(r / math.tan(half), lu / 2.01, lv / 2.01)
        p0 = [b[0] + ux * t, b[1] + uy * t]
        p1 = [b[0] + vx * t, b[1] + vy * t]
        for k in range(n + 1):
            s = k / n
            out.append([r6((1 - s) ** 2 * p0[0] + 2 * s * (1 - s) * b[0] + s * s * p1[0]), r6((1 - s) ** 2 * p0[1] + 2 * s * (1 - s) * b[1] + s * s * p1[1])])
    return out


def grow(pts, t):
    cx = sum(p[0] for p in pts) / len(pts)
    cy = sum(p[1] for p in pts) / len(pts)
    return [[p[0] + (t if p[0] > cx else -t), p[1] + (t if p[1] > cy else -t)] for p in pts]


def tangents(line):
    out = []
    n = len(line)
    for i in range(n):
        a, b = line[max(i - 1, 0)], line[min(i + 1, n - 1)]
        out.append(math.atan2(b[1] - a[1], b[0] - a[0]))
    return out


def band(line, half):
    """Outline of a strip of half thickness half[i] about the centre line (the true side view of a lofted sheet)."""
    th = tangents(line)
    up = [[r6(p[0] - math.sin(t) * h), r6(p[1] + math.cos(t) * h)] for p, t, h in zip(line, th, half)]
    dn = [[r6(p[0] + math.sin(t) * h), r6(p[1] - math.cos(t) * h)] for p, t, h in zip(line, th, half)]
    return up + dn[::-1]


def sheet(id, bone, mat, tone, line, zc, rx, ry, e=3, role="body"):
    """A sheet lofted along a centre line drawn in the side view: zc lateral centre, rx half width, ry half thickness per station."""
    th = tangents(line)
    secs = [sec([p[0], p[1], zc[i]], rx[i], ry[i], e, rotation=qpath(th[i])) for i, p in enumerate(line)]
    return loft(id, bone, mat, tone, secs, role=role)


def arc(cx, cy, r, a0, a1, n):
    return [[r6(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n))), r6(cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n)))] for k in range(n + 1)]


def xsec(x, y0, y1, hw, e, zc=0.0):
    return sec([x, (y0 + y1) / 2, zc], hw, (y1 - y0) / 2, e, rotation=Qy(math.pi / 2))


def pathd(pts, close=True):
    f = lambda v: ("%.3f" % v).rstrip("0").rstrip(".") if "." in ("%.3f" % v) else "%.3f" % v
    return "M " + " L ".join(f(p[0] + 0.0) + "," + f(p[1] + 0.0) for p in pts) + (" Z" if close else "")


def tube(id, bone, mat, tone, pts, r, role="detail"):
    """A bent pipe as a chain of capsules through the points (round ends overlap, the bends stay closed)."""
    return [capsule(f"{id}-{i + 1}" if len(pts) > 2 else id, bone, mat, tone, pts[i], pts[i + 1], r, role=role) for i in range(len(pts) - 1)]


# ---- design lines (side view unless said otherwise) ----
A_TUM = math.atan2(0.04, 0.46)
A_REAR = math.atan2(0.10, 0.46)
ARCH_R = 0.45
_a0 = math.degrees(math.asin((YSILL - RW) / ARCH_R))
ARCH = arc(XR, RW, ARCH_R, _a0, 180 - _a0, 12)
TAIL = [[-1.955, 0.52], [-1.975, 0.60], [-1.99, 0.72], [-1.99, 1.06], [-1.985, 1.10], [-1.968, 1.135], [-1.94, 1.155], [-1.90, 1.16]]
TAIL_IN = [[-1.90, 1.135], [-1.93, 1.132], [-1.948, 1.118], [-1.962, 1.095], [-1.965, 1.06], [-1.965, 0.72], [-1.951, 0.605], [-1.93, 0.52]]
TUB = [[0.52, YSILL]] + ARCH + TAIL + [[0.52, YBELT]]
YHEAD = 1.63
CABIN = [[0.52, YBELT], [0.52 - 0.04 * (YHEAD - YBELT) / 0.46, YHEAD], [-1.90 + 0.10 * (YHEAD - YBELT) / 0.46, YHEAD], [-1.90, YBELT]]
CABIN_LOW = [[0.52, YBELT - 0.02], [0.52, YBELT]] + CABIN[1:3] + [[-1.90, YBELT], [-1.90, YBELT - 0.02]]
WIN_F = [[0.444, 1.225], [0.415, 1.565], [-0.27, 1.565], [-0.27, 1.225]]
WIN_R = [[-0.39, 1.225], [-0.39, 1.565], [-1.05, 1.565], [-1.05, 1.225]]
WIN_Q = [[-1.16, 1.225], [-1.16, 1.565], [-1.692, 1.565], [-1.766, 1.225]]
X_A, X_B, X_C = 0.50, -0.33, -1.105
ROOF_E = 2.6
ROOF = [(0.66, 1.625, 0.57, 0.015), (0.60, 1.634, 0.585, 0.026), (0.48, 1.648, 0.592, 0.043), (0.20, 1.6525, 0.595, 0.0475), (-1.0, 1.6525, 0.595, 0.0475), (-1.55, 1.65, 0.592, 0.045), (-1.76, 1.638, 0.58, 0.034), (-1.84, 1.622, 0.565, 0.02), (-1.875, 1.61, 0.55, 0.011)]
HOOD_ZR, HOOD_ZF, HOOD_RY = 0.50, 0.283, 0.10
COWL = [(0.47, 1.16, 0.62, 30), (0.52, 1.163, 0.615, 30), (0.56, 1.168, 0.60, 14), (0.62, 1.176, 0.565, 10), (0.68, 1.182, 0.53, 8), (0.74, 1.185, 0.505, 8)]
HOOD = [(0.72, 1.185, HOOD_ZR, 4), (1.20, 1.185, 0.3926, 3.2), (1.69, 1.185, HOOD_ZF, 2.6)]
HOOD_B = math.atan2(HOOD_ZR - HOOD_ZF, 0.97)
hood_z = lambda x: HOOD_ZR - (x - 0.72) * math.tan(HOOD_B)
LOUVRES = [0.90 + 0.05 * i for i in range(12)]
SHELL = [[0.52, 0.285], [0.56, 0.292], [1.04, 0.297], [1.10, 0.29], [1.15, 0.262], [1.18, 0.205], [1.195, 0.12], [1.20, 0.0]]
CORE = [[0.58, 0.245], [0.62, 0.257], [1.02, 0.257], [1.08, 0.248], [1.125, 0.218], [1.15, 0.168], [1.16, 0.09], [1.162, 0.0]]
WING_R = 0.47
WING_F = arc(XF, RW, WING_R, -5, 145, 10) + [[1.25, 0.56], [1.17, 0.50], [1.09, 0.465], [1.00, 0.45]]
WING_F_Z = [0.705] * 9 + [0.705, 0.70, 0.66, 0.635, 0.635, 0.635]
WING_F_RX = [0.11, 0.135, 0.145] + [0.145] * 6 + [0.145, 0.15, 0.19, 0.215, 0.215, 0.215]
WING_F_RY = [0.012, 0.018, 0.02] + [0.02] * 6 + [0.02, 0.02, 0.02, 0.018, 0.016, 0.014]
WING_B = [[-0.99, 0.45], [-1.03, 0.47], [-1.05, 0.50]] + arc(XR, RW, WING_R, 20, 185, 11)
WING_B_Z = [0.725] * len(WING_B)
WING_B_RX = [0.125] * (len(WING_B) - 2) + [0.115, 0.095]
WING_B_RY = [0.014, 0.016, 0.018] + [0.02] * (len(WING_B) - 5) + [0.018, 0.012]
RAIL = [(2.08, 0.505, 0.34, 0.05), (1.95, 0.495, 0.34, 0.07), (1.70, 0.475, 0.34, 0.09), (1.30, 0.47, 0.355, 0.10), (0.90, 0.465, 0.385, 0.11), (0.50, 0.465, 0.42, 0.11), (-0.80, 0.465, 0.42, 0.11), (-1.08, 0.48, 0.42, 0.10), (-1.28, 0.55, 0.42, 0.09), (-1.50, 0.585, 0.42, 0.09), (-1.75, 0.58, 0.42, 0.08), (-1.97, 0.56, 0.42, 0.06)]
SPRING_F = [[2.07, 0.455], [1.95, 0.41], [1.83, 0.375], [1.70, 0.36], [1.57, 0.368], [1.44, 0.385], [1.31, 0.405]]
SPRING_R = [[-0.95, 0.46], [-1.13, 0.445], [-1.31, 0.434], [-1.50, 0.43], [-1.66, 0.44], [-1.81, 0.458], [-1.96, 0.48]]
SPRING_T = [0.008, 0.016, 0.024, 0.03, 0.024, 0.016, 0.008]
EXH = [[0.98, 0.78, -0.15], [0.90, 0.60, -0.22], [0.84, 0.36, -0.30], [-0.15, 0.36, -0.30]]
EXH_TAIL = [[-0.85, 0.36, -0.30], [-1.12, 0.37, -0.33], [-1.32, 0.50, -0.35], [-1.72, 0.50, -0.35], [-2.10, 0.42, -0.35]]
TYRE = [[-0.045, 0.245], [-0.058, 0.265], [-0.065, 0.30], [-0.062, 0.335], [-0.048, 0.36], [-0.025, 0.37], [0.025, 0.37], [0.048, 0.36], [0.062, 0.335], [0.065, 0.30], [0.058, 0.265], [0.045, 0.245], [-0.045, 0.245]]
RIM = [[-0.05, 0.222], [-0.05, 0.256], [-0.042, 0.256], [-0.036, 0.24], [0.036, 0.24], [0.042, 0.256], [0.05, 0.256], [0.05, 0.222], [-0.05, 0.222]]
SPARE = (0.92, 0.82, -0.615)
LAMP = (1.775, 0.98, 0.43)
BOWL = [[0, 0], [0.01, 0.045], [0.04, 0.085], [0.09, 0.108], [0.145, 0.112], [0.15, 0.118], [0.165, 0.118], [0.165, 0]]
LENS = [[0.163, 0], [0.163, 0.105], [0.175, 0.09], [0.182, 0.05], [0.184, 0]]
BUMP_Z = [-0.74, -0.71, -0.64, -0.52, 0.0, 0.52, 0.64, 0.71, 0.74]
BUMP_DX = [-0.075, -0.035, -0.008, 0, 0, 0, -0.008, -0.035, -0.075]
BUMP_Y = (0.475, 0.565)
SW_C, SW_D = (0.24, 1.22, -0.30), (-0.753, 0.658, 0.0)

# ---- coarser scales: the same car restated with the lines each scale can show ----
full = lambda half: [[y, -z] for y, z in half] + [[y, z] for y, z in half[::-1][1:]]
ARCHN = {100: 8, 200: 6, 500: 4}
TAILS = {100: [[-1.955, 0.52], [-1.99, 0.72], [-1.99, 1.06], [-1.96, 1.14], [-1.90, 1.16]], 200: [[-1.96, 0.52], [-1.99, 0.70], [-1.99, 1.08], [-1.92, 1.16]], 500: [[-1.97, 0.52], [-1.99, 0.62], [-1.99, 1.10], [-1.93, 1.16]]}
tub_of = lambda den: [[0.52, YSILL]] + arc(XR, RW, ARCH_R, _a0, 180 - _a0, ARCHN[den]) + TAILS[den] + [[0.52, YBELT]]
core_of = lambda den: [[0.52, YSILL]] + TAILS[den] + [[0.52, YBELT]]
ROOFS = {
    100: [(0.66, 1.627, 0.57, 0.017), (0.48, 1.648, 0.592, 0.043), (0.20, 1.6525, 0.595, 0.0475), (-1.55, 1.65, 0.592, 0.045), (-1.80, 1.63, 0.575, 0.027), (-1.875, 1.61, 0.55, 0.012)],
    200: [(0.66, 1.635, 0.57, 0.017), (0.42, 1.6525, 0.593, 0.0475), (-1.55, 1.6525, 0.593, 0.0475), (-1.87, 1.62, 0.555, 0.02)],
    500: [(0.66, 1.655, 0.575, 0.0375), (-0.60, 1.6625, 0.595, 0.0375), (-1.86, 1.655, 0.565, 0.0375)],
}
COWLS = {100: [(0.47, 1.16, 0.62, 30), (0.58, 1.172, 0.59, 12), (0.74, 1.185, 0.505, 8)], 200: [(0.47, 1.16, 0.62, 30), (0.60, 1.174, 0.58, 10), (0.74, 1.185, 0.505, 8)]}
WINGF = {
    100: (arc(XF, RW, WING_R, -5, 145, 7) + [[1.21, 0.53], [1.09, 0.465], [1.00, 0.45]], [0.705] * 7 + [0.705, 0.68, 0.635, 0.635], [0.11, 0.14] + [0.145] * 5 + [0.145, 0.17, 0.215, 0.215], [0.012] + [0.02] * 8 + [0.017, 0.015]),
    200: (arc(XF, RW, WING_R, -5, 145, 6) + [[1.19, 0.515], [1.00, 0.45]], [0.705] * 7 + [0.67, 0.635], [0.115] + [0.145] * 6 + [0.18, 0.215], [0.015] + [0.02] * 6 + [0.02, 0.016]),
}
WINGB = {
    100: ([[-0.99, 0.45], [-1.05, 0.50]] + arc(XR, RW, WING_R, 20, 185, 8), [0.725] * 11, [0.125] * 9 + [0.115, 0.095], [0.015, 0.018] + [0.02] * 7 + [0.018, 0.012]),
    200: ([[-0.99, 0.45]] + arc(XR, RW, WING_R, 20, 185, 7), [0.725] * 9, [0.125] * 8 + [0.10], [0.016] + [0.02] * 7 + [0.015]),
}
_wl = arc(XF, RW, WING_R + 0.02, 0, 140, 6) + [[1.20, 0.52], [1.02, 0.4375], [-0.98, 0.4375], [-1.06, 0.52]] + arc(XR, RW, WING_R + 0.02, 30, 180, 6)
WINGLINE = (_wl, [0.705] * 7 + [0.69, 0.71, 0.725, 0.725] + [0.725] * 7, [0.145] * 7 + [0.16, 0.14, 0.125, 0.125] + [0.125] * 7, [0.0375] * len(_wl))
RAILS = {100: [RAIL[i] for i in (0, 2, 4, 6, 8, 10, 11)], 200: [RAIL[i] for i in (0, 2, 5, 6, 9, 11)]}
SHELLS = {100: [[0.52, 0.285], [1.06, 0.297], [1.15, 0.262], [1.19, 0.15], [1.20, 0.0]], 200: [[0.52, 0.285], [1.07, 0.297], [1.16, 0.24], [1.20, 0.0]]}
CORE100 = [[0.58, 0.25], [1.03, 0.257], [1.12, 0.22], [1.16, 0.0]]
LOUVRES100 = [0.92 + 0.10 * i for i in range(6)]
BUMP100 = ([-0.74, -0.64, 0.0, 0.64, 0.74], [-0.075, -0.01, 0, -0.01, -0.075])
TYRES = {100: [[-0.045, 0.245], [-0.065, 0.29], [-0.06, 0.34], [-0.03, 0.37], [0.03, 0.37], [0.06, 0.34], [0.065, 0.29], [0.045, 0.245], [-0.045, 0.245]], 200: [[-0.05, 0.245], [-0.065, 0.30], [-0.04, 0.37], [0.04, 0.37], [0.065, 0.30], [0.05, 0.245], [-0.05, 0.245]]}
WINS = (WIN_F, WIN_R, WIN_Q)
