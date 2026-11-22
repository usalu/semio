"""🦴 Chisel of the goat author: the load path of the skeleton of `animals/goat` closed where bones meet.

Sacral wings carry the ilia, the pubic rami meet at the symphysis, the symphyseal tendon carries the median
suspensory lamina of the udder, the scapular cartilage reaches its sling at the withers, the sternum takes the
costal cartilages and the first caudal vertebra sits on the sacrum. Radii per scale are the smallest feature or the bone.
"""
import json
import sys

SOURCE = "volumes/animals/goat.json"
RADIUS = {"50": (0.011, 0.008, 0.006), "100": (0.011, 0.008, 0.0078), "200": (0.0155, 0.0155, 0.0155), "500": (0.0378, 0.0378, 0.0378)}


def get(listing, ident):
    return next((e for e in listing if e.get("id") == ident), None)


def bone_solid(ident, p0, p1, radius, member="pelvis"):
    return {"id": ident, "kind": "capsule", "bone": "pelvis", "material": "bone", "role": "body", "tone": 0.9, "p0": p0, "p1": p1, "r0": radius, "r1": radius, "member": member, "part": "member"}


def put(structure, entry, after):
    structure[:] = [e for e in structure if e.get("id") != entry["id"]]
    index = next((i for i, e in enumerate(structure) if e.get("id") == after), len(structure) - 1)
    structure.insert(index + 1, entry)


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    for scale, (wing, ramus, tendon) in RADIUS.items():
        structure = document["volumes"][scale]["structure"]
        anchor = "sacrum" if get(structure, "sacrum") else "pelvis-girdle"
        put(structure, bone_solid("sacral-wings", [0.112, 0.01, -0.058], [0.112, 0.01, 0.058], wing), anchor)
        put(structure, bone_solid("pubis-n", [0.003, 0.004, -0.062], [-0.008, -0.046, -0.004], ramus), "sacral-wings")
        put(structure, bone_solid("pubis-f", [0.003, 0.004, 0.062], [-0.008, -0.046, 0.004], ramus), "pubis-n")
        put(structure, bone_solid("symphyseal-tendon", [-0.008, -0.046, 0], [-0.0103, -0.166, 0], tendon, "mammary-suspension"), "pubis-f")
        get(structure, "symphyseal-tendon")["material"] = "suspensory-tissue"
        tail = get(structure, "caudal-1")
        if tail and "of" not in tail:
            tail["p0"][0] = -0.034
        for index in range(1, 5):
            sternum = get(structure, "sternum-%d" % index)
            if sternum:
                sternum["r0"] = max(sternum["r0"], 0.0145)
                sternum["r1"] = max(sternum["r1"], 0.0145)
        for side, sign in (("n", 1), ("f", -1)):
            blade = get(structure, "scapula-" + side)
            if scale == "50" and blade["sections"][0]["at"][0] > 0.01:
                first = json.loads(json.dumps(blade["sections"][0]))
                first["at"] = [0.002, -0.004, 0.004 * sign]
                first["rx"] = 0.024
                blade["sections"].insert(0, first)
            patella = get(structure, "patella-" + side)
            if patella:
                patella["transform"]["translation"][1] = 0.0225
            sling = get(structure, "joint-scapula-f" + side)
            if sling:
                sling["p0"][2] = -0.006 * sign
                sling["p1"][2] = 0.038 * sign
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] load path written", file=sys.stderr)


main()
