"""🦴 standing-adult: the structural lists, the human skeleton as the load path, one hand-stated model per scale. World metres of the rest pose (x ahead, y up, z left).

Skull with mandible; cervical lordosis, thoracic kyphosis and lumbar lordosis of 24 vertebrae; sacrum and coccyx; twelve ribs per side from their vertebra round to the costal cartilage and the sternum; clavicle and scapula on the shoulder-girdle bone; hip bones with ilium, ischium and pubis; the long bones with their epiphyses; hands and feet.
"""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("rework_01_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-01-lib.py"))
lib = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(lib)
H, X, Z, station, tube, polyline = lib.H, lib.X, lib.Z, lib.station, lib.tube, lib.polyline

BONE, CARTILAGE, T_BONE, T_JOINT, T_MASS = "bone", "costal-cartilage", 0.85, 0.4, 0.2
HX, HY = 0.0132, 1.561
SHOULDER_Z, HIP_Z = 0.153, 0.085
SIDES = ((1, "l"), (-1, "r"))
FINGERS = ["thumb-metacarpal", "thumb-proximal", "thumb-distal"] + [f"{f}-{p}" for f in ["index", "middle", "ring", "little"] for p in ["proximal", "middle", "distal"]]

CERVICAL = [(0.012, 1.560), (0.010, 1.548), (0.006, 1.534), (0.002, 1.520), (-0.003, 1.505), (-0.009, 1.490), (-0.016, 1.474)]
THORACIC = [(-0.026, 1.455), (-0.037, 1.430), (-0.047, 1.405), (-0.056, 1.380), (-0.063, 1.355), (-0.068, 1.330), (-0.070, 1.305), (-0.069, 1.280), (-0.065, 1.255), (-0.058, 1.230), (-0.050, 1.205), (-0.041, 1.180)]
LUMBAR = [(-0.034, 1.150), (-0.028, 1.118), (-0.025, 1.085), (-0.026, 1.051), (-0.031, 1.018)]
SACRUM = [(-0.040, 0.990, 0.056, 0.020), (-0.060, 0.955, 0.046, 0.016), (-0.078, 0.920, 0.028, 0.012), (-0.086, 0.890, 0.014, 0.008), (-0.082, 0.868, 0.0075, 0.0075)]
STERNUM = [(0.062, 1.392, 0.028, 0.007), (0.078, 1.345, 0.019, 0.006), (0.090, 1.280, 0.017, 0.006), (0.098, 1.215, 0.015, 0.0055), (0.098, 1.185, 0.008, 0.004)]
RIB_FRONT = [(0.050, 1.388, 0.040), (0.070, 1.342, 0.050), (0.082, 1.305, 0.060), (0.090, 1.270, 0.070), (0.094, 1.238, 0.080), (0.094, 1.208, 0.092), (0.090, 1.180, 0.102), (0.080, 1.158, 0.112), (0.066, 1.140, 0.120), (0.048, 1.128, 0.124), (0.000, 1.130, 0.122), (-0.035, 1.150, 0.085)]
RIB_BREADTH = [0.060, 0.085, 0.105, 0.118, 0.126, 0.131, 0.134, 0.134, 0.131, 0.126, 0.118, 0.085]
STERNAL_END = [(0.060, 1.383), (0.077, 1.345), (0.083, 1.318), (0.087, 1.292), (0.091, 1.267), (0.094, 1.243), (0.097, 1.222)]
BODY_RADII = {"c": (0.009, 0.0092, 0.012), "t0": (0.012, 0.0130, 0.015), "t1": (0.016, 0.0130, 0.019), "l": (0.019, 0.0172, 0.025)}


SECTIONS = {
    "pelvis": (0.045, 0.009, "Sacrum, solid ellipse 90 x 18 mm", "Kreuzbein, Vollellipse 90 x 18 mm"),
    "lumbar": (0.024, 0.018, "Lumbar vertebral body, solid ellipse 48 x 36 mm", "Lendenwirbelkörper, Vollellipse 48 x 36 mm"),
    "torso": (0.016, 0.014, "Thoracic vertebral body, solid ellipse 32 x 28 mm", "Brustwirbelkörper, Vollellipse 32 x 28 mm"),
    "neck": (0.012, 0.010, "Cervical vertebral body, solid ellipse 24 x 20 mm", "Halswirbelkörper, Vollellipse 24 x 20 mm"),
    "arm": (0.0105, 0.0105, "Humeral shaft, solid circle d = 21 mm", "Oberarmschaft, Vollkreis d = 21 mm"),
    "forearm": (0.008, 0.016, "Radius and ulna side by side, solid ellipse 16 x 32 mm", "Speiche und Elle nebeneinander, Vollellipse 16 x 32 mm"),
    "hand": (0.006, 0.034, "Metacarpus, solid ellipse 12 x 68 mm", "Mittelhand, Vollellipse 12 x 68 mm"),
    "thigh": (0.0135, 0.0135, "Femoral shaft, solid circle d = 27 mm", "Oberschenkelschaft, Vollkreis d = 27 mm"),
    "shank": (0.012, 0.012, "Tibial shaft, solid circle d = 24 mm", "Schienbeinschaft, Vollkreis d = 24 mm"),
    "foot": (0.028, 0.012, "Tarsus and metatarsus, solid ellipse 56 x 24 mm", "Fußwurzel und Mittelfuß, Vollellipse 56 x 24 mm"),
}
SUPPORTS = ("heel-l", "ball-l", "heel-r", "ball-r")


