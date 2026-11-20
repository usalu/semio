"""🍽 Round 3, furniture/dining-table: every body solid of every scale, stated by hand from the measures of the kit.

1:50 shows the whole construction: the 30 mm oak top halves with their 15 mm underside bevel on the three free edges and the square joint edge, the 30 mm shadow gap, the legs tapered on all four faces from 70 mm under the apron to 45 mm at the floor, the 25 x 80 mm aprons tenoned into the legs, the oak blocks in the gap that hold the top, the 25 x 12 mm steel tubes on the long aprons with the stationary 21 x 8 mm centre bars of the telescopic slide running in their bores, and the leaf with the same bevel on its long edges, stored in the gap.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_dining_table_kit", os.path.join(HERE, "r3-dining-table-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

HALF_Z = K.DEPTH / 2


def top(prefix, bone, hand, bevel):
    """🪵 A top half: the 15 mm upper slab and, with `bevel`, the lower 15 mm that slopes in by 15 mm on the three free edges; without it one 30 mm slab."""
    pivot = (K.HALF, 0.0)
    if not bevel:
        return [K.plate(prefix, bone, K.OAK, pivot, K.top_outline(hand, 0.0), (K.TOP_Y0, K.TOP_Y1))]
    upper = K.plate(prefix, bone, K.OAK, pivot, K.top_outline(hand, 0.0), (K.CREASE_Y, K.TOP_Y1))
    sz = HALF_Z / (HALF_Z - K.BEVEL)
    sx = K.HALF / (K.HALF - K.BEVEL)
    lower = K.plate(prefix + "-bevel", bone, K.OAK, pivot, K.top_outline(hand, K.BEVEL), (K.TOP_Y0, K.CREASE_Y), taper=(sz, sx))
    return [upper, lower]


def leaf(bevel, thick=K.LEAF_T, half_z=HALF_Z):
    """🪵 The leaf: 0.60 x 0.90 m, with the bevel of the top on its two long edges and square ends that meet the halves; at 1:500 a block `thick` deep and `half_z` wide that stays inside the apron frame when stored."""
    pivot = (K.LEAF_X0 + 0.3, 0.0)
    full = [(-half_z, -0.3), (-half_z, 0.3), (half_z, 0.3), (half_z, -0.3)]
    y0 = K.GAP_Y0 + K.LEAF_T - thick
    if not bevel:
        return [K.plate("leaf", "leaf", K.OAK, pivot, full, (y0, K.GAP_Y0 + K.LEAF_T), tone=0.65)]
    narrow = [(-(HALF_Z - K.BEVEL), -0.3), (-(HALF_Z - K.BEVEL), 0.3), (HALF_Z - K.BEVEL, 0.3), (HALF_Z - K.BEVEL, -0.3)]
    upper = K.plate("leaf", "leaf", K.OAK, pivot, full, (K.GAP_Y0 + 0.015, K.GAP_Y0 + K.LEAF_T), tone=0.65)
    lower = K.plate("leaf-bevel", "leaf", K.OAK, pivot, narrow, (K.GAP_Y0, K.GAP_Y0 + 0.015), taper=(HALF_Z / (HALF_Z - K.BEVEL), 1.0), tone=0.65)
    return [upper, lower]


def tapered_leg(sid, bone, hand, foot, shoulder, taper_y, top_y=K.TOP_Y0):
    """🦵 The two legs of a half (left and right side), square, tapered on all faces from `shoulder` to `foot` half widths."""
    solid = K.leg(sid, bone, K.wx(K.LEG_X, hand), K.LEG_Z, [(0.0, foot), (taper_y, shoulder), (top_y, shoulder)], tone=0.6)
    return K.repeat(solid, 2, dw=-2 * K.LEG_Z)


def aprons(prefix, bone, hand, y0, y1, t):
    """🧱 The long aprons (both sides, tenoned 20 mm into the legs) and the end apron between the legs."""
    x = K.span(K.LEG_X + K.LEG_TOP - K.TENON, K.HALF, hand)
    side = K.box(prefix + "-side", bone, K.OAK, x, (y0, y1), (K.APRON_Z1 - t, K.APRON_Z1), tone=0.55)
    K.repeat(side, 2, dw=-(2 * K.APRON_Z1 - t))
    ex = K.span(K.END_X0, K.END_X0 + t, hand)
    reach = K.LEG_Z - K.LEG_TOP + K.TENON
    end = K.box(prefix + "-end", bone, K.OAK, ex, (y0, y1), (-reach, reach), tone=0.55)
    return [side, end]


def blocks(prefix, bone, hand):
    """🟫 The oak blocks in the shadow gap that join the top to the aprons: two per long apron outside the leaf bay, two on the end apron."""
    out = []
    for k, x0 in enumerate(K.BLOCK_X):
        b = K.box(f"{prefix}-{k}", bone, K.OAK, K.span(x0, x0 + K.BLOCK_L, hand), (K.GAP_Y0, K.TOP_Y0), (K.BLOCK_Z0, K.BLOCK_Z1), tone=0.45)
        out.append(K.repeat(b, 2, dw=-(K.BLOCK_Z0 + K.BLOCK_Z1)))
    e = K.box(prefix + "-end", bone, K.OAK, K.span(0.12, K.END_X1, hand), (K.GAP_Y0, K.TOP_Y0), (K.END_BLOCK_Z - K.BLOCK_L / 2, K.END_BLOCK_Z + K.BLOCK_L / 2), tone=0.45)
    out.append(K.repeat(e, 2, dw=-2 * K.END_BLOCK_Z))
    return out


def runners(prefix, bone, hand):
    """🛤 The outer tubes of the telescopic slide on the half, screwed to the inner face of both long aprons; the outer end butts against the leg, the centre bar runs in the bore."""
    x = K.span(K.RUN_X0, K.RUN_X1, hand)
    out = []
    for s, side in K.SIDES:
        outer, bore = K.tube(side)
        solid = K.along_x(f"{prefix}-{s}", bone, K.STEEL, outer, x)
        ox, oy = K.ORIGIN[bone]
        solid["holes"] = [[[K.r6(y - oy), K.r6(z)] for y, z in bore]]
        out.append(solid)
    return out


def centre_bars(sid, y0, y1, z0, z1):
    """🔩 The stationary centre bars of the synchronised slide, one per side, 1.00 m long."""
    bar = K.box(sid, "frame", K.STEEL, (K.CENTRE_X0, K.CENTRE_X1), (y0, y1), (z0, z1))
    return K.repeat(bar, 2, dw=-(z0 + z1))


def body50():
    """🔍 1:50, smallest feature 7.5 mm."""
    out = []
    for name, bone, hand in K.HALVES:
        p = name[0]
        out += top("top-" + p, bone, hand, True)
        out.append(tapered_leg("leg-" + p, bone, hand, K.LEG_FOOT, K.LEG_TOP, K.TAPER_Y))
        out += aprons("apron-" + p, bone, hand, K.APRON_Y0, K.APRON_Y1, K.APRON_T)
        out += blocks("block-" + p, bone, hand)
        out += runners("runner-" + p, bone, hand)
    out += leaf(True)
    out.append(centre_bars("centre-bar", K.CENTRE_Y0, K.CENTRE_Y1, K.CENTRE_Z0, K.CENTRE_Z1))
    return out


def body100():
    """🔎 1:100, smallest feature 15 mm: square-edged top halves, aprons, tapered legs, the leaf, one steel slide per side; blocks and runners merged away."""
    out = []
    for name, bone, hand in K.HALVES:
        p = name[0]
        out += top("top-" + p, bone, hand, False)
        out.append(tapered_leg("leg-" + p, bone, hand, K.LEG_FOOT, K.LEG_TOP, K.TAPER_Y))
        out += aprons("apron-" + p, bone, hand, K.APRON_Y0, K.APRON_Y1, K.APRON_T)
    out += leaf(False)
    out.append(centre_bars("slide", 0.667, 0.687, 0.357, 0.375))
    return out


def body200():
    """🗺 1:200, smallest feature 30 mm: top halves, 30 mm aprons, straight 60 mm legs, the leaf and one 30 mm slide per side."""
    out = []
    for name, bone, hand in K.HALVES:
        p = name[0]
        out += top("top-" + p, bone, hand, False)
        x = K.span(K.LEG_X - 0.03, K.LEG_X + 0.03, hand)
        leg = K.box("leg-" + p, bone, K.OAK, x, (0.0, K.TOP_Y0), (K.LEG_Z - 0.03, K.LEG_Z + 0.03), tone=0.6)
        out.append(K.repeat(leg, 2, dw=-2 * K.LEG_Z))
        sx = K.span(K.LEG_X + 0.03 - K.TENON, K.HALF, hand)
        side = K.box("apron-" + p + "-side", bone, K.OAK, sx, (K.APRON_Y0, K.APRON_Y1), (K.APRON_Z1 - 0.03, K.APRON_Z1), tone=0.55)
        out.append(K.repeat(side, 2, dw=-(2 * K.APRON_Z1 - 0.03)))
        out.append(K.box("apron-" + p + "-end", bone, K.OAK, K.span(K.END_X0, K.END_X0 + 0.03, hand), (K.APRON_Y0, K.APRON_Y1), (-(K.LEG_Z - 0.01), K.LEG_Z - 0.01), tone=0.55))
    out += leaf(False)
    out.append(centre_bars("slide", 0.66, K.GAP_Y0, 0.34, K.APRON_Z1 - 0.03))
    return out


def body500():
    """🌍 1:500, smallest feature 75 mm: per half one 75 mm slab for top and gap and a U frame of aprons below it, two 75 mm legs; the 75 mm leaf block (0.78 m wide, the 60 mm per side it lacks is below the 75 mm feature) lies in the slab and the apron frame when stored."""
    out = []
    for name, bone, hand in K.HALVES:
        p = name[0]
        out.append(K.plate("slab-" + p, bone, K.OAK, (K.HALF, 0.0), K.top_outline(hand, 0.0), (K.SLAB_Y0, K.TOP_Y1)))
        s = -hand
        u = [(0.4, 0.0), (0.4, s * 0.7), (-0.4, s * 0.7), (-0.4, 0.0), (-0.325, 0.0), (-0.325, s * 0.625), (0.325, s * 0.625), (0.325, 0.0)]
        out.append(K.plate("frame-" + p, bone, K.OAK, (K.HALF, 0.0), u if hand == -1 else list(reversed(u)), (K.FRAME500_Y0, K.SLAB_Y0), tone=0.55))
        x = K.span(K.LEG_X - 0.0375, K.LEG_X + 0.0375, hand)
        leg = K.box("leg-" + p, bone, K.OAK, x, (0.0, K.FRAME500_Y0 + 0.01), (K.LEG_Z - 0.0375, K.LEG_Z + 0.0375), tone=0.6)
        out.append(K.repeat(leg, 2, dw=-2 * K.LEG_Z))
    out += leaf(False, thick=0.075, half_z=K.LEAF500_Z)
    return out
