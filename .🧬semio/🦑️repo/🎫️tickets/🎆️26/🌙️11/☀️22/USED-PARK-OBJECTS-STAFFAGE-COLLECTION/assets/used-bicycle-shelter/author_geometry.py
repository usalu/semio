"""🚲️ Geometry kernel of the used bicycle shelter: rest frames of the sagittal rig, parts in bone-local metres and their 2D projections.

Every part is authored once in the rest frame of its bone (u along the bone, v its normal, w lateral, left +) and gives the body solid
and its drawings in the side elevation (bone-local u, v), the front elevation (world z, y) and the plan (world x, z) from the same numbers.
World frame of the staffage archive: x ahead, y up, z to the left, ground at y = 0.
"""
import math

C5, S5, T5 = math.cos(math.radians(5)), math.sin(math.radians(5)), math.tan(math.radians(5))


def r6(v):
    """🎯️ Rounds a number to micrometres for stable output."""
    return round(float(v) + 0.0, 6)


def t(en, de):
    """💬️ A bilingual text."""
    return {"en": en, "de": de}


class Rig:
    """🦴️ A planar (sagittal) rig authored by world rest origins and world angles; relative placements are derived."""

    def __init__(self):
        self.bones = {}
        self.order = []

    def add(self, bid, parent, x, y, z, angle, length, joint, title, section=None):
        """➕️ Adds a bone at the world rest origin (x, y, z) with the world angle `angle` (degrees)."""
        self.bones[bid] = {"id": bid, "parent": parent, "x": x, "y": y, "z": z, "a": angle, "L": length, "joint": joint, "title": title, "section": section, "kg": 0.0, "mu": 0.0, "mv": 0.0}
        self.order.append(bid)

    def world(self, bid, u, v, w=0.0):
        """🌍️ A bone-local point in world coordinates."""
        b = self.bones[bid]
        ca, sa = math.cos(math.radians(b["a"])), math.sin(math.radians(b["a"]))
        return (b["x"] + u * ca - v * sa, b["y"] + u * sa + v * ca, b["z"] + w)

    def local(self, bid, x, y, z=None):
        """📍️ A world point in the bone-local frame."""
        b = self.bones[bid]
        ca, sa = math.cos(math.radians(b["a"])), math.sin(math.radians(b["a"]))
        dx, dy = x - b["x"], y - b["y"]
        return (dx * ca + dy * sa, -dx * sa + dy * ca, (z - b["z"]) if z is not None else 0.0)

    def json(self, mass_meta):
        """🧾️ The rig bones with placements relative to their parents."""
        out = []
        for bid in self.order:
            b = self.bones[bid]
            if b["parent"] is None:
                rx, ry, rz, ra = b["x"], b["y"], b["z"], b["a"]
            else:
                p = self.bones[b["parent"]]
                rx, ry, rz = self.local(b["parent"], b["x"], b["y"], b["z"])
                ra = b["a"] - p["a"]
            e = {"id": bid, "parent": b["parent"], "title": b["title"], "x": r6(rx), "y": r6(ry)}
            if abs(rz) > 1e-9:
                e["z"] = r6(rz)
            e.update({"angleDeg": r6(ra), "lengthM": r6(b["L"]), "joint": b["joint"]})
            if b["kg"] > 0:
                e["mass"] = {"kg": round(b["kg"], 3), "comX": r6(b["mu"] / b["kg"]), "comY": r6(b["mv"] / b["kg"]), "distribution": "point", "source": "authored-estimate", "reference": mass_meta}
            if b["section"]:
                e["section"] = b["section"]
            out.append(e)
        return out


def hull(points):
    """🪢️ Convex hull (counter-clockwise) of 2D points, monotone chain."""
    pts = sorted(set((round(a, 7), round(b, 7)) for a, b in points))
    if len(pts) <= 2:
        return pts

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 1e-12:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 1e-12:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def circle_pts(n=24):
    """⭕️ Unit circle samples."""
    return [(math.cos(2 * math.pi * k / n), math.sin(2 * math.pi * k / n)) for k in range(n)]


