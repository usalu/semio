"""👷 Round 5 body of the worker carrying a box at 1:50, every cross-section stated by hand in the world of the rest pose (x ahead, y up, z left; ground 0, ankle 0.096, knee 0.521, hip joint 0.941, lumbosacral joint 1.042, shoulder joint 1.454, C7 1.528, vertex 1.771, helmet crown 1.815) or in the frame of the limb bone (u along the bone, v ahead at rest, w left).

Anthropometry (stocky man, 1.741 m, 84 kg, ANSUR II means): head 0.232 high, 0.197 long, 0.153 broad; neck girth 0.40; biacromial 0.41, bideltoid 0.51; chest 0.33 x 0.255, girth 1.06; waist 0.33 x 0.25; hips 0.353 broad, buttock depth 0.25; thigh girth 0.625 / 0.54 / 0.41; calf girth 0.39; ankle girth 0.225; upper arm girth 0.33; forearm 0.29; wrist 0.176; hand 0.193 x 0.089 x 0.030; foot 0.271 x 0.102, safety boot 0.304 x 0.108 on a 30 / 18 mm sole. Garments: work shirt 6 mm ease, trousers 10 mm, vest 9 mm over the shirt.
"""
import math

import r3_worker_carrying_box_kit as k

SKIN, HAIR, COTTON, VEST, BAND, HDPE, GLOVE, BOOT, BOARD = "skin", "hair", "cotton-clothing", "hi-vis-polyester", "retroreflective", "hdpe-shell", "glove", "leather-shoe", "cardboard"
TOP = 1.771
HIP_Z = 0.0895
GRIP_W = 0.217
BOARD_T = 0.0075
BOX_C = (0.0, -0.085, GRIP_W)
BOX_H = (0.15, 0.15, 0.20)
SLOT = (0.0, -0.010, 0.0525, 0.018)

SHIRT = [
    (0.990, -0.005, 0, 0.166, 0.126, 2.4),
    (1.030, -0.004, 0, 0.169, 0.130, 2.4),
    (1.090, -0.002, 0, 0.167, 0.133, 2.4),
    (1.170, -0.004, 0, 0.164, 0.131, 2.4),
    (1.250, -0.009, 0, 0.165, 0.130, 2.4),
    (1.310, -0.013, 0, 0.171, 0.129, 2.4),
    (1.360, -0.016, 0, 0.178, 0.124, 2.4),
    (1.405, -0.019, 0, 0.190, 0.113, 2.4),
    (1.440, -0.022, 0, 0.198, 0.099, 2.4),
    (1.462, -0.024, 0, 0.192, 0.089, 2.4),
    (1.482, -0.025, 0, 0.168, 0.080, 2.3),
    (1.502, -0.025, 0, 0.130, 0.073, 2.2),
    (1.520, -0.024, 0, 0.096, 0.069, 2.1),
    (1.534, -0.022, 0, 0.074, 0.067, 2.0),
]
VEST_ROWS = [
    (0.930, -0.005, 0, 0.195, 0.141, 2.35),
    (0.980, -0.005, 0, 0.195, 0.142, 2.35),
    (1.030, -0.004, 0, 0.188, 0.141, 2.35),
    (1.100, -0.002, 0, 0.179, 0.142, 2.35),
    (1.180, -0.004, 0, 0.174, 0.140, 2.35),
    (1.260, -0.009, 0, 0.175, 0.139, 2.35),
    (1.320, -0.013, 0, 0.179, 0.138, 2.35),
    (1.360, -0.016, 0, 0.176, 0.133, 2.35),
    (1.400, -0.019, 0, 0.168, 0.123, 2.35),
    (1.440, -0.022, 0, 0.164, 0.107, 2.35),
    (1.470, -0.024, 0, 0.160, 0.090, 2.35),
    (1.489, -0.025, 0, 0.150, 0.080, 2.35),
]
SEAT = [
    (0.838, 0.000, 0, 0.112, 0.088, 2.3),
    (0.862, -0.002, 0, 0.176, 0.120, 2.35),
    (0.900, -0.005, 0, 0.188, 0.129, 2.35),
    (0.950, -0.006, 0, 0.189, 0.132, 2.35),
    (1.000, -0.005, 0, 0.181, 0.130, 2.35),
    (1.048, -0.004, 0, 0.171, 0.128, 2.35),
]
SKULL = [
    (0.000, -0.005, 0, 0.012, 0.016),
    (0.006, -0.005, 0, 0.035, 0.045),
    (0.016, -0.005, 0, 0.050, 0.066),
    (0.030, -0.004, 0, 0.062, 0.082),
    (0.050, -0.003, 0, 0.071, 0.094),
    (0.075, -0.002, 0, 0.0765, 0.0985),
    (0.100, -0.001, 0, 0.076, 0.098),
    (0.125, 0.004, 0, 0.073, 0.091),
    (0.145, 0.010, 0, 0.070, 0.083),
    (0.170, 0.018, 0, 0.064, 0.074),
    (0.190, 0.026, 0, 0.058, 0.066),
    (0.210, 0.034, 0, 0.050, 0.056),
    (0.224, 0.042, 0, 0.038, 0.044),
    (0.232, 0.050, 0, 0.022, 0.028),
]
DOME = [
    (1.815, -0.010, 0, 0.012, 0.015),
    (1.811, -0.010, 0, 0.040, 0.048),
    (1.802, -0.010, 0, 0.062, 0.074),
    (1.787, -0.009, 0, 0.081, 0.097),
    (1.765, -0.008, 0, 0.096, 0.115),
    (1.740, -0.007, 0, 0.105, 0.126),
    (1.715, -0.006, 0, 0.109, 0.130),
    (1.690, -0.005, 0, 0.110, 0.131),
    (1.681, -0.005, 0, 0.110, 0.131),
]
FINGERS = [(0.080, 0.033, 0.045, 0.026, 0.022), (0.082, 0.011, 0.049, 0.029, 0.023), (0.079, -0.011, 0.046, 0.027, 0.022), (0.071, -0.032, 0.037, 0.021, 0.020)]


