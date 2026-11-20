"""⛲ Round 3 cast drinking fountain: the real geometry of a 1.0 m cast-iron pedestal fountain and the solid writers every scale states its parts with.

World: X ahead (the drinker's side, push button), Y up, Z left; metres. The pedestal is three stacked castings (base with die, fluted shaft, neck with capital cup) of 12 mm wall, a spun 1.5 mm stainless bowl of 0.40 m lies in the cup, a 28 mm brass goose-neck stands in the bowl at the rear and a push button sits in a boss on the neck.

@see round3-brief.md
@see design.md
"""
import math

G = 9.80665

Y_BASIN = 0.79
RIM_Y = 0.893
BOWL_A = 0.19
BOWL_B = 0.103
SHEET = 0.0015
BEAD_R = 0.006
BEAD_C = 0.194
HOLE_R = 0.019
CUP_R = 0.1205
CUP_HOLE = 0.024
CUP_T = 0.0105

BX = -0.10
Y_BUBBLER = 0.805
NECK_R = 0.014
BEND = 0.045
BEND_Y = 1.0 - BEND - NECK_R
BEND_END = 50.0
OUT_DEG = BEND_END - 90.0
NOZZLE_L = 0.024
TIP_L = 0.008
JET_R0 = 0.006
JET_V0 = 0.44

BTN_Y = 0.735
BTN_X = 0.096
BTN_R = 0.019

SHAFT_Y0, SHAFT_Y1 = 0.396, 0.690
SHAFT_R0, SHAFT_R1 = 0.092, 0.080
FLUTES = 16
FLUTE_D = 0.006
BORE_R0 = 0.074

DIE_R = 0.118
DOOR_Y0, DOOR_Y1 = 0.115, 0.285
DOOR_HALF = 28.0
DOOR_PROUD = 0.003
BOLT_RC = 0.181

JET_TRAVEL = 0.63


def r6(v):
    """🎯 Rounds to a micrometre and drops negative zero."""
    x = round(v, 6)
    return 0.0 if x == 0 else x


def out_dir():
    """➡️ Unit vector (X, Y) of the outlet axis, 40 degrees below the horizontal."""
    a = math.radians(OUT_DEG)
    return math.cos(a), math.sin(a)


def bend_point(deg):
    """🌀 Centre-line point of the goose-neck bend at the polar angle `deg` about the bend centre."""
    a = math.radians(deg)
    return BX + BEND + BEND * math.cos(a), BEND_Y + BEND * math.sin(a)


def outlet():
    """🚿 World (X, Y) of the centre of the nozzle mouth."""
    x, y = bend_point(BEND_END)
    dx, dy = out_dir()
    return x + (NOZZLE_L + TIP_L) * dx, y + (NOZZLE_L + TIP_L) * dy


BONES = {
    "pedestal": (0.0, 0.0, 90.0),
    "basin": (0.0, Y_BASIN, 90.0),
    "bubbler": (BX, Y_BUBBLER, 90.0),
    "button": (BTN_X, BTN_Y, 0.0),
    "jet": (outlet()[0], outlet()[1], OUT_DEG),
}


def local(bone, X, Y, Z=0.0):
    """🧭 World point to the bone-local (u, v, w) of `bone` at rest."""
    x0, y0, deg = BONES[bone]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    dx, dy = X - x0, Y - y0
    return [r6(dx * c + dy * s), r6(-dx * s + dy * c), r6(Z)]


def local2(bone, X, Y):
    """📐 World (X, Y) of the side elevation to the bone-local elevation (x, y)."""
    p = local(bone, X, Y)
    return [p[0], p[1]]


def direction(bone, dX, dY, dZ=0.0):
    """🧲 World direction to the bone-local frame."""
    deg = BONES[bone][2]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [r6(dX * c + dY * s), r6(-dX * s + dY * c), r6(dZ)]


