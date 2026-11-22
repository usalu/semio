"""🪑️ Author 14, office swivel chair: the structure list carried the arm bosses, a stem joint, an axle joint and a ground contact per castor, but not the castor itself, so the load path from the arm to the floor was open (axle joint and contact pad hung in the air). This adds the swivel head, the centre web that carries the axle and the two wheels (rim and tyre) of every castor as members of the castor bone, by reference to the body solids of the same scale, in front of the castor joints.

Run from the ticket folder: python3 rework-14-office-swivel-chair-structure.py
"""
import json

PATH = "volumes/furniture/office-swivel-chair.json"
CASTORS = ["front", "side-a", "side-b", "back-a", "back-b"]
PARTS = {
    "50": ["neck", "head", "web", "rim-l", "tyre-l", "rim-r", "tyre-r"],
    "100": ["head", "wheel-l", "wheel-r"],
    "200": ["wheel"],
}

document = json.load(open(PATH))
for scale, parts in PARTS.items():
    volume = document["volumes"][scale]
    body = {solid["id"] for solid in volume["body"]}
    structure = [solid for solid in volume["structure"] if not solid["id"].startswith("member-castor-")]
    for castor in CASTORS:
        members = []
        for part in parts:
            of = f"castor-{castor}-{part}"
            assert of in body, of
            members.append({"id": f"member-{of}", "of": of, "member": f"castor-{castor}", "part": "member"})
        index = next(i for i, solid in enumerate(structure) if solid["id"] in (f"joint-castor-{castor}-stem", f"support-contact-castor-{castor}"))
        structure[index:index] = members
    volume["structure"] = structure
json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
