"""🔗 machinery/heat-pump-outdoor-unit: every part is carried by what carries it in the real unit (`volumes-connected`, measured after the fan blades were declared).

1:50: the pilot valve sits on the body of the reversing valve (it hung 4 mm beside it); the lid hinge is a complete load path (bracket on the rear top rail, casing knuckle, lid knuckle, hinge leaf, lid).
1:100: the expansion valve is clipped to the partition by a 30 mm bracket (its 10 mm lines are left to 1:50); the eight grille wires start in the cap; the lid hinge as at 1:50.
1:200: the grille cap is carried by four 30 mm spokes typified from the eight wires (it hung 6 mm in front of the turning hub).
One-shot on `volumes/machinery/heat-pump-outdoor-unit.json`; publish with `volumes-insert.py`.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "machinery", "heat-pump-outdoor-unit.json")
DECISIONS = {
    "50": {"en": "The pilot valve sits on the reversing valve; each lid hinge is a full load path of bracket, casing knuckle, lid knuckle and leaf.", "de": "Das Pilotventil sitzt auf dem Umkehrventil; jedes Haubenscharnier ist ein vollständiger Lastpfad aus Halter, Gehäuserolle, Haubenrolle und Lappen."},
    "100": {"en": "The expansion valve is clipped to the partition by a bracket because its 10 mm lines are left to 1:50; the grille wires start in the cap; each lid hinge is a load path of bracket, knuckle and leaf.", "de": "Das Expansionsventil ist mit einem Halter an die Trennwand geklemmt, weil seine 10-mm-Leitungen dem Maßstab 1:50 bleiben; die Gitterdrähte beginnen in der Kappe; jedes Haubenscharnier ist ein Lastpfad aus Halter, Rolle und Lappen."},
    "200": {"en": "Four 30 mm spokes, typified from the eight wires, carry the grille cap on the rim.", "de": "Vier 30-mm-Speichen, typisiert aus den acht Drähten, tragen die Gitterkappe am Ring."},
}


def index(solids, ident):
    """🔎 Position of the solid with this id."""
    return next(i for i, solid in enumerate(solids) if solid["id"] == ident)


def insert(solids, after, solid):
    """📌 Puts a solid behind another one unless its id is present."""
    if all(item["id"] != solid["id"] for item in solids):
        solids.insert(index(solids, after) + 1, solid)


def main():
    """▶️ Applies the hand-chosen joins to the three scales."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    volumes = document["volumes"]

    fine = volumes["50"]
    for name in ("body", "thermal"):
        pilot = next(solid for solid in fine[name] if solid["id"].endswith("reversing-valve-pilot"))
        pilot["transform"]["translation"] = [0.04, 0.74, 0.39]
    insert(fine["structure"], "hinge-casing", {"id": "hinge-bracket-member", "of": "hinge-brackets", "member": "unit", "part": "member"})
    insert(fine["structure"], "lid-skirt-member", {"id": "lid-hinge-leaf-member", "of": "lid-hinge-leaves", "member": "lid", "part": "member"})

    medium = volumes["100"]
    bracket = {"id": "expansion-valve-bracket", "kind": "box", "bone": "unit", "material": "galvanised-steel", "role": "detail", "tone": 0.3, "size": [0.03, 0.03, 0.03], "transform": {"translation": [0.12, 0.6225, 0.2725]}}
    insert(medium["body"], "expansion-valve", bracket)
    hidden = dict(bracket, id="th-expansion-valve-bracket", region="suction")
    medium["thermal"] = [solid for solid in medium["thermal"] if solid["id"] != hidden["id"]]
    insert(medium["thermal"], "th-expansion-valve", hidden)
    for name in ("body", "thermal"):
        spokes = next(solid for solid in medium[name] if solid["id"].endswith("guard-spokes"))
        spokes["p0"] = [0.2304, 0.852, -0.15]
    insert(medium["structure"], "hinge-lid", {"id": "hinge-bracket-member", "of": "hinge-brackets", "member": "unit", "part": "member"})
    insert(medium["structure"], "lid-skirt-member", {"id": "lid-hinge-leaf-member", "of": "lid-hinge-leaves", "member": "lid", "part": "member"})

    coarse = volumes["200"]
    spokes = {"id": "guard-spokes", "kind": "capsule", "bone": "unit", "material": "galvanised-steel", "role": "detail", "tone": 0.3, "p0": [0.22, 0.85, -0.15], "p1": [0.22, 1.1, -0.15], "r0": 0.015, "r1": 0.015,
              "repeat": {"count": 4, "rotationRad": 1.5708, "rotationAxis": [1, 0, 0], "origin": [0, 0.8, -0.15]}}
    insert(coarse["body"], "guard-rim", spokes)
    insert(coarse["thermal"], "th-guard-rim", {"id": "th-guard-spokes", "of": "guard-spokes", "region": "fan-guard"})

    for scale, sentence in DECISIONS.items():
        if sentence not in volumes[scale]["decisions"]:
            volumes[scale]["decisions"].append(sentence)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1)
    print("applied")


if __name__ == "__main__":
    main()
