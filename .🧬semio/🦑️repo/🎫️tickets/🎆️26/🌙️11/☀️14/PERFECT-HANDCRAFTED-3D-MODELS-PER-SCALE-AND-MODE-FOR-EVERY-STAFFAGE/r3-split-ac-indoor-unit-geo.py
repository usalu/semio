"""❄️ Round 3, appliances/split-ac-indoor-unit: the measures and outlines every other r3-split-ac-indoor-unit script builds on.

Frame of bone `unit` (side elevation, seen from the right): u = x away from the wall plane, v = y up from the underside, w = z lateral (left of the unit is +).
The side profile is a D: flat back on the wall, flat top with the intake grille, a front panel, and a lower quarter circle (centre C, radius R) that carries the discharge flap.
References: Mitsubishi Electric MSZ-AP25VG (299 x 798 x 219 mm, 10.5 kg), Daikin FTXM25N (294 x 811 x 272 mm, 10 kg).
"""
import math

H = 0.29
D = 0.21
HALF = 0.425
INNER = 0.395
FLAP = 0.392
CORNER = 0.035
C = (0.095, 0.115)
R = 0.115
FLAP_REAR_DEG = -82.0
FLAP_FRONT_DEG = -38.0
LIP_REAR_DEG = -82.5
LIP_FRONT_DEG = -37.5
GRILLE = (0.03, 0.135)
GRILLE_FLOOR = 0.281
PANEL = (0.2, 0.118, 0.255)
PIVOT_DEG = -79.8
PIVOT_R = 0.111
PIVOT = (0.1147, 0.0058)
FRONT_MID = (C[0] + PIVOT_R * math.cos(math.radians(FLAP_FRONT_DEG)), C[1] + PIVOT_R * math.sin(math.radians(FLAP_FRONT_DEG)))
VANE_ANGLE = 31.0
VANE_LENGTH = 0.079
FAN = (0.11, 0.115)
FAN_R = 0.05
JET = (0.148, 0.022)


def r6(v):
    """🔢 Rounds a coordinate to a micrometre."""
    return round(v + 0.0, 6)


def pt(p):
    """📍 Rounds a point."""
    return [r6(p[0]), r6(p[1])]


def arc(cx, cy, r, a0, a1, step=6.0):
    """🌙 Points of a circular arc from a0 to a1 degrees (both included), at most `step` degrees apart."""
    n = max(1, int(math.ceil(abs(a1 - a0) / step)))
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def on_c(deg, radius):
    """🎯 Point on the lower front quarter circle at angle `deg` and `radius` from C."""
    return (C[0] + radius * math.cos(math.radians(deg)), C[1] + radius * math.sin(math.radians(deg)))


def outer_d(step=6.0):
    """🅳 The closed outer side profile, counter-clockwise from the lower back corner."""
    pts = [(0.0, 0.0)]
    pts += arc(C[0], C[1], R, -90, 0, step)
    pts += [(D, H - CORNER)]
    pts += arc(D - CORNER, H - CORNER, CORNER, 0, 90, step * 2)[1:]
    pts += [(0.0, H)]
    return dedupe(pts)


def dedupe(pts):
    """🧹 Drops consecutive duplicates and a closing repeat."""
    out = []
    for p in pts:
        if not out or abs(out[-1][0] - p[0]) > 1e-7 or abs(out[-1][1] - p[1]) > 1e-7:
            out.append(p)
    if len(out) > 2 and abs(out[0][0] - out[-1][0]) < 1e-7 and abs(out[0][1] - out[-1][1]) < 1e-7:
        out.pop()
    return out


def duct(lip_inner, simple=False):
    """🌬️ The discharge mouth behind the flap, from the rear lip to the front lip, with lips reaching in to radius `lip_inner`."""
    rear_in = on_c(LIP_REAR_DEG, lip_inner)
    front_in = on_c(LIP_FRONT_DEG, lip_inner)
    if simple:
        return [on_c(LIP_REAR_DEG, R), rear_in, (0.105, 0.05), (0.15, 0.072), front_in, on_c(LIP_FRONT_DEG, R)]
    return [on_c(LIP_REAR_DEG, R), rear_in, (0.097, 0.022), (0.097, 0.045), (0.11, 0.062), (0.135, 0.07), (0.16, 0.066), front_in, on_c(LIP_FRONT_DEG, R)]


