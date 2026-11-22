"""🦴 Cyclist skeleton chisel: single vertebrae, hand and foot bones, femoral trochanter and condyles, zygomatic arches.

Hand-chosen numbers for `humans/cyclist` (woman 1.72 m). Rewrites the rider's structure solids in
`volumes/humans/cyclist.json` at 1:50 (all parts) and 1:100 (thoracolumbar vertebral bodies).
See `rework/assets/humans--cyclist.md`.
"""
import json
import math
from pathlib import Path

SOURCE = Path(__file__).parent / "volumes/humans/cyclist.json"
BONE = {"material": "bone", "role": "body", "tone": 0.9}
S = math.sqrt(0.5)

TORSO_LINE = [(-0.01, 0.0), (0.05, -0.022), (0.11, -0.03), (0.17, -0.02), (0.25, 0.005), (0.33, 0.018), (0.4, 0.012), (0.452, -0.004)]
NECK_LINE = [(0.0, -0.006), (0.03, -0.016), (0.055, -0.02), (0.085, -0.016), (0.108, -0.008)]
LUMBAR = [("l5", 0.034, 0.024, 0.017), ("l4", 0.034, 0.0235, 0.017), ("l3", 0.034, 0.023, 0.0165), ("l2", 0.033, 0.022, 0.016), ("l1", 0.033, 0.021, 0.0155)]
THORACIC = [("t12", 0.028, 0.0205, 0.0155), ("t11", 0.027, 0.02, 0.015), ("t10", 0.026, 0.019, 0.0145), ("t9", 0.025, 0.018, 0.014), ("t8", 0.025, 0.0175, 0.0135), ("t7", 0.024, 0.017, 0.013), ("t6", 0.024, 0.0165, 0.0125), ("t5", 0.023, 0.016, 0.012), ("t4", 0.023, 0.0155, 0.0115), ("t3", 0.022, 0.015, 0.0105), ("t2", 0.022, 0.0145, 0.0095), ("t1", 0.021, 0.014, 0.009)]
CERVICAL = [("c7", 0.017, 0.0125, 0.0085), ("c6", 0.016, 0.0115, 0.008), ("c5", 0.016, 0.011, 0.008), ("c4", 0.016, 0.011, 0.008), ("c3", 0.016, 0.011, 0.008), ("c2", 0.019, 0.0115, 0.008), ("c1", 0.012, 0.026, 0.011)]
SPINOUS = {"l": (0.03, 0.006, 0.0055), "t": (0.036, 0.024, 0.004), "c": (0.012, 0.004, 0.004), "c7": (0.022, 0.008, 0.0045), "c1": None}
TRANSVERSE_LUMBAR = (0.042, 0.0045)
METATARSALS = [(-0.02, -0.03, 0.137, 0.0065), (-0.01, -0.015, 0.14, 0.0045), (0.0, 0.0, 0.135, 0.0045), (0.01, 0.014, 0.128, 0.0045), (0.02, 0.027, 0.118, 0.0045)]
TOES = [(-0.03, 0.048, 0.006), (-0.015, 0.044, 0.004), (0.0, 0.04, 0.004), (0.014, 0.035, 0.004), (0.027, 0.03, 0.004)]


def walk(line, pitches):
    """📏 Points on a polyline at the cumulative pitches, scaled so that they fill it exactly."""
    lengths = [math.dist(a, b) for a, b in zip(line, line[1:])]
    scale = sum(lengths) / sum(pitches)
    points, travelled = [], 0.0
    for target in [0.0] + [scale * sum(pitches[: i + 1]) for i in range(len(pitches))]:
        rest = target
        for (a, b), length in zip(zip(line, line[1:]), lengths):
            if rest <= length + 1e-9:
                t = rest / length
                points.append((a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])))
                break
            rest -= length
        else:
            points.append(line[-1])
    return points


