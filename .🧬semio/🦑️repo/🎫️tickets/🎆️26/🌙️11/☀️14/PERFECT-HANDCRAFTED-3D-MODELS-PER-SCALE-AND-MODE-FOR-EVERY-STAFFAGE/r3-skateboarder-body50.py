"""🛹 Round 3 body of humans/skateboarder at 1:50 (smallest feature 7.5 mm): a 1.78 m, 19-year-old man in a cuffed knit beanie, a hooded cotton-fleece sweatshirt (hood down, kangaroo pocket, drawcords, rib hem and cuffs), loose cotton twill trousers breaking over low vulcanised skate shoes, on an 8.0 x 31.9 in street skateboard (seven-ply maple deck with concave, kicked nose and tail and grip tape, 8.0 in trucks of 53 mm with baseplate, kingpin, bushings, pivot and hanger, 54 mm wheels), every section stated by hand.

`build()` returns the solids with two private keys the assembler strips: `_region` (thermal region) and `_g` (the 2D shape the solid is drawn in).
"""
import importlib.util, math, os

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-skateboarder-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
K = mod("kit")

SKIN, KNIT, FLEECE, TWILL, SHOE, MAPLE, GRIP, ALU, PU = "skin", "knit", "fleece-cotton", "twill", "skate-shoe", "maple", "grip-tape", "aluminium", "polyurethane"
SIDES = ((1, "l"), (-1, "r"))
TONE = {"hoodie": 0.46, "trousers": 0.24, "shoe": 0.16, "sole": 0.93, "skin": 0.8, "beanie": 0.3, "maple": 0.74, "grip": 0.12, "alu": 0.68, "bushing": 0.55, "wheel": 0.95}


def tag(s, region, g):
    s["_region"] = region
    s["_g"] = g
    return s


def rot(p, deg):
    r = math.radians(deg)
    return (p[0] * math.cos(r) - p[1] * math.sin(r), p[0] * math.sin(r) + p[1] * math.cos(r))


def foot_local(x, y):
    """👟 A point of the flat shoe frame (x ahead of the ankle, y up from the ankle, ground at y = -ANKLE) in the foot bone's frame."""
    return rot((x, y), R.FOOT_DROP)


def toe_local(x, y):
    return (x - R.BALL_FWD, y + R.ANKLE - R.BALL_UP)


TILT = math.radians(R.HEAD_TILT)
UP_H = (math.cos(TILT), -math.sin(TILT))
POST_H = (math.sin(TILT), math.cos(TILT))


def hp(y, z, w=0.0):
    """🙂 A head point given upright from C7 (y up, z forward, w lateral) in the head bone frame (u along the bone tilted forward by HEAD_TILT, v posterior)."""
    return [y * UP_H[0] - z * POST_H[0], y * UP_H[1] - z * POST_H[1], w]


HEAD_Q = K.quat(K.W, list(POST_H) + [0.0])


def head_stack(i, rows, material, tone, grow=0.0, role="body", caps=(True, True), **extra):
    """🗿 A head loft of upright sections: rows (y, z front, z back, half width[, exponent]) in C7 coordinates, grown by `grow` on every side."""
    secs = []
    for r in rows:
        y, zf, zb, hw = r[:4]
        e = r[4] if len(r) > 4 else 2.0
        secs.append(K.section(hp(y, (zf + zb) / 2), hw + grow, (zf - zb) / 2 + grow, e, HEAD_Q))
    return K.loft(i, "head", secs, material, tone, role, caps, **extra)


