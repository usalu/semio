"""📐 Hand-chosen numbers and pure geometry of the reclaimed trapezoidal sheet stack (35/207, 0.75 mm, 6 m, thirty nested sheets).

World frame of the staffage archive: x ahead along the sheets, y up, z the asset's left. Bundle frame: origin at the underside
of the bottom sheet at its x = 0 end (world y = 0.10 on the bearers).
@see ../../brief.md
"""
import math

L = 6.0
T = 0.00075
H = 0.035
RIB = 0.207
CREST = 0.035
WEB = 0.025
NEST = 0.0015
SHEETS = 30
CROWNS = [-0.5175 + RIB * k for k in range(6)]
EDGE = 0.535
CUT = 0.15
BEARER = 0.10
BEARER_LEN = 1.20
XB = [0.90, 2.55, 3.45, 5.10]
XS = [0.30, 2.10, 3.90, 5.70]
XL = [1.50, 4.50]
BATTEN_HALF = 0.515
STRAP_W = 0.032
STRAP_T = 0.001
PROT_X = 0.10
PROT_T = 0.002
LOOP_W = 0.025
LOOP_T = 0.010
LEG_LEN = 0.7375
LEG_W = 0.05
LEG_T = 0.016
LIFT = 1.50
OLD_PURLINS = [0.10, 1.55, 3.00, 4.45, 5.90]
HOLE_D = 0.0065

V30 = (SHEETS - 1) * NEST
TOP = V30 + H
V29TOP = (SHEETS - 2) * NEST + H
BATTEN_V0 = TOP
BATTEN_V1 = TOP + BEARER
STRAP_TOP = BATTEN_V1 + STRAP_T
PROT_V0 = 0.0315
PROT_V1 = TOP + PROT_T


def mid_polyline(cut=False):
    """〰️ Mid-surface of one sheet across its width as (z, v): crests at v = H - T/2, valleys at T/2, edge crests at both edges."""
    hi, lo = H - T / 2, T / 2
    pts = [] if cut else [(-EDGE, hi)]
    for k, c in enumerate(CROWNS):
        if k > 0:
            pts.append((c - CREST / 2, hi))
        if k < 5:
            pts.append((c + CREST / 2, hi))
            pts.append((c + CREST / 2 + WEB, lo))
            pts.append((CROWNS[k + 1] - CREST / 2 - WEB, lo))
    pts.append((EDGE, hi))
    return pts


def _offset_line(p, q, d):
    dx, dy = q[0] - p[0], q[1] - p[1]
    n = math.hypot(dx, dy)
    nx, ny = -dy / n, dx / n
    return (p[0] + nx * d, p[1] + ny * d), (q[0] + nx * d, q[1] + ny * d)


def _intersect(a0, a1, b0, b1):
    d1 = (a1[0] - a0[0], a1[1] - a0[1])
    d2 = (b1[0] - b0[0], b1[1] - b0[1])
    den = d1[0] * d2[1] - d1[1] * d2[0]
    if abs(den) < 1e-15:
        return a1
    s = ((b0[0] - a0[0]) * d2[1] - (b0[1] - a0[1]) * d2[0]) / den
    return (a0[0] + d1[0] * s, a0[1] + d1[1] * s)


def offset_polyline(pts, d):
    """➰ Miter offset of an open polyline by d along its left normal (positive d = above for a left-to-right line)."""
    segs = [_offset_line(pts[i], pts[i + 1], d) for i in range(len(pts) - 1)]
    out = [segs[0][0]]
    for i in range(len(segs) - 1):
        out.append(_intersect(segs[i][0], segs[i][1], segs[i + 1][0], segs[i + 1][1]))
    out.append(segs[-1][1])
    return out


def sheet_surfaces(cut=False):
    """🪟 Upper and lower surface of one sheet (z, v), left to right."""
    m = mid_polyline(cut)
    return offset_polyline(m, T / 2), offset_polyline(m, -T / 2)


