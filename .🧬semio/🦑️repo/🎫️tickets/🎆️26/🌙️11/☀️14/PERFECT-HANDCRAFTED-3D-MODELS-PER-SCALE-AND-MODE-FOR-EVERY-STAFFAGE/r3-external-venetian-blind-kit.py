"""🪟 Real dimensions of the external venetian blind (raffstore, Z 90 lamellae) in front of a fixed aluminium window in an ETICS wall.

World: X through the wall (+ outside, facade face at X = 0), Y up, Z lateral (+ = left of the asset).
Bones: `facade` (origin 0, 0), `box` (0, BOX_Y0), `bottom-rail` (BX, RAIL_Y), `slide-i` / `slat-i` (BX, slat_y(i)); none is rotated at rest.
References: Hella AF 80 K data (top rail 58 x 56 mm galvanised steel, bottom rail 80 x 18.6 mm extruded aluminium, lifting tape 6 x 0.28 mm),
Z 90 price list data (top rail 58 x 56 mm, Z 90 lamella 93 x 20 mm), Bandalux (lamella 0.45 mm), asset references (pitch 80 mm, box 150 x 210 mm).
"""

import math

H, Z = 2.75, 0.6
SILL_Y, HEAD_Y = 1.0, 2.25
X_IN, X_BLOCK0, X_BLOCK1, X_EPS1 = -0.27, -0.255, -0.08, -0.01
COURSE, BED = 0.238, 0.012
LINTEL_H = 0.113

FR_X0, FR_X1, FR_STOP_X = -0.227, -0.157, -0.176
FR_FACE, FR_BODY = 0.05, 0.03
JAMB_Z, SIGHT_Z = Z - FR_BODY, Z - FR_FACE
SIGHT_Y0, SIGHT_Y1 = SILL_Y + FR_FACE, HEAD_Y - FR_FACE
REB_Y0, REB_Y1 = SILL_Y + FR_BODY, HEAD_Y - FR_BODY
PANE = 0.004
GL_X0, GL_X1 = -0.2, -0.176
GL_Y0, GL_Y1 = SILL_Y + 0.035, HEAD_Y - 0.035
GL_Z = Z - 0.035
SPACER = 0.012
BEAD_X0 = -0.224

SLOPE = 0.0875
SB_BACK_Y = 1.025
SB_X1 = 0.03
SB_T = 0.002
SB_DRIP_Y = 0.984

BOX_Y0, BOX_H, BOX_D = 2.32, 0.21, 0.15
SHEET = 0.002
HEM_X = 0.14
FLANGE = 0.03
BX = 0.085
HR_W, HR_H, HR_T = 0.058, 0.056, 0.0015
HR_X0, HR_X1 = BX - HR_W / 2, BX + HR_W / 2
HR_V0, HR_V1 = 0.148, 0.204
HR_Z = 0.586
SHAFT_V, SHAFT_R = 0.18, 0.006
MOTOR_R, MOTOR_Z = 0.021, 0.15
TAPE_Z, TAPE_W, TAPE_T = 0.4, 0.008, 0.0005
BEARING_Z0, BEARING_Z1 = 0.37, 0.43
BRACKET_Z0, BRACKET_Z1 = 0.435, 0.465
ANCHOR_V = 0.1375

RAIL_GX0, RAIL_GX1 = BX - 0.01, BX + 0.01
RAIL_GZ0 = 0.57
GUIDE_Y0 = 0.94
HOLDER_Z = 0.585
HOLDER_TOP_Y, HOLDER_BOTTOM_Y = 2.285, 0.955

N = 18
PITCH = 0.08
SLAT_Y0 = 2.405
SLAT_Z = 0.565
SLAT_T = 0.00045
BEAD_R = 0.002
SLAT_LINE = [(-0.045, 0.008), (-0.038, 0.0), (-0.019, 0.0015), (0.0, 0.002), (0.019, 0.0015), (0.038, 0.0), (0.045, -0.008)]
PIN_R, PIN_Z1 = 0.004, 0.588

RAIL_Y, RAIL_H, RAIL_W = 0.985, 0.02, 0.09
RAIL_Z, CAP = 0.561, 0.008
STACK = 0.006
STACK_FIRST = 0.022
LIFT_MAX = 1.347

