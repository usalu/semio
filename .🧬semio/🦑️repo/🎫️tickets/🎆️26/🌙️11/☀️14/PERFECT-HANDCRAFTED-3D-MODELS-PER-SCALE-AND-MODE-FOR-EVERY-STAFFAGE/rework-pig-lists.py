"""🐖 pig: hand edits of the volume lists after the girdle conversion (`rework-pig-rig.py`), every number chosen by hand.

1:50   dewclaws seated on the pastern; the root joint marker sits in the sacrum.
1:100  ear roots sunk into the head / skull; rib heads on the vertebral bodies.
1:200  upper arm modelled (it bridged nothing between shoulder and forearm); the two claws of a foot are one cloven hoof (a 23 mm claw is under the 30 mm feature); scapulothoracic synsarcosis as the joint that carries the trunk.
1:500  body: ears as true 5 mm sheets, one foot solid per leg; structure remodelled for 75 mm: true shafts declared slender (spine, femur, tibia, humerus, radius-ulna with olecranon, metatarsus with calcaneus, ribs), compact parts merged to the feature (neck, sacrum, hip bones, metacarpus, digits), scapula and ear cartilage as declared sheets.

Usage: python3 rework-pig-lists.py     edits `volumes/animals/pig.json` in place (idempotent)
"""
import copy
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "pig.json")
LIMB = [0, -0.70711, 0, 0.70711]
SIDES = {"near": -1.0, "far": 1.0}
TAG = {"near": "n", "far": "f"}


def section(x, y, z, lateral, normal, exponent=2.2, rotation=LIMB):
    return {"at": [round(x, 5), round(y, 5), round(z, 5)], "rx": lateral, "ry": normal, "exponent": exponent, "rotation": rotation}


def loft(solid_id, bone, material, tone, sections, **more):
    return {"id": solid_id, "kind": "loft", "bone": bone, "material": material, "role": "body", "tone": tone, "sections": sections, "capStart": True, "capEnd": True, **more}


def member(solid_id, bone, sections, **more):
    return loft(solid_id, bone, "bone", 0.18, sections, member=bone, part="member", **more)


def find(items, solid_id):
    return next((s for s in items if s["id"] == solid_id), None)


def replace(items, old_ids, new):
    index = min(i for i, s in enumerate(items) if s["id"] in old_ids)
    kept = [s for s in items if s["id"] not in old_ids and s["id"] != new["id"]]
    kept.insert(min(index, len(kept)), new)
    items[:] = kept


def put(items, new, after=None):
    old = find(items, new["id"])
    if old is not None:
        items[items.index(old)] = new
    elif after is not None and find(items, after) is not None:
        items.insert(items.index(find(items, after)) + 1, new)
    else:
        items.append(new)


def sink(solid, depth, toward):
    """Prepends a root section `depth` further along `toward` (bone frame), once."""
    first = copy.deepcopy(solid["sections"][0])
    first["at"] = [round(first["at"][i] + depth * toward[i], 5) for i in range(3)]
    solid["sections"].insert(0, first)


