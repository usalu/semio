"""🪨️ Hand-chosen numbers and the derived geometry of `reclaimed-cobblestone-bag`: granite setts 9/11 laid as a segmental-arc field, packed course by course into a woven big bag on a Euro pallet.

World rest frame (the stored pose): x ahead, y up, z left (staffage convention); the pallet's back edge at x = 0, its centre plane at z = 0.

@see ../../brief.md — the asset contract of this ticket
@see https://www.din.de/ — DIN EN 1342 (setts of natural stone), DIN EN 13698-1 (Euro pallet), DIN EN ISO 21898 (FIBC)
"""
import math
from shapely.geometry import Polygon, box as sbox, Point
from shapely.ops import unary_union
from shapely import affinity

SETT = 0.100
SETT_H = 0.100
SETT_R = 0.008
GRANITE_RHO = 2650.0
ARC_R = 2.20
ARC_PITCH = 0.1025
ROW_PITCH = 0.112
SETTS_PER_ARC = 16
COURSES_PER_PATCH = 6
PATCH_ROWS = 4
FIELD_X0 = 1.50

PALLET_L, PALLET_W, PALLET_H = 1.2, 0.8, 0.144
BOARD_T, BLOCK_H = 0.022, 0.078
TIMBER_RHO = 540.0

BAG_X, BAG_Y0, BAG_H = 0.6, 0.144, 0.900
BAG_T = 0.0004
BAG_RC = 0.06
BAG_BANDS = [(0.0, 0.200, 0.475, 0.495), (0.200, 0.600, 0.495, 0.495), (0.600, 0.760, 0.495, 0.481), (0.760, 0.900, 0.481, 0.481)]
BAG_COARSE_HW = 0.490
PP_RHO = 910.0
BAG_Z = -0.48

LOOP_X = (0.30, 0.90)
LOOP_T = 0.003
LOOP_BAND = 0.08
LOOP_HALF_OUT = 0.15
LOOP_ARCH_C = 0.13
LOOP_SEWN = 0.12
LOOP_RHO = 910.0

DELTA = ARC_PITCH / ARC_R
PHI = [(i - (SETTS_PER_ARC - 1) / 2) * DELTA for i in range(SETTS_PER_ARC)]
RISE = ARC_R * (1 - math.cos(PHI[0]))
LAYER0 = BAG_Y0 + BAG_T


def crown_x(a):
    """👑️ The x of the crown of arc course `a` (0 = the course nearest the pallet)."""
    return FIELD_X0 + RISE + a * ROW_PITCH


def sett_field(a, i):
    """📍️ Centre (x, z) and yaw (rad, x towards z) of sett `i` of arc course `a` in the reinstalled field."""
    cx = crown_x(a) - ARC_R
    return cx + ARC_R * math.cos(PHI[i]), ARC_R * math.sin(PHI[i]), PHI[i]


def square(x, z, yaw, s=SETT):
    """⬜️ The plan footprint of a sett centred at (x, z) turned by `yaw`."""
    h = s / 2
    c, n = math.cos(yaw), math.sin(yaw)
    return [(x + c * px - n * pz, z + n * px + c * pz) for px, pz in ((-h, -h), (h, -h), (h, h), (-h, h))]


PATCH_IDS = [f"{side}{j + 1}" for j in range(PATCH_ROWS) for side in ("r", "l")]
LAYER_ORDER = ["r1", "r2", "r3", "r4", "l1", "l2", "l3", "l4"]


