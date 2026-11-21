"""🧢 The three lists of the worker carrying a box at 1:100 (smallest feature 15 mm), stated anew: fewer and plainer sections, face reduced to nose, ears and chin line, helmet as dome, rim, peak and ridge, vest over a shirt yoke, mitten gloves with one curled finger block and a thumb, boots of shaft, vamp, sole and toe, the carton as a closed board body with the hand holes as 15 mm recesses in its end blocks; a simplified skeleton with four rib hoops per side, one metatarsal fan and one finger block per hand.

Coordinates as in `r3_worker_carrying_box_body50` (rest pose world: x ahead, y up, z left; limbs in their bone frames).
"""
import math

import r5_worker_carrying_box_body50 as b50
import r3_worker_carrying_box_kit as k

SKIN, HAIR, COTTON, VEST, BAND, HDPE, GLOVE, BOOT, BOARD = b50.SKIN, b50.HAIR, b50.COTTON, b50.VEST, b50.BAND, b50.HDPE, b50.GLOVE, b50.BOOT, b50.BOARD
TOP = b50.TOP
HZ = b50.HIP_Z
BONE = "skin"
SIDES = (("l", False, 1), ("r", True, -1))


def drop(rows):
    return [(TOP - r[0],) + tuple(r[1:]) for r in rows]


def curled(S, sid, bone, material, tone, pts, half_width, half_thick, sub=1, role="body", **extra):
    """🤜 A curled band through hand-local points (u, v, w) of `bone`, as wide as the four fingers together across the hand's v axis."""
    rows = [tuple(S.world(bone, p)) + (half_width, half_thick) for p in pts]
    return k.free_loft(S, sid, bone, material, tone, rows, sub=sub, role=role, across=tuple(S.frames[bone]["v"]), **extra)


def finger_path(sign, knuckle=(0.080, 0.000), lengths=(0.047, 0.027, 0.024)):
    """🖐 Knuckle, middle and end joint and tip of a finger block hooked towards the palm (-w for the left hand): 75, 80 and 30 degrees."""
    pts = [(knuckle[0], knuckle[1], 0.0)]
    bend = 0.0
    for length, add in zip(lengths, (75, 80, 30)):
        bend += add
        p = pts[-1]
        pts.append((p[0] + length * math.cos(math.radians(bend)), p[1], p[2] - sign * length * math.sin(math.radians(bend))))
    return pts


def side_at(rows, y):
    return b50.side_at(rows, y)


VEST_ROWS = [(1.030, -0.004, 0, 0.188, 0.141, 2.35), (1.180, -0.004, 0, 0.174, 0.140, 2.35), (1.320, -0.013, 0, 0.179, 0.138, 2.35), (1.400, -0.019, 0, 0.168, 0.123, 2.35), (1.470, -0.024, 0, 0.160, 0.090, 2.35), (1.489, -0.025, 0, 0.150, 0.080, 2.35)]
SKULL = [(0.000, -0.005, 0, 0.014, 0.018), (0.012, -0.005, 0, 0.048, 0.060), (0.035, -0.004, 0, 0.067, 0.087), (0.075, -0.002, 0, 0.0765, 0.0985), (0.125, 0.004, 0, 0.073, 0.091), (0.170, 0.018, 0, 0.064, 0.074), (0.210, 0.034, 0, 0.050, 0.056), (0.232, 0.050, 0, 0.022, 0.028)]
HEM_ROWS = [(0.930, -0.005, 0, 0.195, 0.141, 2.35), (1.000, -0.005, 0, 0.193, 0.1415, 2.35), (1.075, -0.003, 0, 0.1805, 0.1395, 2.35)]


