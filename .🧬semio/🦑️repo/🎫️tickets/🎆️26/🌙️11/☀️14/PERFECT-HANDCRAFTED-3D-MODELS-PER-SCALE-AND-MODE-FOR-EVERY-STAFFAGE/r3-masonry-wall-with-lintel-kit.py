"""🧱 Round 3 masonry wall with lintel: the real construction of the 3.0 m x 2.75 m cavity wall sample and the writers every script states its parts with.

Frame of the bone `wall` (= world at rest): u = x along the wall (0 .. 3), v = y height (0 .. 2.75), w = z through the wall (exterior facing at -0.2225, interior plaster at +0.2225).
`lintel` sits at (0.625, 2.25), `sill` at (0.8, 0.8333), both unrotated, so their frames are the wall frame shifted.

Build-up (outside in): 115 mm NF facing brick in stretcher bond (DIN 105-100, EN 771-1, course 1/12 m = 71 mm brick + 12.3 mm bed joint, 10 mm head joints, DIN 4172),
140 mm full-fill mineral wool (DIN EN 1996-2/NA: cavity at most 150 mm, at least 7 ties of 4 mm stainless wire per m2), 175 mm calcium silicate blocks
(EN 771-2, 248 mm course height with 2 mm thin-bed joints = 0.25 m), 15 mm gypsum plaster.
Opening 1.26 x 1.333 m (octametre opening = n x 0.125 + 0.01, DIN 4172), inner precast lintel 175 x 250 and exposed outer precast lintel 175 x 237.7
(over a 12.3 mm bed) with 0.245 m bearings (EN 13225), 80 mm wool between them; precast cast-stone sill (DIN 18500) 1.4 m with 50 mm projection,
5 degree weathering, drip groove and raised ends in the reveals, 175 mm deep in the wall plane, 80 mm wool behind it.
"""
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
SLUG = "masonry-wall-with-lintel"
ASSET = "components/" + SLUG
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/masonry-wall-with-lintel/🔣️.json"
SOURCE = os.path.join(T, "volumes", "components", SLUG + ".json")
GEN = os.path.join(T, "🗑️generated", "r3-" + SLUG)

C = 1.0 / 12.0
BRICK_H = 0.071
BED = C - BRICK_H
HEAD = 0.010
NF_L = 0.240
PITCH = NF_L + HEAD
KS_H = 0.25
KS_L = 0.25

LENGTH = 3.0
HEIGHT = 2.75
EXT = -0.2225
FILL = -0.1075
LEAF = 0.0325
PLASTER = 0.2075
INT = 0.2225
TONGUE = -0.0475
NOSE = -0.2725

OPEN = (0.87, 2.13)
PARAPET = 11 * C
LINTEL_Y = 2.25
LINTEL_TOP = 2.5
LINTEL_X = (0.625, 2.375)
LINTEL_LEN = 1.75
OUTER_SOFFIT = LINTEL_Y + BED
SILL_Y = 10 * C
SILL_X = (0.8, 2.2)
SILL_LEN = 1.4
SILL_END = 0.07
KS_OPEN = (0.855, 2.145)
SOFFIT_PLASTER = LINTEL_Y - 0.015
CHAMFER_W = 0.010
CHAMFER_V = 0.012

SLOPE = math.tan(math.radians(5.0))
SEAT_W = -0.0875
V_NOSE = C - (SEAT_W - NOSE) * SLOPE
V_FACE = C - (SEAT_W - EXT) * SLOPE
GROOVE = (-0.2575, -0.2475)
GROOVE_D = 0.010

WEEPS = {0: [0.375, 1.375, 2.375], 30: [1.125, 1.875]}

TIE_R = 0.002
TIE_W = (EXT + 0.05, LEAF + 0.05)
TIE_ROWS = list(range(3, 31, 3))

DENSITY = {"brick": 1800, "block": 1800, "concrete": 2400, "wool": 40, "plaster": 1300, "mortar": 1800, "tie-steel": 7900}
TONE = {"brick": 0.55, "mortar": 0.78, "block": 0.7, "wool": 0.9, "plaster": 0.86, "concrete": 0.35, "tie-steel": 0.25}
BONE_ORIGIN = {"wall": (0.0, 0.0), "lintel": (LINTEL_X[0], LINTEL_Y), "sill": (SILL_X[0], SILL_Y), "load-carrier": (0.0, HEIGHT)}


def r5(x):
    """🔢 Rounded to 0.01 mm, without a negative zero."""
    y = round(x, 5)
    return 0.0 if y == 0 else y


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def load(path):
    """📂 A JSON document."""
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def box(id_, u, v, w, material, bone="wall", role="body", tone=None, **extra):
    """📦 An axis-aligned box from three intervals in the frame of `bone` (intervals given in the wall frame are shifted by the bone origin)."""
    ox, oy = BONE_ORIGIN[bone]
    out = {"id": id_, "kind": "box", "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone,
           "size": [r5(u[1] - u[0]), r5(v[1] - v[0]), r5(w[1] - w[0])],
           "transform": {"translation": [r5((u[0] + u[1]) / 2 - ox), r5((v[0] + v[1]) / 2 - oy), r5((w[0] + w[1]) / 2)]}}
    out.update(extra)
    return out


def repeated(solid, count, step):
    """🔁 A solid repeated `count` times along u by `step`."""
    if count > 1:
        solid["repeat"] = {"count": count, "translation": [r5(step), 0, 0]}
    return solid


