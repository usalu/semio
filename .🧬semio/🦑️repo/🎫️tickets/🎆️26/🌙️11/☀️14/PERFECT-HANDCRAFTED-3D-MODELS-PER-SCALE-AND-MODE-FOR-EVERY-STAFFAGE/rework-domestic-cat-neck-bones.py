"""🐈 domestic-cat: the neck slimmed to a real cat's neck (girth 0.21 m behind the head instead of a 0.27 m ball) at every scale in body and thermal, and the 1:200 skeleton given true bone sections (`slender`) instead of 30 mm sausages: femur, tibia, humerus, radius with ulna, cervical and lumbar spine, caudal vertebrae.

One-shot on the volumes source (`volumes/animals/domestic-cat.json`); a second run is refused. Publish with `volumes-insert.py`.
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "domestic-cat.json")
SIDE = [0, -0.70711, 0, 0.70711]
NECK_WIDTH = [(-0.03, 0.94), (0.0, 0.88), (0.022, 0.87), (0.045, 0.886), (0.065, 0.94), (0.082, 1.0)]
NECK_DEPTH = [(-0.03, 1.0), (0.0, 0.98), (0.022, 0.91), (0.045, 0.88), (0.065, 0.97), (0.082, 1.0)]
LONG = {
    "femur": ("thigh-h", [([-0.004, 0, 0], 0.009, 0.008), ([0.05, 0, 0.009], 0.0048, 0.0048), ([0.099, 0.001, 0.012], 0.0095, 0.009)]),
    "tibia": ("shank-h", [([0, 0, 0], 0.0095, 0.009), ([0.055, 0, -0.004], 0.0042, 0.0045), ([0.105, 0, -0.008], 0.0065, 0.0055)]),
    "humerus": ("humerus-f", [([0, 0, -0.003], 0.0085, 0.0085), ([0.045, 0, -0.001], 0.0045, 0.0048), ([0.091, 0, 0], 0.0088, 0.007)]),
    "radius-ulna": ("forearm-f", [([-0.012, -0.004, 0], 0.005, 0.006), ([0, 0, 0], 0.007, 0.009), ([0.042, 0, -0.003], 0.006, 0.005), ([0.0841, 0, -0.006], 0.008, 0.0055)]),
}
AXIAL = {
    "cervical-spine": [([-0.006, 0, 0], 0.0085, 0.007), ([0.036, 0, 0], 0.0085, 0.007), ([0.076, 0, 0], 0.009, 0.0075)],
    "lumbar-spine": [([-0.008, -0.003, 0], 0.0085, 0.009), ([0.066, -0.003, 0], 0.0095, 0.01), ([0.141, -0.002, 0], 0.0085, 0.009)],
    "caudal-1": [([-0.006, 0, 0], 0.006, 0.006), ([0.032, 0, 0], 0.0055, 0.0055), ([0.068, 0, 0], 0.005, 0.005)],
    "caudal-2": [([-0.004, 0, 0], 0.005, 0.005), ([0.032, 0, 0], 0.005, 0.005), ([0.068, 0, 0], 0.0045, 0.0045)],
    "caudal-3": [([-0.004, 0, 0], 0.0045, 0.0045), ([0.032, 0, 0], 0.0045, 0.0045), ([0.067, 0, 0], 0.004, 0.004)],
    "caudal-4": [([-0.004, 0, 0], 0.004, 0.004), ([0.03, 0, 0], 0.004, 0.004), ([0.063, 0, 0], 0.0035, 0.0035)],
    "caudal-5": [([-0.008, 0, 0], 0.0035, 0.0035), ([0.025, 0, 0], 0.003, 0.003), ([0.054, 0, 0], 0.002, 0.002)],
}


def factor(table, x):
    if x <= table[0][0]:
        return table[0][1]
    for (x0, f0), (x1, f1) in zip(table, table[1:]):
        if x <= x1:
            return f0 + (f1 - f0) * (x - x0) / (x1 - x0)
    return table[-1][1]


def sections(rows, sign, rotation):
    return [{"at": [at[0], at[1], sign * at[2] + 0.0], "rx": rx, "ry": ry, "exponent": 2.2, "rotation": rotation} for at, rx, ry in rows]


def main():
    document = json.load(open(SOURCE, encoding="utf8"))
    volumes = document["volumes"]
    if any(solid["id"] == "far-femur" and solid.get("slender") for solid in volumes["200"]["structure"]):
        raise SystemExit("already applied")
    for scale, volume in volumes.items():
        for name in ("body", "thermal"):
            for solid in volume[name]:
                if solid.get("bone") == "neck" and solid["kind"] == "loft" and solid["id"] in ("neck", "th-neck"):
                    for section in solid["sections"]:
                        section["rx"] = round(section["rx"] * factor(NECK_WIDTH, section["at"][0]), 5)
                        section["ry"] = round(section["ry"] * factor(NECK_DEPTH, section["at"][0]), 5)
                    print("[DEBUG] neck", scale, name, solid["id"], [(s["at"][0], s["rx"], s["ry"]) for s in solid["sections"]])
    done = []
    for solid in volumes["200"]["structure"]:
        for side, sign in (("far", 1), ("near", -1)):
            for bone_name, (bone, rows) in LONG.items():
                if solid["id"] == f"{side}-{bone_name}":
                    assert solid["bone"] == bone + side[0], solid
                    solid["sections"] = sections(rows, sign, SIDE)
                    solid["slender"] = True
                    done.append(solid["id"])
        if solid["id"] in AXIAL:
            rotation = solid["sections"][len(solid["sections"]) // 2]["rotation"]
            solid["sections"] = sections(AXIAL[solid["id"]], 1, rotation)
            solid["slender"] = True
            done.append(solid["id"])
    print("[DEBUG] 1:200 true sections", done)
    if len(done) != 15:
        raise SystemExit("not all bones found")
    if "--apply" in sys.argv:
        json.dump(document, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        open(SOURCE, "a", encoding="utf8").write("\n")
        print("written", SOURCE)


if __name__ == "__main__":
    main()