def drop(rows):
    return [(TOP - r[0],) + tuple(r[1:]) for r in rows]


def side_at(rows, y):
    """📏 Centre, half breadth and half depth of a stated trunk loft at height `y` (linear between the stated rows)."""
    for a, b in zip(rows, rows[1:]):
        if a[0] <= y <= b[0]:
            t = (y - a[0]) / (b[0] - a[0])
            return tuple(a[i] + (b[i] - a[i]) * t for i in (1, 3, 4))
    raise SystemExit("height %.3f outside the loft" % y)


def head(S):
    nose = drop([(0.105, 0.094, 0, 0.008, 0.006), (0.125, 0.102, 0, 0.010, 0.010), (0.145, 0.110, 0, 0.013, 0.014), (0.158, 0.114, 0, 0.017, 0.016), (0.166, 0.108, 0, 0.016, 0.012), (0.171, 0.102, 0, 0.012, 0.007)])
    hair = drop([(0.052, -0.012, 0, 0.073, 0.088), (0.080, -0.017, 0, 0.0795, 0.088), (0.110, -0.025, 0, 0.0785, 0.078), (0.135, -0.037, 0, 0.074, 0.062), (0.155, -0.047, 0, 0.064, 0.046), (0.170, -0.050, 0, 0.050, 0.032)])
    neck = [(1.455, -0.031, 0, 0.073, 0.069), (1.500, -0.027, 0, 0.065, 0.064), (1.540, -0.019, 0, 0.061, 0.062), (1.575, -0.012, 0, 0.058, 0.060), (1.605, -0.010, 0, 0.054, 0.056)]
    out = [
        k.loft(S, "neck", "head", SKIN, 0.3, neck, e=2.0, sub=1),
        k.loft(S, "head", "head", SKIN, 0.3, drop(SKULL), e=2.15, sub=2),
        k.loft(S, "hair", "head", HAIR, 0.75, hair, e=2.1, sub=2),
        k.loft(S, "nose", "head", SKIN, 0.3, nose, e=2.0, sub=1, role="detail"),
        k.ellipsoid(S, "lips", "head", SKIN, 0.22, (0.094, TOP - 0.188, 0), (0.008, 0.0075, 0.024), role="detail"),
        k.ellipsoid(S, "chin", "head", SKIN, 0.3, (0.079, TOP - 0.213, 0), (0.017, 0.016, 0.024), role="detail"),
    ]
    for side, mirror in (("l", False), ("r", True)):
        out.append(k.ellipsoid(S, "ear-" + side, "head", SKIN, 0.35, (-0.012, TOP - 0.130, 0.0765), (0.016, 0.031, 0.010), role="detail", mirror=mirror, tilt=-10))
        out.append(k.ellipsoid(S, "eye-" + side, "head", SKIN, 0.9, (0.087, TOP - 0.118, 0.031), (0.006, 0.006, 0.0125), role="detail", mirror=mirror))
        out.append(k.ellipsoid(S, "brow-" + side, "head", SKIN, 0.2, (0.091, TOP - 0.104, 0.032), (0.007, 0.005, 0.022), role="detail", mirror=mirror))
    return out


