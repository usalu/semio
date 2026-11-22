"""🧱️ Authors the reclaimed brick pallet (Reichsformat bricks on an EPAL pallet) from hand-chosen numbers and writes 🔣️.json next to this script: the one authoritative chisel of this asset."""
import math

BL, BW, BH = 0.25, 0.12, 0.065
GAP = 0.010
BED = 0.012
PITCH_L, PITCH_W = BL + GAP, BW + GAP
COURSE = BH + BED
NL = 16
SX0, SX1, SZ0, SZ1, SY0 = 0.080, 1.120, -0.390, 0.390, 0.144
ROW_L = 4 * BL + 3 * GAP
STAGGER = 0.010
STACK_TOP = SY0 + NL * BH
BRICK_KG = 3.51
REMNANT_KG = 0.10
UNIT_KG = BRICK_KG + REMNANT_KG
PALLET_RHO = 550.0
PP_T, PP_LEG = 0.003, 0.065
PET_W, PET_T = 0.019, 0.0009
STRAP_Y = (0.404, 0.924)
LIFT_Y = 0.25
WALL_X0, WALL_Z = 1.45, -0.62
WALL_XM = WALL_X0 + 0.515 + 0.065
COURSE_LAYERS = (12, 14)

BOTTOM = [(-0.400, -0.300), (-0.0725, 0.0725), (0.300, 0.400)]
BLOCK_X = [(0.0, 0.145), (0.5275, 0.6725), (1.055, 1.200)]
DECK = [(-0.400, -0.255), (-0.21375, -0.11375), (-0.0725, 0.0725), (0.11375, 0.21375), (0.255, 0.400)]
Y_BOTTOM, Y_BLOCK, Y_STRINGER, Y_DECK = (0.0, 0.022), (0.022, 0.100), (0.100, 0.122), (0.122, 0.144)


def kind_of(layer):
    """🔀️ "A" for a layer of rows along x (even, from the pallet up), "B" for a layer of rows along z."""
    return "A" if layer % 2 == 0 else "B"


def layer_y(layer):
    """📏️ Bottom height of a layer (0-based) in world metres."""
    return SY0 + layer * BH


def a_x0(j):
    """↔️ Start of A row j: the rows lie side by side, every second one 10 mm further ahead, so no head joint runs through the layer."""
    return SX0 + STAGGER * (j % 2)


def a_z0(j):
    """↕️ Right edge of A row j."""
    return -0.36 + BW * j


def b_x0(i):
    """↔️ Start of B row i (rows side by side)."""
    return 0.12 + BW * i


def b_z0(i):
    """↕️ Right end of B row i, every second row 10 mm further left."""
    return SZ0 + STAGGER * (i % 2)


def extent(kind):
    """📦️ The plan extent (x0, x1, z0, z1) of a layer of a kind: A 1040 x 720 mm, B 960 x 780 mm."""
    return (SX0, SX1, -0.36, 0.36) if kind == "A" else (0.12, 1.08, SZ0, SZ1)


def bricks(layer):
    """🧱️ The 24 bricks of a layer as world rectangles in plan: (key, x0, x1, z0, z1); key (row, index) is the same for every layer of a kind."""
    out = []
    if kind_of(layer) == "A":
        for j in range(6):
            for i in range(4):
                x0, z0 = a_x0(j) + PITCH_L * i, a_z0(j)
                out.append(((j, i), x0, x0 + BL, z0, z0 + BW))
    else:
        for i in range(8):
            for j in range(3):
                x0, z0 = b_x0(i), b_z0(i) + PITCH_L * j
                out.append(((i, j), x0, x0 + BW, z0, z0 + BL))
    return out


def add(a, b):
    """➕️ Vector sum."""
    return tuple(x + y for x, y in zip(a, b))


def sub(a, b):
    """➖️ Vector difference."""
    return tuple(x - y for x, y in zip(a, b))


def dot(a, b):
    """✖️ Scalar product."""
    return sum(x * y for x, y in zip(a, b))


def scale(a, s):
    """🔍️ Vector times scalar."""
    return tuple(x * s for x in a)


X, Y, Z = (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)
UP_FRAME = ((0.0, 1.0, 0.0), (-1.0, 0.0, 0.0), (0.0, 0.0, 1.0))
WORLD_FRAME = (X, Y, Z)