def slab(id_, outline, holes, w, material, bone="wall", role="body", tone=None, **extra):
    """🧱 An extrusion of a wall-frame outline (u, v) with holes through w0 .. w1."""
    ox, oy = BONE_ORIGIN[bone]
    out = {"id": id_, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone,
           "profile": [[r5(x - ox), r5(y - oy)] for x, y in outline], "plane": "xy", "from": r5(w[0]), "to": r5(w[1])}
    if holes:
        out["holes"] = [[[r5(x - ox), r5(y - oy)] for x, y in hole] for hole in holes]
    out.update(extra)
    return out


def section(id_, profile_vw, u, material, bone, role="body", tone=None, **extra):
    """📐 An extrusion along u of a profile given as (v, w) in the wall frame, from u0 to u1 (wall frame)."""
    ox, oy = BONE_ORIGIN[bone]
    out = {"id": id_, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone,
           "profile": [[r5(v - oy), r5(w)] for v, w in profile_vw], "plane": "yz", "from": r5(u[0] - ox), "to": r5(u[1] - ox)}
    out.update(extra)
    return out


def rect(x0, y0, x1, y1):
    """▭ A rectangle as a ring."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


WALL = rect(0, 0, LENGTH, HEIGHT)
FIELD_HOLE_50 = [(LINTEL_X[0], LINTEL_TOP), (LINTEL_X[1], LINTEL_TOP), (LINTEL_X[1], OUTER_SOFFIT), (OPEN[1], OUTER_SOFFIT), (OPEN[1], PARAPET),
                 (SILL_X[1], PARAPET), (SILL_X[1], SILL_Y), (SILL_X[0], SILL_Y), (SILL_X[0], PARAPET), (OPEN[0], PARAPET), (OPEN[0], OUTER_SOFFIT), (LINTEL_X[0], OUTER_SOFFIT)]
FIELD_HOLE = [(LINTEL_X[0], LINTEL_TOP), (LINTEL_X[1], LINTEL_TOP), (LINTEL_X[1], LINTEL_Y), (OPEN[1], LINTEL_Y), (OPEN[1], PARAPET),
              (SILL_X[1], PARAPET), (SILL_X[1], SILL_Y), (SILL_X[0], SILL_Y), (SILL_X[0], PARAPET), (OPEN[0], PARAPET), (OPEN[0], LINTEL_Y), (LINTEL_X[0], LINTEL_Y)]
LEAF_HOLE = [(LINTEL_X[0], LINTEL_TOP), (LINTEL_X[1], LINTEL_TOP), (LINTEL_X[1], LINTEL_Y), (KS_OPEN[1], LINTEL_Y), (KS_OPEN[1], PARAPET),
             (KS_OPEN[0], PARAPET), (KS_OPEN[0], LINTEL_Y), (LINTEL_X[0], LINTEL_Y)]
LEAF_HOLE_COARSE = [(LINTEL_X[0], LINTEL_TOP), (LINTEL_X[1], LINTEL_TOP), (LINTEL_X[1], LINTEL_Y), (OPEN[1], LINTEL_Y), (OPEN[1], PARAPET),
                    (OPEN[0], PARAPET), (OPEN[0], LINTEL_Y), (LINTEL_X[0], LINTEL_Y)]
OPENING = rect(OPEN[0], PARAPET, OPEN[1], LINTEL_Y)


def facing_runs(k):
    """🧱 The brick runs of facing course k as (a, b, end at a, end at b); ends are `cut` (sample boundary), `free` (reveal) or `abut` (a 10 mm joint against lintel or sill)."""
    if k <= 9 or k >= 30:
        return [(0.0, LENGTH, "cut", "cut")]
    if k == 10:
        return [(0.0, SILL_X[0], "cut", "abut"), (SILL_X[1], LENGTH, "abut", "cut")]
    if k <= 26:
        return [(0.0, OPEN[0], "cut", "free"), (OPEN[1], LENGTH, "free", "cut")]
    return [(0.0, LINTEL_X[0], "cut", "abut"), (LINTEL_X[1], LENGTH, "abut", "cut")]


def bed_runs(k):
    """➖ The bed joint runs under facing course k (the bed of course 10 also carries the sill, that of 27 the outer lintel bearings)."""
    if k <= 10 or k >= 30:
        return [(0.0, LENGTH)]
    if k <= 27:
        return [(0.0, OPEN[0]), (OPEN[1], LENGTH)]
    return [(0.0, LINTEL_X[0]), (LINTEL_X[1], LENGTH)]


def bond_start(k):
    """📏 Where the bricks of course k start, modulo the 0.25 m pitch: half bond, aligned so that every reveal course ends with a whole (240) or a half (115) brick."""
    return 0.13 if k % 2 == 0 else 0.255


def course_pieces(k):
    """🧩 Bricks and head joints of facing course k: lists of (kind, a, b) with kind `brick`, `head` or `weep` (an open head joint)."""
    out = []
    g = bond_start(k)
    for a, b, ea, eb in facing_runs(k):
        lo = a + HEAD if ea == "abut" else a
        hi = b - HEAD if eb == "abut" else b
        parts = []
        if ea == "abut":
            parts.append(["head", a, lo])
        n0 = math.floor((lo - g) / PITCH) - 1
        x = g + n0 * PITCH
        while x < hi:
            parts.append(["brick", max(x, lo), min(x + NF_L, hi)])
            parts.append(["head", max(x + NF_L, lo), min(x + PITCH, hi)])
            x += PITCH
        if eb == "abut":
            parts.append(["head", hi, b])
        parts = [p for p in parts if p[2] - p[1] > 1e-9]
        if ea == "cut" and parts[0][0] == "head" and parts[0][2] - parts[0][1] < 0.0075:
            parts[1][1] = parts[0][1]
            parts = parts[1:]
        if eb == "cut" and parts[-1][0] == "head" and parts[-1][2] - parts[-1][1] < 0.0075:
            parts[-2][2] = parts[-1][2]
            parts = parts[:-1]
        for p in parts:
            if p[0] == "head" and any(abs((p[1] + p[2]) / 2 - c) < 1e-6 for c in WEEPS.get(k, [])):
                p[0] = "weep"
        out.extend(tuple(p) for p in parts)
    return out


def groups(pieces, kind, length):
    """🔗 Consecutive pieces of one kind and one length as (first a, count) for a repeat."""
    out = []
    for p in pieces:
        if p[0] == kind and abs(p[2] - p[1] - length) < 1e-9:
            if out and abs(out[-1][0] + out[-1][1] * PITCH - p[1]) < 1e-9:
                out[-1][1] += 1
                continue
            out.append([p[1], 1])
    return out


def facing_bricks(k):
    """🧱 Body solids of facing course k: repeated whole bricks and head joints, cut pieces, its bed joint (flush pointing, 10 mm and 12.3 mm joints)."""
    y0 = k * C
    yb = (y0 + BED, y0 + C)
    w = (EXT, FILL)
    pieces = course_pieces(k)
    out = []
    for i, (a, b) in enumerate(bed_runs(k)):
        out.append(box(f"bed-{k}-{i}", (a, b), (y0, y0 + BED), w, "mortar"))
    for i, (a, n) in enumerate(groups(pieces, "brick", NF_L)):
        out.append(repeated(box(f"brick-{k}-{i}", (a, a + NF_L), yb, w, "brick"), n, PITCH))
    for i, (a, n) in enumerate(groups(pieces, "head", HEAD)):
        out.append(repeated(box(f"head-{k}-{i}", (a, a + HEAD), yb, w, "mortar"), n, PITCH))
    cut = [p for p in pieces if p[0] in ("brick", "head") and abs(p[2] - p[1] - (NF_L if p[0] == "brick" else HEAD)) > 1e-9]
    for i, (kind, a, b) in enumerate(cut):
        out.append(box(f"{'piece' if kind == 'brick' else 'joint'}-{k}-{i}", (a, b), yb, w, "brick" if kind == "brick" else "mortar"))
    return out


def ks_runs(j):
    """🧱 The block runs of calcium silicate course j (0.25 m) as (a, b, v0, v1); course 3 carries the parapet and is cut down under the opening."""
    y0, y1 = j * KS_H, (j + 1) * KS_H
    if j <= 2 or j == 10:
        return [(0.0, LENGTH, y0, y1)]
    if j == 3:
        return [(0.0, LENGTH, y0, y1)]
    if j <= 8:
        return [(0.0, KS_OPEN[0], y0, y1), (KS_OPEN[1], LENGTH, y0, y1)]
    return [(0.0, LINTEL_X[0], y0, y1), (LINTEL_X[1], LENGTH, y0, y1)]


def ks_blocks(j):
    """🧱 Body solids of block course j: repeated whole blocks (248 mm + 2 mm thin bed = 0.25 m, half bond), cut pieces at the ends and, in course 3, the blocks cut down to the parapet under the opening."""
    out = []
    w = (LEAF, PLASTER)
    g = 0.0 if j % 2 == 0 else 0.125
    pieces = []
    for a, b, v0, v1 in ks_runs(j):
        n0 = math.floor((a - g) / KS_L) - 1
        x = g + n0 * KS_L
        while x < b - 1e-9:
            lo, hi = max(x, a), min(x + KS_L, b)
            if hi - lo > 1e-9:
                pieces.append((lo, hi, v0, v1))
            x += KS_L
    whole, rest = [], []
    for p in pieces:
        notched = j == 3 and p[1] > KS_OPEN[0] + 1e-9 and p[0] < KS_OPEN[1] - 1e-9
        if not notched and abs(p[1] - p[0] - KS_L) < 1e-9:
            if whole and abs(whole[-1][0] + whole[-1][1] * KS_L - p[0]) < 1e-9 and whole[-1][2] == p[2]:
                whole[-1][1] += 1
            else:
                whole.append([p[0], 1, p[2], p[3]])
        else:
            rest.append(p)
    for i, (a, n, v0, v1) in enumerate(whole):
        out.append(repeated(box(f"block-{j}-{i}", (a, a + KS_L), (v0, v1), w, "block"), n, KS_L))
    for i, (a, b, v0, v1) in enumerate(rest):
        if j == 3 and b > KS_OPEN[0] + 1e-9 and a < KS_OPEN[1] - 1e-9:
            if a >= KS_OPEN[0] - 1e-9 and b <= KS_OPEN[1] + 1e-9:
                out.append(box(f"block-{j}-cut-{i}", (a, b), (v0, PARAPET), w, "block"))
            else:
                edge = KS_OPEN[0] if a < KS_OPEN[0] else KS_OPEN[1]
                if a < KS_OPEN[0]:
                    ring = [(a, v0), (b, v0), (b, PARAPET), (edge, PARAPET), (edge, v1), (a, v1)]
                else:
                    ring = [(a, v0), (b, v0), (b, v1), (edge, v1), (edge, PARAPET), (a, PARAPET)]
                out.append(slab(f"block-{j}-notch-{i}", ring, None, w, "block"))
        else:
            out.append(box(f"block-{j}-piece-{i}", (a, b), (v0, v1), w, "block"))
    return out


def outer_lintel_profile(chamfer=True):
    """📐 (v, w) of the exposed outer precast lintel over its 12.3 mm bed: 175 deep, 237.7 high, chamfered front soffit edge."""
    v0 = OUTER_SOFFIT
    if not chamfer:
        return [(LINTEL_Y, TONGUE), (LINTEL_TOP, TONGUE), (LINTEL_TOP, EXT), (LINTEL_Y, EXT)]
    return [(v0, TONGUE), (LINTEL_TOP, TONGUE), (LINTEL_TOP, EXT), (v0 + CHAMFER_V, EXT), (v0, EXT + CHAMFER_W)]


def sill_profile(end=False):
    """📐 (v, w) of the cast-stone sill: tongue on the wool, bed on the facing, 50 mm projection with drip groove, 5 degree weathering to a 40 mm seat; the ends in the reveals rise to the course line."""
    v0, vb = SILL_Y, SILL_Y + BED
    pts = [(v0, TONGUE), (v0, FILL), (vb, FILL), (vb, GROOVE[1]), (vb + GROOVE_D, GROOVE[1]), (vb + GROOVE_D, GROOVE[0]), (vb, GROOVE[0]), (vb, NOSE), (v0 + V_NOSE, NOSE)]
    if end:
        pts += [(v0 + V_FACE, EXT), (v0 + C, EXT), (v0 + C, TONGUE)]
    else:
        pts += [(v0 + C, SEAT_W), (v0 + C, TONGUE)]
    return pts


def body50():
    """🔍 1:50 body: every brick, joint and block, wool, plaster with reveal returns, both precast lintels, the sill in three pieces."""
    out = []
    for k in range(33):
        out += facing_bricks(k)
    out.append(slab("cavity-wool", WALL, [FIELD_HOLE_50_WOOL], (FILL, LEAF), "wool"))
    out.append(box("lintel-wool", LINTEL_X, (OUTER_SOFFIT, LINTEL_TOP), (TONGUE, LEAF), "wool"))
    out.append(box("sill-wool", SILL_X, (SILL_Y, PARAPET), (TONGUE, LEAF), "wool"))
    for j in range(11):
        out += ks_blocks(j)
    out.append(slab("plaster", WALL, [rect(OPEN[0], PARAPET, OPEN[1], SOFFIT_PLASTER)], (PLASTER, INT), "plaster"))
    out.append(box("reveal-plaster-left", (KS_OPEN[0], OPEN[0]), (PARAPET, SOFFIT_PLASTER), (LEAF, PLASTER), "plaster"))
    out.append(box("reveal-plaster-right", (OPEN[1], KS_OPEN[1]), (PARAPET, SOFFIT_PLASTER), (LEAF, PLASTER), "plaster"))
    out.append(box("soffit-plaster", KS_OPEN, (SOFFIT_PLASTER, LINTEL_Y), (LEAF, PLASTER), "plaster"))
    out.append(section("lintel-outer", outer_lintel_profile(), LINTEL_X, "concrete", "lintel"))
    out.append(box("lintel-inner", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (LEAF, PLASTER), "concrete", "lintel"))
    out.append(section("sill", sill_profile(), (SILL_X[0] + SILL_END, SILL_X[1] - SILL_END), "concrete", "sill"))
    out.append(section("sill-end-left", sill_profile(True), (SILL_X[0], SILL_X[0] + SILL_END), "concrete", "sill"))
    out.append(section("sill-end-right", sill_profile(True), (SILL_X[1] - SILL_END, SILL_X[1]), "concrete", "sill"))
    return out


FIELD_HOLE_50_WOOL = [(LINTEL_X[0], LINTEL_TOP), (LINTEL_X[1], LINTEL_TOP), (LINTEL_X[1], OUTER_SOFFIT), (OPEN[1], OUTER_SOFFIT), (OPEN[1], PARAPET),
                      (SILL_X[1], PARAPET), (SILL_X[1], SILL_Y), (SILL_X[0], SILL_Y), (SILL_X[0], PARAPET), (OPEN[0], PARAPET), (OPEN[0], OUTER_SOFFIT), (LINTEL_X[0], OUTER_SOFFIT)]
FACING_HOLE_50 = [(LINTEL_X[0], LINTEL_TOP), (LINTEL_X[1], LINTEL_TOP), (LINTEL_X[1], OUTER_SOFFIT), (OPEN[1], OUTER_SOFFIT), (OPEN[1], PARAPET),
                  (SILL_X[1], PARAPET), (SILL_X[1], SILL_Y + BED), (SILL_X[0], SILL_Y + BED), (SILL_X[0], PARAPET), (OPEN[0], PARAPET), (OPEN[0], OUTER_SOFFIT), (LINTEL_X[0], OUTER_SOFFIT)]


def tie_positions(k):
    """📍 The u of the wall ties in the bed joint of course k: 0.5 m apart, the nearest at 90 mm beside the reveals (8 ties per m2, 4 per m along the jambs)."""
    full = [0.25, 0.75, 1.25, 1.75, 2.25, 2.75]
    if 11 <= k <= 26:
        return [0.25, OPEN[0] - 0.09, OPEN[1] + 0.09, 2.75]
    if 27 <= k <= 29:
        return [0.25]
    return full


def ties(scale_feature):
    """🪝 Wall ties: 4 mm stainless wire, 50 mm into the bed joint of each leaf, across the 140 mm cavity, in the bed joints every 3rd course (0.25 m) where both leaves have a joint."""
    out = []
    for k in TIE_ROWS:
        y = k * C + BED / 2
        xs = tie_positions(k)
        runs = []
        for x in xs:
            if runs and abs(runs[-1][0] + runs[-1][1] * 0.5 - x) < 1e-9:
                runs[-1][1] += 1
            else:
                runs.append([x, 1])
        for i, (x, n) in enumerate(runs):
            s = {"id": f"tie-{k}-{i}", "kind": "capsule", "bone": "wall", "material": "tie-steel", "role": "detail", "tone": TONE["tie-steel"],
                 "p0": [r5(x), r5(y), r5(TIE_W[0])], "p1": [r5(x), r5(y), r5(TIE_W[1])], "r0": TIE_R, "r1": TIE_R, "slender": True}
            out.append(repeated(s, n, 0.5))
    return out


def solid_mass(s):
    """⚖️ Mass (kg) and world centre (x, y, z) of a box or extrude, its repeats included."""
    rho = DENSITY[s["material"]]
    ox, oy = BONE_ORIGIN[s["bone"]]
    if s["kind"] == "box":
        vol = s["size"][0] * s["size"][1] * s["size"][2]
        cx, cy, cz = s["transform"]["translation"]
    elif s["kind"] == "capsule":
        a, b = s["p0"], s["p1"]
        L = math.dist(a, b)
        vol = math.pi * s["r0"] ** 2 * L + 4 / 3 * math.pi * s["r0"] ** 3
        cx, cy, cz = [(p + q) / 2 for p, q in zip(a, b)]
    else:
        def area_c(ring):
            A = cx_ = cy_ = 0.0
            for (x0, y0), (x1, y1) in zip(ring, ring[1:] + ring[:1]):
                cr = x0 * y1 - x1 * y0
                A += cr
                cx_ += (x0 + x1) * cr
                cy_ += (y0 + y1) * cr
            A /= 2
            return abs(A), cx_ / (6 * A), cy_ / (6 * A)
        A, px, py = area_c(s["profile"])
        mx, my = A * px, A * py
        for h in s.get("holes", []):
            ha, hx, hy = area_c(h)
            A -= ha
            mx -= ha * hx
            my -= ha * hy
        px, py = mx / A, my / A
        d = s["to"] - s["from"]
        vol = A * d
        mid = (s["to"] + s["from"]) / 2
        if s["plane"] == "xy":
            cx, cy, cz = px, py, mid
        else:
            cx, cy, cz = mid, px, py
    rep = s.get("repeat")
    n = rep["count"] if rep else 1
    step = rep["translation"] if rep else [0, 0, 0]
    cx += step[0] * (n - 1) / 2
    cy += step[1] * (n - 1) / 2
    cz += step[2] * (n - 1) / 2
    return rho * vol * n, (cx + ox, cy + oy, cz)


def bone_masses(solids):
    """⚖️ Mass and world centre per bone of a body list."""
    acc = {}
    for s in solids:
        m, (x, y, z) = solid_mass(s)
        a = acc.setdefault(s["bone"], [0.0, 0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * x
        a[2] += m * y
        a[3] += m * z
    return {b: (a[0], a[1] / a[0], a[2] / a[0], a[3] / a[0]) for b, a in acc.items()}


def sill_profile_100():
    """📐 (v, w) of the sill at 1:100: one straight weathering from the 55 mm nose to the back, bed and drip merged."""
    v0 = SILL_Y
    return [(v0, TONGUE), (v0, NOSE), (v0 + V_NOSE, NOSE), (v0 + C, TONGUE)]


def body100():
    """🔎 1:100 body: the four leaves as slabs, the two lintels and the 80 mm wool between them, the sill with its projection and weathering."""
    return [
        slab("facing", WALL, [FIELD_HOLE], (EXT, FILL), "brick"),
        slab("cavity-wool", WALL, [FIELD_HOLE], (FILL, LEAF), "wool"),
        box("lintel-wool", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (TONGUE, LEAF), "wool"),
        box("sill-wool", SILL_X, (SILL_Y, PARAPET), (TONGUE, LEAF), "wool"),
        slab("inner-leaf", WALL, [LEAF_HOLE_COARSE], (LEAF, PLASTER), "block"),
        slab("plaster", WALL, [OPENING], (PLASTER, INT), "plaster"),
        section("lintel-outer", outer_lintel_profile(False), LINTEL_X, "concrete", "lintel"),
        box("lintel-inner", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (LEAF, PLASTER), "concrete", "lintel"),
        section("sill", sill_profile_100(), SILL_X, "concrete", "sill"),
    ]


def body200():
    """🔭 1:200 body: the wall as one 445 mm mass, the lintel band over the whole thickness, the sill (with its projection) and the parapet cap behind it."""
    return [
        slab("wall", WALL, [FIELD_HOLE], (EXT, INT), "brick"),
        box("lintel", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (EXT, INT), "concrete", "lintel"),
        box("sill", SILL_X, (SILL_Y, PARAPET), (NOSE, TONGUE), "concrete", "sill"),
        box("parapet-cap", SILL_X, (SILL_Y, PARAPET), (TONGUE, INT), "block"),
    ]


def body500():
    """🛰️ 1:500 body: the wall mass with the opening, the lintel band and the sill band through the whole thickness, its nose 50 mm in front of the face (the depth of the asset)."""
    return [
        slab("wall", WALL, [FIELD_HOLE], (EXT, INT), "brick"),
        box("lintel", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (EXT, INT), "concrete", "lintel"),
        box("sill", SILL_X, (SILL_Y, PARAPET), (NOSE, INT), "concrete", "sill"),
    ]


ALL_BUT_OUT = ["-u", "+u", "-v", "+v", "+w"]
FIELD_STRIP_HOLE = [(LINTEL_X[0], LINTEL_TOP), (LINTEL_X[1], LINTEL_TOP), (LINTEL_X[1], LINTEL_Y), (OPEN[1], LINTEL_Y), (OPEN[1], PARAPET),
                    (SILL_X[1], PARAPET), (SILL_X[1], SILL_Y), (SILL_X[0], SILL_Y), (SILL_X[0], PARAPET), (OPEN[0], PARAPET), (OPEN[0], LINTEL_Y), (LINTEL_X[0], LINTEL_Y)]


def th(solid, region, layer=None, faces=None):
    """🌡️ A body-like solid turned thermal: region, layer and section faces."""
    solid["region"] = region
    if layer:
        solid["layer"] = layer
    if faces:
        solid["sectionFaces"] = faces
    solid["tone"] = 0.5
    return solid


def ref(id_, of, region, layer=None, faces=None):
    """🔗 A thermal reference to a body solid of the same scale."""
    out = {"id": id_, "of": of, "region": region}
    if layer:
        out["layer"] = layer
    if faces:
        out["sectionFaces"] = faces
    return out


def thermal50():
    """🌡️ 1:50 thermal: the three regions of the EN ISO 6946 calculation as strips through the wall, every layer a shell at its real thickness, innermost first; the outer layer counts its exterior face only."""
    hole = [FIELD_STRIP_HOLE]
    return [
        th(slab("th-field-plaster", WALL, hole, (PLASTER, INT), "plaster"), "masonry-field", "plaster"),
        th(slab("th-field-inner-leaf", WALL, hole, (LEAF, PLASTER), "block"), "masonry-field", "inner-leaf"),
        th(slab("th-field-fill", WALL, hole, (FILL, LEAF), "wool"), "masonry-field", "fill"),
        th(slab("th-field-facing", WALL, hole, (EXT, FILL), "brick"), "masonry-field", "facing", ALL_BUT_OUT),
        th(box("th-lintel-plaster", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (PLASTER, INT), "plaster", "lintel"), "lintel", "plaster"),
        ref("th-lintel-inner", "lintel-inner", "lintel", "lintel-inner"),
        ref("th-lintel-ins", "lintel-wool", "lintel", "lintel-ins"),
        ref("th-lintel-shell", "lintel-outer", "lintel", "lintel-shell", ALL_BUT_OUT),
        th(box("th-sill-plaster", SILL_X, (SILL_Y, PARAPET), (PLASTER, INT), "plaster", "sill"), "sill", "plaster"),
        th(box("th-sill-block", SILL_X, (SILL_Y, PARAPET), (LEAF, PLASTER), "block", "sill"), "sill", "sill-block"),
        ref("th-sill-ins", "sill-wool", "sill", "sill-ins"),
        ref("th-sill-concrete", "sill", "sill", "sill-concrete", ALL_BUT_OUT),
        ref("th-sill-concrete-left", "sill-end-left", "sill", "sill-concrete", ALL_BUT_OUT),
        ref("th-sill-concrete-right", "sill-end-right", "sill", "sill-concrete", ALL_BUT_OUT),
    ]


def thermal100():
    """🌡️ 1:100 thermal: per region three shells (plaster merged into the inner masonry), the wool and the outer layer as references of the body."""
    hole = [FIELD_STRIP_HOLE]
    return [
        th(slab("th-field-inner", WALL, hole, (LEAF, INT), "block"), "masonry-field", "inner-leaf", ALL_BUT_OUT),
        ref("th-field-fill", "cavity-wool", "masonry-field", "fill", ALL_BUT_OUT),
        ref("th-field-facing", "facing", "masonry-field", "facing", ALL_BUT_OUT),
        th(box("th-lintel-inner", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (LEAF, INT), "concrete", "lintel"), "lintel", "lintel-inner", ALL_BUT_OUT),
        ref("th-lintel-ins", "lintel-wool", "lintel", "lintel-ins", ALL_BUT_OUT),
        ref("th-lintel-shell", "lintel-outer", "lintel", "lintel-shell", ALL_BUT_OUT),
        th(box("th-sill-block", SILL_X, (SILL_Y, PARAPET), (LEAF, INT), "block", "sill"), "sill", "sill-block", ALL_BUT_OUT),
        ref("th-sill-ins", "sill-wool", "sill", "sill-ins", ALL_BUT_OUT),
        ref("th-sill-concrete", "sill", "sill", "sill-concrete", ALL_BUT_OUT),
    ]


def thermal200():
    """🌡️ 1:200 thermal: field and lintel zone with inner masonry, insulation and outer layer as three shells of their own (the body no longer separates them); the 83 mm sill zone as the sill and one merged inner shell."""
    hole = [FIELD_STRIP_HOLE]
    return [
        th(slab("th-field-inner", WALL, hole, (LEAF, INT), "block"), "masonry-field", "inner-leaf", ALL_BUT_OUT),
        th(slab("th-field-fill", WALL, hole, (FILL, LEAF), "wool"), "masonry-field", "fill", ALL_BUT_OUT),
        th(slab("th-field-facing", WALL, hole, (EXT, FILL), "brick"), "masonry-field", "facing", ALL_BUT_OUT),
        th(box("th-lintel-inner", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (LEAF, INT), "concrete", "lintel"), "lintel", "lintel-inner", ALL_BUT_OUT),
        th(box("th-lintel-ins", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (TONGUE, LEAF), "wool", "lintel"), "lintel", "lintel-ins", ALL_BUT_OUT),
        th(box("th-lintel-shell", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (EXT, TONGUE), "concrete", "lintel"), "lintel", "lintel-shell", ALL_BUT_OUT),
        th(box("th-sill-inner", SILL_X, (SILL_Y, PARAPET), (TONGUE, INT), "block", "sill"), "sill", "sill-block", ALL_BUT_OUT),
        ref("th-sill-concrete", "sill", "sill", "sill-concrete", ALL_BUT_OUT),
    ]


def thermal500():
    """🌡️ 1:500 thermal: the field keeps its three shells (the 140 mm wool is wider than the 75 mm feature) over the field and the sill zone, as the 1:500 drawing merges them; lintel and sill are one solid each."""
    hole = [[(LINTEL_X[0], LINTEL_TOP), (LINTEL_X[1], LINTEL_TOP), (LINTEL_X[1], LINTEL_Y), (OPEN[1], LINTEL_Y), (OPEN[1], PARAPET), (OPEN[0], PARAPET), (OPEN[0], LINTEL_Y), (LINTEL_X[0], LINTEL_Y)]]
    return [
        th(slab("th-field-inner", WALL, hole, (LEAF, INT), "block"), "masonry-field", "inner-leaf", ALL_BUT_OUT),
        th(slab("th-field-fill", WALL, hole, (FILL, LEAF), "wool"), "masonry-field", "fill", ALL_BUT_OUT),
        th(slab("th-field-facing", WALL, hole, (EXT, FILL), "brick"), "masonry-field", "facing", ALL_BUT_OUT),
        ref("th-lintel", "lintel", "lintel", "lintel-shell", ALL_BUT_OUT),
        ref("th-sill", "sill", "sill", "sill-concrete", ALL_BUT_OUT),
    ]


def st(solid, member, part, **extra):
    """🏗️ A body-like solid turned structural."""
    solid["member"] = member
    solid["part"] = part
    solid.update(extra)
    return solid


def supports(feature):
    """🟩 One contact pad per foundation anchor, the anchor's 0.5 m x 0.445 m footprint, one smallest feature thick, in the foot of the wall."""
    return [st(box(f"support-{a}", (x - 0.25, x + 0.25), (0.0, feature), (EXT, INT), "concrete", tone=0.15), "wall", "support", anchor=a)
            for a, x in (("found-0", 0.4), ("found-1", 1.5), ("found-2", 2.6))]


