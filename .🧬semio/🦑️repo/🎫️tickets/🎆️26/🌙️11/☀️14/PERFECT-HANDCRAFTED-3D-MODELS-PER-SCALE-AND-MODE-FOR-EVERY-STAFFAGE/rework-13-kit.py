"""🪴 Chisel of author 13 (plants): helpers only, no numbers of any asset.

World frame of the hand-stated numbers: x ahead, y up, z left, metres, in the rest pose or, with `Plant(..., pose=ID)`, in that pose (frames from `rework-13-frames.ts --pose ID`); the drawings are always traced at rest, as the product carries them. A plant script states stations, pads, blades and leaves in the world,
`Plant.solid` carries them into the rest frame of their bone (polyhedron, closed and outward), adds the thermal and structural entry and keeps
the mesh so that `Plant.drawings` can trace the three 2D outlines of the very same solid (raster of a quarter feature, boundary walk, Douglas-Peucker).
Usage in a script: `kit = load_kit(); P = kit.Plant("vegetation", "🌳️vegetation", "japanese-maple")`.
"""
import json
import math
import os
import re

T = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}

exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])


def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def mul(a, s):
    return (a[0] * s, a[1] * s, a[2] * s)


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def norm(a):
    length = math.sqrt(dot(a, a)) or 1.0
    return (a[0] / length, a[1] / length, a[2] / length)


def polar(azimuth_deg, radius, y=0.0, centre=(0.0, 0.0, 0.0)):
    """🧭 World point at an azimuth (0 = ahead +x, 90 = left +z) and a radius round a centre."""
    a = math.radians(azimuth_deg)
    return (centre[0] + radius * math.cos(a), centre[1] + y, centre[2] + radius * math.sin(a))


def rotate(p, axis, angle):
    axis = norm(axis)
    c, s = math.cos(angle), math.sin(angle)
    return add(add(mul(p, c), mul(cross(axis, p), s)), mul(axis, dot(axis, p) * (1 - c)))


def spline(stations, sub_steps):
    """🧵 Catmull-Rom through hand-stated stations (x, y, z, r, ...): every component is interpolated."""
    if sub_steps <= 1 or len(stations) < 3:
        return [tuple(s) for s in stations]
    out = []
    n = len(stations)
    for i in range(n - 1):
        p0, p1, p2, p3 = stations[max(i - 1, 0)], stations[i], stations[i + 1], stations[min(i + 2, n - 1)]
        for k in range(sub_steps):
            t = k / sub_steps
            out.append(tuple(0.5 * (2 * b + (c - a) * t + (2 * a - 5 * b + 4 * c - d) * t * t + (3 * b - a - 3 * c + d) * t ** 3) for a, b, c, d in zip(p0, p1, p2, p3)))
    out.append(tuple(stations[-1]))
    return out


def tube(stations, sides=10, sub_steps=3, start="round", end="round", squash=1.0, squash_axis=(0, 0, 1)):
    """🪵 A limb, culm or stem through stations (x, y, z, r): rings on a parallel-transported frame, rounded or flat ends; `squash` flattens the section along `squash_axis`."""
    path = spline(stations, sub_steps)
    pts = [p[:3] for p in path]
    rad = [max(p[3], 1e-4) for p in path]
    tangents = []
    for i in range(len(pts)):
        tangents.append(norm(sub(pts[min(i + 1, len(pts) - 1)], pts[max(i - 1, 0)])))
    ref = (0, 0, 1) if abs(tangents[0][2]) < 0.9 else (1, 0, 0)
    side = norm(cross(tangents[0], ref))
    rings = []
    centres = []

    def ring(centre, tangent, radius):
        nonlocal side
        side = norm(sub(side, mul(tangent, dot(side, tangent))))
        up = cross(tangent, side)
        row = []
        for j in range(sides):
            a = 2 * math.pi * j / sides
            offset = add(mul(side, math.cos(a) * radius), mul(up, math.sin(a) * radius))
            if squash != 1.0:
                axis = norm(squash_axis)
                offset = sub(offset, mul(axis, dot(offset, axis) * (1 - squash)))
            row.append(add(centre, offset))
        rings.append(row)
        centres.append(centre)

    if start == "round":
        ring(sub(pts[0], mul(tangents[0], rad[0] * 0.45)), tangents[0], rad[0] * 0.78)
    for p, t, r in zip(pts, tangents, rad):
        ring(p, t, r)
    if end == "round":
        ring(add(pts[-1], mul(tangents[-1], rad[-1] * 0.45)), tangents[-1], rad[-1] * 0.78)
    first = sub(centres[0], mul(tangents[0], rad[0] * 0.3)) if start == "round" else centres[0]
    last = add(centres[-1], mul(tangents[-1], rad[-1] * 0.3)) if end == "round" else centres[-1]
    return stitch(rings, first, last)