def bowl_y(r, inner=False):
    """🥣 Height of the outer (or inner) bowl surface at radius r: a half ellipse of 0.38 m by 0.103 m under the rim."""
    a, b = (BOWL_A - SHEET, BOWL_B - SHEET) if inner else (BOWL_A, BOWL_B)
    return RIM_Y - b * math.sqrt(max(0.0, 1 - (r / a) ** 2))


def bowl_curve(r0, r1, n, inner=False):
    """〰️ Points (Y, r) of the bowl surface from r0 to r1, denser towards the rim where it turns upright."""
    a = BOWL_A - SHEET if inner else BOWL_A
    t0, t1 = math.asin(min(1.0, r0 / a)), math.asin(min(1.0, r1 / a))
    out = []
    for i in range(n + 1):
        r = a * math.sin(t0 + (t1 - t0) * i / n)
        out.append([bowl_y(r, inner), r])
    return out


def arc(cy, cr, rad, a0, a1, n):
    """🌙 Points (Y, r) of a circular arc about (cy, cr) from angle a0 to a1 (degrees, 0 = +r, 90 = +Y)."""
    out = []
    for i in range(n + 1):
        a = math.radians(a0 + (a1 - a0) * i / n)
        out.append([cy + rad * math.sin(a), cr + rad * math.cos(a)])
    return out


def quarter(y0, r0, y1, r1, n, start):
    """🏛️ Quarter-ellipse moulding from (y0, r0) to (y1, r1); `start` = "level" leaves along the radius, "plumb" leaves along the axis."""
    out = []
    for i in range(n + 1):
        t = math.radians(90.0 * i / n)
        if start == "level":
            out.append([y1 - (y1 - y0) * math.cos(t), r0 + (r1 - r0) * math.sin(t)])
        else:
            out.append([y0 + (y1 - y0) * math.sin(t), r1 - (r1 - r0) * math.cos(t)])
    return out


def base_outer(n=4):
    """🧱 Outer profile (Y, r) of the base casting: flange, torus, scotia, die, cornice."""
    p = [[0.0, 0.2], [0.018, 0.2], [0.022, 0.196], [0.022, 0.156]]
    p += arc(0.032, 0.156, 0.010, -90, 90, 2 * n)[1:]
    p += [[0.042, 0.148], [0.046, 0.148]]
    p += quarter(0.046, 0.148, 0.074, 0.124, n, "level")[1:]
    p += [[0.080, 0.124], [0.080, DIE_R], [0.320, DIE_R], [0.320, 0.124], [0.327, 0.124]]
    p += quarter(0.327, 0.124, 0.345, 0.138, n, "level")[1:]
    p += [[0.357, 0.138], [0.360, 0.134]]
    return p


def base_loop(n=4):
    """🫙 Closed wall loop of the base casting: 16 mm cast floor, 12 mm die wall, 12 mm top ring with the shaft opening."""
    return loop([[0.0, 0.0]] + base_outer(n) + [[0.360, 0.070], [0.348, 0.070], [0.348, 0.106], [0.016, 0.106], [0.016, 0.0]])


def shaft_base_loop(n=3):
    """🪨 Shaft base ring: spigot into the die, plinth fillet, torus, apophyge up to the fluted shaft."""
    p = [[0.352, 0.062], [0.352, 0.072], [0.359, 0.072], [0.359, 0.112], [0.366, 0.112], [0.366, 0.106]]
    p += arc(0.375, 0.106, 0.009, -90, 90, 2 * n)[1:]
    p += [[0.384, 0.100], [0.388, 0.100]]
    p += quarter(0.388, 0.100, SHAFT_Y0, SHAFT_R0, n, "level")[1:]
    p += [[SHAFT_Y0, BORE_R0], [0.366, BORE_R0], [0.360, 0.062]]
    return loop(p)