RHO = {"masonry": 800.0, "plaster": 1200.0, "render": 1500.0, "eps": 15.0, "glass": 2500.0, "aluminium": 2800.0, "steel": 7850.0, "polyamide": 1150.0}


def r(x, n=5):
    return round(x + 0.0, n)


def slat_y(i):
    return SLAT_Y0 - PITCH * i


def sb_top(x):
    """🌧 Top surface of the sloped window sill board at X."""
    return SB_BACK_Y - SLOPE * (x - FR_X1)


def sill_board():
    """🪟 Bent 2 mm aluminium sill board (X, Y): back upstand on the frame, 5 degree fall, drip leg and return lip."""
    return [(FR_X1, 1.04), (FR_X1, sb_top(FR_X1) - SB_T), (SB_X1 - SB_T, sb_top(SB_X1 - SB_T) - SB_T), (SB_X1 - SB_T, SB_DRIP_Y + SB_T), (SB_X1 - 0.008, SB_DRIP_Y + SB_T), (SB_X1 - 0.008, SB_DRIP_Y), (SB_X1, SB_DRIP_Y), (SB_X1, sb_top(SB_X1)), (FR_X1 + SB_T, sb_top(FR_X1 + SB_T)), (FR_X1 + SB_T, 1.04)]


def sill_board_line():
    return [(FR_X1 + SB_T / 2, 1.04), (FR_X1 + SB_T / 2, sb_top(FR_X1) - SB_T / 2), (SB_X1 - SB_T / 2, sb_top(SB_X1) - SB_T / 2), (SB_X1 - SB_T / 2, SB_DRIP_Y + SB_T / 2), (SB_X1 - 0.008, SB_DRIP_Y + SB_T / 2)]


def sill_bed():
    """🧱 Insulation wedge under the sill board (X, Y)."""
    return [(FR_X1, SILL_Y), (0.0, SILL_Y), (0.0, sb_top(0.0) - SB_T), (FR_X1, sb_top(FR_X1) - SB_T)]


def frame_rail(bottom):
    """🔩 Fixed-light frame rail (X, Y): 70 x 30 mm body with the 19 x 20 mm exterior glazing stop."""
    if bottom:
        return [(FR_X0, SILL_Y), (FR_X1, SILL_Y), (FR_X1, SIGHT_Y0), (FR_STOP_X, SIGHT_Y0), (FR_STOP_X, REB_Y0), (FR_X0, REB_Y0)]
    return [(FR_X0, HEAD_Y), (FR_X0, REB_Y1), (FR_STOP_X, REB_Y1), (FR_STOP_X, SIGHT_Y1), (FR_X1, SIGHT_Y1), (FR_X1, HEAD_Y)]


def cover():
    """🧰 Cover panel (X, v above the box origin): 2 mm bent sheet, wall flange, top, fascia and hem; open below."""
    t = SHEET
    return [(0, BOX_H - FLANGE), (0, BOX_H), (BOX_D, BOX_H), (BOX_D, 0), (HEM_X, 0), (HEM_X, t), (BOX_D - t, t), (BOX_D - t, BOX_H - t), (t, BOX_H - t), (t, BOX_H - FLANGE)]


def cover_line():
    t = SHEET / 2
    return [(t, BOX_H - FLANGE), (t, BOX_H - t), (BOX_D - t, BOX_H - t), (BOX_D - t, t), (HEM_X, t)]


def head_rail():
    """🔧 Head rail (X, v): roll-formed steel U 58 x 56 mm, open below with 8 mm lips."""
    t, lip = HR_T, 0.008
    return [(HR_X0, HR_V0), (HR_X0 + lip, HR_V0), (HR_X0 + lip, HR_V0 + t), (HR_X0 + t, HR_V0 + t), (HR_X0 + t, HR_V1 - t), (HR_X1 - t, HR_V1 - t), (HR_X1 - t, HR_V0 + t), (HR_X1 - lip, HR_V0 + t), (HR_X1 - lip, HR_V0), (HR_X1, HR_V0), (HR_X1, HR_V1), (HR_X0, HR_V1)]


def head_rail_line():
    t = HR_T / 2
    return [(HR_X0 + 0.008, HR_V0 + t), (HR_X0 + t, HR_V0 + t), (HR_X0 + t, HR_V1 - t), (HR_X1 - t, HR_V1 - t), (HR_X1 - t, HR_V0 + t), (HR_X1 - 0.008, HR_V0 + t)]


