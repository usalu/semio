"""🚌️ Round 3 city-bus: the stated dimensions of the real 12 m low-floor electric bus and the plain solid writers every scale of `r3-city-bus-build.py` and the 2D drawing of `r3-city-bus-roots.py` share."""
import math

L = 12.0
HW = 1.275
X0, X1 = 0.35, 11.45
SKIRT = 0.30
FLOOR = 0.34
BELT = 1.10
CANT = 2.62
ROOF = 2.98
LEAN = 0.035
TW = 0.0415
SKIN = 0.0015
PHI = math.atan(LEAN / (CANT - BELT))
S_TOP = (CANT - BELT) / math.cos(PHI)
XR, XF = 3.45, 9.35
AXLE = 0.48
R_TYRE = 0.48
R_ARCH = 0.58
ARCH_HALF = math.sqrt(R_ARCH ** 2 - (AXLE - SKIRT) ** 2)
DOOR_M = (6.275, 7.525)
DOOR_F = (10.2, 11.45)
LEAF = 0.625
DOOR_TOP = 2.36
SILL_GLASS = 1.12
HEAD_GLASS = 2.50
NOSE_D, NOSE_N = 0.535, 3.6
TAIL_D, TAIL_N = 0.335, 6.0
RAKE = math.radians(7.0)
Z_FRONT = 1.05
Z_REAR_OUT, Z_REAR_IN = 1.075, 0.755
Z_WELL_F, Z_WELL_R = 0.45, 0.34
POD_BATTERY = (1.0, 6.3, 0.32)
POD_HVAC = (8.0, 2.5, 0.25)
MIRROR_R, MIRROR_L = -1.50, 1.45
HEIGHT = ROOF + POD_BATTERY[2]
CANT_TABLE = [(2.62, 1.240), (2.72, 1.232), (2.80, 1.212), (2.865, 1.172), (2.912, 1.110), (2.944, 1.020), (2.964, 0.850), (2.976, 0.500), (2.98, 0.0)]
BAYS_L = [(1.795, 3.005), (3.095, 4.305), (4.395, 5.605), (5.695, 6.905), (6.995, 8.205), (8.295, 9.505), (9.595, 10.705), (10.795, 11.33)]
BAYS_R = [(0.45, 1.705), (1.795, 3.005), (3.095, 4.305), (4.395, 5.605), (5.695, 6.185), (7.615, 8.82), (8.91, 10.11)]
POSTS = [0.4, 1.75, 3.05, 4.35, 5.65, 6.95, 8.25, 9.55, 10.75, 11.4]
POSTS_R = [0.4, 1.75, 3.05, 4.35, 5.65, 6.23, 7.57, 8.865, 10.155, 11.4]


def rr(x):
    return round(x, 5)


def hw(y):
    """half width of the body cross-section at height y"""
    if y <= BELT:
        return HW
    if y <= CANT:
        return HW - LEAN * (y - BELT) / (CANT - BELT)
    for (y0, w0), (y1, w1) in zip(CANT_TABLE, CANT_TABLE[1:]):
        if y <= y1:
            return w0 + (w1 - w0) * (y - y0) / (y1 - y0)
    return 0.0


def qx(a):
    return [rr(math.sin(a / 2)), 0, 0, rr(math.cos(a / 2))]


def qy(a):
    return [0, rr(math.sin(a / 2)), 0, rr(math.cos(a / 2))]


def qz(a):
    return [0, 0, rr(math.sin(a / 2)), rr(math.cos(a / 2))]


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [rr(aw * bx + ax * bw + ay * bz - az * by), rr(aw * by - ax * bz + ay * bw + az * bx), rr(aw * bz + ax * by - ay * bx + az * bw), rr(aw * bw - ax * bx - ay * by - az * bz)]


def qfrom(d):
    """shortest rotation taking the u axis onto the direction d"""
    n = math.sqrt(sum(v * v for v in d))
    x, y, z = (v / n for v in d)
    if x < -0.999999:
        return [0, 0, 1, 0]
    w = 1 + x
    q = [0, -z, y, w]
    m = math.sqrt(sum(v * v for v in q))
    return [rr(v / m) for v in q]


def _c(id, bone, mat, role, tone):
    return {"id": id, "bone": bone, "material": mat, "role": role, "tone": tone}


def box(id, bone, mat, role, tone, size, at, rot=None, corner=0.0, **kw):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "box", "size": [rr(v) for v in size], "transform": {"translation": [rr(v) for v in at]}})
    if rot:
        s["transform"]["rotation"] = rot
    if corner:
        s["cornerRadius"] = corner
    s.update(kw)
    return s


def bar(id, bone, mat, role, tone, p0, p1, h, w, **kw):
    """a rectangular bar from p0 to p1, section h (in the sagittal plane for a horizontal bar) by w"""
    d = [b - a for a, b in zip(p0, p1)]
    n = math.sqrt(sum(v * v for v in d))
    return box(id, bone, mat, role, tone, [n, h, w], [(a + b) / 2 for a, b in zip(p0, p1)], rot=None if abs(d[0] - n) < 1e-9 else qfrom(d), **kw)