HOODIE = [(-0.088, 0.100, 0.082, 0.176, 2.6), (-0.070, 0.104, 0.086, 0.180, 2.6), (-0.048, 0.106, 0.088, 0.180, 2.6), (-0.040, 0.112, 0.090, 0.188, 2.6), (-0.005, 0.120, 0.086, 0.182, 2.5), (0.040, 0.126, 0.082, 0.172, 2.4), (0.100, 0.130, 0.088, 0.172, 2.4), (0.160, 0.136, 0.098, 0.178, 2.4), (0.230, 0.143, 0.106, 0.184, 2.4), (0.290, 0.145, 0.110, 0.190, 2.5), (0.345, 0.138, 0.112, 0.198, 2.6), (0.395, 0.122, 0.108, 0.204, 2.8), (0.430, 0.106, 0.102, 0.196, 2.8), (0.455, 0.092, 0.096, 0.172, 2.6), (0.478, 0.080, 0.088, 0.132, 2.3), (0.496, 0.072, 0.080, 0.096, 2.1), (0.508, 0.066, 0.075, 0.080, 2.0)]
SEAT = [(-0.114, 0.030, 0.040, 0.040, 2.0), (-0.100, 0.052, 0.088, 0.100, 2.2), (-0.075, 0.072, 0.118, 0.152, 2.3), (-0.040, 0.088, 0.134, 0.180, 2.4), (0.000, 0.092, 0.132, 0.184, 2.4), (0.050, 0.096, 0.122, 0.178, 2.4), (0.100, 0.100, 0.112, 0.168, 2.4)]


def row_at(rows, u):
    for a, b in zip(rows, rows[1:]):
        if a[0] <= u <= b[0]:
            f = (u - a[0]) / (b[0] - a[0])
            return tuple(a[k] + (b[k] - a[k]) * f for k in range(len(a)))
    raise ValueError(u)


def trunk(out):
    out.append(tag(K.stack("trousers-seat", "pelvis", SEAT, TWILL, TONE["trousers"]), "trousers", "trousers-seat"))
    out.append(tag(K.stack("hoodie", "torso", HOODIE, FLEECE, TONE["hoodie"]), "hoodie-torso", "hoodie"))
    band = [(u, f + 0.004, b + 0.004, hw + 0.004, e) for u, f, b, hw, e in (row_at(HOODIE, -0.086), row_at(HOODIE, -0.066), row_at(HOODIE, -0.046))]
    out.append(tag(K.stack("hoodie-hem", "torso", band, FLEECE, 0.4, role="detail", texture=True), "hoodie-torso", "hoodie-hem"))
    pocket = [(u, row_at(HOODIE, u)[1] + 0.005, -row_at(HOODIE, u)[1] + 0.018, hw, 4.0) for u, hw in ((-0.028, 0.120), (0.030, 0.125), (0.090, 0.112), (0.135, 0.080))]
    out.append(tag(K.stack("hoodie-pocket", "torso", pocket, FLEECE, 0.4, role="detail", texture=True), "hoodie-torso", "hoodie-pocket"))
    hood = [(0.300, -0.090, 0.128, 0.090, 2.4), (0.340, -0.094, 0.140, 0.120, 2.4), (0.390, -0.090, 0.142, 0.140, 2.5), (0.440, -0.080, 0.134, 0.140, 2.5), (0.480, -0.062, 0.120, 0.124, 2.4), (0.515, -0.040, 0.104, 0.100, 2.2), (0.535, -0.012, 0.090, 0.080, 2.0)]
    out.append(tag(K.onto_girdle(K.stack("hood", "torso", hood, FLEECE, 0.4), R.GIRDLE, R.GIRDLE_X), "hoodie-torso", "hood"))
    collar = [(0.016 * math.sin(2 * math.pi * k / 16), 0.072 + 0.016 * math.cos(2 * math.pi * k / 16)) for k in range(16)]
    out.append(tag(K.onto_girdle(K.revolve("hoodie-collar", "torso", collar + [collar[0]], [R.TORSO - 0.004, 0.004, 0.0], [1.0, 0.0, 0.0], FLEECE, 0.4), R.GIRDLE, R.GIRDLE_X), "hoodie-torso", "hood"))
    for s, n2 in SIDES:
        pts = [([R.TORSO - 0.012, -0.068, s * 0.030], 0.0025, 0.0025), ([R.TORSO - 0.060, -0.086, s * 0.034], 0.0025, 0.0025), ([R.TORSO - 0.120, -0.104, s * 0.036], 0.0025, 0.0025), ([R.TORSO - 0.175, -0.116, s * 0.037], 0.0025, 0.0025), ([R.TORSO - 0.186, -0.117, s * 0.037], 0.0045, 0.0045)]
        out.append(tag(K.tube(f"drawcord-{n2}", "torso", pts, FLEECE, 0.9, hint=K.W, role="detail", texture=True), "hoodie-torso", "drawcord"))


