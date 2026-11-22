"""🛋️ Author 14, three-seat sofa: the structure lists were a frame whose parts did not meet (feet 5 to 15 mm under the arm frames and base rails, deck 5 mm over the rails and beside them, back frame 2 mm over the deck and 30 mm beside the arms, back cushion cores 67 mm in front of the back frame). This sets the members as the sofa is built: arm frames and base rails stand on the feet at 0.100 m, the ply deck lies on top of both base rails, the back frame stands on the deck and is bolted through into the arm back posts, corner blocks reach into the arm frames, a ply face on the back uprights takes the back cushions, whose contact pads reach it through the batting.

Bone frames: `frame` v = y - 0.165, w = z; `arm-*` u = y - 0.100; `seat-n` and `back-n` u = x - bone x, v = y - 0.230.
Run from the ticket folder: python3 rework-14-three-seat-sofa-structure.py
"""
import json

PATH = "volumes/furniture/three-seat-sofa.json"
document = json.load(open(PATH))


def solid(scale, ident):
    return next(s for s in document["volumes"][scale]["structure"] if s["id"] == ident)


def box(scale, ident, size=None, at=None):
    s = solid(scale, ident)
    if size is not None:
        s["size"] = size
    if at is not None:
        s["transform"]["translation"] = at
    return s


for scale, rail_w, post_w in (("50", 0.025, 0.025), ("100", 0.025, 0.03)):
    box(scale, "member-base-rail", [1.96, 0.11, 0.025], [1.0, -0.01, -0.4075])
    box(scale, "member-deck", [1.6, 0.018, 0.85], [1.0, 0.054, 0.005])
    box(scale, "member-back-rail", [1.6, 0.072, 0.04], [1.0, 0.099, 0.42])
    for side, x in (("left", 0.195), ("right", 1.805)):
        box(scale, f"member-arm-{side}-post", [0.48, 0.14, post_w], [0.24, 0.0, -0.4175 if scale == "50" else -0.415])
        box(scale, f"member-arm-{side}-bottom-rail", at=[0.0125 if scale == "50" else 0.015, 0.0, 0.0])
        box(scale, f"joint-corner-{side[0]}", [0.09, 0.04, 0.04], [x, -0.015, -0.375])
        box(scale, f"joint-back-bolt-{side[0]}", [0.09, 0.02, 0.02], [x, 0.285, 0.415])
    box("50", "member-arm-left-panel", [0.43, 0.018, 0.81], [0.24, -0.061, 0.0])
    box("50", "member-arm-right-panel", [0.43, 0.018, 0.81], [0.24, 0.061, 0.0])
    face = 0.008 if scale == "50" else 0.015
    structure = [s for s in document["volumes"][scale]["structure"] if s["id"] != "member-back-face"]
    index = next(i for i, s in enumerate(structure) if s["id"] == "member-top-rail") + 1
    structure.insert(index, {"id": "member-back-face", "kind": "box", "bone": "frame", "material": "birch-plywood", "role": "body", "tone": 0.35, "size": [1.6, 0.46, face], "transform": {"translation": [1.0, 0.365, 0.4 - face / 2]}, "member": "frame", "part": "member"})
    for n, centre in ((1, 0.26665), (2, 0.2667), (3, 0.26665)):
        i = next(k for k, s in enumerate(structure) if s["id"] == f"joint-back-{n}")
        reach = 0.4 - face - 0.325
        structure[i] = {"id": f"joint-back-{n}", "kind": "box", "bone": f"back-{n}", "material": "upholstery-fabric", "role": "detail", "tone": 0.2, "size": [0.16, 0.14, reach], "transform": {"translation": [centre, 0.26, 0.325 + reach / 2]}, "member": f"back-{n}", "part": "joint"}
    document["volumes"][scale]["structure"] = structure

box("200", "member-base-rail", [1.96, 0.098, 0.03], [1.0, -0.016, -0.405])
box("200", "member-deck", [1.6, 0.03, 0.85], [1.0, 0.048, 0.005])
box("200", "member-back-panel", [1.6, 0.592, 0.03], [1.0, 0.359, 0.415])
structure = document["volumes"]["200"]["structure"]
for n, (centre, length) in enumerate(((0.26665, 0.5133), (0.2667, 0.5134), (0.26665, 0.5133)), start=1):
    box("200", f"member-seat-{n}", [length, 0.032, 0.15], [centre, 0.014, -0.2])
    i = next(k for k, s in enumerate(structure) if s["id"] == f"member-back-{n}")
    structure[i] = {"id": f"member-back-{n}", "kind": "box", "bone": f"back-{n}", "material": "hr-foam", "role": "body", "tone": 0.4, "size": [length, 0.15, 0.118], "transform": {"translation": [centre, 0.23, 0.341]}, "member": f"back-{n}", "part": "member"}
    box("500", f"member-back-{n}", [length, 0.15, 0.105], [centre, 0.225, 0.3225])

json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
