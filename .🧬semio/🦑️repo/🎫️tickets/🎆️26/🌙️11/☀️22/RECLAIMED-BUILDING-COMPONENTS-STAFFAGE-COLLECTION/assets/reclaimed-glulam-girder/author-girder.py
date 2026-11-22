"""🪵 Authors the asset document of the reclaimed glulam girder from hand-chosen numbers.

The one authoritative chisel of `reclaimed-glulam-girder`: every dimension, mass, load and thermal value is stated here once and both the
2D drawings and the 3D volumes of all four scales are written from it. Run: `python3 author-girder.py` (writes `🔣️.json` next to it).

See: ../../brief.md, ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))

L, H, B = 12.0, 1.0, 0.16
HB, HW = H / 2, B / 2
LAM = 0.04
NLAM = 25
SOFFIT = 0.12
GY = SOFFIT + HB
BEARERS = [1.0, 6.0, 11.0]
BS = 0.12
BLEN = 1.0
XA, XB = 2.77, 8.77
LEG = 4.0
RS = 0.0175
LEGZ = 0.04
THETA = math.degrees(math.acos((XB - XA) / 2 / LEG))
SHOE_L, SHOE_BASE_T, SHOE_BASE_W = 0.30, 0.02, 0.26
SHOE_SIDE_T, SHOE_SIDE_H = 0.01, 0.40
BOLTS_U = [0.10, 0.20]
BOLT_V = -0.28
HEAD_T, NUT_T, BOLT_S = 0.013, 0.016, 0.03
HOLES = [(0.55, 0.16), (0.75, 0.16), (0.55, 0.36), (0.75, 0.36)]
HOLE_R = 0.0105
PURLINS = [0.6, 3.0, 5.4, 7.8, 10.2]
WEDGE_DX0, WEDGE_DX1 = 0.02, 0.10
WEDGE_W, WEDGE_H = 0.15, 0.10
LIFT_DY = 0.5
REINSTALL_DY = -(SOFFIT - SHOE_BASE_T)
DUNNAGE_DZ = 0.9
SHOE_BEARING_X = SHOE_L / 2
END_BEARING_X = L - 0.10
SPAN = END_BEARING_X - SHOE_BEARING_X
LOAD_REST_X = 0.5
MID_X = SHOE_BEARING_X + SPAN / 2

RHO_GL, RHO_C24, RHO_ST = 420.0, 420.0, 7850.0
SLING_KG_M = 0.45
VOL_GIRDER = L * H * B - len(HOLES) * math.pi * HOLE_R ** 2 * B
M_GIRDER = round(VOL_GIRDER * RHO_GL, 2)
M_BASE = SHOE_L * SHOE_BASE_T * SHOE_BASE_W * RHO_ST
M_SIDES = 2 * SHOE_L * SHOE_SIDE_H * SHOE_SIDE_T * RHO_ST
M_BOLTS = 2 * 0.75
M_SHOE = round(M_BASE + M_SIDES + M_BOLTS, 3)
SHOE_COMY = round((M_BASE * (-HB - SHOE_BASE_T / 2) + M_SIDES * (-HB + SHOE_SIDE_H / 2) + M_BOLTS * BOLT_V) / M_SHOE, 4)
M_BEARER = BS * BS * BLEN * RHO_C24
M_WEDGE = 0.5 * WEDGE_W * WEDGE_H * (WEDGE_DX1 - WEDGE_DX0) * RHO_C24
M_BEARER_BONE = round(M_BEARER + 2 * M_WEDGE, 3)
BEARER_COMY = round((M_BEARER * BS / 2 + 2 * M_WEDGE * (BS + WEDGE_H / 3)) / M_BEARER_BONE, 4)
M_LEG = round(LEG * SLING_KG_M, 3)
CHOKER_LEN = 2 * ((H + 2 * RS) + (B + 2 * RS))
M_CHOKER = round(CHOKER_LEN * SLING_KG_M, 3)
MASS = round(M_GIRDER + M_SHOE + 3 * M_BEARER_BONE + 2 * M_LEG + 2 * M_CHOKER, 2)
HEIGHT = round(SOFFIT + H + 2 * RS, 4)
COM_X = (M_GIRDER * L / 2 + M_SHOE * SHOE_L / 2) / (M_GIRDER + M_SHOE)

G_ROOF_KNM2, SPACING = 0.5, 5.0
S_K, MU1 = 0.85, 0.8
W_ROOF = G_ROOF_KNM2 * SPACING * 1000
W_SNOW = S_K * MU1 * SPACING * 1000
Q_POINT = 1000.0

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(x):
    """🎯️ Rounds a coordinate to a tenth of a millimetre-micrometre grid."""
    return round(x + 0.0, 6)


def rect(x, y, w, h):
    """▭️ Rect geometry."""
    return {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}


def poly(points, closed=True):
    """📐️ Poly geometry."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in points], "closed": closed}


def ellipse(cx, cy, rx, ry=None):
    """⭕️ Ellipse geometry."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry if ry is not None else rx)}


def shape(id_, bone, role, z, geom, filled=True, tone=0.5, region=None, regions=None, material=None, hatch=None, pen=None, solid=None):
    """✏️ A 2D shape of a representation."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    s.update(geom)
    if solid:
        s["solid"] = solid
    return s