class Part:
    """🧩️ One physical part in the rest frame of its bone, with its material, thermal region, tone and structural role."""

    def __init__(self, pid, bone, kind, geo, material, region, tone, role="body", flags=None, kg=0.0, struct=None, views=("elevation", "front", "plan"), group=None, extra=None):
        self.id, self.bone, self.kind, self.geo = pid, bone, kind, geo
        self.material, self.region, self.tone, self.role = material, region, tone, role
        self.flags = flags or {}
        self.kg, self.struct, self.views, self.group = kg, struct, set(views), group or {}
        self.extra = extra or {}

    def samples(self):
        """🔬️ Bone-local points whose convex hull is the part (for front and plan projections)."""
        g, k = self.geo, self.kind
        if k == "box":
            (cu, cv, cw), (su, sv, sw) = g["c"], g["s"]
            return [(cu + a * su / 2, cv + b * sv / 2, cw + c * sw / 2) for a in (-1, 1) for b in (-1, 1) for c in (-1, 1)]
        if k == "cyl":
            ax, (p, q), a0, a1, r = g["axis"], g["at"], g["a0"], g["a1"], g["r"]
            out = []
            for a in (a0, a1):
                for cx, cy in circle_pts():
                    if ax == "u":
                        out.append((a, p + r * cx, q + r * cy))
                    elif ax == "v":
                        out.append((p + r * cx, a, q + r * cy))
                    else:
                        out.append((p + r * cx, q + r * cy, a))
            return out
        if k == "cap":
            out = []
            for c in (g["p0"], g["p1"]):
                for d in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)] + [(x * 0.7071, y * 0.7071, 0) for x, y in ((1, 1), (1, -1), (-1, 1), (-1, -1))] + [(x * 0.7071, 0, z * 0.7071) for x, z in ((1, 1), (1, -1), (-1, 1), (-1, -1))] + [(0, y * 0.7071, z * 0.7071) for y, z in ((1, 1), (1, -1), (-1, 1), (-1, -1))]:
                    out.append((c[0] + g["r"] * d[0], c[1] + g["r"] * d[1], c[2] + g["r"] * d[2]))
            return out
        if k in ("torus", "ring"):
            cu, cv, cw = g["c"]
            ro = g["R"] + g["r"] if k == "torus" else g["ro"]
            hw = g["r"] if k == "torus" else g["hw"]
            return [(cu + ro * x, cv + ro * y, cw + s * hw) for x, y in circle_pts(32) for s in (-1, 1)]
        if k == "ext":
            return [(u, v, w) for u, v in g["profile"] for w in (g["w0"], g["w1"])]
        if k == "extx":
            return [(u, v, w) for v, w in g["profile"] for u in (g["u0"], g["u1"])]
        if k == "ell":
            (cu, cv, cw), (ru, rv, rw) = g["c"], g["radii"]
            out = []
            for x, y in circle_pts(16):
                out += [(cu + ru * x, cv + rv * y, cw), (cu + ru * x, cv, cw + rw * y), (cu, cv + rv * x, cw + rw * y)]
            return out
        raise ValueError(k)

    def solid(self):
        """🧊️ The body solid of the part in the bone-local frame."""
        g, k = self.geo, self.kind
        s = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if k == "box":
            s.update({"kind": "box", "size": [r6(x) for x in g["s"]], "transform": {"translation": [r6(x) for x in g["c"]]}})
        elif k == "cyl":
            ax, (p, q), a0, a1, r = g["axis"], g["at"], g["a0"], g["a1"], g["r"]
            axis = {"u": [1, 0, 0], "v": [0, 1, 0], "w": [0, 0, 1]}[ax]
            origin = {"u": [0, p, q], "v": [p, 0, q], "w": [p, q, 0]}[ax]
            s.update({"kind": "revolve", "profile": [[r6(a0), 0], [r6(a0), r6(r)], [r6(a1), r6(r)], [r6(a1), 0]], "axis": axis, "origin": [r6(x) for x in origin]})
        elif k == "cap":
            s.update({"kind": "capsule", "p0": [r6(x) for x in g["p0"]], "p1": [r6(x) for x in g["p1"]], "r0": r6(g["r"]), "r1": r6(g["r"])})
        elif k == "torus":
            prof = [[r6(g["r"] * math.cos(2 * math.pi * i / 16)), r6(g["R"] + g["r"] * math.sin(2 * math.pi * i / 16))] for i in range(16)]
            s.update({"kind": "revolve", "profile": prof + [prof[0]], "axis": [0, 0, 1], "origin": [r6(x) for x in g["c"]]})
        elif k == "ring":
            hw, ri, ro = g["hw"], g["ri"], g["ro"]
            s.update({"kind": "revolve", "profile": [[r6(-hw), r6(ri)], [r6(hw), r6(ri)], [r6(hw), r6(ro)], [r6(-hw), r6(ro)], [r6(-hw), r6(ri)]], "axis": [0, 0, 1], "origin": [r6(x) for x in g["c"]]})
        elif k == "ext":
            s.update({"kind": "extrude", "profile": [[r6(a), r6(b)] for a, b in g["profile"]], "plane": "xy", "from": r6(g["w0"]), "to": r6(g["w1"])})
        elif k == "extx":
            s.update({"kind": "extrude", "profile": [[r6(a), r6(b)] for a, b in g["profile"]], "plane": "yz", "from": r6(g["u0"]), "to": r6(g["u1"])})
        elif k == "ell":
            s.update({"kind": "ellipsoid", "radii": [r6(x) for x in g["radii"]], "transform": {"translation": [r6(x) for x in g["c"]]}})
        for key in ("slender", "sheet", "texture", "envelope"):
            if self.flags.get(key):
                s[key] = True
        if "repeat" in g:
            s["repeat"] = g["repeat"]
        return s

    def elevation(self):
        """✏️ The side-elevation geometry in the bone's own (u, v) plane."""
        g, k = self.geo, self.kind
        if k == "box":
            (cu, cv, _), (su, sv, _) = g["c"], g["s"]
            return {"kind": "rect", "x": r6(cu - su / 2), "y": r6(cv - sv / 2), "w": r6(su), "h": r6(sv)}
        if k == "cyl":
            ax, (p, q), a0, a1, r = g["axis"], g["at"], g["a0"], g["a1"], g["r"]
            if ax == "u":
                return {"kind": "rect", "x": r6(a0), "y": r6(p - r), "w": r6(a1 - a0), "h": r6(2 * r)}
            if ax == "v":
                return {"kind": "rect", "x": r6(p - r), "y": r6(min(a0, a1)), "w": r6(2 * r), "h": r6(abs(a1 - a0))}
            return {"kind": "ellipse", "cx": r6(p), "cy": r6(q), "rx": r6(r), "ry": r6(r)}
        if k == "cap":
            (u0, v0, _), (u1, v1, _) = g["p0"], g["p1"]
            if math.hypot(u1 - u0, v1 - v0) < 1e-6:
                return {"kind": "ellipse", "cx": r6(u0), "cy": r6(v0), "rx": r6(g["r"]), "ry": r6(g["r"])}
            return {"kind": "capsule", "x0": r6(u0), "y0": r6(v0), "x1": r6(u1), "y1": r6(v1), "r0": r6(g["r"]), "r1": r6(g["r"])}
        if k in ("torus", "ring"):
            cu, cv, _ = g["c"]
            ro, ri = (g["R"] + g["r"], g["R"] - g["r"]) if k == "torus" else (g["ro"], g["ri"])
            return annulus(cu, cv, ro, ri)
        if k == "ext":
            return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in g["profile"]], "closed": True}
        if k == "extx":
            vs = [v for v, _ in g["profile"]]
            return {"kind": "rect", "x": r6(g["u0"]), "y": r6(min(vs)), "w": r6(g["u1"] - g["u0"]), "h": r6(max(vs) - min(vs))}
        if k == "ell":
            (cu, cv, _), (ru, rv, _) = g["c"], g["radii"]
            return {"kind": "ellipse", "cx": r6(cu), "cy": r6(cv), "rx": r6(ru), "ry": r6(rv)}
        raise ValueError(k)


