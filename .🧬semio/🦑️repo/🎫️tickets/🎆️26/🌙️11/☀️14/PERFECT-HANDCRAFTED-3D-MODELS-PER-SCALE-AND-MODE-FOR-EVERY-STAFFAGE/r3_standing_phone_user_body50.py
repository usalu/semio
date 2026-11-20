"""🤳 Body of the standing phone user at 1:50, every cross-section stated by hand. Trunk, legs, sleeves and head are stated in the world of the rest pose (x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder 1.3975, C7 1.465, head joint 1.561, vertex 1.710); the ponytail hangs under gravity in the texting pose and is stated in the world of that pose; hands and phone are stated in the grip frame of the phone (a along the phone, b across it towards the figure's left, n out of the screen; see `r3_standing_phone_user_kit.grip`).

Anthropometry (slim woman, stature 1.69 m, 59 kg): head 0.222 high, 0.190 long, 0.148 broad; biacromial 0.375, bideltoid 0.435 nude; chest 0.285 x 0.215; waist 0.250 x 0.175; hips 0.340 x 0.225; thigh girth 0.56 / 0.50 / 0.38; knee 0.105 x 0.115; calf girth 0.355; ankle 0.058 x 0.075; upper arm girth 0.265; forearm 0.245; wrist 0.055 x 0.038; hand 0.181 x 0.079 x 0.027, index 0.072, middle 0.079, ring 0.073, little 0.058, thumb 0.060 past its knuckle; foot 0.250, sneaker 0.275 x 0.100 on a 0.020 sole. Phone 0.147 x 0.0715 x 0.0078, corner radius 0.010. Over-ear headphones: cups 0.086 across and 0.046 deep with the cushion, band 0.024 wide, 0.36 long.
"""
import math

import r3_standing_phone_user_parts as parts

from r3_standing_phone_user_kit import HAND_U, HAND_V, PHONE_LENGTH, PHONE_THICK, PHONE_WIDTH, both, box, capsule, ellipsoid, ellipsoid_axes, frame_loft, free_loft, grip, grip_dir, hair_cap, loft, on_hand, posed, rounded_plate

SKIN, HAIR, FLEECE, DENIM, CANVAS, SOCK, POLYMER, GLASS = "skin", "hair", "fleece-sweatshirt", "denim", "canvas-sneaker", "cotton-sock", "headphone-polymer", "phone-glass-metal"
TOP = 1.710
HX = 0.0132
SH = 0.153
HIP = 0.085


FINGERS = {
    "r": [
        (0.092, 0.028, -5, (0.040, 0.023, 0.019), 0.0088, (0.003, 0.0112, 0.0112, 0.0115)),
        (0.095, 0.008, 0, (0.044, 0.027, 0.020), 0.0090, (0.003, 0.0110, 0.0110, 0.0113)),
        (0.090, -0.011, 5, (0.041, 0.025, 0.019), 0.0086, (0.003, 0.0114, 0.0114, 0.0117)),
        (0.081, -0.029, 11, (0.032, 0.018, 0.017), 0.0078, (0.003, 0.0122, 0.0122, 0.0124)),
    ],
    "l": [
        (0.092, 0.028, -5, (0.040, 0.023, 0.019), 0.0088, (0.003, 0.0190, 0.0200, 0.0200)),
        (0.095, 0.008, 0, (0.044, 0.027, 0.020), 0.0090, (0.003, 0.0190, 0.0198, 0.0198)),
        (0.090, -0.011, 5, (0.041, 0.025, 0.019), 0.0086, (0.003, 0.0120, 0.0150, 0.0160)),
        (0.081, -0.029, 11, (0.032, 0.018, 0.017), 0.0078, (0.003, 0.0040, 0.0060, 0.0080)),
    ],
}
THUMBS = {
    "r": [on_hand(0.022, 0.026, 0.004), (-0.030, -0.060, 0.002), (-0.006, -0.041, 0.0128), (0.012, -0.023, 0.0126)],
    "l": [on_hand(0.022, 0.026, 0.004), (-0.034, -0.060, 0.020), (-0.010, -0.042, 0.0380), (0.010, -0.025, 0.0386)],
}


