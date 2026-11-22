"""🦴 Rework 05, parent carrying a toddler: the corrected rig (lumbar and thoracic spine, clavicles, scapulae and hip bones on the lateral 3D rest placement, finger and thumb chains, toddler hands), the re-authored poses and the anchors, written into the candidate document.

Usage: python3 rework-05-parent-carrying-toddler-roots.py  (reads the catalog asset, writes `🗑️generated/r3-parent-carrying-toddler/🔣️.json`)
@see ./rework-05-kit.py
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "rework-05-kit.py"))
kit = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(kit)

SLUG = "parent-carrying-toddler"
CAT = kit.catalog_path("👤️humans", SLUG)
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-" + SLUG, "🔣️.json")


def rev(lo, hi, twist=None):
    """🔩 A revolute joint with its limits in degrees from rest."""
    j = {"type": "revolute", "minDeg": lo, "maxDeg": hi}
    if twist:
        j["twistMinDeg"], j["twistMaxDeg"] = twist
    return j


def mass(kg, cx, cy=0.0, ref="de-leva-1996", source="literature"):
    """⚖️ A point mass at (cx, cy) of the bone frame."""
    m = {"kg": kg, "comX": cx, "comY": cy, "distribution": "point", "source": source}
    if source == "literature":
        m["reference"] = ref
    return m


def bone(id, parent, en, de, x, y, angle, length, joint, z=None, abduction=None, m=None, twist=None):
    """🦴 One rig bone."""
    b = {"id": id, "parent": parent, "title": kit.text(en, de), "x": x, "y": y, "angleDeg": angle, "lengthM": length, "joint": joint}
    if z is not None:
        b["z"] = z
    if abduction is not None:
        b["abductionDeg"] = abduction
    if twist is not None:
        b["twistDeg"] = twist
    if m is not None:
        b["mass"] = m
    return b


SACROILIAC = (0.075, 0.055, 0.045)
HIP_CENTRE = (0.0, 0.0, 0.0909)


def turn(angle, abduction, twist):
    """🧭 The rest rotation of a bone in its parent frame, columns u, v, w: about the parent's w, then its own v, then its own u (`rig-lateral-rest.md` 1.1)."""
    a, b, t = (math.radians(q) for q in (angle, abduction, twist))
    rw = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    rv = [[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]]
    ru = [[1, 0, 0], [0, math.cos(t), math.sin(t)], [0, -math.sin(t), math.cos(t)]]
    mul = lambda p, q: [[sum(p[i][k] * q[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    return mul(mul(rw, rv), ru)


def angles(local):
    """📐 angleDeg, abductionDeg and twistDeg of a rest rotation given by its columns u, v, w in the parent frame."""
    u = [local[i][0] for i in range(3)]
    v = [local[i][1] for i in range(3)]
    b = math.asin(max(-1.0, min(1.0, u[2])))
    a = 0.0 if math.hypot(u[0], u[1]) < 1e-9 else math.atan2(u[1], u[0])
    v1 = [-math.sin(a), math.cos(a), 0.0]
    w2 = [-math.cos(a) * math.sin(b), -math.sin(a) * math.sin(b), math.cos(b)]
    t = math.atan2(-sum(v[i] * w2[i] for i in range(3)), sum(v[i] * v1[i] for i in range(3)))
    return tuple(round(math.degrees(q), 4) for q in (a, b, t))


def hip_bone(g):
    """🦴 The rest placement of one hip bone in the pelvis frame, from the sacroiliac joint down, forward and out to the centre of the hip joint, and of the thigh hanging from its tip with the frame it always had (down, joint axis to the left): (length, hip angles, thigh angles)."""
    d = [HIP_CENTRE[i] - SACROILIAC[i] for i in range(3)]
    d[2] *= g
    n = math.sqrt(sum(q * q for q in d))
    a = math.degrees(math.atan2(d[1], d[0]))
    b = math.degrees(math.asin(d[2] / n))
    r = turn(a, b, 0)
    th = turn(-180, 0, 0)
    rel = [[sum(r[k][i] * th[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
    return round(n, 5), (round(a, 4), round(b, 4)), angles(rel)


STERNOCLAVICULAR = (0.228, -0.10, 0.02)
ACROMIOCLAVICULAR = (0.25, -0.035, 0.168)
GLENOHUMERAL = (0.21492, -0.03, 0.17265)
SUPERIOR_ANGLE = (0.238, 0.046, 0.07)
INFERIOR_ANGLE = (0.09, 0.06, 0.088)
LATERAL_ANGLE = (0.213, -0.008, 0.146)


def sub(a, b):
    """➖ Difference of two points."""
    return [a[i] - b[i] for i in range(3)]


def dot(a, b):
    """⚫ Scalar product."""
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    """✖️ Vector product."""
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    """📏 The direction of a vector."""
    n = math.sqrt(dot(a, a))
    return [c / n for c in a]


def columns(u, v, w):
    """🧱 A rotation from its columns u, v, w."""
    return [[u[i], v[i], w[i]] for i in range(3)]


def within(r, p):
    """🦴 A vector of the parent frame in the frame with the rotation `r` (columns u, v, w)."""
    return [sum(r[k][i] * p[k] for k in range(3)) for i in range(3)]


def between(r, q):
    """🔗 The rotation `q` as seen from the frame `r` (both given in the same parent frame)."""
    return [[sum(r[k][i] * q[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def pelvic_girdle():
    """🦴 The rest frame of the left hip bone in the pelvis frame: origin at the sacroiliac joint, rotation columns u, v, w."""
    return {"o": list(SACROILIAC), "r": turn(hip_bone(1)[1][0], hip_bone(1)[1][1], 0)}


def shoulder_girdle():
    """🦴 The left shoulder girdle in the thorax frame (up, back, left). Clavicle: from the sternoclavicular to the acromioclavicular joint, normal up and joint axis back, so `angleDeg` shrugs and `abductionDeg` retracts. Scapula: from the acromioclavicular joint along the blade to the inferior angle, joint axis the dorsal normal of the blade, so `angleDeg` rotates the blade on the ribcage. Upper arm: at the glenohumeral centre with the frame of a hanging sagittal arm."""
    d = sub(ACROMIOCLAVICULAR, STERNOCLAVICULAR)
    u = unit(d)
    w = unit(sub((0, 1, 0), [c * u[1] for c in u]))
    clavicle = columns(u, cross(w, u), w)
    n = unit(cross(sub(SUPERIOR_ANGLE, INFERIOR_ANGLE), sub(LATERAL_ANGLE, INFERIOR_ANGLE)))
    n = n if n[1] > 0 else [-c for c in n]
    e = sub(INFERIOR_ANGLE, ACROMIOCLAVICULAR)
    su = unit(sub(e, [c * dot(e, n) for c in n]))
    scapula = columns(su, cross(n, su), n)
    arm = turn(180, 0, 0)
    out = {"clavicle": {"o": list(STERNOCLAVICULAR), "r": clavicle, "length": round(math.sqrt(dot(d, d)), 5), "angles": angles(clavicle)},
           "scapula": {"o": list(ACROMIOCLAVICULAR), "r": scapula, "length": round(dot(e, su), 5), "angles": angles(between(clavicle, scapula))},
           "arm": {"at": [round(c, 5) for c in within(scapula, sub(GLENOHUMERAL, ACROMIOCLAVICULAR))], "angles": angles(between(scapula, arm))}}
    for key, parent in (("clavicle", None), ("scapula", clavicle), ("arm", scapula)):
        want = out[key].get("r", arm)
        got = turn(*out[key]["angles"])
        got = got if parent is None else [[sum(parent[i][k] * got[k][j] for k in range(3)) for j in range(3)] for i in range(3)]
        assert all(abs(got[i][j] - want[i][j]) < 1e-5 for i in range(3) for j in range(3)), key
    return out


def side(s, en, de):
    """↔️ Names and signs of one body side: suffix, English and German side word, lateral sign."""
    return ("-l", "Left", "Linke", 1) if s == "l" else ("-r", "Right", "Rechte", -1)


def rig():
    """🩻 The whole rig: parent (spine in three segments, shoulder girdle, arms with finger and thumb chains, legs with forefoot) and toddler (trunk, neck, head, arms with hands, legs)."""
    est = dict(source="authored-estimate")
    bones = [
        bone("pelvis", None, "Pelvis (hip axis to L5/S1)", "Becken (Hüftachse bis L5/S1)", 0, 0, 90, 0.105, rev(-60, 30), m=mass(8.23, 0.0525)),
        bone("lumbar", "pelvis", "Lumbar spine (L5/S1 to T12/L1)", "Lendenwirbelsäule (L5/S1 bis T12/L1)", 0.105, 0.04, 0, 0.17, rev(-55, 25, (-12, 12)), m=mass(6.6, 0.085)),
        bone("thorax", "lumbar", "Thorax (T12/L1 to C7)", "Brustkorb (T12/L1 bis C7)", 0.17, 0, 0, 0.3035, rev(-35, 20, (-35, 35)), m=mass(13.266, 0.14)),
        bone("neck", "thorax", "Cervical column", "Halswirbelsäule", 0.3035, 0, -6, 0.09732, rev(-40, 35, (-40, 40)), m=mass(0.73, 0.046, -0.007, **est)),
        bone("head", "neck", "Head", "Kopf", 0.09732, -0.01972, 0, 0.14148, rev(-30, 35, (-58, 58)), m=mass(3.679, 0.0361649, 0.021109, **est)),
    ]
    for s in ("l", "r"):
        sg = shoulder_girdle()
        x, en, de, g = side(s, "", "")
        bones += [
            bone("clavicle" + x, "thorax", en + " clavicle (sternoclavicular to acromioclavicular joint)", de + "s Schlüsselbein (Brustbein-Schlüsselbein-Gelenk bis Schultereckgelenk)", STERNOCLAVICULAR[0], STERNOCLAVICULAR[1], sg["clavicle"]["angles"][0], sg["clavicle"]["length"], rev(-8, 35, (-5, 40) if g < 0 else (-40, 5)), z=STERNOCLAVICULAR[2] * g, abduction=sg["clavicle"]["angles"][1] * g, twist=sg["clavicle"]["angles"][2] * g),
            bone("scapula" + x, "clavicle" + x, en + " scapula (acromioclavicular joint to the inferior angle)", de + "s Schulterblatt (Schultereckgelenk bis unterer Schulterblattwinkel)", sg["clavicle"]["length"], 0, sg["scapula"]["angles"][0], sg["scapula"]["length"], rev(-10, 60), abduction=sg["scapula"]["angles"][1] * g, twist=sg["scapula"]["angles"][2] * g),
            bone("upper-arm" + x, "scapula" + x, en + " upper arm", de + "r Oberarm", sg["arm"]["at"][0], sg["arm"]["at"][1], sg["arm"]["angles"][0], 0.316, rev(-60, 180, (-90, 90)), z=sg["arm"]["at"][2] * g, abduction=sg["arm"]["angles"][1] * g, twist=sg["arm"]["angles"][2] * g, m=mass(1.683, 0.182)),
            bone("forearm" + x, "upper-arm" + x, en + " forearm", de + "r Unterarm", 0.316, 0, 0, 0.248, rev(0, 150, (-90, 90)), z=0.01 * g, m=mass(0.911, 0.1134)),
            bone("hand" + x, "forearm" + x, en + " hand (carpus and metacarpus)", de + " Hand (Handwurzel und Mittelhand)", 0.248, 0, 0, 0.095, rev(-65, 75, (-95, 95)), m=mass(0.37, 0.069)),
            bone("fingers" + x, "hand" + x, en + " proximal phalanges of the four fingers", de + " Fingergrundglieder", 0.095, -0.004, 0, 0.045, rev(-20, 20)),
            bone("fingertips" + x, "fingers" + x, en + " middle and distal phalanges", de + " Fingermittel- und -endglieder", 0.045, 0, 0, 0.048, rev(-20, 20)),
            bone("thumb" + x, "hand" + x, en + " thumb (metacarpal and proximal phalanx)", de + "r Daumen (Mittelhandknochen und Grundglied)", 0.022, 0.026, 38, 0.062, rev(-30, 45)),
            bone("thumbtip" + x, "thumb" + x, en + " thumb distal phalanx", de + "s Daumenendglied", 0.062, 0, -8, 0.03, rev(-20, 20)),
            bone("hip-bone" + x, "pelvis", en + " hip bone (sacroiliac joint to the hip joint)", de + "s Hüftbein (Kreuzbein-Darmbein-Gelenk bis Hüftgelenk)", SACROILIAC[0], SACROILIAC[1], hip_bone(g)[1][0], hip_bone(g)[0], rev(-4, 4), z=SACROILIAC[2] * g, abduction=hip_bone(g)[1][1]),
            bone("thigh" + x, "hip-bone" + x, en + " thigh", de + "r Oberschenkel", hip_bone(g)[0], 0, hip_bone(g)[2][0], 0.4, rev(-60, 125), abduction=hip_bone(g)[2][1], twist=hip_bone(g)[2][2], m=mass(9.755, 0.164)),
            bone("shank" + x, "thigh" + x, en + " shank", de + "r Unterschenkel", 0.4, 0, 0, 0.418, rev(-150, 0), m=mass(3.175, 0.1864)),
            bone("foot" + x, "shank" + x, en + " foot", de + "r Fuß", 0.418, 0, 90, 0.21, rev(-65, 60), m=mass(0.851, 0.06, -0.05)),
            bone("toe" + x, "foot" + x, en + " forefoot", de + "r Vorfuß", 0.13, -0.088, 0, 0.08, rev(0, 70)),
        ]
    who = dict(ref="who-2006")
    bones += [
        bone("toddler-torso", "forearm-r", "Toddler trunk (hip joints to C7)", "Rumpf des Kleinkinds (Hüftgelenke bis C7)", 0.2, 0.105, 90, 0.28, rev(-45, 45), z=0.02, m=mass(5.49, 0.14, **who)),
        bone("toddler-neck", "toddler-torso", "Toddler cervical column", "Halswirbelsäule des Kleinkinds", 0.28, 0, -5, 0.045, rev(-20, 24, (-45, 45)), m=mass(0.21, 0.02, 0.009, **est)),
        bone("toddler-head", "toddler-neck", "Toddler skull", "Schädel des Kleinkinds", 0.045, 0.004, 0, 0.16, rev(-40, 40, (-60, 60)), m=mass(1.62, 0.06, -0.007, **est)),
    ]
    for s in ("l", "r"):
        x, en, de, g = side(s, "", "")
        low = en.lower()
        bones += [
            bone("toddler-arm" + x, "toddler-torso", "Toddler " + low + " upper arm", de + "r Oberarm des Kleinkinds", 0.25, -0.005, 180, 0.135, rev(-90, 170, (-90, 90)), z=0.09 * g, m=mass(0.32, 0.06, **who)),
            bone("toddler-forearm" + x, "toddler-arm" + x, "Toddler " + low + " forearm", de + "r Unterarm des Kleinkinds", 0.135, 0, 0, 0.11, rev(0, 145), m=mass(0.29, 0.075, **who)),
            bone("toddler-hand" + x, "toddler-forearm" + x, "Toddler " + low + " hand in its mitten", de + " Hand des Kleinkinds im Fäustling", 0.11, 0, 0, 0.075, rev(-60, 60)),
            bone("toddler-thigh" + x, "toddler-torso", "Toddler " + low + " thigh", de + "r Oberschenkel des Kleinkinds", 0, 0, 180, 0.18, rev(-20, 130), z=0.05 * g, m=mass(0.976, 0.08, **who)),
            bone("toddler-shank" + x, "toddler-thigh" + x, "Toddler " + low + " shank", de + "r Unterschenkel des Kleinkinds", 0.18, 0, 0, 0.17, rev(-140, 5), m=mass(0.66, 0.075, **who)),
            bone("toddler-foot" + x, "toddler-shank" + x, "Toddler " + low + " foot in its rubber boot", de + "r Fuß des Kleinkinds im Gummistiefel", 0.17, 0, 90, 0.115, rev(-50, 40), m=mass(0.194, 0.045, -0.02, **who)),
        ]
    return bones


CARRY = {
    "upper-arm-r": {"angleDeg": -4}, "forearm-r": {"angleDeg": 100}, "hand-r": {"angleDeg": 8, "twistDeg": 90},
    "fingers-r": {"abductionDeg": 14}, "fingertips-r": {"abductionDeg": 22}, "thumb-r": {"angleDeg": -6},
    "toddler-torso": {"angleDeg": -2}, "toddler-neck": {"angleDeg": -2}, "toddler-head": {"angleDeg": -3},
    "toddler-thigh-l": {"angleDeg": 84}, "toddler-shank-l": {"angleDeg": -78}, "toddler-foot-l": {"angleDeg": 6},
    "toddler-thigh-r": {"angleDeg": 80}, "toddler-shank-r": {"angleDeg": -72}, "toddler-foot-r": {"angleDeg": 2},
    "toddler-arm-l": {"angleDeg": 12}, "toddler-forearm-l": {"angleDeg": 52}, "toddler-hand-l": {"angleDeg": 8},
    "toddler-arm-r": {"angleDeg": 6}, "toddler-forearm-r": {"angleDeg": 58}, "toddler-hand-r": {"angleDeg": 10},
}
HOLD_L = {"upper-arm-l": {"angleDeg": 50, "twistDeg": 70}, "forearm-l": {"angleDeg": 95}, "hand-l": {"angleDeg": -8, "twistDeg": 20}, "fingers-l": {"abductionDeg": -30}, "fingertips-l": {"abductionDeg": -35}}
SWING_L = {"fingers-l": {"abductionDeg": -18}, "fingertips-l": {"abductionDeg": -28}, "thumb-l": {"angleDeg": -8}}


def gait(hip, knee, ankle, toe=0):
    """🚶 Hip flexion, knee flexion, ankle dorsiflexion and toe extension of one leg as bone offsets."""
    return {"thigh": {"angleDeg": hip}, "shank": {"angleDeg": -knee}, "foot": {"angleDeg": ankle}, "toe": {"angleDeg": toe}}


CYCLE = [gait(21, 4, 2), gait(17, 15, -4), gait(5, 8, 4), gait(-6, 5, 9), gait(-11, 9, 2, 24), gait(4, 54, -10, 4), gait(15, 56, -3), gait(23, 24, 1)]
ARM_SWING = [14, 10, 2, -7, -12, -8, 1, 9]
BOB = [-0.6, 0.4, 1.0, 0.4, -0.6, 0.4, 1.0, 0.4]
AREAS = {"coat-sleeves": 0.42, "gloves": 0.13, "toddler-boots": 0.136, "toddler-face": 0.055}
ADVANCE = 1.1164
WALK_IDS = ["heel-strike-l", "walk-1", "midstance-l", "walk-3", "walk-4", "walk-5", "walk-6", "walk-7"]


def poses(old):
    """🧍 Every pose re-authored on the corrected rig; titles, contacts, thermal state and loads are kept from the catalog pose of the same id, descriptions are rewritten."""
    by = {p["id"]: p for p in old}
    out = []
    stand = {"lumbar": {"angleDeg": 2.5}, "thorax": {"angleDeg": 1.5}, "neck": {"angleDeg": -2}, "head": {"angleDeg": -1},
             "thigh-l": {"angleDeg": 3}, "shank-l": {"angleDeg": -4}, "foot-l": {"angleDeg": 1}, "thigh-r": {"angleDeg": -1}, "shank-r": {"angleDeg": -2}, "foot-r": {"angleDeg": 3}, **CARRY, **HOLD_L}
    p = dict(by["standing-carry"])
    p["title"] = kit.text("Standing with the toddler on the near forearm", "Stehen mit dem Kleinkind auf dem nahen Unterarm")
    p["description"] = kit.text(
        "Upright stance leaning 4 degrees back against the load. The toddler sits on the near forearm at the front of the parent's hip, its back resting against the parent's chest and shoulder; the carrying hand lies palm up under the near thigh with the fingers closed around it, and the far arm reaches across the child's belly and holds it against the body.",
        "Aufrechter Stand, 4 Grad gegen die Last zurückgelehnt. Das Kleinkind sitzt auf dem nahen Unterarm vor der Hüfte des Elternteils und lehnt mit dem Rücken an dessen Brust und Schulter; die tragende Hand liegt mit der Handfläche nach oben unter dem nahen Oberschenkel und umschließt ihn mit den Fingern, der ferne Arm greift vor dem Bauch des Kindes herum und hält es am Körper.")
    p["bones"] = stand
    out.append(p)
    for k, id in enumerate(WALK_IDS):
        p = dict(by[id])
        left, right = CYCLE[k], CYCLE[(k + 4) % 8]
        bones = {"lumbar": {"angleDeg": 2.5}, "thorax": {"angleDeg": 1.5 + 0.4 * BOB[k]}, "neck": {"angleDeg": -2}, "head": {"angleDeg": -1}, **CARRY, **SWING_L}
        bones = {k2: dict(v) for k2, v in bones.items()}
        for name, v in left.items():
            bones[name + "-l"] = v
        for name, v in right.items():
            bones[name + "-r"] = v
        bones["upper-arm-l"] = {"angleDeg": ARM_SWING[k]}
        bones["forearm-l"] = {"angleDeg": 14 + max(ARM_SWING[k], 0) * 0.8}
        bones["forearm-r"] = {"angleDeg": 101 + BOB[k]}
        bones["toddler-torso"] = {"angleDeg": -2 - BOB[k]}
        bones["toddler-shank-l"] = {"angleDeg": -78 + 5 * BOB[k]}
        bones["toddler-shank-r"] = {"angleDeg": -72 - 5 * BOB[k]}
        bones["toddler-head"] = {"angleDeg": -3 + BOB[k]}
        p["bones"] = bones
        p["description"] = kit.text(
            "Instant %d of 8 of the walking cycle: the legs follow a normal gait at a careful carrying pace, the free far arm swings against the legs, the near arm keeps the toddler seated on the forearm against the chest and the child bobs slightly with the trunk." % (k + 1),
            "Augenblick %d von 8 des Gehzyklus: Die Beine folgen einem normalen Gang in vorsichtigem Tragetempo, der freie ferne Arm schwingt gegenläufig, der nahe Arm hält das Kleinkind auf dem Unterarm an der Brust, und das Kind wippt leicht mit dem Rumpf." % (k + 1))
        out.append(p)
    p = dict(by["hoisting"])
    hoist = {k2: dict(v) for k2, v in stand.items()}
    hoist.update({"lumbar": {"angleDeg": 5}, "thorax": {"angleDeg": 4}, "neck": {"angleDeg": -7}, "head": {"angleDeg": -3},
                  "thigh-l": {"angleDeg": 14}, "shank-l": {"angleDeg": -26}, "foot-l": {"angleDeg": 12}, "thigh-r": {"angleDeg": 12}, "shank-r": {"angleDeg": -24}, "foot-r": {"angleDeg": 12},
                  "upper-arm-r": {"angleDeg": 2}, "forearm-r": {"angleDeg": 112}, "toddler-torso": {"angleDeg": -10},
                  "upper-arm-l": {"angleDeg": 46, "twistDeg": 70}, "forearm-l": {"angleDeg": 100},
                  "toddler-arm-l": {"angleDeg": 60}, "toddler-forearm-l": {"angleDeg": 40}, "toddler-shank-l": {"angleDeg": -60}, "toddler-shank-r": {"angleDeg": -68}})
    p["bones"] = hoist
    p["title"] = kit.text("Hoisting the toddler higher", "Kleinkind höher setzen")
    p["description"] = kit.text(
        "The parent bends the knees a little, leans back and lifts the near forearm so that the toddler slides higher against the chest; the far hand stays on the child's belly and the child tips back against the shoulder.",
        "Das Elternteil beugt leicht die Knie, lehnt sich zurück und hebt den nahen Unterarm, sodass das Kleinkind höher an die Brust rutscht; die ferne Hand bleibt am Bauch des Kindes, das Kind kippt an die Schulter zurück.")
    out.append(p)
    return out


HEAD_ANCHORS = {"mouth": (-0.041, -0.098), "nostrils": (-0.018, -0.118), "crown": (0.166, 0.012), "toddler-mouth": (-0.019, -0.08), "toddler-nostrils": (-0.004, -0.092), "toddler-crown": (0.218, 0.008)}


def anchors(old):
    """⚓ The anchors bound to the corrected rig (the lumbar load anchor moves to the lumbar bone)."""
    out = []
    for a in old:
        a = dict(a)
        if a["bone"] == "torso":
            a["bone"] = "lumbar"
        if a["id"] in HEAD_ANCHORS:
            a["x"], a["y"] = HEAD_ANCHORS[a["id"]]
        out.append(a)
    return out


DRAWN = {
    50: ("Every view is drawn from the 1:50 body in the standing carry: the parent in beanie, scarf and coat with yoke, lapels, buttons and pockets, the toddler on the near forearm against the chest with its large hatted head beside the parent's cheek, the far arm across the child's belly; every solid has its own outline in the frame of its bone, and the twisted far arm and both hands are solid placements so they stay true in every pose.",
         "Jede Ansicht ist aus dem 1:50-Body in der stehenden Tragehaltung gezeichnet: das Elternteil mit Mütze, Schal und Mantel mit Passe, Revers, Knöpfen und Taschen, das Kleinkind auf dem nahen Unterarm an der Brust, sein großer Kopf mit Mütze neben der Wange des Elternteils, der ferne Arm vor dem Bauch des Kindes; jeder Körper hat seinen eigenen Umriss im System seines Knochens, der gedrehte ferne Arm und beide Hände sind Körperplatzierungen, damit sie in jeder Pose stimmen.",
         "Faces with nose, brow, lips, chin, cheeks, ears and eyes, separate gloved fingers and thumbs, boot soles, the toddler's mitten thumbs, hat cuffs and pompom.",
         "Gesichter mit Nase, Braue, Lippen, Kinn, Wangen, Ohren und Augen, einzelne behandschuhte Finger und Daumen, Stiefelsohlen, die Fäustlingsdaumen des Kleinkinds, Mützenränder und Bommel.",
         "Nothing is merged at this scale.", "In diesem Maßstab ist nichts zusammengefasst.",
         "The toddler's zip (8 mm) is modelled as a slender solid and not drawn as a filled shape.", "Der Reißverschluss des Kleinkinds (8 mm) ist als schlanker Körper modelliert und nicht als gefüllte Form gezeichnet."),
    100: ("The same drawing from the 1:100 body: both figures with their garments, the hands with one slab per finger row and a thumb, the faces with nose, ears and the toddler's cheeks.",
          "Dieselbe Zeichnung aus dem 1:100-Body: beide Figuren mit ihrer Kleidung, die Hände mit einer Platte je Fingerreihe und Daumen, die Gesichter mit Nase, Ohren und den Wangen des Kleinkinds.",
          "Nose, ears, coat buttons, scarf end, boot soles, mitten thumbs, hat cuffs and pompom.", "Nase, Ohren, Mantelknöpfe, Schalende, Stiefelsohlen, Fäustlingsdaumen, Mützenränder und Bommel.",
          "The four fingers of each hand are one slab per phalanx row.", "Die vier Finger jeder Hand sind je Gliedreihe eine Platte.",
          "Eyes, lips, brow, lapels, pockets and the zip fall below 15 mm and are left out.", "Augen, Lippen, Braue, Revers, Taschen und Reißverschluss liegen unter 15 mm und entfallen."),
    200: ("The same drawing from the 1:200 body: heads with hat and cuff, trunk, yoke, limbs, mitt hands and boots of both figures.",
          "Dieselbe Zeichnung aus dem 1:200-Body: Köpfe mit Mütze und Rand, Rumpf, Passe, Gliedmaßen, Fäustlingshände und Stiefel beider Figuren.",
          "Hat cuffs, pompom, the scarf end and the toddler's collar.", "Mützenränder, Bommel, Schalende und Kragen des Kleinkinds.",
          "Each hand is one mitt form; each boot includes its sole.", "Jede Hand ist eine Fäustlingsform; jeder Stiefel enthält seine Sohle.",
          "Faces, buttons and thumbs fall below 30 mm and are left out.", "Gesichter, Knöpfe und Daumen liegen unter 30 mm und entfallen."),
    500: ("The same drawing from the 1:500 body: two figures as their masses, the parent with hat, scarf, coat and legs, the toddler with its large head, belly and short limbs on the forearm.",
          "Dieselbe Zeichnung aus dem 1:500-Body: zwei Figuren als ihre Massen, das Elternteil mit Mütze, Schal, Mantel und Beinen, das Kleinkind mit großem Kopf, Bauch und kurzen Gliedmaßen auf dem Unterarm.",
          "The pompom, enlarged to the printable 75 mm.", "Der Bommel, auf druckbare 75 mm vergrößert.",
          "Hands are mitt forms of the printable thickness, boots include their soles.", "Hände sind Fäustlingsformen in druckbarer Dicke, Stiefel enthalten ihre Sohlen.",
          "Hat cuffs, scarf end, collar and every facial feature fall below 75 mm and are left out.", "Mützenränder, Schalende, Kragen und alle Gesichtszüge liegen unter 75 mm und entfallen."),
}
SUMMARY = ("A 1.70 m parent in a wool coat, scarf, beanie, jeans, leather gloves and ankle boots carrying a 24-month toddler (0.865 m, 12.2 kg) in a padded overall, knit hat, mittens and rubber boots: the child sits on the near forearm at the front of the hip with its back against the parent's chest, the carrying hand closed under its thigh and the far arm across its belly; it is a second living body with its own breathing. Standing, a walking cycle with planted feet and a free swinging arm, hoisting and settling; four individually drawn and modelled scales on a rig with lumbar and thoracic spine, shoulder girdle and finger chains.",
           "Ein 1,70 m großes Elternteil in Wollmantel, Schal, Mütze, Jeans, Lederhandschuhen und Stiefeletten trägt ein 24 Monate altes Kleinkind (0,865 m, 12,2 kg) in gefüttertem Overall, Strickmütze, Fäustlingen und Gummistiefeln: Das Kind sitzt auf dem nahen Unterarm vor der Hüfte und lehnt mit dem Rücken an der Brust des Elternteils, die tragende Hand umschließt seinen Oberschenkel von unten, der ferne Arm liegt vor seinem Bauch; es ist ein zweiter lebender Körper mit eigener Atmung. Stehen, ein Gehzyklus mit aufgesetzten Füßen und frei schwingendem Arm, Hochnehmen und Zurechtsetzen; vier einzeln gezeichnete und modellierte Maßstäbe auf einem Rig mit Lenden- und Brustwirbelsäule, Schultergürtel und Fingerketten.")


def texts(a):
    """🗣️ Summary and drawing decisions true to the rebuilt model."""
    a["summary"] = kit.text(*SUMMARY)
    for s, (sil_en, sil_de, ret_en, ret_de, mer_en, mer_de, om_en, om_de) in DRAWN.items():
        d = a["representations"][str(s)]["decisions"]
        d["silhouette"], d["retained"], d["merged"], d["omitted"] = kit.text(sil_en, sil_de), kit.text(ret_en, ret_de), kit.text(mer_en, mer_de), kit.text(om_en, om_de)
        d["thinElements"] = kit.text("No filled shape is thinner than the smallest feature of this scale; parts below it are left out or modelled at the printable size.", "Keine gefüllte Form ist dünner als das kleinste Merkmal dieses Maßstabs; kleinere Teile entfallen oder sind in druckbarer Größe modelliert.")


def main():
    """▶️ Writes the candidate document with the new rig, poses and anchors (volumes and drawings are added by the model and drawing scripts)."""
    doc = kit.read_json(CAT)
    a = doc["asset"]
    a["rig"]["bones"] = rig()
    a["poses"] = poses(a["poses"])
    a["anchors"] = anchors(a["anchors"])
    texts(a)
    a["dimensions"] = {**a["dimensions"], "heightM": 1.744, "widthM": 0.655, "depthM": 0.532}
    for r in a["thermal"]["regions"]:
        if r["id"] in AREAS:
            r["areaM2"] = AREAS[r["id"]]
    for c in a["clips"]:
        if c["id"] == "walk-carrying":
            c["advanceM"] = ADVANCE
    text = __import__("json").dumps(a["thermal"], ensure_ascii=False).replace('"bone": "torso"', '"bone": "thorax"')
    a["thermal"] = __import__("json").loads(text)
    kit.write_json(CANDIDATE, doc)
    print("[DEBUG] wrote", CANDIDATE)


if __name__ == "__main__":
    main()