def tilt(a, b):
    """🧭 Loft section rotation whose normal runs from a to b in the (u, v) plane, rx along w, ry along v."""
    theta = math.atan2(b[1] - a[1], b[0] - a[0])
    c, s = math.cos(theta / 2) * S, math.sin(theta / 2) * S
    return [round(c, 6), round(s, 6), round(c, 6), round(-s, 6)]


def vertebrae(bone, line, rows, processes=True):
    """🧱 One waisted body per vertebra standing on its neighbour, with spinous and lumbar transverse processes."""
    points = walk(line, [row[1] for row in rows])
    solids = []
    for (name, _, half_width, half_depth), a, b in zip(rows, points, points[1:]):
        rotation = tilt(a, b)
        sections = []
        for t, factor in [(0.0, 0.9), (0.12, 1.0), (0.5, 0.86), (0.88, 1.0), (1.0, 0.9)]:
            sections.append({"at": [round(a[0] + t * (b[0] - a[0]), 5), round(a[1] + t * (b[1] - a[1]), 5), 0], "rx": round(half_width * factor, 5), "ry": round(half_depth * factor, 5), "rotation": rotation, "exponent": 2.3})
        solids.append({"id": f"vertebra-{name}", "kind": "loft", "bone": bone, **BONE, "sections": sections, "capStart": True, "capEnd": True, "member": bone, "part": "member"})
        process = SPINOUS.get(name, SPINOUS[name[0]])
        middle = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)
        if processes and process:
            reach, drop, radius = process
            solids.append({"id": f"spinous-{name}", "kind": "capsule", "bone": bone, **BONE, "p0": [round(middle[0], 5), round(middle[1] + half_depth * 0.7, 5), 0], "p1": [round(middle[0] - drop, 5), round(middle[1] + half_depth + reach, 5), 0], "r0": radius, "r1": radius, "member": bone, "part": "member"})
        if processes and name[0] == "l":
            reach, radius = TRANSVERSE_LUMBAR
            solids.append({"id": f"transverse-{name}", "kind": "capsule", "bone": bone, **BONE, "p0": [round(middle[0], 5), round(middle[1] + half_depth * 0.6, 5), -reach], "p1": [round(middle[0], 5), round(middle[1] + half_depth * 0.6, 5), reach], "r0": radius, "r1": radius, "member": bone, "part": "member"})
    return solids


def capsule(identifier, bone, p0, p1, radius, member=None):
    """💊 A bone shaft from p0 to p1."""
    return {"id": identifier, "kind": "capsule", "bone": bone, **BONE, "p0": [round(v, 5) for v in p0], "p1": [round(v, 5) for v in p1], "r0": radius, "r1": radius, "member": member or bone, "part": "member"}


def ellipsoid(identifier, bone, centre, radii, member=None):
    """🥚 A compact bone."""
    return {"id": identifier, "kind": "ellipsoid", "bone": bone, **BONE, "radii": radii, "transform": {"translation": [round(v, 5) for v in centre]}, "member": member or bone, "part": "member"}


def mid(a, b):
    """➗ Midpoint."""
    return [(x + y) / 2 for x, y in zip(a, b)]


def hand(side, body):
    """✋ Carpus, five metacarpals and fourteen phalanges inside the glove that closes round the grip."""
    bone = f"hand-{side}"
    paths = {o["id"]: [s["at"] for s in o["sections"]] for o in body if o["bone"] == bone}
    fingers = [paths[f"fingers-{side}-{i}"] for i in range(4)]
    thumb = paths[f"glove-thumb-{side}"]
    sign = 1 if thumb[0][2] > 0 else -1
    centre_w = sum(f[0][2] for f in fingers) / 4 * 0.3
    solids = [ellipsoid(f"carpus-{side}", bone, [0.012, 0.002, centre_w], [0.014, 0.009, 0.024])]
    thumb_base = [0.008, -0.006, sign * 0.02]
    thumb_knuckle = mid(thumb[1], thumb[2])
    solids.append(capsule(f"metacarpal-1-{side}", bone, thumb_base, thumb_knuckle, 0.0055))
    solids.append(capsule(f"thumb-proximal-{side}", bone, thumb_knuckle, thumb[3], 0.0045))
    solids.append(capsule(f"thumb-distal-{side}", bone, thumb[3], thumb[4], 0.004))
    for index, path in enumerate(fingers):
        ray = index + 2
        solids.append(capsule(f"metacarpal-{ray}-{side}", bone, [0.022, 0.003, path[0][2] * 0.55], path[0], 0.0045))
        solids.append(capsule(f"phalanx-proximal-{ray}-{side}", bone, path[0], path[2], 0.0042))
        solids.append(capsule(f"phalanx-middle-{ray}-{side}", bone, path[2], path[4], 0.004))
        solids.append(capsule(f"phalanx-distal-{ray}-{side}", bone, path[4], path[5], 0.00375))
    return solids


