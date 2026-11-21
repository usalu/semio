"""🧍 Round-5 standing adult at 1:200 (smallest feature 30 mm): one mass per part stated anew against the remodelled 1:50 figure (drop-shoulder sweatshirt, hair hugging the skull, tapered jeans, sneaker with toe spring); thermal and structure as in round 3 (`r3_standing_adult_s200`), whose ids this body keeps.

World of the rest pose: x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder 1.3975, C7 1.465, head joint 1.561, vertex 1.710.
"""
import r3_standing_adult_s200 as base
from r3_standing_adult_kit import loft

SKIN, HAIR, FLEECE, DENIM, CANVAS = base.SKIN, base.HAIR, base.FLEECE, base.DENIM, base.CANVAS
SH, HIP, SIDES, HEAD = base.SH, base.HIP, base.SIDES, base.HEAD
thermal, structure = base.thermal, base.structure


def body():
    hair = [(1.7225, 0.019, 0, 0.020, 0.025), (1.706, 0.018, 0, 0.056, 0.073), (1.672, 0.014, 0, 0.0755, 0.094), (1.640, -0.011, 0, 0.0781, 0.075), (1.600, -0.036, 0, 0.064, 0.042), (1.568, -0.042, 0, 0.040, 0.018)]
    neck = [(1.415, -0.012, 0, 0.060, 0.052), (1.500, 0.008, 0, 0.053, 0.055), (1.568, 0.016, 0, 0.048, 0.046)]
    hoodie = [
        (0.868, -0.017, 0, 0.172, 0.117, 2.4), (0.940, -0.012, 0, 0.180, 0.122, 2.4), (1.100, -0.003, 0, 0.161, 0.116, 2.4), (1.280, 0.000, 0, 0.165, 0.121, 2.4), (1.370, -0.010, 0, 0.192, 0.106, 2.7),
        (1.402, -0.014, 0, 0.202, 0.092, 2.9), (1.432, -0.016, 0, 0.178, 0.074, 2.6), (1.458, -0.011, 0, 0.110, 0.066, 2.2), (1.474, -0.004, 0, 0.075, 0.070, 2.0),
    ]
    hood = [(1.482, -0.072, 0, 0.086, 0.024, 2.2), (1.430, -0.110, 0, 0.110, 0.042, 2.2), (1.350, -0.131, 0, 0.088, 0.030, 2.2), (1.302, -0.128, 0, 0.050, 0.017, 2.2)]
    seat = [(1.015, 0.004, 0, 0.144, 0.095, 2.3), (0.900, -0.013, 0, 0.169, 0.107, 2.3), (0.835, -0.016, 0, 0.130, 0.084, 2.2), (0.800, -0.012, 0, 0.050, 0.038, 2.0)]
    out = [
        loft("head", "head", SKIN, 0.25, HEAD, "y", e=2.15, sub=1),
        loft("hair", "head", HAIR, 0.75, hair, "y", e=2.1, sub=1),
        loft("neck", "neck", SKIN, 0.25, neck, "y"),
        loft("hoodie", "torso", FLEECE, 0.45, hoodie, "y", sub=1),
        loft("hood", "torso", FLEECE, 0.4, hood, "y", sub=1),
        loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        sleeve = [(1.412, -0.010, SH + 0.018, 0.036, 0.042), (1.380, -0.008, SH + 0.020, 0.060, 0.065), (1.250, -0.003, SH + 0.012, 0.057, 0.062), (1.090, 0.001, SH + 0.003, 0.050, 0.055), (1.048, 0.000, SH, 0.032, 0.036)]
        forearm = [(1.116, 0.000, SH, 0.030, 0.034), (1.090, 0.002, SH, 0.047, 0.051), (0.950, 0.003, SH, 0.045, 0.049), (0.890, 0.000, SH, 0.042, 0.046), (0.836, 0.000, SH, 0.026, 0.031)]
        hand = [(0.850, -0.002, SH, 0.017, 0.026, 2.2), (0.785, 0.004, SH - 0.002, 0.016, 0.042, 2.4), (0.720, 0.003, SH - 0.009, 0.0145, 0.042, 2.5), (0.668, 0.000, SH - 0.022, 0.011, 0.024, 2.3)]
        thigh = [(0.980, -0.012, 0.093, 0.040, 0.048), (0.930, -0.012, 0.093, 0.082, 0.095), (0.820, -0.006, 0.090, 0.083, 0.092), (0.660, 0.003, 0.087, 0.071, 0.079), (0.520, 0.007, 0.084, 0.055, 0.059), (0.465, 0.006, 0.084, 0.049, 0.054), (0.445, 0.003, 0.084, 0.030, 0.033)]
        shank = [(0.532, 0.004, HIP, 0.032, 0.036), (0.495, 0.006, HIP, 0.052, 0.057), (0.380, -0.008, HIP, 0.053, 0.062), (0.230, -0.005, HIP, 0.046, 0.051), (0.105, -0.003, HIP, 0.045, 0.049)]
        shoe = [(-0.0745, 0.032, HIP, 0.020, 0.032, 3.0), (-0.050, 0.043, HIP, 0.033, 0.043, 2.9), (0.020, 0.044, HIP, 0.037, 0.044, 2.6), (0.090, 0.034, HIP + 0.002, 0.045, 0.034, 2.8), (0.157, 0.026, HIP + 0.003, 0.0495, 0.026, 3.0)]
        toe = [(0.143, 0.027, HIP + 0.003, 0.050, 0.027, 3.0), (0.180, 0.028, HIP + 0.002, 0.043, 0.024, 3.0), (0.205, 0.030, HIP, 0.016, 0.014, 2.4)]
        out += [
            loft("sleeve" + s, "arm" + s, FLEECE, 0.45, sleeve, "y", sub=1, mirror=mirror),
            loft("forearm" + s, "forearm" + s, FLEECE, 0.45, forearm, "y", sub=1, mirror=mirror),
            loft("hand" + s, "hand" + s, SKIN, 0.3, hand, "y", mirror=mirror),
            loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=1, mirror=mirror),
            loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=1, mirror=mirror),
            loft("shoe" + s, "foot" + s, CANVAS, 0.12, shoe, "x", sub=1, mirror=mirror),
            loft("shoe-toe" + s, "toe" + s, CANVAS, 0.12, toe, "x", mirror=mirror),
        ]
    return out


def decisions():
    out = base.decisions()
    out[0] = {
        "en": "Body at 1:200, one mass per part of the figure: head under a cap of short hair, neck, the drop-shoulder sweatshirt with its hood on the upper back, jeans seat, and per side sleeve, forearm, a mitten hand without a thumb curling towards the thigh, thigh, shank with the calf, sneaker with the toe spring and toe box. Face features, ribs, pocket, cuffs and laces are below the 30 mm feature and are merged.",
        "de": "Körper bei 1:200, eine Masse je Teil der Gestalt: Kopf unter einer Kappe aus kurzem Haar, Hals, das Sweatshirt mit überschnittener Schulter und der Kapuze auf dem oberen Rücken, Jeansgesäß und je Seite Ärmel, Unterarm, eine zum Oberschenkel gekrümmte Fäustlingshand ohne Daumen, Oberschenkel, Unterschenkel mit Wade, Sneaker mit Zehensprengung und Zehenkappe. Gesichtszüge, Bündchen, Tasche und Schnürung liegen unter dem Merkmal von 30 mm und gehen auf.",
    }
    return out