def base(id_, bone, material, tone, role="body", **extra):
    """🧱️ The common fields of a volume solid."""
    s = {"id": id_, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(extra)
    return s


def box(id_, bone, material, tone, size, centre, **extra):
    """📦️ A box solid."""
    s = base(id_, bone, material, tone, **extra)
    s.update({"kind": "box", "size": [r6(v) for v in size], "transform": {"translation": [r6(v) for v in centre]}})
    return s


def extrude(id_, bone, material, tone, profile, plane, a, b, holes=None, **extra):
    """🧈️ An extrude solid."""
    s = base(id_, bone, material, tone, **extra)
    s.update({"kind": "extrude", "profile": [[r6(p), r6(q)] for p, q in profile], "plane": plane, "from": r6(a), "to": r6(b)})
    if holes:
        s["holes"] = [[[r6(p), r6(q)] for p, q in h] for h in holes]
    return s


def cylinder_u(id_, bone, material, tone, length, radius, **extra):
    """🧵️ A cylinder along u from the origin, as an open revolve profile."""
    s = base(id_, bone, material, tone, **extra)
    s.update({"kind": "revolve", "profile": [[0, 0], [0, radius], [length, radius], [length, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0]})
    return s


def sphere(id_, bone, material, tone, r, centre, **extra):
    """🔮️ An ellipsoid with three equal radii."""
    s = base(id_, bone, material, tone, **extra)
    s.update({"kind": "ellipsoid", "radii": [r, r, r], "transform": {"translation": [r6(v) for v in centre]}})
    return s


def circle_pts(cx, cy, r, n=16):
    """🔵️ A circle as an n-gon."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def hexagon(cx, cy, s):
    """⬡️ A hexagon of across-flats width s, flats horizontal."""
    rr = s / math.sqrt(3)
    return [(cx + rr * math.cos(math.pi / 3 * k), cy + rr * math.sin(math.pi / 3 * k)) for k in range(6)]


AXES = {"+u": (1, 0, 0), "-u": (-1, 0, 0), "+v": (0, 1, 0), "-v": (0, -1, 0), "+w": (0, 0, 1), "-w": (0, 0, -1)}


def sub(a, b):
    """➖️ Vector difference."""
    return [a[i] - b[i] for i in range(3)]


def cross(a, b):
    """✖️ Cross product."""
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    """⚫️ Dot product."""
    return sum(a[i] * b[i] for i in range(3))


def unit(a):
    """📏️ Unit vector."""
    n = math.sqrt(dot(a, a))
    return [c / n for c in a]


def frustum(outer, inner):
    """🔺️ The convex frustum between an outer and an inner quad (corresponding corners): vertices, outward triangles and face normals."""
    verts = outer + inner
    c = [sum(v[i] for v in verts) / 8 for i in range(3)]
    quads = [[0, 1, 2, 3], [4, 5, 6, 7]] + [[k, (k + 1) % 4, 4 + (k + 1) % 4, 4 + k] for k in range(4)]
    tris, normals = [], []
    for q in quads:
        a, b_, cc, d = [verts[i] for i in q]
        n = cross(sub(b_, a), sub(cc, a))
        fc = [(a[i] + b_[i] + cc[i] + d[i]) / 4 for i in range(3)]
        if dot(n, sub(fc, c)) < 0:
            q = [q[0], q[3], q[2], q[1]]
            n = [-x for x in n]
        tris += [[q[0], q[1], q[2]], [q[0], q[2], q[3]]]
        normals.append(unit(n))
    return verts, tris, normals


def section_tags(normals, outer_index, outer_axis):
    """✂️ The section-face tags that capture every face but the outer one; asserts each angle is under 44 degrees and the outer face stays free."""
    tags = set()
    for i, n in enumerate(normals):
        if i == outer_index:
            continue
        best = max(AXES, key=lambda k: dot(n, AXES[k]))
        ang = math.degrees(math.acos(max(-1, min(1, dot(n, AXES[best])))))
        assert ang < 44.0, (best, ang)
        tags.add(best)
    out = normals[outer_index]
    for k in tags:
        assert math.degrees(math.acos(max(-1, min(1, dot(out, AXES[k]))))) > 46.0, (k, outer_axis)
    return sorted(tags)


def girder_skins(tu, tv, tw, tone, suffix):
    """🧥️ Six frustum skins of the girder box (picture-frame decomposition) with distinct thicknesses so that every mitre leans under 45 degrees."""
    o = [(0.0, L), (-HB, HB), (-HW, HW)]
    i = [(tu, L - tu), (-HB + tv, HB - tv), (-HW + tw, HW - tw)]
    spec = [("th-side-near", 2, 0, "girder-sides", "glulam-weathered"), ("th-side-far", 2, 1, "girder-sides", "glulam-weathered"),
            ("th-top", 1, 1, "girder-top", "glulam-weathered"), ("th-soffit", 1, 0, "girder-soffit", "glulam-weathered"),
            ("th-old-end", 0, 0, "old-end", "glulam-weathered"), ("th-sawn-end", 0, 1, "sawn-end", "glulam-sawn")]
    out = []
    for name, axis, side, region, material in spec:
        others = [a for a in range(3) if a != axis]

        def corners(box_):
            pts = []
            for (s0, s1) in [(0, 0), (1, 0), (1, 1), (0, 1)]:
                p = [0.0, 0.0, 0.0]
                p[axis] = box_[axis][side]
                p[others[0]] = box_[others[0]][s0]
                p[others[1]] = box_[others[1]][s1]
                pts.append(p)
            return pts

        verts, tris, normals = frustum(corners(o), corners(i))
        outer_axis = ("+" if side else "-") + "uvw"[axis]
        tags = section_tags(normals, 0, outer_axis)
        s = base(f"{name}{suffix}", "girder", material, tone, region=region, sectionFaces=tags)
        s.update({"kind": "polyhedron", "vertices": [[r6(c) for c in v] for v in verts], "triangles": tris})
        out.append(s)
    return out


def tri_prisms(a, b, end_t, suffix):
    """🔻️ Four triangular face prisms of the girder section meeting at its axis, from `a` to `b` along u, with two full-section end blocks of `end_t`."""
    faces = [("th-side-near", [(-HB, -HW), (HB, -HW), (0, 0)], "girder-sides", "-w"), ("th-side-far", [(HB, HW), (-HB, HW), (0, 0)], "girder-sides", "+w"),
             ("th-top", [(HB, -HW), (HB, HW), (0, 0)], "girder-top", "+v"), ("th-soffit", [(-HB, HW), (-HB, -HW), (0, 0)], "girder-soffit", "-v")]
    out = []
    for name, prof, region, outer in faces:
        tags = set(["+u", "-u"])
        for k in range(3):
            p, q = prof[k], prof[(k + 1) % 3]
            ev = (q[0] - p[0], q[1] - p[1])
            n = unit([0.0, ev[1], -ev[0]])
            cen = (sum(x[0] for x in prof) / 3, sum(x[1] for x in prof) / 3)
            mid = ((p[0] + q[0]) / 2, (p[1] + q[1]) / 2)
            if n[1] * (mid[0] - cen[0]) + n[2] * (mid[1] - cen[1]) < 0:
                n = [-x for x in n]
            best = max(AXES, key=lambda kk: dot(n, AXES[kk]))
            if best == outer:
                continue
            ang = math.degrees(math.acos(dot(n, AXES[best])))
            assert ang < 44.0, (name, best, ang)
            tags.add(best)
        out.append(extrude(f"{name}{suffix}", "girder", "glulam-weathered", 0.5, prof, "yz", a, b, region=region, sectionFaces=sorted(tags)))
    out.append(box(f"th-old-end{suffix}", "girder", "glulam-weathered", 0.5, (end_t, H, B), (end_t / 2, 0, 0), region="old-end"))
    out.append(box(f"th-sawn-end{suffix}", "girder", "glulam-sawn", 0.8, (end_t, H, B), (L - end_t / 2, 0, 0), region="sawn-end"))
    return out


def wedge_profiles():
    """🔺️ The two timber chocks of a bearer as (v, w) triangles against the girder sides."""
    return [[(BS, HW), (BS, HW + WEDGE_W), (BS + WEDGE_H, HW)], [(BS, -HW), (BS + WEDGE_H, -HW), (BS, -HW - WEDGE_W)]]


def leg_solid_shape(id_, bone, z, tone, half, filled=True):
    """🎗️ A front or plan shape of a sling leg: its bone-local strip swept laterally, so every view follows the rising leg."""
    zc = LEGZ if bone == "sling-leg-a" else -LEGZ
    geom = rect(0, -half, LEG, 2 * half)
    return shape(id_, bone, "body", z, geom, filled, tone, region="slings", material="polyester-sling",
                 solid={"kind": "extruded", "sweep": "w", "z0": r6(zc - half), "z1": r6(zc + half)})


# ---------------------------------------------------------------- 2D

def elevation(scale):
    """🖼️ The side elevation of one scale (bone-local u, v)."""
    s = []
    gregions = ["girder-sides", "girder-top", "girder-soffit", "old-end", "sawn-end"] + (["steel-shoe"] if scale == 500 else [])
    if scale == 500:
        s.append(shape("girder", "girder", "body", 2, poly([(0, -HB - SHOE_BASE_T), (SHOE_L, -HB - SHOE_BASE_T), (SHOE_L, -HB), (L, -HB), (L, HB), (0, HB)]), tone=0.45, regions=gregions, material="glulam-weathered"))
    else:
        s.append(shape("girder", "girder", "body", 2, rect(0, -HB, L, H), tone=0.5, regions=gregions, material="glulam-weathered", hatch="grain" if scale == 100 else None))
    if scale == 50:
        for k in range(1, NLAM):
            v = -HB + LAM * k
            s.append(shape(f"glue-line-{k:02d}", "girder", "texture", 2.2, poly([(0, v), (L, v)], False), False, 0.3, region="girder-sides"))
        for j, p in enumerate(PURLINS):
            s.append(shape(f"hanger-trace-{j}", "girder", "texture", 2.3, poly([(p - 0.05, HB), (p - 0.05, HB - 0.2), (p + 0.05, HB - 0.2), (p + 0.05, HB)], False), False, 0.35, region="girder-sides"))
    if scale in (50, 100):
        for j, (u, v) in enumerate(HOLES):
            s.append(shape(f"bolt-hole-{j}", "girder", "detail", 2.6, ellipse(u, v, HOLE_R), True, 0.08, region="girder-sides"))
    if scale == 100:
        s.append(shape("shoe", "shoe", "body", 3, rect(0, -HB - SHOE_BASE_T, SHOE_L, SHOE_SIDE_H + SHOE_BASE_T), tone=0.25, region="steel-shoe", material="steel-primed"))
    elif scale != 500:
        base_t = SHOE_BASE_T if scale != 200 else FEATURE[200]
        s.append(shape("shoe-side-plate", "shoe", "body", 3, rect(0, -HB, SHOE_L, SHOE_SIDE_H), tone=0.25, region="steel-shoe", material="steel-primed"))
        s.append(shape("shoe-base-plate", "shoe", "body", 3, rect(0, -HB - SHOE_BASE_T, SHOE_L, base_t), tone=0.22, region="steel-shoe", material="steel-primed"))
    if scale == 50:
        for j, u in enumerate(BOLTS_U):
            s.append(shape(f"bolt-head-{j}", "shoe", "detail", 4, poly(hexagon(u, BOLT_V, BOLT_S)), tone=0.15, region="steel-shoe", material="steel-primed"))
    if scale != 500:
        for c in ("choker-a", "choker-b"):
            s.append(shape(f"{c}-band", c, "body", 3.5, rect(-RS, -HB - 2 * RS, 2 * RS, H + 4 * RS), tone=0.35, region="slings", material="polyester-sling"))
    half = RS if scale != 500 else FEATURE[500] / 2
    for leg in ("sling-leg-a", "sling-leg-b"):
        s.append(shape(f"{leg}-strap", leg, "body", 4, rect(0, -half, LEG, 2 * half), tone=0.35, region="slings", material="polyester-sling"))
    for k in range(3):
        bb = f"bearer-{k + 1}"
        if scale == 500:
            s.append(shape(f"{bb}-block", bb, "body", 1, rect(0, 0, BS, BS + WEDGE_H), tone=0.45, regions=["bearers", "wedges"], material="softwood-bearer"))
        elif scale == 100:
            s.append(shape(f"{bb}-with-chock", bb, "body", 1, poly([(0, 0), (BS, 0), (BS, BS), (WEDGE_DX1, BS), (WEDGE_DX1, BS + WEDGE_H), (WEDGE_DX0, BS + WEDGE_H), (WEDGE_DX0, BS), (0, BS)]), tone=0.5, regions=["bearers", "wedges"], material="softwood-bearer"))
        else:
            s.append(shape(f"{bb}-end", bb, "body", 1, rect(0, 0, BS, BS), tone=0.45, region="bearers", material="softwood-bearer", hatch="end-grain" if scale in (50, 100) else None))
            s.append(shape(f"{bb}-chock", bb, "body", 3, rect(WEDGE_DX0, BS, WEDGE_DX1 - WEDGE_DX0, WEDGE_H), tone=0.6, region="wedges", material="softwood-bearer"))
    return s


def front(scale):
    """🖼️ The front elevation of one scale (rest world: lateral z right, height up), seen from ahead on the sawn end."""
    s = []
    top = SOFFIT + H
    gregions = ["sawn-end"] + (["steel-shoe"] if scale == 500 else [])
    s.append(shape("girder-sawn-end", "girder", "body", 5, rect(-HW, SOFFIT, B, H), tone=0.8, regions=gregions if scale == 500 else None,
                   region=None if scale == 500 else "sawn-end", material="glulam-sawn", hatch="end-grain" if scale in (50, 100) else None))
    if scale == 50:
        for k in range(1, NLAM):
            y = SOFFIT + LAM * k
            s.append(shape(f"end-glue-line-{k:02d}", "girder", "texture", 5.2, poly([(-HW, y), (HW, y)], False), False, 0.3, region="sawn-end"))
        for j, (y0, bend) in enumerate([(0.35, 0.03), (0.62, -0.025), (0.88, 0.02)]):
            s.append(shape(f"saw-kerf-{j}", "girder", "texture", 5.3, poly([(-HW, y0), (0.0, y0 + bend), (HW, y0)], False), False, 0.35, region="sawn-end"))
    if scale == 100:
        y0, y1, y2 = SOFFIT - SHOE_BASE_T, SOFFIT, SOFFIT + SHOE_SIDE_H
        xo, xs, xi = SHOE_BASE_W / 2, HW + SHOE_SIDE_T, HW
        s.append(shape("shoe", "shoe", "body", 2, poly([(-xo, y0), (xo, y0), (xo, y1), (xs, y1), (xs, y2), (xi, y2), (xi, y1), (-xi, y1), (-xi, y2), (-xs, y2), (-xs, y1), (-xo, y1)]), tone=0.25, region="steel-shoe", material="steel-primed"))
    elif scale != 500:
        base_t = SHOE_BASE_T if scale != 200 else FEATURE[200]
        s.append(shape("shoe-side-plate-far", "shoe", "body", 2, rect(HW, SOFFIT, SHOE_SIDE_T, SHOE_SIDE_H), tone=0.25, region="steel-shoe", material="steel-primed"))
        s.append(shape("shoe-side-plate-near", "shoe", "body", 2, rect(-HW - SHOE_SIDE_T, SOFFIT, SHOE_SIDE_T, SHOE_SIDE_H), tone=0.25, region="steel-shoe", material="steel-primed"))
        s.append(shape("shoe-base-plate", "shoe", "body", 2, rect(-SHOE_BASE_W / 2, SOFFIT - SHOE_BASE_T, SHOE_BASE_W, base_t), tone=0.22, region="steel-shoe", material="steel-primed"))
    if scale == 50:
        yb = GY + BOLT_V
        s.append(shape("bolt-heads", "shoe", "detail", 2.5, rect(-HW - SHOE_SIDE_T - HEAD_T, yb - BOLT_S / 2, HEAD_T, BOLT_S), tone=0.15, region="steel-shoe", material="steel-primed"))
        s.append(shape("bolt-nuts", "shoe", "detail", 2.5, rect(HW + SHOE_SIDE_T, yb - BOLT_S / 2, NUT_T, BOLT_S), tone=0.15, region="steel-shoe", material="steel-primed"))
    if scale != 500:
        for c in ("choker-a", "choker-b"):
            s.append(shape(f"{c}-loop", c, "body", 1, rect(-HW - RS * 2, SOFFIT - 2 * RS, B + 4 * RS, H + 4 * RS), tone=0.35, region="slings", material="polyester-sling"))
    half = RS if scale != 500 else FEATURE[500] / 2
    for leg in ("sling-leg-a", "sling-leg-b"):
        s.append(leg_solid_shape(f"{leg}-strap", leg, 6, 0.35, half))
    bb = "bearer-3"
    if scale in (100, 500):
        prof = [(-BLEN / 2, 0), (BLEN / 2, 0), (BLEN / 2, BS), (HW + WEDGE_W, BS), (HW, BS + WEDGE_H), (HW, BS), (-HW, BS), (-HW, BS + WEDGE_H), (-HW - WEDGE_W, BS), (-BLEN / 2, BS)]
        s.append(shape(f"{bb}-block", bb, "body", 3, poly(prof), tone=0.45 if scale == 500 else 0.5, regions=["bearers", "wedges"], material="softwood-bearer"))
    else:
        s.append(shape(f"{bb}-side", bb, "body", 3, rect(-BLEN / 2, 0, BLEN, BS), tone=0.45, region="bearers", material="softwood-bearer"))
        s.append(shape(f"{bb}-chock-far", bb, "body", 4, poly([(HW, BS), (HW + WEDGE_W, BS), (HW, BS + WEDGE_H)]), tone=0.6, region="wedges", material="softwood-bearer"))
        s.append(shape(f"{bb}-chock-near", bb, "body", 4, poly([(-HW, BS), (-HW, BS + WEDGE_H), (-HW - WEDGE_W, BS)]), tone=0.6, region="wedges", material="softwood-bearer"))
    return s


def plan(scale):
    """🖼️ The plan of one scale (rest world: ahead right, lateral up)."""
    s = []
    s.append(shape("girder-top", "girder", "body", 3, rect(0, -HW, L, B), tone=0.55, region="girder-top", material="glulam-weathered", hatch="grain" if scale == 100 else None))
    if scale == 50:
        for j, p in enumerate(PURLINS):
            s.append(shape(f"purlin-seat-{j}", "girder", "texture", 3.2, poly([(p - 0.06, -HW), (p - 0.06, HW), (p + 0.06, HW), (p + 0.06, -HW)], False), False, 0.35, region="girder-top"))
    if scale == 200:
        s.append(shape("girder-axis", "girder", "detail", 3.3, poly([(SHOE_L, 0), (L, 0)], False), False, 0.35, region="girder-top"))
    if scale != 500:
        s.append(shape("shoe-base-plate", "shoe", "body", 1, rect(0, -SHOE_BASE_W / 2, SHOE_L, SHOE_BASE_W), tone=0.22, region="steel-shoe", material="steel-primed"))
        s.append(shape("shoe-side-plate-far", "shoe", "body", 2, rect(0, HW, SHOE_L, SHOE_SIDE_T), tone=0.25, region="steel-shoe", material="steel-primed"))
        s.append(shape("shoe-side-plate-near", "shoe", "body", 2, rect(0, -HW - SHOE_SIDE_T, SHOE_L, SHOE_SIDE_T), tone=0.25, region="steel-shoe", material="steel-primed"))
    if scale == 50:
        for j, u in enumerate(BOLTS_U):
            s.append(shape(f"bolt-head-{j}", "shoe", "detail", 2.5, rect(u - BOLT_S / 2, -HW - SHOE_SIDE_T - HEAD_T, BOLT_S, HEAD_T), tone=0.15, region="steel-shoe", material="steel-primed"))
            s.append(shape(f"bolt-nut-{j}", "shoe", "detail", 2.5, rect(u - BOLT_S / 2, HW + SHOE_SIDE_T, BOLT_S, NUT_T), tone=0.15, region="steel-shoe", material="steel-primed"))
    if scale != 500:
        for c, xc in (("choker-a", XA), ("choker-b", XB)):
            s.append(shape(f"{c}-top", c, "body", 4, rect(xc - RS, -HW - 2 * RS, 2 * RS, B + 4 * RS), tone=0.35, region="slings", material="polyester-sling"))
    half = RS if scale != 500 else FEATURE[500] / 2
    for leg in ("sling-leg-a", "sling-leg-b"):
        s.append(leg_solid_shape(f"{leg}-strap", leg, 5, 0.35, half))
    for k, xc in enumerate(BEARERS):
        bb = f"bearer-{k + 1}"
        if scale == 500:
            s.append(shape(f"{bb}-top", bb, "body", 1, rect(0, -BLEN / 2, BS, BLEN), tone=0.45, regions=["bearers", "wedges"], material="softwood-bearer"))
        else:
            s.append(shape(f"{bb}-top", bb, "body", 1, rect(0, -BLEN / 2, BS, BLEN), tone=0.45, region="bearers", material="softwood-bearer"))
            s.append(shape(f"{bb}-chock-far", bb, "body", 2, rect(WEDGE_DX0, HW, WEDGE_DX1 - WEDGE_DX0, WEDGE_W), tone=0.6, region="wedges", material="softwood-bearer"))
            s.append(shape(f"{bb}-chock-near", bb, "body", 2, rect(WEDGE_DX0, -HW - WEDGE_W, WEDGE_DX1 - WEDGE_DX0, WEDGE_W), tone=0.6, region="wedges", material="softwood-bearer"))
    return s


def fix_plan_bearer_x(shapes):
    """📍️ Plan and front shapes are drawn in rest world coordinates: shift bearer shapes from bone-local x to their world x."""
    for s in shapes:
        if s["bone"].startswith("bearer-"):
            k = int(s["bone"][-1]) - 1
            x0 = BEARERS[k] - BS / 2
            if s["kind"] == "rect":
                s["x"] = r6(s["x"] + x0)
            elif s["kind"] == "poly":
                s["points"] = [[r6(a + x0), b] for a, b in s["points"]]
    return shapes


def shift_plan_shoe(shapes):
    """📍️ Plan shapes of the girder subtree keep their x; nothing to shift because girder and shoe start at world x 0."""
    return shapes


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
        100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
        200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25},
        500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {50: [{"id": "end-grain", "title": t("Timber, end grain", "Holz, Hirnholz"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
           100: [{"id": "grain", "title": t("Timber, longitudinal (lamellae)", "Holz, längs (Lamellen)"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.4, "penMm": 0.1},
                 {"id": "end-grain", "title": t("Timber, end grain", "Holz, Hirnholz"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
           200: [], 500: []}

REP_DECISIONS = {
    50: {
        "silhouette": t("Side elevation of the 12 m girder on edge (1000 mm deep, 20 mm on paper) on its three 120 mm bearers with the timber chocks, the steel shoe at the left end, the two roundsling chokers and the two sling legs lying slack on the top face. Front view onto the fresh sawn end (160 x 1000 mm) with its 25 lamellae, the shoe plates behind it, the choker loops and the nearest bearer with its two chocks. Plan: the top face, the shoe base plate showing beside it, the three bearers and six chocks below, chokers and legs above.",
                        "Seitenansicht des 12 m langen hochkant liegenden Trägers (1000 mm hoch, 20 mm auf Papier) auf seinen drei 120-mm-Lagerhölzern mit den Holzkeilen, dem Stahlschuh am linken Ende, den beiden Rundschlingen-Schnürgängen und den beiden lose auf der Oberseite liegenden Anschlagsträngen. Vorderansicht auf das frisch gesägte Ende (160 x 1000 mm) mit seinen 25 Lamellen, dahinter die Schuhbleche, die Schlingen und das vorderste Lagerholz mit seinen zwei Keilen. Draufsicht: die Oberseite, daneben das Fußblech des Schuhs, darunter die drei Lagerhölzer und sechs Keile, darüber Schlingen und Stränge."),
        "retained": t("All 24 glue lines of the 40 mm lamellae (0.8 mm apart) in side and end view, the four empty 21 mm bolt holes of a removed bracing connection, the two M20 bolts of the shoe with hexagon heads, the 10 mm side plates and the 20 mm base plate, the traces of five removed purlin hangers on the side face and their seats on the top face, three saw kerf arcs on the sawn end.",
                      "Alle 24 Leimfugen der 40-mm-Lamellen (0,8 mm Abstand) in Seiten- und Stirnansicht, die vier leeren 21-mm-Bolzenlöcher eines ausgebauten Verbandsanschlusses, die beiden M20-Bolzen des Schuhs mit Sechskantköpfen, die 10-mm-Seitenbleche und das 20-mm-Fußblech, die Spuren von fünf ausgebauten Pfettenschuhen an der Seitenfläche und ihre Auflager auf der Oberseite, drei Sägebögen auf dem gesägten Ende."),
        "merged": t("Side faces, top, soffit and the old end face are one girder outline in side view; the far chock of each bearer lies behind the near one.", "Seitenflächen, Oberseite, Unterseite und die alte Stirnfläche sind in der Seitenansicht ein Trägerumriss; der hintere Keil jedes Lagerholzes liegt hinter dem vorderen."),
        "omitted": t("Nail holes of the purlin hangers (4 mm), washers and thread ends of the bolts, the arris chamfers of a few millimetres and the sling labels are below the 7.5 mm smallest feature; the two hidden bearers are not repeated in the front view.", "Nagellöcher der Pfettenschuhe (4 mm), Unterlegscheiben und Gewindeenden der Bolzen, die Fasen von wenigen Millimetern und die Schlingenetiketten liegen unter der kleinsten Einzelheit von 7,5 mm; die beiden verdeckten Lagerhölzer werden in der Vorderansicht nicht wiederholt."),
        "thinElements": t("The 35 mm roundslings are closed strips of their true width; the 10 mm side plates are closed strips (0.2 mm); the bolt holes are filled dark discs.", "Die 35-mm-Rundschlingen sind geschlossene Streifen ihrer wahren Breite; die 10-mm-Seitenbleche sind geschlossene Streifen (0,2 mm); die Bolzenlöcher sind gefüllte dunkle Scheiben."),
        "hatching": t("End grain is dotted on the sawn girder end and on the bearer ends; the glue lines are the grain of the side face.", "Hirnholz ist auf dem gesägten Trägerende und auf den Stirnseiten der Lagerhölzer gepunktet; die Leimfugen sind die Faser der Seitenfläche."),
        "styles": t("Outline 0.35 mm, bolts and holes 0.18 mm, glue lines, traces and kerfs 0.1 mm.", "Kontur 0,35 mm, Bolzen und Löcher 0,18 mm, Leimfugen, Spuren und Sägebögen 0,1 mm."),
        "overlays": t("Weathered faces are mid grey, the fresh sawn end light, the primed shoe dark, the slings a darker mid tone.", "Verwitterte Flächen sind mittelgrau, das frisch gesägte Ende hell, der grundierte Schuh dunkel, die Schlingen ein dunklerer Mittelton.")},
    100: {
        "silhouette": t("Side elevation of girder, shoe plates, chokers, sling legs, bearers and chocks at true size; front view onto the sawn end with shoe plates, choker loops, nearest bearer and chocks; plan with the top face, base plate, bearers, chocks, chokers and legs.", "Seitenansicht von Träger, Schuhblechen, Schlingen, Anschlagsträngen, Lagerhölzern und Keilen in wahrer Größe; Vorderansicht auf das gesägte Ende mit Schuhblechen, Schlingen, vorderstem Lagerholz und Keilen; Draufsicht mit Oberseite, Fußblech, Lagerhölzern, Keilen, Schlingen und Strängen."),
        "retained": t("The four empty bolt holes (21 mm, 0.21 mm on paper), the 10 mm side plates and the 20 mm base plate, all chocks and both chokers.", "Die vier leeren Bolzenlöcher (21 mm, 0,21 mm auf Papier), die 10-mm-Seitenbleche und das 20-mm-Fußblech, alle Keile und beide Schlingen."),
        "merged": t("The 25 lamellae become a line hatch at their 40 mm pitch (0.4 mm on paper) instead of single glue lines.", "Die 25 Lamellen werden statt einzelner Leimfugen eine Linienschraffur in ihrer Teilung von 40 mm (0,4 mm auf Papier)."),
        "omitted": t("Bolt heads and nuts (13 to 16 mm), the hanger traces, purlin seats and saw kerfs.", "Bolzenköpfe und Muttern (13 bis 16 mm), die Spuren der Pfettenschuhe, die Pfettenauflager und die Sägebögen."),
        "thinElements": t("Roundslings and side plates stay closed strips of their true width.", "Rundschlingen und Seitenbleche bleiben geschlossene Streifen ihrer wahren Breite."),
        "hatching": t("Lamella hatch on the side face and the top, end-grain dots on the sawn end and the bearer ends.", "Lamellenschraffur auf der Seitenfläche und der Oberseite, Hirnholzpunkte auf dem gesägten Ende und den Stirnseiten der Lagerhölzer."),
        "styles": t("Outline 0.25 mm, holes 0.13 mm, hatches 0.1 mm.", "Kontur 0,25 mm, Löcher 0,13 mm, Schraffuren 0,1 mm."),
        "overlays": t("Tones as at 1:50.", "Töne wie bei 1:50.")},
    200: {
        "silhouette": t("Girder outline with shoe, chokers and sling legs, the three bearers with their chocks; front view of the sawn end with shoe plates, loops, bearer and chocks; plan with the girder axis.", "Trägerumriss mit Schuh, Schlingen und Anschlagsträngen, die drei Lagerhölzer mit ihren Keilen; Vorderansicht des gesägten Endes mit Schuhblechen, Schlingen, Lagerholz und Keilen; Draufsicht mit der Trägerachse."),
        "retained": t("Shoe side plates, chokers, legs and chocks keep their true size; the base plate is generalised to the 30 mm smallest feature upwards into the soffit line, so the bearing level stays true.", "Schuhseitenbleche, Schlingen, Stränge und Keile behalten ihre wahre Größe; das Fußblech ist nach oben in die Unterkante hinein auf die kleinste Einzelheit von 30 mm verallgemeinert, sodass die Auflagerhöhe wahr bleibt."),
        "merged": t("The lamellae merge into one girder tone.", "Die Lamellen gehen in einem Trägerton auf."),
        "omitted": t("Bolt holes (21 mm), bolts, hatches and all texture.", "Bolzenlöcher (21 mm), Bolzen, Schraffuren und alle Textur."),
        "thinElements": t("Roundslings (35 mm, 0.18 mm on paper) and the 10 mm side plates stay closed strips.", "Rundschlingen (35 mm, 0,18 mm auf Papier) und die 10-mm-Seitenbleche bleiben geschlossene Streifen."),
        "hatching": t("No hatches.", "Keine Schraffuren."),
        "styles": t("Outline 0.18 mm, girder axis 0.1 mm.", "Kontur 0,18 mm, Trägerachse 0,1 mm."),
        "overlays": t("Tones as at 1:50.", "Töne wie bei 1:50.")},
    500: {
        "silhouette": t("Girder outline with the shoe base stepped 20 mm below the soffit at its left end, the sling legs as 75 mm strips, three bearer blocks with their chocks merged; front view of the sawn end over the merged bearer and chocks; plan with girder, bearers and legs.", "Trägerumriss mit dem Schuhfuß 20 mm unter der Unterkante am linken Ende, die Anschlagstränge als 75-mm-Streifen, drei Lagerholzblöcke mit eingeschlossenen Keilen; Vorderansicht des gesägten Endes über Lagerholz und Keilen; Draufsicht mit Träger, Lagerhölzern und Strängen."),
        "retained": t("Girder, bearers and legs in their true places; the legs are widened to the 75 mm smallest feature (0.15 mm on paper), so they print to 1:1000.", "Träger, Lagerhölzer und Stränge an ihren wahren Orten; die Stränge sind auf die kleinste Einzelheit von 75 mm (0,15 mm auf Papier) verbreitert, damit sie bis 1:1000 drucken."),
        "merged": t("Shoe plates into the girder end, each pair of chocks into its bearer block (chocks widened to the full bearer width).", "Schuhbleche in das Trägerende, jedes Keilpaar in seinen Lagerholzblock (Keile auf die volle Breite des Lagerholzes verbreitert)."),
        "omitted": t("Chokers (35 mm loops), bolts, holes and all texture.", "Schlingen (35-mm-Schlaufen), Bolzen, Löcher und alle Textur."),
        "thinElements": t("None below the feature remain.", "Es bleiben keine Teile unter der Einzelheit."),
        "hatching": t("No hatches.", "Keine Schraffuren."),
        "styles": t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": t("Two tones: timber and slings.", "Zwei Töne: Holz und Schlingen.")},
}


def representation(scale):
    """🗺️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale],
            "geometry": {"elevation": elevation(scale), "front": fix_plan_bearer_x(front(scale)) if False else front(scale), "plan": fix_plan_bearer_x(plan(scale))},
            "decisions": REP_DECISIONS[scale]}


