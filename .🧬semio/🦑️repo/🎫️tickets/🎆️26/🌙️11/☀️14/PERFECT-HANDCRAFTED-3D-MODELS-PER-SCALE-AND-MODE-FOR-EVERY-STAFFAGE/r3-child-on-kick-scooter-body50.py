"""🛴 Round 3 body of humans/child-on-kick-scooter at 1:50 (smallest feature 7.5 mm): an 8-year-old girl in a zip jacket, leggings, sneakers, helmet and backpack on a 145 mm aluminium kick scooter, every section stated by hand.

`build()` returns the solids with two private keys the assembler strips: `_region` (thermal region) and `_g` (the 2D shape group the solid is drawn in).
"""
import importlib.util, math, os

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-child-on-kick-scooter-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
K = mod("kit")

SKIN, HAIR, HELMET, JACKET, LEGGINGS, SNEAKER, PACK = "skin-child", "hair-blond", "helmet-eps", "zip-jacket", "leggings-knit", "sneaker-textile", "nylon-backpack"
DECK, STEM, GRIP, WHEEL = "alu-deck", "alu-stem", "rubber-grip", "pu-wheel"
HELMET_TILT = 12.0
HELMET_C0 = (0.040, -0.005)
SOLE_L = [(-0.062, 0.0, 0.0), (-0.055, 0.020, 0.018), (-0.040, 0.029, 0.027), (-0.010, 0.032, 0.029), (0.030, 0.036, 0.029), (0.070, 0.043, 0.034), (0.100, 0.045, 0.038)]
TOE_L = [(0.008, 0.045, 0.038), (0.030, 0.040, 0.037), (0.048, 0.030, 0.033), (0.060, 0.016, 0.022), (0.066, 0.004, 0.008)]


def tag(s, region, g):
    s["_region"] = region
    s["_g"] = g
    return s


def lm(x, table):
    for a, b in zip(table, table[1:]):
        if a[0] <= x <= b[0]:
            f = (x - a[0]) / (b[0] - a[0])
            return a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f
    return (table[0][1], table[0][2]) if x < table[0][0] else (table[-1][1], table[-1][2])


def helmet_sections(rows, grow=0.0):
    t = math.radians(HELMET_TILT)
    n = [math.cos(t), math.sin(t), 0.0]
    d = [math.sin(t), -math.cos(t), 0.0]
    q = K.quat(K.W, d)
    return [K.section([HELMET_C0[0] + s * n[0], HELMET_C0[1] + s * n[1], 0], hw + grow, hd + grow, e, q) for s, hd, hw, e in rows]


HELMET_ROWS = [(0.0, 0.117, 0.092, 2.15), (0.004, 0.1195, 0.0945, 2.15), (0.016, 0.121, 0.097, 2.1), (0.045, 0.116, 0.094, 2.0), (0.070, 0.100, 0.083, 2.0), (0.092, 0.070, 0.058, 2.0), (0.104, 0.038, 0.032, 2.0), (0.109, 0.010, 0.009, 2.0)]