def dome_top(x):
    """⛑ Height of the helmet crown line above the walking direction `x` (the dome's own profile)."""
    best = DOME[-1][0]
    for a, b in zip(DOME, DOME[1:]):
        ra, rb = a[4], b[4]
        d = abs(x - (-0.008))
        if ra <= d <= rb:
            t = (d - ra) / (rb - ra)
            best = a[0] + (b[0] - a[0]) * t
    return 1.815 if abs(x + 0.008) < DOME[0][4] else best


def helmet(S):
    rim = [(1.694, -0.005, 0, 0.1125, 0.1335), (1.686, -0.005, 0, 0.1155, 0.1365), (1.677, -0.005, 0, 0.1155, 0.1365), (1.672, -0.005, 0, 0.1125, 0.1335)]
    peak = [(0.092, 1.6835, 0, 0.100, 0.0045), (0.130, 1.680, 0, 0.092, 0.0045), (0.160, 1.676, 0, 0.074, 0.0042), (0.180, 1.673, 0, 0.048, 0.004), (0.188, 1.672, 0, 0.022, 0.004)]
    ridge = [(x, dome_top(x) - 0.003, 0, 0.017, 0.0075) for x in (-0.118, -0.095, -0.065, -0.030, -0.008, 0.015, 0.050, 0.080, 0.100)]
    band = [(1.668, -0.004, 0, 0.082, 0.102), (1.690, -0.004, 0, 0.083, 0.103)]
    return [
        k.loft(S, "helmet", "head", HDPE, 0.92, DOME, e=2.15, sub=1),
        k.loft(S, "helmet-rim", "head", HDPE, 0.92, rim, e=2.15),
        k.loft(S, "helmet-peak", "head", HDPE, 0.92, peak, axes="x", e=2.6),
        k.free_loft(S, "helmet-ridge", "head", HDPE, 0.85, [(r[0], r[1], 0, r[3], r[4]) for r in ridge], e=2.4, role="detail", across=k.Z),
        k.loft(S, "helmet-headband", "head", HDPE, 0.35, band, e=2.1, role="detail"),
    ]


