"""🪟 Real dimensions of the timber tilt window (bottom-hung sash in a lining frame) and the frames of its two bones.

World: X through the wall (+ exterior, room at -X), Y up, Z lateral (+ = left of the asset).
Bone `sill`: origin (-0.15, 0.05), u = X + 0.15, v = Y - 0.05, w = Z.
Bone `sash`: origin = tilt-hinge axis HX, HY; u = Y - HY, v = -(X - HX), w = Z (u up, v towards the room).
References: DIN 68121-1 (timber window profiles, Euro groove 16 x 12 mm, rebate clearance 12 mm),
EN 572-1 / EN 1991-1-1 (glass 25 kN/m3), EN 338 (C24 420 kg/m3), Winkhaus/Baunetz Wissen (tilt opening 70-130 mm).
"""

import math

T_BOARD = 0.05
X0, X1 = -0.15, 0.15
Z_OUT, Z_IN = 0.45, 0.40
GAP = 0.012
SASH_X0, SASH_X1 = -0.045, 0.045
SASH_Y0 = 0.05 + GAP
SASH_H = 1.2
SASH_Y1 = SASH_Y0 + SASH_H
SASH_Z = Z_IN - GAP
HEAD_Y0 = SASH_Y1 + GAP
H = HEAD_Y0 + T_BOARD
STILE = 0.07
RAIL_TOP = 0.07
RAIL_BOTTOM = 0.08
REBATE = 0.018
LIP_X = 0.005
GROOVE_X, GROOVE_D = 0.006, 0.010
GLASS_IN = 0.014
PANE = 0.004
CAVITY = 0.016
GLASS_X0 = LIP_X - 2 * PANE - CAVITY
SEAL = 0.012
EDGE = 0.065
BEAD_X1 = GLASS_X0
CHAMFER = 0.008
STOP_X0, STOP_X1 = SASH_X1, SASH_X1 + 0.025
STOP_W = 0.03
STOP_SILL_Y1 = 0.05 + STOP_W
STOP_HEAD_Y0 = HEAD_Y0 - STOP_W
WEATHER = [(SASH_X1, STOP_SILL_Y1), (0.083, STOP_SILL_Y1), (0.083, STOP_SILL_Y1 + 0.012), (SASH_X1, SASH_Y0 + RAIL_BOTTOM)]
WEATHER_Z = Z_IN - STOP_W
SILL_DROP = 0.010
DRIP_X0, DRIP_X1, DRIP_D = 0.126, 0.134, 0.010
HINGE_R = 0.008
HX, HY = SASH_X0 - HINGE_R, 0.05 + HINGE_R
HINGE_Z, HINGE_L = 0.30, 0.06
LEAF = 0.008
GRIP_Y = SASH_Y1 - RAIL_TOP / 2
GRIP_R, GRIP_REACH, GRIP_HALF = 0.006, 0.033, 0.05
KEEPER_U = 1.15
KEEPER_V = -(0.0 - HX)
KEEPER_R = 0.004
ARC_R = math.hypot(KEEPER_U, KEEPER_V)
ARC_HALF, SLOT_HALF = 0.012, 0.005
ARC_Z0, ARC_Z1 = Z_IN - 0.008, Z_IN
TILT_HALF, TILT_FULL = 3.0, 6.0

GLASS_Z = SASH_Z - STILE + GLASS_IN
SIGHT_Z = SASH_Z - STILE
GLASS_Y0 = SASH_Y0 + RAIL_BOTTOM - GLASS_IN
GLASS_Y1 = SASH_Y1 - RAIL_TOP + GLASS_IN
SIGHT_Y0 = SASH_Y0 + RAIL_BOTTOM
SIGHT_Y1 = SASH_Y1 - RAIL_TOP
CENTRE_Z = SIGHT_Z - EDGE
CENTRE_Y0, CENTRE_Y1 = SIGHT_Y0 + EDGE, SIGHT_Y1 - EDGE

RHO_TIMBER, RHO_GLASS, RHO_SEAL, RHO_STEEL = 420.0, 2500.0, 1500.0, 7850.0


def r(x, n=4):
    return round(x + 0.0, n)


def su(Y):
    return r(Y - HY)


def sv(X):
    return r(-(X - HX))


def lu(X):
    return r(X - X0)


def lv(Y):
    return r(Y - 0.05)


def sash_pt(X, Y):
    return [su(Y), sv(X)]


def sill_pt(X, Y):
    return [lu(X), lv(Y)]


def world_of_sash(u, v, theta_deg=0.0):
    t = math.radians(theta_deg)
    ax, ay = -math.sin(t), math.cos(t)
    bx, by = -math.cos(t), -math.sin(t)
    return HX + u * ax + v * bx, HY + u * ay + v * by


def poly_area(points):
    return abs(sum(points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1] for i in range(len(points)))) / 2


