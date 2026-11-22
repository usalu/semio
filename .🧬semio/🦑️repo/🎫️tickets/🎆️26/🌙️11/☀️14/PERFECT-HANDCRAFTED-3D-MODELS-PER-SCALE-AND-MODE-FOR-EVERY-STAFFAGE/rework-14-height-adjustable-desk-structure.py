"""🖥️ Author 14, height-adjustable desk: two open load paths in the structure lists. At 1:50 the contact pads lay under the levelling glides, but the glides were no members, so 3 mm of air separated every foot from its support; at 1:200 the head joint of each column ended 10 mm under the top board (the bracket that carries the board at the finer scales is merged away there). The glides become members of their foot at 1:50, and the head joints of 1:200 reach from the column head to the underside of the board (0.655 to 0.695 m), the height the motor housing has in the body of that scale.

Run from the ticket folder: python3 rework-14-height-adjustable-desk-structure.py
"""
import json

PATH = "volumes/furniture/height-adjustable-desk.json"
document = json.load(open(PATH))

structure = [s for s in document["volumes"]["50"]["structure"] if s["id"] not in ("m-l-glide", "m-r-glide")]
for side, bone in (("l", "foot-left"), ("r", "foot-right")):
    index = next(i for i, s in enumerate(structure) if s["id"] == f"m-{side}-foot")
    structure.insert(index, {"id": f"m-{side}-glide", "of": f"{side}-glide", "member": bone, "part": "member"})
document["volumes"]["50"]["structure"] = structure

for side in ("l", "r"):
    head = next(s for s in document["volumes"]["200"]["structure"] if s["id"] == f"j-{side}-head")
    head["size"] = [0.04, 0.08, 0.15]
    head["transform"]["translation"] = [0.545, 0.0, 0.0]

SENTENCES = {
    "50": (" In the structure the levelling glides are members of their foot, so each foot stands on its contact pad.", " Im Tragwerk sind die Stellgleiter Glieder ihres Fußes, sodass jeder Fuß auf seinem Kontaktfeld steht."),
    "200": (" In the structure the head joint of each column reaches the underside of the board, as the motor housing does in the body.", " Im Tragwerk reicht das Kopfgelenk jeder Säule bis an die Unterseite der Platte, wie das Motorgehäuse im Körper."),
}
for scale, (en, de) in SENTENCES.items():
    decision = document["volumes"][scale]["decisions"][-1]
    if en not in decision["en"]:
        decision["en"] += en
        decision["de"] += de
json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
