"""🧩 Round 5 writers of the city-car shared by its scales: the body in white opened into a hollow cabin and boot (nose, tail, side walls, stepped floor, wheel houses, belt rails), bumper covers, wrap-around headlamps, lofted mirror housings and wheel-arch lips, all laid on the round 3 master lines of `r3_city_car_kit`.

Reference: VW Polo 6R (3970 x 1682 x 1462, wheelbase 2470, front floor 0.30 m, front seat H-point about 0.52 m, boot sill 0.72 m, boot floor 0.58 m), Suzuki Swift AZG, Honda Jazz GE; see `r3-vehicles-city-car.md` section "Round 5 handcraft".
"""
import math

from r3_city_car_kit import BOTTOM, QX, RA, XF, XR, YA, hull_bottom, hull_exponent, hull_half, hull_top, hull_xs, hull_z, interp, loft, quaternion, r5, rr, sec

X_FIREWALL = 0.78
X_TAIL = -1.79
FLOOR = [(X_TAIL, 0.58), (-0.93, 0.58), (-0.915, 0.45), (-0.62, 0.45), (-0.55, 0.33), (X_FIREWALL, 0.33)]
WHEELHOUSE_Z = 0.585


def unit(v):
    length = math.sqrt(sum(c * c for c in v))
    return [c / length for c in v]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def framed(at, normal, tangent, rx, ry, n):
    """📐 A section in the plane spanned by normal (carrying rx) and tangent x normal (carrying ry), the tangent being the direction of travel."""
    c2 = unit(tangent)
    c0 = unit([normal[k] - sum(normal[j] * c2[j] for j in range(3)) * c2[k] for k in range(3)])
    c1 = cross(c2, c0)
    return sec(at, rx, ry, n, quaternion(c0, c1, c2))


def stations(lo, hi, steps):
    return [lo] + [x for x in hull_xs(steps) if lo + 0.004 < x < hi - 0.004] + [hi]


def hull_range(id, lo, hi, steps, **fields):
    """🛻 The master hull between two stations: the nose ahead of the firewall or the tail panel behind the boot."""
    out = []
    for x in stations(lo, hi, steps):
        bottom, top = hull_bottom(x), hull_top(x)
        out.append(sec([x, (top + bottom) / 2, 0], hull_half(x), (top - bottom) / 2, hull_exponent(x), QX))
    return loft(id, out, **fields)


def side_wall(id, sign, steps, t, **fields):
    """🚪 One side of the cabin and boot: a wall of thickness t whose outer face is the master hull side, from the sill or arch line up to the belt."""
    out = []
    for x in stations(X_TAIL - 0.01, X_FIREWALL + 0.01, steps):
        bottom, top = hull_bottom(x), hull_top(x)
        out.append(sec([x, (top + bottom) / 2, sign * (hull_half(x) - t / 2)], t / 2, (top - bottom) / 2, 8, QX))
    return loft(id, out, **fields)


def floor_pan(id, t_wall, xs, min_t=0.015, **fields):
    """🪵 The floor from the firewall to the tail: front footwells at 0.33 m, the rear seat riser at 0.45 m, the boot floor at 0.58 m; over the rear wheel arches it rests on the arch line, and where the arch rises through it the floor narrows to the wheel-house plates."""
    out = []
    for x in xs:
        top = interp(FLOOR, x)
        bottom = min(hull_bottom(x), top - min_t)
        half = WHEELHOUSE_Z if hull_bottom(x) > top - min_t else hull_half(x) - t_wall / 2
        out.append(sec([x, (top + bottom) / 2, 0], half, (top - bottom) / 2, 8, QX))
    return loft(id, out, **fields)


FLOOR_XS = [X_TAIL, -1.70, -1.665] + [round(XR + 0.365 * math.cos(math.pi * k / 24), 4) for k in range(23, 0, -1)] + [-0.935, -0.93, -0.915, -0.62, -0.55, 0.0, 0.5, X_FIREWALL]


def arch_path(xc, radius, z, a0=12, a1=168, steps=12):
    """⭕ Points on a wheel-arch circle at the lateral station z from a0 to a1 degrees (0 = ahead)."""
    return [(xc + radius * math.cos(math.radians(a)), YA + radius * math.sin(math.radians(a)), z, math.radians(a)) for a in [a0 + (a1 - a0) * k / steps for k in range(steps + 1)]]


