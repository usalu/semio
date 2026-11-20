"""🚲 Round 3 body of humans/cyclist at 1:50 (smallest feature 7.5 mm): a 1.72 m woman in a softshell jacket, stretch trousers with reflective clips, leather gloves, canvas sneakers and a vented city helmet, on a 28 inch low-step city bicycle (wave frame, rigid fork, swept-back bar, fenders, rear carrier, chain guard, side stand, hub dynamo lamp), every section stated by hand.

`build()` returns the solids with two private keys the assembler strips: `_region` (thermal region) and `_g` (the 2D shape the solid is drawn in).
"""
import importlib.util, json, math, os

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-cyclist")


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-cyclist-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
K = mod("kit")
FK = mod("fk")

SKIN, HAIR, JACKET, TROUSER, GLOVE, SHOE, HELMET = "skin", "hair", "softshell", "stretch-trouser", "leather-glove", "canvas-sneaker", "eps-helmet"
STEEL, RUBBER, SADDLE = "steel", "rubber", "leather-saddle"
SIDES = ((1, "l"), (-1, "r"))
TONE = {"jacket-l": 0.22, "jacket-r": 0.3, "jacket": 0.3, "trouser-l": 0.12, "trouser-r": 0.2, "glove-l": 0.07, "glove-r": 0.15, "shoe-l": 0.74, "shoe-r": 0.82}


def tag(s, region, g):
    s["_region"] = region
    s["_g"] = g
    return s


def rot(p, deg):
    r = math.radians(deg)
    return (p[0] * math.cos(r) - p[1] * math.sin(r), p[0] * math.sin(r) + p[1] * math.cos(r))


def foot_local(x, y):
    """👟 A point of the flat shoe frame (x ahead of the ankle, y up from the ankle, ground at y = -ANKLE_SHOE) in the foot bone's frame."""
    return rot((x, y), R.FOOT_DROP)


def toe_local(x, y):
    return (x - R.ANKLE_TO_BALL_X, y + R.ANKLE_SHOE - R.BALL_SHOE)


def posed():
    root = json.load(open(os.path.join(OUT, "root.json")))
    pose = next(p for p in root["poses"] if p["id"] == "riding-upright")
    return FK.chain(root["rig"], pose["bones"], pose.get("root", {}))


def local_dir(f, d):
    return [sum(d[i] * f[a][i] for i in range(3)) for a in ("u", "v", "w")]


JACKET_ROWS = [(-0.124, 0.130, 0.090, 0.174, 2.3), (-0.115, 0.146, 0.104, 0.190, 2.3), (-0.085, 0.142, 0.102, 0.188, 2.3), (-0.05, 0.130, 0.092, 0.177, 2.3), (-0.01, 0.118, 0.068, 0.158, 2.3), (0.04, 0.112, 0.052, 0.140, 2.3), (0.09, 0.112, 0.048, 0.132, 2.3), (0.15, 0.124, 0.052, 0.140, 2.3), (0.20, 0.148, 0.060, 0.150, 2.3), (0.25, 0.168, 0.068, 0.156, 2.35), (0.295, 0.164, 0.074, 0.162, 2.35), (0.34, 0.140, 0.078, 0.176, 2.4), (0.38, 0.112, 0.076, 0.194, 2.4), (0.408, 0.092, 0.070, 0.198, 2.5), (0.430, 0.074, 0.062, 0.182, 2.4), (0.446, 0.060, 0.055, 0.122, 2.2), (0.458, 0.050, 0.048, 0.076, 2.1), (0.468, 0.044, 0.042, 0.060, 2.0)]


def jacket_row(u):
    for a, b in zip(JACKET_ROWS, JACKET_ROWS[1:]):
        if a[0] <= u <= b[0]:
            f = (u - a[0]) / (b[0] - a[0])
            return tuple(a[k] + (b[k] - a[k]) * f for k in range(5))
    raise ValueError(u)


def trunk(out):
    out.append(tag(K.stack("pelvis-trousers", "pelvis", [(-0.112, 0.030, 0.050, 0.060, 2.1), (-0.095, 0.052, 0.090, 0.118, 2.2), (-0.062, 0.066, 0.110, 0.156, 2.3), (-0.025, 0.074, 0.116, 0.168, 2.3), (0.02, 0.078, 0.108, 0.166, 2.3), (0.060, 0.080, 0.094, 0.156, 2.3), (0.092, 0.076, 0.080, 0.144, 2.2)], TROUSER, 0.2), "trousers", "pelvis-trousers"))
    out.append(tag(K.stack("jacket", "torso", JACKET_ROWS, JACKET, 0.3), "jacket", "jacket"))
    zip_rows = [(u, f + 0.003, -f + 0.006, 0.006, 4.0) for u, f, b, hw, e in JACKET_ROWS[1:-2]]
    out.append(tag(K.stack("jacket-zip", "torso", zip_rows, JACKET, 0.1, role="detail", texture=True), "jacket", "jacket-zip"))
    band = [(u, f + 0.003, b + 0.003, hw + 0.003, e) for u, f, b, hw, e in (jacket_row(0.226), jacket_row(0.240), jacket_row(0.254))]
    out.append(tag(K.stack("jacket-reflective", "torso", band, JACKET, 0.9, role="detail", texture=True), "jacket", "jacket-reflective"))
    out.append(tag(K.stack("neck", "neck", [(-0.03, 0.068, 0.020, 0.050), (0.0, 0.074, 0.010, 0.052), (0.04, 0.078, 0.004, 0.052), (0.08, 0.080, 0.006, 0.054), (0.11, 0.078, 0.012, 0.056), (0.13, 0.070, 0.020, 0.050)], SKIN, 0.84), "face-skin", "neck"))
    out.append(tag(K.stack("collar", "neck", [(-0.035, 0.078, 0.060, 0.080, 2.2), (-0.01, 0.084, 0.034, 0.066, 2.2), (0.025, 0.086, 0.022, 0.062, 2.1), (0.05, 0.086, 0.020, 0.062, 2.1), (0.058, 0.084, 0.018, 0.060, 2.1)], JACKET, 0.3), "jacket", "collar"))