def body(S):
    out = [
        k.loft(S, "neck", "head", SKIN, 0.3, [(1.455, -0.031, 0, 0.073, 0.069), (1.540, -0.019, 0, 0.061, 0.062), (1.605, -0.010, 0, 0.054, 0.056)], e=2.0, sub=1),
        k.loft(S, "head", "head", SKIN, 0.3, drop(SKULL), e=2.15, sub=1),
        k.loft(S, "hair", "head", HAIR, 0.75, drop([(0.052, -0.012, 0, 0.073, 0.088), (0.110, -0.025, 0, 0.0785, 0.078), (0.160, -0.047, 0, 0.060, 0.044)]), e=2.1, sub=1),
        k.loft(S, "nose", "head", SKIN, 0.3, drop([(0.110, 0.096, 0, 0.0075, 0.0075), (0.150, 0.111, 0, 0.0125, 0.0135), (0.170, 0.104, 0, 0.010, 0.0075)]), e=2.0, role="detail"),
        k.loft(S, "helmet", "head", HDPE, 0.92, [(1.815, -0.010, 0, 0.014, 0.017), (1.802, -0.010, 0, 0.062, 0.074), (1.765, -0.008, 0, 0.096, 0.115), (1.715, -0.006, 0, 0.109, 0.130), (1.681, -0.005, 0, 0.110, 0.131)], e=2.15, sub=1),
        k.loft(S, "helmet-rim", "head", HDPE, 0.92, [(1.692, -0.005, 0, 0.1135, 0.1345), (1.672, -0.005, 0, 0.1155, 0.1365)], e=2.15),
        k.loft(S, "helmet-peak", "head", HDPE, 0.92, [(0.092, 1.682, 0, 0.100, 0.0075), (0.150, 1.677, 0, 0.082, 0.0075), (0.188, 1.672, 0, 0.026, 0.0075)], axes="x", e=2.6),
        k.free_loft(S, "helmet-ridge", "head", HDPE, 0.85, [(x, b50.dome_top(x) - 0.004, 0, 0.017, 0.0075) for x in (-0.110, -0.060, -0.008, 0.050, 0.095)], e=2.4, role="detail", across=k.Z),
        k.loft(S, "shirt-yoke", "torso", COTTON, 0.5, [(1.380, -0.017, 0, 0.176, 0.126, 2.4), (1.440, -0.022, 0, 0.198, 0.099, 2.4), (1.470, -0.024, 0, 0.184, 0.085, 2.35), (1.500, -0.025, 0, 0.135, 0.074, 2.2), (1.534, -0.022, 0, 0.074, 0.067, 2.0)], e=2.4, sub=1),
        k.loft(S, "vest", "torso", VEST, 0.9, VEST_ROWS, e=2.35, sub=1),
        k.loft(S, "vest-hem", "pelvis", VEST, 0.9, HEM_ROWS, e=2.35),
        k.loft(S, "trousers-seat", "pelvis", COTTON, 0.3, [(0.838, 0.000, 0, 0.112, 0.088, 2.3), (0.875, -0.012, 0, 0.182, 0.135, 2.35), (0.930, -0.012, 0, 0.190, 0.140, 2.35), (1.000, -0.006, 0, 0.181, 0.131, 2.35), (1.048, -0.004, 0, 0.171, 0.128, 2.35)], e=2.35, sub=1),
    ]
    for name, lo, hi in (("vest-band-low", 1.055, 1.105), ("vest-band-high", 1.185, 1.235)):
        rows = [(y,) + (lambda c: (c[0], 0, c[1] + 0.002, c[2] + 0.002, 2.35))(side_at(VEST_ROWS, y)) for y in (lo, (lo + hi) / 2, hi)]
        out.append(k.loft(S, name, "torso", BAND, 0.6, rows, role="detail", texture=True))
    for side, mirror, sign in SIDES:
        out.append(k.ellipsoid(S, "ear-" + side, "head", SKIN, 0.35, (-0.012, TOP - 0.130, 0.0765), (0.016, 0.030, 0.0075), role="detail", mirror=mirror, tilt=-10))
        path = []
        for y in (1.235, 1.340, 1.430, 1.482):
            cx, rz, rx = side_at(VEST_ROWS, y)
            path.append((cx + rx * (1 - (0.105 / rz) ** 2.35) ** (1 / 2.35) + 0.0015, y, 0.105, 0.025, 0.003))
        top = side_at(VEST_ROWS, 1.489)
        path.append((top[0], 1.4915, 0.105, 0.025, 0.003))
        for y in (1.482, 1.430, 1.340, 1.235):
            cx, rz, rx = side_at(VEST_ROWS, y)
            path.append((cx - rx * (1 - (0.105 / rz) ** 2.35) ** (1 / 2.35) - 0.0015, y, 0.105, 0.025, 0.003))
        out.append(k.free_loft(S, "vest-brace-" + side, "torso", BAND, 0.6, path, e=4.0, role="detail", mirror=mirror, across=k.Z, texture=True))
        out += [
            k.loft(S, "thigh-" + side, "thigh-" + side, COTTON, 0.3, [(0.985, -0.004, 0.094, 0.086, 0.104), (0.880, 0.002, 0.097, 0.092, 0.110), (0.760, 0.008, 0.096, 0.086, 0.098), (0.600, 0.009, 0.092, 0.073, 0.080), (0.498, 0.007, 0.090, 0.066, 0.072)], e=2.2, sub=1, mirror=mirror),
            k.ellipsoid(S, "knee-" + side, "shank-" + side, COTTON, 0.3, (0.006, 0.5206, 0.090), (0.064, 0.066, 0.061), mirror=mirror),
            k.loft(S, "shank-" + side, "shank-" + side, COTTON, 0.3, [(0.565, 0.007, 0.090, 0.066, 0.072), (0.470, 0.002, 0.090, 0.065, 0.069), (0.330, -0.007, 0.090, 0.065, 0.070), (0.160, -0.004, 0.090, 0.062, 0.066), (0.104, -0.002, 0.090, 0.062, 0.067)], e=2.2, sub=1, mirror=mirror),
            k.loft(S, "boot-shaft-" + side, "foot-" + side, BOOT, 0.12, [(0.176, -0.012, HZ, 0.047, 0.055), (0.095, -0.010, HZ, 0.047, 0.059), (0.034, -0.008, HZ, 0.047, 0.073)], e=2.2, mirror=mirror),
            k.loft(S, "boot-" + side, "foot-" + side, BOOT, 0.12, [(-0.030, 0.076, HZ, 0.047, 0.046), (0.070, 0.066, HZ + 0.001, 0.053, 0.040), (0.138, 0.055, HZ + 0.002, 0.054, 0.034)], axes="x", e=2.3, sub=1, mirror=mirror),
            k.loft(S, "sole-" + side, "foot-" + side, BOOT, 0.05, [(-0.084, 0.016, HZ, 0.032, 0.015, 3.0), (-0.050, 0.0155, HZ, 0.044, 0.0155, 4.0), (0.040, 0.013, HZ + 0.001, 0.045, 0.010, 4.0), (0.138, 0.0095, HZ + 0.002, 0.056, 0.0095, 4.0)], axes="x", mirror=mirror),
            k.loft(S, "toe-" + side, "toe-" + side, BOOT, 0.12, [(0.118, 0.046, HZ + 0.002, 0.055, 0.046, 2.6), (0.180, 0.040, HZ + 0.002, 0.053, 0.040, 2.6), (0.208, 0.031, HZ + 0.001, 0.042, 0.030, 2.4), (0.2205, 0.021, HZ + 0.001, 0.018, 0.018, 2.2)], axes="x", mirror=mirror),
            k.local_loft("sleeve-" + side, "upper-arm-" + side, COTTON, 0.5, [(-0.040, 0.000, -0.016, 0.012, 0.016), (-0.016, 0.000, -0.004, 0.050, 0.051), (0.060, -0.002, 0.009, 0.060, 0.066), (0.210, 0.001, 0.001, 0.053, 0.050), (0.330, 0.000, 0.000, 0.047, 0.045)], e=2.1, sub=1, mirror=mirror),
            k.local_loft("forearm-" + side, "forearm-" + side, COTTON, 0.5, [(-0.034, 0.000, 0.000, 0.045, 0.044), (0.045, 0.004, 0.000, 0.051, 0.047), (0.170, 0.001, 0.000, 0.040, 0.036), (0.215, 0.000, 0.000, 0.036, 0.032)], e=2.1, sub=1, mirror=mirror),
            k.local_loft("glove-cuff-" + side, "forearm-" + side, GLOVE, 0.4, [(0.200, 0.000, 0.000, 0.043, 0.039), (0.240, 0.000, 0.000, 0.040, 0.034), (0.285, 0.000, 0.000, 0.035, 0.026)], e=2.1, mirror=mirror),
            k.local_loft("hand-" + side, "hand-" + side, GLOVE, 0.4, [(-0.012, 0.000, 0.000, 0.033, 0.024), (0.040, 0.002, 0.001, 0.046, 0.018), (0.088, 0.000, 0.000, 0.045, 0.014)], e=2.6, mirror=mirror),
            curled(S, "fingers-" + side, "hand-" + side, GLOVE, 0.4, finger_path(sign), 0.042, 0.0105),
            k.capsule(S, "thumb-" + side, "hand-" + side, GLOVE, 0.4, S.world("hand-" + side, (0.022, 0.036, -sign * 0.006)), S.world("hand-" + side, (0.078, 0.068, -sign * 0.013)), 0.0130, 0.0115, role="detail"),
        ]
    return out + carton()


