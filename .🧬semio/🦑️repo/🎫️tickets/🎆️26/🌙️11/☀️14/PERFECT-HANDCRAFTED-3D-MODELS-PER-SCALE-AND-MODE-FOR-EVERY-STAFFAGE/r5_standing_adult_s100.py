"""🧍 Round-5 standing adult at 1:100 (smallest feature 15 mm): the body stated anew against the remodelled 1:50 figure (drop-shoulder sweatshirt, hair hugging the skull, tapered jeans with calf, sneaker with toe spring), with fewer and longer sections; thermal and structure as in round 3 (`r3_standing_adult_s100`), whose ids this body keeps.

World of the rest pose: x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder 1.3975, C7 1.465, head joint 1.561, vertex 1.710.
"""
import r3_standing_adult_s100 as base
from r3_standing_adult_kit import capsule, ellipsoid, loft

SKIN, HAIR, FLEECE, DENIM, CANVAS = base.SKIN, base.HAIR, base.FLEECE, base.DENIM, base.CANVAS
SH, HIP, SIDES, HEAD = base.SH, base.HIP, base.SIDES, base.HEAD
thermal, structure = base.thermal, base.structure


def body():
    nose = [(1.606, 0.110, 0, 0.0085, 0.008), (1.580, 0.117, 0, 0.012, 0.013), (1.560, 0.121, 0, 0.0155, 0.017), (1.549, 0.114, 0, 0.012, 0.009)]
    hair = [(1.7225, 0.019, 0, 0.018, 0.022), (1.714, 0.019, 0, 0.048, 0.062), (1.690, 0.017, 0, 0.0665, 0.086), (1.668, 0.012, 0, 0.0760, 0.092), (1.640, -0.011, 0, 0.0781, 0.075), (1.602, -0.036, 0, 0.0652, 0.042), (1.570, -0.042, 0, 0.0420, 0.018)]
    neck = [(1.415, -0.012, 0, 0.060, 0.052), (1.465, 0.001, 0, 0.055, 0.054), (1.520, 0.011, 0, 0.052, 0.054), (1.570, 0.016, 0, 0.047, 0.045)]
    hoodie = [
        (0.868, -0.017, 0, 0.170, 0.116, 2.4), (0.880, -0.016, 0, 0.176, 0.120, 2.4), (0.940, -0.012, 0, 0.180, 0.122, 2.4), (1.060, -0.005, 0, 0.163, 0.115, 2.4), (1.200, -0.001, 0, 0.161, 0.119, 2.4),
        (1.290, 0.000, 0, 0.166, 0.121, 2.4), (1.370, -0.010, 0, 0.192, 0.106, 2.7), (1.400, -0.014, 0, 0.203, 0.093, 2.9), (1.425, -0.016, 0, 0.190, 0.078, 2.7), (1.445, -0.014, 0, 0.148, 0.068, 2.4),
        (1.462, -0.010, 0, 0.098, 0.066, 2.1), (1.476, -0.004, 0, 0.074, 0.070, 2.0),
    ]
    hood = [(1.486, -0.068, 0, 0.080, 0.020, 2.2), (1.462, -0.090, 0, 0.106, 0.040, 2.2), (1.410, -0.120, 0, 0.108, 0.040, 2.2), (1.350, -0.131, 0, 0.088, 0.029, 2.2), (1.300, -0.128, 0, 0.048, 0.016, 2.2)]
    seat = [(0.995, 0.001, 0, 0.150, 0.100, 2.3), (0.950, -0.006, 0, 0.166, 0.108, 2.4), (0.890, -0.013, 0, 0.170, 0.108, 2.3), (0.845, -0.016, 0, 0.142, 0.088, 2.2), (0.812, -0.014, 0, 0.082, 0.058, 2.0), (0.800, -0.012, 0, 0.040, 0.034, 2.0)]
    out = [
        loft("head", "head", SKIN, 0.25, HEAD, "y", e=2.15, sub=1),
        loft("nose", "head", SKIN, 0.25, nose, "y", role="detail"),
        loft("hair", "head", HAIR, 0.75, hair, "y", e=2.1, sub=1),
        loft("neck", "neck", SKIN, 0.25, neck, "y"),
        loft("hoodie", "torso", FLEECE, 0.45, hoodie, "y", sub=1),
        loft("hood", "torso", FLEECE, 0.4, hood, "y", sub=1),
        loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        sleeve = [(1.415, -0.010, SH + 0.018, 0.034, 0.040), (1.400, -0.009, SH + 0.020, 0.052, 0.058), (1.360, -0.007, SH + 0.020, 0.061, 0.066), (1.250, -0.003, SH + 0.012, 0.057, 0.062), (1.100, 0.001, SH + 0.003, 0.051, 0.056), (1.060, 0.001, SH + 0.001, 0.046, 0.051), (1.042, 0.000, SH, 0.024, 0.028)]
        forearm = [(1.118, 0.000, SH, 0.026, 0.030), (1.100, 0.001, SH, 0.046, 0.050), (1.000, 0.004, SH, 0.046, 0.050), (0.902, 0.001, SH, 0.045, 0.049), (0.884, 0.000, SH, 0.034, 0.039), (0.850, 0.000, SH, 0.0275, 0.033), (0.836, 0.000, SH, 0.025, 0.030)]
        hand = [(0.852, -0.002, SH, 0.017, 0.026, 2.2), (0.810, 0.000, SH - 0.001, 0.015, 0.036, 2.4), (0.762, 0.000, SH - 0.003, 0.0142, 0.0405, 2.5), (0.720, 0.000, SH - 0.009, 0.0125, 0.038, 2.5), (0.684, 0.000, SH - 0.017, 0.0100, 0.032, 2.4), (0.668, 0.000, SH - 0.022, 0.0075, 0.020, 2.2)]
        thigh = [(0.985, -0.012, 0.093, 0.040, 0.050), (0.940, -0.012, 0.093, 0.078, 0.092), (0.860, -0.008, 0.091, 0.083, 0.092), (0.720, 0.001, 0.088, 0.074, 0.082), (0.580, 0.005, 0.085, 0.062, 0.067), (0.500, 0.008, 0.084, 0.054, 0.058), (0.465, 0.006, 0.084, 0.049, 0.054), (0.444, 0.003, 0.084, 0.024, 0.028)]
        shank = [(0.540, 0.004, HIP, 0.030, 0.034), (0.520, 0.006, HIP, 0.050, 0.055), (0.450, 0.001, HIP, 0.051, 0.057), (0.370, -0.008, HIP, 0.053, 0.062), (0.270, -0.006, HIP, 0.047, 0.053), (0.140, -0.003, HIP, 0.045, 0.049), (0.105, -0.003, HIP, 0.045, 0.049)]
        shoe = [(-0.0745, 0.032, HIP, 0.018, 0.030, 3.0), (-0.064, 0.042, HIP, 0.030, 0.042, 3.0), (-0.030, 0.043, HIP, 0.034, 0.043, 2.8), (0.020, 0.044, HIP, 0.037, 0.044, 2.6), (0.070, 0.037, HIP + 0.001, 0.042, 0.037, 2.8), (0.120, 0.030, HIP + 0.003, 0.048, 0.030, 3.0), (0.157, 0.026, HIP + 0.003, 0.0495, 0.026, 3.0)]
        toe = [(0.143, 0.027, HIP + 0.003, 0.050, 0.027, 3.0), (0.172, 0.028, HIP + 0.002, 0.046, 0.025, 3.0), (0.192, 0.029, HIP + 0.001, 0.036, 0.021, 2.8), (0.205, 0.030, HIP, 0.014, 0.012, 2.4)]
        out += [
            ellipsoid("ear" + s, "head", SKIN, 0.3, (-0.003, 1.576, 0.0690), (0.015, 0.029, 0.010), role="detail", mirror=mirror, tilt=-12),
            loft("sleeve" + s, "arm" + s, FLEECE, 0.45, sleeve, "y", sub=1, mirror=mirror),
            loft("forearm" + s, "forearm" + s, FLEECE, 0.45, forearm, "y", sub=1, mirror=mirror),
            loft("hand" + s, "hand" + s, SKIN, 0.3, hand, "y", mirror=mirror),
            capsule("thumb" + s, "hand" + s, SKIN, 0.3, (0.020, 0.816, SH - 0.008), (0.044, 0.731, SH - 0.022), 0.0115, 0.0080, mirror=mirror),
            loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=1, mirror=mirror),
            loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=1, mirror=mirror),
            loft("shoe" + s, "foot" + s, CANVAS, 0.12, shoe, "x", sub=1, mirror=mirror),
            loft("shoe-toe" + s, "toe" + s, CANVAS, 0.12, toe, "x", mirror=mirror),
        ]
    return out