def finger_points(i, hand="r"):
    """🖐 Knuckle, middle joint, end joint and tip of finger `i` (index first) in the grip frame of its hand: the finger leaves the palm at (s, t) on the hand, turned `spread` degrees away from the thumb, and its joints lie `rise` above the hand plane (towards the phone)."""
    s, t, spread, lengths, _r, rise = FINGERS[hand][i]
    k = math.radians(40 + spread)
    d = (math.cos(k), math.sin(k))
    p = on_hand(s, t)
    out = [(p[0], p[1], p[2] + rise[0])]
    for length, up in zip(lengths, rise[1:]):
        p = (p[0] + d[0] * length, p[1] + d[1] * length, p[2])
        out.append((p[0], p[1], p[2] + up))
    return out


def skull_rows():
    """💀 The skin of the head from vertex to chin: (height, centre x, 0, half breadth, half depth)."""
    d = lambda v: TOP - v
    return [
        (d(0.000), 0.020, 0, 0.010, 0.013),
        (d(0.004), 0.020, 0, 0.030, 0.040),
        (d(0.012), 0.020, 0, 0.045, 0.060),
        (d(0.025), 0.019, 0, 0.058, 0.076),
        (d(0.045), 0.019, 0, 0.068, 0.089),
        (d(0.070), 0.020, 0, 0.0735, 0.095),
        (d(0.095), 0.0215, 0, 0.072, 0.095),
        (d(0.115), 0.022, 0, 0.069, 0.089),
        (d(0.135), 0.026, 0, 0.067, 0.083),
        (d(0.160), 0.034, 0, 0.061, 0.073),
        (d(0.180), 0.040, 0, 0.055, 0.065),
        (d(0.200), 0.047, 0, 0.046, 0.055),
        (d(0.213), 0.053, 0, 0.034, 0.043),
        (d(0.222), 0.060, 0, 0.018, 0.026),
    ]


def head():
    d = lambda v: TOP - v
    skull = skull_rows()
    nose = [
        (d(0.100), 0.110, 0, 0.008, 0.006),
        (d(0.118), 0.114, 0, 0.0095, 0.010),
        (d(0.136), 0.119, 0, 0.012, 0.014),
        (d(0.150), 0.122, 0, 0.0155, 0.017),
        (d(0.158), 0.118, 0, 0.015, 0.013),
        (d(0.163), 0.112, 0, 0.011, 0.007),
    ]
    out = [
        loft("head", "head", SKIN, 0.25, skull, "y", e=2.15, sub=2),
        loft("nose", "head", SKIN, 0.25, nose, "y", e=2.0, sub=1, role="detail"),
        hair_cap("hair", "head", HAIR, 0.75, (0, 0.08, 0.18, 0.30, 0.42, 0.54, 0.66, 0.77, 0.86, 0.93, 0.975, 0.995)),
        ellipsoid("lips", "head", SKIN, 0.2, (0.101, d(0.181), 0), (0.008, 0.008, 0.023), role="detail"),
        ellipsoid("chin", "head", SKIN, 0.25, (0.088, d(0.207), 0), (0.016, 0.015, 0.022), role="detail"),
    ]
    for side, mirror in (("l", False), ("r", True)):
        out.append(ellipsoid("ear-" + side, "head", SKIN, 0.3, (HX - 0.016, d(0.134), 0.0685), (0.015, 0.029, 0.010), role="detail", mirror=mirror, tilt=-12))
        out.append(ellipsoid("eye-" + side, "head", SKIN, 0.9, (0.098, d(0.114), 0.031), (0.005, 0.0055, 0.0125), role="detail", mirror=mirror))
        out.append(ellipsoid("brow-" + side, "head", SKIN, 0.2, (0.101, d(0.098), 0.032), (0.006, 0.005, 0.021), role="detail", mirror=mirror))
    with posed():
        out.append(parts.tail("ponytail", parts.TAILS[50]))
        out.append(ellipsoid_axes("pony-tie", "head", HAIR, 0.2, (0.024, 1.642, 0), (-0.95, 0.30, 0), (0, 0, 1), (0.007, 0.019, 0.019), role="detail"))
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


