"""🎻 The hand-chosen numbers of the used cello and its wheeled hard case, and the parts built from them (3D solids and 2D outlines).

Every dimension is in metres in the rest pose, which is the stored pose: the case stands upright on its two wheels and its front bumper, the
lid towards +x, the cello upright inside with its top towards the lid, the bow clipped into the lid, the endpin stopper in the endpin pocket.

Frames: world x ahead (towards the lid), y up, z left. Every bone rests upright (u = +y, v = -x, w = +z) unless stated.

@see ./author-cello.py — writes the asset document from these parts
@see ../../reports/used-cello.md — the reference table these numbers come from
"""
import math
from author_geom import (circle, interp, outer_area_tub, pt, r6, ring_loft, section_x, signed_volume, mesh_area, smooth_interp, superellipse_half, tub)

CASE_N = 3.0
CASE_BAND = 0.065
XP = -0.048
WALL = 0.008
CASE_STATIONS = [
    (0.006, 0.150, -0.118, 0.118),
    (0.020, 0.182, -0.132, 0.132),
    (0.050, 0.206, -0.140, 0.140),
    (0.100, 0.226, -0.144, 0.144),
    (0.180, 0.235, -0.145, 0.145),
    (0.300, 0.235, -0.145, 0.145),
    (0.400, 0.224, -0.145, 0.145),
    (0.520, 0.204, -0.145, 0.145),
    (0.640, 0.202, -0.145, 0.145),
    (0.740, 0.194, -0.143, 0.143),
    (0.820, 0.166, -0.134, 0.139),
    (0.880, 0.130, -0.117, 0.129),
    (0.930, 0.111, -0.102, 0.121),
    (1.000, 0.106, -0.096, 0.117),
    (1.150, 0.106, -0.096, 0.117),
    (1.250, 0.106, -0.096, 0.115),
    (1.310, 0.098, -0.092, 0.109),
    (1.340, 0.082, -0.082, 0.097),
    (1.361, 0.058, -0.064, 0.079),
]
CASE_TOP = 1.361

CXC, CYC = -0.048, 0.060
BODY_U0, BODY_L = 0.012, 0.755
RIB = 0.060
ARCH_TOP, ARCH_BACK = 0.024, 0.026
OUTLINE = [
    (0.000, 0.014), (0.002, 0.040), (0.006, 0.066), (0.012, 0.092), (0.022, 0.118), (0.035, 0.142), (0.052, 0.163), (0.075, 0.182),
    (0.100, 0.198), (0.130, 0.210), (0.160, 0.217), (0.190, 0.220), (0.220, 0.218), (0.250, 0.211), (0.280, 0.201), (0.305, 0.192),
    (0.325, 0.188), (0.340, 0.190), (0.348, 0.178), (0.360, 0.150), (0.380, 0.129), (0.400, 0.119), (0.420, 0.116), (0.440, 0.118),
    (0.460, 0.124), (0.478, 0.136), (0.492, 0.152), (0.500, 0.160), (0.512, 0.158), (0.530, 0.160), (0.560, 0.165), (0.590, 0.169),
    (0.620, 0.171), (0.650, 0.168), (0.680, 0.160), (0.705, 0.147), (0.722, 0.131), (0.736, 0.111), (0.746, 0.088), (0.751, 0.064),
    (0.754, 0.040), (0.755, 0.016),
]
BRIDGE_U = 0.367
NUT_U = 1.047
SCROLL_TOP = 1.241
FB_END_U = 0.467
TAIL_U0, TAIL_U1 = 0.020, 0.252
STRINGS = [("c", 0.0225, 0.0090, 0.00095), ("g", 0.0075, 0.0030, 0.00080), ("d", -0.0075, -0.0030, 0.00065), ("a", -0.0225, -0.0090, 0.00050)]
ENDPIN_REST_TIP = -0.025
ENDPIN_LEN = 0.50
ENDPIN_R = 0.005

