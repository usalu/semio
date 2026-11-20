"""🪵 Round 3, furniture/single-bed: every body solid of every scale, stated by hand in world metres (x head to foot, y up, z lateral) and written bone-local by the kit.

1:50 is the bed as it is built: four eased posts with chamfered tops, frame-and-panel head and foot ends with V-jointed boards, side rails with ledgers, a cross beam on a centre leg, the sprung slatted base (base rails, lifting head frame on two bolts, 28 cambered slats in rubber caps) and one piped mattress in three parts that ride on the three slat sections. The coarser scales are authored anew for 15, 30 and 75 mm.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_single_bed_kit", os.path.join(HERE, "r3-single-bed-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

PINE, BIRCH, QUILT, RUBBER, STEEL = K.PINE, K.BIRCH, K.QUILT, K.RUBBER, K.STEEL


def post(sid, x0, top, ease, chamfer, steps=3):
    """🪵 A 60 mm post pair: eased square shaft and a chamfered top, the left one repeated to the right."""
    half = K.POST / 2
    square = K.rounded((-half, half), (-half, half), [ease] * 4, steps)
    shaft = K.along_y(sid, "frame", PINE, [(z, x) for z, x in square], (0, top - chamfer))
    shaft["transform"] = {"translation": [x0 + half, 0, K.POST_Z]}
    K.repeat(shaft, 2, dw=-2 * K.POST_Z)
    cap = K.along_y(sid + "-top", "frame", PINE, [(z, x) for z, x in square], (top - chamfer, top), taperTo=[1, (K.POST - 2 * chamfer) / K.POST])
    cap["transform"] = {"translation": [x0 + half, 0, K.POST_Z]}
    K.repeat(cap, 2, dw=-2 * K.POST_Z)
    return [shaft, cap]


def v_board(sid, x_mid, thick, y, pitch, groove_w, groove_d):
    """🪚 A panel of V-jointed boards between the posts: the horizontal section with a V at every board joint on both faces, pushed up from y[0] to y[1]."""
    front, back = x_mid + thick / 2, x_mid - thick / 2
    joints = [k * pitch for k in range(-4, 5)]
    profile = [(-K.BOARD_Z, back), (-K.BOARD_Z, front)]
    for z in joints:
        profile += [(z - groove_w / 2, front), (z, front - groove_d), (z + groove_w / 2, front)]
    profile += [(K.BOARD_Z, front), (K.BOARD_Z, back)]
    for z in reversed(joints):
        profile += [(z + groove_w / 2, back), (z, back + groove_d), (z - groove_w / 2, back)]
    return K.along_y(sid, "frame", PINE, profile, y, tone=0.55)


def slat(sid, bone, x, half, sections=9):
    """🌈 One sprung slat: 38 x 8 mm birch laminate, cambered 15 mm, from cap to cap."""
    stations = []
    for k in range(sections):
        z = -half + 2 * half * k / (sections - 1)
        y = K.SLAT_END_Y + K.SLAT_CAMBER * (1 - (z / half) ** 2)
        stations.append((x, y, z, K.SLAT_W / 2, K.SLAT_T / 2, 8))
    return K.loft(sid, bone, BIRCH, stations)


def caps(sid, bone, x, z):
    """🧽 The rubber cap that holds a slat end on its rail."""
    cx, cy, cz = K.CAP
    return K.box(sid, bone, RUBBER, (x - cx / 2, x + cx / 2), (K.BASE_Y1, K.BASE_Y1 + cy), (z - cz / 2, z + cz / 2), radius=0.005)


END_50 = [(0.0, 0.045, 0.035), (0.004, 0.027, 0.020), (0.012, 0.013, 0.009), (0.025, 0.004, 0.003), (0.045, 0.0, 0.0)]


def mattress_part(sid, bone, x0, x1, end, shape, inset=0.0, material=QUILT, tone=None):
    """🛏 One part of the mattress as a loft along the bed: the full section 0.90 x 0.20 m (superellipse) between x0 and x1; `end` -1 rounds the head end, +1 the foot end, 0 neither. `shape` lists (distance from the end, shrink of the half width, shrink of the half height)."""
    half_w, half_h = K.MAT_Z - inset, K.MAT_T / 2 - inset
    yc = K.MAT_Y0 + K.MAT_T / 2
    full = lambda x: (x, yc, 0, half_w, half_h, K.MAT_E)
    rounded_end = [(d + inset, half_w - dw, half_h - dh) for d, dw, dh in shape]
    lip = 0.003
    if end == -1:
        stations = [(x0 + d, yc, 0, w, h, K.MAT_E) for d, w, h in rounded_end] + [full(x0 + rounded_end[-1][0] + lip), full(x1 - lip), full(x1)]
    elif end == 1:
        stations = [full(x0), full(x0 + lip), full(x1 - rounded_end[-1][0] - lip)] + [(x1 - d, yc, 0, w, h, K.MAT_E) for d, w, h in reversed(rounded_end)]
    else:
        stations = [full(x0), full(x0 + lip), full(x1 - lip), full(x1)]
    return K.loft(sid, bone, material, stations, tone=tone)


def piping(sid, bone, x_joint, x_end, y, side, radius=0.004):
    """🧵 Half of a piped seam of an end part of the mattress: along the long edge from the joint, round the corner and across the end to the centre line."""
    toward = 1 if x_end > x_joint else -1
    z_run, x_face, corner = 0.447, x_end - toward * 0.002, 0.045
    cx, cz = x_face - toward * corner, z_run - corner
    path = [(x_joint, z_run), (cx, z_run)]
    for degrees in (30, 60):
        a = math.radians(degrees)
        path.append((cx + toward * corner * math.sin(a), cz + corner * math.cos(a)))
    path += [(x_face, cz), (x_face, 0.0)]
    solid = K.loft(sid, bone, QUILT, [(x, y, side * z, radius, radius, 2) for x, z in path], tone=0.7, role="detail")
    return solid


def piping_run(sid, bone, x0, x1, y, side, radius=0.004):
    """🧶 The piped seam along a long edge of the middle part."""
    solid = K.loft(sid, bone, QUILT, [(x0, y, side * 0.447, radius, radius, 2), (x1, y, side * 0.447, radius, radius, 2)], tone=0.7, role="detail")
    return solid


def body50():
    """🔍 1:50, smallest feature 7.5 mm."""
    s = []
    s += post("post-head", 0.0, K.HEAD_H, 0.005, 0.012)
    s += post("post-foot", K.LENGTH - K.POST, K.FOOT_H, 0.005, 0.012)
    top_rail = lambda x0, y0, y1: K.rounded((x0, x0 + K.RAIL_T), (y0, y1), [0, 0, 0.01, 0.01])
    plain_rail = lambda x0, y0, y1: [(x0, y0), (x0 + K.RAIL_T, y0), (x0 + K.RAIL_T, y1), (x0, y1)]
    board = (-K.BOARD_Z, K.BOARD_Z)
    s.append(K.along_z("head-top-rail", "frame", PINE, top_rail(0.015, 0.85, 0.93), board))
    s.append(v_board("head-panel", 0.03, 0.018, (0.29, 0.86), 0.089, 0.008, 0.003))
    s.append(K.along_z("head-low-rail", "frame", PINE, plain_rail(0.015, K.RAIL_Y0, K.RAIL_Y1), board))
    s.append(K.along_z("foot-top-rail", "frame", PINE, top_rail(2.075, 0.45, 0.53), board))
    s.append(v_board("foot-panel", 2.09, 0.018, (0.29, 0.46), 0.089, 0.008, 0.003))
    s.append(K.along_z("foot-low-rail", "frame", PINE, plain_rail(2.075, K.RAIL_Y0, K.RAIL_Y1), board))
    rail = K.rounded((K.RAIL_Y0, K.RAIL_Y1), (K.CLEAR_Z, K.CLEAR_Z + K.RAIL_T), [0.003, 0.008, 0.008, 0.003])
    s.append(K.repeat(K.along_x("side-rail", "frame", PINE, rail, (K.POST, K.LENGTH - K.POST), tone=0.55), 2, dw=-(2 * K.CLEAR_Z + K.RAIL_T)))
    s.append(K.repeat(K.box("ledger", "frame", PINE, (K.POST, K.LENGTH - K.POST), (K.LEDGER_Y1 - K.LEDGER, K.LEDGER_Y1), (K.CLEAR_Z - K.LEDGER, K.CLEAR_Z)), 2, dw=-(2 * K.CLEAR_Z - K.LEDGER)))
    s.append(K.box("cross-beam", "frame", PINE, (1.04, 1.08), (K.RAIL_Y0, K.LEDGER_Y1), (-K.CLEAR_Z, K.CLEAR_Z)))
    leg = K.along_y("leg-mid", "frame", PINE, K.rounded((-0.03, 0.03), (-0.03, 0.03), [0.005] * 4), (0, K.RAIL_Y0))
    leg["transform"] = {"translation": [1.06, 0, 0]}
    s.append(leg)

    base_step = -(K.BASE_Z0 + K.BASE_Z1)
    s.append(K.repeat(K.box("base-rail-mid", "slat-mid", BIRCH, (0.065, K.FOOT_X), (K.BASE_Y0, K.BASE_Y1), (K.BASE_Z0, K.BASE_Z1)), 2, dw=base_step))
    s.append(K.repeat(K.box("base-rail-foot", "slat-foot", BIRCH, (K.FOOT_X, 2.055), (K.BASE_Y0, K.BASE_Y1), (K.BASE_Z0, K.BASE_Z1)), 2, dw=base_step))
    s.append(K.box("base-cross-head", "slat-mid", BIRCH, (0.065, 0.09), (K.BASE_Y0, K.BASE_Y1), (-K.BASE_Z0, K.BASE_Z0)))
    s.append(K.box("base-cross-foot", "slat-foot", BIRCH, (2.03, 2.055), (K.BASE_Y0, K.BASE_Y1), (-K.BASE_Z0, K.BASE_Z0)))
    lift = K.along_z("lift-rail", "slat-head", BIRCH, K.rounded((0.10, 0.80), (K.LIFT_Y0, K.LIFT_Y1), [0, 0.02, 0.02, 0], 4), (K.LIFT_Z0, K.LIFT_Z1))
    s.append(K.repeat(lift, 2, dw=-(K.LIFT_Z0 + K.LIFT_Z1)))
    s.append(K.box("lift-cross", "slat-head", BIRCH, (0.10, 0.125), (K.LIFT_Y0, K.LIFT_Y1), (-K.LIFT_Z0, K.LIFT_Z0)))
    for hand, side in (("l", 1), ("r", -1)):
        bolt = K.turned("hinge-bolt-" + hand, "slat-mid", STEEL, [(0, 0), (0, 0.008), (0.004, 0.008), (0.004, 0.004), (0.066, 0.004), (0.066, 0)], "w", (K.HINGE_X, K.HINGE_Y, side * 0.386), role="detail")
        bolt["axis"] = [0, 0, side]
        s.append(bolt)

    head_half = (K.LIFT_Z0 + K.LIFT_Z1) / 2
    base_half = (K.BASE_Z0 + K.BASE_Z1) / 2
    for name, bone, xs, half in (("head", "slat-head", K.HEAD_SLATS, head_half), ("mid", "slat-mid", K.MID_SLATS, base_half), ("foot", "slat-foot", K.FOOT_SLATS, base_half)):
        pitch = xs[1] - xs[0]
        s.append(K.repeat(slat("slat-" + name, bone, xs[0], half), len(xs), du=pitch))
        s.append(K.repeat(caps("cap-" + name + "-l", bone, xs[0], half), len(xs), du=pitch))
        s.append(K.repeat(caps("cap-" + name + "-r", bone, xs[0], -half), len(xs), du=pitch))

    s.append(mattress_part("mat-head", "mat-head", K.MAT_X0, K.HINGE_X, -1, END_50))
    s.append(mattress_part("mat-mid", "mat-mid", K.HINGE_X, K.FOOT_X, 0, END_50))
    s.append(mattress_part("mat-foot", "mat-foot", K.FOOT_X, K.MAT_X1, 1, END_50))
    for level, y in (("top", K.MAT_Y0 + K.MAT_T - 0.022), ("bottom", K.MAT_Y0 + 0.022)):
        for hand, side in (("l", 1), ("r", -1)):
            s.append(piping(f"piping-{level}-head-{hand}", "mat-head", K.HINGE_X, K.MAT_X0, y, side))
            s.append(piping_run(f"piping-{level}-mid-{hand}", "mat-mid", K.HINGE_X, K.FOOT_X, y, side))
            s.append(piping(f"piping-{level}-foot-{hand}", "mat-foot", K.FOOT_X, K.MAT_X1, y, side))
    return s


END_100 = [(0.0, 0.04, 0.03), (0.015, 0.012, 0.008), (0.045, 0.0, 0.0)]
END_200 = [(0.0, 0.03, 0.025), (0.012, 0.01, 0.007), (0.04, 0.0, 0.0)]


def deck(sid, bone, x, half):
    """🛹 The slats of one section merged with their caps into one cambered deck (1:100)."""
    stations = []
    for k in range(5):
        z = -half + half * k / 2
        crown = 1 - (z / half) ** 2
        low, high = K.BASE_Y1 + 0.004 * crown, K.SLAT_END_Y + K.SLAT_T / 2 + K.SLAT_CAMBER * crown
        stations.append(((x[0] + x[1]) / 2, (low + high) / 2, z, (x[1] - x[0]) / 2, (high - low) / 2, 8))
    return K.loft(sid, bone, BIRCH, stations)


def body100():
    """🔎 1:100, smallest feature 15 mm: plain posts, rails and panels at their true thickness, rail and ledger as one L section, the slats of each section as one cambered deck, the mattress without piping."""
    s = []
    posts = lambda sid, x0, top: K.repeat(K.box(sid, "frame", PINE, (x0, x0 + K.POST), (0, top), (K.POST_Z - 0.03, K.POST_Z + 0.03), radius=0.008), 2, dw=-2 * K.POST_Z)
    board = (-K.BOARD_Z, K.BOARD_Z)
    s.append(posts("post-head", 0.0, K.HEAD_H))
    s.append(posts("post-foot", K.LENGTH - K.POST, K.FOOT_H))
    s.append(K.box("head-top-rail", "frame", PINE, (0.015, 0.045), (0.85, 0.93), board))
    s.append(K.box("head-panel", "frame", PINE, (0.021, 0.039), (0.30, 0.85), board, tone=0.55))
    s.append(K.box("head-low-rail", "frame", PINE, (0.015, 0.045), (K.RAIL_Y0, K.RAIL_Y1), board))
    s.append(K.box("foot-top-rail", "frame", PINE, (2.075, 2.105), (0.45, 0.53), board))
    s.append(K.box("foot-panel", "frame", PINE, (2.081, 2.099), (0.30, 0.45), board, tone=0.55))
    s.append(K.box("foot-low-rail", "frame", PINE, (2.075, 2.105), (K.RAIL_Y0, K.RAIL_Y1), board))
    for hand, side in (("l", 1), ("r", -1)):
        z0, z1, zl = side * K.CLEAR_Z, side * (K.CLEAR_Z + K.RAIL_T), side * (K.CLEAR_Z - K.LEDGER)
        angle = [(K.RAIL_Y0, z0), (K.RAIL_Y0, z1), (K.RAIL_Y1, z1), (K.RAIL_Y1, z0), (K.LEDGER_Y1, z0), (K.LEDGER_Y1, zl), (K.LEDGER_Y1 - K.LEDGER, zl), (K.LEDGER_Y1 - K.LEDGER, z0)]
        s.append(K.along_x("side-rail-" + hand, "frame", PINE, angle, (K.POST, K.LENGTH - K.POST), tone=0.55))
    s.append(K.box("cross-beam", "frame", PINE, (1.04, 1.08), (K.RAIL_Y0, K.LEDGER_Y1), (-K.CLEAR_Z, K.CLEAR_Z)))
    s.append(K.box("leg-mid", "frame", PINE, (1.03, 1.09), (0, K.RAIL_Y0), (-0.03, 0.03)))
    step = -(K.BASE_Z0 + K.BASE_Z1)
    s.append(K.repeat(K.box("base-rail-mid", "slat-mid", BIRCH, (0.065, K.FOOT_X), (K.BASE_Y0, K.BASE_Y1), (K.BASE_Z0, K.BASE_Z1)), 2, dw=step))
    s.append(K.repeat(K.box("base-rail-foot", "slat-foot", BIRCH, (K.FOOT_X, 2.055), (K.BASE_Y0, K.BASE_Y1), (K.BASE_Z0, K.BASE_Z1)), 2, dw=step))
    s.append(K.box("base-cross-head", "slat-mid", BIRCH, (0.065, 0.09), (K.BASE_Y0, K.BASE_Y1), (-K.BASE_Z0, K.BASE_Z0)))
    s.append(K.box("base-cross-foot", "slat-foot", BIRCH, (2.03, 2.055), (K.BASE_Y0, K.BASE_Y1), (-K.BASE_Z0, K.BASE_Z0)))
    s.append(K.repeat(K.box("lift-rail", "slat-head", BIRCH, (0.10, 0.79), (K.LIFT_Y0, K.LIFT_Y1), (K.LIFT_Z0, K.LIFT_Z1)), 2, dw=-(K.LIFT_Z0 + K.LIFT_Z1)))
    s.append(K.box("lift-cross", "slat-head", BIRCH, (0.10, 0.125), (K.LIFT_Y0, K.LIFT_Y1), (-K.LIFT_Z0, K.LIFT_Z0)))
    s.append(deck("slat-head", "slat-head", (0.131, 0.754), 0.415))
    s.append(deck("slat-mid", "slat-mid", (0.796, 1.378), 0.445))
    s.append(deck("slat-foot", "slat-foot", (1.408, 1.99), 0.445))
    s.append(mattress_part("mat-head", "mat-head", K.MAT_X0, K.HINGE_X, -1, END_100))
    s.append(mattress_part("mat-mid", "mat-mid", K.HINGE_X, K.FOOT_X, 0, END_100))
    s.append(mattress_part("mat-foot", "mat-foot", K.FOOT_X, K.MAT_X1, 1, END_100))
    return s


def body200():
    """🔭 1:200, smallest feature 30 mm: posts, head and foot ends as one board each, side rails, cross beam and leg, each slat section as one slab from the ledger to the slat crowns, the mattress with one rounding step."""
    s = []
    posts = lambda sid, x0, top: K.repeat(K.box(sid, "frame", PINE, (x0, x0 + K.POST), (0, top), (K.POST_Z - 0.03, K.POST_Z + 0.03)), 2, dw=-2 * K.POST_Z)
    board = (-K.BOARD_Z, K.BOARD_Z)
    s.append(posts("post-head", 0.0, K.HEAD_H))
    s.append(posts("post-foot", K.LENGTH - K.POST, K.FOOT_H))
    s.append(K.box("head-board", "frame", PINE, (0.015, 0.045), (K.RAIL_Y0, 0.93), board))
    s.append(K.box("foot-board", "frame", PINE, (2.075, 2.105), (K.RAIL_Y0, 0.53), board))
    s.append(K.repeat(K.box("side-rail", "frame", PINE, (K.POST, K.LENGTH - K.POST), (K.RAIL_Y0, K.RAIL_Y1), (K.CLEAR_Z, K.CLEAR_Z + K.RAIL_T), tone=0.55), 2, dw=-(2 * K.CLEAR_Z + K.RAIL_T)))
    s.append(K.box("cross-beam", "frame", PINE, (1.04, 1.08), (K.RAIL_Y0, K.LEDGER_Y1), (-K.CLEAR_Z, K.CLEAR_Z)))
    s.append(K.box("leg-mid", "frame", PINE, (1.03, 1.09), (0, K.RAIL_Y0), (-0.03, 0.03)))
    top = K.MAT_Y0
    s.append(K.box("slat-head", "slat-head", BIRCH, (0.065, K.HINGE_X), (K.BASE_Y0, top), (-K.BASE_Z1, K.BASE_Z1)))
    s.append(K.box("slat-mid", "slat-mid", BIRCH, (K.HINGE_X, K.FOOT_X), (K.BASE_Y0, top), (-K.BASE_Z1, K.BASE_Z1)))
    s.append(K.box("slat-foot", "slat-foot", BIRCH, (K.FOOT_X, 2.055), (K.BASE_Y0, top), (-K.BASE_Z1, K.BASE_Z1)))
    s.append(mattress_part("mat-head", "mat-head", K.MAT_X0, K.HINGE_X, -1, END_200))
    s.append(mattress_part("mat-mid", "mat-mid", K.HINGE_X, K.FOOT_X, 0, END_200))
    s.append(mattress_part("mat-foot", "mat-foot", K.FOOT_X, K.MAT_X1, 1, END_200))
    return s


def body500():
    """🛰 1:500, smallest feature 75 mm: head end and foot end as one panel each, rails and slatted base as one tray on the centre leg, the mattress as three plain blocks with soft long edges."""
    s = []
    wide = (-(K.POST_Z + 0.03), K.POST_Z + 0.03)
    s.append(K.box("head", "frame", PINE, (0, K.POST), (0, K.HEAD_H), wide))
    s.append(K.box("foot", "frame", PINE, (K.LENGTH - K.POST, K.LENGTH), (0, K.FOOT_H), wide))
    s.append(K.box("tray", "frame", PINE, (K.POST, K.LENGTH - K.POST), (K.RAIL_Y0, K.RAIL_Y1), (-(K.CLEAR_Z + K.RAIL_T), K.CLEAR_Z + K.RAIL_T), tone=0.55))
    s.append(K.box("leg-mid", "frame", PINE, (1.03, 1.09), (0, K.RAIL_Y0), (-0.03, 0.03)))
    block = lambda sid, x0, x1: K.loft(sid, sid, QUILT, [(x0, K.MAT_Y0 + K.MAT_T / 2, 0, K.MAT_Z, K.MAT_T / 2, 6), (x1, K.MAT_Y0 + K.MAT_T / 2, 0, K.MAT_Z, K.MAT_T / 2, 6)])
    s.append(block("mat-head", K.MAT_X0, K.HINGE_X))
    s.append(block("mat-mid", K.HINGE_X, K.FOOT_X))
    s.append(block("mat-foot", K.FOOT_X, K.MAT_X1))
    return s
