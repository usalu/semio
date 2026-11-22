"""🍑 worker-carrying-box, second round pass 1 (continued): gluteal masses as lofts instead of balls, and a rib cage that reads at every scale.

Body and thermal 1:50: each buttock is a loft through eight hand-chosen sections from the gluteal fold to the iliac crest (pelvis frame: u up, v back, w left), lying inside the breadth of the trouser seat and 23 mm proud of it at the back.
Structure 1:100: all twelve rib pairs, the seven costal cartilages and the costal arches at the 15 mm feature (ribs are 12 to 15 mm deep in reality) instead of four hoops.
Structure 1:200 and 1:500: the rib cage as one closed basket (first rib to costal margin, sections of the bony thorax) instead of three hoops or a wire outline.

Usage: python3 rework-wcb-form.py    rewrites the volumes source; publish with `volumes-insert.py`

@see ./rework-brief.md
"""
import copy
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "worker-carrying-box.json")
LIMB = [0.5, 0.5, 0.5, 0.5]
GLUTEAL_Z = 0.082
GLUTEAL = [
    (-0.115, 0.050, 0.030, 0.050),
    (-0.095, 0.058, 0.066, 0.074),
    (-0.060, 0.062, 0.082, 0.086),
    (-0.020, 0.062, 0.088, 0.090),
    (0.020, 0.058, 0.084, 0.088),
    (0.060, 0.050, 0.070, 0.080),
    (0.095, 0.042, 0.050, 0.066),
    (0.115, 0.036, 0.028, 0.045),
]
CAGE = {
    "200": [
        (0.470, -0.008, 0.034, 0.046),
        (0.445, -0.024, 0.056, 0.082),
        (0.400, -0.034, 0.074, 0.108),
        (0.350, -0.042, 0.088, 0.126),
        (0.300, -0.046, 0.094, 0.137),
        (0.250, -0.046, 0.094, 0.138),
        (0.205, -0.040, 0.086, 0.132),
        (0.165, -0.030, 0.072, 0.120),
    ],
    "500": [
        (0.470, -0.008, 0.038, 0.048),
        (0.430, -0.028, 0.064, 0.094),
        (0.340, -0.044, 0.090, 0.130),
        (0.250, -0.046, 0.094, 0.138),
        (0.165, -0.030, 0.075, 0.120),
    ],
}


def gluteal(old, side):
    sign = 1 if side == "l" else -1
    keep = {key: old[key] for key in old if key not in ("kind", "radii", "transform", "sections", "capStart", "capEnd")}
    grow = old["radii"][0] - 0.078
    return {**keep, "kind": "loft", "sections": [{"at": [x, y, sign * GLUTEAL_Z], "rx": round(rx + grow, 5), "ry": round(ry + grow, 5), "exponent": 2.2, "rotation": LIMB} for x, y, rx, ry in GLUTEAL], "capStart": True, "capEnd": True}


def cage(scale, like):
    keep = {key: like[key] for key in ("bone", "material", "role", "tone", "member", "part") if key in like}
    return {"id": "rib-cage", "kind": "loft", **keep, "sections": [{"at": [x, y, 0], "rx": rx, "ry": ry, "exponent": 2.3, "rotation": LIMB} for x, y, rx, ry in CAGE[scale]], "capStart": True, "capEnd": True}


def thicken(solid, radius):
    solid = copy.deepcopy(solid)
    for section in solid.get("sections", []):
        section["rx"], section["ry"] = max(section["rx"], radius), max(section["ry"], radius)
    for key in ("r0", "r1"):
        if key in solid:
            solid[key] = max(solid[key], radius)
    return solid


def costal(solid):
    return solid["id"].startswith(("rib-", "cartilage-", "costal-arch-", "open-rib-", "open-sternum"))


document = json.load(open(SOURCE, encoding="utf-8"))
volumes = document["volumes"]
for scale in volumes:
    for name in ("body", "thermal", "structure"):
        for index, solid in enumerate(volumes[scale][name]):
            if solid["id"].endswith(("buttock-l", "buttock-r")) and solid.get("kind") == "ellipsoid":
                volumes[scale][name][index] = gluteal(solid, solid["id"][-1])
                print("gluteal", scale, name, solid["id"])
fine = [solid for solid in volumes["50"]["structure"] if costal(solid)]
first = next(index for index, solid in enumerate(volumes["100"]["structure"]) if costal(solid))
rest = [solid for solid in volumes["100"]["structure"] if not costal(solid)]
volumes["100"]["structure"] = rest[:first] + [thicken(solid, 0.0075) for solid in fine] + rest[first:]
print("ribs 100", len(fine))
for scale in ("200", "500"):
    members = volumes[scale]["structure"]
    first = next(index for index, solid in enumerate(members) if costal(solid))
    like = members[first]
    rest = [solid for solid in members if not costal(solid) and not (scale == "200" and solid["id"] == "sternum")]
    volumes[scale]["structure"] = rest[:first] + [cage(scale, like)] + rest[first:]
    print("cage", scale, len(members), "->", len(volumes[scale]["structure"]))
json.dump(document, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