def astragal_loop(n=3):
    """💍 Astragal ring on top of the shaft with the seat of the capital spigot."""
    p = [[SHAFT_Y1, 0.066], [SHAFT_Y1, SHAFT_R1], [0.693, 0.084]]
    p += arc(0.699, 0.084, 0.006, -90, 90, 2 * n)[1:]
    p += [[0.705, 0.066]]
    return loop(p)


def capital_outer(n=6):
    """🏺 Outer profile (Y, r) of the capital casting from the neck band over the cavetto bell to the lip."""
    p = [[0.705, 0.082], [0.762, 0.082], [0.762, 0.086], [0.767, 0.086], [0.767, 0.083]]
    p += quarter(0.767, 0.083, 0.806, 0.119, n, "plumb")[1:]
    p += [[0.806, 0.122], [cradle_y(CUP_R), 0.122]]
    return p


def cradle_y(r):
    """🛏️ Height of the cast cradle under the bowl (half a millimetre of bedding)."""
    return bowl_y(r) - 0.0005


def capital_loop(n=6):
    """🏆 Closed loop of the capital: spigot, neck band, bell, lip, cradle under the bowl, waste opening and the roof of the bore."""
    p = [[0.699, 0.056], [0.699, 0.067], [0.704, 0.067], [0.704, 0.082]] + capital_outer(n)[1:]
    for i in range(n + 1):
        r = CUP_R + (CUP_HOLE - CUP_R) * i / n
        p.append([cradle_y(r), r])
    for i in range(n + 1):
        r = CUP_HOLE + (0.056 - CUP_HOLE) * i / n
        p.append([bowl_y(r) - CUP_T - 0.0005, r])
    return loop(p)


def flute_ring(radius, depth, per=7):
    """🏛️ Fluted section (v, w) of the shaft: 16 flutes between flat arrises, an arris on each of the four axes."""
    pitch = 2 * math.pi / FLUTES
    half = 0.39 * pitch
    out = []
    for k in range(FLUTES):
        centre = (k + 0.5) * pitch
        for i in range(per):
            s = -1 + 2 * i / (per - 1)
            a = centre + s * half
            rr = radius - depth * math.sqrt(max(0.0, 1 - s * s))
            out.append([r6(rr * math.cos(a)), r6(rr * math.sin(a))])
    return out


def circle(radius, n, cv=0.0, cw=0.0, a0=0.0):
    """⭕ Circle (or regular polygon) of n points in a profile plane."""
    return [[r6(cv + radius * math.cos(a0 + 2 * math.pi * i / n)), r6(cw + radius * math.sin(a0 + 2 * math.pi * i / n))] for i in range(n)]


def sector(r0, r1, centre_deg, half_deg, n):
    """🚪 Ring sector (v, w) between the radii r0 and r1, `half_deg` to each side of `centre_deg`."""
    out = []
    for i in range(n + 1):
        a = math.radians(centre_deg - half_deg + 2 * half_deg * i / n)
        out.append([r6(r1 * math.cos(a)), r6(r1 * math.sin(a))])
    for i in range(n + 1):
        a = math.radians(centre_deg + half_deg - 2 * half_deg * i / n)
        out.append([r6(r0 * math.cos(a)), r6(r0 * math.sin(a))])
    return out


def shaft_r(y):
    """📏 Arris radius of the tapering shaft at height y."""
    return SHAFT_R0 + (SHAFT_R1 - SHAFT_R0) * (y - SHAFT_Y0) / (SHAFT_Y1 - SHAFT_Y0)


def neck_path(n_riser=3, n_bend=10, y0=0.836):
    """🦢 Centre line (X, Y) of the goose-neck: upright riser, 130 degree bend of 45 mm radius, straight nozzle run."""
    pts = [(BX, y0 + (BEND_Y - y0) * i / n_riser) for i in range(n_riser)]
    pts += [bend_point(180 + (BEND_END - 180) * i / n_bend) for i in range(n_bend + 1)]
    dx, dy = out_dir()
    x, y = pts[-1]
    pts.append((x + NOZZLE_L * dx, y + NOZZLE_L * dy))
    return pts