WALL_ZONES = {"lower": rect(0, 0, LENGTH, PARAPET), "pier-left": rect(0, PARAPET, OPEN[0], LINTEL_Y), "pier-right": rect(OPEN[1], PARAPET, LENGTH, LINTEL_Y), "upper": rect(0, LINTEL_TOP, LENGTH, HEIGHT)}


def wall_zone_masses(body):
    """⚖️ The wall's mass lumped per zone (parapet, two piers, band above the lintel) in proportion to the zone's face area, at the zone centre and the wall's mean depth: the wall's own centre of mass lies in the opening, so one sphere there would float in air."""
    m, _, _, z = bone_masses(body)["wall"]
    zones = [(n, (r[2][0] - r[0][0]) * (r[2][1] - r[0][1]), (r[0][0] + r[2][0]) / 2, (r[0][1] + r[2][1]) / 2) for n, r in WALL_ZONES.items()]
    total = sum(a for _, a, _, _ in zones)
    return [(n, m * a / total, x, y, z) for n, a, x, y in zones]


def masses(r, body):
    """🟣 Mass solids: the wall lumped in its four zones (its own centre of mass lies in the opening), lintel and sill at their centres of mass (from the 1:50 body and the declared densities)."""
    out = []
    for name, m, x, y, z in wall_zone_masses(body):
        out.append(st({"id": f"mass-wall-{name}", "kind": "ellipsoid", "bone": "wall", "material": "block", "role": "detail", "tone": 0.2,
                       "radii": [r, r, r], "transform": {"translation": [r5(x), r5(y), r5(z)]}}, "wall", "mass"))
    for bone, (m, x, y, z) in bone_masses(body).items():
        if bone == "wall":
            continue
        out.append(st({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": "concrete" if bone != "wall" else "block", "role": "detail", "tone": 0.2,
                       "radii": [r, r, r], "transform": {"translation": [r5(x - BONE_ORIGIN[bone][0]), r5(y - BONE_ORIGIN[bone][1]), r5(z)]}}, bone, "mass"))
    return out


