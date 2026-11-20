"""🧍 The standing adult at 1:500 (smallest feature 75 mm), authored anew: the fewest masses that still read as a standing person in a sweatshirt at a glance; the thermal model on them; the skeleton as a wire skeleton of slender bones at their real thickness with the S-curved spine behind the trunk axis.

World of the rest pose: x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder 1.3975, C7 1.465, head joint 1.561, vertex 1.710.
"""
from r3_standing_adult_kit import capsule, ellipsoid, face_rows, free_loft, loft, ref, rib_rows, sphere, st, th

SKIN, HAIR, FLEECE, DENIM, CANVAS = "skin", "hair", "fleece-sweatshirt", "denim", "canvas-sneaker"
BONE, MASS = 0.85, 0.2
SH, HIP = 0.153, 0.085
SIDES = (("l", False), ("r", True))
HEAD = [(1.710, 0.020, 0, 0.022, 0.029), (1.680, 0.019, 0, 0.064, 0.084), (1.620, 0.022, 0, 0.072, 0.094), (1.545, 0.038, 0, 0.059, 0.072), (1.490, 0.058, 0, 0.024, 0.032)]
HAIR_EDGE = [(1.660, 0.094), (1.620, 0.056), (1.580, -0.004), (1.550, -0.026), (1.530, -0.1), (1.491, -0.1)]


def body():
    hair = [(1.720, 0.014, 0, 0.024, 0.031), (1.695, 0.009, 0, 0.068, 0.086), (1.670, 0.012, 0, 0.0755, 0.099), (1.661, -0.004, 0, 0.077, 0.084), (1.640, -0.004, 0, 0.078, 0.086), (1.585, -0.038, 0, 0.068, 0.044), (1.548, -0.042, 0, 0.042, 0.016)]
    neck = [(1.415, -0.006, 0, 0.066, 0.062), (1.566, 0.016, 0, 0.049, 0.047)]
    hoodie = [(0.868, -0.017, 0, 0.177, 0.120, 2.4), (0.960, -0.010, 0, 0.181, 0.121, 2.4), (1.120, -0.002, 0, 0.161, 0.116, 2.4), (1.300, -0.014, 0, 0.168, 0.132, 2.4), (1.408, -0.040, 0, 0.188, 0.118, 2.6), (1.440, -0.036, 0, 0.176, 0.094, 2.6), (1.474, -0.004, 0, 0.076, 0.072, 2.0)]
    seat = [(1.010, 0.004, 0, 0.146, 0.096, 2.3), (0.890, -0.014, 0, 0.168, 0.106, 2.3), (0.800, -0.004, 0, 0.070, 0.048, 2.0)]
    out = [
        loft("head", "head", SKIN, 0.25, HEAD, "y", e=2.15, sub=1),
        loft("hair", "head", HAIR, 0.75, hair, "y", e=2.1, sub=1),
        loft("neck", "neck", SKIN, 0.25, neck, "y"),
        loft("hoodie", "torso", FLEECE, 0.45, hoodie, "y", sub=1),
        loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        sleeve = [(1.436, -0.008, SH + 0.006, 0.026, 0.030), (1.380, -0.006, SH + 0.016, 0.060, 0.066), (1.220, -0.002, SH + 0.007, 0.052, 0.057), (1.084, 0.000, SH, 0.0465, 0.0495), (1.079, 0.000, SH, 0.040, 0.043), (1.056, 0.000, SH, 0.0375, 0.0375)]
        forearm = [(1.112, 0.000, SH, 0.0375, 0.0375), (1.089, 0.000, SH, 0.040, 0.043), (1.084, 0.000, SH, 0.045, 0.048), (1.040, 0.003, SH, 0.0455, 0.0495), (0.900, 0.001, SH, 0.038, 0.042), (0.836, 0.000, SH, 0.028, 0.033)]
        hand = [(0.848, 0.000, SH, 0.020, 0.027, 2.3), (0.770, 0.007, SH, 0.0215, 0.049, 2.7), (0.666, 0.000, SH, 0.016, 0.024, 2.6)]
        thigh = [(0.972, -0.010, HIP + 0.003, 0.040, 0.045), (0.900, -0.010, HIP + 0.003, 0.083, 0.093), (0.740, -0.004, HIP + 0.003, 0.077, 0.086), (0.560, 0.004, HIP - 0.002, 0.058, 0.064), (0.4898, 0.0035, HIP - 0.002, 0.052, 0.057), (0.485, 0.0035, HIP - 0.002, 0.0455, 0.0505), (0.450, 0.004, HIP - 0.002, 0.040, 0.044)]
        shank = [(0.528, 0.003, HIP - 0.002, 0.0375, 0.0375), (0.4946, 0.0035, HIP - 0.002, 0.0455, 0.0505), (0.4898, 0.0035, HIP - 0.002, 0.0505, 0.0555), (0.440, 0.000, HIP - 0.002, 0.051, 0.056), (0.350, -0.010, HIP, 0.055, 0.062), (0.108, -0.004, HIP, 0.042, 0.047)]
        shoe = [(-0.0745, 0.034, HIP, 0.022, 0.034, 3.0), (-0.030, 0.045, HIP, 0.035, 0.045, 2.8), (0.040, 0.045, HIP + 0.001, 0.039, 0.045, 2.6), (0.157, 0.027, HIP + 0.003, 0.050, 0.027, 3.0)]
        toe = [(0.143, 0.0275, HIP + 0.003, 0.050, 0.0275, 3.0), (0.185, 0.025, HIP + 0.002, 0.041, 0.025, 3.0), (0.205, 0.021, HIP, 0.020, 0.019, 2.4)]
        out += [
            loft("sleeve" + s, "arm" + s, FLEECE, 0.45, sleeve, "y", sub=1, mirror=mirror),
            loft("forearm" + s, "forearm" + s, FLEECE, 0.45, forearm, "y", sub=1, mirror=mirror),
            loft("hand" + s, "hand" + s, SKIN, 0.3, hand, "y", mirror=mirror),
            loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=1, mirror=mirror),
            loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=1, mirror=mirror),
            loft("shoe" + s, "foot" + s, CANVAS, 0.12, shoe, "x", mirror=mirror),
            loft("shoe-toe" + s, "toe" + s, CANVAS, 0.12, toe, "x", mirror=mirror),
        ]
    return out