# ---------------------------------------------------------------- 3D

def body(scale):
    """🧊️ The body list of one scale."""
    out = []
    if scale == 50:
        for k in range(NLAM):
            v0, v1 = -HB + LAM * k, -HB + LAM * (k + 1)
            holes = [circle_pts(u, v, HOLE_R) for (u, v) in HOLES if v0 < v < v1]
            out.append(extrude(f"lamella-{k:02d}", "girder", "glulam-weathered", 0.48 + 0.04 * (k % 2), [(0, v0), (L, v0), (L, v1), (0, v1)], "xy", -HW, HW, holes=holes or None))
    elif scale == 100:
        out.append(extrude("girder", "girder", "glulam-weathered", 0.5, [(0, -HB), (L, -HB), (L, HB), (0, HB)], "xy", -HW, HW, holes=[circle_pts(u, v, HOLE_R, 12) for (u, v) in HOLES]))
    elif scale == 200:
        out.append(box("girder", "girder", "glulam-weathered", 0.5, (L, H, B), (L / 2, 0, 0)))
    else:
        out.append(extrude("girder", "girder", "glulam-weathered", 0.45, [(0, -HB - SHOE_BASE_T), (SHOE_L, -HB - SHOE_BASE_T), (SHOE_L, -HB), (L, -HB), (L, HB), (0, HB)], "xy", -HW, HW))
    if scale != 500:
        bt = SHOE_BASE_T if scale != 200 else FEATURE[200]
        sheet = {"sheet": True} if scale in (100, 200) else {}
        out.append(box("shoe-base-plate", "shoe", "steel-primed", 0.22, (SHOE_L, bt, SHOE_BASE_W), (SHOE_L / 2, -HB - SHOE_BASE_T + bt / 2, 0)))
        out.append(box("shoe-side-plate-near", "shoe", "steel-primed", 0.25, (SHOE_L, SHOE_SIDE_H, SHOE_SIDE_T), (SHOE_L / 2, -HB + SHOE_SIDE_H / 2, -HW - SHOE_SIDE_T / 2), **sheet))
        out.append(box("shoe-side-plate-far", "shoe", "steel-primed", 0.25, (SHOE_L, SHOE_SIDE_H, SHOE_SIDE_T), (SHOE_L / 2, -HB + SHOE_SIDE_H / 2, HW + SHOE_SIDE_T / 2), **sheet))
    if scale == 50:
        for j, u in enumerate(BOLTS_U):
            out.append(extrude(f"bolt-head-{j}", "shoe", "steel-primed", 0.15, hexagon(u, BOLT_V, BOLT_S), "xy", -HW - SHOE_SIDE_T - HEAD_T, -HW - SHOE_SIDE_T))
            out.append(extrude(f"bolt-nut-{j}", "shoe", "steel-primed", 0.15, hexagon(u, BOLT_V, BOLT_S), "xy", HW + SHOE_SIDE_T, HW + SHOE_SIDE_T + NUT_T))
    if scale != 500:
        for c in ("choker-a", "choker-b"):
            out.append(extrude(f"{c}-loop", c, "polyester-sling", 0.35, [(-HB - 2 * RS, -HW - 2 * RS), (HB + 2 * RS, -HW - 2 * RS), (HB + 2 * RS, HW + 2 * RS), (-HB - 2 * RS, HW + 2 * RS)], "yz", -RS, RS,
                               holes=[[(-HB, -HW), (HB, -HW), (HB, HW), (-HB, HW)]]))
    for leg in ("sling-leg-a", "sling-leg-b"):
        out.append(cylinder_u(f"{leg}-strap", leg, "polyester-sling", 0.35, LEG, RS, slender=True))
    for k in range(3):
        bb = f"bearer-{k + 1}"
        if scale == 500:
            prof = [(0, -BLEN / 2), (0, BLEN / 2), (BS, BLEN / 2), (BS, HW + WEDGE_W), (BS + WEDGE_H, HW), (BS, HW), (BS, -HW), (BS + WEDGE_H, -HW), (BS, -HW - WEDGE_W), (BS, -BLEN / 2)]
            out.append(extrude(f"{bb}-block", bb, "softwood-bearer", 0.45, prof, "yz", 0, BS))
        else:
            out.append(box(f"{bb}-timber", bb, "softwood-bearer", 0.43 + 0.02 * k, (BS, BS, BLEN), (BS / 2, BS / 2, 0)))
            for side, prof in zip(("far", "near"), wedge_profiles()):
                out.append(extrude(f"{bb}-chock-{side}", bb, "softwood-bearer", 0.6, prof, "yz", WEDGE_DX0, WEDGE_DX1))
    return out


