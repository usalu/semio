"""🐴 Horse 1:50: skull as cranium, face, zygomatic arches and facial crests; nuchal bridge that keeps withers and crest closed when the neck drops."""
import json

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
SOURCE = f"{T}/volumes/animals/horse.json"
UP = [0.5, 0.5, 0.5, 0.5]
BONE = {"bone": "head", "material": "short-coat", "role": "body", "tone": 0.85}
MEMBER = {"member": "head", "part": "member"}
FACE = [
    (0.0, -0.025, 0.045, 0.045), (0.05, -0.03, 0.068, 0.06), (0.1, -0.05, 0.085, 0.076), (0.2, -0.066, 0.088, 0.092),
    (0.28, -0.066, 0.082, 0.07), (0.38, -0.052, 0.062, 0.048), (0.48, -0.04, 0.04, 0.034), (0.545, -0.04, 0.032, 0.036), (0.578, -0.042, 0.024, 0.03),
]


def skull():
    solids = [
        {"id": "skull", "kind": "loft", **BONE, "sections": [{"at": [x, y, 0.0], "rx": rx, "ry": ry, "exponent": 2.5, "rotation": UP} for x, y, rx, ry in FACE], "capStart": True, "capEnd": True, **MEMBER},
        {"id": "cranium", "kind": "ellipsoid", **BONE, "radii": [0.085, 0.07, 0.06], "transform": {"translation": [0.07, -0.03, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0]}, **MEMBER},
        {"id": "nuchal-crest", "kind": "capsule", **BONE, "p0": [0.0, -0.004, -0.045], "p1": [0.0, -0.004, 0.045], "r0": 0.016, "r1": 0.016, **MEMBER},
    ]
    for name, sign in (("near", -1), ("far", 1)):
        solids += [
            {"id": f"{name}-zygomatic-arch", "kind": "capsule", **BONE, "p0": [0.1, -0.075, sign * 0.078], "p1": [0.23, -0.078, sign * 0.097], "r0": 0.011, "r1": 0.011, **MEMBER},
            {"id": f"{name}-facial-crest", "kind": "capsule", **BONE, "p0": [0.23, -0.078, sign * 0.097], "p1": [0.34, -0.08, sign * 0.066], "r0": 0.011, "r1": 0.008, **MEMBER},
        ]
    return solids


BRIDGE = {"id": "nuchal-bridge", "kind": "ellipsoid", "bone": "thorax", "material": "short-coat", "role": "body", "tone": 0.62, "radii": [0.19, 0.185, 0.115], "transform": {"translation": [0.82, 0.1, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0]}}


def main():
    document = json.load(open(SOURCE))
    fifty = document["volumes"]["50"]
    structure = fifty["structure"]
    at = next(i for i, s in enumerate(structure) if s["id"] == "skull")
    structure[:] = [s for s in structure if s["id"] not in {x["id"] for x in skull()}]
    structure[at:at] = skull()
    body = [s for s in fifty["body"] if s["id"] != BRIDGE["id"]]
    at = next(i for i, s in enumerate(body) if s["id"] == "withers") + 1
    body[at:at] = [BRIDGE]
    fifty["body"] = body
    json.dump(document, open(SOURCE, "w"), ensure_ascii=False, indent=1)


main()