def trunk(out):
    out.append(tag(K.stack("seat", "pelvis", [(-0.078, 0.022, 0.030, 0.030, 2.2), (-0.068, 0.040, 0.052, 0.058, 2.3), (-0.045, 0.060, 0.076, 0.098, 2.4), (-0.015, 0.068, 0.084, 0.111, 2.4), (0.020, 0.072, 0.078, 0.108, 2.4), (0.050, 0.074, 0.066, 0.100, 2.3)], LEGGINGS, 0.25), "leggings", "seat"))
    out.append(tag(K.stack("skirt", "pelvis", [(-0.016, 0.082, 0.093, 0.121, 2.4), (-0.008, 0.085, 0.095, 0.124, 2.4), (0.006, 0.084, 0.092, 0.121, 2.4), (0.030, 0.084, 0.082, 0.114, 2.4), (0.052, 0.082, 0.074, 0.108, 2.4), (0.066, 0.070, 0.060, 0.094, 2.35), (0.078, 0.056, 0.048, 0.078, 2.3)], JACKET, 0.66), "jacket", "hip"))
    out.append(tag(K.stack("torso", "torso", [(-0.045, 0.060, 0.050, 0.085, 2.3), (-0.022, 0.086, 0.077, 0.109, 2.35), (0.040, 0.089, 0.071, 0.105, 2.35), (0.100, 0.086, 0.071, 0.105, 2.3), (0.160, 0.083, 0.075, 0.109, 2.3), (0.215, 0.081, 0.080, 0.114, 2.3), (0.262, 0.074, 0.083, 0.120, 2.3), (0.296, 0.062, 0.079, 0.122, 2.2), (0.322, 0.050, 0.068, 0.098, 2.1), (0.342, 0.042, 0.056, 0.070, 2.0), (0.356, 0.036, 0.048, 0.052, 2.0)], JACKET, 0.66), "jacket", "torso"))
    out.append(tag(K.stack("placket", "torso", [(-0.02, 0.0875, -0.0835, 0.009, 4.0), (0.04, 0.0925, -0.0885, 0.009, 4.0), (0.10, 0.0895, -0.0855, 0.009, 4.0), (0.16, 0.0865, -0.0825, 0.009, 4.0), (0.215, 0.0845, -0.0805, 0.009, 4.0), (0.262, 0.0775, -0.0735, 0.009, 4.0), (0.30, 0.0640, -0.0600, 0.009, 4.0)], JACKET, 0.5, role="detail", texture=True), "jacket", "front"))
    for k, u, f in ((0, 0.055, 0.0915), (1, 0.135, 0.0875), (2, 0.215, 0.0845)):
        out.append(tag(K.ellipsoid(f"button-{k}", "torso", [0.009, 0.004, 0.009], [u, -f, 0], JACKET, 0.15, role="detail"), "jacket", f"button-{k}"))
    out.append(tag(K.stack("neck", "neck", [(-0.025, 0.052, 0.036, 0.044), (0.020, 0.056, 0.032, 0.040), (0.060, 0.058, 0.030, 0.039), (0.100, 0.060, 0.034, 0.041), (0.120, 0.050, 0.030, 0.036)], SKIN, 0.85), "face-skin", "neck"))
    out.append(tag(K.stack("collar", "neck", [(-0.030, 0.052, 0.058, 0.066), (-0.005, 0.064, 0.048, 0.056), (0.018, 0.064, 0.042, 0.051), (0.030, 0.062, 0.040, 0.049)], JACKET, 0.6), "jacket", "collar"))


