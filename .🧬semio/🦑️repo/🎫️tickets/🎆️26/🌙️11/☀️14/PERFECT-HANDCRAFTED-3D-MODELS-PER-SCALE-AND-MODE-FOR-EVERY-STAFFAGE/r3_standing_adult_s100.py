"""🧍 The standing adult at 1:100 (smallest feature 15 mm), authored anew for this scale: fewer and longer sections, face features merged into the head, hem, collar and cuffs merged into the garment, a mitten hand with a thumb, one shoe form per bone; the thermal model on these forms; the skeleton with merged bones.

World of the rest pose: x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder 1.3975, C7 1.465, head joint 1.561, vertex 1.710.
"""
from r3_standing_adult_kit import capsule, ellipsoid, face_rows, free_loft, loft, ref, rib_rows, sphere, st, th

SKIN, HAIR, FLEECE, DENIM, CANVAS = "skin", "hair", "fleece-sweatshirt", "denim", "canvas-sneaker"
BONE, JOINT, MASS = 0.85, 0.4, 0.2
SH, HIP = 0.153, 0.085
SIDES = (("l", False), ("r", True))


HEAD = [
    (1.710, 0.020, 0, 0.012, 0.016), (1.703, 0.020, 0, 0.038, 0.050), (1.684, 0.019, 0, 0.059, 0.077), (1.645, 0.020, 0, 0.073, 0.095), (1.605, 0.022, 0, 0.071, 0.092),
    (1.570, 0.028, 0, 0.066, 0.081), (1.535, 0.039, 0, 0.056, 0.067), (1.505, 0.049, 0, 0.042, 0.051), (1.488, 0.060, 0, 0.018, 0.026),
]
HAIR_EDGE = [(1.666, 0.097), (1.648, 0.086), (1.630, 0.072), (1.598, 0.020), (1.566, -0.014), (1.544, -0.030), (1.530, -0.1), (1.505, -0.1), (1.489, -0.1)]