def decisions():
    out = base.decisions()
    out[0] = {
        "en": "Body at 1:100, drawn anew with fewer and longer sections: head with nose and ears under short hair that hugs the skull, without eyes, brows and lips; neck; the drop-shoulder sweatshirt as one form in which hem rib and collar rib are merged, the hood on the upper back without its rolled edge, no pocket and no drawstrings; sleeves set in below the shoulder line, forearms with the cuff merged; a mitten hand curling towards the thigh with a thumb along it instead of fingers; jeans seat with the gluteal mass merged, thighs tapering to the knee, shanks with the calf; one sneaker form per bone with the sole and the toe spring merged, no socks and no laces.",
        "de": "Körper bei 1:100, neu gezeichnet mit weniger und längeren Schnitten: Kopf mit Nase und Ohren unter kurzem, am Schädel anliegendem Haar, ohne Augen, Brauen und Lippen; Hals; das Sweatshirt mit überschnittener Schulter als eine Form, in der Saum- und Kragenbund aufgehen, die Kapuze auf dem oberen Rücken ohne Rollrand, ohne Tasche und Kordeln; Ärmel unter der Schulterlinie eingesetzt, Unterarme mit aufgegangenem Bündchen; eine zum Oberschenkel gekrümmte Fäustlingshand mit anliegendem Daumen statt Fingern; Jeansgesäß mit aufgegangener Gesäßmasse, zum Knie verjüngte Oberschenkel, Unterschenkel mit Wade; eine Sneakerform je Knochen mit aufgegangener Sohle und Zehensprengung, ohne Socken und Schnürung.",
    }
    return out