def cap(id, bone, mat, role, tone, p0, p1, r0, r1=None, **kw):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "capsule", "p0": [rr(v) for v in p0], "p1": [rr(v) for v in p1], "r0": rr(r0), "r1": rr(r0 if r1 is None else r1)})
    s.update(kw)
    return s


def ell(id, bone, mat, role, tone, radii, at, rot=None, **kw):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "ellipsoid", "radii": [rr(v) for v in radii], "transform": {"translation": [rr(v) for v in at]}})
    if rot:
        s["transform"]["rotation"] = rot
    s.update(kw)
    return s


def rev(id, bone, mat, role, tone, profile, origin, axis=(0, 0, 1), **kw):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "revolve", "profile": [[rr(a), rr(b)] for a, b in profile], "axis": list(axis), "origin": [rr(v) for v in origin]})
    s.update(kw)
    return s


def loft(id, bone, mat, role, tone, sections, caps=True, **kw):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "loft", "sections": sections, "capStart": caps, "capEnd": caps})
    s.update(kw)
    return s


def sec(at, rx, ry, e=2, rot=None):
    o = {"at": [rr(v) for v in at], "rx": rr(rx), "ry": rr(ry), "exponent": e}
    if rot:
        o["rotation"] = rot
    return o


def ring(y, xc, rx, ry, e):
    """a plan ring of a vertical loft: rx along the bus, ry across"""
    return sec([xc, y, 0], rx, ry, e, [0.70711, 0, 0, 0.70711])


def xsec(x, y0, y1, half, e):
    """a cross-section of a loft along the bus between the heights y0 and y1"""
    return sec([x, (y0 + y1) / 2, 0], half, (y1 - y0) / 2, e, [0.70711, 0, 0.70711, 0])


def ext(id, bone, mat, role, tone, plane, profile, a, b, holes=None, at=None, rot=None, taper=None, **kw):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "extrude", "profile": [[rr(p), rr(q)] for p, q in profile], "plane": plane, "from": rr(min(a, b)), "to": rr(max(a, b))})
    if holes:
        s["holes"] = [[[rr(p), rr(q)] for p, q in h] for h in holes]
    if taper:
        s["taperTo"] = taper
    if at or rot:
        s["transform"] = {}
        if at:
            s["transform"]["translation"] = [rr(v) for v in at]
        if rot:
            s["transform"]["rotation"] = rot
    s.update(kw)
    return s


def rect(x0, y0, x1, y1):
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


def arc(cx, cy, r, a0, a1, n):
    return [[cx + r * math.cos(a0 + (a1 - a0) * i / n), cy + r * math.sin(a0 + (a1 - a0) * i / n)] for i in range(n + 1)]


ARCH_A = math.asin((AXLE - SKIRT) / R_ARCH)


def arch(xa, r=R_ARCH, n=28, base=SKIRT):
    """the wheel arch over the axle xa from its rear foot to its front foot on the line `base`"""
    a = math.asin((AXLE - base) / r)
    return arc(xa, AXLE, r, math.pi + a, -a, n)


def lower_profile(doors, top=BELT, base=SKIRT, r=R_ARCH, n=28, x0=X0, x1=X1):
    """side elevation of the lower side panel: skirt line with both wheel arches, door notches down to the floor"""
    p = [[x0, base]] + arch(XR, r, n, base) + arch(XF, r, n, base)
    if doors:
        p += [[x1, base], [x1, FLOOR], [DOOR_F[0], FLOOR], [DOOR_F[0], top], [DOOR_M[1], top], [DOOR_M[1], FLOOR], [DOOR_M[0], FLOOR], [DOOR_M[0], top], [x0, top]]
    else:
        p += [[x1, base], [x1, top], [x0, top]]
    return p


def upper_profile(doors, s_door, s_top=S_TOP, x0=X0, x1=X1):
    if not doors:
        return rect(x0, 0, x1, s_top)
    return [[x0, 0], [DOOR_M[0], 0], [DOOR_M[0], s_door], [DOOR_M[1], s_door], [DOOR_M[1], 0], [DOOR_F[0], 0], [DOOR_F[0], s_door], [x1, s_door], [x1, s_top], [x0, s_top]]


def on_wall(side, x, s, zc):
    """world point of the leaning upper wall frame: s up the lean from the belt, zc out of the outer face (inward negative on the left, positive on the right)"""
    th = -side * PHI
    return [x, BELT + s * math.cos(th) - zc * math.sin(th), side * HW + s * math.sin(th) + zc * math.cos(th)]


