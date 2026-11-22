"""🧍 standing-adult: the body lists, one hand-stated model per scale. Every number is metres in the world of the rest pose (x ahead, y up, z left; ground y = 0) or along a limb bone from its joint.

Subject: a neutral adult of 1.69 m and 58 kg in a fleece hooded sweatshirt (hood down), slim jeans and canvas sneakers.
"""
import importlib.util
import os

_spec = importlib.util.spec_from_file_location("rework_01_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-01-lib.py"))
lib = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(lib)
H, X, Z, station, tube, polyline = lib.H, lib.X, lib.Z, lib.station, lib.tube, lib.polyline

SKIN, HAIR, FLEECE, DENIM, SOCK, SHOE = "skin", "hair", "fleece-sweatshirt", "denim", "cotton-sock", "canvas-sneaker"
T_SKIN, T_HAND, T_HAIR, T_FLEECE, T_RIB, T_DENIM, T_SOCK, T_CANVAS, T_RUBBER = 0.25, 0.3, 0.75, 0.45, 0.35, 0.62, 0.8, 0.12, 0.88
HX, HY = 0.0132, 1.561
SHOULDER_Z, HIP_Z = 0.153, 0.085
SIDES = ((1, "l"), (-1, "r"))
FINGER_RADII = {
    "thumb-metacarpal": (0.0115, 0.0100), "thumb-proximal": (0.0100, 0.0090), "thumb-distal": (0.0090, 0.0072),
    "index-proximal": (0.0088, 0.0080), "index-middle": (0.0078, 0.0072), "index-distal": (0.0070, 0.0058),
    "middle-proximal": (0.0090, 0.0082), "middle-middle": (0.0080, 0.0073), "middle-distal": (0.0071, 0.0059),
    "ring-proximal": (0.0085, 0.0077), "ring-middle": (0.0075, 0.0069), "ring-distal": (0.0067, 0.0056),
    "little-proximal": (0.0075, 0.0068), "little-middle": (0.0066, 0.0061), "little-distal": (0.0059, 0.0050),
}

