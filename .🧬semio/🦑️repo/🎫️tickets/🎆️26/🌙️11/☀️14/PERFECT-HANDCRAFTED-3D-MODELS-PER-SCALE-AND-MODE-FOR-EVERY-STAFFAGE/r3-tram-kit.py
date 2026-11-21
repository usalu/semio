"""🚋️ Round 3 tram: the stated dimensions of the real low-floor articulated tram section and the plain solid writers that every scale of `r3-tram-build.py` and the 2D drawing of `r3-tram-roots.py` share. World frame: x ahead (cab at +x, articulation at 0), y up from the rail head, z lateral (+ left, the doors are on the right, -z)."""
import copy, math

HW = 1.20
SKIRT = 0.28
FLOOR = 0.35
BELT = 1.15
CANT = 3.10
ROOF = 3.40
LEAN = 0.04
TW = 0.0515
SKIN = 0.0015
PHI = math.atan(LEAN / (CANT - BELT))
S_TOP = (CANT - BELT) / math.cos(PHI)
E0, XC, TIP, BUMP = 0.45, 8.48, 9.42, 0.03
M0, M1 = -6.0, -0.45
NOSE_D, NOSE_N = TIP - XC, 3.0
TANR = 0.30
GLASS_TOP = 2.65
SILL, HEAD = 1.20, 2.40
DOOR_A = (3.5, 4.8)
DOOR_B = (-4.0, -2.7)
LEAF = 0.65
DOOR_Y = (0.30, 2.45)
BOGIE_F, BOGIE_R = 7.2, 1.9
WB = 0.9
R_WHEEL, R_FLANGE = 0.30, 0.322
Z_BACK, Z_FACE, Z_RUN = 0.69, 0.795, 0.7475
R_ARCH = 0.38
ARCH_TOP = 0.3 + R_ARCH
PAN = (2.6, 3.55)
HVAC = (-5.2, -1.4, 0.9, 0.26)
RESISTOR = (4.5, 6.5, 0.85, 0.24, 0.03)
BELLOWS = (1.0, 0.30, 3.25)
PORTAL = (0.65, FLOOR, 2.45)
CANT_TABLE = [(3.10, 1.160), (3.22, 1.148), (3.30, 1.120), (3.345, 1.07), (3.37, 1.0), (3.385, 0.85), (3.395, 0.5), (3.40, 0.0)]
DOME = [(2.80, 8.925), (2.95, 8.865), (3.10, 8.77), (3.22, 8.63), (3.31, 8.43), (3.37, 8.17), (3.395, 7.93)]
PANES_END_R = [('e1', 0.80, 3.30), ('e2', 5.00, 6.30), ('e3', 6.45, 7.47)]
PANES_END_L = [('e1', 0.80, 3.30), ('e4', 3.45, 4.85), ('e2', 5.00, 6.30), ('e3', 6.45, 7.47)]
PANES_MID_R = [('m1', -5.85, -4.25), ('m2', -2.55, -0.85)]
PANES_MID_L = [('m1', -5.85, -4.25), ('m3', -4.10, -2.70), ('m2', -2.55, -0.85)]
POSTS_END = [0.5, 3.4, 4.9, 6.375, 7.545]
POSTS_MID = [-5.95, -4.15, -2.625, -0.5]
QRING = [0.70711, 0, 0, 0.70711]
QXSEC = [0.70711, 0, 0.70711, 0]


def rr(x):
    return round(x, 5)


def hw(y):
    """half width of the car body cross-section at the height y"""
    if y <= BELT:
        return HW
    if y <= CANT:
        return HW - LEAN * (y - BELT) / (CANT - BELT)
    for (y0, w0), (y1, w1) in zip(CANT_TABLE, CANT_TABLE[1:]):
        if y <= y1:
            return w0 + (w1 - w0) * (y - y0) / (y1 - y0)
    return 0.0


def edge(y):
    """x of the raked line where the cab begins, at the height y"""
    return XC - TANR * max(0.0, y - BELT)


def tip(y):
    """x of the foremost point of the cab at the height y"""
    if y <= BELT:
        return TIP
    if y <= DOME[0][0]:
        return TIP - TANR * (y - BELT)
    for (y0, x0), (y1, x1) in zip(DOME, DOME[1:]):
        if y <= y1:
            return x0 + (x1 - x0) * (y - y0) / (y1 - y0)
    return DOME[-1][1]


