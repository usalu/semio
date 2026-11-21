"""🍄 Round 5 (from round 3) propane patio heater: the real geometry of the mushroom heater in world metres (X ahead to the control knob, Y up, Z lateral), the bone frames of the corrected rig and the writers of solids and outlines.

Every bone of the heater stands upright (angle 90 at the root): its u runs up (world +Y), v points to world -X and w is world +Z. The knob bone points ahead (u = +X, v = +Y).

@see r5-patio-heater-roots.py — rig, anchors and texts that use these numbers
@see design.md — feature sizes 7.5 / 15 / 30 / 75 mm
"""
import math

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}

DISH_R = 0.235
DISH_TOP = 0.04
HOUSING_R = 0.21
SHEET = 0.0012
WALL_TOP = 0.70
LID_TOP = 0.73
SOCKET_R = 0.05
POLE_R = 0.03
POLE_T = 0.0015
POLE_Y = 0.73
HEAD_Y = 1.66
DECK_U = 0.14
SCREEN_R = 0.11
DECK_R = SCREEN_R + 0.004
BAND_R = SCREEN_R + 0.0035
ROD_RC = SCREEN_R + 0.0075
CAP_R = SCREEN_R + 0.01
SCREEN_TOP_U = 0.35
CAP_TOP_U = 0.365
REFLECTOR_Y = HEAD_Y + CAP_TOP_U
RIM_U = 0.05
DOME_R = 0.394
DOME_H = 0.15
DOME_P = 1.35
DISH_T = 0.001
TOP_Y = 2.25
KNOB_Y = HEAD_Y + 0.082
KNOB_X = 0.089
WHEEL_X = -0.26
WHEEL_Y = 0.05
WHEEL_Z = 0.16
WHEEL_R = 0.045
WHEEL_W = 0.03
BOTTLE_Y = DISH_TOP
BOTTLE_R = 0.15
DOOR_SWEEP = 1.5
DOOR_Y0 = 0.10
DOOR_Y1 = 0.66
DOOR_PROUD = 0.003
RIB_R = 0.22

BONES = {
    "base": (0.0, 0.0, "up"),
    "bottle": (0.0, BOTTLE_Y, "up"),
    "pole": (0.0, POLE_Y, "up"),
    "burner-head": (0.0, HEAD_Y, "up"),
    "reflector": (0.0, REFLECTOR_Y, "up"),
    "knob": (KNOB_X, KNOB_Y, "ahead"),
}


def r6(v):
    """🎯 Rounds to a tenth of a millimetre and drops negative zero."""
    x = round(v, 6)
    return 0.0 if x == 0 else x


def local(bone, X, Y, Z=0.0):
    """🧭 World point to the bone-local (u, v, w) of `bone` at rest."""
    x0, y0, kind = BONES[bone]
    if kind == "up":
        return [r6(Y - y0), r6(-(X - x0)), r6(Z)]
    return [r6(X - x0), r6(Y - y0), r6(Z)]


def local2(bone, X, Y):
    """📐 World (X, Y) of the side elevation to the bone-local elevation (x, y)."""
    p = local(bone, X, Y)
    return [p[0], p[1]]


def dome_outer(r):
    """⛱️ Height of the outer dome surface above the reflector bone origin at radius r."""
    return RIM_U + DOME_H * (1 - (min(r, DOME_R) / DOME_R) ** DOME_P)


def dome_slope(r):
    """📉 d(height)/dr of the outer dome surface."""
    if r <= 0:
        return 0.0
    return -DOME_H * DOME_P / DOME_R * (r / DOME_R) ** (DOME_P - 1)


def dome_curve(r0, r1, n, offset=0.0):
    """🌂 Points (u, r) of the dome surface from r0 to r1, moved by `offset` along the outward normal (negative = inward)."""
    pts = []
    for i in range(n + 1):
        t = i / n
        r = r0 + (r1 - r0) * (1 - math.cos(t * math.pi / 2)) if r1 > r0 else r0 + (r1 - r0) * math.sin(t * math.pi / 2)
        u = dome_outer(r)
        s = dome_slope(r)
        L = math.hypot(1, s)
        nu, nr = 1 / L, -s / L
        pts.append([r6(u + offset * nu), r6(max(0.0, r + offset * nr))])
    return pts


