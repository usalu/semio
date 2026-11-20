"""🚧 Round 3 sectional garage door: the real construction as numbers (track, frame hardware, panel joint, hinges, rollers, spring line) and the travel path, shared by the volumes and the roots script.

Frames: track bone = world (u = x into the garage, v = y up, w = z lateral, left is +). Panel bones run along the hinge line: joint = hinge pin, u up the panel, v towards the outside, inner face at v = PIN, outer face at v = PIN + THICK.
"""
import math

HALF = 1.2075
CAP = 1.21
PIN = 0.02
THICK = 0.04
V_IN = PIN
V_OUT = PIN + THICK
PANEL = 0.5
WALL_X = -0.075
SHEET = 0.0025

TRACK_Z0, TRACK_Z1 = 1.2175, 1.2475
TRACK_HALF = 0.026
TRACK_T = 0.002
LIP = 0.007
ARC_R = 0.5
ARC_C = (0.5, 2.0)
TRACK_END = 2.6
RUN_Y = 2.5
ARC_LEN = ARC_R * math.pi / 2
CEILING = 2.85

SHAFT = (0.07, 2.75)
SHAFT_R = 0.0127
DRUM_R = 0.09
CABLE_R = 0.08
DRUM_Z0, DRUM_W = 1.145, 0.08
COIL_R, WIRE_R, COIL_TURNS, COIL_PITCH = 0.0335, 0.004, 122, 0.008
COIL_HALF = COIL_TURNS * COIL_PITCH / 2

WHEEL_R = 0.023
STEM_R = 0.0055
WHEEL_Z0, WHEEL_Z1 = 1.2245, 1.2425
STEM_Z0 = 1.095
BOTTOM_ROLLER_U = 0.08
TOP_ROLLER_U = 0.44
CABLE_EYE = (0.06, 0.01)
HINGE_C = (-1.14, -0.38, 0.38, 1.14)
HINGE_STEP = 0.76
EYE_R = 0.008
NOSE_R, SOCKET_R = 0.052, 0.053
RAIL_U = 0.022
RAIL_TOP = 0.045
TOP_CAP_U = 0.48
TOP_FLAT = 0.497
JAMB_Y = (0.0, 0.67, 1.34)
JAMB_H = 0.06
HANGER_X = (1.3, 2.5)
BRACE_A, BRACE_B = (2.485, 2.58), (2.185, 2.85)


def r6(x):
    return round(x + 0.0, 6)


def arc(cx, cy, r, a0, a1, n):
    """🌙 Points on a circle from angle a0 to a1 (degrees, measured from +v towards +u), ends included."""
    out = []
    for k in range(n + 1):
        a = math.radians(a0 + (a1 - a0) * k / n)
        out.append([cx + r * math.sin(a), cy + r * math.cos(a)])
    return out


def ring(cx, cy, r, n, a0=0.0):
    return [[cx + r * math.cos(a0 + 2 * math.pi * k / n), cy + r * math.sin(a0 + 2 * math.pi * k / n)] for k in range(n)]


def mirror_z(profile):
    """🪞 A (z, x) or (v, w)-like profile mirrored in its lateral coordinate; `index` 0 is lateral."""
    return [[-p[0], p[1]] for p in profile][::-1]


def mirror_w(profile):
    return [[p[0], -p[1]] for p in profile][::-1]


def track_profile(lips=True):
    """🛤️ The 2 inch roller track seen along its run, as (z, x) on the left side: web outboard, two flanges, curled lips that hold the wheel."""
    z0, z1, h, t = TRACK_Z0, TRACK_Z1, TRACK_HALF, TRACK_T
    if not lips:
        return [[z0, -h], [z1, -h], [z1, h], [z0, h], [z0, h - t], [z1 - t, h - t], [z1 - t, -h + t], [z0, -h + t]]
    return [[z0, -h], [z1, -h], [z1, h], [z0, h], [z0, h - LIP], [z0 + t, h - LIP], [z0 + t, h - t], [z1 - t, h - t], [z1 - t, -h + t], [z0 + t, -h + t], [z0 + t, -h + LIP], [z0, -h + LIP]]


def angle_zx(z_in, x0, x1, t=SHEET, z_out=1.25, wall=True):
    """📐 A bracket angle as (z, x) on the left side: one leg on the wall plane, one leg square to the wall beside the track web."""
    if wall:
        return [[z_in, x0], [z_out, x0], [z_out, x1], [z_out - t, x1], [z_out - t, x0 + t], [z_in, x0 + t]]
    return [[z_out - t, x0], [z_out, x0], [z_out, x1], [z_out - t, x1]]


def ribbon(w, n, x_end=TRACK_END, x_start=None):
    """🎗️ The track band in the side elevation: a strip of width w along the travel path (vertical, quarter circle, horizontal)."""
    cx, cy = ARC_C
    outer = [[-w / 2, 0.0]] + [[cx - (ARC_R + w / 2) * math.cos(math.pi / 2 * k / n), cy + (ARC_R + w / 2) * math.sin(math.pi / 2 * k / n)] for k in range(n + 1)] + [[x_end, RUN_Y + w / 2]]
    inner = [[w / 2, 0.0]] + [[cx - (ARC_R - w / 2) * math.cos(math.pi / 2 * k / n), cy + (ARC_R - w / 2) * math.sin(math.pi / 2 * k / n)] for k in range(n + 1)] + [[x_end, RUN_Y - w / 2]]
    return outer + inner[::-1]