HOODIE_HEM = [(0.905, 0.098, -0.112, 0.170), (0.932, 0.101, -0.115, 0.173), (0.962, 0.100, -0.111, 0.170)]
HOODIE = [(0.950, 0.100, -0.108, 0.167, 2.5), (0.985, 0.110, -0.107, 0.174, 2.5), (1.030, 0.113, -0.099, 0.171, 2.5), (1.090, 0.112, -0.092, 0.164, 2.5), (1.165, 0.1145, -0.100, 0.162, 2.5), (1.220, 0.119, -0.112, 0.166, 2.5), (1.280, 0.120, -0.124, 0.174, 2.5), (1.330, 0.114, -0.128, 0.184, 2.5), (1.370, 0.104, -0.124, 0.190, 2.4), (1.400, 0.092, -0.114, 0.190, 2.3), (1.425, 0.080, -0.102, 0.176, 2.2), (1.445, 0.068, -0.090, 0.136, 2.1), (1.462, 0.058, -0.078, 0.080, 2.0)]
HOODIE_WEIGHTS = [{"bone": "torso", "weight": 0.6}, {"bone": "lumbar", "weight": 0.4}]
SEAT = [(0.795, 0.045, -0.055, 0.060), (0.830, 0.080, -0.098, 0.140), (0.870, 0.090, -0.112, 0.166), (0.905, 0.092, -0.110, 0.168), (0.950, 0.094, -0.100, 0.160), (0.990, 0.094, -0.090, 0.146)]
THIGH = [(-0.030, 0.075, 0.085, 0.072, 0.082), (0.030, 0.088, 0.094, 0.080, 0.086), (0.090, 0.090, 0.090, 0.080, 0.084), (0.170, 0.086, 0.080, 0.074, 0.078), (0.250, 0.076, 0.070, 0.066, 0.070), (0.320, 0.066, 0.062, 0.060, 0.063), (0.380, 0.060, 0.058, 0.057, 0.059), (0.41325, 0.058, 0.057, 0.056, 0.057)]
SHANK = [(-0.010, 0.056, 0.056, 0.055, 0.056), (0.040, 0.050, 0.058, 0.053, 0.054), (0.100, 0.045, 0.076, 0.057, 0.054), (0.150, 0.043, 0.080, 0.057, 0.053), (0.210, 0.041, 0.068, 0.050, 0.048), (0.280, 0.040, 0.052, 0.044, 0.043), (0.340, 0.041, 0.046, 0.042, 0.042), (0.375, 0.043, 0.047, 0.044, 0.044)]
SLEEVE = [(0.000, 0.052, 0.052, 0.050, 0.056), (0.050, 0.056, 0.054, 0.050, 0.066), (0.120, 0.053, 0.051, 0.048, 0.060), (0.200, 0.049, 0.048, 0.046, 0.053), (0.270, 0.046, 0.047, 0.044, 0.049), (0.3135, 0.044, 0.049, 0.045, 0.046)]
FOREARM = [(0.000, 0.044, 0.049, 0.045, 0.046), (0.050, 0.047, 0.045, 0.045, 0.047), (0.110, 0.043, 0.042, 0.042, 0.043), (0.170, 0.039, 0.039, 0.038, 0.039), (0.205, 0.040, 0.040, 0.039, 0.040), (0.222, 0.037, 0.037, 0.035, 0.036)]
CUFF = [(0.214, 0.034, 0.034, 0.029, 0.029), (0.235, 0.033, 0.033, 0.027, 0.027), (0.254, 0.033, 0.033, 0.027, 0.027)]
PALM = [(-0.020, 0.026, 0.026, 0.017, 0.017, 2.3), (0.005, 0.028, 0.028, 0.016, 0.016, 2.3), (0.030, 0.034, 0.033, 0.0145, 0.0145, 2.6), (0.060, 0.040, 0.039, 0.0135, 0.0135, 2.6), (0.085, 0.041, 0.040, 0.0125, 0.0125, 2.6), (0.098, 0.039, 0.037, 0.011, 0.011, 2.6)]
SOLE = [(-0.058, 0.0, 0.027, 0.027), (-0.040, 0.0, 0.027, 0.033), (0.000, 0.0, 0.026, 0.036), (0.040, 0.0, 0.024, 0.038), (0.090, 0.0, 0.022, 0.046), (0.130, 0.0, 0.021, 0.050), (0.156, 0.0, 0.020, 0.050)]
TOE_SOLE = [(0.146, 0.0, 0.020, 0.050), (0.175, 0.001, 0.020, 0.048), (0.195, 0.003, 0.020, 0.040), (0.207, 0.005, 0.020, 0.028)]
UPPER = [(-0.054, 0.020, 0.070, 0.025), (-0.035, 0.020, 0.080, 0.031), (-0.010, 0.020, 0.078, 0.034), (0.020, 0.020, 0.080, 0.036), (0.050, 0.020, 0.074, 0.039), (0.090, 0.020, 0.060, 0.044), (0.130, 0.018, 0.048, 0.047), (0.156, 0.018, 0.043, 0.047)]
UPPER_TOE = [(0.146, 0.018, 0.044, 0.047), (0.172, 0.018, 0.040, 0.045)]
TOE_CAP = [(0.168, 0.016, 0.041, 0.046), (0.185, 0.016, 0.038, 0.042), (0.198, 0.016, 0.034, 0.034), (0.207, 0.016, 0.029, 0.024)]
FACE = [(0.120, 0.072, 0.020, 0.044), (0.100, 0.090, 0.010, 0.057), (0.075, 0.098, 0.000, 0.064), (0.055, 0.102, -0.005, 0.068), (0.038, 0.098, -0.010, 0.069), (0.012, 0.100, -0.012, 0.068), (-0.015, 0.098, -0.010, 0.062), (-0.040, 0.094, -0.002, 0.053), (-0.060, 0.091, 0.020, 0.040), (-0.073, 0.085, 0.040, 0.026)]
NOSE = [(0.046, 0.104, 0.085, 0.008), (0.028, 0.113, 0.085, 0.010), (0.010, 0.123, 0.085, 0.014), (-0.002, 0.126, 0.085, 0.017), (-0.010, 0.113, 0.085, 0.015)]
NECK = [(1.435, 0.045, -0.062, 0.060), (1.480, 0.050, -0.050, 0.054), (1.520, 0.058, -0.040, 0.051), (1.560, 0.062, -0.035, 0.050)]


