"""🧮 Round 3, machinery/finned-heat-exchanger: the measures of the real coil and the frame helpers shared by the volumes and the 2D scripts.

The coil: a two-row water or refrigerant coil of the P40 geometry (5/8 in tubes, 40 mm tube pitch, 34.64 mm row pitch, staggered), 12 tubes per row, finned length 0.4 m, finned height 0.48 m, full circuits (12 hairpins on the right end, a return and a supply header with one stub each on the left end), in a casing of 1.5 mm galvanised sheet (two flanged tube sheets, a bottom and a top channel).

World frame at rest: x ahead (the air leaves the coil towards +x), y up, z lateral (left of the asset is +, where the headers stand). Every bone stands vertical (`frame` turned 90 degrees, children straight on), so a bone-local point (u, v, w) is world (ox - v, oy + u, w).

Usage: imported by `r3-finned-heat-exchanger-volumes.py` and `r3-finned-heat-exchanger-roots.py`.
"""
import math

RIG = {"frame": (0.0, 0.0), "fins": (0.0, 0.02), "tubes": (0.0, 0.03)}

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}

TUBE_R = 0.00794
TUBE_PITCH = 0.04
ROW_PITCH = 0.03464
ROW_X = (-0.01732, 0.01732)
ROW_Y0 = (0.03, 0.05)
TUBES_PER_ROW = 12
FIN_X = 0.03464
FIN_Y = (0.02, 0.50)
FIN_Z = 0.2
FIN_PITCH = 0.003
FIN_T = 0.0002
FIN_COUNT = 134
SHEET = 0.0015
CASE_X = 0.05
CASE_H = 0.02
TOP = 0.52
FLANGE = 0.025
SHEET_Z = FIN_Z + SHEET
FLANGE_Z = FIN_Z + SHEET + FLANGE
BEND_R = 0.02
BEND_Z0 = -(SHEET_Z + 0.01)
BEND_OUT = BEND_Z0 - BEND_R - TUBE_R
HEADER_R = 0.014
HEADER_Z = SHEET_Z + 0.03
HEADER_Y = ((0.012, 0.488), (0.032, 0.508))
STUB_Y = (0.45, 0.07)
STUB_END = 0.2956
SLEEVE_R = 0.017
SLEEVE_L = 0.02
BORE_R = 0.0125
BORE_L = 0.03
PLUG_R = 0.005
PLUG_L = 0.012
LEFT = STUB_END
RIGHT = BEND_OUT

Q = [0.0, 0.0, -math.sqrt(0.5), math.sqrt(0.5)]


def tube_y(row, k):
    """📍 Height of tube k (0 = lowest) of a row (0 = air-entering row behind, 1 = air-leaving row ahead)."""
    return round(ROW_Y0[row] + TUBE_PITCH * k, 6)


def loc(bone, x, y):
    """📌 A world rest point (x ahead, y up) in the bone-local elevation frame (u along the bone = up, v across = behind)."""
    ox, oy = RIG[bone]
    return [round(y - oy, 6), round(ox - x, 6)]


def vec(x, y, z):
    """➡️ A world direction in the bone-local frame (u, v, w)."""
    return [round(y, 6), round(-x, 6), round(z, 6)]


def place(bone):
    """🧭 The transform that carries a solid authored in world axes and world coordinates into the frame of a vertical bone."""
    ox, oy = RIG[bone]
    return {"rotation": Q, "translation": [round(-oy, 6), round(ox, 6), 0.0]}


def circle(cx, cy, r, n=32):
    """⭕ Closed counter-clockwise polygon of a circle."""
    return [[round(cx + r * math.cos(2 * math.pi * k / n), 6), round(cy + r * math.sin(2 * math.pi * k / n), 6)] for k in range(n)]


def bend_path(k, n):
    """🔁 Centre line of hairpin k: from the end of tube k of the entering row to the end of tube k of the leaving row, a half circle of 20 mm radius beyond the right tube sheet: n + 1 world points."""
    p0 = (ROW_X[0], tube_y(0, k))
    p1 = (ROW_X[1], tube_y(1, k))
    cx, cy = (p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2
    ex, ey = (p0[0] - cx) / BEND_R, (p0[1] - cy) / BEND_R
    return [[round(cx + BEND_R * math.cos(math.pi * i / n) * ex, 6), round(cy + BEND_R * math.cos(math.pi * i / n) * ey, 6), round(BEND_Z0 - BEND_R * math.sin(math.pi * i / n), 6)] for i in range(n + 1)]


def dome(length, r, cap, n):
    """🥚 Revolve profile [axial, radial] of a closed pipe of a length and radius with elliptical end caps of a height, n segments per cap."""
    pts = [[0.0, 0.0]]
    for i in range(1, n + 1):
        a = 0.5 * math.pi * i / n
        pts.append([round(cap * (1 - math.cos(a)), 6), round(r * math.sin(a), 6)])
    for i in range(n, 0, -1):
        a = 0.5 * math.pi * i / n
        pts.append([round(length - cap * (1 - math.cos(a)), 6), round(r * math.sin(a), 6)])
    pts.append([round(length, 6), 0.0])
    return pts
