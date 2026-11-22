"""🧰️ Geometry kit of the used-violin author: bone frames, rotations and parts that write one 3D solid and its matching 2D `solid` shape.

Every part is defined once in the bone-local frame (u along the bone, v its normal, w lateral) and emitted twice: as a `VolumeSolid`
of the `volumes` lists and as an extruded 2D shape with the same profile and span, so the three views and the model agree by construction.

@see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — `VolumeSolid`, `Shape`, `Solid`
@see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts — `solid3`, the 2D placement of extruded shapes
"""
import math

DEG = math.pi / 180


def mat_mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mat_vec(a, v):
    return [sum(a[i][k] * v[k] for k in range(3)) for i in range(3)]


def transpose(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def rot_w(deg):
    c, s = math.cos(deg * DEG), math.sin(deg * DEG)
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rot_v(deg):
    c, s = math.cos(deg * DEG), math.sin(deg * DEG)
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def rot_u(deg):
    c, s = math.cos(deg * DEG), math.sin(deg * DEG)
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def placement(angle=0.0, abduction=0.0, twist=0.0):
    return mat_mul(mat_mul(rot_w(angle), rot_v(abduction)), rot_u(twist))


def decompose(m):
    b = math.asin(max(-1.0, min(1.0, m[2][0])))
    c = math.atan2(-m[2][1], m[2][2])
    a = math.atan2(m[1][0], m[0][0])
    return a / DEG, b / DEG, c / DEG


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def scale(a, k):
    return [x * k for x in a]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a))
    return [x / n for x in a]


def columns(u, v, w):
    return [[u[0], v[0], w[0]], [u[1], v[1], w[1]], [u[2], v[2], w[2]]]


class Frame:
    """📐️ A world frame of a bone: origin and the rotation whose columns are u, v, w."""

    def __init__(self, origin, rotation):
        self.o = list(origin)
        self.r = rotation

    def world(self, p):
        return add(self.o, mat_vec(self.r, p))

    def local(self, p):
        return mat_vec(transpose(self.r), sub(p, self.o))

    def axis(self, i):
        return [self.r[0][i], self.r[1][i], self.r[2][i]]


def compose(parent, x, y, z, rest, offset=None, d=(0.0, 0.0)):
    r_off = offset if offset is not None else [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    origin = parent.world([x + d[0], y + d[1], z])
    return Frame(origin, mat_mul(mat_mul(parent.r, rest), r_off))


def r6(x):
    return round(x + 0.0, 6)


def pts(points):
    return [[r6(a), r6(b)] for a, b in points]


def rect(a0, a1, b0, b1):
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def polygon_circle(cx, cy, r, n=12):
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def stadium(p0, p1, r, n=6):
    du, dv = p1[0] - p0[0], p1[1] - p0[1]
    ang = math.atan2(dv, du)
    out = []
    for k in range(n + 1):
        t = ang - math.pi / 2 + math.pi * k / n
        out.append((p1[0] + r * math.cos(t), p1[1] + r * math.sin(t)))
    for k in range(n + 1):
        t = ang + math.pi / 2 + math.pi * k / n
        out.append((p0[0] + r * math.cos(t), p0[1] + r * math.sin(t)))
    return out


def band(p0, p1, r):
    du, dv = p1[0] - p0[0], p1[1] - p0[1]
    n = math.hypot(du, dv)
    nx, ny = -dv / n * r, du / n * r
    return [(p0[0] + nx, p0[1] + ny), (p1[0] + nx, p1[1] + ny), (p1[0] - nx, p1[1] - ny), (p0[0] - nx, p0[1] - ny)]


def offset_polyline(points, half):
    left, right = [], []
    for i, p in enumerate(points):
        if i == 0:
            d = unit([points[1][0] - p[0], points[1][1] - p[1], 0])
            n = [-d[1], d[0]]
            k = 1.0
        elif i == len(points) - 1:
            d = unit([p[0] - points[i - 1][0], p[1] - points[i - 1][1], 0])
            n = [-d[1], d[0]]
            k = 1.0
        else:
            d0 = unit([p[0] - points[i - 1][0], p[1] - points[i - 1][1], 0])
            d1 = unit([points[i + 1][0] - p[0], points[i + 1][1] - p[1], 0])
            n0, n1 = [-d0[1], d0[0]], [-d1[1], d1[0]]
            n = unit([n0[0] + n1[0], n0[1] + n1[1], 0])[:2]
            k = 1.0 / max(0.2, n[0] * n0[0] + n[1] * n0[1])
        left.append((p[0] + n[0] * half * k, p[1] + n[1] * half * k))
        right.append((p[0] - n[0] * half * k, p[1] - n[1] * half * k))
    return left + right[::-1]


def area2(poly):
    s = 0.0
    for i in range(len(poly)):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % len(poly)]
        s += x0 * y1 - x1 * y0
    return s / 2