def grow(rows, d):
    """📏 The rows of a limb with every extent changed by d (a garment layer under or over the stated surface)."""
    return [(r[0], r[1] + d, r[2] + d, r[3] + d, r[4] + d) + tuple(r[5:]) for r in rows]


def hrows(rows, d=0.0, e=2.5, zc=0.0):
    """📍 Horizontal stations from rows (y, front x, back x, half width[, exponent]) with the surface moved by d."""
    return [H(r[0], r[1] + d, r[2] - d, r[3] + d, zc, r[4] if len(r) > 4 else e) for r in rows]


def xrows(rows, side, d=0.0, e=2.6, y0=0.0):
    """📍 Cross-sections of a shoe from rows (x from the ankle, bottom, top, half width) about the foot axis z = side * HIP_Z."""
    return [X(r[0], y0 + r[1] + d * (r[1] > 0), y0 + r[2] - d, r[3] - d, side * HIP_Z, e) for r in rows]


def head(k, detail):
    out = [k.ellipsoid("head", "head", SKIN, T_SKIN, [HX + 0.000, HY + 0.066, 0], [0.092, 0.084, 0.0745])]
    face = [H(HY + r[0], HX + r[1], HX + r[2], r[3], 0, 2.3) for r in FACE]
    out.append(k.loft("face", "head", SKIN, T_SKIN, face, n=detail, start=0.006, end=0.004))
    nose = [H(HY + r[0], HX + r[1], HX + r[2], r[3], 0, 2.0) for r in NOSE]
    out.append(k.loft("nose", "head", SKIN, T_SKIN, nose, n=min(detail, 2), role="detail", end=0.004))
    out.append(k.ellipsoid("hair", "head", HAIR, T_HAIR, [HX - 0.012, HY + 0.078, 0], [0.092, 0.083, 0.079]))
    return out


def face_details(k):
    out = [k.ellipsoid("lips", "head", SKIN, 0.2, [HX + 0.094, HY - 0.030, 0], [0.008, 0.007, 0.023], role="detail")]
    out.append(k.ellipsoid("chin", "head", SKIN, T_SKIN, [HX + 0.081, HY - 0.058, 0], [0.014, 0.014, 0.022], role="detail"))
    for side, s in SIDES:
        out.append(k.ellipsoid(f"eye-{s}", "head", SKIN, 0.1, [HX + 0.095, HY + 0.040, side * 0.031], [0.004, 0.004, 0.010], role="detail"))
        out.append(k.ellipsoid(f"brow-{s}", "head", HAIR, 0.2, [HX + 0.099, HY + 0.054, side * 0.032], [0.005, 0.004, 0.017], role="detail"))
    return out


def ears(k):
    return [k.ellipsoid(f"ear-{s}", "head", SKIN, 0.3, [HX - 0.004, HY + 0.012, side * 0.066], [0.014, 0.031, 0.010], role="detail", tilt=-12) for side, s in SIDES]


def hoodie(k, detail):
    out = [k.loft("hem-rib", "pelvis", FLEECE, T_RIB, hrows(HOODIE_HEM), n=1, start=0.006, end=0.006)]
    out.append(k.loft("hoodie", "torso", FLEECE, T_FLEECE, hrows(HOODIE), n=detail, weights=HOODIE_WEIGHTS))
    return out


def hood(k, detail):
    pouch = [Z(-0.115, -0.100, 1.420, 0.014, 0.030), Z(-0.075, -0.114, 1.410, 0.030, 0.062), Z(0.0, -0.120, 1.400, 0.034, 0.072), Z(0.075, -0.114, 1.410, 0.030, 0.062), Z(0.115, -0.100, 1.420, 0.014, 0.030)]
    out = [k.loft("hood", "torso", FLEECE, 0.4, pouch, n=detail, start=0.006, end=0.006)]
    for side, s in SIDES:
        path = polyline([[-0.085, 1.476, side * 0.045], [-0.050, 1.469, side * 0.078], [0.000, 1.453, side * 0.085], [0.045, 1.433, side * 0.060], [0.070, 1.406, side * 0.030]], detail)
        out.append(k.loft(f"hood-roll-{s}", "torso", FLEECE, 0.4, tube(path, (0.016, 0.020)), start=0.008, end=0.008))
    return out


