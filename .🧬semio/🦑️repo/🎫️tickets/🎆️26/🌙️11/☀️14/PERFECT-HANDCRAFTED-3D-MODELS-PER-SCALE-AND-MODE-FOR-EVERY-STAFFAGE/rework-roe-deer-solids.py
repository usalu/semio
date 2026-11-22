"""🦌 roe-deer: hand edits of the solids per scale (source `volumes/animals/roe-deer.json`), each scale for its own smallest feature.

1:50 structure: the root marker that hung in the air between the hips is gone, the second phalanx bends into the pedal bone, the two pedal bones of a foot meet at their axial faces.
1:100: one cloven hoof per foot instead of two claws below the feature, sacral wings carry the hip bones, rib 5 reaches the spine, the ear cartilage sits on the skull beside the pedicle.
1:200: pastern and hoof at the feature, first rib under the shoulder blade.
1:500: one hoof per foot, antler as one beam, limbs at the drawn widths; skeleton with true slender shafts, one pelvic block per pelvis bone, one digit per foot.

Usage: python3 rework-roe-deer-solids.py   (idempotent; then `python3 volumes-insert.py volumes/animals/roe-deer.json <catalog file>`)
"""
import copy
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "roe-deer.json")
LIMB = [0.5, 0.5, 0.5, 0.5]
SIDES = {"near": -1.0, "far": 1.0}
FEET = [(side, leg) for side in SIDES for leg in ("hind", "fore")]
BONE = {"near": "n", "far": "f", "hind": "h", "fore": "f"}


def find(solids, solid_id):
    return next((solid for solid in solids if solid["id"] == solid_id), None)


def drop(volume, ids):
    for name in ("body", "thermal", "structure"):
        volume[name] = [solid for solid in volume[name] if solid["id"] not in ids and solid.get("of") not in ids]


def radii(solid, values):
    for section, (rx, ry) in zip(solid["sections"], values):
        section["rx"], section["ry"] = rx, ry


def lateral(solid, values):
    for section, w in zip(solid["sections"], values):
        section["at"][2] = w


def bar(solid, stations, rotation=None):
    like = solid["sections"][0]
    solid["sections"] = [{"at": list(at), "rx": rx, "ry": ry, "exponent": 2, "rotation": rotation or like["rotation"]} for at, rx, ry in stations]


def one_hoof(volume, side, leg, widths):
    first, second = f"{side}-{leg}-claw-1", f"{side}-{leg}-claw-2"
    claw = find(volume["body"], first)
    if claw is None or find(volume["body"], second) is None:
        return
    centre = find(volume["body"], f"{side}-{leg}-pastern")["sections"][0]["at"][2]
    lateral(claw, [centre] * len(claw["sections"]))
    radii(claw, widths)
    drop(volume, {second})


def ear_root(structure, side, sign):
    """The ear cartilage starts inside the skull wall (external acoustic meatus) instead of beside it."""
    ear = find(structure, f"{side}-ear-cartilage")
    if ear["sections"][0]["at"][0] > -0.01:
        ear["sections"].insert(0, {**copy.deepcopy(ear["sections"][0]), "at": [-0.012, 0.003, round(-sign * 0.012, 5)]})


def scale_50(volume):
    structure = volume["structure"]
    drop(volume, {"joint-pelvis"})
    for side, sign in SIDES.items():
        ear_root(structure, side, sign)
    for side, leg in FEET:
        sign = SIDES[side]
        for digit, inner in (("1", True), ("2", False)):
            phalanx = find(structure, f"{side}-{leg}-digit-{digit}")
            pedal = find(structure, f"{side}-{leg}-pedal-{digit}")
            w = phalanx["sections"][-1]["at"][2]
            if len(phalanx["sections"]) == 4:
                phalanx["sections"].append({**copy.deepcopy(phalanx["sections"][-1]), "at": [0.064, -0.012, w], "rx": 0.005, "ry": 0.0055})
            centre = (find(structure, f"{side}-{leg}-digit-1")["sections"][0]["at"][2] + find(structure, f"{side}-{leg}-digit-2")["sections"][0]["at"][2]) / 2
            away = 1.0 if w > centre else -1.0
            lateral(pedal, [round(centre + away * 0.0052, 5), round(centre + away * 0.0062, 5)])
            radii(pedal, [(0.0056, 0.0066), (0.0046, 0.0046)])