def bracket():
    """🪝 Head rail bracket (X, v): 3 mm steel strap, wall plate, arm and saddle over the head rail."""
    s = 0.003
    return [(0, 0.10), (0.004, 0.10), (0.004, 0.171), (HR_X0 - s, 0.171), (HR_X0 - s, HR_V0), (HR_X0, HR_V0), (HR_X0, HR_V1), (HR_X1, HR_V1), (HR_X1, HR_V0), (HR_X1 + s, HR_V0), (HR_X1 + s, HR_V1 + s), (HR_X0 - s, HR_V1 + s), (HR_X0 - s, 0.175), (0, 0.175)]


def bracket_line():
    return [(0.002, 0.10), (0.002, 0.173), (HR_X0 - 0.0015, 0.173), (HR_X0 - 0.0015, HR_V1 + 0.0015), (HR_X1 + 0.0015, HR_V1 + 0.0015), (HR_X1 + 0.0015, HR_V0)]


def end_plate(sign=1, flange=0.014, back=SHEET):
    """🧩 Cover end plate (w, u): 2 mm web with two fixing flanges, set inside the cover."""
    t = SHEET
    pts = [(Z, back), (Z, BOX_D - t), (Z - flange, BOX_D - t), (Z - flange, BOX_D - 2 * t), (Z - t, BOX_D - 2 * t), (Z - t, back + t), (Z - flange, back + t), (Z - flange, back)]
    return pts if sign > 0 else [(-w, u) for w, u in reversed(pts)]


def guide_rail(sign=1):
    """🛤 Guide rail (w, u): extruded aluminium C 30 x 20 mm, 2 mm wall, 10 mm slot towards the blind."""
    t = 0.002
    a, b = RAIL_GX0, RAIL_GX1
    pts = [(RAIL_GZ0, a), (Z, a), (Z, b), (RAIL_GZ0, b), (RAIL_GZ0, BX + 0.005), (RAIL_GZ0 + t, BX + 0.005), (RAIL_GZ0 + t, b - t), (Z - t, b - t), (Z - t, a + t), (RAIL_GZ0 + t, a + t), (RAIL_GZ0 + t, BX - 0.005), (RAIL_GZ0, BX - 0.005)]
    return pts if sign > 0 else [(-w, u) for w, u in reversed(pts)]


def bottom_rail():
    """📏 Bottom rail (u, v about its centre): extruded aluminium 90 x 20 mm with chamfered underside."""
    a, h, c = RAIL_W / 2, RAIL_H / 2, 0.004
    return [(-a, -h + c), (-a + c, -h), (a - c, -h), (a, -h + c), (a, h), (-a, h)]


def bottom_rail_hole():
    a, h, c, t = RAIL_W / 2, RAIL_H / 2, 0.004, 0.0018
    return [(-a + t, -h + c + 0.0005), (-a + t, h - t), (a - t, h - t), (a - t, -h + c + 0.0005), (a - c - 0.0003, -h + t), (-a + c + 0.0003, -h + t)]


def _unit(p, q):
    d = math.hypot(q[0] - p[0], q[1] - p[1])
    return (q[0] - p[0]) / d, (q[1] - p[1]) / d


def _offset(line, d):
    out = []
    for i, p in enumerate(line):
        if i == 0:
            ux, uy = _unit(line[0], line[1])
            out.append((p[0] - uy * d, p[1] + ux * d))
        elif i == len(line) - 1:
            ux, uy = _unit(line[-2], line[-1])
            out.append((p[0] - uy * d, p[1] + ux * d))
        else:
            ax, ay = _unit(line[i - 1], p)
            bx, by = _unit(p, line[i + 1])
            nx, ny = -(ay + by), ax + bx
            n = math.hypot(nx, ny)
            nx, ny = nx / n, ny / n
            cos = nx * -ay + ny * ax
            out.append((p[0] + nx * d / cos, p[1] + ny * d / cos))
    return out


def _bead(centre, direction, half, radius, steps=8):
    a = math.atan2(direction[1], direction[0])
    neck = math.asin(half / radius)
    start, end = math.pi - neck, -(math.pi - neck)
    return [(centre[0] + radius * math.cos(a + start + (end - start) * k / steps), centre[1] + radius * math.sin(a + start + (end - start) * k / steps)) for k in range(steps + 1)]