def spine_y(spine, x):
    stations = [(s["at"][0], s["at"][1]) for s in spine["sections"]]
    for (x0, y0), (x1, y1) in zip(stations, stations[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return stations[0][1] if x < stations[0][0] else stations[-1][1]


def rib_heads(structure):
    spine = find(structure, "thoracic-spine")
    for solid in structure:
        if "-rib-" not in solid["id"]:
            continue
        first = solid["sections"][0]
        if abs(first["at"][2]) <= 0.009:
            continue
        head = copy.deepcopy(first)
        side = -1.0 if first["at"][2] < 0 else 1.0
        head["at"] = [first["at"][0], round(spine_y(spine, first["at"][0]), 5), side * 0.008]
        solid["sections"].insert(0, head)


def leaf(solid, thickness, tip):
    """Lop ear as a leaf: the hand stations of the blade joined by a smooth curve (three steps per span, Catmull-Rom through the stations) with the stated half thickness per station and a rounded tip station; done once."""
    stations = solid["sections"]
    if len(stations) > 7:
        return
    for station, half in zip(stations, thickness):
        station["ry"] = half

    def spline(values, i, t):
        p0, p1, p2, p3 = values[max(i - 1, 0)], values[i], values[i + 1], values[min(i + 2, len(values) - 1)]
        return 0.5 * (2 * p1 + (p2 - p0) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (3 * p1 - p0 - 3 * p2 + p3) * t * t * t)

    out = []
    for i in range(len(stations) - 1):
        for step in range(3):
            t = step / 3
            q0, q1 = stations[i]["rotation"], stations[i + 1]["rotation"]
            flip = -1.0 if sum(a * b for a, b in zip(q0, q1)) < 0 else 1.0
            q = [a * (1 - t) + flip * b * t for a, b in zip(q0, q1)]
            length = sum(c * c for c in q) ** 0.5
            out.append({"at": [round(spline([st["at"][k] for st in stations], i, t), 5) for k in range(3)], "rx": round(spline([st["rx"] for st in stations], i, t), 5), "ry": round(spline([st["ry"] for st in stations], i, t), 5), "exponent": 2, "rotation": [round(c / length, 5) for c in q]})
    last, before = stations[-1], stations[-2]
    out.append(dict(copy.deepcopy(last), exponent=2))
    out.append(dict(copy.deepcopy(last), exponent=2, rx=tip, at=[round(last["at"][k] + 0.25 * (last["at"][k] - before["at"][k]), 5) for k in range(3)]))
    solid["sections"] = out


def scale_50(volumes):
    for side in SIDES:
        leaf(find(volumes["body"], side + "-ear"), [0.008, 0.006, 0.0045, 0.004, 0.004, 0.004], 0.005)
    for solid in volumes["body"]:
        if "dewclaw" in solid["id"] and solid["transform"]["translation"][1] < -0.03:
            solid["transform"]["translation"][1] = round(solid["transform"]["translation"][1] + 0.006, 5)
    joint = find(volumes["structure"], "joint-pelvis")
    joint["transform"]["translation"] = [0.125, 0, 0]
    joint["radii"] = [0.02, 0.02, 0.02]


EAR_TO_HEAD = {"near": [0.392, -0.737, 0.549], "far": [0.392, -0.737, -0.549]}


def scale_100(volumes):
    for side in SIDES:
        ear = find(volumes["body"], side + "-ear")
        if len(ear["sections"]) == 4:
            sink(ear, 0.012, EAR_TO_HEAD[side])
        leaf(ear, [0.0085, 0.0085, 0.0078, 0.0076, 0.0076], 0.007)
        cartilage = find(volumes["structure"], side + "-ear-cartilage")
        if len(cartilage["sections"]) == 2:
            sink(cartilage, 0.02, EAR_TO_HEAD[side])
    rib_heads(volumes["structure"])
    tail = find(volumes["structure"], "caudal-1")["sections"]
    if abs(tail[1]["at"][0] - tail[0]["at"][0] - 0.008) > 1e-6:
        tail.insert(0, dict(copy.deepcopy(tail[0]), at=[round(tail[0]["at"][0] - 0.008, 5), tail[0]["at"][1], tail[0]["at"][2]]))


def cloven(claws, solid_id):
    one, two = claws
    merged = copy.deepcopy(one)
    merged["id"] = solid_id
    for a, b in zip(merged["sections"], two["sections"]):
        a["at"][2] = round((a["at"][2] + b["at"][2]) / 2, 5)
        a["rx"] = round(a["rx"] + abs(a["at"][2] - b["at"][2]), 5)
    return merged


def synsarcosis(side, radius):
    return {"id": side + "-scapulothoracic", "kind": "ellipsoid", "bone": "thorax", "material": "bone", "role": "body", "tone": 0.5, "radii": [radius, radius, radius], "transform": {"translation": [0.3211, -0.0032, SIDES[side] * 0.0525], "rotation": [0, 0, 0, 1]}, "member": "scapula-f" + TAG[side], "part": "joint"}


def scale_200(volumes, finest):
    body, structure = volumes["body"], volumes["structure"]
    for side in SIDES:
        arm = copy.deepcopy(find(finest["body"], side + "-upper-arm"))
        put(body, arm, after=side + "-shoulder")
        for leg in ("hind", "fore"):
            claws = [find(body, f"{side}-{leg}-claw-{k}") for k in (1, 2)]
            if all(claws):
                replace(body, {c["id"] for c in claws}, cloven(claws, f"{side}-{leg}-claws"))
        put(structure, synsarcosis(side, 0.035), after=side + "-fore-scapula")
        for leg in ("hind", "fore"):
            old = {f"th-{side}-{leg}-claw-1", f"th-{side}-{leg}-claw-2"}
            if any(find(volumes["thermal"], i) for i in old):
                replace(volumes["thermal"], old, {"id": f"th-{side}-{leg}-claws", "of": f"{side}-{leg}-claws", "region": "hooves"})


def foot(side, leg):
    z = -0.01 if leg == "hind" else 0.002
    z *= -SIDES[side]
    return loft(f"{side}-{leg}-foot", f"hoof-{leg[0]}{TAG[side]}", "horn", 0.82, [
        section(-0.012, 0.0, z, 0.0245, 0.0245),
        section(0.02, 0.0, z, 0.026, 0.026),
        section(0.055, 0.008, z, 0.028, 0.03),
        section(0.078, 0.016, z, 0.028, 0.03),
        section(0.09, 0.02, z, 0.022, 0.02),
    ])


def scale_500(volumes):
    body, structure = volumes["body"], volumes["structure"]
    for side, sign in SIDES.items():
        mirror = -sign
        ear = find(body, side + "-ear")
        for s in ear["sections"]:
            s["ry"] = 0.0025
        ear["sheet"] = True
        for leg in ("hind", "fore"):
            old = {f"{side}-{leg}-pastern", f"{side}-{leg}-claw-1", f"{side}-{leg}-claw-2"}
            if any(find(body, i) for i in old):
                replace(body, old, foot(side, leg))
            whole = foot(side, leg)["sections"]
            cut = section(0.045, 0.005, whole[0]["at"][2], 0.0275, 0.029)
            thermal = volumes["thermal"]
            pastern = dict(loft(f"th-{side}-{leg}-pastern", f"hoof-{leg[0]}{TAG[side]}", "pig-skin", 0.4, whole[:2] + [cut]), region="skin-legs")
            hoof = dict(loft(f"th-{side}-{leg}-hoof", f"hoof-{leg[0]}{TAG[side]}", "horn", 0.82, [copy.deepcopy(cut)] + whole[2:]), region="hooves")
            gone = {f"th-{side}-{leg}-pastern", f"th-{side}-{leg}-claw-1", f"th-{side}-{leg}-claw-2", f"th-{side}-{leg}-hoof"}
            if any(find(thermal, i) for i in gone):
                replace(thermal, gone, pastern)
                put(thermal, hoof, after=pastern["id"])
        n = TAG[side]
        cartilage = find(structure, side + "-ear-cartilage")
        for s, width in zip(cartilage["sections"], (0.02, 0.045, 0.02)):
            s["rx"], s["ry"] = width, 0.001
        cartilage["sheet"] = True
        blade = find(structure, side + "-fore-scapula")
        for s in blade["sections"]:
            s["rx"] = 0.0025
        blade["sheet"] = True
        put(structure, member(side + "-hind-thigh", "thigh-h" + n, [section(0, 0, mirror * 0.007, 0.019, 0.019), section(0.06, 0, mirror * -0.002, 0.015, 0.015), section(0.203, 0.004, mirror * -0.015, 0.019, 0.019)], slender=True))
        put(structure, member(side + "-hind-shank", "shank-h" + n, [section(0, 0, mirror * -0.015, 0.0175, 0.0175), section(0.03, 0.004, mirror * -0.014, 0.014, 0.015), section(0.178, 0, mirror * -0.011, 0.014, 0.014), section(0.19, 0, mirror * -0.011, 0.016, 0.016)], slender=True))
        put(structure, member(side + "-hind-metatarsus", "metatarsus-h" + n, [section(-0.045, -0.012, mirror * -0.01, 0.011, 0.012), section(0.0, -0.004, mirror * -0.01, 0.016, 0.016), section(0.06, 0, mirror * -0.01, 0.0125, 0.0125), section(0.131, 0, mirror * -0.01, 0.015, 0.015)], slender=True))
        put(structure, member(side + "-fore-humerus", "humerus-f" + n, [section(0, 0, mirror * -0.01, 0.017, 0.017), section(0.06, 0, mirror * -0.008, 0.014, 0.015), section(0.18, 0, mirror * -0.003, 0.017, 0.017)], slender=True))
        put(structure, member(side + "-fore-forearm", "forearm-f" + n, [section(-0.045, -0.018, mirror * -0.003, 0.011, 0.011), section(0.0, -0.002, mirror * -0.003, 0.016, 0.016), section(0.07, 0.004, mirror * 0.0, 0.014, 0.014), section(0.14, 0.002, mirror * 0.002, 0.016, 0.016)], slender=True))
        put(structure, member(side + "-fore-metacarpus", "metacarpus-f" + n, [section(-0.005, 0, mirror * 0.002, 0.0255, 0.0255), section(0.045, 0, mirror * 0.002, 0.0255, 0.0255), section(0.095, 0, mirror * 0.002, 0.0255, 0.0255)]))
        for leg, z in (("hind", -0.01), ("fore", 0.002)):
            digits = member(f"{side}-{leg}-digits", f"hoof-{leg[0]}{n}", [section(0.0, 0.0, mirror * z, 0.0258, 0.0258), section(0.05, 0.006, mirror * z, 0.0258, 0.0258), section(0.08, 0.014, mirror * z, 0.0258, 0.0258)])
            old = {f"{side}-{leg}-digit-1", f"{side}-{leg}-digit-2"}
            if any(find(structure, i) for i in old):
                replace(structure, old, digits)
            else:
                put(structure, digits)
        hip = find(structure, side + "-ilium")
        hip["sections"] = [section(-0.02, 0.0, sign * 0.03, 0.0375, 0.0375, 2), section(0.06, -0.005, sign * 0.008, 0.0375, 0.0375, 2), section(0.133, 0.0, 0.0, 0.0375, 0.0375, 2), section(0.19, -0.03, sign * -0.015, 0.0375, 0.0375, 2)]
        put(structure, synsarcosis(side, 0.04), after=side + "-fore-scapula")
        for rib in structure:
            if rib["id"].startswith(side + "-rib-"):
                rib["slender"] = True
    for spine in ("lumbar-spine", "thoracic-spine"):
        find(structure, spine)["slender"] = True
    for s in find(structure, "cervical-spine")["sections"]:
        s["rx"], s["ry"] = 0.038, 0.038
    sacrum = find(structure, "sacrum")
    sacrum["sections"] = sacrum["sections"][:2]
    sacrum["sections"].append(dict(copy.deepcopy(sacrum["sections"][1]), at=[0.065, 0.09, 0]))
    for s in sacrum["sections"]:
        s["rx"], s["ry"] = 0.038, 0.038
    structure[:] = [s for s in structure if not s["id"].startswith("caudal-")]
    put(structure, {"id": "caudal-1", "kind": "ellipsoid", "bone": "tail-1", "material": "bone", "role": "body", "tone": 0.18, "radii": [0.0375, 0.0375, 0.0375], "transform": {"translation": [-0.04, 0, 0], "rotation": [0, 0, 0, 1]}, "member": "tail-1", "part": "member"}, after="sacrum")
    rib_heads(structure)


def main():
    source = json.load(open(SOURCE, encoding="utf-8"))
    volumes = source["volumes"]
    scale_50(volumes["50"])
    scale_100(volumes["100"])
    scale_200(volumes["200"], volumes["50"])
    scale_500(volumes["500"])
    open(SOURCE, "w", encoding="utf-8").write(json.dumps(source, ensure_ascii=False, separators=(",", ":")))
    for scale in ("50", "100", "200", "500"):
        print(scale, {name: len(volumes[scale][name]) for name in ("body", "thermal", "structure")})


main()


DECISIONS = {
    "50": ("The girdles follow the rig: each shoulder blade runs from its dorsal border beside the withers outward to the shoulder joint and each hip bone from the sacroiliac joint outward to the hip joint, so the limbs hang at the true breadths; the lop ears are smooth leaves 8 mm thick and the dewclaws sit on the pastern.", "Die Gürtel folgen dem Rig: jedes Schulterblatt läuft von seinem Rückenrand neben dem Widerrist nach aussen zum Schultergelenk und jedes Hüftbein vom Kreuzdarmbeingelenk nach aussen zum Hüftgelenk, sodass die Gliedmassen in den wahren Breiten hängen; die Schlappohren sind glatte, 8 mm dicke Blätter und die Afterklauen sitzen an der Fessel."),
    "100": ("Ear roots are sunk into the head and the rib heads sit on the vertebral bodies, so body and skeleton are each one connected object; the ears keep the leaf outline of 1:50 at the 15 mm feature.", "Die Ohrwurzeln sind in den Kopf eingelassen und die Rippenköpfe sitzen auf den Wirbelkörpern, sodass Körper und Skelett je ein zusammenhängendes Objekt sind; die Ohren behalten den Blattumriss von 1:50 beim Merkmal von 15 mm."),
    "200": ("The upper arm is modelled so that shoulder and forearm meet in every stance; the two claws of a foot are one cloven hoof because a single 23 mm claw is below the 30 mm feature; the trunk hangs from the shoulder blades through a scapulothoracic joint solid, the muscular sling that carries it in the animal.", "Der Oberarm ist modelliert, damit Schulter und Unterarm in jedem Stand zusammentreffen; die zwei Klauen eines Fusses sind ein gespaltener Huf, weil eine einzelne Klaue von 23 mm unter dem Merkmal von 30 mm liegt; der Rumpf hängt über einen Gelenkkörper zwischen Schulterblatt und Brustkorb an den Schulterblättern, der Muskelschlinge, die ihn am Tier trägt."),
    "500": ("Each foot is one solid from fetlock to toe and the ears are true 5 mm sheets; the skeleton keeps the true shafts of spine, ribs, femur, tibia, humerus, radius with olecranon and metatarsus with calcaneus as slender members, merges neck, sacrum, hip bones, metacarpus and digits to the 75 mm feature, and states shoulder blades and ear cartilage as sheets.", "Jeder Fuss ist ein Körper vom Fesselgelenk bis zur Zehe und die Ohren sind echte 5-mm-Blätter; das Skelett behält die wahren Schäfte von Wirbelsäule, Rippen, Oberschenkel, Schienbein, Oberarm, Speiche mit Ellbogenhöcker und Mittelfuss mit Fersenbein als schlanke Stäbe, fasst Hals, Kreuzbein, Hüftbeine, Mittelhand und Zehen auf das Merkmal von 75 mm zusammen und gibt Schulterblätter und Ohrknorpel als Blätter an."),
}


def decisions():
    source = json.load(open(SOURCE, encoding="utf-8"))
    for scale, (en, de) in DECISIONS.items():
        items = source["volumes"][scale]["decisions"]
        if not any(item.get("en") == en for item in items):
            items.append({"en": en, "de": de})
    open(SOURCE, "w", encoding="utf-8").write(json.dumps(source, ensure_ascii=False, separators=(",", ":")))


decisions()
