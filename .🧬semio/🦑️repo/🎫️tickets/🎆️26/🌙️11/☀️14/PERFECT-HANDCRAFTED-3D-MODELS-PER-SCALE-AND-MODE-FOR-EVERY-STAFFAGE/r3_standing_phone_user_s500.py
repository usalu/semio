"""🧍 The standing adult at 1:500 (smallest feature 75 mm), authored anew: the fewest masses that still read as a standing person in a sweatshirt at a glance; the thermal model on them; the skeleton as a stick figure of thickened bones.

World of the rest pose: x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder 1.3975, C7 1.465, head joint 1.561, vertex 1.710.
"""
import r3_standing_phone_user_parts as parts
import r3_standing_phone_user_texts as texts
from r3_standing_phone_user_kit import hair_cap, hairline_x, capsule, ellipsoid, face_rows, loft, ref, sphere, st, th

SKIN, HAIR, FLEECE, DENIM, CANVAS = "skin", "hair", "fleece-sweatshirt", "denim", "canvas-sneaker"
BONE, MASS = 0.85, 0.2
SH, HIP = 0.153, 0.085
SIDES = (("l", False), ("r", True))
HEAD = [(1.710, 0.020, 0, 0.022, 0.029), (1.680, 0.019, 0, 0.064, 0.084), (1.620, 0.022, 0, 0.072, 0.094), (1.545, 0.038, 0, 0.059, 0.072), (1.490, 0.058, 0, 0.024, 0.032)]
HAIR_EDGE = [(y, hairline_x(y)) for y in (1.662, 1.640, 1.615, 1.590, 1.565, 1.552)] + [(1.530, -0.1), (1.489, -0.1)]


def body():
    neck = [(1.415, -0.006, 0, 0.066, 0.062), (1.566, 0.016, 0, 0.049, 0.047)]
    hoodie = [(0.868, -0.017, 0, 0.185, 0.121, 2.4), (0.960, -0.010, 0, 0.186, 0.122, 2.4), (1.120, -0.002, 0, 0.161, 0.116, 2.4), (1.300, -0.014, 0, 0.168, 0.132, 2.4), (1.408, -0.040, 0, 0.188, 0.118, 2.6), (1.440, -0.036, 0, 0.176, 0.094, 2.6), (1.474, -0.008, 0, 0.074, 0.066, 2.0)]
    seat = [(1.010, 0.004, 0, 0.146, 0.096, 2.3), (0.890, -0.014, 0, 0.168, 0.106, 2.3), (0.800, -0.004, 0, 0.070, 0.048, 2.0)]
    out = [
        loft("head", "head", SKIN, 0.25, HEAD, "y", e=2.15, sub=1),
        hair_cap("hair", "head", HAIR, 0.75, (0, 0.30, 0.62, 0.88, 0.975), grow=0.008),
        loft("neck", "neck", SKIN, 0.25, neck, "y"),
        loft("hoodie", "torso", FLEECE, 0.45, hoodie, "y", sub=1),
        loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        sleeve = [(1.436, -0.008, SH + 0.006, 0.026, 0.030), (1.380, -0.006, SH + 0.016, 0.060, 0.066), (1.220, -0.002, SH + 0.007, 0.052, 0.057), (1.060, 0.000, SH, 0.042, 0.045)]
        forearm = [(1.110, 0.000, SH, 0.030, 0.032), (1.045, 0.003, SH, 0.0455, 0.0495), (0.900, 0.001, SH, 0.038, 0.042), (0.836, 0.000, SH, 0.028, 0.033)]
        thigh = [(0.972, -0.010, HIP + 0.003, 0.040, 0.045), (0.900, -0.010, HIP + 0.003, 0.083, 0.093), (0.740, -0.004, HIP + 0.003, 0.077, 0.086), (0.540, 0.004, HIP - 0.002, 0.057, 0.063), (0.450, 0.004, HIP - 0.002, 0.040, 0.044)]
        shank = [(0.528, 0.004, HIP - 0.002, 0.040, 0.044), (0.490, 0.004, HIP - 0.002, 0.0565, 0.0625), (0.430, 0.000, HIP - 0.002, 0.054, 0.060), (0.350, -0.010, HIP, 0.055, 0.062), (0.108, -0.004, HIP, 0.042, 0.047)]
        shoe = [(-0.0745, 0.034, HIP, 0.022, 0.034, 3.0), (-0.030, 0.045, HIP, 0.035, 0.045, 2.8), (0.040, 0.045, HIP + 0.001, 0.039, 0.045, 2.6), (0.157, 0.027, HIP + 0.003, 0.050, 0.027, 3.0)]
        toe = [(0.143, 0.0275, HIP + 0.003, 0.050, 0.0275, 3.0), (0.185, 0.026, HIP + 0.002, 0.043, 0.026, 3.0), (0.205, 0.023, HIP, 0.027, 0.022, 2.4)]
        out += [
            loft("sleeve" + s, "arm" + s, FLEECE, 0.45, sleeve, "y", sub=1, mirror=mirror),
            loft("forearm" + s, "forearm" + s, FLEECE, 0.45, forearm, "y", sub=1, mirror=mirror),
            loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=1, mirror=mirror),
            loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=1, mirror=mirror),
            loft("shoe" + s, "foot" + s, CANVAS, 0.12, shoe, "x", mirror=mirror),
            loft("shoe-toe" + s, "toe" + s, CANVAS, 0.12, toe, "x", mirror=mirror),
        ]
    return out + parts.hands(500)


