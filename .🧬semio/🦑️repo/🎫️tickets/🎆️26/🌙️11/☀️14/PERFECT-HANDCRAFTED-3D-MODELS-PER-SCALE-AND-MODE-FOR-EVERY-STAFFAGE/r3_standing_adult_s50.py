"""🧍 The standing adult at 1:50: body (see `r3_standing_adult_body50`), the thermal model in space and the skeleton as the load path, stated by hand in the world of the rest pose (x ahead, y up, z left).

Skeleton landmarks (shod, 20 mm sole): vertex 1.704, atlanto-occipital joint 1.561, C7 1.465, sternal notch 1.405, xiphoid 1.21, iliac crest 1.035, lumbosacral joint 0.998, hip joint 0.903, knee 0.490, ankle 0.086; shoulder joint 1.3975 at 0.153 from the midline, elbow 1.084, wrist 0.837.
"""
import math

import r3_standing_adult_body50 as b50
from r3_standing_adult_body50 import HIP, SH, TOP
from r3_standing_adult_kit import rib_rows, face_rows, capsule, ellipsoid, free_loft, loft, ref, shrink, sphere, st, th

SKIN, HAIR, FLEECE, DENIM, CANVAS, SOCK, COTTON = "skin", "hair", "fleece-sweatshirt", "denim", "canvas-sneaker", "cotton-sock", "cotton-underwear"
BONE, JOINT, MASS = 0.85, 0.4, 0.2
SIDES = (("l", False), ("r", True))


def body():
    return b50.body()


def thermal():
    d = lambda v: TOP - v
    by = {s["id"]: s for s in body()}
    skull = b50.skull_rows()
    edge = [(d(0.048), 0.096), (d(0.058), 0.090), (d(0.072), 0.080), (d(0.090), 0.062), (d(0.108), 0.024), (d(0.128), 0.000), (d(0.146), -0.014), (d(0.162), -0.026), (d(0.176), -0.10), (d(0.200), -0.10), (d(0.213), -0.10), (d(0.221), -0.10)]
    out = [th(loft("th-face", "head", SKIN, 0.25, face_rows(skull, edge), "y", e=2.0), "face-skin")]
    out += [ref(i, "face-skin") for i in ("nose", "lips", "chin", "ear-l", "ear-r", "eye-l", "eye-r", "brow-l", "brow-r", "neck")]
    out.append(ref("hair", "hair"))
    for side, _ in SIDES:
        out += [ref(i, "hand-skin") for i in ["hand-" + side, "thenar-" + side, "thumb-%s-0" % side, "thumb-%s-1" % side] + ["finger-%s-%d%s" % (side, i, p) for i in range(4) for p in "ab"]]
    outer = ["hoodie", "hem-rib", "kangaroo-pocket", "collar-rib", "hood", "hood-roll-l", "hood-roll-r", "drawstring-l", "drawstring-r"] + [p + "-" + s for s, _ in SIDES for p in ("sleeve", "forearm", "cuff-rib")]
    out += [ref(i, "hoodie", "sweatshirt") for i in outer]
    tee = [
        (0.930, -0.010, 0, 0.176, 0.117, 2.4), (1.000, -0.005, 0, 0.167, 0.113, 2.4), (1.080, -0.003, 0, 0.157, 0.110, 2.4), (1.160, -0.002, 0, 0.156, 0.112, 2.4), (1.245, 0.003, 0, 0.158, 0.118, 2.4),
        (1.300, -0.003, 0, 0.161, 0.116, 2.4), (1.350, -0.010, 0, 0.170, 0.106, 2.4), (1.385, -0.014, 0, 0.182, 0.094, 2.5), (1.406, -0.016, 0, 0.184, 0.083, 2.6), (1.424, -0.016, 0, 0.183, 0.073, 2.6),
        (1.438, -0.015, 0, 0.175, 0.066, 2.6), (1.449, -0.013, 0, 0.146, 0.061, 2.4), (1.458, -0.011, 0, 0.108, 0.059, 2.2), (1.464, -0.009, 0, 0.086, 0.059, 2.0),
    ]
    out.append(th(loft("th-tee", "torso", COTTON, 0.8, tee, "y", sub=1), "hoodie", "tee"))
    for side, mirror in SIDES:
        half = [(1.434, -0.008, SH + 0.006, 0.016, 0.020), (1.427, -0.008, SH + 0.010, 0.034, 0.039), (1.410, -0.007, SH + 0.014, 0.049, 0.055), (1.376, -0.006, SH + 0.016, 0.056, 0.062), (1.330, -0.005, SH + 0.014, 0.054, 0.060), (1.290, -0.003, SH + 0.011, 0.051, 0.057), (1.262, -0.003, SH + 0.009, 0.049, 0.055)]
        out.append(th(loft("th-tee-sleeve-" + side, "arm-" + side, COTTON, 0.8, half, "y", sub=1, mirror=mirror), "hoodie", "tee"))
    out += [ref(i, "jeans", "jeans") for i in ["jeans-seat", "buttock-l", "buttock-r"] + [p + "-" + s for s, _ in SIDES for p in ("thigh", "kneecap", "shank")]]
    out.append(th(shrink(by["jeans-seat"], 0.0012, "th-briefs", material=COTTON, tone=0.8), "jeans", "briefs"))
    for side, _ in SIDES:
        out.append(th(shrink(by["buttock-" + side], 0.0012, "th-briefs-seat-" + side, material=COTTON, tone=0.8), "jeans", "briefs"))
    out += [ref(p + "-" + s, "sneakers", "sneaker") for s, _ in SIDES for p in ("sole", "shoe", "toe-sole", "shoe-toe", "toe-cap")]
    for side, _ in SIDES:
        out.append(ref("sock-" + side, "sneakers", "socks"))
        out.append(th(shrink(by["shoe-" + side], 0.004, "th-sock-foot-" + side, material=SOCK, tone=0.8), "sneakers", "socks"))
        out.append(th(shrink(by["toe-cap-" + side], 0.004, "th-sock-toe-" + side, material=SOCK, tone=0.8), "sneakers", "socks"))
    out.append(th(ellipsoid("th-nose-breath", "head", SKIN, 0.1, (0.113, d(0.1625), 0), (0.006, 0.004, 0.010), role="detail"), "face-skin", tags=["emitter", "nose-breath"]))
    out.append(th(ellipsoid("th-mouth-breath", "head", SKIN, 0.1, (0.105, d(0.181), 0), (0.005, 0.004, 0.013), role="detail"), "face-skin", tags=["emitter", "mouth-breath"]))
    out.append(th(ellipsoid("th-body-plume", "head", HAIR, 0.5, (0.012, 1.708, 0), (0.056, 0.0125, 0.046), role="detail"), "hoodie", tags=["emitter", "body-plume"]))
    return out


