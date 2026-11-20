"""🦺 The three lists of the worker carrying a box at 1:200 (smallest feature 30 mm), stated anew: one mass per part, the helmet with its brim and peak as one shell, vest over the shirt collar, mitten gloves hooked into the carton, boots as shaft, foot and toe; a skeleton of 30 mm bones with three rib hoops per side and ball joints at the large joints."""
import r3_worker_carrying_box_body50 as b50
import r3_worker_carrying_box_kit as k
import r3_worker_carrying_box_s100 as s100

SKIN, HAIR, COTTON, VEST, BAND, HDPE, GLOVE, BOOT, BOARD = b50.SKIN, b50.HAIR, b50.COTTON, b50.VEST, b50.BAND, b50.HDPE, b50.GLOVE, b50.BOOT, b50.BOARD
TOP = b50.TOP
HZ = b50.HIP_Z
BONE = "skin"
SIDES = s100.SIDES
drop = s100.drop
V200 = [(1.030, -0.004, 0, 0.189, 0.141, 2.5), (1.250, -0.008, 0, 0.176, 0.140, 2.5), (1.400, -0.019, 0, 0.170, 0.124, 2.5), (1.470, -0.024, 0, 0.190, 0.090, 2.6), (1.492, -0.025, 0, 0.165, 0.075, 2.5)]