NECK = [(-0.050, 0.075, -0.045, 0.062), (-0.010, 0.080, -0.040, 0.061), (0.030, 0.088, -0.032, 0.058), (0.060, 0.094, -0.022, 0.057), (0.085, 0.100, -0.012, 0.058)]
HEAD = [(0.016, 0.142, 0.110, 0.020, 2.0), (0.026, 0.152, 0.060, 0.042, 2.0), (0.045, 0.158, 0.010, 0.056, 2.1), (0.068, 0.160, -0.020, 0.063, 2.1), (0.092, 0.164, -0.036, 0.068, 2.1), (0.120, 0.160, -0.046, 0.073, 2.1), (0.148, 0.163, -0.050, 0.077, 2.1), (0.176, 0.160, -0.051, 0.078, 2.1), (0.202, 0.148, -0.047, 0.075, 2.0), (0.222, 0.126, -0.036, 0.066, 2.0), (0.237, 0.094, -0.014, 0.048, 2.0), (0.2474, 0.052, 0.016, 0.014, 2.0)]


def head_row(y):
    return row_at(HEAD, y)


def beanie_rows():
    """🧶 The cuffed beanie: a dome over the skull from a rim tilted 14 degrees (low at the nape, at mid forehead in front), knit and air 10 mm proud of the scalp over the whole tilted slice, rising 15 mm over the vertex (the slack knit)."""
    rows = []
    drop = 0.11 * math.tan(math.radians(14.0))
    for y in (0.150, 0.165, 0.180, 0.195, 0.210, 0.222, 0.232, 0.240, 0.247, 0.2535, 0.2585, 0.262):
        lo = head_row(max(0.016, min(y - drop, 0.2474)))
        hi = head_row(max(0.016, min(y, 0.2474)))
        zf, zb, hw = hi[1], lo[2], max(lo[3], hi[3])
        if y > 0.2474 - drop:
            f = max(0.0, 1 - ((y - 0.2474 + drop) / (0.0146 + drop)) ** 2.2)
            zc = (zf + zb) / 2
            zf, zb, hw = zc + (zf - zc) * max(f, 0.06), zc - (zc - zb) * max(f, 0.06), max(hw * max(f ** 0.5, 0.06), 0.005)
        rows.append((y, zf + 0.010, zb - 0.010, hw + 0.010, 2.1))
    return rows


def beanie_sections(rows, grow, tilt_deg=14.0):
    """🧶 Beanie sections: each row's ellipse laid in a plane tilted front-up by `tilt_deg` about the lateral axis through its centre (rim low at the nape, high on the forehead)."""
    t = math.radians(tilt_deg)
    post = (POST_H[0] * math.cos(t) - UP_H[0] * math.sin(t), POST_H[1] * math.cos(t) - UP_H[1] * math.sin(t))
    q = K.quat(K.W, [post[0], post[1], 0.0])
    secs = []
    for y, zf, zb, hw, e in rows:
        zc = (zf + zb) / 2
        secs.append(K.section(hp(y, zc), hw + grow, (zf - zb) / 2 / math.cos(t) + grow, e, q))
    return secs