def declare(asset):
    """🏷️ The stress sections of the load-bearing bones (the ellipse a member loft shows at one station) and the four foot supports."""
    for bone in asset["rig"]["bones"]:
        key = bone["id"][:-2] if bone["id"][-2:] in ("-l", "-r") else bone["id"]
        if key in SECTIONS:
            rx, ry, en, de = SECTIONS[key]
            area = math.pi * rx * ry
            bone["section"] = {"areaM2": float(f"{area:.6g}"), "secondMomentM4": float(f"{area * ry * ry / 4:.6g}"), "extremeFibreM": ry, "material": BONE, "title": {"en": en, "de": de}}
    asset["structure"]["supports"] = list(SUPPORTS)

def tangent(points, i):
    a, b = points[max(0, i - 1)], points[min(len(points) - 1, i + 1)]
    n = math.hypot(b[0] - a[0], b[1] - a[1])
    return ((b[0] - a[0]) / n, (b[1] - a[1]) / n)


def member(k, sid, bone, solid_kind, *args, **kw):
    return getattr(k, solid_kind)(sid, bone, BONE, T_BONE, *args, member=kw.pop("of", bone), part="member", **kw)


def rod(k, sid, bone, side, rows, n=1, start=0.0, end=0.0, steps=2):
    """🦯 A member along its bone: sections normal to the bone axis from rows (u, ahead, behind, medial, lateral)."""
    return member(k, sid, bone, "loft", k.limb(bone, side, rows), n=n, start=start or None, end=end or None, steps=steps)


def vertebrae(k, prefix, bone, centres, radii_of, process_m, slope):
    """🧱 One vertebra per stated centre: the body with its disc as a squat ellipsoid across the curve, the spinous process as a capsule from the arch backwards (sloping down by `slope`), the transverse processes as one capsule across."""
    out = []
    for i, (x, y) in enumerate(centres):
        tx, ty = tangent(centres, i)
        tilt = math.degrees(math.atan2(ty, tx)) - 90
        rx, ry, rz = radii_of(i)
        n = i + 1
        out.append(member(k, f"{prefix}{n}", bone, "ellipsoid", [x, y, 0], [rx, ry, rz], tilt=tilt))
        bx, by = -ty, tx
        back = [x - bx * (rx + 0.004), y - by * (rx + 0.004), 0]
        tip = [x - bx * (rx + process_m) - tx * slope, y - by * (rx + process_m) - ty * slope, 0]
        out.append(member(k, f"{prefix}{n}-spinous", bone, "capsule", back, tip, 0.0045, 0.004, role="detail"))
        out.append(member(k, f"{prefix}{n}-transverse", bone, "capsule", [back[0], back[1], -rz - 0.012], [back[0], back[1], rz + 0.012], 0.004, role="detail"))
    return out


def thoracic_radii(i):
    a, b, t = BODY_RADII["t0"], BODY_RADII["t1"], i / 11
    return tuple(a[j] + (b[j] - a[j]) * t for j in range(3))


def spine50(k):
    out = vertebrae(k, "c", "neck", CERVICAL, lambda i: BODY_RADII["c"], 0.026, 0.004)
    out += vertebrae(k, "t", "torso", THORACIC, thoracic_radii, 0.036, 0.018)
    out += vertebrae(k, "l", "lumbar", LUMBAR, lambda i: BODY_RADII["l"], 0.040, 0.002)
    return out


CERVICAL_ROD = [(-0.004, 0.010, 0.010, 0.012, 0.012), (0.050, 0.013, 0.007, 0.012, 0.012), (0.104, 0.010, 0.010, 0.012, 0.012)]
THORACIC_ROD = [(0.000, 0.006, 0.022, 0.016, 0.016), (0.070, -0.018, 0.046, 0.016, 0.016), (0.140, -0.031, 0.059, 0.016, 0.016), (0.210, -0.022, 0.050, 0.016, 0.016), (0.297, 0.008, 0.020, 0.016, 0.016)]
LUMBAR_ROD = [(-0.005, 0.018, 0.018, 0.024, 0.024), (0.045, 0.022, 0.014, 0.024, 0.024), (0.090, 0.026, 0.010, 0.024, 0.024), (0.130, 0.021, 0.015, 0.024, 0.024), (0.172, 0.016, 0.020, 0.024, 0.024)]
SACRUM_ROD = [(0.090, -0.030, 0.048, 0.045, 0.045), (0.055, -0.050, 0.068, 0.040, 0.040), (0.020, -0.066, 0.084, 0.028, 0.028), (-0.010, -0.076, 0.094, 0.014, 0.014), (-0.032, -0.074, 0.092, 0.009, 0.009)]