def stitch(rings, first, last):
    """🧶 Closed outward mesh of rings (each the same count, counter-clockwise seen against the path) with a pole at either end."""
    n = len(rings[0])
    verts = [first]
    for row in rings:
        verts.extend(row)
    verts.append(last)
    tris = []
    for j in range(n):
        tris.append((0, 1 + (j + 1) % n, 1 + j))
    for i in range(len(rings) - 1):
        a, b = 1 + i * n, 1 + (i + 1) * n
        for j in range(n):
            k = (j + 1) % n
            tris.append((a + j, a + k, b + k))
            tris.append((a + j, b + k, b + j))
    a = 1 + (len(rings) - 1) * n
    tip = len(verts) - 1
    for j in range(n):
        tris.append((a + j, a + (j + 1) % n, tip))
    return orient(verts, tris)


def volume(verts, tris):
    return sum(dot(verts[a], cross(verts[b], verts[c])) for a, b, c in tris) / 6


def orient(verts, tris):
    if volume(verts, tris) < 0:
        tris = [(a, c, b) for a, b, c in tris]
    return list(verts), list(tris)


def tri_wave(x):
    x = x % 1.0
    return 1 - 4 * abs(x - 0.5)


def pad(centre, a, b, top, bottom, yaw=0.0, dip=0.0, lobes=(), teeth=(0, 0.0), sides=32, power=0.5, droop=0.0, bumps=(), rim=None):
    """☁️ A leaf cushion: a domed mass over a flatter underside with a scalloped rim.

    `a`, `b` half extents before `yaw` (degrees about y); `top`, `bottom` rise and depth; `lobes` = (waves, amplitude, phase degrees) of the plan outline;
    `teeth` = (count, depth as a fraction of the radius) of the leaf-sized rim serration; `dip` tilts the outer end (towards `yaw`) down by that many degrees;
    `droop` lowers the rim by that height (a drooping fringe); `bumps` = (waves, amplitude, phase degrees) swell the dome into leaf clumps; a list given as `rim` receives the rim points and their outward directions (where a fringe of leaves sits)."""
    def radial(theta):
        value = 1.0
        for waves, amplitude, phase in lobes:
            value += amplitude * math.cos(waves * theta + math.radians(phase))
        return value

    def tooth(theta):
        return teeth[1] * tri_wave(teeth[0] * theta / (2 * math.pi)) * (0.65 + 0.35 * math.cos(3 * theta + teeth[0])) if teeth[0] else 0.0

    def place(x, y, z):
        p = rotate((x, y, z), (0, 1, 0), -math.radians(yaw))
        tangent = (-math.sin(math.radians(yaw)), 0.0, math.cos(math.radians(yaw)))
        return add(centre, rotate(p, tangent, -math.radians(dip)))

    rows = [(0.36, +1, 0.0), (0.68, +1, 0.0), (0.9, +1, 0.3), (1.0, 0, 1.0), (0.9, -1, 0.3), (0.55, -1, 0.0)]
    rings = []
    for rho, sign, tooth_share in rows:
        row = []
        for j in range(sides):
            theta = 2 * math.pi * j / sides
            r = rho * (radial(theta) * (1 - 0.6 * (1 - rho)) + 0.6 * (1 - rho) + tooth(theta) * tooth_share)
            height = (top if sign > 0 else -bottom) * (max(1 - rho * rho, 0.0) ** power) if sign else 0.0
            if sign > 0:
                for waves, amplitude, phase in bumps:
                    height *= 1 + amplitude * math.cos(waves * theta + math.radians(phase)) * math.sin(math.pi * rho)
            height -= droop * rho ** 3
            row.append((r * a * math.cos(theta), height, r * b * math.sin(theta)))
        rings.append(row)
    crest = sum(p[1] for p in rings[0]) / sides
    crown = crest + (top - crest) * 0.55
    rings = [[place(*p) for p in row] for row in rings]
    if rim is not None:
        rim.extend((p, norm(sub(p, place(0, 0, 0)))) for p in rings[3])
    return stitch(rings, place(0, crown, 0), place(0, -bottom, 0))