def spine_rows():
    return [
        (-0.035, 0.998, 0, 0.024, 0.017), (-0.024, 1.045, 0, 0.024, 0.017), (-0.017, 1.095, 0, 0.023, 0.017), (-0.020, 1.150, 0, 0.022, 0.016), (-0.034, 1.210, 0, 0.020, 0.016),
        (-0.047, 1.270, 0, 0.018, 0.015), (-0.053, 1.325, 0, 0.017, 0.014), (-0.050, 1.375, 0, 0.015, 0.013), (-0.038, 1.425, 0, 0.014, 0.012), (-0.020, 1.465, 0, 0.012, 0.011),
    ]


RIBS = [
    (1.438, 0.034, 0.056, 0.046, 0.0),
    (1.414, 0.050, 0.078, 0.064, 0.0),
    (1.388, 0.060, 0.094, 0.072, 0.0),
    (1.360, 0.068, 0.105, 0.079, 0.0),
    (1.331, 0.075, 0.113, 0.084, 0.0),
    (1.302, 0.080, 0.118, 0.086, 0.0),
    (1.273, 0.084, 0.121, 0.088, 0.12),
    (1.244, 0.084, 0.122, 0.087, 0.50),
    (1.215, 0.078, 0.119, 0.082, 0.85),
    (1.186, 0.068, 0.112, 0.072, 1.20),
    (1.158, 0.045, 0.100, 0.050, 1.75),
    (1.132, 0.025, 0.082, 0.025, 2.15),
]


def spine_x(y):
    rows = spine_rows()
    for a, b in zip(rows, rows[1:]):
        if a[1] <= y <= b[1]:
            return a[0] + (b[0] - a[0]) * (y - a[1]) / (b[1] - a[1])
    return rows[-1][0]


def rib(k):
    y, drop, w, front, to = RIBS[k]
    return rib_rows(y, drop, w, spine_x(y), front, 0.0052, to=to, steps=10 if to < 1 else 6)