def trunk():
    body = [
        (0.898, -0.016, 0, 0.186, 0.123, 2.4),
        (0.940, -0.011, 0, 0.183, 0.122, 2.4),
        (1.000, -0.005, 0, 0.171, 0.117, 2.4),
        (1.080, -0.003, 0, 0.161, 0.114, 2.4),
        (1.160, -0.002, 0, 0.160, 0.116, 2.4),
        (1.245, 0.003, 0, 0.162, 0.122, 2.4),
        (1.300, -0.003, 0, 0.165, 0.120, 2.4),
        (1.350, -0.010, 0, 0.174, 0.110, 2.4),
        (1.385, -0.014, 0, 0.186, 0.098, 2.5),
        (1.406, -0.016, 0, 0.188, 0.087, 2.6),
        (1.424, -0.016, 0, 0.187, 0.077, 2.6),
        (1.438, -0.015, 0, 0.179, 0.070, 2.6),
        (1.449, -0.013, 0, 0.150, 0.065, 2.4),
        (1.458, -0.011, 0, 0.112, 0.063, 2.2),
        (1.466, -0.009, 0, 0.086, 0.063, 2.0),
    ]
    hem = [
        (0.866, -0.017, 0, 0.182, 0.119, 2.4),
        (0.872, -0.017, 0, 0.186, 0.122, 2.4),
        (0.898, -0.016, 0, 0.1865, 0.1225, 2.4),
        (0.908, -0.015, 0, 0.184, 0.120, 2.4),
    ]
    pocket = [
        (0.930, -0.001, 0, 0.132, 0.121, 4.0),
        (0.945, 0.000, 0, 0.134, 0.122, 4.0),
        (1.010, 0.004, 0, 0.122, 0.116, 4.0),
        (1.075, 0.006, 0, 0.098, 0.113, 4.0),
        (1.085, 0.005, 0, 0.094, 0.111, 4.0),
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
        (1.410, -0.122, 0, 0.108, 0.040, 2.2),
        (1.365, -0.134, 0, 0.096, 0.032, 2.2),
        (1.325, -0.136, 0, 0.074, 0.024, 2.2),
        (1.295, -0.132, 0, 0.044, 0.014, 2.2),
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
        out.append(capsule("drawstring-" + side, "torso", FLEECE, 0.15, (0.090, 1.396, 0.030), (0.126, 1.262, 0.036), 0.003, 0.003, role="detail", mirror=mirror, slender=True))
    out += parts.band_halves(0.3, 0.006, 0.015)
    for side, sign in (("l", 1), ("r", -1)):
        a, b, along = parts.cup_frame(sign)
        shell = [(along(-0.006), a, b, 0.040, 0.043), (along(0.008), a, b, 0.041, 0.044), (along(0.018), a, b, 0.037, 0.040), (along(0.024), a, b, 0.028, 0.031), (along(0.027), a, b, 0.014, 0.016)]
        cushion = [(along(-0.024), a, b, 0.036, 0.040), (along(-0.020), a, b, 0.042, 0.045), (along(-0.011), a, b, 0.043, 0.046), (along(-0.005), a, b, 0.041, 0.044)]
        out.append(frame_loft("headphone-cup-" + side, "torso", POLYMER, 0.85, shell))
        out.append(frame_loft("headphone-cushion-" + side, "torso", POLYMER, 0.4, cushion, e=2.6))
        out.append(capsule("headphone-yoke-" + side, "torso", POLYMER, 0.3, (0.078, 1.452, sign * 0.096), along(0.022), 0.0055, 0.0065, role="detail"))
    return out


def pelvis():
    seat = [
        (1.030, 0.006, 0, 0.137, 0.092, 2.3),
        (0.990, 0.002, 0, 0.151, 0.100, 2.3),
        (0.950, -0.006, 0, 0.163, 0.106, 2.3),
        (0.915, -0.012, 0, 0.169, 0.106, 2.3),
        (0.880, -0.012, 0, 0.169, 0.104, 2.3),
        (0.850, -0.010, 0, 0.164, 0.100, 2.3),
        (0.825, -0.006, 0, 0.146, 0.088, 2.2),
        (0.806, -0.004, 0, 0.104, 0.066, 2.0),
        (0.796, -0.004, 0, 0.056, 0.040, 2.0),
    ]
    out = [loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1)]
    for side, mirror in (("l", False), ("r", True)):
        out.append(ellipsoid("buttock-" + side, "pelvis", DENIM, 0.62, (-0.058, 0.892, 0.070), (0.058, 0.082, 0.066), mirror=mirror))
    return out