def trunk(S):
    collar = [(1.505, -0.024, 0, 0.082, 0.076), (1.525, -0.022, 0, 0.074, 0.071), (1.545, -0.019, 0, 0.068, 0.068), (1.556, -0.018, 0, 0.066, 0.067)]
    out = [
        k.loft(S, "trousers-seat", "pelvis", COTTON, 0.3, SEAT, e=2.35, sub=1),
        k.loft(S, "shirt", "torso", COTTON, 0.5, SHIRT[1:], e=2.4, sub=1),
        k.loft(S, "shirt-collar", "torso", COTTON, 0.5, collar, e=2.0),
        k.loft(S, "vest", "torso", VEST, 0.9, VEST_ROWS[2:], e=2.35, sub=1),
        k.loft(S, "vest-hem", "pelvis", VEST, 0.9, [(0.930, -0.005, 0, 0.195, 0.141, 2.35), (0.980, -0.005, 0, 0.195, 0.142, 2.35), (1.030, -0.004, 0, 0.1875, 0.1405, 2.35), (1.075, -0.003, 0, 0.1805, 0.1395, 2.35)], e=2.35, sub=1),
    ]
    for side, mirror in (("l", False), ("r", True)):
        out.append(k.ellipsoid(S, "buttock-" + side, "pelvis", COTTON, 0.3, (-0.062, 0.925, 0.088), (0.078, 0.092, 0.088), mirror=mirror, tilt=8))
    for name, lo, hi in (("vest-band-low", 1.055, 1.105), ("vest-band-high", 1.185, 1.235)):
        rows = []
        for y in (lo, (lo + hi) / 2, hi):
            cx, rz, rx = side_at(VEST_ROWS, y)
            rows.append((y, cx, 0, rz + 0.002, rx + 0.002, 2.35))
        out.append(k.loft(S, name, "torso", BAND, 0.6, rows, role="detail", texture=True))
    for side, mirror in (("l", False), ("r", True)):
        path = []
        for y in (1.235, 1.290, 1.340, 1.390, 1.430, 1.462, 1.482):
            cx, rz, rx = side_at(VEST_ROWS, y)
            reach = rx * (1 - (0.105 / rz) ** 2.35) ** (1 / 2.35)
            path.append((cx + reach + 0.0015, y, 0.105, 0.025, 0.003))
        top = side_at(VEST_ROWS, 1.489)
        path.append((top[0], 1.4915, 0.105, 0.025, 0.003))
        for y in (1.482, 1.462, 1.430, 1.390, 1.340, 1.290, 1.235):
            cx, rz, rx = side_at(VEST_ROWS, y)
            reach = rx * (1 - (0.105 / rz) ** 2.35) ** (1 / 2.35)
            path.append((cx - reach - 0.0015, y, 0.105, 0.025, 0.003))
        out.append(k.free_loft(S, "vest-brace-" + side, "torso", BAND, 0.6, path, e=4.0, role="detail", mirror=mirror, across=k.Z, texture=True))
    zip_rows = []
    for y in (1.045, 1.100, 1.200, 1.300, 1.380, 1.440):
        cx, rz, rx = side_at(VEST_ROWS, y)
        zip_rows.append((cx + rx + 0.001, y, 0, 0.006, 0.002))
    out.append(k.free_loft(S, "vest-zip", "torso", VEST, 0.45, zip_rows, e=4.0, role="texture", across=k.Z, texture=True))
    return out