def head(out):
    out.append(tag(K.stack("head", "head", [(-0.073, 0.074, -0.046, 0.016, 2.0), (-0.064, 0.084, -0.026, 0.032, 2.1), (-0.048, 0.091, -0.004, 0.044, 2.1), (-0.030, 0.095, 0.022, 0.052, 2.1), (-0.010, 0.097, 0.050, 0.058, 2.1), (0.012, 0.096, 0.076, 0.062, 2.1), (0.032, 0.094, 0.086, 0.066, 2.0), (0.058, 0.096, 0.088, 0.070, 2.0), (0.086, 0.086, 0.080, 0.066, 2.0), (0.106, 0.064, 0.060, 0.053, 2.0), (0.118, 0.036, 0.034, 0.031, 2.0), (0.124, 0.010, 0.010, 0.009, 2.0)], SKIN, 0.85), "face-skin", "head"))
    out.append(tag(K.stack("nose", "head", [(0.026, 0.097, -0.084, 0.007), (0.008, 0.103, -0.084, 0.009), (-0.008, 0.110, -0.086, 0.012), (-0.016, 0.106, -0.086, 0.013), (-0.022, 0.099, -0.086, 0.010)], SKIN, 0.85, role="detail"), "face-skin", "nose"))
    for s, n in ((1, "l"), (-1, "r")):
        out.append(tag(K.ellipsoid(f"ear-{n}", "head", [0.027, 0.015, 0.0065], [0.013, 0.012, s * 0.066], SKIN, 0.78, role="detail", rotation=K.rot_w(-10)), "face-skin", f"ear-{n}"))
        out.append(tag(K.ellipsoid(f"eye-{n}", "head", [0.0055, 0.004, 0.0095], [0.030, -0.083, s * 0.030], SKIN, 0.2, role="detail"), "face-skin", f"eye-{n}"))
    out.append(tag(K.ellipsoid("mouth", "head", [0.0035, 0.004, 0.017], [-0.036, -0.0915, 0], SKIN, 0.45, role="detail", texture=True), "face-skin", "mouth"))
    out.append(tag(K.stack("hair-nape", "head", [(-0.020, -0.030, 0.062, 0.050), (0.000, -0.022, 0.082, 0.062), (0.020, -0.018, 0.092, 0.068), (0.040, -0.018, 0.094, 0.071), (0.055, -0.018, 0.094, 0.072)], HAIR, 0.3), "hair", "hair-nape"))
    out.append(tag(K.tube("ponytail", "head", [([0.022, 0.088, 0], 0.017, 0.017), ([0.004, 0.106, 0], 0.021, 0.021), ([-0.030, 0.116, 0], 0.022, 0.022), ([-0.075, 0.112, 0], 0.019, 0.019), ([-0.115, 0.102, 0], 0.013, 0.013), ([-0.140, 0.096, 0], 0.006, 0.006)], HAIR, 0.3), "hair", "hair-tail"))
    out.append(tag(K.loft("helmet", "head", helmet_sections(HELMET_ROWS), HELMET, 0.55), "helmet", "helmet"))
    for s, n in ((1, "l"), (-1, "r")):
        out.append(tag(K.tube(f"helmet-strap-front-{n}", "head", [([0.034, -0.030, s * 0.089], 0.0075, 0.001, 4.0), ([0.005, -0.012, s * 0.068], 0.0075, 0.001, 4.0), ([-0.022, 0.0, s * 0.061], 0.0075, 0.001, 4.0)], HELMET, 0.3, hint=K.V, role="detail", texture=True), "helmet", f"helmet-strap-{n}"))
        out.append(tag(K.tube(f"helmet-strap-rear-{n}", "head", [([0.022, 0.046, s * 0.086], 0.0075, 0.001, 4.0), ([0.0, 0.022, s * 0.066], 0.0075, 0.001, 4.0), ([-0.022, 0.0, s * 0.061], 0.0075, 0.001, 4.0)], HELMET, 0.3, hint=K.V, role="detail", texture=True), "helmet", f"helmet-strap-{n}"))
        out.append(tag(K.tube(f"helmet-strap-chin-{n}", "head", [([-0.022, 0.0, s * 0.061], 0.0075, 0.001, 4.0), ([-0.050, -0.022, s * 0.046], 0.0075, 0.001, 4.0), ([-0.072, -0.045, s * 0.022], 0.0075, 0.001, 4.0), ([-0.0775, -0.052, 0.0], 0.0075, 0.001, 4.0)], HELMET, 0.3, hint=K.V, role="detail", texture=True), "helmet", f"helmet-strap-{n}"))


SLEEVE_UP = [(-0.052, 0.0, -0.004, 0.018, 0.016), (-0.040, 0.0, 0.0, 0.034, 0.030), (-0.018, 0.0, 0.004, 0.045, 0.041), (0.012, 0.0, 0.006, 0.047, 0.044), (0.060, 0.001, 0.004, 0.043, 0.040), (0.120, 0.002, 0.002, 0.040, 0.038), (0.180, 0.0, 0.0, 0.037, 0.036), (0.2286, -0.002, 0.0, 0.036, 0.0355), (0.250, -0.002, 0.0, 0.030, 0.030), (0.262, -0.002, 0.0, 0.018, 0.018)]
SLEEVE_FORE = [(-0.032, 0.0, 0.0, 0.015, 0.015), (-0.020, 0.0, 0.0, 0.026, 0.026), (0.0, -0.002, 0.0, 0.0325, 0.0315), (0.040, 0.002, 0.0, 0.0355, 0.0345), (0.090, 0.001, 0.0, 0.032, 0.032), (0.135, 0.0, 0.0, 0.0275, 0.030), (0.160, 0.0, 0.0, 0.0255, 0.0295), (0.168, 0.0, 0.0, 0.0265, 0.0305), (0.176, 0.0, 0.0, 0.025, 0.029)]
PALM = [(-0.010, 0.001, 0.0, 0.0115, 0.022, 2.4), (0.012, 0.001, 0.0, 0.0125, 0.0265, 2.5), (0.040, 0.0, 0.0, 0.0125, 0.0305, 2.6), (0.064, 0.0, 0.0, 0.0115, 0.0305, 2.6), (0.072, -0.001, 0.0, 0.008, 0.029, 2.6)]
FINGERS = [(-0.0215, 0.0068, 170), (-0.0072, 0.0068, 185), (0.0072, 0.0066, 172), (0.0212, 0.0058, 150)]