def structure50():
    """🏗️ 1:50 structure: the 175 mm calcium silicate leaf as the wall member, both precast lintels with their four bearings, the sill on its bed, the beam bearing pad on the wall head, every wall tie, the foundation contacts and the three centres of mass."""
    f = 0.0075
    out = [
        st(slab("member-wall-leaf", WALL, [LEAF_HOLE], (LEAF, PLASTER), "block"), "wall", "member"),
        {"id": "member-lintel-inner", "of": "lintel-inner", "member": "lintel", "part": "member"},
        {"id": "member-lintel-outer", "of": "lintel-outer", "member": "lintel", "part": "member"},
        {"id": "member-sill", "of": "sill", "member": "sill", "part": "member"},
        {"id": "member-sill-end-left", "of": "sill-end-left", "member": "sill", "part": "member"},
        {"id": "member-sill-end-right", "of": "sill-end-right", "member": "sill", "part": "member"},
        st(box("joint-lintel-inner-left", (LINTEL_X[0], OPEN[0] - 0.015), (LINTEL_Y - f, LINTEL_Y), (LEAF, PLASTER), "mortar", "lintel"), "lintel", "joint"),
        st(box("joint-lintel-inner-right", (OPEN[1] + 0.015, LINTEL_X[1]), (LINTEL_Y - f, LINTEL_Y), (LEAF, PLASTER), "mortar", "lintel"), "lintel", "joint"),
        st(box("joint-lintel-outer-left", (LINTEL_X[0], OPEN[0]), (LINTEL_Y, OUTER_SOFFIT), (EXT, FILL), "mortar", "lintel"), "lintel", "joint"),
        st(box("joint-lintel-outer-right", (OPEN[1], LINTEL_X[1]), (LINTEL_Y, OUTER_SOFFIT), (EXT, FILL), "mortar", "lintel"), "lintel", "joint"),
        st(box("joint-sill-bed", SILL_X, (SILL_Y, SILL_Y + BED), (EXT, FILL), "mortar", "sill"), "sill", "joint"),
        st(box("joint-beam-bearing", (0.0, 0.15), (HEIGHT - f, HEIGHT), (LEAF, PLASTER), "mortar", "load-carrier"), "load-carrier", "joint"),
    ]
    out += [st(s, "wall", "joint") for s in ties(f)]
    return out + supports(f) + masses(0.04, body50())