def nose_x(y, z, inset=0.0):
    """x of the cab surface at the height y and the lateral position z"""
    w = hw(y) - inset
    d = tip(y) - edge(y) - inset
    return edge(y) + d * max(0.0, 1 - abs(z / w) ** NOSE_N) ** (1 / NOSE_N)


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


def qrot(q, v):
    x, y, z, w = q
    vx, vy, vz = v
    tx, ty, tz = 2 * (y * vz - z * vy), 2 * (z * vx - x * vz), 2 * (x * vy - y * vx)
    return [vx + w * tx + y * tz - z * ty, vy + w * ty + z * tx - x * tz, vz + w * tz + x * ty - y * tx]


def qfrom(d):
    """shortest rotation taking the u axis onto the direction d"""
    n = math.sqrt(sum(v * v for v in d))
    x, y, z = (v / n for v in d)
    if x < -0.999999:
        return [0, 0, 1, 0]
    q = [0, -z, y, 1 + x]
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


def slab(id, bone, mat, role, tone, x0, x1, y0, y1, z0, z1, corner=0.0, **kw):
    """an axis-aligned box given by its extents"""
    return box(id, bone, mat, role, tone, [x1 - x0, y1 - y0, z1 - z0], [(x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2], corner=corner, **kw)


def bar(id, bone, mat, role, tone, p0, p1, h, w, **kw):
    """a rectangular bar from p0 to p1 with the section h by w"""
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


def rev(id, bone, mat, role, tone, profile, origin, axis=(0, 0, 1), bore=False, **kw):
    if bore:
        assert profile[0][1] == profile[-1][1] and profile[0][1] > 0
        profile = [*profile, profile[0]]
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "revolve", "profile": [[rr(a), rr(b)] for a, b in profile], "axis": list(axis), "origin": [rr(v) for v in origin]})
    s.update(kw)
    return s


def cyl(id, bone, mat, role, tone, p0, axis, length, r, r_in=0.0, **kw):
    """a cylinder (a tube when r_in is given) from p0 along a coordinate axis"""
    return rev(id, bone, mat, role, tone, [[0, r_in], [0, r], [length, r], [length, r_in]], p0, axis, bore=r_in > 0, **kw)


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


def ring(y, xc, rx, ry, e=NOSE_N):
    """a plan ring of a vertical loft: rx along the car, ry across"""
    return sec([xc, y, 0], rx, ry, e, QRING)


def xsec(x, y0, y1, half, e, z=0.0):
    """a cross-section of a loft along the car between the heights y0 and y1"""
    return sec([x, (y0 + y1) / 2, z], half, (y1 - y0) / 2, e, QXSEC)


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


def rrect(x0, y0, x1, y1, r, n=4):
    """a rectangle with rounded corners"""
    p = []
    for cx, cy, a0 in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n)
            p.append([cx + r * math.cos(a), cy + r * math.sin(a)])
    return p


def arc(cx, cy, r, a0, a1, n):
    return [[cx + r * math.cos(a0 + (a1 - a0) * i / n), cy + r * math.sin(a0 + (a1 - a0) * i / n)] for i in range(n + 1)]


def cutout(xb, n=10, base=SKIRT, r=R_ARCH):
    """the bogie opening of the skirt over the bogie centre xb, from its rear foot to its front foot on the line `base`: a quarter round over each wheel and a straight lintel between"""
    a = math.asin((0.3 - base) / r)
    return arc(xb - WB, 0.3, r, math.pi + a, math.pi / 2, n) + arc(xb + WB, 0.3, r, math.pi / 2, -a, n)


def curve(d, w, n=NOSE_N, m=24):
    """plan outline of the rounded cab end as (z, dx) from the left side round the tip to the right side, in equal steps of arc length"""
    fine = []
    for i in range(801):
        t = math.pi * i / 800
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


def dfill(d, w, xc, n=NOSE_N, m=24):
    """the filled plan of the rounded end as an extrude profile on the plane zx"""
    return [[z, xc + dx] for z, dx in curve(d, w, n, m)]


def dband(d, w, t, xc, n=NOSE_N, m=24):
    """a wall of the thickness t following the rounded end, as an extrude profile on the plane zx"""
    return [[z, xc + dx] for z, dx in curve(d, w, n, m)] + [[z, xc + dx] for z, dx in reversed(curve(d - t, w - t, n, m))]


