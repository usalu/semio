"""📦 The three lists of the worker carrying a box at 1:500 (smallest feature 75 mm), stated anew: head under the helmet dome, one trunk mass in the vest, seat, two-part limbs ending in fists and boots, the carton as a block; a stick skeleton of 75 mm bars."""
import r3_worker_carrying_box_body50 as b50
import r3_worker_carrying_box_kit as k
import r3_worker_carrying_box_s100 as s100

SKIN, HAIR, COTTON, VEST, HDPE, GLOVE, BOOT, BOARD = b50.SKIN, b50.HAIR, b50.COTTON, b50.VEST, b50.HDPE, b50.GLOVE, b50.BOOT, b50.BOARD
TOP = b50.TOP
HZ = b50.HIP_Z
BONE = "skin"
SIDES = s100.SIDES
R = 0.0375


def body(S):
    out = [
        k.loft(S, "head", "head", SKIN, 0.3, [(1.520, -0.026, 0, 0.062, 0.064), (1.600, -0.004, 0, 0.068, 0.090), (1.690, 0.010, 0, 0.078, 0.100), (1.740, 0.000, 0, 0.060, 0.075)], e=2.15, sub=1),
        k.loft(S, "helmet", "head", HDPE, 0.92, [(1.815, -0.010, 0, 0.045, 0.052), (1.780, -0.009, 0, 0.096, 0.112), (1.715, -0.005, 0, 0.112, 0.133), (1.672, 0.032, 0, 0.120, 0.150)], e=2.15, sub=1),
        k.loft(S, "vest", "torso", VEST, 0.9, [(1.030, -0.004, 0, 0.190, 0.142, 2.5), (1.300, -0.012, 0, 0.180, 0.139, 2.5), (1.440, -0.022, 0, 0.200, 0.105, 2.6), (1.500, -0.024, 0, 0.140, 0.075, 2.3)], e=2.5, sub=1),
        k.loft(S, "seat", "pelvis", COTTON, 0.3, [(0.850, -0.002, 0, 0.150, 0.110, 2.3), (0.930, -0.010, 0, 0.196, 0.142, 2.5), (1.075, -0.004, 0, 0.181, 0.140, 2.5)], e=2.5),
        k.ellipsoid(S, "hair", "head", HAIR, 0.75, (-0.058, 1.640, 0), (0.0375, 0.0375, 0.060)),
        k.loft(S, "collar", "torso", COTTON, 0.5, [(1.470, -0.023, 0, 0.125, 0.078, 2.2), (1.550, -0.020, 0, 0.070, 0.0675, 2.0)], e=2.1),
    ]
    for side, mirror, sign in SIDES:
        out += [
            k.loft(S, "thigh-" + side, "thigh-" + side, COTTON, 0.3, [(0.985, -0.004, 0.094, 0.088, 0.106), (0.520, 0.007, 0.090, 0.068, 0.074)], e=2.2, mirror=mirror),
            k.loft(S, "shank-" + side, "shank-" + side, COTTON, 0.3, [(0.570, 0.008, 0.090, 0.068, 0.076), (0.100, -0.003, 0.090, 0.060, 0.064)], e=2.2, mirror=mirror),
            k.loft(S, "boot-" + side, "foot-" + side, BOOT, 0.12, [(-0.084, 0.060, HZ, 0.045, 0.060, 3.0), (0.020, 0.075, HZ, 0.050, 0.075, 3.0), (0.135, 0.040, HZ + 0.002, 0.055, 0.040, 3.0)], axes="x", e=3.0, mirror=mirror),
            k.loft(S, "toe-" + side, "toe-" + side, BOOT, 0.12, [(0.120, 0.040, HZ + 0.002, 0.055, 0.040, 2.6), (0.2215, 0.0375, HZ + 0.001, 0.040, 0.0375, 2.4)], axes="x", mirror=mirror),
            k.local_loft("sleeve-" + side, "upper-arm-" + side, COTTON, 0.5, [(-0.030, 0.000, -0.006, 0.045, 0.045), (0.020, -0.002, 0.006, 0.064, 0.060), (0.320, 0.000, 0.000, 0.048, 0.046)], e=2.1, mirror=mirror),
            k.local_loft("forearm-" + side, "forearm-" + side, COTTON, 0.5, [(-0.030, 0.000, 0.000, 0.048, 0.046), (0.280, 0.000, 0.000, 0.040, 0.0375)], e=2.1, mirror=mirror),
            b50.local("fist-" + side, "ellipsoid", "hand-" + side, GLOVE, 0.4, radii=[0.065, 0.048, 0.0375], transform={"translation": [0.060, 0.004, -sign * 0.006]}),
        ]
    cu, cv, cw = b50.BOX_C
    hu, hv, hw = b50.BOX_H
    out.append(b50.local("box", "box", "box-r", BOARD, 0.66, size=[2 * hu, 2 * hv, 2 * hw], transform={"translation": [cu, cv, cw]}))
    return out