def body():
    head = HEAD
    nose = [(1.606, 0.110, 0, 0.0085, 0.008), (1.580, 0.117, 0, 0.012, 0.013), (1.560, 0.121, 0, 0.0155, 0.017), (1.549, 0.114, 0, 0.012, 0.009)]
    hair = [
        (1.720, 0.014, 0, 0.018, 0.024), (1.713, 0.012, 0, 0.046, 0.060), (1.692, 0.010, 0, 0.068, 0.088), (1.670, 0.012, 0, 0.0755, 0.099), (1.661, -0.004, 0, 0.077, 0.084), (1.630, -0.008, 0, 0.078, 0.080),
        (1.598, -0.032, 0, 0.072, 0.052), (1.566, -0.044, 0, 0.060, 0.030), (1.542, -0.042, 0, 0.038, 0.012),
    ]
    neck = [(1.415, -0.012, 0, 0.060, 0.052), (1.465, 0.001, 0, 0.055, 0.054), (1.520, 0.011, 0, 0.052, 0.054), (1.570, 0.016, 0, 0.047, 0.045)]
    hoodie = [
        (0.868, -0.017, 0, 0.175, 0.118, 2.4), (0.878, -0.017, 0, 0.180, 0.121, 2.4), (0.960, -0.009, 0, 0.180, 0.121, 2.4), (1.070, -0.003, 0, 0.162, 0.115, 2.4), (1.180, -0.001, 0, 0.160, 0.117, 2.4),
        (1.260, 0.002, 0, 0.163, 0.122, 2.4), (1.340, -0.008, 0, 0.172, 0.112, 2.4), (1.395, -0.015, 0, 0.187, 0.092, 2.5), (1.424, -0.016, 0, 0.187, 0.077, 2.6), (1.439, -0.015, 0, 0.178, 0.070, 2.6), (1.452, -0.012, 0, 0.134, 0.066, 2.3),
        (1.464, -0.009, 0, 0.090, 0.068, 2.0), (1.476, -0.004, 0, 0.074, 0.072, 2.0),
    ]
    hood = [(1.486, -0.068, 0, 0.080, 0.020, 2.2), (1.462, -0.090, 0, 0.106, 0.040, 2.2), (1.410, -0.122, 0, 0.108, 0.040, 2.2), (1.350, -0.136, 0, 0.088, 0.029, 2.2), (1.300, -0.133, 0, 0.048, 0.016, 2.2)]
    seat = [(1.020, 0.004, 0, 0.142, 0.094, 2.3), (0.960, -0.008, 0, 0.158, 0.106, 2.3), (0.900, -0.014, 0, 0.168, 0.106, 2.3), (0.850, -0.012, 0, 0.164, 0.102, 2.3), (0.815, -0.006, 0, 0.130, 0.080, 2.1), (0.797, -0.004, 0, 0.060, 0.042, 2.0)]
    out = [
        loft("head", "head", SKIN, 0.25, head, "y", e=2.15, sub=1),
        loft("nose", "head", SKIN, 0.25, nose, "y", role="detail"),
        loft("hair", "head", HAIR, 0.75, hair, "y", e=2.1, sub=1),
        loft("neck", "neck", SKIN, 0.25, neck, "y"),
        loft("hoodie", "torso", FLEECE, 0.45, hoodie, "y", sub=1),
        loft("hood", "torso", FLEECE, 0.4, hood, "y", sub=1),
        loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        sleeve = [
            (1.438, -0.008, SH + 0.006, 0.020, 0.024), (1.424, -0.008, SH + 0.011, 0.044, 0.050), (1.380, -0.006, SH + 0.016, 0.060, 0.066), (1.300, -0.004, SH + 0.012, 0.056, 0.062),
            (1.180, -0.001, SH + 0.005, 0.050, 0.055), (1.084, 0.000, SH, 0.0465, 0.0495), (1.079, 0.000, SH, 0.040, 0.043), (1.058, 0.000, SH, 0.038, 0.041), (1.042, 0.000, SH, 0.018, 0.020),
        ]
        forearm = [(1.122, 0.000, SH, 0.018, 0.020), (1.104, 0.000, SH, 0.037, 0.040), (1.089, 0.000, SH, 0.040, 0.043), (1.084, 0.000, SH, 0.045, 0.048), (1.050, 0.003, SH, 0.0455, 0.0495), (0.960, 0.002, SH, 0.042, 0.045), (0.886, 0.000, SH, 0.038, 0.042), (0.870, 0.000, SH, 0.030, 0.035), (0.832, 0.000, SH, 0.026, 0.032)]
        hand = [(0.852, 0.000, SH, 0.018, 0.026, 2.3), (0.810, 0.002, SH, 0.016, 0.034, 2.5), (0.760, 0.001, SH - 0.002, 0.014, 0.040, 2.8), (0.710, 0.000, SH - 0.010, 0.012, 0.037, 2.8), (0.676, 0.000, SH - 0.022, 0.0095, 0.030, 2.6), (0.662, 0.000, SH - 0.028, 0.0075, 0.016, 2.4)]
        thigh = [
            (0.976, -0.010, HIP + 0.003, 0.030, 0.034), (0.952, -0.010, HIP + 0.003, 0.066, 0.075), (0.905, -0.010, HIP + 0.003, 0.083, 0.093), (0.830, -0.008, HIP + 0.004, 0.083, 0.093), (0.700, -0.002, HIP + 0.002, 0.073, 0.082),
            (0.570, 0.003, HIP - 0.001, 0.060, 0.066), (0.4898, 0.0035, HIP - 0.002, 0.052, 0.057), (0.485, 0.0035, HIP - 0.002, 0.0455, 0.0505), (0.462, 0.003, HIP - 0.002, 0.045, 0.049), (0.442, 0.003, HIP - 0.002, 0.022, 0.024),
        ]
        shank = [(0.536, 0.003, HIP - 0.002, 0.018, 0.020), (0.515, 0.003, HIP - 0.002, 0.043, 0.048), (0.4946, 0.0035, HIP - 0.002, 0.0455, 0.0505), (0.4898, 0.0035, HIP - 0.002, 0.0505, 0.0555), (0.455, 0.000, HIP - 0.002, 0.0505, 0.0555), (0.365, -0.010, HIP, 0.055, 0.062), (0.250, -0.006, HIP, 0.047, 0.052), (0.150, -0.004, HIP, 0.0445, 0.0495), (0.105, -0.004, HIP, 0.040, 0.046)]
        shoe = [(-0.0745, 0.030, HIP, 0.018, 0.030, 3.0), (-0.062, 0.040, HIP, 0.030, 0.040, 3.0), (-0.020, 0.045, HIP, 0.035, 0.045, 2.8), (0.030, 0.046, HIP, 0.038, 0.046, 2.6), (0.080, 0.036, HIP + 0.001, 0.044, 0.036, 2.8), (0.125, 0.029, HIP + 0.003, 0.050, 0.029, 3.0), (0.157, 0.027, HIP + 0.003, 0.050, 0.027, 3.0)]
        toe = [(0.143, 0.0275, HIP + 0.003, 0.050, 0.0275, 3.0), (0.172, 0.026, HIP + 0.002, 0.046, 0.026, 3.0), (0.192, 0.023, HIP + 0.001, 0.035, 0.022, 2.8), (0.205, 0.019, HIP, 0.012, 0.012, 2.4)]
        out += [
            ellipsoid("ear" + s, "head", SKIN, 0.3, (-0.003, 1.576, 0.0685), (0.015, 0.029, 0.010), role="detail", mirror=mirror, tilt=-12),
            loft("sleeve" + s, "arm" + s, FLEECE, 0.45, sleeve, "y", sub=1, mirror=mirror),
            loft("forearm" + s, "forearm" + s, FLEECE, 0.45, forearm, "y", sub=1, mirror=mirror),
            loft("hand" + s, "hand" + s, SKIN, 0.3, hand, "y", mirror=mirror),
            capsule("thumb" + s, "hand" + s, SKIN, 0.3, (0.026, 0.808, SH - 0.008), (0.056, 0.738, SH - 0.017), 0.0115, 0.008, mirror=mirror),
            loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=1, mirror=mirror),
            loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=1, mirror=mirror),
            loft("shoe" + s, "foot" + s, CANVAS, 0.12, shoe, "x", sub=1, mirror=mirror),
            loft("shoe-toe" + s, "toe" + s, CANVAS, 0.12, toe, "x", mirror=mirror),
        ]
    return out