def body(S):
    out = [
        k.loft(S, "neck", "head", SKIN, 0.3, [(1.455, -0.031, 0, 0.073, 0.069), (1.605, -0.012, 0, 0.056, 0.058)], e=2.0),
        k.loft(S, "head", "head", SKIN, 0.3, drop([(0.010, -0.005, 0, 0.040, 0.050), (0.040, -0.004, 0, 0.070, 0.090), (0.090, 0.000, 0, 0.077, 0.100), (0.150, 0.012, 0, 0.068, 0.084), (0.205, 0.030, 0, 0.052, 0.060), (0.232, 0.048, 0, 0.024, 0.030)]), e=2.15, sub=1),
        k.loft(S, "hair", "head", HAIR, 0.75, drop([(0.080, -0.020, 0, 0.078, 0.086), (0.150, -0.042, 0, 0.066, 0.052)]), e=2.1),
        k.loft(S, "helmet", "head", HDPE, 0.92, [(1.815, -0.010, 0, 0.030, 0.036), (1.795, -0.010, 0, 0.074, 0.088), (1.750, -0.007, 0, 0.103, 0.123), (1.700, -0.005, 0, 0.111, 0.132), (1.672, -0.005, 0, 0.116, 0.137)], e=2.15, sub=1),
        k.loft(S, "helmet-peak", "head", HDPE, 0.92, [(0.100, 1.680, 0, 0.098, 0.015), (0.188, 1.675, 0, 0.040, 0.015)], axes="x", e=2.6),
        k.loft(S, "shirt-collar", "torso", COTTON, 0.5, [(1.440, -0.022, 0, 0.150, 0.090, 2.4), (1.500, -0.025, 0, 0.100, 0.068, 2.1), (1.536, -0.019, 0, 0.066, 0.068, 2.0)], e=2.2),
        k.loft(S, "vest", "torso", VEST, 0.9, V200, e=2.5),
        k.loft(S, "vest-hem", "pelvis", VEST, 0.9, [(0.930, -0.006, 0, 0.196, 0.141, 2.5), (1.075, -0.003, 0, 0.181, 0.140, 2.5)], e=2.5),
        k.loft(S, "trousers-seat", "pelvis", COTTON, 0.3, [(0.845, 0.000, 0, 0.140, 0.100, 2.3), (0.900, -0.012, 0, 0.190, 0.140, 2.5), (1.040, -0.005, 0, 0.172, 0.129, 2.5)], e=2.5),
    ]
    for name, lo, hi in (("vest-band-low", 1.055, 1.105), ("vest-band-high", 1.185, 1.235)):
        rows = [(y,) + (lambda c: (c[0], 0, c[1] + 0.003, c[2] + 0.003, 2.5))(b50.side_at(V200, y)) for y in (lo, hi)]
        out.append(k.loft(S, name, "torso", BAND, 0.6, rows, role="detail", texture=True))
    for side, mirror, sign in SIDES:
        out += [
            k.loft(S, "thigh-" + side, "thigh-" + side, COTTON, 0.3, [(0.985, -0.004, 0.094, 0.088, 0.106), (0.800, 0.006, 0.097, 0.090, 0.104), (0.520, 0.007, 0.090, 0.067, 0.073)], e=2.2, sub=1, mirror=mirror),
            k.loft(S, "shank-" + side, "shank-" + side, COTTON, 0.3, [(0.570, 0.008, 0.090, 0.068, 0.076), (0.420, -0.003, 0.090, 0.066, 0.071), (0.110, -0.003, 0.090, 0.062, 0.067)], e=2.2, sub=1, mirror=mirror),
            k.loft(S, "boot-shaft-" + side, "foot-" + side, BOOT, 0.12, [(0.176, -0.012, HZ, 0.048, 0.056), (0.050, -0.008, HZ, 0.048, 0.068)], e=2.2, mirror=mirror),
            k.loft(S, "boot-" + side, "foot-" + side, BOOT, 0.12, [(-0.084, 0.045, HZ, 0.040, 0.045, 3.0), (0.000, 0.050, HZ, 0.050, 0.050, 3.0), (0.138, 0.034, HZ + 0.002, 0.055, 0.034, 3.0)], axes="x", e=3.0, sub=1, mirror=mirror),
            k.loft(S, "toe-" + side, "toe-" + side, BOOT, 0.12, [(0.120, 0.044, HZ + 0.002, 0.055, 0.044, 2.6), (0.195, 0.034, HZ + 0.001, 0.046, 0.034, 2.5), (0.2215, 0.020, HZ + 0.001, 0.020, 0.018, 2.2)], axes="x", mirror=mirror),
            k.local_loft("sleeve-" + side, "upper-arm-" + side, COTTON, 0.5, [(-0.030, 0.000, -0.006, 0.030, 0.034), (0.010, -0.002, 0.006, 0.064, 0.060), (0.330, 0.000, 0.000, 0.048, 0.046)], e=2.1, sub=1, mirror=mirror),
            k.local_loft("forearm-" + side, "forearm-" + side, COTTON, 0.5, [(-0.034, 0.000, 0.000, 0.046, 0.045), (0.050, 0.004, 0.000, 0.051, 0.047), (0.220, 0.000, 0.000, 0.037, 0.033)], e=2.1, sub=1, mirror=mirror),
            k.local_loft("glove-cuff-" + side, "forearm-" + side, GLOVE, 0.4, [(0.200, 0.000, 0.000, 0.044, 0.040), (0.285, 0.000, 0.000, 0.036, 0.030)], e=2.1, mirror=mirror),
            k.local_loft("hand-" + side, "hand-" + side, GLOVE, 0.4, [(-0.012, 0.000, 0.000, 0.034, 0.025), (0.060, 0.004, 0.000, 0.048, 0.016), (0.090, 0.002, 0.000, 0.046, 0.016)], e=2.6, mirror=mirror),
            s100.curled(S, "fingers-" + side, "hand-" + side, GLOVE, 0.4, s100.finger_path(sign, (0.082, 0.004), (0.046, 0.030, 0.012)), 0.044, 0.015, sub=0),
        ]
    cu, cv, cw = b50.BOX_C
    hu, hv, hw = b50.BOX_H
    out.append(b50.local("box", "box", "box-r", BOARD, 0.66, size=[2 * hu, 2 * hv, 2 * hw], transform={"translation": [cu, cv, cw]}))
    return out