BOW_FROG, BOW_TIP = -0.425, 0.310


def B(s):
    """🎻 Half-width of the cello body outline at s metres above the bottom edge of the body."""
    return max(smooth_interp(OUTLINE, s), 0.006)


def arch_long(s):
    """⛰️ Longitudinal arching factor 0..1 along the body (flat at the ends, full over the middle)."""
    x = min(max(s / BODY_L, 0.0), 1.0)
    return math.sin(math.pi * x) ** 0.55


def fb_top(u):
    """🎼 The v of the fingerboard top along the cello axis (the neck is set back by its angle)."""
    return -0.094 + 0.1775 * (u - 0.767)


def top_v(s, w):
    """🌊 The v of the outer surface of the top plate at (s, w)."""
    b = B(s)
    r = min(abs(w) / b, 1.0)
    return -RIB - ARCH_TOP * arch_long(s) * (1 - r * r) ** 1.25


def back_v(s, w):
    """🌊 The v of the outer surface of the back plate at (s, w)."""
    b = B(s)
    r = min(abs(w) / b, 1.0)
    return RIB + ARCH_BACK * arch_long(s) * (1 - r * r) ** 1.25


def string_v(u, w):
    """🎶 The v of a string at u between the bridge and the nut (linear), with the bridge curvature for the outer strings."""
    vb = -0.174 + 0.0055 * (abs(w) / 0.0225) ** 2
    vn = fb_top(NUT_U) - 0.002
    t = (u - BRIDGE_U) / (NUT_U - BRIDGE_U)
    return vb + t * (vn - vb)


def corpus_stations(count):
    """📏 Station positions s along the body for a ring loft of about `count` stations, dense at the ends and the corners."""
    marks = sorted(set([0.0, 0.002, 0.006, 0.012, 0.022, 0.035, 0.052, 0.075, 0.100, 0.130, 0.160, 0.190, 0.220, 0.250, 0.280, 0.305, 0.325, 0.340, 0.348,
                        0.360, 0.380, 0.400, 0.420, 0.440, 0.460, 0.478, 0.492, 0.500, 0.512, 0.530, 0.560, 0.590, 0.620, 0.650, 0.680, 0.705,
                        0.722, 0.736, 0.746, 0.751, 0.754, 0.755]))
    if count >= len(marks):
        return marks
    keep = [marks[round(i * (len(marks) - 1) / (count - 1))] for i in range(count)]
    for corner in (0.340, 0.348, 0.492, 0.500) if count > 20 else ():
        if corner not in keep:
            keep.append(corner)
    return sorted(set(keep))


def corpus_ring(s, k):
    """💍 One section ring of the closed cello body at s in cello-local (u, v, w): right rib, top arch, left rib, back arch."""
    b = B(s)
    u = BODY_U0 + s
    ring = [(u, RIB, -b), (u, -RIB, -b)]
    for i in range(1, k + 1):
        w = -b * math.cos(math.pi * i / (k + 1))
        ring.append((u, top_v(s, w), w))
    ring += [(u, -RIB, b), (u, RIB, b)]
    for i in range(1, k + 1):
        w = b * math.cos(math.pi * i / (k + 1))
        ring.append((u, back_v(s, w), w))
    return ring


def plate_ring(s, k, top, thickness):
    """🪵 One section ring of a closed arched plate (top or back) of the given thickness: outer arch then inner arch reversed."""
    b = B(s)
    u = BODY_U0 + s
    sign = -1 if top else 1
    outer, inner = [], []
    for i in range(k + 2):
        w = -b * math.cos(math.pi * i / (k + 1))
        v = top_v(s, w) if top else back_v(s, w)
        outer.append((u, v, w))
        inner.append((u, v - sign * thickness, w * (1 - 0.6 * thickness / b)))
    return outer + list(reversed(inner))