def annulus(cx, cy, ro, ri):
    """💍️ A ring as one even-odd path of two circles."""
    def circ(r):
        return f"M {r6(cx + r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx - r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx + r)} {r6(cy)} Z"
    return {"kind": "path", "d": circ(ro) + " " + circ(ri)}


def project(rig, bone, pts, view):
    """🔭️ Bone-local points to the paper frame of the front (world z, y) or plan (world x, z) view."""
    out = []
    for u, v, w in pts:
        x, y, z = rig.world(bone, u, v, w)
        out.append((z, y) if view == "front" else (x, z))
    return out


def view_geometry(rig, part, view):
    """🗺️ The shape geometry of a part in a view: capsules stay capsules, everything else is the hull of its projected samples."""
    if view == "elevation":
        return part.elevation()
    if part.kind == "cap":
        g = part.geo
        (a0, b0), (a1, b1) = project(rig, part.bone, [g["p0"], g["p1"]], view)
        if math.hypot(a1 - a0, b1 - b0) < 1e-6:
            return {"kind": "ellipse", "cx": r6(a0), "cy": r6(b0), "rx": r6(g["r"]), "ry": r6(g["r"])}
        return {"kind": "capsule", "x0": r6(a0), "y0": r6(b0), "x1": r6(a1), "y1": r6(b1), "r0": r6(g["r"]), "r1": r6(g["r"])}
    h = hull(project(rig, part.bone, part.samples(), view))
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in h], "closed": True}