HEAD = [(-0.080, 0.090, -0.060, 0.016, 2.0), (-0.072, 0.100, -0.040, 0.034, 2.0), (-0.055, 0.106, -0.012, 0.048, 2.1), (-0.030, 0.110, 0.028, 0.058, 2.1), (-0.005, 0.111, 0.062, 0.066, 2.1), (0.025, 0.110, 0.084, 0.071, 2.0), (0.055, 0.108, 0.094, 0.074, 2.0), (0.085, 0.100, 0.094, 0.073, 2.0), (0.112, 0.084, 0.084, 0.066, 2.0), (0.134, 0.058, 0.062, 0.052, 2.0), (0.150, 0.030, 0.034, 0.030, 2.0), (0.158, 0.006, 0.008, 0.007, 2.0)]
HELMET_TILT = 14.0
HELMET_C0 = (0.050, 0.004)
HELMET_ROWS = [(0.0, 0.126, 0.098, 2.3), (0.006, 0.131, 0.103, 2.3), (0.022, 0.134, 0.106, 2.2), (0.050, 0.128, 0.103, 2.1), (0.080, 0.112, 0.091, 2.0), (0.105, 0.084, 0.070, 2.0), (0.122, 0.050, 0.042, 2.0), (0.131, 0.014, 0.012, 2.0)]


def helmet_sections(rows, grow=0.0):
    t = math.radians(HELMET_TILT)
    n = [math.cos(t), math.sin(t), 0.0]
    d = [math.sin(t), -math.cos(t), 0.0]
    q = K.quat(K.W, d)
    return [K.section([HELMET_C0[0] + s * n[0], HELMET_C0[1] + s * n[1], 0], hw + grow, hd + grow, e, q) for s, hd, hw, e in rows]


def helmet_point(s, side, w, lift=0.002):
    """⛑️ A point of the helmet shell at height s along its axis, on the front (side +1) or back (-1), at lateral w."""
    t = math.radians(HELMET_TILT)
    n = (math.cos(t), math.sin(t))
    d = (math.sin(t), -math.cos(t))
    rows = HELMET_ROWS
    for a, b in zip(rows, rows[1:]):
        if a[0] <= s <= b[0]:
            f = (s - a[0]) / (b[0] - a[0])
            hd, hw = a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f
    reach = hd * math.sqrt(max(0.0, 1 - (w / hw) ** 2)) + lift
    return (HELMET_C0[0] + s * n[0] + side * reach * d[0], HELMET_C0[1] + s * n[1] + side * reach * d[1])


def vent_path(w):
    return [helmet_point(0.040, 1, w), helmet_point(0.090, 1, w), helmet_point(0.118, 1, w), helmet_point(0.126, -1, w), helmet_point(0.105, -1, w), helmet_point(0.060, -1, w)]


def head(out):
    out.append(tag(K.stack("head", "head", HEAD, SKIN, 0.86), "face-skin", "head"))
    out.append(tag(K.stack("nose", "head", [(0.044, 0.108, -0.100, 0.006), (0.030, 0.114, -0.100, 0.008), (0.012, 0.124, -0.102, 0.010), (0.003, 0.122, -0.102, 0.012), (-0.002, 0.115, -0.102, 0.010)], SKIN, 0.86, role="detail"), "face-skin", "head"))
    out.append(tag(K.stack("lips", "head", [(-0.032, 0.110, -0.098, 0.016, 2.0), (-0.024, 0.113, -0.100, 0.019, 2.0), (-0.016, 0.112, -0.100, 0.017, 2.0)], SKIN, 0.6, role="detail"), "face-skin", "head"))
    for s, n in SIDES:
        out.append(tag(K.ellipsoid(f"ear-{n}", "head", [0.030, 0.017, 0.008], [0.016, 0.010, s * 0.072], SKIN, 0.8, role="detail", rotation=K.rot_w(-12)), "face-skin", "ear"))
        out.append(tag(K.ellipsoid(f"eye-{n}", "head", [0.006, 0.004, 0.011], [0.044, -0.096, s * 0.033], SKIN, 0.2, role="detail"), "face-skin", "eye"))
    out.append(tag(K.stack("hair-nape", "head", [(-0.004, -0.040, 0.074, 0.054, 2.2), (0.018, -0.040, 0.094, 0.066, 2.2), (0.042, -0.040, 0.103, 0.074, 2.1), (0.064, -0.040, 0.104, 0.076, 2.0)], HAIR, 0.25), "hair", "hair-tail"))
    out.append(tag(K.tube("hair-tail", "head", [([0.040, 0.092, 0], 0.020, 0.020), ([0.024, 0.118, 0], 0.024, 0.022), ([-0.012, 0.132, 0], 0.026, 0.023), ([-0.062, 0.132, 0], 0.022, 0.020), ([-0.110, 0.124, 0], 0.016, 0.015), ([-0.142, 0.118, 0], 0.008, 0.008)], HAIR, 0.25), "hair", "hair-tail"))
    out.append(tag(K.loft("helmet", "head", helmet_sections(HELMET_ROWS), HELMET, 0.82, caps=(False, True)), "helmet", "helmet"))
    for k, w in enumerate((0.0, 0.042, -0.042)):
        pts = [([q[0], q[1], w], 0.0040, 0.0040) for q in vent_path(w)]
        out.append(tag(K.tube(f"helmet-vent-{k}", "head", pts, HELMET, 0.2, hint=K.W, role="detail", texture=True), "helmet", "helmet-vent-1"))
    for s, n in SIDES:
        out.append(tag(K.tube(f"helmet-strap-{n}", "head", [([0.052, -0.050, s * 0.098], 0.0075, 0.0012, 4.0), ([0.018, -0.026, s * 0.077], 0.0075, 0.0012, 4.0), ([-0.012, -0.008, s * 0.068], 0.0075, 0.0012, 4.0), ([-0.050, -0.040, s * 0.058], 0.0075, 0.0012, 4.0), ([-0.075, -0.070, s * 0.030], 0.0075, 0.0012, 4.0), ([-0.083, -0.080, 0.0], 0.0075, 0.0012, 4.0)], HELMET, 0.2, hint=K.V, role="detail", texture=True), "helmet", "helmet-strap"))