def axial():
    out = []
    cranium = [
        (1.704, 0.018, 0, 0.012, 0.016), (1.697, 0.018, 0, 0.036, 0.047), (1.682, 0.017, 0, 0.054, 0.071), (1.660, 0.016, 0, 0.064, 0.085), (1.635, 0.016, 0, 0.0675, 0.089),
        (1.610, 0.015, 0, 0.066, 0.087), (1.590, 0.012, 0, 0.060, 0.078), (1.575, 0.006, 0, 0.050, 0.064), (1.563, 0.002, 0, 0.034, 0.044), (1.556, 0.002, 0, 0.016, 0.022),
    ]
    face = [
        (1.622, 0.078, 0, 0.050, 0.022, 2.6), (1.606, 0.082, 0, 0.058, 0.024, 2.6), (1.588, 0.080, 0, 0.060, 0.026, 2.4), (1.570, 0.078, 0, 0.050, 0.028, 2.2), (1.552, 0.080, 0, 0.036, 0.026, 2.0), (1.540, 0.082, 0, 0.030, 0.022, 2.0),
    ]
    jaw = [(1.538, 0.078, 0, 0.034, 0.024, 2.2), (1.522, 0.078, 0, 0.033, 0.026, 2.2), (1.506, 0.080, 0, 0.028, 0.024, 2.2), (1.495, 0.084, 0, 0.018, 0.016, 2.0)]
    out += [
        st(loft("cranium", "head", SKIN, BONE, cranium, "y", sub=2), "head"),
        st(loft("facial-skeleton", "head", SKIN, BONE, face, "y", sub=1), "head"),
        st(loft("mandible", "head", SKIN, BONE, jaw, "y", sub=1), "head"),
        st(ellipsoid("nasal-bone", "head", SKIN, BONE, (0.108, 1.598, 0), (0.008, 0.014, 0.008), role="detail", tilt=-20), "head"),
    ]
    for side, mirror in SIDES:
        out.append(st(capsule("mandible-ramus-" + side, "head", SKIN, BONE, (0.050, 1.512, 0.040), (0.022, 1.578, 0.050), 0.008, 0.007, mirror=mirror), "head"))
        out.append(st(capsule("zygomatic-arch-" + side, "head", SKIN, BONE, (0.086, 1.590, 0.050), (0.024, 1.588, 0.062), 0.006, 0.005, mirror=mirror), "head"))
    cervical = [(-0.020, 1.462, 0, 0.013, 0.011), (-0.010, 1.488, 0, 0.012, 0.010), (0.001, 1.514, 0, 0.012, 0.010), (0.008, 1.538, 0, 0.012, 0.010), (0.0125, 1.562, 0, 0.013, 0.010)]
    out.append(st(free_loft("cervical-spine", "neck", SKIN, BONE, cervical, sub=1), "neck"))
    out.append(st(free_loft("spine", "torso", SKIN, BONE, spine_rows(), sub=2), "torso"))
    for k, (y, x) in enumerate(((1.02, -0.050), (1.07, -0.045), (1.12, -0.042), (1.18, -0.052), (1.24, -0.066), (1.30, -0.076), (1.355, -0.078), (1.405, -0.068), (1.447, -0.052))):
        tip = (x - 0.024, y - 0.012, 0)
        out.append(st(capsule("spinous-process-%d" % k, "torso", SKIN, BONE, (x + 0.012, y, 0), tip, 0.006, 0.0045, role="detail"), "torso"))
    for side, mirror in SIDES:
        for k in range(len(RIBS)):
            out.append(st(free_loft("rib-%s-%d" % (side, k + 1), "torso", SKIN, BONE, rib(k), mirror=mirror), "torso"))
        arch = [(0.086, 1.214, 0.014, 0.0052, 0.0052)] + [rib(k)[-1] for k in (6, 7, 8, 9)]
        out.append(st(free_loft("costal-arch-" + side, "torso", SKIN, BONE, arch, mirror=mirror), "torso"))
        clavicle = [(0.050, 1.404, 0.022, 0.0075, 0.0065), (0.052, 1.410, 0.065, 0.006, 0.006), (0.030, 1.418, 0.115, 0.006, 0.006), (-0.002, 1.426, 0.150, 0.007, 0.006), (-0.014, 1.428, 0.168, 0.0075, 0.006)]
        out.append(st(free_loft("clavicle-" + side, "torso", SKIN, BONE, clavicle, sub=1, mirror=mirror), "torso"))
        blade = [(1.418, -0.074, 0.102, 0.040, 0.0055, 2.4), (1.395, -0.082, 0.104, 0.048, 0.006, 2.4), (1.360, -0.090, 0.104, 0.046, 0.006, 2.4), (1.325, -0.096, 0.100, 0.036, 0.0055, 2.2), (1.292, -0.099, 0.094, 0.022, 0.005, 2.0), (1.272, -0.099, 0.090, 0.010, 0.0045, 2.0)]
        out.append(st(loft("scapula-" + side, "torso", SKIN, BONE, blade, "y", sub=1, mirror=mirror), "torso"))
        out.append(st(capsule("scapular-spine-" + side, "torso", SKIN, BONE, (-0.082, 1.398, 0.066), (-0.040, 1.424, 0.158), 0.0065, 0.0075, mirror=mirror), "torso"))
        out.append(st(capsule("acromion-" + side, "torso", SKIN, BONE, (-0.040, 1.424, 0.158), (-0.006, 1.430, 0.176), 0.0075, 0.007, mirror=mirror), "torso"))
        out.append(st(ellipsoid("glenoid-" + side, "torso", SKIN, BONE, (-0.022, 1.396, 0.133), (0.014, 0.020, 0.008), mirror=mirror), "torso"))
    sternum = [(0.046, 1.408, 0, 0.026, 0.007), (0.058, 1.385, 0, 0.022, 0.007), (0.070, 1.340, 0, 0.016, 0.006), (0.080, 1.285, 0, 0.018, 0.006), (0.086, 1.235, 0, 0.014, 0.006), (0.087, 1.208, 0, 0.007, 0.005)]
    out.append(st(free_loft("sternum", "torso", SKIN, BONE, sternum, sub=1), "torso"))
    sacrum = [(-0.036, 0.999, 0, 0.054, 0.018), (-0.050, 0.972, 0, 0.048, 0.016), (-0.064, 0.940, 0, 0.036, 0.013), (-0.074, 0.908, 0, 0.022, 0.010), (-0.076, 0.884, 0, 0.010, 0.0065)]
    out.append(st(free_loft("sacrum", "pelvis", SKIN, BONE, sacrum, sub=1), "pelvis"))
    for side, mirror in SIDES:
        ilium = [
            (1.036, -0.006, 0.108, 0.008, 0.026, 2.0, 34), (1.026, -0.005, 0.103, 0.008, 0.048, 2.2, 34), (1.008, -0.004, 0.096, 0.008, 0.064, 2.4, 34), (0.988, -0.002, 0.092, 0.0085, 0.070, 2.4, 34),
            (0.966, 0.002, 0.092, 0.009, 0.062, 2.4, 32), (0.942, 0.004, 0.096, 0.012, 0.046, 2.2, 24), (0.916, 0.000, 0.099, 0.021, 0.038, 2.0, 10), (0.892, -0.006, 0.091, 0.022, 0.034, 2.0, 0),
            (0.868, -0.016, 0.077, 0.015, 0.027, 2.0, 0), (0.848, -0.026, 0.065, 0.012, 0.020, 2.0, 0), (0.838, -0.030, 0.061, 0.008, 0.012, 2.0, 0),
        ]
        out.append(st(loft("hip-bone-" + side, "pelvis", SKIN, BONE, ilium, "y", sub=1, mirror=mirror), "pelvis"))
        out.append(st(capsule("pubic-ramus-" + side, "pelvis", SKIN, BONE, (0.022, 0.896, 0.074), (0.058, 0.880, 0.010), 0.009, 0.010, mirror=mirror), "pelvis"))
        out.append(st(capsule("ischial-ramus-" + side, "pelvis", SKIN, BONE, (-0.022, 0.846, 0.058), (0.052, 0.868, 0.012), 0.007, 0.008, mirror=mirror), "pelvis"))
    return out