def euler(parent, child):
    """🧭️ Rest placement angles (angleDeg, abductionDeg, twistDeg) that turn the parent axes into the child axes, R = Rw(a)·Rv(b)·Ru(c) as the rig solver composes them."""
    r = [[dot(child[j], parent[i]) for j in range(3)] for i in range(3)]
    b = math.asin(max(-1.0, min(1.0, r[2][0])))
    a = math.atan2(r[1][0], r[0][0])
    ca, sa, cb, sb = math.cos(a), math.sin(a), math.cos(b), math.sin(b)
    rw = [[ca, -sa, 0], [sa, ca, 0], [0, 0, 1]]
    rv = [[cb, 0, -sb], [0, 1, 0], [sb, 0, cb]]
    m1 = [[sum(rw[i][k] * rv[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    m = [[sum(m1[k][i] * r[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    c = math.atan2(-m[2][1], m[1][1])
    return math.degrees(a), math.degrees(b), math.degrees(c)


def turned(parent, a, b, c):
    """🔄️ The child axes of a rest placement, exactly as the rig solver turns them (for the self-check of {@link euler})."""
    pu, pv, pw = parent
    ca, sa = math.cos(math.radians(a)), math.sin(math.radians(a))
    u1, v1 = add(scale(pu, ca), scale(pv, sa)), add(scale(pu, -sa), scale(pv, ca))
    cb, sb = math.cos(math.radians(b)), math.sin(math.radians(b))
    u2, w2 = add(scale(u1, cb), scale(pw, sb)), add(scale(u1, -sb), scale(pw, cb))
    cc, sc = math.cos(math.radians(c)), math.sin(math.radians(c))
    return u2, add(scale(v1, cc), scale(w2, -sc)), add(scale(v1, sc), scale(w2, cc))


def course_rows():
    """🪜️ The twelve wall courses: source layer and row, rest origin (bottom of the row at the wall's centre line offset) and wall origin."""
    out = []
    for k in range(12):
        layer = COURSE_LAYERS[0] if k < 6 else COURSE_LAYERS[1]
        j = k % 6
        zc = a_z0(j) + BW / 2
        xc = a_x0(j) + ROW_L / 2
        shift = 0.065 if k % 2 == 0 else -0.065
        rest = (xc + shift, layer_y(layer), zc)
        wall = (WALL_XM, COURSE * k, WALL_Z)
        out.append({"k": k, "layer": layer, "row": j, "rest": rest, "wall": wall, "zc": zc, "xc": xc, "x0": a_x0(j)})
    return out


def frames():
    """🦴️ World rest frames (origin, (u, v, w)) of every bone that carries geometry or anchors, and the course chain placements."""
    f = {"pallet": ((0.0, 0.0, 0.0), WORLD_FRAME), "stack": ((0.6, SY0, 0.0), UP_FRAME), "straps": ((0.6, SY0, 0.0), UP_FRAME), "corners": ((0.6, SY0, 0.0), UP_FRAME)}
    f["layer-14"] = ((0.6, layer_y(13), 0.0), UP_FRAME)
    f["layer-16"] = ((0.6, layer_y(15), 0.0), UP_FRAME)
    for c in course_rows():
        f[f"course-{c['k'] + 1:02d}"] = (c["rest"], UP_FRAME)
    return f


FRAMES = frames()


def local(bone, p):
    """📍️ A world rest point in the bone-local frame (u, v, w)."""
    o, axes = FRAMES[bone]
    d = sub(p, o)
    return tuple(round(dot(d, a), 7) for a in axes)


def local_dir(bone, d):
    """🧭️ A world direction in the bone-local frame."""
    return tuple(round(dot(d, a), 7) for a in FRAMES[bone][1])


def local_size(bone, s):
    """📦️ A world box size in the bone-local frame (the bone axes are permutations of the world axes)."""
    return tuple(round(abs(dot(s, a)), 7) for a in FRAMES[bone][1])


def elev_pt(bone, x, y):
    """✏️ A side-elevation point (world x, y) in the bone's (u, v) plane."""
    l = local(bone, (x, y, FRAMES[bone][0][2]))
    return [round(l[0], 6), round(l[1], 6)]


def T(en, de):
    """🌐️ A bilingual text, English first."""
    return {"en": en, "de": de}


def rnd(v):
    """🎯️ Rounds a number for the document."""
    return round(v, 6)


def tone_of(*seed):
    """🎨️ A deterministic brick tone between 0.36 and 0.50, so neighbouring bricks are a shade apart as fired clay is."""
    h = 0
    for s in str(seed):
        h = (h * 131 + ord(s)) % 1000003
    return round(0.36 + 0.14 * ((h * 0.6180339887) % 1.0), 3)


def erect(bone, x0, x1, y0, y1):
    """▭️ A side-elevation rectangle given in world x, y, as a rect in the bone's (u, v) plane."""
    a, b = elev_pt(bone, x0, y0), elev_pt(bone, x1, y1)
    return {"kind": "rect", "x": rnd(min(a[0], b[0])), "y": rnd(min(a[1], b[1])), "w": rnd(abs(a[0] - b[0])), "h": rnd(abs(a[1] - b[1]))}


def epoly(bone, pts, closed=True):
    """✒️ A side-elevation polygon given in world x, y."""
    return {"kind": "poly", "points": [elev_pt(bone, x, y) for x, y in pts], "closed": closed}


def frect(z0, z1, y0, y1):
    """▯️ A front-elevation rectangle: paper x is the lateral world z, paper y the height."""
    return {"kind": "rect", "x": rnd(z0), "y": rnd(y0), "w": rnd(z1 - z0), "h": rnd(y1 - y0)}


def fpoly(pts, closed=True):
    """〰️ A front-elevation polyline of (z, y) points."""
    return {"kind": "poly", "points": [[rnd(a), rnd(b)] for a, b in pts], "closed": closed}


def prect(x0, x1, z0, z1):
    """⬜️ A plan rectangle: paper x ahead (world x), paper y the lateral world z."""
    return {"kind": "rect", "x": rnd(x0), "y": rnd(z0), "w": rnd(x1 - x0), "h": rnd(z1 - z0)}


def ppoly(pts, closed=True):
    """🔷️ A plan polygon of (x, z) points."""
    return {"kind": "poly", "points": [[rnd(a), rnd(b)] for a, b in pts], "closed": closed}


def shape(sid, bone, role, z, tone, geom, region=None, regions=None, material=None, filled=True, pen=None, hatch=None):
    """✏️ One authored 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
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
    return s


def sbox(sid, bone, c, s, material, tone, role="body", repeat=None, **kw):
    """📦️ A box solid given by its world rest centre and size, written in the bone's frame."""
    out = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": list(local_size(bone, s)), "transform": {"translation": list(local(bone, c))}}
    if repeat:
        out["repeat"] = {"count": repeat[0], "translation": list(local_dir(bone, repeat[1]))}
    out.update(kw)
    return out


def wbox(sid, bone, x0, x1, y0, y1, z0, z1, material, tone, **kw):
    """🧊️ A box solid given by its world rest extents."""
    return sbox(sid, bone, ((x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2), (x1 - x0, y1 - y0, z1 - z0), material, tone, **kw)


def wextrude(sid, bone, pts, z0, z1, material, tone, **kw):
    """🔺️ An extrusion of a side-elevation profile (world x, y) between the lateral positions z0 and z1."""
    o = FRAMES[bone][0]
    out = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": [elev_pt(bone, x, y) for x, y in pts], "plane": "xy", "from": rnd(z0 - o[2]), "to": rnd(z1 - o[2])}
    out.update(kw)
    return out


def ref(sid, of, **kw):
    """🔗️ A thermal or structural reference to a body solid of the same scale."""
    out = {"id": sid, "of": of}
    out.update(kw)
    return out


CLAY, MORTAR, TIMBER, PET, PP = "clay-brick-reclaimed", "lime-mortar-remnant", "pallet-softwood", "pet-strap", "pp-corner"
R_BRICK, R_MORTAR, R_PALLET, R_STRAP, R_CORNER = "brick-faces", "mortar-remnants", "pallet-timber", "pet-straps", "corner-strips"
COURSE_IDS = [f"course-{k + 1:02d}" for k in range(12)]


def bone_of_layer(layer):
    """🦴️ The bone that carries a layer at rest (the two set-down layers and the stack; the course layers are split into rows)."""
    return {13: "layer-14", 15: "layer-16"}.get(layer, "stack")


CHIPS = {(0, (0, 1)): "tr", (2, (0, 3)): "tl", (4, (0, 2)): "tr", (6, (0, 3)): "tl", (8, (0, 2)): "tl", (10, (0, 1)): "tr",
         (1, (3, 0)): "tr", (5, (6, 0)): "tl", (7, (1, 0)): "tr", (9, (4, 0)): "tl", (11, (2, 0)): "tr"}
CHIP_DEPTH = 0.025


def chip_profile(x0, x1, y0, y1, corner, kind):
    """🪨️ The face outline of a brick with a chipped top arris (a conchoidal break approximated by a two-facet cut)."""
    cx, cy = (0.035, 0.020) if kind == "A" else (0.030, 0.018)
    if corner == "tr":
        return [(x0, y0), (x1, y0), (x1, y1 - cy), (x1 - 0.45 * cx, y1 - 0.4 * cy), (x1 - cx, y1), (x0, y1)]
    return [(x0, y0), (x1, y0), (x1, y1), (x0 + cx, y1), (x0 + 0.45 * cx, y1 - 0.4 * cy), (x0, y1 - cy)]


def patches(scale):
    """🩶️ Lime mortar remnants on the faces of the bricks as world boxes 3 mm proud: (id, bone, face, x0, x1, y0, y1, z0, z1)."""
    t = 0.003
    out = []
    side = [(0, 1, 0.41, 0.48, 0.0, 0.018), (6, 1, 0.41, 0.48, 0.0, 0.018), (10, 1, 0.42, 0.50, 0.0, 0.016), (2, 2, 0.68, 0.765, 0.047, 0.065), (8, 2, 0.69, 0.77, 0.047, 0.065)]
    za = a_z0(0)
    for L, i, a, b, c, d in side:
        out.append((f"mortar-side-l{L + 1:02d}", "stack", "side", a, b, layer_y(L) + c, layer_y(L) + d, za - t, za))
    for L, i, a, b, c, d in [(3, 4, 0.625, 0.68, 0.02, 0.05), (9, 4, 0.625, 0.68, 0.02, 0.05), (5, 6, 0.88, 0.93, 0.01, 0.04), (11, 6, 0.88, 0.93, 0.01, 0.04), (1, 2, 0.38, 0.44, 0.03, 0.06)]:
        out.append((f"mortar-end-l{L + 1:02d}", "stack", "side", a, b, layer_y(L) + c, layer_y(L) + d, b_z0(i) - t, b_z0(i)))
    out.append(("mortar-end-l14", "layer-14", "side", 0.50, 0.56, layer_y(13) + 0.02, layer_y(13) + 0.045, b_z0(3) - t, b_z0(3)))
    out.append(("mortar-end-l16", "layer-16", "side", 0.75, 0.80, layer_y(15) + 0.03, layer_y(15) + 0.055, b_z0(5) - t, b_z0(5)))
    for L, j, a, b, c, d in [(0, 2, -0.11, -0.05, 0.01, 0.035), (6, 2, -0.11, -0.05, 0.01, 0.035), (10, 4, 0.15, 0.21, 0.03, 0.055)]:
        xe = a_x0(j) + ROW_L
        out.append((f"mortar-front-l{L + 1:02d}", "stack", "front", xe, xe + t, layer_y(L) + c, layer_y(L) + d, a, b))
    for L, a, b, c, d in [(1, -0.02, 0.06, 0.04, 0.062), (7, -0.02, 0.06, 0.04, 0.062)]:
        xe = b_x0(7) + BW
        out.append((f"mortar-front-l{L + 1:02d}", "stack", "front", xe, xe + t, layer_y(L) + c, layer_y(L) + d, a, b))
    for n, (a, b, c, d) in enumerate([(0.36, 0.44, -0.30, -0.25), (0.75, 0.81, 0.05, 0.11), (1.00, 1.06, 0.20, 0.27)]):
        out.append((f"mortar-top-{n + 1}", "layer-16", "top", a, b, STACK_TOP, STACK_TOP + t, c, d))
    return out


def pallet_parts():
    """🪵️ The EPAL 1 parts as world boxes: (id, x0, x1, y0, y1, z0, z1)."""
    out = []
    for n, (z0, z1) in enumerate(BOTTOM):
        out.append((f"bottom-board-{n + 1}", 0.0, 1.2, *Y_BOTTOM, z0, z1))
    for m, (x0, x1) in enumerate(BLOCK_X):
        for n, (z0, z1) in enumerate(BOTTOM):
            out.append((f"block-{m + 1}{n + 1}", x0, x1, *Y_BLOCK, z0, z1))
    for m, (x0, x1) in enumerate(BLOCK_X):
        out.append((f"stringer-board-{m + 1}", x0, x1, *Y_STRINGER, -0.4, 0.4))
    for n, (z0, z1) in enumerate(DECK):
        out.append((f"deck-board-{n + 1}", 0.0, 1.2, *Y_DECK, z0, z1))
    return out


def box_area(x0, x1, y0, y1, z0, z1):
    """📐️ The surface of a box."""
    a, b, c = x1 - x0, y1 - y0, z1 - z0
    return 2 * (a * b + b * c + c * a)


def overlap(a0, a1, b0, b1):
    """↔️ The overlap length of two intervals."""
    return max(0.0, min(a1, b1) - max(a0, b0))


def contact_area(p, q):
    """🤝️ The face contact of two touching boxes (zero when they do not share a face)."""
    _, ax0, ax1, ay0, ay1, az0, az1 = p
    _, bx0, bx1, by0, by1, bz0, bz1 = q
    e = 1e-9
    if abs(ay1 - by0) < e or abs(by1 - ay0) < e:
        return overlap(ax0, ax1, bx0, bx1) * overlap(az0, az1, bz0, bz1)
    if abs(ax1 - bx0) < e or abs(bx1 - ax0) < e:
        return overlap(ay0, ay1, by0, by1) * overlap(az0, az1, bz0, bz1)
    if abs(az1 - bz0) < e or abs(bz1 - az0) < e:
        return overlap(ax0, ax1, bx0, bx1) * overlap(ay0, ay1, by0, by1)
    return 0.0


def pallet_area():
    """📏️ The outer surface of the pallet: all faces of its parts less the faces they share."""
    parts = pallet_parts()
    total = sum(box_area(*p[1:]) for p in parts)
    shared = sum(contact_area(parts[i], parts[j]) for i in range(len(parts)) for j in range(i + 1, len(parts)))
    return total - 2 * shared


def pallet_volume():
    """🧮️ The timber volume of the pallet."""
    return sum((p[2] - p[1]) * (p[4] - p[3]) * (p[6] - p[5]) for p in pallet_parts())


def brick_area():
    """🧱️ The brick surface the air reaches in the stored stack: every face less the bed faces two layers share (the 10 mm joints are open)."""
    per = 2 * (BL * BW + BW * BH + BL * BH)
    items = [(None, x0, x1, layer_y(L), layer_y(L) + BH, z0, z1) for L in range(NL) for _, x0, x1, z0, z1 in bricks(L)]
    shared = 0.0
    for a in range(len(items)):
        for b in range(a + 1, len(items)):
            if abs(items[a][3] - items[b][3]) < BH * 1.5:
                shared += contact_area(items[a], items[b])
    return NL * 24 * per - 2 * shared, shared


def corner_plates():
    """📐️ The four L-shaped corner strips as two plates each: (id, x0, x1, y0, y1, z0, z1)."""
    out = []
    y0, y1 = SY0, STACK_TOP
    for cx, sx in (("rear", -1), ("front", 1)):
        xe = SX0 if sx < 0 else SX1
        for cz, sz in (("right", -1), ("left", 1)):
            ze = SZ0 if sz < 0 else SZ1
            xa, xb = (xe - PP_T, xe) if sx < 0 else (xe, xe + PP_T)
            za, zb = (ze, ze + PP_LEG) if sz < 0 else (ze - PP_LEG, ze)
            out.append((f"corner-{cx}-{cz}-end", xa, xb, y0, y1, za, zb))
            xa2, xb2 = (xe - PP_T, xe - PP_T + PP_T + PP_LEG) if sx < 0 else (xe - PP_LEG, xe + PP_T)
            za2, zb2 = (ze - PP_T, ze) if sz < 0 else (ze, ze + PP_T)
            out.append((f"corner-{cx}-{cz}-side", xa2, xb2, y0, y1, za2, zb2))
    return out


def strap_sheets():
    """🎗️ The two PET girth straps as four sheets each, lying on the corner strips: (id, x0, x1, y0, y1, z0, z1)."""
    out = []
    xo0, xo1 = SX0 - PP_T - PET_T, SX1 + PP_T + PET_T
    zo0, zo1 = SZ0 - PP_T - PET_T, SZ1 + PP_T + PET_T
    for n, y in enumerate(STRAP_Y):
        y1 = y + PET_W
        out.append((f"strap-{n + 1}-right", xo0, xo1, y, y1, zo0, zo0 + PET_T))
        out.append((f"strap-{n + 1}-left", xo0, xo1, y, y1, zo1 - PET_T, zo1))
        out.append((f"strap-{n + 1}-rear", xo0, xo0 + PET_T, y, y1, zo0 + PET_T, zo1 - PET_T))
        out.append((f"strap-{n + 1}-front", xo1 - PET_T, xo1, y, y1, zo0 + PET_T, zo1 - PET_T))
    return out


def vol(b):
    """🧮️ The volume of a world box tuple."""
    return (b[2] - b[1]) * (b[4] - b[3]) * (b[6] - b[5])


MASS = {
    "pallet": PALLET_RHO * pallet_volume(),
    "stack": 12 * 24 * UNIT_KG,
    "layer-14": 24 * UNIT_KG,
    "layer-16": 24 * UNIT_KG,
    "course": 4 * UNIT_KG,
    "corners": 905.0 * sum(vol(b) for b in corner_plates()),
    "straps": 1380.0 * sum(vol(b) for b in strap_sheets()),
}
MASS_TOTAL = MASS["pallet"] + MASS["stack"] + MASS["layer-14"] + MASS["layer-16"] + 12 * MASS["course"] + MASS["corners"] + MASS["straps"]


def path_frame(delta):
    """🛤️ The world axes of the guide of a course: u along the horizontal part of its set-down travel, v up."""
    hx, hz = delta[0], delta[2]
    n = math.hypot(hx, hz)
    if n < 1e-9:
        hx, hz, n = 1.0, 0.0, 1.0
    hx, hz = hx / n, hz / n
    return ((hx, 0.0, hz), Y, (-hz, 0.0, hx))


def course_chain():
    """⛓️ The twelve wall courses as a chain (course k rides on course k-1): per course the guide bone (fixed, its u along the travel), the course bone (prismatic along the guide) and the pose offset that sets it into the wall."""
    rows = course_rows()
    out = []
    prev_d = (0.0, 0.0, 0.0)
    for c in rows:
        k = c["k"]
        d = sub(c["wall"], c["rest"])
        delta = sub(d, prev_d)
        prev_d = d
        pf = path_frame(delta)
        parent = "pallet" if k == 0 else COURSE_IDS[k - 1]
        p_o, p_axes = FRAMES[parent]
        pos = sub(c["rest"], p_o)
        pa = euler(p_axes, pf)
        chk = turned(p_axes, *pa)
        assert all(abs(dot(chk[i], pf[i]) - 1) < 1e-9 for i in range(3)), ("guide", k)
        ca = euler(pf, UP_FRAME)
        chk = turned(pf, *ca)
        assert all(abs(dot(chk[i], UP_FRAME[i]) - 1) < 1e-9 for i in range(3)), ("course", k)
        h = math.hypot(delta[0], delta[2])
        alpha = math.degrees(math.atan2(delta[1], h))
        travel = math.sqrt(h * h + delta[1] ** 2)
        ar = math.radians(alpha)
        out.append({"k": k, "parent": parent, "pos": [rnd(dot(pos, a)) for a in p_axes], "guide": [round(x, 9) for x in pa], "course": [round(x, 9) for x in ca], "alpha": alpha, "travel": travel, "dx": travel * math.cos(ar), "dy": travel * math.sin(ar), "row": c})
    return out


def rig():
    """🦴️ The rig: pallet (root), the remaining stack, the two set-down layers, strapping, corner strips and the chain of twelve courses with their guides."""
    lit = "literature"
    bones = [
        {"id": "pallet", "parent": None, "title": T("Euro pallet EPAL 1 (stays with the stack)", "Europalette EPAL 1 (bleibt beim Stapel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 1.2, "joint": {"type": "fixed"},
         "mass": {"kg": rnd(MASS["pallet"]), "comX": 0.6, "comY": 0.072, "distribution": "uniform", "source": lit, "reference": "en-13698-1"},
         "section": {"areaM2": rnd(0.635 * 0.022), "secondMomentM4": float(f"{0.635 * 0.022 ** 3 / 12:.6e}"), "extremeFibreM": 0.011, "material": TIMBER, "title": T("Top deck, five boards 22 mm (635 mm wide)", "Deckbretter, fünf Bretter 22 mm (635 mm breit)")}},
        {"id": "stack", "parent": "pallet", "title": T("Brick stack, layers 1 to 12", "Ziegelstapel, Lagen 1 bis 12"), "x": 0.6, "y": SY0, "angleDeg": 90, "lengthM": rnd(12 * BH), "joint": {"type": "fixed"},
         "mass": {"kg": rnd(MASS["stack"]), "comX": rnd(6 * BH), "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1745"},
         "section": {"areaM2": rnd(24 * BL * BW), "secondMomentM4": float(f"{(SZ1 - SZ0) * (SX1 - SX0) ** 3 / 12 * 0.72 / ((SX1 - SX0) * (SZ1 - SZ0)):.6e}"), "extremeFibreM": rnd((SX1 - SX0) / 2), "material": CLAY, "title": T("Dry-stacked layer, 24 bricks (net 0.72 m²)", "Trocken gestapelte Lage, 24 Ziegel (netto 0,72 m²)")}},
        {"id": "straps", "parent": "stack", "title": T("Two PET girth straps", "Zwei PET-Umreifungsbänder"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"},
         "mass": {"kg": rnd(MASS["straps"]), "comX": rnd(sum(STRAP_Y) / 2 + PET_W / 2 - SY0), "comY": 0, "distribution": "point", "source": "authored-estimate"}},
        {"id": "corners", "parent": "stack", "title": T("Four polypropylene corner strips", "Vier Kantenschutzwinkel aus Polypropylen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"},
         "mass": {"kg": rnd(MASS["corners"]), "comX": rnd((STACK_TOP - SY0) / 2), "comY": 0, "distribution": "point", "source": "authored-estimate"}},
    ]
    for name, layer, travel in (("layer-14", 13, BH), ("layer-16", 15, 2 * BH)):
        bones.append({"id": name, "parent": "stack", "title": T(f"Layer {layer + 1} (set down when the courses are taken)", f"Lage {layer + 1} (abgesetzt, wenn die Scharen entnommen sind)"), "x": rnd(layer * BH), "y": 0, "angleDeg": 0, "lengthM": BH,
                      "joint": {"type": "prismatic", "axisDeg": 0, "minM": rnd(-travel), "maxM": 0},
                      "mass": {"kg": rnd(MASS[name]), "comX": rnd(BH / 2), "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1745"},
                      "section": {"areaM2": rnd(24 * BL * BW), "secondMomentM4": float(f"{(SZ1 - SZ0) * (SX1 - SX0) ** 3 / 12 * 0.72 / ((SX1 - SX0) * (SZ1 - SZ0)):.6e}"), "extremeFibreM": rnd((SX1 - SX0) / 2), "material": CLAY}})
    for c in course_chain():
        k = c["k"]
        cid = COURSE_IDS[k]
        g = f"guide-{k + 1:02d}"
        src = c["row"]
        bones.append({"id": g, "parent": c["parent"], "title": T(f"Guide of course {k + 1} (its set-down travel)", f"Führung der Schar {k + 1} (ihr Absetzweg)"), "x": c["pos"][0], "y": c["pos"][1], "z": c["pos"][2], "angleDeg": c["guide"][0], "abductionDeg": c["guide"][1], "twistDeg": c["guide"][2], "lengthM": 0, "joint": {"type": "fixed"}})
        cv = rnd(src["rest"][0] - src["xc"])
        bones.append({"id": cid, "parent": g, "title": T(f"Course {k + 1}: row {src['row'] + 1} of layer {src['layer'] + 1}, four bricks", f"Schar {k + 1}: Reihe {src['row'] + 1} der Lage {src['layer'] + 1}, vier Ziegel"), "x": 0, "y": 0, "angleDeg": c["course"][0], "abductionDeg": c["course"][1], "twistDeg": c["course"][2], "lengthM": BH,
                      "joint": {"type": "prismatic", "axisDeg": c["alpha"], "minM": 0, "maxM": rnd(max(c["travel"], 0.01) + 0.001)},
                      "mass": {"kg": rnd(MASS["course"]), "comX": rnd(BH / 2), "comY": cv, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1745"},
                      "section": {"areaM2": rnd(4 * BL * BW), "secondMomentM4": float(f"{BW * (4 * BL) * (SX1 - SX0) ** 2 / 12:.6e}"), "extremeFibreM": rnd((SX1 - SX0) / 2), "material": CLAY, "title": T("Course of four stretchers, 120 mm wall", "Schar aus vier Läufern, Wand 120 mm")}})
    return {"kind": "mechanical", "root": "pallet", "bones": bones}


def anchors():
    """📍️ Runner contacts on the ground, fork contacts under the stringer boards, the wall footing and the two load points of the reinstalled panel."""
    out = []
    for n, (z0, z1) in zip(("right", "centre", "left"), BOTTOM):
        out.append({"id": f"runner-{n}", "bone": "pallet", "x": 0.6, "y": 0, "z": rnd((z0 + z1) / 2), "kind": "contact", "title": T(f"Bottom board {n} on the yard surface", {"right": "Rechtes", "centre": "Mittleres", "left": "Linkes"}[n] + " Bodenbrett auf der Lagerfläche"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.2, "footprintWidthM": rnd(z1 - z0), "frictionCoefficient": 0.5})
    for n, s in (("right", -1), ("left", 1)):
        out.append({"id": f"fork-{n}", "bone": "pallet", "x": 0.6, "y": Y_STRINGER[0], "z": rnd(s * 0.18625), "kind": "contact", "title": T(f"Fork arm {n} under the stringer boards", ("Rechter" if n == "right" else "Linker") + " Gabelzinken unter den Querbrettern"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.15, "footprintWidthM": 0.1, "frictionCoefficient": 0.3})
    rows = course_rows()

    def lv(k):
        return rnd(-(rows[k]["wall"][0] - (WALL_XM - 0.065 if k % 2 == 0 else WALL_XM + 0.065)) * 0 - ((WALL_XM - 0.065 if k % 2 == 0 else WALL_XM + 0.065) - WALL_XM))
    out.append({"id": "wall-footing", "bone": COURSE_IDS[0], "x": 0, "y": lv(0), "kind": "contact", "title": T("Bed of the first course on the footing", "Lagerfuge der ersten Schar auf dem Fundament"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.03, "footprintWidthM": BW, "frictionCoefficient": 0.6})
    out.append({"id": "wall-top", "bone": COURSE_IDS[11], "x": BH, "y": lv(11), "kind": "load", "title": T("Top of the panel, resultant of the line load", "Wandkrone, Resultierende der Linienlast")})
    out.append({"id": "wall-face", "bone": COURSE_IDS[5], "x": BH, "y": lv(5), "z": rnd(-BW / 2), "kind": "load", "title": T("Centre of the wind pressure on the windward face", "Mitte des Winddrucks auf der Luvseite")})
    return out


def materials():
    """🧱️ The five materials of the lot."""
    return [
        {"id": CLAY, "title": T("Reclaimed solid clay brick, Reichsformat, weathered", "Gebrauchter Vollziegel im Reichsformat, bewittert"), "emissivity": 0.93, "solarAbsorptivity": 0.68, "densityKgM3": 1800, "conductivityWmK": 0.81, "specificHeatJKgK": 1000, "elasticModulusPa": 8.0e9, "strengthPa": 1.5e7, "source": "literature", "reference": "din-4108-4"},
        {"id": MORTAR, "title": T("Lime mortar remnant", "Kalkmörtelrest"), "emissivity": 0.91, "solarAbsorptivity": 0.5, "densityKgM3": 1700, "conductivityWmK": 0.87, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
        {"id": TIMBER, "title": T("Pallet softwood, weathered grey", "Palettenholz (Nadelholz), vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": PALLET_RHO, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 1.0e10, "strengthPa": 2.4e7, "source": "literature", "reference": "en-13698-1"},
        {"id": PET, "title": T("PET strapping 19 x 0.9 mm", "PET-Umreifungsband 19 x 0,9 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 1380, "conductivityWmK": 0.24, "specificHeatJKgK": 1200, "elasticModulusPa": 1.1e10, "strengthPa": 4.0e8, "source": "authored-estimate", "reference": "en-iso-10456"},
        {"id": PP, "title": T("Polypropylene corner strip 65 x 65 x 3 mm", "Kantenschutzwinkel aus Polypropylen 65 x 65 x 3 mm"), "emissivity": 0.92, "solarAbsorptivity": 0.55, "densityKgM3": 905, "conductivityWmK": 0.22, "specificHeatJKgK": 1700, "source": "authored-estimate", "reference": "en-iso-10456"},
    ]


TONE_PALLET, TONE_CORNER, TONE_STRAP, TONE_MORTAR = 0.64, 0.74, 0.28, 0.84
BRICK_PEN = {50: 0.13, 100: 0.13, 200: 0.1, 500: 0.15}


def face_bricks(layer):
    """👁️ The bricks of a layer seen in the side elevation (row at the right face) and in the front elevation (row at the front face), as (key, x0, x1) and (key, z0, z1)."""
    side, front = [], []
    for key, x0, x1, z0, z1 in bricks(layer):
        if (kind_of(layer) == "A" and key[0] == 0) or (kind_of(layer) == "B" and key[1] == 0):
            side.append((key, x0, x1))
        if (kind_of(layer) == "A" and key[1] == 3) or (kind_of(layer) == "B" and key[0] == 7):
            front.append((key, z0, z1))
    return side, front


def brick_layer_shapes(bone, layer, scale, out):
    """🧱️ The 1:50 drawing of one layer brick by brick in the three views (chipped arrises in the side elevation, mortar remnants elsewhere)."""
    kind = kind_of(layer)
    y0, y1 = layer_y(layer), layer_y(layer) + BH
    side, front = face_bricks(layer)
    pen = BRICK_PEN[scale]
    ex0, ex1, ez0, ez1 = extent(kind)
    out["elevation"].append(shape(f"layer-{layer + 1:02d}-joints-side", bone, "body", 1.9, 0.26, erect(bone, ex0, ex1, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
    out["front"].append(shape(f"layer-{layer + 1:02d}-joints-front", bone, "body", 1.9, 0.26, frect(ez0, ez1, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
    for n, (key, x0, x1) in enumerate(side):
        corner = CHIPS.get((layer, key)) if bone == "stack" else None
        geom = epoly(bone, chip_profile(x0, x1, y0, y1, corner, kind)) if corner else erect(bone, x0, x1, y0, y1)
        out["elevation"].append(shape(f"brick-l{layer + 1:02d}-s{n + 1}", bone, "body", 2 + 0.02 * (n % 2) + 0.01 * (layer % 2), tone_of(layer, key), geom, region=R_BRICK, material=CLAY, pen=pen))
    for n, (key, z0, z1) in enumerate(front):
        out["front"].append(shape(f"brick-l{layer + 1:02d}-f{n + 1}", bone, "body", 2 + 0.02 * (n % 2) + 0.01 * (layer % 2), tone_of(layer, key, "f"), frect(z0, z1, y0, y1), region=R_BRICK, material=CLAY, pen=pen))


def course_shapes(k, scale, out):
    """🪜️ The drawing of one wall course (a row of four stretchers) in the three views."""
    c = course_rows()[k]
    cid = COURSE_IDS[k]
    y0, y1 = layer_y(c["layer"]), layer_y(c["layer"]) + (COURSE if scale == 500 else BH)
    z0, z1 = c["zc"] - BW / 2, c["zc"] + BW / 2
    pen = BRICK_PEN[scale]
    ez = round(2 + 0.1 * (5 - c["row"]), 3)
    pz = 2.05 if c["layer"] == COURSE_LAYERS[0] else 2.12
    if scale == 50:
        out["elevation"].append(shape(f"{cid}-joints", cid, "body", ez, 0.26, erect(cid, c["x0"], c["x0"] + ROW_L, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
        out["plan"].append(shape(f"{cid}-joints", cid, "body", pz, 0.26, prect(c["x0"], c["x0"] + ROW_L, z0, z1), region=R_BRICK, material=CLAY, pen=pen))
        for i in range(4):
            x0 = c["x0"] + PITCH_L * i
            out["elevation"].append(shape(f"{cid}-brick-{i + 1}", cid, "body", round(ez + 0.02 + 0.02 * (i % 2), 3), tone_of("c", k, i), erect(cid, x0, x0 + BL, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
            out["plan"].append(shape(f"{cid}-brick-{i + 1}", cid, "body", round(pz + 0.01 + 0.01 * (i % 2), 3), tone_of("c", k, i), prect(x0, x0 + BL, z0, z1), region=R_BRICK, material=CLAY, pen=pen))
    else:
        out["elevation"].append(shape(f"{cid}-row", cid, "body", ez, tone_of("c", k), erect(cid, c["x0"], c["x0"] + ROW_L, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
        out["plan"].append(shape(f"{cid}-row", cid, "body", pz, tone_of("c", k), prect(c["x0"], c["x0"] + ROW_L, z0, z1), region=R_BRICK, material=CLAY, pen=pen))
    if scale <= 100:
            for i in range(3):
                xj = c["x0"] + PITCH_L * i + BL + GAP / 2
                out["elevation"].append(shape(f"{cid}-head-joint-{i + 1}", cid, "texture", round(ez + 0.05, 3), 0.3, epoly(cid, [(xj, y0), (xj, y1)], closed=False), region=R_BRICK, filled=False))
    out["front"].append(shape(f"{cid}-joints" if scale == 50 else f"{cid}-row", cid, "body", 2, tone_of("c", k, "f"), frect(z0, z1, y0, y1), region=R_BRICK, material=CLAY, pen=pen))


def mortar_shapes(scale, out):
    """🩶️ The mortar remnants as small filled texture shapes on the face they sit on."""
    for sid, bone, face, x0, x1, y0, y1, z0, z1 in patches(scale):
        if face == "side":
            out["elevation"].append(shape(sid, bone, "texture", 5, TONE_MORTAR, erect(bone, x0, x1, y0, y1), region=R_MORTAR, material=MORTAR, filled=False))
        elif face == "front":
            out["front"].append(shape(sid, bone, "texture", 5, TONE_MORTAR, frect(z0, z1, y0, y1), region=R_MORTAR, material=MORTAR, filled=False))
        else:
            out["plan"].append(shape(sid, bone, "texture", 5, TONE_MORTAR, prect(x0, x1, z0, z1), region=R_MORTAR, material=MORTAR, filled=False))


def pallet_shapes(scale, out):
    """🪵️ The pallet in the three views: board by board at 1:50 and 1:100, deck and stringers merged into one slab at 1:200, one block at 1:500."""
    b = "pallet"
    kw = {"region": R_PALLET, "material": TIMBER}
    E, F, P = out["elevation"], out["front"], out["plan"]
    if scale in (50, 100):
        E.append(shape("bottom-board-side", b, "body", 1, TONE_PALLET - 0.04, erect(b, 0, 1.2, *Y_BOTTOM), **kw))
        for m, (x0, x1) in enumerate(BLOCK_X):
            if scale == 50:
                E.append(shape(f"block-{m + 1}-side", b, "body", 1.01, TONE_PALLET - 0.06, erect(b, x0, x1, *Y_BLOCK), **kw))
                E.append(shape(f"stringer-board-{m + 1}-end", b, "body", 1.02, TONE_PALLET + 0.04, erect(b, x0, x1, *Y_STRINGER), **kw))
            else:
                E.append(shape(f"block-column-{m + 1}-side", b, "body", 1.01, TONE_PALLET - 0.06, erect(b, x0, x1, Y_BLOCK[0], Y_STRINGER[1]), **kw))
                E.append(shape(f"block-column-{m + 1}-seam", b, "detail", 1.5, 0.3, epoly(b, [(x0, Y_STRINGER[0]), (x1, Y_STRINGER[0])], closed=False), region=R_PALLET, filled=False))
        E.append(shape("deck-board-side", b, "body", 1.03, TONE_PALLET, erect(b, 0, 1.2, *Y_DECK), **kw))
        for n, (z0, z1) in enumerate(BOTTOM):
            F.append(shape(f"bottom-board-{n + 1}-end", b, "body", 1, TONE_PALLET - 0.04, frect(z0, z1, *Y_BOTTOM), **kw))
            F.append(shape(f"block-{n + 1}-end", b, "body", 1.01, TONE_PALLET - 0.06, frect(z0, z1, *Y_BLOCK), **kw))
        F.append(shape("stringer-board-end", b, "body", 1.02, TONE_PALLET + 0.04, frect(-0.4, 0.4, *Y_STRINGER), **kw))
        for n, (z0, z1) in enumerate(DECK):
            F.append(shape(f"deck-board-{n + 1}-end", b, "body", 1.03 + 0.02 * (n % 2), TONE_PALLET, frect(z0, z1, *Y_DECK), **kw))
        for m, (x0, x1) in enumerate(BLOCK_X):
            P.append(shape(f"stringer-board-{m + 1}-top", b, "body", 1, TONE_PALLET + 0.04, prect(x0, x1, -0.4, 0.4), **kw))
        for n, (z0, z1) in enumerate(DECK):
            P.append(shape(f"deck-board-{n + 1}-top", b, "body", 1.1 + 0.02 * (n % 2), TONE_PALLET, prect(0, 1.2, z0, z1), **kw))
    elif scale == 200:
        E.append(shape("bottom-board-side", b, "body", 1, TONE_PALLET - 0.04, erect(b, 0, 1.2, 0, 0.030), **kw))
        for m, (x0, x1) in enumerate(BLOCK_X):
            E.append(shape(f"block-{m + 1}-side", b, "body", 1.01, TONE_PALLET - 0.06, erect(b, x0, x1, 0.030, 0.100), **kw))
        E.append(shape("deck-slab-side", b, "body", 1.02, TONE_PALLET, erect(b, 0, 1.2, 0.100, 0.144), **kw))
        for n, (z0, z1) in enumerate(BOTTOM):
            F.append(shape(f"bottom-board-{n + 1}-end", b, "body", 1, TONE_PALLET - 0.04, frect(z0, z1, 0, 0.030), **kw))
            F.append(shape(f"block-{n + 1}-end", b, "body", 1.01, TONE_PALLET - 0.06, frect(z0, z1, 0.030, 0.100), **kw))
        F.append(shape("deck-slab-end", b, "body", 1.02, TONE_PALLET, frect(-0.4, 0.4, 0.100, 0.144), **kw))
        P.append(shape("deck-slab-top", b, "body", 1, TONE_PALLET, prect(0, 1.2, -0.4, 0.4), **kw))
    else:
        E.append(shape("pallet-side", b, "body", 1, TONE_PALLET, erect(b, 0, 1.2, 0, 0.144), **kw))
        F.append(shape("pallet-end", b, "body", 1, TONE_PALLET, frect(-0.4, 0.4, 0, 0.144), **kw))
        P.append(shape("pallet-top", b, "body", 1, TONE_PALLET, prect(0, 1.2, -0.4, 0.4), **kw))


def layer_slab_shapes(bone, layer, out, scale):
    """〰️ One layer as a slab of its own extent with its joints as texture lines (1:100): in the side elevation the head joints of the right row (A) or the row ends (B), in the front the row ends (A) or the head joints of the front row (B)."""
    kind = kind_of(layer)
    ex0, ex1, ez0, ez1 = extent(kind)
    yb, yt = layer_y(layer), layer_y(layer) + BH
    pen = BRICK_PEN[scale]
    if scale == 50:
        joint_lines(bone, layer, out)
        return
    out["elevation"].append(shape(f"layer-{layer + 1:02d}-side", bone, "body", 2, tone_of(layer), erect(bone, ex0, ex1, yb, yt), region=R_BRICK, material=CLAY, pen=pen))
    out["front"].append(shape(f"layer-{layer + 1:02d}-front", bone, "body", 2, tone_of(layer, "f"), frect(ez0, ez1, yb, yt), region=R_BRICK, material=CLAY, pen=pen))
    joint_lines(bone, layer, out)


def joint_lines(bone, layer, out):
    """〰️ The joints of a layer's visible faces as texture lines, so the line views read the bond as the tone views do."""
    kind = kind_of(layer)
    yb, yt = layer_y(layer), layer_y(layer) + BH
    xs = [a_x0(0) + PITCH_L * i + BL + GAP / 2 for i in range(3)] if kind == "A" else [b_x0(i) + BW for i in range(7)]
    zs = [a_z0(j) + BW for j in range(5)] if kind == "A" else [b_z0(7) + PITCH_L * j + BL + GAP / 2 for j in range(2)]
    for n, xj in enumerate(xs):
        out["elevation"].append(shape(f"joint-l{layer + 1:02d}-s{n + 1}", bone, "texture", 2.5, 0.3, epoly(bone, [(xj, yb), (xj, yt)], closed=False), region=R_BRICK, filled=False))
    for n, zj in enumerate(zs):
        out["front"].append(shape(f"joint-l{layer + 1:02d}-f{n + 1}", bone, "texture", 2.5, 0.3, fpoly([(zj, yb), (zj, yt)], closed=False), region=R_BRICK, filled=False))


def corner_shapes(scale, out):
    """📐️ The corner strips: their faces seen in the side and front elevations and the L of each in plan."""
    b = "corners"
    kw = {"region": R_CORNER, "material": PP}
    y0, y1 = SY0, STACK_TOP
    for n, (x0, x1) in enumerate(((SX0 - PP_T, SX0 + PP_LEG), (SX1 - PP_LEG, SX1 + PP_T))):
        out["elevation"].append(shape(f"corner-strip-side-{n + 1}", b, "body", 3, TONE_CORNER, erect(b, x0, x1, y0, y1), **kw))
    for n, (z0, z1) in enumerate(((SZ0 - PP_T, SZ0 + PP_LEG), (SZ1 - PP_LEG, SZ1 + PP_T))):
        out["front"].append(shape(f"corner-strip-front-{n + 1}", b, "body", 3, TONE_CORNER, frect(z0, z1, y0, y1), **kw))
    for sx, xe in ((-1, SX0), (1, SX1)):
        for sz, ze in ((-1, SZ0), (1, SZ1)):
            xo, zo = xe + sx * PP_T, ze + sz * PP_T
            xi, zi = xe - sx * PP_LEG, ze - sz * PP_LEG
            pts = [(xo, zo), (xi, zo), (xi, ze), (xe, ze), (xe, zi), (xo, zi)]
            out["plan"].append(shape(f"corner-strip-{'rear' if sx < 0 else 'front'}-{'right' if sz < 0 else 'left'}", b, "body", 3, TONE_CORNER, ppoly(pts), **kw))


def strap_shapes(scale, out):
    """🎗️ The two girth straps: bands in the elevations, their centre lines as outlines in plan (texture lines at 1:200)."""
    b = "straps"
    xo0, xo1 = SX0 - PP_T - PET_T, SX1 + PP_T + PET_T
    zo0, zo1 = SZ0 - PP_T - PET_T, SZ1 + PP_T + PET_T
    xc0, xc1, zc0, zc1 = xo0 + PET_T / 2, xo1 - PET_T / 2, zo0 + PET_T / 2, zo1 - PET_T / 2
    for n, y in enumerate(STRAP_Y):
        if scale <= 100:
            out["elevation"].append(shape(f"strap-{n + 1}-side", b, "body", 4, TONE_STRAP, erect(b, xo0, xo1, y, y + PET_W), region=R_STRAP, material=PET))
            out["front"].append(shape(f"strap-{n + 1}-front", b, "body", 4, TONE_STRAP, frect(zo0, zo1, y, y + PET_W), region=R_STRAP, material=PET))
    out["plan"].append(shape("strap-outline", b, "detail" if scale <= 100 else "texture", 4, TONE_STRAP, ppoly([(xc0, zc0), (xc1, zc0), (xc1, zc1), (xc0, zc1)]), region=R_STRAP, filled=False))


def geometry(scale):
    """🗺️ The three views of one preset scale."""
    out = {"elevation": [], "front": [], "plan": []}
    pallet_shapes(scale, out)
    stack_layers = list(range(12))
    if scale == 50:
        for L in stack_layers:
            brick_layer_shapes("stack", L, scale, out)
        brick_layer_shapes("layer-14", 13, scale, out)
        brick_layer_shapes("layer-16", 15, scale, out)
        for bone, L in [("stack", L) for L in stack_layers] + [("layer-14", 13), ("layer-16", 15)]:
            joint_lines(bone, L, out)
    elif scale == 100:
        for L in stack_layers:
            layer_slab_shapes("stack", L, out, scale)
        layer_slab_shapes("layer-14", 13, out, scale)
        layer_slab_shapes("layer-16", 15, out, scale)
    else:
        regions = [R_BRICK, R_MORTAR, R_STRAP] if scale == 200 else [R_BRICK, R_MORTAR, R_CORNER, R_STRAP]
        hatch = "coursing" if scale == 200 else None
        out["elevation"].append(shape("stack-side", "stack", "body", 2, 0.43, erect("stack", SX0, SX1, SY0, layer_y(12)), regions=regions, material=CLAY, hatch=hatch, pen=BRICK_PEN[scale]))
        out["front"].append(shape("stack-front", "stack", "body", 2, 0.45, frect(SZ0, SZ1, SY0, layer_y(12)), regions=regions, material=CLAY, hatch=hatch, pen=BRICK_PEN[scale]))
        bx0, bx1, bz0, bz1 = extent("B")
        for name, L in (("layer-14", 13), ("layer-16", 15)):
            out["elevation"].append(shape(f"{name}-band", name, "body", 2.7, tone_of(L), erect(name, SX0, SX1, layer_y(L - 1), layer_y(L) + BH), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
            out["front"].append(shape(f"{name}-band", name, "body", 2.7, tone_of(L, "f"), frect(SZ0, SZ1, layer_y(L - 1), layer_y(L) + BH), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
    top = extent("B") if scale <= 100 else (SX0, SX1, SZ0, SZ1)
    out["plan"].append(shape("stack-top", "stack", "body", 2, 0.43, prect(*top), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
    out["plan"].append(shape("layer-14-top", "layer-14", "body", 2.1, tone_of(13), prect(*(extent("B") if scale <= 100 else (SX0, SX1, SZ0, SZ1))), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
    if scale == 50:
        out["plan"].append(shape("layer-16-joints-top", "layer-16", "body", 2.15, 0.26, prect(*extent("B")), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
        for n, (key, x0, x1, z0, z1) in enumerate(bricks(15)):
            out["plan"].append(shape(f"brick-l16-t{n + 1}", "layer-16", "body", 2.2 + 0.02 * ((key[0] + key[1]) % 2), tone_of(15, key, "t"), prect(x0, x1, z0, z1), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
    else:
        out["plan"].append(shape("layer-16-top", "layer-16", "body", 2.2, tone_of(15, "t"), prect(*(extent("B") if scale == 100 else (SX0, SX1, SZ0, SZ1))), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
    if scale <= 100:
            for i in range(7):
                xj = b_x0(i) + BW
                out["plan"].append(shape(f"layer-16-row-joint-{i + 1}", "layer-16", "texture", 2.5, 0.3, ppoly([(xj, SZ0), (xj, SZ1)], closed=False), region=R_BRICK, filled=False))
            for i in range(8):
                for j in range(2):
                    zj = b_z0(i) + PITCH_L * j + BL + GAP / 2
                    out["plan"].append(shape(f"layer-16-head-joint-{i + 1}-{j + 1}", "layer-16", "texture", 2.5, 0.3, ppoly([(b_x0(i), zj), (b_x0(i) + BW, zj)], closed=False), region=R_BRICK, filled=False))
    for k in range(12):
        course_shapes(k, scale, out)
    if scale <= 200:
        corner_shapes(scale, out)
        strap_shapes(scale, out)
    if scale <= 100:
        mortar_shapes(scale, out)
    return out


def pallet_body(scale):
    """🪵️ The pallet solids of one scale: every board and block at 1:50 and 1:100, deck and stringers merged into one 44 mm slab and the bottom boards drawn at the 30 mm feature at 1:200, one block at 1:500."""
    b = "pallet"
    out = []
    if scale in (50, 100):
        for n, (z0, z1) in enumerate(BOTTOM):
            out.append(wbox(f"bottom-board-{n + 1}", b, 0, 1.2, *Y_BOTTOM, z0, z1, TIMBER, TONE_PALLET - 0.04 + 0.01 * n))
        for n, (z0, z1) in enumerate(BOTTOM):
            out.append(wbox(f"blocks-{n + 1}", b, *BLOCK_X[0], *Y_BLOCK, z0, z1, TIMBER, TONE_PALLET - 0.06, repeat=(3, (0.5275, 0, 0))))
        out.append(wbox("stringer-boards", b, *BLOCK_X[0], *Y_STRINGER, -0.4, 0.4, TIMBER, TONE_PALLET + 0.04, repeat=(3, (0.5275, 0, 0))))
        for n, (z0, z1) in enumerate(DECK):
            out.append(wbox(f"deck-board-{n + 1}", b, 0, 1.2, *Y_DECK, z0, z1, TIMBER, TONE_PALLET + 0.01 * (n % 3)))
    elif scale == 200:
        for n, (z0, z1) in enumerate(BOTTOM):
            out.append(wbox(f"bottom-board-{n + 1}", b, 0, 1.2, 0, 0.030, z0, z1, TIMBER, TONE_PALLET - 0.04))
        for n, (z0, z1) in enumerate(BOTTOM):
            out.append(wbox(f"blocks-{n + 1}", b, *BLOCK_X[0], 0.030, 0.100, z0, z1, TIMBER, TONE_PALLET - 0.06, repeat=(3, (0.5275, 0, 0))))
        out.append(wbox("deck-slab", b, 0, 1.2, 0.100, 0.144, -0.4, 0.4, TIMBER, TONE_PALLET))
    else:
        out.append(wbox("pallet-block", b, 0, 1.2, 0, 0.144, -0.4, 0.4, TIMBER, TONE_PALLET))
    return out


def stack_body_50():
    """🧱️ The twelve stack layers brick by brick: every brick position of a kind is one column repeated over its six layers, split where a brick of the right face has a chipped arris."""
    out = []
    for kind, first in (("A", 0), ("B", 1)):
        layers = [L for L in range(12) if kind_of(L) == kind]
        for key, x0, x1, z0, z1 in bricks(first):
            run = []
            runs = []
            for L in layers:
                if (L, key) in CHIPS and ((kind == "A" and key[0] == 0) or (kind == "B" and key[1] == 0)):
                    if run:
                        runs.append(("run", run))
                        run = []
                    runs.append(("chip", [L]))
                else:
                    run.append(L)
            if run:
                runs.append(("run", run))
            name = f"brick-{kind.lower()}-{key[0] + 1}-{key[1] + 1}"
            for what, ls in runs:
                L0 = ls[0]
                y0 = layer_y(L0)
                tone = tone_of(kind, key, L0)
                if what == "run":
                    out.append(wbox(f"{name}-l{L0 + 1:02d}", "stack", x0, x1, y0, y0 + BH, z0, z1, CLAY, tone, repeat=(len(ls), (0, 2 * BH, 0)) if len(ls) > 1 else None))
                else:
                    corner = CHIPS[(L0, key)]
                    out.append(wextrude(f"{name}-l{L0 + 1:02d}-chipped", "stack", chip_profile(x0, x1, y0, y0 + BH, corner, kind), z0, z0 + CHIP_DEPTH, CLAY, tone))
                    out.append(wbox(f"{name}-l{L0 + 1:02d}-core", "stack", x0, x1, y0, y0 + BH, z0 + CHIP_DEPTH, z1, CLAY, tone))
    return out


def layer_body_50(bone, layer):
    """🧱️ A set-down B layer as its eight rows, each one brick repeated three times along z."""
    out = []
    for key, x0, x1, z0, z1 in bricks(layer):
        if key[1] == 0:
            out.append(wbox(f"{bone}-row-{key[0] + 1}", bone, x0, x1, layer_y(layer), layer_y(layer) + BH, z0, z1, CLAY, tone_of(layer, key), repeat=(3, (0, 0, PITCH_L))))
    return out


def course_body(k, scale):
    """🪜️ One wall course: four stretchers with their 10 mm joints at 1:50, one row of 1030 x 65 x 120 mm at 1:100 and coarser."""
    c = course_rows()[k]
    cid = COURSE_IDS[k]
    y0 = layer_y(c["layer"])
    if scale == 50:
        return wbox(f"{cid}-bricks", cid, c["x0"], c["x0"] + BL, y0, y0 + BH, c["zc"] - BW / 2, c["zc"] + BW / 2, CLAY, tone_of("c", k), repeat=(4, (PITCH_L, 0, 0)))
    return wbox(f"{cid}-row", cid, c["x0"], c["x0"] + ROW_L, y0, y0 + (COURSE if scale == 500 else BH), c["zc"] - BW / 2, c["zc"] + BW / 2, CLAY, tone_of("c", k, scale))


def stack_block(scale):
    """🧊️ The twelve stack layers as slabs of their own extent at 1:100 (the bricks of a layer merged), as one block of the stack's envelope at 1:200 and 1:500; the two set-down layers as slabs."""
    out = []
    if scale == 100:
        for L in range(12):
            x0, x1, z0, z1 = extent(kind_of(L))
            out.append(wbox(f"stack-layer-{L + 1:02d}", "stack", x0, x1, layer_y(L), layer_y(L) + BH, z0, z1, CLAY, tone_of(L, "slab")))
    else:
        out.append(wbox("stack-block", "stack", SX0, SX1, SY0, layer_y(12), SZ0, SZ1, CLAY, 0.44))
    bx0, bx1, bz0, bz1 = extent("B")
    if scale == 500:
        out += [wbox(f"layer-{L + 1}-band", f"layer-{L + 1}", SX0, SX1, layer_y(L - 1), layer_y(L) + BH, SZ0, SZ1, CLAY, tone_of(L)) for L in (13, 15)]
    else:
        out += [wbox("layer-14-slab", "layer-14", bx0, bx1, layer_y(13), layer_y(13) + BH, bz0, bz1, CLAY, tone_of(13)),
                wbox("layer-16-slab", "layer-16", bx0, bx1, layer_y(15), layer_y(15) + BH, bz0, bz1, CLAY, tone_of(15))]
    return out


def boxes(items, bone, material, tone, **kw):
    """📦️ Box solids from world box tuples."""
    return [wbox(i[0], bone, *i[1:], material, tone, **kw) for i in items]


def patch_boxes(scale):
    """🩶️ The mortar remnants as texture boxes on their bones."""
    return [wbox(p[0], p[1], *p[3:], MORTAR, TONE_MORTAR, role="texture", texture=True) for p in patches(scale)]


def coarse_mortar(scale):
    """🩶️ The mortar remnants of a coarse scale as four thermal crust patches of their 3 mm layer on the right and front faces of the stack (the remnants are below the smallest feature there)."""
    t = 0.003
    items = [("mortar-crust-side-1", SX0 + 0.30, SX0 + 0.62, SY0 + 0.10, SY0 + 0.16, SZ0 - t, SZ0), ("mortar-crust-side-2", SX0 + 0.55, SX0 + 0.90, SY0 + 0.55, SY0 + 0.61, SZ0 - t, SZ0),
             ("mortar-crust-front-1", SX1, SX1 + t, SY0 + 0.20, SY0 + 0.26, -0.20, 0.10), ("mortar-crust-front-2", SX1, SX1 + t, SY0 + 0.62, SY0 + 0.68, -0.05, 0.25)]
    return [dict(wbox(i[0], "stack", *i[1:], MORTAR, TONE_MORTAR, role="texture"), region=R_MORTAR, layer="lime-crust") for i in items]


def volume(scale):
    """🧊️ The handcrafted 3D model of one preset scale: body, thermal and structure lists."""
    body = pallet_body(scale)
    if scale == 50:
        body += stack_body_50() + layer_body_50("layer-14", 13) + layer_body_50("layer-16", 15)
    else:
        body += stack_block(scale)
    body += [course_body(k, scale) for k in range(12)]
    if scale <= 200:
        body += boxes(corner_plates(), "corners", PP, TONE_CORNER, sheet=True)
    if scale <= 100:
        body += boxes(strap_sheets(), "straps", PET, TONE_STRAP, sheet=True) + patch_boxes(scale)
    elif scale == 200:
        body += boxes(strap_sheets(), "straps", PET, TONE_STRAP, role="texture", texture=True)
    region_of = {TIMBER: R_PALLET, CLAY: R_BRICK, PP: R_CORNER, PET: R_STRAP, MORTAR: R_MORTAR}
    layer_of = {R_CORNER: "pp-angle", R_STRAP: "pet-band", R_MORTAR: "lime-crust"}
    thermal = []
    for s in body:
        reg = region_of[s["material"]]
        extra = {"layer": layer_of[reg]} if reg in layer_of else {}
        thermal.append(ref(f"th-{s['id']}", s["id"], region=reg, **extra))
    if scale >= 200:
        thermal += coarse_mortar(scale)
    if scale == 500:
        thermal += [dict(wbox(i[0], "straps", *i[1:], PET, TONE_STRAP), region=R_STRAP, layer="pet-band") for i in strap_sheets()]
        thermal += [dict(wbox(i[0], "corners", *i[1:], PP, TONE_CORNER), region=R_CORNER, layer="pp-angle") for i in corner_plates()]
    structure = []
    if scale == 50:
        structure += [ref(f"st-{s['id']}", s["id"], member="pallet", part="member") for s in body if s["bone"] == "pallet"]
    elif scale == 100:
        structure.append(dict(wbox("st-deck-plate", "pallet", 0, 1.2, *Y_DECK, -0.4, 0.4, TIMBER, TONE_PALLET), member="pallet", part="member"))
        structure.append(dict(wbox("st-stringer-boards", "pallet", *BLOCK_X[0], *Y_STRINGER, -0.4, 0.4, TIMBER, TONE_PALLET, repeat=(3, (0.5275, 0, 0))), member="pallet", part="member"))
        for n, (z0, z1) in enumerate(BOTTOM):
            structure.append(dict(wbox(f"st-blocks-{n + 1}", "pallet", *BLOCK_X[0], *Y_BLOCK, z0, z1, TIMBER, TONE_PALLET, repeat=(3, (0.5275, 0, 0))), member="pallet", part="joint"))
            structure.append(dict(wbox(f"st-bottom-board-{n + 1}", "pallet", 0, 1.2, *Y_BOTTOM, z0, z1, TIMBER, TONE_PALLET), member="pallet", part="member"))
    elif scale == 200:
        structure.append(dict(wbox("st-deck-slab", "pallet", 0, 1.2, 0.100, 0.144, -0.4, 0.4, TIMBER, TONE_PALLET), member="pallet", part="member"))
        for n, (z0, z1) in enumerate(BOTTOM):
            structure.append(dict(wbox(f"st-blocks-{n + 1}", "pallet", *BLOCK_X[0], 0.030, 0.100, z0, z1, TIMBER, TONE_PALLET, repeat=(3, (0.5275, 0, 0))), member="pallet", part="joint"))
            structure.append(dict(wbox(f"st-bottom-board-{n + 1}", "pallet", 0, 1.2, 0, 0.030, z0, z1, TIMBER, TONE_PALLET), member="pallet", part="member"))
    else:
        structure.append(dict(wbox("st-pallet", "pallet", 0, 1.2, 0, 0.144, -0.4, 0.4, TIMBER, TONE_PALLET), member="pallet", part="member"))
    structure.append(dict(wbox("st-stack", "stack", 0.12, 1.08, SY0, layer_y(12), -0.36, 0.36, CLAY, 0.44), member="stack", part="member"))
    for name, L in (("layer-14", 13), ("layer-16", 15)):
        y0 = layer_y(L - 1) if scale == 500 else layer_y(L)
        structure.append(dict(wbox(f"st-{name}", name, *extent("B")[:2], y0, layer_y(L) + BH, *extent("B")[2:], CLAY, 0.44), member=name, part="member"))
    for k in range(12):
        c = course_rows()[k]
        structure.append(dict(wbox(f"st-{COURSE_IDS[k]}", COURSE_IDS[k], c["x0"], c["x0"] + ROW_L, layer_y(c["layer"]), layer_y(c["layer"]) + (COURSE if scale == 500 else BH), c["zc"] - BW / 2, c["zc"] + BW / 2, CLAY, 0.44), member=COURSE_IDS[k], part="member"))
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[scale]}


def areas():
    """📐️ The outer areas of the regions on the 1:50 model (hand-derived from the boxes)."""
    def region_area(items):
        total = sum(box_area(*i[1:]) for i in items)
        shared = sum(contact_area(items[a], items[b]) for a in range(len(items)) for b in range(a + 1, len(items)))
        return total - 2 * shared
    return {R_BRICK: brick_area()[0], R_PALLET: pallet_area(), R_CORNER: region_area(corner_plates()), R_STRAP: region_area(strap_sheets()), R_MORTAR: sum(box_area(*p[3:]) for p in patches(50))}


AREA = areas()
BRICK_HC = round(NL * 24 * BRICK_KG * 1000 / AREA[R_BRICK])
PALLET_HC = round(MASS["pallet"] * 1600 / AREA[R_PALLET])
PLATE_LOAD_N = 3000.0 * ROW_L
WIND_N = 600.0 * (ROW_L + 0.13) * (11 * COURSE + BH)


def thermal_model():
    """🌡️ Five passive regions: the brick surface the air reaches in the open joints (an exchange area), the mortar remnants, the pallet timber, the PET straps and the polypropylene corner strips."""
    lit, est = "literature", "authored-estimate"
    return {
        "environment": {"airTempC": 15, "meanRadiantTempC": 15, "skyTempC": 2, "groundTempC": 12, "relativeHumidity": 0.65, "windMS": 2, "windDirectionDeg": 90, "solarWM2": 600, "diffuseFraction": 0.25, "pressurePa": 101325},
        "passengers": [], "operating": [],
        "regions": [
            {"id": R_BRICK, "title": T("Brick faces (outer faces and the faces in the open 10 mm joints)", "Ziegelflächen (Außenflächen und Flächen in den offenen 10-mm-Fugen)"), "kind": "surface", "model": "passive", "material": CLAY, "areaM2": rnd(AREA[R_BRICK]), "areaBasis": "exchange", "characteristicLengthM": rnd(STACK_TOP - SY0), "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": BRICK_HC, "source": est, "reference": "en-1745"},
            {"id": R_MORTAR, "title": T("Lime mortar remnants on the brick faces", "Kalkmörtelreste auf den Ziegelflächen"), "kind": "surface", "model": "passive", "material": MORTAR, "areaM2": rnd(AREA[R_MORTAR]), "characteristicLengthM": 0.06, "geometry": "vertical-plate", "skyViewFactor": 0.5,
             "layers": [{"id": "lime-crust", "title": T("Lime mortar crust 3 mm", "Kalkmörtelkruste 3 mm"), "kind": "masonry", "material": MORTAR, "thicknessM": 0.003, "resistanceM2KW": rnd(0.003 / 0.87), "heatCapacityJM2K": 5100, "source": lit, "reference": "en-iso-10456"}], "heatCapacityJM2K": 5100, "source": est, "reference": "en-iso-10456"},
            {"id": R_PALLET, "title": T("Pallet timber, weathered", "Palettenholz, vergraut"), "kind": "surface", "model": "passive", "material": TIMBER, "areaM2": rnd(AREA[R_PALLET]), "characteristicLengthM": 0.145, "geometry": "horizontal-plate-up", "skyViewFactor": 0.3, "layers": [], "heatCapacityJM2K": PALLET_HC, "source": est, "reference": "en-iso-10456"},
            {"id": R_STRAP, "title": T("PET girth straps", "PET-Umreifungsbänder"), "kind": "surface", "model": "passive", "material": PET, "areaM2": rnd(AREA[R_STRAP]), "characteristicLengthM": PET_W, "geometry": "vertical-plate", "skyViewFactor": 0.5,
             "layers": [{"id": "pet-band", "title": T("PET band 0.9 mm", "PET-Band 0,9 mm"), "kind": "polymer", "material": PET, "thicknessM": PET_T, "resistanceM2KW": rnd(PET_T / 0.24), "heatCapacityJM2K": round(1380 * 1200 * PET_T, 1), "source": est, "reference": "en-iso-10456"}], "heatCapacityJM2K": round(1380 * 1200 * PET_T, 1), "source": est, "reference": "en-iso-10456"},
            {"id": R_CORNER, "title": T("Polypropylene corner strips", "Kantenschutzwinkel aus Polypropylen"), "kind": "surface", "model": "passive", "material": PP, "areaM2": rnd(AREA[R_CORNER]), "characteristicLengthM": PP_LEG, "geometry": "vertical-plate", "skyViewFactor": 0.5,
             "layers": [{"id": "pp-angle", "title": T("PP angle 3 mm", "PP-Winkel 3 mm"), "kind": "polymer", "material": PP, "thicknessM": PP_T, "resistanceM2KW": rnd(PP_T / 0.22), "heatCapacityJM2K": round(905 * 1700 * PP_T, 1), "source": est, "reference": "en-iso-10456"}], "heatCapacityJM2K": round(905 * 1700 * PP_T, 1), "source": est, "reference": "en-iso-10456"},
        ],
        "emitters": [],
        "assumptions": [
            T(f"The bricks lie side by side with open 10 mm head joints, so the air reaches every face except the faces two bricks share: {AREA[R_BRICK]:.2f} m² for the 384 bricks ({384 * 0.1081:.2f} m² of brick faces less 2 x {brick_area()[1]:.2f} m² of shared beds and sides). That is an exchange area the envelope of the stack (about 4.9 m²) does not show, so it is declared as such; the faces in the head joints exchange mostly by convection through the pockets, which the exposed fraction of 0.35 of the poses expresses.", f"Die Ziegel liegen Seite an Seite mit offenen 10-mm-Stoßfugen; die Luft erreicht daher jede Fläche außer den Flächen, die zwei Ziegel teilen: {AREA[R_BRICK]:.2f} m² für die 384 Ziegel ({384 * 0.1081:.2f} m² Ziegelflächen abzüglich 2 x {brick_area()[1]:.2f} m² geteilter Lager- und Seitenflächen). Das ist eine Austauschfläche, die die Hülle des Stapels (etwa 4,9 m²) nicht zeigt, und ist so deklariert; die Flächen in den Stoßfugen tauschen vorwiegend konvektiv durch die Taschen aus, was der freie Anteil 0,35 der Posen ausdrückt."),
            T(f"The whole brick takes part in the daily heat storage (65 to 120 mm thick against a 24 h penetration depth of about 110 mm for 0.81 W/mK, 1800 kg/m³, 1000 J/kgK): {BRICK_HC} J/m²K = 384 x 3.51 kg x 1000 J/kgK over the exchange area. The weathered red-brown faces absorb 0.68 of the sun and emit 0.93 (ASHRAE, weathered common brick), more than new bricks of a lighter fired colour.", f"Der ganze Ziegel nimmt an der täglichen Wärmespeicherung teil (65 bis 120 mm dick gegen eine 24-h-Eindringtiefe von etwa 110 mm bei 0,81 W/mK, 1800 kg/m³, 1000 J/kgK): {BRICK_HC} J/m²K = 384 x 3,51 kg x 1000 J/kgK über der Austauschfläche. Die bewitterten rotbraunen Flächen absorbieren 0,68 der Sonne und emittieren 0,93 (ASHRAE, bewitterter Mauerziegel), mehr als neue, heller gebrannte Ziegel."),
            T("Mortar remnants are modelled where they show on the outer faces (a 3 mm lime crust, lighter than the brick: solar absorptance 0.5); the remnants on hidden faces are part of the 0.10 kg per brick in the mass and not of this region.", "Mörtelreste sind dort modelliert, wo sie auf den Außenflächen sichtbar sind (eine 3-mm-Kalkkruste, heller als der Ziegel: solarer Absorptionsgrad 0,5); die Reste auf verdeckten Flächen gehören zu den 0,10 kg je Ziegel der Masse und nicht zu dieser Region."),
            T(f"The pallet timber is grey weathered softwood (solar absorptance 0.62, emissivity 0.9); its {AREA[R_PALLET]:.2f} m² are every face of the 22 boards and blocks less the faces they share, including the bottom faces on the ground (contact fraction 0.13 in the stored pose).", f"Das Palettenholz ist grau bewittertes Nadelholz (solarer Absorptionsgrad 0,62, Emissionsgrad 0,9); seine {AREA[R_PALLET]:.2f} m² sind alle Flächen der 22 Bretter und Klötze abzüglich der geteilten Flächen, einschließlich der Unterseiten auf dem Boden (Kontaktanteil 0,13 in der Lagerpose)."),
            T("Reference case: a sunny spring day in an open storage yard, air 15 °C, wind 2 m/s from the left; no element of the lot draws power (passive, 0 W).", "Referenzfall: ein sonniger Frühlingstag auf einem offenen Lagerplatz, Luft 15 °C, Wind 2 m/s von links; kein Element der Partie nimmt Leistung auf (passiv, 0 W)."),
        ],
    }


def binding(state):
    """🌬️ The region binding of a pose: stored or lifted lot, or the reinstalled panel beside the remaining stack."""
    ground = {"fraction": 0.13, "tempC": 12} if state != "lifted" else None
    pallet = {"exposedFraction": 0.9 if state == "lifted" else 0.85, "sunlitFraction": 0.04}
    if ground:
        pallet["contact"] = ground
    return {"regions": {R_BRICK: {"exposedFraction": 0.45 if state.startswith("reinstalled") else 0.35, "sunlitFraction": 0.18 if state.startswith("reinstalled") else 0.12},
                        R_MORTAR: {"exposedFraction": 1, "sunlitFraction": 0.3}, R_PALLET: pallet,
                        R_STRAP: {"exposedFraction": 1, "sunlitFraction": 0.4}, R_CORNER: {"exposedFraction": 0.9, "sunlitFraction": 0.35}}, "emitters": {}}


RUNNERS = ["runner-right", "runner-centre", "runner-left"]
FORKS = ["fork-right", "fork-left"]


def poses():
    """🧍️ Stored on the yard (default), lifted on the forks, the panel reinstalled under its plate load, and the panel in a wind gust."""
    chain = course_chain()
    set_down = {"layer-14": {"dx": rnd(-BH)}, "layer-16": {"dx": rnd(-2 * BH)}}
    for c in chain:
        set_down[COURSE_IDS[c["k"]]] = {"dx": c["dx"], "dy": c["dy"]}
    plate = {"id": "plate-load", "kind": "imposed", "title": T("Timber wall plate and roof of a garden shed, 3 kN/m over the top course", "Holzschwelle und Dach eines Gartenhauses, 3 kN/m über der obersten Schar"), "anchor": "wall-top", "fxN": 0, "fyN": rnd(-PLATE_LOAD_N), "source": "authored-estimate", "reference": "en-1996-1-1"}
    wind = {"id": "wind-pressure", "kind": "wind", "title": T("Wind on the free-standing panel, 0.6 kN/m² net", "Wind auf die frei stehende Wand, 0,6 kN/m² netto"), "anchor": "wall-face", "fxN": 0, "fyN": 0, "fzN": rnd(WIND_N), "source": "authored-estimate", "reference": "en-1991-1-4"}
    return [
        {"id": "stored", "title": T("Stored on the yard", "Auf dem Lagerplatz gelagert"), "description": T("The pallet stands on its three bottom boards on the yard of the component exchange, the 384 cleaned bricks in sixteen interlocked layers, girthed by two PET straps over four corner strips.", "Die Palette steht mit ihren drei Bodenbrettern auf dem Lagerplatz der Bauteilbörse, die 384 gereinigten Ziegel in sechzehn verzahnten Lagen, mit zwei PET-Bändern über vier Kantenschutzwinkeln umreift."), "bones": {}, "ground": "authored", "contacts": RUNNERS, "thermal": binding("stored"), "loads": []},
        {"id": "lifted", "title": T("Lifted on the forks", "Auf der Gabel angehoben"), "description": T(f"A forklift has entered the short side and lifted the loaded pallet {LIFT_Y * 1000:.0f} mm on two fork arms 100 mm wide (ISO 2328) under the three stringer boards; the 1.41 t rest on the forks only.", f"Ein Gabelstapler ist von der Schmalseite eingefahren und hat die beladene Palette auf zwei 100 mm breiten Gabelzinken (ISO 2328) unter den drei Querbrettern um {LIFT_Y * 1000:.0f} mm angehoben; die 1,41 t ruhen nur auf der Gabel."), "bones": {}, "root": {"y": LIFT_Y}, "ground": "authored", "contacts": FORKS, "thermal": binding("lifted"), "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled as a wall panel", "Als Wandscheibe wieder eingebaut"), "description": T("Layers 13 and 15 are relaid beside the pallet as a 120 mm stretcher-bond panel of twelve courses (1.16 x 0.91 m, toothed ends, 12 mm bed and 10 mm head joints for the new lime mortar) carrying a timber wall plate; layers 14 and 16 are set down on the stack.", "Die Lagen 13 und 15 sind neben der Palette als 120 mm dicke Läuferverbandwand aus zwölf Scharen neu vermauert (1,16 x 0,91 m, verzahnte Enden, 12-mm-Lager- und 10-mm-Stoßfugen für den neuen Kalkmörtel) und tragen eine Holzschwelle; die Lagen 14 und 16 sind auf den Stapel abgesetzt."), "bones": set_down, "ground": "authored", "contacts": RUNNERS + ["wall-footing"], "thermal": binding("reinstalled"), "loads": [plate]},
        {"id": "reinstalled-wind", "title": T("Reinstalled panel in a wind gust", "Wiedereingebaute Wand in einer Windböe"), "description": T(f"The reinstalled panel under its plate load and a net wind pressure of 0.6 kN/m² on its face ({WIND_N / 1000:.2f} kN at mid-height).", f"Die wiedereingebaute Wand unter ihrer Schwellenlast und einem Netto-Winddruck von 0,6 kN/m² auf ihrer Fläche ({WIND_N / 1000:.2f} kN in halber Höhe)."), "bones": set_down, "ground": "authored", "contacts": RUNNERS + ["wall-footing"], "thermal": binding("reinstalled-wind"), "loads": [plate, wind]},
    ]


def clips():
    """🎞️ The forklift lift (stored to lifted) and a gusting wind on the reinstalled panel."""
    lift = {"id": "lift", "title": T("Lift on the forks", "Anheben mit der Gabel"), "description": T("The fork arms already sit under the stringer boards; the lift raises the loaded pallet smoothly from the yard to 250 mm and holds it.", "Die Gabelzinken liegen bereits unter den Querbrettern; der Hubmast hebt die beladene Palette weich vom Lagerplatz auf 250 mm und hält sie."), "durationS": 6, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1, "pose": "stored", "ease": "smooth"}, {"t": 5, "pose": "lifted", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 1.2} for a in RUNNERS] + [{"anchor": a, "fromS": 0, "toS": 6} for a in FORKS],
            "phases": [{"id": "engage", "fromS": 0, "toS": 1, "title": T("Forks engaged", "Gabel untergefahren")}, {"id": "raise", "fromS": 1, "toS": 5, "title": T("Raise", "Anheben")}, {"id": "hold", "fromS": 5, "toS": 6, "title": T("Hold", "Halten")}],
            "events": [{"t": 1.2, "kind": "contact-off", "title": T("The bottom boards leave the yard", "Die Bodenbretter lösen sich vom Lagerplatz")}, {"t": 5, "kind": "motion", "title": T("Transport height reached", "Transporthöhe erreicht")}], "tracks": []}
    gust = {"id": "wind-gust", "title": T("Wind gusts on the reinstalled panel", "Windböen auf der wiedereingebauten Wand"), "description": T("The wind pressure on the reinstalled panel rises and falls in two gusts between 30 % and 100 % of its 0.6 kN/m², while the plate load stays.", "Der Winddruck auf die wiedereingebaute Wand steigt und fällt in zwei Böen zwischen 30 % und 100 % seiner 0,6 kN/m², die Schwellenlast bleibt."), "durationS": 8, "fps": 25, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "reinstalled-wind", "ease": "smooth"}, {"t": 8, "pose": "reinstalled-wind", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in RUNNERS + ["wall-footing"]],
            "phases": [{"id": "gusts", "fromS": 0, "toS": 8, "title": T("Two gusts", "Zwei Böen")}],
            "events": [{"t": 2, "kind": "load-on", "title": T("First gust peak", "Erste Böenspitze")}, {"t": 5.5, "kind": "load-on", "title": T("Second gust peak", "Zweite Böenspitze")}],
            "tracks": [{"target": "load:wind-pressure", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": 2, "v": 1.0}, {"t": 3.5, "v": 0.45}, {"t": 5.5, "v": 0.85}, {"t": 8, "v": 0.3}]}]}
    return [lift, gust]


def structure_model():
    """🏗️ No permanent supports: every pose names its contacts (runners on the yard, fork arms, the footing of the panel)."""
    return {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        T(f"Self-weight from section x density: 384 bricks of 3.51 kg (250 x 120 x 65 mm = 1.95 dm³ at 1800 kg/m³) with 0.10 kg of mortar remnant each, the pallet of 0.0452 m³ softwood at 550 kg/m³ ({MASS['pallet']:.1f} kg, EPAL nominal about 25 kg), straps and corner strips: {MASS_TOTAL:.1f} kg, below the 1500 kg nominal load of the EPAL pallet.", f"Eigengewicht aus Querschnitt x Dichte: 384 Ziegel zu 3,51 kg (250 x 120 x 65 mm = 1,95 dm³ bei 1800 kg/m³) mit je 0,10 kg Mörtelrest, die Palette aus 0,0452 m³ Nadelholz zu 550 kg/m³ ({MASS['pallet']:.1f} kg, EPAL-Nennwert etwa 25 kg), Bänder und Kantenschutz: {MASS_TOTAL:.1f} kg, unter der Nennlast 1500 kg der EPAL-Palette."),
        T("Stored: the lot stands on the three bottom boards (line contacts 1.2 m long, friction 0.5). Lifted: it hangs on the two fork arms under the stringer boards (1.15 m contact length, friction 0.3). The dry stack is a column in compression only; the pallet's top deck carries the layers between the blocks.", "Gelagert: die Partie steht auf den drei Bodenbrettern (Linienkontakte 1,2 m lang, Reibung 0,5). Angehoben: sie hängt an den zwei Gabelzinken unter den Querbrettern (1,15 m Kontaktlänge, Reibung 0,3). Der Trockenstapel ist eine nur gedrückte Säule; die Deckbretter tragen die Lagen zwischen den Klötzen."),
        T(f"Reinstalled: the twelve courses form a chain from the top course to the footing, each course a member of the 120 mm wall section (4 x 250 x 120 mm net), joined at the centre line of the panel, so the plate load ({PLATE_LOAD_N / 1000:.2f} kN as the resultant of 3 kN/m) and the wind ({WIND_N / 1000:.2f} kN, 0.6 kN/m² = cp,net 1.2 x qp 0.5 kN/m²) run down the courses to the bed on the footing.", f"Wiedereingebaut: die zwölf Scharen bilden eine Kette von der obersten Schar zum Fundament, jede Schar ein Stab des 120-mm-Wandquerschnitts (4 x 250 x 120 mm netto), verbunden in der Mittellinie der Wand; die Schwellenlast ({PLATE_LOAD_N / 1000:.2f} kN als Resultierende von 3 kN/m) und der Wind ({WIND_N / 1000:.2f} kN, 0,6 kN/m² = cp,net 1,2 x qp 0,5 kN/m²) laufen über die Scharen zur Lagerfuge auf dem Fundament."),
        T("The solver treats the whole asset as one rigid body on its active contacts: in the reinstalled poses the vertical reactions separate exactly into pallet and panel (the two groups of contacts stand apart in plan), the horizontal wind reaction is shared between the footing and the pallet runners in proportion to their stiffness, an idealisation of two bodies that the model cannot separate.", "Der Löser behandelt das ganze Asset als einen starren Körper auf seinen aktiven Kontakten: in den wiedereingebauten Posen trennen sich die senkrechten Reaktionen genau in Palette und Wand (die beiden Kontaktgruppen liegen im Grundriss getrennt), die waagrechte Windreaktion wird zwischen Fundament und Palettenkufen nach ihrer Steifigkeit geteilt, eine Idealisierung zweier Körper, die das Modell nicht trennen kann."),
    ]}


REFERENCES = [
    {"id": "reichsformat", "citation": "Reichsformat 250 x 120 x 65 mm: the solid brick format fixed in Prussia in 1872 and used throughout the German Empire until the Normalformat 240 x 115 x 71 mm of DIN 4172 (1951, Maßordnung im Hochbau); bonding module 260 x 130 x 77 mm with 10 mm head and 12 mm bed joints (two widths and one joint make one length)."},
    {"id": "en-771-1", "citation": "EN 771-1:2011+A1:2015 Specification for masonry units, Part 1: Clay masonry units (declared dimensions, gross dry density and mean compressive strength of solid units; the frame for re-declaring reclaimed units)."},
    {"id": "en-1745", "citation": "EN 1745:2020 Masonry and masonry products, methods for determining thermal properties, Annex A (clay units of gross dry density 1800 kg/m³) and DIN 4108-4:2020 Table A.1 (solid clay brick masonry 1800 kg/m³: 0.81 W/mK)."},
    {"id": "din-4108-4", "citation": "DIN 4108-4:2020-11 Thermal insulation and energy economy in buildings, Part 4: Hygrothermal design values, Table A.1: solid clay bricks of density class 1.8, design conductivity 0.81 W/mK."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties, Table 3 (softwood 500 kg/m³ 0.13 W/mK 1600 J/kgK; lime mortar 1600 to 1800 kg/m³ about 0.8 to 0.9 W/mK 1000 J/kgK; polypropylene 910 kg/m³ 0.22 W/mK 1800 J/kgK; PET (polyester) about 0.24 W/mK)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), Chapter 4, radiation properties of building surfaces: common red brick emissivity about 0.93, solar absorptance 0.63 to 0.70; weathered grey wood about 0.6; lime plaster about 0.4 to 0.5 (rounded typical values)."},
    {"id": "en-13698-1", "citation": "EN 13698-1:2003 Pallet production specification, Part 1: Construction specification for 800 mm x 1200 mm flat wooden pallets (EPAL 1): top deck 2 x 145, 2 x 100 and 1 x 145 mm boards 22 mm thick, three 145 x 22 mm stringer boards, six blocks 145 x 100 x 78 mm and three 145 x 145 x 78 mm, bottom boards 2 x 100 and 1 x 145 mm, overall height 144 mm; EPAL data sheet: nominal load 1500 kg evenly distributed, mass about 25 kg."},
    {"id": "iso-2328", "citation": "ISO 2328:2011 Fork-lift trucks, hook-on type fork arms and fork arm carriages, mounting dimensions (class 2 fork arms 100 mm wide, 40 mm thick, 1150 mm long)."},
    {"id": "en-1996-1-1", "citation": "EN 1996-1-1:2005+A1:2012 Eurocode 6, design of masonry structures, Part 1-1: characteristic compressive strength of masonry from unit and mortar strength (reused solid units re-tested as Category II units), sections 3.6 and 6.1."},
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, actions on structures, Part 1-4: wind actions, section 7.4 (free-standing walls, net pressure coefficients cp,net) with the German national annex peak velocity pressure about 0.5 kN/m² near the ground in wind zone 1 inland."},
    {"id": "atlas-recycling", "citation": "Hillebrandt A., Riegler-Floors P., Rosen A., Seggewies J.-K. (2018): Atlas Recycling, Gebäude als Materialressource. Detail, Munich: reclaimed solid bricks are cleaned of lime mortar by hand, sorted and offered on pallets; lime-mortared historic masonry yields the highest share of reusable units."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, Part 50: basic conventions for representing areas on cuts and sections."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, Part 1: types, contents and basic principles of presentation (scales, line groups, presentation of masonry bonds)."},
]


def rep_decisions(scale):
    """📝️ The eight drawing decisions of one preset scale."""
    if scale == 50:
        return {
            "silhouette": T("Side elevation: the EPAL pallet (1200 x 144 mm, bottom board, three block columns with the stringer ends, top deck board) and on it the sixteen layers of the stack brick by brick: the eight A layers show the long faces of four stretchers with their open 10 mm head joints (the darker brick of the next row behind each joint), the eight B layers the ends of eight bricks laid side by side, and the corner strips and the two girth straps over them. Front elevation: the short side of the pallet with its three bottom boards, three blocks, the full stringer board and five deck boards, the stack with six row ends per A layer and three stretchers per B layer. Plan: the top layer brick by brick over the five deck boards.", "Seitenansicht: die EPAL-Palette (1200 x 144 mm, Bodenbrett, drei Klotzsäulen mit den Querbrettenden, Deckbrett) und darauf die sechzehn Lagen des Stapels Stein für Stein: die acht A-Lagen zeigen die Längsseiten von vier Läufern mit ihren offenen 10-mm-Stoßfugen (dahinter dunkler der Ziegel der nächsten Reihe), die acht B-Lagen die Köpfe von acht nebeneinanderliegenden Ziegeln, darüber die Kantenschutzwinkel und die zwei Umreifungsbänder. Vorderansicht: die Schmalseite der Palette mit drei Bodenbrettern, drei Klötzen, dem durchgehenden Querbrett und fünf Deckbrettern, der Stapel mit sechs Reihenköpfen je A-Lage und drei Läufern je B-Lage. Draufsicht: die oberste Lage Stein für Stein über den fünf Deckbrettern."),
            "retained": T("Every brick of the visible faces with its own tone, the eleven chipped arrises of the right face at their bricks, 20 lime mortar remnants on the faces where they sit, the 41 mm gaps between the deck boards, the 100 mm fork openings of the long side and the 227 mm openings of the short side.", "Jeder Ziegel der sichtbaren Flächen mit eigenem Ton, die elf abgeschlagenen Kanten der rechten Seite an ihren Ziegeln, 20 Kalkmörtelreste auf den Flächen, wo sie sitzen, die 41-mm-Spalte zwischen den Deckbrettern, die 100-mm-Gabelöffnungen der Längsseite und die 227-mm-Öffnungen der Schmalseite."),
            "merged": T("Bricks behind the visible faces are drawn as one darker band per layer that shows in the joints; in plan the twelve stack layers and layer 14 are one footprint under the top layer.", "Ziegel hinter den sichtbaren Flächen sind als ein dunkleres Band je Lage gezeichnet, das in den Fugen erscheint; in der Draufsicht sind die zwölf Stapellagen und die Lage 14 eine Grundfläche unter der obersten Lage."),
            "omitted": T("Nails of the pallet (2.8 mm heads), the EPAL and IPPC stamps on the blocks, the strap seals and the hair cracks of the bricks: below 0.1 mm on paper or only lettering.", "Nägel der Palette (2,8-mm-Köpfe), die EPAL- und IPPC-Stempel auf den Klötzen, die Bandverschlüsse und die Haarrisse der Ziegel: unter 0,1 mm auf Papier oder nur Beschriftung."),
            "thinElements": T("The 0.9 mm straps are 19 mm bands in the elevations and their centre line as an outline in plan; the 3 mm corner strips are drawn by their 65 mm legs.", "Die 0,9-mm-Bänder sind in den Ansichten 19-mm-Bänder und in der Draufsicht ihre Mittellinie als Kontur; die 3-mm-Kantenschutzwinkel sind über ihre 65-mm-Schenkel gezeichnet."),
            "hatching": T("No hatch: the bricks read by their joints and tones, the timber by its board seams.", "Keine Schraffur: die Ziegel lesen sich über Fugen und Töne, das Holz über seine Brettfugen."),
            "styles": T("Pallet outlines 0.25 mm, bricks 0.13 mm over the dark joint band, so each 0.2 mm joint prints as one dark line, straps and corner strips 0.25 mm, mortar remnants as fine texture outlines.", "Palettenkonturen 0,25 mm, Ziegel 0,13 mm über dem dunklen Fugenband, so druckt jede 0,2-mm-Fuge als eine dunkle Linie, Bänder und Kantenschutz 0,25 mm, Mörtelreste als feine Texturkonturen."),
            "overlays": T("Each brick is its own cell on the bone that carries it (the stack, the two set-down layers, the twelve courses), so the reinstalled panel is drawn from the same bricks.", "Jeder Ziegel ist eine eigene Zelle am tragenden Knochen (Stapel, die zwei abgesetzten Lagen, die zwölf Scharen), so wird die wiedereingebaute Wand aus denselben Ziegeln gezeichnet."),
        }
    if scale == 100:
        return {
            "silhouette": T("The pallet board by board as at 1:50 with blocks and stringer ends as one column per side; every layer as one slab of its own outline (A layers 1040 mm, B layers 960 mm long, so the stack keeps its alternating hand-stacked edge), the twelve courses as rows, on the 1200 x 144 mm pallet.", "Die Palette Brett für Brett wie 1:50, Klötze und Querbrettenden als eine Säule je Seite; jede Lage als eine Platte mit eigener Kontur (A-Lagen 1040 mm, B-Lagen 960 mm lang, so behält der Stapel seine wechselnde, von Hand gestapelte Kante), die zwölf Scharen als Reihen, auf der 1200 x 144 mm großen Palette."),
            "retained": T("Every head joint and row end of the visible faces as a texture line (0.65 mm layer pitch), the row joints of the top layer in plan, the straps, the corner strips and the mortar remnants.", "Jede Stoßfuge und jedes Reihenende der sichtbaren Flächen als Texturlinie (0,65 mm Lagenabstand), die Reihenfugen der obersten Lage in der Draufsicht, die Bänder, die Kantenschutzwinkel und die Mörtelreste."),
            "merged": T("The bricks of a layer are merged into its slab (the 0.1 mm joints become lines); the bricks of a course into one row.", "Die Ziegel einer Lage sind zu ihrer Platte zusammengefasst (die 0,1-mm-Fugen werden Linien); die Ziegel einer Schar zu einer Reihe."),
            "omitted": T("The chipped arrises (30 x 18 mm, 0.3 mm on paper) and the tones of single bricks.", "Die abgeschlagenen Kanten (30 x 18 mm, 0,3 mm auf Papier) und die Töne einzelner Ziegel."),
            "thinElements": T("Straps as 19 mm bands, corner strips by their 65 mm legs, the strap outline in plan.", "Bänder als 19-mm-Bänder, Kantenschutz über seine 65-mm-Schenkel, die Bandkontur in der Draufsicht."),
            "hatching": T("No hatch; joint lines 0.1 mm.", "Keine Schraffur; Fugenlinien 0,1 mm."),
            "styles": T("Outlines 0.25 mm, brick rows 0.13 mm, joints 0.1 mm.", "Konturen 0,25 mm, Ziegelreihen 0,13 mm, Fugen 0,1 mm."),
            "overlays": T("One cell per bone: stack block, two layers, twelve courses, pallet parts.", "Eine Zelle je Knochen: Stapelblock, zwei Lagen, zwölf Scharen, Palettenteile."),
        }
    if scale == 200:
        return {
            "silhouette": T("The pallet as deck slab (top deck and stringer boards, 44 mm), three block columns and the bottom board at the 30 mm smallest feature (blocks 70 mm, so the pallet keeps its 144 mm); the stack as one block with the four top layers as rows and slabs; the corner strips by their legs.", "Die Palette als Deckplatte (Deck- und Querbretter, 44 mm), drei Klotzsäulen und das Bodenbrett in der kleinsten Einzelheit von 30 mm (Klötze 70 mm, damit die Palette ihre 144 mm behält); der Stapel als ein Block mit den vier obersten Lagen als Reihen und Platten; die Kantenschutzwinkel über ihre Schenkel."),
            "retained": T("The fork openings of both sides, the coursing of the stack as a line hatch at every second bed joint, the four loose top layers, the strap loop as a texture outline in plan.", "Die Gabelöffnungen beider Seiten, die Schichtung des Stapels als Linienschraffur an jeder zweiten Lagerfuge, die vier losen obersten Lagen, die Bandschlaufe als Texturkontur in der Draufsicht."),
            "merged": T("The five deck boards and the three stringer boards into one slab; the mortar remnants and the straps into the stack (named on its outline); the drawings of layers 14 and 16 take in the course layer below them as 130 mm bands (0.65 mm), so no 65 mm sliver is left between them; the courses show on their own when they are set into the panel.", "Die fünf Deckbretter und die drei Querbretter zu einer Platte; die Mörtelreste und die Bänder in den Stapel (an seiner Kontur benannt); die Zeichnungen der Lagen 14 und 16 nehmen die Scharenlage darunter als 130-mm-Bänder (0,65 mm) auf, damit kein 65-mm-Streifen dazwischen bleibt; die Scharen erscheinen eigenständig, wenn sie in die Wand gesetzt sind."),
            "omitted": T("Single bricks and joints (0.05 mm), the 41 mm deck gaps (0.2 mm) and the chipped arrises.", "Einzelne Ziegel und Fugen (0,05 mm), die 41-mm-Deckspalte (0,2 mm) und die abgeschlagenen Kanten."),
            "thinElements": T("The 19 mm straps (0.1 mm) only as their loop in plan, named on the stack outline in the elevations; the 22 mm bottom boards at 30 mm.", "Die 19-mm-Bänder (0,1 mm) nur als ihre Schlaufe in der Draufsicht, in den Ansichten an der Stapelkontur benannt; die 22-mm-Bodenbretter mit 30 mm."),
            "hatching": T("Coursing: horizontal lines 0.65 mm apart (130 mm, two layers), pen 0.1 mm.", "Schichtung: waagrechte Linien im Abstand 0,65 mm (130 mm, zwei Lagen), Stift 0,1 mm."),
            "styles": T("Outlines 0.18 mm, rows 0.1 mm, texture 0.1 mm.", "Konturen 0,18 mm, Reihen 0,1 mm, Textur 0,1 mm."),
            "overlays": T("One cell per bone; the stack block names bricks and mortar remnants.", "Eine Zelle je Knochen; der Stapelblock benennt Ziegel und Mörtelreste."),
        }
    return {
        "silhouette": T("The pallet as one 1200 x 144 mm block (2.4 x 0.29 mm on paper), the stack as one block 1040 x 780 mm with the four top layers as two 130 mm bands, together 1.18 m high (2.37 mm): a pallet of bricks, not a single brick, stays drawable.", "Die Palette als ein Block 1200 x 144 mm (2,4 x 0,29 mm auf Papier), der Stapel als ein Block 1040 x 780 mm mit den vier obersten Lagen als zwei 130-mm-Bänder, zusammen 1,18 m hoch (2,37 mm): eine Palette Ziegel, kein einzelner Ziegel, bleibt zeichenbar."),
        "retained": T("The overall outline of pallet and stack, and the twelve courses so the reinstalled panel can be drawn at this scale.", "Die Gesamtkontur von Palette und Stapel und die zwölf Scharen, damit die wiedereingebaute Wand in diesem Maßstab gezeichnet werden kann."),
        "merged": T("Boards and blocks into the pallet block; corner strips, straps and mortar remnants into the stack (named on its outline); layers 14 and 16 are drawn as 130 mm bands that take in the course layer below them, the courses show on their own in the reinstalled panel.", "Bretter und Klötze zum Palettenblock; Kantenschutz, Bänder und Mörtelreste in den Stapel (an seiner Kontur benannt); die Lagen 14 und 16 sind als 130-mm-Bänder gezeichnet, die die Scharenlage darunter aufnehmen, die Scharen erscheinen eigenständig in der wiedereingebauten Wand."),
        "omitted": T("Fork openings (0.2 mm), joints, straps (0.04 mm) and every trace of use.", "Gabelöffnungen (0,2 mm), Fugen, Bänder (0,04 mm) und jede Gebrauchsspur."),
        "thinElements": T("The courses are drawn 77 mm high, a course with its bed joint, so the reinstalled panel prints as twelve touching courses down to 1:1000.", "Die Scharen sind 77 mm hoch gezeichnet, eine Schar mit ihrer Lagerfuge, so druckt die wiedereingebaute Wand bis 1:1000 als zwölf aneinanderliegende Scharen."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Outlines 0.15 mm.", "Konturen 0,15 mm."),
        "overlays": T("One cell per bone.", "Eine Zelle je Knochen."),
    }


VOLUME_DECISIONS = {
    50: [T("Body: the lot as it lies: 384 Reichsformat bricks of 250 x 120 x 65 mm in sixteen layers on the EPAL pallet, the A layers (even, from the pallet up) as six rows of four stretchers along x, side by side, with 10 mm head joints and every second row 10 mm further ahead, the B layers as eight rows of three bricks along z staggered the same way, so no joint runs through a layer and the layers bond crosswise; in the twelve stack layers each brick position is one solid repeated over the six layers of its kind (step 130 mm), split where a brick of the right face has a chipped arris (a 25 mm deep slice with the conchoidal two-facet notch and the full core behind it). Layers 14 and 16 are eight rows each, layers 13 and 15 the twelve courses of four bricks that the reinstalled pose sets into the panel.", "Körper: die Partie, wie sie liegt: 384 Reichsformatziegel 250 x 120 x 65 mm in sechzehn Lagen auf der EPAL-Palette, die A-Lagen (gerade, von der Palette aus) als sechs Reihen aus vier Läufern entlang x, Seite an Seite, mit 10-mm-Stoßfugen und jede zweite Reihe 10 mm weiter vorn, die B-Lagen als acht Reihen aus drei Ziegeln entlang z, ebenso versetzt, damit keine Fuge eine Lage durchläuft und die Lagen sich kreuzweise verbinden; in den zwölf Stapellagen ist jede Ziegelposition ein Körper, über die sechs Lagen seiner Art wiederholt (Schritt 130 mm), geteilt, wo ein Ziegel der rechten Seite eine abgeschlagene Kante hat (eine 25 mm tiefe Scheibe mit der muscheligen Zweiflächenkerbe und der volle Kern dahinter). Die Lagen 14 und 16 sind je acht Reihen, die Lagen 13 und 15 die zwölf Scharen aus vier Ziegeln, die die Wiedereinbaupose in die Wand setzt."),
         T("Body: the EPAL pallet board by board (three bottom boards, nine blocks, three stringer boards, five deck boards with their 41 mm gaps), four polypropylene corner strips of 65 x 65 x 3 mm as two declared sheets each, two PET girth straps 19 x 0.9 mm as four declared sheets each lying on the strips, and 20 lime mortar remnants 3 mm proud as texture solids on the faces where they show.", "Körper: die EPAL-Palette Brett für Brett (drei Bodenbretter, neun Klötze, drei Querbretter, fünf Deckbretter mit ihren 41-mm-Spalten), vier Kantenschutzwinkel aus Polypropylen 65 x 65 x 3 mm als je zwei deklarierte Bleche, zwei PET-Umreifungsbänder 19 x 0,9 mm als je vier deklarierte Bleche auf den Winkeln und 20 Kalkmörtelreste, 3 mm erhaben, als Texturkörper auf den Flächen, wo sie sichtbar sind."),
         T("Thermal: every body solid is referenced into its region (bricks, pallet timber, straps, corner strips, mortar remnants; the last three with their one layer). The brick region is an exchange area: the air reaches the faces in the open joints, which the envelope does not show.", "Thermisch: jeder Körper ist in seine Region referenziert (Ziegel, Palettenholz, Bänder, Kantenschutz, Mörtelreste; die letzten drei mit ihrer einen Schicht). Die Ziegelregion ist eine Austauschfläche: die Luft erreicht die Flächen in den offenen Fugen, die die Hülle nicht zeigt."),
         T("Structure: the pallet parts are the members of the pallet bone (the top deck spans between the blocks); the twelve stack layers are one compression member, the two set-down layers and each course a member of their own section, all standing on each other down to the bottom boards on the ground.", "Tragwerk: die Palettenteile sind die Stäbe des Palettenknochens (das Deck spannt zwischen den Klötzen); die zwölf Stapellagen sind ein Druckstab, die zwei abgesetzten Lagen und jede Schar je ein Stab ihres Querschnitts, alle aufeinander stehend bis zu den Bodenbrettern auf dem Boden.")],
    100: [T("Body: the pallet keeps its boards and blocks (22 mm and more, above the 15 mm feature); every layer becomes one slab of its own outline (A 1040 x 720 mm, B 960 x 780 mm), because its 10 mm head joints (0.1 mm) close at this scale; each course is one row of 1030 x 65 x 120 mm; corner strips and straps stay declared sheets, the mortar remnants texture solids; the chipped arrises are left out (below the feature).", "Körper: die Palette behält ihre Bretter und Klötze (22 mm und mehr, über der Einzelheit von 15 mm); jede Lage wird eine Platte mit eigener Kontur (A 1040 x 720 mm, B 960 x 780 mm), weil ihre 10-mm-Stoßfugen (0,1 mm) in diesem Maßstab schließen; jede Schar ist eine Reihe 1030 x 65 x 120 mm; Kantenschutz und Bänder bleiben deklarierte Bleche, die Mörtelreste Texturkörper; die abgeschlagenen Kanten entfallen (unter der Einzelheit)."),
          T("Thermal: references of the body solids into the five regions. Structure: the deck as one plate member, the stringer boards and bottom boards as members, the blocks as the joints between them; stack, layers and courses as at 1:50.", "Thermisch: Referenzen der Körper in die fünf Regionen. Tragwerk: das Deck als ein Plattenstab, Quer- und Bodenbretter als Stäbe, die Klötze als Knoten dazwischen; Stapel, Lagen und Scharen wie 1:50.")],
    200: [T("Body: the pallet as a 44 mm deck slab (top deck and stringer boards merged), nine blocks shortened to 70 mm and three bottom boards drawn at the 30 mm feature, so the pallet keeps its 144 mm and both fork openings; the twelve stack layers as one block of the stack's envelope (1040 x 780 mm; the 30 to 40 mm steps of the hand-stacked edge are about one feature) and the two layer slabs; the courses as rows 1030 x 65 x 120 mm; the corner strips remain declared sheets (65 mm legs, above the 30 mm feature), the straps become texture solids (19 mm, below it).", "Körper: die Palette als 44-mm-Deckplatte (Deck- und Querbretter vereinigt), neun auf 70 mm gekürzte Klötze und drei Bodenbretter in der Einzelheit 30 mm, so behält die Palette ihre 144 mm und beide Gabelöffnungen; die zwölf Stapellagen als ein Block der Stapelhülle (1040 x 780 mm; die Stufen von 30 bis 40 mm der von Hand gestapelten Kante sind etwa eine Einzelheit) und die zwei Lagenplatten; die Scharen als Reihen 1030 x 65 x 120 mm; die Kantenschutzwinkel bleiben deklarierte Bleche (65-mm-Schenkel, über der Einzelheit 30 mm), die Bänder werden Texturkörper (19 mm, darunter)."),
          T("Thermal: references, the straps and corner strips with their layer, the mortar remnants merged into four 3 mm crust patches of their layer on the right and front faces. Structure: deck slab member, blocks as joints, bottom boards, stack, layers and courses.", "Thermisch: Referenzen, Bänder und Kantenschutz mit ihrer Schicht, die Mörtelreste zu vier 3-mm-Krustenflecken ihrer Schicht auf der rechten und vorderen Seite zusammengefasst. Tragwerk: Deckplatte als Stab, Klötze als Knoten, Bodenbretter, Stapel, Lagen und Scharen.")],
    500: [T("Body: the pallet as one 1200 x 144 x 800 mm block (its 100 mm openings are 0.2 mm on paper), the stack block, layers 14 and 16 as 130 mm bands that take in the course layer below them (as the drawing does) and the twelve courses as rows 1030 x 77 x 120 mm, a course with its bed joint, so the reinstalled panel reads as twelve touching courses at the 75 mm feature; corner strips and straps are merged into the stack.", "Körper: die Palette als ein Block 1200 x 144 x 800 mm (ihre 100-mm-Öffnungen sind 0,2 mm auf Papier), der Stapelblock, die Lagen 14 und 16 als 130-mm-Bänder, die die Scharenlage darunter aufnehmen (wie die Zeichnung), und die zwölf Scharen als Reihen 1030 x 77 x 120 mm, eine Schar mit ihrer Lagerfuge, so liest sich die wiedereingebaute Wand als zwölf aneinanderliegende Scharen in der Einzelheit von 75 mm; Kantenschutz und Bänder sind in den Stapel aufgegangen."),
          T("Thermal: references of the blocks; straps, corner strips and mortar remnants keep thermal solids of their layer thickness only (not in the body). Structure: one pallet member, stack, layers and courses.", "Thermisch: Referenzen der Blöcke; Bänder, Kantenschutz und Mörtelreste behalten nur thermische Körper ihrer Schichtdicke (nicht im Körper). Tragwerk: ein Palettenstab, Stapel, Lagen und Scharen.")],
}

PENS = {50: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.25},
        200: {"body": 0.18, "detail": 0.1, "texture": 0.1, "glazing": None, "ground": 0.18}, 500: {"body": 0.15, "detail": None, "texture": None, "glazing": None, "ground": 0.13}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def representation(scale):
    """🖨️ One authored representation."""
    hatches = [{"id": "coursing", "title": T("Coursing, every second bed joint", "Schichtung, jede zweite Lagerfuge"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.65, "penMm": 0.1}] if scale == 200 else []
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": RANGES[scale][0], "maxDenominator": RANGES[scale][1]}, "pens": PENS[scale], "hatches": hatches, "geometry": geometry(scale), "decisions": rep_decisions(scale)}


def document():
    """📄️ The whole asset document."""
    asset = {
        "id": "reclaimed-brick-pallet", "category": "components",
        "title": T("Reclaimed Brick Pallet", "Palette gebrauchter Ziegel"),
        "summary": T(f"A pallet of 384 cleaned Reichsformat solid bricks (250 x 120 x 65 mm) from a demolished lime-mortared wall, hand-stacked in sixteen crosswise interlocked layers with open 10 mm head joints on an EPAL Euro pallet, girthed by two PET straps over four corner strips; chipped arrises and lime mortar remnants show their first life. It stands on the yard, is lifted 250 mm on forklift forks, and two of its layers are relaid beside it as a 120 mm stretcher-bond panel of twelve courses under a wall plate and wind. {MASS_TOTAL / 1000:.2f} t.", f"Eine Palette mit 384 gereinigten Vollziegeln im Reichsformat (250 x 120 x 65 mm) aus einer abgebrochenen, in Kalkmörtel gemauerten Wand, von Hand in sechzehn kreuzweise verzahnten Lagen mit offenen 10-mm-Stoßfugen auf einer EPAL-Europalette gestapelt und mit zwei PET-Bändern über vier Kantenschutzwinkeln umreift; abgeschlagene Kanten und Kalkmörtelreste zeigen ihr erstes Leben. Sie steht auf dem Lagerplatz, wird auf der Staplergabel 250 mm angehoben, und zwei ihrer Lagen sind neben ihr als 120 mm dicke Läuferverbandwand aus zwölf Scharen unter Schwelle und Wind wieder vermauert. {MASS_TOTAL / 1000:.2f} t."),
        "tags": ["component", "reclaimed", "masonry", "brick", "clay", "pallet", "stacked-goods", "reichsformat"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                       "notes": T("Lot of 384 bricks: 16 layers x 24; A layers six rows of four stretchers (1030 mm with three 10 mm head joints, every second row 10 mm further ahead so no joint runs through), B layers eight rows of three bricks across (770 mm, staggered the same way); stack envelope 1040 x 780 x 1040 mm on the 1200 x 800 x 144 mm pallet; the reinstalled panel uses the 48 bricks of layers 13 and 15.", "Partie aus 384 Ziegeln: 16 Lagen x 24; A-Lagen sechs Reihen aus vier Läufern (1030 mm mit drei 10-mm-Stoßfugen, jede zweite Reihe 10 mm weiter vorn, damit keine Fuge durchläuft), B-Lagen acht Reihen aus drei Ziegeln quer (770 mm, ebenso versetzt); Hülle des Stapels 1040 x 780 x 1040 mm auf der Palette 1200 x 800 x 144 mm; die wiedereingebaute Wand verwendet die 48 Ziegel der Lagen 13 und 15.")},
        "dimensions": {"heightM": rnd(STACK_TOP), "widthM": 1.2, "depthM": 0.8, "massKg": round(MASS_TOTAL, 2)},
        "materials": materials(), "rig": rig(), "anchors": anchors(),
        "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
        "symbols": [], "poses": poses(), "clips": clips(), "thermal": thermal_model(), "structure": structure_model(),
        "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    import json
    import os
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🔣️.json")
    with open(out, "w", encoding="utf-8") as f:
        json.dump(document(), f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(out, f"mass {MASS_TOTAL:.2f} kg", {k: round(v, 4) for k, v in AREA.items()})