def depth(rig, part, view):
    """📏️ A painting key: nearer to the viewer paints later (elevation from the right, front from ahead, plan from above)."""
    ws = [rig.world(part.bone, *p) for p in part.samples()]
    if view == "elevation":
        return -min(p[2] for p in ws)
    if view == "front":
        return max(p[0] for p in ws)
    return max(p[1] for p in ws)


def band(points, r):
    """➰️ The filled band of half width r around an open centre polyline (mitred joints), as one simple polygon."""
    def offset(sign):
        out = []
        n = len(points)
        for i in range(n):
            if i == 0:
                dx, dy = points[1][0] - points[0][0], points[1][1] - points[0][1]
            elif i == n - 1:
                dx, dy = points[-1][0] - points[-2][0], points[-1][1] - points[-2][1]
            else:
                ax, ay = points[i][0] - points[i - 1][0], points[i][1] - points[i - 1][1]
                bx, by = points[i + 1][0] - points[i][0], points[i + 1][1] - points[i][1]
                la, lb = math.hypot(ax, ay), math.hypot(bx, by)
                dx, dy = ax / la + bx / lb, ay / la + by / lb
            ln = math.hypot(dx, dy)
            nx, ny = -dy / ln, dx / ln
            scale = 1.0
            if 0 < i < n - 1:
                ax, ay = points[i][0] - points[i - 1][0], points[i][1] - points[i - 1][1]
                la = math.hypot(ax, ay)
                scale = 1.0 / max(0.2, (nx * (-ay / la) + ny * (ax / la)))
            out.append((points[i][0] + sign * r * scale * nx, points[i][1] + sign * r * scale * ny))
        return out
    left, right = offset(1), offset(-1)
    return left + list(reversed(right))


def area_of(part):
    """📐️ Analytic outer area of a part alone (m²), for the first estimate of `areaM2`."""
    g, k = part.geo, part.kind
    rep = g.get("repeat", {}).get("count", 1)
    if k == "box":
        a, b, c = g["s"]
        return rep * 2 * (a * b + b * c + c * a)
    if k == "cyl":
        L = abs(g["a1"] - g["a0"])
        return rep * (2 * math.pi * g["r"] * L + 2 * math.pi * g["r"] ** 2)
    if k == "cap":
        L = math.dist(g["p0"], g["p1"])
        return rep * (2 * math.pi * g["r"] * L + 4 * math.pi * g["r"] ** 2)
    if k == "torus":
        return rep * 4 * math.pi ** 2 * g["R"] * g["r"]
    if k == "ring":
        return rep * (2 * math.pi * (g["ri"] + g["ro"]) * 2 * g["hw"] + 2 * math.pi * (g["ro"] ** 2 - g["ri"] ** 2))
    if k in ("ext", "extx"):
        pr = g["profile"]
        L = abs(g["w1"] - g["w0"]) if k == "ext" else abs(g["u1"] - g["u0"])
        per = sum(math.dist(pr[i], pr[(i + 1) % len(pr)]) for i in range(len(pr)))
        ar = abs(sum(pr[i][0] * pr[(i + 1) % len(pr)][1] - pr[(i + 1) % len(pr)][0] * pr[i][1] for i in range(len(pr)))) / 2
        return rep * (per * L + 2 * ar)
    if k == "ell":
        a, b, c = g["radii"]
        p = 1.6075
        return rep * 4 * math.pi * (((a * b) ** p + (a * c) ** p + (b * c) ** p) / 3) ** (1 / p)
    return 0.0
