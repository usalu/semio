"""🚿 Round 3 shower set: the real measures of a hand shower on a 22 mm riser rail (two end brackets, sliding holder with a tilting conical socket, 110 mm hand shower, wall elbow with a quarter-turn shut-off, 1.25 m hose hanging as a bight) and the frames, profiles and solid writers every scale states its parts with.

World at rest: x ahead from the wall (wall plane x = 0), y up (0 = lower end of the hose nut under the wall elbow), z lateral (left is +; the rail stands at z 0, the wall elbow 0.1 m to its right). Bone frames at rest: rail (origin on the wall at y 0, u up, v = -x), elbow (origin on the wall at the elbow axis, u ahead, v up), lever (on the valve axis, u along the lever), holder (origin on the rail axis, u ahead, v up), shower (origin = tilt axis of the socket, u along the handle 72 degrees above the horizontal), head (origin = centre of the spray disc, u = spray direction 33 degrees below the horizontal, v towards the upper rim). Every number is metres.
"""
import math

RAIL_X = 0.060
RAIL_R = 0.011
RAIL_WALL = 0.0012
RAIL_TOP = 0.810
Y_LOW = 0.102
Y_TOP = 0.788
ROSETTE_R = 0.022
SOCKET_R = 0.015
ELBOW_Y = 0.040
VALVE_X = 0.040
HOLDER_Y = 0.620
PIVOT_X = 0.105
HANDLE_DEG = 72.0
CRANK_DEG = -15.0
HEAD_AT = (0.188, -0.021)
HEAD_R = 0.055
HOSE_R = 0.0065
HOSE_BORE = 0.0045
NUT_END = -0.036

FRAMES = {
    "rail": ((0.0, 0.0), 90.0),
    "elbow": ((0.0, ELBOW_Y), 0.0),
    "lever": ((VALVE_X, ELBOW_Y), 0.0),
    "holder": ((RAIL_X, HOLDER_Y), 0.0),
    "shower": ((PIVOT_X, HOLDER_Y), HANDLE_DEG),
}


def r5(x):
    return round(x, 5) + 0.0


