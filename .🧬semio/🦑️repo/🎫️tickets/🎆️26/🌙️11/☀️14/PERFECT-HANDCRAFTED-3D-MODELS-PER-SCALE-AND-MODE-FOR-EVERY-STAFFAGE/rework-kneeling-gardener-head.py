"""🗿 kneeling-gardener: the head hangs from the atlanto-occipital joint instead of from a pivot at chin level.

The head bone starts at the tip of the cervical column. All head geometry (solids of every list and scale, 2D shapes whose first coordinate is always the bone axis, anchors) was modelled with the chin 18 mm above that pivot, which put the chin 164 mm above the jugular notch and made the figure 1.73 m instead of the stated 1.66 m. The skull base lies 65 mm above the chin, so everything bound to the head moves 65 mm down its axis; the joint solids at the pivot stay. The centre of mass is set 97 mm above the chin, 50 mm above the joint.

Usage: python3 rework-kneeling-gardener-head.py            candidate document
       python3 rework-kneeling-gardener-head.py --apply    catalog file (dimensions, rig, anchors, representations; volumes through volumes-insert.py) and volumes source
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "kneeling-gardener", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "kneeling-gardener.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-kneeling-gardener", "candidate", "🔣️.json")
DROP = 0.065
HEAD_LENGTH = 0.15
HEAD_COM = 0.05
MANDIBLE_UP = 0.003
HEIGHT = 1.033


def down(value):
    return round(value - DROP, 5)


def solid_down(solid):
    if "transform" in solid:
        translation = solid["transform"].setdefault("translation", [0, 0, 0])
        translation[0] = down(translation[0])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"][0] = down(section["at"][0])
    elif solid["kind"] == "capsule":
        solid["p0"][0], solid["p1"][0] = down(solid["p0"][0]), down(solid["p1"][0])
    elif solid["kind"] == "revolve":
        solid["origin"][0] = down(solid["origin"][0])
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not moved")


def convert(asset):
    head = next(bone for bone in asset["rig"]["bones"] if bone["id"] == "head")
    if head["lengthM"] == HEAD_LENGTH:
        raise SystemExit("already converted")
    head["lengthM"] = HEAD_LENGTH
    head["mass"]["comX"] = HEAD_COM
    for anchor in asset["anchors"]:
        if anchor["bone"] == "head":
            anchor["x"] = down(anchor["x"])
    for representation in asset["representations"].values():
        for view, shapes in representation.get("geometry", {}).items():
            for shape in shapes:
                if shape.get("bone") != "head":
                    continue
                if shape["kind"] != "poly":
                    raise SystemExit(f"shape {shape['id']} of kind {shape['kind']} is not moved")
                for point in shape["points"]:
                    point[0] = down(point[0])
    for volume in asset["volumes"].values():
        for name in ("body", "thermal", "structure"):
            for solid in volume[name]:
                if solid.get("bone") == "head" and solid.get("part") != "joint":
                    solid_down(solid)
                    if solid["id"] == "sk-mandible":
                        solid["transform"]["translation"][0] = round(solid["transform"]["translation"][0] + MANDIBLE_UP, 5)
    old = asset["dimensions"]["heightM"]
    asset["dimensions"]["heightM"] = HEIGHT
    text = json.dumps(asset["symbols"], ensure_ascii=False)
    for before, after in ((str(old) + "]", str(HEIGHT) + "]"), (str(old) + " m", str(HEIGHT) + " m"), (str(old).replace(".", ",") + " m", str(HEIGHT).replace(".", ",") + " m")):
        text = text.replace(before, after)
    asset["symbols"] = json.loads(text)


def main():
    original = open(CATALOG, encoding="utf8").read()
    document = json.loads(original)
    asset = document["asset"]
    convert(asset)
    if "--apply" not in sys.argv:
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    text = open(os.path.join(T, "volumes-insert.py"), encoding="utf8").read()
    insert = {}
    exec(compile(text[: text.index("\ndef main(")], "volumes-insert.py", "exec"), insert)
    result = original
    for key in ("dimensions", "rig", "anchors", "representations", "symbols"):
        start, end = insert["member_range"](result, "    ", key)
        result = result[:start] + '    "' + key + '": ' + insert["dump"](asset[key], 2) + result[end:]
    check = json.loads(result)["asset"]
    if any(check[key] != asset[key] for key in ("dimensions", "rig", "anchors", "representations", "symbols")):
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original:
        raise SystemExit("the catalog file changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(result)
    json.dump({"volumes": asset["volumes"]}, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