def scale_100(volume, wings):
    structure = volume["structure"]
    for side, leg in FEET:
        one_hoof(volume, side, leg, [(0.011, 0.008), (0.0135, 0.0098), (0.014, 0.0098), (0.012, 0.008), (0.0078, 0.0076)])
    for side, sign in SIDES.items():
        find(structure, f"{side}-rib-5")["sections"][0]["at"][2] = sign * 0.009
        ear_root(structure, side, sign)
        find(structure, f"{side}-ilium")["sections"][1]["at"] = [0.02, 0.003, sign * 0.004]
    if find(structure, "sacral-wings") is None:
        copy_of = copy.deepcopy(wings)
        radii(copy_of, [(0.009, 0.013), (0.0095, 0.012), (0.011, 0.012), (0.0095, 0.012), (0.009, 0.013)])
        structure.insert(structure.index(find(structure, "sacrum")) + 1, copy_of)


def scale_200(volume):
    structure = volume["structure"]
    for side, leg in FEET:
        one_hoof(volume, side, leg, [(0.01, 0.01), (0.0152, 0.0152), (0.0152, 0.0152), (0.008, 0.008)])
        claw = find(volume["body"], f"{side}-{leg}-claw-1")
        for section, (u, v) in zip(claw["sections"], ((0.066, -0.026), (0.071, -0.019), (0.078, -0.009), (0.084, -0.002))):
            section["at"][0], section["at"][1] = u, v
        radii(find(volume["body"], f"{side}-{leg}-pastern"), [(0.0075, 0.0075), (0.0152, 0.0152), (0.0152, 0.0152), (0.0085, 0.0085)])
    for side, sign in SIDES.items():
        lateral(find(structure, f"{side}-rib-1"), [sign * 0.005, sign * 0.042, sign * 0.022])
        find(structure, f"{side}-scapula")["sections"][0]["at"][2] = round(-sign * 0.016, 5)


def scale_500(volume):
    body, structure = volume["body"], volume["structure"]
    for side, leg in FEET:
        one_hoof(volume, side, leg, [(0.011, 0.0085), (0.014, 0.0105), (0.008, 0.0075)])
        digit = find(structure, f"{side}-{leg}-digit-1")
        if find(structure, f"{side}-{leg}-digit-2") is not None:
            centre = (digit["sections"][0]["at"][2] + find(structure, f"{side}-{leg}-digit-2")["sections"][0]["at"][2]) / 2
            lateral(digit, [centre] * 3)
            radii(digit, [(0.0098, 0.0098)] * 3)
            drop(volume, {f"{side}-{leg}-digit-2"})
    for side, sign in SIDES.items():
        s = BONE[side]
        find(body, f"{side}-ear")["sections"][0]["ry"] = 0.0108
        radii(find(body, f"{side}-antler"), [(0.0146, 0.0146), (0.0146, 0.0146), (0.012, 0.012), (0.0065, 0.0065)])
        drop(volume, {f"{side}-antler-brow", f"{side}-antler-back"})
        find(body, f"{side}-fore-upper")["sections"][2]["rx"] = 0.0262
        find(body, f"{side}-fore-leg")["sections"][1]["rx"] = 0.0166
        radii(find(body, f"{side}-hock"), [(0.012, 0.0125), (0.0162, 0.0162), (0.0162, 0.0162), (0.009, 0.009)])
        find(body, f"{side}-hind-lower")["sections"][1]["rx"] = 0.0162
        for bone in ("femur", "tibia", "metatarsal", "humerus", "radius", "metacarpal", "rib-2", "rib-6", "rib-12", "scapula"):
            find(structure, f"{side}-{bone}")["slender"] = True
        radii(find(structure, f"{side}-humerus"), [(0.0135, 0.014), (0.009, 0.0092), (0.0135, 0.013)])
        radii(find(structure, f"{side}-scapula"), [(0.0042, 0.016), (0.0045, 0.014), (0.0105, 0.013)])
        radii(find(structure, f"{side}-ear-cartilage"), [(0.0105, 0.0105), (0.016, 0.0105), (0.0105, 0.0105)])
        lateral(find(structure, f"{side}-rib-2"), [sign * 0.007, sign * 0.045, sign * 0.056, sign * 0.036, sign * 0.008])
        find(structure, f"{side}-scapula")["sections"][0]["at"][2] = round(-sign * 0.042, 5)
        ilium = find(structure, f"{side}-ilium")
        bar(ilium, [((0, 0, 0), 0.038, 0.038), ((0.045, 0, 0), 0.038, 0.038), ((0.0897, 0, 0), 0.038, 0.038)], LIMB)
    for spine in ("lumbar-spine", "thoracic-spine", "cervical-spine", "mandible"):
        find(structure, spine)["slender"] = True
    find(structure, "thoracic-spine")["sections"][2]["at"][1] = 0.03
    radii(find(structure, "mandible"), [(0.014, 0.012), (0.016, 0.014), (0.012, 0.0095), (0.009, 0.0062), (0.0065, 0.0045)])
    bar(find(structure, "sacrum"), [((0.104, 0, 0), 0.038, 0.038), ((0.05, 0.02, 0), 0.038, 0.038), ((-0.02, 0.04, 0), 0.038, 0.038)])


