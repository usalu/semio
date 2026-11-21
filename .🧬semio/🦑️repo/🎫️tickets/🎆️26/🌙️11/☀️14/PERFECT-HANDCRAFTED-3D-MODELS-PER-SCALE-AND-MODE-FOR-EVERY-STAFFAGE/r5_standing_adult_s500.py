"""🧍 Round-5 standing adult at 1:500 (smallest feature 75 mm): the fewest masses, stated anew against the remodelled 1:50 figure (drop-shoulder sweatshirt, tapered legs, sneaker with toe spring); thermal and the wire skeleton as in round 3 (`r3_standing_adult_s500`), whose ids this body keeps.

World of the rest pose: x ahead, y up, z left; ground 0, ankle 0.086, knee 0.490, hip joint 0.903, shoulder 1.3975, C7 1.465, head joint 1.561, vertex 1.710.
"""
import r3_standing_adult_s500 as base
from r3_standing_adult_kit import loft

SKIN, HAIR, FLEECE, DENIM, CANVAS = base.SKIN, base.HAIR, base.FLEECE, base.DENIM, base.CANVAS
SH, HIP, SIDES, HEAD = base.SH, base.HIP, base.SIDES, base.HEAD
structure = base.structure


def body():
    hair = [(1.7225, 0.019, 0, 0.024, 0.031), (1.695, 0.017, 0, 0.068, 0.086), (1.668, 0.012, 0, 0.0765, 0.094), (1.630, -0.016, 0, 0.0775, 0.070), (1.585, -0.040, 0, 0.060, 0.034), (1.560, -0.042, 0, 0.040, 0.018)]
    neck = [(1.415, -0.006, 0, 0.066, 0.062), (1.566, 0.016, 0, 0.049, 0.047)]
    hoodie = [(0.868, -0.017, 0, 0.174, 0.118, 2.4), (0.950, -0.011, 0, 0.180, 0.122, 2.4), (1.120, -0.002, 0, 0.161, 0.117, 2.4), (1.300, -0.014, 0, 0.168, 0.132, 2.4), (1.405, -0.040, 0, 0.203, 0.118, 2.9), (1.438, -0.036, 0, 0.176, 0.094, 2.6), (1.474, -0.004, 0, 0.076, 0.072, 2.0)]
    seat = [(1.010, 0.004, 0, 0.146, 0.096, 2.3), (0.890, -0.013, 0, 0.169, 0.107, 2.3), (0.800, -0.012, 0, 0.060, 0.042, 2.0)]
    out = [
        loft("head", "head", SKIN, 0.25, HEAD, "y", e=2.15, sub=1),
        loft("hair", "head", HAIR, 0.75, hair, "y", e=2.1, sub=1),
        loft("neck", "neck", SKIN, 0.25, neck, "y"),
        loft("hoodie", "torso", FLEECE, 0.45, hoodie, "y", sub=1),
        loft("jeans-seat", "pelvis", DENIM, 0.62, seat, "y", sub=1),
    ]
    for side, mirror in SIDES:
        s = "-" + side
        sleeve = [(1.410, -0.010, SH + 0.018, 0.040, 0.046), (1.370, -0.007, SH + 0.020, 0.061, 0.066), (1.220, -0.002, SH + 0.010, 0.055, 0.060), (1.080, 0.001, SH + 0.002, 0.050, 0.055), (1.050, 0.000, SH, 0.0375, 0.0400)]
        forearm = [(1.112, 0.000, SH, 0.0375, 0.0400), (1.080, 0.003, SH, 0.0485, 0.0525), (0.900, 0.001, SH, 0.044, 0.048), (0.836, 0.000, SH, 0.028, 0.033)]
        hand = [(0.848, -0.001, SH - 0.021, 0.036, 0.034, 2.3), (0.770, 0.004, SH - 0.021, 0.036, 0.048, 2.6), (0.674, 0.000, SH - 0.021, 0.036, 0.036, 2.4)]
        thigh = [(0.975, -0.012, 0.093, 0.045, 0.052), (0.900, -0.011, 0.092, 0.085, 0.095), (0.740, 0.000, 0.088, 0.077, 0.085), (0.540, 0.006, 0.084, 0.057, 0.061), (0.465, 0.006, 0.084, 0.049, 0.054), (0.450, 0.004, 0.084, 0.0375, 0.0400)]
        shank = [(0.528, 0.004, HIP, 0.0375, 0.0400), (0.490, 0.006, HIP, 0.052, 0.057), (0.370, -0.008, HIP, 0.053, 0.062), (0.108, -0.003, HIP, 0.045, 0.049)]
        shoe = [(-0.0745, 0.034, HIP, 0.022, 0.034, 3.0), (-0.030, 0.044, HIP, 0.035, 0.044, 2.8), (0.040, 0.043, HIP + 0.001, 0.039, 0.043, 2.6), (0.150, 0.027, HIP + 0.003, 0.050, 0.027, 3.0), (0.185, 0.028, HIP + 0.002, 0.044, 0.025, 3.0), (0.205, 0.029, HIP, 0.030, 0.021, 2.6)]
        out += [
            loft("sleeve" + s, "arm" + s, FLEECE, 0.45, sleeve, "y", sub=1, mirror=mirror),
            loft("forearm" + s, "forearm" + s, FLEECE, 0.45, forearm, "y", sub=1, mirror=mirror),
            loft("hand" + s, "hand" + s, SKIN, 0.3, hand, "y", mirror=mirror),
            loft("thigh" + s, "thigh" + s, DENIM, 0.62, thigh, "y", sub=1, mirror=mirror),
            loft("shank" + s, "shank" + s, DENIM, 0.62, shank, "y", sub=1, mirror=mirror),
            loft("shoe" + s, "foot" + s, CANVAS, 0.12, shoe, "x", mirror=mirror),
        ]
    return out


def thermal():
    return [t for t in base.thermal() if t.get("of") not in ("shoe-toe-l", "shoe-toe-r")]


def decisions():
    out = base.decisions()
    out[0] = {
        "en": "Body at 1:500, the fewest masses that read as a standing person: head, hair cap, neck, the drop-shoulder sweatshirt with the hood merged into the upper back, jeans seat, and per side sleeve, forearm, mitten hand, thigh, shank and one sneaker from heel to toe tip on the foot bone (the toe joint and the toe box are below the 75 mm feature). Each mass keeps the true breadth and depth of its part, so shoulders, waist, hips and the taper of the limbs survive; the hand is a mass of 72 mm whose outer face lies where the hand of 1:50 lies, the thinnest band that still prints at the end of the valid range.",
        "de": "Körper bei 1:500, die wenigsten Massen, die als stehende Person lesbar sind: Kopf, Haarkappe, Hals, das Sweatshirt mit überschnittener Schulter und im oberen Rücken aufgegangener Kapuze, Jeansgesäß und je Seite Ärmel, Unterarm, Fäustlingshand, Oberschenkel, Unterschenkel und ein Sneaker von der Ferse bis zur Zehenspitze am Fußknochen (Zehengelenk und Zehenkappe liegen unter dem Merkmal von 75 mm). Jede Masse behält die wahre Breite und Tiefe ihres Teils, sodass Schultern, Taille, Hüfte und die Verjüngung der Glieder erhalten bleiben; die Hand ist eine Masse von 72 mm, deren Außenseite dort liegt, wo die Hand bei 1:50 liegt, das dünnste Band, das am Ende des Gültigkeitsbereichs noch druckt.",
    }
    return out