def head(out):
    out.append(tag(head_stack("neck", NECK, SKIN, TONE["skin"]), "face", "head"))
    out.append(tag(head_stack("head", HEAD, SKIN, TONE["skin"]), "face", "head"))
    jaw = [(0.020, 0.150, 0.100, 0.026, 2.0), (0.040, 0.156, 0.040, 0.054, 2.2), (0.062, 0.150, 0.020, 0.062, 2.3)]
    out.append(tag(head_stack("jaw", jaw, SKIN, TONE["skin"]), "face", "head"))
    pts = [(hp(0.146, 0.158), 0.0085, 0.0075), (hp(0.124, 0.170), 0.0095, 0.0085), (hp(0.104, 0.183), 0.0115, 0.0100), (hp(0.094, 0.181), 0.0130, 0.0090), (hp(0.089, 0.170), 0.0140, 0.0070)]
    out.append(tag(K.onto_girdle(K.tube("nose", "head", [(p, a, b) for p, a, b in pts], SKIN, TONE["skin"], hint=K.W, role="detail"), R.GAZE, R.GAZE_X), "face", "head"))
    out.append(tag(K.ellipsoid("lips", "head", [0.011, 0.007, 0.024], hp(0.071, 0.153), SKIN, 0.62, role="detail", rotation=K.rot_w(R.HEAD_TILT)), "face", "head"))
    for s, n in SIDES:
        out.append(tag(K.ellipsoid(f"ear-{n}", "head", [0.031, 0.014, 0.009], hp(0.138, 0.040, s * 0.077), SKIN, 0.76, role="detail", rotation=K.rot_w(R.HEAD_TILT - 14)), "face", "head"))
        out.append(tag(K.ellipsoid(f"eye-{n}", "head", [0.0055, 0.0045, 0.0115], hp(0.132, 0.152, s * 0.032), SKIN, 0.2, role="detail"), "face", f"eye-{n}"))
        out.append(tag(K.ellipsoid(f"brow-{n}", "head", [0.0045, 0.006, 0.018], hp(0.151, 0.161, s * 0.033), SKIN, 0.35, role="detail", texture=True), "face", f"eye-{n}"))
    rows = beanie_rows()
    out.append(tag(K.loft("beanie", "head", beanie_sections(rows, 0.0), KNIT, TONE["beanie"]), "beanie", "beanie"))
    cuff = [r for r in rows if r[0] <= 0.195]
    out.append(tag(K.loft("beanie-cuff", "head", beanie_sections(cuff, 0.006), KNIT, 0.26, role="detail"), "beanie", "beanie"))


UPPER = [(-0.078, 0.0, -0.016, 0.012, 0.012), (-0.070, 0.0, -0.014, 0.034, 0.032), (-0.052, 0.0, -0.008, 0.048, 0.047), (-0.030, 0.0, -0.002, 0.055, 0.054), (0.010, 0.002, 0.004, 0.058, 0.060), (0.060, 0.004, 0.004, 0.057, 0.058), (0.120, 0.004, 0.002, 0.054, 0.053), (0.190, 0.002, 0.0, 0.050, 0.049), (0.260, -0.002, 0.0, 0.047, 0.046), (0.320, -0.004, 0.0, 0.046, 0.046), (0.352, -0.004, 0.0, 0.040, 0.041), (0.372, -0.004, 0.0, 0.026, 0.028)]
FORE = [(-0.045, -0.004, 0.0, 0.034, 0.036), (-0.015, -0.004, 0.0, 0.045, 0.047), (0.030, -0.002, 0.002, 0.049, 0.050), (0.090, 0.0, 0.002, 0.046, 0.046), (0.150, 0.0, 0.0, 0.043, 0.041), (0.200, 0.0, 0.0, 0.042, 0.038), (0.216, 0.0, 0.0, 0.040, 0.036)]
CUFF = [(0.210, 0.0, 0.0, 0.034, 0.029), (0.222, 0.0, 0.0, 0.035, 0.030), (0.244, 0.0, 0.0, 0.033, 0.028), (0.250, 0.0, 0.0, 0.029, 0.025)]
WRIST = [(0.236, 0.0, 0.0, 0.026, 0.019), (0.250, 0.0, 0.0, 0.027, 0.019), (0.266, 0.0, 0.0, 0.030, 0.018)]
PALM = [(-0.006, 0.002, 0.0, 0.029, 0.019, 2.2), (0.020, 0.004, 0.0, 0.036, 0.016, 2.5), (0.050, 0.004, 0.0, 0.043, 0.0155, 2.7), (0.080, 0.002, 0.0, 0.044, 0.0145, 2.7), (0.097, 0.0, 0.0, 0.041, 0.0125, 2.4)]
FINGERS = [(0.028, 0.94, 0.0090), (0.009, 1.0, 0.0092), (-0.010, 0.95, 0.0088), (-0.027, 0.78, 0.0080)]
PHALANX = (0.046, 0.028, 0.022)
CURL = (18.0, 52.0, 70.0)