def thermal(S, body):
    region = {"head-skin": ["neck", "head"], "hair": ["hair"], "helmet": ["helmet", "helmet-peak"], "torso-clothing": ["shirt-collar"], "vest": ["vest", "vest-hem"], "box-surface": ["box"], "leg-clothing": ["trousers-seat"]}
    out = [k.ref(i, r) for r, ids in region.items() for i in ids]
    for side in ("l", "r"):
        out += [k.ref(i + side, "arm-clothing") for i in ("sleeve-", "forearm-")]
        out += [k.ref(i + side, "gloves") for i in ("glove-cuff-", "hand-", "fingers-")]
        out += [k.ref(i + side, "leg-clothing") for i in ("thigh-", "shank-")]
        out += [k.ref(i + side, "shoes") for i in ("boot-shaft-", "boot-", "toe-")]
    out.append(k.th(k.ellipsoid(S, "th-breath-nose", "head", SKIN, 0.3, (0.112, TOP - 0.168, 0), (0.015, 0.015, 0.016), role="detail"), "head-skin", tags=["emitter", "breath-nose"]))
    out.append(k.th(k.ellipsoid(S, "th-breath-mouth", "head", SKIN, 0.3, (0.092, TOP - 0.194, 0), (0.015, 0.015, 0.018), role="detail"), "head-skin", tags=["emitter", "breath-mouth"]))
    out.append(k.th(k.ellipsoid(S, "th-plume", "head", HDPE, 0.92, (-0.010, 1.802, 0), (0.070, 0.015, 0.060), role="detail"), "vest", tags=["emitter", "plume"]))
    return out