def thermal():
    out = [th(loft("th-face", "head", SKIN, 0.25, face_rows(HEAD, HAIR_EDGE), "y"), "face-skin"), ref("neck", "face-skin"), ref("hair", "hair")]
    out += [ref(i, "hoodie", "sweatshirt") for i in ("hoodie", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r")]
    out += [ref(i, "jeans", "jeans") for i in ("jeans-seat", "thigh-l", "thigh-r", "shank-l", "shank-r")]
    out += [ref(i, "sneakers", "sneaker") for i in ("shoe-l", "shoe-r", "shoe-toe-l", "shoe-toe-r")]
    out += parts.thermal_parts(500)
    out.append(th(ellipsoid("th-nose-breath", "head", SKIN, 0.1, (0.084, 1.570, 0), (0.0375, 0.0375, 0.0375), role="detail"), "face-skin", tags=["emitter", "nose-breath"]))
    out.append(th(ellipsoid("th-mouth-breath", "head", SKIN, 0.1, (0.072, 1.530, 0), (0.0375, 0.0375, 0.0375), role="detail"), "face-skin", tags=["emitter", "mouth-breath"]))
    out.append(th(ellipsoid("th-body-plume", "head", HAIR, 0.5, (0.010, 1.682, 0), (0.060, 0.0375, 0.050), role="detail"), "hoodie", tags=["emitter", "body-plume"]))
    return out


def structure():
    out = [
        st(ellipsoid("skull", "head", SKIN, BONE, (0.022, 1.626, 0), (0.084, 0.072, 0.064)), "head"),
        st(capsule("cervical-spine", "neck", SKIN, BONE, (-0.012, 1.480, 0), (0.010, 1.545, 0), 0.0375, 0.0375), "neck"),
        st(capsule("spine", "torso", SKIN, BONE, (-0.030, 1.020, 0), (-0.026, 1.430, 0), 0.0375, 0.0375), "torso"),
        st(capsule("shoulder-girdle", "torso", SKIN, BONE, (-0.014, 1.398, -0.130), (-0.014, 1.398, 0.130), 0.0375, 0.0375), "torso"),
        st(capsule("pelvic-girdle", "pelvis", SKIN, BONE, (-0.006, 0.930, -0.090), (-0.006, 0.930, 0.090), 0.040, 0.040), "pelvis"),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        out += [
            st(capsule("femur" + s, "thigh" + s, SKIN, BONE, (-0.002, 0.890, HIP + 0.006), (0.003, 0.500, HIP), 0.0375, 0.0375, mirror=mirror), "thigh" + s),
            st(capsule("tibia" + s, "shank" + s, SKIN, BONE, (0.000, 0.470, HIP - 0.001), (-0.003, 0.128, HIP), 0.0375, 0.0375, mirror=mirror), "shank" + s),
            st(capsule("foot-bones" + s, "foot" + s, SKIN, BONE, (-0.030, 0.042, HIP), (0.128, 0.029, HIP + 0.003), 0.0375, 0.0235, mirror=mirror), "foot" + s),
            st(capsule("toe-bones" + s, "toe" + s, SKIN, BONE, (0.152, 0.0275, HIP + 0.003), (0.180, 0.024, HIP + 0.002), 0.0275, 0.0185, mirror=mirror), "toe" + s),
            st(capsule("humerus" + s, "arm" + s, SKIN, BONE, (-0.002, 1.380, SH + 0.004), (0.000, 1.100, SH), 0.0375, 0.0375, mirror=mirror), "arm" + s),
            st(capsule("forearm-bones" + s, "forearm" + s, SKIN, BONE, (0.001, 1.070, SH), (0.000, 0.870, SH), 0.036, 0.029, mirror=mirror), "forearm" + s),
            st(sphere("mass-thigh" + s, "thigh" + s, SKIN, MASS, (0, 0.7537, HIP), 0.042, role="detail", mirror=mirror), "thigh" + s, "mass"),
        ]
    out += [
        st(sphere("mass-pelvis", "pelvis", SKIN, MASS, (0, 0.9601, 0), 0.045, role="detail"), "pelvis", "mass"),
        st(sphere("mass-torso", "torso", SKIN, MASS, (0, 1.2024, 0), 0.052, role="detail"), "torso", "mass"),
    ]
    return out + parts.hand_bones(500) + parts.phone_member(500)


def decisions():
    return texts.VOLUMES[500]
