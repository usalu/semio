"""🏗️ Hand-chosen numbers and the geometry of the reclaimed bolted W-truss, its timber stillage and its two-leg chain sling.

Every part is stated once here in truss or world coordinates; the document script turns them into bone-local solids and the three 2D views of each scale.
Frames: world x ahead (along the truss), y up, z left; the truss frame T has its origin on the axis of the bottom chord at its left end, world = T + (0, Y_TRUSS, 0).
"""
import math

Y_TRUSS = 0.21
LENGTH = 9.40
SPAN = 9.00
RISE = 2.20
HEEL_X = 0.20
B_CHORD = 0.080
H_CHORD = 0.180
H_WEB = 0.120
PLATE_T = 0.006
PLATE_Z = B_CHORD / 2 + PLATE_T
TAN = RISE / (SPAN / 2)
ALPHA = math.degrees(math.atan(TAN))
COS = math.cos(math.radians(ALPHA))
SIN = math.sin(math.radians(ALPHA))
HALF_N = (H_CHORD / 2) / COS
TAIL_X = 0.25
RIDGE_X = HEEL_X + SPAN / 2

A = (HEEL_X, 0.0)
E = (HEEL_X + SPAN, 0.0)
C = (RIDGE_X, RISE)
B = (HEEL_X + SPAN / 4, RISE / 2)
D = (HEEL_X + 3 * SPAN / 4, RISE / 2)
F = (HEEL_X + SPAN / 3, 0.0)
G = (HEEL_X + 2 * SPAN / 3, 0.0)

TRESTLE_X = (2.0, LENGTH - 2.0)
SLEEPER = (0.12, 0.12, 1.50)
POST = (0.10, 1.04, 0.10)
POST_Z = (0.075, 0.175)
WEDGE_Z = (B_CHORD / 2, POST_Z[0])
SLIDE = 2.65
LIFT = 0.60
SET_DOWN = -0.11

R_CHAIN = 0.009
R_EYE = 0.006
BAND_T = 0.008
BAND_W = 0.050
EYE_V = H_CHORD / 2 + BAND_T + R_EYE
CHAIN_OFF = R_CHAIN + R_EYE
CHOKE_S = math.hypot(B[0] - A[0], B[1] - A[1]) + 0.36
BETA = 59.0
RING = (0.120, 0.070, 0.016, 0.088, 0.038)

DENSITY = {"softwood-weathered": 420.0, "softwood-sawn": 450.0, "steel-primed": 7850.0, "chain-steel": 7850.0, "polyester-sling": 1380.0}


def add(p, q):
    """➕️ Sum of two 2D points."""
    return (p[0] + q[0], p[1] + q[1])


def sub(p, q):
    """➖️ Difference of two 2D points."""
    return (p[0] - q[0], p[1] - q[1])


def mul(p, k):
    """✖️ A 2D point scaled by k."""
    return (p[0] * k, p[1] * k)


def unit(deg):
    """🧭️ Unit vector at an angle in degrees."""
    return (math.cos(math.radians(deg)), math.sin(math.radians(deg)))


def normal(deg):
    """📐️ Left normal of the unit vector at an angle in degrees."""
    return (-math.sin(math.radians(deg)), math.cos(math.radians(deg)))


def clip(poly, a, b, c):
    """✂️ Sutherland-Hodgman clip of a polygon by the half plane a·x + b·y <= c."""
    out = []
    for i, p in enumerate(poly):
        q = poly[(i + 1) % len(poly)]
        fp = a * p[0] + b * p[1] - c
        fq = a * q[0] + b * q[1] - c
        if fp <= 1e-12:
            out.append(p)
        if (fp < -1e-12 and fq > 1e-12) or (fp > 1e-12 and fq < -1e-12):
            t = fp / (fp - fq)
            out.append((p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1])))
    return dedupe(out)


def dedupe(poly):
    """🧹️ Drops consecutive duplicate vertices."""
    out = []
    for p in poly:
        if not out or math.hypot(p[0] - out[-1][0], p[1] - out[-1][1]) > 1e-7:
            out.append(p)
    if len(out) > 1 and math.hypot(out[0][0] - out[-1][0], out[0][1] - out[-1][1]) < 1e-7:
        out.pop()
    return out


def area(poly):
    """📏️ Signed area of a polygon."""
    return 0.5 * sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly)))


def centroid(poly):
    """🎯️ Centroid of a simple polygon."""
    a = area(poly)
    cx = sum((poly[i][0] + poly[(i + 1) % len(poly)][0]) * (poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1]) for i in range(len(poly))) / (6 * a)
    cy = sum((poly[i][1] + poly[(i + 1) % len(poly)][1]) * (poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1]) for i in range(len(poly))) / (6 * a)
    return (cx, cy)


def ccw(poly):
    """🔄️ The polygon in counter-clockwise order."""
    return poly if area(poly) > 0 else list(reversed(poly))