def slot(n_half=4):
    cu, cv, hl, hh = b50.SLOT
    ring = []
    for centre, start in ((cu + hl - hh, -90), (cu - hl + hh, 90)):
        for i in range(n_half + 1):
            a = math.radians(start + 180 * i / n_half)
            ring.append([k.r5(centre + hh * math.cos(a)), k.r5(cv + hh * math.sin(a))])
    return ring


def carton():
    """📦 The carton at 1:100: a closed board body whose two end blocks carry the hand holes as 15 mm deep recesses (the 7.5 mm board and the air behind it are one feature at this scale), the tape over the lid."""
    cu, cv, cw = b50.BOX_C
    hu, hv, hw = b50.BOX_H
    d = 0.015
    outer = [[cu - hu, cv - hv], [cu + hu, cv - hv], [cu + hu, cv + hv], [cu - hu, cv + hv]]
    end = lambda sid, w0: b50.local(sid, "extrude", "box-r", BOARD, 0.66, plane="xy", profile=outer, holes=[slot()], **{"from": k.r5(w0), "to": k.r5(w0 + d)})
    return [
        b50.local("box", "extrude", "box-r", BOARD, 0.66, plane="xy", profile=outer, **{"from": k.r5(cw - hw + d), "to": k.r5(cw + hw - d)}),
        end("box-end-r", cw - hw),
        end("box-end-l", cw + hw - d),
        b50.local("box-tape", "box", "box-r", BOARD, 0.82, role="detail", size=[0.05, 0.0012, 2 * hw + 0.0012], transform={"translation": [cu, cv + hv + 0.0006, cw]}, texture=True),
    ]