def patch_setts(pid):
    """🧱️ The setts of one patch in the field: list of (course m, index i, x, z, yaw)."""
    side, j = pid[0], int(pid[1:]) - 1
    idx = range(0, SETTS_PER_ARC // 2) if side == "r" else range(SETTS_PER_ARC // 2, SETTS_PER_ARC)
    return [(m, i, *sett_field(COURSES_PER_PATCH * j + m, i)) for m in range(COURSES_PER_PATCH) for i in idx]


def field_union(pid):
    """🗺️ The plan footprint of a patch in the field as one polygon (setts)."""
    return unary_union([Polygon(square(x, z, y)) for _, _, x, z, y in patch_setts(pid)])


_CENTRES = {}


def field_centre(pid):
    """🎯️ Centre of the bounding box of a patch in the field."""
    if pid not in _CENTRES:
        x0, z0, x1, z1 = field_union(pid).bounds
        _CENTRES[pid] = ((x0 + x1) / 2, (z0 + z1) / 2)
    return _CENTRES[pid]


REST_SHIFT = {pid: (0.0, 0.0) for pid in PATCH_IDS}


def pivot(pid):
    """🔄️ The yaw pivot (x, z) of a patch: half way between its place in the bag and in the field."""
    fx, fz = field_centre(pid)
    sx, sz = REST_SHIFT[pid]
    return (BAG_X + sx + fx) / 2, (sz + fz) / 2


def to_rest(pid, x, z):
    """↩️ A field point of patch `pid` carried to its stored place in the bag (half a turn about the pivot)."""
    px, pz = pivot(pid)
    return 2 * px - x, 2 * pz - z


def layer_y(pid):
    """📶️ The bottom of the layer a patch lies in, in the bag."""
    return LAYER0 + LAYER_ORDER.index(pid) * SETT_H


def rest_squares(pid):
    """⬛️ The rest-pose plan footprints of the setts of a patch."""
    out = []
    for m, i, x, z, yaw in patch_setts(pid):
        rx, rz = to_rest(pid, x, z)
        out.append((m, i, rx, rz, yaw + math.pi))
    return out


def _layer_polys(pid):
    return [(m, Polygon(square(x, z, y))) for m, _, x, z, y in rest_squares(pid)]


def _margin(polys):
    inner = BAG_BANDS[0][2] - BAG_T
    x0, z0, x1, z1 = unary_union(polys).bounds
    return min(x0 - (BAG_X - inner), (BAG_X + inner) - x1, z0 + inner, inner - z1)


def check_packing(verbose=False):
    """🔍️ Proves the stack: no overlap within a layer, every half course (a chain of eight setts touching at their feet) resting on the layer below, every layer clear of the bag wall."""
    report, prev, worst = [], None, 1.0
    for pid in LAYER_ORDER:
        lp = _layer_polys(pid)
        polys = [p for _, p in lp]
        for a in range(len(polys)):
            for b in range(a + 1, len(polys)):
                if polys[a].intersection(polys[b]).area > 1e-9:
                    report.append(f"overlap {pid} {a} {b}")
        margin = _margin(polys)
        if margin < 0.01:
            report.append(f"wall {pid} margin {margin:.4f}")
        if prev is not None:
            for m in range(COURSES_PER_PATCH):
                share = max(p.intersection(prev).area / p.area for k, p in lp if k == m)
                worst = min(worst, share)
        prev = unary_union(polys)
        if verbose:
            report.append(f"{pid} bounds {[round(v, 3) for v in prev.bounds]} margin {margin:.3f}")
    return report, worst


def tune_shifts():
    """🎛️ Chooses the one in-bag shift of the left-hand layers that lets every half course of the first of them rest on the last right-hand layer."""
    under = unary_union([p for _, p in _layer_polys(LAYER_ORDER[3])])
    best, best_share = (0.0, 0.0), -1.0
    for sx in [k * 0.01 for k in range(-6, 7)]:
        for sz in [k * 0.01 for k in range(-6, 7)]:
            for pid in LAYER_ORDER[4:]:
                REST_SHIFT[pid] = (sx, sz)
            lp = _layer_polys(LAYER_ORDER[4])
            if _margin([p for _, p in lp]) < 0.012:
                continue
            share = min(max(p.intersection(under).area / p.area for k, p in lp if k == m) for m in range(COURSES_PER_PATCH))
            if share > best_share + 1e-9:
                best, best_share = (sx, sz), share
    for pid in LAYER_ORDER[4:]:
        REST_SHIFT[pid] = best


def course_band(a, side, closing=0.0):
    """🌈️ The plan outline of one half course (eight setts with their joints merged) in the field."""
    idx = range(0, SETTS_PER_ARC // 2) if side == "r" else range(SETTS_PER_ARC // 2, SETTS_PER_ARC)
    u = unary_union([Polygon(square(*sett_field(a, i)[:2], sett_field(a, i)[2])) for i in idx])
    g = 0.0065
    return u.buffer(g, join_style=2).buffer(-g, join_style=2)


def patch_outline_field(pid, tol):
    """🧩️ The plan outline of a whole patch in the field (six courses, all joints merged), simplified by `tol`."""
    side, j = pid[0], int(pid[1:]) - 1
    u = unary_union([course_band(COURSES_PER_PATCH * j + m, side) for m in range(COURSES_PER_PATCH)])
    g = 0.0075
    u = u.buffer(g, join_style=2).buffer(-g, join_style=2)
    return u.simplify(tol, preserve_topology=True)


def ring_rest(pid, poly):
    """↪️ The exterior of a field polygon carried to the rest place of its patch."""
    pts = list(poly.exterior.coords)[:-1]
    return [to_rest(pid, x, z) for x, z in pts]


def pallet_parts():
    """🪵️ The Euro pallet (DIN EN 13698-1): (id, kind, centre, size) of every board and block in world metres."""
    parts = []
    for n, (z, w) in enumerate([(-0.35, 0.100), (0.0, 0.145), (0.35, 0.100)]):
        parts.append((f"bottom-board-{n + 1}", "bottom", (0.6, BOARD_T / 2, z), (1.2, BOARD_T, w)))
    for r, x in enumerate([0.0725, 0.6, 1.1275]):
        for n, (z, w) in enumerate([(-0.35, 0.100), (0.0, 0.145), (0.35, 0.100)]):
            parts.append((f"block-{r + 1}-{n + 1}", "block", (x, BOARD_T + BLOCK_H / 2, z), (0.145, BLOCK_H, w)))
    for r, x in enumerate([0.0725, 0.6, 1.1275]):
        parts.append((f"stringer-{r + 1}", "stringer", (x, BOARD_T + BLOCK_H + BOARD_T / 2, 0.0), (0.145, BOARD_T, 0.8)))
    for n, (z, w) in enumerate([(-0.3275, 0.145), (-0.16375, 0.100), (0.0, 0.145), (0.16375, 0.100), (0.3275, 0.145)]):
        parts.append((f"deck-board-{n + 1}", "deck", (0.6, PALLET_H - BOARD_T / 2, z), (1.2, BOARD_T, w)))
    return parts


def rounded_square(hw, rc, n=6):
    """🔲️ A rounded square of half width `hw` and corner radius `rc` as (a, b) points, counter-clockwise."""
    pts = []
    for cx, cy, a0 in ((hw - rc, hw - rc, 0), (-(hw - rc), hw - rc, 90), (-(hw - rc), -(hw - rc), 180), (hw - rc, -(hw - rc), 270)):
        for k in range(n + 1):
            t = math.radians(a0 + 90 * k / n)
            pts.append((cx + rc * math.cos(t), cy + rc * math.sin(t)))
    return pts


def loop_profile(n=8):
    """🪢️ The loop as a flat inverted-U band of webbing in its bone frame (u up from the rim, v = -x): outer then inner edge."""
    out, inn = LOOP_HALF_OUT, LOOP_HALF_OUT - LOOP_BAND
    pts = [(-LOOP_SEWN, -out)]
    for k in range(n + 1):
        t = math.pi - math.pi * k / n
        pts.append((LOOP_ARCH_C + out * math.sin(t), out * math.cos(t)))
    pts.append((-LOOP_SEWN, out))
    pts.append((-LOOP_SEWN, inn))
    for k in range(n + 1):
        t = math.pi * k / n
        pts.append((LOOP_ARCH_C + inn * math.sin(t), inn * math.cos(t)))
    pts.append((-LOOP_SEWN, -inn))
    return [(round(u, 6), round(v, 6)) for u, v in pts]


if __name__ == "__main__":
    tune_shifts()
    rep, share = check_packing(True)
    print("\n".join(rep))
    print("worst support share", round(share, 3), "rise", round(RISE, 4), "shifts", REST_SHIFT)
    print("field x", min(field_union(p).bounds[0] for p in PATCH_IDS), max(field_union(p).bounds[2] for p in PATCH_IDS))
    print("field z", min(field_union(p).bounds[1] for p in PATCH_IDS), max(field_union(p).bounds[3] for p in PATCH_IDS))
    area = sum(unary_union([course_band(COURSES_PER_PATCH * (int(p[1:]) - 1) + m, p[0]) for m in range(6)]).area for p in PATCH_IDS)
    print("setts", len(PATCH_IDS) * 48, "hull field area", round(unary_union([field_union(p).convex_hull for p in PATCH_IDS]).area, 3))