def rot(p, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (p[0] * c - p[1] * s, p[0] * s + p[1] * c)


def to_world(bone, p):
    """🌍 A bone-local (u, v) point in world (x, y) at rest."""
    o, a = FRAMES[bone]
    q = rot(p, a)
    return (o[0] + q[0], o[1] + q[1])


def to_local(bone, p):
    """🦴 A world (x, y) point in the rest frame of a bone."""
    o, a = FRAMES[bone]
    return rot((p[0] - o[0], p[1] - o[1]), -a)


FRAMES["head"] = (to_world("shower", HEAD_AT), HANDLE_DEG + CRANK_DEG - 90.0)

ELBOW_Z = -0.100
HOSE_A = (VALVE_X, 0.0, ELBOW_Z)
HOSE_B = to_world("shower", (NUT_END, 0.0)) + (0.0,)
_d = rot((-1.0, 0.0), HANDLE_DEG)
HOSE_SEGMENTS = [
    (HOSE_B, (HOSE_B[0] + 0.05 * _d[0], HOSE_B[1] + 0.05 * _d[1], 0.0), (0.084, 0.50, -0.001), (0.085, 0.44, -0.003)),
    ((0.085, 0.44, -0.003), (0.0863, 0.28, -0.011), (0.086, -0.03, -0.012), (0.080, -0.20, -0.018)),
    ((0.080, -0.20, -0.018), (0.076, -0.32, -0.022), (0.042, -0.32, -0.098), (0.040, -0.20, -0.100)),
    ((0.040, -0.20, -0.100), (0.0387, -0.12, -0.1013), (0.040, -0.05, ELBOW_Z), HOSE_A),
]


def bez(seg, t):
    a, b, c, d = (1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t ** 2, t ** 3
    return tuple(a * seg[0][i] + b * seg[1][i] + c * seg[2][i] + d * seg[3][i] for i in range(3))


def hose_points(counts):
    """〰️ The hose centre line in world (x, y, z) from the hand shower nut to the elbow nut: counts[i] steps on Bezier segment i."""
    out = [HOSE_SEGMENTS[0][0]]
    for seg, n in zip(HOSE_SEGMENTS, counts):
        out += [bez(seg, k / n) for k in range(1, n + 1)]
    return out


def hose_length():
    pts = hose_points((200, 200, 200, 200))
    return sum(math.dist(pts[i], pts[i + 1]) for i in range(len(pts) - 1))


def hose_path_d(view):
    """✏️ The hose centre line as an SVG path: the side elevation in the elbow frame (x ahead, y up from the elbow axis), the front as (lateral, height), the plan as (ahead, lateral)."""
    pick = {"elevation": lambda p: (p[0], p[1] - ELBOW_Y), "front": lambda p: (p[2], p[1]), "plan": lambda p: (p[0], p[2])}[view]
    f = lambda p: " ".join(fmt(c) for c in pick(p))
    d = "M " + f(HOSE_SEGMENTS[0][0])
    for seg in HOSE_SEGMENTS:
        d += " C " + " ".join(f(p) for p in seg[1:])
    return d


def fmt(v):
    s = f"{round(v, 5):.5f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def base(i, bone, material, role, tone):
    return {"id": i, "bone": bone, "material": material, "role": role, "tone": tone}


def revolve(i, bone, material, role, tone, profile, axis, origin, **extra):
    """🏺 A revolve: profile of (axial, radial) about the axis through origin, bone-local."""
    s = {"id": i, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[r5(a), r5(r)] for a, r in profile], "axis": list(axis), "origin": [r5(c) for c in origin]}
    s.update(extra)
    return s


def tube_loop(a0, a1, r_out, r_in):
    """⭕ The closed loop of a tube wall between two axial positions."""
    return [(a0, r_in), (a0, r_out), (a1, r_out), (a1, r_in), (a0, r_in)]


ACROSS_U = [0, -0.70711, 0, 0.70711]


def across(deg):
    """🧭 The section plane across a path that runs in the (u, v) plane at deg from u towards v: the section x stays lateral (w), so that sections never twist against each other along a bent bar."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    m = [[0.0, s, c], [0.0, -c, s], [1.0, 0.0, 0.0]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        r = math.sqrt(t + 1.0) * 2
        q = [(m[2][1] - m[1][2]) / r, (m[0][2] - m[2][0]) / r, (m[1][0] - m[0][1]) / r, r / 4]
    else:
        r = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / r, r / 4, (m[1][2] + m[2][1]) / r, (m[0][2] - m[2][0]) / r]
    return [round(x, 5) for x in q]


def bent_bar(i, bone, material, role, tone, points, radius, **extra):
    """🪝 A round bar bent in the (u, v) plane of its bone through points (u, v): mitred sections halfway between the directions of the two legs that meet at a point."""
    sections, turns = [], {}
    for n, p in enumerate(points):
        a, b = points[max(n - 1, 0)], points[min(n + 1, len(points) - 1)]
        turns[(r5(p[0]), r5(p[1]))] = across(math.degrees(math.atan2(b[1] - a[1], b[0] - a[0])))
        sections.append((p[0], p[1], 0, radius, radius))
    return loft(i, bone, material, role, tone, sections, rotation=lambda s: turns[(r5(s[0]), r5(s[1]))], **extra)


def loft(i, bone, material, role, tone, sections, rotation=None, **extra):
    """🧵 A loft through sections (u, v, w, rx, ry[, exponent]); rotation fixes every section plane (ACROSS_U: across the bone axis, rx lateral, ry along v) where the path wanders off the axis."""
    out = []
    for s in sections:
        e = {"at": [r5(s[0]), r5(s[1]), r5(s[2])], "rx": r5(s[3]), "ry": r5(s[4])}
        if len(s) > 5:
            e["exponent"] = s[5]
        if rotation:
            e["rotation"] = list(rotation(s) if callable(rotation) else rotation)
        out.append(e)
    d = {"id": i, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": out, "capStart": True, "capEnd": True}
    d.update(extra)
    return d


def ellipsoid(i, bone, material, role, tone, at, radii, **extra):
    d = {"id": i, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": [r5(c) for c in radii], "transform": {"translation": [r5(c) for c in at]}}
    d.update(extra)
    return d


def extrude(i, bone, material, role, tone, plane, low, high, profile, **extra):
    """🧇 An extrude: a polygon on a coordinate plane of the bone frame (xy = (u, v) along w, yz = (v, w) along u, zx = (w, u) along v) pushed from low to high."""
    d = {"id": i, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "plane": plane, "from": r5(low), "to": r5(high), "profile": [[r5(a), r5(b)] for a, b in profile]}
    d.update(extra)
    return d


def clip_profile(r_out, r_in, slot_deg, steps):
    """🧲 The C section of the holder socket on the (v, w) plane: a ring of r_in to r_out whose slot of slot_deg opens towards -v, where the hose slips in."""
    half = slot_deg / 2
    at = lambda r, a: (-r * math.cos(math.radians(a)), r * math.sin(math.radians(a)))
    angles = [half + (360 - 2 * half) * n / steps for n in range(steps + 1)]
    return [at(r_out, a) for a in angles] + [at(r_in, a) for a in reversed(angles)]


def capsule(i, bone, material, role, tone, p0, p1, radius, **extra):
    d = {"id": i, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": [r5(c) for c in p0], "p1": [r5(c) for c in p1], "r0": r5(radius), "r1": r5(radius)}
    d.update(extra)
    return d


def hose_loft(i, counts, radius, material="epdm-hose", tone=0.55, into_valve=False, **extra):
    """🪢 The hose (or its water core) as one loft of circles along the centre line, in the elbow frame; into_valve runs it on up to the valve axis where a scale has merged the outlet and its nut into the elbow."""
    points = hose_points(counts) + ([(VALVE_X, ELBOW_Y, ELBOW_Z)] if into_valve else [])
    return loft(i, "elbow", material, "body", tone, [(p[0], p[1] - ELBOW_Y, p[2] - ELBOW_Z, radius, radius) for p in points], **extra)


def mirror_profile(profile):
    """🪞 The closed outline of a revolve profile (axial, radial) seen from the side: (axial, +radial) out, (axial, -radial) back."""
    upper = [(a, r) for a, r in profile if r > 0]
    return upper + [(a, -r) for a, r in reversed(upper)]


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]
