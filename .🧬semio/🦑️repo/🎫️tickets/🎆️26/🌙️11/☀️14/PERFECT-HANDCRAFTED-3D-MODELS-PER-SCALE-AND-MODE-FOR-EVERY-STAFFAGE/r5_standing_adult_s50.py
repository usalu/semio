"""🧍‍♂️ Round-5 standing adult at 1:50: the remodelled body (`r5_standing_adult_body50`), its thermal model and the skeleton as the load path, stated by hand in the world of the rest pose (x ahead, y up, z left).

The axial skeleton, the legs, the joints and the masses are those of round 3 (`r3_standing_adult_s50`, landmarks checked again in round 5); the hand skeleton follows the new three-phalanx fingers and the three-bone thumb, and the thermal inner layers follow the new garment sections.
"""
import r3_standing_adult_s50 as base
import r5_standing_adult_body50 as b50
from r3_standing_adult_kit import capsule, ellipsoid, loft, ref, shrink, sphere, st, th
from r5_standing_adult_body50 import SH

SKIN, COTTON = "skin", "cotton-underwear"
BONE = base.BONE
SIDES = base.SIDES
base.b50 = b50


def body():
    return b50.body()


def hand_ids(side):
    return ["hand-" + side, "thenar-" + side] + ["thumb-%s-%d" % (side, k) for k in range(3)] + ["finger-%s-%d%s" % (side, i, p) for i in range(4) for p in "abc"]


def thermal():
    by = {s["id"]: s for s in body()}
    old = base.thermal()
    keep = [t for t in old if not (t.get("region") == "hand-skin" or t["id"].startswith("th-tee"))]
    hands = [ref(i, "hand-skin") for side, _ in SIDES for i in hand_ids(side)]
    tee = [th(shrink(by["hoodie"], 0.004, "th-tee", material=COTTON, tone=0.8), "hoodie", "tee")]
    for side, mirror in SIDES:
        half = [(1.410, -0.010, SH + 0.018, 0.030, 0.036), (1.398, -0.009, SH + 0.020, 0.048, 0.054), (1.370, -0.007, SH + 0.020, 0.057, 0.062), (1.320, -0.005, SH + 0.017, 0.057, 0.062), (1.270, -0.004, SH + 0.013, 0.054, 0.059)]
        tee.append(th(loft("th-tee-sleeve-" + side, "arm-" + side, COTTON, 0.8, half, "y", sub=1, mirror=mirror), "hoodie", "tee"))
    at = next(i for i, t in enumerate(keep) if t.get("of") == "jeans-seat")
    return keep[:at] + tee + keep[at:] + hands


def arms():
    out = []
    for side, mirror in SIDES:
        s = "-" + side
        distal = [(1.120, 0.000, SH, 0.014, 0.011, 2.0), (1.100, 0.000, SH, 0.026, 0.012, 2.4), (1.088, 0.000, SH, 0.031, 0.013, 2.6), (1.078, 0.002, SH, 0.026, 0.013, 2.4), (1.072, 0.002, SH, 0.016, 0.010, 2.0)]
        out += [
            st(sphere("humeral-head" + s, "arm" + s, SKIN, BONE, (-0.002, 1.3975, SH - 0.004), 0.021, mirror=mirror), "arm" + s),
            st(ellipsoid("greater-tubercle" + s, "arm" + s, SKIN, BONE, (0.004, 1.388, SH + 0.014), (0.013, 0.016, 0.011), mirror=mirror), "arm" + s),
            st(capsule("humerus" + s, "arm" + s, SKIN, BONE, (0.000, 1.372, SH + 0.004), (0.000, 1.118, SH), 0.012, 0.0105, mirror=mirror), "arm" + s),
            st(loft("humeral-condyle" + s, "arm" + s, SKIN, BONE, distal, "y", mirror=mirror), "arm" + s),
            st(capsule("ulna" + s, "forearm" + s, SKIN, BONE, (-0.016, 1.092, SH - 0.002), (-0.017, 0.846, SH), 0.009, 0.0055, mirror=mirror), "forearm" + s),
            st(ellipsoid("olecranon" + s, "forearm" + s, SKIN, BONE, (-0.024, 1.096, SH - 0.002), (0.011, 0.016, 0.011), mirror=mirror), "forearm" + s),
            st(capsule("radius" + s, "forearm" + s, SKIN, BONE, (0.012, 1.070, SH + 0.006), (0.010, 0.870, SH), 0.0065, 0.0085, mirror=mirror), "forearm" + s),
            st(ellipsoid("radial-end" + s, "forearm" + s, SKIN, BONE, (0.008, 0.850, SH), (0.016, 0.014, 0.009), mirror=mirror), "forearm" + s),
            st(ellipsoid("carpus" + s, "hand" + s, SKIN, BONE, (0.000, 0.818, SH - 0.001), (0.024, 0.015, 0.0095), mirror=mirror), "hand" + s),
        ]
        for i, (fx, fy, _l, _r, _a) in enumerate(b50.FINGERS):
            p = b50.finger_points(i)
            out.append(st(capsule("metacarpal%s-%d" % (s, i + 2), "hand" + s, SKIN, BONE, (fx * 0.6, 0.806, SH - 0.001), (fx, fy + 0.002, SH - 0.004), 0.0046, 0.0050, mirror=mirror), "hand" + s))
            for k, part in enumerate("abc"):
                out.append(st(capsule("phalanx%s-%d%s" % (s, i + 2, part), "hand" + s, SKIN, BONE, p[k], p[k + 1], 0.0046 - 0.0003 * k, 0.0043 - 0.0003 * k, mirror=mirror), "hand" + s))
        t = b50.thumb_points()
        for k, name in enumerate(("metacarpal%s-1" % s, "phalanx%s-1a" % s, "phalanx%s-1b" % s)):
            out.append(st(capsule(name, "hand" + s, SKIN, BONE, t[k], t[k + 1], 0.0055 - 0.0005 * k, 0.0050 - 0.0005 * k, mirror=mirror), "hand" + s))
    return out