def legs():
    out = []
    for side, mirror in SIDES:
        s = "-" + side
        shaft = [(-0.006, 0.886, 0.134, 0.016, 0.016), (0.000, 0.850, 0.130, 0.0145, 0.0145), (0.008, 0.760, 0.119, 0.0135, 0.0135), (0.012, 0.670, 0.108, 0.0135, 0.0135), (0.010, 0.590, 0.097, 0.0145, 0.0145), (0.005, 0.545, 0.090, 0.018, 0.017), (0.003, 0.520, 0.086, 0.026, 0.021)]
        condyles = [(0.526, 0.002, HIP, 0.028, 0.022, 2.4), (0.510, 0.000, HIP, 0.039, 0.030, 2.6), (0.494, -0.002, HIP, 0.041, 0.033, 2.6), (0.482, -0.004, HIP, 0.038, 0.030, 2.4), (0.475, -0.004, HIP, 0.028, 0.020, 2.2)]
        tibia = [
            (0.474, 0.000, HIP - 0.001, 0.038, 0.027, 2.6), (0.462, 0.001, HIP - 0.001, 0.036, 0.026, 2.6), (0.440, 0.006, HIP - 0.002, 0.024, 0.021, 2.2), (0.400, 0.009, HIP - 0.003, 0.015, 0.017, 2.0),
            (0.300, 0.008, HIP - 0.004, 0.012, 0.0145, 2.0), (0.190, 0.004, HIP - 0.005, 0.011, 0.012, 2.0), (0.125, 0.001, HIP - 0.006, 0.014, 0.014, 2.0), (0.100, 0.000, HIP - 0.008, 0.021, 0.018, 2.2), (0.090, 0.000, HIP - 0.010, 0.022, 0.018, 2.2), (0.076, 0.000, HIP - 0.024, 0.008, 0.012, 2.0),
        ]
        tarsus = [(0.018, 0.064, HIP - 0.002, 0.024, 0.017, 2.4), (0.040, 0.064, HIP - 0.002, 0.029, 0.016, 2.4), (0.062, 0.059, HIP - 0.001, 0.033, 0.013, 2.4), (0.072, 0.056, HIP, 0.034, 0.011, 2.4)]
        out += [
            st(sphere("femoral-head" + s, "thigh" + s, SKIN, BONE, (0, 0.903, HIP), 0.022, mirror=mirror), "thigh" + s),
            st(capsule("femoral-neck" + s, "thigh" + s, SKIN, BONE, (0, 0.903, HIP + 0.006), (-0.004, 0.884, 0.128), 0.014, 0.015, mirror=mirror), "thigh" + s),
            st(ellipsoid("greater-trochanter" + s, "thigh" + s, SKIN, BONE, (-0.008, 0.888, 0.139), (0.016, 0.024, 0.014), mirror=mirror), "thigh" + s),
            st(free_loft("femur" + s, "thigh" + s, SKIN, BONE, shaft, sub=1, mirror=mirror), "thigh" + s),
            st(loft("femoral-condyles" + s, "thigh" + s, SKIN, BONE, condyles, "y", mirror=mirror), "thigh" + s),
            st(ellipsoid("patella" + s, "thigh" + s, SKIN, BONE, (0.038, 0.499, HIP), (0.0095, 0.021, 0.020), mirror=mirror), "thigh" + s),
            st(loft("tibia" + s, "shank" + s, SKIN, BONE, tibia, "y", sub=1, mirror=mirror), "shank" + s),
            st(capsule("fibula" + s, "shank" + s, SKIN, BONE, (-0.020, 0.458, 0.116), (-0.014, 0.110, 0.110), 0.0075, 0.0055, mirror=mirror), "shank" + s),
            st(ellipsoid("lateral-malleolus" + s, "shank" + s, SKIN, BONE, (-0.013, 0.090, 0.110), (0.011, 0.020, 0.008), mirror=mirror), "shank" + s),
            st(ellipsoid("talus" + s, "foot" + s, SKIN, BONE, (0.006, 0.074, HIP), (0.030, 0.015, 0.020), mirror=mirror, tilt=-12), "foot" + s),
            st(ellipsoid("calcaneus" + s, "foot" + s, SKIN, BONE, (-0.030, 0.048, HIP + 0.003), (0.040, 0.020, 0.017), mirror=mirror, tilt=18), "foot" + s),
            st(loft("tarsus" + s, "foot" + s, SKIN, BONE, tarsus, "x", mirror=mirror), "foot" + s),
        ]
        for k, (z, tip, y_tip) in enumerate(((-0.028, 0.205, 0.031), (-0.013, 0.203, 0.030), (0.001, 0.196, 0.030), (0.015, 0.187, 0.030), (0.029, 0.176, 0.030))):
            zz = HIP + 0.004 + z
            head_x = 0.1496 - (0.004 * k if k else -0.002)
            out.append(st(capsule("metatarsal%s-%d" % (s, k + 1), "foot" + s, SKIN, BONE, (0.070, 0.056 - 0.004 * k, HIP + z * 0.62), (head_x, 0.033, zz), 0.0052 if k else 0.007, 0.0055 if k else 0.0075, mirror=mirror), "foot" + s))
            out.append(st(capsule("toe-bones%s-%d" % (s, k + 1), "toe" + s, SKIN, BONE, (head_x, 0.033, zz), (tip - 0.008, y_tip, zz * 0.97 + 0.003), 0.0052 if k else 0.007, 0.0042 if k else 0.006, mirror=mirror), "toe" + s))
    return out


