"""🧰️ Geometry kit of the used-trumpet author: every physical part is defined once in its bone's rest frame (u along the bone, v its normal, w lateral) and written twice, as a 3D solid of the `volumes` lists and as a 2D shape with a `solid` placement of the same profile and span, so the three views and the model agree by construction in every pose and clip instant.

@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Solid placements", "Volume"
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — `VolumeSolid`, `Shape`, `Solid`
"""
import math

TAU = 2 * math.pi


def T(en, de):
    """🌐️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(x):
    """✂️ Rounds a number to a micrometre for the document."""
    return round(x + 0.0, 6)


def pts(points):
    """📍️ Rounds a 2D point list."""
    return [[r6(a), r6(b)] for a, b in points]


def area2(poly):
    """📐️ Signed area of a closed polygon."""
    s = 0.0
    for i in range(len(poly)):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % len(poly)]
        s += x0 * y1 - x1 * y0
    return s / 2


def ccw(poly):
    """🔄️ The polygon in counter-clockwise order."""
    return list(poly) if area2(poly) > 0 else list(poly)[::-1]


def rect(a0, a1, b0, b1):
    """⬛️ A rectangle as a polygon."""
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def circle(cx, cy, r, n=12):
    """⭕️ A polygon of n points on a circle."""
    return [(cx + r * math.cos(TAU * k / n), cy + r * math.sin(TAU * k / n)) for k in range(n)]


def arc(cx, cy, r, a0, a1, n):
    """🌙️ Points on an arc from a0 to a1 degrees, both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def path_length(path):
    """📏️ Length of a polyline of (a, b, r) points."""
    return sum(math.hypot(q[0] - p[0], q[1] - p[1]) for p, q in zip(path, path[1:]))


def band(path):
    """🎗️ The closed outline of a tube of varying radius along a polyline of (a, b, r) points: left offsets forward, right offsets back."""
    left, right = [], []
    n = len(path)
    for i in range(n):
        a, b, r = path[i]
        if i == 0:
            da, db = path[1][0] - a, path[1][1] - b
        elif i == n - 1:
            da, db = a - path[i - 1][0], b - path[i - 1][1]
        else:
            ta = (a - path[i - 1][0], b - path[i - 1][1])
            tb = (path[i + 1][0] - a, path[i + 1][1] - b)
            la, lb = math.hypot(*ta), math.hypot(*tb)
            da, db = ta[0] / la + tb[0] / lb, ta[1] / la + tb[1] / lb
        ln = math.hypot(da, db)
        nx, ny = -db / ln, da / ln
        left.append((a + nx * r, b + ny * r))
        right.append((a - nx * r, b - ny * r))
    return left + right[::-1]


def tube_path(segments):
    """🪈️ A tube centre line from straight runs and arcs: `("line", (a, b), r)` appends a point, `("arc", (ca, cb), R, a0, a1, n, r)` appends arc points after the first."""
    out = []
    for seg in segments:
        if seg[0] == "line":
            out.append((seg[1][0], seg[1][1], seg[2]))
        else:
            _, c, radius, a0, a1, n, r = seg
            for k, p in enumerate(arc(c[0], c[1], radius, a0, a1, n)):
                if k == 0 and out and math.hypot(out[-1][0] - p[0], out[-1][1] - p[1]) < 1e-9:
                    continue
                out.append((p[0], p[1], r))
    clean = [out[0]]
    for p in out[1:]:
        if math.hypot(p[0] - clean[-1][0], p[1] - clean[-1][1]) > 1e-7:
            clean.append(p)
    return clean


def half_outline(profile):
    """✏️ A revolve profile (axial, radial) closed to the axis for a mirrored 2D revolved shape."""
    return [(profile[0][0], 0.0)] + list(profile) + [(profile[-1][0], 0.0)]


def torus(a, r, ring, n=12):
    """🍩️ A closed loop profile of a ring of wire radius `ring` at axial a and radius r (first point repeated)."""
    loop = circle(a, r, ring, n)
    return loop + [loop[0]]