def spine_rods(k, n):
    """🪵 The spine as one column per region along its curve, each showing the declared vertebral section."""
    out = [rod(k, "cervical-spine", "neck", 1, CERVICAL_ROD, n=n, start=0.004, end=0.004)]
    out.append(rod(k, "thoracic-spine", "torso", 1, THORACIC_ROD, n=n, start=0.004, end=0.004))
    out.append(rod(k, "lumbar-spine", "lumbar", 1, LUMBAR_ROD, n=n, start=0.004, end=0.004))
    out.append(rod(k, "sacrum", "pelvis", 1, SACRUM_ROD, n=n, end=0.004))
    return out


def sacrum(k, n=2):
    points = [[p[0], p[1], 0] for p in SACRUM]
    return member(k, "sacrum", "pelvis", "loft", lib.smooth(tube(points, [(p[2], p[3]) for p in SACRUM], 2.4, up=(1, 0, 0)), n), end=0.004)


def hip_bone(k, side, s, n=2, least=0.0055):
    ey = lib.unit([0.85, 0, side * 0.52])
    ex = lib.unit([-side * 0.52, 0, 0.85])
    wing = [(1.035, -0.020, 0.095, 0.055, least), (1.000, -0.015, 0.090, 0.078, least), (0.960, -0.010, 0.089, 0.058, max(least, 0.0065)), (0.925, 0.000, 0.087, 0.040, 0.012), (0.898, 0.002, 0.086, 0.032, 0.018)]
    stations = [station([x, y, side * z], ey, ex, half, thick, 2.6) for y, x, z, half, thick in wing]
    out = [member(k, f"ilium-{s}", "pelvis", "loft", stations, n=n, start=0.005)]
    out.append(member(k, f"ischium-{s}", "pelvis", "capsule", [-0.008, 0.892, side * 0.076], [-0.045, 0.815, side * 0.058], 0.014, 0.016))
    out.append(member(k, f"pubic-ramus-{s}", "pelvis", "capsule", [0.020, 0.895, side * 0.070], [0.062, 0.864, side * 0.006], 0.009))
    out.append(member(k, f"ischial-ramus-{s}", "pelvis", "capsule", [0.058, 0.852, side * 0.008], [-0.036, 0.815, side * 0.050], 0.0075))
    return out


def skull(k, n=2, fine=True):
    out = [member(k, "cranium", "head", "ellipsoid", [HX - 0.002, HY + 0.068, 0], [0.086, 0.076, 0.068])]
    face = [(0.052, 0.092, 0.030, 0.052), (0.030, 0.086, 0.025, 0.056), (0.008, 0.090, 0.025, 0.054), (-0.012, 0.088, 0.030, 0.034)]
    out.append(member(k, "facial-skeleton", "head", "loft", [H(HY + r[0], HX + r[1], HX + r[2], r[3], 0, 2.4) for r in face], n=n, end=0.004))
    jaw = [[0.010, 0.016, 0.052], [0.005, -0.040, 0.047], [0.045, -0.052, 0.040], [0.074, -0.060, 0.020], [0.084, -0.062, 0.0], [0.074, -0.060, -0.020], [0.045, -0.052, -0.040], [0.005, -0.040, -0.047], [0.010, 0.016, -0.052]]
    if fine:
        path = polyline([[HX + p[0], HY + p[1], p[2]] for p in jaw[1:-1]], n)
        out.append(member(k, "mandible", "head", "loft", tube(path, (0.006, 0.013)), start=0.004, end=0.004))
        for side, s in SIDES:
            out.append(member(k, f"mandible-ramus-{s}", "head", "capsule", [HX + 0.005, HY - 0.040, side * 0.047], [HX + 0.010, HY + 0.016, side * 0.052], 0.008, 0.0065))
            out.append(member(k, f"zygomatic-arch-{s}", "head", "capsule", [HX + 0.072, HY + 0.012, side * 0.054], [HX + 0.014, HY + 0.014, side * 0.060], 0.005))
        out.append(member(k, "nasal-bone", "head", "ellipsoid", [HX + 0.096, HY + 0.036, 0], [0.008, 0.012, 0.008], role="detail", tilt=-25))
    else:
        path = polyline([[HX + p[0], HY + p[1], p[2]] for p in jaw], n)
        out.append(member(k, "mandible", "head", "loft", tube(path, (0.0075, 0.012)), start=0.004, end=0.004, steps=2))
    return out