def hoodie_details(k):
    pocket = [H(0.985, 0.116, 0.050, 0.092, 0, 2.6), H(1.030, 0.119, 0.050, 0.088, 0, 2.6), H(1.080, 0.118, 0.050, 0.076, 0, 2.6), H(1.110, 0.118, 0.050, 0.062, 0, 2.6)]
    out = [k.loft("kangaroo-pocket", "torso", FLEECE, 0.42, pocket, n=2, role="detail", start=0.004, end=0.004, weights=HOODIE_WEIGHTS)]
    out.append(k.loft("collar-rib", "torso", FLEECE, 0.4, [H(1.452, 0.064, -0.072, 0.072), H(1.476, 0.060, -0.064, 0.066)], start=0.005, end=0.005))
    for side, s in SIDES:
        out.append(k.capsule(f"drawstring-{s}", "torso", FLEECE, 0.15, [0.095, 1.394, side * 0.032], [0.120, 1.292, side * 0.036], 0.004, role="detail"))
    return out


def seat(k, detail):
    out = [k.loft("jeans-seat", "pelvis", DENIM, T_DENIM, hrows(SEAT), n=detail, start=0.012)]
    for side, s in SIDES:
        out.append(k.ellipsoid(f"buttock-{s}", "pelvis", DENIM, T_DENIM, [-0.062, 0.885, side * 0.078], [0.058, 0.075, 0.072]))
    return out


def leg(k, side, s, detail):
    out = [k.loft(f"thigh-{s}", f"thigh-{s}", DENIM, T_DENIM, k.limb(f"thigh-{s}", side, THIGH), n=detail, end=0.035)]
    out.append(k.ellipsoid(f"kneecap-{s}", f"thigh-{s}", DENIM, T_DENIM, [0.046, 0.498, side * HIP_Z], [0.011, 0.029, 0.026]))
    out.append(k.loft(f"shank-{s}", f"shank-{s}", DENIM, T_DENIM, k.limb(f"shank-{s}", side, SHANK), n=detail, start=0.030))
    sock = [(0.350, 0.034, 0.038, 0.033, 0.033), (0.390, 0.034, 0.040, 0.034, 0.035), (0.420, 0.036, 0.044, 0.034, 0.036)]
    out.append(k.loft(f"sock-{s}", f"shank-{s}", SOCK, T_SOCK, k.limb(f"shank-{s}", side, sock)))
    return out


def shoe(k, side, s, detail):
    out = [k.loft(f"sole-{s}", f"foot-{s}", SHOE, T_RUBBER, xrows(SOLE, side, e=4.0), n=detail, start=0.006)]
    out.append(k.loft(f"shoe-{s}", f"foot-{s}", SHOE, T_CANVAS, xrows(UPPER, side), n=detail, start=0.005))
    out.append(k.loft(f"toe-sole-{s}", f"toe-{s}", SHOE, T_RUBBER, xrows(TOE_SOLE, side, e=4.0), n=detail, end=0.006))
    out.append(k.loft(f"shoe-toe-{s}", f"toe-{s}", SHOE, T_CANVAS, xrows(UPPER_TOE, side)))
    out.append(k.loft(f"toe-cap-{s}", f"toe-{s}", SHOE, T_RUBBER, xrows(TOE_CAP, side), n=detail, end=0.006))
    return out


def laces(k, side, s):
    rows = [(0.040, 0.0765, -6), (0.058, 0.0725, -14), (0.076, 0.0665, -19), (0.094, 0.0600, -19), (0.112, 0.0545, -17)]
    return [k.box(f"lace-{s}-{i}", f"foot-{s}", SHOE, 0.85, [x, y, side * HIP_Z], [0.006, 0.004, 0.036 + 0.004 * i], role="texture", tilt=tilt, texture=True) for i, (x, y, tilt) in enumerate(rows)]


def arm(k, side, s, detail):
    out = [k.loft(f"sleeve-{s}", f"arm-{s}", FLEECE, T_FLEECE, k.limb(f"arm-{s}", side, SLEEVE), n=detail, start=0.034, end=0.040)]
    out.append(k.loft(f"forearm-{s}", f"forearm-{s}", FLEECE, T_FLEECE, k.limb(f"forearm-{s}", side, FOREARM), n=detail, start=0.040, end=0.008))
    out.append(k.loft(f"cuff-rib-{s}", f"forearm-{s}", FLEECE, T_RIB, k.limb(f"forearm-{s}", side, CUFF), start=0.004, end=0.004))
    return out