def legs(S):
    out = []
    for side, mirror in (("l", False), ("r", True)):
        thigh = [
            (0.985, -0.004, 0.094, 0.086, 0.104),
            (0.940, -0.002, 0.095, 0.093, 0.112),
            (0.880, 0.002, 0.097, 0.092, 0.110),
            (0.840, 0.005, 0.098, 0.091, 0.106),
            (0.760, 0.008, 0.096, 0.086, 0.098),
            (0.680, 0.009, 0.094, 0.080, 0.090),
            (0.600, 0.009, 0.092, 0.073, 0.080),
            (0.540, 0.008, 0.091, 0.068, 0.074),
            (0.498, 0.007, 0.090, 0.066, 0.072),
        ]
        shank = [
            (0.565, 0.007, 0.090, 0.066, 0.072),
            (0.520, 0.006, 0.090, 0.066, 0.072),
            (0.470, 0.002, 0.090, 0.065, 0.069),
            (0.400, -0.005, 0.090, 0.066, 0.071),
            (0.330, -0.007, 0.090, 0.065, 0.070),
            (0.240, -0.006, 0.090, 0.062, 0.066),
            (0.160, -0.004, 0.090, 0.062, 0.066),
            (0.118, -0.002, 0.090, 0.063, 0.068),
            (0.104, -0.002, 0.090, 0.062, 0.067),
        ]
        pad = [(0.585, 0.018, 0.090, 0.047, 0.066), (0.560, 0.022, 0.090, 0.050, 0.068), (0.500, 0.020, 0.090, 0.050, 0.066), (0.440, 0.014, 0.090, 0.048, 0.064), (0.405, 0.010, 0.090, 0.045, 0.062)]
        pocket = [(0.800, 0.006, 0.112, 0.082, 0.058), (0.785, 0.007, 0.114, 0.084, 0.062), (0.700, 0.009, 0.110, 0.080, 0.060), (0.640, 0.009, 0.106, 0.078, 0.058), (0.630, 0.009, 0.104, 0.074, 0.054)]
        shaft = [(0.176, -0.012, HIP_Z, 0.045, 0.053), (0.168, -0.012, HIP_Z, 0.048, 0.056), (0.130, -0.011, HIP_Z, 0.047, 0.056), (0.095, -0.010, HIP_Z, 0.047, 0.059), (0.060, -0.009, HIP_Z, 0.048, 0.070), (0.034, -0.008, HIP_Z, 0.047, 0.073)]
        vamp = [(-0.030, 0.076, HIP_Z, 0.047, 0.046), (0.020, 0.074, HIP_Z, 0.050, 0.046), (0.070, 0.066, HIP_Z + 0.001, 0.053, 0.040), (0.105, 0.060, HIP_Z + 0.002, 0.054, 0.036), (0.138, 0.055, HIP_Z + 0.002, 0.054, 0.034)]
        sole = [(-0.084, 0.016, HIP_Z, 0.030, 0.014, 3.0), (-0.074, 0.0155, HIP_Z, 0.041, 0.0155, 4.0), (-0.030, 0.0155, HIP_Z, 0.044, 0.0155, 4.0), (0.000, 0.015, HIP_Z, 0.043, 0.012, 4.0), (0.040, 0.013, HIP_Z + 0.001, 0.045, 0.010, 4.0), (0.090, 0.010, HIP_Z + 0.002, 0.053, 0.010, 4.0), (0.138, 0.0095, HIP_Z + 0.002, 0.056, 0.0095, 4.0)]
        cap = [(0.118, 0.055, HIP_Z + 0.002, 0.054, 0.034), (0.165, 0.052, HIP_Z + 0.002, 0.053, 0.033), (0.195, 0.047, HIP_Z + 0.001, 0.046, 0.028), (0.211, 0.043, HIP_Z + 0.001, 0.033, 0.021), (0.2185, 0.040, HIP_Z + 0.001, 0.016, 0.012)]
        toe_sole = [(0.120, 0.0095, HIP_Z + 0.002, 0.056, 0.0095, 4.0), (0.180, 0.0105, HIP_Z + 0.002, 0.054, 0.009, 4.0), (0.208, 0.013, HIP_Z + 0.001, 0.040, 0.008, 3.0), (0.2215, 0.016, HIP_Z + 0.001, 0.016, 0.006, 2.5)]
        out += [
            k.loft(S, "thigh-" + side, "thigh-" + side, COTTON, 0.3, thigh, e=2.2, sub=1, mirror=mirror),
            k.loft(S, "cargo-pocket-" + side, "thigh-" + side, COTTON, 0.3, pocket, e=3.0, role="detail", mirror=mirror),
            k.loft(S, "shank-" + side, "shank-" + side, COTTON, 0.3, shank, e=2.2, sub=1, mirror=mirror),
            k.ellipsoid(S, "knee-" + side, "shank-" + side, COTTON, 0.3, (0.004, 0.5206, 0.090), (0.062, 0.064, 0.060), mirror=mirror),
            k.loft(S, "knee-pad-" + side, "shank-" + side, COTTON, 0.2, pad, e=3.0, role="detail", mirror=mirror),
            k.loft(S, "boot-shaft-" + side, "foot-" + side, BOOT, 0.12, shaft, e=2.2, sub=1, mirror=mirror),
            k.loft(S, "boot-" + side, "foot-" + side, BOOT, 0.12, vamp, axes="x", e=2.3, sub=1, mirror=mirror),
            k.loft(S, "sole-" + side, "foot-" + side, BOOT, 0.05, sole, axes="x", mirror=mirror),
            k.loft(S, "toe-cap-" + side, "toe-" + side, BOOT, 0.12, cap, axes="x", e=2.4, sub=1, mirror=mirror),
            k.loft(S, "toe-sole-" + side, "toe-" + side, BOOT, 0.05, toe_sole, axes="x", mirror=mirror),
        ]
        for i, x in enumerate((0.030, 0.052, 0.074)):
            y = 0.118 - (x - 0.030) * 0.50
            out.append(k.box(S, "lace-%s-%d" % (side, i), "foot-" + side, BOOT, 0.6, (x, y, HIP_Z), (0.006, 0.004, 0.046), role="texture", mirror=mirror, tilt=-27, texture=True))
    return out


