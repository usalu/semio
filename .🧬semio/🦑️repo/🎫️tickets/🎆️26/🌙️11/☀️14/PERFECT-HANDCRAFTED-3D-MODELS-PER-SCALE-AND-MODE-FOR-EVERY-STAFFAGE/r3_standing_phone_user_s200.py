"""🧍 The standing adult at 1:200 (smallest feature 30 mm), authored anew: one mass per part of the figure, face features gone, a mitten hand without a thumb, a shoe and a toe box; the thermal model on these masses; the skeleton as thickened bones.

World of the rest pose: x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder 1.3975, C7 1.465, head joint 1.561, vertex 1.710.
"""
import r3_standing_phone_user_parts as parts
import r3_standing_phone_user_texts as texts
from r3_standing_phone_user_kit import hair_cap, hairline_x, capsule, ellipsoid, face_rows, free_loft, loft, ref, rib_rows, sphere, st, th

SKIN, HAIR, FLEECE, DENIM, CANVAS = "skin", "hair", "fleece-sweatshirt", "denim", "canvas-sneaker"
BONE, JOINT, MASS = 0.85, 0.4, 0.2
SH, HIP = 0.153, 0.085
SIDES = (("l", False), ("r", True))
HEAD = [(1.710, 0.020, 0, 0.016, 0.021), (1.694, 0.019, 0, 0.052, 0.068), (1.650, 0.020, 0, 0.073, 0.095), (1.590, 0.026, 0, 0.069, 0.092), (1.540, 0.040, 0, 0.058, 0.070), (1.502, 0.052, 0, 0.040, 0.048), (1.488, 0.060, 0, 0.020, 0.028)]
HAIR_EDGE = [(y, hairline_x(y)) for y in (1.662, 1.640, 1.615, 1.590, 1.565, 1.552)] + [(1.530, -0.1), (1.489, -0.1)]


def body():
    neck = [(1.415, -0.006, 0, 0.066, 0.062), (1.500, 0.008, 0, 0.053, 0.055), (1.568, 0.016, 0, 0.048, 0.046)]
    hoodie = [
        (0.868, -0.017, 0, 0.184, 0.120, 2.4), (0.950, -0.010, 0, 0.186, 0.122, 2.4), (1.100, -0.002, 0, 0.161, 0.115, 2.4), (1.250, 0.002, 0, 0.163, 0.122, 2.4), (1.350, -0.010, 0, 0.174, 0.110, 2.4),
        (1.410, -0.016, 0, 0.188, 0.085, 2.6), (1.438, -0.015, 0, 0.179, 0.070, 2.6), (1.455, -0.012, 0, 0.122, 0.066, 2.3), (1.474, -0.007, 0, 0.072, 0.065, 2.0),
    ]
    hood = [(1.482, -0.072, 0, 0.086, 0.024, 2.2), (1.430, -0.112, 0, 0.110, 0.042, 2.2), (1.350, -0.136, 0, 0.088, 0.030, 2.2), (1.302, -0.132, 0, 0.050, 0.017, 2.2)]
    seat = [(1.015, 0.004, 0, 0.144, 0.095, 2.3), (0.900, -0.014, 0, 0.168, 0.106, 2.3), (0.830, -0.010, 0, 0.150, 0.092, 2.2), (0.798, -0.004, 0, 0.064, 0.044, 2.0)]
    out = [
        loft("head", "head", SKIN, 0.25, HEAD, "y", e=2.15, sub=1),
        hair_cap("hair", "head", HAIR, 0.75, (0, 0.18, 0.42, 0.66, 0.86, 0.975), grow=0.005),
        loft("neck", "neck", SKIN, 0.25, neck, "y"),
        loft("hoodie", "torso", FLEECE, 0.45, hoodie, "y", sub=1),
        loft("hood", "torso", FLEECE, 0.4, hood, "y", sub=1),
        loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        sleeve = [(1.438, -0.008, SH + 0.006, 0.022, 0.026), (1.408, -0.007, SH + 0.014, 0.054, 0.060), (1.350, -0.005, SH + 0.015, 0.060, 0.066), (1.200, -0.001, SH + 0.006, 0.051, 0.056), (1.084, 0.000, SH, 0.0485, 0.0515), (1.050, 0.000, SH, 0.034, 0.036)]
        forearm = [(1.116, 0.000, SH, 0.026, 0.028), (1.060, 0.002, SH, 0.045, 0.049), (0.950, 0.002, SH, 0.041, 0.044), (0.880, 0.000, SH, 0.036, 0.041), (0.834, 0.000, SH, 0.027, 0.032)]
        thigh = [(0.974, -0.010, HIP + 0.003, 0.034, 0.038), (0.930, -0.010, HIP + 0.003, 0.077, 0.087), (0.850, -0.009, HIP + 0.004, 0.084, 0.094), (0.680, -0.001, HIP + 0.002, 0.071, 0.080), (0.540, 0.004, HIP - 0.002, 0.057, 0.063), (0.480, 0.005, HIP - 0.002, 0.055, 0.060), (0.445, 0.003, HIP - 0.002, 0.030, 0.033)]
        shank = [(0.532, 0.003, HIP - 0.002, 0.026, 0.029), (0.480, 0.002, HIP - 0.002, 0.050, 0.055), (0.360, -0.010, HIP, 0.055, 0.062), (0.220, -0.005, HIP, 0.046, 0.051), (0.105, -0.004, HIP, 0.041, 0.047)]
        shoe = [(-0.0745, 0.032, HIP, 0.020, 0.032, 3.0), (-0.050, 0.043, HIP, 0.033, 0.043, 2.9), (0.020, 0.046, HIP, 0.037, 0.046, 2.6), (0.090, 0.034, HIP + 0.002, 0.046, 0.034, 2.8), (0.157, 0.027, HIP + 0.003, 0.050, 0.027, 3.0)]
        toe = [(0.143, 0.0275, HIP + 0.003, 0.050, 0.0275, 3.0), (0.180, 0.025, HIP + 0.002, 0.043, 0.025, 3.0), (0.205, 0.020, HIP, 0.016, 0.016, 2.4)]
        out += [
            loft("sleeve" + s, "arm" + s, FLEECE, 0.45, sleeve, "y", sub=1, mirror=mirror),
            loft("forearm" + s, "forearm" + s, FLEECE, 0.45, forearm, "y", sub=1, mirror=mirror),
            loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=1, mirror=mirror),
            loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=1, mirror=mirror),
            loft("shoe" + s, "foot" + s, CANVAS, 0.12, shoe, "x", sub=1, mirror=mirror),
            loft("shoe-toe" + s, "toe" + s, CANVAS, 0.12, toe, "x", mirror=mirror),
        ]
    return out + parts.ponytail(200) + parts.headphones(200) + parts.hands(200) + parts.phone(200)