def ccw(poly):
    return list(poly) if area2(poly) > 0 else list(poly)[::-1]


class Part:
    """🧩️ One physical part of one scale: its 3D solid and its 2D shape, bound to a bone, a material and optionally a thermal region."""

    def __init__(self, pid, bone, material, kind, geo, region=None, role="body", tone=0.5, flags=None, two_d=None, filled=True, pen=None):
        self.id = pid
        self.bone = bone
        self.material = material
        self.kind = kind
        self.geo = geo
        self.region = region
        self.role = role
        self.tone = tone
        self.flags = flags or {}
        self.two_d = two_d
        self.filled = filled
        self.pen = pen

    def solid(self):
        base = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        g = self.geo
        if self.kind == "pv":
            s = {"kind": "extrude", "profile": pts([(w, u) for u, w in ccw(g["profile"])]), "plane": "zx", "from": r6(g["v0"]), "to": r6(g["v1"])}
            if g.get("taper"):
                s["taperTo"] = list(g["taper"])
        elif self.kind == "pw":
            s = {"kind": "extrude", "profile": pts(ccw(g["profile"])), "plane": "xy", "from": r6(g["w0"]), "to": r6(g["w1"])}
        elif self.kind == "pu":
            s = {"kind": "extrude", "profile": pts(ccw(g["profile"])), "plane": "yz", "from": r6(g["u0"]), "to": r6(g["u1"])}
        elif self.kind == "ring_zx":
            s = {"kind": "extrude", "profile": pts([(w, u) for u, w in ccw(g["outer"])]), "holes": [pts([(w, u) for u, w in ccw(h)][::-1]) for h in g["holes"]], "plane": "zx", "from": r6(g["v0"]), "to": r6(g["v1"])}
        elif self.kind == "ring_xy":
            s = {"kind": "extrude", "profile": pts(ccw(g["outer"])), "holes": [pts(ccw(h)[::-1]) for h in g["holes"]], "plane": "xy", "from": r6(g["w0"]), "to": r6(g["w1"])}
        elif self.kind == "capsule":
            s = {"kind": "capsule", "p0": [r6(x) for x in g["p0"]], "p1": [r6(x) for x in g["p1"]], "r0": r6(g["r"]), "r1": r6(g["r"])}
        elif self.kind == "loft":
            s = {"kind": "loft", "sections": [{"at": [r6(x) for x in sec["at"]], "rx": r6(sec["rx"]), "ry": r6(sec["ry"]), "exponent": sec.get("exponent", 2), "rotation": [0.5, 0.5, 0.5, 0.5]} for sec in g["sections"]], "capStart": True, "capEnd": True}
        elif self.kind == "ellipsoid":
            s = {"kind": "ellipsoid", "radii": [r6(x) for x in g["radii"]], "transform": {"translation": [r6(x) for x in g["at"]]}}
        else:
            raise ValueError(self.kind)
        out = {**{k: base[k] for k in ("id",)}, **s, **{k: base[k] for k in ("bone", "material", "role", "tone")}}
        for key in ("slender", "sheet", "texture", "envelope"):
            if self.flags.get(key):
                out[key] = True
        return out

    def shapes(self, origin_z):
        specs = self.two_d if self.two_d is not None else [(self.kind, self.geo, self.id)]
        out = []
        for kind, g, sid in specs:
            if kind == "pv":
                solid = {"kind": "extruded", "sweep": "v", "z0": r6(g["v0"]), "z1": r6(g["v1"])}
                points = g["profile"]
            elif kind == "pw":
                solid = {"kind": "extruded", "sweep": "w", "z0": r6(origin_z + g["w0"]), "z1": r6(origin_z + g["w1"])}
                if g.get("wtip"):
                    solid["zTip"] = [r6(origin_z + g["wtip"][0]), r6(origin_z + g["wtip"][1])]
                points = g["profile"]
            elif kind == "pu":
                solid = {"kind": "extruded", "sweep": "u", "z0": r6(g["u0"]), "z1": r6(g["u1"])}
                points = g["profile"]
            elif kind == "capsule":
                p0, p1, r = g["p0"], g["p1"], g["r"]
                solid = {"kind": "extruded", "sweep": "w", "z0": r6(origin_z + p0[2] - r), "z1": r6(origin_z + p0[2] + r)}
                if abs(p1[2] - p0[2]) > 1e-9:
                    raise ValueError("capsule 2D needs constant w")
                points = stadium((p0[0], p0[1]), (p1[0], p1[1]), r)
            else:
                raise ValueError(kind)
            shape = {"id": sid, "bone": self.bone, "role": self.role, "z": 0, "filled": self.filled, "tone": self.tone}
            if self.region:
                shape["region"] = self.region
            shape["material"] = self.material
            if self.pen is not None:
                shape["penMm"] = self.pen
            shape.update({"kind": "poly", "points": pts(ccw(points)), "closed": True, "solid": solid})
            out.append(shape)
        return out
