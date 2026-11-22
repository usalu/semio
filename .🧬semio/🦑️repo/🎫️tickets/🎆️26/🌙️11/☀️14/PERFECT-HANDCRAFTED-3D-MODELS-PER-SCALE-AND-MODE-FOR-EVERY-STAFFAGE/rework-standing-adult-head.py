"""💇 standing-adult, second round: the cranium ends at the vertex and the hair is a short cut lying on the skull instead of a ball 21 mm above it: one ellipsoid whose long axis (0.094) follows the hairline plane from the fringe over the forehead to the nape, tilted 49° about the lateral axis, 9 mm above the vertex.

Head frame: u up from the atlanto-occipital joint (vertex 0.149, menton -0.076), v back (glabella -0.100, opisthocranion +0.092), w left (euryon ±0.0745). Radii of the cranium are along (u, v, w); those of the hair along the hairline plane, its normal and w.

Usage: python3 rework-standing-adult-head.py            writes the candidate `🗑️generated/rework-standing-adult/head/🔣️.json`
       python3 rework-standing-adult-head.py --apply    writes the volumes source
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "standing-adult.json")
CATALOG = os.path.abspath(os.path.join(T, *([".."] * 7), "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "standing-adult", "🔣️.json"))
CANDIDATE = os.path.join(T, "🗑️generated", "rework-standing-adult", "head", "🔣️.json")
UPRIGHT = [0, 0, -0.70711, 0.70711]
CRANIUM = {"radii": [0.089, 0.094, 0.0745], "at": [0.060, -0.002, 0.0]}
HAIRLINE = [0, 0, -0.4163, 0.9092]
CAP = ([0.094, 0.082, 0.080], [0.0706, 0.0071, 0.0])


def place(solid, radii, at):
    solid["radii"] = radii
    solid["transform"] = {"translation": at, "rotation": UPRIGHT}


def convert(volumes):
    for scale in (50, 100, 200, 500):
        for name in ("body", "thermal"):
            solids = volumes[str(scale)][name]
            for index, solid in enumerate(list(solids)):
                if solid.get("bone") != "head" or solid.get("kind") != "ellipsoid":
                    continue
                if solid["id"] == "head" and name == "body":
                    place(solid, CRANIUM["radii"], CRANIUM["at"])
                if solid.get("material") == "hair" and solid["radii"][0] > 0.08:
                    place(solid, *CAP)
                    solid["transform"]["rotation"] = HAIRLINE


def main():
    text = open(SOURCE, encoding="utf8").read()
    source = json.loads(text)
    convert(source["volumes"])
    if "--apply" in sys.argv:
        if open(SOURCE, encoding="utf8").read() != text:
            raise SystemExit("the source changed: run again")
        json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        open(SOURCE, "a", encoding="utf8").write("\n")
        print("applied to", SOURCE)
        return
    document = json.load(open(CATALOG, encoding="utf8"))
    document["asset"]["volumes"] = source["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    for scale in ("50", "500"):
        for name in ("body", "thermal"):
            print(scale, name, [(x["id"], x.get("material"), x.get("kind"), x.get("radii"), x.get("of")) for x in source["volumes"][scale][name] if x.get("bone") == "head"])


if __name__ == "__main__":
    main()
