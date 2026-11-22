"""🩹 kneeling-gardener: hand-chosen corrections of solids that did not meet what carries them (right palm, thumbs, weeds, toes on the pad).

Usage: python3 rework-kneeling-gardener-fixes.py <document>      a candidate asset document or the volumes source; idempotent
"""
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)

KNUCKLE_LINE = (0.643, 0.766, 0.0)
CARPUS_END = (0.014, 0.0, -0.008)
KNUCKLE_MIDDLE = (0.0787, -0.0065, -0.0231)


def unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return [c / n for c in v]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def palm(old):
    """The right metacarpus as a plate from the carpus to the knuckle line of the gripping hand (it was a disc that the index finger did not reach)."""
    along = unit([KNUCKLE_MIDDLE[i] - CARPUS_END[i] for i in range(3)])
    normal = unit(cross(along, KNUCKLE_LINE))
    third = cross(KNUCKLE_LINE, normal)
    rotation = K.r5(K.quaternion([[KNUCKLE_LINE[i], normal[i], third[i]] for i in range(3)]))
    sections = [{"at": list(CARPUS_END), "rx": 0.02, "ry": 0.0065, "exponent": 3, "rotation": rotation}, {"at": list(KNUCKLE_MIDDLE), "rx": 0.037, "ry": 0.0055, "exponent": 3, "rotation": rotation}]
    return {"id": old["id"], "kind": "loft", "bone": old["bone"], "material": old["material"], "role": old["role"], "tone": old["tone"], "sections": sections, "capStart": True, "capEnd": True, "member": old["member"], "part": old["part"]}


def fix(volumes):
    for scale, volume in volumes.items():
        for name in ("body", "thermal", "structure"):
            solids = volume[name]
            for index, solid in enumerate(solids):
                key = (scale, name, solid["id"])
                if key == ("50", "structure", "sk-metacarpus-r") and solid["kind"] == "ellipsoid":
                    solids[index] = palm(solid)
                if key == ("50", "structure", "sk-trowel-tang"):
                    solids[index] = {"id": "sk-trowel-handle", "of": "trowel-handle", "member": "trowel", "part": "member"}
                if key == ("50", "structure", "sk-thumb-l"):
                    solid["p0"] = [0.022, -0.002, -0.056]
                if key == ("50", "structure", "sk-thumb-r"):
                    solid["p0"] = [0.022, 0.008, -0.002]
                if key == ("100", "body", "weed-stalk-1"):
                    solid["p0"] = [0.09, 0.18, 0.09]
                if key == ("200", "body", "weeds"):
                    solid["radii"] = [0.224, 0.054, 0.224]
                if key == ("500", "body", "weeds"):
                    solid["radii"] = [0.218, 0.045, 0.218]
                if scale == "200" and name == "structure" and solid["id"] in ("sk-phalanges-l", "sk-phalanges-r"):
                    solid["sections"][0]["at"][1], solid["sections"][1]["at"][1] = 0.033, 0.029


def main():
    path = sys.argv[1]
    document = json.load(open(path, encoding="utf8"))
    fix(document["asset"]["volumes"] if "asset" in document else document["volumes"])
    json.dump(document, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(path, "a", encoding="utf8").write("\n")
    print("fixed", path)


if __name__ == "__main__":
    main()