def thermal(scale):
    """🌡️ The thermal list of one scale."""
    out = []
    if scale == 50:
        out += girder_skins(0.024, 0.030, 0.036, 0.5, "")
    elif scale == 100:
        out += girder_skins(0.028, 0.034, 0.040, 0.5, "")
    elif scale == 200:
        out += tri_prisms(0.036, L - 0.036, 0.036, "")
    else:
        out += tri_prisms(0.075, L - 0.075, 0.075, "")
    if scale != 500:
        for n in ("shoe-base-plate", "shoe-side-plate-near", "shoe-side-plate-far"):
            out.append({"id": f"th-{n}", "of": n, "region": "steel-shoe"})
    else:
        for side, w0 in (("near", -HW - FEATURE[500]), ("far", HW)):
            out.append(box(f"th-shoe-{side}", "shoe", "steel-primed", 0.25, (SHOE_L, SHOE_SIDE_H, FEATURE[500]), (SHOE_L / 2, -HB + SHOE_SIDE_H / 2, w0 + FEATURE[500] / 2), region="steel-shoe"))
    if scale == 50:
        for j in range(2):
            out.append({"id": f"th-bolt-head-{j}", "of": f"bolt-head-{j}", "region": "steel-shoe"})
            out.append({"id": f"th-bolt-nut-{j}", "of": f"bolt-nut-{j}", "region": "steel-shoe"})
    if scale != 500:
        for c in ("choker-a", "choker-b"):
            out.append({"id": f"th-{c}-loop", "of": f"{c}-loop", "region": "slings"})
    for leg in ("sling-leg-a", "sling-leg-b"):
        out.append({"id": f"th-{leg}", "of": f"{leg}-strap", "region": "slings"})
    for k in range(3):
        bb = f"bearer-{k + 1}"
        if scale == 500:
            out.append(box(f"th-{bb}-timber", bb, "softwood-bearer", 0.45, (BS, BS, BLEN), (BS / 2, BS / 2, 0), region="bearers"))
            for side, prof in zip(("far", "near"), wedge_profiles()):
                out.append(extrude(f"th-{bb}-chock-{side}", bb, "softwood-bearer", 0.6, prof, "yz", 0, BS, region="wedges"))
        else:
            out.append({"id": f"th-{bb}-timber", "of": f"{bb}-timber", "region": "bearers"})
            for side in ("far", "near"):
                out.append({"id": f"th-{bb}-chock-{side}", "of": f"{bb}-chock-{side}", "region": "wedges"})
    return out