def thermal(S, body):
    out = [k.ref("head", "head-skin"), k.ref("hair", "hair"), k.ref("helmet", "helmet"), k.ref("vest", "vest"), k.ref("seat", "leg-clothing"), k.ref("box", "box-surface")]
    out.append(k.ref("collar", "torso-clothing"))
    for side in ("l", "r"):
        out += [k.ref("sleeve-" + side, "arm-clothing"), k.ref("forearm-" + side, "arm-clothing"), k.ref("fist-" + side, "gloves")]
        out += [k.ref("thigh-" + side, "leg-clothing"), k.ref("shank-" + side, "leg-clothing"), k.ref("boot-" + side, "shoes"), k.ref("toe-" + side, "shoes")]
    out.append(k.th(k.ellipsoid(S, "th-breath-nose", "head", SKIN, 0.3, (0.100, 1.610, 0.0), (0.0375, 0.0375, 0.0375), role="detail"), "head-skin", tags=["emitter", "breath-nose"]))
    out.append(k.th(k.ellipsoid(S, "th-breath-mouth", "head", SKIN, 0.3, (0.082, 1.570, 0.0), (0.0375, 0.0375, 0.0375), role="detail"), "head-skin", tags=["emitter", "breath-mouth"]))
    out.append(k.th(k.ellipsoid(S, "th-plume", "head", HDPE, 0.92, (-0.010, 1.790, 0), (0.060, 0.0375, 0.050), role="detail"), "vest", tags=["emitter", "plume"]))
    return out


def structure(S, body):
    out = []
    add = lambda solid, member, part="member": out.append(k.st(solid, member, part))
    loc = lambda sid, kind, bone, member, part="member", **f: out.append(k.st(b50.local(sid, kind, bone, BONE, 0.85 if part == "member" else 0.5, **f), member, part))
    add(k.ellipsoid(S, "skull", "head", BONE, 0.85, (0.004, 1.670, 0), (0.090, 0.090, 0.065)), "head")
    add(k.capsule(S, "neck-bar", "head", BONE, 0.85, (-0.030, 1.530, 0), (-0.015, 1.610, 0), R, R), "head")
    add(k.capsule(S, "spine-bar", "torso", BONE, 0.85, (-0.045, 1.060, 0), (-0.035, 1.500, 0), R, R), "torso")
    add(k.capsule(S, "shoulder-girdle", "torso", BONE, 0.85, (-0.020, 1.465, -0.170), (-0.020, 1.465, 0.170), R, R), "torso")
    add(k.capsule(S, "pelvic-girdle", "pelvis", BONE, 0.85, (-0.010, 0.960, -0.090), (-0.010, 0.960, 0.090), R, R), "pelvis")
    add(k.capsule(S, "pelvis-bar", "pelvis", BONE, 0.85, (-0.010, 0.960, 0), (-0.045, 1.040, 0), R, R), "pelvis")
    for side, mirror, sign in SIDES:
        thigh, shank, foot, toe = "thigh-" + side, "shank-" + side, "foot-" + side, "toe-" + side
        arm, fore, hand = "upper-arm-" + side, "forearm-" + side, "hand-" + side
        add(k.capsule(S, "femur-" + side, thigh, BONE, 0.85, (0, 0.940, HZ), (0.004, 0.525, HZ), R, R, mirror=mirror), thigh)
        add(k.capsule(S, "tibia-" + side, shank, BONE, 0.85, (0.004, 0.515, HZ), (0.000, 0.100, HZ), R, R, mirror=mirror), shank)
        add(k.capsule(S, "foot-bar-" + side, foot, BONE, 0.85, (-0.035, 0.0575, HZ), (0.120, 0.040, HZ), R, R, mirror=mirror), foot)
        add(k.capsule(S, "toe-bar-" + side, toe, BONE, 0.85, (0.140, 0.040, HZ), (0.180, 0.038, HZ), R, R, mirror=mirror), toe)
        loc("humerus-" + side, "capsule", arm, arm, p0=[0.010, 0, 0], p1=[0.300, 0, 0], r0=R, r1=R)
        loc("forearm-bar-" + side, "capsule", fore, fore, p0=[0.010, 0, 0], p1=[0.255, 0, 0], r0=R, r1=R)
        loc("hand-bar-" + side, "ellipsoid", hand, hand, radii=[0.055, 0.040, 0.0375], transform={"translation": [0.055, 0.004, -sign * 0.006]})
    rig = {b["id"]: b for b in S.asset["rig"]["bones"]}
    m = rig["torso"]["mass"]
    loc("mass-torso", "ellipsoid", "torso", "torso", "mass", radii=[0.060, 0.060, 0.060], transform={"translation": [m["comX"], m["comY"], 0]})
    out.append({"id": "st-box-r", "of": "box", "member": "box-r", "part": "member"})
    out.append(k.st(b50.local("box-wall-l", "box", "box-r", BOARD, 0.66, size=[0.300, 0.300, 0.075], transform={"translation": [b50.BOX_C[0], b50.BOX_C[1], b50.BOX_C[2] + 0.1625]}), "box-l"))
    loc("mass-box", "ellipsoid", "box-r", "box-r", "mass", radii=[0.060, 0.060, 0.060], transform={"translation": list(b50.BOX_C)})
    return out