def jet_path(n=8):
    """💧 The free jet (X, Y, radius): 3 L/min leave the 12 mm mouth at 0.44 m/s, fall on a parabola and thin as they speed up, down to the bowl."""
    x0, y0 = outlet()
    dx, dy = out_dir()
    vx, vy = JET_V0 * dx, JET_V0 * dy
    lo, hi = 0.0, 0.4
    for _ in range(60):
        t = (lo + hi) / 2
        x, y = x0 + vx * t, y0 + vy * t - G * t * t / 2
        if y > bowl_y(abs(x), True):
            lo = t
        else:
            hi = t
    out = []
    for i in range(n + 1):
        t = lo * (i / n) ** 0.8
        v = math.hypot(vx, vy - G * t)
        out.append((x0 + vx * t, y0 + vy * t - G * t * t / 2, JET_R0 * math.sqrt(JET_V0 / v)))
    return out


def loop(points):
    """➰ Closes a profile into a loop (first point repeated), rounded."""
    pts = [[r6(a), r6(b)] for a, b in points]
    if pts[0] != pts[-1]:
        pts.append(list(pts[0]))
    return pts


def open_profile(points):
    """📈 An open profile (closed through the axis), rounded."""
    return [[r6(a), r6(b)] for a, b in points]


def shift(points, y0):
    """↕️ Profile points (Y, r) with the axial origin moved to the height y0."""
    return [[a - y0, b] for a, b in points]


def solid(sid, kind, bone, material, role, tone, **fields):
    """🧱 One volume solid with the common fields first."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def revolve_up(sid, bone, material, role, tone, profile, X=0.0, **extra):
    """🏺 A solid of revolution about the plumb line through world X; the profile is (world Y, r)."""
    y0 = BONES[bone][1]
    closed = profile[0] == profile[-1] and len(profile) > 3
    pts = shift(profile, y0)
    return solid(sid, "revolve", bone, material, role, tone, profile=loop(pts) if closed else open_profile(pts), axis=[1, 0, 0], origin=[0.0, r6(-(X - BONES[bone][0])), 0.0], **extra)


def revolve_about(sid, bone, material, role, tone, profile, at, towards, **extra):
    """🔩 A solid of revolution about the line through world `at` (X, Y) with direction `towards` (X, Y); the profile is (distance along the line, r)."""
    closed = profile[0] == profile[-1] and len(profile) > 3
    return solid(sid, "revolve", bone, material, role, tone, profile=loop(profile) if closed else open_profile(profile), axis=direction(bone, towards[0], towards[1]), origin=local(bone, at[0], at[1]), **extra)


def extrude_up(sid, bone, material, role, tone, profile, y0, y1, **extra):
    """🏗️ A section (v, w) pushed up the bone from world height y0 to y1."""
    b = BONES[bone][1]
    return solid(sid, "extrude", bone, material, role, tone, profile=profile, plane="yz", **{"from": r6(y0 - b), "to": r6(y1 - b)}, **extra)


def tube(sid, bone, material, role, tone, path, radius, **extra):
    """🧵 A round loft along world (X, Y) centre-line points; `radius` is one number or one per point."""
    radii = radius if isinstance(radius, (list, tuple)) else [radius] * len(path)
    sections = [{"at": local(bone, x, y), "rx": r6(r), "ry": r6(r)} for (x, y), r in zip(path, radii)]
    return solid(sid, "loft", bone, material, role, tone, sections=sections, capStart=True, capEnd=True, **extra)


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


def quat_u(deg):
    """🔄️ Quaternion [x, y, z, w] of a turn by `deg` degrees about the bone axis u."""
    a = math.radians(deg) / 2
    return [r6(math.sin(a)), 0.0, 0.0, r6(math.cos(a))]