def hand(k, side, s, detail):
    f = k.f
    out = [k.loft(f"hand-{s}", f"hand-{s}", SKIN, T_HAND, k.limb(f"hand-{s}", side, PALM), n=detail, end=0.008)]
    wrist = f.rest[f"hand-{s}"]
    out.append(k.ellipsoid(f"thenar-{s}", f"hand-{s}", SKIN, T_HAND, [wrist[0] + 0.020, wrist[1] - 0.040, wrist[2] - side * 0.010], [0.017, 0.032, 0.012]))
    out.append(k.ellipsoid(f"hypothenar-{s}", f"hand-{s}", SKIN, T_HAND, [wrist[0] - 0.027, wrist[1] - 0.050, wrist[2] - side * 0.006], [0.012, 0.036, 0.011]))
    for name, (r0, r1) in FINGER_RADII.items():
        bone = f"{name}-{s}"
        length = f.bones[bone]["lengthM"]
        tip = length - r1 if name.endswith("distal") else length
        out.append(k.capsule(f"skin-{bone}", bone, SKIN, T_HAND, f.world(bone, [0, 0, 0]), f.world(bone, [tip, 0, 0]), r0, r1))
    return out


def body50(k):
    out = head(k, 3) + face_details(k) + ears(k)
    out.append(k.loft("neck", "neck", SKIN, T_SKIN, hrows(NECK, e=2.0), n=2))
    out += hoodie(k, 3) + hood(k, 3) + hoodie_details(k) + seat(k, 3)
    for side, s in SIDES:
        out += arm(k, side, s, 3) + hand(k, side, s, 2) + leg(k, side, s, 3) + shoe(k, side, s, 2) + laces(k, side, s)
    return out


def pick(rows, *index):
    """🎯 The stated rows a coarser scale keeps, named one by one."""
    return [rows[i] for i in index]