def finger(i, bone, v0, scale, r, side):
    """🖐️ A relaxed finger: three phalanges from the knuckle, curled towards the palm (medial) by the cumulative angles of CURL."""
    medial = -side
    p = [0.093, v0, medial * 0.002]
    pts = [(list(p), r * 1.05, r * 0.95)]
    for L, a in zip(PHALANX, CURL):
        L *= scale
        p = [p[0] + L * math.cos(math.radians(a)), p[1] - 0.06 * L * math.copysign(1, v0) * 0.0, p[2] + medial * L * math.sin(math.radians(a))]
        pts.append((list(p), r * (0.9 if a < 60 else 0.78), r * (0.85 if a < 60 else 0.72)))
    return K.tube(i, bone, pts, SKIN, TONE["skin"], hint=K.V, role="detail")


def arms(out):
    for s, n in SIDES:
        out.append(tag(K.limb(f"sleeve-{n}", f"upper-arm-{n}", UPPER, FLEECE, TONE["hoodie"], s), "hoodie-sleeves", f"sleeve-{n}"))
        out.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", FORE, FLEECE, TONE["hoodie"], s), "hoodie-sleeves", f"forearm-{n}"))
        out.append(tag(K.limb(f"cuff-{n}", f"forearm-{n}", CUFF, FLEECE, 0.4, s, role="detail"), "hoodie-sleeves", f"forearm-{n}"))
        out.append(tag(K.limb(f"wrist-{n}", f"forearm-{n}", WRIST, SKIN, TONE["skin"], s), "hands", f"wrist-{n}"))
        out.append(tag(K.limb(f"palm-{n}", f"hand-{n}", PALM, SKIN, TONE["skin"], s), "hands", f"hand-{n}"))
        for k, (v0, sc, r) in enumerate(FINGERS):
            out.append(tag(finger(f"finger-{n}-{k}", f"hand-{n}", v0, sc, r, s), "hands", f"hand-{n}"))
        medial = -s
        out.append(tag(K.tube(f"thumb-{n}", f"hand-{n}", [([0.010, 0.026, medial * 0.004], 0.0125, 0.0120), ([0.040, 0.040, medial * 0.012], 0.0110, 0.0105), ([0.070, 0.045, medial * 0.020], 0.0095, 0.0090), ([0.092, 0.040, medial * 0.026], 0.0085, 0.0080), ([0.104, 0.034, medial * 0.028], 0.0070, 0.0065)], SKIN, TONE["skin"], hint=K.W, role="detail"), "hands", f"hand-{n}"))