def plate(outline, centre, normal, ahead, thickness):
    """🍃 A thin plate from a star-shaped outline of (along, across) pairs in the plane of `normal`, `ahead` giving the along direction."""
    normal = norm(normal)
    ahead = norm(sub(ahead, mul(normal, dot(ahead, normal))))
    across = cross(normal, ahead)
    n = len(outline)
    upper = [add(centre, add(add(mul(ahead, p[0]), mul(across, p[1])), mul(normal, thickness / 2))) for p in outline]
    lower = [sub(p, mul(normal, thickness)) for p in upper]
    cx = sum(p[0] for p in outline) / n
    cy = sum(p[1] for p in outline) / n
    mid = add(centre, add(mul(ahead, cx), mul(across, cy)))
    verts = [add(mid, mul(normal, thickness * 0.9)), sub(mid, mul(normal, thickness * 0.9))] + upper + lower
    tris = []
    for j in range(n):
        k = (j + 1) % n
        tris.append((0, 2 + j, 2 + k))
        tris.append((1, 2 + n + k, 2 + n + j))
        tris.append((2 + j, 2 + n + j, 2 + n + k))
        tris.append((2 + j, 2 + n + k, 2 + k))
    return orient(verts, tris)


def ribbon(stations, sides_profile, sub_steps=3):
    """🌿 A leaf blade, frond leaflet or strap along stations (x, y, z, half width, thickness, roll degrees): a lens section whose flat side follows the roll about the path;
    `sides_profile` = (fold, keel): fold 0 flat .. 1 a V of 90 degrees, keel = extra thickness at the midrib as a fraction of the half width."""
    fold, keel = sides_profile
    path = spline(stations, sub_steps)
    pts = [p[:3] for p in path]
    tangents = [norm(sub(pts[min(i + 1, len(pts) - 1)], pts[max(i - 1, 0)])) for i in range(len(pts))]
    rings = []
    for p, t, s in zip(pts, tangents, path):
        half, thick, roll = max(s[3], 1e-4), max(s[4], 1e-4), s[5] if len(s) > 5 else 0.0
        ref = (0, 1, 0) if abs(t[1]) < 0.95 else (1, 0, 0)
        side = norm(cross(t, ref))
        up = cross(side, t)
        side = rotate(side, t, math.radians(roll))
        up = rotate(up, t, math.radians(roll))
        lift = half * math.tan(math.radians(45 * fold))
        row = []
        for x, y in ((1, lift + 0.0), (0.5, lift * 0.5 + thick * 0.5), (0, thick * 0.5 + 0.0), (-0.5, lift * 0.5 + thick * 0.5), (-1, lift + 0.0), (-0.5, lift * 0.5 - thick * 0.5), (0, -thick * 0.5 - keel * half), (0.5, lift * 0.5 - thick * 0.5)):
            row.append(add(p, add(mul(side, x * half), mul(up, y))))
        rings.append(row)
    first = sub(pts[0], mul(tangents[0], path[0][3] * 0.3))
    last = add(pts[-1], mul(tangents[-1], path[-1][3] * 0.6))
    return stitch(rings, first, last)


def merge(*meshes):
    """🧩 Several closed meshes as one vertex and triangle list (only for tracing an outline, never for a solid)."""
    verts, tris = [], []
    for v, t in meshes:
        base = len(verts)
        verts.extend(v)
        tris.extend((a + base, b + base, c + base) for a, b, c in t)
    return verts, tris