def body100(k):
    out = [k.ellipsoid("head", "head", SKIN, T_SKIN, [HX + 0.000, HY + 0.066, 0], [0.092, 0.084, 0.0745])]
    out.append(k.loft("face", "head", SKIN, T_SKIN, [H(HY + r[0], HX + r[1], HX + r[2], r[3], 0, 2.3) for r in pick(FACE, 0, 1, 3, 5, 7, 8, 9)], n=2, end=0.004))
    out.append(k.loft("nose", "head", SKIN, T_SKIN, [H(HY + r[0], HX + r[1], HX + r[2], r[3], 0, 2.0) for r in pick(NOSE, 0, 2, 3, 4)], role="detail", end=0.004))
    out.append(k.ellipsoid("hair", "head", HAIR, T_HAIR, [HX - 0.012, HY + 0.078, 0], [0.092, 0.083, 0.079]))
    out += ears(k)
    out.append(k.loft("neck", "neck", SKIN, T_SKIN, hrows(pick(NECK, 0, 2, 3), e=2.0)))
    out.append(k.loft("hem-rib", "pelvis", FLEECE, T_RIB, hrows(pick(HOODIE_HEM, 0, 2)), start=0.008, end=0.008))
    out.append(k.loft("hoodie", "torso", FLEECE, T_FLEECE, hrows(pick(HOODIE, 0, 1, 3, 5, 6, 7, 8, 9, 10, 11, 12)), n=2, weights=HOODIE_WEIGHTS))
    pouch = [Z(-0.115, -0.100, 1.420, 0.015, 0.030), Z(0.0, -0.120, 1.400, 0.034, 0.072), Z(0.115, -0.100, 1.420, 0.015, 0.030)]
    out.append(k.loft("hood", "torso", FLEECE, 0.4, pouch, n=3, start=0.008, end=0.008))
    for side, s in SIDES:
        path = polyline([[-0.085, 1.476, side * 0.045], [-0.030, 1.462, side * 0.084], [0.045, 1.433, side * 0.060], [0.070, 1.406, side * 0.030]], 2)
        out.append(k.loft(f"hood-roll-{s}", "torso", FLEECE, 0.4, tube(path, (0.017, 0.020)), start=0.008, end=0.008))
    pocket = [H(0.985, 0.119, 0.050, 0.092, 0, 2.6), H(1.050, 0.121, 0.050, 0.084, 0, 2.6), H(1.110, 0.120, 0.050, 0.062, 0, 2.6)]
    out.append(k.loft("kangaroo-pocket", "torso", FLEECE, 0.42, pocket, role="detail", start=0.005, end=0.005, weights=HOODIE_WEIGHTS))
    out.append(k.loft("jeans-seat", "pelvis", DENIM, T_DENIM, hrows(pick(SEAT, 0, 1, 2, 4, 5)), n=2, start=0.012))
    for side, s in SIDES:
        out.append(k.ellipsoid(f"buttock-{s}", "pelvis", DENIM, T_DENIM, [-0.062, 0.885, side * 0.078], [0.058, 0.075, 0.072]))
        out.append(k.loft(f"sleeve-{s}", f"arm-{s}", FLEECE, T_FLEECE, k.limb(f"arm-{s}", side, pick(SLEEVE, 0, 1, 3, 5)), n=2, start=0.034, end=0.040, steps=2))
        out.append(k.loft(f"forearm-{s}", f"forearm-{s}", FLEECE, T_FLEECE, k.limb(f"forearm-{s}", side, pick(FOREARM, 0, 1, 3, 4, 5)), n=1, start=0.040, end=0.008, steps=2))
        out.append(k.loft(f"cuff-rib-{s}", f"forearm-{s}", FLEECE, T_RIB, k.limb(f"forearm-{s}", side, pick(CUFF, 0, 2)), start=0.004, end=0.004, steps=2))
        f = k.f
        out.append(k.loft(f"hand-{s}", f"hand-{s}", SKIN, T_HAND, k.limb(f"hand-{s}", side, pick(PALM, 0, 2, 4, 5)), end=0.008, steps=2))
        wrist = f.rest[f"hand-{s}"]
        out.append(k.ellipsoid(f"thenar-{s}", f"hand-{s}", SKIN, T_HAND, [wrist[0] + 0.020, wrist[1] - 0.040, wrist[2] - side * 0.010], [0.017, 0.032, 0.012]))
        for name, (r0, r1) in FINGER_RADII.items():
            bone = f"{name}-{s}"
            length = f.bones[bone]["lengthM"]
            r = max(r0, 0.0075)
            out.append(k.capsule(f"skin-{bone}", bone, SKIN, T_HAND, f.world(bone, [0, 0, 0]), f.world(bone, [length - (r if name.endswith("distal") else 0), 0, 0]), r, max(r1, 0.0068)))
        out.append(k.loft(f"thigh-{s}", f"thigh-{s}", DENIM, T_DENIM, k.limb(f"thigh-{s}", side, pick(THIGH, 0, 1, 3, 5, 7)), n=2, end=0.035, steps=2))
        out.append(k.ellipsoid(f"kneecap-{s}", f"thigh-{s}", DENIM, T_DENIM, [0.046, 0.498, side * HIP_Z], [0.011, 0.029, 0.026]))
        out.append(k.loft(f"shank-{s}", f"shank-{s}", DENIM, T_DENIM, k.limb(f"shank-{s}", side, pick(SHANK, 0, 2, 3, 5, 7)), n=2, start=0.030, steps=2))
        out.append(k.loft(f"sock-{s}", f"shank-{s}", SOCK, T_SOCK, k.limb(f"shank-{s}", side, [(0.350, 0.034, 0.038, 0.033, 0.033), (0.420, 0.036, 0.044, 0.034, 0.036)])))
        out.append(k.loft(f"sole-{s}", f"foot-{s}", SHOE, T_RUBBER, xrows(pick(SOLE, 0, 2, 4, 6), side, e=4.0), start=0.006, steps=2))
        out.append(k.loft(f"shoe-{s}", f"foot-{s}", SHOE, T_CANVAS, xrows(pick(UPPER, 0, 1, 3, 5, 7), side), start=0.005, steps=2))
        out.append(k.loft(f"toe-sole-{s}", f"toe-{s}", SHOE, T_RUBBER, xrows(pick(TOE_SOLE, 0, 2, 3), side, e=4.0), end=0.006, steps=2))
        out.append(k.loft(f"toe-cap-{s}", f"toe-{s}", SHOE, T_RUBBER, xrows([(0.146, 0.016, 0.045, 0.048), (0.185, 0.016, 0.039, 0.043), (0.207, 0.016, 0.030, 0.025)], side), end=0.006, steps=2))
    return out