DECISIONS = [
    {"en": "Body 1:500, authored anew for a 75 mm feature: head with neck under the helmet dome (brim merged into the dome), one trunk mass in the vest, the shirt collar as a ring under the neck, a seat mass, two-part legs with boots and toes, two-part arms ending in fists, the carton as one block; the face, ears, peak, bands, gloves' cuffs and the hand holes are below the feature. The silhouette keeps what identifies the scene at a glance: helmet, broad vest, carton in front of the thighs on hanging arms.", "de": "Körper 1:500, neu modelliert für 75 mm Mindestmaß: Kopf mit Hals unter der Helmkuppel (Rand in die Kuppel verschmolzen), eine Rumpfmasse in der Weste, der Hemdkragen als Ring unter dem Hals, eine Gesäßmasse, zweiteilige Beine mit Stiefeln und Zehen, zweiteilige Arme mit Fäusten, der Karton als ein Block; Gesicht, Ohren, Schirm, Streifen, Stulpen und Grifflöcher liegen unter dem Mindestmaß. Die Silhouette behält, was die Szene auf einen Blick erkennbar macht: Helm, breite Weste, Karton vor den Oberschenkeln an hängenden Armen."},
    {"en": "Thermal 1:500: one surface per region; the shirt is the 75 mm collar ring of the body above the vest, the hair as a nape mass under the helmet rim; outlets of 75 mm at the nose, the mouth and on the helmet crown. Structure 1:500: a stick skeleton of 75 mm bars: skull, neck, spine, shoulder and pelvic girdle, one bar per limb segment, a hand mass, the carton block with its second wall for the left hand; joints are inside the bar ends at this scale.", "de": "Thermik 1:500: eine Fläche je Region; das Hemd ist der 75 mm Kragenring des Körpers über der Weste, das Haar als Nackenmasse unter dem Helmrand; Auslässe von 75 mm an Nase, Mund und auf dem Helmscheitel. Tragwerk 1:500: ein Stabskelett aus 75 mm Stäben: Schädel, Hals, Wirbelsäule, Schulter- und Beckengürtel, ein Stab je Gliedmaßenabschnitt, eine Handmasse, der Kartonblock mit seiner zweiten Wand für die linke Hand; die Gelenke liegen in diesem Maßstab in den Stabenden."},
]


def lists(asset):
    S = k.Space(k.pose_frames(asset))
    S.asset = asset
    b = body(S)
    return {"body": b, "thermal": thermal(S, b), "structure": structure(S, b), "decisions": DECISIONS}