def hull(points):
    """🪢️ Convex hull (counter-clockwise) of 2D points."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3:
        return pts

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def inside(poly, p, margin=0.0):
    """🔍️ Whether p lies inside a simple polygon at least `margin` from every edge."""
    hit = False
    best = float("inf")
    for i, a in enumerate(poly):
        b = poly[(i + 1) % len(poly)]
        if (a[1] > p[1]) != (b[1] > p[1]) and p[0] < a[0] + (p[1] - a[1]) * (b[0] - a[0]) / (b[1] - a[1]):
            hit = not hit
        ex, ey = b[0] - a[0], b[1] - a[1]
        t = max(0.0, min(1.0, ((p[0] - a[0]) * ex + (p[1] - a[1]) * ey) / (ex * ex + ey * ey)))
        best = min(best, math.hypot(p[0] - a[0] - t * ex, p[1] - a[1] - t * ey))
    return hit and best >= margin - 1e-9


def mirror(poly):
    """🪞️ Mirror of truss points about the ridge line x = RIDGE_X, counter-clockwise again."""
    return ccw([(2 * RIDGE_X - p[0], p[1]) for p in poly])


def strip(p0, p1, half, extend=0.6):
    """🟫️ A rectangle of half width `half` along the segment p0..p1, lengthened by `extend` at both ends."""
    d = sub(p1, p0)
    length = math.hypot(*d)
    u = (d[0] / length, d[1] / length)
    n = (-u[1], u[0])
    a = sub(p0, mul(u, extend))
    b = add(p1, mul(u, extend))
    return ccw([add(a, mul(n, -half)), add(b, mul(n, -half)), add(b, mul(n, half)), add(a, mul(n, half))])


def top_line(x):
    """📈️ Height of the top face of the left top chord at x (truss frame)."""
    return (x - HEEL_X) * TAN + HALF_N


def under_line(x):
    """📉️ Height of the underside of the left top chord at x (truss frame)."""
    return (x - HEEL_X) * TAN - HALF_N


def members():
    """🪵️ The seven timber members as polygons in the truss frame: chords and webs cut at their joints as a carpenter cuts them."""
    hb = H_CHORD / 2
    bottom = ccw([(0, -hb), (LENGTH, -hb), (LENGTH, hb), (0, hb)])
    x_seat = HEEL_X + (hb + HALF_N) / TAN
    left = ccw([(TAIL_X, hb), (x_seat, hb), (RIDGE_X, under_line(RIDGE_X)), (RIDGE_X, top_line(RIDGE_X)), (TAIL_X, top_line(TAIL_X))])
    right = mirror(left)
    half = H_WEB / 2

    def web(p0, p1, left_x, right_x):
        poly = clip(strip(p0, p1, half), 0, -1, -hb)
        poly = clip(poly, 1, 0, right_x)
        return ccw(clip(poly, -1, 0, -left_x))

    bf = web(F, B, B[0] - 1.0, F[0])
    fc = web(F, C, F[0], RIDGE_X)
    gc = mirror(fc)
    gd = mirror(bf)
    return {"bottom": bottom, "top-l": left, "top-r": right, "bf": bf, "fc": fc, "gc": gc, "gd": gd}


def web_under_chord(poly):
    """🔧️ A web polygon clipped below the underside of the top chord it meets (left or right of the ridge)."""
    out = clip(poly, -TAN, 1, -HEEL_X * TAN - HALF_N)
    return ccw(clip(out, TAN, 1, (HEEL_X + SPAN) * TAN - HALF_N))


def member_polys():
    """🪵️ The timber members with every web cut under both top chords."""
    m = members()
    for k in ("bf", "fc", "gc", "gd"):
        m[k] = web_under_chord(m[k])
    return m


def along(p, deg, s, v=0.0):
    """📍️ The point s along direction deg from p, offset v along its left normal."""
    return add(add(p, mul(unit(deg), s)), mul(normal(deg), v))


DIR = {"bottom": 0.0, "top-l": ALPHA, "top-r": 180 - ALPHA, "bf": math.degrees(math.atan2(B[1] - F[1], B[0] - F[0])), "fc": math.degrees(math.atan2(C[1] - F[1], C[0] - F[0]))}
DIR["gc"] = 180 - DIR["fc"]
DIR["gd"] = 180 - DIR["bf"]


def gussets():
    """🔩️ The steel gusset plates (6 mm, both faces) per node as convex polygons in the truss frame, inset 10 mm from every timber edge."""
    hb = H_CHORD / 2 - 0.010
    hw = H_WEB / 2 - 0.010
    heel = ccw([(0.03, -hb), (0.70, -hb), (0.70, top_line(0.70) - 0.012), (TAIL_X, top_line(TAIL_X) - 0.012), (TAIL_X, hb), (0.03, hb)])
    node_f = hull([(F[0] - 0.30, -hb), (F[0] + 0.30, -hb), (F[0] + 0.30, hb), (F[0] - 0.30, hb), along(F, DIR["bf"], 0.40, hw), along(F, DIR["bf"], 0.40, -hw), along(F, DIR["fc"], 0.40, hw), along(F, DIR["fc"], 0.40, -hw)])
    node_b = hull([along(B, ALPHA, -0.30, hb), along(B, ALPHA, 0.30, hb), along(B, ALPHA, -0.30, -hb), along(B, ALPHA, 0.30, -hb), along(B, DIR["bf"] + 180, 0.36, hw), along(B, DIR["bf"] + 180, 0.36, -hw)])
    node_c = hull([along(C, ALPHA + 180, 0.36, -hb), along(C, ALPHA + 180, 0.36, hb), (RIDGE_X, top_line(RIDGE_X) - 0.012), along(C, -ALPHA, 0.36, hb), along(C, -ALPHA, 0.36, -hb), along(C, DIR["fc"] + 180, 0.40, hw), along(C, DIR["fc"] + 180, 0.40, -hw), along(C, DIR["gc"] + 180, 0.40, hw), along(C, DIR["gc"] + 180, 0.40, -hw)])
    return {"a": heel, "b": node_b, "c": node_c, "d": mirror(node_b), "e": mirror(heel), "f": node_f, "g": mirror(node_f)}


GUSSET_BONE = {"a": "truss", "e": "truss", "f": "truss", "g": "truss", "b": "top-chord-l", "c": "top-chord-l", "d": "top-chord-r"}


def bolts():
    """🔩️ Bolt axes (truss frame) per node: two or three M16 bolts in every member end, on the member axis."""
    left = {
        "a": [(0.12, -0.02), (0.55, -0.035), along(A, ALPHA, 0.31), along(A, ALPHA, 0.47)],
        "b": [along(B, ALPHA, -0.17), along(B, ALPHA, 0.17), along(B, DIR["bf"] + 180, 0.27)],
        "f": [(F[0] - 0.20, -0.02), (F[0], -0.035), (F[0] + 0.20, -0.02), along(F, DIR["bf"], 0.30), along(F, DIR["fc"], 0.30)],
        "c": [along(C, ALPHA + 180, 0.24), along(C, -ALPHA, 0.24), along(C, DIR["fc"] + 180, 0.30), along(C, DIR["gc"] + 180, 0.30)],
    }
    out = dict(left)
    out["e"] = [(2 * RIDGE_X - p[0], p[1]) for p in left["a"]]
    out["d"] = [(2 * RIDGE_X - p[0], p[1]) for p in left["b"]]
    out["g"] = [(2 * RIDGE_X - p[0], p[1]) for p in left["f"]]
    return out


def bolt_owner(node, p, polys):
    """🏷️ The member a bolt passes through."""
    for key in ("bf", "fc", "gc", "gd", "top-l", "top-r", "bottom"):
        if inside(polys[key], p, 0.0):
            return key
    raise ValueError(f"bolt {node} {p} lies in no member")


def check():
    """✅️ Asserts that every bolt washer lies inside its plate and member with clearance, and prints the reference numbers."""
    polys = member_polys()
    plates = gussets()
    for node, pts in bolts().items():
        for p in pts:
            key = bolt_owner(node, p, polys)
            assert inside(plates[node], p, 0.029), (node, p, "plate")
            assert inside(polys[key], p, 0.029), (node, p, key)
    return polys, plates


def sling():
    """⛓️ The two-leg chain sling: choke eye length b, chain length c, and the leg lift angle for a leg angle BETA from the vertical."""
    theta = 90 - BETA - ALPHA
    eye0 = along(A, ALPHA, CHOKE_S, EYE_V)
    u_j = (SPAN / 2 + (EYE_V + CHAIN_OFF) * SIN) / COS
    lifted = ALPHA + theta
    b = 0.0
    for _ in range(80):
        c = u_j - CHOKE_S + b
        x = eye0[0] + (b + c) * math.cos(math.radians(lifted)) + CHAIN_OFF * math.sin(math.radians(lifted))
        b += (RIDGE_X - x) / (2 * math.cos(math.radians(lifted)))
    c = u_j - CHOKE_S + b
    hook = along(eye0, lifted, b + c, -CHAIN_OFF)
    junction = along(A, ALPHA, u_j, EYE_V + CHAIN_OFF)
    return {"theta": theta, "b": b, "c": c, "eye0": eye0, "hook": hook, "junction": junction, "lifted": lifted, "u_j": u_j}


if __name__ == "__main__":
    polys, plates = check()
    for k, v in polys.items():
        print(k, round(area(v), 5), [tuple(round(c, 4) for c in p) for p in v])
    s = sling()
    print({k: (round(v, 4) if isinstance(v, float) else tuple(round(c, 4) for c in v)) for k, v in s.items()})
    print("alpha", ALPHA)