THIGH = [(-0.110, 0.0, 0.0, 0.070, 0.070), (-0.050, 0.004, 0.010, 0.088, 0.090), (0.030, 0.012, 0.012, 0.094, 0.095), (0.110, 0.014, 0.008, 0.090, 0.089), (0.200, 0.012, 0.004, 0.083, 0.080), (0.280, 0.010, 0.0, 0.075, 0.071), (0.350, 0.010, -0.002, 0.068, 0.064), (0.400, 0.010, 0.0, 0.065, 0.062), (0.440, 0.006, 0.0, 0.062, 0.060), (0.470, 0.002, 0.0, 0.052, 0.052), (0.492, 0.0, 0.0, 0.030, 0.030)]
SHANK = [(-0.050, 0.004, 0.0, 0.050, 0.050), (-0.010, 0.006, 0.0, 0.060, 0.058), (0.050, -0.004, 0.0, 0.062, 0.058), (0.120, -0.010, -0.002, 0.066, 0.059), (0.200, -0.008, -0.002, 0.063, 0.058), (0.280, -0.004, 0.0, 0.060, 0.058), (0.350, 0.0, 0.0, 0.060, 0.059), (0.400, 0.004, 0.0, 0.062, 0.061), (0.428, 0.008, 0.0, 0.064, 0.062), (0.438, 0.008, 0.0, 0.060, 0.058)]
HEM = [(0.392, 0.004, 0.0, 0.065, 0.064), (0.404, 0.006, 0.0, 0.0665, 0.065), (0.422, 0.008, 0.0, 0.0665, 0.064), (0.432, 0.008, 0.0, 0.064, 0.061)]
SOLE = [(-0.066, 0.034, 0.030), (-0.058, 0.040, 0.037), (-0.035, 0.042, 0.040), (0.000, 0.043, 0.040), (0.040, 0.046, 0.041), (0.080, 0.052, 0.045), (0.120, 0.056, 0.050), (0.150, 0.057, 0.052)]
SOLE_TOE = [(0.135, 0.057, 0.052), (0.165, 0.056, 0.051), (0.190, 0.052, 0.047), (0.210, 0.043, 0.039), (0.222, 0.029, 0.026), (0.2285, 0.010, 0.009)]
UPPER_SHOE = [(-0.064, -0.018, 2.2), (-0.058, -0.006, 2.4), (-0.040, -0.004, 2.6), (-0.010, 0.002, 2.6), (0.020, 0.010, 2.6), (0.050, 0.004, 2.6), (0.080, -0.008, 2.6), (0.110, -0.018, 2.6), (0.150, -0.026, 2.6)]
TOE_BOX = [(0.135, -0.024, 2.6), (0.160, -0.030, 2.6), (0.185, -0.035, 2.5), (0.205, -0.040, 2.3), (0.220, -0.046, 2.1), (0.226, -0.052, 2.0)]
SOLE_T = 0.014


def lm(x, table):
    for a, b in zip(table, table[1:]):
        if a[0] <= x <= b[0]:
            f = (x - a[0]) / (b[0] - a[0])
            return a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f
    return (table[0][1], table[0][2]) if x < table[0][0] else (table[-1][1], table[-1][2])


def shoe_sections(rows, table, side, conv, inset):
    up = conv(0.0, 1.0)
    o = conv(0.0, 0.0)
    q = K.quat(K.W, [up[0] - o[0], up[1] - o[1], 0.0])
    secs = []
    bottom = -R.ANKLE + SOLE_T - 0.004
    for x, top, e in rows:
        L, M = lm(x, table)
        hw = max((L + M) / 2 - inset, 0.006)
        wc = side * (L - M) / 2
        u, v = conv(x, (top + bottom) / 2)
        secs.append(K.section([u, v, wc], hw, (top - bottom) / 2, e, q))
    return secs


def sole_outline(table, side):
    lat = [(side * L, x) for x, L, M in table]
    med = [(-side * M, x) for x, L, M in reversed(table)]
    return lat + med


