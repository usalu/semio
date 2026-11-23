"""🗑️ objects/waste-bin: the lid of 1:200 and 1:500 is modelled for its scale again, and the moulding is one load path at 1:100, 1:200 and 1:500.

A later pass had put the 1:50 lid parts (4 mm dish, 9 mm end walls, 14 mm hinge rail in three pieces) back into the coarse lists, against their own decisions. 1:200: the lid is the domed dish over the full width (its shell is judged as a hollow part), closed by two 30 mm end blocks that take in the rail corners and by a 30 mm hinge rail between the lug notches; the two 63 mm notches stay open, they are the real air path into the bin (`thermal.minimumAirPassageM`). 1:500: the lid is the plate 0.52 x 0.04 x 0.5 m that the drawing shows. The U skirt stays (30 mm deep, drawn in the plan).
Structure: the chamfered shoulder joins foot skirt and wall, bead and seat join wall and lid (they were body parts without a member, so the wall hung 8 mm above the foot skirt and the lid 11 to 40 mm above the wall).
One-shot on `volumes/objects/waste-bin.json`; publish with `volumes-insert.py`.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "objects", "waste-bin.json")
GONE = ("lid-end-l", "lid-end-r", "lid-rail", "lid-rail-l", "lid-rail-r")
RAIL = [[0.03, 0.0306], [0.02208, 0.02821], [0.01269, 0.02387], [0.00576, 0.01874], [0.00147, 0.01278], [0.0, 0.006], [0.0, 0.0], [0.03, 0.0]]
DECISIONS = {
    "100": {"en": "Structure: the chamfered shoulder is a member too, so foot skirt, wall and floor are one moulding down to the four pads.", "de": "Tragwerk: Auch die gefaste Schulter ist ein Glied, sodass Fußschürze, Wand und Boden bis zu den vier Polstern ein Formteil sind."},
    "200": {"en": "The lid is the domed dish over the full 0.5 m width, closed by two 30 mm end blocks that take in the rail corners and by a 30 mm hinge rail between the two lug notches; the 63 mm notches stay open as the air path into the bin. Structure: shoulder, rim bead, hinge rail and end blocks are members, so slab, wall, rim and lid are one load path.", "de": "Der Deckel ist die gewölbte Schale über die volle Breite von 0,5 m, geschlossen durch zwei 30-mm-Stirnblöcke, die die Leistenecken aufnehmen, und eine 30-mm-Scharnierleiste zwischen den zwei Laschenausschnitten; die 63-mm-Ausschnitte bleiben als Luftweg in den Behälter offen. Tragwerk: Schulter, Randwulst, Scharnierleiste und Stirnblöcke sind Glieder, sodass Platte, Wand, Rand und Deckel ein Lastpfad sind."},
    "500": {"en": "The lid is the plate of 0.52 x 0.04 x 0.5 m over its U skirt. Structure: shoulder, rim bead and seat are members, so plinth, wall and lid are one load path.", "de": "Der Deckel ist die Platte 0,52 x 0,04 x 0,5 m über seiner U-Schürze. Tragwerk: Schulter, Randwulst und Sitz sind Glieder, sodass Sockel, Wand und Deckel ein Lastpfad sind."},
}


def member(ident, of):
    """🧱 A member of the bin moulding by reference to its body solid."""
    return {"id": ident, "of": of, "member": "body", "part": "member"}


def add(structure, after, solid):
    """📌 Inserts a structure solid behind another one unless it is there."""
    if all(item["id"] != solid["id"] for item in structure):
        structure.insert(next(i for i, item in enumerate(structure) if item["id"] == after) + 1, solid)


def main():
    """▶️ Rewrites the lid of the two coarse scales and completes the three structure lists."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    volumes = document["volumes"]
    coarse = volumes["200"]
    body = {solid["id"]: solid for solid in coarse["body"]}
    body["lid-cap"].update({"from": -0.25, "to": 0.25})
    body["lid-end-l"].update({"from": 0.22, "to": 0.25})
    body["lid-end-r"].update({"from": -0.25, "to": -0.22})
    body["lid-rail"].update(profile=RAIL)
    coarse["body"] = [solid for solid in coarse["body"] if solid["id"] not in ("lid-rail-l", "lid-rail-r")]
    coarse["thermal"] = [solid for solid in coarse["thermal"] if solid.get("of") not in ("lid-rail-l", "lid-rail-r")]
    for ident in ("lid-rail", "lid-end-l", "lid-end-r"):
        add(coarse["structure"], "member-lid", {"id": "member-" + ident, "of": ident, "member": "lid", "part": "member"})
    coarse["decisions"][0] = {
        "en": "Body 1:200 authored for 30 mm: pedestal, feet and floor read as one slab, one wall, bead and seat as one 40 mm rim ring; bands, lugs, hinge, handle and bag are not drawn.",
        "de": "Körper 1:200 für 30 mm erstellt: Sockel, Füße und Boden lesen sich als eine Platte, eine Wand, Wulst und Sitz als ein 40-mm-Randring; Bänder, Laschen, Scharnier, Griff und Sack werden nicht gezeichnet.",
    }
    v = volumes["500"]
    cap = next(solid for solid in v["body"] if solid["id"] == "lid-cap")
    for key in ("profile", "plane", "from", "to"):
        cap.pop(key, None)
    cap.update(kind="box", size=[0.52, 0.04, 0.5], transform={"translation": [0.26, 0.02, 0]})
    v["body"] = [solid for solid in v["body"] if solid["id"] not in GONE]
    v["thermal"] = [solid for solid in v["thermal"] if solid.get("of") not in GONE]
    v["structure"] = [solid for solid in v["structure"] if solid.get("of") not in GONE]
    next(solid for solid in v["thermal"] if solid["id"] == "th-lid-cap")["sectionFaces"] = ["-v"]
    add(volumes["100"]["structure"], "member-pedestal", member("member-pedestal-shoulder", "pedestal-shoulder"))
    add(volumes["200"]["structure"], "member-slab", member("member-pedestal-shoulder", "pedestal-shoulder"))
    add(volumes["200"]["structure"], "member-rim", member("member-rim-bead", "rim-bead"))
    add(volumes["500"]["structure"], "member-slab", member("member-pedestal-shoulder", "pedestal-shoulder"))
    add(volumes["500"]["structure"], "member-pedestal-shoulder", member("member-rim-bead", "rim-bead"))
    add(volumes["500"]["structure"], "member-rim-bead", member("member-rim-seat", "rim-seat"))
    for scale, sentence in DECISIONS.items():
        if sentence not in volumes[scale]["decisions"]:
            volumes[scale]["decisions"].append(sentence)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1)
    print("applied")


if __name__ == "__main__":
    main()