UPPER = [(-0.040, 0.0, -0.020, 0.030, 0.028), (-0.015, 0.002, -0.004, 0.042, 0.040), (0.020, 0.004, 0.006, 0.051, 0.048), (0.06, 0.004, 0.006, 0.050, 0.047), (0.11, 0.003, 0.004, 0.047, 0.045), (0.17, 0.004, 0.0, 0.045, 0.042), (0.23, 0.002, 0.0, 0.042, 0.040), (0.29, 0.0, 0.0, 0.040, 0.043), (0.325, -0.002, 0.0, 0.038, 0.041), (0.355, -0.002, 0.0, 0.028, 0.030)]
FORE = [(-0.045, 0.0, 0.0, 0.030, 0.032), (-0.012, -0.002, 0.0, 0.038, 0.041), (0.03, 0.002, 0.002, 0.042, 0.044), (0.08, 0.001, 0.002, 0.039, 0.042), (0.135, 0.0, 0.0, 0.033, 0.039), (0.19, 0.0, 0.0, 0.027, 0.035), (0.222, 0.0, 0.0, 0.025, 0.033), (0.232, 0.0, 0.0, 0.029, 0.037), (0.252, 0.0, 0.0, 0.028, 0.036), (0.262, 0.0, 0.0, 0.022, 0.030)]
PALM = [(-0.022, 0.002, 0.0, 0.024, 0.033, 2.2), (0.0, 0.002, 0.0, 0.020, 0.036, 2.4), (0.03, 0.001, 0.0, 0.017, 0.041, 2.6), (0.06, 0.0, 0.0, 0.016, 0.043, 2.6), (0.088, -0.001, 0.0, 0.0145, 0.042, 2.6), (0.100, -0.004, 0.0, 0.010, 0.039, 2.4)]
FINGERS = [(-0.027, 0.0098, 205), (-0.0085, 0.0102, 212), (0.0095, 0.0098, 208), (0.0265, 0.0088, 196)]


def grip_frame(F, side):
    """✋ The bar axis (point and unit direction) of a grip in the posed hand's frame, with the 'up' (dorsal) and 'ahead' directions around it."""
    f = F[f"hand-{'l' if side > 0 else 'r'}"]
    d = local_dir(f, R.grip_axis(side))
    c = [R.GRIP_LOCAL[0], R.GRIP_LOCAL[1], 0.0]
    up = [0.0, 1.0, 0.0]
    k = sum(up[i] * d[i] for i in range(3))
    up = K.unit([up[i] - k * d[i] for i in range(3)])
    ahead = K.cross(up, d)
    if ahead[0] < 0:
        ahead = [-q for q in ahead]
    return c, d, up, ahead


def finger(i, bone, c, d, up, ahead, t, r, end, material, tone):
    rr = R.GRIP_R + r + 0.0012
    secs = []
    steps = [-12, 30, 75, 120, 165, end]
    for k, ph in enumerate(steps):
        a = math.radians(ph)
        radial = [up[q] * math.cos(a) + ahead[q] * math.sin(a) for q in range(3)]
        tangent = [-up[q] * math.sin(a) + ahead[q] * math.cos(a) for q in range(3)]
        p = [c[q] + d[q] * t + radial[q] * rr for q in range(3)]
        rad = r * (1.12 if k == 0 else 1.0 if k < 4 else 0.92 if k == 4 else 0.78)
        X = K.unit(d)
        Y = K.cross(tangent, X)
        secs.append(K.section(p, rad, rad, 2.0, K.quat(X, Y)))
    return K.loft(i, bone, secs, material, tone, role="detail")


def arms(out, F):
    for s, n in SIDES:
        tone = TONE[f"jacket-{n}"]
        out.append(tag(K.limb(f"sleeve-{n}", f"arm-{n}", UPPER, JACKET, tone, s), "jacket", f"sleeve-{n}"))
        out.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", FORE, JACKET, tone, s), "jacket", f"forearm-{n}"))
        g = TONE[f"glove-{n}"]
        out.append(tag(K.limb(f"glove-{n}", f"hand-{n}", PALM, GLOVE, g, s), "gloves", f"glove-{n}"))
        c, d, up, ahead = grip_frame(F, s)
        medial = -s
        for k, (t, r, end) in enumerate(FINGERS):
            out.append(tag(finger(f"fingers-{n}-{k}", f"hand-{n}", c, d, up, ahead, t, r, end, GLOVE, round(g * 0.55, 4)), "gloves", f"glove-{n}"))
        out.append(tag(K.tube(f"glove-thumb-{n}", f"hand-{n}", [([0.004, -0.010, medial * 0.030], 0.013, 0.012), ([0.030, -0.026, medial * 0.040], 0.0115, 0.0105), ([0.058, -0.044, medial * 0.040], 0.0100, 0.0095), ([0.080, -0.052, medial * 0.030], 0.0090, 0.0085), ([0.096, -0.050, medial * 0.018], 0.0075, 0.0072)], GLOVE, g, hint=K.U, role="detail"), "gloves", f"glove-thumb-{n}"))