def casing_profile(step, lip_inner, recess=True, pocket=PANEL, simple=False):
    """🧱 Side profile of the middle casing: the D with the grille recess, the panel pocket and the discharge mouth cut in."""
    pts = [(0.0, 0.0)]
    pts += arc(C[0], C[1], R, -90, LIP_REAR_DEG, step)
    pts += duct(lip_inner, simple)[1:-1]
    pts += arc(C[0], C[1], R, LIP_FRONT_DEG, 0, step)
    if pocket:
        x, y0, y1 = pocket
        pts += [(D, y0), (x, y0), (x, y1), (D, y1)]
    else:
        pts += [(D, H - CORNER)]
    pts += arc(D - CORNER, H - CORNER, CORNER, 0, 90, step * 2)[1:]
    if recess:
        pts += [(GRILLE[1], H), (GRILLE[1], GRILLE_FLOOR), (GRILLE[0], GRILLE_FLOOR), (GRILLE[0], H)]
    pts += [(0.0, H)]
    return [pt(p) for p in dedupe(pts)]


def centroid(pts):
    """⚖️ Area centroid of a simple polygon."""
    a = cx = cy = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    return (cx / (3 * a), cy / (3 * a)), a / 2


def scaled(pts, k):
    """🔍 The profile scaled by k about its area centroid (what an extrude taper does)."""
    (cx, cy), _ = centroid(pts)
    return [(cx + (x - cx) * k, cy + (y - cy) * k) for x, y in pts]


def to_vane(p):
    """↪️ A unit-frame point in the frame of bone `vane` (joint at the pivot, u along the rest chord)."""
    a = math.radians(-VANE_ANGLE)
    dx, dy = p[0] - PIVOT[0], p[1] - PIVOT[1]
    return (dx * math.cos(a) - dy * math.sin(a), dx * math.sin(a) + dy * math.cos(a))


def from_vane(p):
    """↩️ A vane-frame point in the unit frame (rest pose)."""
    a = math.radians(VANE_ANGLE)
    return (PIVOT[0] + p[0] * math.cos(a) - p[1] * math.sin(a), PIVOT[1] + p[0] * math.sin(a) + p[1] * math.cos(a))


def vane_profile(thick_rear, thick_front, step=4.0):
    """🪽 The closed flap section in the vane frame: outer face on the casing radius, inner face thinning from rear to front, round edges."""
    a0 = FLAP_REAR_DEG + math.degrees(thick_rear / 2 / (R - thick_rear / 2))
    a1 = FLAP_FRONT_DEG - math.degrees(thick_front / 2 / (R - thick_front / 2))
    n = max(4, int(math.ceil((a1 - a0) / step)))
    outer, inner = [], []
    for k in range(n + 1):
        t = k / n
        deg = a0 + (a1 - a0) * t
        outer.append(on_c(deg, R))
        inner.append(on_c(deg, R - (thick_rear + (thick_front - thick_rear) * t)))
    front_c = on_c(a1, R - thick_front / 2)
    rear_c = on_c(a0, R - thick_rear / 2)
    nose = [(front_c[0] + thick_front / 2 * math.cos(math.radians(a1 + a)), front_c[1] + thick_front / 2 * math.sin(math.radians(a1 + a))) for a in (45, 90, 135)]
    tail = [(rear_c[0] + thick_rear / 2 * math.cos(math.radians(a0 + a)), rear_c[1] + thick_rear / 2 * math.sin(math.radians(a0 + a))) for a in (225, 270, 315)]
    ring = outer + nose + inner[::-1] + tail
    return [pt(to_vane(p)) for p in ring]


def band(polyline, t, side=1):
    """🎗️ A closed band of thickness t along a polyline, offset to its left (side 1) or right (side -1)."""
    left = []
    for i, p in enumerate(polyline):
        a = polyline[max(0, i - 1)]
        b = polyline[min(len(polyline) - 1, i + 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        n = math.hypot(dx, dy)
        left.append((p[0] - side * dy / n * t, p[1] + side * dx / n * t))
    return [pt(p) for p in dedupe(polyline + left[::-1])]


def path_d(pts):
    """✏️ An SVG path of a closed polyline."""
    return "M " + " L ".join(f"{r6(x):g},{r6(y):g}" for x, y in pts) + " Z"


def open_d(pts):
    """〰️ An SVG path of an open polyline."""
    return "M " + " L ".join(f"{r6(x):g},{r6(y):g}" for x, y in pts)