HEAD_200 = [(0.128, 0.060, -0.066, 0.050), (0.075, 0.100, -0.092, 0.075), (0.020, 0.101, -0.088, 0.070), (-0.030, 0.096, -0.040, 0.058), (-0.068, 0.088, 0.030, 0.030)]
MITTEN = [(-0.015, 0.027, 0.027, 0.016, 0.016, 2.4), (0.050, 0.040, 0.039, 0.016, 0.016, 2.6), (0.115, 0.038, 0.037, 0.016, 0.016, 2.6), (0.170, 0.026, 0.026, 0.015, 0.015, 2.4)]
SHOE_200 = [(-0.056, 0.0, 0.066, 0.027), (-0.020, 0.0, 0.080, 0.035), (0.040, 0.0, 0.076, 0.038), (0.100, 0.0, 0.056, 0.047), (0.156, 0.0, 0.043, 0.050)]
TOE_200 = [(0.146, 0.0, 0.044, 0.050), (0.185, 0.001, 0.039, 0.044), (0.208, 0.004, 0.031, 0.027)]


def body200(k):
    out = [k.loft("head", "head", SKIN, T_SKIN, [H(HY + r[0], HX + r[1], HX + r[2], r[3], 0, 2.2) for r in HEAD_200], n=2, start=0.022, end=0.006)]
    out.append(k.ellipsoid("hair", "head", HAIR, T_HAIR, [HX - 0.012, HY + 0.078, 0], [0.092, 0.083, 0.079]))
    out.append(k.loft("neck", "neck", SKIN, T_SKIN, hrows(pick(NECK, 0, 3), e=2.0)))
    rows = [(0.905, 0.099, -0.113, 0.171, 2.5)] + pick(HOODIE, 1, 3, 6, 8, 9, 11, 12)
    out.append(k.loft("hoodie", "torso", FLEECE, T_FLEECE, hrows(rows), n=2, start=0.006, weights=HOODIE_WEIGHTS))
    pouch = [Z(-0.105, -0.104, 1.418, 0.018, 0.034), Z(0.0, -0.120, 1.400, 0.034, 0.072), Z(0.105, -0.104, 1.418, 0.018, 0.034)]
    out.append(k.loft("hood", "torso", FLEECE, 0.4, pouch, n=2, start=0.012, end=0.012, steps=2))
    out.append(k.loft("jeans-seat", "pelvis", DENIM, T_DENIM, hrows([(0.795, 0.045, -0.060, 0.060), (0.840, 0.082, -0.112, 0.146), (0.890, 0.092, -0.120, 0.168), (0.990, 0.094, -0.092, 0.146)]), n=2, start=0.012))
    for side, s in SIDES:
        out.append(k.loft(f"sleeve-{s}", f"arm-{s}", FLEECE, T_FLEECE, k.limb(f"arm-{s}", side, pick(SLEEVE, 0, 1, 3, 5)), start=0.034, end=0.040, steps=2))
        out.append(k.loft(f"forearm-{s}", f"forearm-{s}", FLEECE, T_FLEECE, k.limb(f"forearm-{s}", side, [FOREARM[0], FOREARM[2], FOREARM[4], (0.254, 0.034, 0.034, 0.030, 0.030)]), start=0.040, end=0.004, steps=2))
        out.append(k.loft(f"hand-{s}", f"hand-{s}", SKIN, T_HAND, k.limb(f"hand-{s}", side, MITTEN), end=0.010, steps=2))
        out.append(k.loft(f"thigh-{s}", f"thigh-{s}", DENIM, T_DENIM, k.limb(f"thigh-{s}", side, pick(THIGH, 0, 1, 3, 5, 7)), end=0.035, steps=2))
        out.append(k.loft(f"shank-{s}", f"shank-{s}", DENIM, T_DENIM, k.limb(f"shank-{s}", side, pick(SHANK, 0, 3, 5) + [(0.420, 0.040, 0.046, 0.040, 0.040)]), start=0.030, steps=2))
        out.append(k.loft(f"shoe-{s}", f"foot-{s}", SHOE, T_CANVAS, xrows(SHOE_200, side, e=2.8), start=0.006, steps=2))
        out.append(k.loft(f"shoe-toe-{s}", f"toe-{s}", SHOE, T_CANVAS, xrows(TOE_200, side, e=2.8), end=0.006, steps=2))
    return out