THIGH = [(-0.075, 0.0, -0.01, 0.060, 0.058), (-0.035, 0.006, 0.010, 0.084, 0.083), (0.02, 0.010, 0.012, 0.088, 0.084), (0.09, 0.012, 0.008, 0.085, 0.079), (0.17, 0.011, 0.004, 0.079, 0.072), (0.25, 0.010, 0.0, 0.070, 0.064), (0.32, 0.010, -0.004, 0.060, 0.057), (0.37, 0.012, -0.002, 0.055, 0.053), (0.404, 0.012, 0.0, 0.053, 0.051), (0.432, 0.008, 0.0, 0.050, 0.049), (0.455, 0.004, 0.0, 0.042, 0.042), (0.470, 0.0, 0.0, 0.026, 0.026)]
SHANK = [(-0.050, 0.004, 0.0, 0.034, 0.034), (-0.020, 0.006, 0.0, 0.045, 0.044), (0.020, 0.0, 0.0, 0.050, 0.048), (0.055, -0.008, 0.0, 0.054, 0.050), (0.10, -0.015, -0.002, 0.058, 0.051), (0.165, -0.015, -0.003, 0.055, 0.048), (0.24, -0.009, -0.001, 0.046, 0.042), (0.31, -0.004, 0.0, 0.040, 0.037), (0.37, 0.0, 0.0, 0.037, 0.035), (0.405, 0.002, 0.0, 0.037, 0.035), (0.42, 0.002, 0.0, 0.034, 0.033), (0.445, 0.002, 0.0, 0.028, 0.028)]
CLIP = [(0.300, -0.004, 0.0, 0.0445, 0.0415), (0.312, -0.003, 0.0, 0.0445, 0.0410), (0.334, -0.002, 0.0, 0.0435, 0.0400), (0.346, -0.002, 0.0, 0.0425, 0.0395)]
SOLE = [(-0.068, 0.020, 0.020), (-0.060, 0.030, 0.030), (-0.040, 0.034, 0.034), (0.000, 0.034, 0.033), (0.040, 0.036, 0.033), (0.080, 0.044, 0.038), (0.110, 0.048, 0.042), (0.135, 0.048, 0.043)]
SOLE_TOE = [(0.120, 0.048, 0.043), (0.150, 0.046, 0.041), (0.175, 0.040, 0.035), (0.195, 0.028, 0.025), (0.206, 0.012, 0.010)]
UPPER_SHOE = [(-0.064, -0.048, 2.2), (-0.058, -0.008, 2.4), (-0.035, 0.000, 2.5), (-0.005, -0.004, 2.5), (0.025, -0.020, 2.5), (0.055, -0.031, 2.5), (0.085, -0.040, 2.5), (0.115, -0.049, 2.5), (0.132, -0.052, 2.5)]
TOE_BOX = [(0.120, -0.051, 2.5), (0.145, -0.054, 2.5), (0.168, -0.058, 2.4), (0.186, -0.064, 2.2), (0.198, -0.069, 2.0)]
SOLE_T = 0.020


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
    bottom = -R.ANKLE_SHOE + SOLE_T - 0.004
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
        tone = TONE[f"trouser-{n}"]
        out.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", THIGH, TROUSER, tone, s), "trousers", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-{n}", f"shank-{n}", SHANK, TROUSER, tone, s), "trousers", f"shank-{n}"))
        out.append(tag(K.limb(f"trouser-clip-{n}", f"shank-{n}", CLIP, TROUSER, 0.75, s, role="detail"), "trousers", f"trouser-clip-{n}"))
        st = TONE[f"shoe-{n}"]
        out.append(tag(K.loft(f"shoe-rear-{n}", f"foot-{n}", shoe_sections(UPPER_SHOE, SOLE, s, foot_local, 0.002), SHOE, st), "shoes", f"shoe-rear-{n}"))
        sole = K.extrude(f"shoe-sole-{n}", f"foot-{n}", [(z, x) for z, x in sole_outline(SOLE[:-1] + [(0.128, 0.048, 0.043)], s)], "zx", -R.ANKLE_SHOE, -R.ANKLE_SHOE + SOLE_T, SHOE, 0.95)
        sole["transform"] = {"rotation": K.rot_w(R.FOOT_DROP)}
        out.append(tag(sole, "shoes", f"shoe-sole-{n}"))
        out.append(tag(K.loft(f"shoe-toe-{n}", f"toe-{n}", shoe_sections(TOE_BOX, SOLE_TOE, s, toe_local, 0.002), SHOE, st), "shoes", f"shoe-toe-{n}"))
        tsole = K.extrude(f"shoe-toe-sole-{n}", f"toe-{n}", [(z, x - R.ANKLE_TO_BALL_X) for z, x in sole_outline(SOLE_TOE, s)], "zx", -R.BALL_SHOE, -R.BALL_SHOE + SOLE_T, SHOE, 0.95)
        out.append(tag(tsole, "shoes", f"shoe-toe-{n}"))


def free_hint(pts, hint):
    """🧷 A section reference direction that is never along the path: the given hint, else lateral, else up."""
    for h in ([hint] if hint else []) + [K.W, K.V, K.U]:
        if all(abs(sum(h[q] * K.unit([b[q] - a[q] for q in range(3)])[q] for q in range(3))) < 0.95 for a, b in zip(pts, pts[1:])):
            return h
    return K.U


def path_tube(i, bone, pts, r0, r1, material, tone, role="body", hint=None, exponent=2.0, **extra):
    hint = free_hint(pts, hint)
    n = len(pts)
    rows = []
    for k, p in enumerate(pts):
        f = k / (n - 1)
        r = r0 + (r1 - r0) * f
        rr = r if not isinstance(r, tuple) else r
        rows.append((list(p), rr, rr, exponent))
    return K.tube(i, bone, rows, material, tone, hint=hint, role=role, **extra)


def oval_tube(i, bone, pts, a, b, material, tone, role="body", hint=None, **extra):
    """🥖 A tube along a path with an oval section: a across (along the hint), b in the other direction; a and b may be lists per point."""
    hint = free_hint(pts, hint)
    rows = []
    for k, p in enumerate(pts):
        aa = a[k] if isinstance(a, (list, tuple)) else a
        bb = b[k] if isinstance(b, (list, tuple)) else b
        rows.append((list(p), aa, bb))
    return K.tube(i, bone, rows, material, tone, hint=hint, role=role, **extra)


def revolve_axis(i, bone, profile, p0, p1, material, tone, role="body", **extra):
    ax = K.unit([p1[q] - p0[q] for q in range(3)])
    return K.revolve(i, bone, profile, p0, ax, material, tone, role, **extra)


def steer(s, z=0.0):
    p = R.steer_point(s)
    return [p[0], p[1], z]


def seat(L, z=0.0):
    p = R.seat_point(L)
    return [p[0], p[1], z]