def thermal(S, body):
    by = {s["id"]: s for s in body}
    import r3_worker_carrying_box_s50 as s50

    region = {"head-skin": ["neck", "nose", "ear-l", "ear-r"], "hair": ["hair"], "helmet": ["helmet", "helmet-rim", "helmet-peak", "helmet-ridge"], "torso-clothing": ["shirt-yoke"]}
    out = [k.ref(i, r) for r, ids in region.items() for i in ids]
    out.append(k.th(k.loft(S, "th-face", "head", SKIN, 0.3, s50.face_rows([r for r in SKULL if r[0] >= 0.075]), e=2.15, sub=1), "head-skin"))
    out += [k.th(dict(by[i], id="th-" + i, capStart=False, capEnd=False), "vest") for i in ("vest", "vest-hem")]
    for side in ("l", "r"):
        out += [k.ref(i + side, "arm-clothing") for i in ("sleeve-", "forearm-")]
        out += [k.ref(i + side, "gloves") for i in ("glove-cuff-", "hand-", "fingers-", "thumb-")]
        out += [k.ref(i + side, "leg-clothing") for i in ("thigh-", "knee-", "shank-")]
        out += [k.ref(i + side, "shoes") for i in ("boot-shaft-", "boot-", "sole-", "toe-")]
    out.append(k.ref("trousers-seat", "leg-clothing"))
    cu, cv, cw = b50.BOX_C
    out.append(k.th(b50.local("th-box", "box", "box-r", BOARD, 0.66, size=[0.30, 0.30, 0.40], transform={"translation": [cu, cv, cw]}), "box-surface"))
    out.append(k.th(k.ellipsoid(S, "th-breath-nose", "head", SKIN, 0.3, (0.107, TOP - 0.171, 0), (0.0075, 0.0075, 0.012), role="detail"), "head-skin", tags=["emitter", "breath-nose"]))
    out.append(k.th(k.ellipsoid(S, "th-breath-mouth", "head", SKIN, 0.3, (0.098, TOP - 0.190, 0), (0.0075, 0.0075, 0.012), role="detail"), "head-skin", tags=["emitter", "breath-mouth"]))
    out.append(k.th(k.ellipsoid(S, "th-plume", "head", HDPE, 0.92, (-0.010, 1.8035, 0), (0.075, 0.0125, 0.063), role="detail"), "vest", tags=["emitter", "plume"]))
    assert all(r.get("of") in by for r in out if "of" in r)
    return out