HEAD_500 = [(0.122, 0.068, -0.072, 0.056), (0.070, 0.100, -0.092, 0.075), (0.000, 0.100, -0.080, 0.070), (-0.072, 0.090, 0.012, 0.038)]
TRUNK_500 = [(0.905, 0.099, -0.115, 0.171, 2.5), (1.060, 0.112, -0.096, 0.168, 2.5), (1.280, 0.120, -0.132, 0.176, 2.5), (1.380, 0.102, -0.150, 0.192, 2.4), (1.462, 0.060, -0.082, 0.090, 2.0)]
ARM_500 = [(0.000, 0.052, 0.052, 0.050, 0.058), (0.3135, 0.045, 0.048, 0.045, 0.047)]
FOREARM_500 = [(0.000, 0.045, 0.048, 0.045, 0.047), (0.250, 0.038, 0.038, 0.038, 0.038)]
HAND_500 = [(-0.010, 0.044, 0.044, 0.0565, 0.019, 2.4), (0.170, 0.044, 0.044, 0.0565, 0.019, 2.4)]
THIGH_500 = [(-0.030, 0.076, 0.086, 0.074, 0.084), (0.060, 0.090, 0.092, 0.080, 0.086), (0.41325, 0.058, 0.057, 0.056, 0.057)]
SHANK_500 = [(0.000, 0.056, 0.058, 0.055, 0.056), (0.140, 0.044, 0.078, 0.056, 0.053), (0.400, 0.040, 0.046, 0.040, 0.040)]
SHOE_500 = [(-0.056, 0.0, 0.075, 0.038), (0.050, 0.0, 0.076, 0.042), (0.207, 0.0, 0.075, 0.042)]


def body500(k):
    out = [k.loft("head", "head", SKIN, T_SKIN, [H(HY + r[0], HX + r[1], HX + r[2], r[3], 0, 2.2) for r in HEAD_500], start=0.027, end=0.004, steps=2)]
    out.append(k.ellipsoid("hair", "head", HAIR, T_HAIR, [HX - 0.014, HY + 0.080, 0], [0.094, 0.081, 0.082]))
    out.append(k.loft("neck", "neck", SKIN, T_SKIN, hrows(pick(NECK, 0, 3), e=2.0)))
    out.append(k.loft("hoodie", "torso", FLEECE, T_FLEECE, hrows(TRUNK_500), weights=HOODIE_WEIGHTS))
    out.append(k.loft("jeans-seat", "pelvis", DENIM, T_DENIM, hrows([(0.800, 0.060, -0.080, 0.090), (0.880, 0.092, -0.120, 0.168), (0.990, 0.094, -0.092, 0.146)])))
    for side, s in SIDES:
        out.append(k.loft(f"sleeve-{s}", f"arm-{s}", FLEECE, T_FLEECE, k.limb(f"arm-{s}", side, ARM_500), start=0.034, end=0.030, steps=1))
        out.append(k.loft(f"forearm-{s}", f"forearm-{s}", FLEECE, T_FLEECE, k.limb(f"forearm-{s}", side, FOREARM_500), start=0.030, steps=1))
        out.append(k.loft(f"hand-{s}", f"hand-{s}", SKIN, T_HAND, k.limb(f"hand-{s}", side, HAND_500), end=0.012, steps=1))
        out.append(k.loft(f"thigh-{s}", f"thigh-{s}", DENIM, T_DENIM, k.limb(f"thigh-{s}", side, THIGH_500), end=0.030, steps=1))
        out.append(k.loft(f"shank-{s}", f"shank-{s}", DENIM, T_DENIM, k.limb(f"shank-{s}", side, SHANK_500), start=0.030, steps=1))
        out.append(k.loft(f"shoe-{s}", f"foot-{s}", SHOE, T_CANVAS, xrows(SHOE_500, side, e=3.0), start=0.006, end=0.006, steps=1))
    return out