def arch_loft(id, xc, radius, z, rx, ry, n, a0=12, a1=168, steps=12, **fields):
    """🌀 A loft along a wheel-arch circle: rx radial, ry lateral."""
    out = []
    for x, y, zz, a in arch_path(xc, radius, z, a0, a1, steps):
        radial = [math.cos(a), math.sin(a), 0.0]
        tangent = [math.sin(a), -math.cos(a), 0.0]
        out.append(framed([x, y, zz], radial, tangent, rx, ry, n))
    return loft(id, out, **fields)


def wheelhouse(id, sign, xc, z_in, z_out, **fields):
    """🛞 The inner wheel-house shell over a rear wheel: a 15 mm arched band from the inner plate to the side wall."""
    return arch_loft(id, xc, RA + 0.01, sign * (z_in + z_out) / 2, 0.0075, (z_out - z_in) / 2, 8, 4, 176, 16, **fields)


def wheelhouse_plate(id, sign, xc, z, top, half_t=0.0075, **fields):
    """🛞 The inner side plate of a rear wheel house: horizontal chords of the arch circle stacked from the floor line up to the crown."""
    radius = RA + 0.0175
    ys = [top] + [YA + radius * f for f in (0.35, 0.6, 0.8, 0.92, 0.97)]
    out = []
    for y in ys:
        half = math.sqrt(max(radius * radius - (y - YA) ** 2, 0.0004))
        out.append(sec([xc, y, sign * z], half, half_t, 8))
    return loft(id, out, **fields)


def lip(id, sign, xc, steps=14, **fields):
    """🪶 The flared lip of a wheel arch: a round bead on the arch edge standing 8 mm proud of the side."""
    out = []
    for x, y, _, a in arch_path(xc, RA + 0.006, 0, 8, 172, steps):
        z = sign * (hull_z(x, min(y + 0.03, 0.95)) + 0.002)
        out.append(framed([x, y, z], [math.cos(a), math.sin(a), 0.0], [math.sin(a), -math.cos(a), 0.0], 0.012, 0.012, 2.5))
    return loft(id, out, **fields)


def bumper(id, lo, hi, y0, y1, proud, end_x, **fields):
    """🛡️ A bumper cover: a band of the hull between the heights y0 and y1, proud of the skin, wrapped round the end of the car."""
    out = []
    xs = stations(lo, hi, 12)
    for x in xs:
        bottom, top = max(hull_bottom(x), y0), min(hull_top(x), y1)
        mid = (bottom + top) / 2
        out.append(sec([x, mid, 0], hull_z(x, mid) + proud, (top - bottom) / 2 + 0.004, max(hull_exponent(x), 5), QX))
    bottom, top = max(hull_bottom(end_x), y0), min(hull_top(end_x), y1)
    mid = (bottom + top) / 2
    out.append(sec([end_x + (proud if end_x > 0 else -proud), mid, 0], hull_z(end_x, mid) * 0.96, (top - bottom) / 2, max(hull_exponent(end_x), 5), QX))
    if end_x < 0:
        out.reverse()
    return loft(id, out, **fields)


def surface_frame(x, y, sign, sunk):
    """🧭 Point of the hull skin with its outward normal."""
    e = 0.004
    nx = -(hull_z(x + e, y) - hull_z(x - e, y)) / (2 * e)
    ny = -(hull_z(x, y + e) - hull_z(x, y - e)) / (2 * e)
    n = unit([nx, ny, 1.0])
    n = [n[0], n[1], sign * n[2]]
    z = sign * hull_z(x, y)
    return [x - sunk * n[0], y - sunk * n[1], z - sunk * n[2]], n


def headlamp(id, sign, y, xs, half_height, half_depth, **fields):
    """💡 A headlamp unit wrapped round the front corner under the bonnet shut line: a lens loft laid on the hull from the wing to the grille."""
    out = []
    points = [surface_frame(x, y, sign, half_depth * 0.35) for x in xs]
    for k, (p, n) in enumerate(points):
        a = points[max(k - 1, 0)][0]
        b = points[min(k + 1, len(points) - 1)][0]
        tangent = [b[j] - a[j] for j in range(3)]
        out.append(framed(p, n, tangent, half_depth, half_height * (0.75 if k in (0, len(points) - 1) else 1.0), 3))
    return loft(id, out, **fields)


def mirror_housing(id, sign, root, tip, x0, y0, rows, **fields):
    """🪞 A door mirror housing: a loft out from the door, rows (z share, half depth, half height) with the glass face at the rear."""
    out = []
    for share, depth, height in rows:
        z = root + (tip - root) * share
        out.append(sec([x0, y0, sign * z], depth, height, 2.6))
    return loft(id, out, **fields)