def section(t=0.0, y0=SKIRT):
    """the car body cross-section as (y, z) from the right foot up over the roof to the left foot, moved inward by t"""
    half = [(HW, y0), (HW, BELT)] + [(w, y) for y, w in CANT_TABLE]
    pts = [(-w, y) for w, y in half] + [(w, y) for w, y in reversed(half[:-1])]
    if t == 0.0:
        return [[y, z] for z, y in pts]
    out = []
    for i, (z, y) in enumerate(pts):
        a, b = pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)]
        dz, dy = b[0] - a[0], b[1] - a[1]
        m = math.hypot(dz, dy)
        out.append([y - dz / m * t if 0 < i < len(pts) - 1 else y, z + dy / m * t])
    return out


def roof_outline(t=0.0):
    """the roof cross-section from the right cant rail over the crown to the left one as (y, z), moved inward by t"""
    half = [(w, y) for y, w in CANT_TABLE]
    pts = [(-w, y) for w, y in half] + [(w, y) for w, y in reversed(half[:-1])]
    out = []
    for i, (z, y) in enumerate(pts):
        a, b = pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)]
        dz, dy = b[0] - a[0], b[1] - a[1]
        m = math.hypot(dz, dy)
        out.append([y - dz / m * t, z + dy / m * t])
    return out


def on_wall(side, x, s, zc):
    """world point of the leaning upper wall frame: s up the lean from the belt, zc out of the outer face"""
    th = -side * PHI
    return [x, BELT + s * math.cos(th) - zc * math.sin(th), side * HW + s * math.sin(th) + zc * math.cos(th)]


def s_of(y):
    """distance up the leaning wall of the height y"""
    return (y - BELT) / math.cos(PHI)


MID_Q = [0, 0, 1, 0]


def mid(s):
    """a solid stated in the world frame carried into the frame of the suspended module (turned by 180 degrees about the articulation at the height 0.3)"""
    s = copy.deepcopy(s)
    s["bone"] = "body-mid"
    t = s.get("transform", {})
    tr = t.get("translation", [0, 0, 0])
    new = {"translation": [rr(-tr[0]), rr(0.3 - tr[1]), rr(tr[2])], "rotation": qmul(MID_Q, t["rotation"]) if t.get("rotation") else MID_Q}
    s["transform"] = new
    return s


def on(s, bone, origin):
    """a solid stated in the world frame carried into the frame of a bone that is only shifted (bogie, wheelset, door leaf, bellows)"""
    s = copy.deepcopy(s)
    s["bone"] = bone
    o = list(origin) + [0.0] * (3 - len(origin))
    k = s["kind"]
    if k == "capsule":
        s["p0"] = [rr(a - b) for a, b in zip(s["p0"], o)]
        s["p1"] = [rr(a - b) for a, b in zip(s["p1"], o)]
    elif k == "revolve" and "transform" not in s:
        s["origin"] = [rr(a - b) for a, b in zip(s["origin"], o)]
    elif k == "loft":
        for q in s["sections"]:
            q["at"] = [rr(a - b) for a, b in zip(q["at"], o)]
    else:
        t = s.setdefault("transform", {})
        tr = t.get("translation", [0, 0, 0])
        t["translation"] = [rr(a - b) for a, b in zip(tr, o)]
    return s


def lean(s):
    """a door solid stated in the world frame tilted about its bottom edge so that the closed leaf follows the 40 mm tumblehome of the wall above the waist"""
    s = copy.deepcopy(s)
    q = qx(math.atan((HW - hw(DOOR_Y[1])) / (DOOR_Y[1] - DOOR_Y[0])))
    c = [0.0, DOOR_Y[0], -HW]
    t = s.get("transform", {})
    t0 = t.get("translation", [0, 0, 0])
    rt, rc = qrot(q, t0), qrot(q, c)
    s["transform"] = {"translation": [rr(rt[i] + c[i] - rc[i]) for i in range(3)], "rotation": qmul(q, t["rotation"]) if t.get("rotation") else q}
    return s


def wheel_loop():
    """closed section of a 600 mm tram wheel as (axial from the back face outwards, radius): flange, tread, tyre, recessed web, hub"""
    return [[-0.05, 0.055], [-0.05, 0.1], [0, 0.125], [0, 0.31], [0.004, R_FLANGE], [0.016, R_FLANGE], [0.03, 0.302], [0.105, 0.298], [0.105, 0.25], [0.088, 0.238], [0.088, 0.115], [0.105, 0.105], [0.105, 0.055]]
