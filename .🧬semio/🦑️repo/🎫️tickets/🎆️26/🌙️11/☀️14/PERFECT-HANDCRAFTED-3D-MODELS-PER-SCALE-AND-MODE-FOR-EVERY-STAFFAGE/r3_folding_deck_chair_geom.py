"""🏖️ Round 3 geometry of `furniture/folding-deck-chair`: the real three-frame deck chair (back frame, seat frame, notched prop, hanging sling), its kinematics per notch and every hand-stated profile shared by the 3D solids and the 2D drawings.

Reference and decisions: `r3-furniture-folding-deck-chair.md`.
"""
import math

BE, CA, SH = "beech-timber", "cotton-canvas", "steel-hardware"
R_TIMBER, R_BACK, R_SEAT = "frame-timber", "canvas-back", "canvas-seat"

RAIL_T, RAIL_H, END_R = 0.020, 0.040, 0.020
BACK_FOOT, BACK_TOP = -0.30, 0.90
SEAT_LEN, SEAT_PIVOT = 1.04, 0.565
STRUT_PIVOT, STRUT_H = 0.45, 0.032
NOTCHES = (0.10, 0.145, 0.19, 0.235)
NOTCH_R, BAR_R = 0.012, 0.011
TOP_RAIL_U, FRONT_RAIL_U, REAR_RAIL_U, FOOT_RAIL_U = 0.865, 1.005, 0.045, -0.20
CROSS_HALF_U, CROSS_HALF_V, CROSS_R = 0.020, 0.010, 0.004
Z_STRUT, Z_BACK, Z_SEAT, Z_CANVAS = (0.270, 0.290), (0.248, 0.268), (0.226, 0.246), 0.210
T_LOCAL = (TOP_RAIL_U - CROSS_HALF_U, -(CROSS_HALF_V + 0.00375))
S_LOCAL = (FRONT_RAIL_U - CROSS_HALF_U, CROSS_HALF_V + 0.00375)
THETA_REST = 74.0
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def rnd(value, digits=5):
    out = round(value, digits)
    return 0.0 if out == 0 else out


def direction(deg):
    return (math.cos(math.radians(deg)), math.sin(math.radians(deg)))


def place(origin, deg, u, v=0.0):
    c, s = direction(deg)
    return (origin[0] + u * c - v * s, origin[1] + u * s + v * c)


def local(origin, deg, point):
    c, s = direction(deg)
    dx, dy = point[0] - origin[0], point[1] - origin[1]
    return (dx * c + dy * s, -dx * s + dy * c)


def angle_of(a, b):
    return math.degrees(math.atan2(b[1] - a[1], b[0] - a[0]))


def wrap(deg):
    while deg > 180:
        deg -= 360
    while deg <= -180:
        deg += 360
    return deg


def bisect(f, lo, hi, n=80):
    flo = f(lo)
    for _ in range(n):
        mid = (lo + hi) / 2
        fm = f(mid)
        if (fm > 0) == (flo > 0):
            lo, flo = mid, fm
        else:
            hi = mid
    return (lo + hi) / 2


def seat_angle(theta):
    """📐 The seat frame inclination that puts the rear and the front foot rounds on one level for the frame angle `theta`."""
    if theta < 1e-9:
        return 0.0
    def gap(alpha):
        seat_o = (0.0, 0.0)
        pivot = place(seat_o, alpha, SEAT_PIVOT)
        rear = place(seat_o, alpha, END_R)
        front = place(pivot, alpha + 180 - theta, BACK_FOOT + END_R)
        return front[1] - rear[1]
    return bisect(gap, 0.0, 60.0)


def frames(theta):
    """🪑 World frames (origin, angle) of the seat and back bones for the frame angle `theta`, feet on y = 0."""
    alpha = seat_angle(theta)
    seat_o = (0.0, END_R - END_R * math.sin(math.radians(alpha)))
    pivot = place(seat_o, alpha, SEAT_PIVOT)
    return {"seat": (seat_o, alpha), "back": (pivot, alpha + 180 - theta)}