def structure100():
    """🏗️ 1:100 structure: the leaf, both lintels on their bearings, the sill, the beam bearing pad, the ties as slender wires, contacts and masses."""
    f = 0.015
    out = [
        st(slab("member-wall-leaf", WALL, [LEAF_HOLE_COARSE], (LEAF, PLASTER), "block"), "wall", "member"),
        {"id": "member-lintel-inner", "of": "lintel-inner", "member": "lintel", "part": "member"},
        {"id": "member-lintel-outer", "of": "lintel-outer", "member": "lintel", "part": "member"},
        {"id": "member-sill", "of": "sill", "member": "sill", "part": "member"},
        st(box("joint-lintel-left", (LINTEL_X[0], OPEN[0]), (LINTEL_Y - f, LINTEL_Y), (EXT, PLASTER), "mortar", "lintel"), "lintel", "joint"),
        st(box("joint-lintel-right", (OPEN[1], LINTEL_X[1]), (LINTEL_Y - f, LINTEL_Y), (EXT, PLASTER), "mortar", "lintel"), "lintel", "joint"),
        st(box("joint-beam-bearing", (0.0, 0.15), (HEIGHT - f, HEIGHT), (LEAF, PLASTER), "mortar", "load-carrier"), "load-carrier", "joint"),
    ]
    out += [st(s, "wall", "joint") for s in ties(f)]
    return out + supports(f) + masses(0.05, body50())


