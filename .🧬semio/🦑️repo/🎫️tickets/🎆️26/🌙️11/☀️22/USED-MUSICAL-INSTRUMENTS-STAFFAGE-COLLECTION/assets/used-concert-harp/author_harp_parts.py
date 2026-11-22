"""🧩️ Rig at rest and the parts of every authored scale of the used concert harp: each part is one 3D solid with its exact
side, front and plan projection, so the drawings and the volumes are cut from the same numbers.

@see author_harp_geometry.py
"""

import math

from author_harp_geometry import (
    BASE_X0, BASE_X1, BASE_Y0, BASE_Y1, BASE_Z, BOARD_T, CASTOR_X, CASTOR_Z, COL_TOP, COL_X, COL_Y0, COLUMN_FOOT, COVER_HEM,
    COVER_MARGIN, COVER_PAD, COVER_TURN_Z, CROWN, DECK_T, DECK_Y0, CARPET_T, FEET_X, FEET_Z, FOOT_R, NECK_HALF, PEDAL_LEN,
    PEDAL_NOTCH_DEG, PEDAL_PIVOT, PEDALS, PLATE_T, RIM, SB_ANGLE, SB_EXP, SB_LEN, SB_ORIGIN, SHAFT_R0, SHAFT_R1, SHAFT_Y0,
    SHAFT_Y1, STRIP_T, STRIP_W, T_DECK, TROLLEY_HW, TROLLEY_LEN, TROLLEY_X0, WHEEL_R, WHEEL_W, dirv, neck_profile,
    plate_profile, r6, sb_dp, sb_hw, sb_point, strings,
)

PEDAL_REST = 180.0 - PEDAL_NOTCH_DEG


def bone(id_, parent, x, y, angle, length, joint, z=None):
    """🦴️ One rig bone without its texts."""
    b = {"id": id_, "parent": parent, "x": r6(x), "y": r6(y), "angleDeg": r6(angle), "lengthM": r6(length), "joint": joint}
    if z is not None:
        b["z"] = r6(z)
    return b


def rig_core():
    """🌳️ The bones at rest (the stored pose) with their placements in the parent frame."""
    bass = strings()[0]
    out = [
        bone("floor", None, 0, 0, 0, 0, {"type": "fixed"}),
        bone("trolley", "floor", TROLLEY_X0, DECK_Y0, 0, TROLLEY_LEN, {"type": "prismatic", "axisDeg": 0, "minM": -0.1, "maxM": 1.4}),
        bone("castors-rear", "trolley", CASTOR_X[0] - TROLLEY_X0, WHEEL_R - DECK_Y0, 0, WHEEL_R, {"type": "revolute", "minDeg": -7200, "maxDeg": 7200}),
        bone("castors-front", "trolley", CASTOR_X[1] - TROLLEY_X0, WHEEL_R - DECK_Y0, 0, WHEEL_R, {"type": "revolute", "minDeg": -7200, "maxDeg": 7200}),
        bone("harp-lift", "floor", 0, T_DECK, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -T_DECK, "maxM": 0}),
        bone("harp", "harp-lift", 0, 0, 0, FEET_X[1], {"type": "revolute", "minDeg": 0, "maxDeg": 25}),
        bone("column", "harp", COL_X, COL_Y0, 90, COL_TOP - COL_Y0, {"type": "fixed"}),
        bone("neck", "column", 1.73 - COL_Y0, 0, 100.73, 0.967, {"type": "fixed"}),
        bone("soundbox", "harp", SB_ORIGIN[0], SB_ORIGIN[1], SB_ANGLE, SB_LEN, {"type": "fixed"}),
        bone("strings", "harp", bass[2][0], bass[2][1], 90, bass[3] - bass[2][1], {"type": "fixed"}),
    ]
    for name, z in PEDALS:
        out.append(bone(f"pedal-{name}", "harp", PEDAL_PIVOT[0], PEDAL_PIVOT[1], PEDAL_REST, PEDAL_LEN, {"type": "revolute", "minDeg": 0, "maxDeg": r6(2 * PEDAL_NOTCH_DEG)}, z))
    return out