def corpus_polyhedron(count, k):
    """🎻 The closed body of the cello (ribs, arched top and back) as one polyhedron in cello-local coordinates."""
    rings = [corpus_ring(s, k) for s in corpus_stations(count)]
    v, t = ring_loft(rings, "fan")
    return [pt(p) for p in v], t, mesh_area(v, t), signed_volume(v, t)


def plate_polyhedron(count, k, top, thickness):
    """🪵 A closed arched plate polyhedron (structural top or back plate)."""
    rings = [plate_ring(s, k, top, thickness) for s in corpus_stations(count)[1:-1]]
    v, t = ring_loft(rings, "strip")
    return [pt(p) for p in v], t, mesh_area(v, t), signed_volume(v, t)


def outline_face(count, inset=0.0):
    """🖼️ The face outline of the body in cello-local (u, w), right side up then left side down."""
    ss = corpus_stations(count)
    right = [(BODY_U0 + s, -(B(s) - inset)) for s in ss]
    left = [(BODY_U0 + s, B(s) - inset) for s in reversed(ss)]
    return right + left


def corpus_profile(count):
    """📐 The side profile of the body in cello-local (u, v): back arch up, top arch down."""
    ss = corpus_stations(count)
    back = [(BODY_U0 + s, back_v(s, 0.0)) for s in ss]
    top = [(BODY_U0 + s, top_v(s, 0.0)) for s in reversed(ss)]
    return back + top


def cello_to_world(p):
    """🌍 Cello-local (u, v, w) to world rest (x, y, z)."""
    return (CXC - p[1], CYC + p[0], p[2])


def case_local(p):
    """🧳 World (x, y, z) to case-local (u, v, w)."""
    return (p[1], -p[0], p[2])


def case_back_tub(count, wall=WALL):
    """🛁 The back shell of the case (towards -x, carrying wheels and straps) in case-local coordinates."""
    st = [(y, b, xb) for y, b, xb, xf in CASE_STATIONS]
    return tub(st, XP, -1, wall, CASE_N, count, case_local, CASE_BAND)


LID_ORIGIN = (XP, 0.180, -0.235)


def lid_local(p):
    """🚪 World (x, y, z) to lid-local (u, v, w): the lid bone rests upright on its hinge line at the right rim."""
    return (p[1] - LID_ORIGIN[1], -(p[0] - LID_ORIGIN[0]), p[2] - LID_ORIGIN[2])


def case_lid_tub(count, wall=WALL):
    """🚪 The lid of the case (towards +x) in lid-local coordinates."""
    st = [(y, b, xf) for y, b, xb, xf in CASE_STATIONS]
    return tub(st, XP, 1, wall, CASE_N, count, lid_local, CASE_BAND)


def case_b(y):
    """↔️ Half-width of the case outline at height y."""
    return interp([(r[0], r[1]) for r in CASE_STATIONS], y)


def case_xb(y):
    """⬅️ The back face x of the case at height y (on the centre line)."""
    return interp([(r[0], r[2]) for r in CASE_STATIONS], y)


def case_xf(y):
    """➡️ The lid face x of the case at height y (on the centre line)."""
    return interp([(r[0], r[3]) for r in CASE_STATIONS], y)


def case_back_x(y, z):
    """📏 The x of the outer back surface at height y and lateral z."""
    return section_x(XP, case_xb(y), case_b(y), CASE_N, z, CASE_BAND)


def case_lid_x(y, z):
    """📏 The x of the outer lid surface at height y and lateral z."""
    return section_x(XP, case_xf(y), case_b(y), CASE_N, z, CASE_BAND)


def case_outer_areas(count):
    """📐 Outer skin areas of the back shell and the lid (without the parting faces)."""
    back = outer_area_tub([(y, b, xb) for y, b, xb, xf in CASE_STATIONS], XP, -1, CASE_N, count, CASE_BAND)
    lid = outer_area_tub([(y, b, xf) for y, b, xb, xf in CASE_STATIONS], XP, 1, CASE_N, count, CASE_BAND)
    return back, lid