def structure200():
    """🏗️ 1:200 structure: the leaf, the inner lintel as the bearing member over the opening, the outer lintel, the precast sill, contacts and masses; ties and bearings are below the 30 mm feature."""
    f = 0.03
    return [
        st(slab("member-wall-leaf", WALL, [LEAF_HOLE_COARSE], (LEAF, PLASTER), "block"), "wall", "member"),
        st(box("member-lintel-inner", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (LEAF, PLASTER), "concrete", "lintel"), "lintel", "member"),
        st(box("member-lintel-outer", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (EXT, TONGUE), "concrete", "lintel"), "lintel", "member"),
        {"id": "member-sill", "of": "sill", "member": "sill", "part": "member"},
    ] + supports(f) + masses(0.06, body50())


def structure500():
    """🏗️ 1:500 structure: the leaf, the inner lintel, the precast sill in the wall plane, contacts and masses."""
    f = 0.075
    return [
        st(slab("member-wall-leaf", WALL, [LEAF_HOLE_COARSE], (LEAF, PLASTER), "block"), "wall", "member"),
        st(box("member-lintel", LINTEL_X, (LINTEL_Y, LINTEL_TOP), (LEAF, PLASTER), "concrete", "lintel"), "lintel", "member"),
        st(box("member-sill", SILL_X, (SILL_Y, PARAPET), (EXT, TONGUE), "concrete", "sill"), "sill", "member"),
    ] + supports(f) + masses(0.075, body50())