def frames(bones):
    """🧮️ World rest frame of every bone: origin, angle, lateral offset."""
    out = {}
    for b in bones:
        if b["parent"] is None:
            out[b["id"]] = ((b["x"], b["y"]), b["angleDeg"], b.get("z", 0.0))
            continue
        (px, py), pa, pz = out[b["parent"]]
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        out[b["id"]] = ((px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"]), pa + b["angleDeg"], pz + b.get("z", 0.0))
    return out


class Rig:
    """🗺️ Converts between world rest coordinates and the local frame of each bone."""

    def __init__(self, bones):
        self.bones = bones
        self.f = frames(bones)

    def local(self, bid, p):
        """📍️ A world rest point (x, y) or (x, y, z) in the frame of bone `bid`."""
        (ox, oy), a, z0 = self.f[bid]
        u, v = dirv(a), dirv(a + 90)
        dx, dy = p[0] - ox, p[1] - oy
        out = (dx * u[0] + dy * u[1], dx * v[0] + dy * v[1])
        return out if len(p) == 2 else out + (p[2] - z0,)

    def world(self, bid, q):
        """🌍️ A local point (u, v) of bone `bid` in world rest coordinates."""
        (ox, oy), a, _ = self.f[bid]
        u, v = dirv(a), dirv(a + 90)
        return (ox + q[0] * u[0] + q[1] * v[0], oy + q[0] * u[1] + q[1] * v[1])

    def axis(self, bid, d):
        """➡️ A world direction (x, y) in the frame of bone `bid`."""
        _, a, _ = self.f[bid]
        u, v = dirv(a), dirv(a + 90)
        return (d[0] * u[0] + d[1] * u[1], d[0] * v[0] + d[1] * v[1])


def hull(points):
    """🪢️ Counter-clockwise convex hull of 2D points (monotone chain)."""
    pts = sorted(set((r6(x), r6(y)) for x, y in points))
    if len(pts) < 3:
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


def area2(ring):
    """📐️ Signed area of a 2D ring."""
    return 0.5 * sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(ring, ring[1:] + ring[:1]))


def ccw(ring):
    """🔄️ The ring counter-clockwise."""
    return ring if area2(ring) > 0 else list(reversed(ring))


def rect(x0, y0, x1, y1):
    """▭️ A counter-clockwise rectangle ring."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def circle(cx, cy, r, n=24):
    """⚪️ A polygon circle."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def rnd(ring):
    """🎯️ A ring as rounded lists."""
    return [[r6(a), r6(b)] for a, b in ring]


def polyhedron(rings):
    """🪨️ A closed outward triangular shell lofted through rings of equal size (bone-local 3D points), capped by fans from the ring centroids."""
    verts = [list(p) for ring in rings for p in ring]
    n = len(rings[0])
    tris = []
    for k in range(len(rings) - 1):
        for i in range(n):
            a, b = k * n + i, k * n + (i + 1) % n
            c, d = (k + 1) * n + (i + 1) % n, (k + 1) * n + i
            tris += [[a, b, c], [a, c, d]]
    for k, flip in ((0, True), (len(rings) - 1, False)):
        ring = rings[k]
        verts.append([sum(p[j] for p in ring) / n for j in range(3)])
        c = len(verts) - 1
        for i in range(n):
            a, b = k * n + i, k * n + (i + 1) % n
            tris.append([c, b, a] if flip else [c, a, b])
    vol = 0.0
    for a, b, c in tris:
        p, q, r = verts[a], verts[b], verts[c]
        vol += (p[0] * (q[1] * r[2] - q[2] * r[1]) - p[1] * (q[0] * r[2] - q[2] * r[0]) + p[2] * (q[0] * r[1] - q[1] * r[0])) / 6
    if vol < 0:
        tris = [[a, c, b] for a, b, c in tris]
    return {"kind": "polyhedron", "vertices": [[r6(x) for x in v] for v in verts], "triangles": tris}


class Parts:
    """🧰️ Builds the parts of one scale: a 3D solid plus its plain side, front and plan shapes."""

    def __init__(self, rig, scale):
        self.rig = rig
        self.scale = scale
        self.items = []

    def add(self, pid, bid, material, tone, region, solid, elev, front, plan, role="body", extra=None, draw=True, regions=None):
        """➕️ Registers one part."""
        item = {"id": pid, "bone": bid, "material": material, "tone": tone, "region": region, "regions": regions, "role": role, "solid": solid, "elev": elev, "front": front, "plan": plan, "extra": extra or {}, "draw": draw}
        self.items.append(item)
        return item

    def prism(self, pid, bid, material, tone, region, profile_world, z0, z1, **kw):
        """🧱️ A prism of a side profile (world rest x, y) swept laterally from z0 to z1 (world rest z)."""
        _, _, bz = self.rig.f[bid]
        local = ccw([self.rig.local(bid, p) for p in profile_world])
        xs = [p[0] for p in profile_world]
        ys = [p[1] for p in profile_world]
        solid = {"kind": "extrude", "profile": rnd(local), "plane": "xy", "from": r6(z0 - bz), "to": r6(z1 - bz)}
        return self.add(pid, bid, material, tone, region, solid, local, rect(z0, min(ys), z1, max(ys)), rect(min(xs), z0, max(xs), z1), **kw)

    def plate_v(self, pid, bid, material, tone, region, outline_xz, y0, y1, **kw):
        """🟫️ A horizontal plate on a bone that runs along x at rest: plan outline (world x, z) extruded from height y0 to y1."""
        (ox, oy), a, bz = self.rig.f[bid]
        assert abs(a) < 1e-9
        prof = ccw([(z - bz, x - ox) for x, z in outline_xz])
        solid = {"kind": "extrude", "profile": rnd(prof), "plane": "zx", "from": r6(y0 - oy), "to": r6(y1 - oy)}
        xs = [p[0] for p in outline_xz]
        zs = [p[1] for p in outline_xz]
        elev = [self.rig.local(bid, p) for p in rect(min(xs), y0, max(xs), y1)]
        return self.add(pid, bid, material, tone, region, solid, elev, rect(min(zs), y0, max(zs), y1), ccw(list(outline_xz)), **kw)

    def turned(self, pid, bid, material, tone, region, profile_yr, cx, cz, **kw):
        """🏺️ A turned part about the world vertical through (cx, cz): profile of (height, radius) in world rest coordinates."""
        _, _, bz = self.rig.f[bid]
        ax = self.rig.axis(bid, (0.0, 1.0))
        y0 = profile_yr[0][0]
        origin = self.rig.local(bid, (cx, y0)) + (cz - bz,)
        solid = {"kind": "revolve", "profile": [[r6(y - y0), r6(r)] for y, r in profile_yr], "axis": [r6(ax[0]), r6(ax[1]), 0.0], "origin": [r6(x) for x in origin]}
        outline = [(cx + r, y) for y, r in profile_yr] + [(cx - r, y) for y, r in reversed(profile_yr)]
        front = [(cz + r, y) for y, r in profile_yr] + [(cz - r, y) for y, r in reversed(profile_yr)]
        rmax = max(r for _, r in profile_yr)
        return self.add(pid, bid, material, tone, region, solid, ccw([self.rig.local(bid, p) for p in outline]), ccw(front), circle(cx, cz, rmax), **kw)

    def shell(self, pid, bid, material, tone, region, rings_world, **kw):
        """🪨️ A convex polyhedron lofted through rings of world rest 3D points."""
        rings = [[self.rig.local(bid, p) for p in ring] for ring in rings_world]
        pts = [p for ring in rings_world for p in ring]
        elev = hull([(q[0], q[1]) for ring in rings for q in ring])
        return self.add(pid, bid, material, tone, region, polyhedron(rings), elev, hull([(p[2], p[1]) for p in pts]), hull([(p[0], p[2]) for p in pts]), **kw)

    def wire(self, pid, bid, material, tone, region, p0, p1, d, **kw):
        """🧵️ A slender capsule between two world rest points (x, y, z)."""
        a, b = self.rig.local(bid, p0), self.rig.local(bid, p1)
        solid = {"kind": "capsule", "p0": [r6(x) for x in a], "p1": [r6(x) for x in b], "r0": r6(d / 2), "r1": r6(d / 2), "slender": True}
        return self.add(pid, bid, material, tone, region, solid, [a[:2], b[:2]], [(p0[2], p0[1]), (p1[2], p1[1])], [(p0[0], p0[2]), (p1[0], p1[2])], role="detail", **kw)


def sb_ring(u, part, s, n):
    """⭕️ One cross-section of the soundbox at u as world rest 3D points: `slab` from the soundboard top to v = s, `back` from v = s to the back."""
    hw, dp = sb_hw(u), sb_dp(u)
    t_s = math.asin(min(1.0, (max(s, 0.0) / dp) ** (SB_EXP / 2))) if s > 0 else 0.0

    def curve(t):
        c, si = math.cos(t), math.sin(t)
        return (dp * abs(si) ** (2 / SB_EXP), hw * math.copysign(abs(c) ** (2 / SB_EXP), c))

    if part == "back":
        pts = [curve(t_s + (math.pi - 2 * t_s) * k / (n - 1)) for k in range(n)]
    else:
        side = [curve(t_s * k / max(1, n // 4)) for k in range(n // 4 + 1)] if s > 0 else [(0.0, hw)]
        pts = [(-BOARD_T, hw)] + side + [(v, -w) for v, w in reversed(side)] + [(-BOARD_T, -hw)]
    out = []
    for v, w in pts:
        x, y = sb_point(u, v)
        out.append((x, y + T_DECK, w))
    return out


def foot_rings(n):
    """🦶️ Top and bottom ring of the soundbox foot block: the bottom section of the soundbox and its vertical shadow on the base top."""
    top = sb_ring(0.0, "back", 0.0, n)
    x0, y0 = sb_point(0.0, -BOARD_T)
    full = [(x0, y0 + T_DECK, sb_hw(0.0))] + top + [(x0, y0 + T_DECK, -sb_hw(0.0))]
    bottom = [(x, BASE_Y1 + T_DECK - 0.002, z) for x, y, z in full]
    return [full, bottom]


def cover_plane():
    """📐️ Lateral half width of the cover as a plane hw = a + b x + c y over the harp frame, fitted to keep the margin over the soundbox, the crown and the soundbox top."""
    pts = [(sb_point(0.0, 0.0), sb_hw(0.0) + COVER_MARGIN + 0.003), (sb_point(SB_LEN, 0.0), sb_hw(SB_LEN) + COVER_MARGIN + 0.003), ((COL_X, 1.74), 0.075 + COVER_MARGIN + 0.003)]
    (x1, y1), h1 = pts[0]
    (x2, y2), h2 = pts[1]
    (x3, y3), h3 = pts[2]
    det = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
    b = ((h2 - h1) * (y3 - y1) - (h3 - h1) * (y2 - y1)) / det
    c = ((x2 - x1) * (h3 - h1) - (x3 - x1) * (h2 - h1)) / det
    a = h1 - b * x1 - c * y1
    return a, b, c


def clip_above(ring, y0):
    """✂️ The part of a ring above the height y0 (Sutherland-Hodgman)."""
    out = []
    for p, q in zip(ring, ring[1:] + ring[:1]):
        pin, qin = p[1] >= y0, q[1] >= y0
        if pin:
            out.append(p)
        if pin != qin:
            t = (y0 - p[1]) / (q[1] - p[1])
            out.append((p[0] + t * (q[0] - p[0]), y0))
    return out


def simplify(ring, n):
    """🪶️ Keeps the n vertices of a convex ring that carry the largest turning area (Visvalingam)."""
    pts = []
    for p in ring:
        if not pts or math.hypot(p[0] - pts[-1][0], p[1] - pts[-1][1]) > 0.004:
            pts.append(p)
    if math.hypot(pts[0][0] - pts[-1][0], pts[0][1] - pts[-1][1]) <= 0.004:
        pts.pop()
    keep = {min(pts), max(pts), max(pts, key=lambda p: p[1])}
    while len(pts) > n:
        best, k = None, 0
        for i in range(len(pts)):
            a, b, c = pts[i - 1], pts[i], pts[(i + 1) % len(pts)]
            ar = abs((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1]))
            if b[1] <= COVER_HEM + 1e-9 or b in keep:
                ar = 1e9
            if best is None or ar < best:
                best, k = ar, i
        pts.pop(k)
    return pts


def cover_outline(n):
    """🧥️ Side outline of the padded cover in the harp frame: the convex hull of the harp above the hem, grown by the padding and cut at the hem."""
    pts = []
    for y, r in CROWN:
        pts += [(COL_X + r, y), (COL_X - r, y)]
    pts += [(COL_X + SHAFT_R0, SHAFT_Y0), (COL_X + SHAFT_R0, COVER_HEM)]
    pts += neck_profile()
    for k in range(11):
        u = SB_LEN * k / 10
        pts += [sb_point(u, sb_dp(u)), sb_point(u, -BOARD_T)]
    grown = []
    for x, y in hull(pts):
        grown += [(x + COVER_PAD * math.cos(2 * math.pi * j / 16), y + COVER_PAD * math.sin(2 * math.pi * j / 16)) for j in range(16)]
    return simplify(clip_above(hull(grown), COVER_HEM), n)