def centre_line(n, x_end=TRACK_END):
    cx, cy = ARC_C
    return [[0.0, 0.0]] + [[cx - ARC_R * math.cos(math.pi / 2 * k / n), cy + ARC_R * math.sin(math.pi / 2 * k / n)] for k in range(n + 1)] + [[x_end, RUN_Y]]


def panel_outline(k, n=8, rib=True):
    """🚪 The cross-section of panel k (1 = bottom) in its bone frame, counter-clockwise from the inner bottom corner; returns (points, index of the first and last point of the outer skin chain: outer shoulder, face with the rib, lip tip; nose and socket lie inside the joint and belong to the inner chain).

    Finger-protection joint: the top edge is a nose whose outer flank is an arc about the hinge pin (radius NOSE_R), the bottom edge the matching socket (radius SOCKET_R) behind an outer lip, so the joint stays closed while the upper panel swings about the pin.
    """
    top_nose, bottom_socket = k < 4, k > 1
    pts = [[0.001 if bottom_socket else RAIL_U + 0.006, V_IN]]
    if top_nose:
        pts += [[PANEL, V_IN], [PANEL, 0.03], [PANEL + 0.0375, 0.03]]
        pts += arc(PANEL, 0.0, NOSE_R, 46.15, -28.74, n)
        first = len(pts)
        pts += [[PANEL - 0.025, V_OUT]]
    else:
        pts += [[TOP_FLAT, V_IN]]
        first = len(pts)
        pts += [[TOP_FLAT, V_OUT]]
    if rib:
        pts += [[0.257, V_OUT], [0.253, V_OUT - 0.0025], [0.247, V_OUT - 0.0025], [0.243, V_OUT]]
    if bottom_socket:
        pts += [[-0.017, V_OUT]]
        last = len(pts)
        pts += arc(0.0, 0.0, SOCKET_R, -18.71, 46.47, n)
        pts += [[0.0384, 0.029], [0.001, 0.029]]
    else:
        pts += [[RAIL_U + 0.006, V_OUT]]
        last = len(pts) - 1
    return [[r6(a), r6(b)] for a, b in pts], first, last


def panel_outline_100(k):
    """🚪 The panel section at 1:100: slab, chord nose inside the pin circle, socket behind a plain lip; no rib, no groove."""
    pts = [[0.001 if k > 1 else RAIL_U, V_IN]]
    if k < 4:
        pts += [[PANEL, V_IN], [PANEL, 0.03], [PANEL + 0.0375, 0.03], [PANEL + 0.0375, 0.036], [PANEL + 0.02, 0.048], [PANEL, NOSE_R], [PANEL, V_OUT]]
    else:
        pts += [[PANEL, V_IN], [PANEL, V_OUT]]
    if k > 1:
        pts += [[0.0, V_OUT], [0.0, 0.054], [0.039, 0.054], [0.039, 0.029], [0.001, 0.029]]
    else:
        pts += [[RAIL_U, V_OUT]]
    return pts


def inset(points, d):
    """🧅 The outline moved inwards by d (counter-clockwise polygon, mitred corners): vertex i of the result belongs to vertex i of the outline."""
    n, out = len(points), []
    for i in range(n):
        ax, ay = points[i - 1]
        bx, by = points[i]
        cx, cy = points[(i + 1) % n]
        l1, l2 = math.hypot(bx - ax, by - ay), math.hypot(cx - bx, cy - by)
        n1 = (-(by - ay) / l1, (bx - ax) / l1)
        n2 = (-(cy - by) / l2, (cx - bx) / l2)
        mx, my = n1[0] + n2[0], n1[1] + n2[1]
        ml = math.hypot(mx, my)
        mx, my = mx / ml, my / ml
        s = d / max(0.3, mx * n1[0] + my * n1[1])
        out.append([r6(bx + mx * s), r6(by + my * s)])
    return out


def skins(k, t=0.0005):
    """🥪 The sandwich of panel k as three profiles: outer skin strip, core, inner skin strip."""
    pts, first, last = panel_outline(k)
    core = inset(pts, t)
    outer = pts[first:last + 1] + core[first:last + 1][::-1]
    idx = list(range(last, len(pts))) + list(range(0, first + 1))
    inner = [pts[i] for i in idx] + [core[i] for i in idx][::-1]
    return outer, core, inner, pts


def leaf_upper():
    """🔩 Hinge leaf on the upper panel in its bone frame: plate on the inner face, neck, eye round the pin at the joint."""
    eye = [[EYE_R * math.cos(math.radians(a)), EYE_R * math.sin(math.radians(a))] for a in range(116, 401, 28)]
    return [[0.075, V_IN], [0.002, V_IN], [0.002, V_IN - SHEET]] + eye + [[0.014, V_IN - SHEET], [0.075, V_IN - SHEET]]