def structure(scale):
    """🏗️ The structure list of one scale."""
    f = FEATURE[scale]
    out = [box("girder-member", "girder", "glulam-weathered", 0.6, (L, H, B), (L / 2, 0, 0), member="girder", part="member")]
    for k in range(3):
        bb = f"bearer-{k + 1}"
        if scale == 500:
            out.append(box(f"{bb}-member", bb, "softwood-bearer", 0.5, (BS, BS, BLEN), (BS / 2, BS / 2, 0), member=bb, part="member"))
        else:
            out.append({"id": f"{bb}-member", "of": f"{bb}-timber", "member": bb, "part": "member"})
    for leg in ("sling-leg-a", "sling-leg-b"):
        out.append({"id": f"{leg}-member", "of": f"{leg}-strap", "member": leg, "part": "member"})
    if scale != 500:
        out.append(box("joint-load-carrier", "load-carrier", "steel-primed", 0.3, (0.1, f, B), (0, f / 2, 0), member="load-carrier", part="joint"))
    r = {50: 0.03, 100: 0.035, 200: 0.04, 500: 0.045}[scale]
    out.append(sphere("mass-shoe", "shoe", "steel-primed", 0.3, r, (SHOE_L / 2, SHOE_COMY, 0), member="shoe", part="mass"))
    return out


VOL_DECISIONS = {
    50: [t("Body: the girder as it was glued: 25 lamellae of 40 mm, each one extrude over the full 12 m, alternating a shade in tone; the four empty 21 mm holes of a removed bracing connection are true holes through lamellae 16 and 21. The steel shoe is its 20 mm base plate, two 10 mm side plates and two M20 bolts with hexagon heads (s 30, k 13) and nuts (m 16). Each roundsling choker is a 35 mm band wrapped tight around the section; the two 4 m legs are 35 mm cylinders lying slack along the top face, side by side. Three 120 x 120 x 1000 mm squared bearers carry the girder on its soffit, each with two triangular chocks (150 x 100 x 80 mm) against the side faces.",
           "Körper: der Träger, wie er verleimt wurde: 25 Lamellen zu 40 mm, jede ein Extrusionskörper über die vollen 12 m, im Ton um eine Nuance wechselnd; die vier leeren 21-mm-Löcher eines ausgebauten Verbandsanschlusses sind echte Löcher durch die Lamellen 16 und 21. Der Stahlschuh ist sein 20-mm-Fußblech, zwei 10-mm-Seitenbleche und zwei M20-Bolzen mit Sechskantköpfen (SW 30, k 13) und Muttern (m 16). Jeder Rundschlingen-Schnürgang ist ein 35-mm-Band, eng um den Querschnitt gelegt; die beiden 4 m langen Stränge sind 35-mm-Zylinder, die lose nebeneinander auf der Oberseite liegen. Drei Kanthölzer 120 x 120 x 1000 mm tragen den Träger an seiner Unterseite, jedes mit zwei dreieckigen Keilen (150 x 100 x 80 mm) an den Seitenflächen."),
         t("Thermal: the girder faces are skins of their effective storage depth (sides 36 mm, top and soffit 30 mm, ends 24 mm) built as six frusta of one picture frame, so no volume belongs to two regions; the three depths differ so that every mitre leans less than 45 degrees towards one axis and is declared a section face with the inner face. Shoe, bolts, slings, bearers and chocks are referenced from the body.",
           "Thermisch: die Trägerflächen sind Schalen ihrer wirksamen Speichertiefe (Seiten 36 mm, Ober- und Unterseite 30 mm, Enden 24 mm) als sechs Pyramidenstümpfe eines Bilderrahmens, sodass kein Volumen zwei Regionen gehört; die drei Tiefen sind verschieden, damit jede Gehrung um weniger als 45 Grad zu einer Achse geneigt ist und mit der Innenfläche als Schnittfläche erklärt wird. Schuh, Bolzen, Schlingen, Lagerhölzer und Keile sind aus dem Körper referenziert."),
         t("Structure: the member is the true 160 x 1000 mm GL24h section over 12 m; the bearers are members of their own squared section and the sling legs members along their axis; the load slider is a 100 x 160 x 7.5 mm steel load plate on the top face; the shoe mass (32.6 kg) is a marker at its centre. Supports are the contact anchors and support anchors each pose names, so stored, lifted and reinstalled each have their own load path.",
           "Tragwerk: das Glied ist der wahre Querschnitt GL24h 160 x 1000 mm über 12 m; die Lagerhölzer sind Glieder ihres eigenen Kantholzquerschnitts und die Anschlagstränge Glieder entlang ihrer Achse; der Lastschlitten ist eine Stahllastplatte 100 x 160 x 7,5 mm auf der Oberseite; die Masse des Schuhs (32,6 kg) ist eine Marke in seinem Schwerpunkt. Auflager sind die Kontakt- und Auflageranker, die jede Pose nennt, sodass Lagern, Anheben und Wiedereinbau je ihren eigenen Lastpfad haben.")],
    100: [t("Body: the lamellae merge into one 160 x 1000 mm extrude with the four 21 mm bolt holes kept as holes; the bolts (13 to 16 mm) are below the 15 mm feature and left out; the 10 mm side plates are declared thin sheets (300 x 400 mm, 30 times their thickness) and keep their gauge; base plate, chokers, legs, bearers and chocks as at 1:50.",
            "Körper: die Lamellen gehen in einem Extrusionskörper 160 x 1000 mm auf, die vier 21-mm-Bolzenlöcher bleiben Löcher; die Bolzen (13 bis 16 mm) liegen unter der Einzelheit von 15 mm und entfallen; die 10-mm-Seitenbleche sind als dünne Bleche erklärt (300 x 400 mm, das 30-fache ihrer Dicke) und behalten ihre Stärke; Fußblech, Schlingen, Stränge, Lagerhölzer und Keile wie bei 1:50."),
          t("Thermal: the same picture frame of six skins with depths rounded for the coarser scale (sides 40, top and soffit 34, ends 28 mm); shoe, slings, bearers and chocks referenced from the body.",
            "Thermisch: derselbe Bilderrahmen aus sechs Schalen mit für den gröberen Maßstab gerundeten Tiefen (Seiten 40, Ober- und Unterseite 34, Enden 28 mm); Schuh, Schlingen, Lagerhölzer und Keile aus dem Körper referenziert."),
          t("Minimal: the structure keeps the members of 1:50 because each is the whole load path of one bone; the load plate is 15 mm and the shoe mass marker 70 mm across.",
            "Minimal: das Tragwerk behält die Glieder von 1:50, weil jedes der ganze Lastpfad eines Knochens ist; die Lastplatte ist 15 mm dick und die Massenmarke des Schuhs 70 mm groß.")],
    200: [t("Body: the girder is one 160 x 1000 x 12000 mm block (the 21 mm holes are below the 30 mm feature); the base plate is generalised to 30 mm upwards into the soffit so the bearing level stays true; the side plates remain declared sheets; chokers (35 mm), legs, bearers and chocks keep their true size.",
            "Körper: der Träger ist ein Block 160 x 1000 x 12000 mm (die 21-mm-Löcher liegen unter der Einzelheit von 30 mm); das Fußblech ist nach oben in die Unterkante hinein auf 30 mm verallgemeinert, sodass die Auflagerhöhe wahr bleibt; die Seitenbleche bleiben erklärte Bleche; Schlingen (35 mm), Stränge, Lagerhölzer und Keile behalten ihre wahre Größe."),
          t("Thermal: four triangular face prisms that share the section at the girder axis (their inner faces are section faces) and two 36 mm full-section end blocks; the rest is referenced from the body.",
            "Thermisch: vier dreieckige Flächenprismen, die sich den Querschnitt an der Trägerachse teilen (ihre Innenflächen sind Schnittflächen), und zwei 36-mm-Endblöcke über den vollen Querschnitt; der Rest ist aus dem Körper referenziert."),
          t("Minimal: the structure keeps the members of 1:100; the load plate is 30 mm and the shoe mass marker 80 mm across.",
            "Minimal: das Tragwerk behält die Glieder von 1:100; die Lastplatte ist 30 mm dick und die Massenmarke des Schuhs 80 mm groß.")],
    500: [t("Body: the girder is one extrude whose outline steps 20 mm down over the 300 mm of the shoe, taking the shoe plates in; each bearer and its two chocks are one extrude with the chocks widened to the full 120 mm of the bearer; the chokers (35 mm bands) are below the 75 mm feature and left out; the legs keep their true 35 mm section as slender solids.",
            "Körper: der Träger ist ein Extrusionskörper, dessen Umriss über die 300 mm des Schuhs um 20 mm absetzt und die Schuhbleche aufnimmt; jedes Lagerholz und seine zwei Keile sind ein Extrusionskörper, die Keile auf die vollen 120 mm des Lagerholzes verbreitert; die Schlingen (35-mm-Bänder) liegen unter der Einzelheit von 75 mm und entfallen; die Stränge behalten als schlanke Körper ihren wahren Querschnitt von 35 mm."),
          t("Thermal: four triangular face prisms and two 75 mm end blocks; the shoe region is two 75 mm plates beside the girder end; bearers and chocks are separate thermal solids of their own regions inside the merged body block.",
            "Thermisch: vier dreieckige Flächenprismen und zwei 75-mm-Endblöcke; die Schuhregion sind zwei 75-mm-Platten neben dem Trägerende; Lagerhölzer und Keile sind eigene thermische Körper ihrer Regionen im zusammengefassten Körperblock."),
          t("Structure: girder, bearer and leg members and the shoe mass marker (90 mm); the load plate is left out because the moving load is a glyph at this scale.",
            "Tragwerk: Glieder von Träger, Lagerhölzern und Strängen und die Massenmarke des Schuhs (90 mm); die Lastplatte entfällt, weil die wandernde Last bei diesem Maßstab ein Zeichen ist.")],
}