def outline(points, tris, cell, tolerance):
    """✏️ The outer outline of projected triangles: raster of `cell`, walk of the largest boundary loop, Douglas-Peucker at `tolerance`."""
    xs = [p[0] for p in points]
    ys = [p[1] for p in points]
    x0, y0 = min(xs) - 2 * cell, min(ys) - 2 * cell
    width = int((max(xs) - x0) / cell) + 3
    height = int((max(ys) - y0) / cell) + 3
    grid = [bytearray(width) for _ in range(height)]
    for a, b, c in tris:
        tri = (points[a], points[b], points[c])
        lo = min(p[1] for p in tri)
        hi = max(p[1] for p in tri)
        for row in range(int((lo - y0) / cell - 0.5), int((hi - y0) / cell + 0.5) + 1):
            y = y0 + (row + 0.5) * cell
            hits = []
            for i in range(3):
                p, q = tri[i], tri[(i + 1) % 3]
                if (p[1] <= y) != (q[1] <= y):
                    hits.append(p[0] + (y - p[1]) / (q[1] - p[1]) * (q[0] - p[0]))
            if len(hits) < 2 or not 0 <= row < height:
                continue
            i0 = int((min(hits) - x0) / cell)
            i1 = int((max(hits) - x0) / cell)
            grid[row][i0 : i1 + 1] = b"\x01" * (i1 - i0 + 1)
    for p in points:
        grid[int((p[1] - y0) / cell)][int((p[0] - x0) / cell)] = 1
    nxt = {}
    for row in range(height):
        line = grid[row]
        below = grid[row - 1] if row > 0 else None
        for col in range(width):
            if not line[col]:
                continue
            if below is None or not below[col]:
                nxt.setdefault((col, row), []).append((col + 1, row))
            if row + 1 >= height or not grid[row + 1][col]:
                nxt.setdefault((col + 1, row + 1), []).append((col, row + 1))
            if col == 0 or not line[col - 1]:
                nxt.setdefault((col, row + 1), []).append((col, row))
            if col + 1 >= width or not line[col + 1]:
                nxt.setdefault((col + 1, row), []).append((col + 1, row + 1))
    best, best_area = None, 0.0
    while nxt:
        start = next(iter(nxt))
        loop = [start]
        previous = None
        here = start
        while True:
            options = nxt.get(here)
            if not options:
                break
            if len(options) == 1 or previous is None:
                step = options.pop(0)
            else:
                d = (here[0] - previous[0], here[1] - previous[1])
                options.sort(key=lambda q: d[0] * (q[1] - here[1]) - d[1] * (q[0] - here[0]))
                step = options.pop(0)
            if not options:
                del nxt[here]
            previous, here = here, step
            if here == start:
                break
            loop.append(here)
        area = 0.5 * sum(loop[i][0] * loop[(i + 1) % len(loop)][1] - loop[(i + 1) % len(loop)][0] * loop[i][1] for i in range(len(loop)))
        if abs(area) > best_area:
            best, best_area = loop, abs(area)
    if best is None:
        return []
    ring = [(x0 + c * cell, y0 + r * cell) for c, r in best]
    ring = smooth_steps(ring)
    return simplify_ring(ring, tolerance)


def smooth_steps(ring):
    """🪜 Midpoints of the raster steps, so that a staircase becomes the line it stands for."""
    n = len(ring)
    return [((ring[i][0] + ring[(i + 1) % n][0]) / 2, (ring[i][1] + ring[(i + 1) % n][1]) / 2) for i in range(n)]


def simplify_ring(ring, tolerance):
    if len(ring) < 8:
        return ring
    far = max(range(len(ring)), key=lambda i: (ring[i][0] - ring[0][0]) ** 2 + (ring[i][1] - ring[0][1]) ** 2)
    one = simplify_line(ring[: far + 1], tolerance)
    two = simplify_line(ring[far:] + [ring[0]], tolerance)
    return one[:-1] + two[:-1]


def simplify_line(line, tolerance):
    keep = [False] * len(line)
    keep[0] = keep[-1] = True
    stack = [(0, len(line) - 1)]
    while stack:
        i, j = stack.pop()
        ax, ay = line[i]
        bx, by = line[j]
        dx, dy = bx - ax, by - ay
        length = math.hypot(dx, dy) or 1e-12
        worst, index = 0.0, -1
        for k in range(i + 1, j):
            d = abs((line[k][0] - ax) * dy - (line[k][1] - ay) * dx) / length
            if d > worst:
                worst, index = d, k
        if worst > tolerance:
            keep[index] = True
            stack.append((i, index))
            stack.append((index, j))
    return [p for p, k in zip(line, keep) if k]