def sheet_ring(cut=False, dv=0.0):
    """🔲 Closed section polygon of one sheet (z, v), lifted by dv."""
    up, dn = sheet_surfaces(cut)
    return [(z, v + dv) for z, v in up] + [(z, v + dv) for z, v in reversed(dn)]


def pack_ring(top_index, cut=False, bottom_dv=0.0):
    """🧱 Envelope section of nested sheets 1 .. top_index: lower face of sheet 1 up to the upper face of sheet top_index."""
    up, _ = sheet_surfaces(cut)
    _, dn = sheet_surfaces(cut)
    dv = (top_index - 1) * NEST
    return [(z, v + dv) for z, v in up] + [(z, v + bottom_dv) for z, v in reversed(dn)]


def area_centroid_inertia(ring):
    """📏 Area, centroid height and second moment about the horizontal centroidal axis of a simple polygon (z, v)."""
    a = cz = cv = 0.0
    n = len(ring)
    for i in range(n):
        x0, y0 = ring[i]
        x1, y1 = ring[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        a += c
        cv += (y0 + y1) * c
    a /= 2
    cv /= 6 * a
    ixx = 0.0
    for i in range(n):
        x0, y0 = ring[i]
        x1, y1 = ring[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        ixx += (y0 * y0 + y0 * y1 + y1 * y1) * c
    ixx = ixx / 12 - a * cv * cv
    return abs(a), cv, abs(ixx)


def offset_convex(pts, d):
    """⭕ Outward offset of a counter-clockwise convex polygon by d (miter)."""
    n = len(pts)
    lines = []
    for i in range(n):
        p, q = pts[i], pts[(i + 1) % n]
        lines.append(_offset_line(p, q, -d))
    out = []
    for i in range(n):
        a = lines[i - 1]
        b = lines[i]
        out.append(_intersect(a[0], a[1], b[0], b[1]))
    return out


def ccw(pts):
    """🔄 The polygon counter-clockwise."""
    s = sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))
    return pts if s > 0 else list(reversed(pts))


def strap_hull(protector_out=EDGE + PROT_T, protector_v0=None):
    """🎗️ Convex hull (z, v) the strap is tensioned around: bottom valleys, the edge protectors, the batten tops."""
    pv0 = PROT_V0 if protector_v0 is None else protector_v0
    return ccw([(-0.475, 0.0), (0.475, 0.0), (protector_out, pv0), (protector_out, PROT_V1), (BATTEN_HALF, BATTEN_V1), (-BATTEN_HALF, BATTEN_V1), (-protector_out, PROT_V1), (-protector_out, pv0)])


def loop_hull():
    """🪢 Convex hull (z, v) of the sheet stack alone, which the choked round sling hugs."""
    return ccw([(-0.475, 0.0), (0.475, 0.0), (EDGE, H - T), (EDGE, TOP), (-EDGE, TOP), (-EDGE, H - T)])


def flat_hull(v_top):
    """⬜ The rectangular hull of a generalised stack (coarse scales): sheets as a block from v = 0 to v_top, battens on top."""
    return ccw([(-EDGE, 0.0), (EDGE, 0.0), (EDGE, v_top), (BATTEN_HALF, BATTEN_V1), (-BATTEN_HALF, BATTEN_V1), (-EDGE, v_top)])


def ring_length(pts):
    """📐 Perimeter of a closed polygon."""
    return sum(math.dist(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts)))


def edge_crest_ring():
    """🪒 Section (z, v) of the right edge crest, the part trimmed off at the eaves corner."""
    return ccw([(-EDGE, H - T), (-0.5, H - T), (-0.5, H), (-EDGE, H)])


def edge_crest_pack_ring(top_index):
    """🪒 Envelope section (z, v) of the stacked right edge crests of sheets 1 .. top_index."""
    return ccw([(-EDGE, H - T), (-0.5, H - T), (-0.5, H + (top_index - 1) * NEST), (-EDGE, H + (top_index - 1) * NEST)])