def legs(out):
    for s, n in SIDES:
        out.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", THIGH, TWILL, TONE["trousers"], s), "trousers", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-{n}", f"shank-{n}", SHANK, TWILL, TONE["trousers"], s), "trousers", f"shank-{n}"))
        out.append(tag(K.limb(f"hem-{n}", f"shank-{n}", HEM, TWILL, 0.2, s, role="detail", texture=True), "trousers", f"shank-{n}"))
        out.append(tag(K.loft(f"shoe-{n}", f"foot-{n}", shoe_sections(UPPER_SHOE, SOLE, s, foot_local, 0.003), SHOE, TONE["shoe"]), "shoes", f"shoe-{n}"))
        sole = K.extrude(f"sole-{n}", f"foot-{n}", [(z, x) for z, x in sole_outline(SOLE, s)], "zx", -R.ANKLE, -R.ANKLE + SOLE_T, SHOE, TONE["sole"])
        sole["transform"] = {"rotation": K.rot_w(R.FOOT_DROP)}
        out.append(tag(sole, "shoes", f"shoe-{n}"))
        out.append(tag(K.loft(f"shoe-toe-{n}", f"toe-{n}", shoe_sections(TOE_BOX, SOLE_TOE, s, toe_local, 0.003), SHOE, TONE["shoe"]), "shoes", f"shoe-toe-{n}"))
        tsole = K.extrude(f"sole-toe-{n}", f"toe-{n}", [(z, x - R.BALL_FWD) for z, x in sole_outline(SOLE_TOE, s)], "zx", -R.BALL_UP, -R.BALL_UP + SOLE_T, SHOE, TONE["sole"])
        out.append(tag(tsole, "shoes", f"shoe-toe-{n}"))
        stripe = [([*foot_local(x, y)[:2], s * (lm(x, SOLE)[0] - 0.002)], 0.0045, 0.0012, 4.0) for x, y in ((-0.040, -0.055), (0.000, -0.045), (0.040, -0.040), (0.075, -0.055))]
        out.append(tag(K.tube(f"shoe-stripe-{n}", f"foot-{n}", stripe, SHOE, 0.9, hint=K.W, role="detail", texture=True), "shoes", f"shoe-{n}"))


def deck_path(n=26):
    """🛹 The deck's middle surface as (x, y) points in the board frame, from tail tip to nose tip, denser over the kicks."""
    xs = []
    for k in range(n + 1):
        t = k / n
        xs.append(R.TAIL_TIP + (R.NOSE_TIP - R.TAIL_TIP) * (0.5 - 0.5 * math.cos(math.pi * t)))
    return [(x, R.deck_y(x)) for x in xs]


def deck_half(i, side, top_skin=False):
    """🪵 One half of the concave deck: a superelliptic plate from the centre line to the rail, tilted up by the concave (rails 8 mm above the centre), following the kicked centre curve; the two halves cross at the centre line into the concave valley. `top_skin`: the 0.6 mm grip tape on its top face."""
    pts = deck_path()
    secs = []
    tilt = math.atan2(R.CONCAVE, R.DECK_W / 2)
    for k, (x, y) in enumerate(pts):
        a = pts[max(0, k - 1)]
        b = pts[min(len(pts) - 1, k + 1)]
        t = K.unit([b[0] - a[0], b[1] - a[1], 0.0])
        nrm = [-t[1], t[0], 0.0]
        hw = max(R.deck_half_width(x), 0.004)
        X = K.unit([K.W[q] * side * math.cos(tilt) + nrm[q] * math.sin(tilt) for q in range(3)])
        Y = K.cross(X, t) if side > 0 else K.cross(t, X)
        th, rx, lift = R.DECK_T / 2, hw / 2 + 0.003, R.CONCAVE / 4
        if top_skin:
            th, rx, lift = 0.0006, hw / 2 - 0.001, lift + R.DECK_T / 2 + 0.0005
        if k in (0, len(pts) - 1):
            rx = 0.003
        c = [x + nrm[0] * lift + X[0] * (hw / 2 - 0.003), y + nrm[1] * lift + X[1] * (hw / 2 - 0.003), X[2] * (hw / 2 - 0.003)]
        secs.append(K.section(c, rx, th, 6.0 if not top_skin else 4.0, K.quat(X, Y)))
    return secs