def arms():
    out = []
    for side, mirror in SIDES:
        s = "-" + side
        distal = [(1.120, 0.000, SH, 0.014, 0.011, 2.0), (1.100, 0.000, SH, 0.026, 0.012, 2.4), (1.088, 0.000, SH, 0.031, 0.013, 2.6), (1.078, 0.002, SH, 0.026, 0.013, 2.4), (1.072, 0.002, SH, 0.016, 0.010, 2.0)]
        out += [
            st(sphere("humeral-head" + s, "arm" + s, SKIN, BONE, (-0.002, 1.3975, SH - 0.004), 0.021, mirror=mirror), "arm" + s),
            st(ellipsoid("greater-tubercle" + s, "arm" + s, SKIN, BONE, (0.004, 1.388, SH + 0.014), (0.013, 0.016, 0.011), mirror=mirror), "arm" + s),
            st(capsule("humerus" + s, "arm" + s, SKIN, BONE, (0.000, 1.372, SH + 0.004), (0.000, 1.118, SH), 0.012, 0.0105, mirror=mirror), "arm" + s),
            st(loft("humeral-condyle" + s, "arm" + s, SKIN, BONE, distal, "y", mirror=mirror), "arm" + s),
            st(capsule("ulna" + s, "forearm" + s, SKIN, BONE, (-0.016, 1.092, SH - 0.002), (-0.017, 0.846, SH), 0.009, 0.0055, mirror=mirror), "forearm" + s),
            st(ellipsoid("olecranon" + s, "forearm" + s, SKIN, BONE, (-0.024, 1.096, SH - 0.002), (0.011, 0.016, 0.011), mirror=mirror), "forearm" + s),
            st(capsule("radius" + s, "forearm" + s, SKIN, BONE, (0.012, 1.070, SH + 0.006), (0.010, 0.870, SH), 0.0065, 0.0085, mirror=mirror), "forearm" + s),
            st(ellipsoid("radial-end" + s, "forearm" + s, SKIN, BONE, (0.008, 0.850, SH), (0.016, 0.014, 0.009), mirror=mirror), "forearm" + s),
            st(ellipsoid("carpus" + s, "hand" + s, SKIN, BONE, (0.000, 0.818, SH), (0.024, 0.015, 0.0095), mirror=mirror), "hand" + s),
        ]
        for i, (fx, fy, _length, r, _a0, _a1) in enumerate(b50.FINGERS):
            p0, p1, p2 = b50.finger_points(i)
            out.append(st(capsule("metacarpal%s-%d" % (s, i + 2), "hand" + s, SKIN, BONE, (fx * 0.62, 0.806, SH), (fx, fy + 0.004, SH - 0.001), 0.0048, 0.0052, mirror=mirror), "hand" + s))
            out.append(st(capsule("phalanx%s-%da" % (s, i + 2), "hand" + s, SKIN, BONE, p0, p1, 0.0045, 0.0042, mirror=mirror), "hand" + s))
            out.append(st(capsule("phalanx%s-%db" % (s, i + 2), "hand" + s, SKIN, BONE, p1, p2, 0.0042, 0.0039, mirror=mirror), "hand" + s))
        out += [
            st(capsule("metacarpal%s-1" % s, "hand" + s, SKIN, BONE, (0.016, 0.814, SH - 0.004), (0.033, 0.803, SH - 0.008), 0.0055, 0.0055, mirror=mirror), "hand" + s),
            st(capsule("phalanx%s-1a" % s, "hand" + s, SKIN, BONE, (0.033, 0.803, SH - 0.008), (0.053, 0.765, SH - 0.014), 0.005, 0.0045, mirror=mirror), "hand" + s),
            st(capsule("phalanx%s-1b" % s, "hand" + s, SKIN, BONE, (0.053, 0.765, SH - 0.014), (0.0565, 0.737, SH - 0.0185), 0.0045, 0.004, mirror=mirror), "hand" + s),
        ]
    return out


