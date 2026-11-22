"""🔧️ One-off edit of author-brick-pallet.py: rows laid side by side, head joints staggered row to row (no see-through slits)."""
p = "author-brick-pallet.py"
s = open(p).read()
R = []
def rep(a, b):
    global s
    assert a in s, a[:80]
    s = s.replace(a, b)
rep('''SX0, SZ0, SY0 = 0.085, -0.385, 0.144
SX1, SZ1 = SX0 + 4 * BL + 3 * GAP, SZ0 + 6 * BW + 5 * GAP''', '''SX0, SX1, SZ0, SZ1, SY0 = 0.080, 1.120, -0.390, 0.390, 0.144
ROW_L = 4 * BL + 3 * GAP
STAGGER = 0.010''')
rep('''def bricks(layer):''', '''def a_x0(j):
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


def bricks(layer):''')
rep('''                x0, z0 = SX0 + PITCH_L * i, SZ0 + PITCH_W * j''', '''                x0, z0 = a_x0(j) + PITCH_L * i, a_z0(j)''')
rep('''                x0, z0 = SX0 + PITCH_W * i, SZ0 + PITCH_L * j''', '''                x0, z0 = b_x0(i), b_z0(i) + PITCH_L * j''')
rep('''        zc = SZ0 + PITCH_W * j + BW / 2
        shift = 0.065 if k % 2 == 0 else -0.065
        rest = (0.6 + shift, layer_y(layer), zc)''', '''        zc = a_z0(j) + BW / 2
        xc = a_x0(j) + ROW_L / 2
        shift = 0.065 if k % 2 == 0 else -0.065
        rest = (xc + shift, layer_y(layer), zc)''')
rep('''"rest": rest, "wall": wall, "zc": zc})''', '''"rest": rest, "wall": wall, "zc": zc, "xc": xc, "x0": a_x0(j)})''')
rep('''        cv = rnd(-(0.6 - src["rest"][0]))''', '''        cv = rnd(src["rest"][0] - src["xc"])''')
# patches
rep('''    for L, i, a, b, c, d in side:
        out.append((f"mortar-side-l{L + 1:02d}", "stack", "side", a, b, layer_y(L) + c, layer_y(L) + d, SZ0 - t, SZ0))
    for L, a, b, c, d in [(3, 0.625, 0.68, 0.02, 0.05), (9, 0.625, 0.68, 0.02, 0.05), (5, 0.88, 0.93, 0.01, 0.04), (11, 0.88, 0.93, 0.01, 0.04), (1, 0.36, 0.42, 0.03, 0.06)]:
        out.append((f"mortar-end-l{L + 1:02d}", "stack", "side", a, b, layer_y(L) + c, layer_y(L) + d, SZ0 - t, SZ0))
    out.append(("mortar-end-l14", "layer-14", "side", 0.49, 0.55, layer_y(13) + 0.02, layer_y(13) + 0.045, SZ0 - t, SZ0))
    out.append(("mortar-end-l16", "layer-16", "side", 0.75, 0.80, layer_y(15) + 0.03, layer_y(15) + 0.055, SZ0 - t, SZ0))
    for c in course_rows():
        i = c["k"] % 4
        x0 = SX0 + PITCH_L * i
        zf = c["zc"] - BW / 2
        out.append((f"mortar-{COURSE_IDS[c['k']]}", COURSE_IDS[c["k"]], "side", x0 + 0.08, x0 + 0.15, layer_y(c["layer"]), layer_y(c["layer"]) + 0.016, zf - t, zf))
    for L, a, b, c, d in [(0, -0.11, -0.05, 0.01, 0.035), (6, -0.11, -0.05, 0.01, 0.035), (10, 0.15, 0.21, 0.03, 0.055)]:
        out.append((f"mortar-front-l{L + 1:02d}", "stack", "front", SX1, SX1 + t, layer_y(L) + c, layer_y(L) + d, a, b))
    for L, a, b, c, d in [(1, -0.02, 0.06, 0.04, 0.062), (7, -0.02, 0.06, 0.04, 0.062)]:
        out.append((f"mortar-front-l{L + 1:02d}", "stack", "front", SX1, SX1 + t, layer_y(L) + c, layer_y(L) + d, a, b))''', '''    za = a_z0(0)
    for L, i, a, b, c, d in side:
        out.append((f"mortar-side-l{L + 1:02d}", "stack", "side", a, b, layer_y(L) + c, layer_y(L) + d, za - t, za))
    for L, i, a, b, c, d in [(3, 4, 0.625, 0.68, 0.02, 0.05), (9, 4, 0.625, 0.68, 0.02, 0.05), (5, 6, 0.88, 0.93, 0.01, 0.04), (11, 6, 0.88, 0.93, 0.01, 0.04), (1, 2, 0.38, 0.44, 0.03, 0.06)]:
        out.append((f"mortar-end-l{L + 1:02d}", "stack", "side", a, b, layer_y(L) + c, layer_y(L) + d, b_z0(i) - t, b_z0(i)))
    out.append(("mortar-end-l14", "layer-14", "side", 0.50, 0.56, layer_y(13) + 0.02, layer_y(13) + 0.045, b_z0(3) - t, b_z0(3)))
    out.append(("mortar-end-l16", "layer-16", "side", 0.75, 0.80, layer_y(15) + 0.03, layer_y(15) + 0.055, b_z0(5) - t, b_z0(5)))
    for c in course_rows():
        xe = c["x0"] + ROW_L
        out.append((f"mortar-{COURSE_IDS[c['k']]}", COURSE_IDS[c["k"]], "front", xe, xe + t, layer_y(c["layer"]), layer_y(c["layer"]) + 0.016, c["zc"] - 0.03, c["zc"] + 0.01))
    for L, j, a, b, c, d in [(0, 2, -0.11, -0.05, 0.01, 0.035), (6, 2, -0.11, -0.05, 0.01, 0.035), (10, 4, 0.15, 0.21, 0.03, 0.055)]:
        xe = a_x0(j) + ROW_L
        out.append((f"mortar-front-l{L + 1:02d}", "stack", "front", xe, xe + t, layer_y(L) + c, layer_y(L) + d, a, b))
    for L, a, b, c, d in [(1, -0.02, 0.06, 0.04, 0.062), (7, -0.02, 0.06, 0.04, 0.062)]:
        xe = b_x0(7) + BW
        out.append((f"mortar-front-l{L + 1:02d}", "stack", "front", xe, xe + t, layer_y(L) + c, layer_y(L) + d, a, b))''')