def board(out):
    for s, n in SIDES:
        out.append(tag(K.loft(f"deck-{n}", "board", deck_half(f"deck-{n}", s), MAPLE, TONE["maple"]), "deck", "deck"))
        out.append(tag(K.loft(f"grip-{n}", "board", deck_half(f"grip-{n}", s, True), GRIP, TONE["grip"], role="detail", texture=True), "deck", "grip"))
    for tx, tn in ((R.AXLE_X, "f"), (-R.AXLE_X, "r")):
        o = 1 if tx > 0 else -1
        base_x = tx + o * 0.006
        y0 = -R.DECK_T / 2
        out.append(tag(K.box(f"baseplate-{tn}", "board", [0.068, 0.0075, 0.056], [base_x, y0 - 0.00375, 0.0], ALU, TONE["alu"], corner=0.0035), "trucks", f"truck-{tn}"))
        ay = -R.DECK_Y + R.WHEEL_R
        hanger = []
        for z, ry, rx, yc, xc in ((-0.066, 0.0085, 0.0085, ay, tx), (-0.050, 0.0105, 0.0100, ay + 0.002, tx), (-0.025, 0.0135, 0.0125, ay + 0.0065, tx + o * 0.002), (0.0, 0.0165, 0.0140, ay + 0.0095, tx + o * 0.003), (0.025, 0.0135, 0.0125, ay + 0.0065, tx + o * 0.002), (0.050, 0.0105, 0.0100, ay + 0.002, tx), (0.066, 0.0085, 0.0085, ay, tx)):
            hanger.append(K.section([xc, yc, z], rx, ry, 2.4, K.quat(K.U, K.V)))
        out.append(tag(K.loft(f"hanger-{tn}", "board", hanger, ALU, TONE["alu"]), "trucks", f"truck-{tn}"))
        out.append(tag(K.capsule(f"bushing-top-{tn}", "board", [tx - o * 0.017, y0 - 0.011, 0.0], [tx - o * 0.010, ay + 0.024, 0.0], 0.0135, 0.0140, PU, TONE["bushing"]), "trucks", f"truck-{tn}"))
        out.append(tag(K.capsule(f"bushing-bottom-{tn}", "board", [tx - o * 0.006, ay + 0.004, 0.0], [tx - o * 0.003, ay - 0.008, 0.0], 0.0120, 0.0110, PU, TONE["bushing"]), "trucks", f"truck-{tn}"))
        out.append(tag(K.capsule(f"kingpin-{tn}", "board", [tx - o * 0.020, y0 - 0.006, 0.0], [tx - o * 0.001, ay - 0.016, 0.0], 0.0045, 0.0045, ALU, 0.5, role="detail"), "trucks", f"truck-{tn}"))
        out.append(tag(K.capsule(f"pivot-{tn}", "board", [tx + o * 0.004, ay + 0.014, 0.0], [tx + o * 0.026, y0 - 0.010, 0.0], 0.0062, 0.0050, ALU, TONE["alu"]), "trucks", f"truck-{tn}"))
        out.append(tag(K.capsule(f"axle-{tn}", "board", [tx, ay, -R.AXLE_HALF], [tx, ay, R.AXLE_HALF], 0.004, 0.004, ALU, 0.5, role="detail"), "trucks", f"truck-{tn}"))
    wheel = [(-R.WHEEL_W / 2, 0.0), (-R.WHEEL_W / 2, 0.0175), (-R.WHEEL_W / 2 + 0.0015, 0.0205), (-0.012, 0.0245), (-0.0085, 0.0268), (-0.004, R.WHEEL_R), (0.009, R.WHEEL_R), (0.0125, 0.0262), (0.0150, 0.0225), (R.WHEEL_W / 2, 0.0185), (R.WHEEL_W / 2, R.BEARING_R), (R.WHEEL_W / 2 - 0.002, R.BEARING_R), (R.WHEEL_W / 2 - 0.002, 0.0)]
    for b, tn in (("wheel-f", "f"), ("wheel-r", "r")):
        for s, n in SIDES:
            out.append(tag(K.revolve(f"wheel-{tn}-{n}", b, [(a if s > 0 else -a, r) for a, r in wheel], [0.0, 0.0, s * R.WHEEL_Z], [0.0, 0.0, 1.0], PU, TONE["wheel"]), "wheels", f"wheel-{tn}"))


def build():
    out = []
    trunk(out)
    head(out)
    arms(out)
    legs(out)
    board(out)
    return out


if __name__ == "__main__":
    import json
    b = build()
    print(len(b), "solids")
    print(json.dumps(b[0])[:300])