def bicycle(out):
    bb = [R.BB[0], R.BB[1], 0.0]
    fl = R.FORK_LEN
    out.append(tag(revolve_axis("tube-head", "frame", [(0.0, 0.0), (0.0, 0.0215), (0.012, 0.0215), (0.016, 0.0235), (0.030, 0.0235), (0.034, 0.0215), (R.HT_LEN - 0.034, 0.0215), (R.HT_LEN - 0.030, 0.0235), (R.HT_LEN - 0.016, 0.0235), (R.HT_LEN - 0.012, 0.0215), (R.HT_LEN, 0.0215), (R.HT_LEN, 0.0)], steer(fl + 0.012), steer(fl + 0.013), STEEL, 0.2), "bike-metal", "tube-head"))
    out.append(tag(revolve_axis("headset", "frame", [(-0.012, 0.0), (-0.012, 0.0225), (0.0, 0.0225), (0.0, 0.0), ], steer(fl + 0.012), steer(fl + 0.013), STEEL, 0.15, role="detail"), "bike-metal", "tube-head"))
    out.append(tag(revolve_axis("steerer", "frame", [(0.0, 0.0), (0.0, 0.0175), (0.012, 0.0175), (0.012, 0.0143), (0.068, 0.0143), (0.068, 0.0), ], steer(fl + 0.012 + R.HT_LEN), steer(fl + 0.013 + R.HT_LEN), STEEL, 0.15), "bike-metal", "stem"))
    st = R.STEERER_TOP
    out.append(tag(oval_tube("stem", "frame", [[st[0], st[1] - 0.030, 0.0], [st[0] + 0.004, st[1] + 0.006, 0.0], [st[0] + 0.030, st[1] + 0.046, 0.0], [R.CLAMP[0] - 0.012, R.CLAMP[1] - 0.010, 0.0]], [0.017, 0.0175, 0.015, 0.014], [0.017, 0.0175, 0.016, 0.015], STEEL, 0.2), "bike-metal", "stem"))
    out.append(tag(revolve_axis("bar-clamp", "frame", [(-0.022, 0.0), (-0.022, 0.0165), (0.022, 0.0165), (0.022, 0.0)], [R.CLAMP[0], R.CLAMP[1], 0.0], [R.CLAMP[0], R.CLAMP[1], 1.0], STEEL, 0.2), "bike-metal", "stem"))
    for s, n in SIDES:
        ax = R.grip_axis(s)
        g = R.grip_centre(s)
        gi = [g[q] - ax[q] * R.GRIP_LEN / 2 for q in range(3)]
        go = [g[q] + ax[q] * (R.GRIP_LEN / 2 + 0.006) for q in range(3)]
        bend0 = [R.CLAMP[0], R.CLAMP[1], s * 0.080]
        bend1 = [R.CLAMP[0] - 0.012, R.CLAMP[1] - 0.003, s * 0.135]
        pre = [gi[q] - ax[q] * 0.040 for q in range(3)]
        out.append(tag(path_tube(f"handlebar-{n}", "frame", [[R.CLAMP[0], R.CLAMP[1], 0.0], bend0, bend1, pre, gi, go], 0.0127, R.BAR_R, STEEL, 0.2, hint=K.V), "bike-metal", "handlebar"))
        out.append(tag(revolve_axis(f"grip-{n}", "frame", [(-0.004, 0.0), (-0.004, 0.0205), (0.006, 0.0205), (0.010, R.GRIP_R), (R.GRIP_LEN - 0.004, R.GRIP_R + 0.0015), (R.GRIP_LEN, 0.019), (R.GRIP_LEN + 0.004, 0.019), (R.GRIP_LEN + 0.004, 0.0)], gi, [gi[q] + ax[q] for q in range(3)], RUBBER, 0.1), "bike-metal", "grip"))
        perch = [gi[q] - ax[q] * 0.010 for q in range(3)]
        lever = [[gi[q] + ax[q] * t + (0.034, -0.008, 0.0)[q] for q in range(3)] for t in (0.004, 0.045, 0.085)]
        out.append(tag(oval_tube(f"brake-lever-{n}", "frame", [perch, [perch[0] + 0.024, perch[1] - 0.004, perch[2]]] + lever, 0.0045, 0.0085, STEEL, 0.35, role="detail", hint=K.V), "bike-metal", "handlebar"))
    bell = [R.CLAMP[0] - 0.006, R.CLAMP[1] + 0.012, 0.105]
    out.append(tag(K.revolve("bell", "frame", [(0.0, 0.0), (0.0, 0.026), (0.006, 0.024), (0.016, 0.016), (0.020, 0.006), (0.021, 0.0)], bell, K.V, STEEL, 0.75, role="detail"), "bike-metal", "handlebar"))
    crown = R.CROWN
    out.append(tag(K.box("fork-crown", "frame", [0.030, 0.026, 0.118], [crown[0], crown[1] - 0.004, 0.0], STEEL, 0.2, corner=0.009, rotation=K.rot_w(90 - R.HEAD_ANGLE)), "bike-metal", "fork"))
    for s, n in SIDES:
        a = steer(fl - 0.010, s * 0.050)
        pts = [a, [0.927, 0.560, s * 0.051], [0.966, 0.452, s * 0.051], [1.008, 0.385, s * 0.051], [R.FRONT[0], R.FRONT[1], s * 0.051]]
        out.append(tag(oval_tube(f"fork-{n}", "frame", pts, [0.0145, 0.0128, 0.0112, 0.0098, 0.009], [0.0115, 0.0104, 0.0094, 0.0085, 0.008], STEEL, 0.2, hint=K.W), "bike-metal", "fork"))
        out.append(tag(K.box(f"fork-dropout-{n}", "frame", [0.030, 0.022, 0.008], [R.FRONT[0] + 0.004, R.FRONT[1] + 0.004, s * 0.051], STEEL, 0.2, corner=0.0029), "bike-metal", "fork"))
    head_lo = steer(fl + 0.045)
    wave = [[head_lo[0] + 0.006, head_lo[1], 0.0], [0.835, 0.665, 0.0], [0.775, 0.555, 0.0], [0.700, 0.470, 0.0], [0.610, 0.418, 0.0], [0.520, 0.392, 0.0], seat(0.085)]
    out.append(tag(oval_tube("tube-down", "frame", wave, [0.020, 0.020, 0.021, 0.022, 0.022, 0.021, 0.020], [0.026, 0.027, 0.028, 0.028, 0.027, 0.025, 0.022], STEEL, 0.2), "bike-metal", "tube-down"))
    out.append(tag(oval_tube("tube-seat", "frame", [seat(-0.015), seat(0.06), seat(0.30), seat(R.ST_LEN - 0.012)], 0.0165, 0.0165, STEEL, 0.2), "bike-metal", "tube-seat"))
    out.append(tag(revolve_axis("seat-collar", "frame", [(0.0, 0.0), (0.0, 0.0195), (0.020, 0.0195), (0.020, 0.0)], seat(R.ST_LEN - 0.016), seat(R.ST_LEN), STEEL, 0.35, role="detail"), "bike-metal", "tube-seat"))
    out.append(tag(revolve_axis("seatpost", "frame", [(0.0, 0.0), (0.0, 0.0136), (R.POST_LEN - R.ST_LEN + 0.010, 0.0136), (R.POST_LEN - R.ST_LEN + 0.010, 0.0)], seat(R.ST_LEN - 0.006), seat(R.ST_LEN + 0.3), STEEL, 0.6), "bike-metal", "seatpost"))
    out.append(tag(K.revolve("bottom-bracket", "frame", [(-0.036, 0.0), (-0.036, 0.0215), (0.036, 0.0215), (0.036, 0.0)], bb, K.W, STEEL, 0.2), "bike-metal", "bottom-bracket"))
    drop = [R.REAR[0], R.REAR[1], 0.0]
    for s, n in SIDES:
        out.append(tag(oval_tube(f"tube-chainstay-{n}", "frame", [[bb[0] - 0.012, bb[1] + 0.004, s * 0.026], [bb[0] - 0.060, bb[1] + 0.008, s * 0.040], [0.20, 0.322, s * 0.058], [0.05, 0.337, s * 0.064], [0.012, 0.340, s * 0.066]], [0.0125, 0.0120, 0.0105, 0.0090, 0.0085], [0.0095, 0.0092, 0.0085, 0.0075, 0.0072], STEEL, 0.2, hint=K.V), "bike-metal", "tube-chainstay"))
        top = seat(R.ST_LEN - 0.045, s * 0.018)
        out.append(tag(oval_tube(f"tube-seatstay-{n}", "frame", [top, [top[0] - 0.040, top[1] - 0.060, s * 0.036], [0.08, 0.455, s * 0.060], [0.012, 0.352, s * 0.066]], [0.0085, 0.0082, 0.0076, 0.0072], [0.0085, 0.0082, 0.0076, 0.0072], STEEL, 0.2), "bike-metal", "tube-seatstay"))
        out.append(tag(K.box(f"dropout-{n}", "frame", [0.048, 0.040, 0.008], [0.006, 0.342, s * 0.0660], STEEL, 0.2, corner=0.0039), "bike-metal", "tube-chainstay"))
    out.append(tag(oval_tube("bridge-seatstay", "frame", [[0.20, 0.580, 0.040], [0.20, 0.586, 0.0], [0.20, 0.580, -0.040]], 0.0075, 0.0075, STEEL, 0.2, hint=K.U, role="detail"), "bike-metal", "tube-seatstay"))
    fender(out)
    carrier(out)
    drivetrain(out)
    saddle(out)
    lamps(out)
    stand(out)
    brakes(out)