# brick area: all contacts
rep('''    per = 2 * (BL * BW + BW * BH + BL * BH)
    shared = 0.0
    for L in range(NL - 1):
        for _, ax0, ax1, az0, az1 in bricks(L):
            for _, bx0, bx1, bz0, bz1 in bricks(L + 1):
                shared += overlap(ax0, ax1, bx0, bx1) * overlap(az0, az1, bz0, bz1)
    return NL * 24 * per - 2 * shared, shared''', '''    per = 2 * (BL * BW + BW * BH + BL * BH)
    items = [(None, x0, x1, layer_y(L), layer_y(L) + BH, z0, z1) for L in range(NL) for _, x0, x1, z0, z1 in bricks(L)]
    shared = 0.0
    for a in range(len(items)):
        for b in range(a + 1, len(items)):
            if abs(items[a][3] - items[b][3]) < BH * 1.5:
                shared += contact_area(items[a], items[b])
    return NL * 24 * per - 2 * shared, shared''')
# face bricks
rep('''        if abs(z0 - SZ0) < 1e-9:
            side.append((key, x0, x1))
        if abs(x1 - SX1) < 1e-9:
            front.append((key, z0, z1))''', '''        if (kind_of(layer) == "A" and key[0] == 0) or (kind_of(layer) == "B" and key[1] == 0):
            side.append((key, x0, x1))
        if (kind_of(layer) == "A" and key[1] == 3) or (kind_of(layer) == "B" and key[0] == 7):
            front.append((key, z0, z1))''')
rep('''    side, front = face_bricks(layer)
    pen = BRICK_PEN[scale]
''', '''    side, front = face_bricks(layer)
    pen = BRICK_PEN[scale]
    ex0, ex1, ez0, ez1 = extent(kind)
    out["elevation"].append(shape(f"layer-{layer + 1:02d}-joints-side", bone, "body", 1.9, 0.26, erect(bone, ex0, ex1, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
    out["front"].append(shape(f"layer-{layer + 1:02d}-joints-front", bone, "body", 1.9, 0.26, frect(ez0, ez1, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
''')
# course shapes
rep('''            x0 = SX0 + PITCH_L * i
            tz''', '''            x0 = c["x0"] + PITCH_L * i
            tz''')
rep('''    if scale >= 200:
        z0, z1 = c["zc"] - 0.0625, c["zc"] + 0.0625
''', '')
rep('''erect(cid, SX0, SX1, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
        out["plan"].append(shape(f"{cid}-row", cid, "body", 2, tone_of("c", k), prect(SX0, SX1, z0, z1)''', '''erect(cid, c["x0"], c["x0"] + ROW_L, y0, y1), region=R_BRICK, material=CLAY, pen=pen))
        out["plan"].append(shape(f"{cid}-row", cid, "body", 2, tone_of("c", k), prect(c["x0"], c["x0"] + ROW_L, z0, z1)''')
rep('''                xj = SX0 + PITCH_L * i + BL + GAP / 2
                out["elevation"].append(shape(f"{cid}-head-joint''', '''                xj = c["x0"] + PITCH_L * i + BL + GAP / 2
                out["elevation"].append(shape(f"{cid}-head-joint''')
open(p, "w").write(s)
print("ok")
