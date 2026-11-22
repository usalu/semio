"""🪜 Hand-chosen geometry of the reclaimed precast stair flight: every number of the flight in its installed frame and in the bone frames.

Installed frame: x ahead (up the flight), y up, lower landing finished floor at y = 0, first riser face at x = 0.
Flight frame (u', v'): u' along the soffit up the flight, v' its normal into the concrete, origin on the soffit line below x = 0.
Bone frame of `flight`: (u, v) = (u' - U0, v' - V0); world at rest: x = u', y = BEARER + v'.
"""
import math

from shapely.geometry import Polygon, box
from shapely.ops import unary_union

R = 0.175
G = 0.28
N_RISERS = 9
WIDTH = 1.10
HALF = WIDTH / 2
WAIST = 0.16
TERRAZZO = 0.03
ALPHA = math.atan2(R, G)
CA, SA = math.cos(ALPHA), math.sin(ALPHA)
WAIST_V = WAIST / CA
BOTTOM_STRIP = 0.25
NIB = 0.10
BLOCK = 0.20
TOP_END = 2.54
TOP_NIB_FACE = TOP_END - NIB
TOP_NIB_UNDER = 9 * R - NIB
BEARER = 0.12
BEARER_LEN = 1.40
WORN_HALF = 0.30
WEAR_R = 0.025
WEAR_DIP = 0.004
CHIP_STEP = 3
CHIP_Z = (-HALF, -0.47)
CHIP_DX, CHIP_DY = 0.05, 0.022


def soffit_y(x):
    """📐 Height of the soffit line at installed x."""
    return R / G * x - WAIST_V


def local(p):
    """🔄 Installed (x, y) to flight frame (u', v')."""
    x, y = p[0], p[1] + WAIST_V
    return (x * CA + y * SA, -x * SA + y * CA)


X_SOFFIT_LOW = (-BLOCK + WAIST_V) / (R / G)

CORE_INSTALLED = (
    [(-BOTTOM_STRIP, -TERRAZZO), (-BOTTOM_STRIP, -0.045), (-BOTTOM_STRIP + 0.012, -0.062), (-BOTTOM_STRIP - 0.004, -0.081), (-BOTTOM_STRIP + 0.008, -NIB),
     (-0.15, -NIB), (-0.15, -BLOCK), (X_SOFFIT_LOW, -BLOCK), (TOP_NIB_FACE, soffit_y(TOP_NIB_FACE)), (TOP_NIB_FACE, TOP_NIB_UNDER),
     (TOP_END, TOP_NIB_UNDER), (TOP_END, 9 * R - TERRAZZO), (8 * G, 9 * R - TERRAZZO)]
    + [pt for i in range(8, 0, -1) for pt in ((i * G, i * R - TERRAZZO), ((i - 1) * G, i * R - TERRAZZO))]
    + [(0, -TERRAZZO)]
)


def tread_installed(i):
    """🟫 Terrazzo piece i: 0 the bottom landing strip, 1..8 the treads, 9 the top landing strip; (x0, x1, top, nosing)."""
    if i == 0:
        return (-BOTTOM_STRIP, 0.0, 0.0, False)
    if i == 9:
        return (8 * G, TOP_END, 9 * R, True)
    return ((i - 1) * G, i * G, i * R, True)


def tread_profile(i, zone):
    """🟫 Installed profile of terrazzo piece i in a zone: `sharp`, `worn` (nosing rounded by walking) or `chip` (spalled nosing)."""
    x0, x1, top, nosing = tread_installed(i)
    bottom = top - TERRAZZO
    if not nosing or zone == "sharp":
        return [(x0, bottom), (x1, bottom), (x1, top), (x0, top)]
    if zone == "chip":
        return [(x0, bottom), (x1, bottom), (x1, top), (x0 + CHIP_DX, top), (x0 + 0.018, top - 0.012), (x0, top - CHIP_DY)]
    worn = top - WEAR_DIP
    arc = [(x0 + WEAR_R + WEAR_R * math.cos(math.radians(a)), worn - WEAR_R + WEAR_R * math.sin(math.radians(a))) for a in (90, 120, 150, 180)]
    return [(x0, bottom), (x1, bottom), (x1, top), (x0 + 0.16, worn)] + arc


def tread_zones(i):
    """🧭 Lateral zones (w0, w1, profile kind) of terrazzo piece i at 1:50: worn walking zone in the middle, the spalled nosing of tread 3 at the right edge."""
    if i == 0:
        return [(-HALF, HALF, "sharp")]
    right = [(CHIP_Z[0], CHIP_Z[1], "chip"), (CHIP_Z[1], -WORN_HALF, "sharp")] if i == CHIP_STEP else [(-HALF, -WORN_HALF, "sharp")]
    return right + [(-WORN_HALF, WORN_HALF, "worn"), (WORN_HALF, HALF, "sharp")]