def arm(side, mirror):
    s = "-" + side
    upper = [
        (1.438, -0.008, SH + 0.006, 0.020, 0.024),
        (1.430, -0.008, SH + 0.010, 0.038, 0.043),
        (1.412, -0.007, SH + 0.014, 0.053, 0.059),
        (1.376, -0.006, SH + 0.016, 0.060, 0.066),
        (1.330, -0.005, SH + 0.014, 0.058, 0.064),
        (1.280, -0.003, SH + 0.010, 0.054, 0.060),
        (1.200, -0.001, SH + 0.006, 0.051, 0.056),
        (1.130, 0.000, SH + 0.003, 0.049, 0.053),
        (1.084, 0.000, SH, 0.0485, 0.0515),
        (1.066, 0.000, SH, 0.0475, 0.0505),
        (1.054, 0.000, SH, 0.041, 0.044),
        (1.046, 0.000, SH, 0.028, 0.030),
        (1.041, 0.000, SH, 0.013, 0.014),
    ]
    lower = [
        (1.124, 0.000, SH, 0.013, 0.014),
        (1.118, 0.000, SH, 0.027, 0.029),
        (1.104, 0.000, SH, 0.037, 0.040),
        (1.084, 0.000, SH, 0.0425, 0.0455),
        (1.040, 0.004, SH, 0.0455, 0.0495),
        (0.980, 0.003, SH, 0.043, 0.046),
        (0.920, 0.001, SH, 0.039, 0.042),
        (0.890, 0.000, SH, 0.038, 0.042),
        (0.876, 0.000, SH, 0.036, 0.041),
    ]
    cuff = [
        (0.884, 0.000, SH, 0.028, 0.0335),
        (0.870, 0.000, SH, 0.029, 0.0345),
        (0.838, 0.000, SH, 0.0275, 0.033),
        (0.832, 0.000, SH, 0.0255, 0.031),
    ]
    out = [
        loft("sleeve" + s, "arm" + s, FLEECE, 0.45, upper, "y", sub=2, mirror=mirror),
        loft("forearm" + s, "forearm" + s, FLEECE, 0.45, lower, "y", sub=2, mirror=mirror),
        loft("cuff-rib" + s, "forearm" + s, FLEECE, 0.35, cuff, "y", mirror=mirror),
    ]
    return out + hand(side)


def hand(side):
    """✋ One hand in the texting grip, stated in the grip frame: the right hand lies under the lower right of the phone, its fingers flat against the back of the phone, the thumb over the edge on the screen; the left hand is the mirror image 26 mm further behind, its index and middle finger pressed up under the fingers of the right hand, ring and little finger passing under the right palm, its thumb reaching up over the left edge."""
    s = "-" + side
    bone = "hand" + s
    with posed():
        g = lambda p: grip(p, side)
        width, thick = grip_dir(HAND_V, side), grip_dir((0, 0, 1), side)
        palm = [
            (g(on_hand(-0.016, 0.000, 0.000)), width, thick, 0.025, 0.0170, 2.2),
            (g(on_hand(0.008, 0.001, 0.000)), width, thick, 0.029, 0.0165, 2.2),
            (g(on_hand(0.034, 0.002, 0.000)), width, thick, 0.036, 0.0150, 2.3),
            (g(on_hand(0.060, 0.001, 0.001)), width, thick, 0.040, 0.0140, 2.4),
            (g(on_hand(0.082, -0.001, 0.002)), width, thick, 0.041, 0.0130, 2.4),
            (g(on_hand(0.094, -0.001, 0.003)), width, thick, 0.038, 0.0105, 2.4),
        ]
        out = [
            frame_loft("hand" + s, bone, SKIN, 0.3, palm),
            ellipsoid_axes("thenar" + s, bone, SKIN, 0.3, g(on_hand(0.036, 0.024, 0.008)), grip_dir(HAND_U, side), grip_dir(HAND_V, side), (0.030, 0.016, 0.013)),
        ]
        t = [g(p) for p in THUMBS[side]]
        out += [
            capsule("thumb" + s + "-0", bone, SKIN, 0.3, t[0], t[1], 0.0125, 0.0105),
            capsule("thumb" + s + "-1", bone, SKIN, 0.3, t[1], t[2], 0.0105, 0.0092),
            capsule("thumb" + s + "-2", bone, SKIN, 0.3, t[2], t[3], 0.0092, 0.0080),
        ]
        for i in range(4):
            r = FINGERS[side][i][4]
            q = [g(p) for p in finger_points(i, side)]
            out.append(capsule("finger%s-%da" % (s, i), bone, SKIN, 0.3, q[0], q[1], r, r * 0.95))
            out.append(capsule("finger%s-%db" % (s, i), bone, SKIN, 0.3, q[1], q[2], r * 0.95, r * 0.88))
            out.append(capsule("finger%s-%dc" % (s, i), bone, SKIN, 0.3, q[2], q[3], r * 0.88, r * 0.80))
    return out


