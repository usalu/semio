"""🧰 Round 3 solid writers of humans/child-on-kick-scooter: small functions that turn hand-stated sections, profiles and paths into kernel solids (loft, revolve, extrude, box, ellipsoid, capsule) in bone-local metres (u along the bone, v its sagittal normal, w lateral, left +)."""
import math


def r5(x):
    return round(x, 5) + 0.0


def vec(a):
    return [r5(a[0]), r5(a[1]), r5(a[2])]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(sum(q * q for q in a))
    return [q / n for q in a]


def quat(X, Y):
    """🧭 The unit quaternion [x, y, z, w] of the rotation that carries local x to X and local y to Y (both unit, orthogonal)."""
    Z = cross(X, Y)
    m = [[X[0], Y[0], Z[0]], [X[1], Y[1], Z[1]], [X[2], Y[2], Z[2]]]
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
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
    n = math.sqrt(sum(c * c for c in q))
    return [round(c / n, 7) + 0.0 for c in q]


def rot_w(deg):
    h = math.radians(deg) / 2
    return [0, 0, round(math.sin(h), 7), round(math.cos(h), 7)]


W = [0.0, 0.0, 1.0]
V = [0.0, 1.0, 0.0]
U = [1.0, 0.0, 0.0]
ACROSS = quat(W, V)


def base(i, kind, bone, material, tone, role="body", **extra):
    s = {"id": i, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update({k: v for k, v in extra.items() if v is not None})
    return s


def section(at, rx, ry, exponent=2.0, rotation=None):
    s = {"at": vec(at), "rx": r5(rx), "ry": r5(ry)}
    if abs(exponent - 2.0) > 1e-9:
        s["exponent"] = exponent
    if rotation is not None:
        s["rotation"] = rotation
    return s


def loft(i, bone, sections, material, tone, role="body", caps=(True, True), **extra):
    return base(i, "loft", bone, material, tone, role, sections=sections, capStart=caps[0], capEnd=caps[1], **extra)


def stack(i, bone, rows, material, tone, side=0.0, role="body", caps=(True, True), **extra):
    """🥞 A trunk-like loft of sections across the bone: rows (u, front, back, half width[, exponent[, lateral centre]]) with front the reach towards -v and back the reach towards +v."""
    secs = []
    for r in rows:
        u, front, back, hw = r[:4]
        e = r[4] if len(r) > 4 else 2.0
        wc = r[5] if len(r) > 5 else side
        secs.append(section([u, (back - front) / 2, wc], hw, (front + back) / 2, e, ACROSS))
    return loft(i, bone, secs, material, tone, role, caps, **extra)


def limb(i, bone, rows, material, tone, side, role="body", caps=(True, True), **extra):
    """🦵 A limb loft of sections across the bone: rows (u, v centre, w centre outward, v radius, w radius[, exponent]); side +1 left, -1 right mirrors the outward offsets."""
    secs = [section([r[0], r[1], r[2] * side], r[4], r[3], r[5] if len(r) > 5 else 2.0, ACROSS) for r in rows]
    return loft(i, bone, secs, material, tone, role, caps, **extra)


def tube(i, bone, points, material, tone, hint=W, role="body", caps=(True, True), **extra):
    """🧵 A loft along a free path: points (position, radius along the hint side, radius across[, exponent]); every section is perpendicular to the path, its x axis the hint projected into the section plane."""
    secs = []
    n = len(points)
    for k, p in enumerate(points):
        a = points[max(0, k - 1)][0]
        b = points[min(n - 1, k + 1)][0]
        t = unit([b[q] - a[q] for q in range(3)])
        d = sum(hint[q] * t[q] for q in range(3))
        X = unit([hint[q] - d * t[q] for q in range(3)])
        Y = cross(t, X)
        secs.append(section(p[0], p[1], p[2], p[3] if len(p) > 3 else 2.0, quat(X, Y)))
    return loft(i, bone, secs, material, tone, role, caps, **extra)


def revolve(i, bone, profile, origin, axis, material, tone, role="body", **extra):
    return base(i, "revolve", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], axis=vec(axis), origin=vec(origin), **extra)


def extrude(i, bone, profile, plane, lo, hi, material, tone, role="body", **extra):
    return base(i, "extrude", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], plane=plane, **{"from": r5(lo), "to": r5(hi)}, **extra)


def box(i, bone, size, at, material, tone, role="body", corner=None, rotation=None, **extra):
    tr = {"translation": vec(at)}
    if rotation is not None:
        tr["rotation"] = rotation
    return base(i, "box", bone, material, tone, role, size=vec(size), cornerRadius=corner, transform=tr, **extra)


def ellipsoid(i, bone, radii, at, material, tone, role="body", rotation=None, **extra):
    tr = {"translation": vec(at)}
    if rotation is not None:
        tr["rotation"] = rotation
    return base(i, "ellipsoid", bone, material, tone, role, radii=vec(radii), transform=tr, **extra)


def capsule(i, bone, p0, p1, r0, r1, material, tone, role="body", **extra):
    return base(i, "capsule", bone, material, tone, role, p0=vec(p0), p1=vec(p1), r0=r5(r0), r1=r5(r1), **extra)


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def shrink(solid, d, i=None):
    """🧅 The same solid drawn inwards by d on every side (a nested layer shell): loft sections lose d of both radii and the end sections move in by d, ellipsoids, capsules and boxes lose d."""
    import copy
    s = copy.deepcopy(solid)
    if i is not None:
        s["id"] = i
    k = s["kind"]
    if k == "loft":
        secs = s["sections"]
        for q in secs:
            q["rx"] = r5(max(q["rx"] - d, 0.0006))
            q["ry"] = r5(max(q["ry"] - d, 0.0006))
        for a, b in ((0, 1), (-1, -2)):
            pa, pb = secs[a]["at"], secs[b]["at"]
            L = math.sqrt(sum((pa[q] - pb[q]) ** 2 for q in range(3)))
            f = min(0.8, d / L) if L > 0 else 0
            secs[a]["at"] = vec([pa[q] + (pb[q] - pa[q]) * f for q in range(3)])
    elif k == "ellipsoid":
        s["radii"] = vec([max(q - d, 0.0006) for q in s["radii"]])
    elif k == "capsule":
        s["r0"] = r5(max(s["r0"] - d, 0.0006))
        s["r1"] = r5(max(s["r1"] - d, 0.0006))
    elif k == "box":
        s["size"] = vec([max(q - 2 * d, 0.0012) for q in s["size"]])
        if "cornerRadius" in s:
            s["cornerRadius"] = r5(max(0.0, min(s["cornerRadius"] - d, min(s["size"]) / 2)))
    elif k == "extrude":
        cx = sum(p[0] for p in s["profile"]) / len(s["profile"])
        cy = sum(p[1] for p in s["profile"]) / len(s["profile"])
        out = []
        for p in s["profile"]:
            L = math.hypot(p[0] - cx, p[1] - cy)
            f = max(0.05, (L - d) / L) if L > 0 else 1
            out.append([r5(cx + (p[0] - cx) * f), r5(cy + (p[1] - cy) * f)])
        s["profile"] = out
        s["from"] = r5(s["from"] + d)
        s["to"] = r5(s["to"] - d)
    else:
        raise ValueError(f"cannot shrink {k}")
    return s