def strut_tip(theta, notch):
    f = frames(theta)
    return place(*f["seat"], notch, BAR_R), place(*f["back"], STRUT_PIVOT)


REST_FRAMES = frames(THETA_REST)
_n, _q = strut_tip(THETA_REST, NOTCHES[2])
STRUT_LEN = math.dist(_n, _q)


def theta_of_notch(notch):
    return bisect(lambda th: math.dist(*strut_tip(th, notch)) - STRUT_LEN, 40.0, 110.0)


def catenary(t, s, length):
    d, h = s[0] - t[0], s[1] - t[1]
    target = math.sqrt(length * length - h * h)
    a = bisect(lambda a: 2 * a * math.sinh(d / (2 * a)) - target, 0.05, 50.0)
    xm = (t[0] + s[0]) / 2 - a * math.atanh(h / length)
    c = t[1] - a * math.cosh((t[0] - xm) / a)
    return a, xm, c


def curve(cat, x):
    a, xm, c = cat
    return (x, a * math.cosh((x - xm) / a) + c)


def quarter_points(cat, t, s):
    return [curve(cat, t[0] + (s[0] - t[0]) * i / 4) for i in range(5)]


def chords(points):
    return [math.dist(points[i], points[i + 1]) for i in range(len(points) - 1)]


def sling_rest():
    """🧵 The unloaded sling at rest: the catenary between the two canvas rails, carried by four canvas bones over equal forward steps (equal sag under every chord). Its four chords add up to the rail distance of the flat-folded chair, so the folded canvas lies taut."""
    f = REST_FRAMES
    t, s = place(*f["back"], *T_LOCAL), place(*f["seat"], *S_LOCAL)
    folded = S_LOCAL[0] - (SEAT_PIVOT - T_LOCAL[0])
    length = bisect(lambda l: sum(chords(quarter_points(catenary(t, s, l), t, s))) - folded, folded, folded + 0.2)
    cat = catenary(t, s, length)
    return {"t": t, "s": s, "cat": cat, "points": quarter_points(cat, t, s), "length": length, "folded": folded}


SLING = sling_rest()
CHORD = chords(SLING["points"])
SLING_BONES = ("canvas-a", "canvas-a2", "canvas-b2", "canvas-b")
LA, LB = CHORD[0] + CHORD[1], CHORD[2] + CHORD[3]


def sling_point(x):
    return curve(SLING["cat"], x)


def sag_point(t, s):
    d = math.dist(t, s)
    x = (d * d + LA * LA - LB * LB) / (2 * d)
    h = math.sqrt(max(LA * LA - x * x, 0.0))
    ux, uy = (s[0] - t[0]) / d, (s[1] - t[1]) / d
    one = (t[0] + ux * x - uy * h, t[1] + uy * x + ux * h)
    two = (t[0] + ux * x + uy * h, t[1] + uy * x - ux * h)
    return one if one[1] < two[1] else two


def toward(a, b, distance):
    d = math.dist(a, b)
    return (a[0] + (b[0] - a[0]) * distance / d, a[1] + (b[1] - a[1]) * distance / d)


def chain(t, s, loaded):
    """⛓️ The three inner joints of the four canvas bones between the rails `t` and `s`: hanging free on a catenary, pulled into two straight runs by a sitter, or straight when the rails are as far apart as the canvas is long."""
    d = math.dist(t, s)
    if d >= sum(CHORD) - 1e-7:
        return [toward(t, s, CHORD[0]), toward(t, s, LA), toward(s, t, CHORD[3])]
    if loaded:
        hip = sag_point(t, s)
        return [toward(t, hip, CHORD[0]), hip, toward(s, hip, CHORD[3])]
    def walk(length):
        cat = catenary(t, s, length)
        joints, at = [], t
        for c in CHORD[:3]:
            x = bisect(lambda x: math.dist(at, curve(cat, x)) - c, at[0], at[0] + c)
            at = curve(cat, x)
            joints.append(at)
        return joints
    length = bisect(lambda l: CHORD[3] - math.dist(walk(l)[2], s), d + 1e-6, sum(CHORD) + 0.1)
    return walk(length)