def thermal():
    out = [th(loft("th-face", "head", SKIN, 0.25, face_rows(HEAD, HAIR_EDGE), "y"), "face-skin"), ref("neck", "face-skin"), ref("hair", "hair")]
    out += [ref(i, "hoodie", "sweatshirt") for i in ("hoodie", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r")]
    out += [ref(i, "jeans", "jeans") for i in ("jeans-seat", "thigh-l", "thigh-r", "shank-l", "shank-r")]
    out += [ref(i, "sneakers", "sneaker") for i in ("shoe-l", "shoe-r", "shoe-toe-l", "shoe-toe-r")]
    out += [ref(i, "hand-skin") for i in ("hand-l", "hand-r")]
    out.append(th(ellipsoid("th-nose-breath", "head", SKIN, 0.1, (0.084, 1.570, 0), (0.0375, 0.0375, 0.0375), role="detail"), "face-skin", tags=["emitter", "nose-breath"]))
    out.append(th(ellipsoid("th-mouth-breath", "head", SKIN, 0.1, (0.072, 1.530, 0), (0.0375, 0.0375, 0.0375), role="detail"), "face-skin", tags=["emitter", "mouth-breath"]))
    out.append(th(ellipsoid("th-body-plume", "head", HAIR, 0.5, (0.010, 1.682, 0), (0.060, 0.0375, 0.050), role="detail"), "hoodie", tags=["emitter", "body-plume"]))
    return out


def wire(solid):
    """🧵 Marks a bone as slender (C2): at its real thickness, never thickened to the smallest feature."""
    return dict(solid, slender=True)


def structure():
    skull = [(1.702, 0.018, 0, 0.018, 0.024), (1.680, 0.017, 0, 0.056, 0.074), (1.630, 0.016, 0, 0.066, 0.088), (1.585, 0.026, 0, 0.060, 0.082), (1.545, 0.058, 0, 0.042, 0.044), (1.500, 0.074, 0, 0.038, 0.038)]
    spine = [(-0.036, 1.000, 0, 0.011, 0.011), (-0.016, 1.090, 0, 0.011, 0.011), (-0.028, 1.180, 0, 0.011, 0.011), (-0.052, 1.300, 0, 0.011, 0.011), (-0.042, 1.400, 0, 0.011, 0.011), (-0.020, 1.465, 0, 0.011, 0.011)]
    cervical = [(-0.020, 1.462, 0, 0.0025, 0.0025), (-0.004, 1.510, 0, 0.0025, 0.0025), (0.0125, 1.562, 0, 0.0025, 0.0025)]
    sacrum = [(-0.036, 0.999, 0, 0.0026, 0.0026), (-0.060, 0.950, 0, 0.0026, 0.0026), (-0.074, 0.900, 0, 0.0026, 0.0026)]
    out = [
        st(loft("skull", "head", SKIN, BONE, skull, "y", sub=1), "head"),
        st(wire(free_loft("cervical-spine", "neck", SKIN, BONE, cervical)), "neck"),
        st(wire(free_loft("spine", "torso", SKIN, BONE, spine, sub=2)), "torso"),
        st(wire(capsule("sternum", "torso", SKIN, BONE, (0.050, 1.396, 0), (0.084, 1.226, 0), 0.004, 0.004)), "torso"),
        st(wire(free_loft("sacrum", "pelvis", SKIN, BONE, sacrum)), "pelvis"),
    ]
    ribs = [(1.405, 0.052, 0.084, -0.040, 0.058, 0.0, 0.005), (1.320, 0.074, 0.110, -0.053, 0.078, 0.0, 0.007), (1.225, 0.066, 0.110, -0.036, 0.074, 0.85, 0.0045)]
    for side, mirror in SIDES:
        s = "-" + side
        for k, (y, drop, w, sx, front, to, r) in enumerate(ribs):
            out.append(st(wire(free_loft("rib%s-%d" % (s, k + 1), "torso", SKIN, BONE, rib_rows(y, drop, w, sx, front, r, to=to, steps=6), mirror=mirror)), "torso"))
        hip_bone = [(-0.050, 0.985, 0.034, 0.008, 0.008), (-0.052, 1.034, 0.084, 0.008, 0.008), (-0.004, 1.040, 0.134, 0.008, 0.008), (0.044, 0.992, 0.120, 0.008, 0.008), (0.026, 0.915, 0.096, 0.008, 0.008), (0.050, 0.876, 0.014, 0.008, 0.008)]
        girdle = [(0.046, 1.404, 0.028, 0.0065, 0.0065), (0.020, 1.418, 0.110, 0.0065, 0.0065), (-0.020, 1.424, 0.160, 0.0065, 0.0065), (-0.070, 1.400, 0.110, 0.0065, 0.0065), (-0.092, 1.320, 0.098, 0.0065, 0.0065)]
        out += [
            st(wire(free_loft("hip-bone" + s, "pelvis", SKIN, BONE, hip_bone, sub=1, mirror=mirror)), "pelvis"),
            st(wire(free_loft("shoulder-girdle" + s, "torso", SKIN, BONE, girdle, mirror=mirror)), "torso"),
            st(wire(capsule("femur" + s, "thigh" + s, SKIN, BONE, (-0.002, 0.903, HIP + 0.004), (0.003, 0.492, HIP), 0.009, 0.009, mirror=mirror)), "thigh" + s),
            st(wire(capsule("tibia" + s, "shank" + s, SKIN, BONE, (0.001, 0.488, HIP - 0.001), (0.000, 0.090, HIP - 0.004), 0.009, 0.009, mirror=mirror)), "shank" + s),
            st(wire(capsule("foot-bones" + s, "foot" + s, SKIN, BONE, (-0.004, 0.086, HIP), (0.150, 0.030, HIP + 0.003), 0.0038, 0.0038, mirror=mirror)), "foot" + s),
            st(wire(capsule("toe-bones" + s, "toe" + s, SKIN, BONE, (0.150, 0.030, HIP + 0.003), (0.200, 0.022, HIP + 0.001), 0.0012, 0.0012, mirror=mirror)), "toe" + s),
            st(wire(capsule("humerus" + s, "arm" + s, SKIN, BONE, (-0.001, 1.3975, SH), (0.000, 1.086, SH), 0.0072, 0.0072, mirror=mirror)), "arm" + s),
            st(wire(capsule("forearm-bones" + s, "forearm" + s, SKIN, BONE, (-0.002, 1.084, SH), (-0.002, 0.842, SH), 0.0056, 0.0056, mirror=mirror)), "forearm" + s),
            st(wire(capsule("hand-bones" + s, "hand" + s, SKIN, BONE, (0.000, 0.840, SH), (0.002, 0.676, SH), 0.0038, 0.0038, mirror=mirror)), "hand" + s),
            st(sphere("mass-thigh" + s, "thigh" + s, SKIN, MASS, (0, 0.7537, HIP), 0.042, role="detail", mirror=mirror), "thigh" + s, "mass"),
        ]
    out += [
        st(sphere("mass-pelvis", "pelvis", SKIN, MASS, (0, 0.9601, 0), 0.045, role="detail"), "pelvis", "mass"),
        st(sphere("mass-torso", "torso", SKIN, MASS, (0, 1.2024, 0), 0.052, role="detail"), "torso", "mass"),
    ]
    return out


def decisions():
    return [
        {
            "en": "Body at 1:500, the fewest masses that read as a standing person: head, hair cap, neck, sweatshirt with the hood merged into the upper back, jeans seat, and per side sleeve, forearm, mitten hand, thigh, shank, shoe and toe box. Each mass keeps the true breadth and depth of its part, so shoulders, waist, hips and the taper of the limbs survive.",
            "de": "Körper bei 1:500, die wenigsten Massen, die als stehende Person lesbar sind: Kopf, Haarkappe, Hals, Sweatshirt mit im oberen Rücken aufgegangener Kapuze, Jeansgesäß und je Seite Ärmel, Unterarm, Fäustlingshand, Oberschenkel, Unterschenkel, Schuh und Zehenkappe. Jede Masse behält die wahre Breite und Tiefe ihres Teils, sodass Schultern, Taille, Hüfte und die Verjüngung der Glieder erhalten bleiben.",
        },
        {
            "en": "Thermal at 1:500: every region lies on the masses of this scale, layers merged; the face is the head ahead of the hair line; the three outlets are patches of one smallest feature.",
            "de": "Thermik bei 1:500: Jede Region liegt auf den Massen dieses Maßstabs, die Schichten gehen auf; das Gesicht ist der Kopf vor der Haarlinie; die drei Auslässe sind Flecken von einem kleinsten Merkmal.",
        },
        {
            "en": "Structure at 1:500: the skeleton as slender bones at their real thickness (C2), not thickened to the 75 mm feature: skull, cervical spine, the thoracolumbar spine with lumbar lordosis and thoracic kyphosis running behind the trunk axis under the back, sacrum, three rib hoops per side from the spine round to the sternum, sternum, clavicle and scapula as one girdle line per side, each hip bone as a line from the sacroiliac joint over the iliac crest and the acetabulum to the pubis, femur, tibia, foot, toes, humerus, forearm and hand; masses of pelvis, trunk and thighs. Joints are not shown; they are smaller than the feature.",
            "de": "Tragwerk bei 1:500: das Skelett als schlanke Knochen in wahrer Dicke (C2), nicht auf das 75-mm-Merkmal verdickt: Schädel, Halswirbelsäule, die Brust- und Lendenwirbelsäule mit Lendenlordose und Brustkyphose hinter der Rumpfachse unter dem Rücken, Kreuzbein, drei Rippenbögen je Seite von der Wirbelsäule bis zum Brustbein, Brustbein, Schlüsselbein und Schulterblatt als eine Gürtellinie je Seite, jedes Hüftbein als Linie vom Iliosakralgelenk über den Beckenkamm und die Hüftpfanne zum Schambein, Oberschenkelknochen, Schienbein, Fuß, Zehen, Oberarmknochen, Unterarm und Hand; Massen von Becken, Rumpf und Oberschenkeln. Gelenke sind nicht dargestellt, sie sind kleiner als das Merkmal.",
        },
    ]