def foot(side):
    """🦶 Talus, calcaneus, tarsal block, five metatarsals and five toe rays inside the shoe."""
    m = 1 if side == "l" else -1
    bone, toe = f"foot-{side}", f"toe-{side}"
    solids = [
        ellipsoid(f"talus-{side}", bone, [0.006, -0.014, 0], [0.024, 0.016, 0.02]),
        capsule(f"calcaneus-{side}", bone, [-0.024, -0.056, 0.004 * m], [0.024, -0.034, 0.004 * m], 0.017),
        ellipsoid(f"tarsus-{side}", bone, [0.052, -0.022, 0.002 * m], [0.024, 0.014, 0.027]),
    ]
    for ray, (base_w, head_w, head_u, radius) in enumerate(METATARSALS, start=1):
        solids.append(capsule(f"metatarsal-{ray}-{side}", bone, [0.068, -0.02, base_w * m], [head_u, -0.004 - (0.1374 - head_u) * 0.3, head_w * m], radius))
    for ray, (w, length, radius) in enumerate(TOES, start=1):
        start_u = METATARSALS[ray - 1][2] - 0.1374
        solids.append(capsule(f"toe-ray-{ray}-{side}", toe, [start_u, -0.003, w * m], [start_u + length, -0.008, w * m], radius, member=bone))
    return solids


def femur(side):
    """🍗 Greater trochanter and the two condyles on the existing femur loft."""
    m = 1 if side == "l" else -1
    bone = f"thigh-{side}"
    return [
        ellipsoid(f"trochanter-{side}", bone, [0.028, 0.004, 0.047 * m], [0.02, 0.012, 0.012]),
        ellipsoid(f"femur-condyle-lateral-{side}", bone, [0.392, -0.01, 0.021 * m], [0.02, 0.03, 0.017]),
        ellipsoid(f"femur-condyle-medial-{side}", bone, [0.394, -0.01, -0.021 * m], [0.021, 0.031, 0.018]),
    ]


def skull():
    """💀 Zygomatic arches from the cheek bone to the temporal bone."""
    return [capsule(f"zygomatic-{side}", "head", [0.034, -0.042, 0.057 * m], [0.036, 0.008, 0.049 * m], 0.004) for side, m in (("l", 1), ("r", -1))]


def rebuild(volume, scale):
    """🔨 Replace the rods and slabs of one scale by the bones above, each group at the place of the solid it replaces."""
    fine = scale == "50"
    groups = {"spine": vertebrae("torso", TORSO_LINE, LUMBAR + THORACIC, fine)}
    after = {}
    if fine:
        groups["cervical"] = vertebrae("neck", NECK_LINE, CERVICAL)
        after = {"maxilla": skull(), "femur-l": femur("l"), "femur-r": femur("r")}
        for side in "lr":
            groups[f"metacarpus-{side}"] = hand(side, volume["body"])
            groups[f"tarsus-{side}"] = foot(side)
            groups[f"phalanges-{side}"] = []
    owner = {solid["id"]: key for key, group in groups.items() for solid in group}
    added = {solid["id"] for group in after.values() for solid in group}
    emitted, result = set(), []
    for solid in volume["structure"]:
        identifier = solid["id"]
        if identifier in added:
            continue
        key = identifier if identifier in groups else owner.get(identifier)
        if key is None:
            result.append(solid)
            result += after.get(identifier, [])
        elif key not in emitted:
            emitted.add(key)
            result += groups[key]
    volume["structure"] = result


