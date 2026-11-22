"""🧮 Geometry kit of the used-wall-unit author: the rig frames, world-rest to bone-local conversion, solid builders with their volumes and the 2D shape builders.

Imported by `author-used-wall-unit.py`; it holds no asset numbers of its own beyond the rig placement it is given.
"""
import math

R = lambda v: round(v + 0.0, 6)


class Rig:
    """🦴 World-rest placement of every bone: origin (x, y, z) and the world angle of its u axis in the side elevation."""

    def __init__(self):
        self.bones = {}

    def add(self, bone_id, origin, angle):
        self.bones[bone_id] = {"o": origin, "a": angle}

    def loc(self, bone_id, x, y, z=0.0):
        b = self.bones[bone_id]
        ox, oy, oz = b["o"]
        a = math.radians(b["a"])
        dx, dy = x - ox, y - oy
        return (R(dx * math.cos(a) + dy * math.sin(a)), R(-dx * math.sin(a) + dy * math.cos(a)), R(z - oz))

    def uv(self, bone_id, x, y):
        u, v, _ = self.loc(bone_id, x, y, 0.0)
        return [u, v]

    def angle(self, bone_id):
        return self.bones[bone_id]["a"]

    def origin(self, bone_id):
        return self.bones[bone_id]["o"]


def poly_area(pts):
    s = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        s += x0 * y1 - x1 * y0
    return abs(s) / 2.0


def poly_centroid(pts):
    a = 0.0
    cx = cy = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a *= 0.5
    if abs(a) < 1e-15:
        return (sum(p[0] for p in pts) / len(pts), sum(p[1] for p in pts) / len(pts))
    return (cx / (6 * a), cy / (6 * a))


def circle(cx, cy, r, n=24):
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def rect_pts(a0, a1, b0, b1):
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def rounded_strip(x0, x1, y0, y1, n=6):
    r = (y1 - y0) / 2.0
    cy = (y0 + y1) / 2.0
    pts = [(x0 + r, y0), (x1 - r, y0)]
    pts += [(x1 - r + r * math.cos(-math.pi / 2 + math.pi * k / n), cy + r * math.sin(-math.pi / 2 + math.pi * k / n)) for k in range(1, n)]
    pts += [(x1 - r, y1), (x0 + r, y1)]
    pts += [(x0 + r + r * math.cos(math.pi / 2 + math.pi * k / n), cy + r * math.sin(math.pi / 2 + math.pi * k / n)) for k in range(1, n)]
    return pts


