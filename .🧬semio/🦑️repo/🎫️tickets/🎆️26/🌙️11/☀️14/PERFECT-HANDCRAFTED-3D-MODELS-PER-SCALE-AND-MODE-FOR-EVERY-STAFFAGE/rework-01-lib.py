"""🪚 Author 01 chisel: rest frames of a rig, world-to-bone conversion and the solid writers (loft through hand-stated sections, ellipsoid, capsule, box) used by the `rework-01-<slug>-*.py` scripts. It holds no dimension of any asset: every number is stated in the asset scripts.

World: x ahead, y up, z left. Bone frame: u along the bone, v its normal in the sagittal plane (u turned +90 degrees), w lateral (left is +).
"""
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
OUT = os.path.join(T, "🗑️generated", "rework-01")


def catalog_path(category_folder, slug):
    return os.path.join(CATALOG, category_folder, slug, "🔣️.json")


def read(path):
    with open(path, encoding="utf8") as handle:
        return json.load(handle)


def write(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


def r5(value):
    return round(value, 5) + 0.0


class Frames:
    """🦴 Rest frames of every bone in the world."""

    def __init__(self, rig):
        self.bones = {b["id"]: b for b in rig["bones"]}
        self.rest = {}
        for bone in rig["bones"]:
            self._solve(bone["id"])

    def _solve(self, bone_id):
        if bone_id in self.rest:
            return self.rest[bone_id]
        bone = self.bones[bone_id]
        if bone["parent"] is None:
            ox, oy, oz, angle = bone["x"], bone["y"], bone.get("z", 0), bone["angleDeg"]
        else:
            px, py, pz, pa = self._solve(bone["parent"])
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            ox, oy = px + bone["x"] * c - bone["y"] * s, py + bone["x"] * s + bone["y"] * c
            oz, angle = pz + bone.get("z", 0), pa + bone["angleDeg"]
        self.rest[bone_id] = (ox, oy, oz, angle)
        return self.rest[bone_id]

    def local(self, bone_id, p):
        ox, oy, oz, angle = self.rest[bone_id]
        c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
        dx, dy = p[0] - ox, p[1] - oy
        return [dx * c + dy * s, -dx * s + dy * c, p[2] - oz]

    def direction(self, bone_id, d):
        angle = self.rest[bone_id][3]
        c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
        return [d[0] * c + d[1] * s, -d[0] * s + d[1] * c, d[2]]

    def world(self, bone_id, q):
        ox, oy, oz, angle = self.rest[bone_id]
        c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
        return [ox + q[0] * c - q[1] * s, oy + q[0] * s + q[1] * c, oz + q[2]]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(sum(x * x for x in a)) or 1
    return [x / n for x in a]


def quat(ex, ey):
    """🧭 The unit quaternion (x, y, z, w) that turns (1,0,0) into ex and (0,1,0) into ey."""
    ex = unit(ex)
    ez = unit(cross(ex, ey))
    ey = cross(ez, ex)
    m = [[ex[0], ey[0], ez[0]], [ex[1], ey[1], ez[1]], [ex[2], ey[2], ez[2]]]
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        s = math.sqrt(trace + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [r5(c) for c in q]


def station(c, ex, ey, rx, ry, e=2.0):
    """📍 One loft section in the world: centre, the directions of rx and ry, the radii and the superellipse exponent."""
    return {"c": list(c), "ex": list(ex), "ey": list(ey), "rx": rx, "ry": ry, "e": e}


def H(y, front, back, hw, zc=0.0, e=2.0):
    """📍 Horizontal section at height y: the surface reaches world x = front ahead and x = back behind, half width hw about z = zc."""
    return station([(front + back) / 2, y, zc], [0, 0, 1], [1, 0, 0], hw, (front - back) / 2, e)


def X(x, bottom, top, hw, zc=0.0, e=2.0):
    """📍 Vertical cross-section at world x between the heights bottom and top, half width hw about z = zc."""
    return station([x, (bottom + top) / 2, zc], [0, 0, 1], [0, 1, 0], hw, (top - bottom) / 2, e)


def Z(z, xc, yc, rx, ry, e=2.0):
    """📍 Sagittal section at world z, centre (xc, yc), radius rx ahead and ry up."""
    return station([xc, yc, z], [1, 0, 0], [0, 1, 0], rx, ry, e)


def catmull(p0, p1, p2, p3, t):
    return 0.5 * ((2 * p1) + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (-p0 + 3 * p1 - 3 * p2 + p3) * t * t * t)


def smooth(stations, n):
    """🪶 n - 1 sections interpolated between every two stated stations (Catmull-Rom through the stated ones, radii never below the smaller neighbour's 0.9)."""
    if n <= 1 or len(stations) < 3:
        return stations
    out = []
    count = len(stations)
    for i in range(count - 1):
        a, b = stations[i], stations[i + 1]
        p, q = stations[max(0, i - 1)], stations[min(count - 1, i + 2)]
        out.append(a)
        for k in range(1, n):
            t = k / n
            c = [catmull(p["c"][j], a["c"][j], b["c"][j], q["c"][j], t) for j in range(3)]
            lo_x, lo_y = 0.9 * min(a["rx"], b["rx"]), 0.9 * min(a["ry"], b["ry"])
            rx = max(lo_x, catmull(p["rx"], a["rx"], b["rx"], q["rx"], t))
            ry = max(lo_y, catmull(p["ry"], a["ry"], b["ry"], q["ry"], t))
            ex = unit([a["ex"][j] * (1 - t) + b["ex"][j] * t for j in range(3)])
            ey = unit([a["ey"][j] * (1 - t) + b["ey"][j] * t for j in range(3)])
            out.append(station(c, ex, ey, rx, ry, a["e"] * (1 - t) + b["e"] * t))
    out.append(stations[-1])
    return out


def dome(end, toward, depth, steps=3):
    """🥚 Rounded end: `steps` sections that shrink elliptically from the stated end section over `depth` along the unit direction `toward`, last first when used as a start."""
    out = []
    for k in range(1, steps + 1):
        a = (math.pi / 2) * k / (steps + 0.35)
        c = [end["c"][j] + toward[j] * depth * math.sin(a) for j in range(3)]
        out.append(station(c, end["ex"], end["ey"], end["rx"] * math.cos(a), end["ry"] * math.cos(a), end["e"]))
    return out


class Kit:
    """🧰 Writers of solids in the rest frame of their bone from world statements."""

    def __init__(self, frames):
        self.f = frames

    def base(self, sid, kind, bone, material, tone, role, extra):
        solid = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
        solid.update(extra or {})
        return solid

    def loft(self, sid, bone, material, tone, stations, n=1, role="body", start=None, end=None, steps=3, **extra):
        """🧵 Loft through world stations; `start` / `end` are dome depths added before the first and after the last station along the path."""
        sts = smooth(stations, n)
        if start:
            d = unit([sts[0]["c"][j] - sts[1]["c"][j] for j in range(3)])
            sts = list(reversed(dome(sts[0], d, start, steps))) + sts
        if end:
            d = unit([sts[-1]["c"][j] - sts[-2]["c"][j] for j in range(3)])
            sts = sts + dome(sts[-1], d, end, steps)
        sections = []
        for s in sts:
            ex, ey = self.f.direction(bone, s["ex"]), self.f.direction(bone, s["ey"])
            sections.append({"at": [r5(c) for c in self.f.local(bone, s["c"])], "rx": r5(s["rx"]), "ry": r5(s["ry"]), "exponent": round(s["e"], 3), "rotation": quat(ex, ey)})
        solid = self.base(sid, "loft", bone, material, tone, role, None)
        solid.update({"sections": sections, "capStart": True, "capEnd": True})
        solid.update(extra)
        return solid

    def limb(self, bone, side, rows):
        """📍 Stations across a limb bone from rows (u, front, back, medial, lateral[, exponent]): extents from the bone axis ahead, behind, towards the midline and away from it."""
        ox, oy, oz, angle = self.f.rest[bone]
        c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
        u, v = [c, s, 0], [-s, c, 0]
        ahead = 1 if v[0] >= 0 else -1
        out = []
        for row in rows:
            at, front, back, medial, lateral = row[:5]
            e = row[5] if len(row) > 5 else 2.0
            vc, wc = ahead * (front - back) / 2, side * (lateral - medial) / 2
            centre = [ox + u[0] * at + v[0] * vc, oy + u[1] * at + v[1] * vc, oz + wc]
            out.append(station(centre, [0, 0, 1], v, (medial + lateral) / 2, (front + back) / 2, e))
        return out

    def ellipsoid(self, sid, bone, material, tone, centre, radii, role="body", tilt=0.0, **extra):
        """🥚 Ellipsoid with world radii (ahead, up, lateral) about a world centre, tilted by `tilt` degrees in the sagittal plane (counter-clockwise seen from the right)."""
        angle = math.radians(tilt - self.f.rest[bone][3])
        solid = self.base(sid, "ellipsoid", bone, material, tone, role, None)
        solid["radii"] = [r5(x) for x in radii]
        solid["transform"] = {"translation": [r5(x) for x in self.f.local(bone, centre)], "rotation": [0, 0, r5(math.sin(angle / 2)), r5(math.cos(angle / 2))]}
        solid.update(extra)
        return solid

    def capsule(self, sid, bone, material, tone, p0, p1, r0, r1=None, role="body", **extra):
        solid = self.base(sid, "capsule", bone, material, tone, role, None)
        solid.update({"p0": [r5(x) for x in self.f.local(bone, p0)], "p1": [r5(x) for x in self.f.local(bone, p1)], "r0": r5(r0), "r1": r5(r0 if r1 is None else r1)})
        solid.update(extra)
        return solid

    def box(self, sid, bone, material, tone, centre, size, role="body", tilt=0.0, corner=0.0, **extra):
        """📦 Box with world edge lengths (ahead, up, lateral) about a world centre, tilted in the sagittal plane."""
        angle = math.radians(tilt - self.f.rest[bone][3])
        solid = self.base(sid, "box", bone, material, tone, role, None)
        solid["size"] = [r5(x) for x in size]
        if corner:
            solid["cornerRadius"] = r5(corner)
        solid["transform"] = {"translation": [r5(x) for x in self.f.local(bone, centre)], "rotation": [0, 0, r5(math.sin(angle / 2)), r5(math.cos(angle / 2))]}
        solid.update(extra)
        return solid


def polyline(points, n):
    """🪢 Catmull-Rom points through stated world points, n per span."""
    out = []
    count = len(points)
    for i in range(count - 1):
        p, a, b, q = points[max(0, i - 1)], points[i], points[i + 1], points[min(count - 1, i + 2)]
        for k in range(n):
            out.append([catmull(p[j], a[j], b[j], q[j], k / n) for j in range(3)])
    out.append(list(points[-1]))
    return out


def tube(points, radii, e=2.0, up=(0, 1, 0)):
    """🪈 Stations of a tube along world points: sections perpendicular to the path, rx across the path and `up`, ry towards `up`; radii is one (rx, ry) or one per point."""
    out = []
    count = len(points)
    for i, p in enumerate(points):
        a, b = points[max(0, i - 1)], points[min(count - 1, i + 1)]
        t = unit([b[j] - a[j] for j in range(3)])
        ex = cross(up, t)
        if sum(x * x for x in ex) < 1e-8:
            ex = cross((1, 0, 0), t)
        ex = unit(ex)
        ey = unit(cross(t, ex))
        rx, ry = radii[i] if isinstance(radii[0], (list, tuple)) else radii
        out.append(station(p, ex, ey, rx, ry, e))
    return out


def catalog_text(value, depth=0):
    """🖨️ The layout of the catalog files outside `volumes`: two spaces, lists of numbers and lists of number lists on one line."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    number = lambda x: isinstance(x, (int, float)) and not isinstance(x, bool)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + catalog_text(x, depth + 1) for k, x in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if all(number(i) for i in value) or all(isinstance(i, list) and all(number(leaf) for leaf in i) for i in value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + catalog_text(x, depth + 1) for x in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)