SPINE_RADII = [(0.01713, 0.01916), (0.01633, 0.01822), (0.01553, 0.01729), (0.01473, 0.01636), (0.01367, 0.01511), (0.0126, 0.01387), (0.01167, 0.01278), (0.01097, 0.01197)]
TRUE_SHAFTS = ["femur", "tibia", "humerus"]


def coarse(volume, fine):
    """🦴 1:200 and 1:500: long bones and the spine keep their true 1:50 section as declared slender members instead of feature-thick pipes."""
    source = {solid["id"]: solid for solid in fine["structure"]}
    line = TORSO_LINE
    spine = {"id": "spine", "kind": "loft", "bone": "torso", **BONE, "sections": [], "capStart": True, "capEnd": True, "member": "torso", "part": "member", "slender": True}
    for index, (point, (rx, ry)) in enumerate(zip(line, SPINE_RADII)):
        a, b = line[max(index - 1, 0)], line[min(index + 1, len(line) - 1)]
        spine["sections"].append({"at": [point[0], point[1], 0], "rx": rx, "ry": ry, "rotation": tilt(a, b)})
    result = []
    for solid in volume["structure"]:
        identifier = solid["id"]
        stem, _, side = identifier.rpartition("-")
        if identifier == "spine":
            result.append(spine)
        elif stem in TRUE_SHAFTS and side in ("l", "r"):
            result.append({**source[identifier], "slender": True})
        elif stem == "forearm-bones" or (stem in ("ulna", "radius") and side in ("l", "r")):
            if stem != "radius":
                result += [{**source[f"ulna-{side}"], "slender": True}, {**source[f"radius-{side}"], "slender": True}]
        else:
            result.append(solid)
    volume["structure"] = result