def joints():
    out = [
        st(ellipsoid("joint-lumbosacral", "torso", SKIN, JOINT, (-0.035, 0.998, 0), (0.019, 0.006, 0.026), tilt=12), "torso", "joint"),
        st(ellipsoid("joint-cervicothoracic", "neck", SKIN, JOINT, (-0.020, 1.465, 0), (0.012, 0.005, 0.015), tilt=-18), "neck", "joint"),
        st(ellipsoid("joint-atlanto-occipital", "head", SKIN, JOINT, (0.0132, 1.561, 0), (0.012, 0.006, 0.022)), "head", "joint"),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        out += [
            st(sphere("joint-hip" + s, "thigh" + s, SKIN, JOINT, (0, 0.903, HIP), 0.026, mirror=mirror), "thigh" + s, "joint"),
            st(capsule("joint-knee" + s, "shank" + s, SKIN, JOINT, (0, 0.4898, HIP - 0.036), (0, 0.4898, HIP + 0.036), 0.0105, 0.0105, mirror=mirror), "shank" + s, "joint"),
            st(capsule("joint-ankle" + s, "foot" + s, SKIN, JOINT, (0, 0.086, HIP - 0.030), (0, 0.086, HIP + 0.030), 0.009, 0.009, mirror=mirror), "foot" + s, "joint"),
            st(capsule("joint-toes" + s, "toe" + s, SKIN, JOINT, (0.1496, 0.033, HIP - 0.030), (0.136, 0.033, HIP + 0.036), 0.0062, 0.0062, mirror=mirror), "toe" + s, "joint"),
            st(sphere("joint-shoulder" + s, "arm" + s, SKIN, JOINT, (0, 1.3975, SH), 0.0245, mirror=mirror), "arm" + s, "joint"),
            st(capsule("joint-elbow" + s, "forearm" + s, SKIN, JOINT, (0, 1.084, SH - 0.027), (0, 1.084, SH + 0.027), 0.0095, 0.0095, mirror=mirror), "forearm" + s, "joint"),
            st(ellipsoid("joint-wrist" + s, "hand" + s, SKIN, JOINT, (0, 0.837, SH), (0.021, 0.0075, 0.009), mirror=mirror), "hand" + s, "joint"),
        ]
    return out


def masses():
    out = [
        st(sphere("mass-pelvis", "pelvis", SKIN, MASS, (0, 0.9601, 0), 0.020, role="detail"), "pelvis", "mass"),
        st(sphere("mass-torso", "torso", SKIN, MASS, (0, 1.2024, 0), 0.027, role="detail"), "torso", "mass"),
    ]
    for side, mirror in SIDES:
        out.append(st(sphere("mass-thigh-" + side, "thigh-" + side, SKIN, MASS, (0, 0.7537, HIP), 0.021, role="detail", mirror=mirror), "thigh-" + side, "mass"))
    return out


def structure():
    return axial() + legs() + arms() + joints() + masses()


def decisions():
    return [
        {
            "en": "Body at 1:50: every form is a loft through hand-stated anatomical cross-sections. Head with jaw and chin, nose, lips, brows, eyes and ears under a cap of short hair; neck; hooded sweatshirt as one garment over chest, waist and hips with shoulder slope, hem rib, collar rib, kangaroo pocket, drawstrings and the hood lying on the upper back with its rolled edge round the neck; sleeves with the deltoid mass, the forearm taper and rib cuffs; hands with thenar, thumb and four fingers of two segments each; jeans with seat, gluteal masses, thighs, kneecaps and calves; socks; canvas sneakers with rubber sole, toe cap and laces.",
            "de": "Körper bei 1:50: Jede Form ist ein Loft durch von Hand gesetzte anatomische Querschnitte. Kopf mit Kiefer und Kinn, Nase, Lippen, Brauen, Augen und Ohren unter einer Kappe aus kurzem Haar; Hals; Kapuzensweatshirt als ein Kleidungsstück über Brust, Taille und Hüfte mit Schulterschräge, Saumbund, Kragenbund, Kängurutasche, Kordeln und der auf dem oberen Rücken liegenden Kapuze mit ihrem gerollten Rand um den Hals; Ärmel mit Deltamasse, Unterarmverjüngung und Bündchen; Hände mit Daumenballen, Daumen und vier Fingern aus je zwei Gliedern; Jeans mit Gesäß, Gesäßmuskeln, Oberschenkeln, Kniescheiben und Waden; Socken; Stoffsneaker mit Gummisohle, Zehenkappe und Schnürung.",
        },
        {
            "en": "Thermal at 1:50: the outer garment surfaces are the body solids themselves. Inside them lie the real inner layers: the T-shirt as a shell 4 mm under the sweatshirt on the trunk and as short sleeves on the upper arms only, the briefs 1.2 mm under the denim on the pelvis only, the socks 4 mm inside the sneaker uppers and visible at the ankles. The face is its own solid, the skull ahead of the hair line, so that the scalp counts as hair and not as bare skin. Breath leaves from a patch under the nose tip and one on the lips, the body plume from a patch on the crown.",
            "de": "Thermik bei 1:50: Die äußeren Kleidungsflächen sind die Körper der Gestalt selbst. Darin liegen die wirklichen inneren Schichten: das T-Shirt als Schale 4 mm unter dem Sweatshirt am Rumpf und als kurze Ärmel nur an den Oberarmen, die Unterhose 1,2 mm unter dem Denim nur am Becken, die Socken 4 mm im Schaft der Sneaker und an den Knöcheln sichtbar. Das Gesicht ist ein eigener Körper, der Schädel vor der Haarlinie, damit die Kopfhaut als Haar und nicht als nackte Haut zählt. Der Atem tritt aus einem Fleck unter der Nasenspitze und einem auf den Lippen aus, die Körperfahne aus einem Fleck auf dem Scheitel.",
        },
        {
            "en": "Structure at 1:50: the skeleton is the load path. Cranium, facial skeleton and mandible with rami; cervical spine; thoracolumbar spine with lumbar lordosis and thoracic kyphosis and spinous processes; twelve ribs each side sinking forward, the lower ones ending free; sternum, clavicles, scapulae with spine, acromion and glenoid; sacrum, hip bones with iliac wings, pubic and ischial rami; femur with head, neck, greater trochanter, bowed shaft and condyles; patella, tibia with malleolus, fibula; talus, calcaneus, tarsus, five metatarsals and toes; humerus with head and condyle, ulna with olecranon, radius, carpus, metacarpals and phalanges. Joints: discs at the lumbosacral and cervicothoracic joints, the atlanto-occipital joint, balls at hip and shoulder, hinge pins at knee, ankle, toes and elbow, the wrist. Masses of pelvis, trunk and thighs at their de Leva centres.",
            "de": "Tragwerk bei 1:50: Das Skelett ist der Lastpfad. Hirnschädel, Gesichtsschädel und Unterkiefer mit Ästen; Halswirbelsäule; Brust- und Lendenwirbelsäule mit Lendenlordose, Brustkyphose und Dornfortsätzen; zwölf Rippen je Seite, nach vorn abfallend, die unteren frei endend; Brustbein, Schlüsselbeine, Schulterblätter mit Gräte, Schulterhöhe und Gelenkpfanne; Kreuzbein, Hüftbeine mit Darmbeinschaufeln, Scham- und Sitzbeinästen; Oberschenkelknochen mit Kopf, Hals, großem Rollhügel, gebogenem Schaft und Gelenkknorren; Kniescheibe, Schienbein mit Knöchel, Wadenbein; Sprungbein, Fersenbein, Fußwurzel, fünf Mittelfußknochen und Zehen; Oberarmknochen mit Kopf und Gelenkknorren, Elle mit Ellenhaken, Speiche, Handwurzel, Mittelhandknochen und Fingerglieder. Gelenke: Scheiben am Lumbosakral- und am Zervikothorakalgelenk, das Atlantookzipitalgelenk, Kugeln an Hüfte und Schulter, Scharnierbolzen an Knie, Sprunggelenk, Zehen und Ellbogen, das Handgelenk. Massen von Becken, Rumpf und Oberschenkeln in ihren Schwerpunkten nach de Leva.",
        },
    ]