def rib(S, side, mirror, level, half, front, drop, n):
    sx, sy = b50_spine()[level]
    rows = [(sx - 0.004, sy + 0.004, 0.018, 0.0085, 0.0085), (sx - 0.026, sy + 0.002, 0.046, 0.0085, 0.0085)]
    back = sx - 0.034
    xc, depth = (front + back) / 2, (front - back) / 2
    for i in range(7):
        a = math.pi - 0.50 + (0.40 - (math.pi - 0.50)) * i / 6
        rows.append((xc + depth * math.cos(a), sy - drop * (1 + math.cos(a)) / 2, half * math.sin(a) ** 0.8, 0.0085, 0.0085))
    return k.st(k.free_loft(S, "rib-%s-%d" % (side, n), "torso", BONE, 0.85, rows, mirror=mirror, sub=1), "torso")


def b50_spine():
    import r3_worker_carrying_box_s50 as s50

    return s50.SPINE


def structure(S, body):
    spine = b50_spine()
    out = []
    add = lambda solid, member, part="member": out.append(k.st(solid, member, part))
    add(k.ellipsoid(S, "cranium", "head", BONE, 0.85, (-0.012, TOP - 0.080, 0), (0.090, 0.072, 0.070)), "head")
    add(k.loft(S, "face-bones", "head", BONE, 0.85, [(TOP - 0.098, 0.050, 0, 0.056, 0.040), (TOP - 0.150, 0.060, 0, 0.056, 0.036), (TOP - 0.182, 0.062, 0, 0.032, 0.030)], e=2.2), "head")
    add(k.free_loft(S, "mandible", "head", BONE, 0.85, [(-0.010, TOP - 0.135, 0.058, 0.0075, 0.013), (0.004, TOP - 0.196, 0.052, 0.0075, 0.011), (0.064, TOP - 0.216, 0.024, 0.008, 0.012), (0.076, TOP - 0.219, 0.000, 0.008, 0.013), (0.064, TOP - 0.216, -0.024, 0.008, 0.012), (0.004, TOP - 0.196, -0.052, 0.0075, 0.011), (-0.010, TOP - 0.135, -0.058, 0.0075, 0.013)], sub=1), "head")
    add(k.capsule(S, "cervical-spine", "head", BONE, 0.85, (-0.030, 1.530, 0), (-0.015, 1.640, 0), 0.0125, 0.0115), "head")
    add(k.free_loft(S, "spine", "torso", BONE, 0.85, [(x, y, 0, 0.022 - 0.004 * i / 9, 0.019 - 0.004 * i / 9) for i, (x, y) in enumerate(spine[::2])] + [(spine[-1][0], spine[-1][1], 0, 0.017, 0.0145)], sub=1, across=k.Z), "torso")
    for n, (level, half, front, drop) in enumerate(((16, 0.104, 0.068, 0.058), (13, 0.134, 0.097, 0.094), (10, 0.139, 0.094, 0.112), (8, 0.124, 0.040, 0.070))):
        for side, mirror, _ in SIDES:
            out.append(rib(S, side, mirror, level, half, front, drop, n + 1))
    add(k.free_loft(S, "sternum", "torso", BONE, 0.85, [(0.048, 1.462, 0, 0.026, 0.0075), (0.082, 1.360, 0, 0.018, 0.0075), (0.103, 1.268, 0, 0.009, 0.0075)], e=2.5, across=k.Z), "torso")
    add(k.loft(S, "sacrum", "pelvis", BONE, 0.85, [(1.046, -0.050, 0, 0.056, 0.024), (0.965, -0.092, 0, 0.034, 0.016), (0.898, -0.086, 0, 0.010, 0.0075)], e=2.2, sub=1), "pelvis")
    add(k.ellipsoid(S, "symphysis", "pelvis", BONE, 0.85, (0.060, 0.915, 0), (0.012, 0.022, 0.014)), "pelvis")
    add(k.ellipsoid(S, "joint-lumbosacral", "torso", BONE, 0.6, (-0.045, 1.042, 0), (0.026, 0.0085, 0.030), tilt=20), "torso", "joint")
    add(k.ellipsoid(S, "joint-c7", "head", BONE, 0.6, (-0.030, 1.528, 0), (0.019, 0.0085, 0.022), tilt=10), "head", "joint")
    for side, mirror, sign in SIDES:
        thigh, shank, foot, toe = "thigh-" + side, "shank-" + side, "foot-" + side, "toe-" + side
        arm, fore, hand = "upper-arm-" + side, "forearm-" + side, "hand-" + side
        add(k.capsule(S, "clavicle-" + side, "torso", BONE, 0.85, (0.046, 1.458, 0.022), (-0.022, 1.488, 0.188), 0.0085, 0.0080, mirror=mirror), "torso")
        add(k.ellipsoid(S, "scapula-" + side, "torso", BONE, 0.85, (-0.100, 1.400, 0.112), (0.009, 0.078, 0.052), mirror=mirror, tilt=8, yaw=-28), "torso")
        add(k.capsule(S, "scapular-spine-" + side, "torso", BONE, 0.85, (-0.106, 1.440, 0.075), (-0.030, 1.490, 0.195), 0.0075, 0.0085, mirror=mirror), "torso")
        add(k.ellipsoid(S, "ilium-" + side, "pelvis", BONE, 0.85, (-0.024, 1.012, 0.112), (0.072, 0.066, 0.013), mirror=mirror, tilt=-12, yaw=38), "pelvis")
        add(k.capsule(S, "pubic-ramus-" + side, "pelvis", BONE, 0.85, (0.004, 0.946, 0.078), (0.060, 0.916, 0.012), 0.011, 0.011, mirror=mirror), "pelvis")
        add(k.capsule(S, "ischium-" + side, "pelvis", BONE, 0.85, (-0.010, 0.930, 0.082), (-0.036, 0.864, 0.060), 0.014, 0.015, mirror=mirror), "pelvis")
        add(k.ellipsoid(S, "femoral-head-" + side, thigh, BONE, 0.85, (0, 0.9406, HZ), (0.024, 0.024, 0.024), mirror=mirror), thigh)
        add(k.free_loft(S, "femur-" + side, thigh, BONE, 0.85, [(0, 0.9406, HZ, 0.015, 0.015), (-0.004, 0.912, HZ + 0.054, 0.019, 0.019), (0.004, 0.840, HZ + 0.040, 0.0150, 0.0150), (0.016, 0.650, HZ + 0.010, 0.0145, 0.0145), (0.006, 0.556, HZ + 0.001, 0.026, 0.020)], mirror=mirror, sub=1, across=k.Z), thigh)
        add(k.ellipsoid(S, "femoral-condyles-" + side, thigh, BONE, 0.85, (0.002, 0.532, HZ), (0.034, 0.024, 0.043), mirror=mirror), thigh)
        add(k.ellipsoid(S, "patella-" + side, shank, BONE, 0.85, (0.046, 0.530, HZ), (0.0105, 0.022, 0.021), mirror=mirror), shank)
        add(k.loft(S, "tibia-" + side, shank, BONE, 0.85, [(0.506, 0.000, HZ, 0.040, 0.027), (0.420, 0.008, HZ - 0.002, 0.0155, 0.0165), (0.180, 0.002, HZ - 0.003, 0.013, 0.013), (0.104, 0.000, HZ - 0.004, 0.024, 0.019)], e=2.0, sub=1, mirror=mirror), shank)
        add(k.capsule(S, "fibula-" + side, shank, BONE, 0.85, (-0.014, 0.486, HZ + 0.036), (-0.008, 0.088, HZ + 0.031), 0.0075, 0.0085, mirror=mirror), shank)
        add(k.ellipsoid(S, "talus-" + side, foot, BONE, 0.85, (0.004, 0.084, HZ), (0.028, 0.017, 0.021), mirror=mirror), foot)
        add(k.capsule(S, "calcaneus-" + side, foot, BONE, 0.85, (-0.048, 0.055, HZ - 0.002), (0.006, 0.066, HZ), 0.021, 0.016, mirror=mirror), foot)
        add(k.free_loft(S, "metatarsus-" + side, foot, BONE, 0.85, [(0.030, 0.068, HZ, 0.026, 0.012), (0.090, 0.046, HZ, 0.036, 0.0085), (0.130, 0.035, HZ - 0.002, 0.040, 0.0075)], mirror=mirror, across=k.Z), foot)
        add(k.free_loft(S, "toe-bones-" + side, toe, BONE, 0.85, [(0.134, 0.034, HZ - 0.002, 0.040, 0.0075), (0.165, 0.032, HZ - 0.003, 0.036, 0.0075), (0.190, 0.029, HZ - 0.006, 0.020, 0.0075)], mirror=mirror, across=k.Z), toe)
        loc = lambda sid, kind, bone, **f: out.append(k.st(b50.local(sid, kind, bone, BONE, 0.85, **f), bone))
        loc("humeral-head-" + side, "ellipsoid", arm, radii=[0.024, 0.024, 0.024], transform={"translation": [0.004, 0, 0]})
        out.append(k.st(k.local_loft("humerus-" + side, arm, BONE, 0.85, [(0.010, 0.000, 0.004, 0.019, 0.019), (0.160, 0.002, 0.002, 0.0115, 0.0115), (0.300, 0.000, 0.000, 0.015, 0.028)], mirror=mirror, sub=1), arm))
        loc("ulna-" + side, "capsule", fore, p0=[-0.020, -0.014, 0], p1=[0.262, -0.013, 0], r0=0.0115, r1=0.0075)
        loc("radius-" + side, "capsule", fore, p0=[0.012, 0.014, 0], p1=[0.268, 0.013, 0], r0=0.0075, r1=0.0115)
        loc("carpus-" + side, "ellipsoid", hand, radii=[0.015, 0.026, 0.011], transform={"translation": [0.012, 0.001, 0]})
        out.append(k.st(k.local_loft("metacarpus-" + side, hand, BONE, 0.85, [(0.024, 0.000, 0.000, 0.022, 0.0075), (0.080, 0.000, 0.000, 0.038, 0.0075)], mirror=mirror), hand))
        out.append(k.st(curled(S, "phalanges-" + side, hand, BONE, 0.85, finger_path(sign, (0.082, 0.0), (0.045, 0.026, 0.020)), 0.036, 0.0075), hand))
        out.append(k.st(k.capsule(S, "thumb-bones-" + side, hand, BONE, 0.85, S.world(hand, (0.012, 0.022, -sign * 0.004)), S.world(hand, (0.080, 0.070, -sign * 0.013)), 0.0075, 0.0075), hand))
        joint = lambda sid, bone, world, radii: out.append(k.st(k.ellipsoid(S, sid, bone, BONE, 0.6, world, radii, mirror=mirror), bone, "joint"))
        joint("joint-hip-" + side, thigh, (0, 0.9406, HZ), (0.028, 0.028, 0.028))
        joint("joint-knee-" + side, shank, (0, 0.5206, HZ), (0.014, 0.014, 0.046))
        joint("joint-ankle-" + side, foot, (0, 0.0958, HZ), (0.012, 0.012, 0.034))
        out.append(k.st(b50.local("joint-shoulder-" + side, "ellipsoid", arm, BONE, 0.6, radii=[0.028, 0.028, 0.028]), arm, "joint"))
        out.append(k.st(b50.local("joint-elbow-" + side, "ellipsoid", fore, BONE, 0.6, radii=[0.012, 0.012, 0.034]), fore, "joint"))
        out.append(k.st(b50.local("joint-wrist-" + side, "ellipsoid", hand, BONE, 0.6, radii=[0.010, 0.026, 0.012]), hand, "joint"))
        out.append(k.st(b50.local("mass-thigh-" + side, "ellipsoid", thigh, BONE, 0.5, radii=[0.034, 0.034, 0.034], transform={"translation": [0.172, 0, 0]}), thigh, "mass"))
    rig = {b["id"]: b for b in S.asset["rig"]["bones"]}
    for bone, r in (("torso", 0.046), ("pelvis", 0.033)):
        m = rig[bone]["mass"]
        out.append(k.st(b50.local("mass-" + bone, "ellipsoid", bone, BONE, 0.5, radii=[r, r, r], transform={"translation": [m["comX"], m["comY"], 0]}), bone, "mass"))
    out += [{"id": "st-box", "of": "box", "member": "box-r", "part": "member"}, {"id": "st-box-end-r", "of": "box-end-r", "member": "box-r", "part": "member"}, {"id": "st-box-end-l", "of": "box-end-l", "member": "box-l", "part": "member"}]
    for side, w in (("r", 0.0), ("l", 2 * b50.GRIP_W)):
        out.append(k.st(b50.local("mass-box-" + side, "ellipsoid", "box-r", BOARD, 0.5, radii=[0.034, 0.034, 0.034], transform={"translation": [0, -0.085, b50.GRIP_W + (0.1 if side == "l" else -0.1)]}), "box-" + side, "mass"))
    return out