def slat(thickness=SLAT_T, bead=BEAD_R, line=None):
    """🍃 Z 90 lamella section (u towards the outside, v up): cambered web, inner edge up, outer edge down, rolled beads on both edges."""
    line = line or SLAT_LINE
    half = thickness / 2
    left = _offset(line, half)
    right = _offset(line, -half)
    if bead:
        end = _bead(line[-1], _unit(line[-2], line[-1]), half, bead)
        start = _bead(line[0], _unit(line[1], line[0]), half, bead)
        pts = left[1:-1] + end + right[-2:0:-1] + start
    else:
        pts = left + right[::-1]
    area = sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))
    return pts if area > 0 else pts[::-1]


def developed(line=None):
    line = line or SLAT_LINE
    return sum(math.hypot(line[i + 1][0] - line[i][0], line[i + 1][1] - line[i][1]) for i in range(len(line) - 1))


def lifted(lift):
    """⬆ Rise of every slide and of the bottom rail for a bottom-rail lift: the rail collects the lamellae from below into a stack of 6 mm pitch."""
    lift = min(max(lift, 0.0), LIFT_MAX)
    rail = RAIL_Y + lift
    return [max(0.0, rail + STACK_FIRST + (N - 1 - i) * STACK - slat_y(i)) for i in range(N)], lift


def masses():
    """⚖ Mass budget (kg) per bone from the modelled parts."""
    w = 2 * Z
    block = (X_BLOCK1 - X_BLOCK0) * (SILL_Y + H - HEAD_Y) * w * RHO["masonry"]
    plaster = 0.015 * (SILL_Y + H - HEAD_Y) * w * RHO["plaster"]
    render = 0.01 * (SILL_Y + H - HEAD_Y) * w * RHO["render"]
    eps = 0.07 * (SILL_Y + H - HEAD_Y) * w * RHO["eps"]
    glass = 2 * PANE * (GL_Y1 - GL_Y0) * 2 * GL_Z * RHO["glass"]
    frame = 1.6 * 2 * (w + HEAD_Y - SILL_Y)
    board = 0.24 * w * SB_T * RHO["aluminium"]
    facade = block + plaster + render + eps + glass + frame + board
    cover_kg = (BOX_H + BOX_D + FLANGE + 0.01) * w * SHEET * RHO["aluminium"] + 2 * 0.17 * BOX_D * BOX_H * 0 + 2 * BOX_D * BOX_H * SHEET * RHO["aluminium"]
    rail_kg = (2 * HR_H + HR_W + 0.016) * 2 * HR_Z * HR_T * RHO["steel"]
    motor = 1.8
    shaft = 0.89 * 0.82
    guides = 2 * 0.00019 * (BOX_Y0 - GUIDE_Y0) * RHO["aluminium"]
    hardware = 0.6
    box = cover_kg + rail_kg + motor + shaft + guides + hardware
    bottom = 0.0004 * 2 * SLAT_Z * RHO["aluminium"] + 0.06
    lam = developed() * SLAT_T * 2 * SLAT_Z * RHO["aluminium"] + 0.01
    return {"facade": facade, "box": box, "bottom-rail": bottom, "slat": lam, "parts": {"block": block, "plaster": plaster, "render": render, "eps": eps, "glass": glass, "frame": frame, "board": board, "cover": cover_kg, "head-rail": rail_kg, "motor": motor, "shaft": shaft, "guides": guides, "hardware": hardware}}


if __name__ == "__main__":
    m = masses()
    print({k: (round(v, 3) if not isinstance(v, dict) else {a: round(b, 2) for a, b in v.items()}) for k, v in m.items()})
    print("total", round(m["facade"] + m["box"] + m["bottom-rail"] + N * m["slat"], 2))
    print("developed", developed(), "slat area 2 sides", 2 * developed() * 2 * SLAT_Z, "x", N, "=", N * 2 * developed() * 2 * SLAT_Z)
    for lift in (0, 0.27, LIFT_MAX):
        dy, _ = lifted(lift)
        print(lift, [round(slat_y(i) + d, 4) for i, d in enumerate(dy)])
    print(len(slat()), [(round(a, 5), round(b, 5)) for a, b in slat()][:12])