def poly_centroid(points):
    a = 0.0
    cx = cy = 0.0
    for i in range(len(points)):
        x0, y0 = points[i]
        x1, y1 = points[(i + 1) % len(points)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    return cx / (3 * a), cy / (3 * a)


def rail_section(y0, y1, glass_side_top):
    """🧱 Rail section (X, Y) with the glass rebate on the room side of its glass edge."""
    if glass_side_top:
        return [(SASH_X0, y0), (SASH_X1, y0), (SASH_X1, y1), (LIP_X, y1), (LIP_X, y1 - REBATE), (SASH_X0, y1 - REBATE)]
    return [(SASH_X0, y1), (SASH_X1, y1), (SASH_X1, y0), (LIP_X, y0), (LIP_X, y0 + REBATE), (SASH_X0, y0 + REBATE)]


def stile_section():
    """🪵 Left stile section (X, Z): rebate on the room side of the glass edge, Euro groove on the outer side face."""
    z0, z1 = SIGHT_Z, SASH_Z
    return [(SASH_X0, z0 + REBATE), (SASH_X0, z1), (-GROOVE_X, z1), (-GROOVE_X, z1 - GROOVE_D), (GROOVE_X, z1 - GROOVE_D), (GROOVE_X, z1), (SASH_X1, z1), (SASH_X1, z0), (LIP_X, z0), (LIP_X, z0 + REBATE)]


def bead_section(top):
    """🪚 Glazing bead (X, Y) with an 8 mm chamfer on its visible arris."""
    if top:
        y0, y1 = SIGHT_Y1, SIGHT_Y1 + REBATE
        return [(SASH_X0, y1), (SASH_X0, y0 + CHAMFER), (SASH_X0 + CHAMFER, y0), (BEAD_X1, y0), (BEAD_X1, y1)]
    y0, y1 = SIGHT_Y0 - REBATE, SIGHT_Y0
    return [(SASH_X0, y0), (SASH_X0, y1 - CHAMFER), (SASH_X0 + CHAMFER, y1), (BEAD_X1, y1), (BEAD_X1, y0)]


def sill_section():
    """🌧 Sill board (X, Y): weathered exterior top falling 10 mm and a drip groove underneath."""
    return [(X0, 0.0), (DRIP_X0, 0.0), (DRIP_X0, DRIP_D), (DRIP_X1, DRIP_D), (DRIP_X1, 0.0), (X1, 0.0), (X1, 0.05 - SILL_DROP), (STOP_X1, 0.05), (X0, 0.05)]


def arc_points(radius, a0_deg, a1_deg, n):
    """⌒ Points (X, Y) of a circle about the tilt axis, angles from vertical towards +X."""
    out = []
    for i in range(n + 1):
        a = math.radians(a0_deg + (a1_deg - a0_deg) * i / n)
        out.append((HX + radius * math.sin(a), HY + radius * math.cos(a)))
    return out


KEEPER_DEG = math.degrees(math.atan2(-KEEPER_V, KEEPER_U))
PIN_DEG = math.degrees(KEEPER_R / ARC_R) + 0.05
SLOT_A0, SLOT_A1 = KEEPER_DEG + PIN_DEG, KEEPER_DEG - TILT_FULL - PIN_DEG
BAR_A0, BAR_A1 = SLOT_A0 + 1.0, SLOT_A1 - 0.75


def arc_bar():
    outer = arc_points(ARC_R + ARC_HALF, BAR_A0, BAR_A1, 16)
    inner = arc_points(ARC_R - ARC_HALF, BAR_A1, BAR_A0, 16)
    return outer + inner


def arc_slot():
    outer = arc_points(ARC_R + SLOT_HALF, SLOT_A0, SLOT_A1, 12)
    inner = arc_points(ARC_R - SLOT_HALF, SLOT_A1, SLOT_A0, 12)
    return outer + inner


def sash_parts():
    """⚖ (name, kg, u, v) of every sash part, u/v of its centre of mass in the sash frame."""
    parts = []
    st = stile_section()
    a = poly_area(st)
    cx, _ = poly_centroid(st)
    parts.append(("stiles (2)", 2 * a * SASH_H * RHO_TIMBER, su((SASH_Y0 + SASH_Y1) / 2), sv(cx)))
    rail_len = 2 * (SIGHT_Z + REBATE)
    for name, sec in (("bottom rail", rail_section(SASH_Y0, SIGHT_Y0, True)), ("top rail", rail_section(SIGHT_Y1, SASH_Y1, False))):
        a = poly_area(sec)
        cx, cy = poly_centroid(sec)
        parts.append((name, a * rail_len * RHO_TIMBER, su(cy), sv(cx)))
    for name, top in (("bottom bead", False), ("top bead", True)):
        sec = bead_section(top)
        a = poly_area(sec)
        cx, cy = poly_centroid(sec)
        parts.append((name, a * 2 * SIGHT_Z * RHO_TIMBER, su(cy), sv(cx)))
    a = poly_area(bead_section(False))
    cx, _ = poly_centroid(bead_section(False))
    parts.append(("side beads (2)", 2 * a * (SIGHT_Y1 - SIGHT_Y0) * RHO_TIMBER, su((SIGHT_Y0 + SIGHT_Y1) / 2), sv(cx)))
    a = poly_area(WEATHER)
    cx, cy = poly_centroid(WEATHER)
    parts.append(("weather bar", a * 2 * WEATHER_Z * RHO_TIMBER, su(cy), sv(cx)))
    gw, gh = 2 * GLASS_Z, GLASS_Y1 - GLASS_Y0
    parts.append(("panes 2 x 4 mm", 2 * gw * gh * PANE * RHO_GLASS, su((GLASS_Y0 + GLASS_Y1) / 2), sv(GLASS_X0 + PANE + CAVITY / 2)))
    ring = gw * gh - (gw - 2 * SEAL) * (gh - 2 * SEAL)
    parts.append(("edge seal", ring * CAVITY * RHO_SEAL, su((GLASS_Y0 + GLASS_Y1) / 2), sv(GLASS_X0 + PANE + CAVITY / 2)))
    hinge = 2 * (math.pi * HINGE_R ** 2 * HINGE_L + HINGE_L * LEAF * 0.06) * RHO_STEEL
    parts.append(("tilt hinges (2)", hinge, su(HY + 0.02), sv(HX)))
    grip = (math.pi * GRIP_R ** 2 * (2 * GRIP_HALF + 2 * GRIP_REACH)) * RHO_STEEL
    parts.append(("pull grip", grip, su(GRIP_Y), sv(SASH_X0 - GRIP_REACH / 2)))
    pins = 2 * math.pi * KEEPER_R ** 2 * 0.011 * RHO_STEEL
    parts.append(("stay pins (2)", pins, KEEPER_U, KEEPER_V))
    return parts


def sash_mass():
    parts = sash_parts()
    kg = sum(p[1] for p in parts)
    return kg, sum(p[1] * p[2] for p in parts) / kg, sum(p[1] * p[3] for p in parts) / kg, parts


def lining_mass():
    sill = poly_area(sill_section()) * 2 * Z_IN
    head = T_BOARD * (X1 - X0) * 2 * Z_IN
    jambs = 2 * T_BOARD * (X1 - X0) * H
    stops = (STOP_X1 - STOP_X0) * STOP_W * (2 * (HEAD_Y0 - 0.05) + 2 * 2 * (Z_IN - STOP_W))
    timber = (sill + head + jambs + stops) * RHO_TIMBER
    arcs = 2 * (poly_area(arc_bar()) - poly_area(arc_slot())) * (ARC_Z1 - ARC_Z0) * RHO_STEEL
    leaves = 2 * HINGE_L * LEAF * 0.06 * RHO_STEEL
    return timber + arcs + leaves


def second_moment_x(points):
    """📐 Area, centroid X and second moment about the centroidal axis normal to X of a polygon (X, Z)."""
    a = 0.0
    sx = sxx = 0.0
    for i in range(len(points)):
        x0, y0 = points[i]
        x1, y1 = points[(i + 1) % len(points)]
        c = x0 * y1 - x1 * y0
        a += c / 2
        sx += (x0 + x1) * c / 6
        sxx += (x0 * x0 + x0 * x1 + x1 * x1) * c / 12
    cx = sx / a
    return abs(a), cx, abs(sxx - a * cx * cx)


def stay_angle_check():
    """🔩 The keeper pin stays on the arc centre line in every tilt (concentric with the tilt axis)."""
    out = []
    for t in (0.0, TILT_HALF, TILT_FULL):
        x, y = world_of_sash(KEEPER_U, KEEPER_V, t)
        out.append((t, round(math.hypot(x - HX, y - HY) - ARC_R, 9), round(x, 4), round(y, 4)))
    return out


if __name__ == "__main__":
    kg, cu, cv, parts = sash_mass()
    for p in parts:
        print("%-18s %7.3f kg  u %.3f v %.3f" % p)
    print("sash %.2f kg com u %.4f v %.4f; lining+hardware %.2f kg; total %.2f" % (kg, cu, cv, lining_mass(), kg + lining_mass()))
    print("H", r(H), "hinge", HX, HY, "arc R", ARC_R, "keeper deg", KEEPER_DEG, "slot", SLOT_A0, SLOT_A1, "bar", BAR_A0, BAR_A1)
    print("arc X range", min(p[0] for p in arc_bar()), max(p[0] for p in arc_bar()))
    print(stay_angle_check())
    for t in (0, TILT_HALF, TILT_FULL):
        print("tilt", t, "top gap at head (interior corner X)", round(world_of_sash(su(SASH_Y1), sv(SASH_X0), t)[0], 4))