def fender(out):
    for n, c, a0, sweep, w in (("front", R.FRONT, -38.0, 205.0, 0.028), ("rear", R.REAR, 18.0, 207.0, 0.030)):
        r = R.FENDER_R
        prof = [(-w, r - 0.012), (-w + 0.004, r - 0.002), (-w + 0.012, r + 0.003), (0.0, r + 0.005), (w - 0.012, r + 0.003), (w - 0.004, r - 0.002), (w, r - 0.012), (w - 0.0015, r - 0.012), (w - 0.0045, r - 0.0035), (w - 0.012, r + 0.0015), (0.0, r + 0.0035), (-w + 0.012, r + 0.0015), (-w + 0.0045, r - 0.0035), (-w + 0.0015, r - 0.012), (-w, r - 0.012)]
        s = K.revolve(f"{n}-fender", "frame", prof, [c[0], c[1], 0.0], K.W, STEEL, 0.3, sweepRad=round(math.radians(sweep), 6))
        s["transform"] = {"rotation": K.rot_w(a0), "translation": [K.r5(c[0] - (c[0] * math.cos(math.radians(a0)) - c[1] * math.sin(math.radians(a0)))), K.r5(c[1] - (c[0] * math.sin(math.radians(a0)) + c[1] * math.cos(math.radians(a0)))), 0.0]}
        out.append(tag(s, "bike-metal", f"{n}-fender"))
        for side in (1, -1):
            a = math.radians(a0 + (25 if n == "front" else sweep - 25))
            p = [c[0] + (r + 0.002) * math.cos(a), c[1] + (r + 0.002) * math.sin(a), side * (w + 0.004)]
            out.append(tag(path_tube(f"{n}-fender-stay-{'l' if side > 0 else 'r'}", "frame", [p, [c[0] + 0.02 * math.cos(a), c[1] + 0.02 * math.sin(a), side * 0.058]], 0.0024, 0.0024, STEEL, 0.25, role="detail", slender=True), "bike-metal", f"{n}-fender"))