def rib_path(i, side, to_sternum=False):
    x, y = THORACIC[i]
    fx, fy, fz = RIB_FRONT[i]
    a = RIB_BREADTH[i]
    scale = a / 0.134
    y_lat = y - 0.55 * (y - fy)
    points = [[x - 0.006, y + 0.004, side * 0.011], [x - 0.012 - 0.030 * scale, y - 0.006, side * 0.062 * scale]]
    if i == 11:
        return points + [[fx, fy, side * fz]]
    points.append([-0.012, y_lat, side * a])
    if i < 10:
        points.append([(fx - 0.012) / 2 + 0.018 * scale, (y_lat + fy) / 2 - 0.004, side * ((a + fz) / 2 + 0.018 * scale)])
    points.append([fx, fy, side * fz])
    if to_sternum and i < 10:
        if i >= 7:
            points.append([0.086, 1.196, side * 0.050])
        sx, sy = STERNAL_END[min(i, 6)]
        points.append([sx, sy, side * 0.006])
    return points


def sternum(k, least=0.0):
    points = [[p[0], p[1], 0] for p in STERNUM]
    return member(k, "sternum", "torso", "loft", lib.smooth(tube(points, [(p[2], max(p[3], least)) for p in STERNUM], 2.6, up=(1, 0, 0)), 2), start=0.004, end=0.004)


def rib_cage(k, n=4):
    out = []
    for side, s in SIDES:
        ends = []
        for i in range(12):
            path = polyline(rib_path(i, side), n)
            out.append(member(k, f"rib-{s}-{i + 1}", "torso", "loft", tube(path, (0.0045, 0.006)), start=0.003, end=0.003, steps=2))
            fx, fy, fz = RIB_FRONT[i]
            if i < 7:
                sx, sy = STERNAL_END[i]
                target = [sx, sy, side * 0.014]
            elif i < 10:
                target = ends[-1]
            else:
                continue
            start = [fx, fy, side * fz]
            out.append(k.capsule(f"cartilage-{s}-{i + 1}", "torso", CARTILAGE, 0.78, start, target, 0.0045, member="torso", part="member"))
            ends.append([(start[j] * 0.35 + target[j] * 0.65) for j in range(3)])
    return out + [sternum(k)]


def rib_hoops(k, which, n):
    """➰ Ribs with their cartilage as one slender hoop from the vertebra to the sternum."""
    out = []
    for side, s in SIDES:
        for i in which:
            path = polyline(rib_path(i, side, True), n)
            out.append(member(k, f"rib-{s}-{i + 1}", "torso", "loft", tube(path, (0.005, 0.0065)), start=0.003, end=0.003, steps=1, slender=True))
    return out


def girdle(k, side, s, n=3, least=0.0045):
    bone = f"clavicle-{s}"
    clavicle = polyline([[0.057, 1.392, side * 0.022], [0.068, 1.400, side * 0.065], [0.052, 1.412, side * 0.115], [0.020, 1.418, side * 0.150], [0.006, 1.421, side * 0.168]], n)
    r = max(0.0065, least)
    out = [member(k, f"clavicle-{s}", bone, "loft", tube(clavicle, (r, max(0.006, least))), start=0.004, end=0.004, steps=2)]
    ex = lib.unit([0.060, 0, side * 0.070])
    ey = lib.cross([0, 1, 0], ex)
    blade = [(1.425, -0.086, 0.086, 0.020), (1.400, -0.068, 0.103, 0.046), (1.360, -0.086, 0.097, 0.032), (1.320, -0.094, 0.090, 0.022), (1.282, -0.098, 0.086, 0.010)]
    out.append(member(k, f"scapula-{s}", bone, "loft", [station([x, y, side * z], ex, ey, half, least, 2.5) for y, x, z, half in blade], n=2, start=0.004, end=0.004, steps=2))
    out.append(member(k, f"scapular-spine-{s}", bone, "capsule", [-0.096, 1.395, side * 0.072], [-0.022, 1.421, side * 0.160], max(0.006, least)))
    out.append(member(k, f"acromion-{s}", bone, "capsule", [-0.030, 1.422, side * 0.158], [0.006, 1.421, side * 0.173], max(0.007, least)))
    out.append(member(k, f"scapular-neck-{s}", bone, "capsule", [-0.042, 1.400, side * 0.130], [-0.008, 1.399, side * 0.132], 0.009))
    if least < 0.007:
        out.append(member(k, f"coracoid-{s}", bone, "capsule", [-0.012, 1.411, side * 0.126], [0.030, 1.398, side * 0.118], 0.006))
        out.append(member(k, f"glenoid-{s}", bone, "ellipsoid", [-0.004, 1.399, side * 0.131], [0.014, 0.019, 0.006]))
    return out