def thermal():
    face = face_rows(HEAD, HAIR_EDGE)
    out = [th(loft("th-face", "head", SKIN, 0.25, face, "y"), "face-skin"), ref("nose", "face-skin"), ref("ear-l", "face-skin"), ref("ear-r", "face-skin"), ref("neck", "face-skin"), ref("hair", "hair")]
    out += [ref(i, "hoodie", "sweatshirt") for i in ("hoodie", "hood", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r")]
    out += [ref(i, "jeans", "jeans") for i in ("jeans-seat", "thigh-l", "thigh-r", "shank-l", "shank-r")]
    out += [ref(i, "sneakers", "sneaker") for i in ("shoe-l", "shoe-r", "shoe-toe-l", "shoe-toe-r")]
    out += [ref(i, "hand-skin") for i in ("hand-l", "hand-r", "thumb-l", "thumb-r")]
    out.append(th(ellipsoid("th-nose-breath", "head", SKIN, 0.1, (0.113, 1.548, 0), (0.0075, 0.0075, 0.011), role="detail"), "face-skin", tags=["emitter", "nose-breath"]))
    out.append(th(ellipsoid("th-mouth-breath", "head", SKIN, 0.1, (0.098, 1.528, 0), (0.0075, 0.0075, 0.014), role="detail"), "face-skin", tags=["emitter", "mouth-breath"]))
    out.append(th(ellipsoid("th-body-plume", "head", HAIR, 0.5, (0.012, 1.708, 0), (0.054, 0.013, 0.044), role="detail"), "hoodie", tags=["emitter", "body-plume"]))
    return out


def structure():
    skull = [(1.704, 0.018, 0, 0.014, 0.018), (1.690, 0.017, 0, 0.048, 0.062), (1.650, 0.016, 0, 0.066, 0.088), (1.605, 0.018, 0, 0.065, 0.088), (1.578, 0.030, 0, 0.056, 0.074), (1.560, 0.050, 0, 0.046, 0.052), (1.540, 0.070, 0, 0.034, 0.032)]
    jaw = [(1.538, 0.066, 0, 0.040, 0.036, 2.2), (1.518, 0.070, 0, 0.036, 0.033, 2.2), (1.497, 0.080, 0, 0.022, 0.020, 2.0)]
    spine = [(-0.035, 0.998, 0, 0.024, 0.017), (-0.018, 1.090, 0, 0.023, 0.017), (-0.030, 1.200, 0, 0.020, 0.016), (-0.052, 1.310, 0, 0.017, 0.014), (-0.044, 1.410, 0, 0.014, 0.012), (-0.020, 1.465, 0, 0.012, 0.011)]
    cervical = [(-0.020, 1.462, 0, 0.013, 0.011), (-0.004, 1.505, 0, 0.012, 0.010), (0.0125, 1.562, 0, 0.013, 0.010)]
    sternum = [(0.046, 1.408, 0, 0.026, 0.0078), (0.066, 1.350, 0, 0.018, 0.0078), (0.084, 1.250, 0, 0.016, 0.0078), (0.087, 1.210, 0, 0.009, 0.0076)]
    sacrum = [(-0.036, 0.999, 0, 0.054, 0.018), (-0.058, 0.955, 0, 0.042, 0.015), (-0.076, 0.890, 0, 0.012, 0.008)]
    out = [
        st(loft("skull", "head", SKIN, BONE, skull, "y", sub=1), "head"),
        st(loft("mandible", "head", SKIN, BONE, jaw, "y"), "head"),
        st(free_loft("cervical-spine", "neck", SKIN, BONE, cervical, sub=1), "neck"),
        st(free_loft("spine", "torso", SKIN, BONE, spine, sub=2), "torso"),
        st(free_loft("sternum", "torso", SKIN, BONE, sternum), "torso"),
        st(free_loft("sacrum", "pelvis", SKIN, BONE, sacrum, sub=1), "pelvis"),
    ]
    ribs = [(1.428, 0.040, 0.066, -0.033, 0.050, 0.0), (1.376, 0.064, 0.100, -0.049, 0.074, 0.0), (1.318, 0.078, 0.116, -0.053, 0.085, 0.0), (1.258, 0.084, 0.122, -0.045, 0.088, 0.30), (1.198, 0.072, 0.115, -0.030, 0.076, 1.05), (1.146, 0.035, 0.092, -0.022, 0.036, 1.95)]
    for side, mirror in SIDES:
        s = "-" + side
        for k, (y, drop, w, sx, front, to) in enumerate(ribs):
            out.append(st(free_loft("rib%s-%d" % (s, k + 1), "torso", SKIN, BONE, rib_rows(y, drop, w, sx, front, 0.0078, to=to, steps=8 if to < 1 else 5), mirror=mirror), "torso"))
        ilium = [(1.034, -0.006, 0.106, 0.0085, 0.030, 2.0, 34), (1.006, -0.004, 0.096, 0.0085, 0.066, 2.4, 34), (0.966, 0.002, 0.092, 0.009, 0.062, 2.4, 32), (0.916, 0.000, 0.099, 0.021, 0.038, 2.0, 10), (0.868, -0.016, 0.077, 0.015, 0.027, 2.0, 0), (0.838, -0.030, 0.061, 0.008, 0.012, 2.0, 0)]
        blade = [(1.420, -0.074, 0.102, 0.040, 0.0078, 2.4), (1.385, -0.085, 0.104, 0.048, 0.0078, 2.4), (1.325, -0.096, 0.100, 0.036, 0.0078, 2.2), (1.274, -0.099, 0.091, 0.012, 0.0076, 2.0)]
        femur = [(-0.005, 0.888, 0.134, 0.016, 0.016), (0.004, 0.800, 0.124, 0.0135, 0.0135), (0.012, 0.660, 0.107, 0.0135, 0.0135), (0.006, 0.550, 0.091, 0.017, 0.017), (0.000, 0.505, HIP, 0.036, 0.030), (-0.003, 0.478, HIP, 0.034, 0.028)]
        tibia = [(0.474, 0.000, HIP - 0.001, 0.037, 0.027, 2.6), (0.440, 0.006, HIP - 0.002, 0.024, 0.021, 2.2), (0.330, 0.008, HIP - 0.004, 0.012, 0.0145, 2.0), (0.150, 0.002, HIP - 0.006, 0.0115, 0.0125, 2.0), (0.096, 0.000, HIP - 0.009, 0.022, 0.018, 2.2), (0.080, 0.000, HIP - 0.018, 0.012, 0.014, 2.0)]
        hindfoot = [(-0.066, 0.046, HIP + 0.003, 0.012, 0.014), (-0.040, 0.048, HIP + 0.003, 0.018, 0.022), (0.000, 0.062, HIP, 0.021, 0.026), (0.030, 0.064, HIP - 0.001, 0.027, 0.018), (0.070, 0.056, HIP, 0.033, 0.012)]
        forefoot = [(0.066, 0.056, HIP, 0.032, 0.011, 2.6), (0.110, 0.043, HIP + 0.002, 0.037, 0.009, 2.8), (0.152, 0.033, HIP + 0.004, 0.040, 0.008, 2.8)]
        toes = [(0.146, 0.033, HIP + 0.004, 0.040, 0.008, 2.8), (0.175, 0.031, HIP + 0.002, 0.036, 0.0078, 2.8), (0.197, 0.030, HIP - 0.004, 0.022, 0.0076, 2.4)]
        metacarpus = [(0.832, 0.000, SH, 0.0095, 0.022, 2.4), (0.800, 0.001, SH, 0.0085, 0.026, 2.6), (0.748, 0.000, SH - 0.001, 0.0078, 0.034, 2.8)]
        fingers = [(0.750, 0.000, SH - 0.001, 0.0078, 0.034, 2.8), (0.712, 0.000, SH - 0.009, 0.0078, 0.032, 2.8), (0.670, 0.000, SH - 0.024, 0.0076, 0.022, 2.6)]
        out += [
            st(loft("hip-bone" + s, "pelvis", SKIN, BONE, ilium, "y", sub=1, mirror=mirror), "pelvis"),
            st(capsule("pubic-ramus" + s, "pelvis", SKIN, BONE, (0.020, 0.892, 0.072), (0.056, 0.876, 0.010), 0.0095, 0.010, mirror=mirror), "pelvis"),
            st(free_loft("clavicle" + s, "torso", SKIN, BONE, [(0.050, 1.404, 0.022, 0.0078, 0.0078), (0.048, 1.412, 0.080, 0.0078, 0.0078), (0.010, 1.424, 0.140, 0.0078, 0.0078), (-0.014, 1.428, 0.170, 0.0078, 0.0078)], sub=1, mirror=mirror), "torso"),
            st(loft("scapula" + s, "torso", SKIN, BONE, blade, "y", mirror=mirror), "torso"),
            st(capsule("scapular-spine" + s, "torso", SKIN, BONE, (-0.082, 1.398, 0.066), (-0.016, 1.428, 0.170), 0.0078, 0.0078, mirror=mirror), "torso"),
            st(sphere("femoral-head" + s, "thigh" + s, SKIN, BONE, (0, 0.903, HIP), 0.022, mirror=mirror), "thigh" + s),
            st(capsule("femoral-neck" + s, "thigh" + s, SKIN, BONE, (0, 0.903, HIP + 0.006), (-0.005, 0.884, 0.132), 0.014, 0.016, mirror=mirror), "thigh" + s),
            st(free_loft("femur" + s, "thigh" + s, SKIN, BONE, femur, sub=1, mirror=mirror), "thigh" + s),
            st(ellipsoid("patella" + s, "thigh" + s, SKIN, BONE, (0.037, 0.499, HIP), (0.0095, 0.021, 0.020), mirror=mirror), "thigh" + s),
            st(loft("tibia" + s, "shank" + s, SKIN, BONE, tibia, "y", sub=1, mirror=mirror), "shank" + s),
            st(capsule("fibula" + s, "shank" + s, SKIN, BONE, (-0.020, 0.456, 0.115), (-0.013, 0.084, 0.110), 0.0078, 0.0078, mirror=mirror), "shank" + s),
            st(loft("hindfoot" + s, "foot" + s, SKIN, BONE, hindfoot, "x", sub=1, mirror=mirror), "foot" + s),
            st(loft("metatarsus" + s, "foot" + s, SKIN, BONE, forefoot, "x", mirror=mirror), "foot" + s),
            st(loft("toe-bones" + s, "toe" + s, SKIN, BONE, toes, "x", mirror=mirror), "toe" + s),
            st(sphere("humeral-head" + s, "arm" + s, SKIN, BONE, (-0.002, 1.3975, SH - 0.004), 0.021, mirror=mirror), "arm" + s),
            st(capsule("humerus" + s, "arm" + s, SKIN, BONE, (0.000, 1.378, SH + 0.004), (0.000, 1.110, SH), 0.012, 0.0105, mirror=mirror), "arm" + s),
            st(ellipsoid("humeral-condyle" + s, "arm" + s, SKIN, BONE, (0.000, 1.092, SH), (0.013, 0.020, 0.031), mirror=mirror), "arm" + s),
            st(capsule("ulna" + s, "forearm" + s, SKIN, BONE, (-0.018, 1.098, SH - 0.002), (-0.017, 0.848, SH), 0.0095, 0.0078, mirror=mirror), "forearm" + s),
            st(capsule("radius" + s, "forearm" + s, SKIN, BONE, (0.012, 1.070, SH + 0.006), (0.009, 0.850, SH), 0.0078, 0.0095, mirror=mirror), "forearm" + s),
            st(loft("metacarpus" + s, "hand" + s, SKIN, BONE, metacarpus, "y", mirror=mirror), "hand" + s),
            st(loft("finger-bones" + s, "hand" + s, SKIN, BONE, fingers, "y", mirror=mirror), "hand" + s),
            st(capsule("thumb-bones" + s, "hand" + s, SKIN, BONE, (0.018, 0.812, SH - 0.005), (0.055, 0.742, SH - 0.017), 0.0078, 0.0076, mirror=mirror), "hand" + s),
            st(sphere("joint-hip" + s, "thigh" + s, SKIN, JOINT, (0, 0.903, HIP), 0.026, mirror=mirror), "thigh" + s, "joint"),
            st(capsule("joint-knee" + s, "shank" + s, SKIN, JOINT, (0, 0.4898, HIP - 0.035), (0, 0.4898, HIP + 0.035), 0.0105, 0.0105, mirror=mirror), "shank" + s, "joint"),
            st(capsule("joint-ankle" + s, "foot" + s, SKIN, JOINT, (0, 0.086, HIP - 0.028), (0, 0.086, HIP + 0.028), 0.009, 0.009, mirror=mirror), "foot" + s, "joint"),
            st(sphere("joint-shoulder" + s, "arm" + s, SKIN, JOINT, (0, 1.3975, SH), 0.0245, mirror=mirror), "arm" + s, "joint"),
            st(capsule("joint-elbow" + s, "forearm" + s, SKIN, JOINT, (0, 1.084, SH - 0.026), (0, 1.084, SH + 0.026), 0.0095, 0.0095, mirror=mirror), "forearm" + s, "joint"),
            st(ellipsoid("joint-wrist" + s, "hand" + s, SKIN, JOINT, (0, 0.837, SH), (0.021, 0.008, 0.0095), mirror=mirror), "hand" + s, "joint"),
            st(sphere("mass-thigh" + s, "thigh" + s, SKIN, MASS, (0, 0.7537, HIP), 0.021, role="detail", mirror=mirror), "thigh" + s, "mass"),
        ]
    out += [
        st(ellipsoid("joint-lumbosacral", "torso", SKIN, JOINT, (-0.035, 0.998, 0), (0.019, 0.0078, 0.026), tilt=12), "torso", "joint"),
        st(ellipsoid("joint-cervicothoracic", "neck", SKIN, JOINT, (-0.020, 1.465, 0), (0.013, 0.0076, 0.016), tilt=-18), "neck", "joint"),
        st(ellipsoid("joint-atlanto-occipital", "head", SKIN, JOINT, (0.0132, 1.561, 0), (0.013, 0.0076, 0.022)), "head", "joint"),
        st(sphere("mass-pelvis", "pelvis", SKIN, MASS, (0, 0.9601, 0), 0.020, role="detail"), "pelvis", "mass"),
        st(sphere("mass-torso", "torso", SKIN, MASS, (0, 1.2024, 0), 0.027, role="detail"), "torso", "mass"),
    ]
    return out


def decisions():
    return [
        {
            "en": "Body at 1:100, drawn anew with fewer and longer sections: head with nose and ears under the hair cap, without eyes, brows and lips; neck; the sweatshirt as one form in which hem rib and collar rib are merged, the hood on the upper back without its rolled edge, no pocket and no drawstrings; sleeves, forearms with the cuff merged; a mitten hand with a thumb instead of fingers; jeans seat with the gluteal mass merged, thighs with the knee merged, shanks with the calf; one shoe form per bone with the sole merged, no socks and no laces.",
            "de": "Körper bei 1:100, neu gezeichnet mit weniger und längeren Schnitten: Kopf mit Nase und Ohren unter der Haarkappe, ohne Augen, Brauen und Lippen; Hals; das Sweatshirt als eine Form, in der Saum- und Kragenbund aufgehen, die Kapuze auf dem oberen Rücken ohne Rollrand, ohne Tasche und Kordeln; Ärmel, Unterarme mit aufgegangenem Bündchen; eine Fäustlingshand mit Daumen statt Fingern; Jeansgesäß mit aufgegangener Gesäßmasse, Oberschenkel mit aufgegangenem Knie, Unterschenkel mit Wade; eine Schuhform je Knochen mit aufgegangener Sohle, ohne Socken und Schnürung.",
        },
        {
            "en": "Thermal at 1:100: every region lies on the body forms of this scale; the layers are merged into the outer garment surface (the inner shells of T-shirt, briefs and socks are thinner than the smallest feature). The face is its own solid ahead of the hair line; breath leaves under the nose and at the lips, the body plume from a patch on the crown.",
            "de": "Thermik bei 1:100: Jede Region liegt auf den Körperformen dieses Maßstabs; die Schichten gehen in der äußeren Kleidungsfläche auf (die inneren Schalen von T-Shirt, Unterhose und Socken sind dünner als das kleinste Merkmal). Das Gesicht ist ein eigener Körper vor der Haarlinie; der Atem tritt unter der Nase und an den Lippen aus, die Körperfahne aus einem Fleck auf dem Scheitel.",
        },
        {
            "en": "Structure at 1:100: the skeleton with merged bones. Skull in one piece with the mandible, cervical and thoracolumbar spine with their curvature, six rib hoops each side standing for the twelve ribs, sternum, clavicles, scapulae, sacrum and hip bones with pubic rami; femur in one piece from head and neck to the condyles, patella, tibia, fibula; hindfoot, metatarsus and toes as three plates; humerus with head and condyle, ulna, radius, metacarpus, fingers and thumb. Joints at hip, knee, ankle, shoulder, elbow, wrist and the three joints of the spine; masses of pelvis, trunk and thighs.",
            "de": "Tragwerk bei 1:100: das Skelett mit zusammengefassten Knochen. Schädel in einem Stück mit Unterkiefer, Hals- sowie Brust- und Lendenwirbelsäule mit ihrer Krümmung, sechs Rippenbögen je Seite für die zwölf Rippen, Brustbein, Schlüsselbeine, Schulterblätter, Kreuzbein und Hüftbeine mit Schambeinästen; Oberschenkelknochen in einem Stück von Kopf und Hals bis zu den Gelenkknorren, Kniescheibe, Schienbein, Wadenbein; Rückfuß, Mittelfuß und Zehen als drei Platten; Oberarmknochen mit Kopf und Gelenkknorren, Elle, Speiche, Mittelhand, Finger und Daumen. Gelenke an Hüfte, Knie, Sprunggelenk, Schulter, Ellbogen, Handgelenk und den drei Gelenken der Wirbelsäule; Massen von Becken, Rumpf und Oberschenkeln.",
        },
    ]