def dcurve(d, w, n, t0=0.0, t1=math.pi, m=48):
    """plan outline of a rounded end as (z, dx) from the left side round the tip to the right side, in m equal steps of arc length"""
    fine = []
    for i in range(801):
        t = t0 + (t1 - t0) * i / 800
        c, s = math.cos(t), math.sin(t)
        fine.append([w * math.copysign(abs(c) ** (2 / n), c), d * abs(s) ** (2 / n)])
    acc = [0.0]
    for a, b in zip(fine, fine[1:]):
        acc.append(acc[-1] + math.hypot(b[0] - a[0], b[1] - a[1]))
    out, j = [], 0
    for i in range(m + 1):
        want = acc[-1] * i / m
        while j < 799 and acc[j + 1] < want:
            j += 1
        f = 0.0 if acc[j + 1] == acc[j] else (want - acc[j]) / (acc[j + 1] - acc[j])
        out.append([fine[j][0] + (fine[j + 1][0] - fine[j][0]) * f, fine[j][1] + (fine[j + 1][1] - fine[j][1]) * f])
    out[0], out[-1] = fine[0], fine[-1]
    return out


def dband(d, w, n, t, xc, sign=1, t0=0.0, t1=math.pi, m=48):
    """a wall of thickness t following the rounded end, as an extrude profile on the plane zx (w, u)"""
    o = dcurve(d, w, n, t0, t1, m)
    i = dcurve(d - t, w - t, n, t0, t1, m)
    return [[z, xc + sign * dx] for z, dx in o] + [[z, xc + sign * dx] for z, dx in reversed(i)]


def dfill(d, w, n, xc, sign=1, m=48):
    return [[z, xc + sign * dx] for z, dx in dcurve(d, w, n, 0.0, math.pi, m)]


def nose_dx(z, d=NOSE_D, w=HW, n=NOSE_N):
    return d * max(0.0, 1 - abs(z / w) ** n) ** (1 / n)


GLASS_D, GLASS_W, GLASS_H, GLASS_K = 0.529, HW - 0.006, 1.592, 0.9675
GLASS_CX = 0.36


def glass_pt(s, z, proud=0.0):
    """world point on the outer face of the raked windscreen at the height s along the rake and the lateral position z (before taper)"""
    k = 1 - (1 - GLASS_K) * s / GLASS_H
    dx = GLASS_CX + (nose_dx(z, GLASS_D, GLASS_W) - GLASS_CX) * k + proud
    c, sn = math.cos(RAKE), math.sin(RAKE)
    return [X1 + dx * c - s * sn, SILL_GLASS + dx * sn + s * c, z * k]


def roof_outline(t=0.0):
    """the roof cross-section from the right cant rail over the crown to the left one as (y, z), moved inward by t"""
    half = [(w, y) for y, w in CANT_TABLE]
    pts = [(-w, y) for w, y in half] + [(w, y) for w, y in reversed(half[:-1])]
    if t == 0.0:
        return [[y, z] for z, y in pts]
    out = []
    for i, (z, y) in enumerate(pts):
        a = pts[max(i - 1, 0)]
        b = pts[min(i + 1, len(pts) - 1)]
        dz, dy = b[0] - a[0], b[1] - a[1]
        m = math.hypot(dz, dy)
        nz, ny = dy / m, -dz / m
        out.append([y + ny * t, z + nz * t])
    return out


def tyre_loop(h=0.1375, grooves=()):
    """closed section of a 275/70 R 22.5 tyre as (axial, radius): tread, shoulders, sidewalls down to the rim flange, the bead toe tucked into the rim"""
    top = [[-0.09, 0.48]]
    for g in grooves:
        top += [[g - 0.005, 0.48], [g - 0.004, 0.469], [g + 0.004, 0.469], [g + 0.005, 0.48]]
    top += [[0.09, 0.48]]
    return [[-0.105, 0.300], [-h, 0.345], [-h, 0.425], [-0.128, 0.452], [-0.112, 0.470]] + top + [[0.112, 0.470], [0.128, 0.452], [h, 0.425], [h, 0.345], [0.105, 0.300], [0.012, 0.290], [-0.012, 0.290], [-0.105, 0.300]]


def rim_loop(d):
    """closed section of a 22.5 x 8.25 steel disc wheel as (axial, radius), the mounting face 0.16 m towards d (+1 outboard: front wheel and inner twin, -1: outer twin)"""
    p = [[-0.110, 0.302], [-0.104, 0.302], [-0.104, 0.286], [0.104, 0.286], [0.104, 0.302], [0.110, 0.302], [0.110, 0.279], [0.16, 0.20], [0.16, 0.105], [0.148, 0.105], [0.148, 0.196], [0.098, 0.274], [-0.110, 0.274], [-0.110, 0.302]]
    return [[d * a, r] for a, r in p]


def zband(d0, d1, n, za, zb, xc, w=HW, m=10):
    """a strip between the lateral positions za and zb on a rounded end, from the depth d1 out to d0, as an extrude profile on the plane zx"""
    zs = [za + (zb - za) * i / m for i in range(m + 1)]
    return [[z, xc + nose_dx(z, d0, w, n)] for z in zs] + [[z, xc + nose_dx(z, d1, w - (d0 - d1), n)] for z in reversed(zs)]