def arm_bones(k, side, s, least=0.0):
    z = side * SHOULDER_Z
    out = [member(k, f"humeral-head-{s}", f"arm-{s}", "ellipsoid", [0, 1.399, z - side * 0.002], [0.022, 0.022, 0.022])]
    out.append(member(k, f"greater-tubercle-{s}", f"arm-{s}", "ellipsoid", [0.002, 1.392, z + side * 0.018], [0.013, 0.016, 0.011]))
    out.append(member(k, f"humerus-{s}", f"arm-{s}", "capsule", [0, 1.380, z + side * 0.004], [0, 1.105, z], 0.0105, 0.0095))
    out.append(member(k, f"humeral-condyle-{s}", f"arm-{s}", "capsule", [0.002, 1.090, z - side * 0.030], [0.002, 1.090, z + side * 0.026], 0.012, 0.010))
    out.append(member(k, f"ulna-{s}", f"forearm-{s}", "capsule", [-0.014, 1.090, z], [-0.016, 0.846, z], 0.0078, max(0.0055, least)))
    out.append(member(k, f"olecranon-{s}", f"forearm-{s}", "ellipsoid", [-0.021, 1.097, z], [0.012, 0.017, 0.011]))
    out.append(member(k, f"radius-{s}", f"forearm-{s}", "capsule", [0.012, 1.076, z], [0.014, 0.848, z], max(0.0062, least), 0.0092))
    if least < 0.007:
        out.append(member(k, f"radial-head-{s}", f"forearm-{s}", "ellipsoid", [0.012, 1.078, z], [0.011, 0.007, 0.011]))
    return out


def hand_bones(k, side, s):
    f = k.f
    wrist = f.rest[f"hand-{s}"]
    out = [member(k, f"carpus-{s}", f"hand-{s}", "ellipsoid", [wrist[0], wrist[1] - 0.010, wrist[2]], [0.024, 0.014, 0.010])]
    for n, name in ((2, "index"), (3, "middle"), (4, "ring"), (5, "little")):
        knuckle = f.world(f"{name}-proximal-{s}", [0, 0, 0])
        base = [wrist[0] + (knuckle[0] - wrist[0]) * 0.55, wrist[1] - 0.020, wrist[2]]
        out.append(member(k, f"metacarpal-{s}-{n}", f"hand-{s}", "capsule", base, knuckle, 0.0046, 0.0052))
    for name in FINGERS:
        bone = f"{name}-{s}"
        length = f.bones[bone]["lengthM"]
        out.append(member(k, f"shaft-{bone}", bone, "capsule", f.world(bone, [0.0015, 0, 0]), f.world(bone, [length - 0.0015, 0, 0]), 0.0042 if "thumb" in name else 0.0039, 0.00375))
        out.append(k.ellipsoid(f"articulation-{bone}", bone, BONE, 0.83, f.world(bone, [0, 0, 0]), [0.0048, 0.0048, 0.0048], role="detail", member=bone, part="joint"))
    return out


HAND_ROD = [(-0.004, 0.022, 0.022, 0.006, 0.006), (0.030, 0.034, 0.034, 0.006, 0.006), (0.095, 0.036, 0.034, 0.006, 0.006)]
METACARPUS_100 = [(0.000, 0.022, 0.022, 0.0075, 0.0075), (0.030, 0.034, 0.034, 0.0075, 0.0075), (0.095, 0.036, 0.034, 0.0075, 0.0075)]