def arc(cu, cr, rad, a0, a1, n):
    """🌀 Points (u, r) of a circular arc about (cu, cr) from angle a0 to a1 (degrees, 0 = +r, 90 = +u)."""
    out = []
    for i in range(n + 1):
        a = math.radians(a0 + (a1 - a0) * i / n)
        out.append([r6(cu + rad * math.sin(a)), r6(cr + rad * math.cos(a))])
    return out


def loop(points):
    """➰ Closes a profile into a loop (first point repeated)."""
    pts = [list(p) for p in points]
    if pts[0] != pts[-1]:
        pts.append(list(pts[0]))
    return pts


def quat_u(angle):
    """🔄️ Quaternion [x, y, z, w] of a turn by `angle` radians about the bone axis u."""
    return [r6(math.sin(angle / 2)), 0.0, 0.0, r6(math.cos(angle / 2))]


def solid(sid, kind, bone, material, role, tone, **fields):
    """🧱 One volume solid with the common fields first."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def revolve_up(sid, bone, material, role, tone, profile, **extra):
    """🏺 A solid of revolution about the upright bone axis u."""
    return solid(sid, "revolve", bone, material, role, tone, profile=profile, axis=[1, 0, 0], origin=[0.0, 0.0, 0.0], **extra)


def ref(sid, of, **extra):
    """🔗 A reference to a body solid of the same scale."""
    out = {"id": sid, "of": of}
    out.update(extra)
    return out


def tag(s, **extra):
    """🏷️ The solid with extra fields (region, member, part, anchor...)."""
    out = dict(s)
    out.update(extra)
    return out


def ring(u0, u1, ri, ro):
    """⭕ Loop profile of a ring or tube wall between radii ri and ro from u0 to u1."""
    return loop([[r6(u0), r6(ri)], [r6(u0), r6(ro)], [r6(u1), r6(ro)], [r6(u1), r6(ri)]])


def dome_shell(thickness, n=24, r_start=DOME_R):
    """🛖 Loop profile of the reflector shell of the given wall thickness."""
    outer = dome_curve(r_start, 0.0, n)
    inner = dome_curve(0.0, r_start, n, -thickness)
    outer[-1][1] = 0.0
    inner[0][1] = 0.0
    inner[-1] = [r6(outer[0][0] - thickness), r6(r_start)]
    return loop(outer + inner)


def lid_outer(n):
    """🎩 Outer (Y, r) of the pressed housing lid from the wall top to the socket."""
    pts = []
    for i in range(n + 1):
        t = i / n * math.pi / 2
        c, s = math.cos(t), math.sin(t)
        pts.append([r6(WALL_TOP + (LID_TOP - WALL_TOP) * s ** (2 / 3)), r6(0.046 + (HOUSING_R - 0.046) * c ** (2 / 3))])
    return pts


def lid_shell(t, n=10):
    """🪖 Loop profile (u, r) of the lid sheet, `t` thick, offset downward and inward."""
    outer = lid_outer(n)
    inner = []
    for i, (y, r) in enumerate(outer):
        a = outer[max(0, i - 1)]
        b = outer[min(len(outer) - 1, i + 1)]
        dy, dr = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dy, dr) or 1
        ny, nr = dr / L, -dy / L
        if ny < 0:
            ny, nr = -ny, -nr
        inner.append([r6(y - t * ny), r6(r - t * nr)])
    inner[0] = [WALL_TOP, r6(HOUSING_R - t)]
    return loop(outer + list(reversed(inner)))


def circle_pts(cx, cy, r, n, a0=0.0, a1=360.0):
    """⚪ Points of a circle or arc in a plane."""
    out = []
    for i in range(n + 1 if a1 - a0 < 360 else n):
        a = math.radians(a0 + (a1 - a0) * i / n)
        out.append([r6(cx + r * math.cos(a)), r6(cy + r * math.sin(a))])
    return out


def offset_in(points, t):
    """🫧 The polyline (u, r) moved by `t` along its normal towards the inside (left of the direction of travel from the lower axis end)."""
    out = []
    n = len(points)
    for i, (u, r) in enumerate(points):
        a = points[max(0, i - 1)]
        b = points[min(n - 1, i + 1)]
        du, dr = b[0] - a[0], b[1] - a[1]
        L = math.hypot(du, dr) or 1
        nu, nr = dr / L, -du / L
        out.append([r6(u + t * nu), r6(max(0.0, r + t * nr))])
    return out


def shell_loop(points, t):
    """🥚 Closed loop of a shell `t` thick inside the outer polyline `points` that starts and ends on the axis."""
    inner = offset_in(points, t)
    inner[0] = [r6(points[0][0] + t), 0.0]
    inner[-1] = [r6(points[-1][0] - t), 0.0]
    return loop(points + list(reversed(inner)))


FINIAL_TOP_U = TOP_Y - REFLECTOR_Y
SPACER_R = 0.095


def finial_profile(n=6):
    """🔘 Open profile (u, r) of the finial on the crown, from the dome surface to its top."""
    ub = dome_outer(0.05) - DISH_T
    h = FINIAL_TOP_U - ub - 0.0045
    pts = [[r6(ub), 0.0], [r6(ub), 0.05], [r6(ub + 0.0045), 0.05]]
    for i in range(1, n + 1):
        a = math.pi / 2 * i / n
        pts.append([r6(ub + 0.0045 + h * math.sin(a)), r6(0.05 * math.cos(a))])
    pts[-1][1] = 0.0
    return pts


def dish_profile(edge):
    """🥘 Filled base pan (u, r): flat sole, rounded lower and upper edges of radius `edge`."""
    pts = [[0.0, 0.0], [0.0, r6(DISH_R - edge)]]
    pts += arc(edge, DISH_R - edge, edge, -90, 0, 4)[1:]
    pts += arc(DISH_TOP - edge, DISH_R - edge, edge, 0, 90, 4)
    pts += [[DISH_TOP, 0.0]]
    return pts


def head_profile(scale):
    """🫖 Open profile (u, r) of sleeve, valve housing and the flare up to the burner deck."""
    if scale == 50:
        return [[0.0, 0.0], [0.0, 0.035], [0.012, 0.036], [0.030, 0.060], [0.040, 0.080], [0.046, 0.085], [0.118, 0.085],
                [0.124, 0.090], [0.132, 0.104], [0.136, DECK_R], [DECK_U, DECK_R], [DECK_U, 0.0]]
    return [[0.0, 0.0], [0.0, 0.036], [0.030, 0.060], [0.046, 0.085], [0.118, 0.085], [0.136, DECK_R], [DECK_U, DECK_R], [DECK_U, 0.0]]


def cap_profile():
    """🎩 Open profile (u, r) of the pressed emitter cap."""
    return [[SCREEN_TOP_U, 0.0], [SCREEN_TOP_U, CAP_R - 0.004], [r6(SCREEN_TOP_U + 0.003), CAP_R], [r6(CAP_TOP_U - 0.004), CAP_R], [CAP_TOP_U, r6(CAP_R - 0.006)], [CAP_TOP_U, 0.0]]


def emitter_half(scale):
    """🔆 Right half (r, u) of the side silhouette of screen, bands and cap above the deck."""
    if scale == 50:
        prof = [[DECK_U, BAND_R], [DECK_U + 0.012, BAND_R], [DECK_U + 0.012, SCREEN_R], [SCREEN_TOP_U - 0.012, SCREEN_R], [SCREEN_TOP_U - 0.012, BAND_R], [SCREEN_TOP_U, BAND_R]]
        prof += [[u, r] for u, r in cap_profile()[1:-1]]
        return prof
    return [[DECK_U, SCREEN_R], [r6(CAP_TOP_U - 0.006), SCREEN_R], [r6(CAP_TOP_U - 0.006), CAP_R], [CAP_TOP_U, r6(CAP_R - 0.004)]]


def rod_points():
    """🧷 Bone-local start and end of the first guard rod (the other seven are turned copies)."""
    v, w = r6(-ROD_RC * math.cos(math.pi / 8)), r6(ROD_RC * math.sin(math.pi / 8))
    return [r6(DECK_U + 0.008), v, w], [r6(SCREEN_TOP_U - 0.004), v, w]


KNOB_HALF = {
    50: [[0.0, 0.022], [0.004, 0.0235], [0.022, 0.0235], [0.026, 0.020], [0.027, 0.0]],
    100: [[-0.006, 0.0235], [0.022, 0.0235], [0.030, 0.016], [0.034, 0.0]],
    200: [[-0.006, 0.0235], [0.030, 0.0235], [0.034, 0.0]],
    500: [[-0.03, 0.0375], [0.025, 0.0375], [0.04, 0.025], [0.045, 0.0]],
}