def volume(scale):
    """🧊️ One authored volume."""
    return {"status": "authored", "denominator": scale, "body": body(scale), "thermal": thermal(scale), "structure": structure(scale), "decisions": VOL_DECISIONS[scale]}


# ---------------------------------------------------------------- physics

MATERIALS = [
    {"id": "glulam-weathered", "title": t("Glulam GL24h, weathered surface", "Brettschichtholz GL24h, verwitterte Oberfläche"), "emissivity": 0.9, "solarAbsorptivity": 0.75,
     "densityKgM3": RHO_GL, "elasticModulusPa": 11500000000, "strengthPa": 24000000, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-14080"},
    {"id": "glulam-sawn", "title": t("Glulam GL24h, fresh sawn end grain", "Brettschichtholz GL24h, frisch gesägtes Hirnholz"), "emissivity": 0.9, "solarAbsorptivity": 0.45,
     "densityKgM3": RHO_GL, "elasticModulusPa": 11500000000, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "source": "authored-estimate", "reference": "ashrae-fundamentals"},
    {"id": "steel-primed", "title": t("Steel S235, primed and weathered", "Stahl S235, grundiert und bewittert"), "emissivity": 0.88, "solarAbsorptivity": 0.7,
     "densityKgM3": RHO_ST, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "conductivityWmK": 50, "specificHeatJKgK": 450, "source": "literature", "reference": "en-10025-2"},
    {"id": "softwood-bearer", "title": t("Squared softwood C24, weathered", "Kantholz Nadelholz C24, verwittert"), "emissivity": 0.9, "solarAbsorptivity": 0.72,
     "densityKgM3": RHO_C24, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-338"},
    {"id": "polyester-sling", "title": t("Polyester roundsling", "Polyester-Rundschlinge"), "emissivity": 0.9, "solarAbsorptivity": 0.6,
     "densityKgM3": 1380, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "en-1492-2"},
]

REFERENCES = [
    ("en-14080", "EN 14080:2013 Timber structures, glued laminated timber and glued solid timber, requirements; Table 5: GL24h f_m,k 24 MPa, E0,mean 11.5 GPa, rho_mean 420 kg/m3; lamella thickness up to 45 mm."),
    ("din-1052-1988", "DIN 1052-1:1988 Holzbauwerke, Berechnung und Ausführung: Brettschichtholz BS 11 and BS 14, the classes of hall girders of the period, today graded as GL24h and GL28h."),
    ("en-338", "EN 338:2016 Structural timber, strength classes: C24 rho_mean 420 kg/m3, E0,mean 11 GPa."),
    ("en-1995-1-1", "EN 1995-1-1:2004+A2:2014 Eurocode 5, design of timber structures: effective span between bearing centres; 8.5 bolted connections (holes in timber d + 1 mm)."),
    ("en-10025-2", "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy structural steels (S235: f_y 235 MPa, E 210 GPa, rho 7850 kg/m3)."),
    ("iso-4014-din-934", "ISO 4014:2011 hexagon head bolts (M20: across flats 30 mm, head height 12.5 mm) and DIN 934 hexagon nuts (M20: height 16 mm)."),
    ("informationsdienst-holz-bsh", "Informationsdienst Holz, Brettschichtholz (BS-Holz) and Holzbau Handbuch Reihe 1 Teil 1 Folge 4: typical hall girder sections 160 x 600 to 200 x 1600 mm and steel shoe supports with base plate, side plates and bolts or dowels."),
    ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1, Part 1-1, 6.3.4: roofs of category H not accessible except for maintenance, Q_k 1.0 kN (recommended value)."),
    ("en-1991-1-3", "EN 1991-1-3:2003 with DIN EN 1991-1-3/NA:2019: snow load zone 2 below 285 m, s_k 0.85 kN/m2; shape coefficient mu1 0.8 for roof pitch up to 30 degrees."),
    ("en-1492-2", "EN 1492-2:2000+A1:2008 Textile slings, roundslings of man-made fibres: WLL 2000 kg (green), mode factor of a choker hitch 0.8."),
    ("dguv-109-017", "DGUV Regel 109-017 Betreiben von Lastaufnahmemitteln im Hebezeugbetrieb (2020): inclination of a two-leg sling at most 60 degrees from the vertical, edge protection on timber arrises."),
    ("ashrae-fundamentals", "ASHRAE Handbook Fundamentals (2021), chapter 33: thermal radiation properties of building materials (emissivity of wood and painted metal about 0.9, solar absorptance of fresh wood about 0.45 and of greyed weathered wood 0.7 to 0.8; rounded typical values, not table look-ups)."),
    ("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties: softwood 0.13 W/mK, 1600 J/kgK; steel 50 W/mK, 450 J/kgK."),
    ("en-iso-13786", "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); about half of the 24 h depth is the effective storage depth of an exposed face."),
]


def region(id_, title, material, area, char, geom, sky, hc, src="authored-estimate", ref="en-iso-13786", extra=None):
    """🔥️ A passive thermal region."""
    r = {"id": id_, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": round(area, 4), "characteristicLengthM": char, "geometry": geom,
         "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": hc, "source": src, "reference": ref}
    if extra:
        r.update(extra)
    return r


AREAS = {"steel-shoe": 0.457}


def regions():
    """🔥️ The nine thermal regions of the asset."""
    wedge_area = 6 * (0.5 * WEDGE_W * WEDGE_H * 2 + (WEDGE_DX1 - WEDGE_DX0) * (WEDGE_W + WEDGE_H + math.hypot(WEDGE_W, WEDGE_H)))
    sling = 2 * (2 * math.pi * RS * LEG + 2 * math.pi * RS ** 2) + 2 * (2 * ((H + 4 * RS) + (B + 4 * RS)) * 2 * RS + 2 * ((H + 4 * RS) * (B + 4 * RS) - H * B))
    shoe = 2 * SHOE_L * SHOE_BASE_W + 2 * SHOE_BASE_T * (SHOE_L + SHOE_BASE_W) + 2 * (2 * SHOE_L * SHOE_SIDE_H + 2 * SHOE_SIDE_T * (SHOE_L + SHOE_SIDE_H))
    a = {"girder-sides": 2 * L * H, "girder-top": L * B, "girder-soffit": L * B, "old-end": H * B, "sawn-end": H * B, "steel-shoe": shoe,
         "bearers": 3 * 2 * (BS * BS + 2 * BS * BLEN), "wedges": wedge_area, "slings": sling}
    a.update(AREAS)
    return [
        region("girder-sides", t("Girder side faces, weathered", "Trägerseitenflächen, verwittert"), "glulam-weathered", a["girder-sides"], 1.0, "vertical-plate", 0.5, 24000),
        region("girder-top", t("Girder top face with purlin seats", "Trägeroberseite mit Pfettenauflagern"), "glulam-weathered", a["girder-top"], 0.16, "horizontal-plate-up", 0.95, 20000),
        region("girder-soffit", t("Girder soffit", "Trägerunterseite"), "glulam-weathered", a["girder-soffit"], 0.16, "horizontal-plate-down", 0.05, 20000),
        region("old-end", t("Old girder end inside the shoe", "Altes Trägerende im Schuh"), "glulam-weathered", a["old-end"], 1.0, "vertical-plate", 0.4, 16000),
        region("sawn-end", t("Fresh sawn end", "Frisch gesägtes Ende"), "glulam-sawn", a["sawn-end"], 1.0, "vertical-plate", 0.5, 16000, ref="ashrae-fundamentals"),
        region("steel-shoe", t("Steel shoe and bolts, primed", "Stahlschuh und Bolzen, grundiert"), "steel-primed", a["steel-shoe"], 0.4, "vertical-plate", 0.4, 20000, ref="en-iso-10456",
               extra={"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("bearers", t("Squared timber bearers", "Lagerhölzer aus Kantholz"), "softwood-bearer", a["bearers"], 0.12, "horizontal-plate-up", 0.4, 24000),
        region("wedges", t("Timber chocks", "Holzkeile"), "softwood-bearer", a["wedges"], 0.1, "vertical-plate", 0.45, 16800),
        region("slings", t("Roundslings (chokers and legs)", "Rundschlingen (Schnürgänge und Stränge)"), "polyester-sling", a["slings"], 0.035, "cylinder", 0.6, 4900, ref="en-1492-2"),
    ]


def binding(sun, extra=None):
    """🌞️ A region binding of a pose: every region exposed, with sunlit fractions."""
    b = {}
    for rid, s in sun.items():
        b[rid] = {"exposedFraction": 1, "sunlitFraction": s}
    if extra:
        for rid, v in extra.items():
            b[rid].update(v)
    return {"regions": b, "emitters": {}}


SUN_YARD = {"girder-sides": 0.5, "girder-top": 1, "girder-soffit": 0, "old-end": 0.3, "sawn-end": 0.5, "steel-shoe": 0.4, "bearers": 0.3, "wedges": 0.4, "slings": 0.8}
SUN_LIFTED = {"girder-sides": 0.5, "girder-top": 1, "girder-soffit": 0, "old-end": 0.4, "sawn-end": 0.5, "steel-shoe": 0.5, "bearers": 0.6, "wedges": 0.6, "slings": 0.9}
SUN_HALL = {k: 0 for k in SUN_YARD}


def poses():
    """🧍️ Stored, slings taut (key), lifted, reinstalled and reinstalled with the maintenance load."""
    stored_contacts = ["bearer-contact-1", "bearer-contact-2", "bearer-contact-3"]
    yard_extra = {"bearers": {"exposedFraction": 0.76, "contact": {"fraction": 0.24, "tempC": 18}}, "girder-soffit": {"exposedFraction": 0.97, "contact": {"fraction": 0.03, "tempC": 18}}}
    hall_extra = {"girder-top": {"exposedFraction": 0.85}}
    dead = {"id": "roof-dead-load", "kind": "imposed", "title": t(f"Roof dead load {W_ROOF / 1000:.1f} kN/m", f"Dachlast {W_ROOF / 1000:.1f} kN/m".replace(".", ",")),
            "fxN": 0, "fyN": 0, "distributed": {"bone": "girder", "fromM": 0, "toM": L, "wxNPerM": 0, "wyNPerM": -W_ROOF}, "source": "authored-estimate", "reference": "informationsdienst-holz-bsh"}
    snow = {"id": "snow-load", "kind": "snow", "title": t(f"Snow load {W_SNOW / 1000:.1f} kN/m", f"Schneelast {W_SNOW / 1000:.1f} kN/m".replace(".", ",")),
            "fxN": 0, "fyN": 0, "distributed": {"bone": "girder", "fromM": 0, "toM": L, "wxNPerM": 0, "wyNPerM": -W_SNOW}, "source": "literature", "reference": "en-1991-1-3"}
    point = {"id": "maintenance-load", "kind": "live", "title": t("Maintenance point load 1.0 kN", "Wartungseinzellast 1,0 kN"), "anchor": "load-point", "fxN": 0, "fyN": -Q_POINT, "source": "literature", "reference": "en-1991-1-1"}
    lift_bones = {"girder": {"dy": LIFT_DY}, "sling-leg-a": {"angleDeg": round(THETA, 3)}, "sling-leg-b": {"angleDeg": round(-THETA, 3)}}
    taut_bones = {"sling-leg-a": {"angleDeg": round(THETA, 3)}, "sling-leg-b": {"angleDeg": round(-THETA, 3)}}
    re_bones = {"girder": {"dy": round(REINSTALL_DY, 4)}, "dunnage": {"dx": DUNNAGE_DZ}}
    re_point = dict(re_bones)
    re_point["load-carrier"] = {"dx": round(MID_X - LOAD_REST_X, 4)}
    return [
        {"id": "stored", "title": t("Stored on bearers", "Auf Lagerhölzern gelagert"),
         "description": t("The girder stands on edge on three squared bearers in the yard, held upright by two timber chocks on each bearer; the roundslings stay choked on it with their legs lying slack on the top face.",
                          "Der Träger steht hochkant auf drei Kanthölzern im Lager, aufrecht gehalten von zwei Holzkeilen auf jedem Lagerholz; die Rundschlingen bleiben angeschlagen, ihre Stränge liegen lose auf der Oberseite."),
         "bones": {}, "ground": "authored", "contacts": stored_contacts, "thermal": binding(SUN_YARD, yard_extra), "loads": []},
        {"id": "slings-taut", "key": True, "title": t("Slings taut", "Schlingen gespannt"), "description": t("The crane hook has taken up the slack; the girder still rests on its bearers.", "Der Kranhaken hat die Schlingen gespannt; der Träger liegt noch auf seinen Lagerhölzern."),
         "bones": taut_bones, "ground": "authored", "contacts": stored_contacts, "thermal": binding(SUN_YARD, yard_extra), "loads": []},
        {"id": "lifted", "title": t("Lifted on a two-leg sling", "Am Zweistrang-Gehänge angehoben"),
         "description": t(f"The girder hangs 0.5 m above its bearers from two roundsling legs of 4 m choked at 2.77 m and 8.77 m, symmetric about its centre of mass at 5.77 m, each leg {90 - THETA:.1f} degrees from the vertical; bearers and chocks stay on the ground.",
                          f'Der Träger hängt 0,5 m über seinen Lagerhölzern an zwei 4 m langen Rundschlingensträngen, angeschlagen bei 2,77 m und 8,77 m symmetrisch zu seinem Schwerpunkt bei 5,77 m, jeder Strang {f"{90 - THETA:.1f}".replace(".", ",")} Grad gegen die Senkrechte geneigt; Lagerhölzer und Keile bleiben am Boden.'),
         "bones": lift_bones, "ground": "authored", "contacts": ["sling-point-a", "sling-point-b"], "thermal": binding(SUN_LIFTED), "loads": []},
        {"id": "reinstalled", "title": t("Reinstalled under roof and snow load", "Wiedereingebaut unter Dach- und Schneelast"),
         "description": t(f"Back in a hall roof: the shoe stands on its bearing (pin), the sawn end on a new bearing (roller), span {SPAN:.2f} m, girder spacing 5 m; roof dead load 0.5 kN/m2 and snow 0.68 kN/m2 give {W_ROOF / 1000:.1f} and {W_SNOW / 1000:.1f} kN/m. The yard timber lies aside.",
                          f"Wieder in einem Hallendach: der Schuh steht auf seinem Auflager (Festlager), das gesägte Ende auf einem neuen Auflager (Loslager), Stützweite {SPAN:.2f} m, Binderabstand 5 m; Dachlast 0,5 kN/m2 und Schnee 0,68 kN/m2 ergeben {W_ROOF / 1000:.1f} und {W_SNOW / 1000:.1f} kN/m. Das Lagerholz liegt beiseite.".replace("11.75", "11,75").replace("2.5 und", "2,5 und").replace("3.4 kN", "3,4 kN")),
         "bones": re_bones, "ground": "authored", "contacts": ["shoe-bearing", "end-bearing"], "thermal": binding(SUN_HALL, hall_extra), "loads": [dead, snow]},
        {"id": "reinstalled-point-load", "title": t("Reinstalled with a maintenance load at midspan", "Wiedereingebaut mit Wartungslast in Feldmitte"),
         "description": t("Roof dead load plus the 1.0 kN maintenance point load of a roof not accessible except for maintenance, at midspan.", "Dachlast und die Wartungseinzellast von 1,0 kN eines nur zur Instandhaltung begehbaren Dachs in Feldmitte."),
         "bones": re_point, "ground": "authored", "contacts": ["shoe-bearing", "end-bearing"], "thermal": binding(SUN_HALL, hall_extra), "loads": [dead, point]},
    ]


def clips():
    """🎞️ Lift, moving maintenance load and snow ramp."""
    return [
        {"id": "lift", "title": t("Lift off the bearers", "Von den Lagerhölzern abheben"),
         "description": t("The hook takes up the slack of both legs in 2 s, then hoists the girder 0.5 m clear of its bearers in 4 s.", "Der Haken spannt in 2 s beide Stränge, dann hebt er den Träger in 4 s um 0,5 m von seinen Lagerhölzern ab."),
         "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "slings-taut", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": f"bearer-contact-{k}", "fromS": 0, "toS": 2} for k in (1, 2, 3)] + [{"anchor": a, "fromS": 2, "toS": 6} for a in ("sling-point-a", "sling-point-b")],
         "phases": [{"id": "take-up-slack", "fromS": 0, "toS": 2, "title": t("Take up slack", "Schlingen spannen")}, {"id": "hoist", "fromS": 2, "toS": 6, "title": t("Hoist", "Heben")}],
         "events": [{"t": 2, "kind": "contact-off", "title": t("Girder leaves its bearers", "Träger verlässt seine Lagerhölzer")}], "tracks": []},
        {"id": "moving-maintenance-load", "title": t("Moving maintenance load", "Wandernde Wartungslast"),
         "description": t("The 1.0 kN maintenance load walks along the reinstalled girder from the shoe to the sawn end.", "Die Wartungslast von 1,0 kN wandert auf dem wiedereingebauten Träger vom Schuh zum gesägten Ende."),
         "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": {"dx": 0}}}, {"t": 8, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": {"dx": 11.0}}}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in ("shoe-bearing", "end-bearing")],
         "phases": [{"id": "walk", "fromS": 0, "toS": 8, "title": t("Walk along the span", "Gang entlang der Stützweite")}],
         "events": [{"t": 0, "kind": "load-on", "title": t("Load at the shoe", "Last am Schuh")}], "tracks": []},
        {"id": "snowfall", "title": t("Snowfall on the reinstalled girder", "Schneefall auf dem wiedereingebauten Träger"),
         "description": t("The snow load builds up from zero to its characteristic value while the roof dead load stays.", "Die Schneelast wächst von null auf ihren charakteristischen Wert, die Dachlast bleibt."),
         "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled", "ease": "smooth"}, {"t": 6, "pose": "reinstalled", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in ("shoe-bearing", "end-bearing")],
         "phases": [{"id": "snowing", "fromS": 0, "toS": 5, "title": t("Snowing", "Schneefall")}, {"id": "hold", "fromS": 5, "toS": 6, "title": t("Hold", "Halten")}],
         "events": [{"t": 5, "kind": "load-on", "title": t("Snow load complete", "Schneelast vollständig")}],
         "tracks": [{"target": "load:snow-load", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 1}, {"t": 6, "v": 1}]}]},
    ]