def thermal():
    out = [th(loft("th-face", "head", SKIN, 0.25, face_rows(HEAD, HAIR_EDGE), "y"), "face-skin"), ref("neck", "face-skin"), ref("hair", "hair")]
    out += [ref(i, "hoodie", "sweatshirt") for i in ("hoodie", "hood", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r")]
    out += [ref(i, "jeans", "jeans") for i in ("jeans-seat", "thigh-l", "thigh-r", "shank-l", "shank-r")]
    out += [ref(i, "sneakers", "sneaker") for i in ("shoe-l", "shoe-r", "shoe-toe-l", "shoe-toe-r")]
    out += parts.thermal_parts(200)
    out.append(th(ellipsoid("th-nose-breath", "head", SKIN, 0.1, (0.106, 1.556, 0), (0.015, 0.015, 0.015), role="detail"), "face-skin", tags=["emitter", "nose-breath"]))
    out.append(th(ellipsoid("th-mouth-breath", "head", SKIN, 0.1, (0.092, 1.524, 0), (0.015, 0.015, 0.016), role="detail"), "face-skin", tags=["emitter", "mouth-breath"]))
    out.append(th(ellipsoid("th-body-plume", "head", HAIR, 0.5, (0.012, 1.704, 0), (0.052, 0.016, 0.042), role="detail"), "hoodie", tags=["emitter", "body-plume"]))
    return out


def structure():
    skull = [(1.702, 0.018, 0, 0.018, 0.024), (1.680, 0.017, 0, 0.056, 0.074), (1.630, 0.016, 0, 0.066, 0.088), (1.585, 0.026, 0, 0.060, 0.082), (1.545, 0.058, 0, 0.042, 0.044), (1.500, 0.074, 0, 0.026, 0.026)]
    spine = [(-0.035, 0.998, 0, 0.024, 0.0155), (-0.019, 1.100, 0, 0.022, 0.0155), (-0.050, 1.300, 0, 0.017, 0.0155), (-0.020, 1.465, 0, 0.0155, 0.0155)]
    cervical = [(-0.020, 1.462, 0, 0.0155, 0.0155), (-0.002, 1.510, 0, 0.0155, 0.0155), (0.0125, 1.562, 0, 0.0155, 0.0155)]
    sacrum = [(-0.036, 0.999, 0, 0.054, 0.018), (-0.060, 0.950, 0, 0.040, 0.016), (-0.074, 0.900, 0, 0.018, 0.0155)]
    out = [
        st(loft("skull", "head", SKIN, BONE, skull, "y", sub=1), "head"),
        st(free_loft("cervical-spine", "neck", SKIN, BONE, cervical), "neck"),
        st(free_loft("spine", "torso", SKIN, BONE, spine, sub=2), "torso"),
        st(capsule("sternum", "torso", SKIN, BONE, (0.050, 1.396, 0), (0.084, 1.226, 0), 0.0155, 0.0155), "torso"),
        st(free_loft("sacrum", "pelvis", SKIN, BONE, sacrum), "pelvis"),
    ]
    ribs = [(1.405, 0.052, 0.084, -0.040, 0.058, 0.0), (1.320, 0.074, 0.110, -0.053, 0.078, 0.0), (1.225, 0.066, 0.110, -0.036, 0.074, 0.85)]
    for side, mirror in SIDES:
        s = "-" + side
        for k, (y, drop, w, sx, front, to) in enumerate(ribs):
            out.append(st(free_loft("rib%s-%d" % (s, k + 1), "torso", SKIN, BONE, rib_rows(y, drop, w, sx, front, 0.0155, to=to, steps=6), mirror=mirror), "torso"))
        ilium = [(1.030, -0.005, 0.100, 0.0155, 0.036, 2.0, 34), (0.985, -0.002, 0.092, 0.0155, 0.066, 2.4, 34), (0.915, 0.000, 0.097, 0.021, 0.038, 2.0, 10), (0.845, -0.026, 0.064, 0.0155, 0.022, 2.0, 0)]
        girdle = [(0.046, 1.404, 0.028, 0.0155, 0.0155), (0.020, 1.418, 0.110, 0.0155, 0.0155), (-0.020, 1.424, 0.160, 0.0155, 0.0155), (-0.070, 1.400, 0.110, 0.0155, 0.0155), (-0.092, 1.320, 0.098, 0.0155, 0.0155)]
        foot = [(-0.062, 0.046, HIP + 0.002, 0.016, 0.018), (0.000, 0.062, HIP, 0.022, 0.026), (0.070, 0.050, HIP + 0.001, 0.032, 0.0155), (0.152, 0.036, HIP + 0.004, 0.040, 0.0155)]
        toes = [(0.146, 0.036, HIP + 0.004, 0.040, 0.0155, 2.8), (0.196, 0.034, HIP - 0.002, 0.026, 0.0152, 2.6)]
        out += [
            st(loft("hip-bone" + s, "pelvis", SKIN, BONE, ilium, "y", sub=1, mirror=mirror), "pelvis"),
            st(free_loft("shoulder-girdle" + s, "torso", SKIN, BONE, girdle, mirror=mirror), "torso"),
            st(sphere("femoral-head" + s, "thigh" + s, SKIN, BONE, (0, 0.903, HIP), 0.022, mirror=mirror), "thigh" + s),
            st(capsule("femur" + s, "thigh" + s, SKIN, BONE, (-0.004, 0.888, 0.128), (0.003, 0.500, HIP), 0.0165, 0.024, mirror=mirror), "thigh" + s),
            st(capsule("tibia" + s, "shank" + s, SKIN, BONE, (0.001, 0.470, HIP - 0.001), (0.000, 0.094, HIP - 0.006), 0.022, 0.0155, mirror=mirror), "shank" + s),
            st(loft("foot-bones" + s, "foot" + s, SKIN, BONE, foot, "x", sub=1, mirror=mirror), "foot" + s),
            st(loft("toe-bones" + s, "toe" + s, SKIN, BONE, toes, "x", mirror=mirror), "toe" + s),
            st(capsule("humerus" + s, "arm" + s, SKIN, BONE, (-0.001, 1.3975, SH), (0.000, 1.092, SH), 0.020, 0.0155, mirror=mirror), "arm" + s),
            st(capsule("forearm-bones" + s, "forearm" + s, SKIN, BONE, (-0.003, 1.082, SH), (-0.002, 0.848, SH), 0.0185, 0.0155, mirror=mirror), "forearm" + s),
            st(sphere("joint-hip" + s, "thigh" + s, SKIN, JOINT, (0, 0.903, HIP), 0.026, mirror=mirror), "thigh" + s, "joint"),
            st(sphere("joint-knee" + s, "shank" + s, SKIN, JOINT, (0, 0.4898, HIP), 0.027, mirror=mirror), "shank" + s, "joint"),
            st(sphere("joint-shoulder" + s, "arm" + s, SKIN, JOINT, (0, 1.3975, SH), 0.0245, mirror=mirror), "arm" + s, "joint"),
            st(sphere("joint-elbow" + s, "forearm" + s, SKIN, JOINT, (0, 1.084, SH), 0.022, mirror=mirror), "forearm" + s, "joint"),
            st(sphere("mass-thigh" + s, "thigh" + s, SKIN, MASS, (0, 0.7537, HIP), 0.024, role="detail", mirror=mirror), "thigh" + s, "mass"),
        ]
    out += [
        st(sphere("mass-pelvis", "pelvis", SKIN, MASS, (0, 0.9601, 0), 0.022, role="detail"), "pelvis", "mass"),
        st(sphere("mass-torso", "torso", SKIN, MASS, (0, 1.2024, 0), 0.030, role="detail"), "torso", "mass"),
    ]
    return out + parts.hand_bones(200) + parts.phone_member(200)


def decisions():
    return texts.VOLUMES[200]