def finger(i, bone, w, r, end, material, tone):
    cu, cv = R.GRIP_LOCAL
    rr = R.GRIP_R + r + 0.0004
    secs = []
    steps = [5, 40, 80, 120, 155, end]
    for k, ph in enumerate(steps):
        a = math.radians(ph)
        t = [math.cos(a), -math.sin(a), 0.0]
        rad = r * (1.06 if k == 0 else 1.0 if k < 4 else 0.94 if k == 4 else 0.8)
        secs.append(K.section([cu + rr * math.sin(a), cv + rr * math.cos(a), w], rad, rad, 2.0, K.quat(K.W, [-t[1], t[0], 0.0])))
    return K.loft(i, bone, secs, material, tone, role="detail")


def arms(out):
    for s, n in ((1, "l"), (-1, "r")):
        out.append(tag(K.limb(f"sleeve-up-{n}", f"arm-{n}", SLEEVE_UP, JACKET, 0.66, s), "jacket", f"sleeve-up-{n}"))
        out.append(tag(K.limb(f"sleeve-fore-{n}", f"forearm-{n}", SLEEVE_FORE, JACKET, 0.66, s), "jacket", f"sleeve-fore-{n}"))
        out.append(tag(K.limb(f"palm-{n}", f"hand-{n}", PALM, SKIN, 0.8, s), "hands", f"hand-{n}"))
        for k, (w, r, end) in enumerate(FINGERS):
            out.append(tag(finger(f"finger-{n}{k}", f"hand-{n}", -s * w, r, end, SKIN, 0.8), "hands", f"hand-{n}"))
        inner = -s
        out.append(tag(K.tube(f"thumb-{n}", f"hand-{n}", [([0.016, -0.004, inner * 0.026], 0.0095, 0.0095), ([0.034, -0.020, inner * 0.028], 0.0085, 0.0085), ([0.046, -0.041, inner * 0.022], 0.0078, 0.0078), ([0.060, -0.0485, inner * 0.012], 0.0066, 0.0066)], SKIN, 0.8, hint=K.U, role="detail"), "hands", f"thumb-{n}"))


THIGH = [(-0.062, -0.004, 0.0, 0.022, 0.020), (-0.040, -0.005, 0.0, 0.048, 0.040), (-0.010, -0.006, 0.001, 0.064, 0.051), (0.030, -0.004, 0.002, 0.066, 0.056), (0.090, 0.0, 0.002, 0.061, 0.055), (0.160, 0.003, 0.0, 0.054, 0.050), (0.230, 0.004, 0.0, 0.048, 0.046), (0.280, 0.005, 0.0, 0.045, 0.045), (0.302, 0.004, 0.0, 0.044, 0.045), (0.326, 0.002, 0.0, 0.036, 0.037), (0.342, 0.0, 0.0, 0.020, 0.020)]
SHANK = [(-0.036, 0.002, 0.0, 0.016, 0.016), (-0.022, 0.003, 0.0, 0.030, 0.031), (0.0, 0.004, 0.0, 0.0395, 0.0405), (0.030, 0.002, 0.0, 0.0405, 0.0415), (0.085, -0.005, 0.001, 0.043, 0.040), (0.140, -0.004, 0.0, 0.038, 0.035), (0.200, -0.001, 0.0, 0.031, 0.029), (0.245, 0.0, 0.0, 0.0275, 0.0265), (0.268, 0.0, 0.0, 0.027, 0.0265), (0.2934, 0.0, 0.0, 0.0285, 0.029), (0.310, 0.0, 0.0, 0.022, 0.022)]
SHOE = [(-0.060, -0.036, 0.012, 2.2), (-0.052, -0.031, 0.024, 2.4), (-0.030, -0.027, 0.030, 2.5), (0.0, -0.026, 0.031, 2.5), (0.028, -0.024, 0.031, 2.5), (0.060, -0.031, 0.025, 2.5), (0.092, -0.038, 0.019, 2.5), (0.104, -0.040, 0.017, 2.5)]
TOEBOX = [(-0.010, 0.0235, 0.0185, 2.5), (0.012, 0.022, 0.018, 2.5), (0.034, 0.019, 0.016, 2.4), (0.052, 0.015, 0.012, 2.2), (0.063, 0.012, 0.007, 2.0)]
SOLE_T = 0.015


