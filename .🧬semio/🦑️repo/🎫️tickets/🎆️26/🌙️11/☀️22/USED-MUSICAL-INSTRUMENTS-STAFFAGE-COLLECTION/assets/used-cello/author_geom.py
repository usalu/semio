"""🧰 Geometry chisel of the used cello: closed polyhedra from station rings, half-shell tubs, areas and volumes, 2D helpers.

Pure Python (no numpy). World frame of the staffage: x ahead, y up, z left. Every function returns plain lists so that the
author script can write them straight into the asset document.

@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""
import math


def r6(x):
    """✂️ Rounds a coordinate to 0.01 mm and normalises -0."""
    v = round(x, 5)
    return 0.0 if v == 0 else v


def pt(p):
    """📍 Rounds a 2D or 3D point."""
    return [r6(c) for c in p]


def sub(a, b):
    """➖ Vector difference."""
    return [a[i] - b[i] for i in range(3)]


def cross(a, b):
    """✖️ Cross product."""
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    """⚫ Dot product."""
    return sum(a[i] * b[i] for i in range(3))


def norm(a):
    """📏 Euclidean length."""
    return math.sqrt(dot(a, a))


def orient(vertices, triangles):
    """🧭 Orients the triangles of one closed manifold consistently and outward (positive signed volume)."""
    edges = {}
    for index, (a, b, c) in enumerate(triangles):
        for p, q in ((a, b), (b, c), (c, a)):
            edges.setdefault(frozenset((p, q)), []).append(index)
    flipped = [None] * len(triangles)
    tris = [list(t) for t in triangles]
    for seed in range(len(tris)):
        if flipped[seed] is not None:
            continue
        flipped[seed] = False
        stack = [seed]
        while stack:
            current = stack.pop()
            a, b, c = tris[current]
            for p, q in ((a, b), (b, c), (c, a)):
                for other in edges[frozenset((p, q))]:
                    if other == current or flipped[other] is not None:
                        continue
                    o = tris[other]
                    same = any(o[i] == p and o[(i + 1) % 3] == q for i in range(3))
                    if same:
                        tris[other] = [o[0], o[2], o[1]]
                    flipped[other] = False
                    stack.append(other)
    if signed_volume(vertices, tris) < 0:
        tris = [[t[0], t[2], t[1]] for t in tris]
    return tris


def signed_volume(vertices, triangles):
    """🧊 Signed volume of a closed triangle mesh."""
    total = 0.0
    for a, b, c in triangles:
        total += dot(vertices[a], cross(vertices[b], vertices[c])) / 6.0
    return total


def mesh_area(vertices, triangles):
    """📐 Surface area of a triangle mesh."""
    return sum(norm(cross(sub(vertices[b], vertices[a]), sub(vertices[c], vertices[a]))) / 2 for a, b, c in triangles)


def check_manifold(vertices, triangles):
    """🩺 Raises when a mesh is open or non-manifold (every edge exactly twice)."""
    count = {}
    for a, b, c in triangles:
        for p, q in ((a, b), (b, c), (c, a)):
            key = (min(p, q), max(p, q))
            count[key] = count.get(key, 0) + 1
    bad = [k for k, v in count.items() if v != 2]
    if bad:
        raise ValueError(f"mesh not closed: {len(bad)} bad edges, e.g. {bad[:3]}")
    used = set(i for t in triangles for i in t)
    if len(used) != len(vertices):
        raise ValueError("unused vertices")


def ring_loft(rings, caps="fan"):
    """🍥 A closed polyhedron through station rings of equal point count (each ring a closed loop of 3D points), capped at both ends.

    `caps` is `fan` (convex end rings, fanned from their centroid) or `strip` (each end ring is two rails of equal length, outer then inner reversed, joined as a strip)."""
    n = len(rings[0])
    vertices = [p for ring in rings for p in ring]
    triangles = []
    for s in range(len(rings) - 1):
        for i in range(n):
            a, b = s * n + i, s * n + (i + 1) % n
            c, d = (s + 1) * n + (i + 1) % n, (s + 1) * n + i
            triangles += [[a, b, c], [a, c, d]]
    for s in (0, len(rings) - 1):
        base = s * n
        if caps == "fan":
            centre = [sum(p[k] for p in rings[s]) / n for k in range(3)]
            vertices.append(centre)
            ci = len(vertices) - 1
            for i in range(n):
                triangles.append([ci, base + i, base + (i + 1) % n])
        else:
            half = n // 2
            for i in range(half - 1):
                o0, o1 = base + i, base + i + 1
                i0, i1 = base + n - 1 - i, base + n - 2 - i
                triangles += [[o0, o1, i1], [o0, i1, i0]]
    triangles = orient(vertices, triangles)
    check_manifold(vertices, triangles)
    return vertices, triangles


def superellipse_half(cx, depth, half_width, n, count, side, band=0.0):
    """🥚 Points of half a superelliptic section in the (x, z) plane, from the left rim (z = +b) round the back (`side` -1) or the front (`side` +1) to the right rim (z = -b); `band` is a straight side wall at the rim before the curve starts."""
    band = min(band, 0.6 * depth)
    if band > 0:
        inner = superellipse_half(cx + side * band, depth - band, half_width, n, count, side)
        return [(cx, half_width)] + inner + [(cx, -half_width)]
    out = []
    for k in range(count + 1):
        t = math.pi * k / count
        c, s = math.cos(t), math.sin(t)
        z = half_width * math.copysign(abs(c) ** (2.0 / n), c)
        x = cx + side * depth * abs(s) ** (2.0 / n)
        out.append((x, z))
    return out


def tub(stations, xp, side, wall, n, count, to_local, band=0.0):
    """🛁 One half of a closed shell (the back shell or the lid of a case) as a single closed polyhedron with a real wall.

    `stations` are (y, half_width, x_extreme) from the bottom face to the top face; `xp` is the parting plane x; `side` -1 for the back half
    (towards -x), +1 for the lid (towards +x); `wall` the wall thickness; `to_local` maps a world point (x, y, z) to bone-local (u, v, w)."""
    y0, y1 = stations[0][0], stations[-1][0]
    outer_rings, inner_rings = [], []
    for y, b, xe in stations:
        depth = abs(xe - xp)
        outer_rings.append([(x, y, z) for x, z in superellipse_half(xp, depth, b, n, count, side, band)])
        yi = min(max(y, y0 + wall), y1 - wall)
        inner_rings.append([(x, yi, z) for x, z in superellipse_half(xp, depth - wall, b - wall, n, count, side, band)])
    verts = []
    index = {}

    def vid(p):
        key = (round(p[0], 9), round(p[1], 9), round(p[2], 9))
        if key not in index:
            index[key] = len(verts)
            verts.append(list(p))
        return index[key]

    tris = []
    S, P = len(stations), count + 1 + (2 if band > 0 else 0)
    O = [[vid(p) for p in ring] for ring in outer_rings]
    I = [[vid(p) for p in ring] for ring in inner_rings]
    for s in range(S - 1):
        for i in range(P - 1):
            tris += [[O[s][i], O[s][i + 1], O[s + 1][i + 1]], [O[s][i], O[s + 1][i + 1], O[s + 1][i]]]
            tris += [[I[s][i], I[s + 1][i + 1], I[s][i + 1]], [I[s][i], I[s + 1][i], I[s + 1][i + 1]]]
    mid_ob = vid((xp, y0, 0.0))
    mid_ot = vid((xp, y1, 0.0))
    mid_ib = vid((xp, y0 + wall, 0.0))
    mid_it = vid((xp, y1 - wall, 0.0))
    for i in range(P - 1):
        tris.append([mid_ob, O[0][i], O[0][i + 1]])
        tris.append([mid_ot, O[S - 1][i + 1], O[S - 1][i]])
        tris.append([mid_ib, I[0][i + 1], I[0][i]])
        tris.append([mid_it, I[S - 1][i], I[S - 1][i + 1]])
    outer_loop = [O[s][0] for s in range(S)] + [mid_ot] + [O[s][P - 1] for s in reversed(range(S))] + [mid_ob]
    inner_loop = [I[s][0] for s in range(S)] + [mid_it] + [I[s][P - 1] for s in reversed(range(S))] + [mid_ib]
    m = len(outer_loop)
    for i in range(m):
        a, b = outer_loop[i], outer_loop[(i + 1) % m]
        c, d = inner_loop[(i + 1) % m], inner_loop[i]
        tris += [[a, b, c], [a, c, d]]
    tris = orient(verts, tris)
    check_manifold(verts, tris)
    local = [pt(to_local(p)) for p in verts]
    return local, tris, mesh_area(verts, tris), signed_volume(verts, tris)


def outer_area_tub(stations, xp, side, n, count, band=0.0):
    """📐 The outer skin area of a tub (outer surface and its two end faces, without the parting face), numerically."""
    area = 0.0
    rings = [[(x, y, z) for x, z in superellipse_half(xp, abs(xe - xp), b, n, count, side, band)] for y, b, xe in stations]
    for s in range(len(rings) - 1):
        for i in range(len(rings[0]) - 1):
            a, b, c, d = rings[s][i], rings[s][i + 1], rings[s + 1][i + 1], rings[s + 1][i]
            area += norm(cross(sub(b, a), sub(c, a))) / 2 + norm(cross(sub(c, a), sub(d, a))) / 2
    for s in (0, len(rings) - 1):
        ring = rings[s]
        for i in range(len(ring) - 1):
            a, b = ring[i], ring[i + 1]
            area += abs((a[0] - xp) * b[2] - (b[0] - xp) * a[2]) / 2
    return area


def section_x(xp, xe, b, n, z, band=0.0):
    """📏 The x of a superelliptic half-section (with a straight side band) at lateral position z."""
    r = min(abs(z) / b, 1.0)
    sgn = 1 if xe > xp else -1
    band = min(band, 0.6 * abs(xe - xp))
    return xp + sgn * band + (xe - xp - sgn * band) * (1 - r ** n) ** (1.0 / n)


def interp(table, x, column=1):
    """📈 Linear interpolation in a table of rows sorted by the first column."""
    if x <= table[0][0]:
        return table[0][column]
    for a, b in zip(table, table[1:]):
        if x <= b[0]:
            t = (x - a[0]) / (b[0] - a[0])
            return a[column] + t * (b[column] - a[column])
    return table[-1][column]


def smooth_interp(table, x):
    """〰️ Catmull-Rom interpolation of (x, y) rows, clamped at the ends, for drawn outlines."""
    if x <= table[0][0]:
        return table[0][1]
    if x >= table[-1][0]:
        return table[-1][1]
    for i in range(len(table) - 1):
        if table[i][0] <= x <= table[i + 1][0]:
            p0 = table[max(i - 1, 0)]
            p1, p2 = table[i], table[i + 1]
            p3 = table[min(i + 2, len(table) - 1)]
            t = (x - p1[0]) / (p2[0] - p1[0])
            m1 = (p2[1] - p0[1]) / (p2[0] - p0[0]) * (p2[0] - p1[0])
            m2 = (p3[1] - p1[1]) / (p3[0] - p1[0]) * (p2[0] - p1[0])
            t2, t3 = t * t, t * t * t
            return (2 * t3 - 3 * t2 + 1) * p1[1] + (t3 - 2 * t2 + t) * m1 + (-2 * t3 + 3 * t2) * p2[1] + (t3 - t2) * m2
    return table[-1][1]


def poly_area(points):
    """🔷 Signed area of a 2D polygon."""
    return sum(points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1] for i in range(len(points))) / 2


def ccw(points):
    """🔄 The polygon in counter-clockwise order."""
    return points if poly_area(points) > 0 else list(reversed(points))


def circle(cx, cy, r, count=24):
    """⭕ A polygon approximating a circle."""
    return [(cx + r * math.cos(2 * math.pi * k / count), cy + r * math.sin(2 * math.pi * k / count)) for k in range(count)]


def simplify(points, tolerance):
    """🪶 Douglas-Peucker simplification of a closed ring."""
    if len(points) < 5:
        return points

    def dp(seq):
        if len(seq) < 3:
            return seq
        a, b = seq[0], seq[-1]
        dx, dy = b[0] - a[0], b[1] - a[1]
        length = math.hypot(dx, dy) or 1e-12
        far, at = -1, 0
        for i in range(1, len(seq) - 1):
            d = abs(dx * (seq[i][1] - a[1]) - dy * (seq[i][0] - a[0])) / length
            if d > far:
                far, at = d, i
        if far <= tolerance:
            return [a, b]
        return dp(seq[: at + 1])[:-1] + dp(seq[at:])

    split = max(range(len(points)), key=lambda i: math.hypot(points[i][0] - points[0][0], points[i][1] - points[0][1]))
    first = dp(points[: split + 1])
    second = dp(points[split:] + [points[0]])
    return first[:-1] + second[:-1]