DECISIONS = {
    "50": {
        "en": "1:50 structure: the rider's skeleton bone by bone and the bicycle's load path. Skull with maxilla, mandible with both rami and the zygomatic arches; seven cervical, twelve thoracic and five lumbar vertebrae as waisted bodies standing on each other along the lordosis and kyphosis of the seated back, each with its spinous process, the lumbar ones with transverse processes; twelve rib pairs to sternum and costal arch; clavicles and scapulae with acromion on the clavicle bones, sacrum and the hip bones with ilium, pubis and ischium on the hip-bone bones; humerus, ulna and radius; carpus, five metacarpals and fourteen phalanges that follow the gloved fingers round the grip; femur with greater trochanter and both condyles, patella, tibia and fibula; talus, calcaneus, tarsal block, five metatarsals and five toe rays inside the shoe; joints and segment masses. The bicycle members are the frame tubes, fork, stays, bar, cranks, pedals, rims and spokes of the body list.",
        "de": "1:50-Tragwerk: das Skelett der Fahrerin Knochen für Knochen und der Lastpfad des Fahrrads. Schädel mit Oberkiefer, Unterkiefer mit beiden Ästen und den Jochbögen; sieben Hals-, zwölf Brust- und fünf Lendenwirbel als taillierte, aufeinander stehende Körper entlang der Lordose und Kyphose des sitzenden Rückens, jeder mit Dornfortsatz, die Lendenwirbel mit Querfortsätzen; zwölf Rippenpaare zu Brustbein und Rippenbogen; Schlüsselbeine und Schulterblätter mit Schulterhöhe an den Schlüsselbein-Knochen, Kreuzbein und die Hüftbeine mit Darm-, Scham- und Sitzbein an den Hüftbein-Knochen; Oberarmknochen, Elle und Speiche; Handwurzel, fünf Mittelhandknochen und vierzehn Fingerglieder, die den Handschuhfingern um den Griff folgen; Oberschenkelknochen mit großem Rollhügel und beiden Gelenkknorren, Kniescheibe, Schien- und Wadenbein; Sprungbein, Fersenbein, Fußwurzelblock, fünf Mittelfußknochen und fünf Zehenstrahlen im Schuh; Gelenke und Segmentmassen. Die Glieder des Fahrrads sind Rahmenrohre, Gabel, Streben, Lenker, Kurbeln, Pedale, Felgen und Speichen der Körperliste.",
    },
    "100": {
        "en": "1:100 structure: the same skeleton with every compact part at least 15 mm: the seventeen thoracic and lumbar vertebrae remain as single bodies without processes, the cervical spine is one rod, every second rib pair is kept, hand and foot are one plate each; scapulae, clavicles and acromion are kept, the bicycle members are those of 1:50 without the bridge and dropouts.",
        "de": "1:100-Tragwerk: dasselbe Skelett mit jedem gedrungenen Teil ab 15 mm: die siebzehn Brust- und Lendenwirbel bleiben einzelne Körper ohne Fortsätze, die Halswirbelsäule ist ein Stab, jedes zweite Rippenpaar bleibt, Hand und Fuß sind je eine Platte; Schulterblätter, Schlüsselbeine und Schulterhöhe bleiben, die Glieder des Fahrrads sind die von 1:50 ohne Steg und Ausfallenden.",
    },
    "200": {
        "en": "1:200 structure: spine, femur, tibia, humerus, ulna and radius keep their true 1:50 section as declared slender members, so the limbs read as bones and not as pipes; three open rib loops and the sternum line stand for the ribcage, the pelvis is sacrum and two hip plates, skull and facial skeleton are two forms, hand and foot one bar each; joints and masses are floored at 30 mm; the bicycle members are the merged tubes of the body list.",
        "de": "1:200-Tragwerk: Wirbelsäule, Oberschenkelknochen, Schienbein, Oberarmknochen, Elle und Speiche behalten ihren wahren Querschnitt von 1:50 als erklärte schlanke Glieder, damit die Gliedmaßen als Knochen und nicht als Rohre lesbar sind; drei offene Rippenschleifen und die Brustbeinlinie stehen für den Brustkorb, das Becken besteht aus Kreuzbein und zwei Hüftplatten, Hirn- und Gesichtsschädel sind zwei Formen, Hand und Fuß je ein Stab; Gelenke und Massen liegen an der 30-mm-Grenze; die Glieder des Fahrrads sind die zusammengefassten Rohre der Körperliste.",
    },
    "500": {
        "en": "1:500 structure: the long bones (femur, tibia, humerus, ulna, radius) and the spine keep their true section as declared slender members; everything compact is floored at 75 mm: skull and facial skeleton, one pelvic block with a hip-bone bar per side, clavicle bars, hand and foot bars, joints and masses; two open rib loops mark the ribcage; the bicycle is its slender tubes of true diameter.",
        "de": "1:500-Tragwerk: die langen Knochen (Oberschenkelknochen, Schienbein, Oberarmknochen, Elle, Speiche) und die Wirbelsäule behalten ihren wahren Querschnitt als erklärte schlanke Glieder; alles Gedrungene liegt an der 75-mm-Grenze: Hirn- und Gesichtsschädel, ein Beckenblock mit einem Hüftbeinstab je Seite, Schlüsselbeinstäbe, Hand- und Fußstäbe, Gelenke und Massen; zwei offene Rippenschleifen zeigen den Brustkorb; das Fahrrad besteht aus seinen schlanken Rohren wahren Durchmessers.",
    },
}


def main():
    """▶️ Rewrite the source."""
    document = json.loads(SOURCE.read_text())
    for scale in ("50", "100"):
        rebuild(document["volumes"][scale], scale)
        print(scale, len(document["volumes"][scale]["structure"]))
    for scale in ("200", "500"):
        coarse(document["volumes"][scale], document["volumes"]["50"])
        print(scale, len(document["volumes"][scale]["structure"]))
    for scale, text in DECISIONS.items():
        document["volumes"][scale]["decisions"][2] = text
    SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")


if __name__ == "__main__":
    main()