class Kit:
    """🧰 Builds solids (bone-local, with volume and centroid for the mass budget) and 2D shapes from world-rest geometry."""

    def __init__(self, rig):
        self.rig = rig

    def _base(self, sid, bone, mat, role, tone, flags):
        s = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone}
        for k in ("slender", "sheet", "envelope", "texture"):
            if flags.get(k):
                s[k] = True
        if flags.get("repeat"):
            s["repeat"] = flags["repeat"]
        return s

    def box(self, sid, bone, mat, role, tone, x0, x1, y0, y1, z0, z1, **flags):
        rig = self.rig
        if abs(rig.angle(bone)) < 1e-9:
            s = self._base(sid, bone, mat, role, tone, flags)
            cu, cv, cw = rig.loc(bone, (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2)
            s.update({"kind": "box", "size": [R(x1 - x0), R(y1 - y0), R(z1 - z0)], "transform": {"translation": [cu, cv, cw]}})
            return self._meta(s, (x1 - x0) * (y1 - y0) * (z1 - z0), ((x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2), flags)
        return self.xprof(sid, bone, mat, role, tone, rect_pts(x0, x1, y0, y1), z0, z1, **flags)

    def xprof(self, sid, bone, mat, role, tone, pts, z0, z1, holes=None, **flags):
        rig = self.rig
        s = self._base(sid, bone, mat, role, tone, flags)
        oz = rig.origin(bone)[2]
        s.update({"kind": "extrude", "plane": "xy", "profile": [rig.uv(bone, x, y) for x, y in pts], "from": R(z0 - oz), "to": R(z1 - oz)})
        area = poly_area(pts)
        if holes:
            s["holes"] = [[rig.uv(bone, x, y) for x, y in h] for h in holes]
            area -= sum(poly_area(h) for h in holes)
        cx, cy = poly_centroid(pts)
        return self._meta(s, area * (z1 - z0), (cx, cy, (z0 + z1) / 2), flags)

    def yzring(self, sid, bone, mat, role, tone, outer, holes, x0, x1, **flags):
        rig = self.rig
        ox, oy, oz = rig.origin(bone)
        a = rig.angle(bone)
        s = self._base(sid, bone, mat, role, tone, flags)
        if abs(a) < 1e-9:
            s.update({"kind": "extrude", "plane": "yz", "profile": [[R(y - oy), R(z - oz)] for y, z in outer], "from": R(x0 - ox), "to": R(x1 - ox)})
            if holes:
                s["holes"] = [[[R(y - oy), R(z - oz)] for y, z in h] for h in holes]
        elif abs(a - 90) < 1e-9:
            s.update({"kind": "extrude", "plane": "zx", "profile": [[R(z - oz), R(y - oy)] for y, z in outer], "from": R(ox - x1), "to": R(ox - x0)})
            if holes:
                s["holes"] = [[[R(z - oz), R(y - oy)] for y, z in h] for h in holes]
        else:
            raise ValueError("yzring needs a bone at 0 or 90 degrees")
        area = poly_area(outer) - sum(poly_area(h) for h in (holes or []))
        cy, cz = poly_centroid(outer)
        return self._meta(s, area * (x1 - x0), ((x0 + x1) / 2, cy, cz), flags)

    def xzring(self, sid, bone, mat, role, tone, outer, holes, y0, y1, **flags):
        rig = self.rig
        ox, oy, oz = rig.origin(bone)
        s = self._base(sid, bone, mat, role, tone, flags)
        s.update({"kind": "extrude", "plane": "zx", "profile": [[R(z - oz), R(x - ox)] for x, z in outer], "from": R(y0 - oy), "to": R(y1 - oy)})
        if holes:
            s["holes"] = [[[R(z - oz), R(x - ox)] for x, z in h] for h in holes]
        area = poly_area(outer) - sum(poly_area(h) for h in (holes or []))
        cx, cz = poly_centroid(outer)
        return self._meta(s, area * (y1 - y0), (cx, (y0 + y1) / 2, cz), flags)

    def knob(self, sid, bone, mat, role, tone, x0, y, z, profile, **flags):
        rig = self.rig
        a = math.radians(rig.angle(bone))
        s = self._base(sid, bone, mat, role, tone, flags)
        u, v, w = rig.loc(bone, x0, y, z)
        s.update({"kind": "revolve", "profile": [[R(p), R(q)] for p, q in profile], "axis": [R(math.cos(a)), R(-math.sin(a)), 0], "origin": [u, v, w]})
        area = poly_area(profile + [(profile[-1][0], 0.0), (profile[0][0], 0.0)])
        cr = poly_centroid(profile + [(profile[-1][0], 0.0), (profile[0][0], 0.0)])[1]
        return self._meta(s, 2 * math.pi * abs(cr) * area, (x0 + profile[-1][0] / 2, y, z), flags)

    def capsule(self, sid, bone, mat, role, tone, p0, p1, r, **flags):
        rig = self.rig
        s = self._base(sid, bone, mat, role, tone, flags)
        s.update({"kind": "capsule", "p0": list(rig.loc(bone, *p0)), "p1": list(rig.loc(bone, *p1)), "r0": r, "r1": r})
        length = math.dist(p0, p1)
        return self._meta(s, math.pi * r * r * length + 4 / 3 * math.pi * r ** 3, tuple((a + b) / 2 for a, b in zip(p0, p1)), flags)

    def ellipsoid(self, sid, bone, mat, role, tone, c, radii, **flags):
        rig = self.rig
        s = self._base(sid, bone, mat, role, tone, flags)
        s.update({"kind": "ellipsoid", "radii": list(radii), "transform": {"translation": list(rig.loc(bone, *c))}})
        return self._meta(s, 4 / 3 * math.pi * radii[0] * radii[1] * radii[2], c, flags)

    def _meta(self, s, vol, centroid, flags):
        count = flags.get("repeat", {}).get("count", 1) if flags.get("repeat") else 1
        s["_vol"] = vol * count
        step = flags["repeat"].get("translation", [0, 0, 0]) if flags.get("repeat") else [0, 0, 0]
        s["_c"] = tuple(centroid[i] + step[i] * (count - 1) / 2 for i in range(3)) if flags.get("repeat") else centroid
        return s


def clean(solid):
    return {k: v for k, v in solid.items() if not k.startswith("_")}


def ref(sid, of, **label):
    out = {"id": sid, "of": of}
    out.update(label)
    return out


class Draw:
    """✏️ The 2D shapes: elevation in the bone's own (u, v) plane, front and plan in world-rest coordinates."""

    def __init__(self, rig):
        self.rig = rig

    @staticmethod
    def shape(sid, bone, role, z, filled, tone, geom, region=None, material=None, pen=None, hatch=None, solid=None, regions=None):
        s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
        s.update(geom)
        if region:
            s["region"] = region
        if regions:
            s["regions"] = regions
        if material:
            s["material"] = material
        if pen is not None:
            s["penMm"] = pen
        if hatch:
            s["hatch"] = hatch
        if solid:
            s["solid"] = solid
        return s

    @staticmethod
    def rect(x0, x1, y0, y1):
        return {"kind": "rect", "x": R(min(x0, x1)), "y": R(min(y0, y1)), "w": R(abs(x1 - x0)), "h": R(abs(y1 - y0))}

    @staticmethod
    def poly(pts, closed=True):
        return {"kind": "poly", "points": [[R(x), R(y)] for x, y in pts], "closed": closed}

    @staticmethod
    def ellipse(cx, cy, rx, ry):
        return {"kind": "ellipse", "cx": R(cx), "cy": R(cy), "rx": R(rx), "ry": R(ry)}

    @staticmethod
    def path(rings):
        d = " ".join("M " + " L ".join(f"{R(x)} {R(y)}" for x, y in ring) + " Z" for ring in rings)
        return {"kind": "path", "d": d}

    def erect(self, bone, x0, x1, y0, y1):
        if abs(self.rig.angle(bone)) < 1e-9:
            u0, v0 = self.rig.uv(bone, x0, y0)
            u1, v1 = self.rig.uv(bone, x1, y1)
            return self.rect(u0, u1, v0, v1)
        return self.poly([self.rig.uv(bone, x, y) for x, y in rect_pts(x0, x1, y0, y1)])

    def epoly(self, bone, pts, closed=True):
        return self.poly([self.rig.uv(bone, x, y) for x, y in pts], closed)
