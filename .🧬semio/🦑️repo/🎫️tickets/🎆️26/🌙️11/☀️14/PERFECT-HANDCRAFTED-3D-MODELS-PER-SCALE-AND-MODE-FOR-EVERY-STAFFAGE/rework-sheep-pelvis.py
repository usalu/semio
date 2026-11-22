"""🐑 Sheep skeleton connections: the hip bone runs from the sacroiliac joint to the hip joint with its wing out to the tuber coxae, the pubis reaches the acetabulum, the scapula lies on the ribs, the rib heads of 1:100 reach the spine; hand-chosen stations in the pelvis frame, written to the volumes source.

Usage: python3 rework-sheep-pelvis.py
"""
import copy
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "sheep.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/sheep/🔣️.json"
K = {"__file__": os.path.join(T, "rig-lateral-standing-adult.py")}
exec(open(K["__file__"], encoding="utf-8").read().split("def convert")[0], K)
BODY_50 = [(0.112, 0.014, 0.022, 0.008, 0.022), (0.085, 0.010, 0.046, 0.008, 0.020), (0.045, 0.006, 0.064, 0.009, 0.013), (0.001, 0.0017, 0.084, 0.016, 0.020)]
WING_50 = [(0.100, 0.010, 0.040, 0.022, 0.007), (0.122, 0.012, 0.072, 0.020, 0.007), (0.142, 0.0093, 0.103, 0.014, 0.009)]
HIP_BONE_COARSE = [(0.105, 0.012, 0.024), (0.001, 0.0017, 0.084), (-0.0713, 0.0559, 0.070)]
SCAPULA_TO_RIBS = {"50": 0.0065, "200": 0.007, "500": 0.012}


def dump():
    names = {}
    exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("def member_end")[0], names)
    return names["dump"]


def main():
    frames = K["frames"](json.load(open(CATALOG, encoding="utf-8"))["asset"]["rig"]["bones"])
    source = json.load(open(SOURCE, encoding="utf-8"))
    volumes = source["volumes"]
    if any(solid["id"] == "near-ilium-wing" for solid in volumes["50"]["structure"]):
        raise SystemExit("already applied")
    pelvis = frames["pelvis"]

    def in_ilium(bone, point):
        world = [pelvis[0][i] + K["apply"](pelvis[1], list(point))[i] for i in range(3)]
        return K["r5"](K["apply"](K["transpose"](frames[bone][1]), [world[i] - frames[bone][0][i] for i in range(3)]))

    for side, sign, bone in (("near", -1, "ilium-hn"), ("far", 1, "ilium-hf")):
        across = K["r5"](K["quaternion"](K["mul"](K["transpose"](frames[bone][1]), pelvis[1])))
        structure = volumes["50"]["structure"]
        body = next(solid for solid in structure if solid["id"] == f"{side}-ilium")
        for part, (x, y, z, rx, ry) in zip(body["sections"], BODY_50):
            part["at"], part["rx"], part["ry"] = in_ilium(bone, (x, y, sign * z)), rx, ry
        wing = copy.deepcopy(body)
        wing["id"] = f"{side}-ilium-wing"
        template = body["sections"][0]
        wing["sections"] = [dict(template, at=in_ilium(bone, (x, y, sign * z)), rx=rx, ry=ry, rotation=across) for x, y, z, rx, ry in WING_50]
        structure.insert(structure.index(body) + 1, wing)
        for scale, name in (("100", "hip-bone"), ("200", "hip-bone"), ("500", "ilium")):
            solid = next(item for item in volumes[scale]["structure"] if item["id"] == f"{side}-{name}")
            solid["sections"] = solid["sections"][:3]
            for part, (x, y, z) in zip(solid["sections"], HIP_BONE_COARSE):
                part["at"] = in_ilium(bone, (x, y, sign * z))
        for scale, shift in SCAPULA_TO_RIBS.items():
            for solid in volumes[scale]["structure"]:
                if solid.get("bone") == "scapula-f" + side[0] and solid.get("part") == "member" and solid["kind"] == "loft":
                    for part in solid["sections"]:
                        part["at"][2] = round(part["at"][2] - sign * shift, 5)
        for rib in ("rib-9", "rib-11"):
            solid = next(item for item in volumes["100"]["structure"] if item["id"] == f"{side}-{rib}")
            head = copy.deepcopy(solid["sections"][0])
            head["at"][2] = 0.0
            solid["sections"].insert(0, head)
    pubis = next(solid for solid in volumes["50"]["structure"] if solid["id"] == "pubis")
    pubis["sections"][0]["at"] = [-0.002, -0.012, -0.078]
    pubis["sections"][1]["at"] = [-0.006, -0.028, 0.0]
    pubis["sections"][2]["at"] = [-0.002, -0.012, 0.078]
    for solid in volumes["50"]["structure"]:
        if solid["id"].startswith("joint-hoof-"):
            solid["radii"] = [0.006, 0.006, 0.006]
    open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + dump()(volumes, 0) + "}\n")


main()