GIRDLE = {
    "en": " The limbs hang from the girdles: each hip bone runs from the sacroiliac joint beside the sacrum down and out to the hip joint 50 mm beside the median plane and carries the femur, whose stifle stands 18 mm further out; each shoulder blade lies on the rib cage, its dorsal border 45 mm and the shoulder joint 68 mm beside the plane, held to the trunk by muscle only.",
    "de": " Die Gliedmaßen hängen an den Gürteln: jedes Hüftbein läuft vom Kreuzdarmbeingelenk neben dem Kreuzbein nach unten außen zum Hüftgelenk 50 mm neben der Medianebene und trägt den Oberschenkel, dessen Knie 18 mm weiter außen steht; jedes Schulterblatt liegt dem Brustkorb auf, sein Rückenrand 45 mm und das Schultergelenk 68 mm neben der Ebene, am Rumpf nur durch Muskeln gehalten.",
}
COARSE = [
    {
        "en": "Body at 1:500 (smallest feature 75 mm, limbs at their drawn widths): trunk, neck and head are three plain masses; each leg is thigh or upper arm, shank or forearm, cannon, pastern and one hoof per foot, the cleft between the claws is not modelled; each antler is one beam without tines and each ear one leaf, both at the width of their drawing.",
        "de": "Körper bei 1:500 (kleinstes Merkmal 75 mm, Läufe in ihrer gezeichneten Breite): Rumpf, Hals und Kopf sind drei schlichte Massen; jeder Lauf besteht aus Keule oder Oberarm, Unterschenkel oder Unterarm, Röhre, Fessel und einer Schale je Fuß, der Spalt zwischen den Klauen wird nicht modelliert; jede Stange ist ein Spieß ohne Sprossen und jeder Lauscher ein Blatt, beide in der Breite ihrer Zeichnung.",
    },
    None,
    {
        "en": "Structure at 1:500: skull and mandible, the spine as three true slender shafts, three ribs per side, the long bones of the limbs with their true shaft sections, one digit per foot and the pelvis as one block of sacrum and two hip-bone halves on their own bones; the shoulder blades are bars that meet over the withers, because the muscular sling that carries the trunk between them cannot be shown at this scale.",
        "de": "Tragwerk bei 1:500: Schädel und Unterkiefer, die Wirbelsäule als drei wahre schlanke Stäbe, drei Rippen je Seite, die Röhrenknochen der Läufe mit ihren wahren Schaftquerschnitten, ein Zehenstrahl je Fuß und das Becken als ein Block aus Kreuzbein und zwei Hüftbeinhälften an ihren eigenen Knochen; die Schulterblätter sind Stäbe, die sich über dem Widerrist treffen, weil die Muskelschlinge, die den Rumpf zwischen ihnen trägt, in diesem Maßstab nicht darstellbar ist.",
    },
    {
        "en": "Each foot stands on one hoof form that takes both claws; pastern and hoof turn together about the fetlock.",
        "de": "Jeder Fuß steht auf einer Schalenform, die beide Klauen aufnimmt; Fessel und Schale drehen gemeinsam um das Fesselgelenk.",
    },
]


def decisions(volumes):
    for scale in ("50", "100", "200"):
        entry = volumes[scale]["decisions"][2]
        if "sacroiliac" not in entry["en"]:
            entry["en"] += GIRDLE["en"]
            entry["de"] += GIRDLE["de"]
    for index, entry in enumerate(COARSE):
        if entry is not None:
            volumes["500"]["decisions"][index] = entry


def main():
    source = json.load(open(SOURCE, encoding="utf8"))
    volumes = source["volumes"]
    scale_50(volumes["50"])
    scale_100(volumes["100"], find(volumes["50"]["structure"], "sacral-wings"))
    scale_200(volumes["200"])
    scale_500(volumes["500"])
    decisions(volumes)
    open(SOURCE, "w", encoding="utf8").write(json.dumps(source, ensure_ascii=False, separators=(",", ":")) + "\n")
    print({scale: {name: len(volume[name]) for name in ("body", "thermal", "structure")} for scale, volume in volumes.items()})


if __name__ == "__main__":
    main()