def arm_rows():
    upper = [(-0.040, 0.000, -0.016, 0.012, 0.016), (-0.032, 0.000, -0.012, 0.034, 0.038), (-0.016, 0.000, -0.004, 0.050, 0.051), (0.010, -0.002, 0.008, 0.060, 0.064), (0.060, -0.002, 0.009, 0.060, 0.066), (0.130, 0.000, 0.005, 0.056, 0.059), (0.210, 0.001, 0.001, 0.053, 0.050), (0.280, 0.000, 0.000, 0.049, 0.047), (0.330, 0.000, 0.000, 0.047, 0.045)]
    fore = [(-0.034, 0.000, 0.000, 0.045, 0.044), (0.000, 0.000, 0.000, 0.050, 0.047), (0.045, 0.004, 0.000, 0.051, 0.047), (0.110, 0.003, 0.000, 0.046, 0.042), (0.170, 0.001, 0.000, 0.040, 0.036), (0.215, 0.000, 0.000, 0.036, 0.032), (0.232, 0.000, 0.000, 0.036, 0.032)]
    cuff = [(0.198, 0.000, 0.000, 0.040, 0.036), (0.206, 0.000, 0.000, 0.044, 0.040), (0.240, 0.000, 0.000, 0.040, 0.034), (0.285, 0.000, 0.000, 0.035, 0.026)]
    return upper, fore, cuff


def finger_points(i, sign):
    """🤏 Knuckle and the three joints of finger `i` (index first) in the frame of the hand, hooked towards the palm (-w for the left hand): 75 degrees at the knuckle, 80 at the middle joint, 30 at the end joint."""
    u, v, l1, l2, l3 = FINGERS[i]
    pts = [(u, v, 0.0)]
    bend = 0.0
    for length, add in ((l1, 75), (l2, 80), (l3, 30)):
        bend += add
        p = pts[-1]
        pts.append((p[0] + length * math.cos(math.radians(bend)), v, p[2] - sign * length * math.sin(math.radians(bend))))
    return pts


def local(sid, kind, bone, material, tone, role="body", **fields):
    return {**k.base(sid, kind, bone, material, tone, role), **fields}