def carrier(out):
    y = R.CARRIER_Y
    xs = (-0.115, 0.215)
    for s, n in SIDES:
        out.append(tag(path_tube(f"carrier-rail-{n}", "frame", [[xs[0], y, s * 0.068], [xs[1], y, s * 0.068]], 0.005, 0.005, STEEL, 0.25, slender=True), "bike-metal", "carrier"))
        out.append(tag(path_tube(f"carrier-leg-{n}", "frame", [[-0.075, y - 0.004, s * 0.070], [-0.030, 0.560, s * 0.074], [0.008, 0.352, s * 0.072]], 0.0050, 0.0050, STEEL, 0.25, slender=True), "bike-metal", "carrier"))
        out.append(tag(path_tube(f"carrier-leg2-{n}", "frame", [[0.040, y - 0.004, s * 0.070], [0.025, 0.560, s * 0.073], [0.010, 0.355, s * 0.071]], 0.0050, 0.0050, STEEL, 0.25, slender=True), "bike-metal", "carrier"))
        out.append(tag(path_tube(f"carrier-stay-{n}", "frame", [[0.205, y, s * 0.068], [0.250, 0.770, s * 0.030], seat(R.ST_LEN - 0.060, s * 0.016)], 0.004, 0.004, STEEL, 0.25), "bike-metal", "carrier"))
    for k, x in enumerate((-0.115, -0.03, 0.06, 0.15, 0.215)):
        out.append(tag(path_tube(f"carrier-bar-{k}", "frame", [[x, y, 0.072], [x, y, -0.072]], 0.0042, 0.0042, STEEL, 0.25), "bike-metal", "carrier"))
    out.append(tag(K.box("tail-lamp", "frame", [0.026, 0.040, 0.070], [-0.128, y - 0.022, 0.0], STEEL, 0.55, corner=0.006, role="detail"), "bike-metal", "carrier"))


def drivetrain(out):
    for s, n in SIDES:
        out.append(tag(K.extrude(f"crank-arm-{n}", f"crank-{n}", [(-0.012, 0.0), (-0.010, 0.016), (0.0, 0.020), (0.150, 0.0125), (0.170, 0.0125), (0.182, 0.0), (0.170, -0.0125), (0.150, -0.0125), (0.0, -0.020), (-0.010, -0.016)], "xy", s * R.CRANK_Z - 0.0065, s * R.CRANK_Z + 0.0065, STEEL, 0.35 if n == "r" else 0.3), "bike-metal", f"crank-arm-{n}"))
        z0, z1 = sorted((s * R.PEDAL_Z0, s * R.PEDAL_Z1))
        out.append(tag(K.extrude(f"pedal-{n}", f"pedal-{n}", [(-0.052, -0.0125), (-0.052, 0.0125), (-0.030, 0.0105), (0.030, 0.0105), (0.052, 0.0125), (0.052, -0.0125), (0.030, -0.0105), (-0.030, -0.0105)], "xy", z0, z1, STEEL, 0.15), "bike-metal", f"pedal-{n}"))
        zs = sorted((s * (R.CRANK_Z + 0.006), s * R.PEDAL_Z0))
        out.append(tag(K.revolve(f"pedal-spindle-{n}", f"pedal-{n}", [(zs[0], 0.0), (zs[0], 0.0075), (zs[1], 0.0075), (zs[1], 0.0)], [0, 0, 0], K.W, STEEL, 0.2, role="detail"), "bike-metal", f"pedal-{n}"))
    out.append(tag(K.revolve("chainring", "crank-r", [(-0.056, 0.0), (-0.056, 0.0805), (-0.052, 0.0835), (-0.048, 0.0805), (-0.048, 0.050), (-0.052, 0.040), (-0.052, 0.0)], [0, 0, 0], K.W, STEEL, 0.55), "bike-metal", "chainring"))
    out.append(tag(K.revolve("bb-spindle", "crank-r", [(-R.CRANK_Z, 0.0), (-R.CRANK_Z, 0.0105), (R.CRANK_Z, 0.0105), (R.CRANK_Z, 0.0)], [0, 0, 0], K.W, STEEL, 0.6, role="detail"), "bike-metal", "bb-spindle"))
    out.append(tag(K.revolve("rear-cog", "rear-wheel", [(-0.058, 0.0), (-0.058, 0.0425), (-0.050, 0.0425), (-0.050, 0.0)], [0, 0, 0], K.W, STEEL, 0.5, role="detail"), "bike-metal", "rear-cog"))
    rb, rc = 0.0805, 0.0410
    bx, by = R.BB
    d = math.hypot(bx - R.REAR[0], by - R.REAR[1])
    phi = math.atan2(by - R.REAR[1], bx - R.REAR[0])
    beta = math.asin((rb - rc) / d)
    for k, sgn in ((0, 1), (1, -1)):
        a = phi + sgn * (math.pi / 2 + beta)
        p0 = [R.REAR[0] + rc * math.cos(a), R.REAR[1] + rc * math.sin(a), -0.052]
        p1 = [bx + rb * math.cos(a), by + rb * math.sin(a), -0.052]
        out.append(tag(oval_tube(f"chain-{'top' if sgn > 0 else 'bottom'}", "frame", [p0, p1], 0.0042, 0.0038, STEEL, 0.2, role="detail", hint=K.W, slender=True), "bike-metal", f"chain-{'top' if sgn > 0 else 'bottom'}"))
    guard = K.arc(bx, by, 0.104, 95, -20, 12) + K.arc(bx, by, 0.091, -20, 95, 12) + [(0.070, 0.389), (0.058, 0.397), (0.070, 0.405)]
    out.append(tag(K.extrude("chain-guard", "frame", guard, "xy", -0.0705, -0.063, STEEL, 0.25), "bike-metal", "chain-guard"))