def leg_bones(k, side, s, n=3, least=0.0):
    z = side * HIP_Z
    out = [member(k, f"femoral-head-{s}", f"thigh-{s}", "ellipsoid", [0, 0.903, z], [0.023, 0.023, 0.023])]
    out.append(member(k, f"femoral-neck-{s}", f"thigh-{s}", "capsule", [0, 0.903, z], [-0.004, 0.882, z + side * 0.047], 0.015, 0.016))
    out.append(member(k, f"greater-trochanter-{s}", f"thigh-{s}", "ellipsoid", [-0.008, 0.888, z + side * 0.063], [0.018, 0.024, 0.014]))
    shaft = polyline([[-0.004, 0.868, z + side * 0.055], [0.011, 0.700, z + side * 0.030], [0.004, 0.512, z + side * 0.004]], n)
    out.append(member(k, f"femur-{s}", f"thigh-{s}", "loft", tube(shaft, (0.0135, 0.0135)), start=0.006, end=0.006, steps=2))
    out.append(member(k, f"femoral-condyle-medial-{s}", f"thigh-{s}", "ellipsoid", [-0.006, 0.493, z - side * 0.024], [0.031, 0.023, 0.020]))
    out.append(member(k, f"femoral-condyle-lateral-{s}", f"thigh-{s}", "ellipsoid", [-0.006, 0.493, z + side * 0.024], [0.030, 0.022, 0.019]))
    out.append(member(k, f"patella-{s}", f"thigh-{s}", "ellipsoid", [0.027, 0.499, z], [0.011, 0.021, 0.022]))
    out.append(member(k, f"tibial-plateau-{s}", f"shank-{s}", "ellipsoid", [-0.002, 0.466, z], [0.027, 0.015, 0.038]))
    shin = polyline([[0.004, 0.452, z], [0.011, 0.300, z - side * 0.004], [0.003, 0.108, z - side * 0.006]], n)
    out.append(member(k, f"tibia-{s}", f"shank-{s}", "loft", tube(shin, [(0.0135 - 0.003 * i / (len(shin) - 1), 0.0135 - 0.003 * i / (len(shin) - 1)) for i in range(len(shin))]), start=0.005, end=0.005, steps=2))
    out.append(member(k, f"tibial-pilon-{s}", f"shank-{s}", "ellipsoid", [0.001, 0.100, z - side * 0.008], [0.020, 0.016, 0.022]))
    out.append(member(k, f"medial-malleolus-{s}", f"shank-{s}", "ellipsoid", [0.002, 0.084, z - side * 0.028], [0.011, 0.014, 0.008]))
    out.append(member(k, f"fibula-{s}", f"shank-{s}", "capsule", [-0.016, 0.462, z + side * 0.028], [-0.012, 0.086, z + side * 0.032], max(0.0065, least), max(0.006, least)))
    out.append(member(k, f"lateral-malleolus-{s}", f"shank-{s}", "ellipsoid", [-0.012, 0.078, z + side * 0.033], [0.011, 0.017, 0.009]))
    return out


def tarsal_bones(k, side, s):
    z = side * HIP_Z
    out = [member(k, f"talus-{s}", f"foot-{s}", "ellipsoid", [0.006, 0.072, z], [0.028, 0.015, 0.020])]
    out.append(member(k, f"calcaneus-{s}", f"foot-{s}", "ellipsoid", [-0.028, 0.046, z + side * 0.004], [0.036, 0.021, 0.016], tilt=14))
    out.append(member(k, f"tarsus-{s}", f"foot-{s}", "loft", [X(0.026, 0.038, 0.068, 0.025, z, 2.4), X(0.064, 0.034, 0.058, 0.031, z, 2.4)], start=0.004, end=0.004, steps=2))
    return out


def foot_bones(k, side, s):
    z = side * HIP_Z
    out = tarsal_bones(k, side, s)
    rays = [(-0.021, -0.027, 0.150, 0.0072, 0.200, 0.0068), (-0.008, -0.011, 0.153, 0.0050, 0.198, 0.0042), (0.004, 0.003, 0.149, 0.0048, 0.192, 0.0040), (0.015, 0.017, 0.142, 0.0048, 0.183, 0.0040), (0.025, 0.030, 0.132, 0.0052, 0.170, 0.0040)]
    for n, (z0, z1, x1, r, tip, rt) in enumerate(rays, 1):
        out.append(member(k, f"metatarsal-{s}-{n}", f"foot-{s}", "capsule", [0.064, 0.046, z + side * z0], [x1, 0.020 + r, z + side * z1], r))
        out.append(member(k, f"toe-bones-{s}-{n}", f"toe-{s}", "capsule", [x1 + 0.004, 0.020 + r, z + side * z1], [tip, 0.022 + rt, z + side * z1 * 1.02], max(r - 0.0006, 0.004), rt))
    return out


def supports(k, scale=1.0, least=0.0):
    """🟩 One pad per foot support under the heel bone and under the metatarsal heads, resting on the ground."""
    out = []
    for side, s in SIDES:
        z = side * HIP_Z
        out.append(k.ellipsoid(f"support-heel-{s}", f"foot-{s}", "canvas-sneaker", 0.3, [-0.032, max(0.015, least) * scale, z], [0.030 * scale, max(0.015, least) * scale, 0.026 * scale], role="detail", member=f"foot-{s}", part="support", anchor=f"heel-{s}"))
        out.append(k.ellipsoid(f"support-ball-{s}", f"foot-{s}", "canvas-sneaker", 0.3, [0.146, max(0.013, least) * scale, z], [0.024 * scale, max(0.013, least) * scale, 0.042 * min(scale, 1.2)], role="detail", member=f"foot-{s}", part="support", anchor=f"ball-{s}"))
    return out


def joint(k, sid, bone, centre, r, lateral=0.0):
    if lateral:
        return k.capsule(sid, bone, BONE, T_JOINT, [centre[0], centre[1], centre[2] - lateral], [centre[0], centre[1], centre[2] + lateral], r, member=bone, part="joint")
    return k.ellipsoid(sid, bone, BONE, T_JOINT, centre, [r, r, r], member=bone, part="joint")