def arms():
    upper, fore, cuff = arm_rows()
    out = []
    for side, mirror, sign in (("l", False, 1), ("r", True, -1)):
        palm = [(-0.012, 0.000, 0.000, 0.033, 0.024), (0.012, 0.002, 0.000, 0.039, 0.020), (0.040, 0.002, 0.001, 0.046, 0.018), (0.066, 0.001, 0.001, 0.047, 0.017), (0.084, 0.000, 0.000, 0.045, 0.0135)]
        out += [
            k.local_loft("sleeve-" + side, "upper-arm-" + side, COTTON, 0.5, upper, e=2.1, sub=1, mirror=mirror),
            k.local_loft("forearm-" + side, "forearm-" + side, COTTON, 0.5, fore, e=2.1, sub=1, mirror=mirror),
            k.local_loft("glove-cuff-" + side, "forearm-" + side, GLOVE, 0.4, cuff, e=2.1, mirror=mirror),
            k.local_loft("hand-" + side, "hand-" + side, GLOVE, 0.4, palm, e=2.6, sub=1, mirror=mirror),
            local("thenar-" + side, "ellipsoid", "hand-" + side, GLOVE, 0.4, radii=[0.030, 0.021, 0.014], transform={"translation": [0.034, 0.027, -sign * 0.010]}),
        ]
        for i in range(4):
            pts = finger_points(i, sign)
            for j, (a, b) in enumerate(zip(pts, pts[1:])):
                r = 0.0112 - 0.0006 * j - (0.0012 if i == 3 else 0)
                out.append(local("finger-%s-%d%s" % (side, i, "abc"[j]), "capsule", "hand-" + side, GLOVE, 0.4, role="detail", p0=[k.r5(c) for c in a], p1=[k.r5(c) for c in b], r0=k.r5(r), r1=k.r5(r - 0.0006)))
        thumb = [(0.022, 0.036, -sign * 0.006), (0.052, 0.060, -sign * 0.011), (0.080, 0.070, -sign * 0.013)]
        for j, (a, b) in enumerate(zip(thumb, thumb[1:])):
            out.append(local("thumb-%s-%d" % (side, j), "capsule", "hand-" + side, GLOVE, 0.4, role="detail", p0=list(a), p1=list(b), r0=0.0135 - 0.001 * j, r1=0.0125 - 0.001 * j))
    return out


def slot_ring():
    cu, cv, hl, hh = SLOT
    ring = []
    for centre, start in ((cu + hl - hh, -90), (cu - hl + hh, 90)):
        for i in range(7):
            a = math.radians(start + 30 * i)
            ring.append((centre + hh * math.cos(a), cv + hh * math.sin(a)))
    return ring


def carton():
    """📦 The carton in the frame of the right box bone (u ahead, v up, w left; origin at the right hand hole): a sleeve of four walls closed by two end panels with a hand hole each, 7.5 mm double-wall board."""
    cu, cv, cw = BOX_C
    hu, hv, hw = BOX_H
    outer = [(cu - hu, cv - hv), (cu + hu, cv - hv), (cu + hu, cv + hv), (cu - hu, cv + hv)]
    inner = [(cu - hu + BOARD_T, cv - hv + BOARD_T), (cu + hu - BOARD_T, cv - hv + BOARD_T), (cu + hu - BOARD_T, cv + hv - BOARD_T), (cu - hu + BOARD_T, cv + hv - BOARD_T)]
    plate = lambda sid, w0: local(sid, "extrude", "box-r", BOARD, 0.66, plane="xy", profile=[list(p) for p in outer], holes=[[list(map(k.r5, p)) for p in slot_ring()]], **{"from": k.r5(w0), "to": k.r5(w0 + BOARD_T)})
    top = cv + hv
    return [
        local("box", "extrude", "box-r", BOARD, 0.66, plane="xy", profile=[list(p) for p in outer], holes=[[list(p) for p in inner]], **{"from": k.r5(cw - hw + BOARD_T), "to": k.r5(cw + hw - BOARD_T)}),
        plate("box-end-r", cw - hw),
        plate("box-end-l", cw + hw - BOARD_T),
        local("box-tape", "box", "box-r", BOARD, 0.82, role="detail", size=[0.05, 0.0012, 2 * hw + 0.0012], transform={"translation": [cu, top + 0.0006, cw]}, texture=True),
        local("box-tape-end-r", "box", "box-r", BOARD, 0.82, role="detail", size=[0.05, 0.04, 0.0012], transform={"translation": [cu, top - 0.02, cw - hw - 0.0006]}, texture=True),
        local("box-tape-end-l", "box", "box-r", BOARD, 0.82, role="detail", size=[0.05, 0.04, 0.0012], transform={"translation": [cu, top - 0.02, cw + hw + 0.0006]}, texture=True),
        local("box-label", "box", "box-r", BOARD, 0.9, role="texture", size=[0.001, 0.07, 0.10], transform={"translation": [cu + hu + 0.0005, cv + 0.03, cw + 0.08]}, texture=True),
    ]


def body(S):
    return head(S) + helmet(S) + trunk(S) + legs(S) + arms() + carton()