def configuration(theta, strut_world=None, notch=None, loaded=False, tilt=0.0):
    """🧮 Absolute world frames of every bone for a frame angle; the prop sits in `notch` or points along `strut_world` degrees; `tilt` raises the whole chair about its rear feet (the folded chair rests on its foot rail)."""
    f = frames(theta)
    seat_o, alpha = f["seat"][0], f["seat"][1] + tilt
    pivot = place(seat_o, alpha, SEAT_PIVOT)
    back_deg = alpha + 180 - theta
    q = place(pivot, back_deg, STRUT_PIVOT)
    if notch is not None:
        strut_deg = angle_of(q, place(seat_o, alpha, notch, BAR_R))
    else:
        strut_deg = strut_world
    t, s = place(pivot, back_deg, *T_LOCAL), place(seat_o, alpha, *S_LOCAL)
    j1, j2, j3 = chain(t, s, loaded)
    return {
        "seat": (seat_o, alpha),
        "back": (pivot, back_deg),
        "strut": (q, strut_deg),
        "canvas-a": (t, angle_of(t, j1)),
        "canvas-a2": (j1, angle_of(j1, j2)),
        "canvas-b": (s, angle_of(s, j3)),
        "canvas-b2": (j3, angle_of(j3, j2)),
    }


PARENT = {"seat": None, "back": "seat", "strut": "back", "canvas-a": "back", "canvas-a2": "canvas-a", "canvas-b": "seat", "canvas-b2": "canvas-b"}
REST = configuration(THETA_REST, notch=NOTCHES[2])


def relative(config):
    out = {}
    for bone, (_, deg) in config.items():
        parent = PARENT[bone]
        out[bone] = wrap(deg - (config[parent][1] if parent else 0.0))
    return out


REST_REL = relative(REST)


def offsets(config):
    rel = relative(config)
    return {bone: {"angleDeg": rnd(wrap(rel[bone] - REST_REL[bone]), 4)} for bone in PARENT if abs(wrap(rel[bone] - REST_REL[bone])) > 1e-6}


def arc(centre, radius, a0, a1, segments):
    return [(centre[0] + radius * math.cos(math.radians(a0 + (a1 - a0) * i / segments)), centre[1] + radius * math.sin(math.radians(a0 + (a1 - a0) * i / segments))) for i in range(segments + 1)]


def stadium(u0, u1, half, segments):
    """💊 A rail seen from the side: straight edges with half-round ends, tips at `u0` and `u1`."""
    return arc((u1 - half, 0.0), half, -90, 90, segments) + arc((u0 + half, 0.0), half, 90, 270, segments)


def notched_rail(segments, round_notches):
    """🪚 The seat rail seen from the side: half-round ends and the four prop notches cut into its upper edge."""
    half = RAIL_H / 2
    points = arc((SEAT_LEN - half, 0.0), half, -90, 90, segments)
    for n in reversed(NOTCHES):
        if round_notches:
            points += [(n + NOTCH_R, half)] + arc((n, NOTCH_R), NOTCH_R, 0, -180, 6) + [(n - NOTCH_R, half)]
        else:
            points += [(n + NOTCH_R, half), (n + NOTCH_R, 0.0), (n - NOTCH_R, 0.0), (n - NOTCH_R, half)]
    return points + arc((half, 0.0), half, 90, 270, segments)


def rounded_rect(cu, cv, hu, hv, r, segments=3):
    if r <= 0:
        return [(cu - hu, cv - hv), (cu + hu, cv - hv), (cu + hu, cv + hv), (cu - hu, cv + hv)]
    return (
        arc((cu + hu - r, cv - hv + r), r, -90, 0, segments)
        + arc((cu + hu - r, cv + hv - r), r, 0, 90, segments)
        + arc((cu - hu + r, cv + hv - r), r, 90, 180, segments)
        + arc((cu - hu + r, cv - hv + r), r, 180, 270, segments)
    )