class Part:
    """🧩️ One physical part of one scale on one bone: its 3D solid and its 2D `solid` shape with the same geometry.

    Kinds (bone-local metres): `box` lo/hi (u, v, w), drawn as a v sweep of its (u, w) rectangle; `pv` an extrusion of a (u, w) profile along v from v0 to v1 (optional `holes`);
    `pu` an extrusion of a (v, w) profile along u from u0 to u1; `ru` a revolve about the bone axis u of an open (closed through the axis) or closed-loop profile (axial, radial), drawn with `outline` (default the profile closed to the axis);
    `tube` a loft of round sections along a path of (u, w, r) in the plane v = `v`, drawn as its band swept across v; `cap_v` a capsule along v at (u, w) from v0 to v1; `cap_u` a capsule along u at (v, w) from u0 to u1.
    With `zrest` (the rest lateral position of an unrotated bone's origin) a box or a `cap_u` is drawn as its (u, v) side swept laterally (`sweep: "w"`), the plane the product ranks paint depth in.
    """

    def __init__(self, pid, bone, material, kind, geo, region=None, role="body", tone=0.5, flags=None, pen=None, draw=True, mass=0.0, com=None, extra=None, solid3d=True, zrest=None):
        self.solid3d = solid3d
        self.zrest = zrest
        self.id = pid
        self.bone = bone
        self.material = material
        self.kind = kind
        self.geo = geo
        self.region = region
        self.role = role
        self.tone = tone
        self.flags = flags or {}
        self.pen = pen
        self.draw = draw
        self.mass = mass
        self.com = com
        self.extra = extra or {}

    def solid(self):
        """🧊️ The 3D solid of the part."""
        g = self.geo
        if self.kind == "box":
            lo, hi = g["lo"], g["hi"]
            s = {"kind": "box", "size": [r6(h - l) for l, h in zip(lo, hi)], "transform": {"translation": [r6((l + h) / 2) for l, h in zip(lo, hi)]}}
        elif self.kind == "pv":
            s = {"kind": "extrude", "profile": pts([(w, u) for u, w in ccw(g["profile"])]), "plane": "zx", "from": r6(g["v0"]), "to": r6(g["v1"])}
            if g.get("holes"):
                s["holes"] = [pts([(w, u) for u, w in ccw(h)][::-1]) for h in g["holes"]]
        elif self.kind == "pu":
            s = {"kind": "extrude", "profile": pts(ccw(g["profile"])), "plane": "yz", "from": r6(g["u0"]), "to": r6(g["u1"])}
            if g.get("holes"):
                s["holes"] = [pts(ccw(h)[::-1]) for h in g["holes"]]
        elif self.kind == "ru":
            s = {"kind": "revolve", "profile": pts(g["profile"]), "axis": [1.0, 0.0, 0.0], "origin": [0.0, 0.0, 0.0]}
        elif self.kind == "tube":
            s = {"kind": "loft", "sections": [{"at": [r6(a), r6(g["v"]), r6(b)], "rx": r6(r), "ry": r6(r)} for a, b, r in g["path"]], "capStart": True, "capEnd": True}
        elif self.kind == "cap_v":
            u, w, r = g["u"], g["w"], g["r"]
            s = {"kind": "capsule", "p0": [r6(u), r6(g["v0"]), r6(w)], "p1": [r6(u), r6(g["v1"]), r6(w)], "r0": r6(r), "r1": r6(r)}
        elif self.kind == "cap_u":
            v, w, r = g["v"], g["w"], g["r"]
            s = {"kind": "capsule", "p0": [r6(g["u0"] + r), r6(v), r6(w)], "p1": [r6(g["u1"] - r), r6(v), r6(w)], "r0": r6(r), "r1": r6(r)}
        else:
            raise ValueError(self.kind)
        out = {"id": self.id, **s, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        for key in ("slender", "sheet", "texture", "envelope"):
            if self.flags.get(key):
                out[key] = True
        out.update(self.extra)
        return out

    def shape(self, z):
        """✏️ The 2D shape of the part with its `solid` placement (the same in elevation, front and plan)."""
        g = self.geo
        if self.zrest is not None and self.kind == "box":
            lo, hi = g["lo"], g["hi"]
            points, solid = rect(lo[0], hi[0], lo[1], hi[1]), {"kind": "extruded", "sweep": "w", "z0": r6(self.zrest + lo[2]), "z1": r6(self.zrest + hi[2])}
        elif self.zrest is not None and self.kind == "cap_u":
            points, solid = rect(g["u0"], g["u1"], g["v"] - g["r"], g["v"] + g["r"]), {"kind": "extruded", "sweep": "w", "z0": r6(self.zrest + g["w"] - g["r"]), "z1": r6(self.zrest + g["w"] + g["r"])}
        elif self.kind == "box":
            lo, hi = g["lo"], g["hi"]
            points, solid = rect(lo[0], hi[0], lo[2], hi[2]), {"kind": "extruded", "sweep": "v", "z0": r6(lo[1]), "z1": r6(hi[1])}
        elif self.kind == "pv":
            points, solid = g["profile"], {"kind": "extruded", "sweep": "v", "z0": r6(g["v0"]), "z1": r6(g["v1"])}
        elif self.kind == "pu":
            points, solid = g["profile"], {"kind": "extruded", "sweep": "u", "z0": r6(g["u0"]), "z1": r6(g["u1"])}
        elif self.kind == "ru":
            points = g.get("outline") or (g["profile"][:-1] if g["profile"][0] == g["profile"][-1] else half_outline(g["profile"]))
            solid = {"kind": "revolved", "axisBone": self.bone, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
        elif self.kind == "tube":
            rmax = max(r for _, _, r in g["path"])
            points, solid = band(g["path"]), {"kind": "extruded", "sweep": "v", "z0": r6(g["v"] - rmax), "z1": r6(g["v"] + rmax)}
        elif self.kind == "cap_v":
            points, solid = circle(g["u"], g["w"], g["r"], 12), {"kind": "extruded", "sweep": "v", "z0": r6(min(g["v0"], g["v1"])), "z1": r6(max(g["v0"], g["v1"]))}
        elif self.kind == "cap_u":
            points, solid = circle(g["v"], g["w"], g["r"], 12), {"kind": "extruded", "sweep": "u", "z0": r6(g["u0"]), "z1": r6(g["u1"])}
        else:
            raise ValueError(self.kind)
        shape = {"id": self.id, "bone": self.bone, "role": self.role, "z": round(z, 3), "filled": True, "tone": self.tone}
        if self.region:
            shape["region"] = self.region
        shape["material"] = self.material
        if self.pen is not None:
            shape["penMm"] = self.pen
        ring = pts(ccw(points) if self.kind != "ru" else points)
        if self.kind == "pv" and g.get("holes"):
            d = "M " + " L ".join(f"{a} {b}" for a, b in ring) + " Z " + " ".join("M " + " L ".join(f"{a} {b}" for a, b in pts(ccw(h)[::-1])) + " Z" for h in g["holes"])
            shape.update({"kind": "path", "d": d, "solid": solid})
        else:
            shape.update({"kind": "poly", "points": ring, "closed": True, "solid": solid})
        return shape