def joints(k, least=0.0):
    f = k.f
    o = lambda bone: list(f.rest[bone][:3])
    out = [joint(k, "joint-lumbosacral", "lumbar", o("lumbar"), 0.013), joint(k, "joint-thoracolumbar", "torso", o("torso"), 0.012), joint(k, "joint-cervicothoracic", "neck", o("neck"), 0.011), joint(k, "joint-atlanto-occipital", "head", o("head"), 0.010)]
    for side, s in SIDES:
        out.append(k.ellipsoid(f"joint-sacroiliac-{s}", "pelvis", BONE, T_JOINT, [-0.058, 0.975, side * 0.058], [0.020, 0.026, 0.018], member="pelvis", part="joint"))
        out.append(joint(k, f"joint-sternoclavicular-{s}", f"clavicle-{s}", o(f"clavicle-{s}"), 0.009))
        out.append(joint(k, f"joint-shoulder-{s}", f"arm-{s}", o(f"arm-{s}"), 0.013))
        out.append(joint(k, f"joint-elbow-{s}", f"forearm-{s}", o(f"forearm-{s}"), 0.0095, 0.026))
        out.append(joint(k, f"joint-wrist-{s}", f"hand-{s}", o(f"hand-{s}"), 0.009))
        out.append(joint(k, f"joint-hip-{s}", f"thigh-{s}", o(f"thigh-{s}"), 0.014))
        out.append(joint(k, f"joint-knee-{s}", f"shank-{s}", o(f"shank-{s}"), 0.0105, 0.036))
        out.append(joint(k, f"joint-ankle-{s}", f"foot-{s}", o(f"foot-{s}"), 0.0095, 0.026))
        out.append(joint(k, f"joint-toes-{s}", f"toe-{s}", [o(f"toe-{s}")[0], 0.028, o(f"toe-{s}")[2]], max(0.006, least), 0.032))
    return out


def masses(k, asset, least_kg, scale=1.0):
    out = []
    for bone in asset["rig"]["bones"]:
        mass = bone.get("mass")
        if mass is None or mass["kg"] < least_kg:
            continue
        r = 0.012 * mass["kg"] ** (1 / 3) * scale
        centre = k.f.world(bone["id"], [mass["comX"], mass.get("comY", 0), 0])
        out.append(k.ellipsoid(f"mass-{bone['id']}", bone["id"], "skin", T_MASS, centre, [r, r, r], role="detail", member=bone["id"], part="mass"))
    return out


def structure50(k, asset):
    out = skull(k) + spine50(k) + [sacrum(k)] + rib_cage(k)
    for side, s in SIDES:
        out += hip_bone(k, side, s) + girdle(k, side, s) + arm_bones(k, side, s) + hand_bones(k, side, s) + leg_bones(k, side, s) + foot_bones(k, side, s)
    return out + joints(k) + supports(k) + masses(k, asset, 0.5)


def crest(k, sid, bone, points, r):
    return member(k, sid, bone, "loft", tube(polyline([[x, y, 0] for x, y in points], 2), (r, 2 * r), up=(1, 0, 0)), start=0.004, end=0.004, steps=2)


def structure100(k, asset):
    """🦴 1:100, nothing under 15 mm except the slender rib hoops: the vertebrae merge into one column per region with the line of the spinous processes behind, rib and cartilage into one hoop, the metacarpals and the metatarsals into one plate each; the phalanges are below the feature."""
    least = 0.0075
    out = skull(k, n=2, fine=False) + spine_rods(k, 2)
    out.append(crest(k, "thoracic-processes", "torso", [(THORACIC[i][0] - 0.028, THORACIC[i][1] - 0.012) for i in (0, 3, 6, 9, 11)], 0.008))
    out.append(crest(k, "lumbar-processes", "lumbar", [(LUMBAR[i][0] - 0.032, LUMBAR[i][1]) for i in (0, 2, 4)], 0.008))
    out += rib_hoops(k, range(1, 10), 3) + [sternum(k, least)]
    for side, s in SIDES:
        z = side * HIP_Z
        out += hip_bone(k, side, s, n=1, least=least) + girdle(k, side, s, n=2, least=least) + arm_bones(k, side, s, least) + leg_bones(k, side, s, n=2, least=least) + tarsal_bones(k, side, s)
        wrist = k.f.rest[f"hand-{s}"]
        out.append(member(k, f"carpus-{s}", f"hand-{s}", "ellipsoid", [wrist[0], wrist[1] - 0.010, wrist[2]], [0.024, 0.014, 0.010]))
        out.append(rod(k, f"metacarpus-{s}", f"hand-{s}", side, METACARPUS_100, end=0.004))
        out.append(member(k, f"metatarsus-{s}", f"foot-{s}", "loft", [X(0.060, 0.036, 0.056, 0.030, z, 2.6), X(0.110, 0.026, 0.044, 0.038, z, 2.6), X(0.150, 0.020, 0.036, 0.042, z, 2.6)], end=0.004, steps=2))
        out.append(member(k, f"toe-bones-{s}", f"toe-{s}", "loft", [X(0.150, 0.020, 0.036, 0.042, z, 2.6), X(0.196, 0.020, 0.035, 0.034, z, 2.6)], end=0.006, steps=2))
    return out + joints(k, least) + supports(k) + masses(k, asset, 0.7, 1.15)


