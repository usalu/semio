"""🧍‍♀️ Round-5 body of the standing adult at 1:50, every cross-section stated by hand in the world of the rest pose (x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder joint 1.3975 at 0.153 from the midline, elbow 1.084, wrist 0.837, C7 1.465, head joint 1.561, vertex 1.710).

Reference (slim adult, stature 1.69 m barefoot, 59 kg; DIN 33402-2, ISO 7250 / ANSUR II, Winter 2009): head 0.222 high, 0.192 long, 0.147 broad; bizygomatic 0.130; biacromial 0.375, bideltoid 0.435 nude (0.46 in a drop-shoulder sweatshirt); chest 0.285 x 0.215; waist 0.250 x 0.175; hips 0.340 x 0.225; thigh girth 0.56 / 0.50 / 0.38; knee 0.36; calf 0.355; ankle 0.22; upper arm 0.265; forearm 0.245; wrist 0.055 x 0.038; hand 0.181 x 0.079 x 0.027, palm 0.101, middle finger 0.080; foot 0.250, low-top canvas sneaker 0.281 x 0.100, sole 26 mm at the heel and 20 mm at the ball, toe spring 12 mm.
"""
import math

from r3_standing_adult_kit import box, capsule, ellipsoid, free_loft, loft

SKIN, HAIR, FLEECE, DENIM, CANVAS, SOCK = "skin", "hair", "fleece-sweatshirt", "denim", "canvas-sneaker", "cotton-sock"
TOP = 1.710
HX = 0.0132
SH = 0.153
HIP = 0.085

FINGERS = [
    (0.0260, 0.7430, (0.037, 0.022, 0.018), (0.0084, 0.0077, 0.0068), (7, 15, 10)),
    (0.0090, 0.7390, (0.040, 0.025, 0.019), (0.0087, 0.0080, 0.0070), (8, 17, 11)),
    (-0.0080, 0.7420, (0.038, 0.024, 0.018), (0.0081, 0.0075, 0.0066), (9, 19, 12)),
    (-0.0240, 0.7510, (0.030, 0.019, 0.016), (0.0072, 0.0066, 0.0058), (10, 21, 12)),
]
THUMB = [(0.020, 0.816, -0.008), (0.036, 0.785, -0.014), (0.042, 0.753, -0.019), (0.044, 0.729, -0.022)]
THUMB_R = (0.0110, 0.0095, 0.0085, 0.0074)


def finger_points(i):
    """🖐️ Knuckle, middle joint, end joint and tip of finger `i` (index first) of the left hand, flexing gently towards the palm (towards the thigh) more with every finger from index to little finger."""
    fx, fy, lengths, _radii, angles = FINGERS[i]
    p = [(fx, fy, SH - 0.004)]
    a = 0.0
    for length, step in zip(lengths, angles):
        a += math.radians(step)
        q = p[-1]
        p.append((q[0], q[1] - length * math.cos(a), q[2] - length * math.sin(a)))
    return p


def thumb_points():
    """👍 Carpometacarpal joint, metacarpophalangeal joint, interphalangeal joint and tip of the left thumb, lying along the index finger ahead of the palm."""
    return [(x, y, SH + z) for x, y, z in THUMB]


def skull_rows():
    """💀 The skin of the head from vertex to chin: (height, centre x, 0, half breadth, half depth); brow at 0.098 below the vertex, eyes 0.114, cheekbones 0.135, mouth 0.181, jaw angle 0.195, chin 0.222."""
    d = lambda v: TOP - v
    return [
        (d(0.000), 0.019, 0, 0.010, 0.013),
        (d(0.004), 0.019, 0, 0.031, 0.041),
        (d(0.012), 0.018, 0, 0.047, 0.062),
        (d(0.025), 0.017, 0, 0.059, 0.078),
        (d(0.045), 0.016, 0, 0.0685, 0.090),
        (d(0.070), 0.017, 0, 0.0735, 0.0955),
        (d(0.095), 0.019, 0, 0.0730, 0.0955),
        (d(0.115), 0.021, 0, 0.0715, 0.091),
        (d(0.135), 0.025, 0, 0.0695, 0.085),
        (d(0.155), 0.031, 0, 0.0655, 0.077),
        (d(0.175), 0.038, 0, 0.0590, 0.068),
        (d(0.195), 0.046, 0, 0.0505, 0.058),
        (d(0.210), 0.054, 0, 0.0385, 0.046),
        (d(0.218), 0.060, 0, 0.0260, 0.034),
        (d(0.222), 0.064, 0, 0.0120, 0.020),
    ]