DECISIONS = [
    {"en": "Body 1:100, authored anew for a 15 mm feature: head as one skull loft with nose, chin and ears (eyes, brows and lips are below the feature), neck, hair at the nape; helmet as dome, rim, 15 mm peak and ridge (headband omitted); hi-vis vest over a shirt yoke and collar in one loft, the vest hem on the pelvis so that it follows the hips in the lift, reflective bands and braces as surface texture; trouser seat with the gluteal mass merged, thigh, knee and shank (cargo pockets and knee pads omitted); boots as shaft, vamp, 19 mm sole and one toe-cap mass; sleeves with the deltoid, forearms, glove cuffs, mitten gloves with one curled finger block and a thumb; the carton as a closed board body whose end blocks carry the hand holes as 15 mm deep recesses (board and air behind it are one feature at this scale), tape over the lid.", "de": "Körper 1:100, neu modelliert für 15 mm Mindestmaß: Kopf als ein Schädel-Loft mit Nase, Kinn und Ohren (Augen, Brauen und Lippen liegen unter dem Mindestmaß), Hals, Haar im Nacken; Helm als Kuppel, Rand, 15 mm Schirm und Grat (Kopfband entfällt); Warnweste über Hemdpasse und Kragen in einem Loft, der Westensaum am Becken, damit er beim Heben der Hüfte folgt, Reflexstreifen und Träger als Oberflächentextur; Hosengesäß mit der Gesäßmasse verschmolzen, Oberschenkel, Knie und Unterschenkel (Beintaschen und Knieschoner entfallen); Stiefel aus Schaft, Blatt, 19 mm Sohle und einer Zehenkappenmasse; Ärmel mit Deltamuskel, Unterarme, Handschuhstulpen, Fäustlingshandschuhe mit einem gekrümmten Fingerblock und Daumen; der Karton als geschlossener Wellpappenkörper, dessen Stirnblöcke die Grifflöcher als 15 mm tiefe Mulden tragen (Pappe und Luft dahinter sind in diesem Maßstab ein Mindestmaß), Klebeband über dem Deckel."},
    {"en": "Thermal 1:100: the layers of each region are merged into its outer surface (layer shells only at 1:50); every region references its body solids; the carton surface is its closed 0.40 x 0.30 x 0.30 m box; breath outlets under the nose and on the lips, the body plume as a patch on the helmet crown. Structure 1:100: skull with facial skeleton and mandible, cervical and thoracolumbar spine without separate vertebrae, four rib hoops per side, sternum, clavicles, scapulae, sacrum and hip bones, femur with head and condyles, patella, tibia and fibula, talus, calcaneus, one metatarsal fan and one toe row, humerus, ulna and radius, carpus, one metacarpal plate, one curled finger row and the thumb; the carton walls carry the load; joints at hip, knee, ankle, shoulder, elbow, wrist, L5/S1 and C7.", "de": "Thermik 1:100: die Schichten jeder Region sind in ihre Außenfläche zusammengefasst (Schichtschalen nur in 1:50); jede Region verweist auf ihre Körperteile; die Kartonoberfläche ist sein geschlossener Quader 0,40 x 0,30 x 0,30 m; Atemauslässe unter der Nase und an den Lippen, die Wärmefahne als Fleck auf dem Helmscheitel. Tragwerk 1:100: Schädel mit Gesichtsschädel und Unterkiefer, Hals- und Brust-Lenden-Wirbelsäule ohne einzelne Wirbel, vier Rippenbögen je Seite, Brustbein, Schlüsselbeine, Schulterblätter, Kreuzbein und Hüftbeine, Oberschenkelknochen mit Kopf und Kondylen, Kniescheibe, Schien- und Wadenbein, Sprung- und Fersenbein, ein Mittelfußfächer und eine Zehenreihe, Oberarmknochen, Elle und Speiche, Handwurzel, eine Mittelhandplatte, eine gekrümmte Fingerreihe und der Daumen; die Kartonwände tragen die Last; Gelenke an Hüfte, Knie, Knöchel, Schulter, Ellbogen, Handgelenk, L5/S1 und C7."},
]


def lists(asset):
    S = k.Space(k.pose_frames(asset))
    S.asset = asset
    b = body(S)
    return {"body": b, "thermal": thermal(S, b), "structure": structure(S, b), "decisions": DECISIONS}