STUB_Z = (-0.40, -0.13, 0.13, 0.40)
STUB_INSTALLED = ((-BOTTOM_STRIP + 0.006, -0.066), (-BOTTOM_STRIP - 0.050, -0.090))
MORTAR_TOP = [(TOP_NIB_FACE + 0.015, TOP_NIB_UNDER - 0.012), (TOP_END - 0.006, TOP_NIB_UNDER - 0.012), (TOP_END - 0.006, TOP_NIB_UNDER), (TOP_NIB_FACE + 0.015, TOP_NIB_UNDER)]
MORTAR_TOP_Z = (-0.38, 0.27)
MORTAR_BOTTOM = [(-0.145, -BLOCK - 0.012), (-0.03, -BLOCK - 0.012), (-0.03, -BLOCK), (-0.145, -BLOCK)]
MORTAR_BOTTOM_Z = (-0.20, 0.46)
SOCKETS = [(2, -0.35), (2, 0.35), (7, -0.35), (7, 0.35)]
SOCKET_X = 0.15
SOCKET_D = 0.04
HOLES = [(1.5, 0.30), (3.5, 0.30), (5.5, 0.30), (7.5, 0.30)]
CUT_BARS_Z = (-0.45, -0.30, -0.15, 0.0, 0.15, 0.30, 0.45)


def to_local(points):
    """🔄 A list of installed points in the flight frame."""
    return [local(p) for p in points]


CORE_LOCAL = to_local(CORE_INSTALLED)
U_SOFFIT_LOW = local((X_SOFFIT_LOW, -BLOCK))[0]
U_SOFFIT_HIGH = local((TOP_NIB_FACE, soffit_y(TOP_NIB_FACE)))[0]
U0 = U_SOFFIT_LOW
V0 = WAIST / 2
LENGTH = U_SOFFIT_HIGH - U_SOFFIT_LOW
BEARER_U = (U_SOFFIT_LOW + 0.24, U_SOFFIT_HIGH - 0.40)
DENSITY = {"reinforced-concrete": 2500.0, "terrazzo": 2400.0, "softwood": 450.0, "cement-mortar": 1900.0, "steel": 7850.0}


def area_centroid(points):
    """📐 Area and centroid of a simple polygon."""
    poly = Polygon(points)
    return poly.area, poly.centroid.x, poly.centroid.y


def flight_mass():
    """⚖️ Masses and centre of the flight in the flight frame: concrete core, terrazzo pieces (full width at their sharp profile less wear and spall), mortar, bar stubs."""
    parts = []
    a, cx, cy = area_centroid(CORE_LOCAL)
    parts.append(("core", a * WIDTH * DENSITY["reinforced-concrete"], cx, cy))
    for i in range(10):
        for w0, w1, kind in tread_zones(i):
            a, cx, cy = area_centroid(to_local(tread_profile(i, kind)))
            parts.append((f"terrazzo-{i}-{kind}", a * (w1 - w0) * DENSITY["terrazzo"], cx, cy))
    for pts, (z0, z1) in ((MORTAR_TOP, MORTAR_TOP_Z), (MORTAR_BOTTOM, MORTAR_BOTTOM_Z)):
        a, cx, cy = area_centroid(to_local(pts))
        parts.append(("mortar", a * (z1 - z0) * DENSITY["cement-mortar"], cx, cy))
    m = sum(p[1] for p in parts)
    return m, sum(p[1] * p[2] for p in parts) / m, sum(p[1] * p[3] for p in parts) / m, parts


def prism_area(profile, w0, w1):
    """📏 Outer area of a prism of a profile along w."""
    poly = Polygon(profile)
    return 2 * poly.area + poly.length * (w1 - w0)


def zone_union_area(i):
    """📏 Outer area of terrazzo piece i: every zone prism, the seams between neighbouring zones counted only where their profiles differ."""
    zones = tread_zones(i)
    total = 0.0
    polys = [Polygon(to_local(tread_profile(i, k))) for _, _, k in zones]
    for (w0, w1, _), poly in zip(zones, polys):
        total += poly.length * (w1 - w0)
    total += polys[0].area + polys[-1].area
    for a, b in zip(polys, polys[1:]):
        total += a.symmetric_difference(b).area
    return total


if __name__ == "__main__":
    m, cu, cv = flight_mass()[:3]
    print("[DEBUG] alpha", math.degrees(ALPHA), "waist_v", WAIST_V, "x_soffit_low", X_SOFFIT_LOW)
    print("[DEBUG] core local bounds", Polygon(CORE_LOCAL).bounds, "area", Polygon(CORE_LOCAL).area)
    print("[DEBUG] soffit u", U_SOFFIT_LOW, U_SOFFIT_HIGH, "length", LENGTH, "mid", (U_SOFFIT_LOW + U_SOFFIT_HIGH) / 2)
    print("[DEBUG] flight mass", m, "centre", cu, cv)
    print("[DEBUG] bearer mass", 2 * BEARER * BEARER * BEARER_LEN * DENSITY["softwood"])
    allp = unary_union([Polygon(CORE_LOCAL)] + [Polygon(to_local(tread_profile(i, "sharp"))) for i in range(10)])
    print("[DEBUG] flight union bounds", allp.bounds)