def leaf_lower():
    return [[PANEL - u, v] for u, v in leaf_upper()][::-1]


def roller_bracket(u_eye, u0, u1):
    """🧷 Roller carrier on the inner face of an end stile: plate from u0 to u1 with the sleeve eye for the roller stem at u_eye."""
    eye = [[u_eye + 0.009 * math.cos(math.radians(a)), 0.009 * math.sin(math.radians(a))] for a in range(130, 411, 28)]
    return [[u1, V_IN], [u0, V_IN], [u0, V_IN - SHEET], [u_eye - 0.014, V_IN - SHEET]] + eye + [[u_eye + 0.014, V_IN - SHEET], [u1, V_IN - SHEET]]


def roller_profile(z_start=STEM_Z0):
    """🛞 Roller as [axial, radial] from the inboard stem end: 11 mm stem, hub, 46 mm nylon wheel with rounded tread."""
    a0, a1 = WHEEL_Z0 - z_start, WHEEL_Z1 - z_start
    return [[0, 0], [0, STEM_R], [a0 - 0.005, STEM_R], [a0 - 0.005, 0.010], [a0, 0.010], [a0, WHEEL_R - 0.0025], [a0 + 0.0025, WHEEL_R], [a1 - 0.0025, WHEEL_R], [a1, WHEEL_R - 0.0025], [a1, 0]]


def drum_profile(grooves=True):
    """🧵 Cable drum as a closed [axial, radial] loop on the shaft: hub, inboard flange, grooved cable seat, outboard flange."""
    seat = CABLE_R - 0.0015
    p = [[0, SHAFT_R], [0, 0.032], [0.014, 0.032], [0.014, DRUM_R], [0.02, DRUM_R], [0.02, seat]]
    if grooves:
        for k in range(6):
            a = 0.02 + 0.008 * k
            p += [[a + 0.0065, seat], [a + 0.007, CABLE_R], [a + 0.0075, CABLE_R], [a + 0.008, seat]]
    else:
        p += [[0.068, seat]]
    p += [[0.068, DRUM_R], [DRUM_W, DRUM_R], [DRUM_W, SHAFT_R], [0, SHAFT_R]]
    return p


def path_point(s):
    """🧭 The travel path of the rollers: s metres from the floor up the vertical track, round the quarter circle, along the horizontal track."""
    if s <= ARC_C[1]:
        return (0.0, s)
    if s <= ARC_C[1] + ARC_LEN:
        a = (s - ARC_C[1]) / ARC_R
        return (ARC_C[0] - ARC_R * math.cos(a), ARC_C[1] + ARC_R * math.sin(a))
    return (ARC_C[0] + s - ARC_C[1] - ARC_LEN, RUN_Y)


def path_s(p):
    best = min(range(0, 60001), key=lambda i: math.dist(p, path_point(i * 0.0001)))
    return best * 0.0001


def chord(s0, length, ahead=True):
    """📏 The path position whose point lies `length` from the point at s0, ahead of it or behind it."""
    p0 = path_point(s0)
    lo, hi = (s0, s0 + length * 1.6) if ahead else (s0 - length * 1.6, s0)
    for _ in range(60):
        mid = (lo + hi) / 2
        far = math.dist(p0, path_point(mid)) > length
        if ahead:
            lo, hi = (lo, mid) if far else (mid, hi)
        else:
            lo, hi = (mid, hi) if far else (lo, mid)
    return (lo + hi) / 2


def solve_pose(s1):
    """🧮 The door with hinge 1 at path position s1: bottom roller, hinges 1 to 3 and top roller on the path; returns the bone values and the points."""
    sb = chord(s1, PANEL - BOTTOM_ROLLER_U, False)
    s2 = chord(s1, PANEL)
    s3 = chord(s2, PANEL)
    st = chord(s3, TOP_ROLLER_U)
    b, j1, j2, j3, t = (path_point(s) for s in (sb, s1, s2, s3, st))
    a1 = math.atan2(j1[1] - b[1], j1[0] - b[0])
    a2 = math.atan2(j2[1] - j1[1], j2[0] - j1[0])
    a3 = math.atan2(j3[1] - j2[1], j3[0] - j2[0])
    a4 = math.atan2(t[1] - j3[1], t[0] - j3[0])
    j0 = (j1[0] - PANEL * math.cos(a1), j1[1] - PANEL * math.sin(a1))
    deg = lambda a: round(math.degrees(a), 6)
    bones = {
        "carriage-x": {"dx": round(j0[0], 6)},
        "carriage-y": {"dy": round(j0[1], 6)},
        "panel-1": {"angleDeg": deg(a1 - math.pi / 2)},
        "panel-2": {"angleDeg": deg(a2 - a1)},
        "panel-3": {"angleDeg": deg(a3 - a2)},
        "panel-4": {"angleDeg": deg(a4 - a3)},
        "spring-shaft": {"angleDeg": round(math.degrees((sb - BOTTOM_ROLLER_U) / CABLE_R), 4)},
    }
    return bones, (j0, b, j1, j2, j3, t)
