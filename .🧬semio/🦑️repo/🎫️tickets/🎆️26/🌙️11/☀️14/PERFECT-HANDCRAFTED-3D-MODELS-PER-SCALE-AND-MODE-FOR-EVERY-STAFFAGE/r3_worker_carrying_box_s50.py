"""🧱 The three lists of the worker carrying a box at 1:50: the body (`r3_worker_carrying_box_body50`), the thermal model as nested layer shells and outlet patches, and the anatomical skeleton with the carton as the load path."""
import math

import r3_worker_carrying_box_body50 as b50
import r3_worker_carrying_box_kit as k
import r3_worker_carrying_box_texts as texts

BONE = "skin"
TOP = b50.TOP
HZ = b50.HIP_Z
SPINE = [(-0.045, 1.042), (-0.036, 1.072), (-0.031, 1.104), (-0.033, 1.137), (-0.040, 1.170), (-0.048, 1.202), (-0.056, 1.232), (-0.063, 1.261), (-0.068, 1.289), (-0.071, 1.316), (-0.072, 1.342), (-0.071, 1.367), (-0.068, 1.391), (-0.064, 1.414), (-0.058, 1.436), (-0.051, 1.457), (-0.044, 1.477), (-0.038, 1.496), (-0.032, 1.514), (-0.030, 1.528)]
NECK = [(-0.030, 1.528), (-0.027, 1.546), (-0.022, 1.563), (-0.018, 1.580), (-0.015, 1.597), (-0.014, 1.613), (-0.014, 1.628), (-0.015, 1.642)]
RIBS = [(18, 0.058, 0.030, 0.030), (17, 0.084, 0.052, 0.045), (16, 0.104, 0.068, 0.058), (15, 0.118, 0.080, 0.070), (14, 0.128, 0.090, 0.082), (13, 0.134, 0.097, 0.094), (12, 0.138, 0.102, 0.106), (11, 0.140, 0.100, 0.112), (10, 0.139, 0.094, 0.112), (9, 0.134, 0.082, 0.104), (8, 0.120, 0.020, 0.060), (7, 0.100, -0.020, 0.036)]


def beads(points, r_body, r_disc):
    """🦴 Loft rows of a spine along `points` (x, y): a vertebral body at every point and a narrower disc between two."""
    rows = []
    for i, (x, y) in enumerate(points):
        grow = 1 - 0.22 * i / (len(points) - 1)
        rows.append((x, y, 0, r_body * grow * 1.15, r_body * grow))
        if i + 1 < len(points):
            nx, ny = points[i + 1]
            rows.append(((x + nx) / 2, (y + ny) / 2, 0, r_disc * grow * 1.15, r_disc * grow))
    return rows


def rib(S, index, level, half, front, drop, mirror):
    """🎗 One rib as a free loft: from its vertebra back and out to the rib angle, round the flank and forwards to its end `front` metres ahead of the hip axis, sinking by `drop`."""
    sx, sy = SPINE[level]
    rows = [(sx - 0.004, sy + 0.004, 0.018, 0.006, 0.006), (sx - 0.026, sy + 0.002, 0.046, 0.006, 0.006)]
    back = sx - 0.034
    xc, depth = (front + back) / 2, (front - back) / 2
    steps = 9 if index < 10 else 5
    end = 0.30 if index < 7 else (0.55 if index < 10 else 1.75)
    start = math.pi - 0.50
    for i in range(steps + 1):
        a = start + (end - start) * i / steps
        rows.append((xc + depth * math.cos(a), sy - drop * (1 + math.cos(a)) / 2, half * math.sin(a) ** 0.8, 0.006, 0.006))
    side = "r" if mirror else "l"
    return k.st(k.free_loft(S, "rib-%s-%d" % (side, index + 1), "torso", BONE, 0.85, rows, mirror=mirror, sub=1), "torso")