def axial():
    """🦴 The axial skeleton of round 3 with the acromion carried out to 0.183 m from the midline, so that the bony biacromial breadth is 0.380 m (0.225 of stature, DIN 33402-2 / ANSUR II)."""
    out = []
    for solid in base.axial():
        if solid["id"].startswith("acromion-"):
            mirror = solid["id"].endswith("-r")
            solid = st(capsule(solid["id"], "torso", SKIN, BONE, (-0.040, 1.424, 0.158), (-0.004, 1.431, 0.176), 0.0075, 0.007, mirror=mirror), "torso")
        out.append(solid)
    return out


def structure():
    return axial() + base.legs() + arms() + base.joints() + base.masses()


def decisions():
    out = base.decisions()
    out[0] = {
        "en": "Body at 1:50: every form is a loft through hand-stated anatomical cross-sections. Head with jaw and chin, cheekbones, nose, lips, brows, eyes and ears under short hair with a fringe; neck; a drop-shoulder hooded sweatshirt as one garment whose yoke carries the shoulder slope out over the deltoid, with hem rib, collar rib, kangaroo pocket, drawstrings and the hood lying on the upper back with its rolled edge round the neck; sleeves set in below the shoulder line, bunching above the rib cuffs; hands hanging relaxed with the palm towards the thigh, thenar, a three-bone thumb along the index finger and four grouped fingers of three phalanges each, flexed a little more from index to little finger; jeans with seat, gluteal masses, thighs tapering to the knee, kneecaps, calves and a straight hem; ankle socks; canvas sneakers with a rubber sole 26 mm at the heel and 20 mm at the ball, toe spring, heel counter, collar, instep, toe cap and laces.",
        "de": "Körper bei 1:50: Jede Form ist ein Loft durch von Hand gesetzte anatomische Querschnitte. Kopf mit Kiefer und Kinn, Wangenknochen, Nase, Lippen, Brauen, Augen und Ohren unter kurzem Haar mit Stirnfransen; Hals; ein Kapuzensweatshirt mit überschnittener Schulter als ein Kleidungsstück, dessen Passe die Schulterschräge über den Deltamuskel hinausführt, mit Saumbund, Kragenbund, Kängurutasche, Kordeln und der auf dem oberen Rücken liegenden Kapuze mit ihrem gerollten Rand um den Hals; Ärmel unter der Schulterlinie eingesetzt, über den Bündchen gestaut; locker hängende Hände mit der Handfläche zum Oberschenkel, Daumenballen, ein dreigliedriger Daumen am Zeigefinger und vier geschlossene Finger aus je drei Gliedern, vom Zeige- zum kleinen Finger etwas stärker gebeugt; Jeans mit Gesäß, Gesäßmuskeln, zum Knie verjüngten Oberschenkeln, Kniescheiben, Waden und geradem Saum; Knöchelsocken; Stoffsneaker mit Gummisohle von 26 mm an der Ferse und 20 mm am Ballen, Zehensprengung, Fersenkappe, Schaftrand, Rist, Zehenkappe und Schnürung.",
    }
    return out