def rig():
    """🦴️ Yard reference, dunnage on a lateral slide with three bearers, the girder on a vertical slide with shoe, chokers, sling legs and load carrier."""
    bones = [
        {"id": "yard", "parent": None, "title": t("Yard reference", "Lagerplatz-Bezug"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "dunnage-track", "parent": "yard", "title": t("Lateral track of the yard timber", "Seitliche Bahn des Lagerholzes"), "x": 0, "y": 0, "angleDeg": 0, "abductionDeg": 90, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "dunnage", "parent": "dunnage-track", "title": t("Yard timber (bearers and chocks)", "Lagerholz (Lagerhölzer und Keile)"), "x": 0, "y": 0, "angleDeg": 0, "abductionDeg": -90, "lengthM": 0,
         "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.2}},
    ]
    for k, xc in enumerate(BEARERS):
        bones.append({"id": f"bearer-{k + 1}", "parent": "dunnage", "title": t(f"Bearer {k + 1} with two chocks", f"Lagerholz {k + 1} mit zwei Keilen"), "x": r6(xc - BS / 2), "y": 0, "angleDeg": 0, "lengthM": BS,
                      "joint": {"type": "fixed"}, "mass": {"kg": M_BEARER_BONE, "comX": BS / 2, "comY": BEARER_COMY, "distribution": "point", "source": "literature", "reference": "en-338"}})
    bones += [
        {"id": "girder", "parent": "yard", "title": t("Glulam girder", "Brettschichtholzträger"), "x": 0, "y": GY, "angleDeg": 0, "lengthM": L, "joint": {"type": "prismatic", "axisDeg": 90, "minM": round(REINSTALL_DY, 4), "maxM": 0.8},
         "mass": {"kg": M_GIRDER, "comX": L / 2, "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-14080"},
         "section": {"areaM2": B * H, "secondMomentM4": round(B * H ** 3 / 12, 8), "extremeFibreM": HB, "material": "glulam-weathered", "title": t("GL24h 160 x 1000 mm", "GL24h 160 x 1000 mm")}},
        {"id": "shoe", "parent": "girder", "title": t("Remaining steel shoe", "Verbliebener Stahlschuh"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"},
         "mass": {"kg": M_SHOE, "comX": SHOE_L / 2, "comY": SHOE_COMY, "distribution": "point", "source": "literature", "reference": "en-10025-2"}},
        {"id": "choker-a", "parent": "girder", "title": t("Choker hitch A", "Schnürgang A"), "x": XA, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"},
         "mass": {"kg": M_CHOKER, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "en-1492-2"}},
        {"id": "choker-b", "parent": "girder", "title": t("Choker hitch B", "Schnürgang B"), "x": XB, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"},
         "mass": {"kg": M_CHOKER, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "en-1492-2"}},
        {"id": "sling-leg-a", "parent": "choker-a", "title": t("Sling leg A", "Anschlagstrang A"), "x": 0, "y": r6(HB + RS), "z": LEGZ, "angleDeg": 0, "lengthM": LEG,
         "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 60}, "mass": {"kg": M_LEG, "comX": LEG / 2, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1492-2"}},
        {"id": "sling-leg-b", "parent": "choker-b", "title": t("Sling leg B", "Anschlagstrang B"), "x": 0, "y": r6(HB + RS), "z": -LEGZ, "angleDeg": 180, "lengthM": LEG,
         "joint": {"type": "revolute", "minDeg": -60, "maxDeg": 0}, "mass": {"kg": M_LEG, "comX": LEG / 2, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1492-2"}},
        {"id": "load-carrier", "parent": "girder", "title": t("Load carrier (massless, moves the maintenance load)", "Lastträger (masselos, bewegt die Wartungslast)"), "x": LOAD_REST_X, "y": HB, "angleDeg": 0, "lengthM": 0,
         "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 11.0}},
    ]
    return {"kind": "structural", "root": "yard", "bones": bones}


def anchors():
    """📍️ Bearer contacts, sling points, the two bearings of the reinstalled girder and the load point."""
    out = []
    for k, xc in enumerate(BEARERS):
        out.append({"id": f"bearer-contact-{k + 1}", "bone": "girder", "x": xc, "y": -HB, "kind": "contact", "title": t(f"Soffit on bearer {k + 1}", f"Unterseite auf Lagerholz {k + 1}"),
                    "support": "contact", "footprintLengthM": BS, "footprintWidthM": B, "frictionCoefficient": 0.5})
    out += [
        {"id": "sling-point-a", "bone": "girder", "x": XA, "y": HB, "kind": "support", "title": t("Lifting point A (choker)", "Anschlagpunkt A (Schnürgang)"), "support": "pin"},
        {"id": "sling-point-b", "bone": "girder", "x": XB, "y": HB, "kind": "support", "title": t("Lifting point B (choker)", "Anschlagpunkt B (Schnürgang)"), "support": "roller", "normalDeg": 90},
        {"id": "shoe-bearing", "bone": "girder", "x": SHOE_BEARING_X, "y": -HB - SHOE_BASE_T, "kind": "support", "title": t("Shoe bearing, pin", "Schuhauflager, Festlager"), "support": "pin"},
        {"id": "end-bearing", "bone": "girder", "x": END_BEARING_X, "y": -HB, "kind": "support", "title": t("New bearing at the sawn end, roller", "Neues Auflager am gesägten Ende, Loslager"), "support": "roller", "normalDeg": 90},
        {"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": t("Maintenance load", "Wartungslast")},
    ]
    return out


def document():
    """📄️ The whole asset document."""
    reps = {str(s): representation(s) for s in (50, 100, 200, 500)}
    vols = {str(s): volume(s) for s in (50, 100, 200, 500)}
    asset = {
        "id": "reclaimed-glulam-girder", "category": "components",
        "title": t("Reclaimed Glulam Girder", "Gebrauchter Brettschichtholzträger"),
        "summary": t(f"A cut-down GL24h hall girder of 160 x 1000 mm and 12 m from a dismantled hall: a fresh sawn end, the remaining steel shoe with two M20 bolts and four empty bolt holes at the other end, traces of removed purlin hangers; stored on edge on three squared bearers with timber chocks, lifted on a two-leg roundsling and reinstalled over {SPAN:.2f} m under roof, snow and maintenance loads.",
                     f"Ein abgelängter Hallenbinder GL24h 160 x 1000 mm und 12 m aus einer rückgebauten Halle: ein frisch gesägtes Ende, der verbliebene Stahlschuh mit zwei M20-Bolzen und vier leeren Bolzenlöchern am anderen Ende, Spuren ausgebauter Pfettenschuhe; hochkant auf drei Kanthölzern mit Holzkeilen gelagert, an einem Zweistrang-Rundschlingengehänge angehoben und über {SPAN:.2f} m unter Dach-, Schnee- und Wartungslast wiedereingebaut.".replace("11.75", "11,75")),
        "tags": ["component", "reclaimed", "structural", "timber", "glulam", "girder", "beam", "steel"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22",
                       "references": [{"id": i, "citation": c} for i, c in REFERENCES],
                       "notes": t(f"Girder 160 x 1000 x 12000 mm, 25 lamellae of 40 mm, mass {M_GIRDER} kg from 420 kg/m3; steel shoe {M_SHOE} kg; three bearers 120 x 120 x 1000 mm with two chocks each, {3 * M_BEARER_BONE:.2f} kg; roundslings {2 * (M_LEG + M_CHOKER):.2f} kg; total {MASS} kg. Reinstalled span {SPAN:.2f} m between the shoe centre and the centre of a 200 mm bearing at the sawn end.",
                                  f"Träger 160 x 1000 x 12000 mm, 25 Lamellen zu 40 mm, Masse {M_GIRDER} kg aus 420 kg/m3; Stahlschuh {M_SHOE} kg; drei Lagerhölzer 120 x 120 x 1000 mm mit je zwei Keilen, {3 * M_BEARER_BONE:.2f} kg; Rundschlingen {2 * (M_LEG + M_CHOKER):.2f} kg; gesamt {MASS} kg. Stützweite im Wiedereinbau {SPAN:.2f} m zwischen der Schuhmitte und der Mitte eines 200-mm-Auflagers am gesägten Ende.")},
        "dimensions": {"heightM": HEIGHT, "widthM": L, "depthM": BLEN, "massKg": MASS},
        "materials": MATERIALS, "rig": rig(), "anchors": anchors(), "representations": reps, "symbols": [], "poses": poses(), "clips": clips(),
        "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
                    "operating": [], "regions": regions(), "emitters": [], "passengers": [],
                    "assumptions": [
                        t("Weathered timber faces are greyed (solar absorptance 0.75 against 0.45 of the fresh sawn end); emissivity 0.9 for both; the primed shoe 0.88 and 0.7.", "Verwitterte Holzflächen sind vergraut (solarer Absorptionsgrad 0,75 gegenüber 0,45 des frisch gesägten Endes); Emissionsgrad 0,9 für beide; der grundierte Schuh 0,88 und 0,7."),
                        t("Passive timber faces store heat in an effective depth of about 36 mm (half the 24 h penetration depth of 73 mm for lambda 0.13 W/mK, rho 420 kg/m3, c 1600 J/kgK, EN ISO 13786).", "Passive Holzflächen speichern Wärme in einer wirksamen Tiefe von etwa 36 mm (die Hälfte der 24-h-Eindringtiefe von 73 mm für Lambda 0,13 W/mK, Rho 420 kg/m3, c 1600 J/kgK, EN ISO 13786)."),
                        t("Stored and lifted in the open yard on a mild sunny day; reinstalled inside a hall, so no face is sunlit and the roof deck covers part of the top face.", "Gelagert und angehoben im offenen Lager an einem milden sonnigen Tag; wiedereingebaut in einer Halle, daher ist keine Fläche besonnt und die Dachschale deckt einen Teil der Oberseite."),
                        t("Faces pressed against another solid of the asset (shoe plates on the timber, slings on the girder, the soffit on the bearers) exchange no heat with the air and are not part of the outer areas.", "Flächen, die an einem anderen Körper des Assets anliegen (Schuhbleche am Holz, Schlingen am Träger, die Unterseite auf den Lagerhölzern), tauschen keine Wärme mit der Luft und gehören nicht zu den Außenflächen."),
                        t("No element draws power: every region is passive (0 W).", "Kein Element nimmt Leistung auf: alle Regionen sind passiv (0 W)."),
                    ]},
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [],
                      "assumptions": [
                          t("Supports belong to the pose: stored, the soffit rests on the three bearers (contacts with friction 0.5); lifted, the two choker points carry it (pin and vertical roller, the horizontal components of the inclined legs are not applied); reinstalled, the shoe is a pin and the new bearing at the sawn end a roller.",
                            "Die Auflager gehören zur Pose: gelagert liegt die Unterseite auf den drei Lagerhölzern (Kontakte mit Reibung 0,5); angehoben tragen ihn die beiden Schnürpunkte (Festlager und senkrechtes Loslager, die waagrechten Anteile der geneigten Stränge werden nicht angesetzt); wiedereingebaut ist der Schuh ein Festlager und das neue Auflager am gesägten Ende ein Loslager."),
                          t(f"E0,mean 11.5 GPa (GL24h, EN 14080), section 160 x 1000 mm (I = 0.01333 m4), uniform self-weight from 420 kg/m3; the shoe is a point mass of {M_SHOE} kg.",
                            f"E0,mean 11,5 GPa (GL24h, EN 14080), Querschnitt 160 x 1000 mm (I = 0,01333 m4), gleichmäßiges Eigengewicht aus 420 kg/m3; der Schuh ist eine Punktmasse von {M_SHOE} kg.".replace(".586", ",586")),
                          t(f"The whole asset is one instantaneously rigid body: the weight of the yard timber ({3 * M_BEARER_BONE:.1f} kg) reaches the supports of the girder through the yard reference, a cut of about 0.19 kN at the girder origin, 2 % of the total weight.",
                            f"Das ganze Asset ist ein momentan starrer Körper: das Gewicht des Lagerholzes ({3 * M_BEARER_BONE:.1f} kg) erreicht die Auflager des Trägers über den Lagerplatz-Bezug, ein Schnitt von etwa 0,19 kN am Trägeranfang, 2 % des Gesamtgewichts.".replace(".7 kg", ",7 kg")),
                          t("Shear and axial deformation are neglected in the deflection.", "Schub- und Längsverformung werden in der Durchbiegung vernachlässigt."),
                      ], "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def de_decimals(doc):
    """🔢️ Writes decimal commas into the German provenance notes."""
    import re
    notes = doc["asset"]["provenance"]
    notes["notes"]["de"] = re.sub(r"(\d)\.(\d)", r"\1,\2", notes["notes"]["de"])
    return doc


if __name__ == "__main__":
    doc = de_decimals(document())
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print(f"[DEBUG] mass {MASS} kg, girder {M_GIRDER}, shoe {M_SHOE} (comY {SHOE_COMY}), bearer bone {M_BEARER_BONE}, leg {M_LEG}, choker {M_CHOKER}, theta {THETA:.2f}, COM x {COM_X:.4f}, height {HEIGHT}")
