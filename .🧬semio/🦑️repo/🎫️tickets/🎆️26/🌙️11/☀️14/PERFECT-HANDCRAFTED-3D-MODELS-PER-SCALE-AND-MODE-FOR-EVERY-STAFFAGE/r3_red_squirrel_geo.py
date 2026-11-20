"""📐 Round 3 red squirrel: the small geometry kit of the handcrafted solids. Stations are stated by hand in millimetres in the bone frame (u along the bone, v its normal in the sagittal plane, w lateral); the kit only orients the sections, rounds the ends as spherical domes about the joints, refines between the stated stations with a monotone cubic (no overshoot) and offsets a solid inwards for a fur layer."""
import math

MM = 0.001


def r5(x):
    return round(x, 5)


def quat(X, Y):
    """The unit quaternion [x, y, z, w] of the rotation whose x axis is X and y axis is Y (orthonormal)."""
    Z = [X[1] * Y[2] - X[2] * Y[1], X[2] * Y[0] - X[0] * Y[2], X[0] * Y[1] - X[1] * Y[0]]
    m = [[X[0], Y[0], Z[0]], [X[1], Y[1], Z[1]], [X[2], Y[2], Z[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1.0) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s]
    n = math.sqrt(sum(c * c for c in q))
    return [round(c / n, 6) for c in q]


def pchip(xs, ys, x):
    """Monotone cubic (Fritsch-Carlson) through (xs, ys) at x."""
    n = len(xs)
    h = [xs[i + 1] - xs[i] for i in range(n - 1)]
    d = [(ys[i + 1] - ys[i]) / h[i] for i in range(n - 1)]
    m = [0.0] * n
    m[0], m[-1] = d[0], d[-1]
    for i in range(1, n - 1):
        if d[i - 1] * d[i] > 0:
            w1, w2 = 2 * h[i] + h[i - 1], h[i] + 2 * h[i - 1]
            m[i] = (w1 + w2) / (w1 / d[i - 1] + w2 / d[i])
    i = max(0, min(n - 2, next((k for k in range(n - 1) if x <= xs[k + 1]), n - 2)))
    t = (x - xs[i]) / h[i]
    h00, h10, h01, h11 = 2 * t**3 - 3 * t**2 + 1, t**3 - 2 * t**2 + t, -2 * t**3 + 3 * t**2, t**3 - t**2
    return h00 * ys[i] + h10 * h[i] * m[i] + h01 * ys[i + 1] + h11 * h[i] * m[i + 1]


def st(u, rw, rv, vc=0.0, wc=0.0):
    """A station: position u along the axis, half width rw (lateral) and half depth rv (along v), centre offset (vc, wc); millimetres."""
    return dict(u=float(u), rw=float(rw), rv=float(rv), vc=float(vc), wc=float(wc))


DOME = (0.42, 0.72, 0.9, 0.985)


def dome(u0, length, rw, rv, vc=0.0, wc=0.0, vc1=None, wc1=None):
    """Stations of a rounded end beyond u0 (towards +u for a positive length): the quarter ellipse from the full radii at u0 to the pole at u0 + length."""
    out = []
    for t in DOME:
        k = math.sqrt(1 - t * t)
        out.append(st(u0 + length * t, max(rw * k, 0.6), max(rv * k, 0.6), vc + ((vc1 if vc1 is not None else vc) - vc) * t, wc + ((wc1 if wc1 is not None else wc) - wc) * t))
    return out if length > 0 else out[::-1]


def refine(stations, sub):
    """Inserts sub - 1 stations between the stated ones by the monotone cubic of every field over u."""
    if sub <= 1:
        return [dict(s) for s in stations]
    us = [s["u"] for s in stations]
    out = []
    for i in range(len(stations) - 1):
        for k in range(sub):
            u = us[i] + (us[i + 1] - us[i]) * k / sub
            out.append({f: (u if f == "u" else pchip(us, [s[f] for s in stations], u)) for f in ("u", "rw", "rv", "vc", "wc")})
    out.append(dict(stations[-1]))
    return out


AXES = {
    "u": dict(A=(1, 0, 0), X=(0, 0, 1), Y=(0, 1, 0)),
}


def loft(sid, bone, material, tone, stations, e=2.0, sub=2, role="body", frame=None, **extra):
    """A loft along the bone axis u (or along `frame` = (origin, A, X, Y) in bone millimetres: axis A, lateral X, depth Y) through the stations."""
    o, A, X, Y = frame if frame else ((0, 0, 0), (1, 0, 0), (0, 0, 1), (0, 1, 0))
    q = quat(list(X), list(Y))
    sections = []
    for s in refine(stations, sub):
        p = [o[i] + A[i] * s["u"] + Y[i] * s["vc"] + X[i] * s["wc"] for i in range(3)]
        sections.append({"at": [r5(c * MM) for c in p], "rx": r5(s["rw"] * MM), "ry": r5(s["rv"] * MM), "exponent": e, "rotation": q})
    solid = {"id": sid, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": sections, "capStart": True, "capEnd": True}
    solid.update(extra)
    solid["_st"] = dict(stations=[dict(s) for s in stations], sub=sub, e=e, frame=(o, A, X, Y))
    return solid


def ellipsoid(sid, bone, material, tone, at, radii, role="body", **extra):
    solid = {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": [r5(c * MM) for c in radii], "transform": {"translation": [r5(c * MM) for c in at]}}
    solid.update(extra)
    return solid


def shrink(solid, t, sid, tone=None, **extra):
    """The solid offset inwards by t millimetres (a fur layer boundary): every half axis is t smaller, an end rounding ends where its radius reaches t."""
    floor = 0.5
    if solid["kind"] == "ellipsoid":
        radii = [max(c / MM - t, floor) for c in solid["radii"]]
        out = ellipsoid(sid, solid["bone"], solid["material"], solid["tone"] if tone is None else tone, [c / MM for c in solid["transform"]["translation"]], radii, role=solid["role"])
    else:
        meta = solid["_st"]
        kept = [st(s["u"], s["rw"] - t, s["rv"] - t, s["vc"], s["wc"]) for s in refine(meta["stations"], meta["sub"]) if s["rw"] - t >= floor and s["rv"] - t >= floor]
        if len(kept) < 2:
            raise SystemExit(f"layer {sid}: the solid {solid['id']} is thinner than {t} mm")
        out = loft(sid, solid["bone"], solid["material"], solid["tone"] if tone is None else tone, kept, e=meta["e"], sub=1, role=solid["role"], frame=meta["frame"])
    out.update(extra)
    return out


def clean(solid):
    """The solid as it is written to the source (authoring notes removed)."""
    return {k: v for k, v in solid.items() if not k.startswith("_")}


def outline(solid, view):
    """The outline of a loft seen along w (`view` 'uv': points (u, v)) or along v (`view` 'uw': points (u, w)) in bone millimetres: the two chains of the section extents, closed."""
    meta = solid["_st"]
    o, A, X, Y = meta["frame"]
    k = 1 if view == "uv" else 2
    lo, hi = [], []
    for s in refine(meta["stations"], max(meta["sub"], 2)):
        c = [o[i] + A[i] * s["u"] + Y[i] * s["vc"] + X[i] * s["wc"] for i in range(3)]
        half_u = math.hypot(s["rw"] * X[0], s["rv"] * Y[0])
        half_k = math.hypot(s["rw"] * X[k], s["rv"] * Y[k])
        if abs(A[0]) >= abs(A[k]):
            lo.append((c[0], c[k] - half_k))
            hi.append((c[0], c[k] + half_k))
        else:
            lo.append((c[0] - half_u, c[k]))
            hi.append((c[0] + half_u, c[k]))
    return hi + lo[::-1]


def simplify(points, tol):
    """Douglas-Peucker on a closed ring (millimetres)."""
    def dp(pts):
        if len(pts) < 3:
            return pts
        (x0, y0), (x1, y1) = pts[0], pts[-1]
        L = math.hypot(x1 - x0, y1 - y0) or 1e-9
        best, bi = 0.0, 0
        for i in range(1, len(pts) - 1):
            d = abs((x1 - x0) * (y0 - pts[i][1]) - (x0 - pts[i][0]) * (y1 - y0)) / L
            if d > best:
                best, bi = d, i
        if best <= tol:
            return [pts[0], pts[-1]]
        return dp(pts[: bi + 1])[:-1] + dp(pts[bi:])
    n = len(points)
    far = max(range(n), key=lambda i: (points[i][0] - points[0][0]) ** 2 + (points[i][1] - points[0][1]) ** 2)
    a = dp(points[: far + 1])
    b = dp(points[far:] + [points[0]])
    return a[:-1] + b[:-1]