def saddle(out):
    rows = [(0.160, 0.072, 0.906, 0.018), (0.172, 0.094, 0.918, 0.026), (0.200, 0.102, 0.924, 0.028), (0.240, 0.094, 0.922, 0.027), (0.280, 0.068, 0.918, 0.024), (0.325, 0.040, 0.916, 0.022), (0.370, 0.030, 0.913, 0.020), (0.405, 0.024, 0.909, 0.017), (0.425, 0.012, 0.903, 0.010)]
    secs = [K.section([x, top - h, 0.0], hw, h, 2.6 if k else 2.2, K.ACROSS) for k, (x, hw, top, h) in enumerate(rows)]
    out.append(tag(K.loft("saddle", "frame", secs, SADDLE, 0.15), "saddle", "saddle"))
    for s, n in SIDES:
        out.append(tag(path_tube(f"saddle-rail-{n}", "frame", [[0.190, 0.880, s * 0.030], [0.215, 0.872, s * 0.024], [0.320, 0.872, s * 0.020], [0.390, 0.886, s * 0.012]], 0.0040, 0.0040, STEEL, 0.4, role="detail", slender=True), "saddle", "saddle"))
        out.append(tag(revolve_axis(f"saddle-spring-{n}", "frame", [(0.0, 0.0), (0.0, 0.010), (0.034, 0.012), (0.034, 0.0)], [0.188, 0.858, s * 0.034], [0.188, 0.900, s * 0.034], STEEL, 0.45, role="detail"), "saddle", "saddle"))
    out.append(tag(K.box("saddle-clamp", "frame", [0.030, 0.026, 0.044], [R.POST_TOP[0], R.POST_TOP[1] - 0.004, 0.0], STEEL, 0.5, corner=0.006, role="detail"), "bike-metal", "seatpost"))


def lamps(out):
    p = [R.CROWN[0] + 0.050, R.CROWN[1] + 0.020, 0.0]
    out.append(tag(K.revolve("headlamp", "frame", [(-0.032, 0.0), (-0.032, 0.018), (-0.010, 0.024), (0.004, 0.025), (0.008, 0.023), (0.008, 0.0)], p, K.U, STEEL, 0.7, role="detail"), "bike-metal", "headlamp"))
    out.append(tag(path_tube("headlamp-bracket", "frame", [[R.CROWN[0] + 0.004, R.CROWN[1] + 0.012, 0.0], [p[0] - 0.030, p[1] - 0.008, 0.0]], 0.0045, 0.0045, STEEL, 0.25, role="detail"), "bike-metal", "headlamp"))


def stand(out):
    out.append(tag(path_tube("kickstand", "frame", [[0.392, 0.296, 0.050], [0.300, 0.306, 0.072], [0.140, 0.322, 0.084]], 0.0085, 0.0075, STEEL, 0.25, role="detail"), "bike-metal", "kickstand"))


def brakes(out):
    for s, n in SIDES:
        f = [R.CROWN[0] + 0.020, R.CROWN[1] - 0.110, s * 0.058]
        out.append(tag(path_tube(f"brake-front-{n}", "frame", [[f[0] - 0.006, f[1] - 0.012, f[2]], [f[0] + 0.004, f[1] + 0.075, s * 0.040]], 0.0045, 0.0045, STEEL, 0.4, role="detail"), "bike-metal", "fork"))
        r = [0.150, 0.640 - 0.050, s * 0.058]
        out.append(tag(path_tube(f"brake-rear-{n}", "frame", [[0.140, 0.610, s * 0.060], [0.200, 0.672, s * 0.040]], 0.0045, 0.0045, STEEL, 0.4, role="detail"), "bike-metal", "tube-seatstay"))


def wheels(out):
    rc = R.WHEEL_R - R.TYRE_H / 2
    tyre = []
    for k in range(24):
        a = 2 * math.pi * k / 24
        tyre.append((R.TYRE_W / 2 * math.sin(a), rc + R.TYRE_H / 2 * math.cos(a) * (1.0 if math.cos(a) > 0 else 0.92)))
    tyre.append(tyre[0])
    rim = [(-0.012, R.RIM_OUT), (-0.012, R.RIM_OUT - 0.006), (-0.009, R.RIM_IN + 0.004), (-0.004, R.RIM_IN), (0.004, R.RIM_IN), (0.009, R.RIM_IN + 0.004), (0.012, R.RIM_OUT - 0.006), (0.012, R.RIM_OUT), (0.0105, R.RIM_OUT), (0.0105, R.RIM_OUT - 0.004), (-0.0105, R.RIM_OUT - 0.004), (-0.0105, R.RIM_OUT), (-0.012, R.RIM_OUT)]
    for n, hub in (("rear", [(-0.068, 0.0), (-0.068, 0.006), (-0.046, 0.006), (-0.046, 0.032), (-0.040, 0.032), (-0.036, 0.040), (0.036, 0.040), (0.040, 0.032), (0.046, 0.032), (0.046, 0.006), (0.068, 0.006), (0.068, 0.0)]), ("front", [(-0.054, 0.0), (-0.054, 0.006), (-0.040, 0.006), (-0.040, 0.030), (-0.034, 0.030), (-0.030, 0.034), (0.030, 0.034), (0.034, 0.030), (0.040, 0.030), (0.040, 0.006), (0.054, 0.006), (0.054, 0.0)])):
        b = f"{n}-wheel"
        out.append(tag(K.revolve(f"{n}-tyre", b, tyre, [0, 0, 0], K.W, RUBBER, 0.12), "tyres", f"{n}-tyre"))
        out.append(tag(K.revolve(f"{n}-rim", b, rim, [0, 0, 0], K.W, STEEL, 0.75), "bike-metal", f"{n}-rim"))
        out.append(tag(K.revolve(f"{n}-hub", b, hub, [0, 0, 0], K.W, STEEL, 0.6), "bike-metal", f"{n}-hub"))
        fl = 0.032 if n == "rear" else 0.030
        for k, z in enumerate((0.030, -0.030)):
            a0 = math.radians(5 * k)
            p0 = [fl * math.cos(a0), fl * math.sin(a0), z]
            a1 = a0 + math.radians(28 if k == 0 else -28)
            p1 = [(R.RIM_IN + 0.002) * math.cos(a1), (R.RIM_IN + 0.002) * math.sin(a1), 0.0]
            sp = K.capsule(f"{n}-spokes-{k}", b, p0, p1, 0.001, 0.001, STEEL, 0.6, role="texture", slender=True, texture=True)
            sp["repeat"] = {"count": 18, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 18, 7)}
            out.append(tag(sp, "bike-metal", f"{n}-spoke"))


def build():
    F = posed()
    out = []
    legs(out)
    arms(out, F)
    trunk(out)
    head(out)
    bicycle(out)
    wheels(out)
    return out