def foot_sections(rows, table, side, inset, shift=0.0):
    a = math.radians(R.FOOT_DIR)
    X = K.W
    Y = [math.sin(a), math.cos(a), 0.0]
    q = K.quat(X, Y)
    secs = []
    for x, yc, hh, e in rows:
        L, M = lm(x + shift, table)
        hw = max((L + M) / 2 - inset, 0.006)
        wc = side * (L - M) / 2
        u, v = R.foot_local(x, yc)
        secs.append(K.section([u, v, wc], hw, hh, e, q))
    return secs


def sole_outline(table, side):
    lat = [(side * L, x) for x, L, M in table]
    med = [(-side * M, x) for x, L, M in reversed(table)]
    pts = lat + med
    out = []
    for p in pts:
        if not out or math.hypot(p[0] - out[-1][0], p[1] - out[-1][1]) > 1e-6:
            out.append(p)
    if math.hypot(out[0][0] - out[-1][0], out[0][1] - out[-1][1]) < 1e-6:
        out.pop()
    return out


def legs(out):
    for s, n in ((1, "l"), (-1, "r")):
        out.append(tag(K.limb(f"thigh-g-{n}", f"thigh-{n}", THIGH, LEGGINGS, 0.25, s), "leggings", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-g-{n}", f"shank-{n}", SHANK, LEGGINGS, 0.25, s), "leggings", f"shank-{n}"))
        out.append(tag(K.loft(f"shoe-rear-{n}", f"foot-{n}", foot_sections(SHOE, SOLE_L, s, 0.0015), SNEAKER, 0.85), "sneakers", f"shoe-rear-{n}"))
        sole = K.extrude(f"sole-rear-{n}", f"foot-{n}", sole_outline(SOLE_L, s), "zx", -R.ANKLE_Y, -R.ANKLE_Y + SOLE_T, SNEAKER, 0.95)
        sole["transform"] = {"rotation": K.rot_w(-R.FOOT_DIR)}
        out.append(tag(sole, "sneakers", f"shoe-rear-{n}"))
        secs = []
        for u, vc, hh, e in TOEBOX:
            L, M = lm(max(u, 0.008), TOE_L) if u > 0.008 else lm(u + R.BALL_X, SOLE_L)
            secs.append(K.section([u, vc, s * (L - M) / 2], max((L + M) / 2 - 0.0015, 0.005), hh, e, K.quat(K.W, K.V)))
        out.append(tag(K.loft(f"shoe-front-{n}", f"toe-{n}", secs, SNEAKER, 0.85), "sneakers", f"shoe-front-{n}"))
        toe_sole = [(-0.006, 0.045, 0.038)] + TOE_L
        out.append(tag(K.extrude(f"sole-front-{n}", f"toe-{n}", sole_outline(toe_sole, s), "zx", 0.0, SOLE_T, SNEAKER, 0.95), "sneakers", f"shoe-front-{n}"))
        for k, (x, y) in enumerate(((0.030, 0.0055), (0.048, -0.0015), (0.066, -0.0095))):
            u, v = R.foot_local(x, y)
            out.append(tag(K.capsule(f"lace-{n}{k}", f"foot-{n}", [u, v, -0.014], [u, v, 0.014], 0.003, 0.003, SNEAKER, 0.5, role="detail", texture=True), "sneakers", f"lace-{n}{k}"))


def backpack(out):
    rows = [(0.028, 0.118, 0.030, 0.085, 3.0), (0.040, 0.126, 0.050, 0.108, 3.2), (0.070, 0.131, 0.057, 0.118, 3.4), (0.180, 0.134, 0.060, 0.122, 3.4), (0.270, 0.130, 0.054, 0.118, 3.2), (0.312, 0.122, 0.042, 0.104, 3.0), (0.332, 0.112, 0.026, 0.078, 2.6)]
    out.append(tag(K.stack("backpack", "torso", [(u, -(vc - hd), vc + hd, hw, e) for u, vc, hd, hw, e in rows], PACK, 0.4), "backpack", "backpack"))
    rows = [(0.050, 0.190, 0.012, 0.070, 3.0), (0.065, 0.198, 0.022, 0.084, 3.4), (0.150, 0.200, 0.024, 0.086, 3.4), (0.185, 0.196, 0.018, 0.080, 3.0), (0.196, 0.190, 0.010, 0.066, 2.6)]
    out.append(tag(K.stack("backpack-pocket", "torso", [(u, -(vc - hd), vc + hd, hw, e) for u, vc, hd, hw, e in rows], PACK, 0.36), "backpack", "backpack-pocket"))
    path = [(0.322, 0.094, 0.068), (0.350, 0.052, 0.071), (0.3595, 0.006, 0.074), (0.340, -0.038, 0.077), (0.296, -0.066, 0.081), (0.225, -0.085, 0.087), (0.150, -0.089, 0.096), (0.100, -0.062, 0.108), (0.070, 0.004, 0.113), (0.058, 0.078, 0.108)]
    for s, n in ((1, "l"), (-1, "r")):
        out.append(tag(K.tube(f"backpack-strap-{n}", "torso", [([u, v, s * w], 0.017, 0.005, 3.0) for u, v, w in path], PACK, 0.3, role="detail"), "backpack", f"backpack-strap-{n}"))
    out.append(tag(K.tube("sternum-strap", "torso", [([0.262, -0.0755, -0.084], 0.008, 0.0015, 4.0), ([0.262, -0.0795, 0.0], 0.008, 0.0015, 4.0), ([0.262, -0.0755, 0.084], 0.008, 0.0015, 4.0)], PACK, 0.3, hint=K.U, role="detail", texture=True), "backpack", "sternum-strap"))


def stem_at(t, off=0.0):
    x, y = R.stem_point(t)
    return [x - off * R.AXIS[1], y + off * R.AXIS[0], 0.0]


def scooter(out):
    ax = [R.AXIS[0], R.AXIS[1], 0.0]
    o = [R.HT_BOTTOM[0], R.HT_BOTTOM[1], 0.0]
    out.append(tag(K.extrude("deck", "frame", [(R.DECK_TOP, -0.057), (R.DECK_TOP, 0.057), (0.082, R.DECK_HALF), (0.070, R.DECK_HALF), (R.DECK_BOTTOM, 0.047), (R.DECK_BOTTOM, -0.047), (0.070, -R.DECK_HALF), (0.082, -R.DECK_HALF)], "yz", R.DECK_X0, R.DECK_X1, DECK, 0.3), "deck", "deck"))
    out.append(tag(K.box("griptape", "frame", [0.300, 0.002, 0.100], [0.245, R.DECK_TOP + 0.001, 0], DECK, 0.12, role="texture", texture=True), "deck", "griptape"))
    plate = [(0.110, 0.058), (0.110, 0.084), (0.040, 0.090), (0.004, 0.094), (-0.016, 0.088), (-0.024, 0.074), (-0.016, 0.060), (0.010, 0.056)]
    out.append(tag(K.extrude("strut-l", "frame", plate, "xy", 0.0195, 0.0275, DECK, 0.7), "deck", "strut-l"))
    out.append(tag(K.extrude("strut-r", "frame", plate, "xy", -0.0275, -0.0195, DECK, 0.7), "deck", "strut-r"))
    axle = [(-0.034, 0.0), (-0.034, 0.008), (-0.0285, 0.008), (-0.0285, 0.005), (0.0285, 0.005), (0.0285, 0.008), (0.034, 0.008), (0.034, 0.0)]
    out.append(tag(K.revolve("axle-rear", "frame", axle, [0, R.WHEEL_R, 0], K.W, STEM, 0.5, role="detail"), "stem", "axle-rear"))
    out.append(tag(K.revolve("axle-front", "frame", axle, [R.WB, R.WHEEL_R, 0], K.W, STEM, 0.5, role="detail"), "stem", "axle-front"))
    c = (0.0, R.WHEEL_R)
    band = K.arc(c[0], c[1], R.FENDER_R1, 140, 30, 22) + [(0.096, 0.097), (0.094, 0.088), (0.086, 0.092)] + K.arc(c[0], c[1], R.FENDER_R0, 32, 140, 22)
    out.append(tag(K.extrude("fender", "frame", band, "xy", -0.022, 0.022, DECK, 0.2), "deck", "fender"))
    pad = K.arc(c[0], c[1], R.FENDER_R0, 118, 66, 10) + K.arc(c[0], c[1], R.FENDER_R0 - 0.003, 66, 118, 10)
    out.append(tag(K.extrude("brake-pad", "frame", pad, "xy", -0.016, 0.016, DECK, 0.2, role="detail", texture=True), "deck", "brake-pad"))
    out.append(tag(K.box("hinge-block", "frame", [0.050, 0.046, 0.058], [0.428, 0.078, 0], STEM, 0.7, corner=0.006), "stem", "head-tube"))
    top = stem_at(0.070, -0.012)
    out.append(tag(K.tube("neck-arm", "frame", [([0.430, 0.094, 0], 0.019, 0.017, 3.0), ([0.466, 0.136, 0], 0.018, 0.016, 3.0), ([0.498, 0.182, 0], 0.018, 0.016, 3.0), ([top[0], top[1], 0], 0.018, 0.017, 3.0)], STEM, 0.7), "stem", "head-tube"))
    out.append(tag(K.revolve("hinge-pin", "frame", [(-0.033, 0.0), (-0.033, 0.006), (0.033, 0.006), (0.033, 0.0)], [0.428, 0.064, 0], K.W, STEM, 0.5, role="detail"), "stem", "hinge-pin"))
    out.append(tag(K.revolve("head-tube", "frame", [(R.HT_T0, 0.0), (R.HT_T0, 0.023), (R.HT_T0 + 0.008, 0.023), (R.HT_T0 + 0.008, 0.020), (R.HT_T1 - 0.008, 0.020), (R.HT_T1 - 0.008, 0.023), (R.HT_T1, 0.023), (R.HT_T1, 0.0)], o, ax, STEM, 0.7), "stem", "head-tube"))
    cc = stem_at(R.CROWN_T)
    out.append(tag(K.box("fork-crown", "frame", [0.030, 0.024, 0.060], cc, STEM, 0.7, corner=0.004, rotation=K.rot_w(math.degrees(R.RAKE))), "stem", "fork-crown"))
    a0, a1 = stem_at(R.CROWN_T + 0.010, 0.013), stem_at(R.CROWN_T + 0.010, -0.013)
    blade = [(a0[0], a0[1]), (a1[0], a1[1]), (R.WB + 0.013, R.WHEEL_R + 0.006), (R.WB + 0.013, R.WHEEL_R - 0.006), (R.WB + 0.004, R.WHEEL_R - 0.014), (R.WB - 0.008, R.WHEEL_R - 0.012), (R.WB - 0.014, R.WHEEL_R)]
    out.append(tag(K.extrude("fork-l", "frame", blade, "xy", 0.0195, 0.0275, STEM, 0.7), "stem", "fork-l"))
    out.append(tag(K.extrude("fork-r", "frame", blade, "xy", -0.0275, -0.0195, STEM, 0.7), "stem", "fork-r"))
    out.append(tag(K.revolve("stem-lower", "frame", [(R.HT_T1, 0.0), (R.HT_T1, 0.017), (0.372, 0.017), (0.372, 0.0)], o, ax, STEM, 0.7), "stem", "stem"))
    out.append(tag(K.revolve("clamp", "frame", [(0.362, 0.0), (0.362, 0.0225), (0.394, 0.0225), (0.394, 0.0)], o, ax, STEM, 0.5, role="detail"), "stem", "clamp"))
    lever = stem_at(0.366, -0.0275)
    out.append(tag(K.box("clamp-lever", "frame", [0.050, 0.011, 0.012], [lever[0], lever[1], 0.0], STEM, 0.4, role="detail", corner=0.003, rotation=K.rot_w(90 + math.degrees(R.RAKE))), "stem", "clamp"))
    out.append(tag(K.revolve("stem-upper", "frame", [(0.394, 0.0), (0.394, 0.0145), (R.BAR_T - 0.008, 0.0145), (R.BAR_T - 0.008, 0.0)], o, ax, STEM, 0.7), "stem", "stem"))
    bar = [R.BAR_X, R.BAR_Y, 0.0]
    out.append(tag(K.revolve("bar-tee", "frame", [(-0.030, 0.0), (-0.030, 0.0165), (0.030, 0.0165), (0.030, 0.0)], bar, K.W, STEM, 0.7), "stem", "handlebar"))
    out.append(tag(K.revolve("handlebar", "frame", [(-0.168, 0.0), (-0.168, 0.011), (0.168, 0.011), (0.168, 0.0)], bar, K.W, STEM, 0.7), "stem", "handlebar"))
    for s, n in ((1, "l"), (-1, "r")):
        prof = [(s * 0.060, 0.0), (s * 0.060, 0.019), (s * 0.066, 0.019), (s * 0.066, R.GRIP_R), (s * 0.164, R.GRIP_R), (s * 0.164, 0.018), (s * 0.172, 0.018), (s * 0.172, 0.0)]
        out.append(tag(K.revolve(f"grip-{n}", "frame", prof if s > 0 else prof[::-1], bar, K.W, GRIP, 0.12), "grips", f"grip-{n}"))
    tyre = [(-0.015, 0.047), (-0.015, 0.063), (-0.0125, 0.0685), (-0.0075, 0.0715), (0.0, R.WHEEL_R), (0.0075, 0.0715), (0.0125, 0.0685), (0.015, 0.063), (0.015, 0.047), (-0.015, 0.047)]
    rim = [(-0.012, 0.036), (-0.012, 0.047), (0.012, 0.047), (0.012, 0.036), (-0.012, 0.036)]
    for n in ("rear", "front"):
        b = f"wheel-{n}"
        out.append(tag(K.revolve(f"tyre-{n}", b, tyre, [0, 0, 0], K.W, WHEEL, 0.14), "wheels", f"tyre-{n}"))
        out.append(tag(K.revolve(f"rim-{n}", b, rim, [0, 0, 0], K.W, WHEEL, 0.8), "wheels", f"rim-{n}"))
        sp = K.box(f"spoke-{n}", b, [0.076, 0.009, 0.014], [0, 0, 0], WHEEL, 0.6, role="detail")
        sp["repeat"] = {"count": 3, "rotationAxis": [0, 0, 1], "rotationRad": round(math.pi / 3, 7)}
        out.append(tag(sp, "wheels", f"spoke-{n}"))
        out.append(tag(K.revolve(f"hub-{n}", b, [(-0.013, 0.0), (-0.013, 0.014), (0.013, 0.014), (0.013, 0.0)], [0, 0, 0], K.W, WHEEL, 0.3, role="detail"), "wheels", f"hub-{n}"))


def build():
    out = []
    legs(out)
    arms(out)
    trunk(out)
    head(out)
    backpack(out)
    scooter(out)
    return out