HAIRLINE = [(-0.0125, None, 0.013), (-0.009, None, 0.013), (-0.004, None, 0.013), (0.006, None, 0.013), (0.020, None, 0.012), (0.036, None, 0.007), (0.050, 0.082, 0.004), (0.070, 0.064, 0.007), (0.090, 0.036, 0.006), (0.108, 0.006, 0.006), (0.128, -0.018, 0.006), (0.148, -0.032, 0.006), (0.164, -0.040, 0.004)]


def skull_at(v):
    """🧭 Centre x, half breadth and half depth of the head skin at `v` below the vertex (linear between the stated rows)."""
    rows = skull_rows()
    for a, b in zip(rows, rows[1:]):
        if b[0] <= TOP - v <= a[0]:
            t = (a[0] - (TOP - v)) / (a[0] - b[0])
            return tuple(a[i] + (b[i] - a[i]) * t for i in (1, 3, 4))
    raise ValueError(v)


CROWN = {-0.0125: (0.016, 0.020), -0.009: (0.036, 0.046), -0.004: (0.048, 0.062), 0.006: (0.058, 0.075), 0.020: (0.0665, 0.086)}


def hair_rows():
    """💇 Short hair hugging the skull: the crown as a dome 12.5 mm above the vertex (`CROWN`: half breadth and half depth per depth below the vertex), below it per stated depth the hairline ahead (None = over the forehead) and the thickness of the hair (10 mm at the front of the crown, 6 mm over the temples and the back, 4 mm at the nape)."""
    rows = []
    for v, line, t in HAIRLINE:
        cx, w0, r0 = skull_at(max(v, 0.0))
        w, r = CROWN.get(v, (w0 + t, r0 + t))
        back = cx - r
        ahead = cx + r if line is None else max(min(line, cx + r), back + 0.03)
        mid = (ahead + back) / 2
        if v not in CROWN:
            w = w0 * max(1 - abs((mid - cx) / r0) ** 2.15, 0) ** (1 / 2.15) + t
        rows.append((TOP - v, mid, 0, w, (ahead - back) / 2))
    return rows


def head():
    d = lambda v: TOP - v
    nose = [
        (d(0.100), 0.109, 0, 0.008, 0.006),
        (d(0.118), 0.113, 0, 0.0095, 0.010),
        (d(0.136), 0.118, 0, 0.0115, 0.0135),
        (d(0.150), 0.121, 0, 0.0150, 0.0165),
        (d(0.158), 0.117, 0, 0.0150, 0.0130),
        (d(0.163), 0.111, 0, 0.0110, 0.0070),
    ]
    hair = hair_rows()
    out = [
        loft("head", "head", SKIN, 0.25, skull_rows(), "y", e=2.15, sub=2),
        loft("nose", "head", SKIN, 0.25, nose, "y", e=2.0, sub=1, role="detail"),
        loft("hair", "head", HAIR, 0.75, hair, "y", e=2.1, sub=2),
        ellipsoid("lips", "head", SKIN, 0.2, (0.098, d(0.182), 0), (0.008, 0.0075, 0.021), role="detail"),
        ellipsoid("chin", "head", SKIN, 0.25, (0.085, d(0.206), 0), (0.0135, 0.012, 0.019), role="detail"),
    ]
    for side, mirror in (("l", False), ("r", True)):
        out.append(ellipsoid("ear-" + side, "head", SKIN, 0.3, (HX - 0.016, d(0.134), 0.0690), (0.015, 0.029, 0.010), role="detail", mirror=mirror, tilt=-12))
        out.append(ellipsoid("eye-" + side, "head", SKIN, 0.9, (0.097, d(0.114), 0.031), (0.005, 0.0055, 0.0125), role="detail", mirror=mirror))
        out.append(ellipsoid("brow-" + side, "head", SKIN, 0.2, (0.100, d(0.099), 0.032), (0.005, 0.0038, 0.019), role="detail", mirror=mirror))
    return out