def sleeve(cu, cv, hu, hv, r, t, segments=4):
    """🧣 The canvas lapped round a cross rail: a U of thickness `t` over the lower face, the far end and the upper face, open towards the bone origin."""
    inner = [(cu - hu, cv - hv)] + arc((cu + hu - r, cv - hv + r), r, -90, 0, segments) + arc((cu + hu - r, cv + hv - r), r, 0, 90, segments) + [(cu - hu, cv + hv)]
    outer = [(cu - hu, cv - hv - t)] + arc((cu + hu - r, cv - hv + r), r + t, -90, 0, segments) + arc((cu + hu - r, cv + hv - r), r + t, 0, 90, segments) + [(cu - hu, cv + hv + t)]
    return outer + inner[::-1]


def sling_curve(bone, segments):
    """🪢 The hanging canvas of one sling bone as points in its bone frame (origin at its joint nearer its rail, x along the chord)."""
    origin, deg = REST[bone]
    i = SLING_BONES.index(bone)
    x0, x1 = SLING["points"][i][0], SLING["points"][i + 1][0]
    if bone in ("canvas-b", "canvas-b2"):
        x0, x1 = x1, x0
    return [local(origin, deg, sling_point(x0 + (x1 - x0) * k / segments)) for k in range(segments + 1)]


def band(points, thickness, lead=0.0, tail=0.0):
    """🎗️ A sheet of `thickness` along a centre line, cut square at both ends and run on by `lead` before the first and `tail` after the last point along the end tangents."""
    def unit(a, b):
        l = math.hypot(b[0] - a[0], b[1] - a[1])
        return ((b[0] - a[0]) / l, (b[1] - a[1]) / l)
    pts = list(points)
    if lead:
        t = unit(pts[0], pts[1])
        pts = [(pts[0][0] - t[0] * lead, pts[0][1] - t[1] * lead)] + pts
    if tail:
        t = unit(pts[-2], pts[-1])
        pts = pts + [(pts[-1][0] + t[0] * tail, pts[-1][1] + t[1] * tail)]
    h, left, right = thickness / 2, [], []
    for i, p in enumerate(pts):
        t = unit(pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)])
        left.append((p[0] - t[1] * h, p[1] + t[0] * h))
        right.append((p[0] + t[1] * h, p[1] - t[0] * h))
    return left + right[::-1]


def pts(points, digits=5):
    return [[rnd(p[0], digits), rnd(p[1], digits)] for p in points]


def quaternion_z(deg):
    h = math.radians(deg) / 2
    return [0.0, 0.0, rnd(math.sin(h), 6), rnd(math.cos(h), 6)]


def foot_anchor(bone):
    """🦶 The lowest point of a foot round at rest, in the bone frame."""
    origin, deg = REST[bone]
    centre = (END_R, 0.0) if bone == "seat" else (BACK_FOOT + END_R, 0.0)
    world = place(origin, deg, *centre)
    return local(origin, deg, (world[0], world[1] - END_R))


if __name__ == "__main__":
    print("[DEBUG] strut", round(STRUT_LEN, 4), "sling length", round(SLING["length"], 4), "points", [(round(p[0], 3), round(p[1], 3)) for p in SLING["points"]], "chords", [round(c, 4) for c in CHORD])
    for name, cfg in (("rest", REST), ("notch1 loaded", configuration(theta_of_notch(NOTCHES[0]), notch=NOTCHES[0], loaded=True)), ("half", configuration(37.0, strut_world=200.0)), ("folded", configuration(0.0, strut_world=181.6, tilt=1.6))):
        print("[DEBUG]", name, {b: (round(o[0], 3), round(o[1], 3), round(d, 2)) for b, (o, d) in cfg.items()})