def path_d(ring, closed=True):
    return "M " + " L ".join("%s %s" % (round(x, 4), round(y, 4)) for x, y in ring) + (" Z" if closed else "")


class Plant:
    """🌳 One asset under the chisel: catalog document, rest frames, the lists of the scale in work and the drawings traced from them."""

    def __init__(self, category, folder, slug, pose=None):
        self.category, self.slug = category, slug
        self.asset_path = os.path.join(CATALOG, folder, slug, "🔣️.json")
        self.source_path = os.path.join(T, "volumes", category, slug + ".json")
        self.document = json.load(open(self.asset_path, encoding="utf-8"))
        self.asset = self.document["asset"]
        self.rest = json.load(open(os.path.join(T, "🗑️generated", "rework-13", slug + ".frames.json"), encoding="utf-8"))
        self.frames = self.rest if pose is None else json.load(open(os.path.join(T, "🗑️generated", "rework-13", "%s.frames.%s.json" % (slug, pose)), encoding="utf-8"))
        self.bones = {b["id"]: b for b in self.asset["rig"]["bones"]}
        self.volumes = {}
        self.geometry = {}
        self.scale = None

    def local(self, bone, p):
        f = self.frames[bone]
        d = sub(p, f["o"])
        return [round(dot(d, f["u"]), 5), round(dot(d, f["v"]), 5), round(dot(d, f["w"]), 5)]

    def world(self, bone, q, rest=False):
        f = (self.rest if rest else self.frames)[bone]
        return add(f["o"], add(add(mul(f["u"], q[0]), mul(f["v"], q[1])), mul(f["w"], q[2])))

    def at_rest(self, bone, p):
        """🛌 Where a world point stated in the authoring pose lies when its bone is at rest (the frame the drawings are made in)."""
        return p if self.frames is self.rest else self.world(bone, self.local(bone, p), rest=True)

    def begin(self, scale):
        self.scale = scale
        self.body, self.thermal, self.structure, self.drawn, self.strokes = [], [], [], [], []
        self.box = [[1e9] * 3, [-1e9] * 3]
        self.joint_at, self.mass_at = {}, {}

    def solid(self, ident, bone, mesh, material, role="body", tone=0.5, region=None, member=None, draw=None, texture=False, slender=False, repeat=None, tags=None, layer=None):
        """🧱 One body solid from a world mesh, with its thermal reference (`region`), its structural reference (`member` = part name) and its drawing style (`draw` = dict of role, tone, optional hatch, pen, group)."""
        verts, tris = mesh
        entry = {"id": ident, "kind": "polyhedron", "bone": bone, "material": material, "role": role, "tone": tone}
        if texture:
            entry["texture"] = True
        if slender:
            entry["slender"] = True
        entry["vertices"] = [self.local(bone, p) for p in verts]
        entry["triangles"] = [list(t) for t in tris]
        if repeat:
            entry["repeat"] = repeat
        self.body.append(entry)
        if not texture:
            for p in verts:
                for i in range(3):
                    self.box[0][i] = min(self.box[0][i], p[i])
                    self.box[1][i] = max(self.box[1][i], p[i])
        if region:
            ref = {"id": "th-" + ident, "of": ident, "region": region}
            if layer:
                ref["layer"] = layer
            self.thermal.append(ref)
        if member:
            self.structure.append({"id": "st-" + ident, "of": ident, "member": bone, "part": member})
        if draw:
            self.drawn.append((ident, bone, mesh, material, region, draw))
        return entry

    def stroke(self, ident, bone, points, role="texture", tone=0.2, pen=None, material=None, region=None, after=None):
        """🖋️ An open drawn line through world points (a twig, a vein, a culm node); `after` = id of the solid it is drawn just above."""
        self.strokes.append((ident, bone, points, role, tone, pen, material, region, after))

    def extra_thermal(self, entry):
        self.thermal.append(entry)

    def extra_structure(self, entry):
        self.structure.append(entry)

    def lens(self, ident, bone, centre, radius, rise, material, region, tag):
        """💧 An emitter patch: a shallow lens on a surface point, its own thermal solid."""
        rise = max(rise, FEATURE[self.scale])
        verts, tris = pad(centre, radius, radius, rise, rise * 0.4, sides=12)
        self.thermal.append({"id": ident, "kind": "polyhedron", "bone": bone, "material": material, "role": "detail", "tone": 0.6, "region": region, "tags": [tag], "vertices": [self.local(bone, p) for p in verts], "triangles": [list(t) for t in tris]})

    def ball(self, ident, bone, centre, radii, material, part, member=None, anchor=None, tone=0.3):
        entry = {"id": ident, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": tone, "member": member or bone, "part": part, "radii": [round(r, 5) for r in radii], "transform": {"translation": self.local(bone, centre)}}
        if anchor:
            entry["anchor"] = anchor
        self.structure.append(entry)

    def skeleton_marks(self, wood, mass_material, joint_radius=None, skip_mass=()):
        """🦴 Joint balls at the joint of every bone and a mass ball at every bone's centre of mass (radius of a wood sphere of that mass, capped)."""
        for bone in self.asset["rig"]["bones"]:
            f = self.frames[bone["id"]]
            section = bone.get("section")
            r = (joint_radius or (lambda b, s: max((s or {}).get("extremeFibreM", 0.0) * 0.8, 0.6 * FEATURE[self.scale])))(bone, section)
            if bone.get("parent"):
                self.ball("joint-" + bone["id"], bone["id"], self.joint_at.get(bone["id"], tuple(f["o"])), (r, r, r), wood, "joint", tone=0.35)
            mass = bone.get("mass")
            if mass and bone["id"] not in skip_mass:
                centre = self.mass_at.get(bone["id"], self.world(bone["id"], (mass.get("comX", 0), mass.get("comY", 0), 0)))
                rs = max(0.4 * (section or {}).get("extremeFibreM", 0.0), 0.6 * FEATURE[self.scale])
                long = max(min(0.2 * bone.get("lengthM", 0.1), (3 * mass["kg"] / (4 * math.pi * 700 * rs * rs))), rs) if section and bone["id"] not in self.mass_at else rs
                self.ball("mass-" + bone["id"], bone["id"], centre, (long, rs, rs), mass_material, "mass")

    def end(self, decisions):
        self.volumes[str(self.scale)] = {"status": "authored", "denominator": self.scale, "body": self.body, "thermal": self.thermal, "structure": self.structure, "decisions": decisions}
        self.geometry[str(self.scale)] = self.drawings()
        size = [self.box[1][i] - self.box[0][i] for i in range(3)]
        print("1:%d body %d thermal %d structure %d  extent x %.3f y %.3f..%.3f z %.3f  x %.3f..%.3f z %.3f..%.3f" % (self.scale, len(self.body), len(self.thermal), len(self.structure), size[0], self.box[0][1], self.box[1][1], size[2], self.box[0][0], self.box[1][0], self.box[0][2], self.box[1][2]))

    def drawings(self):
        """📐 Elevation (bone frame u, v), front (world z, y) and plan (world x, z) outline of every drawn solid, layered by depth towards the viewer."""
        feature = FEATURE[self.scale]
        representation = self.asset["representations"][str(self.scale)]
        pens = representation["pens"]
        hatches = {h["id"] for h in representation.get("hatches", [])}
        views = {"elevation": [], "front": [], "plan": []}
        depth = {"elevation": lambda c: -c[2], "front": lambda c: c[0], "plan": lambda c: c[1]}
        entries = []
        for ident, bone, mesh, material, region, draw in self.drawn:
            verts, tris = mesh
            verts = [self.at_rest(bone, p) for p in verts]
            centre = [sum(p[i] for p in verts) / len(verts) for i in range(3)]
            entries.append((ident, bone, verts, tris, material, region, draw, centre))
        order_of = {}
        for view in views:
            ranked = sorted(entries, key=lambda e: (e[6].get("layer", 0), depth[view](e[7]) + e[6].get("lift", {}).get(view, 0.0)))
            for rank, (ident, bone, verts, tris, material, region, draw, centre) in enumerate(ranked):
                f = self.rest[bone]
                if view == "elevation":
                    pts = [(dot(sub(p, f["o"]), f["u"]), dot(sub(p, f["o"]), f["v"])) for p in verts]
                elif view == "front":
                    pts = [(p[2], p[1]) for p in verts]
                else:
                    pts = [(p[0], p[2]) for p in verts]
                span = max(max(p[0] for p in pts) - min(p[0] for p in pts), max(p[1] for p in pts) - min(p[1] for p in pts))
                cell = max(feature / 4, span / 500)
                ring = outline(pts, tris, cell, draw.get("tolerance", feature / 5))
                z = round(2 + rank * 0.004, 4)
                order_of[(view, ident)] = z
                if len(ring) < 3:
                    continue
                shape = {"id": ident, "bone": bone, "role": draw.get("role", "body"), "z": z, "filled": True, "tone": draw.get("tone", 0.5)}
                if region:
                    shape["region"] = region
                shape["material"] = material
                if draw.get("hatch") in hatches:
                    shape["hatch"] = draw["hatch"]
                if draw.get("pen"):
                    shape["penMm"] = draw["pen"]
                shape["kind"] = "path"
                shape["d"] = path_d(ring)
                views[view].append(shape)
            for ident, bone, points, role, tone, pen, material, region, after in self.strokes:
                if pens.get(role) is None:
                    continue
                f = self.rest[bone]
                points = [self.at_rest(bone, p) for p in points]
                if view == "elevation":
                    pts = [(dot(sub(p, f["o"]), f["u"]), dot(sub(p, f["o"]), f["v"])) for p in points]
                elif view == "front":
                    pts = [(p[2], p[1]) for p in points]
                else:
                    pts = [(p[0], p[2]) for p in points]
                if math.hypot(pts[-1][0] - pts[0][0], pts[-1][1] - pts[0][1]) < 2 * feature and len(pts) < 4:
                    continue
                shape = {"id": ident, "bone": bone, "role": role, "z": round(order_of.get((view, after), 1.9) + 0.002, 4), "filled": False, "tone": tone}
                if region:
                    shape["region"] = region
                if material:
                    shape["material"] = material
                if pen:
                    shape["penMm"] = pen
                shape["kind"] = "path"
                shape["d"] = path_d(pts, closed=False)
                views[view].append(shape)
            views[view].sort(key=lambda s: s["z"])
        return views

    def save_source(self):
        with open(self.source_path, "w", encoding="utf-8") as handle:
            json.dump({"volumes": self.volumes}, handle, ensure_ascii=False, separators=(",", ":"))
            handle.write("\n")
        print("source", os.path.getsize(self.source_path), "bytes")

    def candidate(self, edit=None):
        """📝 The catalog document with the new drawings (and `edit(asset)` root changes) as `🗑️generated/r3-<slug>/🔣️.json` for `r3-validate --file` and `r3-2d --file`."""
        document = json.load(open(self.asset_path, encoding="utf-8"))
        asset = document["asset"]
        for scale, views in self.geometry.items():
            asset["representations"][scale]["geometry"] = views
        asset["volumes"] = self.volumes
        if edit:
            edit(asset)
        folder = os.path.join(T, "🗑️generated", "r3-" + self.slug)
        os.makedirs(folder, exist_ok=True)
        path = os.path.join(folder, "🔣️.json")
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(document, handle, ensure_ascii=False)
        print("candidate", path)
        return document

    def publish_root(self, members, edit=None):
        """📌 Writes the named root members of the asset (after the same edits as the candidate) into the catalog file, every other byte kept; the file is re-read right before the write."""
        text = open(self.asset_path, encoding="utf-8").read()
        document = json.loads(text)
        asset = document["asset"]
        for scale, views in self.geometry.items():
            asset["representations"][scale]["geometry"] = views
        if edit:
            edit(asset)
        indent = re.search(r'\n( +)"id"\s*:', text).group(1)
        depth = len(indent) // 2
        result = text
        for key in members:
            start, end = member_range(result, indent, key)
            result = result[:start] + indent + json.dumps(key) + ": " + dump(asset[key], depth) + result[end:]
        check = json.loads(result)
        for key in check["asset"]:
            if key not in members and check["asset"][key] != json.loads(text)["asset"][key]:
                raise SystemExit("root write changed member " + key)
        if open(self.asset_path, encoding="utf-8").read() != text:
            raise SystemExit("asset changed while writing; retry")
        open(self.asset_path, "w", encoding="utf-8").write(result)
        print("catalog root written:", ", ".join(members))