SKULL_COARSE = [(0.130, 0.050, -0.060, 0.045), (0.070, 0.088, -0.086, 0.068), (0.010, 0.090, -0.060, 0.058), (-0.060, 0.086, 0.020, 0.030)]
FEMUR_ROD = [(-0.004, 0.0135, 0.0135, 0.0135, 0.0135), (0.030, 0.0135, 0.0135, -0.030, 0.057), (0.200, 0.0235, 0.0035, -0.012, 0.039), (0.380, 0.0170, 0.0100, 0.011, 0.016), (0.41325, 0.0135, 0.0135, 0.0135, 0.0135)]
FOOT_ROD = [(-0.010, 0.012, 0.012, 0.028, 0.028), (0.080, 0.012, 0.012, 0.028, 0.028), (0.17251, 0.012, 0.012, 0.028, 0.028)]


def structure_coarse(k, asset, d):
    """🪵 1:200 and 1:500: one member per load-bearing bone at the real thickness of its declared section (a member loft shows the section at a station normal to its bone), the skull as one form, slender rib hoops and one slender girdle line per side."""
    fine = d == 200
    rows = SKULL_COARSE if fine else [SKULL_COARSE[0], SKULL_COARSE[1], SKULL_COARSE[3]]
    out = [member(k, "skull", "head", "loft", [H(HY + r[0], HX + r[1], HX + r[2], r[3]) for r in rows], n=2 if fine else 1, start=0.016, end=0.004, steps=2)]
    out += spine_rods(k, 1 if fine else 1)
    out += rib_hoops(k, (0, 2, 4, 6, 8) if fine else (1, 5, 8), 3 if fine else 2)
    for side, s in SIDES:
        z = side * HIP_Z
        wing = polyline([[-0.060, 0.985, side * 0.048], [-0.030, 1.034, side * 0.118], [0.045, 0.975, side * 0.125], [0.002, 0.903, side * 0.088], [0.060, 0.862, side * 0.008]], 2 if fine else 1)
        out.append(member(k, f"hip-bone-{s}", "pelvis", "loft", tube(wing, (0.016, 0.009)), start=0.004, end=0.004, steps=1))
        line = polyline([[0.057, 1.392, side * 0.022], [0.060, 1.404, side * 0.090], [0.004, 1.420, side * 0.165], [-0.006, 1.400, side * 0.136], [-0.070, 1.392, side * 0.100], [-0.096, 1.300, side * 0.086]], 2 if fine else 1)
        out.append(member(k, f"shoulder-girdle-{s}", f"clavicle-{s}", "loft", tube(line, (0.0075, 0.0075)), start=0.004, end=0.004, steps=1, slender=True))
        out.append(rod(k, f"humerus-{s}", f"arm-{s}", side, [(-0.010, 0.0105, 0.0105, 0.0105, 0.0105), (0.3135, 0.0105, 0.0105, 0.0105, 0.0105)], start=0.008, end=0.008, steps=1))
        out.append(rod(k, f"forearm-bones-{s}", f"forearm-{s}", side, [(-0.004, 0.016, 0.016, 0.008, 0.008), (0.247, 0.016, 0.016, 0.008, 0.008)], start=0.006, end=0.006, steps=1))
        out.append(rod(k, f"hand-bones-{s}", f"hand-{s}", side, HAND_ROD, end=0.004, steps=1))
        out.append(rod(k, f"femur-{s}", f"thigh-{s}", side, FEMUR_ROD, start=0.010, end=0.010, steps=1))
        out.append(rod(k, f"tibia-{s}", f"shank-{s}", side, [(-0.008, 0.012, 0.012, 0.012, 0.012), (0.40375, 0.012, 0.012, 0.012, 0.012)], start=0.008, end=0.008, steps=1))
        out.append(rod(k, f"foot-bones-{s}", f"foot-{s}", side, FOOT_ROD, start=0.006, end=0.006, steps=1))
        out.append(member(k, f"calcaneus-{s}", f"foot-{s}", "loft", [X(-0.056, 0.018, 0.050, 0.014, z), X(-0.030, 0.014, 0.064, 0.018, z), X(0.004, 0.030, 0.074, 0.020, z)], start=0.004, steps=1))
    return out + (supports(k, 1.0, 0.0152) if fine else supports(k, 2.6, 0.0145)) + masses(k, asset, 2.0 if fine else 7.0, 1.3 if fine else 1.7)