def neck():
    rows = [
        (1.415, -0.006, 0, 0.066, 0.062),
        (1.450, -0.001, 0, 0.057, 0.057),
        (1.485, 0.006, 0, 0.053, 0.054),
        (1.515, 0.010, 0, 0.052, 0.054),
        (1.545, 0.014, 0, 0.052, 0.052),
        (1.572, 0.016, 0, 0.046, 0.044),
    ]
    return [loft("neck", "neck", SKIN, 0.25, rows, "y", e=2.0, sub=1)]


def front(rows, y, z):
    """📏 The x of the front surface of a "y" loft stated by `rows` at height `y` and lateral offset `z` (linear between the stated rows)."""
    for a, b in zip(rows, rows[1:]):
        if a[0] <= y <= b[0]:
            t = (y - a[0]) / (b[0] - a[0])
            cx, rz, rx, e = (a[i] + (b[i] - a[i]) * t for i in (1, 3, 4, 5))
            return cx + rx * max(1 - (z / rz) ** e, 0) ** (1 / e)
    raise ValueError(y)


def trunk():
    body = [
        (0.898, -0.016, 0, 0.176, 0.120, 2.4),
        (0.930, -0.013, 0, 0.180, 0.122, 2.4),
        (0.990, -0.008, 0, 0.172, 0.118, 2.4),
        (1.060, -0.005, 0, 0.163, 0.115, 2.4),
        (1.140, -0.003, 0, 0.160, 0.116, 2.4),
        (1.220, 0.000, 0, 0.162, 0.120, 2.4),
        (1.280, 0.000, 0, 0.165, 0.121, 2.4),
        (1.330, -0.004, 0, 0.174, 0.116, 2.5),
        (1.370, -0.010, 0, 0.192, 0.106, 2.7),
        (1.398, -0.014, 0, 0.203, 0.094, 2.9),
        (1.418, -0.016, 0, 0.198, 0.082, 2.8),
        (1.432, -0.016, 0, 0.180, 0.074, 2.6),
        (1.445, -0.014, 0, 0.148, 0.068, 2.4),
        (1.456, -0.012, 0, 0.114, 0.065, 2.2),
        (1.466, -0.009, 0, 0.086, 0.064, 2.0),
    ]
    hem = [
        (0.866, -0.017, 0, 0.170, 0.116, 2.4),
        (0.872, -0.017, 0, 0.174, 0.119, 2.4),
        (0.898, -0.016, 0, 0.1745, 0.1195, 2.4),
        (0.908, -0.015, 0, 0.172, 0.117, 2.4),
    ]
    pocket = [
        (0.930, 0.000, 0, 0.130, 0.116, 4.0),
        (0.945, 0.000, 0, 0.132, 0.118, 4.0),
        (1.010, 0.000, 0, 0.120, 0.118, 4.0),
        (1.075, 0.000, 0, 0.096, 0.117, 4.0),
        (1.085, 0.000, 0, 0.092, 0.115, 4.0),
    ]
    collar = [
        (1.446, -0.011, 0, 0.090, 0.071, 2.0),
        (1.458, -0.009, 0, 0.083, 0.069, 2.0),
        (1.469, -0.007, 0, 0.075, 0.067, 2.0),
        (1.475, -0.006, 0, 0.069, 0.064, 2.0),
    ]
    hood = [
        (1.488, -0.066, 0, 0.076, 0.018, 2.2),
        (1.476, -0.078, 0, 0.098, 0.032, 2.2),
        (1.450, -0.098, 0, 0.110, 0.042, 2.2),
        (1.410, -0.120, 0, 0.108, 0.040, 2.2),
        (1.365, -0.130, 0, 0.096, 0.032, 2.2),
        (1.325, -0.132, 0, 0.074, 0.024, 2.2),
        (1.295, -0.128, 0, 0.044, 0.014, 2.2),
    ]
    out = [
        loft("hoodie", "torso", FLEECE, 0.45, body, "y", sub=2),
        loft("hem-rib", "torso", FLEECE, 0.35, hem, "y", sub=0),
        loft("kangaroo-pocket", "torso", FLEECE, 0.42, pocket, "y", sub=1, role="detail"),
        loft("collar-rib", "torso", FLEECE, 0.4, collar, "y", sub=1),
        loft("hood", "torso", FLEECE, 0.4, hood, "y", sub=2),
    ]
    for side, mirror in (("l", False), ("r", True)):
        roll = [
            (-0.078, 1.462, 0.070, 0.020, 0.020),
            (-0.044, 1.471, 0.086, 0.024, 0.022),
            (-0.006, 1.467, 0.086, 0.023, 0.021),
            (0.032, 1.452, 0.070, 0.020, 0.018),
            (0.062, 1.430, 0.046, 0.016, 0.015),
            (0.084, 1.406, 0.022, 0.011, 0.010),
        ]
        out.append(free_loft("hood-roll-" + side, "torso", FLEECE, 0.4, roll, sub=1, mirror=mirror))
        out.append(free_loft("drawstring-" + side, "torso", FLEECE, 0.15, [(front(body, y, z) + 0.0025, y, z, 0.003, 0.003) for y, z in ((1.400, 0.030), (1.370, 0.032), (1.335, 0.034), (1.300, 0.035), (1.262, 0.036))], role="detail", mirror=mirror, slender=True))
    return out