def leg(side, mirror):
    s = "-" + side
    thigh = [
        (0.978, -0.010, HIP + 0.003, 0.028, 0.032),
        (0.962, -0.010, HIP + 0.003, 0.056, 0.064),
        (0.936, -0.010, HIP + 0.003, 0.075, 0.085),
        (0.903, -0.010, HIP + 0.003, 0.083, 0.093),
        (0.860, -0.010, HIP + 0.003, 0.084, 0.094),
        (0.815, -0.008, HIP + 0.004, 0.082, 0.092),
        (0.740, -0.004, HIP + 0.003, 0.077, 0.086),
        (0.660, 0.000, HIP + 0.001, 0.069, 0.077),
        (0.580, 0.003, HIP - 0.001, 0.061, 0.067),
        (0.530, 0.004, HIP - 0.002, 0.056, 0.062),
        (0.4898, 0.004, HIP - 0.002, 0.055, 0.060),
        (0.476, 0.003, HIP - 0.002, 0.0535, 0.0575),
        (0.464, 0.003, HIP - 0.002, 0.045, 0.049),
        (0.454, 0.003, HIP - 0.002, 0.030, 0.033),
        (0.448, 0.003, HIP - 0.002, 0.013, 0.014),
    ]
    shank = [
        (0.538, 0.003, HIP - 0.002, 0.016, 0.018),
        (0.531, 0.003, HIP - 0.002, 0.032, 0.036),
        (0.513, 0.003, HIP - 0.002, 0.044, 0.049),
        (0.4898, 0.003, HIP - 0.002, 0.049, 0.054),
        (0.455, 0.000, HIP - 0.002, 0.0505, 0.0555),
        (0.400, -0.006, HIP - 0.001, 0.0525, 0.058),
        (0.358, -0.010, HIP, 0.055, 0.062),
        (0.300, -0.009, HIP, 0.051, 0.056),
        (0.240, -0.006, HIP, 0.047, 0.051),
        (0.180, -0.004, HIP, 0.045, 0.049),
        (0.128, -0.004, HIP, 0.0445, 0.0495),
        (0.120, -0.004, HIP, 0.0435, 0.0485),
    ]
    ankle = [
        (0.160, -0.004, HIP, 0.031, 0.037),
        (0.118, -0.004, HIP, 0.030, 0.0375),
        (0.092, -0.004, HIP, 0.034, 0.039),
        (0.060, -0.004, HIP, 0.031, 0.036),
    ]
    sole = [
        (-0.0745, 0.012, HIP, 0.018, 0.012, 3.0),
        (-0.068, 0.012, HIP, 0.027, 0.012, 3.5),
        (-0.045, 0.012, HIP, 0.0335, 0.012, 4.0),
        (0.000, 0.012, HIP, 0.035, 0.012, 4.0),
        (0.050, 0.012, HIP + 0.001, 0.039, 0.012, 4.0),
        (0.100, 0.012, HIP + 0.002, 0.047, 0.012, 4.0),
        (0.130, 0.012, HIP + 0.003, 0.0505, 0.012, 4.0),
        (0.1496, 0.012, HIP + 0.003, 0.050, 0.012, 4.0),
        (0.156, 0.012, HIP + 0.003, 0.049, 0.012, 4.0),
    ]
    toe_sole = [
        (0.143, 0.012, HIP + 0.003, 0.050, 0.012, 4.0),
        (0.1496, 0.012, HIP + 0.003, 0.050, 0.012, 4.0),
        (0.175, 0.012, HIP + 0.002, 0.045, 0.012, 4.0),
        (0.192, 0.0125, HIP + 0.001, 0.035, 0.0115, 3.5),
        (0.201, 0.013, HIP, 0.022, 0.011, 3.0),
        (0.205, 0.014, HIP, 0.010, 0.009, 3.0),
    ]
    upper = [
        (-0.071, 0.054, HIP, 0.015, 0.032, 2.4),
        (-0.062, 0.056, HIP, 0.026, 0.036, 2.4),
        (-0.036, 0.055, HIP, 0.0315, 0.035, 2.4),
        (-0.005, 0.055, HIP, 0.034, 0.035, 2.4),
        (0.030, 0.056, HIP, 0.037, 0.036, 2.3),
        (0.065, 0.051, HIP + 0.001, 0.041, 0.031, 2.3),
        (0.100, 0.046, HIP + 0.002, 0.045, 0.026, 2.4),
        (0.130, 0.041, HIP + 0.003, 0.0475, 0.021, 2.5),
        (0.1496, 0.038, HIP + 0.003, 0.047, 0.018, 2.6),
        (0.156, 0.037, HIP + 0.003, 0.046, 0.017, 2.6),
    ]
    toe_box = [
        (0.143, 0.039, HIP + 0.003, 0.047, 0.019, 2.6),
        (0.1496, 0.038, HIP + 0.003, 0.047, 0.018, 2.6),
        (0.160, 0.0375, HIP + 0.003, 0.046, 0.0175, 2.6),
    ]
    toe_cap = [
        (0.158, 0.038, HIP + 0.003, 0.0475, 0.0185, 2.6),
        (0.175, 0.0365, HIP + 0.002, 0.043, 0.017, 2.5),
        (0.190, 0.034, HIP + 0.001, 0.033, 0.014, 2.4),
        (0.199, 0.031, HIP, 0.019, 0.010, 2.2),
        (0.2025, 0.029, HIP, 0.008, 0.006, 2.0),
    ]
    out = [
        loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=2, mirror=mirror),
        ellipsoid("kneecap" + s, "thigh" + s, DENIM, 0.62, (0.050, 0.500, HIP - 0.002), (0.017, 0.036, 0.031), mirror=mirror),
        loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=2, mirror=mirror),
        loft("sock" + s, "shank" + s, SOCK, 0.8, ankle, "y", mirror=mirror),
        loft("sole" + s, "foot" + s, CANVAS, 0.88, sole, "x", sub=1, mirror=mirror),
        loft("shoe" + s, "foot" + s, CANVAS, 0.12, upper, "x", sub=2, mirror=mirror),
        loft("toe-sole" + s, "toe" + s, CANVAS, 0.88, toe_sole, "x", sub=1, mirror=mirror),
        loft("shoe-toe" + s, "toe" + s, CANVAS, 0.12, toe_box, "x", mirror=mirror),
        loft("toe-cap" + s, "toe" + s, CANVAS, 0.88, toe_cap, "x", sub=1, mirror=mirror),
    ]
    for i, lx in enumerate((0.048, 0.068, 0.088, 0.108)):
        top = 0.092 - (lx - 0.030) * 0.44
        out.append(box("lace%s-%d" % (s, i), "foot" + s, CANVAS, 0.85, (lx, top - 0.001, HIP + 0.001), (0.007, 0.004, 0.034 + i * 0.003), role="texture", mirror=mirror, tilt=-24, texture=True))
    return out


def phone():
    """📱 The phone in the frame of its own bone (origin at the middle of its lower edge, u along it, w out of the screen): aluminium body with rounded corners and the glass over it."""
    half = PHONE_THICK / 2
    return [
        rounded_plate("phone", "phone", GLASS, 0.92, 0.0, PHONE_LENGTH, PHONE_WIDTH / 2, 0.010, -half, half),
        rounded_plate("phone-screen", "phone", GLASS, 0.12, 0.0, PHONE_LENGTH, PHONE_WIDTH / 2, 0.010, half, half + 0.0005, role="glazing", inset=0.003),
    ]


def body():
    return head() + neck() + trunk() + pelvis() + both(arm) + both(leg) + phone()