def skeleton(S, body):
    out = []
    add = lambda solid, member, part="member": out.append(k.st(solid, member, part))
    add(k.ellipsoid(S, "cranium", "head", BONE, 0.85, (-0.012, TOP - 0.078, 0), (0.090, 0.070, 0.070)), "head")
    add(k.loft(S, "face-bones", "head", BONE, 0.85, [(TOP - 0.098, 0.050, 0, 0.056, 0.040), (TOP - 0.125, 0.056, 0, 0.060, 0.036), (TOP - 0.150, 0.060, 0, 0.056, 0.036), (TOP - 0.172, 0.062, 0, 0.040, 0.034), (TOP - 0.182, 0.062, 0, 0.032, 0.030)], e=2.2), "head")
    jaw = [(-0.010, TOP - 0.135, 0.058, 0.008, 0.014), (-0.004, TOP - 0.170, 0.056, 0.007, 0.012), (0.004, TOP - 0.196, 0.052, 0.007, 0.010), (0.034, TOP - 0.208, 0.042, 0.007, 0.011), (0.064, TOP - 0.216, 0.024, 0.008, 0.012), (0.076, TOP - 0.219, 0.000, 0.008, 0.013), (0.064, TOP - 0.216, -0.024, 0.008, 0.012), (0.034, TOP - 0.208, -0.042, 0.007, 0.011), (0.004, TOP - 0.196, -0.052, 0.007, 0.010), (-0.004, TOP - 0.170, -0.056, 0.007, 0.012), (-0.010, TOP - 0.135, -0.058, 0.008, 0.014)]
    add(k.free_loft(S, "mandible", "head", BONE, 0.85, jaw, sub=1), "head")
    add(k.free_loft(S, "cervical-spine", "head", BONE, 0.85, beads(NECK, 0.013, 0.009), across=k.Z), "head")
    add(k.free_loft(S, "spine", "torso", BONE, 0.85, beads(SPINE, 0.021, 0.015), across=k.Z), "torso")
    for i in range(1, len(SPINE) - 1, 2):
        x, y = SPINE[i]
        add(k.capsule(S, "spinous-%d" % (i // 2 + 1), "torso", BONE, 0.85, (x - 0.020, y, 0), (x - 0.050, y - 0.014, 0), 0.006, 0.0045), "torso")
    for index, (level, half, front, drop) in enumerate(RIBS):
        out.append(rib(S, index, level, half, front, drop, False))
        out.append(rib(S, index, level, half, front, drop, True))
    add(k.free_loft(S, "sternum", "torso", BONE, 0.85, [(0.048, 1.462, 0, 0.026, 0.006), (0.060, 1.430, 0, 0.022, 0.006), (0.082, 1.360, 0, 0.018, 0.0055), (0.098, 1.300, 0, 0.016, 0.005), (0.103, 1.268, 0, 0.008, 0.004)], e=2.5, across=k.Z), "torso")
    for side, mirror in (("l", False), ("r", True)):
        arch = [(0.098, 1.290, 0.018, 0.006, 0.006), (0.094, 1.245, 0.060, 0.006, 0.006), (0.078, 1.212, 0.100, 0.006, 0.006), (0.050, 1.195, 0.126, 0.006, 0.006)]
        add(k.free_loft(S, "costal-arch-" + side, "torso", BONE, 0.85, arch, mirror=mirror, sub=1), "torso")
        add(k.capsule(S, "clavicle-" + side, "torso", BONE, 0.85, (0.046, 1.458, 0.022), (-0.022, 1.488, 0.188), 0.008, 0.007, mirror=mirror), "torso")
        add(k.ellipsoid(S, "scapula-" + side, "torso", BONE, 0.85, (-0.100, 1.395, 0.112), (0.009, 0.078, 0.052), mirror=mirror, tilt=8, yaw=-28), "torso")
        add(k.capsule(S, "scapular-spine-" + side, "torso", BONE, 0.85, (-0.106, 1.440, 0.075), (-0.030, 1.490, 0.200), 0.007, 0.009, mirror=mirror), "torso")
        add(k.ellipsoid(S, "glenoid-" + side, "torso", BONE, 0.85, (-0.022, 1.452, 0.158), (0.016, 0.022, 0.010), mirror=mirror), "torso")
        add(k.ellipsoid(S, "ilium-" + side, "pelvis", BONE, 0.85, (-0.024, 1.012, 0.112), (0.072, 0.066, 0.013), mirror=mirror, tilt=-12, yaw=38), "pelvis")
        add(k.capsule(S, "pubic-ramus-" + side, "pelvis", BONE, 0.85, (0.004, 0.948, 0.078), (0.060, 0.918, 0.010), 0.011, 0.012, mirror=mirror), "pelvis")
        add(k.capsule(S, "ischium-" + side, "pelvis", BONE, 0.85, (-0.010, 0.930, 0.082), (-0.036, 0.862, 0.060), 0.014, 0.016, mirror=mirror), "pelvis")
        add(k.capsule(S, "ischial-ramus-" + side, "pelvis", BONE, 0.85, (-0.034, 0.860, 0.058), (0.050, 0.900, 0.014), 0.008, 0.008, mirror=mirror), "pelvis")
        add(k.ellipsoid(S, "acetabulum-" + side, "pelvis", BONE, 0.85, (-0.004, 0.944, HZ + 0.004), (0.034, 0.034, 0.018), mirror=mirror), "pelvis")
        thigh, shank, foot, toe = "thigh-" + side, "shank-" + side, "foot-" + side, "toe-" + side
        add(k.ellipsoid(S, "femoral-head-" + side, thigh, BONE, 0.85, (0, 0.9406, HZ), (0.024, 0.024, 0.024), mirror=mirror), thigh)
        add(k.capsule(S, "femoral-neck-" + side, thigh, BONE, 0.85, (0, 0.9406, HZ), (-0.004, 0.918, HZ + 0.050), 0.015, 0.017, mirror=mirror), thigh)
        add(k.ellipsoid(S, "trochanter-" + side, thigh, BONE, 0.85, (-0.006, 0.915, HZ + 0.058), (0.020, 0.030, 0.017), mirror=mirror), thigh)
        femur = [(-0.004, 0.900, HZ + 0.052, 0.017, 0.017), (0.004, 0.840, HZ + 0.040, 0.0150, 0.0150), (0.014, 0.740, HZ + 0.022, 0.0140, 0.0140), (0.016, 0.650, HZ + 0.010, 0.0145, 0.0145), (0.010, 0.580, HZ + 0.003, 0.018, 0.017), (0.004, 0.548, HZ, 0.032, 0.022)]
        add(k.free_loft(S, "femur-" + side, thigh, BONE, 0.85, femur, mirror=mirror, sub=1, across=k.Z), thigh)
        add(k.ellipsoid(S, "femoral-condyles-" + side, thigh, BONE, 0.85, (0.002, 0.532, HZ), (0.034, 0.024, 0.043), mirror=mirror), thigh)
        add(k.ellipsoid(S, "patella-" + side, shank, BONE, 0.85, (0.046, 0.530, HZ), (0.011, 0.022, 0.021), mirror=mirror), shank)
        tibia = [(0.000, 0.506, HZ, 0.040, 0.027), (0.004, 0.480, HZ, 0.026, 0.022), (0.008, 0.420, HZ - 0.002, 0.0155, 0.0165), (0.006, 0.300, HZ - 0.003, 0.0135, 0.0145), (0.002, 0.180, HZ - 0.003, 0.013, 0.013), (0.000, 0.125, HZ - 0.002, 0.017, 0.016), (0.000, 0.104, HZ - 0.004, 0.024, 0.019)]
        add(k.loft(S, "tibia-" + side, shank, BONE, 0.85, [(r[1], r[0], r[2], r[3], r[4]) for r in tibia], e=2.0, sub=1, mirror=mirror), shank)
        add(k.capsule(S, "fibula-" + side, shank, BONE, 0.85, (-0.014, 0.486, HZ + 0.036), (-0.008, 0.088, HZ + 0.031), 0.0075, 0.0085, mirror=mirror), shank)
        add(k.ellipsoid(S, "talus-" + side, foot, BONE, 0.85, (0.004, 0.084, HZ), (0.028, 0.017, 0.021), mirror=mirror), foot)
        add(k.capsule(S, "calcaneus-" + side, foot, BONE, 0.85, (-0.048, 0.055, HZ - 0.002), (0.006, 0.066, HZ), 0.021, 0.016, mirror=mirror), foot)
        add(k.ellipsoid(S, "tarsus-" + side, foot, BONE, 0.85, (0.048, 0.066, HZ + 0.002), (0.028, 0.017, 0.030), mirror=mirror, tilt=-14), foot)
        for i, (dz, reach, toe_len) in enumerate(((-0.026, 0.132, 0.062), (-0.012, 0.136, 0.060), (0.002, 0.132, 0.056), (0.015, 0.125, 0.050), (0.027, 0.116, 0.042))):
            add(k.capsule(S, "metatarsal-%s-%d" % (side, i + 1), foot, BONE, 0.85, (0.066, 0.058, HZ + dz * 0.8), (reach, 0.034, HZ + dz * 1.25), 0.0065 if i else 0.0085, 0.006 if i else 0.008, mirror=mirror), foot)
            add(k.capsule(S, "toe-bone-%s-%d" % (side, i + 1), toe, BONE, 0.85, (reach + 0.004, 0.033, HZ + dz * 1.25), (reach + toe_len, 0.029, HZ + dz * 1.3), 0.0055 if i else 0.0075, 0.005 if i else 0.0065, mirror=mirror), toe)
        arm, fore, hand = "upper-arm-" + side, "forearm-" + side, "hand-" + side
        sign = -1 if mirror else 1
        loc = lambda sid, kind, bone, **f: k.st(b50.local(sid, kind, bone, BONE, 0.85, **f), bone)
        out.append(loc("humeral-head-" + side, "ellipsoid", arm, radii=[0.024, 0.024, 0.024], transform={"translation": [0.004, 0, 0]}))
        out.append(k.st(k.local_loft("humerus-" + side, arm, BONE, 0.85, [(0.010, 0.000, 0.004, 0.019, 0.019), (0.045, 0.000, 0.004, 0.0135, 0.0135), (0.160, 0.002, 0.002, 0.0115, 0.0115), (0.260, 0.001, 0.000, 0.012, 0.014), (0.300, 0.000, 0.000, 0.015, 0.028)], mirror=mirror, sub=1), arm))
        out.append(loc("humeral-condyle-" + side, "ellipsoid", arm, radii=[0.015, 0.014, 0.031], transform={"translation": [0.3135, 0.002, 0]}))
        out.append(loc("ulna-" + side, "capsule", fore, p0=[-0.020, -0.014, 0], p1=[0.262, -0.013, 0], r0=0.0115, r1=0.006))
        out.append(loc("radius-" + side, "capsule", fore, p0=[0.012, 0.014, 0], p1=[0.268, 0.013, 0], r0=0.0075, r1=0.0115))
        out.append(loc("carpus-" + side, "ellipsoid", hand, radii=[0.015, 0.026, 0.011], transform={"translation": [0.012, 0.001, 0]}))
        for i in range(4):
            pts = b50.finger_points(i, sign)
            out.append(loc("metacarpal-%s-%d" % (side, i + 2), "capsule", hand, p0=[0.024, k.r5(pts[0][1] * 0.6), 0], p1=[k.r5(c) for c in pts[0]], r0=0.0055, r1=0.0055))
            for j, (a, b) in enumerate(zip(pts, pts[1:])):
                out.append(loc("phalanx-%s-%d%s" % (side, i + 2, "abc"[j]), "capsule", hand, p0=[k.r5(c) for c in a], p1=[k.r5(c) for c in b], r0=0.0048, r1=0.0044))
        out.append(loc("metacarpal-%s-1" % side, "capsule", hand, p0=[0.012, 0.022, -sign * 0.004], p1=[0.040, 0.050, -sign * 0.009], r0=0.006, r1=0.006))
        out.append(loc("phalanx-%s-1a" % side, "capsule", hand, p0=[0.040, 0.050, -sign * 0.009], p1=[0.062, 0.064, -sign * 0.012], r0=0.0052, r1=0.005))
        out.append(loc("phalanx-%s-1b" % side, "capsule", hand, p0=[0.062, 0.064, -sign * 0.012], p1=[0.082, 0.071, -sign * 0.013], r0=0.005, r1=0.0045))
        joint = lambda sid, bone, world, radii: out.append(k.st(k.ellipsoid(S, sid, bone, BONE, 0.6, world, radii, mirror=mirror), bone, "joint"))
        joint("joint-hip-" + side, thigh, (0, 0.9406, HZ), (0.028, 0.028, 0.028))
        joint("joint-knee-" + side, shank, (0, 0.5206, HZ), (0.014, 0.014, 0.046))
        joint("joint-ankle-" + side, foot, (0, 0.0958, HZ), (0.012, 0.012, 0.034))
        out.append(k.st(b50.local("joint-shoulder-" + side, "ellipsoid", arm, BONE, 0.6, radii=[0.028, 0.028, 0.028]), arm, "joint"))
        out.append(k.st(b50.local("joint-elbow-" + side, "ellipsoid", fore, BONE, 0.6, radii=[0.012, 0.012, 0.034]), fore, "joint"))
        out.append(k.st(b50.local("joint-wrist-" + side, "ellipsoid", hand, BONE, 0.6, radii=[0.010, 0.026, 0.012]), hand, "joint"))
        out.append(k.st(b50.local("mass-thigh-" + side, "ellipsoid", thigh, BONE, 0.5, radii=[0.034, 0.034, 0.034], transform={"translation": [0.172, 0, 0]}), thigh, "mass"))
    add(k.loft(S, "sacrum", "pelvis", BONE, 0.85, [(1.046, -0.050, 0, 0.056, 0.024), (1.010, -0.070, 0, 0.050, 0.020), (0.965, -0.092, 0, 0.034, 0.015), (0.925, -0.096, 0, 0.018, 0.010), (0.898, -0.086, 0, 0.008, 0.006)], e=2.2, sub=1), "pelvis")
    add(k.ellipsoid(S, "symphysis", "pelvis", BONE, 0.85, (0.060, 0.915, 0), (0.012, 0.022, 0.014)), "pelvis")
    add(k.ellipsoid(S, "joint-lumbosacral", "torso", BONE, 0.6, (-0.045, 1.042, 0), (0.026, 0.008, 0.030), tilt=20), "torso", "joint")
    add(k.ellipsoid(S, "joint-c7", "head", BONE, 0.6, (-0.030, 1.528, 0), (0.018, 0.006, 0.021), tilt=10), "head", "joint")
    rig = {b["id"]: b for b in S.asset["rig"]["bones"]}
    for bone, r in (("torso", 0.046), ("pelvis", 0.033)):
        m = rig[bone]["mass"]
        out.append(k.st(b50.local("mass-" + bone, "ellipsoid", bone, BONE, 0.5, radii=[r, r, r], transform={"translation": [m["comX"], m["comY"], 0]}), bone, "mass"))
    out.append({"id": "st-box", "of": "box", "member": "box-r", "part": "member"})
    out.append({"id": "st-box-end-r", "of": "box-end-r", "member": "box-r", "part": "member"})
    out.append({"id": "st-box-end-l", "of": "box-end-l", "member": "box-l", "part": "member"})
    for side, w in (("r", 0.0), ("l", 2 * b50.GRIP_W)):
        out.append(k.st(b50.local("joint-grip-" + side, "capsule", "box-r", BONE, 0.6, p0=[-0.040, -0.010, w], p1=[0.040, -0.010, w], r0=0.008, r1=0.008), "box-" + side, "joint"))
        out.append(k.st(b50.local("mass-box-" + side, "ellipsoid", "box-r", "cardboard", 0.5, radii=[0.034, 0.034, 0.034], transform={"translation": [0, -0.085, b50.GRIP_W + (0.1 if side == "l" else -0.1)]}), "box-" + side, "mass"))
    return out


def face_rows(skull=b50.SKULL):
    """🎭 The bare part of the head as loft rows: per height the part of the skull section (rows below the vertex) ahead of the ear line, 0.5 mm proud of the skull solid."""
    rows = []
    for d, cx, _z, rz, rx in skull:
        if d < 0.092:
            continue
        back = -0.004 + 0.10 * (d - 0.092)
        front = cx + rx
        mid, half = (front + back) / 2, (front - back) / 2
        width = rz * (max(1 - abs((mid - cx) / rx) ** 2.15, 0.03)) ** (1 / 2.15)
        rows.append((TOP - d, mid + 0.0005, 0, width + 0.0005, half))
    return rows


def thermal(S, body):
    by = {s["id"]: s for s in body}
    out = [k.th(k.loft(S, "th-face", "head", "skin", 0.3, face_rows(), e=2.15, sub=1), "head-skin"), k.ref("neck", "head-skin")]
    out += [k.ref(i, "head-skin") for i in ("nose", "lips", "chin", "ear-l", "ear-r", "eye-l", "eye-r", "brow-l", "brow-r")]
    out.append(k.ref("hair", "hair"))
    scalp = [(TOP - r[0], r[1], 0, r[3] + 0.006, r[4] + 0.006) for r in b50.SKULL if r[0] <= 0.092]
    out.append(k.th(k.loft(S, "th-helmet-hair", "head", "hair-layer", 0.75, scalp, e=2.15, sub=1), "helmet", "hm-hair"))
    out.append(k.th(k.shrink(by["helmet"], 0.003, "th-helmet-air", material="harness-air"), "helmet", "hm-air"))
    out.append(k.th(dict(k.loft(S, "th-helmet-shell", "head", "hdpe-shell", 0.92, b50.DOME[:-1], e=2.15, sub=1, caps=(True, False))), "helmet", "hm-shell"))
    yoke = k.loft(S, "th-shirt-yoke", "torso", "cotton-clothing", 0.5, [r for r in b50.SHIRT if r[0] >= 1.40], e=2.4, sub=1)
    out += [k.th(k.shrink(yoke, 0.002, "th-shirt-yoke-air", material="still-air"), "torso-clothing", "ts-air"), k.th(k.shrink(yoke, 0.0012, "th-shirt-yoke-tee", material="tee-cotton"), "torso-clothing", "ts-tee"), k.th(yoke, "torso-clothing", "ts-shirt"), k.ref("shirt-collar", "torso-clothing", "ts-shirt")]
    out += [k.th(k.shrink(by["shirt"], 0.002, "th-vest-air", material="still-air"), "vest", "vs-air"), k.th(k.shrink(by["shirt"], 0.0012, "th-vest-tee", material="tee-cotton"), "vest", "vs-tee"), k.ref("shirt", "vest", "vs-shirt", "th-vest-shirt")]
    out.append(k.th(dict(by["vest"], id="th-vest", capStart=False, capEnd=False), "vest", "vs-vest"))
    out.append(k.th(dict(by["vest-hem"], id="th-vest-hem", capStart=False, capEnd=False), "vest", "vs-vest"))
    for side in ("l", "r"):
        for part in ("sleeve-", "forearm-"):
            out.append(k.th(k.shrink(by[part + side], 0.0012, "th-%s%s-air" % (part, side), material="still-air"), "arm-clothing", "am-air"))
            out.append(k.ref(part + side, "arm-clothing", "am-shirt"))
        gloves = [s for s in body if s["bone"] == "hand-" + side] + [by["glove-cuff-" + side]]
        for g in gloves:
            out.append(k.th(k.shrink(g, 0.002, "th-%s-air" % g["id"], material="still-air"), "gloves", "gl-air"))
            out.append(k.th(k.shrink(g, 0.001, "th-%s-lining" % g["id"], material="cotton-clothing"), "gloves", "gl-lining"))
            out.append(k.ref(g["id"], "gloves", "gl-leather"))
        for part in ("thigh-", "shank-"):
            out.append(k.th(k.shrink(by[part + side], 0.002, "th-%s%s-air" % (part, side), material="still-air"), "leg-clothing", "lg-air"))
        for part in ("thigh-", "shank-", "knee-", "knee-pad-", "cargo-pocket-", "buttock-"):
            out.append(k.ref(part + side, "leg-clothing", "lg-trousers"))
        for part in ("boot-shaft-", "boot-", "toe-cap-"):
            out.append(k.th(k.shrink(by[part + side], 0.011, "th-%s%s-sock" % (part, side), material="work-sock"), "shoes", "sh-sock"))
            out.append(k.th(k.shrink(by[part + side], 0.006, "th-%s%s-air" % (part, side), material="still-air"), "shoes", "sh-air"))
            out.append(k.ref(part + side, "shoes", "sh-leather"))
        for part in ("sole-", "toe-sole-"):
            out.append(k.ref(part + side, "shoes", "sh-leather"))
    out += [k.th(k.shrink(by["trousers-seat"], 0.0028, "th-seat-air", material="still-air"), "leg-clothing", "lg-air"), k.th(k.shrink(by["trousers-seat"], 0.002, "th-seat-briefs", material="briefs"), "leg-clothing", "lg-briefs"), k.ref("trousers-seat", "leg-clothing", "lg-trousers")]
    cu, cv, cw = b50.BOX_C
    out.append(k.th(b50.local("th-box", "box", "box-r", "cardboard", 0.66, size=[0.30, 0.30, 0.40], transform={"translation": [cu, cv, cw]}), "box-surface"))
    out.append(k.th(k.ellipsoid(S, "th-breath-nose", "head", "skin", 0.3, (0.1075, TOP - 0.1705, 0), (0.0039, 0.0039, 0.010), role="detail"), "head-skin", tags=["emitter", "breath-nose"]))
    out.append(k.th(k.ellipsoid(S, "th-breath-mouth", "head", "skin", 0.3, (0.1, TOP - 0.188, 0), (0.0039, 0.0039, 0.0065), role="detail"), "head-skin", tags=["emitter", "breath-mouth"]))
    out.append(k.th(k.ellipsoid(S, "th-plume", "head", "hdpe-shell", 0.92, (-0.010, 1.8005, 0), (0.078, 0.016, 0.066), role="detail"), "vest", tags=["emitter", "plume"]))
    return out


def lists(asset):
    S = k.Space(k.pose_frames(asset))
    S.asset = asset
    body = b50.body(S)
    return {"body": body, "thermal": thermal(S, body), "structure": skeleton(S, body), "decisions": texts.DECISIONS_3D["50"]}