def pelvis():
    seat = [
        (0.995, 0.001, 0, 0.150, 0.100, 2.3),
        (0.980, 0.000, 0, 0.156, 0.103, 2.4),
        (0.950, -0.006, 0, 0.166, 0.108, 2.4),
        (0.910, -0.012, 0, 0.170, 0.108, 2.4),
        (0.870, -0.012, 0, 0.165, 0.104, 2.3),
        (0.840, -0.016, 0, 0.138, 0.085, 2.2),
        (0.815, -0.014, 0, 0.088, 0.060, 2.0),
        (0.800, -0.012, 0, 0.040, 0.034, 2.0),
    ]
    out = [loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1)]
    for side, mirror in (("l", False), ("r", True)):
        out.append(ellipsoid("buttock-" + side, "pelvis", DENIM, 0.62, (-0.058, 0.884, 0.068), (0.058, 0.078, 0.062), mirror=mirror))
    return out


def arm(side, mirror):
    s = "-" + side
    upper = [
        (1.415, -0.010, SH + 0.018, 0.034, 0.040),
        (1.400, -0.009, SH + 0.020, 0.052, 0.058),
        (1.370, -0.007, SH + 0.020, 0.061, 0.066),
        (1.320, -0.005, SH + 0.017, 0.061, 0.066),
        (1.250, -0.003, SH + 0.012, 0.057, 0.062),
        (1.170, -0.001, SH + 0.007, 0.053, 0.058),
        (1.100, 0.001, SH + 0.003, 0.051, 0.056),
        (1.070, 0.001, SH + 0.002, 0.049, 0.054),
        (1.050, 0.001, SH, 0.040, 0.045),
        (1.040, 0.000, SH, 0.022, 0.026),
    ]
    lower = [
        (1.118, 0.000, SH, 0.026, 0.030),
        (1.105, 0.001, SH, 0.044, 0.048),
        (1.080, 0.003, SH, 0.0485, 0.0525),
        (1.000, 0.004, SH, 0.0460, 0.0500),
        (0.930, 0.002, SH, 0.0430, 0.0470),
        (0.902, 0.001, SH, 0.0450, 0.0490),
        (0.884, 0.000, SH, 0.0400, 0.0450),
        (0.876, 0.000, SH, 0.0300, 0.0350),
    ]
    cuff = [
        (0.886, 0.000, SH, 0.0285, 0.0335),
        (0.872, 0.000, SH, 0.0295, 0.0345),
        (0.842, 0.000, SH, 0.0275, 0.0330),
        (0.836, 0.000, SH, 0.0255, 0.0310),
    ]
    palm = [
        (0.852, -0.002, SH, 0.0170, 0.026, 2.2),
        (0.832, -0.001, SH, 0.0155, 0.030, 2.3),
        (0.810, 0.000, SH - 0.001, 0.0150, 0.036, 2.4),
        (0.785, 0.000, SH - 0.002, 0.0150, 0.0395, 2.5),
        (0.760, 0.000, SH - 0.003, 0.0142, 0.0405, 2.5),
        (0.743, -0.001, SH - 0.005, 0.0130, 0.0395, 2.4),
        (0.733, -0.001, SH - 0.007, 0.0110, 0.0360, 2.3),
        (0.726, -0.001, SH - 0.009, 0.0075, 0.0300, 2.2),
    ]
    out = [
        loft("sleeve" + s, "arm" + s, FLEECE, 0.45, upper, "y", sub=2, mirror=mirror),
        loft("forearm" + s, "forearm" + s, FLEECE, 0.45, lower, "y", sub=2, mirror=mirror),
        loft("cuff-rib" + s, "forearm" + s, FLEECE, 0.35, cuff, "y", mirror=mirror),
        loft("hand" + s, "hand" + s, SKIN, 0.3, palm, "y", sub=1, mirror=mirror),
        ellipsoid("thenar" + s, "hand" + s, SKIN, 0.3, (0.022, 0.795, SH - 0.010), (0.016, 0.027, 0.012), mirror=mirror),
    ]
    t = thumb_points()
    for k in range(3):
        out.append(capsule("thumb%s-%d" % (s, k), "hand" + s, SKIN, 0.3, t[k], t[k + 1], THUMB_R[k], THUMB_R[k + 1], mirror=mirror))
    for i, (_fx, _fy, _lengths, radii, _angles) in enumerate(FINGERS):
        p = finger_points(i)
        rr = list(radii) + [radii[-1] * 0.9]
        for k, part in enumerate("abc"):
            out.append(capsule("finger%s-%d%s" % (s, i, part), "hand" + s, SKIN, 0.3, p[k], p[k + 1], rr[k], rr[k + 1], mirror=mirror))
    return out