def structure(S, body):
    spine = s100.b50_spine()
    out = []
    add = lambda solid, member, part="member": out.append(k.st(solid, member, part))
    loc = lambda sid, kind, bone, member, part="member", **f: out.append(k.st(b50.local(sid, kind, bone, BONE, 0.85 if part == "member" else 0.6, **f), member, part))
    add(k.ellipsoid(S, "skull", "head", BONE, 0.85, (0.004, TOP - 0.095, 0), (0.098, 0.090, 0.070), tilt=-8), "head")
    add(k.free_loft(S, "mandible", "head", BONE, 0.85, [(-0.008, TOP - 0.150, 0.054, 0.015, 0.015), (0.060, TOP - 0.208, 0.020, 0.015, 0.015), (0.060, TOP - 0.208, -0.020, 0.015, 0.015), (-0.008, TOP - 0.150, -0.054, 0.015, 0.015)]), "head")
    add(k.capsule(S, "cervical-spine", "head", BONE, 0.85, (-0.030, 1.530, 0), (-0.015, 1.640, 0), 0.016, 0.015), "head")
    add(k.free_loft(S, "spine", "torso", BONE, 0.85, [(x, y, 0, 0.022, 0.018) for (x, y) in spine[::3]] + [(spine[-1][0], spine[-1][1], 0, 0.018, 0.016)], sub=1, across=k.Z), "torso")
    for n, (level, half, front, drop_) in enumerate(((16, 0.104, 0.064, 0.058), (13, 0.132, 0.092, 0.094), (10, 0.134, 0.088, 0.112))):
        for side, mirror, _ in SIDES:
            rib = s100.rib(S, side, mirror, level, half, front, drop_, n + 1)
            for s in rib["sections"]:
                s["rx"], s["ry"] = 0.015, 0.015
            out.append(rib)
    add(k.free_loft(S, "sternum", "torso", BONE, 0.85, [(0.050, 1.460, 0, 0.026, 0.015), (0.095, 1.275, 0, 0.016, 0.015)], e=2.5, across=k.Z), "torso")
    add(k.capsule(S, "shoulder-girdle", "torso", BONE, 0.85, (-0.030, 1.480, -0.190), (-0.030, 1.480, 0.190), 0.016, 0.016), "torso")
    add(k.loft(S, "sacrum", "pelvis", BONE, 0.85, [(1.046, -0.052, 0, 0.056, 0.026), (0.900, -0.088, 0, 0.018, 0.015)], e=2.2), "pelvis")
    add(k.capsule(S, "pelvic-girdle", "pelvis", BONE, 0.85, (0.040, 0.925, -0.092), (0.040, 0.925, 0.092), 0.016, 0.016), "pelvis")
    for side, mirror, sign in SIDES:
        thigh, shank, foot, toe = "thigh-" + side, "shank-" + side, "foot-" + side, "toe-" + side
        arm, fore, hand = "upper-arm-" + side, "forearm-" + side, "hand-" + side
        add(k.ellipsoid(S, "scapula-" + side, "torso", BONE, 0.85, (-0.098, 1.405, 0.112), (0.015, 0.075, 0.050), mirror=mirror, tilt=8, yaw=-28), "torso")
        add(k.ellipsoid(S, "ilium-" + side, "pelvis", BONE, 0.85, (-0.020, 1.010, 0.110), (0.070, 0.064, 0.015), mirror=mirror, tilt=-12, yaw=38), "pelvis")
        add(k.capsule(S, "femur-" + side, thigh, BONE, 0.85, (-0.004, 0.912, HZ + 0.040), (0.004, 0.540, HZ), 0.017, 0.022, mirror=mirror), thigh)
        add(k.capsule(S, "tibia-" + side, shank, BONE, 0.85, (0.002, 0.505, HZ), (0.000, 0.105, HZ), 0.022, 0.017, mirror=mirror), shank)
        add(k.free_loft(S, "foot-bones-" + side, foot, BONE, 0.85, [(-0.050, 0.055, HZ, 0.020, 0.018), (0.010, 0.075, HZ, 0.026, 0.020), (0.130, 0.036, HZ, 0.040, 0.015)], mirror=mirror, across=k.Z), foot)
        add(k.free_loft(S, "toe-bones-" + side, toe, BONE, 0.85, [(0.134, 0.034, HZ, 0.040, 0.015), (0.190, 0.030, HZ - 0.004, 0.024, 0.015)], mirror=mirror, across=k.Z), toe)
        loc("humerus-" + side, "capsule", arm, arm, p0=[0.010, 0, 0.004], p1=[0.300, 0, 0], r0=0.017, r1=0.016)
        loc("forearm-bones-" + side, "loft", fore, fore, sections=[{"at": [-0.015, 0, 0], "rx": 0.017, "ry": 0.022, "exponent": 2.0, "rotation": k.quaternion((0, 1, 0), (0, 0, 1), (1, 0, 0))}, {"at": [0.265, 0, 0], "rx": 0.016, "ry": 0.024, "exponent": 2.0, "rotation": k.quaternion((0, 1, 0), (0, 0, 1), (1, 0, 0))}], capStart=True, capEnd=True)
        loc("hand-bones-" + side, "ellipsoid", hand, hand, radii=[0.050, 0.032, 0.015], transform={"translation": [0.050, 0.004, 0]})
        joint = lambda sid, bone, world, r: out.append(k.st(k.ellipsoid(S, sid, bone, BONE, 0.6, world, (r, r, r), mirror=mirror), bone, "joint"))
        joint("joint-hip-" + side, thigh, (0, 0.9406, HZ), 0.030)
        joint("joint-knee-" + side, shank, (0.002, 0.5206, HZ), 0.030)
        loc("joint-shoulder-" + side, "ellipsoid", arm, arm, "joint", radii=[0.030, 0.030, 0.030])
        loc("joint-elbow-" + side, "ellipsoid", fore, fore, "joint", radii=[0.024, 0.024, 0.026])
    rig = {b["id"]: b for b in S.asset["rig"]["bones"]}
    m = rig["torso"]["mass"]
    out.append(k.st(b50.local("mass-torso", "ellipsoid", "torso", BONE, 0.5, radii=[0.050, 0.050, 0.050], transform={"translation": [m["comX"], m["comY"], 0]}), "torso", "mass"))
    out.append({"id": "st-box-r", "of": "box", "member": "box-r", "part": "member"})
    out.append(k.st(b50.local("box-wall-l", "box", "box-r", BOARD, 0.66, size=[0.300, 0.300, 0.030], transform={"translation": [b50.BOX_C[0], b50.BOX_C[1], b50.BOX_C[2] + 0.185]}), "box-l"))
    out.append(k.st(b50.local("mass-box", "ellipsoid", "box-r", BOARD, 0.5, radii=[0.040, 0.040, 0.040], transform={"translation": list(b50.BOX_C)}), "box-r", "mass"))
    return out