def shoe_top(x):
    """👟 Height of the top line of the sneaker upper at `x` ahead of the ankle (collar at the heel, instep, vamp, toe box)."""
    pts = [(-0.072, 0.078), (-0.064, 0.086), (-0.040, 0.085), (-0.010, 0.080), (0.020, 0.086), (0.050, 0.080), (0.085, 0.068), (0.120, 0.056), (0.150, 0.049)]
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return pts[-1][1]


def leg(side, mirror):
    s = "-" + side
    thigh = [
        (0.985, -0.012, 0.093, 0.040, 0.050),
        (0.940, -0.012, 0.093, 0.078, 0.092),
        (0.880, -0.010, 0.092, 0.084, 0.094),
        (0.800, -0.004, 0.090, 0.080, 0.089),
        (0.720, 0.001, 0.088, 0.074, 0.082),
        (0.640, 0.004, 0.086, 0.069, 0.076),
        (0.570, 0.005, 0.085, 0.061, 0.066),
        (0.530, 0.006, 0.084, 0.056, 0.060),
        (0.500, 0.008, 0.084, 0.054, 0.058),
        (0.470, 0.006, 0.084, 0.050, 0.055),
        (0.452, 0.004, 0.084, 0.040, 0.045),
        (0.442, 0.003, 0.084, 0.020, 0.024),
    ]
    shank = [
        (0.540, 0.004, HIP, 0.030, 0.034),
        (0.520, 0.006, HIP, 0.050, 0.055),
        (0.490, 0.006, HIP, 0.052, 0.057),
        (0.440, 0.000, HIP, 0.051, 0.057),
        (0.380, -0.008, HIP, 0.053, 0.062),
        (0.330, -0.009, HIP, 0.051, 0.059),
        (0.270, -0.006, HIP, 0.047, 0.053),
        (0.200, -0.004, HIP, 0.045, 0.049),
        (0.140, -0.003, HIP, 0.045, 0.049),
        (0.115, -0.003, HIP, 0.046, 0.050),
        (0.105, -0.003, HIP, 0.044, 0.048),
    ]
    ankle = [
        (0.112, -0.004, HIP, 0.031, 0.037),
        (0.095, -0.004, HIP, 0.033, 0.039),
        (0.075, -0.004, HIP, 0.034, 0.041),
        (0.055, -0.004, HIP, 0.032, 0.040),
    ]
    sole = [
        (-0.0745, 0.014, HIP, 0.016, 0.012, 3.0),
        (-0.068, 0.013, HIP, 0.028, 0.013, 3.5),
        (-0.050, 0.013, HIP, 0.034, 0.013, 4.0),
        (0.000, 0.012, HIP, 0.034, 0.012, 4.0),
        (0.050, 0.011, HIP + 0.001, 0.039, 0.011, 4.0),
        (0.100, 0.010, HIP + 0.002, 0.047, 0.010, 4.0),
        (0.1496, 0.010, HIP + 0.003, 0.050, 0.010, 4.0),
        (0.156, 0.010, HIP + 0.003, 0.0495, 0.010, 4.0),
    ]
    toe_sole = [
        (0.143, 0.010, HIP + 0.003, 0.050, 0.010, 4.0),
        (0.1496, 0.010, HIP + 0.003, 0.050, 0.010, 4.0),
        (0.175, 0.0125, HIP + 0.002, 0.046, 0.010, 4.0),
        (0.192, 0.0175, HIP + 0.001, 0.037, 0.0095, 3.5),
        (0.202, 0.0225, HIP, 0.024, 0.009, 3.0),
        (0.2065, 0.0260, HIP, 0.010, 0.007, 3.0),
    ]
    upper = [
        (-0.072, 0.050, HIP, 0.016, 0.028, 2.4),
        (-0.064, 0.052, HIP, 0.028, 0.034, 2.4),
        (-0.040, 0.052, HIP, 0.033, 0.033, 2.4),
        (-0.010, 0.050, HIP, 0.035, 0.030, 2.4),
        (0.020, 0.052, HIP, 0.037, 0.034, 2.3),
        (0.050, 0.050, HIP + 0.001, 0.040, 0.030, 2.3),
        (0.085, 0.044, HIP + 0.002, 0.044, 0.024, 2.4),
        (0.120, 0.038, HIP + 0.003, 0.047, 0.018, 2.5),
        (0.1496, 0.034, HIP + 0.003, 0.047, 0.015, 2.6),
        (0.156, 0.034, HIP + 0.003, 0.0465, 0.0148, 2.6),
    ]
    toe_box = [
        (0.143, 0.035, HIP + 0.003, 0.047, 0.016, 2.6),
        (0.1496, 0.034, HIP + 0.003, 0.047, 0.015, 2.6),
        (0.165, 0.033, HIP + 0.003, 0.045, 0.0145, 2.6),
    ]
    toe_cap = [
        (0.160, 0.0340, HIP + 0.003, 0.0475, 0.0160, 2.6),
        (0.178, 0.0340, HIP + 0.002, 0.0425, 0.0150, 2.5),
        (0.192, 0.0345, HIP + 0.001, 0.0345, 0.0130, 2.4),
        (0.201, 0.0345, HIP, 0.0215, 0.0100, 2.2),
        (0.2055, 0.0340, HIP, 0.0090, 0.0060, 2.0),
    ]
    out = [
        loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=2, mirror=mirror),
        ellipsoid("kneecap" + s, "thigh" + s, DENIM, 0.62, (0.038, 0.497, HIP - 0.002), (0.022, 0.032, 0.028), mirror=mirror),
        loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=2, mirror=mirror),
        loft("sock" + s, "shank" + s, SOCK, 0.8, ankle, "y", mirror=mirror),
        loft("sole" + s, "foot" + s, CANVAS, 0.88, sole, "x", sub=1, mirror=mirror),
        loft("shoe" + s, "foot" + s, CANVAS, 0.12, upper, "x", sub=2, mirror=mirror),
        loft("toe-sole" + s, "toe" + s, CANVAS, 0.88, toe_sole, "x", sub=1, mirror=mirror),
        loft("shoe-toe" + s, "toe" + s, CANVAS, 0.12, toe_box, "x", mirror=mirror),
        loft("toe-cap" + s, "toe" + s, CANVAS, 0.88, toe_cap, "x", sub=1, mirror=mirror),
    ]
    for i, lx in enumerate((0.040, 0.060, 0.080, 0.100)):
        out.append(box("lace%s-%d" % (s, i), "foot" + s, CANVAS, 0.85, (lx, shoe_top(lx) - 0.002, HIP + 0.001), (0.007, 0.004, 0.030 + i * 0.003), role="texture", mirror=mirror, tilt=-22, texture=True))
    return out


def body():
    return head() + neck() + trunk() + pelvis() + arm("l", False) + arm("r", True) + leg("l", False) + leg("r", True)