DECISIONS = [
    {"en": "Body 1:200, authored anew for a 30 mm feature: one mass per part. The head is one skull loft without nose and ears; the helmet is one shell with its rim and a 30 mm peak; the shirt shows only as the collar above the vest; the vest has its hem on the pelvis and the two reflective bands as surface texture (the braces are below the feature); trouser seat, thigh and shank per leg without knee mass; each boot is shaft, foot with sole and toe; each arm is sleeve, forearm, glove cuff and a mitten whose finger block hooks over the hand hole; the carton is one closed 0.40 x 0.30 x 0.30 m box (hand holes and tape below the feature).", "de": "Körper 1:200, neu modelliert für 30 mm Mindestmaß: eine Masse je Teil. Der Kopf ist ein Schädel-Loft ohne Nase und Ohren; der Helm ist eine Schale mit Rand und 30 mm Schirm; das Hemd zeigt sich nur als Kragen über der Weste; die Weste hat ihren Saum am Becken und die zwei Reflexstreifen als Oberflächentextur (die Träger liegen unter dem Mindestmaß); Hosengesäß, Oberschenkel und Unterschenkel je Bein ohne Kniemasse; jeder Stiefel ist Schaft, Fuß mit Sohle und Zehe; jeder Arm ist Ärmel, Unterarm, Handschuhstulpe und ein Fäustling, dessen Fingerblock über das Griffloch greift; der Karton ist ein geschlossener Quader 0,40 x 0,30 x 0,30 m (Grifflöcher und Klebeband unter dem Mindestmaß)."},
    {"en": "Thermal 1:200: one surface per region by reference to its body solids, layers merged; breath and plume outlets of 30 mm. Structure 1:200: a skeleton of 30 mm bones: skull, mandible, cervical and thoracolumbar spine, three rib hoops per side, sternum, one bar for the shoulder girdle with the scapulae, sacrum with the hip bones and one bar for the pubic arch, femur, tibia with the fibula merged, foot and toe bones, humerus, one forearm bone pair, one hand; ball joints at hip, knee, shoulder and elbow; the carton block carries the load into both hands.", "de": "Thermik 1:200: eine Fläche je Region durch Verweis auf ihre Körperteile, Schichten zusammengefasst; Atem- und Fahnenauslässe von 30 mm. Tragwerk 1:200: ein Skelett aus 30 mm starken Knochen: Schädel, Unterkiefer, Hals- und Brust-Lenden-Wirbelsäule, drei Rippenbögen je Seite, Brustbein, ein Stab für den Schultergürtel mit den Schulterblättern, Kreuzbein mit den Hüftbeinen und ein Stab für den Schambogen, Oberschenkelknochen, Schienbein mit eingeschlossenem Wadenbein, Fuß- und Zehenknochen, Oberarmknochen, ein Unterarmknochenpaar, eine Hand; Kugelgelenke an Hüfte, Knie, Schulter und Ellbogen; der Kartonblock leitet die Last in beide Hände."},
]


def lists(asset):
    S = k.Space(k.pose_frames(asset))
    S.asset = asset
    b = body(S)
    return {"body": b, "thermal": thermal(S, b), "structure": structure(S, b), "decisions": DECISIONS}
